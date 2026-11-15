# R013 v33 原生 wire 编码契约修正

2026-11-13，继续同一 `target/nm-role-authority-20261112/workspace`。
仅修改原候选 writer.rs 并增加一个 cfg(test) 文件；没有应用任何管理候选到原仓。
当前2044源码+4fixture reactors，83 existing+58 new−3fixture=138路径UNAPPLIED。
读取 `native-wire-v33/{verification-summary-v33.json,source-checkpoint-v33-current.json,
candidate-source-pins-v33-current.json,kernel-v1-adjudication.json}`；不重放任何执行脚本。

## 修正内容

6个独立红契约复现：MAC payload不支持、接口前缀被编码成带星号的精确字符串、
ct state的DREG/KEY值颠倒、base chain接收hook/type/prio却未编码、IP版本四位字段/
inet地址不支持，以及裸星号/模糊名称未拒绝。实际修正保留IPv4/IPv6协议约束，
MAC以六字节编码并增加Ethernet iiftype保护，版本使用0xf0掩码和位移比较，
ct state读取数据寄存器1/状态key0；set字段有界且拒绝未知字段。接口精确匹配带
NUL，前缀仅比较去掉星号的stem，限定现有Kura可表达的ASCII名称与操作符。

base chain完整序列化type/hook/有符号priority/policy，使用CREATE标志；regular
chain不捏造hook/policy，部分base描述拒绝。既有链更新仍走相同完整描述，不只
验证新建链。未扩大陌生原生表达式的反序列化/重发范围，原raw inverse路径不变。

参照来自独立nftables1.0.9新命名空间和完整Git blob核验的
[Linux UAPI](https://github.com/torvalds/linux/blob/v6.1/include/uapi/linux/netfilter/nf_tables.h)。
[Netfilter接口名称合同](https://netfilter.org/projects/nftables/manpage.html)规定尾星号
表示前缀、不接受裸星号；本组不会把模糊名称默认为全接口。原参照compile image
缺少nft的失败保留，未执行Netfilter；随后用已验证本地包建立独立参照，不改主机。

## 验证范围

红测0/6，修正后writer8项过；宿主主回归545 passed/22 ignored、严格Clippy过。
之后仅追加一个ignored的链更新专用测试，旧测试文件完整prefix及生产源码精确；
没有再重跑旧整套。当前宿主测试编译/严格Clippy过，Linux当前普通/测试构建、
严格Clippy过，native默认23 passed/5 ignored。旧一次性fixture默认未开启。
三个Linux构建普通ELF同为e83377556c973f429770078006d3836ce948e63701b51fa50da174e7c49de47d，
全部新旧fixture环境标记均不在普通ELF。Java/GWT及其源未变，保留v32证据。

新的wire-only内核场景从真实零计数器开始，只发送1个私有回环UDP包。37组
谓词计数与实际字节数36均正确，验证前缀正反、ct state/非state、版本/地址、
复合UDP端口和MAC排除非Ethernet；12链hook/type/prio/policy及后加DROP链正确。
没有声称物理Ethernet正例或真实网关验收。此场景已闭环，不再运行。

初始Python包装器将整个NFgen消息体要求相等而报失败，Rust实际内核测试已过。
原失败result/log保持原样：208字节中仅offset3由2变4，全部204字节规则属性和
JSON逐字相同。核对完整主线源码
[nf_tables_fill_rule_info](https://github.com/torvalds/linux/blob/v6.1/net/netfilter/nf_tables_api.c)
和nfnetlink helper，res_id携带全局base sequence低16位；两次已ACK写入恰使2→4。
生产raw reader本就分离该字段与GETGEN，并未为绿测改normalizer。离线核定只
读取现存文件，没有重跑场景或重写失败证据，见kernel-v1-adjudication.json。

另一个独立namespace验证原Transfer::batch的完整现有链描述：ip/ip6/inet三类
现有链accept→drop→accept，原handle及其余每个字段逐项保留。外来规则完整
属性与JSON精确，未重跑上述包流量场景；case结果passed。两场景均无导出、
paired fence、真实JVM/drain、Core owner或正常agent启动，不是整链迁移验收。

## 保全和余项

18原仓Git/未跟踪源码（本次两个文档除外）、管理1986输入/20pins、上游508
present+89absence、credentials141原值+5已知覆盖、ca9/f07和共享API522精确。
v32及上游v15封存绑定保持，27个本任务容器均stopped，正常console/PG与Orb/
Parallels状态保持。不是全119GiB target内容hash声明。未commit/push/reset/clean，
未改系统后端，UNKNOWN不重发，不重放旧验收。周79%普通额度可用，credits/reset未动。

本组只关闭有界wire编码前提，preparationReady/nativePlanProduced仍false。
继续同一Baseline/Store/Coordinator的完整factory、私有初始化及committed receipt、
既有链布局和owner；原始digest0与初始化digest/layout必须分开绑定，不改原基线
来制造相等。完整raw/native管理规则匹配仍需覆盖原iptables-save无法展示的陌生
规则，不能选择/翻译/重发未选中内容；CM/runtime、ruleType、automatic NAT/role/
profile的精确映射也仍pending。旧链hook/order/RETURN/default policy语义必须保留。

原14组、7API2、R017 GUI部署/TLS reload/bulk/校验发布crate、A外部CA/custody均
保留，只有3 external deferred。上游20源码组已接受671；保留运行集configuration1.2
与构建1.8配套留R017，非API2原因、不宣称部署通过。目标active。
