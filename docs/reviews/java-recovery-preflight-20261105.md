# Java 校验接入 agent 恢复事务（2026-11-05）

整体目标继续 active。本轮把上一轮短时 Java 工具真正接入现有恢复事务，不是完整业务恢复；完整工厂/核心启动/实际业务核验仍 pending。原 opaque 诊断保留原标签，不重放原一次性验收。

## 已实现

本机运维可通过 AGENT_RECOVERY_VALIDATOR_CONFIG 明确选择独立 0600 JSON：固定 Java/JAR、明文或整包 AES 模式、严格或明确 legacy 密码策略、持久原 AES key 文件或明确既有默认 key。原密钥不是 console/agent 的身份私钥，不生成、替换或轮换密钥。工具和依赖、配置及 key 在私有 journal 绑定；公开状态不泄露该指纹。

prepare、执行停服前、确认停服后三处调用真实 Java，校对 artifact SHA、精确报告和工具配置。工具/依赖/配置/key 改变先拒绝再次启动 Java；不允许已 prepare 的事务在重启后降级到 opaque。执行前失败不触碰 unit；停服后失败使用既有补偿及 RECOVERY_REQUIRED 挡写，保留现场。全部输出/错误有界，child stderr不向请求方转发；固定参数无shell，30秒超时结束整个进程组，清空继承环境防止 JAVA_TOOL_OPTIONS/JDK_JAVA_OPTIONS/CLASSPATH/loader 注入，限制CPU/GC/heap。原通用 systemctl 等进程调用保留原环境行为。

回执增加经过严格 schema/上下界/计数一致性核对的 snapshotValidation；新模式明确 snapshot-format-and-unit-only，仍 kuraRecoveryVerified=false、fullRecoveryVerified=false。keyAuthenticated只证明整包或至少一个密码，legacy字段单列，不能推断所有密码均认证。

配置默认不变：恢复 unit allowlist 仍空，firewall/远程默认关闭。无 validator 配置时，旧显式 allowlist 下的 artifact-and-unit-only 诊断保持原行为；它没有被冒充成 Kura 恢复。Java 工具尚未装入正式 agent 包。packaging 增加配置示例和可选恢复 JVM 的512 MiB/64 tasks有界覆盖，原基础 observation unit不改、仍不依赖 Kura 启动。工具代码按只读实现运行；不能据此宣称 JVM 获得了独立的只读 OS 身份或已完成权限隔离产品。

## 验证

- 宿主及 Orb Rust各121通过/2 ignored；nft环境1和显式实际Java测试1默认ignored。宿主实际Java测试另显式执行1通过，真实25组件/6工厂/206属性快照穿过Java/Rust三次验证及事务回执，unit是mock，不算业务验收。两处严格Clippy、fmt通过。宿主packaging54通过。
- 新合同覆盖：无效Java输入不创建事务/不停unit/不泄漏child输出；工具/JAR/lib/配置变化在第二次Java启动前拒绝；停服后原key权限失效补偿并保留staging；错误SHA/越界/额外字段/过度宣称成功的Java报告不能授权停服；私有fingerprint不公开；子进程仅固定环境；崩溃窗口回执报告漂移挡写且保留已提交候选。
- IDEA原生重启上轮完整282项隔离Kura，原admin/admin中文登录、既有实际快照页通过；未重放prepare/密钥生成/身份交付。PID78161已停止，数据及快照保留。截图target/java-recovery-agent-20261105/idea-restart-snapshots.jpg。
- Orb原生当前agent、真实JVM、真实UDS和持久sleep probe unit：整包AES材料来自上轮真实Kura快照，经上一轮实际CryptoService生成；无效材料在unit动作前拒绝；真实prepare→停/启probe→校验/备份/创建snapshot_8.xml及回执通过。原snapshot_7/key未变；密钥漂移在停服前拒绝、明确rollback仅关闭该未执行prepare，PID未变；remote仍OFF。**probe是sleep，不是Kura业务恢复**。
- Orb配置MemoryMax=536870912、TasksMax=64实际返回；实际ProtectSystem/ProtectHome/PrivateTmp/NoNewPrivileges均未生效（值no），不记namespace沙箱通过。与既有Orb限制一致，不能拿请求属性替代真实沙箱证据。Parallels缺Java/Kura，相关范围deferred，本轮未启动VM。

首次Linux夹具采用瞬态probe，停止后被systemd回收，导致LoadState不再loaded；agent拒绝并保留RECOVERY_REQUIRED、backup_ready=false、无候选发布。失败夹具/原journal及日志保留，未清除或重放；新独立持久unit验证通过。没有放宽生产unit检查。启动测试期间一次并行网络观察看到系统变更锁忙，未执行网络变更。

## 源与运行状态

当前生产Rust源与最终native已测stage（agent-source-upload-final）逐文件一致；最终Linux agent SHA256 fb8446e3269ecd5ca77f539d4336a863c8232b9062ff9ef5b36aee9f43b5295d。target/java-recovery-agent-20261105/source-artifacts.json 记录源码与宿主/Linux二进制SHA。未构建/安装deb或正式发行。

宿主IDEA PID78161不存在，无新宿主daemon残留；实际Java/mock-unit临时现场两份保留。Orb编译目录/var/tmp/yofc-recovery-java-agent-20261105-6e3df0c1保留；成功系统夹具/opt/yofc-java-recovery-agent-20261105-014d5d11的agent及持久probe均停止，probe为static未启用自启动；失败/opt/yofc-java-recovery-agent-20261105-7cc7aeae停服保留RECOVERY_REQUIRED。没有启动新中心/PG。

原Orb agent、console、console@center2和phase2-agent均active；包仍agent0.1.0~package20261103-20、console0.1.0~package20261102-26。Parallels Ubuntu/Windows均stopped。18仓HEAD/branch未改变、所有旧工作树保留，无提交推送/reset/clean；未连接192.168.2.125、未清理个人或运行中runtime/旧夹具/state/key/备份/quarantine/卷。

## 必须继续的原目标

下一pending：受控核心恢复启动/完整目标计划，factory缺失删除及完整替换、当前外部网络/防火墙PID权威、禁止静默回退旧快照、精确restore ID/hash/逐PID应用回执、真实桥接和业务健康核验。原密钥停服可读本轮仅通过独立测试文件证明，不代表已为各真实部署完成凭据运维授权。

完整组件适用面、防火墙适配/NAT/远程/早启、restart/reboot闭环、一致备份恢复、身份运维/轮换/转移及正式发行继续pending。外部deferred继续单列Parallels Java/Kura、真实NM/cloud-init/物理有线/硬件、amd64、磁盘满/硬断电。NM/Netplan保持核定现状。下一轮不要用本轮probe/格式合同替代业务恢复，也不要重复执行旧一次性入口。

## 最终收尾补验：回执崩溃窗口及持续挡写

最终新增核对新Java报告与私有journal的精确一致性：若回执已落盘但journal仍VERIFYING，重启只认可同一份报告及false业务标记；报告变化则保留候选、RECOVERY_REQUIRED，不重放或自动回滚。单元合同和真实Linux/JVM/UDS/probe在全新/opt/yofc-java-recovery-agent-20261105-d21d0de5现场验证该路径。所有原件先保存在journal-before-audit.json/receipt-before-audit.json，再对本次新夹具注入崩溃窗口及报告改动。

此实际故障注入又发现：旧upload入口允许在未核定的recovery journal下继续写暂存；不能把这一轮整体记通过。已补upload入口journal必须settled的检查，并复验：host/Orb最终各121过2ignored、严格Clippy；最后Linux二进制从同一故障现场另起agent，只验证新upload拒绝，不重放prepare/execute，也不清除此前错误准入的暂存文件。journal、回执、已提交候选及旧暂存均字节一致；remote仍OFF。native-system-receipt.json保留该中间失败，native-upload-gate.json记录最终修复通过。

最终新/opt/...-d21d0de5原agent/audit-agent/修复验证agent均停止，probe停止/static；RECOVERY_REQUIRED及全部故障材料保留。两个前序系统夹具和三个独立编译stage/缓存也保留；未改旧已闭环夹具。宿主新core/IDEA运行未改（最后改动仅Rust恢复入口），原console/agent等状态仍按上述核定。此修复不扩大为完整Kura恢复验收。

项目知识库全文：`Experience/2026-11-05-Kura离线Java预检接入与恢复挡写系统验证续接-8d11e921.md`；Tasks/Session Summaries及上一交接已通过官方CLI追加最新核定。
