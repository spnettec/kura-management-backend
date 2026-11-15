# R013 V41：原布局生成与只读预览（2026-11-13）

本阶段已实现按原 ip/ip6 表与链生成规则，以及绑定原初始化凭据的只读 Core 预览。**原布局更新 plan、普通事务及界面接入仍未完成**。同一 shadow 为2069源码＋4构建夹具；88既有＋83新增−3夹具＝168个生产/测试候选路径 UNAPPLIED，较V40变化31个路径。原18仓源码未部分应用，不提交推送。

先修复原模型的两项信息损失：NAT protocol=null 不再丢失源/目的条件；零地址配非零前缀也须保留。PortForwardRule 实际目的前缀来自同一有序运行时集合，由 actual observer 传给 API/bridge，不能由 public address 猜测。net.transaction 仍为未发布的1.3.0，record 增加第10字段 runtimePortForwardDestinationPrefixes，同时保留真正旧8/9参数构造器。旧调用方不制造前缀证据；已编译的8/9参数调用方均在最终新 JAR 链接运行通过。旧凭据缺类型或前缀仍可查询原凭据，但明确不可投影，不重新导出。

新的纯 Rust renderer 保留原两族 filter/nat/mangle、固定跳转、RETURN、顺序、PF正反向、三种手动 NAT 链及自动 NAT。before 前缀只取绑定的实际来源，desired destinationPrefixes 必须显式提供且逐族/逐行校验。自动 NAT 归属不能由普通草稿改变。3组实际 Java observer→bridge→renderer 主输入共416规则逐项对照；测试 backend/executor 为明确夹具，未执行命令，不能记为真实 CM/drain/内核验收。4份固定模板来自真实 Java renderer。

firewall.kura.preview 在既有 registered UID 只读通道中接入；源 revision/hash CAS、不可变初始化 receipt/文件绑定、模型编译后原链拓扑/条件一致性通过后，才返回 draft 与 bounded wire preflight 摘要。actual Core view 保留运行时目的前缀，最终 API/bridge/firewall JAR 已解码真实 Rust 响应。两者仍标 initializationAcknowledgement，不能当实时内核读回；preview 的 applyReady/writeEnabled 均 false，事务 coordinator 缺失明确拒绝，不产生 transaction/history、owner grant、export 或 drain。

另修复 ICMP 模型编译丢失 header/fragment guard 的问题。typed icmpHeader 保留原8字节模块参数和正确 creation protocol context；IPv6 type255须按字面匹配，不能套用 IPv4 wildcard。依据为 [Linux v6.1 IPv4 primary source](https://raw.githubusercontent.com/torvalds/linux/v6.1/net/ipv4/netfilter/ip_tables.c) 和 [IPv6 primary source](https://raw.githubusercontent.com/torvalds/linux/v6.1/net/ipv6/netfilter/ip6_tables.c)，下载原文及 Git blob/SHA 见 primary/kernel-provenance.json。Java numeric/all/protocol-only ICMP 不再附加无依据的默认模块；有显式 ICMP module 却缺 type 的历史语法仍 UNSUPPORTED，其他未支持 named module 也不删条件强行放行。新 typed ICMP wire/guard 尚需后续完整新场景的内核验收，属于内部 pending。

验证与证据：

- 最终 Java original-full-java-v13 完整11项目构建，2462 tests、0失败/错误/跳过；1799输入精确对应当前 Java，9个生产 JAR 归档。旧8/9参数 ABI 与 actual-core-wire-probe-v41.json 通过。synthetic LocalAgentService 不算 registered JVM。
- 最终185 Rust 输入严格 Clippy 与 UID授权通过。Linux v41b normal/test/Clippy，以及 native59pass/10ignored、初始化12、Origin2、renderer2、authorization1均通过；完整185输入精确。normal ELF 56572ed93f13bf6b39623971864289862d4ed437d17f1303822ea3fd33bc7acd，未启动 normal agent。此前宿主成功记录按实际旧输入保留，格式化/授权测试前的结果不冒充最终185全等。
- protocol-null 原始 red、两次 Java 编译错误、Rust 编译错误、strict Clippy/嵌套测试中间问题全部保留，修复后另有终态证据。未重复旧完整宿主套件或任何 closed 物理场景。
- 首轮 Linux 容器73a8620b因宿主 ENOSPC/OrbStack daemon 失联而中断。先保留 UNKNOWN，不重启；恢复接口后查询原容器确认 exit255、无 build-result，按中断失败保留。随后独立 v41b 构建通过，不改变首轮记录。
- 仅删除本任务可重建、无绑定的 host/historical Rust incremental 缓存；保留当前中断缓存、源码、夹具、日志、选定二进制、target/runtime及个人配置。可用磁盘恢复约34GiB。此事件不等于既定 disk-full/hard-power 验收通过。
- 原中心/PG 随 OrbStack 按既有 unless-stopped 策略自动重启，现健康；gw-001恢复 running，Parallels两VM仍 stopped。明确记录运行中断，不能沿用“未重启”。18仓 Git/源码、原1986管理输入＋20 overlay、508present/89absent、凭据141＋5、共享API522及既有绑定核对通过。47个任务容器均停止。
- 本检查点11fork上游 tips 均 UNCHANGED，18仓 refs/index/remotes未变。20功能组/73 dispositions、PLC671及7nonfork/327Maven＋1Cargo关联保留；按功能/依赖/消费者继续阶段核对，不整树合并。

下一步读取 remaining-transfer-implementation-v41.md 和 next-original-update-audit-v41.json：在 SAME firewall transaction/history 域建立 current-policy lineage、实际 current receipt 与 handle/generation-bound Transfer forward/inverse；初始化 anchor 和原 ownerjournal 不变。rollback 后即使逻辑revision相同也要记录新实际 generation/handles；confirm 不再 apply。实际 Core/NM lease、drain与当前cohort合同成立后，才能接 GUI/task/protocol、普通boot/recovery、registered整条迁移和退役/再接管。不能删除 private guard 后借 private inet renderer 代替原布局。

原14组＋7API2＋R017完整保留，包括 configurationAPI1.2/1.8运行时配对、GUI部署/TLS reload/bulk/发布校验protocol crate、ordinary/modem/DHCP/DNS/Wi-Fi/VLAN/multiWAN、联合generation、168整体guarded apply和A外部CA/custody。仅3 externaldeferred：NM/cloud-init/物理；amd64；disk-full/hard-power。UNKNOWN不重发/新ID绕过/重导出。V15/V18/V24/V26/V28/V30/V31/V32/V33/V34/V35v3/V36context closed 禁止重放；V35v1/v2与V36v1失败不改绿，V36v1不live query/inverse/重发。周88%仍可开发，credits/reset未用，goal active；仅实际耗尽才必要收尾后按用户授权暂停。
