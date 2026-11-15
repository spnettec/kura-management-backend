# Conditional local configuration plans

2026-11-07. Typed Core plans and selected revision-aware Wires/GWT integration implemented; complete component and recovery coverage remains internal pending. This is not a remotely exposed lifecycle operation.

`ConfigurationRevisionService.applyConfigurationPlanIfUnchanged(updates, creations, deletions, expectedRevisions)` accepts a typed plan rather than an arbitrary callback. All existing mutation targets need a displayed revision. Additional revisions can guard read-only dependencies. A creation requires PID absence, including dormant ConfigurationAdmin entries, or an explicitly declared deletion with its old revision. A same-PID replacement does not use the old instance's native provider definition or defaults when validating the new factory.

Admission holds the same Core monitor as ordinary ConfigurationService update/create/delete. It checks every revision, tracked target, identity, factory and absence condition, copies caller-owned property containers, and holds native provider validation leases before the first dispatch. Direct ConfigurationAdmin and independently mutating component-owned views are outside this monitor; this contract does not claim to lock them.

Execution uses ordinary deletes and creations without intermediate snapshots, followed by one final snapshot through the ordinary updater. When there are no property updates, it explicitly calls `snapshot()` so deleting the final configuration persists an empty snapshot. A batch of external callbacks is not an atomic transaction. Revision admission counters advance once before dispatch, including identical-value submissions and read-only guards. A failure after dispatch withholds revisions for every guard/new PID and cannot be reported as a pre-write `Rejected` result. There is no automatic retry or compensating replay. Pending property views remain unavailable until the component exposes the submitted values; synthesized factory identity is not required in a self-configuring component's public view.

The API defaults reject unsupported operations without invoking legacy writers. `readRevisionViews(pids)` captures a complete bounded set of paired views under the provider monitor and detaches property containers. It preserves public OCD implementations rather than assuming Core `Tocd`. Limits follow the existing local revision contract: at most 512 guards, 512 total update/create/delete entries, 512 properties per configuration and the existing revision container/encoding limits. These limits do not establish coverage of every installed component.

## Implemented Wires integration

2026-11-07 checkpoint 6ce49a2e: `WireGraphService.getForEditing(additionalPids)` and `updateIfUnchanged(graph, additionalUpdates, expectedRevisions)` have unsupported defaults. The provider serializes the conditional operations with its synchronized legacy update; the GWT editor now uses only conditional admission. Legacy callers retain their existing API behavior.

The editing view derives topology from the persisted Core `WireGraph` property, then batch-reads paired member/driver views and confirms the graph revision still matches. It does not attach fresh tokens to the older DS `currentConfiguration`. The DTO preserves all displayed revisions, including removed nodes and graph PID; static factory/channel definitions carry no existing-instance revision. Production GWT policies serialize that full map.

Before any write, the provider validates complete baselines, node/edge/port identities and bounds, factory replacement, creation and additional changes; it constructs one Core plan containing deletions, creations, updates and the serialized graph. GWT authorizes existing PIDs against actual Wire/driver services, without interpolating the submitted PIDs into an LDAP query. Existing assets outside the graph may be adopted only when they are actual WireAssets; an unrelated factory is never deleted as an asset. New factory values come from typed parameters and do not inherit the previous factory's password placeholder or UI metadata.

The Core plan executes before WireAdmin cleanup; the tracker remains open. A post-admission failure is UNKNOWN and blocks a new editing view. The provider's extra uncertainty flag is in memory and is not a durable recovery protocol. No automatic write retry or compensating lifecycle replay exists. A successful UI submission performs bounded read-only rebaseline (up to ten 200 ms waits on pending conflicts) before clearing dirty state; a failed/UNKNOWN submission never enters that path.

Selected fresh original GWT/IDEA CM/DS/disk, bilingual conflict, same-page saves and new-instance checks, and OrbStack/Parallels real lifecycle checks are recorded in `reviews/gwt-wire-plan-20261107.md`. Real WireAdmin message delivery was checked after node and same-PID factory replacement. Native controlled post-admission WireAdmin failure verified property/graph UNKNOWN, unchanged subsequent observation and blocked fresh reads without resending. That test did not interrupt a non-idempotent factory mutation or snapshot write. Empty graph persistence was proven; an entirely empty Core configuration set was only unit tested, not established by these live fixtures.

### Internal pending for Wires

Complete native factory/component/metatype and port constraints; custom import/clone/CSV/asset/channel paths and dirty-form races; actual failed non-idempotent factory deletion/creation and snapshot/partial-dispatch failure; lost-reply business-result reconciliation, persistence and restart behavior of uncertainty; production recovery and exact minimum API install/upgrade compatibility. Current selected proofs do not close those items or the complete redesign. Formal old-API linkage is not guaranteed by the new default methods.

## Shared GWT factory creation

The shared factory creation helper now invokes one Core typed creation plan, which checks PID absence including dormant CM and performs the snapshot. It never falls back to the legacy create/delete writer. The later observer inspects matching service references and SCR DTO state without obtaining the target service. A DS service must have a matching current component ID and service registration ID, with its current description present. ACTIVE is ready. SATISFIED is ready only for an intentionally delayed service declaration (not immediate, no ComponentFactory identifier, nonempty service interfaces); missing references, failed activation, missing/stale service DTOs and immediate SATISFIED are not ready. This proves registered readiness after the Core plan, not business health. It deliberately preserves first-consumer activation. A non-DS service proves registration only, not business health. The read polling uses a monotonic deadline and never opens a ServiceTracker that can synchronously activate the target. Timeout, interrupted observation and post-dispatch failure produce CONFIGURATION_OUTCOME_UNKNOWN, without deleting the admitted configuration or taking a compensating snapshot. A pre-admission typed rejection is returned as conflict/unsupported/invalid argument. This does not introduce a durable creation request receipt or prove recovery after a lost RPC response.

Shared new-instance conversion contains typed parameters and explicit common name/description/factory properties; UI data-map fields are excluded. Shared service authorization and activation waits use an escaped literal PID filter; authorization additionally compares the actual service PID exactly, so a wildcard cannot authorize another PID. Delayed/SATISFIED readiness now has contract and packaged-linkage tests plus selected real CM/SCR/snapshot and native H2 first-consumer SQL proof on IDEA, OrbStack and Parallels. The web2 bundle explicitly imports org.osgi.framework.dto for ServiceReferenceDTO access. This selected proof does not cover all twelve declared delayed production factories, unmatched/dynamic providers, or durable UNKNOWN recovery. Shared GWT deletion now carries the displayed revision through confirmation and asynchronous token acquisition, then submits one Core deletion plan with its final snapshot. General, restricted keystore/SSL and driver/asset RPCs retain their original authorization and XSRF checks. Old PID-only deletion and raw updateProperties explicitly reject OPERATION_NOT_SUPPORTED without any legacy writer. A post-dispatch failure is UNKNOWN with no retry/compensation; the current ServicesUi editor seals its write actions. This selected integration does not close all dedicated lifecycle/domain/dependency/dirty-form coverage.

## Other dedicated paths

Driver/asset info uses the OSGi-filtered general helper; cloud/pub-sub and restricted-service reads use the filtered single helper, and their updates delegate to the general conditional helper. These call chains are evidence of routing, not complete UI/lifecycle validation. Selected real original driver creation/update, WireAsset creation/two same-page updates and channel reads, BaseAsset update, plus Linux conditional helper operations are now recorded in reviews/gwt-conditional-delete-20261107.md. Successful driver/asset writes perform bounded read-only rebaseline to a complete paired DTO before clearing dirty state; an observation failure never triggers another write or relabels an acknowledged success as UNKNOWN. Asset Apply uses native modified updates instead of separate delete/create RPCs. WireAsset and BaseAsset now have distinct correct factory declarations; native Asset views exclude DS registration metadata so it cannot be overlaid into portable CM/snapshots. BaseAsset metadata binds actual English/Chinese resources. Full dirty-form races, CSV/import/clone/channel constraints and remaining domain lifecycle paths stay pending.

Formal minimum-version/install/upgrade handling, full component/metatype coverage, production recovery, NM/NAT writer ownership, encrypted-center firewall editing, early boot, consistent backups/key rotation/migration/release and the original external deferred ledger remain open.

## Full paired portable configuration views

2026-11-07 checkpoint72d42c1d fixes the untyped full-view path recorded in64235d8f. Full views retain the declared localized definitions/defaults/constraints and wire identity, adding explicit runtime types/cardinality only for extra fields. Supported portable scalars, typed reference/primitive arrays and homogeneous nonempty lists/vectors round-trip through the existing conditional writer. Passwords use placeholders; significant new Password and Character whitespace is preserved. Ambiguous/null/nonportable/oversize values reject before exposing an editable full view. The old v5 pre-dispatch ILLEGAL_ARGUMENT receipt remains historical evidence, not a current passing test. See reviews/gwt-full-view-20261107.md for177 tests, actual final native BaseAsset/WireAsset full-view updates, clean CM/SCR/snapshots and real business41. Browser write RPCs and new Linux full-view validation are not claimed for this round. Real provider password/container/empty/add/remove, all components and remaining dedicated lifecycle/domain/dirty-form coverage stay internal pending. Durable creation/deletion UNKNOWN receipts, true partial factory/snapshot failure recovery and arbitrary-provider hard RPC deadlines remain unproved. Raw CM stays rejected; no UNKNOWN write retry.

## Selected cloud Pub/Sub deletion

2026-11-07 checkpoint23dc0a5f: original cloud Pub/Sub Delete now captures its displayed paired revision and target before confirmation/token acquisition, rejects dirty/unavailable views and calls an audited conditional RPC. Actual Pub/Sub service authorization precedes one shared Core deletion plan/final snapshot; old PID-only delete rejects, post-dispatch UNKNOWN is sealed/no retry. Async configuration selection uses a generation; stale callbacks cannot populate newer tabs. Stack delete now checks XSRF before discovery. Whole-stack factory creation/deletion and Pub/Sub creation are still legacy and remain internal pending, along with native ownership/shared dependency rules and actual settings-write rebaseline. See reviews/gwt-cloud-lifecycle-20261107.md for185 tests and real RawMqttPublisher original GWT stale/current/dirty/UNKNOWN proof; no broker delivery/durable outcome/new Linux claim.


## Native cloud stack conditional lifecycle — checkpoint3b5580cd

The earlier23dc0a5f legacy cloud lifecycle statement is superseded for the selected four standard factories. Core now offers a stronger DeletionGuard plan with exact factories/member tokens and bounded CM/portable/proposed/SCR reference closure under final admission. Native shared factory creation has one absence-checked plan/final snapshot; paired stack reads and deletion bind original membership/revisions. Old GWT PID-only stack deletion and Pub/Sub creation reject. Pub/Sub creation binds the displayed endpoint revision/native factory domain; UNKNOWN never retries or compensates and page creation actions seal. See reviews/cloud-stack-plan-20261108.md for146 Core/12 helper/196 GWT and actual selected native evidence. This does not establish all native cloud lifecycle behavior: CloudService deletion is still CONFLICT with unresolved consumer classification, Eclipse deletion untested, fifth single-member Camel unsupported, successful settings rebaseline pending, actual snapshot fault/live uncertainty inspection blocked by a fixture superclass reflection defect. Actual partial CM allocation UNKNOWN UI was observed; durable receipt/restart/reconciliation not established. Direct CA writers and all unregistered non-DS consumers remain outside the closure guarantee.


## Explicit detachable lifecycle observations — checkpoint2c124fba

A component author may declare the private DS property `.kura.configuration.deletion.observers` with an exact reference name (or String array). Core reads only ComponentDescriptionDTO.properties and original ReferenceDTO metadata. Only optional dynamic0..n references with an unbind callback, no declared target and no actual runtime target are detachable observations. CM/component instance properties cannot authorize them. Persisted PID/target dependencies and all undeclared, required, static or single references still block. The selected18 management RequestHandlerRegistry subscribers explicitly opt in; their unregister callbacks were reviewed and exact packaged metadata checked. Actual original GWT CloudService/Eclipse deletion now succeeds. This supersedes the3b5580cd unresolved observer classification in this selected scope. See reviews/cloud-observer-closure-20261108.md for Core150, actual snapshot/partial-CM failure and live uncertainty proofs. In-process UNKNOWN protection is verified; durable receipt/restart/business reconciliation and complete dependency/provider universes remain unestablished.


## Camel and successful paired save readback — checkpoint8c0383b0

Camel's native factory has one exact member and now implements conditional creation/read/strong guarded deletion directly against Core. Shared ServicesUi captures one intent, writes once and performs at most5 subsequent read-only attempts. Only actual paired values/defaults/types/password placeholders plus their own revision replace the editable baseline. An acknowledged write with failed observation remains acknowledged and read-only; it is not reported UNKNOWN and never replayed. General/cloud/restricted reads retain their authorization. Cloud reads full membership, rerenders only when other tabs can be safely replaced, preserves dirty/unavailable tabs and selection generations, and permits lifecycle actions only when all rendered paired versions equal the captured member map. See reviews/cloud-camel-rebaseline-20261108.md for8 Camel/206 GWT tests and selected original GWT two-save/delete/stale-endpoint evidence. Actual readback failure UI, all backend/provider/membership/concurrency paths and durable recovery remain unestablished.


## 2026-11-08 1274e471：真实读回失败与恢复

最新审查：`docs/reviews/cloud-readback-failure-20261108.md`（本文件若位于 reviews 下则使用同目录文件）；证据：`docs/evidence/cloud-readback-failure-20261108.json`，前序8c0383b0继续保留。真实中英文 acknowledged-write 后五次只读失败已证；发现并修复已保存仍 dirty、侧栏缓存旧配置及无版本视图写控问题。修复后无需整页刷新，重选同一服务读取实际值与版本，独立新意图可保存。Raw订阅中文名/描述修正并经原 GWT选择框验证。最终GWT207及login/denali各6排列通过；资源包install通过。新实例3297/4428已停、现场保留，原Orb4服务和两VM在清单封口核对。完整目标仍active；restricted/三成员栈、英文/Linux、完整竞态/provider与其余14项台账未整体闭环，外部deferred独立保留。历史初始CONFLICT精确持有者及证据缺口未补证。不要重放冻结的一次性入口，不重发UNKNOWN、不提交推送。下一步先读本轮审查/清单并实时验证18仓及指纹，再推进restricted/三成员栈实际配对读回。


## 2026-11-08 1f93dd7c：restricted与三成员栈实际读回

最新审查 `docs/reviews/cloud-stack-readback-20261108.md`，清单 `docs/evidence/cloud-stack-readback-20261108.json`，证据根 `target/cloud-stack-readback-20261108`。前序1274e471保留。原英文GWT restricted SSL同页两次保存、Raw三成员DataService同页两次及transport一次保存成功；仅目标成员版本改变；保存前整栈版本条件删除CONFLICT且CM/服务/八快照不变。PID4941已停现场保留，生产代码/构建未改、测试未重跑（保留GWT207/Core150/helper12/Camel8）。完整目标仍active，14项pending及外部deferred独立继承；未覆盖dirty/异步选择/成员竞态或所有provider/Linux。源码发现restricted选择异步回调与删除可用性需审计，尚无实际竞态复现，不记修复。下一步构造受控延迟读取并实现/验证选择代次保护；先实时复核本轮全部指纹/18仓与运行状态，不重放闭环一次性入口，不重发UNKNOWN、不提交推送。


## 2026-11-08 5661efcd：挂起读回期限

最新审查 `docs/reviews/bounded-readback-20261108.md`，清单 `docs/evidence/bounded-readback-20261108.json`，证据根 `target/restricted-selection-race-20261108`。前序1f93dd7c保留。原生等待遮罩阻止普通restricted选择切换，未复现/未修复所假定竞态；源码内部竞态仍pending。实际修复ConfigurationSave读取永不回调时无限等待：已确认写入后每次只读2000ms、间隔200ms最多5次，名义10800ms受浏览器调度影响。迟到回复不采用baseline，云栈reader重建视图前检查isReadPending。旧实现回归11项中1失败已留档，最终UI install/GWT两模块各6排列及210 tests通过。原生PID6469一次写入/五个挂起读RPC仍进入saved/unavailable；五个迟到回复返回也不解禁，显式重读后独立新保存成功。PID5424/6469已停现场保留，完整目标active。未取消服务器请求、未证明provider/Core锁/线程有界性；真实云栈迟到全栈守卫尚未挂起实测。14项pending及外部deferred独立继承。下一步新隔离实际工厂只读委托延迟全栈读取、验证迟到视图不重建，再推进服务器边界；先实时复核指纹/18仓/运行状态，不重放闭环一次性入口、不重发UNKNOWN、不提交推送。


## 2026-11-08 01e7f75c：原生云栈迟到回复对照

审查 `docs/reviews/cloud-late-readback-20261108.md`，清单 `docs/evidence/cloud-late-readback-20261108.json`，证据根 `target/cloud-late-readback-20261108`；前序5661efcd保留。无守卫历史UI JAR与当前UI在两个新IDEA实例对照，其余生产bundle SHA相同，实际Raw工厂/Core审计门面业务保持。一次UI写61后五个实际旧Map61挂住，独立Core写者推进13，释放旧Map：无守卫恢复可编辑61（Core13）；当前守卫继续禁写，显式Refresh并重选才读13，独立新保存62成功。各服务ID保持、uncertainty空、无过期写/UNKNOWN重发。PID6971/7153停机现场保留。生产代码/构建未改，210/Core150/helper12/Camel8仅保留前序测试未重跑。整体active，14项pending及外部deferred独立继承。下一步真实Core读取/provider/同步锁服务器执行与资源边界，未建立生产取消能力；先实时核对本轮指纹/18仓/运行状态，不重放闭环一次性入口，不提交推送。


## 2026-11-08 5ab34951：Core自配置读取隔离

最新审查 `docs/reviews/server-read-bounds-20261108.md`，清单 `docs/evidence/server-read-bounds-20261108.json`，证据根 `target/server-read-bounds-20261108`；前序01e7f75c保留。Core新增provider读取隔离：单实例2工作线程/2排队、2000ms名义读预算、顶层去重、8层嵌套/循环拒绝，超时不提前unget，接纳前不可用拒绝baseline/CONFLICT。旧Core阻塞回归已留，最终Core install与155测试通过；真实OSGi provider忽略中断，2006ms拒绝且其他线程可读Core，重复读1ms拒绝且原回调仍1，显式释放后新版本恢复。PID9360已停现场保留；未重跑UI210/helper12/Camel8。目标active，14项pending及外部deferred独立继承。不是全服务器期限/强制取消：worker改线程，所有provider线程/ThreadLocal/Core/source锁兼容性、跨bundle重启累计资源、其他回调/I/O/plan/rollback/派发后超时仍pending。下一步先审计ProviderDefinition source/validate锁与SCC/快照交叉、完整计划读预算及派发后超时；实时复核新指纹/18仓/运行状态，不重放闭环入口、不重发UNKNOWN、不提交推送。


## 2026-11-08 6fbe49cc Source lease / 计划 guard 读取边界

详见 `docs/reviews/provider-lock-scopes-20261108.md`、`docs/evidence/provider-lock-scopes-20261108.json`。Core 私有 Source 状态检查改为 volatile 可见性，不再等待调用方写入 lease 的 monitor；原 write/invalidate 串行化保留。计划首次 guard 遍历和最后 guard 重核各自共用 SCC read budget，均在实际派发前结束，不是全计划 deadline。不可变 map overlay 修复。Core159/install，fresh native source-held read9ms / two-provider plan CONFLICT2003ms，admissions 未变且 uncertainty 空；PID10843已停。派发后超时保留 UNKNOWN 仅新增进程内证明，实际 CM/完整快照及其它 provider、Core 重入、closure/rollback/validation/IO 仍 pending。原 14 项 pending、26 项历史 disposition、外部 deferred 和历史证据缺口全保留；目标 active，无提交推送或一次性重放。


## 2026-11-08 f82f50c4 删除 closure SCC 预算与普通 CM getter 重入

见 `docs/reviews/deletion-read-budget-20261108.md` 和 `docs/evidence/deletion-read-budget-20261108.json`。closure 共用 SCC read budget，Unavailable 为准入前 CONFLICT；CA/SCR 自身仍无全期限。密码类型缓存全部访问改独立锁，所选 SCC→ordinary CM getter 不再等待 caller Core monitor。Core163/install通过；实际 guarded deletion2007ms拒绝且三成员版本/CM/SCR/快照字节未变；重入前2004ms CONFLICT、后1ms SUCCESS。PID11674/12606已停机保留。其余 synchronized Core API、validation/rollback/真实派发后快照/server整体deadline及其它14项台账剩余内容仍pending；外部deferred独立。目标active，无提交推送或UNKNOWN重发。


## 2026-11-08 4a3d0f68 provider校验阶段预算与实际post-delete快照超时

见 `docs/reviews/validation-read-budget-20261108.md` / `docs/evidence/validation-read-budget-20261108.json`。一次递归provider校验共用SCC预算，进入write continuation前关闭，Unavailable为校验阶段CONFLICT；不是全write deadline。Core165/install通过。实际校验2010ms拒绝且无准入/UNKNOWN；全新Raw三成员各真实CM删除一次后snapshot SCC读2023ms失败，普通KuraException、旧快照未变、三个成员保持uncertainty，释放回调不解锁且无重发。该UNKNOWN证据限当前Core进程，未证明跨重启或对账恢复。第一批量update后读注入未触发，合法写入/快照61保留，错误probe断言已如实记录。PID13532/13820已停机保留。单项updateRevision后准入Rejected分类待核实，rollback/完整server期限及原台账仍pending，外部deferred独立，目标active，无提交推送。


## 2026-11-08 2b724e67 条件写入器进入后的分类/uncertainty

见 `docs/reviews/single-write-outcome-20261108.md` / `docs/evidence/single-write-outcome-20261108.json`。单项write continuation后Kura/Runtime包为普通CONFIGURATION_UPDATE并保留cause，late Rejected不再透传准入前分类；single/batch Error先加入uncertainty再原样传播。4项原实现负向均失败，最终Core169/install通过。校正后的新Core165/169两fixture同源probe各真实CM更新一次60→61，前raw IllegalStateException、后KuraException(cause原runtime)，门禁与旧快照保留，无重发。首误装Core169的“before”失败receipt排除并保留，真实mutation未重放。PID14755/14864/14908均已停机。rollback源码确认apply早于逐项metadata/defaults读取，完整入口preflight/partial/成功门禁需下一轮实施，未作全恢复验收。原台账/外部deferred及历史证据缺口保留，目标active，无提交推送。


## 2026-11-08 0d9a5379 rollback preflight/dispatch/scope gate

见 `docs/reviews/rollback-preflight-20261108.md` / `docs/evidence/rollback-preflight-20261108.json`。metadata/defaults先prepare且共享SCC预算，source lease复核后才admit/external apply/CM；partial/late owner错误保留本次gate，成功仅清本次范围，同值rollback也失效旧token。Core177/install通过。实际完整入口预检2016ms拒绝，owner apply/CM mutation0且全CM属性/changeCount/快照/准入不变；新正常入口16ms、owner apply1、CM4(3delete+Console update)，旧快照保留/新快照落地/无关UNKNOWN不清。首正常fixture postcheck contains(null)失败已保留归类，其已完成回滚未重投；PID17011/17401/17859全停机保留。假owner仅私有marker，不代表NM/全Kura恢复。成功后异步SCC public-view收敛/密码/defaults及逐PID语义仍需下一轮验证，原台账/外部deferred保持，目标active，无提交推送或UNKNOWN重发。


## 2026-11-08 62d9e9b8 回滚收敛与密码分离

189 配置测试全过/install；实际 IDEA PID23362 的新唯一回滚命令证明异步 ManagedService 旧 SCC 视图被 CONFLICT 拦住，原回调释放后取新 token，5 次 CM 变更无重发。CM 密码严格加密、快照逻辑值独立准备；默认和移除字段参与收敛，runtime identity 排除。审查与边界见 [rollback-convergence-20261108.md](./reviews/rollback-convergence-20261108.md)。下一步新目标验证实际 Console 密码/新快照解码；完整恢复、持久 UNKNOWN 和全部14项仍 pending，外部 deferred 不变。目标 active，不重放旧现场。


## 2026-11-08 d350b27d 实际密码与 Console 差异校正

Console 无 PASSWORD 元数据，auth.methodPassword 是 Boolean；此前 Console CM update 是初始空字典的7字段默认持久化，公开逻辑值不变。真实 MQTT PASSWORD 实际 AES CM 存储、新回滚快照两次解码和 paired 公共值一致；唯一新命令15ms/4次CM变更，无重发。189配置测试产物精确复用，本轮无生产改动/构建/测试重跑。两次夹具前置失败已分类保留，三实例停止保留。全文见 docs/reviews/rollback-password-native-20261108.md；下一步全新程序化 SCC 密码非空/数组/Vector链，14项pending及外部deferred不变，目标active。


## 2026-11-08 821a895f 元数据容器与实际密码恢复

修复SCC Vector被拒绝、primitive数组ClassCast、类型不匹配仍返回true及null元素被接受；真实入口4项负向3fail/1error，最终193配置测试/install。全新实际Kura PID24919唯一命令22ms/5次CM变更，6密码字段标量/数组/Vector/空容器的AES CM、paired公开值、新快照重复解码、普通字段恢复与新增字段删除全过，三次真实callback无重发。CM实际ArrayList与SCC公开Vector分开验证。四实例停止保留，夹具错误和真实负向已分类。全文docs/reviews/rollback-password-containers-20261108.md。下一步严格逻辑值恰为有效密文的解密边界回归；14项pending与外部deferred不变，目标active。


## 2026-11-08 101967da 未修复逻辑密码边界

真实193测试产物Kura复现：原逻辑密码恰为有效AES密文时，XML adapter读回正确，getSnapshot二次解密改值。新增5回归3失败/2legacy通过，当前工作树不是全绿；未改生产/未install，193完整通过只为旧冻结产物。新快照须明确logical表示版本并按来源归一化，保留旧无标记CM路径；不能直接全删解密。全文docs/reviews/password-logical-boundary-20261108.md给出具体跨Core/XML provider实施点，先保存XML provider指纹再改。两实例停止保留，未业务CM写入/rollback/重发。14项pending及外部deferred不变，目标active。


## 2026-11-08 9ee0ebed V1逻辑快照密码与来源合同

新snapshot根passwordEncoding=BASE64_LOGICAL_V1，标记PASSWORD只Base64 decode，不再AES试探/二次解密；CM、SCC/confirmed、已加密补丁与decoded来源分别处理，启动重新加密后推CM，known update decode失败停止文件生成。配置210测试/install、XML provider8测试/install。最终27908 stable-map公开/快照有效密文样式逻辑密码保持，6字段真实恢复25ms/6CM/3callback无重发；新runtime27990未复制CM、无目标SCC，冻结标记快照启动CM AES正确。四实例全停；旧无标记歧义/全部provider合同/混合版本与降级等内部pending保留，不能用V1证明全部旧恢复。全文docs/reviews/snapshot-password-format-20261108.md。下一核对legacy provider密码表示与成对安装边界，再durable UNKNOWN/全时限等完整14项；外部deferred不变，目标active。


## 2026-11-08 4a8845f3 SCC密码来源合同

未标记SCC沿旧存储解码，CCI显式false/true区分存储/逻辑，Core复制后解码不改provider；NM明确stored，Asset按动态PASSWORD AD转型并明确stored。Core/XML双向语义能力1.x、Asset/NM sources要求；真实Equinox5个fresh解析场景通过（NM只requirement匹配）。Core214/XML8/Asset38/NM1497测试install。IDEA30627新sources-once四来源paired/V1快照标量数组Vector全过，已停保留。全文docs/reviews/scc-password-contracts-20261108.md。公开Core DTO增量，org.eclipse.kura.api未改；旧provider/热refresh/降级/全发行与真实NM仍未全部验证。14项pending和外部deferred分开，继续durable UNKNOWN和全时限。


## 2026-11-08 af09be80 持久派发与启动门禁

Core私有journal在conditional single/batch/plan与rollback首派发前CREATE_NEW+文件/目录force意图，成功append绑定complete；未决挡写/其他PID可编辑token，普通activate先于snapshot→CM拒绝。恢复决议绑定completion代次，不能复用旧完成清新UNKNOWN。Core227测试/install；新IDEA实际1CM/1callback后snapshot故障留下intent，最终独立进程Core失败激活/无可用实例、CM changeCount1与after值保持、两snapshot逐字节不变，源308文件未变；四实例停。前置/后置夹具失败与零字节snapshot保留分类，未把原失败receipt算通过；Orb4JVM纯文件合同通过。全文docs/reviews/durable-uncertainty-20261108.md。完整receipt/lost-reply/非幂等实际恢复对账、所有legacy/directCA/provider与全时限等14项仍pending，外部deferred不变。UNKNOWN不重发，不删除intent解闸，目标active。


## 2026-11-08 bc82f837 journal与精确释放回执交接

修复“完成后无普通写、立即关闭受控JVM”会遗留intent阻挡普通启动。先force create-only release-link，绑定intent/completion/snapshot/decision及固定receipt Path/SHA，再发布原receipt；新进程只按精确不可变receipt核定，不重发释放。旧发布器遇未决在callback前拒绝，Core导出新增重载，bootstrap要求release1.x能力。Core233/工具58 install全过，真实resolver3场景正确；新普通IDEA34369无agent/无request，Core+新bootstrap正常、原GWT登录，已停保留；Orb6JVM纯合同成功/丢回复/未发布分支通过（直接自有admission夹具，不是真实授权恢复全链）。全文docs/reviews/durable-recovery-contract-20261108.md。14项内部pending含真实授权journal恢复交接、公开receipt/业务对账、一致journal+receipt备份迁移，外部deferred分列。UNKNOWN不重发、不删intent、旧一次性验收不重放，目标active。


## 2026-11-08 3e81381f 实际正向恢复日志与普通运行交接

3e81381f：Core238/工具62/Rust229（11忽略）+Clippy通过；新增默认关闭 applied-forward normal 私有授权。补齐 V1 工具校验，明文 logical-v1 不伪称认证；真实 normal 的两项 keystore 密文漂移经停止后复制 CM 取证，Core 保留已核对原密文，strict managed CM 比较不放宽。新 native-v6：一次 UNKNOWN→实际正向→normal25配置/181属性严格匹配+真业务/非空密码→独立完成→无 REQUEST ordinary→驱动退出后快照+原 GWT 登录，实际链1通过，所有自有现场已停保留。前5现场/构建失败和误输入旧XML的精确前缀恢复审计全保留，不冒充通过。18仓/父43031pins入口核对，原Orb四PID与包不变，两VM stopped；没有新Linux部署或系统沙箱验收。14内部pending与外部deferred保持；下一步客户端operationId/receipt/lost-reply对账及全台账，不重发UNKNOWN、不重放一次性入口、不提交推送。

详细审查与范围：`docs/reviews/authorized-journal-handoff-20261108.md`；证据 `target/authorized-journal-handoff-20261108`；清单 `docs/evidence/authorized-journal-handoff-20261108.json`。目标继续 active。

## 2026-11-08 40141a77 客户端回执 Core 基础

公开 configuration API1.8 新增带客户端 op-UUID 的 plan overload 与独立 ReceiptService；Core 在预检前 force 值无关 reservation，writer 前 force 原 intent 和精确绑定，所有 ID 复用拒绝，非幂等重放接口。查询独立于 ConfigurationService/CM/Crypto 激活，不调 writer；CORE_COMMITTED 不代表业务健康，后续恢复为 RECOVERED，UNKNOWN/NOT_FOUND 均不能自动重发。只对完全未预留 ID 允许显式封存以阻止迟到请求，孤立派发证据拒绝封存。API24/Core253/工具62 install 全过，新真实打包 Core/SCR 无 CM/Crypto provider 的独立 Receipt DS 检查1通过；未重跑旧场景。GWT/client 和真实创建/删除丢回复对账尚未接入，14项内部 pending 与外部 deferred 保留。合同 `docs/lifecycle-client-receipts-v1.md`，审查 `docs/reviews/lifecycle-client-receipts-20261108.md`，清单 `docs/evidence/lifecycle-client-receipts-20261108.json`。下一步客户端保留 ID/RPC 携带与只查询对账，不重发旧 UNKNOWN；目标 active，无提交推送。


## 2026-11-08 5dc22468 selected GWT client integration

The earlier Core-only client status is superseded for generic New Component and default generic Services delete only. Retained value-free IDs, recorded RPCs, independent original receipt queries, separate current readiness/absence inspection and explicit unreserved sealing are now integrated and locally verified after lost response/reload/restart. See `gwt-client-receipts-v1.md` and `reviews/gwt-client-receipts-20261108.md`. Broader domain, real post-CM UNKNOWN/business recovery, server deadlines and formal deployment remain pending.


## 2026-11-08 936572c2 post-CM UNKNOWN and authenticated independent receipts

Selected actual single factory creation UNKNOWN/original-ID query/refresh/restart and Core-unavailable authenticated GWT child now verified after fixing persisted role-store availability and skipping normal service-list fetch while a retained ID is pending. Read-only role backend has fixed lowest priority, no defaults and rejects mutations; a distinct healthy native control verifies normal role-store selection/login/services. Useradmin17/Web2 226 pass. Original op-c4cca347-40ff-415b-bc0a-911754697f28 remains UNKNOWN in retained postcm-receipt-45ded12b; no recovery/release/inverse/replay. Full non-idempotent recovery/business handoff, dedicated domains and full14-item ledger remain pending. See `persisted-read-only-identities-v1.md`, `reviews/gwt-postcm-receipts-20261108.md`, and `evidence/gwt-postcm-receipts-20261108.json`.
