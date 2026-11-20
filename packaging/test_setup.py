import json
import os
from pathlib import Path
from types import SimpleNamespace
import shutil
import tempfile
import unittest
from unittest.mock import patch

import build
import setup as console_setup


class ConsoleInstanceTests(unittest.TestCase):
    def test_instance_layout_is_explicit_and_path_safe(self):
        root = Path(tempfile.mkdtemp())
        self.addCleanup(lambda: shutil.rmtree(root))
        default = console_setup.Setup(root)
        instance = console_setup.Setup(root, instance='center2')
        self.assertEqual(default.config, '/etc/kura-management-console')
        self.assertEqual(instance.config, '/etc/kura-management-console-center2')
        self.assertEqual(instance.state, '/var/lib/kura-management-console-center2')
        self.assertEqual(instance.logdir, '/var/log/kura-management-console-center2')
        self.assertEqual(instance.unit, 'kura-management-console@center2.service')
        for value in ['', 'Center2', 'center_2', 'center/slash', 'a' * 33]:
            with self.assertRaisesRegex(RuntimeError, 'Invalid console instance'):
                console_setup.Setup(root, instance=value)

    @unittest.skipUnless(os.geteuid() == 0, 'root ownership is required by the packaged initializer')
    def test_instance_directories_are_private_and_marker_is_explicit(self):
        root = Path(tempfile.mkdtemp())
        self.addCleanup(lambda: shutil.rmtree(root))
        uid = os.getuid()
        gid = os.getgid()
        with patch.object(console_setup.pwd, 'getpwnam', return_value=SimpleNamespace(pw_uid=uid)), \
                patch.object(console_setup.grp, 'getgrnam', return_value=SimpleNamespace(gr_gid=gid)):
            def run(command, **kwargs):
                if command[0] == '/usr/bin/chown':
                    path = Path(command[2])
                    os.chown(path, uid, gid)
                return SimpleNamespace(returncode=0, stdout='', stderr='')
            setup = console_setup.Setup(root, run=run, instance='center2')
            setup.init_instance()
        self.assertTrue((root / 'etc/kura-management-console-center2/console.env').parent.is_dir())
        self.assertEqual(((root / 'etc/kura-management-console-center2').stat().st_mode & 0o777), 0o750)
        self.assertEqual(((root / 'var/lib/kura-management-console-center2').stat().st_mode & 0o777), 0o700)
        self.assertEqual(((root / 'var/log/kura-management-console-center2').stat().st_mode & 0o777), 0o700)
        marker = root / 'var/lib/kura-management-console-center2/.package-v1'
        self.assertEqual(marker.read_bytes(), b'kura-management-console-v1\n')
        self.assertEqual(marker.stat().st_mode & 0o777, 0o600)

    @unittest.skipUnless(os.geteuid() == 0, 'default runtime permissions require root fixtures')
    def test_default_and_instance_runtime_settings_use_their_own_layout(self):
        root = Path(tempfile.mkdtemp())
        self.addCleanup(lambda: shutil.rmtree(root))
        uid = os.getuid()
        gid = os.getgid()
        with patch.object(console_setup.pwd, 'getpwnam', return_value=SimpleNamespace(pw_uid=uid)), \
                patch.object(console_setup.grp, 'getgrnam', return_value=SimpleNamespace(gr_gid=gid)):
            for instance in [None, 'center2']:
                setup = console_setup.Setup(root, instance=instance)
                config = setup.path(setup.config)
                config.mkdir(parents=True, exist_ok=True)
                os.chown(config, 0, gid)
                config.chmod(0o750)
                env = setup.path(setup.config + '/console.env')
                env.write_text(
                    'DATABASE_URL=postgresql://fixture\n'
                    f'CONSOLE_DATA_KEY_FILE={setup.state}/task-data.key\n'
                    'CONSOLE_ORIGIN=https://fixture.invalid:8443\n'
                    'CONSOLE_HTTP_ADDR=127.0.0.1:18443\n'
                    'CONSOLE_GRPC_ADDR=127.0.0.1:18444\n'
                    f'CONSOLE_TLS_CERT={setup.state}/tls/server.pem\n'
                    f'CONSOLE_TLS_KEY={setup.state}/tls/server.key\n'
                    f'CONSOLE_CLIENT_CA={setup.state}/tls/ca.pem\n'
                    f'CONSOLE_WEB_DIR={setup.state}/react\n'
                )
                env.chmod(0o600)
                tls = setup.path(setup.state + '/tls/server.key')
                tls.parent.mkdir(parents=True, exist_ok=True)
                tls.write_bytes(b'tls-key\n')
                os.chown(tls, 0, gid)
                tls.chmod(0o640)
                key = setup.path(setup.state + '/task-data.key')
                key.write_bytes(b'a' * 32)
                os.chown(key, uid, gid)
                key.chmod(0o600)
                web = setup.path(setup.state + '/react')
                (web / 'assets').mkdir(parents=True, exist_ok=True)
                (web / 'index.html').write_text('<html></html>')
                (web / 'assets/app.js').write_text('console.log(1)')
                settings = setup.settings()
                self.assertEqual(settings['CONSOLE_WEB_DIR'], f'{setup.state}/react')

    def test_template_units_are_parsed_for_upgrade_guards(self):
        root = Path(tempfile.mkdtemp())
        self.addCleanup(lambda: shutil.rmtree(root))
        (root / 'run/systemd/system').mkdir(parents=True)
        wants = root / 'run/systemd/system/multi-user.target.wants'
        wants.mkdir()
        (wants / 'kura-management-console@link-only.service').symlink_to(
            '/usr/lib/systemd/system/kura-management-console@.service')
        def run(command, **kwargs):
            if command[1]=='list-unit-files':
                return SimpleNamespace(returncode=0, stdout=(
                    'kura-management-console@.service indirect enabled\n'
                    'kura-management-console@later.service enabled enabled\n'
                ))
            return SimpleNamespace(returncode=0, stdout=(
                'kura-management-console@center2.service loaded active running Console center2\n'
                'kura-management-console@center3.service loaded active running Console center3\n'
            ), stderr='')
        setup = console_setup.Setup(root, run=run)
        self.assertEqual(setup.template_units(), [
            'kura-management-console@center2.service',
            'kura-management-console@center3.service',
            'kura-management-console@later.service',
            'kura-management-console@link-only.service',
        ])

    def test_removal_rejects_enabled_override_without_disabling_it(self):
        commands = []
        def run(command, **kwargs):
            commands.append(command)
            return SimpleNamespace(stdout='UnitFileState=enabled\nFragmentPath=/etc/systemd/system/local.service\n')
        with tempfile.TemporaryDirectory() as root:
            base = Path(root)
            (base/'run/systemd/system').mkdir(parents=True)
            setup = console_setup.Setup(base, run, 'center2')
            with self.assertRaisesRegex(RuntimeError, 'does not use the installed package unit'):
                setup.removal_enablement()
        self.assertEqual([command[1] for command in commands], ['show'])

    def test_release_stages_instance_service_and_example(self):
        parent = Path(tempfile.mkdtemp())
        self.addCleanup(lambda: shutil.rmtree(parent))
        destination = parent / 'stage'
        manifest = build.stage(destination)
        paths = {record['path'] for record in manifest['sourceFiles']}
        self.assertIn('packaging/kura-management-console@.service', paths)
        self.assertIn('packaging/console-instance.env.example', paths)

    def test_clean_install_guard_needs_no_env_or_database(self):
        with tempfile.TemporaryDirectory() as directory:
            setup = console_setup.Setup(Path(directory), run=lambda *a, **k: self.fail('unexpected process'))
            setup.guard_all_instances()
            marker = setup.path(setup.state + '/.package-v1')
            marker.parent.mkdir(parents=True)
            marker.write_text('kura-management-console-v1\n')
            setup.guard_all_instances()
            (marker.parent / 'task-data.key').write_bytes(b'retained')
            with self.assertRaisesRegex(RuntimeError, 'recovery required'):
                setup.guard_all_instances()

    def test_unloaded_configured_instance_is_guarded(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / 'etc/kura-management-console-offline').mkdir(parents=True)
            setup = console_setup.Setup(root)
            visited = []
            with patch.object(console_setup.Setup, 'guard_tasks', lambda item: visited.append(item.unit)):
                setup.guard_all_instances()
            self.assertEqual(visited, [console_setup.UNIT, 'kura-management-console@offline.service'])

    def test_activation_targets_the_selected_instance(self):
        setup = console_setup.Setup(instance='center2')
        calls = []
        with patch.object(setup, 'launch_check'), patch.object(setup, 'systemctl', side_effect=lambda *args: calls.append(args)):
            setup.activate()
        self.assertEqual(calls, [('daemon-reload',), ('enable', setup.unit), ('start', setup.unit)])

    def test_database_credentials_are_removed_from_psql_arguments(self):
        uri, credentials = console_setup.psql_connection('postgresql://user:p%40ss@[::1]:5432/db?sslmode=require&password=query%2Bsecret')
        self.assertEqual(uri, 'postgresql://user@[::1]:5432/db?sslmode=require')
        self.assertEqual(credentials, {'PGPASSWORD': 'query+secret'})
        safe, credentials = console_setup.psql_connection('postgresql:///db?host=%2Fvar%2Frun%2Fpostgresql')
        self.assertEqual(credentials, {})
        self.assertIn('host=%2Fvar%2Frun%2Fpostgresql', safe)


if __name__ == '__main__':
    unittest.main()
