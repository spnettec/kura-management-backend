# 网络后端方向核定：保留 NetworkManager

2026-11-05 用户明确选择“保留 NM 作为系统网络后端；Kura 可独立使用，agent 增加远程和故障管理能力”。这是当前方案要求，替代 2026-10-31 方案中默认迁移到 Netplan/systemd-networkd、移除 NM/Java NM 依赖的目标；不是两个同等正式网络后端的长期维护方案。

## 当前产品边界

- 未安装 agent，原 Kura 和既有 NM 网络管理仍完整可用。agent 不作为 Kura/GWT 的 mandatory 服务或启动等待项。
- agent 安装后始终运行；“开启远程管理”仅控制到中心的通道，不停止本机事务恢复。
- 原 GWT 的远程管理菜单按正式包安装情况显示。安装检测与 agent 运行、注册、证书和中心连通性分开；未安装时隐藏，后端同样拒绝无安装条件的密钥生成。
- Kura Java 后端生成私钥/CSR，不调用 agent 生成服务；身份交付和状态查询使用可选适配。身份仍独立于 Kura Web 账号、keystore和快照，私钥不回网页/日志/ConfigAdmin/快照。
- NM 是系统网络权威；Kura 与 agent 的远程/故障网络操作必须围绕同一个后端协调，而非各自生成权威文件。必须实现版本/归属/未决事务协调，不能以“都调用 NM”代替并发合同。
- Kura 停止或故障时，agent 继续提供授权的远程管理、固定系统控制和离线恢复；真实 Kura 业务恢复的核验要求不变。

## 实施约束

不删除已实现 Netplan 代码、持久状态、一次性夹具、密钥、证据或已确认配置。旧 Netplan 结果保留原日期/版本语义，不能作为 NM 适配已验的证据；不自动切换现有网关的 renderer、服务、地址、路由或所有权。原 Orb 三服务保持原包运行。

下一步重新核对原 NM 公共接口/旧字段和当前 Kura 写入路径，确定 agent 的受限 NM 后端、事务确认/回退和写入者协调。优先复用原有 Kura NM 能力，不继续增加默认 Netplan 迁移或删除 NM 的工作。NM 实际恢复、冲突、远程写和包组合均为 pending，不能套用 dummy Netplan 证据。

其他未交付项仍保留：完整组件适用面；Java XML/加密/factory/核心离线恢复及真实Kura业务核验；防火墙适配/NAT/远程流程/早启；restart/reboot闭环；一致备份恢复和正式发行。外部物理有线/amd64/硬断电等 deferred 独立保留。

## Netplan 与 NM 的关系澄清

Netplan 是声明式配置/生成层，renderer 可选 NetworkManager 或 networkd，不与 NM 天然互斥。真正要防止的是同一接口同时被 NM 和 networkd 接管，或 Netplan/cloud-init/Kura/agent 对同一配置各自持有不一致的持久来源。保留 NM 不意味着卸载 Netplan；可将目标接口交由 NM renderer，或让 Netplan 不为该接口维护竞争配置。当前不对现有系统执行全局 renderer 修改。

官方资料：[Netplan renderer](https://netplan.readthedocs.io/en/stable/netplan-yaml/)、[委托 NetworkManager 的示例](https://netplan.readthedocs.io/en/stable/examples/#how-to-use-networkmanager-as-a-renderer)。先前把NM/Netplan作为同层替代的表述不准确，现以本节为准；实际软件版本的Ubuntu集成行为仍须按目标系统核对。

## 实施策略确认

用户随后确认先按保留NM并允许Netplan按系统实际配置共存的方向实施，实际测试发现冲突时再基于现场决定取舍；不预先卸载服务、不自动修改renderer，也不为了预设冲突扩大迁移。此条不是暂停实施，其他管理体系工作继续。
