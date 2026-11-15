# 防火墙旧写入方隔离合同 v1

2026-11-07，2e050db1。接续本地 agent 防火墙 v2 事务；本合同只交付旧写入方的抑制，
完整 Kura API/PID 适配、正式所有权交接、中心流程和早启继续 pending。

## 归属判断

`org.eclipse.kura.linux.net.iptables.FirewallOwnership` 是独立的本机判断：

- 未配置时 `kura.firewall.provider=legacy`，无归属标记则保持原 Kura 行为。
- `provider=agent` 只禁止 Java 旧防火墙写入，不授予 agent 写权限，也不探测 agent 进程或 UDS。
  未知 provider 拒绝，不回落至 legacy。
- 稳定标记 `/etc/kura/firewall-owner-agent` 位于版本化 Core 目录之外；
  `${kura.home}/framework/firewall-owner-agent` 是隔离/迁移目录可使用的额外抑制标记。
  任一标记存在，即使设置 legacy、agent 离线或标记为悬空符号链接，也不恢复旧写入。
  Java 读取标记遇到非“不存在”的 I/O 错误时拒绝。
- 四个安装/卸载/启动脚本检查稳定标记或对应 Kura home 标记；Java 的 provider 属性
  不能替代脚本标记。尚无自动创建/删除标记、正式安装 profile、迁移或退役入口。

标记本身不是授权凭据。agent 的默认关闭、独立授权、事务锁、确认、回滚及归属检查继续生效。
本轮没有在任何宿主创建稳定系统标记，没有更改原安装或启用现网防火墙。
正式交接仍需先核验并停止其他写入方、完成显式授权和回滚预案，不能把本文当迁移操作脚本。

## 已抑制入口

`IptablesConfig` 在命令/备份写入之前拒绝清链、默认策略、应用、保存与文件恢复。
IPv4/IPv6 `AbstractLinuxFirewall` 在缓冲规则或 forwarding sysctl 改动之前拒绝初始化、
规则增删、启停、ICMP 和附加威胁规则；构造器在 agent 归属时不种植默认端口或恢复旧文件。

旧 FirewallConfigurationService 的 activate/CM updated/配置读取与三个写接口、
附加威胁规则入口明确拒绝，避免将陈旧 legacy 视图冒称 agent 当前状态。
旧 API 的 `getConfiguration()` 保持 KuraException 包装合同，原因为归属拒绝。

NM `FirewallNatConfigWriter` 在 agent 归属时不构造旧 LinuxFirewall。无实际 NAT 对的
普通网络配置保持无操作；同接口对也不形成 NAT。真实不同接口 NAT 请求明确拒绝，
不能静默丢失。尚未将原 NAT 语义转发给 agent，不主张真实 NM D-Bus 或物理网络验收。

`firewall.init` / `firewall-only.init` 提前退出；网络安装和卸载脚本跳过旧 firewall
服务切换、iptables 备份/清空/重放及 forwarding 恢复。无标记时原顺序和行为保留。
这些脚本合同通过重定位私有路径和提取原函数体验证，不执行旧安装/卸载一次性入口。

## 验证与后续

72 项 Java 测试与三份 bundle 构建、宿主及两种 Linux 的脚本合同通过。
新完整隔离 Kura 从 IDEA UI 启动，再在 OrbStack、现有 Parallels 验证真实执行器可用，
36 项旧入口拒绝、执行器调用为零、agent 已确认规则及外部哨兵表未变，原 GWT 登录正常。
三个 product bundle 由观察器安装并在 start level 100 解析，未启动 NM DS/D-Bus；
因此该证据不等于新防火墙 provider 或 NM 网络管理端到端验证。

后续必须交付完整旧 Firewall API/PID/页面语义映射，正式 ownership adoption/retirement，nft
其他管理者策略、中心加密持久任务及 UNKNOWN 原命令只读核定、双语远程流程、新受影响管理
连接证明，以及只加载 confirmed 的实际早启/正式部署。

详见 [本轮审查](reviews/firewall-writer-ownership-20261107.md) 与
[机读证据](evidence/firewall-writer-ownership-20261107.json)。
