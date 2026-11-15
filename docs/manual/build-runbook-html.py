#!/usr/bin/env python3
"""Convert the provisioning runbook (Markdown + screenshots) into a single
self-contained HTML file that can be opened in a browser and saved as PDF.
Images are embedded as base64 so the file is portable."""
import base64, html, os, re

HERE = os.path.dirname(os.path.abspath(__file__))
SRC = os.path.join(HERE, "provisioning-runbook-zh-20261111.md")
DEST = os.path.join(HERE, "provisioning-runbook-zh-20261111.html")
IMG_DIR = os.path.join(HERE, "screenshots-runbook-20261111")

text = open(SRC, encoding="utf-8").read()

def md_inline(s):
    s = html.escape(s, quote=False)
    s = re.sub(r"`([^`]+)`", r"<code>\1</code>", s)
    s = re.sub(r"\*\*([^*]+)\*\*", r"<strong>\1</strong>", s)
    return s

lines = text.splitlines()
out = []
in_code = False
table_buf = []

def flush_table():
    global table_buf
    if not table_buf:
        return
    rows = table_buf
    out.append("<table>")
    for i, row in enumerate(rows):
        cells = [c.strip() for c in row.strip().strip("|").split("|")]
        if i == 0:
            out.append("<thead><tr>" + "".join(f"<th>{md_inline(c)}</th>" for c in cells) + "</tr></thead><tbody>")
        elif re.match(r"^[\s:|-]+$", row):
            continue
        else:
            out.append("<tr>" + "".join(f"<td>{md_inline(c)}</td>" for c in cells) + "</tr>")
    out.append("</tbody></table>")
    table_buf = []

def img_block(path, alt):
    full = os.path.join(IMG_DIR, os.path.basename(path))
    b64 = base64.b64encode(open(full, "rb").read()).decode()
    return (f'<figure class="shot"><img src="data:image/png;base64,{b64}" alt="{html.escape(alt)}">'
            f'<figcaption>{html.escape(alt)}</figcaption></figure>')

for raw in lines:
    line = raw.rstrip()
    if line.startswith("```"):
        flush_table()
        if in_code:
            out.append("</code></pre>")
            in_code = False
        else:
            out.append('<pre class="console"><code>')
            in_code = True
        continue
    if in_code:
        out.append(html.escape(raw))
        continue
    m = re.match(r"^!\[([^\]]*)\]\(([^)]+)\)$", line)
    if m:
        flush_table()
        out.append(img_block(m.group(2), m.group(1)))
        continue
    if line.startswith("|"):
        table_buf.append(line)
        continue
    flush_table()
    m = re.match(r"^(#{1,4})\s+(.*)$", line)
    if m:
        level = len(m.group(1))
        out.append(f"<h{level}>{md_inline(m.group(2))}</h{level}>")
        continue
    if line.strip() == "---":
        out.append("<hr>")
        continue
    if line.strip():
        out.append(f"<p>{md_inline(line)}</p>")
flush_table()
if in_code:
    out.append("</code></pre>")

body = "\n".join(out)

html_doc = """<!doctype html>
<html lang="zh">
<head>
<meta charset="utf-8">
<title>YOFC 管理中心完整操作手册（方案 A · 从零到在线）</title>
<style>
  :root { color-scheme: light; }
  * { box-sizing: border-box; }
  body {
    font-family: "PingFang SC", "Hiragino Sans GB", "Microsoft YaHei", -apple-system, sans-serif;
    font-size: 14px; line-height: 1.7; color: #1a2e22;
    background: #f4f6f4; margin: 0; padding: 0;
  }
  .page { max-width: 920px; margin: 0 auto; background: #fff; padding: 48px 64px 64px; }
  h1 { font-size: 26px; color: #12331f; border-bottom: 3px solid #1a6b45; padding-bottom: 12px; margin: 0 0 16px; }
  h2 { font-size: 20px; color: #12331f; border-left: 5px solid #1a6b45; padding-left: 12px; margin-top: 40px; page-break-after: avoid; }
  h3 { font-size: 16px; color: #1a6b45; margin-top: 28px; page-break-after: avoid; }
  p, li { margin: 8px 0; }
  code {
    font-family: "SF Mono", Menlo, Consolas, monospace; font-size: 12px;
    background: #eef3ef; border: 1px solid #d7e2d9; border-radius: 3px; padding: 1px 5px;
    word-break: break-all;
  }
  pre.console { background: #14231a; color: #d5e5da; padding: 14px 16px; border-radius: 6px; overflow-x: auto; }
  pre.console code { background: none; border: none; color: inherit; padding: 0; font-size: 12px; }
  table { border-collapse: collapse; width: 100%; margin: 12px 0; font-size: 13px; page-break-inside: avoid; }
  th, td { border: 1px solid #cfd9d2; padding: 7px 10px; text-align: left; vertical-align: top; }
  th { background: #e6efe9; color: #12331f; }
  figure.shot { margin: 18px 0 26px; page-break-inside: avoid; text-align: center; }
  figure.shot img { max-width: 100%; border: 1px solid #c9d6cd; border-radius: 4px; box-shadow: 0 2px 8px rgba(20,60,40,.12); }
  figcaption { font-size: 12px; color: #55705f; margin-top: 6px; }
  hr { border: none; border-top: 1px solid #d7e2d9; margin: 28px 0; }
  .cover { text-align: center; padding: 30px 0 10px; }
  .cover .badge { display: inline-block; background: #1a6b45; color: #fff; padding: 4px 18px; border-radius: 20px; font-size: 13px; letter-spacing: 2px; }
  .cover h1 { border: none; margin-top: 14px; }
  .meta { color: #55705f; font-size: 13px; }
  .toolbar {
    position: sticky; top: 0; background: #12331f; color: #fff; padding: 10px 24px;
    display: flex; gap: 16px; align-items: center; justify-content: space-between;
  }
  .toolbar button {
    background: #1a6b45; color: #fff; border: none; padding: 8px 20px; border-radius: 5px;
    font-size: 13px; cursor: pointer;
  }
  .toolbar button:hover { background: #24855a; }
  .toolbar .hint { font-size: 12px; opacity: .8; }
  @media print {
    body { background: #fff; }
    .toolbar { display: none; }
    .page { max-width: none; padding: 0; }
    figure.shot img { box-shadow: none; }
    @page { margin: 18mm 14mm; }
  }
</style>
</head>
<body>
<div class="toolbar">
  <span><strong>YOFC 管理中心完整操作手册</strong> · 方案 A · 从零到在线</span>
  <span class="hint">浏览器菜单选择「打印 → 另存为 PDF」，或直接点击右侧按钮</span>
  <button onclick="window.print()">打印 / 另存为 PDF</button>
</div>
<div class="page">
<div class="cover">
  <div class="badge">YOFC 边缘管理体系</div>
  <h1>管理中心 + 网关代理完整操作手册</h1>
  <p class="meta">方案 A（CA 私钥不进中心）· 从零部署到网关在线 · 含全流程截图<br>
  截图轮次 2026-11-11 · 中心 yofc-management-console / 网关代理 0.1.0~manual20261111</p>
</div>
<hr>
""" + body + """
</div>
</body>
</html>"""

open(DEST, "w", encoding="utf-8").write(html_doc)
print("written:", DEST, f"{os.path.getsize(DEST)/1024/1024:.1f} MB")
