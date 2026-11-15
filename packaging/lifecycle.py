"""Embedded in Debian maintainer scripts."""
import grp
import pwd


def lifecycle(script, action):
    setup = Setup()
    if script == 'preinst' and action in ('install', 'upgrade'):
        setup.guard_all_instances()
    elif script == 'postinst' and action == 'configure':
        try:
            user = pwd.getpwnam(SERVICE_USER)
        except KeyError:
            setup.run(['/usr/sbin/useradd', '--system', '--user-group', '--no-create-home',
                       '--home-dir', STATE, '--shell', '/usr/sbin/nologin', SERVICE_USER], check=True)
            user = pwd.getpwnam(SERVICE_USER)
        if user.pw_uid == 0:
            raise RuntimeError('Console service user must not be root')
        config = setup.path(CONFIG)
        config.mkdir(parents=True, exist_ok=True, mode=0o700)
        service_gid = grp.getgrnam(SERVICE_USER).gr_gid
        info = config.stat()
        group_private = info.st_mode & 0o070 == 0 or (info.st_gid == service_gid and info.st_mode & 0o020 == 0)
        if info.st_uid != 0 or info.st_mode & 0o007 or not group_private:
            raise RuntimeError('Console configuration directory must be root-owned and inaccessible to other identities')
        for name in [STATE, LOGDIR]:
            path = setup.path(name)
            path.mkdir(parents=True, exist_ok=True, mode=0o700)
            info = path.stat()
            if info.st_uid != user.pw_uid:
                if info.st_uid != 0:
                    raise RuntimeError('Console state directory has an unexpected owner')
                setup.run(['/usr/bin/chown', f'{SERVICE_USER}:{SERVICE_USER}', str(path)], check=True)
            if path.stat().st_mode & 0o077:
                raise RuntimeError('Console state directories must be private')
        marker = setup.path(STATE + '/.package-v1')
        if not marker.exists():
            setup.put(marker, 'kura-management-console-v1\n', 0o600)
            setup.run(['chown', f'{SERVICE_USER}:{SERVICE_USER}', str(marker)], check=True)
        setup.systemctl('daemon-reload')
        print('Installed without starting. Configure TLS and task key, migrate, then explicitly activate.')
    elif script == 'prerm' and action in ('remove', 'upgrade', 'deconfigure'):
        setup.stop_all_instances()
        setup.guard_all_instances()
        setup.inactive()
        for unit in setup.template_units():
            setup.systemctl('stop', unit)
            instance = unit.removeprefix('kura-management-console@').removesuffix('.service')
            Setup(setup.root, setup.run, instance).inactive()
        if action == 'remove':
            setup.systemctl('disable', UNIT)
    elif script == 'postrm':
        if action in ('remove', 'purge'):
            setup.systemctl('daemon-reload')
        if action == 'purge':
            print('Retaining PostgreSQL data and task data key; remove them only after verified recovery.')


if __name__ == '__main__':
    lifecycle(SCRIPT_NAME, sys.argv[1] if len(sys.argv) > 1 else '')
