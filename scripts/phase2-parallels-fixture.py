#!/usr/bin/env python3
"""Observation-only acceptance on the explicitly provisioned Parallels VM.

Does not change Netplan, NetworkManager, cloud-init, routes, or delegation.
The TCP relay preserves end-to-end TLS and binds only the Parallels Shared IP.
This is a development fixture, not an enrollment or release installer.
"""
import argparse
import grp
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import platform
import select
import shlex
import shutil
import socket
import socketserver
import ssl
import stat
import struct
import subprocess
import sys
import tempfile
import time
import uuid

ROOT = Path(__file__).resolve().parents[1]
DEV = ROOT / '.phase1-dev'
VM = '8061d57e-0120-42ec-a9a9-5799b436c4eb'
GATEWAY = 'parallels-2604-gateway'
SHARED = Path('/media/psf/Home/iot-kura-develop/git/kura-management-console')
UNIT = 'kura-management-agent.service'
HOST_IP = '10.211.55.2'


def run(args, **kw):
    return subprocess.run([str(a) for a in args], check=True, **kw)


def output(*args):
    return run(args, capture_output=True, text=True).stdout.strip()


def guest(*args, timeout=60):
    # prlctl joins argv into a guest shell command. Quote every argument there.
    return run(['/usr/local/bin/prlctl', 'exec', VM,
                *[shlex.quote(str(a)) for a in args]],
               capture_output=True, text=True, timeout=timeout).stdout.strip()


def inside(action, *args):
    for attempt in range(3 if action == 'probe' else 1):
        try:
            return json.loads(guest('python3', SHARED / 'scripts' / Path(__file__).name,
                                    '--inside', action, *args))
        except subprocess.CalledProcessError as error:
            # Guest Tools occasionally returns a transport error. Only retry the
            # read-only inventory, never installation, control requests or reboot.
            if action == 'probe' and error.returncode == 255 and attempt < 2:
                time.sleep(.3)
                continue
            raise RuntimeError(f'guest {action} failed ({error.returncode}): {error.stderr[:1500]}') from error


def write_report(name, value):
    p = ROOT / 'target' / f'parallels-{name}.json'
    p.write_text(json.dumps(value, ensure_ascii=False, indent=2) + '\n')
    return p


def hashes(directory, pattern='*'):
    root = Path(directory)
    return {str(p): {'sha256': hashlib.sha256(p.read_bytes()).hexdigest(),
                     'mode': oct(stat.S_IMODE(p.stat().st_mode))}
            for p in sorted(root.glob(pattern)) if p.is_file()}


def probe():
    release = dict(line.split('=', 1) for line in Path('/etc/os-release').read_text().splitlines()
                   if '=' in line)
    assert release['VERSION_ID'].strip('"') == '26.04', release
    assert platform.machine() == 'aarch64'
    virt = output('systemd-detect-virt')
    assert virt == 'parallels', virt
    service = output('systemctl', 'show', UNIT,
                     '--property=ActiveState,SubState,MainPID,UnitFileState,NoNewPrivileges,ProtectSystem,ProtectHome,PrivateTmp,CapabilityBoundingSet,RestrictAddressFamilies,TasksMax,MemoryMax,Group')
    result = {'os': release['PRETTY_NAME'].strip('"'), 'architecture': platform.machine(),
              'virtualization': virt, 'bootId': Path('/proc/sys/kernel/random/boot_id').read_text().strip(),
              'links': json.loads(output('ip', '-j', 'address')),
              'defaultRoutes': json.loads(output('ip', '-j', '-4', 'route', 'show', 'default')),
              'netplan': hashes('/etc/netplan', '*.yaml'),
              'nmProfiles': {**hashes('/etc/NetworkManager/system-connections'),
                             **hashes('/run/NetworkManager/system-connections')},
              'networkdRuntime': hashes('/run/systemd/network'),
              'service': dict(line.split('=', 1) for line in service.splitlines() if '=' in line),
              'networkServices': {},
              'packages': output('dpkg-query', '-W', 'netplan.io', 'systemd', 'network-manager', 'cloud-init')}
    for name in ['systemd-networkd', 'NetworkManager', 'cloud-init-local', 'cloud-init-network', 'cloud-init']:
        r = subprocess.run(['systemctl', 'is-active', name], capture_output=True, text=True)
        result['networkServices'][name] = r.stdout.strip()
    pid = int(result['service'].get('MainPID', '0'))
    if pid:
        result['processSecurity'] = {k: v.strip() for k, v in
            (line.split(':', 1) for line in Path(f'/proc/{pid}/status').read_text().splitlines()
             if ':' in line) if k in ['CapBnd', 'CapEff', 'NoNewPrivs', 'Seccomp']}
        result['agentSha256'] = hashlib.sha256(Path('/usr/bin/yofc-management-agent').read_bytes()).hexdigest()
    return result


def install_inside():
    assert os.geteuid() == 0
    before = probe()
    env_path = Path('/etc/kura-management-agent/agent.env')
    if env_path.exists():
        text = env_path.read_text()
        assert f'AGENT_GATEWAY_ID={GATEWAY}\n' in text, 'existing installation belongs to another gateway'
        assert 'AGENT_NETWORK_INTERFACES=\n' in text, 'existing write delegation; inspect instead of overwriting'
    else:
        assert not Path('/usr/bin/yofc-management-agent').exists(), 'unknown existing agent binary'
        assert not Path('/var/lib/kura-management-agent').exists(), 'unknown existing agent state'
    hosts = Path('/etc/hosts')
    existing = [line.split('#', 1)[0].split() for line in hosts.read_text().splitlines()]
    mappings = [row[0] for row in existing if 'host.orb.internal' in row[1:]]
    assert not mappings or mappings == [HOST_IP], 'existing host.orb.internal maps elsewhere'
    unit = Path('/etc/systemd/system') / UNIT
    if unit.exists():
        run(['systemctl', 'stop', UNIT], stdout=subprocess.DEVNULL)
    try:
        grp.getgrnam('kura-management')
    except KeyError:
        run(['groupadd', '--system', 'kura-management'])
    config = env_path.parent
    config.mkdir(mode=0o700, exist_ok=True)
    for src, dst in [('ca.pem', 'ca.pem'), ('parallels-agent.pem', 'agent.pem'), ('parallels-agent.key', 'agent.key')]:
        run(['install', '-m', '600', SHARED / '.phase1-dev' / src, config / dst])
    run(['install', '-m', '755', SHARED / '.phase1-dev/parallels-agent-linux', '/usr/bin/yofc-management-agent'])
    source_unit = SHARED.parent / 'kura-management-agent/packaging' / UNIT
    shutil.copyfile(source_unit, unit)
    unit.chmod(0o644)
    env_path.write_text(f'''AGENT_CONSOLE_ENDPOINT=https://host.orb.internal:18444
AGENT_GATEWAY_ID={GATEWAY}
AGENT_CA=/etc/kura-management-agent/ca.pem
AGENT_CERT=/etc/kura-management-agent/agent.pem
AGENT_KEY=/etc/kura-management-agent/agent.key
AGENT_STATE_DB=/var/lib/kura-management-agent/results.sqlite
AGENT_SOCKET=/run/kura-management-agent/control.sock
AGENT_KURA_UID=65534
AGENT_ADMIN_UID=0
AGENT_NETWORK_INTERFACES=
AGENT_NETWORK_ALLOW_VIRTUAL_FIXTURE=0
RUST_LOG=info
''')
    env_path.chmod(0o600)
    if not mappings:
        with hosts.open('a') as f:
            f.write(f'\n{HOST_IP} host.orb.internal # YOFC isolated development center\n')
    run(['systemctl', 'daemon-reload'], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    run(['systemctl', 'enable', '--now', UNIT], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    after = probe()
    assert before['netplan'] == after['netplan']
    assert before['nmProfiles'] == after['nmProfiles']
    return {'before': before, 'after': after, 'delegation': [], 'unit': str(source_unit)}


def local(method, params=None):
    frame = {'version': 1, 'kind': 'request', 'requestId': str(uuid.uuid4()), 'method': method}
    if params is not None:
        frame['params'] = params
    with socket.socket(socket.AF_UNIX) as s:
        s.settimeout(40)
        s.connect('/run/kura-management-agent/control.sock')
        raw = json.dumps(frame).encode()
        s.sendall(struct.pack('>I', len(raw)) + raw)

        def read(n):
            data = b''
            while len(data) < n:
                part = s.recv(n - len(data))
                if not part:
                    raise EOFError()
                data += part
            return data

        size = struct.unpack('>I', read(4))[0]
        assert size <= 1024 * 1024
        return json.loads(read(size))


def generate_inside():
    before = probe()
    with tempfile.TemporaryDirectory(prefix='yofc-netplan-stage-') as directory:
        stage = Path(directory)
        for relative in ['lib/netplan', 'etc/netplan', 'run/netplan']:
            target = stage / relative
            target.mkdir(parents=True)
            for source in Path('/' + relative).glob('*.yaml'):
                shutil.copy2(source, target / source.name)
        candidate = stage / 'etc/netplan/99-yofc-fixture-networkd.yaml'
        candidate.write_text('''network:
  version: 2
  ethernets:
    eth1:
      renderer: networkd
      dhcp4: false
      addresses: [192.0.2.11/24]
      mtu: 1400
      optional: true
''')
        candidate.chmod(0o600)
        generator = '/usr/libexec/netplan/configure'
        assert Path(generator).is_file(), 'Ubuntu 26.04 native configure entry missing'
        result = subprocess.run([generator, '--root-dir', str(stage)],
                                capture_output=True, text=True)
        generated = stage / 'run/systemd/network/10-netplan-eth1.network'
        content_valid = generated.is_file() and '192.0.2.11/24' in generated.read_text()
        backends = hashes(stage / 'run/systemd/network')
    after = probe()
    assert before['networkdRuntime'] == after['networkdRuntime'], 'live networkd backend changed'
    assert before['nmProfiles'] == after['nmProfiles'], 'live NM profiles changed'
    assert before['netplan'] == after['netplan'], 'live Netplan sources changed'
    assert before['defaultRoutes'] == after['defaultRoutes'], 'live default routes changed'
    return {'generator': generator, 'exitCode': result.returncode,
            'passed': result.returncode == 0 and content_valid,
            'diagnostic': result.stderr[:3000],
            'stagedNetworkdBackend': backends, 'liveFilesUnchanged': True,
            'limitation': 'private staging generation only; no Netplan apply or writer handover'}


class Relay(socketserver.BaseRequestHandler):
    def handle(self):
        with socket.create_connection(('127.0.0.1', 18444), timeout=5) as upstream:
            self.request.settimeout(30)
            while True:
                ready, _, _ = select.select([self.request, upstream], [], [], 30)
                if not ready:
                    return
                for source in ready:
                    data = source.recv(65536)
                    if not data:
                        return
                    (upstream if source is self.request else self.request).sendall(data)


def relay():
    class Server(socketserver.ThreadingTCPServer):
        allow_reuse_address = True
        daemon_threads = True
    with Server((HOST_IP, 18444), Relay) as server:
        server.serve_forever()


def start_relay():
    pidfile = DEV / 'parallels-relay.pid'
    if pidfile.exists():
        pid = int(pidfile.read_text())
        p = subprocess.run(['ps', '-p', str(pid), '-o', 'command='], capture_output=True, text=True)
        if p.returncode == 0 and str(Path(__file__).resolve()) + ' relay' in p.stdout:
            return
    with (DEV / 'parallels-relay.log').open('ab') as log:
        child = subprocess.Popen([sys.executable, str(Path(__file__).resolve()), 'relay'],
                                 stdout=log, stderr=log, start_new_session=True)
    pidfile.write_text(str(child.pid))
    for _ in range(30):
        if child.poll() is not None:
            raise RuntimeError('Parallels relay exited; inspect its log')
        try:
            with socket.create_connection((HOST_IP, 18444), timeout=.5):
                return
        except OSError:
            time.sleep(.1)
    raise TimeoutError('relay not ready')


def install(expected):
    before = inside('probe')
    if not (ROOT / 'target/parallels-inventory.json').exists():
        write_report('inventory', before)
    os.umask(0o077)
    source = '/home/heyoulin/.cache/yofc-management-phase1/target-agent/debug/yofc-management-agent'
    binary = run(['orb', 'run', '-m', 'kura-package-validation-20261029', 'cat', source], capture_output=True).stdout
    assert binary.startswith(b'\x7fELF')
    assert hashlib.sha256(binary).hexdigest() == expected, 'Linux build changed; review evidence first'
    (DEV / 'parallels-agent-linux').write_bytes(binary)
    cert = DEV / 'parallels-agent.pem'
    if not cert.exists():
        key = DEV / 'parallels-agent.key'
        run(['openssl', 'req', '-newkey', 'rsa:2048', '-nodes', '-subj', '/CN=parallels-2604-fixture',
             '-keyout', key, '-out', DEV / 'parallels-agent.csr'], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        ext = DEV / 'parallels-agent.ext'
        ext.write_text('basicConstraints=critical,CA:FALSE\nkeyUsage=critical,digitalSignature\nextendedKeyUsage=clientAuth\n')
        run(['openssl', 'x509', '-req', '-in', DEV / 'parallels-agent.csr', '-CA', DEV / 'ca.pem',
             '-CAkey', DEV / 'ca.key', '-CAserial', DEV / 'ca-serial', '-days', '7', '-extfile', ext,
             '-out', cert], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        key.chmod(0o600)
    spec = importlib.util.spec_from_file_location('dev', ROOT / 'scripts/phase1-dev.py')
    dev = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(dev)
    registered = output(dev.PG / 'psql', '-h', DEV / 'pgsock', '-p', dev.PG_PORT,
                        '-d', 'management', '-Atc',
                        f"SELECT tenant_id,certificate_sha256,enabled FROM mgmt_gateways WHERE id='{GATEWAY}'")
    if registered:
        fingerprint = hashlib.sha256(ssl.PEM_cert_to_DER_cert(cert.read_text())).hexdigest()
        assert registered == f'dev|{fingerprint}|t', 'existing gateway binding differs; refusing replacement'
    else:
        run([ROOT / 'target/debug/yofc-management-console', 'register-gateway', GATEWAY, 'dev',
             'Parallels Ubuntu 26.04 独立只读验证', cert], env=dev.env(), stdout=subprocess.DEVNULL)
    start_relay()
    result = inside('install')
    write_report('install', result)
    print(json.dumps({'vm': VM, 'gateway': GATEWAY, 'writeDelegation': [],
                      'evidence': str(ROOT / 'target/parallels-install.json')}, ensure_ascii=False))


def smoke():
    spec = importlib.util.spec_from_file_location('smoke', ROOT / 'scripts/phase1-smoke.py')
    m = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(m)
    client = m.Client()
    client.login('admin')
    checks = []
    data = {}

    def check(name, condition):
        if not condition:
            write_report('smoke-failed', {'checks': checks, 'failed': name, 'data': data})
            raise AssertionError(name)
        checks.append(name)

    def remote(operation, **extra):
        status, response = client.request(f'/api/v1/gateways/{GATEWAY}/tasks', 'POST',
                                         {'operation': operation, 'idempotencyKey': str(uuid.uuid4()), **extra})
        assert status == 202, (status, response)
        return m.wait_task(client, response['id'])

    for _ in range(60):
        status, row = client.request(f'/api/v1/gateways/{GATEWAY}')
        if status == 200 and row.get('online'):
            break
        time.sleep(.5)
    data['gateway'] = row
    check('dedicated certificate-bound gateway online over mTLS', status == 200 and row.get('online'))
    p = inside('probe')
    data['probe'] = p
    check('Ubuntu 26.04 aarch64 on full Parallels hypervisor', p['virtualization'] == 'parallels' and p['architecture'] == 'aarch64')
    check('packaging service active and enabled', p['service']['ActiveState'] == 'active' and p['service']['UnitFileState'] == 'enabled')
    check('real process has no capabilities and NoNewPrivileges', p['processSecurity']['CapEff'] == '0000000000000000' and p['processSecurity']['CapBnd'] == '0000000000000000' and p['processSecurity']['NoNewPrivs'] == '1')
    check('ProtectSystem ProtectHome PrivateTmp applied', p['service']['ProtectSystem'] == 'strict' and p['service']['ProtectHome'] == 'yes' and p['service']['PrivateTmp'] == 'yes')
    a = remote('agent.status')
    data['status'] = a
    check('agent managed without Kura JVM', a['state'] == 'SUCCEEDED' and a['result']['agentAvailable'] and not a['result']['kuraAvailable'])
    a = remote('kura.configuration.list')
    check('missing Kura produces KURA_UNAVAILABLE', a['state'] == 'FAILED' and a['error'] == 'KURA_UNAVAILABLE')
    a = remote('network.inspect')
    data['network'] = a
    check('HTTPS gRPC network inspection returns both wired interfaces', a['state'] == 'SUCCEEDED' and {'eth0', 'eth1'} <= {i['name'] for i in a['result']['interfaces']})
    view = a['result']
    check('observation profile delegates no writes', not view['writeEnabled'] and not any(i['allowed'] for i in view['interfaces']))
    check('sandbox still reads real interface addresses and routes', all(i['runtime'] is not None for i in view['interfaces'] if i['name'] in ['eth0', 'eth1']))
    observed = {i['name']: i['runtime'] for i in view['interfaces']}
    actual = {i['ifname']: i for i in p['links']}
    check('sandbox IPv4 and MTU match independent root observations', all(
        sorted(observed[name]['addresses']) == sorted(f"{a['local']}/{a['prefixlen']}"
            for a in actual[name]['addr_info'] if a['family'] == 'inet')
        and observed[name]['mtu'] == actual[name]['mtu'] for name in ['eth0', 'eth1']))
    a = remote('network.prepare', networkChange={'expectedRevision': view['revision'], 'sourceDigest': view['sourceDigest'],
        'confirmSeconds': 30, 'config': {'interface': 'eth1', 'dhcp4': False, 'addresses': ['192.0.2.11/24'],
        'routes': [], 'nameservers': [], 'searchDomains': [], 'mtu': 1400, 'optional': True, 'dhcpMetric': 100}})
    check('undelegated remote write is FORBIDDEN', a['state'] == 'FAILED' and a['error'] == 'FORBIDDEN')
    a = inside('local', 'agent.status')
    check('authorized root UDS works independently of gRPC', a.get('result', {}).get('agentAvailable') is True)
    check('UDS is restricted to owner and group', guest('stat', '-c', '%a', '/run/kura-management-agent/control.sock') == '660')
    after = inside('probe')
    baseline = json.loads((ROOT / 'target/parallels-inventory.json').read_text())
    check('original Netplan bytes and modes retained', baseline['netplan'] == after['netplan'])
    # NM regenerates runtime UUIDs during a real boot. Compare this boot's
    # before/after observation; persistent Netplan still uses the install baseline.
    check('real NM profiles retained within this boot', p['nmProfiles'] == after['nmProfiles'])
    check('default management routes retained', baseline['defaultRoutes'] == after['defaultRoutes'])
    result = {'passed': len(checks), 'checks': checks, 'vm': VM, 'gatewayId': GATEWAY, 'bootId': p['bootId'],
              'data': data, 'limitations': ['observation-only; real NM/cloud-init handover not implemented',
                  'no Linux Kura runtime installed', 'not physical NIC or hard-power acceptance']}
    write_report('smoke', result)
    print(json.dumps({'passed': len(checks), 'checks': checks}, ensure_ascii=False))


def lifecycle():
    spec = importlib.util.spec_from_file_location('dev', ROOT / 'scripts/phase1-dev.py')
    dev = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(dev)
    spec = importlib.util.spec_from_file_location('smoke', ROOT / 'scripts/phase1-smoke.py')
    m = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(m)
    c = m.Client()
    c.login('admin')

    def identity():
        return output(dev.PG / 'psql', '-h', DEV / 'pgsock', '-p', dev.PG_PORT,
                      '-d', 'management', '-Atc',
                      f"SELECT boot_id,connection_epoch FROM mgmt_gateways WHERE id='{GATEWAY}'").split('|')

    original = inside('probe')
    records = []
    for kind in ['service-restart', 'normal-vm-reboot']:
        before = inside('probe')
        old_boot, old_epoch = identity()
        command = ['systemctl', 'restart', UNIT] if kind == 'service-restart' else ['systemctl', 'reboot']
        r = subprocess.run(['/usr/local/bin/prlctl', 'exec', VM, *command],
                           capture_output=True, text=True, timeout=20)
        # A reboot may terminate the guest-tools command channel before its acknowledgement.
        # Never resend a mutation; determine its outcome from boot ID and readiness.
        deadline = time.monotonic() + 120
        last = None
        early_routes = []
        while time.monotonic() < deadline:
            try:
                after = inside('probe')
                boot, epoch = identity()
                status, row = c.request(f'/api/v1/gateways/{GATEWAY}')
                if after['defaultRoutes'] != original['defaultRoutes']:
                    early_routes.append(after['defaultRoutes'])
                if (after['service']['ActiveState'] == 'active' and status == 200 and row.get('online')
                        and boot == after['bootId'] and epoch != old_epoch
                        and after['defaultRoutes'] == original['defaultRoutes']
                        and (kind != 'normal-vm-reboot' or boot != old_boot)):
                    break
            except (RuntimeError, subprocess.CalledProcessError, subprocess.TimeoutExpired, OSError, ValueError) as error:
                last = type(error).__name__
            time.sleep(1)
        else:
            raise AssertionError(f'{kind} did not recover; last probe: {last}')
        assert after['netplan'] == original['netplan'], 'persistent Netplan changed'
        assert after['defaultRoutes'] == original['defaultRoutes'], 'management default route changed'
        assert after['service']['UnitFileState'] == 'enabled'
        status, task = c.request(f'/api/v1/gateways/{GATEWAY}/tasks', 'POST',
                                {'operation': 'agent.status', 'idempotencyKey': str(uuid.uuid4())})
        assert status == 202, task
        result = m.wait_task(c, task['id'])
        assert result['state'] == 'SUCCEEDED' and not result['result']['kuraAvailable'], result
        uds = inside('local', 'agent.status')
        assert uds.get('result', {}).get('agentAvailable')
        records.append({'kind': kind, 'commandExit': r.returncode, 'beforeBootId': before['bootId'],
                        'afterBootId': after['bootId'], 'oldEpoch': old_epoch, 'newEpoch': epoch,
                        'serviceEnabled': True, 'remoteTask': result['id'], 'UDS': 'passed',
                        'earlyBootRouteSnapshots': early_routes,
                        'netplanAndDefaultRoutes': 'unchanged', 'kuraAvailable': False})
        write_report('lifecycle-progress', {'completed': records})
        print(f'{kind}: new mTLS epoch, remote task and UDS passed', flush=True)
    write_report('lifecycle', {'passed': 2, 'checks': records,
                             'finalProbe': inside('probe'),
                             'limitations': ['normal reboot, not hard power loss', 'observation service only']})


def hybrid():
    """Use the existing OrbStack VM and this one full VM; no extra full VM."""
    spec = importlib.util.spec_from_file_location('smoke', ROOT / 'scripts/phase1-smoke.py')
    m = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(m)
    c = m.Client()
    c.login('admin')
    checks = []
    details = {}

    def check(name, condition):
        if not condition:
            raise AssertionError(name)
        checks.append(name)

    def submit(gateway, operation):
        status, task = c.request(f'/api/v1/gateways/{gateway}/tasks', 'POST',
                                {'operation': operation, 'idempotencyKey': str(uuid.uuid4())})
        assert status == 202, task
        return task['id']

    for gateway in [GATEWAY, 'network-gateway']:
        status, row = c.request(f'/api/v1/gateways/{gateway}')
        check(f'{gateway} independently online', status == 200 and row['online'])
        task = m.wait_task(c, submit(gateway, 'network.inspect'))
        check(f'{gateway} inspection succeeds', task['state'] == 'SUCCEEDED')
        details[gateway] = task
    p = details[GATEWAY]['result']
    o = details['network-gateway']['result']
    check('Parallels wired observation and OrbStack delegation stay separate',
          not p['writeEnabled'] and not any(i['allowed'] for i in p['interfaces'])
          and o['writeEnabled'] and {i['name'] for i in o['interfaces'] if i['allowed']} == {'yofc-test0', 'yofc-adopt0'})
    original = inside('probe')
    pending = None
    try:
        guest('systemctl', 'stop', UNIT)
        for _ in range(30):
            status, row = c.request(f'/api/v1/gateways/{GATEWAY}')
            if status == 200 and not row['online']:
                break
            time.sleep(.5)
        check('stopped Parallels agent reported offline', status == 200 and not row['online'])
        pending = submit(GATEWAY, 'agent.status')
        other = m.wait_task(c, submit('network-gateway', 'agent.status'))
        check('OrbStack remains manageable while Parallels agent is stopped',
              other['state'] == 'SUCCEEDED' and other['gatewayId'] == 'network-gateway'
              and other['result']['agentAvailable'])
        status, rows = c.request('/api/v1/tasks')
        queued = next(t for t in rows['items'] if t['id'] == pending)
        check('offline task stays bound to its gateway and queued',
              queued['gatewayId'] == GATEWAY and queued['state'] == 'QUEUED')
    finally:
        guest('systemctl', 'start', UNIT)
    result = m.wait_task(c, pending)
    check('reconnected Parallels receives its own queued task',
          result['state'] == 'SUCCEEDED' and result['gatewayId'] == GATEWAY)
    details['queuedTask'] = result
    after = inside('probe')
    check('Parallels sources and default routes retained during stream isolation',
          original['netplan'] == after['netplan'] and original['defaultRoutes'] == after['defaultRoutes'])
    orb = m.wait_task(c, submit('network-gateway', 'network.inspect'))
    check('OrbStack revision and transaction retained',
          orb['state'] == 'SUCCEEDED' and orb['result']['revision'] == o['revision']
          and orb['result']['transaction'] == o['transaction'])
    write_report('hybrid', {'passed': len(checks), 'checks': checks, 'data': details,
                           'fullVMs': 1, 'existingOrbStackVM': 'yofc-management-network-20261101',
                           'limitations': ['gateway stream/task isolation only; no competing center transfer']})
    print(json.dumps({'passed': len(checks), 'checks': checks}, ensure_ascii=False))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('action', choices=['probe', 'install', 'local', 'smoke', 'lifecycle', 'hybrid', 'generate', 'relay'])
    parser.add_argument('params', nargs='*')
    parser.add_argument('--inside', action='store_true')
    parser.add_argument('--binary-sha256', default='4572d2e256f939741ad7e5ffb21459fdc7c1034e8623f99e1efbecb98d36e508',
                        help='reviewed Linux build digest required for fixture binary replacement')
    args = parser.parse_args()
    if args.inside:
        if args.action == 'probe':
            result = probe()
        elif args.action == 'install':
            result = install_inside()
        elif args.action == 'local':
            result = local(args.params[0], json.loads(args.params[1]) if len(args.params) > 1 else None)
        elif args.action == 'generate':
            result = generate_inside()
        else:
            parser.error('invalid guest action')
        print(json.dumps(result, ensure_ascii=False))
    elif args.action == 'probe':
        result = inside('probe')
        write_report('probe', result)
        print(json.dumps(result, ensure_ascii=False))
    elif args.action == 'install':
        install(args.binary_sha256)
    elif args.action == 'smoke':
        smoke()
    elif args.action == 'lifecycle':
        lifecycle()
    elif args.action == 'hybrid':
        hybrid()
    elif args.action == 'generate':
        result = inside('generate')
        write_report('generate', result)
        print(json.dumps(result, ensure_ascii=False))
    elif args.action == 'relay':
        relay()
    else:
        parser.error('local is a guest-only action')


if __name__ == '__main__':
    main()
