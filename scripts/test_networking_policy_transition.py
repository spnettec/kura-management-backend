#!/usr/bin/env python3
# Copyright (c) 2026 YOFC IOT and others
# SPDX-License-Identifier: EPL-2.0
"""Real dpkg conffile migration checks in a NEW disposable Linux container."""
import hashlib
import importlib.util
import os
from pathlib import Path
import shutil
import stat
import subprocess
import sys
import tempfile
import unittest

sys.dont_write_bytecode = True
REPO = Path('/workspace/kura-networking')
SPEC = importlib.util.spec_from_file_location('transition', REPO / 'distrib/build-legacy-nm-transition.py')
TRANSITION = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(TRANSITION)
SOURCE = REPO / 'distrib'
POLICY = Path('/etc/NetworkManager/conf.d/99-kura-nm.conf')
TEMPLATE = SOURCE / 'src/main/resources/scripts/99-kura-nm.conf'


def metadata(path):
    info = path.lstat()
    return dict(sha256=hashlib.sha256(path.read_bytes()).hexdigest(), uid=info.st_uid,
                gid=info.st_gid, mode=stat.S_IMODE(info.st_mode), nlink=info.st_nlink,
                inode=info.st_ino, mtimeNs=info.st_mtime_ns, ctimeNs=info.st_ctime_ns)


class NativeTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory(prefix='nm-transition-tests-')
        self.root = Path(self.tmp.name)
        self.assertEqual(0, os.geteuid())
        self.assertFalse(POLICY.exists())
        POLICY.parent.mkdir(parents=True, exist_ok=True)

    def tearDown(self):
        subprocess.run(['dpkg', '--purge', 'kura-networking'], stdout=subprocess.DEVNULL,
                       stderr=subprocess.DEVNULL, check=True)
        if POLICY.is_symlink() or POLICY.exists():
            POLICY.unlink()
        self.tmp.cleanup()

    def package(self, version, policy=False, conffile=False, transition=False,
                current_controls=False, drift=False):
        tree = self.root / ('tree-' + version)
        control = tree / 'DEBIAN'
        control.mkdir(parents=True)
        (control / 'control').write_text('Package: kura-networking\nVersion: ' + version +
            '\nArchitecture: all\nMaintainer: YOFC fixture <test@example.invalid>\n'
            'Description: disposable dpkg migration fixture, not a release\n')
        if current_controls:
            for filename in ['preinst', 'postinst', 'prerm']:
                shutil.copyfile(SOURCE / 'deb/control' / filename, control / filename)
                (control / filename).chmod(0o755)
        else:
            text = (SOURCE / 'deb/control/preinst').read_bytes()
            (control / 'preinst').write_bytes(TRANSITION.guard(text, version) if transition else text)
            (control / 'preinst').chmod(0o755)
        if drift:
            with (control / 'preinst').open('ab') as stream:
                stream.write(b'\n# unreviewed drift\n')
        template = tree / 'opt/eclipse/kura/libexec/addons/networking/99-kura-nm.conf'
        template.parent.mkdir(parents=True)
        shutil.copyfile(TEMPLATE, template)
        if policy:
            target = tree / POLICY.relative_to('/')
            target.parent.mkdir(parents=True)
            shutil.copyfile(TEMPLATE, target)
        if conffile:
            (control / 'conffiles').write_text(str(POLICY) + '\n')
        package = self.root / ('package-' + version + '.deb')
        subprocess.run(['dpkg-deb', '--root-owner-group', '--build', str(tree), str(package)],
                       check=True, stdout=subprocess.DEVNULL)
        return package

    def install(self, package, keep=False):
        command = ['dpkg'] + (['--force-confold'] if keep else []) + ['-i', str(package)]
        return subprocess.run(command, capture_output=True, text=True, timeout=15)

    def legacy(self):
        old = self.package('1', policy=True)
        # The historical ordinary-file package has no new migration guard.
        tree = self.root / 'tree-1'
        (tree / 'DEBIAN/preinst').unlink()
        subprocess.run(['dpkg-deb', '--root-owner-group', '--build', str(tree), str(old)],
                       check=True, stdout=subprocess.DEVNULL)
        result = self.install(old)
        self.assertEqual(0, result.returncode, result.stderr)

    def retained_round_trip(self, custom):
        self.legacy()
        if custom:
            POLICY.write_bytes(b'[connection]\n# administrator custom settings\n')
            POLICY.chmod(0o600)
        before = metadata(POLICY)
        bridge = self.package('2', policy=True, conffile=True, transition=True)
        result = self.install(bridge, keep=True)
        self.assertEqual(0, result.returncode, result.stderr)
        self.assertEqual(before, metadata(POLICY))
        normal = self.package('3')
        result = self.install(normal)
        self.assertEqual(0, result.returncode, result.stderr)
        self.assertEqual(before, metadata(POLICY))
        conffiles = subprocess.check_output(['dpkg-query', '-W', '-f=${Conffiles}',
                                           'kura-networking'], text=True)
        self.assertIn(str(POLICY), conffiles)
        self.assertTrue(conffiles.rstrip().endswith('obsolete'))
        diversions = subprocess.check_output(['dpkg-divert', '--list', str(POLICY)], text=True)
        self.assertEqual('', diversions)

    def test_unchanged_legacy_file_is_retained_without_metadata_changes(self):
        self.retained_round_trip(False)

    def test_customized_legacy_file_is_retained_without_metadata_changes(self):
        self.retained_round_trip(True)

    def test_transition_is_rejected_on_fresh_install(self):
        result = self.install(self.package('2', policy=True, conffile=True, transition=True), keep=True)
        self.assertNotEqual(0, result.returncode)
        self.assertIn('only for an existing', result.stderr)
        self.assertFalse(POLICY.exists())

    def test_missing_keep_option_rejects_before_unpack(self):
        self.legacy()
        before = metadata(POLICY)
        result = self.install(self.package('2', policy=True, conffile=True, transition=True))
        self.assertNotEqual(0, result.returncode)
        self.assertIn('--force-confold', result.stderr)
        self.assertEqual(before, metadata(POLICY))
        self.assertEqual('1', subprocess.check_output(['dpkg-query', '-W', '-f=${Version}',
                                                     'kura-networking'], text=True))

    def test_conflicting_conffile_options_reject_before_configuration_access(self):
        self.legacy()
        before = metadata(POLICY)
        for force in ['confold,confnew', 'confmiss,confold', 'confold,confask']:
            result = subprocess.run(['bash', str(SOURCE / 'deb/legacy-nm-policy-preinst'),
                                     'upgrade', '1'], env={'PATH': '/usr/bin:/bin', 'DPKG_FORCE': force},
                                    capture_output=True, text=True)
            self.assertNotEqual(0, result.returncode)
            self.assertIn('Conflicting', result.stderr)
            self.assertEqual(before, metadata(POLICY))

    def test_transition_cannot_downgrade_the_existing_package(self):
        self.legacy()
        before = metadata(POLICY)
        result = subprocess.run(['bash', str(SOURCE / 'deb/legacy-nm-policy-preinst'),
                                 'upgrade', '1'], env={'PATH': '/usr/bin:/bin', 'DPKG_FORCE': 'confold',
                                 'NM_POLICY_TRANSITION_VERSION': '0'}, capture_output=True, text=True)
        self.assertNotEqual(0, result.returncode)
        self.assertIn('cannot downgrade', result.stderr)
        self.assertEqual(before, metadata(POLICY))

    def test_unsafe_existing_policy_is_rejected_without_changing_target(self):
        self.legacy()
        guard = SOURCE / 'deb/legacy-nm-policy-preinst'
        original = POLICY.read_bytes()
        outside = self.root / 'outside'
        for kind in ['symlink', 'hardlink', 'writable', 'foreign-owner', 'parent-writable']:
            with self.subTest(kind=kind):
                outside.write_bytes(original)
                if kind == 'symlink':
                    POLICY.unlink(); POLICY.symlink_to(outside)
                elif kind == 'hardlink':
                    POLICY.unlink(); os.link(outside, POLICY)
                elif kind == 'writable': POLICY.chmod(0o666)
                elif kind == 'foreign-owner': os.chown(POLICY, 501, 501)
                else: POLICY.parent.chmod(0o777)
                before = metadata(outside)
                result = subprocess.run(['bash', str(guard), 'upgrade', '1'],
                    env={'PATH': '/usr/sbin:/usr/bin:/sbin:/bin', 'DPKG_FORCE': 'confold',
                         'NM_POLICY_TRANSITION_VERSION': '2'},
                    capture_output=True, text=True)
                self.assertNotEqual(0, result.returncode)
                self.assertEqual(before, metadata(outside))
                POLICY.parent.chmod(0o755)
                POLICY.unlink(); POLICY.write_bytes(original); POLICY.chmod(0o644)

    def test_normal_fresh_package_has_no_active_policy(self):
        result = self.install(self.package('3'))
        self.assertEqual(0, result.returncode, result.stderr)
        self.assertFalse(POLICY.exists())

    def test_builder_binds_exact_base_controls_and_preserves_all_other_payload(self):
        base = self.package('3', current_controls=True)
        sha = hashlib.sha256(base.read_bytes()).hexdigest()
        receipt = TRANSITION.build(base, sha, self.root / 'output')
        self.assertTrue(receipt['requiresForceConfold'])
        self.assertEqual('3~nm-policy-retain1', receipt['transitionVersion'])
        tree = self.root / 'unpacked'
        subprocess.run(['dpkg-deb', '--raw-extract', receipt['package'], str(tree)], check=True)
        self.assertEqual(str(POLICY) + '\n', (tree / 'DEBIAN/conffiles').read_text())
        self.assertIn(b'requires dpkg --force-confold', (tree / 'DEBIAN/preinst').read_bytes())
        self.assertEqual(TEMPLATE.read_bytes(), (tree / POLICY.relative_to('/')).read_bytes())
        for filename in ['postinst', 'prerm']:
            self.assertEqual((SOURCE / 'deb/control' / filename).read_bytes(),
                             (tree / 'DEBIAN' / filename).read_bytes())
        bad = self.package('4', current_controls=True, drift=True)
        with self.assertRaisesRegex(ValueError, 'preinst.*current source'):
            TRANSITION.build(bad, hashlib.sha256(bad.read_bytes()).hexdigest(), self.root / 'bad')
        with self.assertRaisesRegex(ValueError, 'SHA256'):
            TRANSITION.build(base, '0' * 64, self.root / 'bad-sha')
        active = self.package('5', policy=True, current_controls=True)
        with self.assertRaisesRegex(ValueError, 'active NM policy'):
            TRANSITION.build(active, hashlib.sha256(active.read_bytes()).hexdigest(), self.root / 'bad-active')


if __name__ == '__main__':
    unittest.main(verbosity=2)
