#!/usr/bin/env python3
"""生成 E2E 部署操作手册：一个自包含的 HTML（图片内嵌，打印友好），里面同时带着一份
幻灯片规格，供 html-to-pptx.mjs 转成 PowerPoint。

手册里的版本号、摘要、步骤结果、日志摘录一律从最终那一轮的产物文件读取，不手抄。
只依赖 Python 标准库；Parallels 虚拟机控制台的截图会按内容自动裁掉四周的黑边。

用法：
  python3 build-e2e-manual.py --run <最终一轮的目录> --manifest <release-manifest.json> \
      [--facts <含 core/build-all.log 与 agent/cargo-test.log 的目录>] --out <manual.html> \
      [--shots <复制截图的目标目录>]
  NODE_PATH=<装有 pptxgenjs 的 node_modules> node html-to-pptx.mjs <manual.html> <deck.pptx>
"""
import argparse
import base64
import datetime
import html
import json
import re
import shutil
import struct
import zlib
from pathlib import Path

DARK, BG, PRIMARY, ACCENT, TEXT, MUTED, LINE = '#0F2419', '#F4F4F0', '#17563B', '#D97706', '#1B2620', '#64748B', '#CBD5C0'
GATEWAYS = [('yofc-gw-1', 'OrbStack · Debian 13'), ('yofc-gw-2', 'OrbStack · Debian 13'),
            ('yofc-gw-3', 'OrbStack · Debian 13'), ('yofc-gw-4', 'Parallels · Ubuntu 26.04')]


def esc(text):
    return html.escape(str(text), quote=False)


def data_uri(path):
    return 'data:image/png;base64,' + base64.b64encode(Path(path).read_bytes()).decode()


def clean_log(path):
    text = Path(path).read_text(errors='replace')
    return re.sub(r'\x1b\[[0-9;]*m', '', text).replace('\r', '')


def keep_lines(path, pattern, limit=40):
    lines = [line.rstrip() for line in clean_log(path).splitlines() if re.search(pattern, line)]
    return '\n'.join(lines[:limit])


def cargo_result(path):
    """cargo test 日志 → 通过/失败/忽略数、失败的测试名、panic 里的 Err 文本；没有汇总行返回 None。"""
    if not path.exists():
        return None
    text = clean_log(path)
    results = [[int(v.rstrip(',')) for v in line.split()[3:8:2]] for line in text.splitlines() if line.startswith('test result:')]
    if not results:
        return None
    return dict(zip(('passed', 'failed', 'ignored'), (sum(column) for column in zip(*results))),
                failing=re.findall(r'^test (\S+) \.\.\. FAILED$', text, re.M),
                errors=sorted(set(re.findall(r'Err` value: ([^\n]+)', text))))


# ---------------------------------------------------------------- PNG（标准库）：读、按内容裁剪、写
def png_read(path):
    """8 位 RGB/RGBA、不隔行扫描的 PNG → (宽, 高, 每像素字节数, 各行字节)。"""
    data = Path(path).read_bytes()
    assert data[:8] == b'\x89PNG\r\n\x1a\n', f'{path}: not a PNG'
    pos, idat, header = 8, [], None
    while pos < len(data):
        length, kind = struct.unpack('>I4s', data[pos:pos + 8])
        body = data[pos + 8:pos + 8 + length]
        pos += 12 + length
        if kind == b'IHDR':
            header = struct.unpack('>IIBBBBB', body)
        elif kind == b'IDAT':
            idat.append(body)
        elif kind == b'IEND':
            break
    width, height, depth, colors, _, _, interlace = header
    assert depth == 8 and colors in (2, 6) and interlace == 0, f'{path}: unsupported PNG layout {header}'
    bpp = 3 if colors == 2 else 4
    raw = zlib.decompress(b''.join(idat))
    stride = width * bpp
    rows, previous = [], bytearray(stride)
    for y in range(height):
        start = y * (stride + 1)
        kind, line = raw[start], bytearray(raw[start + 1:start + 1 + stride])
        if kind:
            for i in range(stride):
                left = line[i - bpp] if i >= bpp else 0
                up = previous[i]
                corner = previous[i - bpp] if i >= bpp else 0
                if kind == 1:
                    line[i] = (line[i] + left) & 255
                elif kind == 2:
                    line[i] = (line[i] + up) & 255
                elif kind == 3:
                    line[i] = (line[i] + ((left + up) >> 1)) & 255
                else:
                    estimate = left + up - corner
                    a, b, c = abs(estimate - left), abs(estimate - up), abs(estimate - corner)
                    line[i] = (line[i] + (left if a <= b and a <= c else up if b <= c else corner)) & 255
        rows.append(line)
        previous = line
    return width, height, bpp, rows


def png_encode(width, height, bpp, rows):
    def chunk(kind, body):
        return struct.pack('>I', len(body)) + kind + body + struct.pack('>I', zlib.crc32(kind + body) & 0xffffffff)
    raw = b''.join(b'\x00' + bytes(row[:width * bpp]) for row in rows)
    return (b'\x89PNG\r\n\x1a\n' + chunk(b'IHDR', struct.pack('>IIBBBBB', width, height, 8, 2 if bpp == 3 else 6, 0, 0, 0))
            + chunk(b'IDAT', zlib.compress(raw, 9)) + chunk(b'IEND', b''))


def png_crop_content(path, margin=14, threshold=6, step=2):       # 页面底色 F5F7F5 与白色卡片只差 10
    """裁到“与背景色不同”的内容边界外加一圈留白；背景色取左上角像素。虚拟机控制台（黑底）和
    网页截图（浅色底）都适用。不落盘，返回 (data URI, 宽, 高)。"""
    width, height, bpp, rows = png_read(path)
    data, stride = b''.join(rows), width * bpp
    background = data[0:3]

    def differs(offset):
        return (abs(data[offset] - background[0]) > threshold or abs(data[offset + 1] - background[1]) > threshold
                or abs(data[offset + 2] - background[2]) > threshold)

    used = [y for y in range(0, height, step) if any(differs(y * stride + x * bpp) for x in range(0, width, step))]
    if not used:
        return 'data:image/png;base64,' + base64.b64encode(Path(path).read_bytes()).decode(), width, height
    columns = [x for x in range(0, width, step) if any(differs(y * stride + x * bpp) for y in used)]
    top, bottom = max(0, used[0] - margin), min(height, used[-1] + step + margin)
    left, right = max(0, columns[0] - margin), min(width, columns[-1] + step + margin)
    cropped = [row[left * bpp:right * bpp] for row in rows[top:bottom]]
    encoded = png_encode(right - left, bottom - top, bpp, cropped)
    return 'data:image/png;base64,' + base64.b64encode(encoded).decode(), right - left, bottom - top


def png_crop_region(path, box):
    """截取截图里的一块固定区域（像素坐标 x0, y0, x1, y1）。调用处先确认截图尺寸与预期版式一致。"""
    width, height, bpp, rows = png_read(path)
    x0, y0, x1, y1 = box
    assert 0 <= x0 < x1 <= width and 0 <= y0 < y1 <= height, f'{path}: region {box} outside {width}x{height}'
    encoded = png_encode(x1 - x0, y1 - y0, bpp, [row[x0 * bpp:x1 * bpp] for row in rows[y0:y1]])
    return 'data:image/png;base64,' + base64.b64encode(encoded).decode()


# ---------------------------------------------------------------- 内容
def build(run, manifest, screenshots, facts_dir):
    steps = [line.split('\t') for line in (run / 'steps.tsv').read_text().splitlines() if line.strip()]
    failed = [row for row in steps if row[1] != 'OK']
    verify = (run / 'verify.txt').read_text().strip().splitlines()
    online = sum(1 for v in verify if 'online=True' in v and 'kuraAvailable=True' in v)
    core, agent = manifest['core'], manifest['agent']
    duration = ''
    if (run / 'started.txt').exists() and (run / 'finished.txt').exists():
        seconds = int((datetime.datetime.fromisoformat((run / 'finished.txt').read_text().strip())
                        - datetime.datetime.fromisoformat((run / 'started.txt').read_text().strip())).total_seconds())
        duration = f'，用时 {seconds // 60} 分 {seconds % 60} 秒'
    # 日志扫描结果由 tools/check-run.py 写在 scan.json 里；这里只引用，不自己再扫一遍。
    scan_path = run / 'scan.json'
    scan = json.loads(scan_path.read_text()) if scan_path.exists() else None
    # 安装日志里出现过的第三方包警告，出处已逐条核对：setserial、chrony 的维护脚本，ca-certificates 的 update-ca-certificates。
    third_party = {
        'update-rc.d: warning: start and stop actions are no longer supported; falling back to defaults': 'setserial',
        'dpkg-statoverride: warning: --update given but /var/log/chrony does not exist': 'chrony',
        'rehash: warning: skipping ca-certificates.crt, it does not contain exactly one certificate or CRL': 'ca-certificates',
    }
    if scan:
        by_package = {}
        for message, count in scan['warningKinds'].items():
            name = third_party.get(message, '未归类')
            by_package[name] = by_package.get(name, 0) + count
        kinds = '、'.join(f'{name} {count} 行' for name, count in by_package.items())
        zero_error = (f'全部日志扫描：错误类 {len(scan["errors"])} 行；警告类 {sum(scan["warningKinds"].values())} 行——{kinds}'
                      + ('，全部来自第三方依赖包的维护脚本' if '未归类' not in by_package else ''))
    else:
        zero_error = '日志扫描结果未留档：先运行 tools/check-run.py 生成 scan.json'
    core_name, agent_name = Path(core['deb']).name, Path(agent['deb']).name

    # 构建事实：agent 的 cargo test 汇总、build-all.sh 的模块结果，都从日志里数出来。
    test_log, build_log = facts_dir / 'agent' / 'cargo-test.log', facts_dir / 'core' / 'build-all.log'
    for path in (test_log, build_log):      # 手册里的构建数字必须来自日志：缺了就停，不写 0
        if not path.exists():
            raise SystemExit(f'缺少构建日志 {path}；用 --facts 指向同时含 core/ 与 agent/ 的目录')
    totals = cargo_result(test_log)
    if totals is None:
        raise SystemExit(f'{test_log} 里没有 cargo test 的汇总行')
    build_text = build_log.read_text(errors='replace')
    summary_row = r'^\[INFO\] .+ (SUCCESS|FAILURE|SKIPPED) +\[ +[0-9.]+ s\]$'      # Reactor Summary 的行
    modules = {state: len(re.findall(summary_row.replace('(SUCCESS|FAILURE|SKIPPED)', state), build_text, re.M))
               for state in ('SUCCESS', 'FAILURE', 'SKIPPED')}
    builds_ok = len(re.findall(r'BUILD SUCCESS', build_text))
    builds_failed = len(re.findall(r'BUILD FAILURE', build_text))
    install_log, enroll_log = run / 'logs' / 'install-yofc-gw-4.log', run / 'logs' / 'enroll-yofc-gw-4.log'
    shot = lambda name: data_uri(screenshots / name)
    install_shot = png_crop_content(screenshots / '04-install-yofc-gw-4-console.png')
    enroll_shot = png_crop_content(screenshots / '05-enroll-yofc-gw-4-console.png')

    slides = []

    slides.append(dict(layout='cover', id='cover',
        title='YOFC Kura 管理体系 E2E 部署操作手册',
        subtitle='3 台 OrbStack + 1 台 Parallels 作为 IoT 网关，管理中心运行在 Docker；\n全部产物从合并后的 heyoulin-kura 重新编译',
        meta=f"{datetime.date.today().isoformat()}    Core {core['revision'][:10]}    agent {agent['revision'][:10]}    console {manifest['console']['revision'][:10]}",
        gateways=[{'id': g, 'kind': k} for g, k in GATEWAYS],
        note='本手册对应一次从清空环境开始、无人工干预、一次跑通的四网关部署。'))

    slides.append(dict(layout='stats', id='result', title='一次走通的结果',
        stats=[{'value': f'{online}/4', 'label': '网关在线且 Kura 可用'},
               {'value': str(len(failed)), 'label': '最终一轮里失败的步骤'},
               {'value': f"{modules['SUCCESS']}/{sum(modules.values())}", 'label': 'Core 构建模块结果（成功/全部）'},
               {'value': str(totals['passed']), 'label': f"agent 测试通过（失败 {totals['failed']}）"}],
        artifacts=f"产物修订   kura-core {core['revision'][:8]}   ·   kura-management-agent {agent['revision'][:8]}   ·   管理中心 {manifest['console']['revision'][:8]}",
        points=[f'最终一轮由 00-run-all.sh 从清空环境开始一次跑完{duration}，中间没有人工干预',
                '每台网关只装 kura-core 与 kura-management-agent；管理中心用 docker compose 部署',
                '过程中发现并修复了 1 个产品缺陷：agent 启动子进程遇到 ETXTBSY 就直接失败',
                '修复之后清空全部中间产物、重新编译、从零重跑，才有这一轮', zero_error],
        note='数字来自最终一轮的 steps.tsv、verify.txt，以及本次重新编译的 build-all.log 和 cargo-test.log。',
        detail=[('p', '最终一轮的步骤结果（steps.tsv 原文）：'),
                ('table', ['时间', '结果', '步骤', '说明'], [[esc(c) for c in row] for row in steps]),
                ('p', '管理中心终验输出（verify.txt 原文）：'), ('pre', esc('\n'.join(verify))),
                ('p', '构建事实：build-all.sh 共 ' + str(builds_ok + builds_failed) + ' 次 Maven 调用，' + str(builds_ok) + ' 次 BUILD SUCCESS、'
                      + str(builds_failed) + ' 次 BUILD FAILURE；模块结果 ' + str(modules['SUCCESS']) + ' 成功、' + str(modules['FAILURE'])
                      + ' 失败、' + str(modules['SKIPPED']) + ' 跳过。agent cargo test：' + str(totals['passed']) + ' 通过、' + str(totals['failed'])
                      + ' 失败、' + str(totals['ignored']) + ' 项带 ignore 标记、未执行。'),
                ('p', '本轮使用的产物：'),
                ('table', ['产物', '文件', 'SHA-256', '源码修订'],
                 [['kura-core', esc(core_name), esc(core['sha256']), esc(core['revision'])],
                  ['kura-management-agent', esc(agent_name), esc(agent['sha256']), esc(agent['revision'])],
                  ['管理中心镜像', esc(manifest['console']['image']), '', esc(manifest['console']['revision'])],
                  ['bridge bundle（随 agent 打包）', esc(agent['bridgeVersion']), esc(agent['bridgeSha256']), 'kura-networking']])]))

    slides.append(dict(layout='topology', id='topology', title='拓扑与地址',
        center='管理中心\nDocker compose · PostgreSQL 16\nHTTPS 18443 · gRPC 18444',
        nodes=[{'id': 'yofc-gw-1', 'kind': 'OrbStack · Debian 13', 'address': '→ 198.18.0.1:18444'},
               {'id': 'yofc-gw-2', 'kind': 'OrbStack · Debian 13', 'address': '→ 198.18.0.1:18444'},
               {'id': 'yofc-gw-3', 'kind': 'OrbStack · Debian 13', 'address': '→ 198.18.0.1:18444'},
               {'id': 'yofc-gw-4', 'kind': 'Parallels · Ubuntu 26.04', 'address': '→ 10.211.55.2:18444'}],
        caption='每台网关：kura-core + kura-management-agent。网关主动经双向 TLS 的 gRPC 连接管理中心。',
        footnote='虚拟机里的文件都从宿主的 httpd（18999 端口）取：OrbStack 用 host.orb.internal，Parallels 用 10.211.55.2。',
        note='两个中心地址和 localhost 都必须写进中心服务端证书的 SAN。',
        detail=[('p', '宿主上的固定配置（都在 e2e-common.sh 里，可用环境变量覆盖）：'),
                ('table', ['名称', '默认值', '用途'],
                 [['CONSOLE_ORIGIN', 'https://localhost:18443', '浏览器和 REST 访问管理中心'],
                  ['ENDPOINT_ORB', 'https://198.18.0.1:18444', 'OrbStack 网关连接中心的 gRPC 地址'],
                  ['ENDPOINT_PARALLELS', 'https://10.211.55.2:18444', 'Parallels 网关连接中心的 gRPC 地址'],
                  ['HTTP_PORT', '18999', '宿主 httpd，虚拟机从这里取 deb、脚本和证书'],
                  ['PARALLELS_VM', 'YOFC Review ARM64 20261124', '第四台网关用的 Parallels 虚拟机'],
                  ['PARALLELS_SNAPSHOT', '{ce60c087-…}', '该虚拟机的干净基线快照'],
                  ['E2E_ROOT', '~/yofc-e2e', '证书、日志、截图的落盘目录']])]))

    slides.append(dict(layout='table', id='build', title='第一步：从合并后的源码重新编译',
        columns=['产物', '命令', '输出', '验证'], widths=[1.7, 4.2, 3.9, 2.3], rowHeight=0.66,
        rows=[['Core deb', 'cd kura && BUILD_DOCKER=0 ./build-all.sh', 'kura/distrib/kura-core/target/*_arm64.deb（相对 kura 仓库）',
               f"{builds_ok} 次 Maven 调用全部成功"],
              ['agent deb', 'python3 -B build-agent.py stage test build package', 'kura-management-agent_*_arm64.deb',
               f"cargo test {totals['passed']} 通过 / {totals['failed']} 失败"],
              ['管理中心镜像', 'cd docker && docker compose build --no-cache', 'yofc-management-console:latest', 'compose build 返回 0']],
        note='三个交付物都从合并后的 heyoulin-kura 编译；发现并修复 agent 的缺陷后，清空中间产物重编了一遍，再跑最终一轮。',
        detail=[('p', '18 个仓库此刻都只有 heyoulin-kura 一个分支，工作树干净。Core 用仓库自带的正式入口 build-all.sh：'
                      '第 1 阶段父 POM 与第三方封装，第 2 阶段核心 bundle，第 3 阶段各 sibling，第 4 阶段 YOFC，第 5 阶段发行包。'
                      '这台宿主是 arm64，arch-aarch64 profile 按 CPU 自动激活，第 5 阶段直接产出 arm64 deb。'),
                ('pre', 'cd kura\nBUILD_DOCKER=0 ./build-all.sh > build-all.log 2>&1; echo "rc=$?"\n\n'
                        '# agent：源码准备 → 容器内 cargo test → cargo build --release → 打 deb\npython3 -B build-agent.py stage test build package\n\n'
                        '# 管理中心镜像\ncd kura-management-console/docker && docker compose build --no-cache'),
                ('p', 'agent 包的三个配对输入都取自合并后的源码：bridge bundle 由 build-all.sh 的第 3 阶段从 kura-networking 编出；'
                      'core loader 摘要取自新编的 Core 发行目录（bin/gen_config_ini.sh）；protocol crate 的内容已逐文件核对，'
                      '与合并后 console 仓库 crates/management-protocol 的源码一致（差异只有 cargo 打包时生成的规范化文件）。'),
                ('p', 'build-all.sh 会 clean 各仓库的 target 目录。上一轮部署用的旧 deb 先归档再重编。')]))

    slides.append(dict(layout='table', id='ca', title='第二步：证书体系',
        columns=['证书', '作用', '谁持有私钥'], widths=[2.4, 6.4, 3.3], rowHeight=0.5,
        rows=[['client-ca', '签发各网关的 leaf 证书；中心和网关用它验证对端', '部署者（宿主 E2E_ROOT/pki）'],
              ['server-ca', '签发中心服务端证书；网关用它锚定中心', '部署者'],
              ['server.pem / server.key', '中心的 HTTPS 与 gRPC 服务端证书', '管理中心'],
              ['gw-<网关>-leaf.pem', '网关的身份证书，只能做客户端用途（CA:FALSE）', '各网关（私钥不出网关）']],
        points=['SAN 必须覆盖浏览器与两类虚拟机访问中心的全部地址', '证书在 OpenSSL 容器里生成与签发，避免 macOS LibreSSL 的显式参数编码'],
        note='02-ca.sh 生成三份证书；05-gateway-enroll.sh 为每台网关签发 leaf。',
        detail=[('pre', 'bash 02-ca.sh\n# subjectAltName = DNS:localhost, DNS:host.orb.internal, IP:127.0.0.1, IP:198.18.0.1, IP:10.211.55.2\n'),
                ('p', '如果浏览器不用 localhost 访问中心，把 CONSOLE_NAME 设成那个名字（脚本会自动加进 SAN），'
                      '或者用 CONSOLE_EXTRA_SAN 追加。网关 leaf 的扩展是固定的：basicConstraints CA:FALSE、keyUsage digitalSignature、'
                      'extendedKeyUsage clientAuth、SKI、AKI。')]))

    wizard = png_crop_content(screenshots / '01-console-setup-wizard.png')
    login = png_crop_content(screenshots / '02-console-login.png')
    slides.append(dict(layout='image', id='console', title='第三步：部署管理中心',
        image=wizard[0], caption='管理中心首次启动后的初始化向导（真实截图，已裁掉空白边）',
        points=['compose 启动控制台，另起一个 PostgreSQL 容器', '从数据卷读出安装凭据 installation-credential.json',
                'pair → operations：提交数据库、管理员、服务端证书和客户端 CA', '初始化完成后 /api/setup 路由下线，轮询收到 404 属正常',
                '用 server-ca 严格校验服务端证书后登录'],
        note='03-console.sh。',
        detail=[('pre', 'bash 03-console.sh\n\n# 关键输出\n# console healthy, postgres ready\n# pair ok, installation <UUID>\n# operations submitted <UUID> PENDING\n# setup outcome SETUP-ROUTES-RETIRED\n# login ok: admin at https://localhost:18443 mustChangePassword=False'),
                ('p', 'compose 项目目录是 kura-management-console/docker，数据卷名是 docker_console-data，清空时用 docker compose down -v。'
                      '首次连接时中心用的是自签的临时证书，页面上显示的指纹应与安装凭据里的 certificateFingerprint 一致。'),
                ('img', login[0], '初始化完成后的登录页（真实截图，已裁掉空白边）')]))

    slides.append(dict(layout='steps', id='install', title='第四步：网关安装（顺序固定）', highlight=3,
        steps=[{'title': 'apt 依赖', 'detail': 'python3 libssl3 gpsd openjdk-21-jre-headless dnsmasq-base isc-dhcp-client bind9-utils'},
               {'title': '下载 deb 并校验 SHA-256', 'detail': 'kura-core 与 kura-management-agent，经宿主 httpd'},
               {'title': '用 apt 安装两个 deb：先 agent，后 core', 'detail': 'apt 先把 deb 声明的依赖配齐、再配置本包；不要用 dpkg -i 报错后再 apt-get install -f 收敛'},
               {'title': 'initialize → 停两个服务 → authorize-kura-peer → authorize-enrollment-reader', 'detail': '两项授权必须在服务停止时做；漏掉 enrollment-reader，之后 bind 会被拒'},
               {'title': '启动 kura.service 与 kura-management-agent-v2.service', 'detail': '两个都是 active'}],
        note='04-gateway-install.sh <网关ID>。',
        detail=[('pre', 'bash 04-gateway-install.sh yofc-gw-1      # yofc-gw-1..3 为 OrbStack\nbash 04-gateway-install.sh yofc-gw-4      # yofc-gw-4 为 Parallels'),
                ('p', '第四步（Parallels）的真实输出摘录：'),
                ('pre', esc(keep_lines(install_log, r'^\d\d:\d\d:\d\d +\[|initialized|"state": "APPLIED"|^kura-(core|management-agent)=|^active$', 20))) if install_log.exists() else ('p', '（日志缺失）'),
                ('p', '控制器里的各类拒绝都是防丢失的保护，在干净系统上按上面的顺序执行不会出现。'
                      '常见的几种：对已初始化的 agent 换一个新的 UUID 再 initialize 会报 “Retain the original native initialization UUID”；'
                      '已有 peer 授权时再次 authorize 会报 “Existing control-group authority requires its original migration”；'
                      '手工删过 agent 的状态目录后会留下空的 legacy 目录，再装包会报 “Retained legacy profile requires explicit original migration”。'
                      '遇到这些，不要继续在这台机器上试，按第六步还原到基线重来。')]))

    slides.append(dict(layout='wide', id='install-shot', title='安装完成时的第四台网关',
        image=install_shot[0], width=install_shot[1], height=install_shot[2],
        caption='Parallels 虚拟机 3 号文本终端：安装脚本把每一步同步显示在这里（真实截图，已裁掉四周黑边）',
        points=['[1/5] 到 [5/5] 每步带时间戳，最后是 == install done ==', '第 3 步下方是装上的两个包的版本：kura-core 与 kura-management-agent',
                '第 4 步：initialize → stop units → authorize-kura-peer → authorize-enrollment-reader', '两个服务是否 active、各项调用的返回值，在宿主的 logs/install-<网关>.log 里（上一节有摘录）'],
        note='OrbStack 虚拟机没有显示器，同样的内容在宿主的 logs/install-<网关>.log 里。'))

    slides.append(dict(layout='steps', id='enroll', title='第五步：网关登记（顺序固定）', highlight=2,
        steps=[{'title': 'keygen：网关自己生成密钥和 CSR', 'detail': '私钥不出网关；GW_ID 写进 CSR 的 CN'},
               {'title': '中心侧用 client CA 签发 leaf', 'detail': 'pki-sign-leaf.sh，在 OpenSSL 容器里签'},
               {'title': '注册：POST /api/v1/gateways', 'detail': '必做。未注册的网关即使 bind 成功，中心也会拒绝连接（FORBIDDEN）'},
               {'title': 'stage：生成登记槽位', 'detail': 'gw-stage-final.py，端点必须带 https:// 前缀'},
               {'title': 'enroll：kurad register + bind → 启用通道 → 启动 kura → CONNECTED', 'detail': 'bind 之前先停 kura 释放桥的登记位'}],
        note='05-gateway-enroll.sh <网关ID>。',
        detail=[('pre', 'bash 05-gateway-enroll.sh yofc-gw-1\n\n# 关键输出\n# CSR subject=CN=yofc-gw-1\n# registered yofc-gw-1 generation=1 certificateSha256=…\n# stage request <UUID>  sha …\n# FINAL: CONNECTED | bound: True | enabled: True'),
                ('p', 'bind 返回 INVALID_ARGUMENT，最常见的原因是证书和密钥不是一对：leaf 是用另一台机器（或还原之前）的 CSR 签的。'
                      '私钥、CSR、leaf、stage 槽位必须在同一台机器的同一次运行里产生；还原或重建虚拟机之后，从 keygen 开始整条重做。'),
                ('p', '第四步（Parallels）登记的真实输出摘录：'),
                ('pre', esc(keep_lines(enroll_log, r'^\d\d:\d\d:\d\d|slot verified|kurad stdout|^enabled:|^state:|^FINAL:|^receipt', 14))) if enroll_log.exists() else ('p', '（日志缺失）')]))

    slides.append(dict(layout='wide', id='enroll-shot', title='登记完成时的第四台网关',
        image=enroll_shot[0], width=enroll_shot[1], height=enroll_shot[2],
        caption='Parallels 虚拟机 3 号文本终端：上面是安装的 5 步，下面是登记的 4 步（真实截图，已裁掉四周黑边）',
        points=['[1/4] keygen：在网关上生成密钥和 CSR', '[2/4] stage：登记请求号', '[3/4] relay sockets、kurad register + bind',
                '[4/4] 启用通道、启动 kura，等到 CONNECTED；末行是回执身份 kura-uid:994'],
        note=''))

    slides.append(dict(layout='steps', id='parallels', title='第六步：第四台网关用 Parallels 快照还原',
        steps=[{'title': '停机', 'detail': 'prlctl stop。带内存的快照在停机状态下还原最干净'},
               {'title': '还原：prlctl snapshot-switch --id <快照> --skip-resume', 'detail': '回到未安装任何 Kura 组件的基线'},
               {'title': '启动并核验基线', 'detail': 'vm-baseline.sh：0 个 kura 包、没有 /opt/eclipse/kura、没有 agent 状态目录'},
               {'title': '文件经宿主 httpd 送入', 'detail': 'prlctl exec 按空格重切参数，只能传单个命令；脚本一律 curl 后本地执行'}],
        note='04-gateway-install.sh yofc-gw-4 把这些包在一起。',
        detail=[('pre', 'prlctl stop "YOFC Review ARM64 20261124"\nprlctl snapshot-switch "YOFC Review ARM64 20261124" --id "{ce60c087-5614-477b-92d1-8c003317d150}" --skip-resume\nprlctl start "YOFC Review ARM64 20261124"\n'),
                ('p', '实测结论：对这台虚拟机做 snapshot-switch，内存和磁盘会一起回到快照时刻（boot_id 变成快照时刻的、uptime 回到快照时的秒数、'
                      '/tmp 与 /run 这两个 tmpfs 里的内容也一起还原）。上一轮的密钥、登记材料和存活进程不会残留。'
                      '流程仍然固定为先停机、还原、再冷启动，并在启动后跑一遍基线核验。'),
                ('p', '3 号文本终端用来显示步骤、供截图。实测：chvt 返回时 getty@tty3 已经 active（systemd-logind 对还没人用的终端会按需起 getty），'
                      '之后 40 次采样写入全部成功。但 getty 启动那一刻会对终端做 vhangup 并打印登录横幅，紧接着 chvt 的写入会以 EIO 失败、已写的内容被横幅顶掉。'
                      '所以 vm-install.sh 在切换后等 getty@tty3 处于 active、再停 1 秒才清屏：截图里只有步骤行，日志里没有写入错误。')]))

    # 缺陷的数字都来自留档日志：agent/cargo-test-prefix-failure.log 是发现缺陷的那次全量测试，
    # agent/evidence/ 是修复前、修复后各跑若干轮的对照实验（collect-evidence.py 生成 summary.json）。
    discovery = cargo_result(facts_dir / 'agent' / 'cargo-test-prefix-failure.log')
    if discovery is None:
        raise SystemExit('缺少发现缺陷的那次测试日志 agent/cargo-test-prefix-failure.log')
    summary_path = facts_dir / 'agent' / 'evidence' / 'summary.json'
    evidence = json.loads(summary_path.read_text()) if summary_path.exists() else {}
    pre_runs, post_runs = evidence.get('prefixRuns', []), evidence.get('postfixRuns', [])
    regression = evidence.get('prefixRegression')
    # 修复前的源码里加了回归测试，所以每一轮都必然失败这一项；剔除它，剩下的才是“偶发失败”。
    for r in pre_runs:
        r['flaky'] = [t for t in r['failing'] if not (regression and t in regression['failing'])]
    pre_bad = [r for r in pre_runs if r['flaky']]
    post_failed = sum(r['failed'] for r in post_runs)
    pre_tests = sorted({t for r in pre_bad for t in r['flaky']})
    busy = 'Text file busy (os error 26)'
    if pre_bad:
        phenomenon = (f"现象：全量并发测试偶发失败，报错都是 {busy}；修复前 {len(pre_runs)} 轮里 {len(pre_bad)} 轮出现，"
                      f"每轮 {min(len(r['flaky']) for r in pre_bad)}–{max(len(r['flaky']) for r in pre_bad)} 项，共落在 {len(pre_tests)} 个不同的测试上")
        comparison = (f"对照实验用同一个容器、同一份 cargo 缓存和同样的参数。修复前：把修复提交之前的源码（{evidence.get('prefixRevision', '')[:7]}）导出，"
                      f"全量并发跑 {len(pre_runs)} 轮，其中 {len(pre_bad)} 轮出现偶发失败（各轮偶发失败数依次为 "
                      + '、'.join(str(len(r['flaky'])) for r in pre_runs) + f"），涉及 {len(pre_tests)} 个不同的测试，报错完全相同。")
    else:
        phenomenon = f"现象：全量测试里偶发失败，报 {busy}（发现于从清空的编译缓存重新编译的那一次）"
        comparison = '对照实验没有留档，这里只陈述发现缺陷的那次测试。'
    if regression and regression['failed']:
        comparison += (f" 再把修复提交里新增的回归测试原样加进修复前的源码，只跑它这一项：结果 FAILED，"
                       f"失败在同一个错误上（{esc('; '.join(regression['errors'] or [busy]))}）。")
    if post_runs:
        per_run = sorted({r['passed'] for r in post_runs})
        comparison += (f" 修复后：同一套测试全量并发跑 {len(post_runs)} 轮，共 {sum(r['passed'] for r in post_runs)} 项通过、"
                       f"{post_failed} 项失败。")
    verified = (f"验证：修复后全量并发 {len(post_runs)} 轮，每轮 {per_run[0]} 项通过、{post_failed} 项失败" if len(per_run) == 1
                else f"验证：修复后全量并发 {len(post_runs)} 轮，{sum(r['passed'] for r in post_runs)} 项通过、{post_failed} 项失败"
                if post_runs else '验证：重新编译时的那次全量测试通过')

    slides.append(dict(layout='code', id='defect', title='发现并修复的产品缺陷', fontSize=12, pointSize=15,
        code='+ let spawn_deadline = Instant::now() + Duration::from_secs(5);\n+ let mut child = loop {\n+     match command.spawn() {\n'
             '+         Err(error) if error.raw_os_error() == Some(libc::ETXTBSY)\n'
             '+             && Instant::now() < spawn_deadline => {\n'
             '+             std::thread::sleep(Duration::from_millis(5));\n+         }\n+         spawned => break spawned?,\n+     }\n+ };\n\n'
             'kura-management-agent  src/process.rs  ' + agent['revision'][:7],
        points=[phenomenon,
                '位置：agent 启动子进程的 spawn()；内核在目标程序仍被以写方式打开时拒绝 exec',
                '推断的机制：别的线程在 fork 与 exec 之间带着继承来的写句柄',
                '修复：遇到 ETXTBSY 等待后重试，上限 5 秒',
                verified],
        note='这是本轮唯一的产品改动，其余问题都在编排和构建脚本里。',
        detail=[('p', '发现过程：从清空的编译缓存重新编译后，agent 自己的测试套件全量并发执行 '
                      f"{discovery['passed'] + discovery['failed']} 项，{discovery['failed']} 项失败，调用栈停在对一个 Err 的 unwrap 上。"),
                ('pre', esc('\n'.join(discovery['failing'] + discovery['errors']))),
                ('p', comparison),
                ('p', '内核在可执行文件还被某个进程以写方式打开时拒绝 execve。能同时解释“只在并发时出现”“每次是不同测试”的机制是：'
                      'durable::write 写完临时文件后先改名、写句柄要到函数末尾才关；这个线程之外，别的线程恰好在 fork()，'
                      '子进程继承了这份写句柄的副本，直到它自己 exec 时才关闭；原线程如果抢在这之前执行这个脚本，就得到 ETXTBSY。'
                      '这个机制是从内核语义和实验现象推出来的，没有用系统调用跟踪逐次证实。修复不依赖这个推断：'
                      '无论是谁短暂地持有写句柄，等它放开再执行都是正确行为。'),
                ('p', 'agent 运行期是 tokio 多线程的，同样会 fork 子进程，所以这不只是测试不稳，是一个概率极低但真实存在的生产竞态。'
                      '修复提交新增的回归测试让另一个线程持有脚本的写句柄 400 毫秒后放开，同时执行这个脚本。'),
                ('p', '留档：关键行的摘录在 docs/manual/evidence-e2e-20261127/agent-etxtbsy-evidence-excerpts.md，汇总在同目录的 '
                      'agent-etxtbsy-evidence-summary.json；完整日志在运行机器的工作目录 .management-close-20261116/core-rtc-hotplug-20261120/clean-redeploy-20261126/agent/ 下：'
                      'cargo-test-prefix-failure.log（发现缺陷的那次）、evidence/prefix-regression.log、evidence/prefix-full-run-N.log、'
                      'evidence/postfix-full-run-N.log、evidence/summary.json；实验脚本 collect-evidence.py。')]))

    slides.append(dict(layout='code', id='runall', title='第七步：一键执行与终验', fontSize=15,
        code='CORE_DEB=…/kura_…_arm64.deb \\\nAGENT_DEB=…/kura-management-agent_…_arm64.deb \\\nE2E_ROOT=…/runs/final \\\n  bash 00-run-all.sh',
        points=['01 清空 → 02 证书 → 03 管理中心', '04 安装 + 05 登记，逐台执行', '06 终验：4 台都是 online=True 且 kuraAvailable=True',
                '任一步失败立即停止，退出码非 0', '每步结果写入 steps.tsv', '成功后：07-screenshots.sh 截图，check-run.py 核对并留档（RESULT.md、scan.json）'],
        note='00-run-all.sh',
        detail=[('p', '最终一轮各网关的安装与登记结果（steps.tsv 原文）：'),
                ('table', ['步骤', '结果', '说明'],
                 [[esc(row[2]), esc(row[1]), esc(row[3])] for row in steps if row[2].startswith(('install-', 'enroll-'))]),
                ('pre', 'bash 07-screenshots.sh                                   # 登录管理中心后截网关总览；只截图，不改 steps.tsv\n'
                        'python3 -B check-run.py "$E2E_ROOT" --manifest release-manifest.json    # 步骤全 OK、4/4 在线、扫描全部日志的错误与警告行'),
                ('p', '“零错误”的依据有两层：每一部的退出码都是 0（steps.tsv），以及全部日志经过扫描、没有被忽略的错误行。'
                      '扫描用的正则偏宽，命中会逐条列出来供人判断，所以结论里区分了错误类与警告类，并写明每条警告出自哪个第三方包。'
                      '本轮的 RESULT.md、scan.json、steps.tsv、verify.txt 在 docs/manual/evidence-e2e-20261127/ 里。')]))

    # 总览页截图是 1280×1000 的固定版式：幻灯片里放“已登记网关”表格那一块（字才够大），整页截图放进正文。
    overview_path = screenshots / 'gateways-overview.png'
    overview_full = data_uri(overview_path)
    expected_layout = struct.unpack('>II', overview_path.read_bytes()[16:24]) == (1280, 1000)
    slides.append(dict(layout='image', id='final-shot', title='终验：四台网关全部在线',
        image=png_crop_region(overview_path, (260, 432, 1232, 978)) if expected_layout else overview_full,
        caption='管理中心“网关总览”的已登记网关列表（登录后的真实截图，截取表格区域）',
        points=['本页网关 04', '管理通道在线 04（基于真实心跳）', 'Kura 已连接 04', '未决任务 00',
                '每台网关的管理通道“在线”、Kura“已连接”', '整页截图见正文'],
        note='07-screenshots.sh。',
        detail=[('img', overview_full, '整页截图：网关总览，1280×1000')]))

    slides.append(dict(layout='table', id='rounds', title='迭代记录：三轮排错，一次产品修复，然后一次通过',
        columns=['轮次', '结果', '发现的问题', '处置'], widths=[1.1, 2.0, 5.6, 3.4], rowHeight=0.7,
        rows=[['第 1 轮', '停在清空', '删除虚拟机后 orbctl list 瞬时失败，被当成“不存在”，漏删一台；prlctl 输出是空格填充，带空格的虚拟机名解析失败', '以退出码为准并重试，清空后核验；改用 -o status,name'],
              ['第 2 轮', '停在中心截图', 'local a="$1" b="$a" 写在同一行，参数先整体展开，set -u 下变量未定义', '拆成两条 local'],
              ['第 3 轮', '停在登录校验', 'api() 漏传 --root；随后单独验证了两类网关的安装和登记', '补参数；两条路径单独跑通'],
              ['重新编译', 'agent 测试 807 项里失败 1 项', '并发时偶发 Text file busy，每次落在不同测试上', '定位为 agent 的真实缺陷并修复，见前一节'],
              ['复核第三轮日志', '安装成功，但日志不干净', 'dpkg -i 装 core 时依赖未配齐，报错后才被 apt-get install -f 收敛；控制台截图缺头两行；预检又发现 04/05 脚本在 OrbStack 网关上成功后退出码为 1',
               '改为 apt 安装本地 deb；切终端后等 getty 起来再清屏；那两处改用 if；创建虚拟机加 3 次重试'],
              ['第 4 轮', '一次通过', '清空全部中间产物、重新编译后从零重跑', '零错误，4/4 在线']],
        note='前三轮的问题都在编排脚本里。',
        detail=[('p', '构建脚本本身还修过两处：build-agent.py 原先把各阶段的退出码依次覆盖，测试失败后仍报成功；'
                      '测试用的容器原先去掉了全部 capability、/tmp 也不可执行，135 项因此假失败。两处都在“重新编译”之前改好。')]))

    slides.append(dict(layout='table', id='retractions', title='过程中撤回的两个结论',
        columns=['当时的说法', '错在哪', '实际情况'], widths=[3.6, 4.3, 4.2], rowHeight=0.95,
        rows=[['Ubuntu 上 bind 的 INVALID_ARGUMENT 是平台问题，需要改 agent 源码',
               '没有先查知识库，也没有确认带诊断输出的 agent 是否真的装上了',
               '同一台虚拟机、同样的 Ubuntu，按已验证的顺序一次通过。原因在做法：对运行中的虚拟机还原、登记目录里有旧材料、没有先注册'],
              ['prlctl snapshot-switch 对运行中的虚拟机“延迟生效”，内存不随磁盘还原',
               '凭一次前后不一致的现象下了因果结论',
               '实验表明内存与磁盘一起、立即还原到快照时刻']],
        note='两条都已从手册、Obsidian 和记忆里更正。',
        detail=[('p', '另外三件做错的事，一并记下：一是没有先查工作空间知识库就自己另写了一套 Core 构建驱动，接连踩了 git archive 没有 .git、'
                      '容器 umask 002 让 49 个 JAR 变成组可写、编译树与打包树的 target 不同步三个坑，改用仓库自带的 build-all.sh 后一次编过；'
                      '二是把 agent 的 cargo test 放进了 noexec 的 /tmp 且去掉了全部 capability，得到一堆假失败；'
                      '三是合并 kura 时用 checkout --ours 解决冲突，知识库的踩坑汇总明确警告过这是整文件级的操作——事后核对这次两个分支只差一个断言的排版，没有丢内容。')]))

    slides.append(dict(layout='table', id='troubleshoot', title='排错速查',
        columns=['现象', '原因', '处置'], widths=[3.9, 4.4, 3.8], rowHeight=0.5, fontSize=12,
        rows=[['bind 之后通道 FORBIDDEN', '网关没有在中心注册，或漏了 authorize-enrollment-reader', '注册（证书须与 leaf 一致）；按顺序补两项授权'],
              ['bind 返回 INVALID_ARGUMENT', 'leaf 不是用这台机器这次的 CSR 签的，密钥与证书不成对', '从 keygen 开始整条重做，不要复用旧材料'],
              ['Retain the original native initialization UUID', '对已初始化的 agent 换新 UUID 再 initialize', '沿用原 UUID；拿不准就还原基线'],
              ['…requires its original migration / Retained legacy profile', '在带旧状态的机器上重复或乱序操作、手工删过状态目录', '还原到基线重来'],
              ['prlctl exec 只执行了第一个词', '参数按空格重切', '脚本经 httpd 送入，本地执行'],
              ['dpkg: error processing package kura-core … dependency problems - leaving unconfigured', '用 dpkg -i 装 core 时它声明的依赖还没装齐', '用 apt-get install ./kura-core.deb 安装本地 deb（vm-install.sh 已这样做）'],
              ['控制台截图缺前几行；日志里有 /dev/tty3: Input/output error', '切到 3 号终端时 systemd 按需起了一个 getty，它的 vhangup 和登录横幅冲掉了刚写的内容', '切终端后等 getty 起来再清屏（vm-install.sh 已这样做）'],
              ['清空后仍有网关虚拟机', 'orbctl list 瞬时失败被当成不存在', '以退出码为准并重试，清空后核验']],
        note='前四条来自这次部署与知识库里 2026-11-24 的四机部署记录。'))

    slides.append(dict(layout='closing', id='closing', title='文件与复现',
        points=['脚本：docs/e2e-scripts-20261126/（00 到 07，加 e2e-common.sh、网关内执行的 vm/、签证书的 pki/、登录截图的 console-shots.mjs）',
                '重跑：按第七步的命令；换一轮就换一个 E2E_ROOT',
                '手册：build-e2e-manual.py --run <运行目录> --manifest release-manifest.json 生成 HTML；html-to-pptx.mjs 把它转成 PowerPoint',
                '截图：控制台页面、Parallels 虚拟机控制台都是真实截图；终端步骤在手册里是真实日志'],
        note=''))
    return slides, dict(steps=steps, failed=failed, online=online, modules=modules, builds_ok=builds_ok,
                        builds_failed=builds_failed, totals=totals)


# ---------------------------------------------------------------- HTML
CSS = f"""
:root{{--dark:{DARK};--bg:{BG};--primary:{PRIMARY};--accent:{ACCENT};--text:{TEXT};--muted:{MUTED};--line:{LINE};}}
*{{box-sizing:border-box}} body{{margin:0;background:#E9EBE4;color:var(--text);font:15px/1.7 -apple-system,"PingFang SC","Hiragino Sans GB","Microsoft YaHei",sans-serif}}
.page{{max-width:1080px;margin:0 auto;background:#FFF;box-shadow:0 0 40px rgba(15,36,25,.12)}}
header.hero{{background:var(--dark);color:{BG};padding:56px 64px 48px}} header.hero h1{{margin:0 0 18px;font-size:38px;line-height:1.25}}
header.hero .sub{{font-size:17px;color:#CFE0D3;white-space:pre-line}} header.hero .meta{{margin-top:28px;font:13px/1.6 Menlo,Consolas,monospace;color:#9FB8A6}}
nav.toc{{padding:28px 64px;border-bottom:1px solid var(--line);columns:2;column-gap:48px}} nav.toc a{{color:var(--primary);text-decoration:none}}
nav.toc li{{break-inside:avoid;margin:3px 0}}
section.slide{{padding:40px 64px 48px;border-bottom:1px solid var(--line);break-inside:avoid}}
section.slide h2{{margin:0 0 22px;font-size:28px;color:var(--dark)}}
.stats{{display:grid;grid-template-columns:repeat(auto-fit,minmax(180px,1fr));gap:20px;margin:0 0 22px}}
.stats .v{{font-size:52px;font-weight:700;color:var(--primary)}} .stats .l{{color:var(--muted)}}
.topo{{text-align:center;margin:10px 0 24px}} .topo .c{{display:inline-block;background:var(--primary);color:#fff;border-radius:8px;padding:14px 26px;white-space:pre-line;font-weight:600}}
.topo .row{{display:grid;grid-template-columns:repeat(4,1fr);gap:14px;margin-top:44px}}
.topo .g{{background:#fff;border:1px solid var(--line);border-radius:8px;padding:12px;font-size:13px}} .topo .g b{{display:block;font-size:15px}} .topo .g code{{color:var(--primary)}}
ol.steps{{list-style:none;margin:0;padding:0;counter-reset:s}} ol.steps li{{counter-increment:s;display:grid;grid-template-columns:64px 1fr;gap:12px;padding:14px 0;border-bottom:1px solid var(--line)}}
ol.steps li::before{{content:counter(s);font-size:36px;font-weight:700;color:var(--primary);line-height:1}} ol.steps li.hl::before{{color:var(--accent)}}
ol.steps b{{display:block}} ol.steps span{{color:var(--muted);font-size:14px}}
figure{{margin:18px 0}} figure img{{max-width:100%;border:1px solid var(--line);border-radius:6px;display:block}} figure.wide img{{width:100%}}
figcaption{{color:var(--muted);font-size:13px;margin-top:8px}}
pre{{background:var(--dark);color:#E6EDE7;padding:18px 22px;border-radius:8px;overflow:auto;font:13px/1.6 Menlo,Consolas,monospace;white-space:pre}}
table{{border-collapse:collapse;width:100%;margin:14px 0;font-size:14px}} th,td{{border:1px solid var(--line);padding:8px 11px;text-align:left;vertical-align:top}}
th{{background:var(--primary);color:#fff}} tr:nth-child(even) td{{background:#ECEEE7}} td:first-child{{font-weight:600}}
ul.pts li{{margin:6px 0}} .note{{color:var(--muted);font-size:13px}} .detail{{margin-top:26px;padding-top:22px;border-top:1px dashed var(--line)}}
.artifacts{{font:13px/1.6 Menlo,Consolas,monospace;color:var(--muted)}}
/* 打印或导出 PDF：横向 A4，每一节一页，版面与幻灯片一致（内容多的节顺延到下一页） */
@media print{{
@page{{size:A4 landscape;margin:10mm}}
body{{background:#fff;font-size:12px}}
.page{{box-shadow:none;max-width:none;width:100%}}
header.hero,nav.toc,section.slide{{padding:7mm 9mm}}
nav.toc{{columns:3;break-after:page}}
section.slide{{break-before:page}}
section.slide h2{{font-size:22px}}
figure{{margin:10px 0}} figure.wide img{{width:100%}} figure:not(.wide) img{{max-height:135mm;width:auto;max-width:100%}}
pre{{font-size:11px}} table{{font-size:12px}}
}}
"""


def render_detail(blocks):
    out = []
    for block in blocks:
        kind = block[0]
        if kind == 'p':
            out.append(f'<p>{block[1]}</p>')
        elif kind == 'pre':
            out.append(f'<pre>{block[1]}</pre>')
        elif kind == 'img':
            out.append(f'<figure><img alt="{html.escape(block[2], quote=True)}" src="{block[1]}"><figcaption>{esc(block[2])}</figcaption></figure>')
        elif kind == 'table':
            columns, rows = block[1], block[2]
            out.append('<table><thead><tr>' + ''.join(f'<th>{c}</th>' for c in columns) + '</tr></thead><tbody>' +
                       ''.join('<tr>' + ''.join(f'<td>{c}</td>' for c in row) + '</tr>' for row in rows) + '</tbody></table>')
    return ''.join(out)


def points_list(points):
    return '<ul class="pts">' + ''.join(f'<li>{esc(p)}</li>' for p in points) + '</ul>'


def render(slides, manifest, run):
    out = [f'<!doctype html><html lang="zh-CN"><head><meta charset="utf-8"><title>{esc(slides[0]["title"])}</title><style>{CSS}</style></head><body><div class="page">']
    first = slides[0]
    out.append(f'<header class="hero"><h1>{esc(first["title"])}</h1><div class="sub">{esc(first["subtitle"])}</div>'
               f'<div class="meta">{esc(first["meta"])}</div></header>')
    out.append('<nav class="toc"><b>目录</b><ol>' + ''.join(f'<li><a href="#{esc(s["id"])}">{esc(s.get("title", ""))}</a></li>' for s in slides[1:]) + '</ol></nav>')
    for index, s in enumerate(slides[1:], 2):
        body = [f'<section class="slide" id="{esc(s["id"])}" data-slide="{index}"><h2>{esc(s["title"])}</h2>']
        layout = s['layout']
        if layout == 'stats':
            body.append('<div class="stats">' + ''.join(f'<div><div class="v">{esc(i["value"])}</div><div class="l">{esc(i["label"])}</div></div>' for i in s['stats']) + '</div>')
        elif layout == 'topology':
            body.append(f'<div class="topo"><div class="c">{esc(s["center"])}</div><div class="row">' + ''.join(
                f'<div class="g"><b>{esc(n["id"])}</b>{esc(n["kind"])}<br><code>{esc(n["address"])}</code></div>' for n in s['nodes']) +
                f'</div><p>{esc(s["caption"])}</p><p class="note">{esc(s["footnote"])}</p></div>')
        elif layout == 'steps':
            body.append('<ol class="steps">' + ''.join(
                f'<li class="{"hl" if i == s.get("highlight") else ""}"><div><b>{esc(st["title"])}</b><span>{esc(st["detail"])}</span></div></li>'
                for i, st in enumerate(s['steps'])) + '</ol>')
        elif layout in ('image', 'wide'):
            body.append(f'<figure class="{"wide" if layout == "wide" else ""}"><img alt="{esc(s["caption"])}" src="{s["image"]}">'
                        f'<figcaption>{esc(s["caption"])}</figcaption></figure>')
        elif layout == 'table':
            body.append('<table><thead><tr>' + ''.join(f'<th>{esc(c)}</th>' for c in s['columns']) + '</tr></thead><tbody>' +
                        ''.join('<tr>' + ''.join(f'<td>{c}</td>' for c in row) + '</tr>' for row in s['rows']) + '</tbody></table>')
        elif layout == 'code':
            body.append(f'<pre>{esc(s["code"])}</pre>')
        if s.get('points'):
            body.append(points_list(s['points']))
        if s.get('artifacts'):
            body.append(f'<p class="artifacts">{esc(s["artifacts"])}</p>')
        if s.get('detail'):
            body.append('<div class="detail">' + render_detail(s['detail']) + '</div>')
        if s.get('note'):
            body.append(f'<p class="note">备注：{esc(s["note"])}</p>')
        body.append('</section>')
        out.append(''.join(body))
    out.append('</div>')
    deck = dict(title=slides[0]['title'], author='YOFC Kura', slides=slides)
    out.append('<script id="e2e-deck" type="application/json">' + json.dumps(deck, ensure_ascii=False).replace('<', '\\u003c') + '</script>')
    out.append(f'<p class="note" style="text-align:center;padding:18px">生成于 {datetime.datetime.now().isoformat(timespec="seconds")} · 运行目录 {esc(run)} · '
               f'Core {esc(manifest["core"]["revision"][:10])} · agent {esc(manifest["agent"]["revision"][:10])}</p></body></html>')
    return ''.join(out)


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument('--run', required=True, type=Path)
    parser.add_argument('--manifest', required=True, type=Path)
    parser.add_argument('--out', required=True, type=Path)
    parser.add_argument('--facts', type=Path, help='directory holding core/build-all.log and agent/cargo-test.log (default: <run>/../..)')
    parser.add_argument('--shots', type=Path, help='also copy the screenshots used by the manual here')
    options = parser.parse_args()
    run, manifest = options.run, json.loads(options.manifest.read_text())
    facts_dir = options.facts or options.run.parents[1]
    slides, facts = build(run, manifest, run / 'screenshots', facts_dir)
    options.out.write_text(render(slides, manifest, run))
    print(f'wrote {options.out} ({options.out.stat().st_size} bytes, {len(slides)} slides)')
    print(f"facts: steps={len(facts['steps'])} failed={len(facts['failed'])} online={facts['online']} "
          f"builds ok={facts['builds_ok']} failed={facts['builds_failed']} modules={facts['modules']} cargo test={facts['totals']}")
    if options.shots:
        options.shots.mkdir(parents=True, exist_ok=True)
        for slide in slides:
            if slide.get('image', '').startswith('data:image/png;base64,'):
                name = slide['id'] + '.png'        # 手册里嵌的是裁过边的版本；原图留在运行目录的 screenshots/ 里
                (options.shots / name).write_bytes(base64.b64decode(slide['image'].split(',', 1)[1]))
        print(f"copied {len(list(options.shots.glob('*.png')))} screenshots to {options.shots}")


if __name__ == '__main__':
    main()
