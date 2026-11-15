# R013 V35 完整原生参数识别及 IPv6 DNAT 修正

2026-11-13，继续同一shadow；2050源码+4fixture reactors，84existing+64new−3fixture=145生产/测试路径UNAPPLIED。
当前权威是native-raw-matcher-v35/{source-checkpoint-v35-current.json,candidate-source-pins-v35-current.json,verification-summary-v35.json,final-binding.json}。

## 实际行为

认证writer model存在时，控制器直接以完整同代raw/native规则审查，前后重查roles、stored CM、原模型/注册及kernel generation。
不再以整份iptables-save作为前提；广告能力读取失败不退回CM或兼容视图。无模型时保留原只读兼容审查。
按原表/链/顺序唯一匹配所有managed clauses，只消费有界纯谓词及已知模块参数；unknown保留为未选中，不翻译/抹除/重发。
拒绝缺失任何raw参数（包括外来规则）、重排、重复歧义、未知字段/模块版本/副作用和未消费的报文读取条件。
原留存真实116规则对应原handles；丰富实际Java渲染器覆盖MAC、地址/接口前缀/排除、端口范围、IPv4/IPv6 DNAT、IP/PF链及自动NAT真/假。

短IP网络地址加载与整宽掩码作有界等价归一化；不泛化未知表达式。默认TCP/UDP兼容模块保留真实分片/报文头/hotdrop行为，
通过仅支持两个精确info16形式的typed transportHeader表达，不作为纯协议号丢弃，也不开放任意xt字节透传。
反向重建已知tcp/udp/icmp/icmp6模块时补GETRULE不返回的creation-only协议校验上下文，原表达式/userdata字节保持原样。
依据来自完整Git blob核验的[内核传输模块](https://github.com/torvalds/linux/blob/v6.1/net/netfilter/xt_tcpudp.c)、
[兼容层](https://github.com/torvalds/linux/blob/v6.1/net/netfilter/nft_compat.c)及[MASQUERADE](https://github.com/torvalds/linux/blob/v6.1/net/netfilter/xt_MASQUERADE.c)。
这些新增header/creation-context路径的实际forward/inverse内核ABI仍须新专门场景验证，不能借用旧V31物理结论。

实际Java渲染器暴露压缩IPv6 DNAT被getHostAddress字符串相等比较丢弃，输出:2450；真实红测与首份错误夹具保留。
NatPreroutingChainRule现在保留有效压缩字面量并只加一层括号，IPv4原行为保持；新夹具成功渲染[2001:db8:2::8]:2450。

## 证据和边界

最终完整宿主571pass/25ignored，严格Clippy过；Linux普通/测试/严格Clippy过，native49pass/7ignored。
普通ELF 89d67d3adaea8c0707c857aa5a966a9d7d546c4ad0a4e1b5bc79c021dae1b44b 无新旧测试控制标记，未启动正常agent。Java11模块package全2455pass/0skip，1794输入精确；XML已归档，原日志独立计数一致。
保留9个当前Java包，修复类与所测classes逐字相同；共享API522未install/覆盖，正常运行集未重组。当前DS/live不是通过项。

新V35v3隔离namespace：IPv4/IPv6的compat工具均exit0、stderr空，stdout却报告不兼容并省略整个filter表。
完整raw-only审查仍选中全部144模型规则，2条numgen原生外来规则保持未选，前后完整kernel digest/JSON不变。
此场景CLOSED，不再执行。它使用真实Java生产者文本但合成注册，不代表实际JVM/drain/export/fence/Coreowner或整链迁移。

V35v1包装器漏看stdout而在调用Rust之前失败；V2真实源在短前缀/默认传输模块/IPv6 MASQUERADE布局上安全拒绝。
两次失败result/driver/log/完整参数均保留，不改写为pass；诊断和离线回归有独立文件。Rust中间编译/Clippy诊断也保留，未降断言。
旧V18/V15/V24/V26/V28/V30/V31/V32/V33/V34成功一次性场景未重放；UNKNOWN不重发/新id绕过/重导出。

18原仓Git/未跟踪源码（本次审查交接文档除外）、管理1986+20pins、上游508present+89absence、credentials141原值+5既知overlay、
ca9/f07、共享API522、V32/V33/V34/V15 bindings精确，35任务容器停止，正常console/PG/Orb/Parallels保持；不是全target内容hash声明。
11上游tip均UNCHANGED；20功能组/73dispositions、7nonfork及327Maven+1Cargo关系保留。

## 接续

本组关闭整份兼容视图缺失时的原生识别阻塞，preparationReady/nativePlanProduced仍false，管理145路径未应用。
继续同一Baseline/Store/template/Coordinator完整factory、私有初始化回执、imported原链布局、startup/Coreowner；原digest0与初始化digest/layout显式绑定，
不改基线制造相等、不并建owner journal、不空计划/熟悉名字adoption、不平铺inet替代RETURN/原链/外来顺序。
Transfer结果判定仍需支持新lowered header/真实dump形状；不得以自定义transportHeader JSON严格相等误判，也不得放宽未选中raw恒等。
CM/runtime/ruleType（原属性格式可能丢失类型）/角色/配置/自动NAT完整映射、普通/modem/DHCP/DNS/Wi-Fi/VLAN/multiWAN、joint generations、retirement/history/re-adoption、GUI/current paired DS/live及整批应用仍内部pending。
原14组/7API2/R017、A外部CA/custody及仅3externaldeferred保留；R017包括runtime configuration1.2/build1.8配套，非API2原因。
全文续接见remaining-transfer-implementation-v35.md；目标active，周83%普通可用，credits/reset未动，无commit/push/reset/clean。
