# 阶段 2：配置归属协调与双向 UDS 基础

2026-11-01。此增量为实际 Kura 网络 provider 的前置机制；网络 provider、旧字段映射、
NetworkStatusService 及发行选择仍未交付。不要将 synthetic PID 的通过写成系统网络接管通过。

## 核心配置入口

公共 API `ExternalConfigurationHandler` 由外部权威服务的适配器实现，与
SelfConfiguringComponent 一并注册。安装配置必须在 ConfigurationService 构造前用
`kura.configuration.external.pids` 预留 PID；预留在该服务生命周期内固定，不能依靠
稍后注册的 handler 临时接管一个已经启动的普通 PID。缺失或重复 handler 均拒绝请求。
普通 Kura profile 未设置该属性时保留原有行为。

- 通用配置更新在任何写入前验证原始请求，合并已确认属性后交给 handler，不写该 PID 的 ConfigAdmin。
- 公开读取返回 handler 的确认值及独立属性 Map。候选、待确认值和旧 ConfigAdmin 缓存不作为确认状态。
- 快照写入把所有预留 PID 替换成确认值；owner 缺席时拒绝保存，保留旧快照。
- 显式 snapshot rollback 在重置其他配置之前验证预留 PID，缺少任一预留 PID 时拒绝。
  历史目标以 `SNAPSHOT_ROLLBACK` 来源交给 handler，不将旧 revision 当作当前并发令牌。
- 启动加载旧快照跳过预留 PID。factory create/delete、重复目标和 factory 元数据不能绕过归属。

Handler 必须自行验证字段、revision、来源并调用权威事务服务；禁止反向重入
ConfigurationService。validate 不能产生副作用。apply 表示请求被接受，不能将其等同于网络已确认。
跨 PID/组件操作没有全局原子事务；外部应用后其他组件失败、超时或结果不明时，需要查询权威事务，
不能自动重试修改。未来网络 handler 对历史快照必须保留当前已确认系统网络，不能回放历史网络。
本轮测试 handler 专门接收历史值以验证路由，不定义正式网络回滚策略。

API 包 `org.eclipse.kura.configuration` 导出版本由 1.2.0 升至 1.3.0，核心配置实现要求
`[1.3,2.0)`；artifact snapshot 版本保持。工作空间现有配置包导入范围接受 1.3。
实际网络安装 profile 还须预留原 PID，并确保旧 NM writer 不加载；本轮没有启用该 profile。

## 本机双向传输

Java bridge 新增 OSGi `LocalAgentService`，提供有关联 ID 的反向调用；读取、有限队列写入和
Kura callback 各自执行。单次连接最多四个反向调用，写队列八项，callback 队列四项。
默认调用期限 25 秒，握手/写入 watchdog 5 秒。关联验证连接 owner、ID 和方法；断线立即结束等待，
异常保留协议码和原 request ID；畸形 error 返回 PROTOCOL_MISMATCH。超时不表示取消，也不自动重发。
已过期的排队修改不会因为 ID 被复用而发送。

Rust agent 对应拆开持续读帧、有限队列写帧与最多四个本机请求。等待请求完成和向对端发帧
不会取消读到一半的帧。并发同 ID 关闭该会话；回执校验 owner/ID/方法及 result/error 形状。
取消上层只读调用会清除关联并释放容量，迟到回执忽略。断线、超时不构成已接受系统操作取消证明，
网络事务仍以持久 journal 和后续查询为准。

**本轮没有给 Kura 连接授予 network 方法。** 即使本机 admin 与 Kura 测试 UID 相同，已登记连接
仍受 Kura 角色限制。真正 provider 所需的最小权限、字段限制、来源和 revision 协调在下一增量实现。
现有 `kura.*` 入站能力仍仅配置 PID 列表与快照 ID 列表。

## 验证与限制

| 验证 | 结果 | 证据 |
| --- | --- | --- |
| 核心 API | 23 测试通过 | `target/phase2-provider-api-maven.log` |
| 核心配置 | 61 测试通过，含 14 项外部归属测试 | `target/phase2-provider-core-final-maven.log` |
| 核心 reactor | 编译/安装通过，跳过测试；不等同完整回归 | `target/phase2-provider-core-reactor.log` |
| networking reactor | 1972 项通过，无失败/错误/跳过；此时 bridge 为 11 项 | `target/phase2-provider-networking-reactor.log` |
| 最终 Java bridge | 12 项通过，含 10 项实际 UNIX socket 测试，构建安装通过 | `target/phase2-provider-bridge-reviewed-maven.log` |
| Rust agent | macOS/Linux 各 32 项通过，含 9 项本机双向流/隔离测试；macOS Clippy/build 及 Linux build 通过 | `target/phase2-provider-agent-tests.log`、`phase2-provider-linux-agent-tests.log`、`phase2-provider-agent-clippy.log` |
| React 契约/浏览器 | 初次 3 项纯契约 + 7 项浏览器通过；最终新版 agent 再跑 10 项通过，0 unexpected/flaky/skipped | `target/phase2-provider-browser.json`、`phase2-provider-transport-browser.json` |
| IDEA 完整隔离 Kura | synthetic PID 10 项真实容器检查通过；fixtureOnly=true | `target/phase2-provider-runtime-probe.json` |
| 原 Kura 控制台与 Rust 通道 | admin UI 登录；经 HTTPS/gRPC/UDS 读到 24 PID、真实快照；停 JVM 后 agent 可管理且 Kura 方法正确拒绝 | `target/phase2-provider-kura-live.json`、`target/provider-kura-login.png` |
| 最终 agent 配套正常 Kura profile | IDEA 启动 PID 92077，原控制台登录，24 PID 与原 snapshot_0；验后经 IDEA 停止 | 同 JSON 的 reviewedAgentRun、`target/provider-transport-kura-login.png` |
| OrbStack 实际网络事务 | 17 项通过，含超时/显式回滚、进程重启、正常 VM 重启；随后 mTLS 已回连 | `target/phase2-provider-orbstack.json`、`phase2-provider-orbstack-install.json` |

运行探针 source 位于 core configuration 的 test 源码，不进入产品 JAR。准备/恢复脚本
`scripts/phase2-core-runtime-probe.py` 仅操作本轮 `.phase1-dev/kura-runtime` 和其独立数据。
原 snapshot_0、参数和 config 有备份；新测试快照在停止隔离 JVM 后归档保存，移除测试 PID 预留。
通过构建的 API/core/bridge 留在该隔离运行集。原运行目录和个人 `~/.kura-dev` 未清理。

最终 Linux ELF 为 `7d825d4402dfbc54a259eceb4d87c941bf8ce4548c2323f9cc72c48fe239a247`。
仅更新 `yofc-management-network-20261101` 的已有网络 fixture，旧二进制在该 VM 的
`/opt/yofc-management-phase2/agent-backup-ae7231b77e04a485` 保留；配置、身份、数据库和委托未重建。
第一次启动后立即检查时 socket 尚未创建，随后核对实际 service/socket 就绪；没有重复安装。
Parallels 与另一台旧 phase1 OrbStack 运行二进制仍为此前版本，不把源码 Linux 测试成功写成全 VM 升级成功。

四个真实子仓库图谱已刷新，关键新 Java/Rust/Python/TS 路径为 metadata_match/no_recorded_issue；
manifest 的 freshness 为 not_tracked，已直接读取核对。原有核心/console 解析缺口仍保留，
不把 ready/indexed 当成完整覆盖。快照与 source/evidence 哈希见 `evidence/provider-coordination-20261101.json`。

新网络 provider、真实旧 PID、原网络 REST/UI、Linux 完整 Kura、NM/cloud-init 退出、
DHCP/resolver 全矩阵、物理有线/硬断电、正式包生命周期都没有因此通过。缺环境项继续 deferred，
未实现项继续 pending。Parallels 不为此再开第二台 VM。

所有 consolidation 与实施改动仍未提交、未推送，历史 manifest 保留。
