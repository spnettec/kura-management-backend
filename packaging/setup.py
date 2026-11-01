#!/usr/bin/python3
"""Root-only checks and key lifecycle wrappers for the packaged management console."""
import argparse
import fcntl
import json
import os
from pathlib import Path
import pwd
import re
import shutil
import stat
import subprocess
import sys
from urllib.parse import quote

CONFIG = '/etc/kura-management-console'
STATE = '/var/lib/kura-management-console'
LOGDIR = '/var/log/kura-management-console'
UNIT = 'kura-management-console.service'
BINARY = '/usr/bin/yofc-management-console'
SERVICE_USER = 'yofc-console'


class Setup:
    def __init__(self, root=Path('/'), run=subprocess.run):
        self.root, self.run = root, run

    def path(self, name):
        path = self.root / name.lstrip('/')
        for entry in [path, *path.parents]:
            if entry.is_symlink():
                raise RuntimeError('Refusing symlink in console-owned path')
            if entry == self.root:
                break
        return path

    def settings(self):
        path = self.path(CONFIG + '/console.env')
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
        required = ['DATABASE_URL', 'CONSOLE_DATA_KEY_FILE', 'CONSOLE_ORIGIN', 'CONSOLE_HTTP_ADDR',
                    'CONSOLE_GRPC_ADDR', 'CONSOLE_TLS_CERT', 'CONSOLE_TLS_KEY', 'CONSOLE_CLIENT_CA',
                    'CONSOLE_WEB_DIR']
        missing = [key for key in required if not settings.get(key)]
        if missing:
            raise RuntimeError('Missing console settings: ' + ', '.join(missing))
        for key in required:
            if key not in {'DATABASE_URL', 'CONSOLE_ORIGIN', 'CONSOLE_HTTP_ADDR', 'CONSOLE_GRPC_ADDR'}:
                target = self.path(settings[key]) if settings[key].startswith('/') else Path(settings[key])
                if target.is_symlink():
                    raise RuntimeError(f'{key} must not be a symlink')
        if settings['CONSOLE_ORIGIN'] != 'https://' + settings['CONSOLE_ORIGIN'].split('://', 1)[1].rstrip('/'):
            raise RuntimeError('CONSOLE_ORIGIN must be an HTTPS origin without trailing slash')
        if not self.path(settings['CONSOLE_WEB_DIR'] + '/index.html').is_file():
            raise RuntimeError('Console web payload is missing index.html')
        key = self.path(settings['CONSOLE_DATA_KEY_FILE'])
        info = key.lstat()
        if not stat.S_ISREG(info.st_mode) or info.st_uid != pwd.getpwnam(SERVICE_USER).pw_uid \
                or info.st_mode & 0o777 != 0o600:
            raise RuntimeError('Task data key must be owned by yofc-console with mode 0600')
        return settings

    def inactive(self):
        result = self.systemctl('show', UNIT, '-p', 'MainPID', '-p', 'ActiveState')
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

    def guard_tasks(self):
        settings = self.settings()
        database = settings['DATABASE_URL']
        url = database.split('?', 1)[0]
        options = database.split('?', 1)[1] if '?' in database else ''
        if '=' in options:
            pairs = []
            for item in options.split('&'):
                key, _, value = item.partition('=')
                pairs.append('--set' if key == 'search_path' else '')
                pairs.append(value)
            query = ['--set', quote(url)] + pairs
        query = ['-h', self.path('/var/run/postgresql'), '-d', url.rsplit('/', 1)[-1], '-Atc',
                 "SELECT count(*) FROM mgmt_tasks WHERE state IN ('QUEUED','RUNNING','UNKNOWN')"]
        result = self.run(['/usr/bin/psql', *query], env={**os.environ, 'PGOPTIONS': '-c statement_timeout=5000'},
                          capture_output=True, text=True)
        if result.returncode:
            raise RuntimeError('Unable to verify unsettled console tasks; retain database access')
        if int(result.stdout.strip() or '0'):
            raise RuntimeError('Settle queued, running or unknown console tasks before package maintenance')

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

    def key_command(self, command, arguments):
        self.inactive()
        settings = self.settings()
        env = {**os.environ, **settings, 'DATABASE_URL': settings['DATABASE_URL'],
               'CONSOLE_DATA_KEY_FILE': settings['CONSOLE_DATA_KEY_FILE']}
        result = self.run([BINARY, command, *map(str, arguments)], env=env, capture_output=True, text=True)
        if result.returncode:
            raise RuntimeError(result.stderr.strip() or result.stdout.strip() or 'key command failed')
        return result.stdout

    def migrate(self):
        self.inactive()
        output = self.key_command('migrate', [])
        print(output or 'Database migrations verified')

    def activate(self):
        self.check()
        self.systemctl('daemon-reload')
        self.systemctl('enable', UNIT)
        self.systemctl('start', UNIT)


def main():
    parser = argparse.ArgumentParser()
    sub = parser.add_subparsers(dest='command', required=True)
    sub.add_parser('check')
    sub.add_parser('migrate')
    for command in ['init-key', 'rotate-key', 'backup-key', 'restore-key']:
        action = sub.add_parser(command)
        action.add_argument('arguments', nargs='+')
    sub.add_parser('activate')
    args = parser.parse_args()
    if os.geteuid() != 0:
        raise RuntimeError('Run console setup as root')
    setup = Setup()
    lock = setup.path('/run/lock/kura-management-console-setup.lock')
    with os.fdopen(os.open(lock, os.O_RDWR | os.O_CREAT | os.O_NOFOLLOW, 0o600), 'r+') as stream:
        fcntl.flock(stream, fcntl.LOCK_EX)
        if args.command in ['init-key', 'rotate-key', 'backup-key', 'restore-key']:
            if len(args.arguments) != (3 if args.command == 'rotate-key' else 2):
                raise RuntimeError(f'Invalid arguments for {args.command}')
            print(setup.key_command(args.command, args.arguments), end='')
        elif args.command == 'check':
            print(json.dumps(setup.check(), indent=2))
        elif args.command == 'migrate':
            setup.migrate()
        elif args.command == 'activate':
            setup.activate()


if __name__ == '__main__':
    main()
