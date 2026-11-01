# 执行前日志、未知结果协调与双语回执验证

日期：2026-11-01。接续配置详情读取增量。阶段 2–5 继续进行；配置 update、快照 save/rollback
尚未开放。本次先交付执行前持久化、断流隔离和 UNKNOWN 回执查询，避免后续写操作在失联后被自动重放。

## 实际代码与语义

共享 Protobuf 为 `ConsoleFrame` 增加字段 3 `TaskQuery`，只按 UUID 查询回执，不授予执行权。
agent 通过 `task.reconcile.v1` 能力声明支持，中心只向支持者发送查询；单帧最多 16 个 ID，
中心每批查询最多 2 项，查询租约 10 秒。新增错误 `OUTCOME_UNKNOWN`，HTTP/OpenAPI/生成 TS 同步。
只读操作显式列为 1、2、3、4、8、9、11，其他及未来未知操作保守视为有副作用。

agent 的 `executions` 表在动作前原子写入 RUNNING admission，只有首次 claim 获得执行权。
完整 Protobuf 请求的 SHA 绑定操作、幂等键、目标与有效期；并发重复返回 RUNNING 或原回执。
SQLite 使用 WAL/FULL。执行计划和新回执均 AES-256-GCM 加密，分别绑定 `plan:<request_hash>`
与 `result:<request_hash>` AAD；随机 nonce、独立 32 字节密钥位于数据库相邻 `.plan-key`。
密钥要求当前 UID 所有、0600、O_NOFOLLOW，首次写入后 fsync 文件与目录。
已有 admission 但密钥缺失、权限不安全或密文校验失败均拒绝继续，不生成替代身份或重执行动作。

执行 worker 属于 agent 进程，跨 gRPC 连接共享 2 个执行槽；断流不会取消已经接纳的 future。
先落最终回执，再尝试发送。进程重启将仍 RUNNING 的 admission 改为 UNKNOWN / AGENT_RESTARTED，
不自动重放。查不到回执返回 UNKNOWN / RECEIPT_NOT_FOUND，查询本身不创建 admission。
终态至少保留 7 天，且有效期已过才可清理；RUNNING/UNKNOWN 不自动淘汰。
旧 `results` 表保留并兼容查询，旧历史没有被清空或批量改写为密文。

中心只对只读操作重新分配过期执行租约。运行中副作用任务的租约或 TTL 过期，保留为
UNKNOWN / OUTCOME_UNKNOWN；未分发任务和只读超时仍可 EXPIRED。
回执须匹配 enabled 网关、租户、当前连接 epoch 和本次 dispatch epoch。
UNKNOWN 可以接纳原 TTL 之后的迟到终态，RUNNING 查询回执不抹除已有 UNKNOWN，终态不可被随后回执覆盖。
状态事件与审计同 PostgreSQL 事务保存。新增迁移 `0006_task_reconciliation.sql`。
有未决副作用 UNKNOWN 时拒绝新写任务，幂等重试返回原任务；明确 rollback 作为补偿操作保留。

React 提供中英文 OUTCOME_UNKNOWN 警示、能力标签和状态；未决计数包含 UNKNOWN。
打开的任务抽屉按最新列表派生，迟到回执到达时即时更新；配置 get 的成功不再误显示为网络事务回执。
补齐页面分类标题的中英文。原始协议码、任务 JSON 和用户输入保持原值。

agent 包维护脚本检查执行日志：存在未决副作用 admission 或数据库不可读时，拒绝生命周期清理。
普通卸载继续保留身份、密钥与数据；purge 仍受明确归属与网络交接检查限制。

## 开发检查与故障契约

证据路径相对本仓库。机器清单为 `docs/evidence/task-journal-20261101.json`。

| 验证 | 实际结果 | 证据 |
| --- | --- | --- |
| macOS agent | 50 项通过 | `target/task-journal-agent-fifty-20261101.log` |
| Rust console / protocol | 2 / 6 项通过 | `target/task-journal-console-final-20261101.log` |
| PostgreSQL UNKNOWN 契约 | 默认 ignored 的 1 项显式运行通过；独立 schema 创建后清理 | `target/task-journal-postgres-contract-20261101.log` |
| mTLS 负面契约 | 默认 ignored 的 3 项显式运行全部通过 | `target/task-journal-mtls-20261101.log` |
| 包 Python 单元 | 17 项通过 | `target/task-journal-packaging-unit-20261101.log` |
| Rust fmt / 两端编译 | 通过 | 本次 fmt 检查；`task-journal-{agent,console}-build-20261101.log` |
| React 类型与生产构建 | 通过，大 chunk 提示仍存在 | `target/task-journal-react-build-final-20261101.log` |
| 默认浏览器 | 16 通过，0 skipped/unexpected/flaky | `target/task-journal-browser-final-20261101.json` |
| 实际断流与回执协调 | 19 项通过 | `target/task-journal-live-contract-first-20261101.json` |

PostgreSQL 契约覆盖副作用 UNKNOWN、读取与未分发过期、旧 epoch/跨租户/disabled 拒绝、
新 epoch 补迟到终态、Running 不抹未知、终态不覆盖及审计。

19 项故障契约使用新的宿主身份 `task-journal-20261101-gateway`、独立 SQLite/socket 和明确标识的
受控测试桥接。通过真实 HTTPS/PostgreSQL/mTLS gRPC/UDS，先确认 RUNNING 已落盘，再令该夹具的
连接 epoch 失效，观察实际 gRPC 关闭后才释放 UDS 回复。worker 继续完成，桥接调用计数为 1。
随后只对该夹具注入中心丢回执和 TTL 已过状态，新连接取回原回执，SQLite 密文、updated_at 均不变。
另注入没有本地 admission 的 UNKNOWN 写任务，证明查询不执行、拒绝新写、幂等返回原任务、rollback 可用。
这些注入是协议故障证据，不是 Java Kura 写入或真实 Linux 未决网络变更证据。
该宿主夹具最终停服保留、gateway disabled，不重放 prepare 或删除 state。

浏览器中的网络任务与迟到结果为明确模拟 UI 契约。覆盖中文/英文警示、配置读取成功标签、
UNKNOWN 未决计数和打开的抽屉随事件刷新；截图为
`target/task-journal-{uncertain,reconciled}-ui-20261101.png`。

## IDEA 完整运行门槛

在单元/契约和默认浏览器之后，通过 IDEA Run `Kura management phase1 isolated` 启动完整 Kura，
PID 23780。CUA 实际登录 `http://localhost:18080/admin/console`，状态页可见 admin 和设备信息。
真实配置 HTTP 31 项及 React live 浏览器 1 项通过，中英文切换、密码脱敏走真实 OSGi 桥接。
截图为 `target/task-journal-idea-configuration-ui-20261101.png`。
IDEA Stop 后独立 ps 确认退出；281 个运行集清单项 SHA 全匹配。
门槛汇总 `target/task-journal-idea-gate-20261101.json`，未清理原 target/runtime 或个人 ~/.kura-dev。

首轮 HTTP 检查在 viewer 登录处失败，日志 `target/task-journal-idea-live-20261101.log` 保留。
后续独立重跑为 `target/task-journal-idea-live-retry-20261101.json`，31 项通过；服务端登录限流未改。
本轮 Java bridge 源码未修改，不将历史 Java 单元测试计为本轮新执行。

## Ubuntu 24.04 arm64 与新版包

IDEA 门槛后复用既有编译 VM `kura-package-validation-20261029`。
新 protocol 以独立 Cargo target 产出本地 0.1.0 crate；经 SHA 校验形成完整 staged source，
复制到 VM 独立目录和 Cargo target，不依赖 console sibling checkout。
Linux 50 项通过，debug ELF 编译成功，最大 glibc 需求 2.39。
日志 `target/task-journal-linux-{tests-isolated,build}-20261101.log`。
第一次未指定 VM 专用工具链 PATH，cargo 不存在、退出 127；失败日志
`task-journal-linux-tests-20261101.log` 保留，不算通过。

新候选 `kura-management-agent_0.1.0~taskjournal20261101-1_arm64.deb`：

| 制品 | SHA256 |
| --- | --- |
| Linux debug agent | `d928d36e0f38c066059f595ea292bcc87e13a48ec302860f4072e7f5aaa269f1` |
| 新 agent deb | `522a4a9f33c134e2798df887cebe4f940d9f356baefae1b0bf64ec0543d16f02` |

联调使用既有 `yofc-management-network-20261101`，新夹具 `/opt/yofc-task-journal-package-20261101`，
gateway `task-journal-package-gateway-20261101`。只安装本次 agent 包，不安装 Kura/core/networking 包；
首次不会自动启动，显式导入独立 mTLS 身份再 activate。网络 allowlist 空，Kura UID 未授权。

| 实际步骤 | 结果 | 证据 |
| --- | --- | --- |
| 安装、显式身份导入与启动 | 通过；Kura 缺席的 26 项 HTTP 检查通过 | `target/task-journal-package-{install,identity}-20261101.json`、`task-journal-linux-absent-20261101.json` |
| agent 重启与原回执查询 | 13 项通过，原 key/密文/时间戳/执行数量不变；全部线程 no_new_privs/CapEff0；nobody socket EACCES | `target/task-journal-linux-reconcile-20261101.json` |
| 未决任务维护脚本防护 | 实际 dpkg remove/purge 均拒绝，身份/数据未变 | `target/task-journal-package-guard-20261101.json` |
| 同候选重新安装的 upgrade 维护路径 | 通过，逻辑执行记录与保留文件 SHA 不变；不是从旧 revision5 升级 | `target/task-journal-package-upgrade-20261101.json` |
| 普通 remove、重装、重新激活 | 身份、执行加密 key、用户标记和逻辑日志保留 | `target/task-journal-package-{remove,reinstall,resume}-20261101.json` |
| 重装后再次重启与回执查询 | 13 项通过 | `target/task-journal-linux-reinstalled-20261101.json` |
| 停服和明确 purge | 归档本夹具拥有的身份/状态后清理；旧 owner、网络与旧夹具 SHA 不变 | `target/task-journal-package-{settle,purge}-20261101.json` |

维护脚本防护使用停机后的明确 synthetic UNKNOWN 元数据行，没有为它创建中心任务或执行系统动作。
拒绝后只按该测试 UUID 和测试幂等键删除注入行，复核原逻辑日志完全恢复；这是测试恢复步骤，
不是人工解除真实 UNKNOWN 的产品接口。SQLite 原始结果、身份与 purge 前归档只在 VM 私有目录保留，
11 份无凭据步骤回执复制在 `target/task-journal-linux-receipts-20261101/`。
Linux 回执协调注入的是已完成只读任务丢回复，不宣称验证了真实未决系统写动作的业务对账。

新包夹具最终 purged、测试 gateway disabled，私有归档与步骤证据保留，不能删 state 重放。
宿主独立中心、开发 PostgreSQL 和 dev-gateway agent 继续运行；IDEA Kura 已停止。
旧 `yofc-management-phase2-agent.service` 始终 active；系统网络写入为 0。
Parallels Ubuntu 26.04/Windows 均保持 stopped，没有增开 VM 或连接 192.168.2.125。

## 明确剩余边界

- agent 重启后的 UNKNOWN 目前保留并查询，不根据网络事务日志自动证明业务结果，也没有人工结案产品流程。
- 中心 PostgreSQL input/result 与旧 agent results 历史仍为原有存储方式。本次只加密新的 agent 执行计划与回执，
  不能宣称全系统敏感存储已完成；开放配置写入前继续实施这一前提。
- 新包的真实 Linux Kura UID/可写组合、旧 revision5 数据升级、Ubuntu26.04 新版回归仍 pending。
  本轮没有复用旧包验收。完整 systemd 文件系统沙箱受 OrbStack 全局覆盖限制，amd64/物理有线/硬断电仍 deferred。
- 配置 update、snapshot save/rollback、完整旧网络字段/NM/cloud-init 交接、可写发行权限与排空、
  nftables、系统控制和离线恢复继续按主方案推进。
- 聊天 codebase-memory MCP 仍 `Transport closed`；本地官方 CLI 能刷新/查询真实子仓库。
  不索引父工作空间，不把 indexed 解释为全文件完整解析。

代码全部保留未提交，未 reset/clean、推送或改远端。
