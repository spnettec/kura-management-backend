"""Native delivery control tests. Root filesystem fixtures are not host acceptance."""
import copy
import hashlib
import json
import os
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest.mock import patch
import uuid
from host_control import HostControl
from native_packages import NativePackages, BASE, CATALOG, PROGRAM
from setup import Setup


def digest(data):
    return hashlib.sha256(data).hexdigest()


def release():
    payload = {
        '/usr/bin/yofc-management-console': b'unit-only executable; never launched',
        '/usr/share/kura-management-console/source-manifest.json': b'{"unitOnly":true}\n',
        '/usr/lib/systemd/system/kura-management-console.service': b'unit-only default unit\n',
        '/usr/lib/systemd/system/kura-management-console@.service': b'unit-only template\n',
        '/usr/share/kura-management-console/package-v1': b'kura-management-console-v1\n',
        '/usr/share/kura-management-console/react/index.html': b'unit-only html\n',
        '/usr/share/kura-management-console/react/assets/unit.js': b'unit-only js\n',
    }
    binary = digest(payload['/usr/bin/yofc-management-console'])
    source = digest(payload['/usr/share/kura-management-console/source-manifest.json'])
    provenance = {'schemaVersion': 1, 'maintenanceProtocolVersion': 1, 'hostPackageFenceProtocolVersion': 1,
                  'binarySha256': binary, 'sourceManifestSha256': source,
                  'localReadCommands': ['installation-maintenance-check', 'installation-status', 'installation-credential']}
    payload['/usr/share/kura-management-console/local-command-provenance.json'] = json.dumps(provenance).encode()
    package = {'key': 'unit-release', 'package': 'kura-management-console', 'version': '0.1.0~native-unit-1',
               'architecture': 'arm64', 'archiveSha256': digest(b'unit-deb'), 'archiveSize': len(b'unit-deb'),
               'binarySha256': binary, 'sourceManifestSha256': source,
               'maintenanceProtocolVersion': 1, 'hostPackageFenceProtocolVersion': 1,
               'files': [{'path': name, 'sha256': digest(data), 'mode': 0o755 if name.startswith('/usr/bin/') else 0o644}
                         for name, data in payload.items()]}
    return package, payload


class CatalogTests(unittest.TestCase):
    def test_approved_catalog_requires_complete_fixed_namespace_and_protocol(self):
        package, _ = release()
        self.assertEqual(NativePackages.validate_catalog({'schemaVersion': 1, 'packages': [package]}), [package])
        for change in [lambda p: p.update(package='sudo'), lambda p: p.update(archiveSha256='changed'),
                       lambda p: p.update(hostPackageFenceProtocolVersion=0), lambda p: p.update(hostPackageFenceProtocolVersion=True), lambda p: p.update(archiveSize=0),
                       lambda p: p['files'].append({'path': '/usr/bin/sudo', 'sha256': 'a'*64, 'mode': 0o755}),
                       lambda p: p['files'].pop(), lambda p: p['files'].append(p['files'][0])]:
            bad = copy.deepcopy(package); change(bad)
            with self.assertRaises(RuntimeError):
                NativePackages.validate_catalog({'schemaVersion': 1, 'packages': [bad]})
        with self.assertRaises(RuntimeError):
            NativePackages.validate_catalog({'schemaVersion': 1, 'packages': [package, package]})

    def test_operation_identity_cannot_be_zero_noncanonical_or_a_path(self):
        for value in ['00000000-0000-0000-0000-000000000000', '../other', str(uuid.uuid4()).upper()]:
            with self.assertRaises((RuntimeError, ValueError)):
                NativePackages.identity(value)


@unittest.skipUnless(os.geteuid() == 0, 'requires real root-owned private filesystem fixtures')
class NativeControlTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.root = Path(self.temporary.name)
        self.package, self.payload = release()
        self.calls = []; self.process_state = 'inactive'; self.dpkg_return = 0; self.dpkg_installed = False
        self.after_install = None; self.after_postinst = None
        self.setup = Setup(self.root, self.run_command)
        self.control = NativePackages(self.setup)
        for name in ['/run/systemd/system', '/var/lib']:
            self.setup.path(name).mkdir(parents=True, exist_ok=True)
        self.write(CATALOG, json.dumps({'schemaVersion': 1, 'packages': [self.package]}).encode())
        self.write(PROGRAM, b'unit-controller; never launched')
        self.arch = patch('native_packages.platform.machine', return_value='aarch64'); self.arch.start()
        self.live = patch.object(NativePackages, 'live', return_value=False); self.live.start()
        self.dispatch = patch('native_packages.subprocess.run', side_effect=self.dispatch_worker); self.dispatch.start()

    def tearDown(self):
        self.dispatch.stop(); self.live.stop(); self.arch.stop(); self.temporary.cleanup()

    def write(self, name, data, mode=0o644):
        path = self.setup.path(name); path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(data); path.chmod(mode)
        return path

    def dispatch_worker(self, arguments, **kwargs):
        self.calls.append(arguments)
        if arguments[:1] != ['/usr/bin/systemd-run']:
            raise AssertionError(arguments)
        return subprocess.CompletedProcess(arguments, 0)

    def run_command(self, arguments, **kwargs):
        self.calls.append(arguments)
        if Path(arguments[0]).name == 'systemctl':
            if arguments[1] in ('list-units', 'list-unit-files'):
                return subprocess.CompletedProcess(arguments, 0, '', '')
            if arguments[1] == 'show':
                active = self.process_state
                return subprocess.CompletedProcess(arguments, 0,
                    f'ActiveState={active}\nMainPID={"123" if active=="active" else "0"}\nJob=\n', '')
            raise AssertionError('unexpected systemd mutation: '+str(arguments))
        if arguments[:1] == ['/usr/bin/dpkg-query']:
            if not self.dpkg_installed:
                return subprocess.CompletedProcess(arguments, 1, '', 'unit-only package absent')
            return subprocess.CompletedProcess(arguments, 0,
                f'install ok installed\n{self.package["version"]}\narm64\n', '')
        if arguments[:2] == ['/usr/bin/dpkg', '--install']:
            self.assertEqual(kwargs['env'], {'PATH': '/usr/sbin:/usr/bin:/sbin:/bin', 'DEBIAN_FRONTEND': 'noninteractive'})
            for name, data in self.payload.items():
                self.write(name, data, 0o755 if name.startswith('/usr/bin/') else 0o644)
            self.dpkg_installed = True
            coordinator = HostControl(self.setup)
            active = coordinator.status()['active']
            coordinator.complete(active['operationId'], self.package['binarySha256'], self.package['sourceManifestSha256'])
            # postinst may verify the payload, but it cannot release the package
            # fence before the real package-command return receipt is persisted.
            self.assertTrue(coordinator.status()['activeFencePresent'])
            if self.after_postinst:
                self.after_postinst()
            if self.after_install:
                self.after_install()
            return subprocess.CompletedProcess(arguments, self.dpkg_return)
        raise AssertionError(arguments)

    def upload(self):
        operation = str(uuid.uuid4())
        value = self.control.prepare_upload(operation, self.package['key'])
        self.write(str(self.control.upload_path(operation).relative_to(self.root)), b'unit-deb', 0o600)
        self.assertFalse(value['uploadReady'])
        self.assertTrue(self.control.status(operation)['uploadReady'])
        return operation

    def accepted(self):
        operation = self.upload()
        self.control.install(operation)
        self.assertEqual(self.control.record(operation)['phase'], 'ACCEPTED')
        return operation

    def install_calls(self):
        return [row for row in self.calls if row[:2] == ['/usr/bin/dpkg', '--install']]

    def test_status_is_readonly_and_never_certifies_unobserved_delivery(self):
        operation = str(uuid.uuid4()); before = set(self.root.rglob('*'))
        value = self.control.status(operation)
        self.assertEqual(value['phase'], 'UNOBSERVED')
        self.assertFalse(value['transportNonDeliveryCertified'])
        self.assertEqual(before, set(self.root.rglob('*')))
        self.assertTrue(self.control.overview()['allStopped'])
        self.assertFalse(self.setup.path(BASE).exists())

    def test_absent_core_is_readonly_and_does_not_probe_an_executable_or_create_state(self):
        before = set(self.root.rglob('*'))
        self.assertEqual(self.control.installed_core(), {'state': 'ABSENT'})
        self.assertEqual(before, set(self.root.rglob('*')))
        self.assertTrue(all(row[0] == '/usr/bin/dpkg-query' for row in self.calls))

    def test_dpkg_dependency_placeholder_is_absent_only_without_any_core_payload(self):
        # dpkg creates this selection when another installed package references
        # an as-yet uninstalled core in Replaces/Depends.
        with patch.object(self.setup, 'run', return_value=subprocess.CompletedProcess(
                ['/usr/bin/dpkg-query'], 0, 'unknown ok not-installed\n\n\n', '')):
            self.assertEqual(self.control.installed_core(), {'state': 'ABSENT'})
            self.write('/usr/bin/yofc-management-console', b'partial installation', 0o755)
            self.assertEqual(self.control.installed_core(), {'state': 'UNVERIFIED'})

    def test_installed_core_requires_real_protected_program_source_and_package_metadata(self):
        operation = self.accepted(); self.control.worker(operation)
        self.calls.clear()
        value = self.control.installed_core()
        self.assertEqual(value['state'], 'VERIFIED')
        self.assertEqual(value['binarySha256'], self.package['binarySha256'])
        self.assertEqual(value['version'], self.package['version'])
        self.assertTrue(all(row[0] == '/usr/bin/dpkg-query' for row in self.calls))
        original = self.control.record(operation)
        self.write('/usr/bin/yofc-management-console', b'corrupt program', 0o755)
        self.assertEqual(self.control.installed_core(), {'state': 'UNVERIFIED'})
        self.assertEqual(self.control.record(operation), original)

    def test_partial_installation_and_malformed_provenance_never_enable_first_start(self):
        self.write('/usr/bin/yofc-management-console', b'partial package', 0o755)
        self.assertEqual(self.control.installed_core(), {'state': 'UNVERIFIED'})
        for name, data in self.payload.items():
            self.write(name, data, 0o755 if name.startswith('/usr/bin/') else 0o644)
        self.dpkg_installed = True
        self.write('/usr/share/kura-management-console/local-command-provenance.json', b'[]')
        self.assertEqual(self.control.installed_core(), {'state': 'UNVERIFIED'})
        self.assertFalse(self.setup.path(BASE).exists())

    def test_upload_mismatch_and_symlink_are_rejected_before_install_intent(self):
        operation = self.upload(); path = self.control.upload_path(operation)
        path.write_bytes(b'wrongxxx')
        self.assertFalse(self.control.status(operation)['uploadReady'])
        self.assertTrue(self.control.install(operation)['rejectedBeforeIntent'])
        self.assertNotIn('sealIntent', self.control.record(operation))
        path.unlink(); path.symlink_to('/etc/passwd')
        with self.assertRaises(OSError):
            self.control.status(operation)
        self.assertFalse(self.install_calls())

    def test_original_identity_and_retirement_fence_delayed_requests(self):
        operation = self.upload()
        with self.assertRaises(RuntimeError):
            self.control.prepare_upload(str(uuid.uuid4()), self.package['key'])
        self.assertEqual(self.control.retire(operation)['phase'], 'RETIRED')
        self.assertEqual(self.control.install(operation)['phase'], 'RETIRED')
        self.assertEqual(self.control.prepare_upload(operation, self.package['key'])['phase'], 'RETIRED')
        self.assertTrue(self.control.upload_path(operation).exists())
        unseen = str(uuid.uuid4()); self.control.retire(unseen)
        self.assertEqual(self.control.prepare_upload(unseen, self.package['key'])['phase'], 'RETIRED')
        self.assertFalse(self.install_calls())

    def test_active_centers_are_not_stopped_and_rejection_precedes_acceptance(self):
        operation = self.upload(); self.process_state = 'active'
        self.assertTrue(self.control.install(operation)['rejectedBeforeIntent'])
        self.assertNotIn('sealIntent', self.control.record(operation))
        self.assertFalse(self.control.upload_path(operation, accepted=True).exists())
        self.assertFalse(any(len(row)>1 and row[1] in ['stop','start','restart'] for row in self.calls))

    def test_unsupported_installed_core_is_not_upgraded_online(self):
        operation = self.upload()
        self.write('/usr/bin/yofc-management-console', b'old protocol-zero executable', 0o755)
        self.assertTrue(self.control.install(operation)['rejectedBeforeIntent'])
        self.assertNotIn('launchIntent', self.control.record(operation))
        self.assertFalse(self.install_calls())

    def test_accepted_package_is_a_distinct_inode_and_dispatch_is_not_repeated(self):
        operation = self.accepted()
        original = self.control.upload_path(operation); sealed = self.control.upload_path(operation, accepted=True)
        self.assertNotEqual(original.stat().st_ino, sealed.stat().st_ino)
        original.write_bytes(b'delayed upload replacement')
        self.assertEqual(sealed.read_bytes(), b'unit-deb')
        self.assertEqual(self.control.install(operation)['phase'], 'UNKNOWN')
        self.assertEqual(len([x for x in self.calls if x[0] == '/usr/bin/systemd-run']), 1)

    def test_successful_install_keeps_services_stopped_and_never_runs_twice(self):
        operation = self.accepted()
        result = self.control.worker(operation)
        self.assertEqual(result['phase'], 'APPLIED'); self.assertFalse(result['finalizationRequired'])
        self.assertIsNone(HostControl(self.setup).status()['active'])
        self.assertEqual(self.control.install(operation)['phase'], 'APPLIED')
        with self.assertRaises(RuntimeError):
            self.control.worker(operation)
        self.assertEqual(len(self.install_calls()), 1)
        self.assertFalse(any(len(row)>1 and row[1] in ['stop','start','restart','enable'] for row in self.calls))

    def test_missing_command_receipt_stays_unknown_despite_installed_files_and_postinst_receipt(self):
        operation = self.accepted()
        def loss():
            raise subprocess.TimeoutExpired('unit-only command', 1)
        self.after_postinst = loss
        with self.assertRaises(subprocess.TimeoutExpired):
            self.control.worker(operation)
        self.assertEqual(self.control.status(operation)['phase'], 'UNKNOWN')
        self.assertNotIn('installReceipt', self.control.record(operation))
        self.assertTrue(HostControl(self.setup).status()['activeFencePresent'])
        with self.assertRaises(RuntimeError): self.control.verify(operation)
        with self.assertRaises(RuntimeError): self.control.finalize(operation)
        with self.assertRaises(RuntimeError): self.control.retire(operation)
        with self.assertRaises(RuntimeError): self.control.prepare_upload(str(uuid.uuid4()), self.package['key'])
        self.assertEqual(len(self.install_calls()), 1)

    def test_actual_success_receipt_allows_explicit_payload_verification_without_reinstalling(self):
        operation = self.accepted(); name = '/usr/share/kura-management-console/react/assets/unit.js'
        self.after_install = lambda: self.write(name, b'corrupt-after-postinst')
        with self.assertRaises(RuntimeError): self.control.worker(operation)
        self.assertEqual(self.control.status(operation)['phase'], 'UNKNOWN')
        self.assertTrue(self.control.status(operation)['canVerify'])
        with self.assertRaises(RuntimeError): self.control.verify(operation)
        self.write(name, self.payload[name])
        self.assertEqual(self.control.verify(operation)['phase'], 'APPLIED')
        self.assertEqual(len(self.install_calls()), 1)

    def test_lost_release_can_only_finish_an_applied_original_without_install_or_start(self):
        operation = self.accepted()
        with patch.object(HostControl, 'release_package', side_effect=OSError('unit-only pointer loss')):
            with self.assertRaises(OSError): self.control.worker(operation)
        self.assertEqual(self.control.status(operation)['phase'], 'APPLIED')
        self.assertTrue(self.control.status(operation)['finalizationRequired'])
        self.assertTrue(HostControl(self.setup).status()['activeFencePresent'])
        self.assertFalse(self.control.finalize(operation)['finalizationRequired'])
        self.assertEqual(len(self.install_calls()), 1)

    def test_controller_change_after_acceptance_never_executes_dpkg(self):
        operation = self.accepted(); self.write(PROGRAM, b'replaced-controller')
        with self.assertRaises(RuntimeError): self.control.worker(operation)
        self.assertEqual(self.control.status(operation)['phase'], 'UNKNOWN')
        self.assertFalse(self.install_calls())

    def test_failed_dpkg_keeps_the_original_fence_and_never_rolls_back_or_retries(self):
        operation = self.accepted(); self.dpkg_return = 2
        with self.assertRaises(RuntimeError): self.control.worker(operation)
        self.assertEqual(self.control.status(operation)['phase'], 'UNKNOWN')
        self.assertTrue(HostControl(self.setup).status()['activeFencePresent'])
        self.assertEqual(self.control.install(operation)['phase'], 'UNKNOWN')
        self.assertEqual(len(self.install_calls()), 1)

    def test_applied_label_without_the_actual_command_receipt_cannot_unblock_another_id(self):
        operation = self.accepted(); self.control.worker(operation)
        value = self.control.record(operation); value.pop('installReceipt'); self.control.save(value)
        self.assertEqual(self.control.status(operation)['phase'], 'UNKNOWN')
        self.assertFalse(self.control.status(operation)['canVerify'])
        with self.assertRaises(RuntimeError): self.control.prepare_upload(str(uuid.uuid4()), self.package['key'])
        with self.assertRaises(RuntimeError): self.control.verify(operation)
        self.assertEqual(len(self.install_calls()), 1)

    def test_another_package_hold_is_never_presented_as_the_old_applied_jobs_finalization(self):
        operation = self.accepted(); self.control.worker(operation)
        other = str(uuid.uuid4()); maintenance = str(uuid.uuid4())
        HostControl(self.setup).write('package-install.json', {'schemaVersion':1,'operationId':maintenance,
            'packageOperationId':other,'packageSha256':self.package['archiveSha256'],
            'binarySha256':self.package['binarySha256'],'sourceManifestSha256':self.package['sourceManifestSha256']}, new=True)
        self.assertFalse(self.control.status(operation)['finalizationRequired'])
        with self.assertRaises(RuntimeError): self.control.finalize(operation)
        self.assertEqual(HostControl(self.setup).package_hold()['packageOperationId'], other)
        self.assertEqual(len(self.install_calls()), 1)


if __name__ == '__main__':
    unittest.main()
