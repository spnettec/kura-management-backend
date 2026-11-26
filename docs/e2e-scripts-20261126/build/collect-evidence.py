"""Evidence for the ETXTBSY defect: the same test suite on the source before and after the fix.

pre-fix:   the agent tree at the parent of the fix commit, exported and staged exactly like build-agent.py
           does it, with one addition: the regression test of the fix is appended to src/process.rs so
           that it can be run against the unfixed spawn() path. Its own target directory; same cargo cache,
           same container, same cargo flags.
post-fix:  the staged HEAD tree and the release target of the last rebuild (build-agent.py stage test).

Logs go to evidence/<name>.log, the numbers to evidence/summary.json. Nothing here is hand-written:
the manual reads these files.

Usage: python3 -B collect-evidence.py [--runs N]
"""
import argparse
import importlib.util
import io
import json
import re
import shutil
import subprocess as sp
import sys
import tarfile
from pathlib import Path

here = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('build_agent', here / 'build-agent.py')
ba = importlib.util.module_from_spec(spec)
spec.loader.exec_module(ba)

evidence = here / 'evidence'
exported, prefix_source, prefix_target = evidence / 'exported', evidence / 'prefix-source', evidence / 'prefix-target'
REGRESSION = 'transient_text_file_busy_from_a_concurrent_writer_is_waited_out'


def stage_prefix():
    fix = ba.read('git', '-C', str(ba.repo), 'rev-parse', ba.revision)
    parent = ba.read('git', '-C', str(ba.repo), 'rev-parse', fix + '^')
    shutil.rmtree(exported, ignore_errors=True)
    shutil.rmtree(prefix_source, ignore_errors=True)
    exported.mkdir(parents=True)
    with tarfile.open(fileobj=io.BytesIO(sp.check_output(['git', '-C', str(ba.repo), 'archive', parent]))) as archive:
        archive.extractall(exported, filter='data')
    code = ba.logged([sys.executable, '-B', exported / 'packaging/build.py', 'source', '--protocol',
                      ba.protocol['file'], '--sha256', ba.protocol['sha256'], '--destination', prefix_source], 'evidence/prefix-stage')
    assert code == 0, 'staging the pre-fix tree failed'
    # 把修复提交里新增的回归测试原样加到修复前的 process.rs 里（它是 mod tests 的最后一项）。
    fixed = sp.check_output(['git', '-C', str(ba.repo), 'show', fix + ':src/process.rs'], text=True)
    start = fixed.index('    #[test]\n    fn ' + REGRESSION)
    snippet = fixed[start:fixed.rindex('\n}\n') + 1]
    process = prefix_source / 'src/process.rs'
    before = process.read_text()
    assert REGRESSION not in before and before.rstrip().endswith('}')
    process.write_text(before.rstrip()[:-1] + '\n' + snippet + '}\n')
    return parent


def parse(log):
    text = log.read_text(errors='replace')
    results = [[int(v.rstrip(',')) for v in line.split()[3:8:2]] for line in text.splitlines() if line.startswith('test result:')]
    passed, failed, ignored = (sum(column) for column in zip(*results))
    return dict(passed=passed, failed=failed, ignored=ignored,
                failing=re.findall(r'^test (\S+) \.\.\. FAILED$', text, re.M),
                errors=sorted(set(re.findall(r'Err` value: ([^\n]+)', text))))


def run(name, source_dir, target_dir, *filter_):
    code = ba.cargo('test', '--release', '--locked', '--offline', *filter_, sandbox=False, log='evidence/' + name,
                    source_dir=source_dir, target_dir=target_dir)
    result = parse(evidence / (name + '.log'))
    result['exit'] = code
    print(name, json.dumps(result, ensure_ascii=False), flush=True)
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument('--runs', type=int, default=5, help='full-suite runs before and after the fix')
    parser.add_argument('--reuse-prefix', action='store_true', help='skip exporting the pre-fix tree again')
    options = parser.parse_args()
    evidence.mkdir(exist_ok=True)
    summary = {}
    if not options.reuse_prefix or not (prefix_source / 'Cargo.toml').exists():
        summary['prefixRevision'] = stage_prefix()
    prefix_target.mkdir(exist_ok=True)
    summary['prefixRegression'] = run('prefix-regression', prefix_source, prefix_target, REGRESSION)
    summary['prefixRuns'] = [run(f'prefix-full-run-{n}', prefix_source, prefix_target) for n in range(1, options.runs + 1)]
    summary['postfixRuns'] = [run(f'postfix-full-run-{n}', ba.source, ba.target) for n in range(1, options.runs + 1)]
    summary['postfixRevision'] = ba.read('git', '-C', str(ba.repo), 'rev-parse', ba.revision)
    (evidence / 'summary.json').write_text(json.dumps(summary, indent=2, ensure_ascii=False) + '\n')
    before = [r for r in summary['prefixRuns'] if r['failed']]
    after = [r for r in summary['postfixRuns'] if r['failed']]
    print(f"pre-fix: {len(before)}/{len(summary['prefixRuns'])} runs failed, failing tests {sorted({t for r in before for t in r['failing']})} ; "
          f"post-fix: {len(after)}/{len(summary['postfixRuns'])} runs failed", flush=True)


if __name__ == '__main__':
    main()
