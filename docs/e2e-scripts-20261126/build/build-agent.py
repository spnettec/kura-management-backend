"""Rebuild the management agent deb from merged heyoulin-kura (documented pipeline).

stage:   packaging/build.py source with the protocol crate (contents verified equal to the
         merged console source).
test:    cargo test inside the rust:1.95 ARM64 container against the workspace cargo cache.
build:   cargo build --release in the same container (release packaging uses this ELF).
package: packaging/build.py deb with the bridge bundle compiled by build-all.sh from the merged
         kura-networking tree and the core loader digest of the merged Core assembly.

Usage: python3 -B build-agent.py stage test build package
"""
from pathlib import Path
import datetime
import hashlib
import io
import json
import shutil
import subprocess as sp
import sys
import tarfile

here = Path(__file__).resolve().parent
hot = here.parents[1]
workspace = Path('/Users/heyoulin/iot-kura-develop/git')
repo = workspace / 'kura-management-agent'
revisions = dict(line.split()[:2] for line in (here.parent / 'source-revisions.txt').read_text().splitlines())
revision = revisions['kura-management-agent']
protocol = dict(
    file=hot.parent / 'core-networking-agent-peer-20261120/rust/protocol-target/package/yofc-management-protocol-0.1.0.crate',
    sha256='c1304333bba2b54d79751091eb8f8b92d5d50b73ddcafcdf5a7c9f7bc97f1e64')
bridge = workspace / 'kura-networking/bundles/org.eclipse.kura.management.bridge/target/org.eclipse.kura.management.bridge-1.0.0-SNAPSHOT.jar'
core_loader = workspace / 'kura/kura/distrib/kura-core/target/kura_6.0.0-SNAPSHOT_linux/bin/gen_config_ini.sh'
cache = hot.parent / 'configuration-capability-20261120/linux-agent-cargo'
image = 'sha256:7cf1e580ef5539f03b58560753e8ab84c8c360960d99dff714004aa98f203977'
packager = 'yofc-agent-packager:latest'
tool = '/usr/local/rustup/toolchains/1.95.0-aarch64-unknown-linux-gnu/bin'
exported, source, target, out = here / 'git', here / 'source', here / 'target', here / 'release'
binary = target / 'release/yofc-management-agent'


def read(*args):
    return sp.check_output(args, text=True).strip()


def sha(path):
    with path.open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()


def logged(argv, log, **keywords):
    with (here / (log + '.log')).open('w') as stream:
        code = sp.run(argv, stdout=stream, stderr=sp.STDOUT, **keywords).returncode
    print(log, 'exit', code, flush=True)
    return code


def mount(path, target_path, readonly=True):
    return ['--mount', f'type=bind,source={path},target={target_path}' + (',readonly' if readonly else '')]


def cargo(*arguments, sandbox, log=None, source_dir=None, target_dir=None):
    # build: hardened (read-only root, no capabilities, noexec /tmp).
    # test: the suites execute helper scripts from /tmp and chown files, so /tmp must be exec-able
    # and the container keeps its default capabilities; everything else is identical.
    # source_dir / target_dir / log are used by collect-evidence.py to run another source tree.
    hardening = ['--read-only', '--cap-drop', 'ALL', '--security-opt', 'no-new-privileges'] if sandbox else []
    temporary = '/tmp' if sandbox else '/tmp:rw,exec'
    args = ['docker', 'run', '--rm', '--pull=never', '--platform', 'linux/arm64', *hardening,
            *mount(source_dir or source, '/source', sandbox), *mount(cache, '/cargo', False),
            *mount(target_dir or target, '/target', False),
            '-e', 'CARGO_HOME=/cargo', '-e', 'CARGO_TARGET_DIR=/target', '-e', 'CARGO_INCREMENTAL=0',
            '-e', 'PATH=' + tool + ':/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin',
            '--tmpfs', temporary, '-w', '/source', '--entrypoint', tool + '/cargo', image, *arguments]
    return logged(args, log or 'cargo-' + arguments[0])


def stage():
    assert read('git', '-C', str(repo), 'rev-parse', 'HEAD') == revision
    assert not read('git', '-C', str(repo), 'status', '--porcelain'), 'commit the merged tree first'
    assert sha(protocol['file']) == protocol['sha256'] and sha(core_loader) and bridge.is_file()
    shutil.rmtree(exported, ignore_errors=True)
    shutil.rmtree(source, ignore_errors=True)
    exported.mkdir()
    with tarfile.open(fileobj=io.BytesIO(sp.check_output(['git', '-C', str(repo), 'archive', revision]))) as archive:
        archive.extractall(exported, filter='data')
    assert logged([sys.executable, '-B', exported / 'packaging/build.py', 'source', '--protocol', protocol['file'],
                   '--sha256', protocol['sha256'], '--destination', source], 'stage') == 0


def test():
    target.mkdir(exist_ok=True)
    return cargo('test', '--release', '--locked', '--offline', sandbox=False)


def build():
    target.mkdir(exist_ok=True)                # 编译缓存由 rebuild-all.sh 的清空步骤负责，这里不再删 target/release
    assert cargo('build', '--release', '--locked', '--offline', sandbox=True) == 0


def package():
    head = read('git', '-C', str(repo), 'rev-parse', 'HEAD')
    stamp = datetime.datetime.fromtimestamp(int(read('git', '-C', str(repo), 'show', '-s', '--format=%ct', head)),
                                            datetime.timezone.utc).strftime('%Y%m%d%H%M%S')
    version = '0.1.0~cohort' + stamp + '.' + head[:7] + '-1'
    shutil.rmtree(out, ignore_errors=True)
    out.mkdir()
    digest, bridge_digest, loader_digest = sha(binary), sha(bridge), sha(core_loader)
    code = logged(['docker', 'run', '--rm', '--network', 'none', '--platform', 'linux/arm64',
                   '--entrypoint', '/usr/bin/python3', '-v', f'{source}:/source:ro', '-v', f'{binary}:/inputs/agent:ro',
                   '-v', f'{bridge}:/inputs/bridge.jar:ro', '-v', f'{out}:/output', '-w', '/source', packager, '-B',
                   'packaging/build.py', 'deb', '--binary', '/inputs/agent', '--sha256', digest, '--architecture', 'arm64',
                   '--version', version, '--destination', '/output/release', '--source-manifest', '/source/source-manifest.json',
                   '--bridge-bundle', '/inputs/bridge.jar', '--bridge-sha256', bridge_digest,
                   '--core-loader-sha256', loader_digest, '--runtime-profile', 'native-nonroot-v1'], 'package')
    assert code == 0, 'packaging failed'
    receipt = json.loads((out / 'release/package.json').read_text())
    artifact = out / 'release' / Path(receipt['package']).name
    assert sha(artifact) == receipt['sha256'] and receipt['binarySha256'] == digest == sha(binary)
    summary = dict(revision=head, version=version, package=str(artifact), sha256=receipt['sha256'], bytes=artifact.stat().st_size,
                   binarySha256=digest, bridgeSha256=bridge_digest, bridgeVersion=receipt['javaBridge']['version'],
                   coreLoaderSha256=loader_digest, protocolCrateSha256=protocol['sha256'],
                   sourceManifestSha256=receipt['sourceManifestSha256'])
    (here / 'release.json').write_text(json.dumps(summary, indent=2) + '\n')
    print(json.dumps(summary, indent=1))
    shutil.copyfile(artifact, here / artifact.name)


if __name__ == '__main__':
    status = 0
    for action in sys.argv[1:]:
        status = globals()[action]() or 0      # test 返回 cargo 的退出码，其余阶段失败时直接抛出
        if status:
            break                              # 某一阶段失败就停在这里，退出码不被后面的阶段覆盖
    raise SystemExit(status)
