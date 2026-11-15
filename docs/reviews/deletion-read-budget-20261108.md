# 删除依赖读取期限与 SCC 普通 CM 读取重入 — 2026-11-08 f82f50c4

从 6fbe49cc 续接；目标 active，整个管理体系改造未完成。本轮局部锁修复不能追认为历史初始 GWT CONFLICT 的确切根因。

## 入口与现场

入口验证 24,049 个上一轮 source/artifact/evidence 指纹、18 仓工作树/HEAD/分支；221 个将被构建或文档更新触及的旧文件原始字节已复制。原 Orb 四服务 PID 244516/244911/286620/268、console package26/agent20 保持运行，Parallels 两 VM 停止。无提交推送、reset/clean、旧状态删除、原 runtime 清理、192.168.2.125 连接或 UNKNOWN 重发。

## 实现与负向证据

1. `requireDeletionClosure` 的公共视图逐项读取原先重置 SCC budget，且不可用读取落入泛化异常分支变为 UNSUPPORTED。新三项回归中，原实现两个失败：两个 200ms provider 在 300ms budget 下竟完成删除；closed reads 被归为 UNSUPPORTED。`deletion-budget-negative.log` 与原始 Surefire bytes 保留。现对整次依赖检查设独立 SCC read budget，Unavailable 转为准入前 CONFLICT。此方法返回后预算结束，最终 guard 重核与实际写入不继承已耗尽的预算。CA/SCR 自身调用、过滤器等仍无完整 deadline，不能声称整个 closure 2 秒内必返回。
2. 实际 SCC 回调向普通 CM 配置 getter 同步读取时，`getPasswordPropertiesForPid` / `trackPasswordProperties` 获取外层调用方持有的 Core monitor，造成回调等待。native 前例 2004ms CONFLICT，原回调在 caller 超时释放 Core 后才完成。新 unit 在改锁前也实际 CONFLICT，原报告保留。现将密码类型缓存的**全部访问**（元数据记忆、注销移除、类型跟踪和读取复制）统一到独立 `passwordPropertiesLock`；不在该锁内调用外部 provider 或 CM，不改动 Core 写入准入锁或缓存值语义。原有密码/元数据及注销相关回归继续通过。
3. 单一生产实现文件 `ConfigurationServiceImpl.java`，无公共 API 改动。两个新增测试文件共 4 项；最终 Core configuration **163 tests / 0 failure / 0 error / 0 skipped**，JDK21/Maven3.10、显式同一个 Maven cache，install 成功。独立缓存锁只证明所选 ordinary CM getter 重入；`readRevision` 等其它 synchronized Core API、任意 SCC 线程假设/生命周期/递归/属性 map 并发一致性仍待核对。

## 实际 OSGi 证据

- 首实例 PID11674 / HTTP18603，Core162 中间 jar；新 Raw 三成员 stack.deletion 仅为本次删除期限故障路径前置创建一次，随后安装两个自有只读 SCC，各慢 1200ms。真实原生 cloud factory guarded deletion 在 **2007ms** 返回 CONFLICT，两个慢回调实际进入，三个成员 revision、准入 map、CM/SCR/service identity 和快照文件名/字节完全不变，uncertainty 空。没有再次投递该删除或创建；目标完整保留。
- 同一首实例新只读 SCC 的普通 Clock CM getter 重入得到 **2004ms CONFLICT**；原 callback 确在 Core 释放后完成，独立后续只读 observation 正常。该证据保留为修复前对照。
- 第二个全新实例 PID12606 / HTTP18604，只验证新的只读 SCC 重入，使用最终 Core163 jar。同一 callback body 在 **1ms SUCCESS**，callback 在 caller 返回时已完成，有效 revision 正常返回。除 Core 和 probe 外，两个 runtime 生产 bundle 字节一致。Probe 的前/后 callback body 相同；外围预期/记录代码不同，不宣称整个 probe source 相同。
- 第二实例先在失败编译期间准备，**未启动**；最终 install 成功后，仅刷新自己尚未运行的 Core jar/清单再首次启动，旧/新 hash 记录在 prelaunch-core-refresh.json。未替换任何正在运行或历史封存 runtime 的 jar。
- 两实例各 282 实际 bundle + 1 自有 probe，清单各283；三个独立新命令均只 admitted 一次。两实例已停机保留全部 runtime/home/命令/日志/快照观察。未运行 GWT、Linux cloud 或 MQTT 投递验收；GWT210/helper12/Camel8 为保留历史证据。
- 正向 raw JSON 沿用了 `completedAfterCallerTimeout` 这个旧字段名；它在正向只是完成计数，不能据此声称发生过超时。正向实际时序由 SUCCESS、elapsedMillis=1 和 completedAtCallerReturn=1 证明，字段名局限在此明确保留。

## 中间失败与证据保留

Core162/install 原中间 JAR、classes/test-classes 和完整 reports 已存 intermediate-core162；随后原 negative reentry 报告另存。编辑脚本误删相邻两个 unmarshal 方法被编译捕获，已按封存原始源码恢复，最终 delta 不再含删除；失败 log/patch 保留。首次改锁后的 full163 只因新测试漏 stub `owned.reentry` CM change count 出现 NPE，其完整 reports 亦保留，补齐 fixture 后最终163/install通过。这些失败不隐藏或记为通过，不再宣称第一轮 aggregate 成功。

## 台账

原14项 pending 和原始26项 disposition 链均保留；第2项只关闭 closure SCC 遍历预算/准入前分类与普通 CM getter 密码缓存锁的子项。完整 server deadline、CA/SCR/metadata/AD.validate/crypto/marshaller/files/external-owner/writer bounds、多 provider validation、rollback、真实派发后快照超时/UNKNOWN、其它同步 Core API 重入及 Linux/provider/竞态组合仍 pending。

1. Actual English/new Linux cloud/Camel/lifecycle and broader restricted/stack providers or failure recovery (selected English restricted two saves and Raw three-member DataService/transport successful readback closed 1f93dd7c)
2. Complete server operation deadlines, arbitrary SCC/Core reentry and other source/provider lock compatibility, metadata/AD.validate/CA/crypto/marshaller/file/external-owner/write bounds, requireDeletionClosure public-view traversal, multi-provider validation/rollback/dispatch scopes, actual post-dispatch snapshot timeout/UNKNOWN and remaining concurrency/provider/Linux cases (Source state-check cross-thread lease and first/final plan guard traversal budget subitems closed6fbe49cc; SCC bounds5ab34951 and earlier selected client/Raw cases retained)
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

外部 deferred 保持独立：real NM/cloud-init/physical hardware, amd64, disk full/hard power。历史初始 CONFLICT exact holder、旧证据缺口、全防火墙/全恢复/发行结论保持未建立。

下一安全动作：审查并实现多 provider validation 与 rollback 的读取阶段预算，补真实派发后快照超时证据；核对 SCC 调用其它 synchronized Core API 的可支持边界。不能把实际写入移到异步线程后将超时误记为“未写入”。

机器证据 `docs/evidence/deletion-read-budget-20261108.json`；原始文件 `target/deletion-read-budget-20261108/`。
