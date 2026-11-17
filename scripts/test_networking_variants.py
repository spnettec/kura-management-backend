#!/usr/bin/env python3
# Copyright (c) 2026 YOFC IOT and others
# SPDX-License-Identifier: EPL-2.0
"""Firewall compatibility checks, in a NEW disposable Linux container only."""
import argparse
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest
import xml.etree.ElementTree as ET

sys.dont_write_bytecode = True
from test_sibling_package_security import NativePackageTests as Fixture, WORKSPACE, NS

REPO = WORKSPACE / 'kura-networking'
SOURCE = REPO / 'distrib-firewall'
SNAPSHOT = '<?xml version="1.0" encoding="UTF-8" standalone="yes"?>\n<esf:configurations xmlns:esf="urn:kura-test"><esf:configuration pid="keep"/></esf:configurations>\n'
PID = 'org.eclipse.kura.net.admin.FirewallConfigurationService'


class SourceTests(unittest.TestCase):
    def test_four_jar_mappings_and_installer_code_are_root_owned(self):
        jars = 0
        helper = False
        for data in ET.parse(SOURCE / 'pom.xml').findall('.//m:dataSet/m:data', NS):
            src = data.findtext('m:src', '', NS)
            mapper = data.find('m:mapper', NS)
            if '/target/plugins/' in src:
                jars += 1
                self.assertEqual('root', mapper.findtext('m:user', '', NS))
                self.assertEqual('root', mapper.findtext('m:group', '', NS))
                self.assertEqual('644', mapper.findtext('m:filemode', '', NS))
            if src.endswith('sibling_registry.sh'):
                helper = True
                self.assertEqual('/opt/eclipse/kura/libexec/addons/firewall', mapper.findtext('m:prefix', '', NS))
                self.assertEqual('root', mapper.findtext('m:user', '', NS))
        self.assertEqual(4, jars)
        self.assertTrue(helper)


class FirewallTests(unittest.TestCase):
    setUpClass = classmethod(Fixture.setUpClass.__func__)
    stub = Fixture.stub
    reset = Fixture.reset
    metadata = staticmethod(Fixture.metadata)
    execute = Fixture.execute
    preserved = Fixture.preserved

    def setUp(self):
        Fixture.setUp(self)
        self.stub('systemctl', 'echo "$*" >> "${FW_CALLS}"\nexit "${FW_SYSTEMCTL_STATUS:-0}"\n')
        self.stub('iptables', 'echo "iptables $*" >> "${FW_CALLS}"\nexit "${FW_IPTABLES_STATUS:-0}"\n')
        self.stub('ip6tables', 'echo "ip6tables $*" >> "${FW_CALLS}"\nexit 0\n')
        self.stub('iptables-restore', 'cat >/dev/null\necho ipv4-restore >> "${FW_CALLS}"\nexit 37\n')
        self.stub('ip6tables-restore', 'cat >/dev/null\necho ipv6-restore >> "${FW_CALLS}"\nexit 0\n')
        self.calls = Path(self.tmp.name) / 'calls'
        self.env['FW_CALLS'] = str(self.calls)
        self.code = self.home / 'libexec/addons/firewall'
        shutil.copytree(SOURCE / 'src/main/resources/scripts', self.code)
        shutil.copyfile(REPO / 'distrib/src/main/resources/scripts/sibling_registry.sh', self.code / 'sibling_registry.sh')
        self.snapshots = self.home / 'user/snapshots'
        self.snapshots.mkdir(mode=0o700)
        os.chown(self.snapshots, self.account.pw_uid, self.account.pw_gid)
        self.rules = Path('/etc/sysconfig/iptables')
        self.rules.parent.mkdir(exist_ok=True)
        self.rules.write_bytes(b'original unrelated fixture rules\n')

    def tearDown(self):
        Fixture.tearDown(self)
        if Path('/etc/kura').exists():
            shutil.rmtree('/etc/kura')
        for name in ('iptables', 'ip6tables', 'iptables.kura-firewall-only-disabled', 'ip6tables.kura-firewall-only-disabled'):
            Path('/etc/sysconfig', name).unlink(missing_ok=True)

    def snapshot(self, number, text=SNAPSHOT):
        path = self.snapshots / ('snapshot_' + str(number) + '.xml')
        path.write_text(text)
        path.chmod(0o600)
        os.chown(path, self.account.pw_uid, self.account.pw_gid)
        return path

    def hook(self, hook, *arguments, **env):
        return self.execute(['bash', str(SOURCE / 'deb/control' / hook), *arguments], **env)

    def test_upgrade_hooks_preserve_encrypted_data_rules_and_install_order(self):
        path = self.snapshot(0, 'opaque encrypted fixture; no key material')
        self.registry.write_text('first\nfirewall\nlast\n')
        before = self.metadata(path)
        rules = self.metadata(self.rules)
        for hook, args in [('preinst', ('upgrade', '1.0')), ('prerm', ('upgrade', '2.0')), ('postinst', ('configure', '1.0'))]:
            result = self.hook(hook, *args)
            self.assertEqual(0, result.returncode, result.stderr)
        self.assertEqual('first\nfirewall\nlast\n', self.registry.read_text())
        self.assertEqual(before, self.metadata(path))
        self.assertEqual(rules, self.metadata(self.rules))
        self.assertNotIn('iptables', self.calls.read_text())
        self.assertNotIn('disable', self.calls.read_text())
        self.preserved()

    def test_fresh_baseline_and_latest_change_only_once_as_application(self):
        baseline = self.snapshot(0)
        history = self.snapshot(3)
        latest = self.snapshot(11)
        history_before = self.metadata(history)
        result = self.hook('postinst', 'configure')
        self.assertEqual(0, result.returncode, result.stderr)
        for path in (baseline, latest):
            self.assertEqual(1, path.read_text().count(PID))
            self.assertEqual((self.account.pw_uid, self.account.pw_gid, 0o600),
                             (path.stat().st_uid, path.stat().st_gid, path.stat().st_mode & 0o777))
        self.assertEqual(history_before, self.metadata(history))
        before = {path: self.metadata(path) for path in (baseline, latest)}
        result = self.hook('postinst', 'configure')
        self.assertEqual(0, result.returncode, result.stderr)
        self.assertEqual(before, {path: self.metadata(path) for path in before})
        self.assertEqual('firewall\n', self.registry.read_text())
        self.preserved()

    def test_invalid_latest_is_detected_before_baseline_or_host_edits(self):
        baseline = self.snapshot(0)
        self.snapshot(9, 'opaque encrypted fixture; no key material')
        before = self.metadata(baseline)
        rules = self.metadata(self.rules)
        result = self.hook('postinst', 'configure')
        self.assertNotEqual(0, result.returncode)
        self.assertEqual(before, self.metadata(baseline))
        self.assertEqual(rules, self.metadata(self.rules))
        self.assertFalse(self.calls.exists())
        self.assertFalse(self.registry.exists())
        self.preserved()

    def test_agent_marker_preserves_rule_files_and_snapshots_on_install_and_remove(self):
        Path('/etc/kura').mkdir()
        Path('/etc/kura/firewall-owner-agent').write_bytes(b'fixture ownership marker; not a Root journal')
        snapshot = self.snapshot(0, 'opaque encrypted fixture; no key material')
        before = self.metadata(snapshot)
        rules = self.metadata(self.rules)
        result = self.hook('postinst', 'configure')
        self.assertEqual(0, result.returncode, result.stderr)
        result = self.hook('prerm', 'remove')
        self.assertEqual(0, result.returncode, result.stderr)
        self.assertEqual(before, self.metadata(snapshot))
        self.assertEqual(rules, self.metadata(self.rules))
        self.assertNotIn('iptables', self.calls.read_text())
        self.assertNotIn('disable', self.calls.read_text())
        self.assertEqual('', self.registry.read_text())
        self.preserved()

    def test_registry_link_is_rejected_before_snapshot_or_kernel_changes(self):
        snapshot = self.snapshot(0)
        before = self.metadata(snapshot)
        self.registry.symlink_to(self.target)
        for hook, arg in [('postinst', 'configure'), ('prerm', 'remove')]:
            result = self.hook(hook, arg)
            self.assertNotEqual(0, result.returncode)
        self.assertEqual(before, self.metadata(snapshot))
        self.assertFalse(self.calls.exists())
        self.preserved()

    def test_kernel_failure_is_reported_and_registry_stays_registered(self):
        self.registry.write_text('first\nfirewall\nlast\n')
        result = self.hook('prerm', 'remove', FW_IPTABLES_STATUS='37')
        self.assertNotEqual(0, result.returncode)
        self.assertEqual('first\nfirewall\nlast\n', self.registry.read_text())
        self.assertNotIn('ip6tables', self.calls.read_text())
        self.preserved()

    def test_both_boot_writers_propagate_ipv4_restore_failure(self):
        binary = self.home / 'bin/firewall'
        binary.parent.mkdir()
        for source in (SOURCE / 'src/main/resources/scripts/firewall-only.init',
                       REPO / 'distrib/src/main/resources/scripts/firewall.init'):
            with self.subTest(source=source.name):
                self.calls.unlink(missing_ok=True)
                shutil.copyfile(source, binary)
                result = self.execute(['bash', str(binary)])
                self.assertEqual(37, result.returncode)
                self.assertEqual('ipv4-restore\n', self.calls.read_text())
        self.preserved()

    def test_failed_systemd_reload_is_not_reported_as_success(self):
        self.registry.write_text('first\n')
        result = self.hook('postinst', 'configure', '1.0', FW_SYSTEMCTL_STATUS='43')
        self.assertNotEqual(0, result.returncode)
        self.assertEqual('first\n', self.registry.read_text())
        self.preserved()


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--linux', action='store_true')
    parser.add_argument('--container', action='store_true')
    args = parser.parse_args()
    if args.container:
        image = 'yofc-kura-sibling-package-tests:ubuntu24.04'
        with tempfile.TemporaryDirectory(prefix='variant-test-image-') as directory:
            shutil.copyfile(Path(__file__).with_name('test_sibling_package_security.Dockerfile'), Path(directory) / 'Dockerfile')
            subprocess.run(['docker', 'build', '-t', image, directory], check=True)
        command = ['docker', 'run', '--rm', '--network', 'none']
        for folder in ('distrib-firewall', 'distrib'):
            command += ['--mount', 'type=bind,src=' + str(REPO / folder) + ',dst=/workspace/kura-networking/' + folder + ',readonly']
        for filename in (Path(__file__).name, 'test_sibling_package_security.py'):
            command += ['--mount', 'type=bind,src=' + str(Path(__file__).with_name(filename)) + ',dst=/workspace/kura-management-console/scripts/' + filename + ',readonly']
        command += [image, 'python3', '/workspace/kura-management-console/scripts/' + Path(__file__).name, '--linux']
        raise SystemExit(subprocess.run(command).returncode)
    suite = unittest.defaultTestLoader.loadTestsFromTestCase(SourceTests)
    if args.linux:
        suite.addTests(unittest.defaultTestLoader.loadTestsFromTestCase(FirewallTests))
    result = unittest.TextTestRunner(verbosity=2).run(suite)
    raise SystemExit(not result.wasSuccessful())
