# R013 V40：确认策略与真实运行时 NAT 类型修复（2026-11-13）

本阶段修复进入普通更新前发现的原始语义缺口；**尚未实现原布局更新 plan 或普通事务**。目录名 imported-native-plan-v40 不表示 plan 已实现。全文续接 remaining-transfer-implementation-v40.md 与 next-original-update-audit-v40.json。

同一shadow2061source＋4fixture reactors，85existing＋75new−3fixture＝157个production/test候选路径UNAPPLIED，较V39变化14个路径。新增既有文件改动为NATRule.java，新增文件为OriginalLayoutTemplateTest.java。原18仓、运行现场及前阶段封存不改。

V39 Core 投影取了 Java renderer 固定默认 base policy，但原 factory 迁移保留实际原内核策略。先运行 imported-policy-red-v1，确实复现 INPUT/OUTPUT 与模型默认值不同时返回错误策略；V40从不可变初始化 ACK snapshot 的实际 ip/ip6 filter basechain读取策略，逐项约束family/table/name/type/hook/prio与accept/drop。模型 layout 仍明确标记 legacy.writer.model.v1，策略另标 initialization.native.snapshot，不假称实时内核观察。混合两族策略契约通过，原journal/intent/digest0不改。

原NAT属性语法不保存ruleType，LegacyFirewallSource曾把运行时NAT也统一标成IP_FORWARDING，可能与实际PORT_FORWARDING/GENERIC链不同。现在实际AbstractLinuxFirewall observer按同一manual LinkedHashSet顺序导出不可变 runtimeNatRuleTypes；bridge仅在runtime modelRules补入真实类型，stored CM另保留。Rust Origin对可选字段逐族检查数量、合法类型与modelRules一致；旧None字段序列化省略，旧原模型参数不改。旧manual NAT凭据缺类型证据时仍能查询原始receipt/layout，但projectable=false、candidate=null、readRestriction=RUNTIME_NAT_TYPE_UNAVAILABLE，不自动重导出、替换凭据或推断IP_FORWARDING。

NATRule.equals/hashCode现在纳入实际链类型。IP_FORWARDING、PORT_FORWARDING、GENERIC相同其他字段的规则不会在集合中错误合并；null与GENERIC按renderer实际含义视为等价。原错误可能在导出前已丢弃另一种类型，此修复不伪造已丢失的历史规则。新增真实observer/完整集合测试核对三种链和不可变类型列表，bridge测试同时证明runtime类型与stored CM分开、错计数拒绝。

net.transaction导出1.3.0；使用新字段/构造器的linux.net与management.bridge导入[1.3,2.0)。新增record字段保留旧八参数构造器，旧构造器不制造运行时类型证据。已用V35旧API JAR编译真实八参数调用方，并在最终新API JAR运行通过。此项为shadow ABI/消费者证据，不是实际DS/网关验收；共享API522未安装替换。

验证：

- 最终Rust源码179输入：imported-types-final-v12初始化/读取10pass；严格Clippy v11过。Linux最终v40b normal/test/Clippy过，native57pass/10ignored、初始化10pass、Origin2pass，完整输入哈希对应当前源码。normal ELF 5d6ed31f8aa2f11572c9d3f0ef15fd361e4bd8d6ecc3411869e84f203c57140e，不含既有fixture触发器，未启动normal agent。
- 宿主57项native和2项Origin在最终一处测试夹具Clippy格式修复前通过；其完整旧输入保留，按intermediate-tests-provenance.json区分，不冒充最终179输入完全相同。Linux最终补齐当前源码对应验证。未重跑V38完整宿主589项或旧物理场景。
- 最终Java runtime-full-java-final-v13完整11项目reactor包构建成功，2459tests/0fail/0error/0skip，1797源码输入精确，最终9个生产JAR归档 java-packaged-final、java-artifact-binding-final-v40.json。此前v8通过后进一步补齐null/GENERIC等价，保留前版本与最终版本，未覆盖旧证据。
- 当前Rust实际Core响应经最终归档API/bridge/firewall JAR解码，IPv4/IPv6均保留IP_FORWARDING＋PORT_FORWARDING、自动NAT含false和confirmed properties，代码来源确认最终归档JAR。LocalAgentService明确synthetic，不能记为registered JVM/DS/Coreowner实测。actual-core-wire-probe-final-v40.json同时证明旧已编译调用方可链接。
- 新纯Java测试生成4份真实IptablesConfig/IPv6空模板（ICMP on/off），executor未使用；最终归档JAR再次渲染与4模板逐字相等。这是下步原布局renderer的primary输入，不是renderer/plan/物理迁移验收。
- red策略失败、宿主Clippy v10测试夹具collapsible-if失败、对应第一轮Linux严格检查失败全部保留。修正后最终源码独立验证；首轮normal与最终normal ELF相同，但不将首轮测试ELF冒充最终ELF。
- 45个本任务容器停止；18仓、原1986管理输入＋20已知overlay、508present/89absent、凭据141＋5、API522、正常console/PG/Orb/Parallels状态保留。11fork上游tip只读核对全UNCHANGED；7nonfork跳过直接合并但保留327Maven＋1Cargo关联，20功能组/73dispositions和PLC671不变。未fetch/ref/index变更、未commit/push。

下一步仍为真实原布局普通更新。已有Transfer支持original-empty插入anchor、规则数量变化和全generation forward/inverse，可复用，但不能以draft/source名字直接当写权限。先解决两项primary source发现：PortForwardRule实际addressMask可能是24/64，单独public address不能证明目的前缀，需要真实runtime前缀证据/保留合同；NATRule.getNatPostroutingChainRule的protocol-null分支会丢source/destination条件，新的请求不能静默放宽。Java与Rust需一起明确语义并与真实renderer对照。

随后实现SAME firewall transaction/history域的current-policy lineage和原布局prepare/apply/confirm/rollback，初始化anchor与原ownerjournal不可变。rollback后logical revision不一定变，但generation/handle变化必须形成新的实际current source receipt；confirm不能再apply一次。需要真实Core/NM owner lease与cohort更新，不能仅凭registered UID/write flag。再接GUI/task/protocol、实际CM/runtime/两族roles/NAT、registered drain/startup/Coreowner/普通boot/recovery与完整一次forward/inverse、退役/再接管。

原14组＋7API2＋R017、A外部CA/custody及157整体guarded apply继续内部pending；configurationAPI1.2/1.8配对R017与此net.transaction1.3分开。仅3externaldeferred：NM/cloud-init/物理；amd64；disk-full/hard-power。closed V15/V18/V24/V26/V28/V30/V31/V32/V33/V34/V35v3/V36context不重放。V35v1/v2与V36v1原失败不改绿；V36v1不现网query/inverse/重发。UNKNOWN不重发/新ID绕过/重导出。周86% ordinary allowed，credits/reset未用，goalactive；实际额度耗尽才必要收尾并按用户授权暂停。
