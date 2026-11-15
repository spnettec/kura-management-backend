# Rollback 完整入口读取预检与派发边界 — 2026-11-08 0d9a5379

从2b724e67续接；目标active，完整管理体系/完整Kura恢复未完成。历史初始GWT CONFLICT exact holder不因本轮新锁/回滚证据被追认。

## 入口与实现

父轮27,236个source/artifact/evidence指纹及18仓HEAD/分支/工作树匹配，241个将被构建或文档更新触及的旧文件bytes保留。原Orb四服务 PID244516/244911/286620/268、console package26/agent20仍运行；Parallels两VM停止。无提交推送、reset/clean、旧现场清理、192.168.2.125连接、UNKNOWN重发或历史闭环一次性验收重放。

rollback从“external apply后逐项metadata读取”拆为两个阶段：

1. `prepareRollback` 在任何external apply/CM更新/删除前，准备全部目标配置与需要重置的defaults，复制snapshot property map，明确factory删除集合，整轮共享SCC读取budget。Unavailable为准入前CONFLICT。factory→factory替换读取新factory的定义/defaults，避免预读旧instance默认值。此阶段没有配置写入或准入/uncertainty更新。
2. 预读涉及的ProviderDefinition按source generation去重，派发前重新取得source lease，撤销的source不能进入准入。然后仅对prepared affected集合增加admission并预置revisionUncertain，调用external apply及prepared CM操作；metadata/defaults不在该CM循环里重新读取。IOException等partial结果仍保留KuraPartialSuccessException，late Rejected/Runtime包为普通CONFIGURATION_UPDATE，Error仍传播但已置gate。成功保存所需快照后，仅清除本次范围pending/uncertainty及pending-delete，而非全局清空无关UNKNOWN。同值rollback也作废旧token。

SCC budget只约束所选读取阶段，**不约束**全部metadata/AD.validate/crypto/CA/SCR/文件/external owner或writer等待；source lease仍可能跨执行阶段等待。shared saveSnapshot自身原有pending-delete处理未改，未宣称全部并发/生命周期与全操作deadline已经解决。

## 测试与负向证据

新增8项完整rollback入口回归：共享预检超时不进入owner/CM；所有metadata读取先于apply；partial CM mutation保留全部准入目标uncertainty；成功只清本次范围pending/UNKNOWN；同值回滚失效旧token；owner apply进入后Rejected不冒充预检拒绝；预读后source撤销不准入；替换factory使用新定义/defaults。前5项在原实现均失败（negative-v3原始reports）；第6项在中间实现仍失败（owner-rejected-negative reports）后修复。前两次夹具编译/缺stub问题、缺LinkedHashMap import编译失败日志，以及中间7项pass报告均保留，不记为首次aggregate成功。

最终Core configuration **177 tests / 0 failure / 0 error / 0 skipped**、JDK21/Maven3.10、显式同一cache，install成功。原两项factory注册回归继续通过；其private旧入口保留，生产完整rollback走prepared分支，不把private函数测试当完整回滚证明。新增partial/source/defaults测试用mock CM、私有owner与可控SCC，不能作为真实NM/硬件恢复验收。

## 实际完整入口：预检拒绝

新IDEA实例PID17011 / HTTP18610，282实际bundle加probe，清单283；首次启动前核对最终Core177及全部清单SHA。只在自己私有实例注入新的external owner保留集合，先断言原集合为空，owner只管理一个 `stack.rollback.blocked.external` confirmed marker；没有模拟或替换现网NM/防火墙，也不替代旧SecurityService/deployment gateway deferred。

实际Core snapshot保存checkpoint，再创建三个快照外Raw factory成员；一个自有真实OSGi SCC仅在rollback metadata预检时不合作阻塞、忽略interrupt。真实rollback入口 **2016ms CONFLICT**：owner validate1/apply0、全部CM mutation尝试0，**所有CM属性及changeCount的摘要、快照文件名/bytes、准入及已有uncertainty均不变**，owner live状态不回滚。原回调仍在运行时已返回拒绝，随后单独释放。未重投该回滚。counter只是观察interposer，所有CM调用都委托真实原CM。

## 首个正常实例的post-check失败

另一个新实例PID17401 / HTTP18611，Core rollback已经返回，随后fixture检查某些不含kura.service.pid的CM时调用immutable Set.contains(null)而NPE，故原receipt失败，**不是产品回滚失败**。三次delete和一次update事件保留；独立只读inspect证明无关UNKNOWN仍在；原始快照序列saved→live→saved且最新无extra root，证明新快照已保存。原owner计数未在该失败assert前独立写文件，不补造原始计数。完整失败receipt/源码/现场/只读inspect/快照SHA保留于postcheck-failure-classification.json。已完成的该回滚没有重投。

## 新正常实例的完整观察

新PID17859 / HTTP18612、全新私有目标，修正fixture null检查并在Core返回后先独立记录结果和每次owner apply。所有生产bundle与前实例相同，Core均为最终177，probe仅为检查/日志修正；启动前283项SHA核对通过。只admit一个新的rollback-healthy-corrected-01。

真实完整rollback入口 **16ms** 返回成功：owner validate/apply各1，confirmed marker恢复saved；4次CM mutation为3个新Raw成员delete及一次Console update；三个快照外factory CM成员已删除。本次prepared scope的uncertainty清除，无关sentinel UNKNOWN保留，旧observer revision失效，新持久快照写成，旧快照bytes保留。CM/deletion及owner事件均CREATE_NEW；未投递第二次写入。

此处owner是独立程序化marker夹具，不能记为实际NM/cloud-init/防火墙恢复。正常回滚包含Console一次update，尚未逐PID证明全部业务public view、password/defaults投影和异步SCC回调收敛；CM方法完成和选定scope gate正确不等于完整Kura恢复。未验证原UNKNOWN操作的对账/跨重启持久化、full empty/add/remove/所有factory身份转换、所有provider/linux/竞态。三个新实例均已停机，完整runtime/home/CM/快照/命令/日志保留，原服务未停。无新GWT/Linux/MQTT投递验收；GWT210/helper12/Camel8是保留旧证据。

## 台账与下一步

原14项pending及26项disposition链、外部deferred分别保留。第2项仅关闭rollback metadata/default预读与派发分离、selected source lease/partial/success gate子项；第8/9项仅增加选定同进程和原生入口证据，不关闭整项。下一轮优先验证：成功rollback后SCC public view滞后时能否取得编辑token；根据snapshot中原始public字段而非补齐defaults，设计pending/convergence投影，特别检查密码和身份字段；同时核对正常回滚的逐PID语义差异（含Console update），不能凭返回成功清业务恢复结论。

1. Actual English/new Linux cloud/Camel/lifecycle and broader restricted/stack providers or failure recovery (selected English restricted two saves and Raw three-member DataService/transport successful readback closed 1f93dd7c)
2. Complete server operation deadlines, rollback metadata/default read-before-external-apply/CM-dispatch and partial/success uncertainty/token boundaries, other synchronized Core API reentry/arbitrary SCC thread/lock/lifecycle/attribute-map concurrency, metadata/AD.validate/CA/SCR/crypto/marshaller/file/external-owner/write bounds and remaining provider/Linux/race cases (single post-continuation Kura/Runtime classification and single/batch Error gate child closed2b724e67; prior recursive validation4a3d0f68, closure/reentryf82f50c4, Source/guards6fbe49cc and SCC bounds5ab34951 retained)
3. All bilingual/provider UX coverage (Raw publisher/subscriber Chinese labels and selected acknowledged-readback messages closed 1274e471)
4. All provider normalized/password/container/empty/add/remove portable paired values and complete full-view typing constraints
5. Complete dedicated cloud/Wire/driver/asset/channel/CSV/import/clone/upload/read/write/domain/dependency/actual-port constraints
6. All twelve delayed production factories, unmatched/dynamic factories and programmatic/non-DS business health/other PID-filter callers
7. Core plan audit facade/events, direct CA and unregistered non-DS writer ownership; formal minimum API/install/upgrade enforcement
8. Durable creation/deletion UNKNOWN and lost-reply receipts/business-result reconciliation/restart; non-idempotent partial/snapshot provider recovery; selected same-Core-lifetime actual three-member conditional deletion followed by SCC snapshot read timeout, one delete/member and retained uncertainty after callback release proven4a3d0f68; durable UNKNOWN/restart/lost reply/business reconciliation and other failure paths remain pending; selected single entered-writer late Rejected/Runtime/Error and batch Error gate regressions, plus actual same-Core native post-CM Runtime classification contrast closed2b724e67; native late Rejected/Error, durable UNKNOWN/restart/lost reply/business reconciliation and other partial paths remain pending
9. Entire-empty-Core live snapshot/restoration and empty/add/remove fields; production/external PID recovery and partial/inconsistent CM policies
10. NM automatic NAT and other-writer ownership/adoption/retirement/install profiles
11. Encrypted center firewall tasks, bilingual remote editing and affected management connection proof
12. Confirmed-only actual early boot and Core install/upgrade/bootstrap/handoff
13. Consistent DB/artifact/key backup, restore, rotation, migration and release
14. Historical initial GWT CONFLICT exact holder and earlier evidence gaps remain unproved/unchanged

外部deferred独立：real NM/cloud-init/physical hardware, amd64, disk full/hard power。历史初始CONFLICT根因与旧证据缺口、全防火墙/全恢复/发行仍未建立。

机器证据 `docs/evidence/rollback-preflight-20261108.json`；原始证据 `target/rollback-preflight-20261108/`。
