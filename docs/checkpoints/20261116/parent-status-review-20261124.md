# 2026-11-24 冻结父项复核

沿用 11 月 22 日暂停时的 12 项合同。复核以当时的完整冻结台账、可读检查点、实装版本和新增终验证据为依据。按子场景确认已有工作，整个父项满足原合同后才记闭环。

| # | 父项 | 已验收的范围 | 仍待的证据或实现 |
| --- | --- | --- | --- |
| 1 | 完整密钥生命周期 | 多代备份及恢复、原 READY 续配与归档、原 Main provider 验签、受控快照恢复及写释放中断均有原号证据。跨格式发布冷读已恢复。 | 逐个 provider 的盘点、原子重加密与跨包连续性，其他 launcher、完整 GUI 管理和旧弱钥安全换代。 |
| 2 | CA custody | 方案 A 外部签发。本轮生产 TLS 激活已受 quorum 约束。独立 PostgreSQL 与真实 HTTPS/mTLS 测试覆盖重复管理员、错误租户／版本、禁用审批人、两人通过和原号回执。 | 跨人实体保管流程、负责保管人员的名单管理、批准策略管理及批量操作验收。 |
| 3 | 最小权限与完整发行 | 既有 Core Main、broker 和部分六件套维护有单独的 ARM64 证据。本轮 3 台 OrbStack 与 1 台 Parallels 用同版 Core+agent 首装和入网成功。独立 ARM64 实测当前完整六组件首次 APT 安装，agent 原号分段升级后合法配对，Wires 解包后原配置续接，Main 前后实载 8 个 bundle。另在当前配对下实测 Networking remove、purge、half-configured 续配，原配对失配时拒绝 agent 启动，续配后原号 READY。已补当前受审 drop-in 的后继入口，现行六组件用相同载荷的后继 Debian 修订实际联合换版，原配对和数据连续。 | 当前六组件中余下的升级场景、各 provider 和 launcher 的完整升级、移除、purge、中断及平台矩阵；核验原生隔离的 `NoNewPrivileges` 证据。已绑定网关联合维护已有实现和单测，实际装包及证书和加密业务连续性纳入合并全链。 |
| 4 | 原 14 组及 API | 原 14 组、API2、1.2/1.8/1.9 演进记录可追溯。已有局部构建、消费者及 provider 证据。 | 七个 API2 的正式版本配对及全消费者核验，原 14 组逐项的实际发布证据。 |
| 5 | 完整实际接管链 | 已核对正式 Main 的恢复输入、原 CM 回执、固定启停、受保护启动与写释放。 | 在同一原现场沿原授权覆盖原 forward／confirm／inverse、source observation、停机/到期全链。 |
| 6 | 网络矩阵及历史 | 保留 NM；已有实际 DHCP、部分 IPv4/IPv6 防火墙、重启与原号读回证据。 | 完整网络组合、回退、退役及原历史继续核验。真实 NM/cloud-init/物理硬件交接按原延期项。 |
| 7 | 中心与部署 GUI | 隔离部署验证正式中心登录、网关列表、TLS 设置及四网关状态。 | 主机、实例、容器端到端流程，容量与归档、首次 TLS UNKNOWN、半凭据和历史 CSR 的全部 GUI 错误恢复。 |
| 8 | 数据库与协议 | 合成旧 schema 的真实 PG 备份/恢复/rekey，正式中心新库的迁移 0028。 | 客户旧加密库的真实恢复、完整 GUI 数据库流程、协议 0 在线升级及发布互操作与校验和。 |
| 9 | R017 与文档/技能 | 方案 A 和父库交接已更新；本轮 agent-first 安装成功，并单独保留授权顺序与配对记录。当前六组件修复 Networking 有效 drop-in 校验，实装 agent 在原号完成本地配对且保持 OFF，已验收匹配 Networking drop-in 经移除和中断续配后的原号 OFF 与 READY。已实际验证当前配对下 Core 与 Networking 后继联合维护仍保留该状态。 | R017 的产品化安装、import/CSR 顺序和所有 UI 操作路径，全套 TLS reload、证书批量流及平台体验。 |
| 10 | G021/G022 | G021 27 个描述符，5 个状态周期。G022 的三个非 Java POM、.NET 三协议生成、11 个 nupkg 和 Go 31 条历史诊断已核对。 | 原检查点仍保留 Go 默认 vet 债务、KNX 数据漂移 review 与多语言生成规则。 |
| 11 | 上游逻辑同步 | 11 fork／7 nonfork 的策略和历史审查游标可追溯，PLC4X 局部修复有独立证据。 | 按冻结游标完成剩余逻辑组、shared SLMP/.NET 范围及跨仓来源审查。 |
| 12 | 正式源码／发行门禁 | 484c4c843c 的全量 ARM64 Equinox，后续构建记录及本轮 ARM64 安装和当前 Console 测试分别可核对。当前六组件选定 8 个实际加载的 JAR 与正式 deb 内容逐个验同，本轮 Networking 中断续配及当前两包后继联合升级后保持相同证据。 | 按当前联合发行版本完成原 guarded apply、正式发布标签与制品校验，并验收整套发行合同。 |

**冻结闭环 0/12。** 外部延期仅真实 NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。其余按原合同内部待办。

相关记录：[冻结台账](OPEN-TASKS.md) · [G021](actual-g021-service-pid-runtime-checkpoint.json) · [G022](actual-g022-plc4x-validation-checkpoint.json) · [新版四网关部署](../../e2e-deployment-guide-20261122.md) · [当前六组件证据](../../../../.management-close-20261116/core-rtc-hotplug-20261120/six-current-native-20261124/HANDOFF.md) · [Networking 中断续配证据](../../../../.management-close-20261116/core-rtc-hotplug-20261120/six-networking-purge-20261124/HANDOFF.md) · [当前配对后继联合升级](../../../../.management-close-20261116/core-rtc-hotplug-20261120/six-companion-current-update-20261124/HANDOFF.md) · [已绑定联合维护实现](../../../../.management-close-20261116/core-rtc-hotplug-20261120/companion-retained-enrollment-20261124/HANDOFF.md)
