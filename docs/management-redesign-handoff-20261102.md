# Kura 管理体系改造交接 — 2026-11-02

续接 `management-redesign-handoff-20261101.md`。配置元数据约束/组件适用性、snapshot
save/rollback、UNKNOWN 持久业务结果核定、任务 key rotation/backup/restore 已有独立闭环证据，
本页不重述为新的验收。

## 本轮新增闭环

正式中心包已达到 Ubuntu 24.04 arm64 安装候选：
- staged 65 文件源 manifest，包含 protocol `.proto`。
- React typecheck/build 与 Linux staged Rust test/release build 通过。
- `kura-management-console_0.1.0~package20261102-11_arm64.deb` 在 OrbStack
  `yofc-management-network-20261101` 实装，显式 migrate/check/activate。
- service 以 yofc-console UID990/GID983、CapEff0 运行。
- HTTPS health/login/session/gateway/list/detail 通过；专用 agent online；
  `agent.status` SUCCEEDED。临时 agent 停止且 state 保留。

权威证据见 `docs/evidence/center-package-20261102.json` 与
`docs/phase2-center-package-validation-20261102.md`。

## 当前状态

- `kura-management-console` 本地分支 `heyoulin-kura` 有本轮 packaging/文档改动，未提交或
  仅保留本地保存点；不推送。
- 开发中心、原 host agent、Kura 运行集和旧夹具 state 未清理。
- 192.168.2.125 未连接；Parallels VM 未新增；OrbStack 旧 fixture 仍保留。

## 下一批工作

1. 将最终发行包脚本、失败修正、文档与证据做本地保存点提交；不推送。
2. 复用同一 VM 做升级 guard：显式注入受控 QUEUED/RUNNING/UNKNOWN 前置，验证 preinst/prerm
   拒绝维护，再按真实路径终结后升级。
3. 在已安装包 payload 上跑 Playwright 中英文浏览器登录/网关/配置读取集合；不改旧 fixture。
4. 验证普通 remove/重装和 purge 的保留/归档契约；PostgreSQL 与 task key 只做可恢复性检查。
5. 之后进入 console/agent 独立归属、正式发行矩阵和剩余阶段；缺失环境继续 deferred。

## 2026-11-02 补充：升级 guard 已复验

正式包 preinst/prerm 已用受控 QUEUED/RUNNING/UNKNOWN task 拒绝维护；真实 `dpkg -i`
同版本包也在 old prerm 与 new preinst 两处拒绝且版本未变。task settle 后同版本重装通过，
postinst 保持不启动，显式 activate 后恢复 active/enabled。证据见
`docs/evidence/center-package-upgrade-guard-20261102.json` 和
`target/package/upgrade-guard-20261102-005558/`。

下一步优先级不变：包 payload Playwright 中英文集合、remove/purge 契约、恢复矩阵和发行矩阵。

## 2026-11-02 补充：包 payload 浏览器

已新增 package-center profile 并通过双语 HTTPS browser smoke：登录错误文案、英文登录、
gateway Online、operator status、Tasks Completed、reload/中文切换、logout、无 pageerror。
测试端口 28443 只为绕开宿主开发中心 18443；测试后服务已恢复 18443。临时账号禁用、agent
停止但 state 保留。证据见 `docs/evidence/center-package-browser-20261102.json`。

## 2026-11-02 补充：remove/purge 恢复契约

`dpkg -r`、同版本重装、`dpkg -P` 和 purge 后恢复安装均已实测。remove 后包 payload 全部
移除但 operator 配置、TLS、task key、key backup 和外部 PostgreSQL 保留；purge 后同样保留，
不删除数据库或 key。每次基线比较覆盖包文件、保留文件 SHA256、数据库行数和 task digest。
packaged `task-key-backup/restore` 也通过，错误 passphrase 拒绝且不创建目标文件。

purge 后重装可显式 migrate/check/activate 并恢复 health ready。证据见
`docs/evidence/center-package-lifecycle-20261102.json` 和
`target/package/lifecycle-20261102-005558/`。下一批继续 console/agent 独立归属流程、
发行矩阵和剩余 deferred 边界；不把本 fixture 结果推广到物理网关或 amd64/26.04。

## 2026-11-02 补充：formal agent 包与归属非抢占

重新生成当前 protocol crate 后，formal agent 包推进到
`kura-management-agent_0.1.0~package20261102-6_arm64.deb`。Linux staged source 66 项 Rust
测试全部通过，release build 通过。安装后不自动启动；对 console package candidate 12 显式
导入 identity，`AGENT_BINDING_GENERATION=1` 持久化，activate 后 online。

再次以另一 endpoint、相同 gateway 和相同证书导入被拒绝，返回 explicit transfer/recovery
required；agent env、CA、cert 和 key SHA256 全部不变。随后原中心连接恢复 online。证据见
agent `docs/evidence/binding-generation-package-runtime-20261102.json`。这证明普通另一中心
登记不会抢占归属，但签名转移、审批和代次递增仍未实施。

## 2026-11-02 补充：gateway-local rebind 基础

agent 新增显式 `transfer-identity`：必须精确匹配旧 `bindingGeneration`、提供 operator、
服务停止，且 writer 授权、未决 network transaction、RUNNING/UNKNOWN 副作用执行任一存在都
拒绝。通过后验证新 mTLS chain/key，备份旧 identity 和 receipt，再把
`AGENT_BINDING_GENERATION` 从 1 递增到 2。第二中心普通 import 仍然拒绝。

formal agent candidate 已升级到 `0.1.0~package20261102-7`；Linux staged 66 项 Rust 测试通过，
release build 通过，本机 rebind 实测成功并恢复在线。证据见 agent
`docs/evidence/local-rebind-20261102.json`。由于 fixture 使用同一逻辑中心的两个本机名，
这只证明本机 rebind 机制，不宣称独立双中心运营；签名授权/审批仍未实现。
