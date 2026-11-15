# R013 V44：普通提案准备与当前运行上下文（2026-11-13）

已接入真实授权收集路径，以及 SAME firewall 域内普通提案的 review/prepare、未派发取消、原结果查询和 metadata-only recovery。**实际内核 apply/confirm/rollback、已改变运行进程的合法续接、Kura-off authority、UI/task/protocol 和完整 registered 验收仍未完成；不能计为完整普通更新或迁移通过。**

同一shadow2077源码＋4构建夹具，88existing＋91new−3fixture＝176生产/测试候选路径UNAPPLIED。较V43变化14路径，只新增 coordinator/imported.rs、initialization/journal/producer.rs 和 tests/producer_cases.rs。18仓管理源码未部分应用，不提交推送。

新入口 firewall.kura.imported.review/prepare 经过原本已注册且精确匹配的 Kura UID，prepare 还要求独立写权限与已配置的NM写协调器。普通 owner 必须来自已COMMITTED的原接管租约，完整原三PID owner/startup receipt/marker/template均校验。只读getter没有开放任意反序列化Verified；请求JSON不能制造实时授权。

协调器在释放系统EX guard后读取真实Core三PID confirmed cohort、参与式network writer drain、当前startup association和owner，再核对完整原CM firewall参数及legacy drain。当前网络cohort绑定实际NM profile/source identity、原委派binding、角色、forwarding、automatic NAT和ownership marker；它不把manual firewall自己的pending阶段混入网络分量。真实全raw/handle/generation内核来源必须匹配当前已确认来源。重新取得EX guard后再核对原租约、注册、网络cohort及实际内核，避免Core反向读取和EX锁互等。

补上了运行上下文校验：复用既有startup terminal/current检查，读取选中的实际文件、stop policy、当前unit/PID和fresh Core startup witness，记录runtime观察摘要。原租约、启动receipt与baseline0不重写。桥接重连必须提供当前实际同一运行上下文，不能只凭相同配置续租。真实进程/unit变化时旧startup receipt会拒绝；合法restart/boot/Kura-off续接仍为internalpending。储存的runtime/owner/hash只是历史证据，后续apply必须重新收集真实Verified，不能把文件中metadata当新grant。

prepare使用新schema2的同一firewall/transaction.json及原kura-history命名域。原schema1仍为只读历史证据。UID/request/owner hash/input hash、显式operation UUID、cohort digest、deadline、原初始化receipt、完整原生UpdatePlan、plan文件及各级目录identity绑定在一起。计划create-only发布，记录16KiB、artifact1MiB、native512KiB、history depth/inventory headroom与完整终态ACK引用预算均在发布前检查，且重查授权deadline。当前/历史/孤立artifact内的同一次操作不能重提；未决提案挡住所有新ID。没有新owner journal，也不切换到private inet编译器。

未派发取消不调用kernel：先记录原ROLLING_BACK(no forward intent)，只发布一次immutable ROLLED_BACK no-effect archive，再补原active final marker。丢回复只原查询；仅当原archive已存在且完全一致时可恢复final marker。缺archive/active不重建，不把过期当自动取消，不重发。取消和原结果查询不依赖仍开启的NM/写grant。原UID/owner/input/token均校验；归档结果独立于当前损坏/不可用指针。替换plan inode、降级为无grant schema1、伪造输入或失去active文件均拒绝新动作。

既有firewall.kura接口和新入口均保留注册UID边界；新namespace apply/confirm/rollback尚未启用，输出dispatchEnabled/writeEnabled/ownerGrant均false，UI不把提案当已生效策略。持久化prepare已实现，整个普通更新producer仍未完成。

验证：最终193Rust输入宿主632 pass/28ignored、注册UID/grant单测1及严格Clippy；Linuxnormal/test/严格Clippy、native63pass11ignored、initialization37、Origin2、renderer2、UID授权1、旧private history1。7项新增storage合约明确使用synthetic Verified且fixture没有COMMITTED owner，不能替代真实registered migration/owner验收；生产协调器重检拒绝storage-only proof，真实路径须当前startup/unit/Core/NM证据。实际kernel dispatch或新physical场景均未运行。最终normal ELF ba767c79953a02981221932a9c698e29b9fd83a72948b26d191956392553ed71。Java1799输入与V41精确相同，复用2462tests/9JAR，不重放已闭环Java/GWT/浏览器验收。

初版编译因Owner/Binding私有字段失败，修为只读内部getter；runtime spec的Value误用?导致编译失败也完整保留源码/log。没有放宽生产权限/匹配/旧凭据。较早7项storage及37项初始化各有自身source pins；最终632全量与Linux当前193输入精确。

最终核对18仓Git/源码、1986管理输入＋20overlay、508present/89absent、凭据141＋5、API522与全部旧绑定。52任务容器全停，正常console/PG健康、StartedAt与V41恢复后相同；Orb gw-001running、Parallels两VMstopped。本轮无正常服务重启/磁盘清理，V41 ENOSPC及V43容量guard历史均保留，不计diskfull/hardpower验收。

11fork上游tip本阶段全UNCHANGED；20功能组/73dispositions/PLC671与7nonfork/327Maven＋1Cargo关联保留，继续按阶段/逻辑/前置依赖与消费者影响审查，不整批merge。原14组＋7API2＋R017、配置API1.2/1.8运行配对、完整ordinary/modem/DHCP/DNS/Wi-Fi/VLAN/multiWAN/joint generation、UIdeploy/TLSreload/bulk/protocol发布校验及A外部CA/custody均保留。仅3externaldeferred：NM/cloud-init/物理、amd64、disk-full/hard-power；其余全部internalpending。

下步在SAME记录上接显式单次apply/inverse intent、真实raw批次/ACK/NACK、WAIT_CONFIRM/confirm/current pointer/history及原查询恢复，不复用会再次apply的private commit。每次effect需要fresh actual owner/runtime/network/native proof；继续补合法runtime/Kura-off/current-source cohort续接，再UI/task/protocol/新的完整registered一次流程/boot-recovery/退役再接管和176整体guardedapply。

权威 target/nm-role-authority-20261112/imported-ordinary-producer-v44/final-binding.json、source/candidate pins、verification-summary-v44.json、next-ordinary-dispatch-audit-v44.json与remaining-transfer-implementation-v44.md。周93%ordinaryallowed，credits/reset未用，goalactive；实际耗尽后必要收尾并按用户授权暂停。
