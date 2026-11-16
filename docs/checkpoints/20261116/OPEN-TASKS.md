# 未验收与剩余实施台账

2026-11-17 Core `b36afab0d6` 已接**发布确认后、尚未申请的原首次启动授权续期**。仅原交接凭据和恢复Main peer（或本地Root）可申请；Root核对原成功发布及request tag、不可变发布、原停止写者/source/generation/cohort、同boot和原writer锁、Core inactive、无agent及无其他未决操作。原固定启动编号须在全部原生目录严格缺失，且从未有任何阶段的启动预留，包括已永久关闭的NOT_ADMITTED。续期另存带Root命名空间、原grant/发布摘要和固定编号的分域HMAC回执；只给原首次启动15分钟单调时间窗口，不重写原grant、不延长发布窗口、不换原启动ID，不发布、不启动。

恢复端新增持久handoff-renewal准入，Root先预留，再保存所属binding并进入首次续期调用；INTAKE先于原生续期意图。浏览器先保留续期UUID/意图，之后才发送HTTPS。回复丢失只读原结果；INTAKE/缺原件不准再发或另起续期。仅正面未受理可显式关闭旧续期预留，再由用户手动新建；原字节、编号、回执及容量全部保留。独立恢复页新增中英文续期和原预留关闭确认，用户仍须另行确认启动；Web Locks、存储失败拒绝、取消焦点/返回值清空和下一步焦点均保留。

12项新增Root实际密码/文件/锁检查、30项原交接、21项撤销相关配置化检查及198项Linux模型（含8项新恢复端）通过，无skip。17组交接页真实HTTPS/生产组件浏览器（新增8组续期）及33组Linux恢复页浏览器通过，Root/人类Main仍为模型。179连接/三UID回归仍空能力/NNP1、Root不能读两方私有目录、真实密钥操作0；**未覆盖新增续期的实际原生写入或安装Main联合验收**，不能外推。最初宽泛unittest误将11项需配置的检查按无配置入口运行，并包含1个不存在模块；随后11项由正确CLI通过，相关模型198项通过，未改生产门禁迁就夹具。受cohort固定源码加载上限由64KiB调整为独立128KiB有界值，以容纳当前代模块；属主/权限/链接和SHA核对未放宽，配置化检查走实际加载器。

新ARM64包203,400,558字节，45项载荷/提取reader、9脚本/3资源/3单元、335无pyc条目及4个原生Python输入匹配。GWT仍 `be67271d03`，JAR和316个Java11类、45个原生输入类核对未变；此前137 JVM/双模块12变体及GWT浏览器证明保留原范围，本次未重跑。候选未部署，原3现场/7进程/38私有元数据/6旧数据未变，UNKNOWN未重发。本批stage、旧c226包和日志直接清理417,979,566逻辑字节，空闲实增214,421,504字节，仍6容器/17卷，无备份、无远程推送。

已存在启动预留/意图或永久关闭编号后的合法恢复、原生续期INTENT处置、缺原grant/跨boot/换锁、完成授权归档及容量、全部升级状态/正式六组件配对和实际安装Main联合验收仍内部pending；未放宽初始化状态白名单。完整12父项和仅3外部边界不变，目标active。周普通额度90%仍可用，credits未变；modern-web-guidance本次只随恢复页功能应用dialog/accessibility，仍为次要任务。

此前Core `c226ff396f` / GWT `be67271d03` 已接三类GWT**已保留私有意图的原未准入结算**：LEGACY、代准备/核验及初始handoff签发都须核对原管理员、脱敏请求、原命名空间及完整原文件。Root明确关闭为NOT_ADMITTED且同一UUID在全部原生目录严格缺失后，另存绑定原预留和原字节摘要的关闭回执；原意图、旧编号和历史容量全部保留。读取不关闭，旧编号永久不能发送；未知、INTAKE、破损、缺命名空间或跨类型冲突继续阻止。界面显示“原预留未受理”，不会伪装为密钥操作成功；用户显式新建才分配下一组编号，初始签发仍保留准备/核验，只更换交接/发布/启动三编号并重新核对与明确授权。同时修复生命周期读取LEGACY时误用锁文件：现在使用Java实际持有的`.key.lock`，独立进程阻塞检查通过；初始化状态白名单未放宽。

137项选定JVM、75项Root相关检查、223项Linux模型、51项配置化交接与71项实际密码/存储检查均通过，无skip；双GWT模块12变体打包通过。生产GWT/HTTPS/RPC的18组代准备流程、11组预留管理浏览器通过，Root/人类Main仍为模型。实际三UID/179连接已覆盖三类保留意图的原预留关闭和3次迟到旧发送帧拒绝：Root/GWT/恢复全空能力/NNP1，Root仍不能读两方私有目录，未创建规范数据密钥，真实密钥操作0；Main/systemd及写handler仍为模型。该179连接不证明新增未使用grant撤销或正式安装Main联合验收。新ARM64包203,393,616字节，45项载荷/提取reader、9脚本/3资源/3单元、335无pyc归档条目及4个原生Python输入匹配；UI包34,061,833字节，316个server/shared Java11类及45个原生输入类匹配，主缓存各6。新包未部署，旧一次性验收未重放。

原3现场/7进程/38私有元数据/6旧数据未变，UNKNOWN未重发。本批stage、旧3f9包、两个浏览器夹具目录、GWT编译目录及日志已直接清理550,191,309逻辑字节，空闲实增298,209,280字节；仍6容器/17卷，无备份、无远程推送。Root INTAKE与破损/遗留命名空间、发布后首次启动续期、归档/容量、完整生命周期升级及六组件正式配对仍内部pending；完整12父项和仅3外部边界不变，目标active。周普通额度89%仍可用、credits未变；modern-web-guidance本次仅随功能应用accessibility状态及焦点，仍为次要任务。

此前Core `3f9a81d8a2` / GWT `3ef9dbff5c` 已增加**未使用交接授权**的显式撤销及原管理员手动重新签发。撤销须用原专用凭据和真实恢复Main peer，并核对原Core的PID/ticks/boot/InvocationID、writer锁、source/cohort及准备历史；两个固定原生编号在各目录严格缺失，匹配预留只能为RESERVED/NOT_ADMITTED。Root先保存带命名空间和HMAC的不可变撤销记录，再关闭旧预留；旧发布/启动编号在所有发送方永久禁用。撤销中断显示REVOKING，查询不会代为完成。接收端保留原binding/意图/全部编号，核对Root原撤销和原关闭回执后另存处置元数据；Root INTAKE、破损、异命名空间、续接或原件缺失均不据此放行。原控制台仅接受正面GRANT_REVOKED，保留原准备、已核验备份及历史，由原管理员手动分配新的交接/发布/启动编号，并重新核对条件和明确授权；过期、REVOKING、UNKNOWN和NOT_RECORDED不能替代撤销证明。

21项新增Linux检查（Root/Core模型、实际密码与持久文件；其中私有包读取门禁3项）、含它们的51项配置化交接检查、71项独立实际密码/存储检查、211项相关Linux模型、131项选定JVM检查均通过，无skip；两个GWT模块共12变体构建通过。真实HTTPS/生产组件的15组GWT RPC浏览器与Linux生产handler的33组接收页浏览器通过，Root/人类Main仍为模型。另73连接/三UID验证了现有共同准入和冷恢复实际密码授权回归、空能力/NNP1及Root不能读两方私有目录；**新增撤销本身尚无三UID原生传输或正式安装Main联合验收**，不能用该73连接外推。新ARM64包203,391,746字节，45项载荷、提取reader、9脚本/3资源/3单元和335无pyc归档条目匹配；UI包34,051,031字节，315个server/shared Java11类（含原50密钥类）及19个原生输入类匹配，主缓存各6。候选未部署。原3现场/7进程/38私有元数据/6旧数据未变，UNKNOWN未重发；清理本批stage、旧681包、GWT测试/编译目录及日志488,930,371逻辑字节，空闲实增236,466,176字节，仍6容器/17卷，无备份，无远程推送。

此候选只处理正面证明未使用的授权；GWT已落盘意图/初始签发绑定、发布后首次启动授权过期、完成归档与容量、全部升级状态及正式六组件配对继续内部pending，未放宽包状态白名单。完整12父项及仅3外部边界不变，目标active。modern-web-guidance随这两个页面的小范围功能应用，仍为次要任务。

此台账与 Git WIP 源码一起保留，完整目标未完成。内部 pending 与外部 deferred 严格区分。历史继承范围另保留于 [legacy-scope-v51.md](legacy-scope-v51.md)；其中旧路径、PID、计数、旧“禁止提交清理”等是历史，按 README 的当前状态和用户最新授权执行，不要求恢复旧证据。

## 内部 pending

Core本地候选 `6812e4c89a` 已接冷恢复私有意图及提前pending的原未准入结算。只有Root仍可证明尚未受理、原生记录精确缺失、原备份摘要与口令经保留原钥核验、私有记录完整匹配且无关联启动子记录时，才可显式关闭。关闭后把原pending文件以不覆盖的原子重命名移到原操作目录，保留原inode、内容、属主、权限及单链接；移动前同步原文件，之后同步两侧目录。原恢复意图和所有原ID保留，旧ID仍禁止发送；界面显式确认后才允许用户手动新建恢复。读取不移动标记；关闭回复丢失、文件同步失败或本地关闭回执已有而标记尚未移完，均不准确认或绕过，须沿用原授权完成。丢失标记、重复活跃标记、跨类型冲突、关联启动、Root INTAKE、破损及异命名空间仍阻止；关闭记录仍消耗历史容量。

211项相关模型（含25项新增恢复/包读取检查）与独立入口的71项实际密码组件检查通过，无skip。Linux容器运行生产页面/handler，宿主Chromium经真实HTTPS完成27组浏览器检查；这里Root/解密仍为模型。另一次原生三UID/73连接已使用实际JDK21密码JAR、生产backup-inspect和接收方Controller验证原备份＋口令：错误口令、同钥的另一备份及其他钥备份被拒，详细恢复和提前pending均保留原标记完成结算，迟到原帧被拒。Root/GWT/恢复均空能力/NNP1，Root不能读两方私有目录，保留回执未变，未创建规范数据密钥；真实密钥操作0。此原生检查的Main/systemd及写handler仍为模型，启动分支的实际密码授权和正式安装Main联合验收未据此完成。GWT测试提交 `752acc8` 仅改夹具，生产与 `8a4eb9fb15` 一致；原138 JVM/9组GWT/双模块12变体不重跑。

JDK21/Maven3.10.1离线ARM64构建成功，新包203,386,874字节，45项及提取reader通过，335归档条目无Python缓存；9脚本/3资源/3密钥单元匹配Git和stage，实际密码检查输入JAR也与包一致，19个原生输入类及50个Java11类匹配。新包未部署；Root原子辅助函数虽保持既有系统调用语义，但旧实际Main/发布证明仍只属于原版本，不套给新包。原3现场/7进程/38私有元数据/6旧数据未变，原UNKNOWN未重发。测试容器及临时JDK卷全删，仍6容器/17卷；无挂载/打开文件引用后直接清理本批stage、旧86包及日志417,939,848逻辑字节，清理前后空闲实增231,067,648字节，约387.20 GiB可用，无备份、无远程推送。完整12父项及仅3外部边界不变，目标active；界面继续复用既有forms/dialog交互，仍为次要任务。详见 [Core检查点](managed-core-start-checkpoint.json) 和 [原恢复结算检查点](gwt-key-handoff-checkpoint.json)。

此前本地候选已推进到 Core `6338ad89e4`、GWT `6e83007778`：Root共同持久预留现先于三类GWT及独立恢复的本地**密钥操作意图**，此前仅登记命名空间的缺口已有实现。发送方另存不含密钥/口令/备份/bearer的准入尝试；即使预留回复丢失，原编号也不能获得迟到的首次发送。Root先落INTAKE再进入原生处理，同一请求至多进入一次；缺原件继续UNKNOWN，不能用缺失推断未发送。只有尚未进入处理的RESERVED可显式关闭为NOT_ADMITTED；普通读取不关闭。已受理的原成功结果可结算，升级也可只读核对匹配原件，无须改写旧准入。新Root包括UID0在内拒绝旧写帧，正式包版本/cohort/升级顺序及本地管理客户端配对仍pending。

132项相关JVM检查通过；Linux模型分批去重共173例通过，其中19项Root准入、6项接收端准入。另一次真实UDS/三UID测试有12连接：Root UID0、GWT UID65534、恢复UID65533均空能力/NNP1，Root不能读两方私有目录，仍可在GWT预留时拒绝恢复写入，并在恢复INTAKE且原件缺失时拒绝GWT新写入；原生handler和systemd/Main仍明确为模型，真实密钥操作0。ARM64包45项及提取reader、GWT双模块12变体/48个Java11类均匹配当前源码与测试输入，尚未部署。原3现场/7进程/38份私有元数据/6份旧数据保持，UNKNOWN未重发；清理本批可重建产物及旧156包684,120,676逻辑字节，无备份、无远程推送。原预留查询/关闭的完整GUI、独立恢复未准入回执后的合法界面恢复、INTAKE缺原件恢复、旧未登记目录、全状态升级及完整12父项继续pending；不以本候选宣称全局协调或正式验收完成。详见 [GWT检查点](gwt-key-handoff-checkpoint.json)。

此前本地WIP为 Core `156fa83cc0`、GWT `e0f77f1ba1`。三类GWT写请求在本地意图落盘后，使用SystemService本次有效data目录的路径/dev/inode/UID，与Root状态目录身份及journal.key分域HMAC生成的命名空间登记绑定；只存元数据，不读Core私有文件，不生成或替换密钥。Root持久登记和Core私有标记均不覆盖，重配置不匹配或Root命名空间改变时拒绝；已有标记的原件读取也携带绑定，避免向另一个Root状态取证。新Root拒绝旧GWT写帧，Core/GWT必须配套，正式版本/cohort/升级顺序门禁仍pending。升级已校验登记目录内三类GWT日志及未发送回执，匹配Root原结果或认证授权，并以Java同类POSIX锁保持到停止服务之后；缺原件、INTENT、破损、移动、跨类型ID冲突和隐藏Root预选操作均拒绝。126项JVM检查、111项Linux模型（含23项新增）通过；另一次实际UDS核对共13连接/9类断言，Root UID0与Java UID65534均空能力/NNP1，Root仍不能读Core私有目录，并拒绝错误绑定、其他目录、同UID其他PID和旧写帧；Main/systemd/cgroup仍为模型，密钥操作0次。双模块12变体新包34,015,153字节，43个Java11类及native输入逐字节匹配；ARM64包45项及提取reader通过，未部署。未登记遗留目录、登记失败目录、意图之前的跨UID共同持久预留和整个原子升级门禁仍pending；不能把本增量当成全局协调完成。旧065浏览器证明未重放，完整12父项不关闭。原3现场/7进程/38私有元数据/6旧数据保持；已清理469,792,527逻辑字节本轮中间产物和旧770c包，无备份、无远程推送。详见 [GWT检查点](gwt-key-handoff-checkpoint.json)。

此前本地WIP为 Core `770c87a8a8`、GWT `704d2e7617`。旧密钥RPC和公开恢复资料RPC已改为Java进程内Unix连接，四类GWT密钥客户端共用有界传输，不再启动Python子进程或各自的管道线程池；密钥请求前后的原始字节缓冲区及时清零，原ADMIN/XSRF/HTTPS和公开资料权限边界不变。116项相关JVM检查及双模块12变体打包通过，41个服务端/共享类为Java11且与包一致；新包34,007,852字节，主缓存各6。12项新增Linux实际Unix连接检查证明普通UID65534、空能力、NNP1下，两旧客户端的peer PID与同一JVM的fcntl锁持有者一致，并拒绝不安全socket/目录/链接及非Root peer；仅发送status/recovery-info，没有密钥写操作，也不是实际安装Main验收。Core修复重新配置误拒接收端handoffs目录：空目录允许，已有交接必须由原生命周期验证器确认本地完成且Root原发布/启动成功，否则保留并拒绝。补齐python -I下受保护同目录源码加载，拒绝不安全权限/链接并绕开pyc；9项新增宿主模型和含它们的88项Linux相关模型通过。未扩大Root运行服务DAC能力。新ARM64 deb的45项载荷、9脚本/3资源/单元与Git和stage匹配，提取reader通过，尚未部署。旧065浏览器11组及NoDispatch Linux12项证明保留原提交范围，本轮未重跑。原3现场/7进程/38私有元数据/6旧数据未变；确认无运行引用后清理本轮可重建目录、日志和旧a277包，共469,790,755逻辑字节，未备份，无远程推送。跨UID持久共同准入、所有日志升级门禁及实际data/Root命名空间绑定仍未完成，完整12父项继续pending。详见 [GWT检查点](gwt-key-handoff-checkpoint.json) 和 [Core检查点](managed-core-start-checkpoint.json)。

此前GWT本地WIP `065ffacc58`（恢复流程实现 `8cb27cca0d`） 已为明确未调用发送方的共同准入拒绝保存原编号未发送回执：私有元数据/完成摘要落盘后才返回NOT_DISPATCHED/SERVER_NO_DISPATCH；任何迟到的同编号请求仍被阻止。收尾复核补齐跨发送类型的原编号占用检查：新增用例修复前1失败、修复后通过；无法stat私有目录/意图也不当作缺失。回执绑定类型、操作人、脱敏请求及当前data目录路径/dev/inode，破损、搬移、链接或与已发送意图冲突均拒绝；不存口令、备份或bearer。除回调尚未执行外，还要求严格解析的Root原件缺失回复；普通错误不构成证明，已经发送的UNKNOWN不转成未发送。准备/核验/签发三步骤可读原回执后手动新建申请，旧5编号保留，新建动作不调用writer；原准备、已核验资料及未授权发布/启动ID按步骤保留。旧备份页也接未发送回执确认，但尚无其实际浏览器联合验收。单RPC内固定一次实际data目录解析，跨JVM/配置/Root命名空间绑定仍未完成。103项相关检查、11组真实GWT组件/HTTPS/RPC浏览器检查，以及其中12项Linux/JDK21普通UID无能力的回执模型通过；Root/人类会话仍为替身。双模块12变体新包34,016,525字节，37服务端/共享类匹配且为Java11，主缓存各6。界面按modern-web-guidance/forms和accessibility使用原生辅助按钮、关联说明及下一步焦点。Core仍a277，原3现场/7进程/38私有元数据/6旧数据未变，无远程推送。跨UID独立恢复、全日志升级准入、BUSY/容量/Root不可用等尚无未发送证明的合法恢复和完整12父项仍pending。详见 [GWT交接检查点](gwt-key-handoff-checkpoint.json)。

此前Core本地源码 `a2770c21e9` 已实现独立HTTPS固定操作页、浏览器与接收端各自的持久发送意图，已有意图只读原结果，不因NOT_RECORDED重新发送。发布与启动分开确认，原发布续接须匹配认证回执；不确定续接不能再次转发。原钥冷恢复与交接写操作互斥；包生命周期准入除本地完成回执外，还必须核对Root原发布/启动均已成功。27项新检查在两种权限模型通过（同一27例，不计为54种），22项原恢复回归、121项Linux/JDK21模型和9项真实Chromium/HTTPS测试通过；Root回复仍为替身，新包未部署。旧8b原生身份、7e实际发布续接和4c6实际Main证明不套给此次已改的恢复/lifecycle代码。授权材料下载上传、实际安装Main/GWT/HTTPS联合验收、原授权续期/撤销/归档、过期首次启动及不确定续接后的合法恢复仍pending。详见 [HTTPS交接检查点](https-key-handoff-checkpoint.json)。以下12个父项和3个外部边界不变。

以下较早增量只保留各自提交的原范围；被页首新实现替代的缺项按页首及末尾顺序继续。

此前GWT本地源码 `d83ba8ee14` 已接独立类型协议、同JVM Root客户端和ADMIN/XSRF/HTTPS门禁，以及32份上限、无bearer的固定ID发送意图；任何已有意图只读原结果，包括NOT_RECORDED。只读页面可读取申请列表并核对原授权，应用modern-web-guidance/accessibility；没有签发/发布/启动按钮。双GWT模块12变体、打包和51项相关单元测试通过，6项Linux实际连接检查证明同JVM PID/普通UID与非Root peer拒绝。尚无实际安装Main内的人类会话/XSRF/HTTPS/浏览器联合验收；准备/核验/签发页面、独立HTTPS发送意图、原授权续期/撤销/归档仍pending，12个内部父项均未关闭。详见 [GWT交接检查点](gwt-key-handoff-checkpoint.json)。

此前Core服务实现 `8b0747a10c` 新增Root固定操作交接授权，282项相关模型通过；真实Core进程签发/读取、冷恢复UID受限查询及agent/同UID其他进程拒绝已在同一夹具核验。未发布或启动新代。首次自动化漏列UNMANAGED_WRITER正确拒绝码而失败，已修正但未重跑；整轮凭据元数据初始基线未持久化，不能标为全程元数据或完整自动化通过。当前仅native协议，不是GWT人类管理员/XSRF、HTTPS/浏览器或完整broker。GWT类型/角色接口、独立页及发送意图日志、原授权续期/撤销/归档仍pending；过期不能换ID绕过，NOT_RECORDED不能当成重发许可。现有恢复页的备份＋口令授权未扩大。

此前Core服务实现 `7e1415b02c` 已加入带原停止/写锁交接认证凭据的发布本地续接，252项相关隔离模型通过。一个全新安装在实际EBUSY后保留原INTENT，重启仅该密钥服务仍保留同一原凭据，之后按原UUID完成剩余固定文件，未重发发布/Stop/Start；3条原操作结算，普通启动仍拒绝未经独立授权。首次自动化因夹具命名空间未通过，后在同一原环境核验剩余步骤；未重跑整轮，不记为完整自动化绿灯。完整边界见本轮检查点。无原停止凭据、跨boot/原锁重建、维护启动、独立GUI/权限grant、全部provider及两个保留原UNKNOWN仍pending。

此前Core `f68fa1ce91` 已实现准备历史分页、保留材料的取消及原取消回执结算，219项相关模型及1次实际提取deb/Root服务联调的原范围保留；第二轮只在旧记录经原回执证明CANCELLED/PUBLISHED后准入。它不是全部安装/Main/GUI验收。不得删除历史绕过，不重发取消、不补造缺回执、不处理其他原UNKNOWN。

此前增量：Core/agent/networking `0a6da0e3e0` / `3cf66e4` / `c16cf57`。单个合成profile的实际Main/PID1 ring/CM已在4c6源码通过（17配置/141属性），普通写仍阻止；不等于全部provider或正式安装准入。新增store.save原子保存/读失败保留，以及Java/Rust NM模板KURA2兼容。详见 [本轮检查点](managed-core-start-checkpoint.json) 和 [静态盘点](key-provider-inventory.md)。以下12个父项均未关闭，新源码和实际运行验证范围分别读取，不把旧证明套给新改动。

2026-11-16 此前密钥增量：Core/agent `f718cbf799` / `499bc64` 已有不向 agent 交付原钥的只读 FD 验证候选，并修复了真实systemd `0440` ACL凭据被Java误拒的缺陷。新增包/原生辅助程序配对约束；最新80项相关检查含2种权限/32次实际JVM凭据场景，新ARM64 deb 45项载荷通过。此前FD委托的104项原范围保留。安装 agent 时的发布禁令仍生效；运行时能力/配置准入、privileged broker、实际受控主Kura启动/CM应用、GUI、全部provider及原UNKNOWN恢复尚未完成。本条更新下面第1项中“agent仍为单钥入口”的实现状态，不关闭其父任务；详见 [managed-credentials-checkpoint.json](managed-credentials-checkpoint.json)。

1. **完整密钥生命周期**：Root 可恢复轮换事务、原意图/回执及 UNKNOWN 门禁；原/新代双侧持久保留；多代加密备份与严格原钥恢复；安全的停机/启动交接；全部 provider 数据盘点、逐项原子重加密、原文件保留、重启核对和 GUI 下载/核对/确认。当前已有新 reader、Root多代备份、严格原集合缺失文件恢复与显式启动门禁的未部署候选；离线及Java受控reader现已支持显式指纹绑定的多代文件，修复logical-V1形似密文的合法密码恢复值错误（70测试及macOS/Linux实际JAR通过）。Root已增加不切换现用钥的代准备、不可变暂存历史及原加密备份上传核验；普通单钥/多代共同Root启动门禁已加入source默认unit和3个实际启动入口，原UNKNOWN即使单钥完好仍拒绝启动。reader/launcher/Core单元包文件配对、绑定包指纹的JVM运行期锁、核验原备份后的原生发布事务及独立generation-core-start授权已有候选。此前228检查点的Linux195项原生检查、2种权限的真实systemd/JDK21锁交接及1次实际Core闭环通过，当时ARM64 deb的43个实际文件及提取JAR通过；主Kura损坏不拖住冷恢复基础服务。原生故障测试继续使用明确的systemd/进程替身；另已通过一次实际发行包安装后的Core停机/发布/独立重启：真实Crypto与配置审计facade、快照reader就绪，旧密文保留且仍可解密、新密文代号切换。安装后模板清理缺陷已修复，195项原生+2种权限的systemd锁+1实际Core闭环通过；当时228包非缓存载荷逐项等同于实际验收包，不适用于后续已改变的候选。此为单个Ubuntu24.04 ARM64独立Core/合成数据profile，完整provider/runtime盘点、原子重加密及更多模式仍pending。维护后多代启动授权、原发布UNKNOWN恢复以及取消/已提交历史的GUI处置仍pending；原生历史/取消的当前有限范围见页首。agent已具备Core保管下的只读FD委托候选，完整运行时准入/权限仍pending，不能扩大agent原钥读取权。完整中断发布恢复（含缺原停止凭据/跨boot）、完整恢复/启动界面、provider重加密和正式六组件安装单元配对尚未完成，不能启用实际轮换或自动删除旧钥。旧弱钥仅迁移保留尚未完成安全轮换。
2. **CA custody**：保持用户选定方案A、外部签发；中心不默认保存 CA 私钥。多人保管/审批、quorum、批量管理仍需实施和验证，不能把“同一人持钥”改称已解决。
3. **最小权限与完整发行**：完整 privileged broker、各 provider/launcher/模式/资源/平台/purge 组合；Main 仍11 ambient/13 bounding caps、NNP0。当前三运行场景使用已安装 Core12 等旧组合；最新候选ARM64 Core deb已包含reader及包文件配对，已在全新隔离环境完成单个Core profile安装/换钥，但尚未正式发行或完成全平台安装验收。完整当前源码六组件发行、包配对、安装升级卸载、凭据连续性和全平台门禁仍待完成。
4. **原14组及 API**：原1–6核心/编辑语义、7与12直接 CA/非DS ownership及生命周期、8与9广义配置恢复、10与11 NM/远程管理、13一致备份/迁移/发行、14历史边界全部保留；七个 API2、R017、全部消费者与实际 provider 验证未因局部修复关闭。配置 API 历史运行1.2/构建1.8、后续1.9的正式配对仍需明确。
5. **完整实际接管链**：合法原记录恢复；registered CM/runtime/source→一次加密导出→paired fences→真实 Java drain→native/realStore 初始化→startup/Core owner→ordinary boot/recovery→原 forward/confirm/inverse/source observation/stopped/expiry。已经发生的 UNKNOWN 不可绕过以重做整轮；未执行/未证明的阶段保持 pending。
6. **网络矩阵及历史**：ordinary/recovery、NetworkManager/CM/runtime、双族、NAT true/false、多WAN/modem/DHCP/DNS/Wi-Fi/VLAN/packet/fragment/unsupported；boot/netns/handle 重建后的合法 restore；初始化先退役再原 native inverse；子记录结算、退役、历史、重新接管。原 forward/confirm/inverse、observation、stopped/expiry 的实际完整接受不以模型/临时存储验证替代。初始 INTERNAL 生命周期根因和并行 maintenance-lock flake 保留。
7. **中心与部署 GUI**：中心地址、数据库、CSR legacy、半凭据、首次 TLS UNKNOWN、容量与归档；完整主机/实例/容器用户流程及错误恢复。已完成的首次配置、TLS联合切换、CSR、续期和只读恢复资料各自有限范围保留，不能外推整套正式交付。
8. **数据库与协议**：客户旧加密任务库及完整 GUI 数据库备份/恢复；协议0在线升级、正式发布的校验和协议 crate、跨组件 joint generations。已通过的是合成 schema9 的真实PG备份恢复/当前迁移/rekey，不能重放或替代客户场景。
9. **R017 及文档/技能**：单机部署中安装器已启动 agent 与要求 inactive 的 import/CSR 顺序；统一方案A、工作空间知识库交接；完整 TLS reload/bulk/证书下载上传及平台体验的父项验收。后续用户操作从界面完成，不能以设置环境变量/运行脚本代替产品流程。
10. **G021/G022**：实际 LDAP present/absent；Go31 vet 债务、三个非Java模板 POM、可选生成与未执行.NET范围。保持内部 pending。
11. **上游逻辑同步**：按实际 fork 与依赖关系继续，非fork不直接 merge。保留11 fork/7 nonfork关系与已审查游标，不能因 WIP 提交/推送而前移。PLC4X 两个新增逻辑组、shared SLMP/.NET 三提交等剩余范围继续 review；不盲目批量 merge，不恢复 YOFC 明确删除的模块。
12. **正式源码/发行门禁**：WIP 已保存完整447候选并不等于原 guarded apply/正式发布通过。继续以新源码执行必要范围验证和全盘 review，不把旧报告、green manifest 或已闭环写操作重新跑一遍当作证明。

## 仅有的 external deferred

- 实际 NM/cloud-init/物理硬件交接。
- amd64。
- 磁盘满/硬断电。

其余缺项均为内部 pending。现有内存/磁盘/构建环境问题、产品流程未完成、provider 缺失及 UNKNOWN 都不能自动移入 external deferred。

## 下一步建议顺序

尚未申请的原首次启动已有当前续期候选，保留固定原启动编号，续期不启动、不发布。下一步做实际原生续期及安装Main/恢复peer验收，并继续完成授权归档、Root/发送方历史容量与完整授权生命周期。已有固定启动预留、永久NOT_ADMITTED、私有意图或UNKNOWN不能套用此续期；这些原记录的合法恢复、原生续期INTENT、缺原grant、跨boot/换锁仍须单独推进。不删除历史、不自动换号、不重发UNKNOWN，备份＋口令原钥恢复与专用交接凭据保持分开。

继续旧未登记/登记失败/切换目录发现迁移，完整preparations/handoffs/current-generation及各发送日志的升级/移除状态。本次只新增严格续期记录/准入形状，不放宽初始化状态白名单。正式六组件版本、binary/cohort、协议及安装升级顺序仍pending，不向原现场部署候选。

17组交接页与33组Linux恢复页浏览器使用生产页面、Handler和真实HTTPS，Root/人类Main仍为模型。179连接覆盖现有共同准入/原私有意图结算/冷恢复授权回归，未覆盖新增续期实际原生写入、未使用grant撤销原生流程或启动分支的实际备份密码授权。正式安装Main、人类ADMIN/XSRF/GWT/HTTPS及独立恢复页联合验收、旧备份Tab完整浏览器流程仍需完成。此前一次性实际发布/Main/场景验收不重放更新戳。缺原停止凭据、跨boot/旧锁重建、维护完整链、完整broker、全部provider原子重加密/退役及12父项继续；安装agent仍阻止代发布。用户从界面操作，modern-web-guidance随功能应用且为次要任务；JDK最低21，不追加27。以Git和本台账继续，不恢复旧大型证据链；周普通额度耗尽时只做必要保存收尾再暂停，不提前暂停。
