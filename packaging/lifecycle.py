"""Embedded in Debian maintainer scripts."""
import grp
import pwd
import uuid
if 'HostControl' not in globals():
    from host_control import HostControl


def configure_context(setup, previous_version):
    maintainer = setup.path('/var/lib/dpkg/info/kura-management-console.postinst')
    if os.environ.get('DPKG_MAINTSCRIPT_PACKAGE')!='kura-management-console' \
            or os.environ.get('DPKG_MAINTSCRIPT_NAME')!='postinst' or Path(__file__)!=maintainer \
            or setup.debian_package_state()!=(PACKAGE_VERSION, 'half-configured'):
        raise RuntimeError('Exact installed Debian postinst and half-configured package required')
    root = setup.root.stat()
    boot = setup.path('/proc/sys/kernel/random/boot_id').read_text().strip()
    uuid.UUID(boot)
    with maintainer.open('rb') as stream:
        info = os.fstat(stream.fileno())
        if not stat.S_ISREG(info.st_mode) or info.st_uid!=0 or info.st_nlink!=1 or info.st_mode&0o022:
            raise RuntimeError('Protected installed Debian postinst required')
        digest = hashlib.file_digest(stream, 'sha256').hexdigest()
    return {'version':PACKAGE_VERSION, 'previousVersion':previous_version,
            'bootId':boot, 'rootDevice':root.st_dev, 'rootInode':root.st_ino,
            'scriptInode':info.st_ino, 'scriptSha256':digest}


def lifecycle(script, action, previous_version=''):
    setup = Setup()
    control = HostControl(setup)
    if script == 'preinst' and action in ('install', 'upgrade'):
        active=control.status()['active']
        identity=active['operationId'] if active else str(uuid.uuid4())
        control.begin(identity)
        control.stop(identity)
    elif script == 'postinst' and action == 'configure':
        context = configure_context(setup, previous_version)
        active = control.active()
        if active is None:
            control.complete(None, PACKAGE_BINARY_SHA256, PACKAGE_SOURCE_SHA256, debian=context)
            return
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
        control.complete(active['operationId'], PACKAGE_BINARY_SHA256, PACKAGE_SOURCE_SHA256, debian=context)
        print('Installed without starting. Open the host installer page to start the HTTPS setup wizard.')
    elif script == 'prerm' and action in ('remove', 'upgrade', 'deconfigure'):
        active=control.status()['active']
        identity=active['operationId'] if active else str(uuid.uuid4())
        control.begin(identity)
        control.stop(identity)
        if action == 'remove':
            control.disable_for_removal(identity)
    elif script == 'postrm':
        if action in ('remove', 'purge'):
            setup.systemctl('daemon-reload')
        if action == 'purge':
            print('Retaining PostgreSQL data and task data key; remove them only after verified recovery.')


if __name__ == '__main__':
    lifecycle(SCRIPT_NAME, sys.argv[1] if len(sys.argv) > 1 else '', sys.argv[2] if len(sys.argv) > 2 else '')
