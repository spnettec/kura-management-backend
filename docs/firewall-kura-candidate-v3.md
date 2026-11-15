# Kura 防火墙结构化候选 v3

2026-11-07，5e0d8a41。接续本地 v2 事务和 2e050db1 的旧写入隔离。
本合同交付原 Kura 结构化规则与 agent 内核事务之间的数据映射；尚未注册新的 Firewall
provider、接通原 PID/GWT 事务入口、授权 Kura UID 防火墙写入或增加中心操作。
Kura 不安装 agent 仍可独立启动，默认后端及防火墙写闸门不变。

## 数据与版本

`schemaVersion:3` 必须同时提供 `ipv4` / `ipv6`，每项明确包含 `inputPolicy`、
`forwardPolicy`（accept/drop）与 `allowIcmp`（boolean）。缺失字段不自动补开放策略。
`input`、`portForward`、`nat` 三个数组也必须提供，总计最多 256 项。
未知字段、错误类型、混合地址族、注入字符、倒置范围和冲突接口条件拒绝，不能跳过坏行继续写。
v1/v2 的持久化形态及文本保持兼容，非法版本不回落为另一版本。

| 类别 | 字段与原 API 对应 |
|---|---|
| input | family、TCP/UDP、ports.first/last；source、inputInterface 或 excludedInterface、sourceMac、sourcePorts.first/last |
| portForward | family、TCP/UDP、inputInterface/outputInterface、address、inPort/outPort、masquerade；source、sourceMac、sourcePorts |
| nat | family、inputInterface/outputInterface、protocol、masquerade、automatic、ruleType；source、destination |

端口为 1–65535；源和目标范围独立。输入协议与端口转发沿原 NetProtocol 的 TCP/UDP。
NAT 支持 all/tcp/udp/icmp/icmpv6/gre/esp/ah/sctp 与整数协议号 0–255；0 沿旧含义表示 all。
Java 将原 `ipv6-icmp` 映射为候选 `icmpv6`，回读时还原；其他命名别名须先明确审核为协议号，
不直接拼接 nft 文本。MAC 校验六段十六进制并规范大小写。CIDR 按实际地址族校验。

接口保留旧 `name+` 前缀通配语义，编译为 nft 的 `name*`。单独 `+` 表示所有接口；
输入规则的 excludedInterface=`+` 是不匹配任何入口的允许规则，保留其候选/API 数据并省略
对应 accept，但仍先校验其全部字段。允许与排除接口同时填写会明确拒绝，要求消除冲突。

`automatic` 区分原 FirewallAutoNatConfig 与手工 FirewallNatConfig；ruleType 保留
GENERIC/IP_FORWARDING/PORT_FORWARDING 或原空值，自动 NAT 的 protocol/ruleType 均为空。
空协议与显式 all 分别保留：旧 Kura 空协议的 masquerade 仅匹配出口接口，source/destination
仍约束 forwarding；显式协议的 masquerade 继续匹配地址。false 不生成 masquerade。

## 编译与兼容含义

v3 仍只更新自有 `inet yofc_kura`，无全局清空、路由或 sysctl 写入。需要的地址族转发开关
必须由系统所有者预先开启。两种地址族各自有显式终止策略，不能用一项 inet policy 复制开放面。
每条 v3 家族规则还包含实际 IP 头版本条件。保留 established/related、loopback 和必要 IPv6
错误/邻居发现控制流量；allowIcmp 控制额外 ICMP 允许，不移除必要 IPv6 控制基线。

端口转发生成原语义对应的 DNAT、forwarding 和可选 masquerade。原 Kura 的 forwarding
允许选定目标主机/协议/源条件，未限制为 outPort；本映射保留这一已存在的范围，实际流量有
独立检查。不能将它宣传为新增的“仅开放映射端口”安全策略。固定 22/443 不自动加入候选。

内核回读采用 [nft 官方定义的 numeric-protocol 选项](https://netfilter.org/projects/nftables/manpage.html)
以避免宿主协议名别名影响比较。仅规范化确实由同协议 payload 隐含的依赖条件，冲突谓词仍是漂移；
表身份、链/策略、规则顺序、范围、MAC、接口排除和动作继续完整比对。首次实际失败的 raw
kernel/expected 已成为回归输入；缺失地址族条件不会被宽松归一化隐藏。

## Java 与实际验证边界

`org.eclipse.kura.management.bridge.KuraFirewallCandidate` 为纯转换器：encode 接受两份原
FirewallConfiguration 和两份明确策略；decode 恢复原规则对象、自动/手工来源和规则类型。
不发 UDS、不改 CM、不调用系统命令。bridge 导出包升为 1.1.0，uses/import 显式关联实际
Core net 2.7 / firewall 3.0 包；Guava 仅补测试依赖，生产仍由原 API 提供。

IDEA → Orb → 现有 Parallels 的完整 Kura 中，均用真实 API 对象生成 2 input、2 portForward、
4 NAT 候选并双向核对。两 VM 候选字节与 native 相同，随后由本夹具管理员经实际 root UDS
交给生产 ELF；每处 25 项真实包检查、确认、回滚和跨版本重开补偿通过。
这是 Java API 数据到真实内核执行的证据，不是 Java provider/原 PID/GWT 写入/中心远程接线的证据。
浏览器只验证原 GWT 登录兼容；本地 loopback 测试代理不是受影响远程管理路径的新连接证明。

参见 [审查](reviews/firewall-kura-semantics-20261107.md) 与
[机读台账](evidence/firewall-kura-semantics-20261107.json)。下一步继续 provider/PID/页面事务、
Kura UID 显式授权与原命令核定、NM NAT/正式所有权交接、中心流程和 confirmed-only 实际早启。
