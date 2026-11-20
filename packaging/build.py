#!/usr/bin/env python3
"""Stage checksummed console sources or package a validated Linux release."""
import argparse
import hashlib
import json
from pathlib import Path
import re
import shutil
import struct
import subprocess
from native_packages import NativePackages

ROOT = Path(__file__).resolve().parents[1]
PACKAGING = ROOT / 'packaging'


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def source_files(root):
    root = root.resolve()
    sources = []
    sources += [root / 'Cargo.toml', root / 'Cargo.lock', root / 'rust-toolchain.toml']
    sources += [root / 'docs/openapi-v1.json', root/'docs/console-first-setup-platform-v1.md']
    sources += sorted((root / 'crates').glob('**/*.rs'))
    sources += sorted((root / 'crates').glob('**/Cargo.toml'))
    sources += sorted((root / 'crates').glob('**/*.proto'))
    sources += sorted((root / 'crates/console/tests/fixtures').rglob('*'))
    sources += sorted((root / 'migrations').glob('*.sql'))
    react = root / 'frontend/react'
    sources += [react / 'package.json', react / 'package-lock.json', react / 'index.html', react / 'tsconfig.json', react / 'vite.config.ts']
    sources += [path for path in sorted(react.glob('*.config.ts')) if path not in sources]
    sources += sorted((react / 'src').glob('**/*'))
    sources += sorted((react / 'tests').glob('**/*.ts'))
    sources += [root / 'packaging/kura-management-console.service', root / 'packaging/kura-management-console@.service',
                root / 'packaging/console.env.example', root / 'packaging/console-instance.env.example',
                root / 'packaging/setup.py', root / 'packaging/lifecycle.py', root / 'packaging/build.py',
                    root / 'packaging/installer.py', root/'packaging/host_control.py',root/'packaging/docker_control.py',root/'packaging/docker_maintenance.py',root/'packaging/native_packages.py',root/'packaging/agent_control.py',root/'packaging/images.json']
    sources += sorted((root/'packaging').glob('test_*.py'))
    sources += sorted((root/'packaging/cockpit').glob('*'))
    sources += [root/'packaging/desktop/com.yofc.KuraConsoleSetup.desktop',
                root/'packaging/desktop/com.yofc.console.host-ui.policy', root/'packaging/host_ui.py']
    sources += [root/'docker'/name for name in ['Dockerfile','entrypoint.sh','compose.yml','compose-legacy.yml','README.md']]
    return sources


def release_sources():
    result = []
    for source in source_files(ROOT):
        relative = source.relative_to(ROOT)
        if source.is_symlink() or any(parent.is_symlink() for parent in source.parents if parent != ROOT and ROOT in parent.parents):
            raise RuntimeError('Source symlinks are not release inputs: ' + str(relative))
        if not source.is_file():
            if source.suffix in {'.rs', '.sql', '.ts', '.tsx', '.proto', '.desktop', '.policy', '.py'} or source.name in {'Cargo.toml', 'package.json', 'package-lock.json', 'index.html', 'tsconfig.json', 'vite.config.ts'}:
                raise RuntimeError('Missing release source: ' + str(source))
            continue
        if any(part in {'target', 'node_modules', 'dist', '.phase1-dev', '.git'} for part in relative.parts):
            raise RuntimeError('Release source includes generated path: ' + str(relative))
        result.append(source)
    if len(result) != len(set(result)):
        raise RuntimeError('Duplicate release source path')
    return result


def verify_source_manifest(provenance):
    if provenance.get('schemaVersion') != 1 or provenance.get('siblingSourceRequired') is not False:
        raise RuntimeError('Self-contained source provenance required')
    rows = provenance.get('sourceFiles', [])
    records = {row['path']: row['sha256'] for row in rows}
    actual = {str(path.relative_to(ROOT)): sha(path) for path in release_sources()}
    if len(records) != len(rows) or records != actual:
        raise RuntimeError('Release source differs from staged provenance')


def stage(destination):
    if destination.exists():
        raise RuntimeError('Keep prior staged source; select a new destination')
    records = []
    destination.mkdir(parents=True)
    for source in release_sources():
        relative = source.relative_to(ROOT)
        target = destination / relative
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(source, target)
        records.append({'path': str(relative), 'sha256': sha(source)})
    result = {'schemaVersion': 1, 'siblingSourceRequired': False, 'build': 'frontend/react npm ci && npm run build; cargo build --release',
              'sourceFiles': records}
    (destination / 'source-manifest.json').write_text(json.dumps(result, indent=2) + '\n')
    return result


def web_manifest(web_dist):
    web_dist = web_dist.resolve()
    if not (web_dist / 'index.html').is_file():
        raise RuntimeError('Web distribution is missing index.html')
    records = []
    for path in sorted(web_dist.rglob('*')):
        if path.is_dir():
            continue
        if path.is_symlink() or not path.is_file():
            raise RuntimeError('Web distribution must contain only regular files and directories')
        records.append({'path': str(path.relative_to(web_dist)), 'sha256': sha(path)})
    if not any(record['path'].startswith('assets/') and record['path'].endswith('.js') for record in records):
        raise RuntimeError('Web distribution has no JavaScript assets')
    return records


def deb(binary, expected, architecture, version, destination, source_manifest, web_dist):
    if not re.fullmatch(r'[0-9][A-Za-z0-9.+:~\-]*', version):
        raise RuntimeError('Invalid package version')
    if sha(binary) != expected:
        raise RuntimeError('Binary digest mismatch')
    with binary.open('rb') as source:
        header = source.read(20)
    if len(header) != 20 or header[:6] != b'\x7fELF\x02\x01' or struct.unpack('<H', header[18:20])[0] != {'arm64': 183, 'amd64': 62}[architecture]:
        raise RuntimeError('Expected a matching Linux ELF64 binary')
    versions = subprocess.check_output(['readelf', '--version-info', str(binary)], text=True)
    required = {(int(a), int(b)) for a, b in re.findall(r'GLIBC_(\d+)\.(\d+)', versions)}
    if not required or max(required) > (2, 39):
        raise RuntimeError('Binary does not satisfy the Ubuntu 24.04 glibc floor')
    provenance = json.loads(source_manifest.read_text())
    verify_source_manifest(provenance)
    web = web_manifest(web_dist)
    if destination.exists():
        raise RuntimeError('Keep prior package output; select a new destination')
    destination.mkdir(parents=True)
    payload = destination / 'root'
    entries = [
        (binary, 'usr/bin/yofc-management-console', 0o755),
        (ROOT/'docs/console-first-setup-platform-v1.md', 'usr/share/doc/kura-management-console/first-setup.md', 0o644),
        (PACKAGING / 'kura-management-console.service', 'usr/lib/systemd/system/kura-management-console.service', 0o644),
        (PACKAGING / 'kura-management-console@.service', 'usr/lib/systemd/system/kura-management-console@.service', 0o644),
        (PACKAGING / 'console.env.example', 'usr/share/doc/kura-management-console/console.env.example', 0o644),
        (PACKAGING / 'console-instance.env.example', 'usr/share/doc/kura-management-console/console-instance.env.example', 0o644),
        (source_manifest, 'usr/share/kura-management-console/source-manifest.json', 0o644),
    ]
    for source, path, mode in entries:
        target = payload / path
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(source, target)
        target.chmod(mode)
    web_root = payload / 'usr/share/kura-management-console/react'
    web_root.mkdir(parents=True)
    for record in web:
        target = web_root / record['path']
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(web_dist / record['path'], target)
        target.chmod(0o644)
    marker = payload / 'usr/share/kura-management-console/package-v1'
    marker.write_text('kura-management-console-v1\n')
    (payload/'usr/share/kura-management-console/local-command-provenance.json').write_text(json.dumps({
          'schemaVersion':1,'maintenanceProtocolVersion':1,'hostPackageFenceProtocolVersion':1,'binarySha256':expected,'sourceManifestSha256':sha(source_manifest),
        'localReadCommands':['installation-credential','installation-maintenance-check','installation-status']},indent=2)+'\n')
    control = payload / 'DEBIAN'
    control.mkdir()
    (control / 'control').write_text(f'''Package: kura-management-console
Version: {version}
Architecture: {architecture}
Section: admin
Priority: optional
Maintainer: YOFC Kura maintainers
Pre-Depends: python3
Depends: libc6 (>= 2.39), libgcc-s1, postgresql-client, systemd, passwd, adduser, openssl, ca-certificates, kura-management-console-installer (>= {version})
Description: Independent YOFC Rust multi-gateway management console
 Rust HTTPS/gRPC console with React payload and externally operated PostgreSQL.
''')
    setup = (PACKAGING / 'setup.py').read_text().split("if __name__ == '__main__':")[0].replace('from host_control import HostControl\n','')
    host_control = (PACKAGING/'host_control.py').read_text()
    lifecycle = (PACKAGING / 'lifecycle.py').read_text()
    for name in ['preinst', 'postinst', 'prerm', 'postrm']:
        script = control / name
        script.write_text('#!/usr/bin/python3\nSCRIPT_NAME = '+repr(name)+'\nPACKAGE_BINARY_SHA256 = '+repr(expected)+
                          '\nPACKAGE_SOURCE_SHA256 = '+repr(sha(source_manifest))+
                          '\nPACKAGE_VERSION = '+repr(version)+'\n'+host_control+'\n'+setup+'\n'+lifecycle)
        script.chmod(0o755)
    for path in payload.rglob('*'):
        if path.is_dir():
            path.chmod(0o755)
        elif not path.stat().st_mode & 0o111:
            path.chmod(0o644)
    package = destination / f'kura-management-console_{version}_{architecture}.deb'
    subprocess.run(['dpkg-deb', '--root-owner-group', '--build', str(payload), str(package)], check=True)
    required_files, documentation_files = installed_payload_pins(payload)
    receipt = {'package': str(package), 'packageName':'kura-management-console','version':version,
               'archiveSize':package.stat().st_size,'sha256': sha(package), 'binarySha256': expected,
               'architecture': architecture, 'maximumGlibc': '.'.join(map(str, max(required))),
               'webFiles': len(web), 'sourceManifestSha256': sha(source_manifest),
               'maintenanceProtocolVersion':1,'hostPackageFenceProtocolVersion':1,
               'files':required_files, 'documentationFiles':documentation_files}
    (destination / 'package.json').write_text(json.dumps(receipt, indent=2) + '\n')
    return receipt


def installed_payload_pins(payload):
    """dpkg may omit documentation through a host's path-exclude policy.

    Bind it in the archive receipt, separately from mandatory runtime files.
    Existing operation records retain their original exact requirements.
    """
    required, documentation = [], []
    for path in sorted((payload/'usr').rglob('*')):
        if path.is_symlink():
            raise RuntimeError('Release payload cannot contain a symlink')
        if not path.is_file():
            continue
        row = {'path': '/'+str(path.relative_to(payload)), 'sha256': sha(path),
               'mode': path.stat().st_mode & 0o777}
        target = documentation if row['path'].startswith('/usr/share/doc/kura-management-console/') else required
        target.append(row)
    return required, documentation


def installer_deb(version,destination,source_manifest,native_receipts=(),with_cockpit=True):
    """Host tools can be repaired while a server remains durably frozen.

    No server binaries, units, state, migrations or maintainer scripts belong to
    this package. Replaces transfers only files owned by the old monolithic
    prototype; future console packages depend on this package instead.
    """
    if not re.fullmatch(r'[0-9][A-Za-z0-9.+:~\-]*',version):raise RuntimeError('Invalid package version')
    if destination.exists():raise RuntimeError('Keep prior package output; select a new destination')
    manifest=json.loads(source_manifest.read_text())
    if manifest.get('siblingSourceRequired') is not False:raise RuntimeError('Self-contained source provenance required')
    staged={row['path']:row['sha256'] for row in manifest['sourceFiles']}
    sources=[PACKAGING/name for name in ['setup.py','installer.py','host_control.py','docker_control.py','docker_maintenance.py','native_packages.py','agent_control.py','images.json']]
    sources+=sorted((PACKAGING/'cockpit').glob('*'))
    if with_cockpit:
        sources+=[PACKAGING/'desktop/com.yofc.KuraConsoleSetup.desktop',
                  PACKAGING/'desktop/com.yofc.console.host-ui.policy', PACKAGING/'host_ui.py']
    for source in sources:
        if source.is_symlink() or not source.is_file() or staged.get(str(source.relative_to(ROOT)))!=sha(source):
            raise RuntimeError('Host installer source differs from its staged provenance')
    destination.mkdir(parents=True);payload=destination/'root';critical=[]
    for source in sources:
        relative=Path('usr/share/polkit-1/actions')/source.name if source.suffix=='.policy' else \
            Path('usr/share/applications')/source.name if source.parent.name=='desktop' else \
            Path('usr/share/cockpit/yofc-console-setup')/source.name if source.parent.name=='cockpit' else \
            Path('usr/share/kura-management-console-installer/images.json') if source.name=='images.json' else Path('usr/lib/kura-management-console')/source.name
        target=payload/relative;target.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(source,target);target.chmod(0o644)
        critical.append({'path':'/'+str(relative),'sha256':sha(target)})
    wrapper=payload/'usr/sbin/kura-console-setup';wrapper.parent.mkdir(parents=True)
    wrapper.write_text('#!/bin/sh\nexec /usr/bin/python3 /usr/lib/kura-management-console/setup.py "$@"\n');wrapper.chmod(0o755)
    critical.append({'path':'/usr/sbin/kura-console-setup','sha256':sha(wrapper)})
    if with_cockpit:
        host_wrapper=payload/'usr/sbin/kura-console-host-ui'
        host_wrapper.write_text('#!/bin/sh\nexec /usr/bin/python3 /usr/lib/kura-management-console/host_ui.py root "$@"\n')
        host_wrapper.chmod(0o755)
        critical.append({'path':'/usr/sbin/kura-console-host-ui','sha256':sha(host_wrapper)})
    share=payload/'usr/share/kura-management-console-installer';share.mkdir(parents=True,exist_ok=True)
    # This generated release catalog is not a build source: otherwise including
    # the core deb's own checksum in its source manifest creates a hash cycle.
    packages=[]
    for file in native_receipts:
        receipt=json.loads(file.read_text());artifact=Path(receipt['package'])
        if not artifact.is_absolute():artifact=file.parent/artifact
        if receipt.get('packageName')!='kura-management-console' or sha(artifact)!=receipt['sha256'] \
                or artifact.stat().st_size!=receipt.get('archiveSize') \
                or receipt.get('maintenanceProtocolVersion')!=1 or receipt.get('hostPackageFenceProtocolVersion')!=1:
            raise RuntimeError('Verified core package receipt and matching artifact required')
        packages.append({'key':'native-'+receipt['architecture']+'-'+receipt['sha256'][:16],
                         'package':receipt['packageName'],'version':receipt['version'],'architecture':receipt['architecture'],
                         'archiveSha256':receipt['sha256'],'archiveSize':receipt['archiveSize'],
                         'binarySha256':receipt['binarySha256'],'sourceManifestSha256':receipt['sourceManifestSha256'],
                         'maintenanceProtocolVersion':1,'hostPackageFenceProtocolVersion':1,'files':receipt['files']})
    NativePackages.validate_catalog({'schemaVersion':1,'packages':packages})
    catalog=share/'packages.json';catalog.write_text(json.dumps({'schemaVersion':1,'packages':packages},indent=2)+'\n')
    critical.append({'path':'/usr/share/kura-management-console-installer/packages.json','sha256':sha(catalog)})
    shutil.copyfile(source_manifest,share/'source-manifest.json')
    (share/'provenance-v1.json').write_text(json.dumps({'schemaVersion':1,'packageVersion':version,
        'maintenanceProtocolVersion':1,'desktopIntegration':with_cockpit,
        'hostUiBootstrapProtocolVersion':1 if with_cockpit else 0,
        'sourceManifestSha256':sha(source_manifest),'files':critical},indent=2)+'\n')
    doc=payload/'usr/share/doc/kura-management-console-installer';doc.mkdir(parents=True)
    shutil.copyfile(ROOT/'docs/console-first-setup-platform-v1.md',doc/'first-setup.md')
    installed_size=sum((path.stat().st_size+1023)//1024 for path in payload.rglob('*') if path.is_file())
    control=payload/'DEBIAN';control.mkdir()
    (control/'control').write_text(f'''Package: kura-management-console-installer
Version: {version}
Architecture: all
Installed-Size: {installed_size}
Section: admin
Priority: optional
Maintainer: YOFC Kura maintainers
Depends: python3, systemd, postgresql-client, passwd, adduser, openssl, ca-certificates, util-linux, coreutils{', cockpit-ws, cockpit-bridge, cockpit-system, sudo, xdg-utils, pkexec, python3-gi, gir1.2-gtk-3.0, dbus-user-session' if with_cockpit else ''}
Suggests: docker.io
Replaces: kura-management-console (<< {version})
Description: Authenticated YOFC console host installation and maintenance tools
 Cockpit tools and root helpers, independently updatable without stopping centers.
''')
    for path in payload.rglob('*'):
        if path.is_dir():path.chmod(0o755)
        elif not path.stat().st_mode&0o111:path.chmod(0o644)
    package=destination/f'kura-management-console-installer_{version}_all.deb'
    subprocess.run(['dpkg-deb','--root-owner-group','--build',str(payload),str(package)],check=True)
    receipt={'package':str(package),'sha256':sha(package),'version':version,'architecture':'all',
             'sourceManifestSha256':sha(source_manifest),'serverStopOrStartRequired':False,
               'maintainerScripts':[],'criticalFiles':critical,'guiEntrypointDependency':with_cockpit,
               'desktopEntrypoint':'com.yofc.KuraConsoleSetup.desktop' if with_cockpit else None,
                 'hostUiBootstrapProtocolVersion':1 if with_cockpit else 0,
               'approvedNativePackages':len(packages),'nativePackageCatalogSha256':sha(catalog)}
    (destination/'package.json').write_text(json.dumps(receipt,indent=2)+'\n');return receipt


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest='action', required=True)
    source = sub.add_parser('source')
    source.add_argument('--destination', type=Path, required=True)
    package = sub.add_parser('deb')
    package.add_argument('--binary', type=Path, required=True)
    package.add_argument('--sha256', required=True)
    package.add_argument('--architecture', choices=['arm64', 'amd64'], required=True)
    package.add_argument('--version', required=True)
    package.add_argument('--destination', type=Path, required=True)
    package.add_argument('--source-manifest', type=Path, required=True)
    package.add_argument('--web-dist', type=Path, required=True)
    installer = sub.add_parser('installer-deb')
    installer.add_argument('--version',required=True)
    installer.add_argument('--destination',type=Path,required=True)
    installer.add_argument('--source-manifest',type=Path,required=True)
    installer.add_argument('--native-package-receipt',type=Path,action='append',default=[])
    installer.add_argument('--without-cockpit',action='store_true',help='Explicit headless tools/repair package; default delivery includes Cockpit')
    args = parser.parse_args()
    if args.action == 'source':
        result = stage(args.destination)
    elif args.action=='installer-deb':
        result=installer_deb(args.version,args.destination,args.source_manifest,args.native_package_receipt,not args.without_cockpit)
    else:
        result = deb(args.binary, args.sha256, args.architecture, args.version, args.destination, args.source_manifest, args.web_dist)
    print(json.dumps(result, indent=2))
