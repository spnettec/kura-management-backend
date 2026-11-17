#!/usr/bin/env python3
# Copyright (c) 2026 YOFC IOT and others
# SPDX-License-Identifier: EPL-2.0
"""Inspect explicit current .deb inputs; never install, start or query a gateway."""
import argparse
import hashlib
import io
import json
from pathlib import Path, PurePosixPath
import subprocess
import tarfile
import zipfile
import xml.etree.ElementTree as ET


WORKSPACE = Path(__file__).resolve().parents[2]


def sha(path):
    digest = hashlib.sha256()
    with path.open('rb') as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b''):
            digest.update(chunk)
    return digest.hexdigest()


def archives(path):
    result = {}
    with path.open('rb') as stream:
        assert stream.read(8) == b'!<arch>\n', path
        while header := stream.read(60):
            assert len(header) == 60 and header[-2:] == b'`\n'
            name = header[:16].decode().strip().rstrip('/')
            length = int(header[48:58])
            if name.startswith(('control.tar', 'data.tar')):
                result[name.split('.', 1)[0]] = stream.read(length)
            else:
                stream.seek(length, 1)
            if length % 2:
                stream.read(1)
    assert set(result) == {'control', 'data'}
    return result


def headers(raw):
    lines = raw.decode('utf-8').replace('\r\n', '\n').replace('\n ', '').splitlines()
    return dict(line.split(': ', 1) for line in lines if ': ' in line)


def clauses(value):
    result, start, quoted = {}, 0, False
    for position, character in enumerate(value + ','):
        if character == '"':
            quoted = not quoted
        elif character == ',' and not quoted:
            item = value[start:position].strip()
            result[item.split(';', 1)[0]] = item.replace(' ', '')
            start = position + 1
    return result


def inspect(path):
    parts = archives(path)
    with tarfile.open(fileobj=io.BytesIO(parts.pop('control')), mode='r:*') as archive:
        control = {member.name.removeprefix('./'): archive.extractfile(member).read()
                   for member in archive if member.isfile()}
    metadata = headers(control['control'])
    package = metadata['Package']
    if package == 'kura-firewall-only':
        name, repository, module = 'firewall', 'kura-networking', 'distrib-firewall'
    elif package == 'kura-networking-netplan':
        name, repository, module = 'networking-netplan', 'kura-networking', 'distrib-netplan'
    else:
        name, repository, module = package.removeprefix('kura-'), package, 'distrib'
    repo = WORKSPACE / repository
    assert repo.is_dir() and package.startswith('kura-')
    if package != 'kura-networking-netplan':
        assert 'util-linux' in metadata['Depends']
    source = repo / module
    for hook in ('preinst', 'postinst', 'prerm', 'postrm'):
        current = source / ('target/control' if package == 'kura-networking-netplan' else 'deb/control') / hook
        if current.exists():
            assert control[hook] == current.read_bytes(), (package, hook)
    manifests = {}
    tracked = subprocess.check_output(['git', '-C', str(repo), 'ls-files', '*MANIFEST.MF'], text=True)
    for file in tracked.splitlines():
        manifest = headers((repo / file).read_bytes())
        symbolic = manifest.get('Bundle-SymbolicName', '').split(';', 1)[0]
        if symbolic:
            manifests[symbolic] = manifest
    scripts = source / 'src/main/resources/scripts'
    script_roots = [scripts]
    if package == 'kura-firewall-only':
        script_roots.append(repo / 'distrib/src/main/resources/scripts')
    ns = {'m': 'http://maven.apache.org/POM/4.0.0'}
    code_modes = {}
    for data in ET.parse(source / 'pom.xml').findall('.//m:dataSet/m:data', ns):
        mapper = data.find('m:mapper', ns)
        if mapper is not None and '/libexec/addons/' in mapper.findtext('m:prefix', '', ns):
            filename = Path(data.findtext('m:src', '', ns)).name
            code_modes[filename] = int(mapper.findtext('m:filemode', '', ns), 8)
    jars, installed_code, contracts, additional_code = [], [], [], []
    with tarfile.open(fileobj=io.BytesIO(parts.pop('data')), mode='r:*') as archive:
        for member in archive:
            relative = member.name.removeprefix('./')
            assert not member.name.startswith('/') and '..' not in PurePosixPath(relative).parts
            if not relative.startswith('opt/eclipse/kura/'):
                continue
            if member.isdir():
                assert member.uid == 0 and not member.mode & 0o022, relative
                continue
            assert member.isfile(), relative
            if relative.endswith('.jar') and '/siblings/' in relative:
                assert (member.uid, member.gid, member.uname, member.gname, member.mode) == (0, 0, 'root', 'root', 0o644), relative
                raw = archive.extractfile(member).read()
                manifest = headers(zipfile.ZipFile(io.BytesIO(raw)).read('META-INF/MANIFEST.MF'))
                symbolic = manifest.get('Bundle-SymbolicName', '').split(';', 1)[0]
                declared = clauses(manifests.get(symbolic, {}).get('Import-Package', ''))
                actual = clauses(manifest.get('Import-Package', ''))
                for api in ('org.eclipse.kura.crypto', 'org.eclipse.kura.configuration'):
                    if api in declared:
                        assert actual.get(api) == declared[api], (package, symbolic, api, declared[api], actual.get(api))
                        contracts.append({'bundle': symbolic, 'api': api, 'clause': actual[api]})
                jars.append({'path': relative, 'sha256': hashlib.sha256(raw).hexdigest(), 'symbolicName': symbolic})
            elif '/libexec/addons/' in relative:
                assert (member.uid, member.gid, member.uname, member.gname) == (0, 0, 'root', 'root'), relative
                # Bash-sourced/read scripts may deliberately be 0644. Require
                # the source-declared mode, without adding executable bits.
                assert member.mode == code_modes.get(PurePosixPath(relative).name, 0o644), relative
                raw = archive.extractfile(member).read()
                matches = [candidate for folder in script_roots
                           for candidate in folder.rglob(PurePosixPath(relative).name)
                           if candidate.is_file() and candidate.read_bytes() == raw]
                assert matches, relative
                installed_code.append(relative)
            elif relative == 'opt/eclipse/kura/bin/firewall' and package in ('kura-networking', 'kura-firewall-only'):
                assert (member.uid, member.gid, member.uname, member.gname, member.mode) == (0, 0, 'root', 'root', 0o700), relative
                filename = 'firewall-only.init' if package == 'kura-firewall-only' else 'firewall.init'
                assert archive.extractfile(member).read() == (scripts / filename).read_bytes(), relative
                additional_code.append(relative)
            elif '/.data/' in relative and relative.endswith(('.sh', '.py')):
                raise AssertionError('Privileged code remains in runtime data: ' + relative)
    assert jars
    expected = 'opt/eclipse/kura/libexec/addons/' + name + '/'
    if package == 'kura-networking-netplan':
        # This compatibility adapter has five JARs, no mutable registry and no
        # installer code in the payload. Its control script is generated above.
        assert len(jars) == 5 and not installed_code and not additional_code
        assert all('/siblings/networking-netplan/' in jar['path'] for jar in jars)
        assert 'postinst' not in control
    else:
        assert len(installed_code) >= (2 if package == 'kura-firewall-only' else 3)
        assert expected + 'sibling_registry.sh' in installed_code
    return {'package': package, 'version': metadata['Version'], 'path': str(path.resolve()),
            'sha256': sha(path), 'bytes': path.stat().st_size,
            'sourceHead': subprocess.check_output(['git', '-C', str(repo), 'rev-parse', 'HEAD'], text=True).strip(),
            'jars': jars, 'installedCodeFiles': installed_code,
            'additionalCodeFiles': additional_code, 'sourceApiContracts': contracts}


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('packages', nargs='+', type=Path)
    parser.add_argument('--report', required=True, type=Path)
    args = parser.parse_args()
    results = []
    for path in args.packages:
        result = inspect(path)
        results.append(result)
        print(result['package'], len(result['jars']), 'Root/0644 JARs;',
              len(result['installedCodeFiles']), 'source-matched code/template files;',
              len(result['sourceApiContracts']), 'source API contracts matched', flush=True)
    args.report.write_text(json.dumps(results, indent=2) + '\n')
