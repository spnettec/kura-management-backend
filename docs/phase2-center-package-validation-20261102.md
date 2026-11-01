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
