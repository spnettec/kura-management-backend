# R013 V39：导入策略 Core 读取契约（2026-11-13）

本阶段在同一隔离 shadow 中实现，未应用到原工作树。V38 已先封存，4,296 个绑定文件逐一哈希核对；其中性能中间版 v7 与最终版的四处差异另记 intermediate-performance-provenance.json，未修改原测试记录。V38 最终宿主589pass/28ignored仍仅覆盖V38源码，不能冒充V39完整回归。

V39 当前源码2060项＋4个fixture reactors，84existing＋74new−3fixture＝155个production/test候选路径UNAPPLIED；相比V38共7个路径变化。权威在 target/nm-role-authority-20261112/imported-core-view-v39/，包括 final-binding.json、source-checkpoint-v39-current.json、candidate-source-pins-v39-current.json、verification-summary-v39.json 和 remaining-transfer-implementation-v39.md。

实际 `firewall.kura.view` 现在先选择由原 journal COMMITTED receipt、原 operation/plan、初始化文件 inode/device/hash 和两种 marker 绑定的 imported source。与 source witness 共用同一读取验证路径，不创建第二份 owner journal，不改 baseline digest0、初始化文件或 original intent。此读取属于已确认初始化配置，不执行 nft、不声称当前实时 kernel 验证，也不授予 owner。

返回的 v4 candidate 明确限于 structuredKuraConfiguration：保留原 modelRules 的所有结构化字段；从两种地址族的实际原模型 filter base chain 读取 INPUT/FORWARD/OUTPUT 策略和原 allowIcmp；附加每族原 automaticNat（含 masquerade=false）。没有缺失字段默认值。另返回原模型的 table/chain/rule order、RETURN、additionalRules、原 operation/plan/digest0、初始化 receipt 和完整 kernel layout digest；明确 layoutSourceKind=legacy.writer.model.v1、kernelObservation=initializationAcknowledgement。它不是扁平 private inet 布局的完整替代策略。private compiler 仅验证结构化字段语法，输出丢弃、不持久化、不执行。

返回数据按既有 Java FirewallValues 的 depth24/16384values 和1MiB字节预算检查。writeEnabled=false、writeRestriction=IMPORTED_LAYOUT_UPDATE_UNAVAILABLE。Kura prepare/confirm/rollback 显式拒绝 imported 布局，private 事务的执行前重新核对 backend，防止预检与执行之间切换。未提交、替换文件、不安全权限及 private/imported 重叠不降级为 revision0，不自动修复文件或重发。

证据：

- 先保留 imported-core-red-v1：实际 Core 请求在旧实现上因 private-only guard 失败。v2 保留缺少 re-export 的编译失败；修正后 v3 初始化/读取共8pass，含原4项文件契约和4项新增请求/策略/边界契约。真实临时文件系统，kernel与Java transport明确为fake，不是真实迁移验收。
- 宿主 firewall 回归51pass/2ignored；backend1pass；严格Clippy通过。V39未重复V38全宿主589项，也未重放物理场景。
- 从通过的真实 Rust Core 请求日志提取 imported-core-wire-view.json，逐字作为 Java test resource。实际 AgentFirewall/AgentFirewallIPv6、KuraFirewallCandidate、FirewallValues 的20项测试通过，typed snapshot 两族 open2/portForward1/staticNAT2/automaticNAT2 和 confirmed properties、元数据/布局无损 Map 回环均通过。通信为mock，不能记为真实 registered JVM/DS/Coreowner 验收。Java v1 包名编译错误、v2错误假定fixture有非空additionalRules的断言均原样保留，v3按实际空附加集合和真实RETURN布局核对通过。未制造非空现场。
- 隔离 Linux ARM64 normal/test 构建、Clippy、firewall51pass/3ignored通过；首轮 wrapper 错把 Linux ignored 数设成宿主2，保留 build-result.json 的失败记录。finish-v2只执行原先未运行的初始化8pass和backend1pass，不重跑上述通过项。两轮使用同一源码179输入与同一ELF，normal SHA 6ace98f0c144309bffff8e72caae386dd4716961183476186e2d05ae2e443a30；normal不含既有fixture触发器，未启动normal agent。
- 无新物理 namespace，无旧场景重放。43个本任务容器全部停止；18仓、原1986管理源码＋20已知overlay、上游508present/89absent、凭据141＋5、共享API522、正常console/PG/Orb/Parallels状态核对。具体值见 preservation-final.json。
- 本阶段只读 ls-remote 核对11个fork tip全部UNCHANGED；7个nonfork不做直接合并，327Maven＋1Cargo关联与已接受20功能组/73dispositions继续保留。没有fetch/ref/index变更，没有commit/push。

下一步：实现 SAME firewall transaction/history 域中的 typed imported current-policy lineage 和原布局普通prepare/apply/confirm/rollback，保留不可变初始化anchor，处理规则增删/数量变化的顺序、handle、全generation、CAS、ACK、UNKNOWN原查询恢复；不能使用private inet compile替代原布局，不能在confirm再apply一次。同步current-source/Core/CM/owner readers。然后GUI/task/protocol、实际family-aware CM/runtime/role/NAT、registered drain/startup/Coreowner/普通boot/recovery、完整forward/inverse与授权退役/再接管。详见 next-imported-transaction-audit-v39.json。

原14组＋7API2＋R017、A外部CA/custody、155路径整体guarded apply等内部pending继续。仅3externaldeferred：NM/cloud-init/物理硬件；amd64；disk-full/hard-power。closed V15/V18/V24/V26/V28/V30/V31/V32/V33/V34/V35v3/V36context不重放。V35v1/v2及V36v1原失败保留；V36v1不查询现网、不inverse、不重发。UNKNOWN不新ID绕过、不重导出。周用量85%，ordinary allowed；credits与3个reset未使用，goal active，额度实际耗尽才必要收尾并按用户授权暂停。
