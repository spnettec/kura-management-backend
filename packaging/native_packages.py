#!/usr/bin/python3
"""Approved native packages, retained original jobs and a held maintenance fence.

Only a root-packaged catalog can approve a deb. Browser-supplied paths, hashes,
versions, apt arguments and commands are never accepted. Package installation
does not start or implicitly stop a center.
"""
from contextlib import contextmanager
import argparse
import fcntl
import hashlib
import json
import os
from pathlib import Path, PurePosixPath
import platform
import re
import stat
import subprocess
import sys
import time
from docker_control import atomic, protected, ENV
from host_control import HostControl
from setup import Setup

BASE = '/var/lib/yofc-console-native-packages'
CATALOG = '/usr/share/kura-management-console-installer/packages.json'
PROGRAM = '/usr/lib/kura-management-console/native_packages.py'
PACKAGE = 'kura-management-console'
MAX_PACKAGE = 256 * 1024 * 1024
TERMINAL = {'APPLIED', 'RETIRED'}


class NativePackages:
    def __init__(self, setup=None):
        self.setup = setup or Setup()
        self.directory = self.setup.path(BASE)

    @staticmethod
    def identity(value):
        return HostControl.identity(value)

    def prepare(self):
        for directory in [self.directory, self.directory/'operations', self.directory/'uploads']:
            directory.mkdir(mode=0o700, parents=True, exist_ok=True)
            info = directory.lstat()
            if not stat.S_ISDIR(info.st_mode) or info.st_uid != 0 or info.st_mode & 0o077:
                raise RuntimeError('Root-private native package directories required')

    @contextmanager
    def locked(self, wait=False):
        self.prepare()
        fd = os.open(self.directory/'install.lock', os.O_RDWR | os.O_CREAT | os.O_NOFOLLOW | os.O_NONBLOCK, 0o600)
        try:
            info = os.fstat(fd)
            if not stat.S_ISREG(info.st_mode) or info.st_uid != 0 or info.st_mode & 0o077:
                raise RuntimeError('Invalid native package lock')
            deadline = time.monotonic() + (10 if wait else 0)
            while True:
                try:
                    fcntl.flock(fd, fcntl.LOCK_EX | fcntl.LOCK_NB)
                    break
                except BlockingIOError:
                    if time.monotonic() >= deadline:
                        raise
                    time.sleep(0.02)
            yield
        finally:
            os.close(fd)

    def path(self, operation):
        return self.directory/'operations'/(self.identity(operation)+'.json')

    def record(self, operation):
        value = json.loads(protected(self.path(operation)))
        if value.get('schemaVersion') != 1 or value.get('operationId') != operation:
            raise RuntimeError('Invalid original native package journal')
        return value

    def save(self, value):
        atomic(self.path(value['operationId']), value)

    def catalog(self):
        catalog = json.loads(protected(self.setup.path(CATALOG)))
        return self.validate_catalog(catalog)

    @staticmethod
    def validate_catalog(catalog):
        rows = catalog.get('packages')
        if type(catalog.get('schemaVersion')) is not int or catalog['schemaVersion'] != 1 or not isinstance(rows, list) or len(rows) > 64:
            raise RuntimeError('Invalid native release catalog')
        keys = set()
        for row in rows:
            if not re.fullmatch(r'[a-z0-9][a-z0-9-]{0,63}', row.get('key', '')) or row['key'] in keys \
                    or row.get('package') != PACKAGE or row.get('architecture') not in ['arm64', 'amd64'] \
                    or not re.fullmatch(r'[0-9][A-Za-z0-9.+:~\-]{0,127}', row.get('version', '')) \
                    or type(row.get('archiveSize')) is not int or not 0 < row['archiveSize'] <= MAX_PACKAGE \
                    or any(not re.fullmatch(r'[a-f0-9]{64}', row.get(name, '')) for name in ['archiveSha256', 'binarySha256', 'sourceManifestSha256']) \
                    or type(row.get('maintenanceProtocolVersion')) is not int or row['maintenanceProtocolVersion'] != 1 \
                    or type(row.get('hostPackageFenceProtocolVersion')) is not int or row['hostPackageFenceProtocolVersion'] != 1:
                raise RuntimeError('Invalid approved native release')
            files = row.get('files')
            if not isinstance(files, list) or not 1 <= len(files) <= 512:
                raise RuntimeError('Complete native payload pins required')
            paths = set()
            for entry in files:
                name = entry.get('path', '')
                path = PurePosixPath(name)
                allowed = name in ['/usr/bin/yofc-management-console',
                    '/usr/lib/systemd/system/kura-management-console.service',
                    '/usr/lib/systemd/system/kura-management-console@.service'] or name.startswith(
                    ('/usr/share/kura-management-console/', '/usr/share/doc/kura-management-console/'))
                if not allowed or str(path) != name or '..' in path.parts or name in paths \
                        or not re.fullmatch(r'[a-f0-9]{64}', entry.get('sha256', '')) \
                        or entry.get('mode') not in [0o644, 0o755]:
                    raise RuntimeError('Invalid native payload pin')
                paths.add(name)
            pins = {entry['path']: entry['sha256'] for entry in files}
            if pins.get('/usr/bin/yofc-management-console') != row['binarySha256'] or pins.get(
                    '/usr/share/kura-management-console/source-manifest.json') != row['sourceManifestSha256'] \
                    or not {'/usr/lib/systemd/system/kura-management-console.service',
                            '/usr/lib/systemd/system/kura-management-console@.service',
                            '/usr/share/kura-management-console/local-command-provenance.json',
                            '/usr/share/kura-management-console/package-v1',
                            '/usr/share/kura-management-console/react/index.html'} <= pins.keys() \
                    or not any(name.startswith('/usr/share/kura-management-console/react/assets/') and name.endswith('.js') for name in pins):
                raise RuntimeError('Native executable/source pins required')
            keys.add(row['key'])
        return rows

    def package(self, key):
        selected = next((row for row in self.catalog() if row['key'] == key), None)
        if selected is None or selected['architecture'] != {'aarch64': 'arm64', 'x86_64': 'amd64'}.get(platform.machine()):
            raise RuntimeError('Select an approved package for this host architecture')
        return selected

    def unresolved(self):
        if not self.directory.exists():
            return []
        self.setup.path(BASE+'/operations')
        return [value for file in sorted((self.directory/'operations').glob('*.json'))
                if (value := self.status(file.stem))['phase'] not in TERMINAL]

    def no_other_job(self, operation):
        if any(row['operationId'] != operation for row in self.unresolved()):
            raise RuntimeError('Observe the original package job before accepting another')

    def upload_path(self, operation, accepted=False):
        return self.directory/'uploads'/(self.identity(operation)+('.accepted.deb' if accepted else '.deb'))

    def file_matches(self, path, package):
        try:
            with os.fdopen(os.open(path, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK), 'rb') as stream:
                info = os.fstat(stream.fileno())
                return stat.S_ISREG(info.st_mode) and info.st_uid == 0 and not info.st_mode & 0o022 \
                    and info.st_size == package['archiveSize'] \
                    and hashlib.file_digest(stream, 'sha256').hexdigest() == package['archiveSha256']
        except FileNotFoundError:
            return False

    def unit(self, operation):
        return 'yofc-console-package-'+self.identity(operation)+'.service'

    def live(self, operation):
        try:
            result = subprocess.run(['/usr/bin/systemctl', 'show', self.unit(operation), '-p', 'ActiveState', '-p', 'MainPID'],
                env=ENV, capture_output=True, text=True, timeout=3, check=True)
            fields = dict(line.split('=', 1) for line in result.stdout.splitlines() if '=' in line)
            pid = fields.get('MainPID', '')
            if fields.get('ActiveState') not in ['active', 'activating'] or not re.fullmatch(r'[1-9][0-9]*', pid):
                return False
            return (Path('/proc')/pid/'cmdline').read_bytes().rstrip(b'\0').split(b'\0') == [
                b'/usr/bin/python3', PROGRAM.encode(), b'worker', b'--operation', operation.encode()]
        except (OSError, ValueError, subprocess.SubprocessError):
            return False

    def status(self, operation):
        self.identity(operation)
        try:
            value = dict(self.record(operation))
        except FileNotFoundError:
            return {'operationId': operation, 'phase': 'UNOBSERVED', 'transportNonDeliveryCertified': False}
        if value['phase'] == 'APPLIED':
            try:
                expected = {'maintenanceProtocolVersion': 1, 'binarySha256': value['package']['binarySha256'],
                            'sourceManifestSha256': value['package']['sourceManifestSha256']}
                receipt = HostControl(self.setup).journal(value['maintenanceOperationId'])
                if value.get('installReceipt') != {'commandReturnedSuccess': True} or value.get('verifiedPayload') != expected \
                        or receipt.get('phase') != 'COMPLETE' or receipt.get('verifiedPayload') != expected \
                        or receipt.get('packageInstallOperationId') != operation:
                    value['phase'] = 'UNKNOWN'
            except (OSError, ValueError, RuntimeError, KeyError, TypeError):
                value['phase'] = 'UNKNOWN'
        if value['phase'] == 'RETIRED' and (value.get('lateActionsFenced') is not True or any(
                value.get(key) for key in ['sealIntent', 'launchIntent', 'installIntent', 'installReceipt'])):
            value['phase'] = 'UNKNOWN'
        if value['phase'] in ['SEALING', 'ACCEPTED', 'RUNNING']:
            value['phase'] = 'PENDING' if self.live(operation) else 'UNKNOWN'
        if value['phase'] == 'UPLOADING':
            value['uploadReady'] = self.file_matches(self.upload_path(operation), value['package'])
            value['uploadPath'] = str(self.upload_path(operation))
        value['canVerify'] = value['phase'] == 'UNKNOWN' and value.get('installReceipt') == {'commandReturnedSuccess': True}
        if value['phase'] == 'APPLIED':
            control = HostControl(self.setup)
            hold = control.package_hold()
            value['finalizationRequired'] = hold is not None and hold['packageOperationId'] == operation
        return value

    def installed_core(self):
        """Inspect fixed package metadata and provenance; never probe the binary.

        This is a software prerequisite, not readiness or an installation receipt.
        An absent package cannot clear any retained native operation or state.
        """
        try:
            observed = self.setup.run(['/usr/bin/dpkg-query', '--show',
                '--showformat=${Status}\n${Version}\n${Architecture}\n', PACKAGE], env=ENV,
                capture_output=True, text=True, timeout=10)
            fixed = ['/usr/bin/yofc-management-console',
                     '/usr/share/kura-management-console/local-command-provenance.json',
                     '/usr/share/kura-management-console/source-manifest.json',
                     '/usr/share/kura-management-console/package-v1',
                     '/usr/lib/systemd/system/kura-management-console.service',
                     '/usr/lib/systemd/system/kura-management-console@.service',
                     '/usr/share/kura-management-console/react/index.html']
            paths = [self.setup.path(name) for name in fixed]
            metadata = observed.stdout.splitlines()
            absent = observed.returncode == 1 and not observed.stdout or observed.returncode == 0 \
                and len(metadata) == 3 and metadata[0] in ['unknown ok not-installed',
                    'deinstall ok not-installed', 'purge ok not-installed', 'deinstall ok config-files']
            if absent and not any(path.exists() for path in paths):
                return {'state': 'ABSENT'}
            architecture = {'aarch64': 'arm64', 'x86_64': 'amd64'}.get(platform.machine())
            if observed.returncode or len(metadata) != 3 or metadata[0] != 'install ok installed' \
                    or not re.fullmatch(r'[0-9][A-Za-z0-9.+:~\-]*', metadata[1]) or metadata[2] != architecture:
                return {'state': 'UNVERIFIED'}
            for path in paths:
                protected(path, maximum=MAX_PACKAGE)
            record = self.setup.verify_local_command('installation-status')
            return {'state': 'VERIFIED', 'version': metadata[1], 'architecture': metadata[2],
                    'binarySha256': record['binarySha256'],
                    'sourceManifestSha256': record['sourceManifestSha256']}
        except (OSError, ValueError, RuntimeError, TypeError, KeyError, AttributeError, subprocess.SubprocessError):
            return {'state': 'UNVERIFIED'}

    def overview(self):
        control = HostControl(self.setup)
        fields = [{'unit': item.unit, **control.state(item)} for item in control.participants()]
        active = control.status()
        pending = self.unresolved()
        arch = {'aarch64': 'arm64', 'x86_64': 'amd64'}.get(platform.machine())
        return {'packages': [row for row in self.catalog() if row['architecture'] == arch],
                'units': fields, 'allStopped': all(control.stopped(row) for row in fields),
                  'maintenance': active['active'], 'installedCore': self.installed_core(),
                'current': self.status(pending[0]['operationId']) if pending else None}

    def prepare_upload(self, operation, key):
        self.identity(operation)
        # A retired original is observed even after its old catalog was replaced.
        if self.path(operation).exists():
            old = self.record(operation)
            if old['phase'] == 'RETIRED':
                return self.status(operation)
        package = self.package(key)
        with self.locked():
            self.no_other_job(operation)
            if self.path(operation).exists():
                old = self.record(operation)
                if old.get('package') != package:
                    raise RuntimeError('Retain the original approved package selection')
                return self.status(operation)
            self.save({'schemaVersion': 1, 'operationId': operation, 'phase': 'UPLOADING', 'package': package})
        return self.status(operation)

    def retire(self, operation):
        self.identity(operation)
        with self.locked():
            try:
                value = self.record(operation)
            except FileNotFoundError:
                value = {'schemaVersion': 1, 'operationId': operation, 'phase': 'UPLOADING'}
            if value['phase'] not in ['UPLOADING', 'RETIRED'] or value.get('launchIntent') or value.get('sealIntent'):
                raise RuntimeError('An accepted or uncertain install cannot be retired')
            value.update(phase='RETIRED', lateActionsFenced=True, transportNonDeliveryCertified=False)
            self.save(value)
        return self.status(operation)

    def install(self, operation):
        original = self.record(operation)
        if original['phase'] != 'UPLOADING':
            return self.status(operation)  # Never dispatch an accepted original twice.
        with self.locked():
            value = self.record(operation)
            self.no_other_job(operation)
            if value['phase'] != 'UPLOADING':
                return self.status(operation)
            if self.package(value['package']['key']) != value['package']:
                raise RuntimeError('Original approved package catalog changed')
            if not self.file_matches(self.upload_path(operation), value['package']):
                return {**self.status(operation), 'rejectedBeforeIntent': True}
            control = HostControl(self.setup)
            with control.locked(), control.locked(writers=True):
                participants = control.participants()
                if not all(control.stopped(control.state(item)) for item in participants):
                    return {**self.status(operation), 'rejectedBeforeIntent': True}
                active = control.status()['active']
                if active and active['phase'] != 'STOPPED':
                    return {**self.status(operation), 'rejectedBeforeIntent': True}
                if not control.supported() and self.setup.path('/usr/bin/yofc-management-console').exists():
                    return {**self.status(operation), 'rejectedBeforeIntent': True}
                value['maintenanceOperationId'] = active['operationId'] if active else operation
            value.update(phase='SEALING', sealIntent=True)
            self.save(value)
            # Copy to a distinct inode. A delayed upload cannot mutate the sealed
            # bytes through an older open descriptor or an atomic replacement.
            target = self.upload_path(operation, accepted=True)
            with os.fdopen(os.open(target, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o600), 'wb') as output:
                with os.fdopen(os.open(self.upload_path(operation), os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK), 'rb') as source:
                    copied = 0
                    while chunk := source.read(65536):
                        copied += len(chunk)
                        if copied > value['package']['archiveSize']:
                            raise RuntimeError('Upload changed while sealing; preserve original job')
                        output.write(chunk)
                output.flush()
                os.fsync(output.fileno())
            if not self.file_matches(target, value['package']):
                raise RuntimeError('Sealed upload differs; preserve original job')
            parent = os.open(target.parent, os.O_RDONLY | os.O_DIRECTORY)
            try:
                os.fsync(parent)
            finally:
                os.close(parent)
            value.update(phase='ACCEPTED', launchIntent=True,
                controllerSha256=hashlib.sha256(protected(self.setup.path(PROGRAM))).hexdigest())
            self.save(value)
            try:
                result = subprocess.run(['/usr/bin/systemd-run', '--quiet', '--unit', self.unit(operation),
                    '--property=Type=exec', '--property=RuntimeMaxSec=1800', '--property=MemoryMax=384M',
                    '--property=CPUQuota=100%', '--property=StandardOutput=null', '--property=StandardError=null',
                    '/usr/bin/python3', PROGRAM, 'worker', '--operation', operation], env=ENV,
                    capture_output=True, timeout=8)
                if result.returncode:
                    raise RuntimeError('Package worker dispatch is unverified')
            except (OSError, RuntimeError, subprocess.TimeoutExpired):
                return {'operationId': operation, 'phase': 'UNKNOWN', 'transportNonDeliveryCertified': False}
        return self.status(operation)

    def verify_installed(self, package):
        observed = self.setup.run(['/usr/bin/dpkg-query', '--show',
            '--showformat=${Status}\n${Version}\n${Architecture}\n', PACKAGE], env=ENV,
            capture_output=True, text=True, timeout=10, check=True)
        if observed.stdout.splitlines() != ['install ok installed', package['version'], package['architecture']]:
            raise RuntimeError('Installed native package metadata differs')
        for row in package['files']:
            path = self.setup.path(row['path'])
            if hashlib.sha256(protected(path, maximum=MAX_PACKAGE)).hexdigest() != row['sha256'] \
                    or stat.S_IMODE(path.stat().st_mode) != row['mode']:
                raise RuntimeError('Installed native package payload differs')

    def finish(self, value):
        if value.get('installReceipt') != {'commandReturnedSuccess': True}:
            raise RuntimeError('Actual successful original package command receipt required')
        self.verify_installed(value['package'])
        control = HostControl(self.setup)
        maintenance = control.journal(value['maintenanceOperationId'])
        expected = {'maintenanceProtocolVersion': 1, 'binarySha256': value['package']['binarySha256'],
                    'sourceManifestSha256': value['package']['sourceManifestSha256']}
        if maintenance['phase'] != 'COMPLETE' or maintenance.get('verifiedPayload') != expected:
            raise RuntimeError('Original package post-install verification receipt required')
        value.update(phase='APPLIED', verifiedPayload=expected)
        self.save(value)  # Durable package receipt precedes release of its host fence.
        control.release_package(value['maintenanceOperationId'], value['operationId'])

    def verify(self, operation):
        with self.locked():
            value = self.record(operation)
            if self.status(operation)['phase'] == 'APPLIED':
                return self.status(operation)
            if self.live(operation):
                raise RuntimeError('Observe the live original worker before verification')
            if value.get('installReceipt') != {'commandReturnedSuccess': True}:
                raise RuntimeError('Actual successful original package command receipt required')
            value['postInstallVerification'] = {'previousObservedPhase': self.status(operation)['phase'],
                'originalSuccessfulCommandReceiptRequired': True, 'packageCommandsIssued': 0}
            self.save(value)
            self.finish(value)  # No dpkg, stop, start or replay.
        return self.status(operation)

    def finalize(self, operation):
        with self.locked():
            value = self.record(operation)
            if value['phase'] != 'APPLIED' or value.get('installReceipt') != {'commandReturnedSuccess': True}:
                raise RuntimeError('Verified original installation receipt required')
            self.verify_installed(value['package'])
            HostControl(self.setup).release_package(value['maintenanceOperationId'], operation)
        return self.status(operation)

    def worker(self, operation):
        with self.locked(wait=True):
            value = self.record(operation)
            if value['phase'] != 'ACCEPTED' or not value.get('launchIntent') or value.get('installIntent'):
                raise RuntimeError('Only the untouched accepted package worker may run')
            if self.package(value['package']['key']) != value['package'] or hashlib.sha256(
                    protected(self.setup.path(PROGRAM))).hexdigest() != value['controllerSha256']:
                raise RuntimeError('Accepted package/controller provenance changed')
            value['phase'] = 'RUNNING'
            self.save(value)
            control = HostControl(self.setup)
            maintenance_id = value['maintenanceOperationId']
            if control.status()['active'] is None:
                control.begin(maintenance_id, require_stopped=True)
            control.stop(maintenance_id, require_stopped=True)
            control.hold_package(maintenance_id, operation, value['package'])
            value['installIntent'] = {'packageSha256': value['package']['archiveSha256']}
            self.save(value)
            package = self.upload_path(operation, accepted=True)
            if not self.file_matches(package, value['package']):
                raise RuntimeError('Original sealed package differs')
            # Installer dependencies were resolved while installing the tools.
            # This fixed approved deb is the only dpkg target; no apt arguments,
            # arbitrary repositories, dependency removal or autostart are used.
            result = self.setup.run(['/usr/bin/dpkg', '--install', str(package)], env={**ENV,
                'DEBIAN_FRONTEND': 'noninteractive'}, stdin=subprocess.DEVNULL,
                stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, timeout=1200)
            if result.returncode:
                raise RuntimeError('Package command outcome requires original review')
            value['installReceipt'] = {'commandReturnedSuccess': True}
            self.save(value)
            self.finish(value)
        return self.status(operation)


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('action', choices=['worker'])
    parser.add_argument('--operation', required=True)
    args = parser.parse_args()
    try:
        if os.geteuid() != 0:
            raise RuntimeError('Host administrator required')
        from installer import verify_installer_payload
        verify_installer_payload()
        NativePackages().worker(args.operation)
    except Exception:
        print('Original native package operation requires review; no action will be repeated.', file=sys.stderr)
        sys.exit(1)
