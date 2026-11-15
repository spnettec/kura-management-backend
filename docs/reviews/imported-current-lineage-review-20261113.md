# R013 V42：原布局原生更新计划与当前来源凭据（2026-11-13）

已实现从实际 Transfer ACK 派生的当前物理来源凭据，以及完整原 ip/ip6 布局的普通更新计划构造；新增一次隔离内核 forward/inverse 场景通过并 CLOSED。**SAME firewall 普通事务 coordinator、持久化 current-policy lineage、真实 owner/cohort 与 GUI 仍未完成**。目录名 imported-current-lineage-v42 不代表持久化 lineage 已完成。

同一 shadow2072源码＋4构建夹具，88既有＋86新增−3夹具＝171生产/测试候选路径 UNAPPLIED，较V41变化7个路径。原18仓源码未部分应用，不提交推送。新增源码只有 native/lineage.rs 及两个测试文件；其余为 native 模块声明和既有初始化预览/测试接入。

CurrentSource 先调用原 Transfer 的完整 acknowledge 校验，再以同 generation 的完整原生参数、实际 handles、原 receipt/tag/顺序派生选择集。随后按已确认 policy 与显式目的前缀编译原布局，逐条检查实际 raw predicates/counter conditions 和6个实际 filter base policy。来源摘要绑定初始化 receipt、逻辑 metadata、原 Transfer identity/方向、实际原生快照与选择集。相同链名、只改 JSON policy、缺 raw 参数、错误 generation 或未确认原结果均不能成为来源。该类型是来源凭据，不是 owner grant。

UpdatePlan 复用既有 Transfer，保留原链/规则位置、RETURN、外来规则/对象及原始 inverse。显式 UUID 不可为 nil，不可复用初始化 operation；实际 filter policy 只在改变时生成修改。源/目标规则数量可以变化，目的前缀与自动 NAT 归属仍按 V41 严格合同绑定。每条替换保留对应来源的 counter 数量/guard；新增行沿用链中末个受管行的 counter profile。内核 batch 绑定原 generation，不能把已有 update 的 ACK 当成再次 forward 的 source。

firewall.kura.preview 增加可选 operationId，旧 renderer-only 请求保持兼容。给定显式 ID 时，返回 nativePlan 摘要、source/transfer/batch/draft hashes、数量与 wire preflight；不生成随机新 ID，不 dispatch 或创建 journal。仍标 initializationAcknowledgement，applyReady/writeEnabled=false，要求 SAME_FIREWALL_TRANSACTION_AND_CURRENT_CORE_NM_LEASE。基于原 ACK 的 review 不能假称实时内核准备或可执行授权。

实际内核场景 kernel-original-update-v42：新建 network=none、仅 NET_ADMIN、1CPU/512MiB 的独立容器，使用最终188输入 test ELF。加载 V41 实际 Java observer→bridge case0 输出，两族各加入一条原生 numgen 外来规则；使用生产 raw observer、factory、writer 和 acknowledge。原 Java model 与内核规则实际匹配，首先完成新的 native initialization primitive，再生成并执行原布局 update 与 inverse。这里的注册和逻辑初始化 anchor 明确合成，没有真实 Store 初始化/owner lease，不能计为完整迁移或普通事务验收。

- 受管168→126→168，外来2条在全部4份快照中的完整 counter-normalized raw fingerprint/payload不变。逆向后的规则 metadata 与每链顺序恢复，generation 9→10→11→12。
- update 同时清空动态 input、关闭 IPv6 allowIcmp、修改 IPv4 INPUT policy、PF目标端口2450→2451及目的前缀24→32。原 policy/source重新绑定，无 private inet 替代或 table flush。
- 实际 ACK 可派生逻辑revision2的下一计划；inverse 后逻辑revision1但新的 generation/handles/source receipt，原 forward 被拒绝。following/after-rollback 只编译 review，未再次 dispatch。
- 新 typed ICMP module/context 在真实初始原布局 batch 中被内核接受，并由完整原生读回匹配；不是新报文/分片行为矩阵的验收。旧不支持默认/未知 module 仍明确拒绝。
- 三次 raw dispatch 各有先写 intent、单次 ACK 和保存快照。原景 CLOSED，不再启动/重放；saved-kernel-offline-audit-v42.json 只读取现存证据，独立核对源 canonicalization、168/126/2、顺序、generation及合成 anchor稳定，无live query。

验证：宿主 current-handover-v4 为154pass/12ignored，来源187输入；随后仅增加 Linux cfg(test) fixture，不能把该记录冒充最终188全输入精确。最终宿主严格 Clippy、显式ID预览1项和 Linux全部188输入精确；Linux normal/test/严格Clippy、native63pass/11ignored、初始化13、Origin2、renderer2、UID授权1通过。最终 normal ELF622ff8c218a7bdd5cdf60c4f206e133b8f90a8f1e85a1c657ada1cfe39864b81；normal agent未启动，V42 fixture marker不进入normal。Java1799输入未改，精确复用V41的2462tests/9生产JAR，不重复构建。

最初两个宿主编译失败均为测试辅助函数可见性/临时借用等问题；v3仅失败于测试 dump helper 错删 userdata 而非 creation compat。修正 helper后154项通过，原错误源码/log仍保留，未降低生产matcher。离线审查初版把 canonical Source 与原始 nft envelope直接逐字比较而失败；初版脚本保留，修正为排除metainfo/保持每链规则顺序的 canonical 比较后通过，未修改内核证据或重放场景。

preservation核对原18仓 Git/源码、1986管理输入＋20 overlay、508present/89absent、凭据141＋5、共享API522与既有绑定。49任务容器全部停止，正常console/PG的StartedAt与V41恢复后相同且healthy；Orb gw-001running，Parallels两VMstopped。本阶段无运行重启/磁盘清理；V41 ENOSPC/自动恢复历史保留，不改成“始终未中断”。11fork上游tip本阶段仍全UNCHANGED；20功能组/73 dispositions、PLC671与7nonfork/327Maven＋1Cargo关联保留。

下一步必须在 SAME firewall/transaction.json 与既有 history 域接入 typed UpdatePlan、持久化 current receipt 与 prepare/apply/confirm/rollback/original-query。当前 CurrentSource 的逻辑revision/anchor来自可信内部调用方；仍需由实际已提交事务/历史推导并完整验证，不能把反序列化JSON或调用方revision当授权。初始化文件/receipt/原ownerjournal/baseline0不改，不建第二ownerjournal。confirm不能再apply，rollback后必须持久化新实际source；UNKNOWN仅原查询，不换ID/重导出/重发。实际Core/NM ownerlease、drain与随当前firewall revision合法推进的cohort之后，再接GUI/task/protocol、registered完整迁移/boot/recovery/退役再接管与171整体guardedapply。

原14组＋7API2＋R017、configurationAPI1.2/1.8运行时配对、GUIdeploy/TLSreload/bulk/已发布校验protocolcrate、ordinary/modem/DHCP/DNS/Wi-Fi/VLAN/multiWAN、联合generation与A外部CA/custody全部保留。仅3externaldeferred：NM/cloud-init/物理；amd64；disk-full/hard-power。V42新CLOSED与V15/V18/V24/V26/V28/V30/V31/V32/V33/V34/V35v3/V36context均不重放；V35v1/v2及V36v1失败保留，V36v1不live query/inverse/重发。周89%ordinaryallowed，credits/reset未用，goalactive；仅实际耗尽必要收尾后按用户授权暂停。
