#!/usr/bin/env node
// 把 e2e-deployment-manual-*.html 里内嵌的幻灯片规格（<script id="e2e-deck" type="application/json">）
// 转成 16:9 的 PowerPoint。手册与演示共用同一份内容，改手册的生成脚本后两边一起更新。
//
//   NODE_PATH=<装有 pptxgenjs 的 node_modules> node html-to-pptx.mjs <manual.html> <out.pptx>
//
// 支持的版式：cover / stats / topology / steps / image / wide / table / code / closing
// wide：宽幅截图（虚拟机文本控制台）占满整行，要点在下面分两栏。
import { createRequire } from 'node:module';
import { readFileSync, writeFileSync } from 'node:fs';

const require = createRequire(import.meta.url);
const pptxgen = require('pptxgenjs');

const [src, out] = process.argv.slice(2);
if (!src || !out) {
    console.error('usage: node html-to-pptx.mjs <manual.html> <out.pptx>');
    process.exit(2);
}

// ---- 取出内嵌的幻灯片规格 ----------------------------------------------------
const html = readFileSync(src, 'utf8');
const match = html.match(/<script id="e2e-deck" type="application\/json">([\s\S]*?)<\/script>/);
if (!match) {
    console.error('no embedded deck spec (<script id="e2e-deck">) found');
    process.exit(1);
}
const deck = JSON.parse(match[1]);

// ---- 设计常量：取自管理中心界面（深墨绿 / 米白 / 祖母绿），琥珀色只做点睛 ----------------
const W = 13.33, H = 7.5, M = 0.6;
const DARK = '0F2419', BG = 'F4F4F0', PRIMARY = '17563B', ACCENT = 'D97706';
const TEXT = '1B2620', MUTED = '64748B', LIGHT = 'F4F4F0', LINE = 'CBD5C0';
const SANS = 'PingFang SC', MONO = 'Menlo';
const TITLE = 32, BODY = 17, SMALL = 13, MIN = 12;

const pres = new pptxgen();
pres.layout = 'LAYOUT_WIDE';
pres.title = deck.title;
pres.author = deck.author || 'YOFC Kura';

const shadow = () => ({ type: 'outer', color: '0F2419', blur: 8, offset: 3, angle: 90, opacity: 0.14 });
const bullet = () => ({ code: '25B8', indent: 14 });

function heading(slide, title, dark = false) {
    slide.addText(title, { x: M, y: 0.42, w: W - 2 * M, h: 0.7, fontFace: SANS, fontSize: TITLE, bold: true,
        color: dark ? LIGHT : DARK, margin: 0, valign: 'middle' });
}

function notes(slide, text) {
    if (text) slide.addNotes(text);
}

function bullets(slide, points, box, size = BODY) {
    if (!points || !points.length) return;
    slide.addText(points.map((text, i) => ({ text, options: { bullet: bullet(), breakLine: i < points.length - 1 } })), {
        x: box.x, y: box.y, w: box.w, h: box.h, fontFace: SANS, fontSize: size, color: TEXT,
        paraSpaceAfter: 10, margin: 0, valign: 'top',
    });
}

// PNG 的原始尺寸直接从 IHDR 里读（签名 8 + 长度 4 + 类型 4 之后是宽、高各 4 字节）。
function pngSize(dataUri) {
    if (!dataUri.startsWith('data:image/png;base64,')) return [1280, 800];
    const header = Buffer.from(dataUri.slice(dataUri.indexOf(',') + 1, dataUri.indexOf(',') + 65), 'base64');
    const w = header.readUInt32BE(16), h = header.readUInt32BE(20);
    return w > 0 && h > 0 ? [w, h] : [1280, 800];
}

// 图片按原始比例放进目标框并居中，绝不拉伸；描一圈细线把浅色截图和底色分开。返回实际落位。
function picture(slide, dataUri, box) {
    const [w, h] = pngSize(dataUri);
    const scale = Math.min(box.w / w, box.h / h);
    const placed = { x: box.x + (box.w - w * scale) / 2, y: box.y + (box.h - h * scale) / 2, w: w * scale, h: h * scale };
    slide.addImage({ data: dataUri, ...placed, shadow: shadow(), altText: box.alt || 'screenshot' });
    slide.addShape(pres.shapes.RECTANGLE, { ...placed, fill: { type: 'none' }, line: { color: '9AA89C', width: 0.75 } });
    return placed;
}

const layouts = {
    cover(s, d) {
        s.background = { color: DARK };
        s.addText(deck.title, { x: M, y: 1.75, w: 7.6, h: 2.0, fontFace: SANS, fontSize: 44, bold: true, color: LIGHT, margin: 0, valign: 'top' });
        s.addText(d.subtitle, { x: M, y: 3.95, w: 7.6, h: 1.2, fontFace: SANS, fontSize: 20, color: 'CFE0D3', margin: 0, valign: 'top' });
        s.addText(d.meta, { x: M, y: 6.5, w: 7.6, h: 0.5, fontFace: SANS, fontSize: SMALL, color: '9FB8A6', margin: 0 });
        // 右侧：管理中心 + 四台网关的示意
        const cx = 10.45;
        s.addShape(pres.shapes.ROUNDED_RECTANGLE, { x: cx - 1.35, y: 1.9, w: 2.7, h: 1.0, fill: { color: PRIMARY }, line: { color: '2E7D5B', width: 1 }, rectRadius: 0.08 });
        s.addText('管理中心', { x: cx - 1.35, y: 1.98, w: 2.7, h: 0.5, fontFace: SANS, fontSize: 16, bold: true, color: LIGHT, align: 'center', margin: 0 });
        s.addText('Docker · 18443 / 18444', { x: cx - 1.35, y: 2.42, w: 2.7, h: 0.4, fontFace: MONO, fontSize: MIN, color: 'CFE0D3', align: 'center', margin: 0 });
        d.gateways.forEach((g, i) => {
            const gx = cx - 2.5 + (i % 2) * 2.65, gy = 4.3 + Math.floor(i / 2) * 1.15;
            s.addShape(pres.shapes.LINE, { x: cx, y: 2.9, w: gx + 1.17 - cx, h: gy - 2.9, line: { color: '4F8C6E', width: 1.25 } });
            s.addShape(pres.shapes.ROUNDED_RECTANGLE, { x: gx, y: gy, w: 2.35, h: 0.85, fill: { color: '173A2A' }, line: { color: '4F8C6E', width: 1 }, rectRadius: 0.08 });
            s.addText([{ text: g.id, options: { bold: true, breakLine: true } }, { text: g.kind, options: { fontSize: MIN, color: 'CFE0D3' } }],
                { x: gx, y: gy + 0.06, w: 2.35, h: 0.72, fontFace: SANS, fontSize: 14, color: LIGHT, align: 'center', valign: 'middle', margin: 0 });
        });
        notes(s, d.note);
    },

    stats(s, d) {
        s.background = { color: BG };
        heading(s, d.title);
        const n = d.stats.length, gap = 0.4, cw = (W - 2 * M - gap * (n - 1)) / n;
        d.stats.forEach((item, i) => {
            const x = M + i * (cw + gap);
            // 值按字数缩小：195/195 这样的值用 60 磅会在栏宽内折成两行
            const size = item.value.length > 3 ? Math.min(60, Math.floor(290 / item.value.length)) : 60;
            s.addText(item.value, { x, y: 1.35, w: cw, h: 1.3, fontFace: SANS, fontSize: size, bold: true, color: i === 0 ? PRIMARY : DARK, align: 'left', fit: 'shrink', margin: 0, valign: 'middle' });
            s.addText(item.label, { x, y: 2.7, w: cw, h: 0.9, fontFace: SANS, fontSize: 15, color: MUTED, margin: 0, valign: 'top' });
        });
        s.addShape(pres.shapes.LINE, { x: M, y: 3.72, w: W - 2 * M, h: 0, line: { color: LINE, width: 1 } });
        bullets(s, d.points, { x: M, y: 4.0, w: W - 2 * M, h: 2.3 }, 19);
        if (d.artifacts) s.addText(d.artifacts, { x: M, y: 6.45, w: W - 2 * M, h: 0.45, fontFace: MONO, fontSize: MIN, color: MUTED, margin: 0, valign: 'middle' });
        notes(s, d.note);
    },

    topology(s, d) {
        s.background = { color: BG };
        heading(s, d.title);
        const center = { x: 4.07, y: 1.3, w: 5.2, h: 1.85 };
        s.addShape(pres.shapes.ROUNDED_RECTANGLE, { ...center, fill: { color: PRIMARY }, line: { color: PRIMARY }, rectRadius: 0.08, shadow: shadow() });
        s.addText(d.center, { ...center, x: center.x + 0.1, w: center.w - 0.2, y: center.y + 0.08, h: center.h - 0.16, fontFace: SANS, fontSize: 15, bold: true, color: LIGHT, align: 'center', valign: 'middle', margin: 0 });
        const gw = (i) => ({ x: M + i * ((W - 2 * M - 0.3 * 3) / 4 + 0.3), y: 4.3, w: (W - 2 * M - 0.3 * 3) / 4, h: 1.8 });
        d.nodes.forEach((node, i) => {
            const b = gw(i);
            // 连线从说明文字下面画到网关框顶，不从中心框底出发，避免穿过文字
            const sx = center.x + center.w / 2 + (i - 1.5) * 0.35;
            s.addShape(pres.shapes.LINE, { x: Math.min(sx, b.x + b.w / 2), y: 3.98, w: Math.abs(b.x + b.w / 2 - sx), h: b.y - 3.98,
                flipH: b.x + b.w / 2 < sx, line: { color: '4F8C6E', width: 1.5 } });
            s.addShape(pres.shapes.ROUNDED_RECTANGLE, { ...b, fill: { color: 'FFFFFF' }, line: { color: LINE, width: 1 }, rectRadius: 0.08 });
            s.addText([{ text: node.id, options: { bold: true, fontSize: 17, breakLine: true } },
                       { text: node.kind, options: { fontSize: SMALL, color: MUTED, breakLine: true } },
                       { text: node.address, options: { fontFace: MONO, fontSize: MIN, color: PRIMARY, breakLine: true } },
                       { text: node.extra || '', options: { fontSize: MIN, color: MUTED } }],
                { x: b.x + 0.12, y: b.y + 0.1, w: b.w - 0.24, h: b.h - 0.2, fontFace: SANS, fontSize: SMALL, color: TEXT, align: 'center', valign: 'middle', margin: 0 });
        });
        // 说明文字在连线之后画并带底色，连线不会穿过文字
        if (d.caption) s.addText(d.caption, { x: M, y: 3.3, w: W - 2 * M, h: 0.62, fontFace: SANS, fontSize: 16, color: TEXT, align: 'center', valign: 'middle', margin: 0, fill: { color: BG } });
        if (d.footnote) s.addText(d.footnote, { x: M, y: 6.35, w: W - 2 * M, h: 0.5, fontFace: SANS, fontSize: MIN, color: MUTED, margin: 0 });
        notes(s, d.note);
    },

    steps(s, d) {
        s.background = { color: BG };
        heading(s, d.title);
        const n = d.steps.length, top = 1.35, rowH = Math.min(1.12, (6.45 - top) / n);
        d.steps.forEach((st, i) => {
            const y = top + i * rowH;
            s.addText(String(i + 1), { x: M, y, w: 0.95, h: rowH - 0.12, fontFace: SANS, fontSize: 40, bold: true, color: i === d.highlight ? ACCENT : PRIMARY, margin: 0, valign: 'middle' });
            s.addText([{ text: st.title, options: { bold: true, breakLine: true } }, { text: st.detail, options: { fontSize: SMALL, color: MUTED } }],
                { x: M + 1.05, y, w: W - 2 * M - 1.05, h: rowH - 0.12, fontFace: SANS, fontSize: BODY, color: TEXT, margin: 0, valign: 'middle' });
            if (i < n - 1) s.addShape(pres.shapes.LINE, { x: M + 1.05, y: y + rowH - 0.06, w: W - 2 * M - 1.05, h: 0, line: { color: LINE, width: 1 } });
        });
        notes(s, d.note);
    },

    image(s, d) {
        s.background = { color: BG };
        heading(s, d.title);
        const left = 4.55;
        bullets(s, d.points, { x: M, y: 1.4, w: left - M - 0.3, h: 5.3 });
        picture(s, d.image, { x: left + 0.2, y: 1.3, w: W - M - left - 0.2, h: 5.05, alt: d.caption });
        if (d.caption) s.addText(d.caption, { x: left + 0.2, y: 6.45, w: W - M - left - 0.2, h: 0.5, fontFace: SANS, fontSize: MIN, color: MUTED, align: 'center', margin: 0 });
        notes(s, d.note);
    },

    // 宽幅截图：图占满整行，高度按原始比例算；要点紧跟在图下面分两栏。
    wide(s, d) {
        s.background = { color: BG };
        heading(s, d.title);
        const [w, h] = pngSize(d.image);
        const iw = W - 2 * M, ih = Math.min(3.9, iw * h / w);
        picture(s, d.image, { x: M, y: 1.3, w: iw, h: ih, alt: d.caption });
        const top = 1.3 + ih + 0.22, half = Math.ceil(d.points.length / 2), column = (iw - 0.4) / 2;
        bullets(s, d.points.slice(0, half), { x: M, y: top, w: column, h: 6.5 - top }, d.pointSize || SMALL);
        bullets(s, d.points.slice(half), { x: M + column + 0.4, y: top, w: column, h: 6.5 - top }, d.pointSize || SMALL);
        if (d.caption) s.addText(d.caption, { x: M, y: 6.55, w: iw, h: 0.55, fontFace: SANS, fontSize: MIN, color: MUTED, align: 'center', margin: 0 });
        notes(s, d.note);
    },

    table(s, d) {
        s.background = { color: BG };
        heading(s, d.title);
        const size = d.fontSize || SMALL;
        const head = d.columns.map((c) => ({ text: c, options: { bold: true, color: LIGHT, fill: { color: PRIMARY }, fontSize: size } }));
        const body = d.rows.map((r, i) => r.map((c, j) => ({ text: c, options: {
            fontSize: size, color: j === 0 ? DARK : TEXT, bold: j === 0,
            fill: { color: i % 2 ? 'FFFFFF' : 'ECEEE7' } } })));
        s.addTable([head, ...body], { x: M, y: 1.4, w: W - 2 * M, colW: d.widths, rowH: d.rowHeight || 0.42,
            fontFace: SANS, border: { type: 'solid', pt: 0.5, color: LINE }, valign: 'middle', margin: 0.07 });
        if (d.points) bullets(s, d.points, { x: M, y: 1.4 + (d.rows.length + 1) * (d.rowHeight || 0.42) + 0.3, w: W - 2 * M, h: 1.6 }, SMALL);
        notes(s, d.note);
    },

    code(s, d) {
        s.background = { color: BG };
        heading(s, d.title);
        bullets(s, d.points, { x: M, y: 1.4, w: 4.1, h: 5.3 }, d.pointSize || BODY);
        const box = { x: 4.95, y: 1.35, w: W - M - 4.95, h: 5.35 };
        s.addShape(pres.shapes.ROUNDED_RECTANGLE, { ...box, fill: { color: DARK }, line: { color: DARK }, rectRadius: 0.06 });
        s.addText(d.code, { x: box.x + 0.25, y: box.y + 0.2, w: box.w - 0.5, h: box.h - 0.4, fontFace: MONO, fontSize: d.fontSize || SMALL,
            color: 'E6EDE7', margin: 0, valign: 'top', lineSpacingMultiple: 1.15 });
        notes(s, d.note);
    },

    closing(s, d) {
        s.background = { color: DARK };
        s.addText(d.title, { x: M, y: 2.2, w: W - 2 * M, h: 1.3, fontFace: SANS, fontSize: 44, bold: true, color: LIGHT, margin: 0, valign: 'top' });
        s.addText(d.points.map((t, i) => ({ text: t, options: { bullet: bullet(), breakLine: i < d.points.length - 1 } })),
            { x: M, y: 3.75, w: W - 2 * M, h: 2.2, fontFace: SANS, fontSize: 20, color: 'CFE0D3', paraSpaceAfter: 10, margin: 0 });
        notes(s, d.note);
    },
};

deck.slides.forEach((spec, i) => {
    const slide = pres.addSlide();
    const build = layouts[spec.layout];
    if (!build) throw new Error(`slide ${i + 1}: unknown layout "${spec.layout}"`);
    build(slide, spec);
    if (i > 0 && i < deck.slides.length - 1) {
        slide.addText(`${i + 1} / ${deck.slides.length}`, { x: W - M - 1.2, y: H - 0.5, w: 1.2, h: 0.35, fontFace: SANS, fontSize: MIN, color: MUTED, align: 'right', margin: 0 });
    }
});

await pres.writeFile({ fileName: out });
writeFileSync(out.replace(/\.pptx$/, '') + '.slides.json', JSON.stringify(deck.slides.map((s) => ({ layout: s.layout, title: s.title || '' })), null, 1));
console.log(`wrote ${out} (${deck.slides.length} slides)`);
