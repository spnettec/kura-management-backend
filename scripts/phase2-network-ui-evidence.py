#!/usr/bin/env python3
"""Read-only verification of the named, already-restored GWT UI regression run."""
import hashlib
import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[1]
DEV = ROOT / '.phase1-dev'
TARGET = ROOT / 'target'
RUN_ID = 'ui-regression-20261101'


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def verify():
    checks = []

    def require(name, condition):
        checks.append({'name': name, 'passed': bool(condition)})

    for filename, expected in [('netplan-ui-react-browser-final.json', 11),
                               (f'gwt-network-{RUN_ID}-browser.json', 4)]:
        stats = json.loads((TARGET / filename).read_text())['stats']
        require(filename, stats['expected'] == expected and
                all(stats[key] == 0 for key in ['unexpected', 'flaky', 'skipped']))
    before = (TARGET / 'netplan-ui-regression-before.log').read_text()
    require('regressions reproduced before fix', 'Tests run: 16, Failures: 2, Errors: 0, Skipped: 0' in before)
    build = (TARGET / 'netplan-ui-regression-build.log').read_text()
    require('complete GWT build and 65 server tests', 'BUILD SUCCESS' in build and
            'Tests run: 65, Failures: 0, Errors: 0, Skipped: 0' in build)
    log = (TARGET / 'netplan-ui-idea-kura.log').read_text()
    require('status page hardware type log regression', 'Kura version YOFC SMART IOT EDGE GATEWAY' in log and
            'Error getting HwType' not in log)
    fixture = (TARGET / f'gwt-network-{RUN_ID}-fixture.txt').read_text()
    require('test owner counted real RPC writes only', all(line + '\n' in fixture for line in
            ['testOnly=true', 'systemNetworkWrites=0', 'prepares=4', 'confirms=1', 'rollbacks=1']))
    require('no token in fixture evidence', 'confirmationToken' not in fixture)
    for filename in ['network-gwt-profile-state.json', f'network-gwt-{RUN_ID}-state.json']:
        state = json.loads((DEV / filename).read_text())
        require(filename + ' restored', state['status'] == 'restored')
        for row in state['originals'] + state['artifacts']:
            require(filename + ': ' + str(Path(row['path']).relative_to(DEV)),
                    digest(Path(row['path'])) == row['sha256'] == digest(Path(row['backup'])))
        for path, sha in state['snapshots'].items():
            require(filename + ': snapshot retained', digest(Path(path)) == sha)
    rows = subprocess.run(['ps', 'ax', '-o', 'pid=,command='], check=True,
                          capture_output=True, text=True).stdout.splitlines()
    require('isolated Kura exited', not any('/bin/java ' in row and str(DEV / 'kura-runtime') in row for row in rows))
    result = subprocess.run(['python3', str(ROOT / 'scripts/phase2-network-gwt-runtime.py'),
                             'prepare', '--run-id', RUN_ID], capture_output=True, text=True)
    require('restored named fixture cannot be replayed', result.returncode != 0 and
            'Keep existing one-shot state/evidence; do not repeat prepare' in result.stderr)
    output = {'passed': sum(row['passed'] for row in checks), 'total': len(checks), 'checks': checks,
              'testOnlyNetworkOwner': True, 'systemNetworkWrites': 0, 'linuxRegression': 'pending'}
    (TARGET / 'netplan-ui-final-check.json').write_text(json.dumps(output, indent=2) + '\n')
    print(json.dumps(output, indent=2))
    if output['passed'] != output['total']:
        raise SystemExit(1)


if __name__ == '__main__':
    verify()
