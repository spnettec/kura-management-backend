# 条件写入器进入后的异常分类与不确定状态 — 2026-11-08 2b724e67

从 4a3d0f68 续接；目标 active，完整管理体系改造未完成，不追认历史初始 GWT CONFLICT 根因。

## 入口与实现

入口核对26,048个父轮 source/artifact/evidence 指纹及18仓HEAD/分支/工作树，保留235个将被构建/文档触及的旧文件原始bytes。原Orb四服务 PID244516/244911/286620/268、console package26/agent20保持运行；Parallels两VM停止。无提交推送、reset/clean、旧状态清理、192.168.2.125连接、UNKNOWN重发或历史一次性验收重放。

`updateRevision` 已进入write continuation并消耗准入后，下层KuraException/RuntimeException现在统一包为普通 `KuraException(CONFIGURATION_UPDATE, cause)`，原始cause保留。此时下层Rejected不能再作为整体准入前Rejected向外透传。准入前stale/type/provider validation等既有Rejected语义保留。单项和batch条件写入新补 `Error` 分支：将已准入PID加入revisionUncertain后原样传播Error，与plan已有分支一致；不把Error转换为成功或普通可恢复错误。

新增4项负向回归，原实现全部失败：late Rejected仍是Rejected；Runtime直接逃逸；single Error及batch Error未保留uncertainty。原始log和Surefire bytes在覆盖前保留。修复后完整Core configuration **169 tests / 0 failure / 0 error / 0 skipped**，JDK21/Maven3.10、显式同一Maven cache，install成功。4项新增测试中writer有模拟mutation，故是进程内准入/分类/门禁证明，不当作真实CM或物理故障验收。`Error`用AssertionError验证，不证明OOM、VM崩溃或硬断电时的恢复。

生产仅Core私有实现变动，无公共API/GWT/前端代码改动。既有准入前拒绝、密码保留、stale及其它Core回归仍通过。

## 首个对照误装载，保留并排除

首个全新实例 PID14755 / HTTP18607原计划使用归档Core165；准备脚本的descriptor包复制循环在后续覆盖了Core，实际装载Core169。因此预期旧类型的probe检查失败，该receipt**不用于修复前分类证据**。其自有Raw DataService已经真实CM更新60→61一次，独立只读inspect证明target保留uncertainty；该业务写入未重新投递，实例停机保留。实际异常类型未在该失败assert前单独记录，不能补写或声称取得了原始类型。失败receipt、stage源码、实际inventory SHA、CM事件、只读inspect、现场均保留，见misstaged-comparison-classification.json。

随后使用**另一个全新**前对照实例，并在所有覆盖循环完成后恢复归档Core165；正向实例首次启动前仅细化probe，让observed type先写CREATE_NEW证据再比较预期。其旧probe源码/JAR/hash也保留。两个正确实例在首次启动前验证了全部566项清单文件、准确Core165/Core169 hash、其它生产bundle一致和probe source相同，避免沿用错误标记。没有替换任何正在运行或历史封存runtime的JAR。

## 校正后的真实CM对照

- 正确前例 PID14864 / HTTP18609，归档Core165；后例 PID14908 / HTTP18608，最终Core169。两者各282实际bundle加1个自有probe，各清单283。probe source完全相同，调用参数只指定预期分类不同；其它生产bundle byte一致，只Core不同。
- 各实例新建不同私有Raw三成员，选择其DataService retry integer；对真实Core私有ConfigurationAdmin的自有interposer委托原CM更新一次，再抛同一运行时异常。未伪造CM变更或writer返回。实际CM均60→61、准入各增加1、旧快照字节不变、uncertainty保留，fresh readRevision均CONFLICT，业务写入各仅一次。
- 前例实际 `java.lang.IllegalStateException`；后例 `org.eclipse.kura.KuraException`，cause `java.lang.IllegalStateException`。异常在检查预期之前独立记录于single-observed-failure.json。此 native 对照证明Runtime在单项真实CM mutation之后的分类变化及门禁；late Rejected与Error为新增进程内证明，未宣称native或浏览器验收。
- 三个新实例全部已停机，runtime/home/CM61/旧snapshot60/命令/日志/事件保留。未清除uncertainty或作对账恢复；结论限同Core lifetime，不证明跨重启持久化。没有新GWT、Linux cloud或MQTT投递验收；GWT210/helper12/Camel8为保留历史证据。

## rollback 审查及继续点

本轮读取了rollback、manageCurrentConfigs、manageSnapshotConfigs、rollbackConfigurationInternal和现有FactoryRollbackRegistrationTest。rollback当前先externalConfigurations.apply，再在逐项处理中读getRegisteredOCD/merge defaults；current factory还可能在后续metadata失败前删除。现有两项factory rollback测试仅反射调用private内部函数，不能证明完整入口的读写边界。

rollback本轮**未修改或运行**。下一轮需先以负向回归证明“全部所需metadata/defaults读失败时不得进入external apply或CM写入”，再拆出preflight与实际apply阶段；校验过程中也需审查partial失败如何保留uncertainty、旧token如何失效、成功恢复何时清门禁。不能直接给贯穿整个rollback的旧过程加一个会在派发后继承的过期budget，也不能把private方法两项测试当成全恢复验收。

## 台账

原14项pending、原26项disposition链和外部deferred独立保留；第2项只关闭单项late-exception分类与single/batch Error gate子项。其它Core同步API重入、metadata/AD.validate/CA/SCR/crypto/marshaller/files/owner/writer边界、完整server deadline、rollback、durable UNKNOWN/重启/对账及剩余provider/Linux/竞态仍pending。

1. Actual English/new Linux cloud/Camel/lifecycle and broader restricted/stack providers or failure recovery (selected English restricted two saves and Raw three-member DataService/transport successful readback closed 1f93dd7c)
2. Complete server operation deadlines, single updateRevision post-continuation Rejected classification, rollback read-before-dispatch/partial boundaries, other synchronized Core API reentry/arbitrary SCC thread/lock/lifecycle/attribute-map concurrency, metadata/AD.validate/CA/SCR/crypto/marshaller/file/external-owner/write bounds and remaining provider/Linux/race cases (one recursive validation SCC read budget and close-before-continuation subitems closed4a3d0f68; selected native same-lifetime post-delete snapshot timeout child proven; prior closure/reentryf82f50c4, Source/guards6fbe49cc and SCC bounds5ab34951 retained)
3. All bilingual/provider UX coverage (Raw publisher/subscriber Chinese labels and selected acknowledged-readback messages closed 1274e471)
4. All provider normalized/password/container/empty/add/remove portable paired values and complete full-view typing constraints
5. Complete dedicated cloud/Wire/driver/asset/channel/CSV/import/clone/upload/read/write/domain/dependency/actual-port constraints
6. All twelve delayed production factories, unmatched/dynamic factories and programmatic/non-DS business health/other PID-filter callers
7. Core plan audit facade/events, direct CA and unregistered non-DS writer ownership; formal minimum API/install/upgrade enforcement
8. Durable creation/deletion UNKNOWN and lost-reply receipts/business-result reconciliation/restart; non-idempotent partial/snapshot provider recovery; selected same-Core-lifetime actual three-member conditional deletion followed by SCC snapshot read timeout, one delete/member and retained uncertainty after callback release proven4a3d0f68; durable UNKNOWN/restart/lost reply/business reconciliation and other failure paths remain pending
9. Entire-empty-Core live snapshot/restoration and empty/add/remove fields; production/external PID recovery and partial/inconsistent CM policies
10. NM automatic NAT and other-writer ownership/adoption/retirement/install profiles
11. Encrypted center firewall tasks, bilingual remote editing and affected management connection proof
12. Confirmed-only actual early boot and Core install/upgrade/bootstrap/handoff
13. Consistent DB/artifact/key backup, restore, rotation, migration and release
14. Historical initial GWT CONFLICT exact holder and earlier evidence gaps remain unproved/unchanged

外部deferred独立：real NM/cloud-init/physical hardware, amd64, disk full/hard power。历史初始CONFLICT exact holder、旧证据缺口、全防火墙/全Kura恢复/发行仍未建立。

下一安全动作：对rollback完整入口建立preflight metadata读取失败不派发的负向回归，并实现读阶段与partial/成功门禁边界；不重放本轮三个已经mutation的私有目标。

机器证据 `docs/evidence/single-write-outcome-20261108.json`；原始证据 `target/single-write-outcome-20261108/`。
