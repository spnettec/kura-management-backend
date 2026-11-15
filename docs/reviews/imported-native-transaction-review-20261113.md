# R013 V45 普通导入防火墙事务候选审查

V44 之后继续使用同一隔离工作空间，本阶段 2079 个源码输入、4 个夹具 reactor；88 个已有路径、93 个新路径减去 3 个夹具路径，共 178 个生产/测试候选路径仍未整体应用。原 18 仓脏工作树、正常运行现场和原始初始化/owner/baseline0 保留。没有提交、推送、共享 API 安装或普通 agent 部署。

本阶段实现同一 firewall/transaction.json、confirmed.json、kura-history 中的普通 native apply/confirm/rollback 与原操作 ACK/NACK 查询。pending ticket 必须由原始初始化、真实历史 parent、受保护计划文件以及当前活动记录的文件身份导出，不能从调用方 revision 或持久化 authorization 构造新的运行许可。已发出或到期的动作在反向 Core 取证前拒绝；每次可执行动作重新取得并复核真实 Core/NM/原启动 unit/PID/association/owner/原始 CM/网络 cohort/完整 raw 与 generation。

派发前保存单次 intent，记录本次完整 Owner、runtime/cohort/source 及编码 batch 的哈希。两次复核均为当前真实证明；历史授权记录不能成为新写入许可。地址族 forwarding 参与 cohort，并按实际目标规则分别验证 IPv4/IPv6 系统开关。沿用 16 KiB 事务记录、1 MiB 文件、512 KiB native 源/transfer/wire 等既有限制；提前保留 forward/inverse/NACK/终态空间和历史条目余量。

每个方向只调用一次原始 native sender。成功回复后仍需完整原操作的物理规则、外来规则、raw、句柄、回执表及 generation 匹配，才发布 ACK。完整匹配的内核负 ACK 保留原始回复十六进制、错误码、序列及原 batch 绑定；forward NACK 终态为 REJECTED 且不推进 current，inverse NACK 为 RECOVERY_REQUIRED，不能重试或另起 ID 绕过。普通超时、回复丢失或不完整结果为 UNKNOWN，不能根据旧规则仍在就推断未执行。

原查询通过既有 UID/owner/input 身份鉴别后，最多观察该原操作已有的完整内核回执。capture intent 与 settlement intent 区分尚未发布和已经尝试发布；已经尝试而缺失的 ACK/history/current 不重建。第一次尚未开始的原确认/回滚终态发布可由原查询完成；已有不可变历史和 current 齐全时，元数据恢复只补终态 marker。已归档结果不因当前可变现场损坏而重新调用后端。

确认只发布已有 forward ACK 的持久化决定、历史与 current，不再次 apply。普通回滚保留逻辑策略和版本，但由新实际 ACK 记录新句柄/generation/物理 source。未生成第二份 owner journal，也不改写原初始化、原租约、原 startup receipt 或 baseline0。

验证以 verification-summary-v45.json 与源码绑定为准。早期编译失败和两轮过粗容量估算拒绝均保留原日志/源码；没有发生内核调用。新增 5 组存储/传输测试明确使用 synthetic authority、无真实 COMMITTED/live owner，不算新注册 JVM/网关验收。最终宿主全量测试、严格 Clippy、Linux 普通 ELF/测试 ELF/精选契约分别记录；Linux 不是全量执行。Java 1799 输入与 9 个生产 JAR 字节未变，精确复用 V41 的 2462 测试，不重跑。

仍需内部实施：合法 unit/process/boot 变化后的运行证明续接、Kura-off confirmed authority、NM/foreign generation 变化后的合法 current-source renewal、导入事务自动到期巡检/原操作恢复调度、中心 task/protocol 与界面接线、terminal countdown/REJECTED 展示、完整 NAT/地址族/多 WAN/调制解调器等实际矩阵、全新完整注册生命周期、startup-child settlement、初始化退役与再接纳、178 路径整体 guarded apply。普通 native 代码通过契约不代表这些项目已完成。Scheme A 外置 CA 与密钥保管决策不变；原 14 组、7 个 API2、R017、配置 API1.2/current build1.8 配对及其它完整台账保持内部 pending。

上游继续按功能组、前置依赖及消费者影响周期复核：11 fork、7 nonfork 关联分析、327 Maven+1 Cargo 声明、20 功能组/73 处置/PLC671、508 present+89 absence 联合证据保留。本阶段最新 11 个 advertised tip 均未变化，没有整仓 merge。

外部 deferred 仍仅三类：NM/cloud-init/物理硬件；amd64；disk-full/hard-power。历史 V41 ENOSPC 与 V43 容量 guard/cache-only reclaim 保留，不算计划验收。本阶段不重放任何已闭环物理场景、不重新导出，不重启正常服务、不清理磁盘。

最终宿主 637 pass / 28 ignored；Linux 111 selected pass / 11 ignored，strict Clippy 与普通/测试 ELF 完成。普通 ELF SHA256 `4ff020634b4abf39b0e600ba8f5e943824e8b21f924c06857fed96d3b89a59d2`。周额度 94%、ordinaryAllowed=True；额度与保管现场见本组件证据。

输入口径：195 项指原生构建及关联文件清单，其中 117 个 `.rs` 文件；其余含配置、夹具、协议、保留 Java 与前端关联输入，不表示有 195 个 Rust 源文件。
