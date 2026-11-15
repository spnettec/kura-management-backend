import hashlib
import io
import json
import os
from pathlib import Path
import subprocess
import tarfile
import tempfile
import unittest
from unittest.mock import patch
import build
import installer
from setup import Setup


class IndependentInstallerPackageTests(unittest.TestCase):
    def test_runtime_pins_retain_assets_but_allow_package_manager_documentation_exclusions(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            paths = ['usr/bin/yofc-management-console',
                     'usr/lib/systemd/system/kura-management-console.service',
                     'usr/share/kura-management-console/react/assets/help.js',
                     'usr/share/doc/kura-management-console/first-setup.md']
            for name in paths:
                target = root/name; target.parent.mkdir(parents=True, exist_ok=True)
                target.write_text(name)
            required, documentation = build.installed_payload_pins(root)
            self.assertEqual({row['path'] for row in required}, {'/'+name for name in paths[:3]})
            self.assertEqual([row['path'] for row in documentation], ['/'+paths[3]])
            (root/paths[3]).unlink()
            self.assertEqual(build.installed_payload_pins(root)[0], required)
            (root/paths[2]).unlink()
            self.assertNotEqual(build.installed_payload_pins(root)[0], required)

    def test_actual_package_has_no_server_units_state_or_maintainer_scripts(self):
        with tempfile.TemporaryDirectory() as temporary:
            root=Path(temporary);staged=root/'source';build.stage(staged)
            result=build.installer_deb('0.1.0~v52-readiness20261114-6',root/'package',staged/'source-manifest.json')
            package=Path(result['package'])
            control=tarfile.open(fileobj=io.BytesIO(subprocess.check_output(['dpkg-deb','--ctrl-tarfile',str(package)])),mode='r:')
            self.assertEqual({member.name.removeprefix('./') for member in control if member.isfile()},{'control'})
            archive=tarfile.open(fileobj=io.BytesIO(subprocess.check_output(['dpkg-deb','--fsys-tarfile',str(package)])),mode='r:')
            names={member.name.removeprefix('./') for member in archive if member.isfile()}
            self.assertFalse(any(name.startswith(('var/','etc/','usr/lib/systemd/','usr/bin/')) for name in names))
            self.assertIn('usr/lib/kura-management-console/host_control.py',names)
            self.assertIn('usr/share/kura-management-console-installer/provenance-v1.json',names)
            self.assertEqual(result['maintainerScripts'],[])
            self.assertFalse(result['serverStopOrStartRequired'])
            metadata=subprocess.check_output(['dpkg-deb','--field',str(package)],text=True)
            self.assertIn('Architecture: all',metadata)
            self.assertGreater(int(subprocess.check_output(['dpkg-deb','--field',str(package),'Installed-Size'],text=True)),0)
            self.assertIn('Replaces: kura-management-console (<< 0.1.0~v52-readiness20261114-6)',metadata)
            self.assertNotIn('Depends: kura-management-console\n',metadata)
            dependencies=subprocess.check_output(['dpkg-deb','--field',str(package),'Depends'],text=True)
            self.assertIn('cockpit',dependencies)
            self.assertIn('xdg-utils',dependencies)
            names_of_dependencies={entry.strip().split()[0] for entry in dependencies.split(',')}
            self.assertTrue({'cockpit-ws','cockpit-bridge','cockpit-system','xdg-utils','pkexec',
                             'python3-gi','gir1.2-gtk-3.0','dbus-user-session'}<=names_of_dependencies)
            self.assertNotIn('cockpit',names_of_dependencies)
            desktop='usr/share/applications/com.yofc.KuraConsoleSetup.desktop'
            self.assertIn(desktop,names)
            contents=archive.extractfile('./'+desktop).read().decode()
            self.assertIn('Terminal=false',contents)
            self.assertIn('Exec=/usr/bin/python3 /usr/lib/kura-management-console/host_ui.py gui',contents)
            self.assertIn('Name[zh_CN]=YOFC 管理中心安装设置',contents)
            self.assertEqual(result['hostUiBootstrapProtocolVersion'],1)
            self.assertEqual(len(result['criticalFiles']),18)
            self.assertTrue({'usr/lib/kura-management-console/host_ui.py','usr/sbin/kura-console-host-ui',
                             'usr/share/polkit-1/actions/com.yofc.console.host-ui.policy'}<=names)
            import xml.etree.ElementTree as ET
            policy=ET.fromstring(archive.extractfile('./usr/share/polkit-1/actions/com.yofc.console.host-ui.policy').read())
            action=policy.find('action')
            self.assertEqual([node.text for node in action.find('defaults')],['auth_admin']*3)
            self.assertEqual(action.find('annotate').attrib['key'],'org.freedesktop.policykit.exec.path')
            self.assertEqual(action.find('annotate').text,'/usr/sbin/kura-console-host-ui')
            self.assertFalse(any(node.attrib.get('key')=='org.freedesktop.policykit.exec.allow_gui' for node in action.findall('annotate')))
            self.assertIn('usr/lib/kura-management-console/native_packages.py',names)
            self.assertIn('usr/share/kura-management-console-installer/packages.json',names)
            self.assertIn('usr/share/cockpit/yofc-console-setup/native-packages.js',names)
            if os.geteuid()==0:
                service=Setup(root/'package/root')
                with patch.object(installer,'Setup',return_value=service):
                    record=installer.verify_installer_payload()
                    self.assertEqual(record['hostUiBootstrapProtocolVersion'],1)
                    provenance=service.path('/usr/share/kura-management-console-installer/provenance-v1.json')
                    record['hostUiBootstrapProtocolVersion']=True;provenance.write_text(json.dumps(record))
                    with self.assertRaises(RuntimeError):installer.verify_installer_payload()
                    record['hostUiBootstrapProtocolVersion']=1;provenance.write_text(json.dumps(record))
                    service.path('/usr/share/polkit-1/actions/com.yofc.console.host-ui.policy').write_text('changed policy')
                    with self.assertRaisesRegex(RuntimeError,'differs'):installer.verify_installer_payload()

    def test_explicit_headless_package_has_no_desktop_entry_or_browser_dependencies(self):
        with tempfile.TemporaryDirectory() as temporary:
            root=Path(temporary);staged=root/'source';build.stage(staged)
            result=build.installer_deb('0.1.0~headless-1',root/'package',staged/'source-manifest.json',with_cockpit=False)
            package=Path(result['package']);archive=tarfile.open(fileobj=io.BytesIO(subprocess.check_output(['dpkg-deb','--fsys-tarfile',str(package)])),mode='r:')
            self.assertFalse(any(member.name.startswith('./usr/share/applications/') for member in archive))
            dependencies=subprocess.check_output(['dpkg-deb','--field',str(package),'Depends'],text=True)
            self.assertNotIn('cockpit',dependencies);self.assertNotIn('xdg-utils',dependencies)
            self.assertNotIn('pkexec',dependencies);self.assertNotIn('python3-gi',dependencies)
            self.assertNotIn('dbus-user-session',dependencies)
            self.assertIsNone(result['desktopEntrypoint'])
            self.assertEqual(result['hostUiBootstrapProtocolVersion'],0)
            self.assertFalse(any('host_ui.py' in member.name or 'kura-console-host-ui' in member.name
                                 or member.name.startswith('./usr/share/polkit-1/') for member in archive))
            self.assertEqual(result['maintainerScripts'],[])

    @unittest.skipUnless(os.geteuid()==0,'root-owned provenance fixture')
    def test_declared_desktop_provenance_requires_its_exact_protected_launcher(self):
        with tempfile.TemporaryDirectory() as temporary:
            service=Setup(Path(temporary));digest=lambda data:hashlib.sha256(data).hexdigest();rows=[]
            for name in installer.INSTALLER_FILES:
                path=service.path(name);path.parent.mkdir(parents=True,exist_ok=True);path.write_bytes(name.encode());path.chmod(0o644)
                rows.append({'path':name,'sha256':digest(path.read_bytes())})
            base=service.path('/usr/share/kura-management-console-installer');base.mkdir(parents=True,exist_ok=True)
            source=base/'source-manifest.json';source.write_text('{}')
            record={'schemaVersion':1,'maintenanceProtocolVersion':1,'desktopIntegration':True,'sourceManifestSha256':digest(source.read_bytes()),'files':rows}
            provenance=base/'provenance-v1.json';provenance.write_text(json.dumps(record))
            with patch.object(installer,'Setup',return_value=service):
                with self.assertRaises(RuntimeError):installer.verify_installer_payload()
                name=next(iter(installer.DESKTOP_FILES));launcher=service.path(name);launcher.parent.mkdir(parents=True,exist_ok=True)
                launcher.write_bytes(b'unit-only launcher');launcher.chmod(0o644);rows.append({'path':name,'sha256':digest(launcher.read_bytes())});provenance.write_text(json.dumps(record))
                self.assertEqual(installer.verify_installer_payload(),record)
                launcher.write_bytes(b'changed launcher')
                with self.assertRaises(RuntimeError):installer.verify_installer_payload()

    @unittest.skipUnless(os.geteuid()==0,'root-owned provenance fixture')
    def test_host_tools_verify_all_fixed_files_and_their_separate_source_manifest(self):
        with tempfile.TemporaryDirectory() as temporary:
            service=Setup(Path(temporary));sha=lambda data:hashlib.sha256(data).hexdigest()
            rows=[]
            for name in installer.INSTALLER_FILES:
                path=service.path(name);path.parent.mkdir(parents=True,exist_ok=True)
                data=('unit-only protected installer '+name).encode();path.write_bytes(data);path.chmod(0o644)
                rows.append({'path':name,'sha256':sha(data)})
            base=service.path('/usr/share/kura-management-console-installer');base.mkdir(parents=True,exist_ok=True)
            source=base/'source-manifest.json';source.write_text('{}')
            record={'schemaVersion':1,'maintenanceProtocolVersion':1,'sourceManifestSha256':sha(source.read_bytes()),'files':rows}
            (base/'provenance-v1.json').write_text(json.dumps(record))
            with patch.object(installer,'Setup',return_value=service):
                self.assertEqual(installer.verify_installer_payload(),record)
                target=service.path('/usr/lib/kura-management-console/installer.py');target.write_bytes(b'changed')
                with self.assertRaisesRegex(RuntimeError,'differs'):installer.verify_installer_payload()

if __name__=='__main__':unittest.main()
