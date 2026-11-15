# 防火墙事务 v2：显式地址族、转发与基础 NAT

2026-11-07 增量。当前入口为 agent 的管理员 UDS，默认仍关闭
`AGENT_FIREWALL_WRITE`。本文件不表示 Kura API/PID 适配、中心远程界面、早期启动加载或正式发行已完成。

`firewall.prepare` 保留原 v1 参数；v2 使用下面的独立 `candidate`，不得与 v1 的
`policy`/`rules` 混用。嵌套对象拒绝未知字段，`schemaVersion` 必须为 2。

```json
{
  "candidate": {
    "schemaVersion": 2,
    "inputPolicy": "drop",
    "forwardPolicy": "drop",
    "input": [
      {"family":"ipv4","protocol":"tcp","port":18443,"inputInterface":"wan0"}
    ],
    "forward": [
      {"family":"ipv4","inputInterface":"wan0","outputInterface":"lan0",
       "destination":"198.51.100.2","protocol":"tcp","port":18080}
    ],
    "destinationNat": [
      {"family":"ipv4","inputInterface":"wan0","protocol":"tcp","port":18081,
       "destination":"192.0.2.1","translatedAddress":"198.51.100.2","translatedPort":18080}
    ],
    "sourceNat": [
      {"family":"ipv4","outputInterface":"wan0","source":"198.51.100.0/24",
       "translation":{"type":"masquerade"}}
    ]
  },
  "confirmSeconds":180
}
```

除版本号外的六个候选字段（含四个规则数组）均须显式提供。`inputPolicy` 与
`forwardPolicy` 只接受 `accept`/`drop`；每条规则的 `family` 只接受
`ipv4`/`ipv6`，不会把未提供源网段的 IPv4 规则扩成 IPv6 开放规则。
四个数组合计最多 256 条。接口名为 1–15 字节的 ASCII 字母、数字或
`_.:-`；不接受任意 nft 表达式、程序、路径、表名或链优先级。

- `input`：TCP/UDP 的非零单端口，可选 `source` 与 `inputInterface`。
- `forward`：必须给出输入和输出接口；可选源/目标 CIDR、TCP/UDP、单目标端口。
  端口必须与协议一起提供；不填协议/端口表示这两个接口之间该地址族的 IP 转发。
- `destinationNat`：按入口接口、地址族、明确的原目标主机地址和 TCP/UDP 端口匹配；
  可选源 CIDR，映射至同地址族的目标主机和非零端口。
- `sourceNat`：按出口接口、地址族、源 CIDR 匹配，可选目标 CIDR。
  `translation` 为 `{"type":"masquerade"}` 或
  `{"type":"snat","address":"192.0.2.9"}`；IPv6 同样要求同地址族地址。

CIDR 在生成规则前解析并规范化；主机地址按 `/32`、`/128` 回读语义处理。
NAT 目标拒绝未指定地址和多播地址。NAT 不会自动创建转发放行规则，
防火墙也不配置路由或改变系统转发开关。候选包含转发/NAT 的地址族必须已由系统所有者
开启 `/proc/sys/net/ipv4/ip_forward` 或 `/proc/sys/net/ipv6/conf/all/forwarding`；
prepare、apply、commit 都重新检查，缺失或关闭即拒绝。

所有规则只在固定 `inet yofc_kura` 表内，以一次批处理替换该表。
input/forward 使用 filter 优先级 0；prerouting/postrouting 为 nat，分别使用
-100/100，NAT 链策略固定 accept。输入保留 loopback、已建立/关联流量和必要的
IPv6 错误/路由器/邻居发现 ICMP 控制报文；转发保留已建立/关联流量。
没有隐含的 SSH、HTTPS 或其他管理端口。部署方必须显式安排管理入口及确认路径。
这些 hook 与 NAT 的约束遵循
[nftables 官方手册](https://netfilter.org/projects/nftables/manpage.html) 和
[官方 NAT 文档](https://wiki.nftables.org/wiki-nftables/index.php/Performing_Network_Address_Translation_(NAT))。

prepare 仍需通过 `nft -c`，apply 后比较完整链身份、类型、hook、优先级、策略和有序规则表达式。
对 nft 已能从 IP/ICMPv6 payload 推断的冗余协议条件作等价规范化；不忽略冲突的地址族条件。
未知表内对象和其他额外属性仍构成漂移。比较失败时先以唯一文件名保留内核原始输出及预期值，
再进入既有回滚流程。其他防火墙表导致拒绝，不代表对旧 iptables、启动恢复脚本或未来并发写者
已完成所有权交接；该交接仍为内部 pending。

apply/confirm/rollback/status 沿用原 UDS 方法及 token，30–300 秒确认窗口、共享系统变更锁、
confirmed revision 和日志兼容不变。旧 v1 历史日志继续按原语义恢复；v1 不带源地址时的双地址族
语义也没有被暗中更改。回滚可跨 v1/v2 恢复上一已确认版本；进程重启会补偿未确认候选。
不清空 conntrack，已建立连接的旧 NAT 映射不等同于新连接的规则效果。

目前的本地管理员 token 不能当作中心新管理路径已重新连通的证据。中心任务、
UNKNOWN 的只读结果核定、双语 React 流程、Kura 原防火墙 API/PID、旧 Java writer 退让、
管理入口新连接确认和启动前仅加载 confirmed 值仍须继续交付。不得把本次命名空间流量验收
计为这些业务完成，也不得重放历史的一次性安装、接管或恢复入口。
