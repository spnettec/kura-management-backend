#!/usr/bin/env python3
"""Read retained profile evidence and verify restoration without changing a VM."""
import hashlib
import json
from pathlib import Path
import subprocess
import xml.etree.ElementTree as ET
import zipfile

ROOT = Path(__file__).resolve().parents[1]
TARGET = ROOT / 'target'
checks = []


def load(name):
    return json.loads((TARGET / ('netplan-profile-' + name + '.json')).read_text())


def check(name, condition):
    if not condition:
        raise AssertionError(name)
    checks.append(name)


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def main():
    output = TARGET / 'netplan-profile-final-check.json'
    assert not output.exists(), 'preserve existing evidence'
    for path, value in load('original-runtime').items():
        check('original default runtime preserved: ' + Path(path).name, sha(path) == value)
    host = load('idea-prepare')
    check('isolated host snapshot_0 preserved',
          sha(Path(host['dataHome']) / 'user/snapshots/snapshot_0.xml') == host['originalSnapshotSha256'])
    for pid in [6505, 7884]:
        check(f'IDEA PID {pid} exited', subprocess.run(['ps', '-p', str(pid)], capture_output=True).returncode != 0)
    unit = (TARGET / 'netplan-profile-unit-final.log').read_text()
    check('18 runtime tests passed', 'Ran 18 tests' in unit and '\nOK\n' in unit)
    suites = [ET.parse(path).getroot().attrib for path in
              (ROOT.parent / 'kura-management-ui/bundles/org.eclipse.kura.web2.test/target/surefire-reports').glob('TEST-*.xml')]
    counts = {key: sum(int(item[key]) for item in suites) for key in ['tests', 'failures', 'errors', 'skipped']}
    check('63 GWT server tests passed without skip', counts == dict(tests=63, failures=0, errors=0, skipped=0))
    stats = load('react-browser')['stats']
    check('11 React browser checks passed without skip',
          stats['expected'] == 11 and all(stats[key] == 0 for key in ['skipped', 'unexpected', 'flaky']))
    for name, total in [('host-live-fixed', 12), ('host-stopped-fixed', 11), ('vm-live-fixed', 12), ('vm-stopped', 11)]:
        result = load(name)
        check(name + ' bridge checks passed', result['status'] == 'passed' and result['passed'] == total)
    before = load('vm-before-write')
    baseline = before['networkBefore']
    check('real Kura UID and private socket', before['kuraUid'] == 999 and before['socketGid'] == 990
          and before['socketMode'] == '0o660')
    for name in ['mtu-pending-rollback', 'mtu-pending-confirm']:
        result = load('vm-' + name)
        expected = dict(baseline, dummyMtu=1390)
        check(name + ' candidate and confirmed values separate', result['networkNow'] == expected
              and result['network']['revision'] == 3
              and result['network']['confirmedYaml'] == before['network']['confirmedYaml']
              and result['network']['transaction']['state'] == 'WAIT_CONFIRM')
    rolled = load('vm-mtu-rolled-back')
    check('explicit rollback restored baseline', rolled['networkNow'] == baseline
          and rolled['network']['revision'] == 3 and rolled['network']['transaction']['state'] == 'ROLLED_BACK')
    confirmed = load('vm-mtu-confirmed')
    check('confirmed MTU 1390 at revision 4', confirmed['networkNow']['dummyMtu'] == 1390
          and confirmed['network']['revision'] == 4 and confirmed['network']['transaction']['state'] == 'COMMITTED'
          and confirmed['network']['confirmedYaml'] != before['network']['confirmedYaml'])
    pending = load('vm-baseline-pending')
    check('restoration candidate preserves previous confirmed value', pending['networkNow']['dummyMtu'] == 1500
          and pending['network']['revision'] == 4
          and pending['network']['confirmedYaml'] == confirmed['network']['confirmedYaml']
          and pending['network']['transaction']['state'] == 'WAIT_CONFIRM')
    final = load('vm-baseline-confirmed')
    check('revision 5 restores exact YAML and all network baseline values', final['networkNow'] == baseline
          and final['network']['confirmedYaml'] == before['network']['confirmedYaml']
          and final['network']['revision'] == 5 and final['network']['transaction']['state'] == 'COMMITTED')
    ids = [load('vm-' + name)['network']['transaction']['id']
           for name in ['mtu-rolled-back', 'mtu-confirmed', 'baseline-confirmed']]
    check('three distinct real transactions', len(set(ids)) == 3)
    restored = load('vm-restored')
    check('delegation restored and Kura stopped', restored['status'] == 'restored' and restored['kuraPid'] == 0
          and restored['agentEnvironmentRestored'] and restored['socketGid'] == 0
          and restored['socketMode'] == '0o660' and not restored['agentStatus']['kuraAvailable'])
    check('VM original snapshot and network retained', restored['networkNow'] == baseline
          and restored['originalSnapshotPreserved'])
    runtime = ROOT.parent / 'kura/kura-dev-runtime/target/runtime-netplan'
    inventory = json.loads((runtime / 'inventory.json').read_text())
    # inventory also contains launcher.jar and framework.jar outside plugins/.
    check('280 plugin bundles and all 282 artifact hashes match', len(inventory) == 282
          and sum(item['path'].startswith('plugins/') for item in inventory) == 280
          and all(sha(runtime / item['path']) == item['sha256'] for item in inventory))
    web = next(item for item in inventory if item['symbolicName'] == 'org.eclipse.kura.web2')
    updated = load('vm-web2-updated')['web2Update']
    check('VM received exact production web2', updated['replacement']['sha256'] == web['sha256']
          and updated['previous']['sha256'] != web['sha256'])
    with zipfile.ZipFile(runtime / web['path']) as jar:
        check('production web2 has no test probe', not any('/probe/' in name for name in jar.namelist()))
    result = dict(status='passed', checks=checks, passed=len(checks), gwtTests=counts, reactBrowser=stats,
                  transactions=dict(prepares=3, rollbacks=1, confirms=2, initialRevision=3, finalRevision=5),
                  developmentRuntime=True, productionPackageValidated=False, physicalManagementPathValidated=False)
    with output.open('x') as stream:
        stream.write(json.dumps(result, ensure_ascii=False, indent=2) + '\n')
    print(json.dumps(dict(status='passed', checks=len(checks), output=str(output))))


if __name__ == '__main__':
    main()
