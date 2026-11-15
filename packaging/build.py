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

ROOT = Path(__file__).resolve().parents[1]
PACKAGING = ROOT / 'packaging'


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def source_files(root):
    root = root.resolve()
    sources = []
    sources += [root / 'Cargo.toml', root / 'Cargo.lock', root / 'rust-toolchain.toml']
    sources += sorted((root / 'crates').glob('**/*.rs'))
    sources += sorted((root / 'crates').glob('**/Cargo.toml'))
    sources += sorted((root / 'crates').glob('**/*.proto'))
    sources += sorted((root / 'crates/console/tests/fixtures').glob('*.pem'))
    sources += sorted((root / 'migrations').glob('*.sql'))
    react = root / 'frontend/react'
    sources += [react / 'package.json', react / 'package-lock.json', react / 'index.html', react / 'tsconfig.json', react / 'vite.config.ts']
    sources += sorted((react / 'src').glob('**/*'))
    sources += sorted((react / 'tests').glob('**/*.ts'))
    sources += [root / 'packaging/kura-management-console.service', root / 'packaging/kura-management-console@.service',
                root / 'packaging/console.env.example', root / 'packaging/console-instance.env.example',
                root / 'packaging/setup.py', root / 'packaging/lifecycle.py', root / 'packaging/build.py']
    return sources


def stage(destination):
    if destination.exists():
        raise RuntimeError('Keep prior staged source; select a new destination')
    records = []
    destination.mkdir(parents=True)
    for source in source_files(ROOT):
        if source.is_symlink() or not source.is_file():
            if source.suffix in {'.rs', '.sql', '.ts', '.tsx', '.proto'} or source.name in {'Cargo.toml', 'package.json', 'package-lock.json', 'index.html', 'tsconfig.json', 'vite.config.ts'}:
                raise RuntimeError(f'Missing release source: {source}')
            continue
        relative = source.relative_to(ROOT)
        if any(part in {'target', 'node_modules', 'dist', '.phase1-dev', '.git'} for part in relative.parts):
            raise RuntimeError(f'Release source includes generated path: {relative}')
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
    if provenance.get('siblingSourceRequired') is not False:
        raise RuntimeError('Self-contained source provenance required')
    web = web_manifest(web_dist)
    if destination.exists():
        raise RuntimeError('Keep prior package output; select a new destination')
    destination.mkdir(parents=True)
    payload = destination / 'root'
    entries = [
        (binary, 'usr/bin/yofc-management-console', 0o755),
        (PACKAGING / 'setup.py', 'usr/lib/kura-management-console/setup.py', 0o644),
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
    wrapper = payload / 'usr/sbin/kura-console-setup'
    wrapper.parent.mkdir(parents=True)
    wrapper.write_text('#!/bin/sh\nexec /usr/bin/python3 /usr/lib/kura-management-console/setup.py "$@"\n')
    wrapper.chmod(0o755)
    control = payload / 'DEBIAN'
    control.mkdir()
    (control / 'control').write_text(f'''Package: kura-management-console
Version: {version}
Architecture: {architecture}
Section: admin
Priority: optional
Maintainer: YOFC Kura maintainers
Pre-Depends: python3
Depends: libc6 (>= 2.39), libgcc-s1, postgresql-client, systemd, passwd, adduser
Description: Independent YOFC Rust multi-gateway management console
 Rust HTTPS/gRPC console with React payload and externally operated PostgreSQL.
''')
    setup = (PACKAGING / 'setup.py').read_text().split("if __name__ == '__main__':")[0]
    lifecycle = (PACKAGING / 'lifecycle.py').read_text()
    for name in ['preinst', 'postinst', 'prerm', 'postrm']:
        script = control / name
        script.write_text('#!/usr/bin/python3\nSCRIPT_NAME = ' + repr(name) + '\n' + setup + '\n' + lifecycle)
        script.chmod(0o755)
    for path in payload.rglob('*'):
        if path.is_dir():
            path.chmod(0o755)
        elif not path.stat().st_mode & 0o111:
            path.chmod(0o644)
    package = destination / f'kura-management-console_{version}_{architecture}.deb'
    subprocess.run(['dpkg-deb', '--root-owner-group', '--build', str(payload), str(package)], check=True)
    receipt = {'package': str(package), 'sha256': sha(package), 'binarySha256': expected,
               'architecture': architecture, 'maximumGlibc': '.'.join(map(str, max(required))),
               'webFiles': len(web), 'sourceManifestSha256': sha(source_manifest)}
    (destination / 'package.json').write_text(json.dumps(receipt, indent=2) + '\n')
    return receipt


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
    args = parser.parse_args()
    if args.action == 'source':
        result = stage(args.destination)
    else:
        result = deb(args.binary, args.sha256, args.architecture, args.version, args.destination, args.source_manifest, args.web_dist)
    print(json.dumps(result, indent=2))
