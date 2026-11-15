"""Local, unprivileged entry to the authenticated host installer.

The privileged entry accepts only a fixed socket start or a read-only inspection.
It does not install/start a center, enable a unit, change trust, or export a key.
"""
import argparse
import base64
import contextlib
import datetime
import errno
import fcntl
import hashlib
import json
import os
from pathlib import Path
import re
import socket
import ssl
import stat
import subprocess
import sys
import threading
import uuid

from host_control import HostControl
from setup import Setup

URI = 'https://localhost:9090/yofc-console-setup'
HELPER = '/usr/sbin/kura-console-host-ui'
CONTROL = '/var/lib/yofc-console-host-ui'
CERTIFICATES = '/etc/cockpit/ws-certs.d'
START = ['/usr/bin/systemctl', '--no-ask-password', 'start', 'cockpit.socket']
PROPERTIES = ['LoadState', 'ActiveState', 'SubState', 'Job']


def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def digest(value):
    return hashlib.sha256(json.dumps(value, sort_keys=True).encode()).hexdigest()


def protected_directory(path):
    info = path.lstat()
    if not stat.S_ISDIR(info.st_mode) or info.st_uid != 0 or info.st_mode & 0o022:
        raise RuntimeError('Host installation records must be protected by the administrator')


def loopback_certificate():
    """Read only the peer certificate; never send HTTP or credentials."""
    peers = []
    absent = {errno.ECONNREFUSED, errno.EAFNOSUPPORT, errno.ENETUNREACH,
              errno.EADDRNOTAVAIL, errno.EHOSTUNREACH}
    for address in ['127.0.0.1', '::1']:
        context = ssl.SSLContext(ssl.PROTOCOL_TLS_CLIENT)
        context.check_hostname = False
        context.verify_mode = ssl.CERT_NONE
        try:
            with socket.create_connection((address, 9090), timeout=3) as raw:
                with context.wrap_socket(raw, server_hostname='localhost') as connection:
                    peers.append(connection.getpeercert(binary_form=True))
        except OSError as exc:
            if exc.errno not in absent:
                raise RuntimeError('The local HTTPS certificate could not be checked') from None
    if not peers or any(value != peers[0] for value in peers):
        raise RuntimeError('The local HTTPS listeners do not present one verifiable certificate')
    return peers[0]


class HostUi:
    def __init__(self, setup=None, peer=loopback_certificate):
        self.setup = setup or Setup()
        self.store = HostControl(self.setup)
        self.store.directory = self.setup.path(CONTROL)
        self.peer = peer

    def operations(self):
        directory = self.store.directory
        if not directory.exists() and not directory.is_symlink():
            return []
        protected_directory(directory)
        protected_directory(directory / 'operations')
        accepted = self.store.read('accepted.json')
        if not isinstance(accepted, list) or len(set(accepted)) != len(accepted) \
                or any(identity != HostControl.identity(identity) for identity in accepted):
            raise RuntimeError('The retained host installation registry requires review')
        values = []
        for path in sorted((directory / 'operations').iterdir()):
            if path.suffix != '.json' or path.stem != HostControl.identity(path.stem):
                raise RuntimeError('Retained host installation records require review')
            value = self.store.read('operations/' + path.name)
            identity = HostControl.identity(value.get('operationId', ''))
            intent = value.get('intent')
            if type(value.get('schemaVersion')) is not int or value['schemaVersion'] != 1 \
                    or identity != path.stem or not isinstance(intent, dict) \
                    or set(intent) != {'operationId', 'command', 'requestedAt'} \
                    or not isinstance(intent.get('requestedAt'), str) or not intent['requestedAt'] \
                    or intent.get('operationId') != identity or intent.get('command') != START \
                    or value.get('intentSha256') != digest(intent):
                raise RuntimeError('The original host installation request is not verifiable')
            receipt = value.get('receipt')
            if receipt is not None and (not isinstance(receipt, dict)
                    or receipt.get('operationId') != identity
                    or receipt.get('intentSha256') != value['intentSha256']
                    or receipt.get('command') != START
                    or type(receipt.get('returnCode')) is not int or receipt['returnCode'] != 0):
                raise RuntimeError('The original host installation receipt is not verifiable')
            values.append(dict(operationId=identity, phase='APPLIED' if receipt else 'UNKNOWN',
                               intentSha256=value['intentSha256'], receipt=receipt))
        if {row['operationId'] for row in values} != set(accepted):
            raise RuntimeError('A retained host installation request is missing or incomplete')
        return values

    def service(self):
        result = self.setup.run(['/usr/bin/systemctl', 'show', 'cockpit.socket', '--property',
                                 ','.join(PROPERTIES)], capture_output=True, text=True, timeout=15)
        if result.returncode:
            raise RuntimeError('The host installation service could not be inspected')
        values = dict(line.split('=', 1) for line in result.stdout.splitlines() if '=' in line)
        if any(key not in values for key in PROPERTIES):
            raise RuntimeError('The host installation service status is incomplete')
        return {key: values[key] for key in PROPERTIES}

    def start(self, operation):
        identity = HostControl.identity(operation)
        first = not self.store.directory.exists() and not self.store.directory.is_symlink()
        with self.store.locked():
            if first and self.store.read('accepted.json', optional=True) is None:
                if any((self.store.directory / 'operations').iterdir()):
                    raise RuntimeError('Retained host installation requests require review')
                self.store.write('accepted.json', [], new=True)
            operations = self.operations()
            retained = next((value for value in operations if value['operationId'] == identity), None)
            unresolved = next((value for value in operations if value['phase'] == 'UNKNOWN'), None)
            if retained or unresolved:
                return dict(operationId=identity, startIssued=False, phase='ORIGINAL_RETAINED',
                            originalOperation=retained or unresolved)
            if len(operations) >= 1024:
                return dict(operationId=identity, startIssued=False, phase='REJECTED_BEFORE_START',
                            reason='HOST_UI_HISTORY_REQUIRES_REVIEW')
            service = self.service()
            if service['LoadState'] != 'loaded' or service['Job'] not in ['', '0'] \
                    or service['ActiveState'] not in ['inactive', 'failed']:
                return dict(operationId=identity, startIssued=False, phase='REJECTED_BEFORE_START',
                            service=service)
            intent = dict(operationId=identity, command=START, requestedAt=now())
            record = dict(schemaVersion=1, operationId=identity, intent=intent,
                          intentSha256=digest(intent), receipt=None)
            name = 'operations/' + identity + '.json'
            self.store.write(name, record, new=True)
            self.store.write('accepted.json', [row['operationId'] for row in operations] + [identity])
            try:
                result = self.setup.run(START, capture_output=True, text=True, timeout=30)
            except (OSError, subprocess.TimeoutExpired):
                record['failure'] = 'COMMAND_RESULT_UNKNOWN'
            else:
                if result.returncode == 0:
                    record['receipt'] = dict(operationId=identity, intentSha256=record['intentSha256'],
                                            command=START, returnCode=0, completedAt=now())
                else:
                    record['failure'] = 'COMMAND_DID_NOT_RETURN_SUCCESS'
                    record['commandReturnCode'] = result.returncode
            self.store.write(name, record)
            return dict(operationId=identity, startIssued=True,
                        phase='APPLIED' if record['receipt'] else 'UNKNOWN')

    def certificate_files(self):
        directory = self.setup.path(CERTIFICATES)
        protected_directory(directory)
        values = []
        for path in sorted(directory.glob('*.cert')):
            descriptor = os.open(path, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK)
            with os.fdopen(descriptor, 'rb') as stream:
                info = os.fstat(stream.fileno())
                if not stat.S_ISREG(info.st_mode) or info.st_uid != 0 or info.st_mode & 0o022 \
                        or info.st_size > 1048576:
                    raise RuntimeError('The host public certificate must be protected by the administrator')
                data = stream.read(1048577)
                if len(data) > 1048576:
                    raise RuntimeError('The host certificate file is too large')
            match = re.search(br'-----BEGIN CERTIFICATE-----\s*([A-Za-z0-9+/=\r\n]+)\s*-----END CERTIFICATE-----', data)
            if match is None:
                raise RuntimeError('A host certificate file contains no public certificate')
            try:
                der = base64.b64decode(re.sub(br'\s', b'', match.group(1)), validate=True)
            except ValueError:
                raise RuntimeError('The host public certificate is not valid PEM') from None
            values.append(dict(path=str(path.relative_to(self.setup.root)),
                               sha256=hashlib.sha256(der).hexdigest(), publicPem=ssl.DER_cert_to_PEM_cert(der),
                               fileIdentity=self.file_identity(info)))
        return values

    @staticmethod
    def file_identity(info):
        return dict(device=info.st_dev, inode=info.st_ino, size=info.st_size,
                    modifiedNs=info.st_mtime_ns, uid=info.st_uid, mode=stat.S_IMODE(info.st_mode))

    def snapshot(self, authorized_certificate=None):
        operations = self.operations()
        service = self.service()
        value = dict(schemaVersion=1, uri=URI, service=service, operations=operations,
                     certificate=None, certificateError=None, certificateAuthorizationRequired=False)
        if service['ActiveState'] != 'active' or service['Job'] not in ['', '0']:
            return value
        try:
            # A normal TLS handshake also activates Cockpit's stock TLS listener.
            peer = self.peer()
            try:
                candidates = self.certificate_files()
            except PermissionError:
                value['certificateAuthorizationRequired'] = True
                if authorized_certificate is None:
                    return value
                candidate = authorized_certificate
                relative = Path(candidate['path'])
                if relative.parent != Path(CERTIFICATES.lstrip('/')) or relative.suffix != '.cert':
                    raise RuntimeError('Unexpected authorized public certificate path')
                try:
                    der = ssl.PEM_cert_to_DER_cert(candidate['publicPem'])
                except (TypeError, ValueError):
                    raise RuntimeError('The authorized public certificate is not verifiable') from None
                if ssl.DER_cert_to_PEM_cert(der) != candidate['publicPem'] \
                        or hashlib.sha256(der).hexdigest() != candidate['sha256']:
                    raise RuntimeError('The authorized public certificate is not verifiable')
                path = self.setup.path('/' + str(relative))
                protected_directory(path.parent)
                info = path.lstat()
                if not stat.S_ISREG(info.st_mode) or info.st_uid != 0 or info.st_mode & 0o022 \
                        or self.file_identity(info) != candidate['fileIdentity']:
                    raise RuntimeError('The authorized host certificate has changed; inspect it again')
                candidates = [candidate]
            peer_sha = hashlib.sha256(peer).hexdigest()
            matches = [candidate for candidate in candidates if candidate['sha256'] == peer_sha]
            if not matches:
                raise RuntimeError('The HTTPS peer does not match an administrator-protected host certificate')
            value['certificate'] = matches[0]
            value['certificateAuthorizationRequired'] = False
        except (OSError, RuntimeError) as exc:
            value['certificateError'] = str(exc)
        return value


class UserRequests:
    """Persist the public original UUID before requesting administrator authorization."""
    def __init__(self, directory=None):
        self.directory = directory or Path.home() / '.local/state/yofc-console-host-ui'

    def check_directory(self):
        info = self.directory.lstat()
        if not stat.S_ISDIR(info.st_mode) or info.st_uid != os.getuid() or info.st_mode & 0o077:
            raise RuntimeError('The retained local request directory requires review')

    def read(self):
        if not self.directory.exists() and not self.directory.is_symlink():
            return []
        self.check_directory()
        path = self.directory / 'requests.json'
        try:
            descriptor = os.open(path, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK)
        except FileNotFoundError:
            if path.is_symlink() or any(p.name != 'requests.lock' for p in self.directory.iterdir()):
                raise RuntimeError('The retained local request file requires review') from None
            return []
        with os.fdopen(descriptor, 'rb') as stream:
            info = os.fstat(stream.fileno())
            if not stat.S_ISREG(info.st_mode) or info.st_uid != os.getuid() or info.st_mode & 0o077 \
                    or info.st_size > 4194304:
                raise RuntimeError('The retained local request file requires review')
            contents = stream.read(4194305)
            if len(contents) > 4194304:
                raise RuntimeError('The retained local request file is too large')
            data = json.loads(contents)
        if not isinstance(data, list) or any(not isinstance(row, dict)
                or row.get('operationId') != HostControl.identity(row.get('operationId', ''))
                or row.get('transportOutcome') not in ['INTENT', 'UNKNOWN', 'REPLIED',
                    'AUTHORIZATION_NOT_GRANTED', 'REJECTED_BEFORE_START', 'EXEC_NOT_STARTED'] for row in data):
            raise RuntimeError('The retained local request file is invalid')
        if len({row['operationId'] for row in data}) != len(data):
            raise RuntimeError('The retained local requests are not unique')
        return data

    @staticmethod
    def pending(rows, operations):
        applied = {row['operationId'] for row in operations if row['phase'] == 'APPLIED'
                   and isinstance(row.get('receipt'), dict)
                   and row['receipt'].get('operationId') == row['operationId']
                   and row['receipt'].get('intentSha256') == row.get('intentSha256')
                   and row['receipt'].get('command') == START
                   and type(row['receipt'].get('returnCode')) is int and row['receipt']['returnCode'] == 0}
        return [row for row in rows if row['transportOutcome'] in ['INTENT', 'UNKNOWN', 'REPLIED']
                and row['operationId'] not in applied]

    @contextlib.contextmanager
    def locked(self):
        self.directory.mkdir(parents=True, mode=0o700, exist_ok=True)
        self.check_directory()
        descriptor = os.open(self.directory / 'requests.lock', os.O_CREAT | os.O_RDWR | os.O_NOFOLLOW, 0o600)
        try:
            info = os.fstat(descriptor)
            if not stat.S_ISREG(info.st_mode) or info.st_uid != os.getuid() or info.st_mode & 0o077:
                raise RuntimeError('The local request lock requires review')
            fcntl.flock(descriptor, fcntl.LOCK_EX | fcntl.LOCK_NB)
            yield
        finally:
            os.close(descriptor)

    def write(self, rows):
        marker = self.directory / 'requests.seen'
        try:
            descriptor = os.open(marker, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o600)
        except FileExistsError:
            info = marker.lstat()
            if not stat.S_ISREG(info.st_mode) or info.st_uid != os.getuid() or info.st_mode & 0o077:
                raise RuntimeError('The retained local request marker requires review')
        else:
            with os.fdopen(descriptor, 'w') as stream:
                stream.write('1\n')
                stream.flush()
                os.fsync(stream.fileno())
        temporary = self.directory / ('.requests-' + uuid.uuid4().hex)
        descriptor = os.open(temporary, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o600)
        try:
            with os.fdopen(descriptor, 'w') as stream:
                json.dump(rows, stream, sort_keys=True)
                stream.write('\n')
                stream.flush()
                os.fsync(stream.fileno())
            os.replace(temporary, self.directory / 'requests.json')
            descriptor = os.open(self.directory, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW)
            try:
                os.fsync(descriptor)
            finally:
                os.close(descriptor)
        finally:
            temporary.unlink(missing_ok=True)

    def begin(self, operations):
        with self.locked():
            rows = self.read()
            if self.pending(rows, operations) or any(row['phase'] == 'UNKNOWN' for row in operations):
                raise RuntimeError('Observe the original request; an unconfirmed start cannot be submitted again')
            identity = str(uuid.uuid4())
            rows.append(dict(operationId=identity, requestedAt=now(), transportOutcome='INTENT'))
            self.write(rows)
            return identity

    def finish(self, identity, returncode, response, operations, helper_unchanged):
        with self.locked():
            rows = self.read()
            record = next(row for row in rows if row['operationId'] == identity)
            original = next((row for row in operations if row['operationId'] == identity), None)
            outcome = 'UNKNOWN'
            # The fixed helper reserves 126/127 for pkexec, never forwarding a
            # child return code. A root intent always overrides that inference.
            if returncode in [126, 127] and original is None and helper_unchanged:
                outcome = 'AUTHORIZATION_NOT_GRANTED'
            elif returncode == 0 and isinstance(response, dict) and response.get('operationId') == identity:
                if response.get('startIssued') is False and original is None:
                    outcome = 'REJECTED_BEFORE_START'
                elif original and original['phase'] == 'APPLIED' and original['receipt']:
                    outcome = 'REPLIED'
            elif returncode == 'EXEC_NOT_STARTED':
                outcome = 'EXEC_NOT_STARTED'
            record.update(transportOutcome=outcome, processReturnCode=returncode, observedAt=now())
            if original and original['receipt']:
                record['originalReceipt'] = original['receipt']
            self.write(rows)


def verify_tools():
    from installer import verify_installer_payload
    value = verify_installer_payload()
    if value.get('hostUiBootstrapProtocolVersion') != 1 or value.get('desktopIntegration') is not True:
        raise RuntimeError('This tool package does not declare the host desktop bootstrap protocol')
    return value


def gui():
    if os.geteuid() == 0:
        raise RuntimeError('Open the installation window as the desktop user')
    verify_tools()
    import gi
    gi.require_version('Gtk', '3.0')
    from gi.repository import GLib, Gtk
    GLib.set_prgname('yofc-console-host-ui')
    GLib.set_application_name('YOFC Console Setup')
    chinese = os.environ.get('LANG', '').lower().startswith('zh')
    text = lambda english, zh: zh if chinese else english
    control, requests = HostUi(), UserRequests()
    window = Gtk.Window(title=text('YOFC Console Setup', 'YOFC 管理中心安装设置'))
    window.set_default_size(820, 470)
    window.set_border_width(20)
    box = Gtk.Box(orientation=Gtk.Orientation.VERTICAL, spacing=14)
    window.add(box)

    def label(value):
        widget = Gtk.Label(label=value, xalign=0)
        widget.set_line_wrap(True)
        widget.set_max_width_chars(90)
        widget.set_selectable(True)
        box.pack_start(widget, False, False, 0)
        return widget

    label(text('Open the authenticated host installation page. Enabling this page does not start a management center.',
               '打开需要主机管理员登录的安装页面。启用此页面不会启动管理中心。'))
    status_label = label(text('Reading host status…', '正在读取主机状态…'))
    fingerprint = label('')
    label(text('Compare this SHA-256 with the browser certificate details before trusting this host. '
               'Saving a certificate exports only its public certificate; no private key is included.',
               '请将此 SHA-256 与浏览器证书详情核对，匹配后再信任该主机。保存内容只含公开证书，不含任何私钥。'))
    label(URI)
    buttons = Gtk.Box(spacing=8)
    box.pack_start(buttons, False, False, 0)
    cache = {'certificate': None, 'snapshot': None, 'busy': False}

    def button(english, zh):
        widget = Gtk.Button(label=text(english, zh))
        buttons.pack_start(widget, False, False, 0)
        return widget

    refresh = button('Read host status', '读取主机状态')
    start = button('Enable installation page', '启用安装页面')
    authorized = button('Read public certificate as administrator', '授权读取公开证书')
    save = button('Save public certificate', '保存公开证书')
    browser = button('Open browser', '打开浏览器')
    widgets = [refresh, start, authorized, save, browser]

    def render(value=None, error=None):
        cache['busy'] = False
        for widget in widgets:
            widget.set_sensitive(False)
        refresh.set_sensitive(True)
        if error:
            fingerprint.set_text('')
            status_label.set_text(text('Host status could not be verified: ', '无法核对主机状态：') + str(error))
            return False
        cache['snapshot'] = value
        try:
            rows = requests.read()
        except Exception as exc:
            return render(error=str(exc))
        pending = UserRequests.pending(rows, value['operations'])
        unresolved = [row for row in value['operations'] if row['phase'] == 'UNKNOWN']
        service = value['service']
        message = text('Host installation page: ', '主机安装页面：') + service['ActiveState']
        try:
            session_ready = (Path('/run/user') / str(os.getuid()) / 'bus').is_socket()
        except OSError:
            session_ready = False
        if not session_ready:
            message += '\n' + text('The current desktop login session is not ready for host administration. '
                                   'Sign out and sign in again after installing the session component.',
                                   '当前桌面登录会话尚未就绪。会话组件安装后，请退出桌面并重新登录，再使用主机管理页面。')
        if pending or unresolved:
            identity = (unresolved or pending)[0]['operationId']
            message += '\n' + text('Original request is unconfirmed; observe it without resubmitting: ',
                                    '原请求尚未核定，请只查询，不要再次启用：') + identity
        if value['certificateError']:
            message += '\n' + value['certificateError']
        status_label.set_text(message)
        certificate = value['certificate']
        if certificate:
            cache['certificate'] = certificate
            fingerprint.set_text('SHA-256: ' + ':'.join(certificate['sha256'][i:i+2].upper() for i in range(0, 64, 2)))
        else:
            fingerprint.set_text(text('Host public certificate has not been verified.', '尚未核对主机公开证书。'))
        blocked = bool(pending or unresolved)
        start.set_sensitive(not blocked and service['LoadState'] == 'loaded'
                            and service['ActiveState'] in ['inactive', 'failed'] and service['Job'] in ['', '0'])
        authorized.set_sensitive(value['certificateAuthorizationRequired'])
        save.set_sensitive(bool(certificate) and not blocked)
        browser.set_sensitive(bool(certificate) and not blocked)
        return False

    def work(function):
        cache['busy'] = True
        for widget in widgets:
            widget.set_sensitive(False)
        def run():
            try:
                value = function()
            except Exception as exc:
                GLib.idle_add(render, None, str(exc))
            else:
                GLib.idle_add(render, value, None)
        threading.Thread(target=run, daemon=True).start()

    def inspect():
        verify_tools()
        return control.snapshot(cache['certificate'])

    def enable(_):
        def action():
            before = verify_tools()
            identity = requests.begin(control.operations())
            try:
                result = subprocess.run(['/usr/bin/pkexec', '--disable-internal-agent', HELPER,
                                         'start', '--operation', identity], capture_output=True, text=True)
            except OSError:
                requests.finish(identity, 'EXEC_NOT_STARTED', None, control.operations(), True)
                raise RuntimeError('The system authorization program could not be started') from None
            try:
                response = json.loads(result.stdout)
            except ValueError:
                response = None
            try:
                unchanged = verify_tools() == before
            except Exception:
                unchanged = False
            requests.finish(identity, result.returncode, response, control.operations(), unchanged)
            return inspect()
        work(action)

    def privileged_read(_):
        def action():
            verify_tools()
            result = subprocess.run(['/usr/bin/pkexec', '--disable-internal-agent', HELPER, 'inspect'],
                                    capture_output=True, text=True)
            if result.returncode:
                raise RuntimeError('Administrator authorization did not return a public certificate')
            value = json.loads(result.stdout)
            if value.get('schemaVersion') != 1 or value.get('uri') != URI:
                raise RuntimeError('The authorized host inspection is not valid')
            cache['certificate'] = value.get('certificate')
            return inspect()
        work(action)

    def save_certificate(_):
        try:
            value = inspect()
            if not value['certificate'] or UserRequests.pending(requests.read(), value['operations']) \
                    or any(row['phase'] == 'UNKNOWN' for row in value['operations']):
                raise RuntimeError('Verify the original request and host certificate first')
            dialog = Gtk.FileChooserDialog(title=text('Save host public certificate', '保存主机公开证书'),
                parent=window, action=Gtk.FileChooserAction.SAVE)
            dialog.add_buttons(Gtk.STOCK_CANCEL, Gtk.ResponseType.CANCEL, Gtk.STOCK_SAVE, Gtk.ResponseType.OK)
            folder = Path(GLib.get_user_special_dir(GLib.UserDirectory.DIRECTORY_DOWNLOAD) or str(Path.home()))
            dialog.set_current_folder(str(folder if folder.is_dir() else Path.home()))
            dialog.set_current_name('yofc-host-public-certificate.pem')
            try:
                if dialog.run() == Gtk.ResponseType.OK:
                    current = inspect()
                    if not current['certificate'] or current['certificate']['sha256'] != value['certificate']['sha256'] \
                            or UserRequests.pending(requests.read(), current['operations']) \
                            or any(row['phase'] == 'UNKNOWN' for row in current['operations']):
                        raise RuntimeError('The host certificate or original request has changed; read it again')
                    filename = dialog.get_filename()
                    descriptor = os.open(filename, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o600)
                    with os.fdopen(descriptor, 'w') as stream:
                        stream.write(current['certificate']['publicPem'])
                        stream.flush()
                        os.fsync(stream.fileno())
            finally:
                dialog.destroy()
            render(value)
        except Exception:
            try:
                render(inspect())
                status_label.set_text(text('The public certificate was not saved. Read the host status and choose a new file in a writable folder.',
                                           '公开证书未保存。请重新读取主机状态，并在可写目录选择新文件名。'))
            except Exception as exc:
                render(error=exc)

    def open_browser(_):
        def action():
            value = inspect()
            if not value['certificate'] or UserRequests.pending(requests.read(), value['operations']) \
                    or any(row['phase'] == 'UNKNOWN' for row in value['operations']):
                raise RuntimeError('Verify the original request and host certificate first')
            subprocess.Popen(['/usr/bin/xdg-open', URI], stdin=subprocess.DEVNULL,
                             stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, start_new_session=True)
            return value
        work(action)

    refresh.connect('clicked', lambda _: work(inspect))
    start.connect('clicked', enable)
    authorized.connect('clicked', privileged_read)
    save.connect('clicked', save_certificate)
    browser.connect('clicked', open_browser)
    window.connect('destroy', Gtk.main_quit)
    window.show_all()
    work(inspect)
    Gtk.main()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest='mode', required=True)
    sub.add_parser('gui')
    root = sub.add_parser('root')
    root.add_argument('action', choices=['start', 'inspect'])
    root.add_argument('--operation')
    args = parser.parse_args()
    if args.mode == 'gui':
        gui()
        return
    if os.geteuid() != 0:
        raise RuntimeError('System administrator authorization is required')
    verify_tools()
    control = HostUi()
    if args.action == 'start':
        if args.operation is None:
            raise RuntimeError('The original request identifier is required')
        value = control.start(args.operation)
    else:
        if args.operation is not None:
            raise RuntimeError('Unexpected operation selection for a read-only inspection')
        value = control.snapshot()
    print(json.dumps(value))


if __name__ == '__main__':
    try:
        main()
    except Exception as exc:
        print(str(exc), file=sys.stderr)
        # Do not propagate child exit codes: 126/127 belong only to pkexec.
        sys.exit(1)
