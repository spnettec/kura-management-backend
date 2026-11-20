#!/usr/bin/python3
"""Root-only checks and key lifecycle wrappers for the packaged management console."""
import argparse
import grp
import fcntl
import hashlib
import json
import os
from pathlib import Path
import pwd
import re
import shutil
import stat
import subprocess
import sys
from contextlib import nullcontext
from urllib.parse import parse_qsl, unquote, urlencode, urlsplit, urlunsplit
from host_control import HostControl

CONFIG = '/etc/kura-management-console'
STATE = '/var/lib/kura-management-console'
LOGDIR = '/var/log/kura-management-console'
UNIT = 'kura-management-console.service'
TEMPLATE = 'kura-management-console@.service'
BINARY = '/usr/bin/yofc-management-console'
SERVICE_USER = 'yofc-console'
RUNUSER = '/usr/sbin/runuser'


def valid_instance(instance):
    return bool(re.fullmatch(r'[a-z0-9][a-z0-9-]{0,31}', instance or ''))


def psql_connection(uri):
    """Keep database credentials out of process argv during maintenance."""
    parsed = urlsplit(uri)
    if parsed.scheme not in ('postgres', 'postgresql'):
        raise RuntimeError('Maintenance requires a PostgreSQL connection URI')
    userinfo, separator, address = parsed.netloc.rpartition('@')
    password = None
    authority = parsed.netloc
    if separator:
        username, colon, value = userinfo.partition(':')
        if colon:
            password = unquote(value)
        authority = username + '@' + address
    query = []
    for key, value in parse_qsl(parsed.query, keep_blank_values=True):
        if key == 'password':
            password = value
        else:
            query.append((key, value))
    safe_uri = urlunsplit((parsed.scheme, authority, parsed.path, urlencode(query), parsed.fragment))
    return safe_uri, {} if password is None else {'PGPASSWORD': password}


class Setup:
    def __init__(self, root=Path('/'), run=subprocess.run, instance=None):
        self.root, self.run = root, run
        if instance is None:
            self.config, self.state, self.logdir = CONFIG, STATE, LOGDIR
            self.unit = UNIT
        else:
            if not valid_instance(instance):
                raise RuntimeError('Invalid console instance name')
            suffix = '-' + instance
            self.config = CONFIG + suffix
            self.state = STATE + suffix
            self.logdir = LOGDIR + suffix
            self.unit = 'kura-management-console@' + instance + '.service'
            self.instance = instance

    def path(self, name):
        path = self.root / name.lstrip('/')
        for entry in [path, *path.parents]:
            if entry.is_symlink():
                raise RuntimeError('Refusing symlink in console-owned path')
            if entry == self.root:
                break
        return path

    def overrides(self, missing_ok=False):
        path = self.path(self.config + '/console.env')
        if missing_ok and not path.exists():
            return {}
        info = path.lstat()
        if not stat.S_ISREG(info.st_mode) or info.st_uid != 0 or info.st_mode & 0o077:
            raise RuntimeError('Console env must be a root-owned regular file with mode 0600')
        settings = {}
        for line in path.read_text().splitlines():
            if not line or line.startswith('#'):
                continue
            key, separator, value = line.partition('=')
            if not separator or not re.fullmatch(r'[A-Z_]+', key) or key in settings or '\x00' in value:
                raise RuntimeError('Invalid or duplicate console setting')
            settings[key] = value
        if settings.get('CONSOLE_STATE_DIR','').strip() and settings['CONSOLE_STATE_DIR']!=self.state:
            raise RuntimeError('Packaged state directory must match the selected instance')
        return settings

    def settings(self, runtime=True):
        settings = self.overrides()
        required = ['DATABASE_URL', 'CONSOLE_DATA_KEY_FILE', 'CONSOLE_ORIGIN', 'CONSOLE_HTTP_ADDR',
                    'CONSOLE_GRPC_ADDR', 'CONSOLE_TLS_CERT', 'CONSOLE_TLS_KEY', 'CONSOLE_CLIENT_CA',
                    'CONSOLE_WEB_DIR']
        missing = [key for key in required if not settings.get(key)]
        if missing:
            raise RuntimeError('Missing console settings: ' + ', '.join(missing))
        if settings['CONSOLE_ORIGIN'] != 'https://' + settings['CONSOLE_ORIGIN'].split('://', 1)[1].rstrip('/'):
            raise RuntimeError('CONSOLE_ORIGIN must be an HTTPS origin without trailing slash')
        if not runtime:
            return settings
        service_gid = grp.getgrnam(SERVICE_USER).gr_gid
        config_info = self.path(self.config).stat()
        if config_info.st_uid != 0 or config_info.st_gid != service_gid or config_info.st_mode & 0o027:
            raise RuntimeError('Console configuration directory must allow only root and yofc-console traversal')
        for key in required:
            if key not in {'DATABASE_URL', 'CONSOLE_ORIGIN', 'CONSOLE_HTTP_ADDR', 'CONSOLE_GRPC_ADDR'}:
                target = self.path(settings[key]) if settings[key].startswith('/') else Path(settings[key])
                if target.is_symlink():
                    raise RuntimeError(f'{key} must not be a symlink')
        if not self.path(settings['CONSOLE_WEB_DIR'] + '/index.html').is_file():
            raise RuntimeError('Console web payload is missing index.html')
        tls_key = self.path(settings['CONSOLE_TLS_KEY'])
        tls_info = tls_key.lstat()
        if not stat.S_ISREG(tls_info.st_mode) or tls_info.st_uid != 0 or tls_info.st_gid != service_gid \
                or tls_info.st_mode & 0o040 == 0 or tls_info.st_mode & 0o007:
            raise RuntimeError('TLS private key must be root:yofc-console mode 0640 or safer')
        key = self.path(settings['CONSOLE_DATA_KEY_FILE'])
        info = key.lstat()
        if not stat.S_ISREG(info.st_mode) or info.st_uid != pwd.getpwnam(SERVICE_USER).pw_uid \
                or info.st_mode & 0o777 != 0o600:
            raise RuntimeError('Task data key must be owned by yofc-console with mode 0600')
        return settings

    def put(self, name, content, mode):
        target = name if isinstance(name, Path) else self.path(name)
        target.parent.mkdir(parents=True, exist_ok=True)
        descriptor = os.open(target, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, mode)
        with os.fdopen(descriptor, 'w') as stream:
            stream.write(content)
        target.chmod(mode)

    def inactive(self):
        result = self.systemctl('show', self.unit, '-p', 'MainPID', '-p', 'ActiveState')
        if result:
            values = dict(line.split('=', 1) for line in result.stdout.splitlines() if '=' in line)
            if values.get('MainPID') != '0' or values.get('ActiveState') not in ('inactive', 'failed'):
                raise RuntimeError('Stop the packaged console before package maintenance')

    def systemctl(self, *args):
        if self.path('/run/systemd/system').is_dir():
            return self.run(['/bin/systemctl', *args], check=True, capture_output=True, text=True)
        return None

    def service_env(self):
        settings = self.settings()
        return {**os.environ, **settings}

    @staticmethod
    def instance_from_unit(unit):
        if unit.startswith('kura-management-console@') and unit.endswith('.service'):
            name = unit.removeprefix('kura-management-console@').removesuffix('.service')
            if valid_instance(name):
                return name
        return None

    def template_units(self):
        pattern = 'kura-management-console@*.service'
        loaded = self.systemctl('list-units', pattern, '--all', '--no-legend', '--plain')
        installed = self.systemctl('list-unit-files', pattern, '--no-legend', '--no-pager')
        units = set()
        for result in (loaded, installed):
            for line in result.stdout.splitlines() if result is not None else []:
                fields = line.split()
                if fields and self.instance_from_unit(fields[0]) is not None:
                    units.add(fields[0])
        # On systemd, an enabled instance may exist only as a target link.
        # The unit-file listing can report just its template as indirect.
        for scope in ('/etc', '/run'):
            directory = self.path(scope + '/systemd/system/multi-user.target.wants')
            for link in directory.glob(pattern):
                if link.is_symlink() and self.instance_from_unit(link.name) is not None:
                    units.add(link.name)
        # An inactive, unloaded instance can still own encrypted tasks. Its
        # retained directories are authoritative even when systemd lists none.
        for base in [CONFIG, STATE]:
            parent = self.path(str(Path(base).parent))
            if parent.exists():
                for path in parent.glob(Path(base).name + '-*'):
                    instance = path.name[len(Path(base).name) + 1:]
                    if valid_instance(instance):
                        self.path(str(path.relative_to(self.root)))
                        units.add('kura-management-console@' + instance + '.service')
        return sorted(units)

    def removal_enablement(self):
        result = self.systemctl('show', self.unit, '-p', 'UnitFileState', '-p', 'FragmentPath')
        if result is None:
            raise RuntimeError('A running systemd host is required for package removal')
        fields = dict(line.split('=', 1) for line in result.stdout.splitlines() if '=' in line)
        state = fields['UnitFileState']
        if state in ('enabled', 'enabled-runtime'):
            vendor = '/usr/lib/systemd/system/' + (UNIT if self.unit==UNIT else TEMPLATE)
            if fields.get('FragmentPath') != vendor:
                raise RuntimeError('Enabled console unit does not use the installed package unit')
        return state

    def guard_all_instances(self):
        self.guard_tasks()
        for unit in self.template_units():
            instance = unit.removeprefix('kura-management-console@').removesuffix('.service')
            Setup(self.root, self.run, instance).guard_tasks()

    def stop_all_instances(self):
        self.systemctl('stop', self.unit)
        for unit in self.template_units():
            self.systemctl('stop', unit)

    def guard_tasks(self):
        state = self.path(self.state)
        if any((state / name).exists() for name in ['console-config-active.json', 'setup', 'setup-operations']):
            self.bound_command('installation-maintenance-check', include_overrides=True)
            return
        if not self.path(self.config + '/console.env').exists():
            # A clean install and a never-configured package marker have no DB.
            # Retained credentials or data without their env cannot be checked.
            for base in [self.config, self.state]:
                directory = self.path(base)
                if directory.exists() and (not directory.is_dir() or any(
                        entry.name != '.package-v1' for entry in directory.iterdir())):
                    raise RuntimeError('Retained console data has no console.env; explicit recovery required')
            return
        settings = self.settings(runtime=False)
        database, credentials = psql_connection(settings['DATABASE_URL'])
        psql_env = {**os.environ, **credentials, 'PGOPTIONS': '-c statement_timeout=5000', 'PGCONNECT_TIMEOUT': '5'}
        exists = self.run(['/usr/bin/psql', database, '-v', 'ON_ERROR_STOP=1', '-Atc',
                           "SELECT to_regclass('mgmt_tasks') IS NOT NULL"],
                          env=psql_env, capture_output=True, text=True, timeout=12)
        if exists.returncode:
            raise RuntimeError('Unable to verify console task table; retain database access')
        unsettled = 0
        if exists.stdout.strip() == 't':
            result = self.run(['/usr/bin/psql', database, '-v', 'ON_ERROR_STOP=1', '-Atc',
                               "SELECT count(*) FROM mgmt_tasks WHERE state IN ('QUEUED','RUNNING','UNKNOWN')"],
                              env=psql_env, capture_output=True, text=True, timeout=12)
            if result.returncode:
                raise RuntimeError('Unable to verify unsettled console tasks; retain database access')
            unsettled = int(result.stdout.strip() or '0')
        if unsettled:
            raise RuntimeError('Settle queued, running or unknown console tasks before package maintenance')

    def init_instance(self, quiet=False):
        """Create only instance directories and marker; configuration remains explicit."""
        service_user = pwd.getpwnam(SERVICE_USER)
        service_gid = grp.getgrnam(SERVICE_USER).gr_gid
        config = self.path(self.config)
        created = not config.exists()
        config.mkdir(parents=True, exist_ok=True, mode=0o750)
        if created:
            self.run(['/usr/bin/chown', f'root:{SERVICE_USER}', str(config)], check=True)
            config.chmod(0o750)
        config_info = config.stat()
        if config_info.st_uid != 0 or config_info.st_gid != service_gid or config_info.st_mode & 0o007:
            raise RuntimeError('Instance configuration directory must be root:yofc-console mode 0750')
        for path_name in [self.state, self.logdir]:
            path = self.path(path_name)
            path.mkdir(parents=True, exist_ok=True, mode=0o700)
            info = path.stat()
            if info.st_uid != service_user.pw_uid:
                if info.st_uid != 0:
                    raise RuntimeError('Instance state directory has an unexpected owner')
                self.run(['/usr/bin/chown', f'{SERVICE_USER}:{SERVICE_USER}', str(path)], check=True)
            if path.stat().st_mode & 0o077:
                raise RuntimeError('Instance state directories must be private')
        marker = self.path(self.state + '/.package-v1')
        if not marker.exists():
            self.put(marker, 'kura-management-console-v1\n', 0o600)
            self.run(['/usr/bin/chown', f'{SERVICE_USER}:{SERVICE_USER}', str(marker)], check=True)
        self.systemctl('daemon-reload')
        if not quiet:
            print('Instance directories created without config, migrate, or activation.')

    def check(self):
        self.settings()
        binary = self.path(BINARY)
        if not binary.is_file() or not os.access(binary, os.X_OK):
            raise RuntimeError('Console binary is missing')
        web = self.path(self.settings()['CONSOLE_WEB_DIR'])
        assets = list((web / 'assets').glob('*.js')) if (web / 'assets').is_dir() else []
        if not assets:
            raise RuntimeError('Console web payload has no JavaScript assets')
        return {'ok': True, 'assets': len(assets)}

    def launch_check(self):
        """Check installed payload and private state without requiring a database."""
        control=HostControl(self)
        control.provision()
        if control.status()['active'] is not None:
            raise RuntimeError('Maintenance is frozen; preserve the original operation')
        control.require_configured_package()
        overrides = self.overrides(missing_ok=True)
        state_override = overrides.get('CONSOLE_STATE_DIR', '')
        if state_override.strip() and state_override != self.state:
            raise RuntimeError('Packaged state directory must match the selected instance')
        binary = self.path(BINARY)
        if not binary.is_file() or not os.access(binary, os.X_OK):
            raise RuntimeError('Console binary is missing')
        state = self.path(self.state)
        info = state.lstat()
        if not stat.S_ISDIR(info.st_mode) or info.st_uid != pwd.getpwnam(SERVICE_USER).pw_uid or info.st_mode & 0o077:
            raise RuntimeError('Console state must be owned by its service identity with mode 0700')
        web_override = overrides.get('CONSOLE_WEB_DIR', '')
        web = self.path(web_override if web_override.strip() else '/usr/share/kura-management-console/react')
        assets = list((web/'assets').glob('*.js'))
        if not (web/'index.html').is_file() or not assets:
            raise RuntimeError('Console web payload is incomplete')
        return {'ok': True, 'assets': len(assets)}

    def initial_listener(self, port, allow_unpaired=False):
        if not isinstance(port, int) or not 1024 <= port <= 65535:
            raise RuntimeError('Setup port must be between 1024 and 65535')
        overrides = self.overrides(missing_ok=True)
        address = overrides.get('CONSOLE_HTTP_ADDR', '').strip()
        current_port = address.rsplit(':', 1)[-1] if address else overrides.get('CONSOLE_SETUP_PORT','18443')
        if current_port == str(port):
            return
        if self.path(self.config+'/console.env').exists():
            raise RuntimeError('Existing launch settings cannot be replaced by first setup')
        state = self.path(self.state)
        if state.exists() and any(entry.name != '.package-v1' for entry in state.iterdir()):
            if not allow_unpaired:
                raise RuntimeError('Existing installation identity requires its original listener')
            # A known stopped, never-paired bootstrap can recover from a port
            # collision without replacing its certificate or installation ID.
            # Any session, operation, generation, unknown file or symlink keeps
            # the original listener binding and requires explicit review.
            allowed_files = {'.package-v1', 'tls-runtime.lock', 'setup/bootstrap.key',
                             'setup/bootstrap-request.cnf', 'setup/bootstrap.pem',
                             'setup/installation-credential.json'}
            allowed_dirs = {'setup', 'setup-operations', 'installation-generations'}
            entries = list(state.rglob('*'))
            files = {str(entry.relative_to(state)) for entry in entries if entry.is_file()}
            if files != allowed_files or any(entry.is_symlink() or
                    (entry.is_dir() and str(entry.relative_to(state)) not in allowed_dirs)
                    or not (entry.is_file() or entry.is_dir()) for entry in entries):
                raise RuntimeError('Existing installation identity requires its original listener')
            self.bound_command('installation-credential')  # Validate the existing identity; never generate one.
        config = self.path(self.config)
        config.mkdir(parents=True, exist_ok=True, mode=0o700)
        info = config.lstat()
        if info.st_uid != 0 or info.st_mode & 0o027:
            raise RuntimeError('Unsafe host configuration directory')
        self.put(self.config+'/console.env', 'CONSOLE_SETUP_PORT='+str(port)+'\n', 0o600)

    def bound_command(self, command, include_overrides=False):
        if command not in ['installation-credential', 'installation-maintenance-check', 'installation-status']:
            raise RuntimeError('Unsupported installation helper action')
        self.verify_local_command(command)
        environment = {'PATH': '/usr/sbin:/usr/bin:/sbin:/bin'}
        if include_overrides:
            environment.update(self.overrides(missing_ok=True))
        environment['CONSOLE_STATE_DIR'] = self.state
        result = self.run([RUNUSER, '-u', SERVICE_USER, '--', BINARY, command], env=environment,
                            capture_output=True, text=True, timeout=5 if command=='installation-status' else 15)
        if result.returncode:
            raise RuntimeError('Installation operation cannot be verified; preserve state and use the original record')
        return json.loads(result.stdout)

    def active_process_digest(self, pid):
        if not re.fullmatch(r'[1-9][0-9]*',pid):
            raise RuntimeError('Invalid service process identity')
        # This is a fixed kernel-managed magic link, not a console-owned path.
        # Keep Setup.path's symlink refusal for all configuration/state files.
        executable = self.path('/proc') / pid / 'exe'
        try:
            return hashlib.sha256(executable.read_bytes()).hexdigest()
        except PermissionError:
            # A host administrator without CAP_SYS_PTRACE may still read the
            # executable with the service's own credentials. Drop all privileges
            # in the child and hash the kernel path with a fixed program.
            result = self.run([
                '/usr/bin/setpriv', '--reuid=' + SERVICE_USER, '--regid=' + SERVICE_USER,
                '--clear-groups', '--bounding-set=-all', '--inh-caps=-all', '--ambient-caps=-all',
                '--no-new-privs', '/usr/bin/sha256sum', '--', str(executable),
            ], env={'PATH': '/usr/bin'}, stdin=subprocess.DEVNULL,
                capture_output=True, text=True, timeout=5)
            output = result.stdout.removesuffix('\n')
            digest = output.removesuffix('  ' + str(executable))
            if result.returncode or digest == output or not re.fullmatch(r'[a-f0-9]{64}', digest):
                raise RuntimeError('Active service executable could not be verified')
            return digest

    def debian_package_state(self):
        result = self.run(['/usr/bin/dpkg-query', '-W', '-f=${Version}\t${db:Status-Status}',
                           'kura-management-console'], capture_output=True, text=True, timeout=5)
        version, separator, status = result.stdout.partition('\t')
        if result.returncode or not separator:
            return '', 'not-installed'
        return version, status

    def verify_local_command(self, command):
        """Do not probe an old executable with an unknown argument: old startup
        dispatch may migrate before it reports an unsupported command."""
        base='/usr/share/kura-management-console/'
        provenance=self.path(base+'local-command-provenance.json')
        source=self.path(base+'source-manifest.json')
        binary=self.path(BINARY)
        for path in [provenance,source,binary]:
            if not path.is_file():
                raise RuntimeError('Verified local command provenance is required before invoking the console')
            info=path.lstat()
            if info.st_uid!=0 or info.st_mode & 0o022:
                raise RuntimeError('Local command provenance and executable must be protected by root')
        record=json.loads(provenance.read_text())
        if record.get('schemaVersion')!=1 or command not in record.get('localReadCommands',[]) \
                or record.get('binarySha256')!=hashlib.sha256(binary.read_bytes()).hexdigest() \
                or record.get('sourceManifestSha256')!=hashlib.sha256(source.read_bytes()).hexdigest():
            raise RuntimeError('Console does not have verified support for this local command')
        return record

    def key_command(self, command, arguments):
        self.inactive()
        settings = self.settings(runtime=command != 'init-key')
        env = {**os.environ, **settings, 'DATABASE_URL': settings['DATABASE_URL'],
               'CONSOLE_DATA_KEY_FILE': settings['CONSOLE_DATA_KEY_FILE']}
        binary_command = {
            'init-key': 'task-key-init',
            'rotate-key': 'task-key-rotate',
            'backup-key': 'task-key-backup',
            'restore-key': 'task-key-restore',
        }.get(command, command)
        result = self.run([RUNUSER, '-u', SERVICE_USER, '--', BINARY, binary_command, *map(str, arguments)],
                          env=env, capture_output=True, text=True)
        if result.returncode:
            raise RuntimeError(result.stderr.strip() or result.stdout.strip() or 'key command failed')
        return result.stdout

    def migrate(self):
        self.inactive()
        output = self.key_command('migrate', [])
        print(output or 'Database migrations verified')

    def activate(self):
        self.launch_check()
        self.systemctl('daemon-reload')
        self.systemctl('enable', self.unit)
        self.systemctl('start', self.unit)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--instance')
    sub = parser.add_subparsers(dest='command', required=True)
    sub.add_parser('check')
    sub.add_parser('launch-check')
    sub.add_parser('installation-credential')
    sub.add_parser('migrate')
    sub.add_parser('init-instance')
    for command, arity in [('init-key', 1), ('rotate-key', 2), ('backup-key', 3), ('restore-key', 3)]:
        action = sub.add_parser(command)
        action.add_argument('arguments', nargs=arity)
    sub.add_parser('activate')
    args = parser.parse_args()
    if os.geteuid() != 0:
        raise RuntimeError('Run console setup as root')
    setup = Setup(instance=args.instance)
    if args.command == 'activate':
        with HostControl(setup).starting(): setup.activate()
        return
    if args.command == 'installation-credential':
        print(json.dumps(setup.bound_command('installation-credential')))
        return
    lock = setup.path('/run/lock/kura-management-console-setup.lock')
    admission=HostControl(setup).starting() if args.command not in ['check','launch-check'] else nullcontext()
    with admission, os.fdopen(os.open(lock, os.O_RDWR | os.O_CREAT | os.O_NOFOLLOW, 0o600), 'r+') as stream:
        fcntl.flock(stream, fcntl.LOCK_EX)
        if args.command == 'init-instance':
            if not args.instance:
                raise RuntimeError('init-instance requires --instance INSTANCE')
            setup.init_instance()
        elif args.command in ['init-key', 'rotate-key', 'backup-key', 'restore-key']:
            arity = {'init-key': 1, 'rotate-key': 2, 'backup-key': 3, 'restore-key': 3}
            if len(args.arguments) != arity[args.command]:
                raise RuntimeError(f'Invalid arguments for {args.command}')
            print(setup.key_command(args.command, args.arguments), end='')
        elif args.command == 'check':
            print(json.dumps(setup.check(), indent=2))
        elif args.command == 'launch-check':
            print(json.dumps(setup.launch_check(), indent=2))
        elif args.command == 'migrate':
            setup.migrate()


if __name__ == '__main__':
    main()
