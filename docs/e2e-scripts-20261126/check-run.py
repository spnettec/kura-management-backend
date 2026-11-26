#!/usr/bin/env python3
"""最终一轮的核对与留档：步骤是否全部 OK、管理中心是否 4/4 在线、日志里有没有错误行，并写 RESULT.md。

用法：check-run.py <运行目录> [--manifest <release-manifest.json>]

“零错误”分两层：每一部的退出码都是 0（steps.tsv），以及 transcripts/日志里没有被忽略的错误。
第二层用一组正则扫全部日志，命中逐条列出来供人判断——apt 的提示、脚本自己打印的“排除项”说明都算命中，
所以这里不自动下结论，只保证没有错误行被漏看。
"""
import argparse
import json
import re
import sys
from pathlib import Path

SUSPICIOUS = re.compile(
    r'(?i)\berror\b|\bfailed\b|\bfailure\b|\bfatal\b|traceback|panicked|\bdenied\b|\brefused\b|'
    r'Input/output|\bEIO\b|No such file|cannot |\bcannot\b|unable to|exit code [1-9]|\brc=[1-9]|E2E-EXIT [1-9]|'
    r'\bE: |W: .*(error|failed)|FAILED\b|TIMEOUT|not ready|未达标|\bFAIL\b')
# 明确属于正常输出的行：例如 agent 测试里 “0 failed”、脚本对保护性拒绝的说明文字
BENIGN = [
    re.compile(r'^test result: ok\. \d+ passed; 0 failed'),
    re.compile(r'(?i)0 failed'),
]


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument('run', type=Path)
    parser.add_argument('--manifest', type=Path)
    options = parser.parse_args()
    run = options.run.resolve()
    steps = [line.split('\t') for line in (run / 'steps.tsv').read_text().splitlines() if line.strip()]
    failed = [row for row in steps if row[1] != 'OK']
    verify = (run / 'verify.txt').read_text().strip().splitlines() if (run / 'verify.txt').exists() else []
    online = [v for v in verify if 'online=True' in v and 'kuraAvailable=True' in v]
    transcript = run / 'transcript.log'
    passed_banner = transcript.exists() and 'ALL STEPS PASSED' in transcript.read_text(errors='replace')

    hits, warns = [], []
    for path in sorted(list(run.glob('*.log')) + list((run / 'logs').glob('*.log'))):
        for number, line in enumerate(path.read_text(errors='replace').splitlines(), 1):
            line = re.sub(r'\x1b\[[0-9;]*m', '', line)
            if SUSPICIOUS.search(line) and not any(b.search(line) for b in BENIGN):
                hits.append(f'{path.relative_to(run)}:{number}: {line[:220]}')
            elif re.search(r'(?i)\bwarn', line):
                warns.append(f'{path.relative_to(run)}:{number}: {line[:220]}')

    manifest = json.loads(options.manifest.read_text()) if options.manifest and options.manifest.exists() else {}
    # 供手册引用：错误、警告逐条列出；警告再按文本汇总（只数 logs/ 里的，transcript 是它们的回显）
    warning_kinds = {}
    for entry in warns:
        if entry.startswith('logs/'):
            message = entry.split(': ', 1)[1]
            warning_kinds[message] = warning_kinds.get(message, 0) + 1
    (run / 'scan.json').write_text(json.dumps({'errors': hits, 'warnings': warns, 'warningKinds': warning_kinds},
                                              indent=2, ensure_ascii=False) + '\n')
    lines = ['# 最终一轮结果', '',
             f'- 运行目录：`{run}`',
             f'- 步骤：{len(steps)} 步，{len(steps) - len(failed)} 步 OK，{len(failed)} 步失败；transcript 末尾 {"有" if passed_banner else "没有"} `ALL STEPS PASSED`',
             f'- 管理中心终验：{len(online)}/4 台 online=True 且 kuraAvailable=True',
             f'- 日志扫描：{len(hits)} 行命中错误类模式，{len(warns)} 行命中警告类模式，逐条判断']
    if manifest:
        lines += ['', '## 产物', '',
                  f"- kura-core：`{Path(manifest['core']['deb']).name}`  SHA-256 `{manifest['core']['sha256']}`  源码 {manifest['core']['revision'][:10]}",
                  f"- kura-management-agent：`{Path(manifest['agent']['deb']).name}`  SHA-256 `{manifest['agent']['sha256']}`  源码 {manifest['agent']['revision'][:10]}",
                  f"- 管理中心镜像：`{manifest['console']['image']}`  源码 {manifest['console']['revision'][:10]}"]
    lines += ['', '## 步骤（steps.tsv 原文）', '', '| 时间 | 结果 | 步骤 | 说明 |', '|---|---|---|---|']
    lines += ['| ' + ' | '.join(row) + ' |' for row in steps]
    lines += ['', '## 管理中心终验（verify.txt 原文）', '', '```', *verify, '```', '', '## 日志扫描命中（错误类，逐条判断）', '']
    lines += ['```', *hits, '```'] if hits else ['（没有命中）']
    lines += ['', '## 截图', ''] + [f'- `{p.name}`' for p in sorted((run / 'screenshots').glob('*.png'))] if (run / 'screenshots').exists() else []
    (run / 'RESULT.md').write_text('\n'.join(lines) + '\n')
    print('\n'.join(lines[:6]))
    for hit in hits:
        print('  E', hit)
    for hit in warns:
        print('  W', hit)
    print(f'wrote {run / "RESULT.md"}')
    return 0 if not failed and len(online) == 4 and passed_banner else 1


if __name__ == '__main__':
    sys.exit(main())
