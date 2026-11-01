# Management Console

本仓库是多网关管理中心，包含服务端与 `frontend/`。2026-10-31 从原
`kura-management-backend` 改名，并吸收原独立前端；不要重新建立同级 backend 项目。

- 完整改造方案与验收边界以 `docs/management-redesign-plan.md` 为准。
- 最新续接入口为 `docs/management-redesign-handoff-20261101.md`。本轮受限远程配置update和中英文编辑器
  已接通（默认关闭、审核PID显式开启），见 `docs/phase2-configuration-update-validation-20261101.md`。
  本轮收尾后用户要求暂停goal；后续pending以最新交接为准，下段按增量保留历史范围。
- 旧基线为 Java/Spring Boot + Vue/MQTT。2026-11-01 已实现 Rust/Axum/Tonic + React/TypeScript
  的独立只读闭环和受限 Netplan 网络事务原型；并非全方案完成。续接先读
  `docs/phase1-validation-20261101.md`、`docs/phase2-validation-20261101.md`、
  `docs/phase2-takeover-validation-20261101.md`、`docs/phase2-writers-validation-20261101.md`
  与 `docs/phase2-provider-coordination-20261101.md`、网络契约，避免重做整合。
  核心配置协调及实际旧网络 PID 的确认值/状态适配已实现，证据另见
  `docs/phase2-netplan-adapter-validation-20261101.md`。新增公共事务 API 和原 REST 附加事务入口，
  真实 IDEA/HTTP/UDS 证据见 `docs/phase2-network-rest-validation-20261101.md`，
  使用契约见 `docs/network-transactions-rest-v1.md`。原 GWT 已有受限网络事务 UI，支持明确确认/回退、
  会话隔离和中英文；见 `docs/network-transactions-gwt-v1.md` 与
  `docs/phase2-network-gwt-validation-20261101.md`。其 IDEA/浏览器写事务使用明确的 test-only owner，
  不代表 Linux 实际 Kura/agent 写入验收；完整旧字段写入映射和正式 provider 发行 profile 仍未交付。
  后续开发运行集已增加显式 none/netplan 变体；真实 Linux 低权限 Kura 经 UDS 完成
  dummy 接口的 prepare/rollback/confirm，修正无默认网关仅改 MTU 的 GWT 校验。
  见 `docs/phase2-network-profile-validation-20261101.md`。这是开发运行集与受控接口证据，
  正式 deb/profile、NM/cloud-init 与物理有线验收仍未交付。VM fixture 已 restored，不重放 prepare/resume。
  原 GWT 后续补齐 Netplan Ethernet 类型并修正双语终态倒计时；完整构建/65 项、React 浏览器 11 项、
  新命名 test-only IDEA/GWT 浏览器 4 项通过，见 `docs/phase2-network-ui-regression-20261101.md`。
  该命名 fixture 也已 restored，同名不能重放；这次没有追加 Linux 写入验收。
  图谱连接现状见 `docs/codebase-graph-repair-20261101.md`。
- 已增加显式 `kura-core-netplan` / `kura-networking-netplan` deb 入口；旧默认模块保留，
  构建须同时选 `netplan-package` 与对应 `-pl`。13 项本地回归、11 项 React 浏览器、
  IDEA 原控制台及 OrbStack24.04 arm64 包升级/remove/重装/purge 验证已记录，见
  `docs/phase2-netplan-package-validation-20261101.md`。新包业务组合及 agent UID/socket 授权仍 pending；
  OrbStack 全局覆盖导致本包完整 systemd 沙箱 deferred，不能套用旧 Parallels 结果。
  `/opt/yofc-kura-netplan-package-20261101` 最终 purged，失败和保留数据归档仍在 VM，禁止删 state 重放。
- React console 已提供中英文，词库与扩展约定见 `docs/console-i18n-20261101.md`。
  新界面文案需同时补齐双语，协议码、ID 和用户输入保留原值。
- 核心配置条件更新 API 与 Java 内部 handler 已实施；73 项核心配置、24 项 bridge、131 项 container、
  浏览器17、IDEA与Linux真实CM/DS/快照26+12证据见 `docs/phase2-configuration-revision-validation-20261101.md`。
  新配置包最低1.4，container须用放宽上限后的新制品。远程update随后已受限接通，完整配置管理仍未完成。
  新宿主与Linux fixture 均stopped-data-retained，不重放prepare；失败目录/日志保留。
- 网络 UNKNOWN 已有认证原计划与网络业务日志绑定的结果核定、一次审计和双语说明。
  57 项 agent、17 项浏览器、IDEA HTTP31/live1 及 Linux 最终23项证据见
  `docs/phase2-network-outcome-validation-20261101.md`。Linux 为真实 dummy 动作后的回执损失注入，
  不是真实断电；首次观察捕获 SLAAC 收敛差异，原失败保留、最终全基线恢复。
  新fixture `/opt/yofc-network-outcome-20261101` 已 stopped-data-retained / gateway disabled，
  禁止删 state 重放。旧无命令标记/证据不足仍 UNKNOWN，配置写入/快照和阶段2–5仍pending。
- 后续中心任务 input/result 已接入独立文件密钥的加密与旧数据原子迁移。
  CLI/serve 需要 `CONSOLE_DATA_KEY_FILE`，初始化与恢复见 `docs/task-payload-storage-v1.md`，
  当前代码与准确验收范围见 `docs/phase2-task-storage-validation-20261101.md`。
  这只替代下段“中心敏感存储未实施”中 input/result 的状态；旧介质/agent历史、key轮换和正式发行仍未完成。
- 执行前日志/UNKNOWN 查询已实施，包含新的 agent 计划及回执加密、跨连接执行隔离和双语迟到回执。
  宿主/Linux agent各50项、默认浏览器16项、IDEA实际登录与读取、新版agent包生命周期证据见
  `docs/phase2-task-journal-validation-20261101.md`。新包夹具已purged、gateway disabled，不重放。
  这不代表中心敏感存储、真实未决写入业务对账、Linux新Kura组合或配置写入已完成。
- 配置详情 get 已打通 React/HTTPS/持久化任务/mTLS/UDS/OSGi，默认浏览器15项及真实
  IDEA/Linux双语读取、密码脱敏和agent重连通过，见 `docs/phase2-configuration-read-validation-20261101.md`。
  新只读fixture `/opt/yofc-config-read-20261101` 最终stopped-data-retained，测试gateway disabled，
  禁止重放prepare。该增量未重打agent deb，旧包revision5不能代表新get协议验收；写入/快照仍pending。
- 独立观察模式 agent deb、显式身份导入和 Kura 实际 UID 授权已实施，真实包组合的
  配置/快照/网络查询、mTLS重连和升级/remove/重装/purge见
  `docs/phase2-agent-package-validation-20261101.md`。启动锁死锁已修复；最终候选revision5。
  新fixture `/opt/yofc-agent-package-20261101` 最终purged，归档/失败保留，测试gateway已disabled。
  无网络写入授权，完整沙箱与发行矩阵仍deferred，阶段2–5未完成。
- 多网关验证采用一台 2 vCPU / 4 GiB Parallels Ubuntu 26.04 配合现有 OrbStack，
  只读系统沙箱通过不代表实际 NM/cloud-init 交接通过；见 `docs/phase2-parallels-validation-20261101.md`。
- 新架构：浏览器 HTTPS API/SSE；中心与 agent 为 agent 主动建立的 gRPC 双向流/Protobuf/mTLS；
  agent 与 Kura 为受系统身份校验的 Unix domain socket。
- 仅网关 agent 执行 Netplan/nftables 和离线恢复；中心不直接写网关文件。
- `kura-management-agent` 保持独立进程和独立仓库，不进入 Kura JVM。
- 使用父工作空间的知识库，显式 `--scope ws --dir=/Users/heyoulin/iot-kura-develop/git`。
- 代码发现优先 codebase-memory，选择本仓库真实 root_path；改名后旧 backend 索引不是当前证据。
- 当前统一构建：`./scripts/build-with-frontend.sh -Dmaven.repo.local=/Users/heyoulin/.m2/repository`。
- 当前本地 `origin` 仍指向原 backend 远端；旧 console 远端已有独立历史。
  远端整理须保留两套历史，不能将新项目直接强推覆盖旧前端远端。
