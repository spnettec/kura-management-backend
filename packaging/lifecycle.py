"""Embedded in Debian maintainer scripts."""
import grp
import pwd


def lifecycle(script, action):
    setup = Setup()
    if script == 'preinst' and action in ('install', 'upgrade'):
        setup.guard_tasks()
    elif script == 'postinst' and action == 'configure':
        try:
            user = pwd.getpwnam(SERVICE_USER)
        except KeyError:
            setup.run(['/usr/sbin/useradd', '--system', '--user-group', '--no-create-home',
                       '--home-dir', STATE, '--shell', '/usr/sbin/nologin', SERVICE_USER], check=True)
            user = pwd.getpwnam(SERVICE_USER)
        if user.pw_uid == 0:
            raise RuntimeError('Console service user must not be root')
        for name in [CONFIG, STATE, LOGDIR]:
            path = setup.path(name)
            path.mkdir(parents=True, exist_ok=True, mode=0o700)
            if path.stat().st_uid != user.pw_uid or path.stat().st_mode & 0o077:
                raise RuntimeError('Console retained directories must be service-user owned and private')
        marker = setup.path(STATE + '/.package-v1')
        if not marker.exists():
            setup.put(marker, 'kura-management-console-v1\n', 0o600)
            setup.run(['chown', f'{SERVICE_USER}:{SERVICE_USER}', str(marker)], check=True)
        setup.systemctl('daemon-reload')
        print('Installed without starting. Configure PostgreSQL, TLS and task key, migrate, then explicitly activate.')
    elif script == 'prerm' and action in ('remove', 'upgrade', 'deconfigure'):
        setup.guard_tasks()
        setup.inactive()
        setup.systemctl('stop', UNIT)
        if action == 'remove':
            setup.systemctl('disable', UNIT)
    elif script == 'postrm':
        if action in ('remove', 'purge'):
            setup.systemctl('daemon-reload')
        if action == 'purge':
            print('Retaining PostgreSQL data and task data key; remove them only after verified recovery.')


if __name__ == '__main__':
    lifecycle(SCRIPT_NAME, sys.argv[1] if len(sys.argv) > 1 else '')
