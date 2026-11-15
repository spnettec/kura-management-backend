# 多 provider 校验读取预算与实际派发后快照超时 — 2026-11-08 4a3d0f68

从 f82f50c4 续接，目标 active，完整管理体系改造未完成。历史初始 GWT CONFLICT exact holder 与旧证据缺口不因本轮局部证据被追认为已证明。

## 入口与实现

入口验证父轮 25,166 个 source/artifact/evidence 指纹和18仓 HEAD/分支/工作树；保留230个将被构建或文档更新触及的旧文件 bytes。原 Orb 四服务 PID244516/244911/286620/268、console package26/agent20 保持运行，Parallels 两VM停止。无提交推送、reset/clean、旧状态清理、192.168.2.125连接、UNKNOWN重发或旧闭环一次性验收重放。

`withProviderValidation` 现在为一次递归 provider 校验共用 SCC read budget，递归进入新私有 `validateProviders`。读取当前 SCC 不可用时为准入前 CONFLICT。在进入 write continuation 前显式关闭本轮预算；writer/快照/它调用的后续校验分别建立自己的读取阶段。预算的 finally 再关闭只清理本线程上下文；不延伸为整个 write 的绝对期限。原 Source lease 和 AD.validate 的执行/串行化保留。

新增两项回归：两个各200ms SCC在300ms校验预算下原实现仍派发，负向实际失败报告保留；新实现超时不消耗准入、不标记UNKNOWN。另一项在校验已耗时后进入writer，延迟再读取另一个SCC，证明writer不继承已耗尽的校验预算。最终 Core configuration **165 tests / 0 failure / 0 error / 0 skipped**，JDK21/Maven3.10、显式同一cache，install成功。生产仅一个Core私有实现文件变化，无公共API、GWT或前端改动。

## 实际多 provider 校验

首实例 PID13532 / HTTP18605，282实际bundle+probe，清单283。两个自有SCC通过真实OSGi注册；仅自有PID的Core元数据map注入真实classloader创建的空属性ProviderDefinition，不能替代生产动态元数据注册链。回调每个PID首次guard读取立即返回，第二次校验读取慢1200ms。真实计划在 **2010ms CONFLICT**，两个provider各调用2次，准入/uncertainty/CM/service/快照均未改变。只admit一个新命令，无重发。

## 第一处派发后夹具假设失败，证据保留

同一首实例另建全新Raw三成员，为DataService实际CM更新60→61后才阻塞旁路SCC。该**注入未触发**：普通 `updateConfigurationsInternal` 在CM写入前 buildCurrentConfiguration，写入后 saveSnapshot(configs)，不重新读取SCC。Core正常返回成功且实际CM和新持久快照均为61、uncertainty为空。probe错误断言“snapshot timeout falsely acknowledged success”是夹具错误，不是产品错误确认。失败receipt/log、实际CM写入事件、独立只读inspect、两版快照值与SHA均保留于 injection-miss-classification.json；没有再次投递这个已经完成的写入。不能把此次未发生的超时记为通过。

## 真实派发后超时

改用第二个**全新**实例 PID13820 / HTTP18606，同一最终Core165 jar及全部其它生产bundle字节。只运行新的 postdelete-snapshot-01，创建新的Raw stack.deleted作为该故障路径前置。Core私有ConfigurationAdmin interposer 委托原真实CM；包装返回的Configuration及Configuration数组，仅在自己三个CM对象的真实delete已返回后开启旁路SCC阻塞。三个delete分别记录到独立CREATE_NEW事件文件。interposer没有伪造删除/异常/超时，SCC才是受控不合作的只读提供者。

真实原生 guarded deletion 已各删除三个CM配置一次，随后 deletion-only plan 调用真正 snapshot() 重新读取当前配置。SCC回调忽略interrupt，Core在 **2023ms** 返回 **org.eclipse.kura.KuraException**，并非准入前Rejected；此时原回调仍未结束。真实CM列表中三个对象均消失，各成员准入仅增加1，revisionUncertain包含全部三个，readRevision均CONFLICT，旧持久快照文件名和bytes完全不变。原回调释放后确实结束，但uncertainty仍不清除。没有第二次删除、补写或UNKNOWN重发。此证据只证明同一个Core进程内的所选后派发快照读取失败边界，不证明跨重启UNKNOWN持久化、业务结果对账或恢复已完成。

两个实例均各启动一次，现均停止；完整fixture/runtime/home、实际CM、原快照、事件/命令/日志/观察全部保留，原服务未停。第二个probe在首次启动前细化为预先捕获CM identity、每次delete用独立不可变事件文件，旧source/JAR/hash保留；没有替换运行中runtime。两实例除probe外全部bundle SHA一致，Core均匹配最终install产物。未运行新GWT/Linux/cloud MQTT投递验收；GWT210/helper12/Camel8仅保留旧证据。

## 台账与继续方向

原14项pending、原26项disposition链和外部deferred分别保留。第2项仅关闭每轮递归校验SCC读取预算；第8项仅关闭同进程内真实post-delete snapshot SCC期限失败保留uncertainty子项。rollback尚未修改：其metadata读取可能发生在external apply或CM操作之后，需要先审查/分离读取与派发阶段，不能直接套一个贯穿整个rollback的过期budget。AD.validate、metadata、crypto、CA/SCR、文件与writer自身仍无全操作期限。

另有待验证源码风险：单项 `updateRevision` 在write continuation已进入后捕获KuraException并原样抛出；若下层此时抛Rejected，可能仍对外暴露准入前分类，而batch/plan已显式包装。下一轮优先用负向回归核实并修复这个边界，不因本轮真实plan路径正确而宣称单项也已覆盖。

1. Actual English/new Linux cloud/Camel/lifecycle and broader restricted/stack providers or failure recovery (selected English restricted two saves and Raw three-member DataService/transport successful readback closed 1f93dd7c)
2. Complete server operation deadlines, other synchronized Core API reentry/arbitrary SCC thread and lock compatibility, lifecycle/attribute-map concurrency, metadata/AD.validate/CA/SCR/crypto/marshaller/file/external-owner/write bounds, multi-provider validation/rollback/dispatch scopes, actual post-dispatch snapshot timeout/UNKNOWN and remaining provider/Linux/race cases (deletion closure SCC traversal shared budget/pre-admission CONFLICT and password-cache ordinary CM getter reentry subitems closedf82f50c4; Source and first/final guard scopes6fbe49cc, SCC bounds5ab34951 and prior selected client/Raw cases retained)
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

外部deferred独立：real NM/cloud-init/physical hardware, amd64, disk full/hard power。全防火墙、全Kura恢复和发行未建立。

下一安全动作：先核实单项updateRevision在write回调后Rejected的分类；再为rollback设计先读后派发的预算/partial边界，保留所有未完成provider/生命周期/竞态及完整台账项。

机器证据 `docs/evidence/validation-read-budget-20261108.json`；原始证据 `target/validation-read-budget-20261108/`。
