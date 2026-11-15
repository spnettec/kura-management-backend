# Core source lease 与计划读取期限审查 — 2026-11-08 6fbe49cc

从 5ab34951 续接。目标 active，完整改造未完成；不将以下局部修复记为历史初始 GWT CONFLICT 的根因。

## 入口与现场

入口核对上一轮 23,486 个 source/artifact/evidence 指纹以及 18 仓 HEAD、分支、工作树。保留 216 个将被本轮构建或文档更新触及的旧文件原始字节。原 Orb 四服务 PID 244516/244911/286620/268、console package26/agent20 保持运行；Parallels Ubuntu/Windows 均停止。无提交、推送、reset、clean、旧状态删除、原现场清理或 192.168.2.125 连接。UNKNOWN 无重发，已闭环的一次性验收未重放。

## 缺陷、实现和负向证据

1. **Source 锁交叉已复现。** 5ab 将 SCC 回调移到工作线程；调用方在 `ProviderDefinition.withSource` 持锁期间，回调若读取同一定义的默认值，就会在 `Source.requireCurrent` 等待调用方持有的监视器。`negative-v2.log` 和 `negative-v3-overlay-fixed.log` 保留实际期限耗尽。现在 `active` 使用 volatile，纯状态检查不获取 source 监视器；`invalidate`、`withSource` 和 `withValidatedProperties` 仍保留原锁，既有注销等待已准入写入的测试继续通过。这只消除 Source 状态检查的交叉，不证明任意 SCC/Core 重入或其它 provider 锁兼容。
2. **完整计划逐 guard 重置读取期限已复现。** 原实现中两个各 200ms 的 SCC 可在 300ms 预算下全部通过并派发，`negative-v3-overlay-fixed.log` 的“Expected Rejected but nothing was thrown”保留此证据。现在首次 guard 集合遍历共用一个 budget；有 deletion guard 时的最后 guard 重核也各自共用一个 budget。作用域在准入/实际写入前关闭，避免把已耗尽的预读期限带入后续快照。两轮各自计时，不能宣称整条计划只有 2 秒期限。
3. **不可变属性 Map 的 overlay 空指针。** 初版负向测试实际遇到 `Map.of(...).containsKey(null)` 的 NPE，两个原始报告均保留；已改为遍历 entry 检查 null key。不可变正常 map 可覆盖，null key 仍拒绝，未放宽类型/容量限制。
4. **派发后边界仅有进程内证明。** 测试先执行一次 writer，再由真实 `ProviderConfigurationReads`/Core SCC 读取遍历耗尽预算，外层抛普通 `KuraException` 而非准入前 Rejected，所有 guard 进入 revisionUncertain，后续 readRevision 仍 CONFLICT，writer 次数保持 1。OSGi BundleContext/服务在该测试中由 Mockito 提供，writer 是测试实现；不能记为实际 CM 写入或真实完整快照失败的 native 证明。

## 验证

- JDK21/Maven3.10，显式 `/Users/heyoulin/.m2/repository`，Core configuration 模块 **159 tests / 0 failure / 0 error / 0 skipped**，install 成功。增加 4 项测试：source 锁下工作线程默认值读取；计划共享预读期限；派发后 SCC 超时不释放不确定状态；不可变 map/null key。集中 11 项测试亦通过。所有负向/中间 Surefire 报告在被覆盖前保留；第一轮两个 NPE 是真实缺陷，不伪记为期限回归。
- 新 IDEA 完整隔离 runtime，282 个实际 bundle 加自有 probe，清单 283 项；PID **10843**，端口 18602。只运行一个新的 `source-lease-plan-01` 命令。反射仅用于创建 Core 实际 classloader 的私有 Source/Write 夹具并持有 source lease，SCC 通过真实 OSGi 注册和 Core 跟踪，revision/plan 调用真实 Core。
- 调用方持 source lease 时，实际 SCC 工作线程读取默认值并返回有效 revision，**9ms**；2 个 provider 的计划预读在 **2003ms** 返回 CONFLICT，准入 map 完全不变，uncertainty 为空。6 次回调只出现在两条 `kura-configuration-provider-*` 工作线程上。该夹具定义为空属性的自有元数据，不替代所有生产 provider 兼容验收。
- 新实例已单独停止，fixture/runtime/home、命令准入、结果、日志和快照观察全部保留。原服务未停止。没有新 GWT 编译/测试、Linux cloud 或 MQTT 投递验收；原 GWT210/helper12/Camel8 仅保留历史证据。
- 实际 Core 仓索引已刷新，四条本轮实现/测试路径 freshness metadata_match，无 recorded issue；图谱依然 best effort。

## 当前 pending 与外部 deferred

下列 14 项沿用父台账，不消去原始 26 项 disposition 链。第 2 项仅关闭上面的 Source 状态检查/两个 guard 遍历子项。`requireDeletionClosure` 自身公共视图遍历、withProviderValidation 中多 provider 读取、rollback、其它元数据/AD.validate/CA/crypto/marshaller/文件/外部 owner/实际 writer 截止，以及任意 SCC 回调向 synchronized Core 同步重入仍 pending。真实派发后 provider 超时、注销/完整 DS 元数据链组合和 Linux 兼容尚未补齐。

1. Actual English/new Linux cloud/Camel/lifecycle and broader restricted/stack providers or failure recovery (selected English restricted two saves and Raw three-member DataService/transport successful readback closed 1f93dd7c)
2. Complete server operation deadlines, callback-thread/Core/source-monitor compatibility, other provider/CA/metadata/AD.validate/marshaller/file/external-owner/write bounds, full plan/rollback/dispatch phase scopes, and remaining concurrency/provider/Linux cases (selected single-Core SCC snapshot-read wait/workers/queue/de-dup/release closed5ab34951; earlier selected client/Raw cases retained)
3. All bilingual/provider UX coverage (Raw publisher/subscriber Chinese labels and selected acknowledged-readback messages closed 1274e471)
4. All provider normalized/password/container/empty/add/remove portable paired values and complete full-view typing constraints
5. Complete dedicated cloud/Wire/driver/asset/channel/CSV/import/clone/upload/read/write/domain/dependency/actual-port constraints
6. All twelve delayed production factories, unmatched/dynamic factories and programmatic/non-DS business health/other PID-filter callers
7. Core plan audit facade/events, direct CA and unregistered non-DS writer ownership; formal minimum API/install/upgrade enforcement
8. Durable creation/deletion UNKNOWN and lost-reply receipts/business-result reconciliation/restart; non-idempotent partial/snapshot provider recovery
9. Entire-empty-Core live snapshot/restoration and empty/add/remove fields; production/external PID recovery and partial/inconsistent CM policies
10. NM automatic NAT and other-writer ownership/adoption/retirement/install profiles
11. Encrypted center firewall tasks, bilingual remote editing and affected management connection proof
12. Confirmed-only actual early boot and Core install/upgrade/bootstrap/handoff
13. Consistent DB/artifact/key backup, restore, rotation, migration and release
14. Historical initial GWT CONFLICT exact holder and earlier evidence gaps remain unproved/unchanged

外部 deferred（保持独立）：real NM/cloud-init/physical hardware, amd64, disk full/hard power。历史初始 CONFLICT exact holder、旧证据缺口保留，完整防火墙、完整恢复和发行未建立。

下一安全动作：先补 requireDeletionClosure 的 SCC 遍历预算/准入前异常归类，验证 synchronized Core 重入；继续真实派发后快照期限与 rollback 的边界。不能直接把整个 write 异步超时视为未写入。

机器证据：`docs/evidence/provider-lock-scopes-20261108.json`；原始文件：`target/provider-lock-scopes-20261108/`。
