#!/usr/bin/env python3
# Copyright (c) 2026 YOFC IOT and others
# SPDX-License-Identifier: EPL-2.0
"""Package-source checks; --linux additionally requires a NEW disposable container.

Mount only each distrib source tree and this script, read-only. Never run the
native cases in a gateway, an existing Kura container or a personal runtime.
No key operation, backup, restore, generation publication or Main start occurs.
"""
import argparse
import hashlib
import os
from pathlib import Path
import pwd
import shutil
import subprocess
import tempfile
import unittest
import xml.etree.ElementTree as ET


WORKSPACE = Path(__file__).resolve().parents[2]
NAMES = ('artemis', 'camel', 'cloud', 'container', 'deployment', 'networking',
         'opcua', 'position', 'triton', 'wires', 'yofc-runtime', 'management-ui')
NS = {'m': 'http://maven.apache.org/POM/4.0.0'}


def distrib(name):
    return WORKSPACE / ('kura-' + name) / 'distrib'


def installer_name(name, action='install'):
    return 'kura_' + ('network' if name == 'networking' else name.replace('-', '_')) + '_' + action + '.sh'


class PackageSourceTests(unittest.TestCase):
    def test_every_reviewed_jar_input_has_root_code_permissions(self):
        count = 0
        for name in NAMES:
            for data in ET.parse(distrib(name) / 'pom.xml').findall('.//m:dataSet/m:data', NS):
                src = data.findtext('m:src', '', NS)
                if '/plugins' not in src and '/input_files/' not in src:
                    continue
                with self.subTest(name=name, src=src):
                    mapper = data.find('m:mapper', NS)
                    self.assertIsNotNone(mapper)
                    self.assertEqual('root', mapper.findtext('m:user', '', NS))
                    self.assertEqual('root', mapper.findtext('m:group', '', NS))
                    self.assertEqual('644', mapper.findtext('m:filemode', '', NS))
                    count += 1
        self.assertEqual(51, count)  # 49 remaining sibling inputs plus two GWT JARs.

    def test_installer_and_helper_mappings_are_protected_code(self):
        for name in NAMES:
            maps = {}
            for data in ET.parse(distrib(name) / 'pom.xml').findall('.//m:dataSet/m:data', NS):
                src = data.findtext('m:src', '', NS)
                if '/src/main/resources/scripts/' in src:
                    maps[Path(src).name] = data
            for filename in (installer_name(name), installer_name(name, 'uninstall'), 'sibling_registry.sh'):
                with self.subTest(name=name, filename=filename):
                    mapper = maps[filename].find('m:mapper', NS)
                    self.assertEqual('/opt/eclipse/kura/libexec/addons/' + name, mapper.findtext('m:prefix', '', NS))
                    self.assertEqual('root', mapper.findtext('m:user', '', NS))
                    self.assertEqual('root', mapper.findtext('m:group', '', NS))
                    self.assertEqual('755', mapper.findtext('m:filemode', '', NS))

    def test_postrm_keeps_the_exact_updater_after_payload_removal(self):
        expected = None
        for name in NAMES:
            helper = (distrib(name) / 'src/main/resources/scripts/sibling_registry.sh').read_text()
            if expected is None:
                expected = helper
            self.assertEqual(expected, helper, name)
            if name != 'networking':
                postrm = (distrib(name) / 'deb/control/postrm').read_text()
                self.assertIn(helper + '\nupdate_sibling_registry remove ' + name, postrm)

    def test_no_privileged_shell_dispatch_remains_in_runtime_data(self):
        for name in NAMES:
            sources = list((distrib(name) / 'deb/control').iterdir())
            sources += list((distrib(name) / 'src/main/resources/scripts').glob('*.sh'))
            for source in sources:
                if source.is_file():
                    with self.subTest(path=str(source)):
                        self.assertNotRegex(source.read_text(), r'\.data/[^\s\"]*\.sh')


class NativePackageTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        assert os.geteuid() == 0 and Path('/.dockerenv').exists()
        for path in ('/etc/kura', '/var/lib/kura-key-service', '/opt/eclipse'):
            assert not os.path.lexists(path), 'Never adopt an installed gateway: ' + path
        subprocess.run(['useradd', '--system', 'kurad'], check=True)
        cls.account = pwd.getpwnam('kurad')

    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory(prefix='sibling-package-test-')
        self.addCleanup(self.tmp.cleanup)
        self.bin = Path(self.tmp.name) / 'bin'
        self.bin.mkdir()
        self.stub('systemd-cat', 'cat\nexit "${YOFC_LOG_STATUS:-0}"\n')
        self.stub('systemctl', 'case "$1" in show) echo loaded;; esac\nexit 0\n')
        self.env = dict(os.environ, PATH=str(self.bin) + ':' + os.environ['PATH'])
        unit = Path('/usr/lib/systemd/system/kura.service.d/kura-networking.conf')
        self.addCleanup(lambda: unit.unlink(missing_ok=True))
        self.reset()

    def tearDown(self):
        shutil.rmtree('/opt/eclipse')
        Path('/tmp/install_error').unlink(missing_ok=True)

    def stub(self, name, body):
        path = self.bin / name
        path.write_text('#!/bin/sh\n' + body)
        path.chmod(0o755)

    def reset(self):
        if Path('/opt/eclipse').exists():
            shutil.rmtree('/opt/eclipse')
        self.home = Path('/opt/eclipse/kura')
        self.framework = self.home / 'framework'
        self.framework.mkdir(parents=True, mode=0o755)
        self.registry = self.framework / 'sibling-install-order'
        self.protected = []
        for name in ('.data', 'user', 'data'):
            parent = self.home / name
            parent.mkdir(mode=0o700)
            os.chown(parent, self.account.pw_uid, self.account.pw_gid)
            for filename in ('existing.bin', 'personal.conf'):
                path = parent / filename
                path.write_bytes(b'opaque existing data; not a real credential\x00')
                path.chmod(0o600)
                os.chown(path, self.account.pw_uid, self.account.pw_gid)
                self.protected.append(path)
        self.before = {str(p): self.metadata(p) for p in self.protected}
        self.target = self.home / 'data/existing.bin'

    @staticmethod
    def metadata(path):
        s = path.lstat()
        return (s.st_ino, s.st_uid, s.st_gid, s.st_mode, s.st_nlink,
                s.st_mtime_ns, s.st_ctime_ns, hashlib.sha256(path.read_bytes()).hexdigest())

    def preserved(self):
        self.assertEqual(self.before, {str(p): self.metadata(p) for p in self.protected})

    def execute(self, args, **env):
        return subprocess.run(args, capture_output=True, text=True, timeout=40, env=dict(self.env, **env))

    def install(self, name):
        if name == 'networking':
            self.stage(name)
            script = self.home / 'libexec/addons/networking' / installer_name(name)
            action = 'upgrade'
        else:
            script = distrib(name) / 'src/main/resources/scripts' / installer_name(name)
            action = 'install'
        return self.execute(['bash', str(script), action, '/opt/eclipse', 'kura'])

    def stage(self, name):
        dest = self.home / 'libexec/addons' / name
        if not dest.exists():
            shutil.copytree(distrib(name) / 'src/main/resources/scripts', dest)
        return dest

    def test_fresh_and_repeated_registration_preserves_data_and_order(self):
        for name in NAMES:
            with self.subTest(name=name):
                self.reset()
                for _ in range(2):
                    result = self.install(name)
                    self.assertEqual(0, result.returncode, result.stderr)
                self.assertEqual(name + '\n', self.registry.read_text())
                s = self.registry.stat()
                self.assertEqual((0, 0, 0o644), (s.st_uid, s.st_gid, s.st_mode & 0o777))
                self.preserved()

    def test_legacy_registry_is_replaced_without_reordering(self):
        for name in NAMES:
            with self.subTest(name=name):
                self.reset()
                content = 'first\n' + name + '\nlast\n'
                self.registry.write_text(content)
                os.chown(self.registry, self.account.pw_uid, self.account.pw_gid)
                self.registry.chmod(0o664)
                result = self.install(name)
                self.assertEqual(0, result.returncode, result.stderr)
                self.assertEqual(content, self.registry.read_text())
                self.assertEqual(0, self.registry.stat().st_uid)
                self.preserved()

    def test_install_and_remove_refuse_links_without_modifying_targets(self):
        for name in NAMES:
            for hardlink in (False, True):
                with self.subTest(name=name, hardlink=hardlink):
                    self.reset()
                    if hardlink:
                        os.link(self.target, self.registry)
                    else:
                        self.registry.symlink_to(self.target)
                    target_before = self.metadata(self.target)
                    self.assertNotEqual(0, self.install(name).returncode)
                    if name != 'networking':
                        result = self.execute(['bash', str(distrib(name) / 'deb/control/postrm'), 'remove'])
                        self.assertNotEqual(0, result.returncode)
                    self.assertEqual(target_before, self.metadata(self.target))

    def test_unsafe_framework_and_lock_are_rejected(self):
        for name in NAMES:
            for condition in ('owner', 'mode', 'lock-link', 'lock-owner'):
                with self.subTest(name=name, condition=condition):
                    self.reset()
                    lock = self.framework / 'sibling-install-order.lock'
                    if condition == 'owner':
                        os.chown(self.framework, self.account.pw_uid, self.account.pw_gid)
                    elif condition == 'mode':
                        self.framework.chmod(0o775)
                    elif condition == 'lock-link':
                        lock.symlink_to(self.target)
                    else:
                        lock.write_bytes(b'')
                        os.chown(lock, self.account.pw_uid, self.account.pw_gid)
                    self.assertNotEqual(0, self.install(name).returncode)
                    self.assertFalse(self.registry.exists())
                    self.preserved()

    def test_postinst_reports_child_and_log_sink_failure(self):
        for name in NAMES:
            with self.subTest(name=name):
                self.reset()
                dest = self.stage(name)
                script = dest / installer_name(name)
                script.write_text('#!/bin/bash\nexit 37\n')
                postinst = distrib(name) / 'deb/control/postinst'
                result = self.execute(['bash', str(postinst), 'configure', '1.0'])
                self.assertNotEqual(0, result.returncode)
                self.assertIn('installation failed', result.stdout)
                script.write_text('#!/bin/bash\necho completed\nexit 0\n')
                result = self.execute(['bash', str(postinst), 'configure', '1.0'], YOFC_LOG_STATUS='29')
                self.assertNotEqual(0, result.returncode)
                self.preserved()

    def test_postrm_removes_only_its_entry_after_payload_is_absent(self):
        for name in NAMES:
            if name == 'networking':
                continue  # Networking deregisters synchronously in prerm.
            with self.subTest(name=name):
                self.reset()
                self.registry.write_text('first\n' + name + '\nlast\n')
                self.assertFalse((self.home / 'libexec').exists())
                result = self.execute(['bash', str(distrib(name) / 'deb/control/postrm'), 'remove'])
                self.assertEqual(0, result.returncode, result.stderr)
                self.assertEqual('first\nlast\n', self.registry.read_text())
                self.assertEqual(0o644, self.registry.stat().st_mode & 0o777)
                self.preserved()

    def test_upgrade_removal_hooks_do_not_touch_registry_or_runtime(self):
        for name in NAMES:
            with self.subTest(name=name):
                self.reset()
                self.registry.write_text('first\n' + name + '\nlast\n')
                before = self.metadata(self.registry)
                hook = 'prerm' if name == 'networking' else 'postrm'
                for phase in ('upgrade', 'failed-upgrade'):
                    result = self.execute(['bash', str(distrib(name) / 'deb/control' / hook), phase, '1.0'])
                    self.assertEqual(0, result.returncode, result.stderr)
                self.assertEqual(before, self.metadata(self.registry))
                self.preserved()

    def test_atomic_replace_failure_does_not_erase_original_registry(self):
        self.stub('mv', 'exit 42\n')
        for name in NAMES:
            with self.subTest(name=name):
                self.reset()
                self.registry.write_text('keep\n')
                before = self.metadata(self.registry)
                self.assertNotEqual(0, self.install(name).returncode)
                self.assertEqual(before, self.metadata(self.registry))
                self.assertEqual([], list(self.framework.glob('.sibling-install-order.*')))
                self.preserved()

    def test_concurrent_registrations_keep_all_siblings_once(self):
        processes = []
        for name in NAMES:
            dest = self.stage(name)
            processes.append(subprocess.Popen(['bash', str(dest / installer_name(name)),
                'upgrade' if name == 'networking' else 'install', '/opt/eclipse', 'kura'],
                stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, env=self.env))
        for process in processes:
            output, error = process.communicate(timeout=40)
            self.assertEqual(0, process.returncode, error + output)
        lines = self.registry.read_text().splitlines()
        self.assertEqual(sorted(NAMES), sorted(lines))
        self.preserved()

    def test_network_code_upgrade_accepts_encrypted_snapshot_without_rewriting_it(self):
        dest = self.stage('networking')
        snapshots = self.home / 'user/snapshots'
        snapshots.mkdir()
        snapshot = snapshots / 'snapshot_0.xml'
        snapshot.write_bytes(b'opaque encrypted snapshot fixture; no real key')
        before = self.metadata(snapshot)
        result = self.execute(['bash', str(distrib('networking') / 'deb/control/preinst'), 'upgrade', '1.0'])
        self.assertEqual(0, result.returncode, result.stderr)
        result = self.execute(['bash', str(distrib('networking') / 'deb/control/postinst'), 'configure', '1.0'])
        self.assertEqual(0, result.returncode, result.stderr)
        self.assertEqual(before, self.metadata(snapshot))
        self.assertTrue((dest / 'customize_snapshot_network.py').exists())
        self.preserved()

    def test_network_children_finish_and_propagate_failure_before_registration(self):
        dest = self.stage('networking')
        self.stub('java', 'exit 0\n')
        self.stub('keytool', 'exit 0\n')
        self.stub('systemctl', 'case "$1" in show) echo loaded;; daemon-reload) exit 47;; *) exit 0;; esac\n')
        result = self.execute(['bash', str(distrib('networking') / 'deb/control/postinst'), 'configure'])
        self.assertNotEqual(0, result.returncode)
        self.assertFalse(self.registry.exists())
        # Fail at the first uninstall child; no network restore is performed.
        (dest / 'kura_network_options.sh').write_text('#!/bin/bash\nsleep 0.2\nexit 43\n')
        result = self.execute(['bash', str(distrib('networking') / 'deb/control/prerm'), 'remove'])
        self.assertNotEqual(0, result.returncode)
        self.assertIn('uninstallation failed', result.stdout)
        self.preserved()

    def test_network_options_failure_prevents_registration(self):
        dest = self.stage('networking')
        marker = Path(self.tmp.name) / 'permissions-child-entered'
        (dest / 'kura_network_options.sh').write_text(
            '#!/bin/bash\ntouch ' + str(marker) + '\nexit 43\n')
        self.stub('java', 'exit 0\n')
        self.stub('keytool', 'exit 0\n')
        self.stub('systemctl', 'case "$1" in show) echo not-found; exit 4;; *) exit 0;; esac\n')
        result = self.execute(['bash', str(distrib('networking') / 'deb/control/postinst'), 'configure'])
        self.assertNotEqual(0, result.returncode)
        self.assertTrue(marker.exists(), result.stderr + result.stdout)
        self.assertFalse(self.registry.exists())
        self.preserved()

    def test_cloud_property_links_cannot_redirect_data_writes(self):
        for hardlink in (False, True):
            with self.subTest(hardlink=hardlink):
                self.reset()
                config = self.home / 'user/kura_custom.properties'
                if hardlink:
                    os.link(self.target, config)
                else:
                    config.symlink_to(self.target)
                before = self.metadata(self.target)
                self.assertNotEqual(0, self.install('cloud').returncode)
                self.assertEqual(before, self.metadata(self.target))
                self.assertFalse(self.registry.exists())

    def test_core_owned_home_symlink_keeps_the_same_data_and_all_entries(self):
        canonical = self.home.with_name('kura_fixture_version')
        self.home.rename(canonical)
        self.home.symlink_to(canonical.name)
        try:
            for name in NAMES:
                result = self.install(name)
                self.assertEqual(0, result.returncode, name + ': ' + result.stderr)
            self.assertEqual(list(NAMES), self.registry.read_text().splitlines())
            self.preserved()
        finally:
            self.home.unlink()
            canonical.rename(self.home)

    def test_network_provider_lifecycle_preserves_system_authority_and_encrypted_snapshots(self):
        self.stage('networking')
        paths = [Path(value) for value in (
            '/etc/netplan/10-administrator.yaml',
            '/etc/cloud/cloud.cfg.d/99-disable-network-config.cfg',
            '/etc/NetworkManager/conf.d/administrator.conf',
            '/etc/network/interfaces', '/etc/default/dnsmasq',
            '/etc/sysconfig/iptables', '/etc/sysconfig/iptables.kurasave',
            '/var/lib/dhcp/dnsmasq.leases',
            '/opt/eclipse/kura/user/snapshots/snapshot_0.xml',
            '/opt/eclipse/kura/user/snapshots/snapshot_1700000000000.xml')]
        for path in paths:
            self.assertFalse(os.path.lexists(path), str(path))
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(b'opaque administrator state; never render or decrypt\x00')
            path.chmod(0o600)
            self.addCleanup(lambda p=path: p.unlink(missing_ok=True))
        # Existing system policy and resolver are equally outside package scope.
        paths += [Path('/etc/pam.d/su'), Path('/etc/resolv.conf')]
        before = {str(path): self.metadata(path) for path in paths}
        calls = Path(self.tmp.name) / 'systemctl-calls'
        self.stub('systemctl', 'printf "%s\\n" "$*" >> ' + str(calls) + '\nexit 0\n')
        for command in ('netplan', 'udevadm', 'iptables', 'ip6tables', 'iptables-restore', 'nmcli'):
            self.stub(command, 'echo forbidden-system-write >&2\nexit 91\n')
        for hook, args in [('preinst', ['install']), ('postinst', ['configure']),
                           ('preinst', ['upgrade', '1.0']), ('postinst', ['configure', '1.0'])]:
            result = self.execute(['bash', str(distrib('networking') / 'deb/control' / hook), *args])
            self.assertEqual(0, result.returncode, result.stderr + result.stdout)
        self.assertEqual(['daemon-reload', 'daemon-reload'], calls.read_text().splitlines())
        config = self.home / 'user/kura_custom.properties'
        self.assertIn(b'kura.have.net.admin=true', config.read_bytes())
        self.assertEqual(self.account.pw_uid, config.stat().st_uid)
        result = self.execute(['bash', str(self.home / 'libexec/addons/networking/kura_network_uninstall.sh'),
                               'remove', '/opt/eclipse', 'kura'])
        self.assertEqual(0, result.returncode, result.stderr + result.stdout)
        self.assertEqual(['daemon-reload'] * 3, calls.read_text().splitlines())
        self.assertNotIn('networking', self.registry.read_text().splitlines())
        self.assertIn(b'kura.have.net.admin=false', config.read_bytes())
        self.assertEqual(before, {str(path): self.metadata(path) for path in paths})
        self.preserved()

    def test_legacy_package_policy_is_retained_by_rejecting_before_unpack(self):
        path = Path('/etc/NetworkManager/conf.d/99-kura-nm.conf')
        self.assertFalse(os.path.lexists(path))
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text('[main]\nno-auto-default=*\n')
        self.addCleanup(lambda: path.unlink(missing_ok=True))
        before = self.metadata(path)
        self.stub('dpkg-query', 'echo ' + str(path) + '\n')
        result = self.execute(['bash', str(distrib('networking') / 'deb/control/preinst'), 'upgrade', '1.0'])
        self.assertNotEqual(0, result.returncode)
        self.assertIn('retained-configuration migration', result.stderr)
        self.assertEqual(before, self.metadata(path))
        self.preserved()

    def test_network_properties_reject_links_and_leave_existing_flags_unchanged_on_repeat(self):
        dest = self.stage('networking')
        path = self.home / 'user/kura_custom.properties'
        for hardlink in (False, True):
            with self.subTest(hardlink=hardlink):
                if hardlink:
                    os.link(self.target, path)
                else:
                    path.symlink_to(self.target)
                before = self.metadata(self.target)
                result = self.execute(['bash', str(dest / 'kura_network_options.sh'), 'true'])
                self.assertNotEqual(0, result.returncode)
                self.assertEqual(before, self.metadata(self.target))
                path.unlink()
        path.write_bytes(b'# personal configuration\ncustom.value=preserve\x00\n')
        os.chown(path, self.account.pw_uid, self.account.pw_gid)
        path.chmod(0o600)
        for action in ('true', 'false'):
            result = self.execute(['bash', str(dest / 'kura_network_options.sh'), action])
            self.assertEqual(0, result.returncode, result.stderr)
            before = self.metadata(path)
            result = self.execute(['bash', str(dest / 'kura_network_options.sh'), action])
            self.assertEqual(0, result.returncode, result.stderr)
            self.assertEqual(before, self.metadata(path))
            self.assertTrue(path.read_bytes().startswith(b'# personal configuration\ncustom.value=preserve\x00\n'))

    def test_modified_network_dropin_blocks_install_and_removal_without_overwriting(self):
        dest = self.stage('networking')
        path = Path('/usr/lib/systemd/system/kura.service.d/kura-networking.conf')
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text('[Service]\nEnvironment=PERSONAL=yes\n')
        before = self.metadata(path)
        for action in ('install', 'remove'):
            filename = installer_name('networking', 'uninstall' if action == 'remove' else 'install')
            result = self.execute(['bash', str(dest / filename), action, '/opt/eclipse', 'kura'])
            self.assertNotEqual(0, result.returncode)
            self.assertEqual(before, self.metadata(path))
            self.assertFalse(self.registry.exists())
            self.assertFalse((self.home / 'user/kura_custom.properties').exists())


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--linux', action='store_true')
    parser.add_argument('--container', action='store_true', help='Build the declared test image and run the native suite in a fresh container')
    args = parser.parse_args()
    if args.container:
        assert not args.linux
        image = 'yofc-kura-sibling-package-tests:ubuntu24.04'
        with tempfile.TemporaryDirectory(prefix='sibling-test-image-') as directory:
            shutil.copyfile(Path(__file__).with_suffix('.Dockerfile'), Path(directory) / 'Dockerfile')
            subprocess.run(['docker', 'build', '-t', image, directory], check=True)
        command = ['docker', 'run', '--rm', '--network', 'none']
        for name in NAMES:
            command += ['--mount', 'type=bind,src=' + str(distrib(name)) + ',dst=/workspace/kura-' + name + '/distrib,readonly']
        command += ['--mount', 'type=bind,src=' + str(Path(__file__).resolve()) + ',dst=/workspace/kura-management-console/scripts/' + Path(__file__).name + ',readonly']
        command += [image, 'python3', '/workspace/kura-management-console/scripts/' + Path(__file__).name, '--linux']
        raise SystemExit(subprocess.run(command).returncode)
    suite = unittest.defaultTestLoader.loadTestsFromTestCase(PackageSourceTests)
    if args.linux:
        suite.addTests(unittest.defaultTestLoader.loadTestsFromTestCase(NativePackageTests))
    result = unittest.TextTestRunner(verbosity=2).run(suite)
    raise SystemExit(not result.wasSuccessful())
