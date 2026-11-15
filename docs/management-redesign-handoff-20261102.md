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

## 2026-11-02 补充：binding generation 握手与任务隔离

protocol Hello 新增 `binding_generation=6`；console connect 要求正数且精确匹配
`mgmt_gateways.binding_generation`，任务 admission 保存当时 gateway generation。generation
变化时，旧 QUEUED/RUNNING 标记 `EXPIRED/BINDING_GENERATION_CHANGED`；dispatch 和 UNKNOWN
reconciliation 只匹配当前 generation。agent 从 identity env 发送 generation，缺失或零值
拒绝启动。UNKNOWN 历史保留原 generation，不迁移。

console package candidate 13 和 agent candidate 8 已交叉实装。先观测 agent generation 2 对
center generation 1 被 FORBIDDEN 拒绝；将 fixture center 行显式对齐 2 后 agent 立即连接。
host/macOS 与 Linux staged Rust 全部通过。证据见 agent
`docs/evidence/binding-handshake-20261102.json`。

## 2026-11-02 补充：signed transfer token 基础

console 新增 `generate-transfer-key` / `issue-gateway-transfer`，使用 Ed25519 签发一次性
`YOFC-AGENT-TRANSFER-v1` token，有效期不超过 900 秒，并绑定新旧 binding generation、
gateway、endpoint 和新 CA/client hash。agent identity 可显式导入 registered
transfer-signer；`transfer-identity --authorization-token` 只接受精确 claim 和有效签名。
agent packaging 23/23、Linux staged Rust 66/66 通过；formal agent candidate 9 已构建。
证据见 agent `docs/evidence/signed-transfer-20261102.json`。外部审批流程和独立双中心运营
仍未实施；console 包也尚未重建到该 CLI 版本。

## 2026-11-02 补充：signed transfer E2E

formal console candidate 14 / agent candidate 12 已交叉升级。console 14 可生成/签发 Ed25519
one-time token；agent 12 可显式登记 original-center signer 并验证 unpadded token payload。
token 绑定 generation 2→3、gateway、endpoint、新 CA/client hash。签名 transfer 成功后，center
仍 generation 2 时 agent reconnect 被 FORBIDDEN；center 显式对齐 generation 3 后恢复在线。
旧 generation 0 SUCCEEDED history 保持不变。

host/macOS console 11 项、protocol 8 项；Linux staged console 11 项、protocol 8 项；
agent Linux Rust 66 项、packaging Python 25 项全部通过。证据见 agent
`docs/evidence/signed-transfer-e2e-20261102.json`。这仍是同一逻辑 center 的两个本机 endpoint，
不宣称独立双中心运营；外部审批和 token 撤销仍未实现。

## 2026-11-02 补充：独立逻辑中心 signed transfer E2E

同一 OrbStack VM 内建立 center1 / center2：独立 PostgreSQL database、独立 CA/TLS、独立
console process 和 18443/18444、28443/28444 端口。center1 使用 Ed25519 signer 签发
generation 3→4 token，绑定新 endpoint/gateway/CA/client hash；formal agent 包 13 成功转入
center2。center2 generation 4 online，agent 重启仍回连 center2；center1 generation 3 history
保留并心跳超时 offline。

runtime evidence 位于 agent
`target/package/independent-center-e2e-20261102/runtime/`，机器可核对证据为 agent
`docs/evidence/independent-center-e2e-20261102.json`。该结果覆盖独立逻辑 state/trust/process；
独立物理主机、网络分区、外部审批 UI 和 token 撤销仍 deferred。

## 2026-11-02 补充：审批与签名方权威记录

新增迁移 `0013_transfer_authority.sql` 和 CLI 流程：签名方注册/撤销、双人 transfer approval、精确 claim 匹配后一次性签发，以及 open approval 拒绝/撤销。签发事务消费 approval；approval 被消费或 signer 被撤销后不能再次签发。macOS workspace 13+8 项 Rust 单测通过，其中新增 Ed25519 SPKI 匹配测试；临时 OrbStack PostgreSQL 独立库完成 0013 全链迁移，并用一次性契约库实测签名方注册、双人审批、一次性签发、复用拒绝和撤销后拒绝。 React typecheck/build 基线通过。权威证据见 `docs/evidence/transfer-approval-authority-20261102.json`。

后续已补 React/API 双语审批面和 agent token 重放账本；approval-to-agent 全链 E2E、正式发行矩阵、writer handover、防火墙、离线恢复和物理/断网边界继续 deferred。

## 2026-11-02 补充：转移审批 HTTP 与双语界面

新增 transfer signer/approval HTTP 契约和 React admin 面板。界面同步中英文，显示签名方指纹与状态；
请求需明确确认当前 generation；批准由另一 admin 执行；拒绝/撤销和 signer 撤销有独立操作。
浏览器和 API 不签发、显示或保存 token；一次性 token 仍走显式 CLI。

macOS Rust 13+8 通过；React typecheck/build 和浏览器 27/27 通过（含新增 transfer approval 契约）。
OrbStack 临时 HTTPS 服务覆盖 401、403、幂等登记、无效 key、同网关重复 open request、
不同管理员批准、重复批准、一次性签发、signer 撤销后拒发。React 审批界面截图契约通过。
权威证据见 `docs/evidence/transfer-approval-authority-20261102.json`。agent token 重放账本见下节；
审批后的完整 agent signed-transfer E2E 仍 pending。

## 2026-11-02 补充：agent token 重放账本

agent setup 在所有转移前置检查和 staged identity 校验后、首个身份字节替换前，把 signed token 的
transferId、原始 token SHA256、签名方 SHA256、精确 claims 和 operator fsync 写入
`/var/lib/kura-management-agent/transfer-token-uses/<transferId>.json`。同 ID 或同 token 重放直接失败；
写账本后中断也不会自动重放，必须重新审批/签发。transferId 被限定为安全路径标识。

agent packaging Python 28/28 通过（新增 durable replay 和 ID escape 用例），agent Rust 66/66 通过。
approval-to-agent 完整 E2E、生产 center2 systemd、物理主机/断网和发行矩阵仍 pending。

## 2026-11-02 补充：approval-to-agent staged E2E

在现有 OrbStack 上建立一次性 PostgreSQL authority，分别生成旧/新 CA/client identity；console 完成签名方注册、请求、另一 admin 批准和一次性签发。当前 agent Setup root 接收同一 token，迁移到新 endpoint/gateway 和 generation 2，transfer-token-use 记录的 raw-token SHA256 与签发 token 一致，setup check 通过。

60 秒请求实测 59,987ms 有效。发现旧 shared cross-platform target binary 可发过期 token；已抽出具名毫秒换算函数并新增 1..=900 回归测试，fresh independent Linux target 后 E2E 通过。这是 staged authority/agent 链路，不宣称 installed package、gRPC heartbeat、物理主机或断网验收。临时 DB/角色/工作树已删除。证据见 agent `docs/evidence/approval-agent-e2e-20261102.json`。

## 2026-11-02 补充：多中心 systemd 模板 candidate

新增 `kura-management-console@.service` 和 instance env 示例。setup 支持显式 `--instance`，布局为
`/etc/kura-management-console-<instance>`、`/var/lib/kura-management-console-<instance>`、
`/var/log/kura-management-console-<instance>` 和 `kura-management-console@<instance>.service`。
`init-instance` 只建目录/marker，不生成配置、不迁移、不启动。包维护现在枚举全部 template instance
并按各实例任务状态挡升级。

Linux root packaging 4/4、systemd-analyze、console Rust 14+8、React typecheck/build 通过。
formal console candidate 15c 已构建：deb SHA256 `7ed0b8fa60094075e7d1376d9d282c9899bbfc73a65df8932aa61deff0b0832d`。
现有 center2 仍为手工 active 进程；candidate 未安装、未切换 systemd，实例 start/health 未验证。
证据见 `docs/evidence/center2-systemd-20261102.json` 与 `docs/center-instances-v1.md`。

## 2026-11-02 补充：center2 systemd 实装与回归修复

candidate15c 首次升级暴露真实回归：默认实例 setup 遗留 `self.CONFIG`，ExecStartPre 失败，
OrbStack 默认中心短暂进入失败循环。已停止循环、补 default/instance runtime layout root 测试、
改为 `self.config`，并重建 candidate15d。默认中心恢复 ready 后完成受控升级。

candidate15d：deb SHA256 `4cc21a5930975fa675d0f72e9538fbefea60972b8054fe9281ff8a662255ce67`。
升级后 default 和 center2 均 ready。手工 center2 PID 138748 显式停止；config/env/task key 内容哈希不变，
仅按契约调整 ownership/mode。`init-instance/migrate/check` 通过后由
`kura-management-console@center2.service` 接管，MainPID 运行在 systemd cgroup。

center2 agent `center2-formal-agent-20261102` generation 4 重连 online；systemd restart 后仍恢复。
active instance 调用 migrate 被 `Stop the packaged console` 阻挡。两个中心均无 unsettled task。
该结果仅覆盖现有 OrbStack；生产发行、amd64/26.04 和物理/断网边界继续 deferred。

## 2026-11-02 补充：agent replay candidate14 runtime 升级

agent formal candidate14 已用 freshly packaged protocol crate 做 self-contained Linux staged test/release build。
agent packaging 28/28、staged Rust 66/66 通过。candidate14 在现有 center2 runtime 从 package13 升级到14；
安装 payload 含 durable replay/durable write；generation4 与 transfer backups 2–4 全部保留。
显式 activate 后 agent active，center2 gateway online，health ready，无未决任务。未重放一次性 token。

candidate14 deb SHA256 `351ae8077bce4ed34d039edfc12d504649e1b28f6b0505896551ffb0dbe3fe7a`。
证据见 agent `docs/evidence/transfer-replay-package-runtime-20261102.json`。

## 2026-11-02 补充：transfer signer rotation

console新增 `issue-transfer-signer-rotation`，用当前 private signer签发一次性
`YOFC-AGENT-SIGNER-ROTATION-v1`。agent新增 `rotate-transfer-signer`：精确绑定 gateway/generation/
当前与目标 SPKI SHA256/operator，durably consume授权后备份旧 signer/token，再原子替换 signer。
错误 signer、claim mismatch、过期和 replay 都不改状态。console Rust 14+8、agent Rust 66、
agent packaging 30/30 通过。此功能减少“不同 signer 需要未定义 recovery”的安全缺口；
正式 rotation E2E、CA/client 证书轮换和发行矩阵仍 pending。

## 2026-11-02 补充：signer rotation staged E2E

`issue-transfer-signer-rotation` 已移到离线 CLI 初始化路径，明确 unset `DATABASE_URL` 和
`CONSOLE_DATA_KEY_FILE` 后仍可签发。当前 agent Setup root 先导入 old signer 至 generation 1，
再接收 console 一次性 rotation token。E2E 结果：SPKI/current/new hash 精确绑定，setup check 通过，
old signer/token 备份一致，rotation use ledger 一条，replay blocked，new signer 保持有效。

该结果基于当前源码和临时 staged root，不宣称 installed runtime 已执行 rotation，也不代表正式包
rotation E2E。临时目录已删除。证据见 agent
`docs/evidence/signer-rotation-e2e-20261102.json`。

## 2026-11-02 补充：installed runtime signer rotation

formal console candidate25 / agent candidate15 已升级到现有 OrbStack runtime。console25 支持离线
`issue-transfer-signer-rotation`，并修复 83-byte legacy Ed25519 private structure 的 seed 解析。
agent15 支持 `rotate-transfer-signer`、use ledger、旧 signer/token 备份和原子替换。

runtime rotation 实测成功：gateway `center2-formal-agent-20261102` generation 4 保持不变；
old signer `fc8a9fd9...` 轮换为 `9e962aa8...`；rotationId
`af9f8f99-5ee2-4461-85d0-2cec91eb9e0b`；use ledger 一条；agent setup check 通过。
显式 activate 后 agent active，center2 gateway online，双中心 health ready，无未决任务。
新 signer/state 保存在 center2；旧 center1 private 未删除。证据见 agent
`docs/evidence/signer-rotation-runtime-20261102.json`。

## 2026-11-02 补充：新 signer approval-to-agent 全链与 token 撤销

恢复目标后本轮三项完成，installed runtime 未动（仍为 console25/agent15）。

**runtime dispatch 复验**：center2 创建两个 admin 后，新 signer（9e962aa8）状态下
gateway center2-formal-agent-20261102 generation 4 online、revision 6459，
`agent.status` 任务 576a862f QUEUED→SUCCEEDED，回执 agentAvailable=true。

**新 signer 全链 E2E**（agent `docs/evidence/approval-signer-runtime-e2e-20261102.json`）：
真实 center2 authority 上注册轮换后的 9e962aa8 签名方；新 approval `d8aad327`（双人）、
新一次性 token（SHA256 4fa85e7c...）。staged agent root 复刻生产 identity 坐标
（endpoint 28444/gateway/gen 4/signer）消费 token 转移到 generation 5，setup check 通过；
token use ledger 精确匹配；把 staged 状态完整回退到转移前坐标重放同一 token，
被 durable ledger 以 "already been used" 拒绝且账本仍一条。真实附带发现：首次签发因
token 输出目录无写权限失败，approval 事务已先行提交为 ISSUED（c02b12f2），契约的
"交付失败必须重新请求批准"路径得到真实证据。

**token 撤销 registry/检查**（console `docs/evidence/transfer-revocation-20261102.json`、
agent `docs/evidence/transfer-revocation-e2e-20261102.json`）：迁移 0014、
`revoke-transfer-issuance` CLI、gRPC `CheckTransferRevocation`（mTLS 网关身份，ACTIVE/
REVOKED/NOT_FOUND，全部审计）、agent 一次性 CLI 与 setup.py 消费前 fail-closed 检查。
临时 authority E2E：撤销后同网关经真实 mTLS 查询被拒且 staged root 不变；ACTIVE token
正常转移 generation 2；未知 id NOT_FOUND 拒绝。macOS console Rust 14+protocol 8、
agent Rust 67（新增状态判定单测）、packaging Python 32（新增撤销拒绝/缺二进制拒绝）、
React typecheck/build 通过。vendored protoc 3.3.0 不能接受 rpc 名与 message 同名，
请求消息命名为 `TransferRevocationCheck`。

遗留：带撤销功能的 formal 包构建与 installed runtime 升级 pending（升级顺序 console 先、
agent 后）；撤销的 HTTP/React 面板未实现；CA/client 证书轮换、writer handover、防火墙、
离线恢复、发行矩阵、物理主机与断网转移继续 pending/deferred。

构建缓存备注：VM `/tmp/revocation-{console,agent}-src` 与 `-/target` 保留（与当前源码
一致、release 已构建），下轮 formal 包可增量复用；protocol crate 需按打包流程重新
`cargo package` 取 fresh artifact。

## 2026-11-02 补充：console26/agent16 实装与真实 runtime 撤销验收

console candidate26（deb SHA `8f06001f`）与 agent candidate16（deb SHA `baf49cd7`，
fresh protocol crate `b7392f3c`）已按 console 先、agent 后顺序实装到现有 OrbStack
runtime。双实例 migrate 应用 0014；default 与 @center2 均 ready；agent16 显式
activate 后 center2 gateway generation 4 重连 online，新 agent.status 任务
SUCCEEDED（新进程 uptime 25s）。

真实 runtime 撤销验收：注册专用 probe gateway（真实 center2 client CA 签发，
不动生产 gateway），用轮换后 signer 9e962aa8 走全新双人 approval → 一次性 token →
立即撤销；staged root 内置 installed agent16 二进制，transfer-identity 经真实
28444 mTLS CheckTransferRevocation 被拒（staged root 逐字节不变）；installed CLI
查零 UUID 得 NOT_FOUND。审计两行：transfer.revocation SUCCEEDED、
transfer.revocation.check REVOKED。probe gateway 已禁用保留为证据。
权威证据：agent `docs/evidence/transfer-revocation-runtime-20261102.json`。

构建备忘记（本轮实测）：VM `/tmp` 为 tmpfs（约 7GB）且不进 Mac 侧共享——cargo
debug 产物会填满它导致 cc 报奇怪编译失败，构建前清旧 target，产物走磁盘路径再
经 `~/OrbStack/<vm>/...` 直接 `cp`；orbstack-vm-dev skill 已补此坑与双向直拷说明。
`/tmp/formal-{console-26,agent-16}-*` 保留供下轮增量。

剩余：CA/client 证书轮换（下一项）、撤销 HTTP/React 双语面板、writer handover、
防火墙、离线恢复、发行矩阵（amd64/26.04）、物理主机与断网转移。

## 2026-11-02 补充：同中心 client 证书轮换（任务4）

agent 新增 `rotate-client-certificate`：服务停止且无未决副作用时，校验新证书对由
当前信任链签发、cert/key 匹配，拒绝 foreign CA/mismatch/同证书且零状态变化；
备份旧对与 receipt 到 `certificate-rotations/<n>/` 后原子替换 agent.pem/key，
endpoint/gateway/generation 全不变、无 transfer token。agent packaging 34/34
（新增 2 用例）、staged Rust 67/67。agent17（deb `198fa439`）已实装。

真实轮换（生产 gateway center2-formal-agent-20261102）：center2 client CA 签新
叶证书（`368c50b9`）→ 停服轮换（备份 rotation 1，receipt 绑定 generation 4）→
console26 `rotate-gateway-certificate` 停机窗口内登记新 SHA（无重连空窗）→
activate 后 generation 4 保持 online，agent.status SUCCEEDED，signer 9e962aa8
与 endpoint 不变，审计 SUCCEEDED。回滚路径：手工恢复备份对并用旧证书重跑
rotate-gateway-certificate。边界：**只覆盖既有 CA 下的叶证书轮换，不覆盖中心
client CA 信任锚本身的轮换**；同中心叶轮换为 operator 级 CLI，无跨中心双人审批。
证据：agent `docs/evidence/client-certificate-rotation-20261102.json`。

## 2026-11-02 补充：撤销 HTTP/CSRF 接口与 React 双语面板

复用 signer/approval 模式：`GET /api/v1/transfer/issuances`（admin，只读）与
`POST /api/v1/transfer/issuances/{id}/revoke`（admin+同源+CSRF；401/403/404/409 语义，
`transfer.revocation` 审计）；openapi-v1.json 扩展并重新生成 schema。React 转移面板新增
"已签发转移授权"中英文表格与撤销操作（仅 ACTIVE 可撤、撤销人/时间/说明可见），token
字节不经浏览器。验证：console cargo check、React typecheck/build、mock 浏览器
transfer-approval 2/2 + 全量 22/22（本地 vite preview 4173；18443 现被 OrbStack 代理到
VM、证书与 dev SPKI 不匹配）；OrbStack 临时真实实例 HTTP 契约全过（401/403×3/列表/
撤销/REVOKED+撤销人/409/404 + installed agent mTLS 对同一实例 REVOKED 拒绝、ACTIVE
放行 + 审计 4 行）；新增 live profile `transfer-revocation` 浏览器 1/1（真实登录→drawer→
UI 撤销→双语切换，三张截图）。临时 DB/实例/凭据已清理，生产 runtime 未动（console26/
agent17 不含本面板，随下一 candidate 发行）。证据：
`docs/evidence/transfer-revocation-http-20261102.json`；契约见 `transfer-approval-v1.md`。
剩余：writer handover、防火墙、离线恢复、发行矩阵、物理主机/断网；CA 信任锚轮换仍开放。

## 2026-11-02 补充：网络写入者显式交接（writer handover）

agent setup.py 新增正向命令：`grant-network-writer --interfaces --operator`（停服、
journal settled、当前无 writer 且无 confirmed state 才可授予；接口名严格校验）与
`handover-network-writer --operator [--discard-config]`（停服、journal settled；
默认保留 /etc/netplan/90-yofc-managed.yaml 让系统沿用最后确认配置，归档
confirmed.json 与 receipt 到 network/handovers/<n>/，清除 writer 列表——同时解除
purge 阻挡；--discard-config 仅在受管文件与确认内容逐字节一致时删除，漂移拒绝）。
packaging 38/38（新增 4 用例）、staged Rust 67/67（src 未变）。agent18
（最终 deb `3e09b61d`，同版本重打修复空段校验，首版 81e6ad22 作废）已实装；
installed setup.py staged E2E 11 步全过（grant 校验×4/purge 阻挡复验/未决拒绝/
keep 交接归档/purge 放行/drift 拒绝/discard 生效）。生产 agent 无 writer、
network state 未动，activate 后 gen4 online、dispatch SUCCEEDED。证据：agent
`docs/evidence/network-writer-handover-20261102.json`；命令语义见 agent
`packaging/README.md`。边界：本轮仅 setup/state 层，未做真实接口写入。
剩余：防火墙（nftables）、离线恢复、发行矩阵、物理主机/断网；CA 信任锚轮换开放。

## 2026-11-03 补充：防火墙自有 nftables 表（方案 §10 v1）

agent 新增 `src/firewall.rs`：自有表 `inet yofc_kura` 的完整候选生成
（input 链 allow-list，tcp/udp 端口 + 显式 ip/ip6 源前缀，policy drop|accept）、
`nft -c -f -` 预检、`destroy table`+声明单事务原子应用（绝不全局 flush）、
`nft -j` 回读校验（{"nftables":[...]} 真实 wire 格式，逐 rule 元素）、
token 绑定 confirm/rollback（恢复上一确认规则集或销毁表）、启动/超时自动回退、
外来防火墙管理表检测（prepare 拒绝 FORBIDDEN、status 报告）。UDS 方法
`firewall.status|prepare|apply|confirm|rollback`（首帧白名单扩展，root/admin
peer，类型化错误）。packaging unit 显式授予 `CapabilityBoundingSet=
CAP_NET_ADMIN`（原空集连 list ruleset 都 EPERM）。验证：macOS 与 Linux staged
Rust 各 72/72（firewall 单测 4 项 + UDS 分发契约 1 项新增）；root 真实 nft
集成 `cargo test real_nft -- --ignored` 通过；installed agent19（最终 deb
`6a33f9ec`，同版本三次重打：首帧白名单修复、unit 能力修复）生产 UDS E2E
12 步全过（真实表应用/回读/commit/rollback 恢复/外来表拒绝+报告/清理无残留）；
终态 gen4 online、dispatch SUCCEEDED、双中心 active、无防火墙表残留。
证据：agent `docs/evidence/firewall-nftables-20261103.json`。边界：v1 仅
allow-list 输入链；端口转发/NAT、Kura 防火墙 API/PID 适配、中心协议/HTTP/
React 面板、早启加载与真实流量验收均 pending/deferred（方案阶段 3 后续）。
途中如实记录：nft JSON 三个结构教训入档；一次 cargo fmt 顺带规范化既有
未提交文件格式（语义无变化、全绿）。

## 2026-11-03 补充：离线恢复事务（方案 §11 v1）

agent 新增 `src/recovery.rs`（默认禁用，`AGENT_RECOVERY_UNITS` 显式白名单 +
`AGENT_RECOVERY_SNAPSHOT_DIR` 可覆盖）：配额化 opaque 制品上传（32/64MiB、
128 文件、0600 create-only）→ SHA256 精确校验 + 白名单 unit 的 prepare →
token 绑定 execute 走 §11.4 全状态机 STOPPING（确认停）→BACKED_UP（文件级
备份+manifest）→COMMITTING（放置为 snapshot_<max+1>.xml）→STARTING→VERIFYING
（active+字节哈希）→SUCCEEDED（durable receipt）；任一步失败 COMPENSATING
（哈希证明的候选移除、备份逐文件还原、未知文件清除、重启），结局 FAILED/
RECOVERY_REQUIRED；启动时非终态 journal 先补偿。UDS
recovery.status|upload|prepare|execute（首帧白名单、类型化错误）。测试：
macOS 76/76（recovery 4 新用例，状态化 fake systemctl——grep -qx 教训入档）；
Linux staged 首跑 1 个既有 network 用例并行 flaky（与 recovery 无关，单线程
+三次复跑 76/76 全绿，如实记录）。installed agent20（deb `0f895fb3`）配
专用 probe unit + env 白名单后生产 UDS E2E 16 步全过（真实 systemd 停启/
snapshot_1 逐字节放置/备份/receipt/COMMITTING 中断→重启补偿：候选移除、
committed 保留、FAILED；清理 pristine）；移除 env 后 enabled=false、gen4
online、dispatch SUCCEEDED、probe 全清。证据：agent
`docs/evidence/offline-recovery-20261103.json`。边界：制品 v1 为 opaque
（XML 解析/加密快照/密钥校验/factory 删除语义/业务健康核验/Java 工具/
中心协议 React 面板均 pending）；amd64/26.04 deferred。至此下一批清单的
可执行项（新 signer E2E、revocation、证书轮换、writer handover、防火墙、
离线恢复）全部闭环；剩余为发行矩阵（缺环境 deferred）、物理主机/断网
（deferred）与 CA 信任锚轮换（开放）。

## 2026-11-03 补充：操作手册（安装/卸载/控制台，含截图）

新增 `docs/manual/operations-manual-zh.md` + `docs/manual/screenshots/`（11 张 PNG）。
覆盖：安装前准备、中心安装/初始化/多实例、agent 安装/身份导入/Kura 授权/网络写入者
授予交接/防火墙与恢复开关、remove/purge 保留矩阵、升级 guard、控制台全操作（登录/
错误/语言切换/网关总览/查询状态/详情抽屉/转移审批三段/已签发授权/任务记录，中英
双语）、中心 CLI 速查、排障速查（含 Origin 403、/health/ready、staged 调试绝对路径
坑）。截图取自 2026-11-03 当前源码构建的临时实例（专用 DB/证书 + staged agent 真实
心跳在线，含一次真实 agent.status 查询），事后临时实例/DB/进程全清，生产三服务
active/ready 未动；命令转录为同日实测输出。过程教训入手册排障节：应用为单页视图
切换无 URL 路由（Playwright 等 row 不等 URL）；登录 403=Origin 与 CONSOLE_ORIGIN
不一致；staged 身份 env 的 CA/CERT/KEY/STATE_DB/SOCKET 为绝对路径需逐项覆盖。

## 2026-11-04 补充：Docker/compose 部署 + 默认账号首登强改 + 3 VM 接入验证

**首登强改（全栈功能）**：迁移 0015（mgmt_users.must_change_password）；
`ensure_bootstrap_admin` 在 migrate/serve 时 seed——仅当库中无启用 admin 时创建
`admin/admin`（tenant default，argon2 unchecked hash，audit user.bootstrap）；
login/session 返回 `mustChangePassword`；新 `POST /api/v1/auth/password`
（CSRF+同源、验当前密码、新密码走 ≥12 强度、清标记、撤销该用户其他会话、审计）；
axum 中间件 `enforce_password_change`（from_fn_with_state）对 must-change 会话在
服务端拦白名单外全部 API（403 PASSWORD_CHANGE_REQUIRED）。React 强制改密视图
（中英、两次确认、成功后回登录页）+ 闸门（must-change 时不连 SSE/不轮询）。
测试：console Rust 15/15（新增 seed unchecked 单测）；mock 浏览器 24/24
（新增 2 用例：mismatch 守卫/成功回登录/已改密直达主界面）。

**Docker**：`docker/Dockerfile` 多阶段（node:24 build React → rust:1.95 --locked →
ubuntu:24.04 运行时，非 root console 用户，136MB）；`entrypoint.sh`（root 备状态→
setpriv 降权：task-key-init 缺失时 → migrate（含幂等 seed）→ exec serve；其他子命令
透传）；`compose.yml`（postgres:16-alpine healthcheck + console，卷 pg-data/console-data、
./tls 只读挂载、端口 18443/18444）；`tls-gen.sh`（演示 CA+server SAN
localhost/127.0.0.1/host.orb.internal + 按名签发网关 client）；README 部署/接入/生产注意。
构建踩坑入档：sqlx::migrate! 编译期需要 migrations 目录（COPY 补）；entrypoint 函数内
exec 会整体替换脚本（init 后容器退出循环——改为普通调用仅 serve exec）；带 State 的
middleware 必须 from_fn_with_state。

**compose 全链验证**：health ready → admin/admin 登录 mustChangePassword=true →
gateways 403 PASSWORD_CHANGE_REQUIRED（服务端强制）→ 错误当前密码 401、弱新密码
400 → 改密 200 → 旧密码登录 401 → 新密码 mustChange=false 且 gateways 200。

**3 台新建 OrbStack VM 接入**（yofc-agent-01/02/03，ubuntu 24.04 arm64，各 ~810MB）：
agent 包 20（/mnt/mac 直读 dpkg -i）→ import-identity（endpoint
https://host.orb.internal:18444，client 身份 gw-001..003 由 tls-gen 签发，VM 本地
/tmp/ident 0600）→ activate → 中心 register-gateway ×3 → 三台全部 online；
admin 改密后会话各下发 agent.status → 三台 SUCCEEDED（agentAvailable=true）。
途中修正：tls-gen client 循环 dash 不支持 <()；server 证书 SAN 需含 host.orb.internal；
exec 需 `-u console`（task key 目录归属检查）。

收尾：compose 栈 down（卷保留：pg-data/console-data 可随时 up 回到改密后状态；
admin 测试密码存 .phase1-dev/docker-compose-admin-pass.txt）；镜像
yofc-management-console:{20261104,latest} 136MB 保留；3 台 VM 保留运行（agent 会随
栈停止转 offline，up 后自动重连）。生产 VM 三服务 active/ready 未受影响；未提交未推送。


## 2026-11-05 补充：架构重构、缺陷修复及完整受影响验证

按用户追加要求，从 47efca29/de6eed61 交接起全量核对后续台账，并审查代码质量与逻辑架构。F01–F14 已修复，新增 F15 协议温缓存、F16 公钥格式、F17 host nft JSON、F18 flock 关闭继承窗口、F19 demo PKI 默认弱摘要均已修复。重构共享 durable/process/system_change、typed 系统状态与中心 task_lifecycle/审批业务。admin/admin 保留，首次设置只填新密码/确认，后台强制闸门，成功直接进入控制台；普通改密仍验当前密码。

Java configuration78/bridge33、agent host/Linux90、中心16+协议8、显式PG5+mTLS3、Python agent42/console Linux9、严格Clippy、React构建/类型及默认浏览器30通过。IDEA真实完整282项运行集/原GWT登录/10项业务通过；OrbStack公开UDS内核/UID16与命名/未加载实例guard12；Parallels实际严格沙箱17、中心干净首装与包生命周期保留8；Orb/Parallels/Docker各实际首次设置双语通过。精确制品与证据以质量审查最终节为准。

当前候选 console review20261105-1 / agent review20261105-2 均 arm64。原 OrbStack 默认中心、center2、agent 继续 active 原包26/20；新夹具停止保留。Parallels新中心包purge、身份/密钥/数据库保留，原手工agent未覆盖，VM恢复正常stopped。没有reset/clean、提交、推送或远端覆盖。

整个改造没有完成：防火墙/NAT/适配/远程流程/早启、完整Java业务恢复、restart/reboot闭环、整库一致恢复与正式发行仍待实现；真实NM/cloud-init、物理硬件、amd64及硬断电等保留deferred。不能把本轮 opaque+unit 恢复证据核定为 Kura 业务恢复。原11-04 Review 文档按审查当时日期保留，本段与质量审查最终节替代其已修复项状态。

下一会话完整提示词：`reviews/continuation-prompt-20261105.md`。本轮按用户要求完成收尾后暂停目标。

## 2026-11-05 恢复实施：配置投影与本机配置入口新需求

整体goal恢复active。按最新交接保留全部旧工作树，六仓HEAD未变。基准metatype与本地化展示分离，补齐字段并集/权威类型及脱敏/中英只读原因。bridge宿主及Orb各37、受影响浏览器7、IDEA新完整282项运行集/原GWT登录/实际25个PID中英50次读取通过；精确范围和失败记录见 `phase2-configuration-projection-20261105.md`，不扩大为全部factory业务或完整配置编辑器。新夹具停止保留/网关禁用；原Orb三服务active原包26/20。Parallels缺Java，本轮Java验证未执行，已安排正常关机。

用户新增在原Kura GWT设置agent证书、console认证与远程管理开关的需求，纳入当前实施，具体合同与顺序见 `local-agent-provisioning-v1.md`。开关控制远程通道，agent本机恢复继续运行；身份仍独立、归属不抢占、私钥不进入Kura快照。当前为pending，下一步实际接通，其他未交付业务继续保留。

## 2026-11-05 远程管理开关基础与网络方向确认

用户确认先保留NM并允许Netplan按系统配置共存，实际测试出现冲突后再基于现场取舍，不预先卸载或切renderer。Kura可独立使用和安装后才显示远程管理入口的要求继续有效。

agent已增加无身份本机运行、默认关闭的持久通道开关、明确Kura委托、准入计数/旧session挡写、开关意图与RECOVERY_REQUIRED保留及包维护守卫。宿主95+Clippy、Python43、真实无身份公开UDS7通过；新夹具停止保留。GWT开关/身份交付/正式包初始化与迁移及Linux系统验证仍pending，不扩大为完整产品交付。详见 `agent-remote-control-v1.md`；原Orb三服务active原包、Parallelsstopped，未提交推送。


## 2026-11-06 受保护普通启动续接

新增独立normal startup及可选原快照格式授权，保持闸门实际走原snapshot/CM加载；IDEA/Orb原GWT Count9、真实DS与密码、放行前26/187精确CM及真实agent重读挡写通过。详见 `reviews/guarded-normal-startup-20261106.md`，包含本轮原业务pending/deferred台账、失败现场和准确停服状态。NORMAL_BOOT_ADMITTED仍非完成，后续桥接/全健康、显式完成/普通写释放及无闸门启动继续pending。goal active；知识库6537c595；原Orb四服务原console26/agent20 active，新运行全停，两VMstopped。
