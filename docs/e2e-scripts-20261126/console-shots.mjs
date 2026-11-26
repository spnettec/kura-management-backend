#!/usr/bin/env node
// 登录管理中心后截图：无头 Chrome + DevTools 协议，只依赖 Node 内置模块（Node 22+）。
// 由 07-screenshots.sh 调用。
//
//   node console-shots.mjs --origin https://localhost:18443 --out <目录> [--user …] [--password …]
//       [--path <路径> --name <文件名>]...   登录后再截指定页面，可重复多组
//       [--click-row <网关ID> <按钮文字> <文件名>]...   回到总览页，点该网关那一行里的按钮，等 2.5 秒后截图
//       [--width 1280] [--height 1000]      视口大小；总览页要一屏看下四台网关，高度给到 1000
//       [--dump]                            额外打印登录后页面的链接与文字（用来找页面路径）
//
// 登录后的首页固定截为 gateways-overview.png。
import { spawn } from 'node:child_process';
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';

const args = process.argv.slice(2);
const option = (name, fallback) => { const i = args.indexOf(name); return i >= 0 && i + 1 < args.length ? args[i + 1] : fallback; };
const origin = option('--origin', 'https://localhost:18443').replace(/\/$/, '');
const out = option('--out');
if (!out) { console.error('usage: console-shots.mjs --origin <url> --out <dir> [--path <p> --name <n>]...'); process.exit(2); }
const user = option('--user', 'admin'), password = option('--password', process.env.ADMIN_PASSWORD || '');
const chrome = option('--chrome', '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome');
const width = Number(option('--width', '1280')), height = Number(option('--height', '1000'));
const pages = [];
for (let i = 0; i + 1 < args.length; i++) if (args[i] === '--path' && args[i + 2]) pages.push({ path: args[i + 1], name: args[i + 2] });
for (let i = 0; i + 2 < args.length; i++) if (args[i] === '--click-row' && args[i + 3]) pages.push({ row: args[i + 1], button: args[i + 2], name: args[i + 3] });

const port = 9400 + Math.floor(Math.random() * 400);
const profile = mkdtempSync(join(tmpdir(), 'e2e-shots-'));
const browser = spawn(chrome, ['--headless=new', '--disable-gpu', `--remote-debugging-port=${port}`, `--user-data-dir=${profile}`,
    `--window-size=${width},${height}`, '--ignore-certificate-errors', '--no-first-run', '--no-default-browser-check', 'about:blank'], { stdio: 'ignore' });
const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));
async function waitFor(probe, ms, what) {
    const end = Date.now() + ms;
    let last;
    while (Date.now() < end) {
        try { const value = await probe(); if (value) return value; } catch (error) { last = error; }
        await sleep(250);
    }
    throw new Error(`timeout waiting for ${what}${last ? `: ${last.message}` : ''}`);
}

try {
    await waitFor(() => fetch(`http://127.0.0.1:${port}/json/version`).then((r) => (r.ok ? r.json() : null)), 20000, 'Chrome');
    const target = await fetch(`http://127.0.0.1:${port}/json/new?about:blank`, { method: 'PUT' }).then((r) => r.json());
    const socket = new WebSocket(target.webSocketDebuggerUrl);
    await new Promise((resolve, reject) => { socket.onopen = resolve; socket.onerror = reject; });
    let seq = 0;
    const pending = new Map();
    socket.onmessage = (event) => {
        const message = JSON.parse(event.data);
        const waiter = pending.get(message.id);
        if (!waiter) return;
        pending.delete(message.id);
        if (message.error) waiter.reject(new Error(message.error.message)); else waiter.resolve(message.result);
    };
    const send = (method, params = {}) => new Promise((resolve, reject) => {
        const id = ++seq;
        pending.set(id, { resolve, reject });
        socket.send(JSON.stringify({ id, method, params }));
    });
    const evaluate = async (expression) => {
        const result = await send('Runtime.evaluate', { expression, returnByValue: true, awaitPromise: true });
        if (result.exceptionDetails) throw new Error(result.exceptionDetails.exception?.description || result.exceptionDetails.text);
        return result.result.value;
    };
    await send('Page.enable');
    await send('Emulation.setDeviceMetricsOverride', { width, height, deviceScaleFactor: 1, mobile: false });
    const goto = async (url) => {
        await send('Page.navigate', { url });
        await waitFor(() => evaluate('document.readyState !== "loading" && location.href.startsWith("http")'), 20000, `loading ${url}`);
        await sleep(1500);
    };
    const shot = async (name) => {
        await sleep(2500);        // 等接口返回和图表/列表渲染完
        const { data } = await send('Page.captureScreenshot', { format: 'png' });
        mkdirSync(out, { recursive: true });
        writeFileSync(join(out, `${name}.png`), Buffer.from(data, 'base64'));
        console.log(`    screenshot ${join(out, `${name}.png`)}`);
    };

    await goto(`${origin}/`);
    // 登录表单：找到密码框即填入用户名、密码并点提交按钮。React 受控输入要经由原型上的 setter 赋值再派发 input 事件。
    await waitFor(() => evaluate(`(() => {
        const password = document.querySelector('input[type=password]');
        if (!password) return false;
        const form = password.form || password.closest('form') || document;
        const inputs = [...form.querySelectorAll('input')].filter((input) => input !== password && ['text', 'email', ''].includes(input.type));
        const username = inputs.find((input) => /user|name|account|mail/i.test(input.autocomplete + input.name + input.id + input.placeholder)) || inputs[0];
        if (!username) return false;
        const setter = Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, 'value').set;
        for (const [input, value] of [[username, ${JSON.stringify(user)}], [password, ${JSON.stringify(password)}]]) {
            input.focus();
            setter.call(input, value);
            input.dispatchEvent(new Event('input', { bubbles: true }));
            input.dispatchEvent(new Event('change', { bubbles: true }));
        }
        const submit = form.querySelector('button[type=submit], button:not([type])') || form.querySelector('button');
        submit.click();
        return true;
    })()`), 30000, 'login form');
    await waitFor(() => evaluate('!document.querySelector("input[type=password]")'), 30000, 'login to complete');
    console.log(`    logged in at ${origin} as ${user}`);
    if (args.includes('--dump')) {
        console.log(await evaluate(`location.href + "\\n" + [...document.querySelectorAll("a[href]")].map((a) => a.getAttribute("href") + "  |  " + a.textContent.trim().slice(0, 40)).join("\\n") + "\\n-----\\n" + document.body.innerText.slice(0, 8000)`));
    }
    await shot('gateways-overview');
    for (const page of pages) {
        if (page.row) {
            await goto(`${origin}/`);
            await waitFor(() => evaluate(`(() => {
                const row = [...document.querySelectorAll('tr')].find((tr) => tr.textContent.includes(${JSON.stringify(page.row)}));
                const button = row && [...row.querySelectorAll('button')].find((b) => b.textContent.trim() === ${JSON.stringify(page.button)});
                if (!button) return false;
                button.click();
                return true;
            })()`), 30000, `row ${page.row}`);
        } else {
            await goto(origin + page.path);
        }
        await shot(page.name);
    }
    socket.close();
} finally {
    browser.kill('SIGKILL');
    rmSync(profile, { recursive: true, force: true });
}
