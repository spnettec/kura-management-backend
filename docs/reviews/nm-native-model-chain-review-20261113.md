# R013 V34 原链模型编译及 jump/RETURN 契约

2026-11-13，同一 shadow workspace；2045源码+4fixture reactors，83existing+59new−3fixture=139路径UNAPPLIED。
本组修改4个Rust路径：classify.rs、writer.rs、原match_tests.rs及新增classify/model.rs。
不改Java/GWT/原运行集，不install shadow API；V32 Java证据继续适用。

## 行为

已认证 managed renderer 的规则逐条编译，保持ip/ip6、filter/nat/mangle及原链次序。
保留正反接口、尾+前缀、排除全接口的不可满足条件、地址/CIDR/族、MAC、TCP/UDP端口
范围、ICMP类型/代码、连接状态、NAT目标及链跳转/RETURN。每个canonical字段必须消费；
未知模块/参数、模糊名字、跨族、无明确传输协议的端口或NAT端口等拒绝。
仅允许同表已声明regular chain目标。分类v2把replacementExpressions及数量、实际handles/
ordinal/raw指纹、完整原source/generation/model一同绑定，不通过平铺inet链代替原链。
实际Java夹具116条逐项编译及wire encode通过，包括反向RELATED/ESTABLISHED规则。
该夹具未覆盖所有PF/IPF、CM/runtime及role/profile组合；复杂字段还有单元契约，完整映射仍pending。

底层补充jump：verdict register0、NFT_JUMP=-3、NFTA_VERDICT_CHAIN=2及NUL结尾目标。
独立核对V33已验证完整Git blob的[Linux UAPI](https://github.com/torvalds/linux/blob/v6.1/include/uapi/linux/netfilter/nf_tables.h)。
未增加goto或陌生表达式翻译；未选中规则不被编译/重发。

## 证据

先6个compiler红契约，随后2个jump/wire红契约复现缺项；中间一处z(Value)/str编译错误
单独保留，修正后未降低断言。补/0guard前宿主完整555pass/24ignored/0fail；最终/0跨族红契约复现后修正，当前native34项、legacy26pass/1ignored目标回归和严格Clippy过；Linux普通/
测试构建、严格Clippy过，native34pass/6ignored。全部旧fixture默认未开启。
普通ELF ec35e795ba375599a0a4c1d19a6a165f30b24bf5f4d8079c1b65260460633b73不含任何测试控制标记。

全新隔离compiler/jump namespace：ip/ip6/inet三类、多层jump后RETURN逐帧恢复，子链
RETURN后规则不执行。实际18个零计数器起点，2个私有IPv4/IPv6 UDP包，18组计数精确；
外来规则raw属性/JSON精确，正常agent未启动。此新场景CLOSED，不可重放。
内核proof来自/0guard前固定driver，wire及fixture函数逐字未变；/0修正独立验证，未重跑；见frozen-host-and-kernel-reuse.json。V33 packet/
policy-update、V32及更早一次性场景未重跑；无导出/配对fence/真实JVM drain/Coreowner。

离线读取V32既存raw参数，仅生成raw-compat-module-scout.json，无命令重放。发现conntrack
rev3 info168、icmp/icmp6 rev0 info8；原comment/MARK/MASQUERADE仍未选中。primary/保留
6个Linux6.1完整Git blob核验头文件（含[conntrack](https://github.com/torvalds/linux/blob/v6.1/include/uapi/linux/netfilter/xt_conntrack.h)和
[compat](https://github.com/torvalds/linux/blob/v6.1/include/uapi/linux/netfilter/nf_tables_compat.h)），作为下一步raw matcher的依据；没有声称matcher已实现。

## 保全和余项

18原仓、管理1986/20pins、上游508present+89absence、credentials141原值+5覆盖、ca9/f07、
共享API522及V32/V33/V15封存绑定精确，30任务容器均stopped；正常服务/Orb/Parallels保持。
11上游tip均UNCHANGED；20源码功能组/73dispositions、7nonfork及327Maven+1Cargo关系保留。

preparationReady/nativePlanProduced仍false。本组没有完整original Transfer factory、私有初始化
及原计划绑定回执、imported layout/owner。完整raw/native管理匹配仍需解决iptables-save无法
展示陌生native规则的情形。不能用flattened candidateRulesetSha256初始化，不能改原digest0
制造before/after相等，不能平行增设owner journal。当前源码和详尽续接见native-model-v34/
{source-checkpoint-v34-current.json,candidate-source-pins-v34-current.json,remaining-transfer-implementation-v34.md,next-factory-audit.json,final-binding.json}。

原14管理组/7API2/R017、A外部CA/custody、仅3externaldeferred保留；内部pending独立。
R017保留runtime configuration1.2/build1.8配套，非API2原因。无commit/push/reset/clean，
UNKNOWN不重发，目标active；周81%普通可用，credits/reset未动。
