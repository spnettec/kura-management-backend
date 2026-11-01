# 正式中心包构建与 Ubuntu 24.04 安装验证

日期：2026-11-02。本增量接续配置 UNKNOWN 持久业务结果核定和任务 key 备份恢复，
不重复已完成迁移动作。整体发行仍未完成；本页只记录正式中心包已验证范围。

## 制品与构建

- 版本化源码 staging：`target/package/console-source-20261102-012012`，65 个源文件，
  manifest SHA256 `25adbad43d8639c4b395104b22fdfd8c331861206b192a3db1f05ecfa6e66b40`。
- 修复 staging 漏掉 `crates/management-protocol/proto/management/v1/control.proto` 后，
  staged source 才能在 Linux 自包含编译。
- staged React 源执行 `npm ci`、`tsc --noEmit`、`vite build`，产出 3 个 web 文件。
- OrbStack Ubuntu 24.04 arm64 staged source 执行 `cargo test --locked`：
  19 executed passed、5 ignored、0 failed；随后 `cargo build --release --locked`。
- Linux ELF SHA256 `6a69ce0443a20b31be145acd206639ad969e739f9ff10af2d7a279f7e5e821b9`，
  最高 GLIBC symbol `2.34`。
- 最终 deb：`0.1.0~package20261102-11`，SHA256
  `c9f20f293ba3038966fa1d5a9856e6af464f1800d0ab4bb5786991a84645c12b`。
  接收文件：`target/package/console-deb-20261102-012012/package.json`。

## 安装与生命周期修正

复用现有 OrbStack VM `yofc-management-network-20261101`，新建 PostgreSQL 16 fixture
`console_pkg_20261102_005558`，未触碰旧夹具 state。安装前显式写入 root0600 env；
首装不自动启动、不 enable。

实际失败暴露并修正了 packaging 缺陷：marker 写入收到 `Path`、fresh install 被未安装
payload/key 卡死、psql guard 忽略 DATABASE_URL、empty schema 误判失败、key wrapper 命令名/
arity 不匹配、TLS/config 权限不可读、activate setup lock 与 ExecStartPre 互相等待、
prerm 在 auto-restart 下先检查 inactive。最终 maintainer scripts 编译通过，preinst/prerm
在缺表/已迁移两种状态均能通过本轮实际安装或升级路径验证。

最终权限为 env root0600、config/tls root:yofc-console0750、TLS key root:yofc-console0640、
task key 与 state/log yofc-console0700/0600。PostgreSQL 数据和 key 不由 purge 删除。

## 运行与业务烟测

`kura-console-setup migrate` 返回 `migrated_tasks=0`，`check` 返回 `ok/assets=1`，
`activate` 后 service active/enabled，PID 37299 以 UID/GID 990/983 和 CapEff 0 运行。
OrbStack 全局 LXC drop-in 使 effective NoNewPrivileges/ProtectSystem/ProtectHome 为关闭；
unit 源定义未被弱化，完整 hardening 继续只可在非 OrbStack 环境验收。

HTTPS `/health/ready` 返回 ready；临时 admin 通过 HTTPS login 200 后读取 session、gateway
list/detail。专用 fixture gateway `console-pkg-agent-20261102-005558` online；
`agent.status` 任务
`df4e9e44-f10d-4a35-802f-d2110f5e1416` 返回 SUCCEEDED。烟测后停止该临时 agent，
state 保留；旧 phase2 agent/link 仍 active。临时登录账号已 disable，session/凭据文件清理，
审计行保留。

## 证据与边界

机器可核对证据：`docs/evidence/center-package-20261102.json`。制品、构建日志、manifest、
gateway JSON、agent.status receipt 和最终 service state 保存在
`target/package/console-deb-20261102-012012/` 与
`target/package/console-package-final-state-20261102-012012.json`。

本轮只覆盖 Ubuntu 24.04 arm64。amd64、26.04、VM 重启、真实物理网关/NM/cloud-init 交接、
生产 systemd hardening 和带真实 QUEUED/RUNNING/UNKNOWN 任务的升级 guard 仍 deferred/pending。
浏览器 UI 在包 payload 的静态资源可达和 API smoke 通过；完整浏览器登录本轮未重复，
不记为通过。

## 升级 guard 复验

在同一 fixture 中用三个受控 task 分别进入 QUEUED/RUNNING/UNKNOWN，再执行维护入口。
`preinst install` 与 `prerm upgrade` 均拒绝并返回 1；`prerm` 先停止服务后 guard 仍拒绝，
符合“先停止运行进程，再禁止未终结业务进入维护”的顺序。随后受控 task 全部落为
SUCCEEDED，两个脚本均接受。

再执行真实 `dpkg -i` 同版本包：未终结状态下 old prerm 与 new preinst 均拒绝，包版本保持
`0.1.0~package20261102-11` 未被替换；task settle 后重新 `dpkg -i` 成功，postinst 保持
inactive，随后显式 `activate` 恢复 active/enabled。最终 PID 38389，三个受控 task 都是
SUCCEEDED，数据库未终结任务为 0。

原始输出在 `target/package/upgrade-guard-20261102-005558/`；机器可核对记录在
`docs/evidence/center-package-upgrade-guard-20261102.json`。该验证不清理 state，只变更
本轮新建 fixture 的受控 task 状态，并把 guard 建/重置/终结写入 fixture audit。

## 包 payload 浏览器烟测

新增 `package-center` Playwright profile 和 `package-center-live.spec.ts`。因宿主 18443
被独立开发中心占用，测试期把包 fixture 临时放到 OrbStack `localhost:28443`，使用包内
React payload、包证书 SPKI pin 和临时 operator。测试通过中文登录页、错误文案切英文、
英文登录、gateway Online、operator `agent.status`、Tasks Completed、reload、切回中文、
logout 和无 pageerror。1 expected passed，0 failed/skipped/unexpected/flaky。

测试后停掉临时 agent，保留其 SQLite state；禁用临时 browser 账号并清理密码文件；包服务
恢复为 `127.0.0.1:18443`、origin `https://localhost:18443`，PostgreSQL 未终结任务为 0。
guard fixture 中复制合法 encrypted result 到不同 task ID 后，列表接口正确 fail closed；
清空这些复制 result 后恢复。这是 context binding 生效的证据，不是生产缺陷。

证据：`docs/evidence/center-package-browser-20261102.json`，
`target/center-package-browser-20261102.json` 和三张截图。此为包 payload focused live
smoke；既有开发中心专用完整浏览器集合不宣称被替代。

## remove / reinstall / purge 恢复契约

在同一 fixture 上先用包内 `task-key-backup/restore` 建立恢复 proof：task key 原件、
restored key SHA256 均为 `70c37409a084bd63039d410e41a1effe298b48da60ecace1decaf7b313435b92`，
Argon2id/AES-GCM backup SHA256 为
`76e89cc684d8501c5dfe3e613699b2e17dee7c88b7b44e176a45345a587033f7`。错误 passphrase restore
返回 1 且没有创建目标文件。backup/restored/passphrase 文件均为 yofc-console 0600。

`dpkg -r` 成功：包状态 `rc`，service not-found，10 个包 payload 全部 absent；operator 配置、
TLS、task key、key backup/restored、agent fixture SQLite 和外部 PostgreSQL 数据保留。逐项
SHA256 与 baseline 一致，数据库 1 gateway / 5 tasks / 32 audit / 180 events 及 task digest
不变。同版本重装后 postinst 保持 inactive/disabled；显式 migrate 返回 `migrated_tasks=0`、
check 通过、activate 后 health ready，包文件与保留文件均匹配 baseline。

`dpkg -P` 成功：包 payload 10/10 absent，包状态 not-installed；postrm 明确保留 PostgreSQL
数据和 task key。operator 的 `/etc` 配置、TLS、task key、backup、agent SQLite 与数据库逐项
SHA256/digest 不变。随后恢复安装同一 deb，postinst 仍不启动；显式 migrate/check/activate 后
service active/enabled、health ready，包文件与保留数据再次全部匹配。最终未终结任务为 0。

证据：`docs/evidence/center-package-lifecycle-20261102.json` 和
`target/package/lifecycle-20261102-005558/`。本轮没有删除外部 PostgreSQL 数据或 task key；
`/etc` operator 配置不是包 conffile，purge 后按合同保留。
