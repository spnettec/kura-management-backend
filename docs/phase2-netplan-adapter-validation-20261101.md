# 阶段 2：Netplan 确认值适配与 Kura 最小网络委托

日期：2026-11-01。接续 `phase2-provider-coordination-20261101.md`。
本增量已实现真实旧网络 PID 的确认值/状态适配及本机事务服务，尚未交付原网络 UI/REST
的事务确认入口、完整旧字段写入映射或正式发行 profile，不能视为阶段 2 完成。

## 实际代码与行为

`kura-networking/bundles/org.eclipse.kura.netplan` 新增 NetplanNetworkService，发布
SelfConfiguringComponent、ExternalConfigurationHandler、NetworkStatusService 和
NetplanTransactionService；保留原 PID `org.eclipse.kura.net.admin.NetworkConfigurationService`。
同时设置 service.pid 与 kura.service.pid，确保原配置列表可以发现该 PID。

配置仅镜像 agent 已确认的 revision/sourceDigest 和有界安全配置。候选不进入确认值或快照，
IPv4/IPv6 DNS 分开，旧单地址字段不能准确表达多值时不伪造值。运行态观测与确认配置分开；
UP/carrier 不作为完整连通性证明，不将 Netplan optional 当作 autoconnect。
不支持字段令对应接口只读，原始 entry 仍留在 agent 权威存储，镜像只含允许字段、完整内容摘要
及未知顶层字段名，不把 NM/认证字段带入 Kura 明文快照。

原字段修改仍明确拒绝（相同值可 no-op），`netplan.legacyWrites=false`。
历史 SNAPSHOT_ROLLBACK 保留当前已确认网络，不回放历史网络。
独立 NetplanTransactionService 提供 prepare/query/confirm/rollback；保留调用 request ID，
超时不重发、不承诺取消。需显式 `kura.networking.provider=netplan`、预留原 PID，
并排除旧 org.eclipse.kura.nm writer。安装旧 writer 时新 provider 拒绝激活。

Java 对 LocalAgentService 使用 optional dynamic 引用，允许 agent 迟到或断开，
不可用时立即返回 AGENT_UNAVAILABLE。核心启动加载旧快照时不读取外部确认值，也跳过预留 PID；
正常公开读取和新快照仍必须取得权威确认值，不能绕过 owner 不可用错误。
该修正来自真实 OSGi 启动循环依赖，新增核心回归覆盖早注册、断开 owner 的场景。

Rust agent 的 `AGENT_KURA_NETWORK_INTERFACES` 未设置时拒绝网络委托，显式空值仅供 view，
非空必须是 agent 管理接口名单的唯一子集。只有已登记 Kura 连接可用 `network.kura.*`，
同 UID 普通 admin 连接不能冒充，已登记 Kura 也不能使用通用 network admin 方法或 takeover。
事务按 local-kura 来源和委托接口隔离，其他 actor 的令牌被隐藏，查询及操作均不能越界。
查询只需要 transactionId，确认/回滚要求非空正确 token；幂等键使用来源前缀的 SHA256，
避免不同 actor 的 durable receipt 相撞。旧 journal 新增 IPv6 观测字段带 serde 默认值。

## 验证证据

| 范围 | 本轮结果 | 证据（console 仓库 target） |
| --- | --- | --- |
| Rust agent | macOS/Linux 各 40 测试；Clippy、两端 build 通过 | phase2-kura-network-agent-tests.log、linux-agent-tests.log、agent-clippy-final.log、agent-build.log、linux-agent-build.log（共同前缀 phase2-kura-network-） |
| Java 新适配器 | 9 测试、构建安装通过 | phase2-kura-network-java-reviewed-final.log |
| Java bridge | 本轮前置构建 12 测试通过 | phase2-kura-network-java.log |
| core configuration | 62 测试，含 15 外部归属测试通过 | phase2-kura-network-core-final.log |
| React 双语/契约/浏览器 | 10 通过，0 skipped/unexpected/flaky | phase2-kura-network-browser-final.json；kura-network-final-*.png |
| IDEA 完整隔离 Kura | 真实 Java→Rust UDS，8 项容器检查；原 UI 成功登录，25 PID 包含原网络 PID | phase2-kura-network-runtime.json、center-final-live.json；kura-network-final-original-login.jpg |
| 新快照 | 原网络 PID 一份，无 confirmationToken | phase2-kura-network-snapshot.json |
| 停 Kura 后 | agent.status 成功且 kuraAvailable=false；Kura 方法返回 KURA_UNAVAILABLE | phase2-kura-network-after-stop.json |
| OrbStack 本机角色/网络事务 | 17 项通过，含真实 socket、角色隔离、候选与确认镜像分离、显式回滚、本机同值确认 revision 2→3 | phase2-kura-network-linux-role.json、.log |
| Linux mTLS 回连 | 中心 network.inspect 成功，确认 revision=3 | phase2-kura-network-after-stop.json |

Linux 本轮使用已登记的 synthetic Kura 角色，`javaProviderLinuxValidated=false`，
不是 Linux 实际 Kura JVM/provider 或物理有线验收。仅在现有
`yofc-management-network-20261101` fixture 上修改 dummy 接口；持久 YAML、eth0 配置和默认路由未变。
原 UID、环境及禁用的 Kura 委托已恢复，service 已重启。新 agent ELF SHA256：
`c95b06bd147ffccc4c1fcb720a8869da71043d0ba225d82670722209085ffddc`；旧 ELF 归档在
VM `/opt/yofc-management-phase2/agent-backup-7d825d4402dfbc54`。本轮没有重复 VM 重启；
历史正常重启记录保持历史。Parallels 仍停止，另一台 phase1 OrbStack 未升级。

## 恢复与剩余工作

测试 helper `scripts/phase2-netplan-runtime-probe.py` 只操作本轮独立 `.phase1-dev`。
最终 IDEA JVM PID96075 已停止，新增快照 snapshot_1793474630626.xml 已归档到
netplan-adapter-backup/created-snapshots，原 snapshot_0.xml、config.ini、jvm.args 的 SHA256
已核对恢复。通过构建的 provider/core JAR 保留在隔离运行集，但恢复后的 bundle 列表不加载 provider。
失败启动和各次探针产物继续归档；没有清理原运行目录或个人 ~/.kura-dev。

下一步实现原网络 UI/REST 的 prepare/查询/确认/回滚交互和无损旧字段映射，之后推进
真实 NM/cloud-init 退出、正式安装 profile、注册/转交、nftables、系统控制、离线恢复和发布。
Linux 完整 Kura、DHCP/resolver 全矩阵、物理有线及硬断电等无证据项仍 deferred；未实现项 pending。
新证据 manifest 为 `evidence/netplan-adapter-20261101.json`，前一 manifest 仅代表前一增量。
所有目录整合和实现改动仍未提交、未推送，远端未修改。
