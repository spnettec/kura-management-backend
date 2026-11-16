# YOFC Kura 阶段性源码检查点 — 2026-11-16

本检查点按用户最新要求保存本地Git提交并清理可重建中间产物，取代此前“不提交推送”的阶段限制。它保存未完成的工作，不是正式发行或完整验收。后续从这批 Git 源码和 [未验收台账](OPEN-TASKS.md) 开始，不需要恢复旧 `target/`、旧 Maven/Cargo 缓存、证据清单或源副本。

18 仓共同使用**本地**分支 `wip/management-close-20261116`；按用户最后确认，未达到主线合并条件的代码只留本地，**没有推送远程 WIP 分支**。每个有修改的仓先保存原工作树，再保存完整管理候选；可通过提交历史回退重构。干净仓也保留同名本地分支。447路径中仅2个独立CI文件已合入自己的远程 `spnettec/kura` 的 `heyoulin-kura` 主线，提交 `dc788aa2361d2c14971c0b45533848e83f17aadd`；其余445路径保留本地，见 [候选归并审查](CANDIDATE-REVIEW.md)。远程提交从原远程主线单独建立，没有顺带推送本地主线中另3个未推送的检查点提交。最终本地提交与远程读回见 `.management-close-20261116/`。

管理候选共 447 个路径，涉及 core、networking、GWT、agent、console、Docker。用于历史隔离构建的 3 个生产树替代文件与 4 个小 reactor 单独保存在 [build-fixtures](build-fixtures/)，没有覆盖正式 reactor/生产清单。新增密钥代号代码、配置边界测试和 JDK 最低版本修复一并保存。此处的 WIP 源码归档不替代原“完整 guarded apply / 正式发行”验收门禁；G034 未封签，最后接受 G033/V61、最后完整代码封签 V51。

## 已保存的最新工作

- 2026-11-17 已在全新隔离安装场景验证 Core `b36afab0d6` 的**实际原首次启动续期**；可复现测试源码本地提交 `43e3ad899e`，生产文件未变。当前包及声明依赖真实安装，使用实际PID1/Core、Root和恢复Main；浏览器经实际HTTPS/生产页面与Handler执行且固定证书指纹。授权由本地Root签发，**不是GWT人类ADMIN/XSRF联合验收**。原授权按真实boot时钟等待900秒，不修改时钟或原授权。页面只各发送一次publish、start-renew、start和complete；原生准备/核验/发布/续期/启动5项均成功，原启动编号不变；续期后普通启动门禁仍拒绝，明确启动后实际新JVM的Crypto、配置审计facade及快照bundle就绪，合成旧密文原字节保留且解密成功，新密文使用发布钥。原grant完整元数据及Root/恢复PID/ticks保持，Root/恢复全能力集0、NNP1，同恢复UID的其他进程在预留前被拒。Core仍保留原11 ambient/13 bounding、NNP0，完整broker未完成；运行profile声明256MiB堆、DS探针和旧unit种子，不能外推全部provider/平台。

这一新场景1项自动化997.044秒通过，6个原编号已闭环只读；专用容器和JDK卷已删，不恢复或在其他Root中查询这些编号。首次host测试脚本在启动worker/安装前因stdio描述符未打开失败，修正后新夹具完整通过，未分发密钥请求、未重放旧闭环。内部Docker网络端口探测不可达，最终采用断开网卡、仅lo及透明TLS字节转发，没有替换Root/HTTPS回复。原3现场/7进程/38私有元数据/6旧数据仍精确，UNKNOWN未重发；仍6容器/17卷，无备份、无推送。

同时独立模型复现两个**P1容量缺口**：255个共同准入后续期占满256，原启动预留被CAPACITY拒绝；255个原生操作后续期占满256，原启动进入INTAKE后才因CAPACITY被拒，原生启动原件缺失。没有启动Kura，也未改动实际场景。此问题未修复，不能以普通路径实测通过封签；先在共同锁内保证固定发布/启动的双侧容量承诺，再做完成授权归档/历史容量，不能仅提上限、删历史或换ID。详见 [容量审查](../../reviews/handoff-archive-capacity-review-20261117.md) 和 [实际场景检查点](first-start-installed-checkpoint.json)。完整12父项和仅3外部deferred原文不变，目标active；周普通额度90%仍可用，credits未变。界面本次复用既有生产组件，modern-web-guidance仍为次要；JDK最低21，不追加27。

- 2026-11-17 Core `b36afab0d6` 已接**发布确认后、尚未申请的原首次启动授权续期**。仅原交接凭据和恢复Main peer（或本地Root）可申请；Root核对原成功发布及request tag、不可变发布、原停止写者/source/generation/cohort、同boot和原writer锁、Core inactive、无agent及无其他未决操作。原固定启动编号须在全部原生目录严格缺失，且从未有任何阶段的启动预留，包括已永久关闭的NOT_ADMITTED。续期另存带Root命名空间、原grant/发布摘要和固定编号的分域HMAC回执；只给原首次启动15分钟单调时间窗口，不重写原grant、不延长发布窗口、不换原启动ID，不发布、不启动。

恢复端新增持久handoff-renewal准入，Root先预留，再保存所属binding并进入首次续期调用；INTAKE先于原生续期意图。浏览器先保留续期UUID/意图，之后才发送HTTPS。回复丢失只读原结果；INTAKE/缺原件不准再发或另起续期。仅正面未受理可显式关闭旧续期预留，再由用户手动新建；原字节、编号、回执及容量全部保留。独立恢复页新增中英文续期和原预留关闭确认，用户仍须另行确认启动；Web Locks、存储失败拒绝、取消焦点/返回值清空和下一步焦点均保留。

12项新增Root实际密码/文件/锁检查、30项原交接、21项撤销相关配置化检查及198项Linux模型（含8项新恢复端）通过，无skip。17组交接页真实HTTPS/生产组件浏览器（新增8组续期）及33组Linux恢复页浏览器通过，Root/人类Main仍为模型。179连接/三UID回归仍空能力/NNP1、Root不能读两方私有目录、真实密钥操作0；**未覆盖新增续期的实际原生写入或安装Main联合验收**，不能外推。最初宽泛unittest误将11项需配置的检查按无配置入口运行，并包含1个不存在模块；随后11项由正确CLI通过，相关模型198项通过，未改生产门禁迁就夹具。受cohort固定源码加载上限由64KiB调整为独立128KiB有界值，以容纳当前代模块；属主/权限/链接和SHA核对未放宽，配置化检查走实际加载器。

新ARM64包203,400,558字节，45项载荷/提取reader、9脚本/3资源/3单元、335无pyc条目及4个原生Python输入匹配。GWT仍 `be67271d03`，JAR和316个Java11类、45个原生输入类核对未变；此前137 JVM/双模块12变体及GWT浏览器证明保留原范围，本次未重跑。候选未部署，原3现场/7进程/38私有元数据/6旧数据未变，UNKNOWN未重发。本批stage、旧c226包和日志直接清理417,979,566逻辑字节，空闲实增214,421,504字节，仍6容器/17卷，无备份、无远程推送。

已存在启动预留/意图或永久关闭编号后的合法恢复、原生续期INTENT处置、缺原grant/跨boot/换锁、完成授权归档及容量、全部升级状态/正式六组件配对和实际安装Main联合验收仍内部pending；未放宽初始化状态白名单。完整12父项和仅3外部边界不变，目标active。周普通额度90%仍可用，credits未变；modern-web-guidance本次只随恢复页功能应用dialog/accessibility，仍为次要任务。

- 此前Core `c226ff396f` / GWT `be67271d03` 已接三类GWT**已保留私有意图的原未准入结算**：LEGACY、代准备/核验及初始handoff签发都须核对原管理员、脱敏请求、原命名空间及完整原文件。Root明确关闭为NOT_ADMITTED且同一UUID在全部原生目录严格缺失后，另存绑定原预留和原字节摘要的关闭回执；原意图、旧编号和历史容量全部保留。读取不关闭，旧编号永久不能发送；未知、INTAKE、破损、缺命名空间或跨类型冲突继续阻止。界面显示“原预留未受理”，不会伪装为密钥操作成功；用户显式新建才分配下一组编号，初始签发仍保留准备/核验，只更换交接/发布/启动三编号并重新核对与明确授权。同时修复生命周期读取LEGACY时误用锁文件：现在使用Java实际持有的`.key.lock`，独立进程阻塞检查通过；初始化状态白名单未放宽。

137项选定JVM、75项Root相关检查、223项Linux模型、51项配置化交接与71项实际密码/存储检查均通过，无skip；双GWT模块12变体打包通过。生产GWT/HTTPS/RPC的18组代准备流程、11组预留管理浏览器通过，Root/人类Main仍为模型。实际三UID/179连接已覆盖三类保留意图的原预留关闭和3次迟到旧发送帧拒绝：Root/GWT/恢复全空能力/NNP1，Root仍不能读两方私有目录，未创建规范数据密钥，真实密钥操作0；Main/systemd及写handler仍为模型。该179连接不证明新增未使用grant撤销或正式安装Main联合验收。新ARM64包203,393,616字节，45项载荷/提取reader、9脚本/3资源/3单元、335无pyc归档条目及4个原生Python输入匹配；UI包34,061,833字节，316个server/shared Java11类及45个原生输入类匹配，主缓存各6。新包未部署，旧一次性验收未重放。

原3现场/7进程/38私有元数据/6旧数据未变，UNKNOWN未重发。本批stage、旧3f9包、两个浏览器夹具目录、GWT编译目录及日志已直接清理550,191,309逻辑字节，空闲实增298,209,280字节；仍6容器/17卷，无备份、无远程推送。Root INTAKE与破损/遗留命名空间、发布后首次启动续期、归档/容量、完整生命周期升级及六组件正式配对仍内部pending；完整12父项和仅3外部边界不变，目标active。周普通额度89%仍可用、credits未变；modern-web-guidance本次仅随功能应用accessibility状态及焦点，仍为次要任务。

- 此前Core `3f9a81d8a2` / GWT `3ef9dbff5c` 已增加**未使用交接授权**的显式撤销及原管理员手动重新签发。撤销须用原专用凭据和真实恢复Main peer，并核对原Core的PID/ticks/boot/InvocationID、writer锁、source/cohort及准备历史；两个固定原生编号在各目录严格缺失，匹配预留只能为RESERVED/NOT_ADMITTED。Root先保存带命名空间和HMAC的不可变撤销记录，再关闭旧预留；旧发布/启动编号在所有发送方永久禁用。撤销中断显示REVOKING，查询不会代为完成。接收端保留原binding/意图/全部编号，核对Root原撤销和原关闭回执后另存处置元数据；Root INTAKE、破损、异命名空间、续接或原件缺失均不据此放行。原控制台仅接受正面GRANT_REVOKED，保留原准备、已核验备份及历史，由原管理员手动分配新的交接/发布/启动编号，并重新核对条件和明确授权；过期、REVOKING、UNKNOWN和NOT_RECORDED不能替代撤销证明。

21项新增Linux检查（Root/Core模型、实际密码与持久文件；其中私有包读取门禁3项）、含它们的51项配置化交接检查、71项独立实际密码/存储检查、211项相关Linux模型、131项选定JVM检查均通过，无skip；两个GWT模块共12变体构建通过。真实HTTPS/生产组件的15组GWT RPC浏览器与Linux生产handler的33组接收页浏览器通过，Root/人类Main仍为模型。另73连接/三UID验证了现有共同准入和冷恢复实际密码授权回归、空能力/NNP1及Root不能读两方私有目录；**新增撤销本身尚无三UID原生传输或正式安装Main联合验收**，不能用该73连接外推。新ARM64包203,391,746字节，45项载荷、提取reader、9脚本/3资源/3单元和335无pyc归档条目匹配；UI包34,051,031字节，315个server/shared Java11类（含原50密钥类）及19个原生输入类匹配，主缓存各6。候选未部署。原3现场/7进程/38私有元数据/6旧数据未变，UNKNOWN未重发；清理本批stage、旧681包、GWT测试/编译目录及日志488,930,371逻辑字节，空闲实增236,466,176字节，仍6容器/17卷，无备份，无远程推送。

此候选只处理正面证明未使用的授权；GWT已落盘意图/初始签发绑定、发布后首次启动授权过期、完成归档与容量、全部升级状态及正式六组件配对继续内部pending，未放宽包状态白名单。完整12父项及仅3外部边界不变，目标active。modern-web-guidance随这两个页面的小范围功能应用，仍为次要任务。

- Core本地候选 `6812e4c89a` 已接冷恢复私有意图及提前pending的原未准入结算。只有Root仍可证明尚未受理、原生记录精确缺失、原备份摘要与口令经保留原钥核验、私有记录完整匹配且无关联启动子记录时，才可显式关闭。关闭后把原pending文件以不覆盖的原子重命名移到原操作目录，保留原inode、内容、属主、权限及单链接；移动前同步原文件，之后同步两侧目录。原恢复意图和所有原ID保留，旧ID仍禁止发送；界面显式确认后才允许用户手动新建恢复。读取不移动标记；关闭回复丢失、文件同步失败或本地关闭回执已有而标记尚未移完，均不准确认或绕过，须沿用原授权完成。丢失标记、重复活跃标记、跨类型冲突、关联启动、Root INTAKE、破损及异命名空间仍阻止；关闭记录仍消耗历史容量。

211项相关模型（含25项新增恢复/包读取检查）与独立入口的71项实际密码组件检查通过，无skip。Linux容器运行生产页面/handler，宿主Chromium经真实HTTPS完成27组浏览器检查；这里Root/解密仍为模型。另一次原生三UID/73连接已使用实际JDK21密码JAR、生产backup-inspect和接收方Controller验证原备份＋口令：错误口令、同钥的另一备份及其他钥备份被拒，详细恢复和提前pending均保留原标记完成结算，迟到原帧被拒。Root/GWT/恢复均空能力/NNP1，Root不能读两方私有目录，保留回执未变，未创建规范数据密钥；真实密钥操作0。此原生检查的Main/systemd及写handler仍为模型，启动分支的实际密码授权和正式安装Main联合验收未据此完成。GWT测试提交 `752acc8` 仅改夹具，生产与 `8a4eb9fb15` 一致；原138 JVM/9组GWT/双模块12变体不重跑。

JDK21/Maven3.10.1离线ARM64构建成功，新包203,386,874字节，45项及提取reader通过，335归档条目无Python缓存；9脚本/3资源/3密钥单元匹配Git和stage，实际密码检查输入JAR也与包一致，19个原生输入类及50个Java11类匹配。新包未部署；Root原子辅助函数虽保持既有系统调用语义，但旧实际Main/发布证明仍只属于原版本，不套给新包。原3现场/7进程/38私有元数据/6旧数据未变，原UNKNOWN未重发。测试容器及临时JDK卷全删，仍6容器/17卷；无挂载/打开文件引用后直接清理本批stage、旧86包及日志417,939,848逻辑字节，清理前后空闲实增231,067,648字节，约387.20 GiB可用，无备份、无远程推送。完整12父项及仅3外部边界不变，目标active；界面继续复用既有forms/dialog交互，仍为次要任务。详见 [Core检查点](managed-core-start-checkpoint.json) 和 [原恢复结算检查点](gwt-key-handoff-checkpoint.json)。

- 本地候选已推进到 Core `6338ad89e4`、GWT `6e83007778`：Root共同持久预留现先于三类GWT及独立恢复的本地**密钥操作意图**，此前仅登记命名空间的缺口已有实现。发送方另存不含密钥/口令/备份/bearer的准入尝试；即使预留回复丢失，原编号也不能获得迟到的首次发送。Root先落INTAKE再进入原生处理，同一请求至多进入一次；缺原件继续UNKNOWN，不能用缺失推断未发送。只有尚未进入处理的RESERVED可显式关闭为NOT_ADMITTED；普通读取不关闭。已受理的原成功结果可结算，升级也可只读核对匹配原件，无须改写旧准入。新Root包括UID0在内拒绝旧写帧，正式包版本/cohort/升级顺序及本地管理客户端配对仍pending。

132项相关JVM检查通过；Linux模型分批去重共173例通过，其中19项Root准入、6项接收端准入。另一次真实UDS/三UID测试有12连接：Root UID0、GWT UID65534、恢复UID65533均空能力/NNP1，Root不能读两方私有目录，仍可在GWT预留时拒绝恢复写入，并在恢复INTAKE且原件缺失时拒绝GWT新写入；原生handler和systemd/Main仍明确为模型，真实密钥操作0。ARM64包45项及提取reader、GWT双模块12变体/48个Java11类均匹配当前源码与测试输入，尚未部署。原3现场/7进程/38份私有元数据/6份旧数据保持，UNKNOWN未重发；清理本批可重建产物及旧156包684,120,676逻辑字节，无备份、无远程推送。原预留查询/关闭的完整GUI、独立恢复未准入回执后的合法界面恢复、INTAKE缺原件恢复、旧未登记目录、全状态升级及完整12父项继续pending；不以本候选宣称全局协调或正式验收完成。详见 [GWT检查点](gwt-key-handoff-checkpoint.json)。

- 此前本地WIP为 Core `156fa83cc0`、GWT `e0f77f1ba1`。三类GWT写请求在本地意图落盘后，使用SystemService本次有效data目录的路径/dev/inode/UID，与Root状态目录身份及journal.key分域HMAC生成的命名空间登记绑定；只存元数据，不读Core私有文件，不生成或替换密钥。Root持久登记和Core私有标记均不覆盖，重配置不匹配或Root命名空间改变时拒绝；已有标记的原件读取也携带绑定，避免向另一个Root状态取证。新Root拒绝旧GWT写帧，Core/GWT必须配套，正式版本/cohort/升级顺序门禁仍pending。升级已校验登记目录内三类GWT日志及未发送回执，匹配Root原结果或认证授权，并以Java同类POSIX锁保持到停止服务之后；缺原件、INTENT、破损、移动、跨类型ID冲突和隐藏Root预选操作均拒绝。126项JVM检查、111项Linux模型（含23项新增）通过；另一次实际UDS核对共13连接/9类断言，Root UID0与Java UID65534均空能力/NNP1，Root仍不能读Core私有目录，并拒绝错误绑定、其他目录、同UID其他PID和旧写帧；Main/systemd/cgroup仍为模型，密钥操作0次。双模块12变体新包34,015,153字节，43个Java11类及native输入逐字节匹配；ARM64包45项及提取reader通过，未部署。未登记遗留目录、登记失败目录、意图之前的跨UID共同持久预留和整个原子升级门禁仍pending；不能把本增量当成全局协调完成。旧065浏览器证明未重放，完整12父项不关闭。原3现场/7进程/38私有元数据/6旧数据保持；已清理469,792,527逻辑字节本轮中间产物和旧770c包，无备份、无远程推送。详见 [GWT检查点](gwt-key-handoff-checkpoint.json)。

- 此前本地WIP为 Core `770c87a8a8`、GWT `704d2e7617`。旧密钥RPC和公开恢复资料RPC已改为Java进程内Unix连接，四类GWT密钥客户端共用有界传输，不再启动Python子进程或各自的管道线程池；密钥请求前后的原始字节缓冲区及时清零，原ADMIN/XSRF/HTTPS和公开资料权限边界不变。116项相关JVM检查及双模块12变体打包通过，41个服务端/共享类为Java11且与包一致；新包34,007,852字节，主缓存各6。12项新增Linux实际Unix连接检查证明普通UID65534、空能力、NNP1下，两旧客户端的peer PID与同一JVM的fcntl锁持有者一致，并拒绝不安全socket/目录/链接及非Root peer；仅发送status/recovery-info，没有密钥写操作，也不是实际安装Main验收。Core修复重新配置误拒接收端handoffs目录：空目录允许，已有交接必须由原生命周期验证器确认本地完成且Root原发布/启动成功，否则保留并拒绝。补齐python -I下受保护同目录源码加载，拒绝不安全权限/链接并绕开pyc；9项新增宿主模型和含它们的88项Linux相关模型通过。未扩大Root运行服务DAC能力。新ARM64 deb的45项载荷、9脚本/3资源/单元与Git和stage匹配，提取reader通过，尚未部署。旧065浏览器11组及NoDispatch Linux12项证明保留原提交范围，本轮未重跑。原3现场/7进程/38私有元数据/6旧数据未变；确认无运行引用后清理本轮可重建目录、日志和旧a277包，共469,790,755逻辑字节，未备份，无远程推送。跨UID持久共同准入、所有日志升级门禁及实际data/Root命名空间绑定仍未完成，完整12父项继续pending。详见 [GWT检查点](gwt-key-handoff-checkpoint.json) 和 [Core检查点](managed-core-start-checkpoint.json)。
- 此前GWT本地WIP `065ffacc58`（恢复流程实现 `8cb27cca0d`） 已为明确未调用发送方的共同准入拒绝保存原编号未发送回执：私有元数据/完成摘要落盘后才返回NOT_DISPATCHED/SERVER_NO_DISPATCH；任何迟到的同编号请求仍被阻止。收尾复核补齐跨发送类型的原编号占用检查：新增用例修复前1失败、修复后通过；无法stat私有目录/意图也不当作缺失。回执绑定类型、操作人、脱敏请求及当前data目录路径/dev/inode，破损、搬移、链接或与已发送意图冲突均拒绝；不存口令、备份或bearer。除回调尚未执行外，还要求严格解析的Root原件缺失回复；普通错误不构成证明，已经发送的UNKNOWN不转成未发送。准备/核验/签发三步骤可读原回执后手动新建申请，旧5编号保留，新建动作不调用writer；原准备、已核验资料及未授权发布/启动ID按步骤保留。旧备份页也接未发送回执确认，但尚无其实际浏览器联合验收。单RPC内固定一次实际data目录解析，跨JVM/配置/Root命名空间绑定仍未完成。103项相关检查、11组真实GWT组件/HTTPS/RPC浏览器检查，以及其中12项Linux/JDK21普通UID无能力的回执模型通过；Root/人类会话仍为替身。双模块12变体新包34,016,525字节，37服务端/共享类匹配且为Java11，主缓存各6。界面按modern-web-guidance/forms和accessibility使用原生辅助按钮、关联说明及下一步焦点。Core仍a277，原3现场/7进程/38私有元数据/6旧数据未变，无远程推送。跨UID独立恢复、全日志升级准入、BUSY/容量/Root不可用等尚无未发送证明的合法恢复和完整12父项仍pending。详见 [GWT交接检查点](gwt-key-handoff-checkpoint.json)。
- 此前Core本地源码 `a2770c21e9` 已接独立 HTTPS `/handoff` 操作页及接收端持久意图，发布、原凭据续接、独立启动均绑定原5个UUID；浏览器先落盘意图、跨页协调，HTTPS接收方再落盘，已有意图一律只查原结果，NOT_RECORDED不准重发。页面只接受专用交接凭据，备份＋口令的严格原钥恢复权限未扩大；两种恢复流程互斥。原生确认框默认取消，Esc不会沿用上次确认值；界面按modern-web-guidance/dialog及accessibility随功能改造。27项新检查分别在非Root接收方和Root包准入权限模型中通过，另22项原恢复回归、121项Linux/JDK21相关模型、9项真实Chromium/HTTPS页面检查通过；浏览器的Root回复为替身，不是实际Main联合验收。ARM64新包45项及提取reader通过、9脚本/3资源/单元均匹配Git和stage，尚未部署；旧8b包确认无live bind后删除。当前恢复服务和生命周期已改，旧8b身份、7e发布续接、4c6实际Main证明仍仅属于各自原版本。实际安装GWT联合核验、授权材料下载上传、原授权续期/撤销/归档、过期首次启动及不确定续接后的合法恢复继续pending，12个父项均未关闭。详见 [HTTPS交接检查点](https-key-handoff-checkpoint.json)。
- 此前GWT本地源码 `d83ba8ee14` 新增独立交接类型协议、同JVM Unix连接和ADMIN/XSRF/HTTPS入口；发送前落盘固定ID意图，重复请求与NOT_RECORDED只查询原授权，不重发、不换ID。原操作人限定显式bearer读取，发送日志及只读页面不存bearer。原密钥页新增中英文交接申请查询区，按modern-web-guidance/accessibility使用原生标题、label/select/button、progress和完成状态提示。JDK21下双GWT模块12个编译变体、打包及51项相关单元测试通过；另6项无密钥Linux/JDK21实际UDS连接检查通过，选定9个server/shared类为Java11且与打包产物一致，新RPC序列化策略在包内。它不是实际安装Main内的GWT/XSRF/HTTPS/浏览器联合验收；页面签发、独立HTTPS交接操作与发送意图、原授权续期/撤销/归档及完整台账继续pending。Core/native源码未改，原生闭环证明仍精确属于各自旧提交，不重跑。详见 [GWT交接检查点](gwt-key-handoff-checkpoint.json)。
- 此前Core服务实现 `8b0747a10c`，后续测试提交见 [本轮检查点](managed-core-start-checkpoint.json)；agent/networking仍为 `3cf66e4` / `c16cf57`。Root侧新增固定操作交接凭据：绑定当前已注册Core实例/源/包cohort、原准备及验证、预选发布和启动ID，15分钟boottime窗口只允许首次发布/启动；重复命令仅读匹配原请求HMAC的结果，已受理发布的认证本地续接可在窗口后继续。凭据不接受任意路径/命令/替换ID，不改变备份＋口令的原钥恢复权限，Root授权记录不存bearer。282项相关模型通过；新增真实Core Java进程及冷恢复UID/agent cgroup核验，在同一原夹具完成剩余检查，没有发布/启动新代。首次自动化漏列正确拒绝码UNMANAGED_WRITER而停止，已修正但未重跑整轮；原进程内整轮凭据元数据基线未持久化，不外推全程元数据连续。它不是GWT人类ADMIN/XSRF、HTTPS/浏览器或完整broker验收。新ARM64包45项及提取reader通过，9脚本匹配Git；旧7e包无live bind后删。下一步接GWT类型/权限、独立HTTPS与发送意图日志，以及原凭据续期/撤销/归档；界面发布未开放，完整原台账不缩项。
- 此前Core服务实现 `7e1415b02c` 新增原发布的认证续接凭据、只读查询和显式本地续接：只有原StopUnit确认及独占写锁交接已完成才留凭据，原UUID/请求HMAC、旧文件完整身份、包/cohort、同boot/原锁inode及所有其他UNKNOWN均须匹配。252项相关隔离模型通过。实际新安装发生规范key原子替换EBUSY，主Kura停止且原INTENT保持；在同一夹具释放故障、重启仅该密钥服务并核对原凭据/前缀不变后，原3条操作结算SUCCEEDED，普通启动仍需独立授权。首次自动化因夹具挂载命名空间问题未通过，后在原环境验证剩余步骤；未重建/重放整轮。现存ring和暂存原件在成功续接期间完整元数据不变，旧密文字节匹配原Core输出；首次进程内整轮旧密文元数据基线未持久化，不外推。这是7e原范围，不套给后续已改动的native。
- 此前Core `861352a3a3`（服务实现 `f68fa1ce91`）已实现 `generation-history` 分页、保留全部旧材料的 `generation-cancel`，以及仅核验并结算原取消回执的 `generation-cancel-recover`；缺原回执不补造，其他UNKNOWN仍阻止。新一轮准备只在全部旧记录经原回执证明CANCELLED/PUBLISHED后准入，不删目录绕过。219项相关隔离Linux/JDK21模型检查和1次实际systemd Root worker新接口联调通过；真实服务重启后原取消结果可读、两轮暂存均保留、现用密钥完整元数据不变，实际kurad/recovery/agent cgroup权限边界符合预期。这次提取deb运行服务，不是完整安装/Main/GUI验收；其原范围保留，不重放来更新源码戳。
- 此前Core `d20f4b56eb` 新增只读 `generation-preparation-read`，只返回经不可变历史校验的准备元数据，不返回密钥/加密备份，不推断后续发布或启动；原INTENT保持UNKNOWN。当时6项隔离Linux模型通过。暂存取消/分页的旧缺口由上条实现替代，其他边界继续保留。
- 此前Core/agent/networking本地源码为 `0a6da0e3e0` / `3cf66e4` / `c16cf57`。受控启动先设业务屏障再校验目标级别，修复初始framework stop不能阻止后续激活的问题；完整Core隔离profile显式声明级别6。`4c6d50ce63`的一次真实Main/PID1 ring/CM联调通过：17配置、141属性一致，旧密文字节未改且可解密，普通写仍阻止，没有发布代、完成或释放写。后续0a6修复store.save直接覆盖/读失败丢条目，9类相关118项及隔离Linux6项通过。当前ARM64包45项文件及提取reader通过，Main/离线Crypto与测试类一致；实际Main证明仍精确属于4c6源码，不重放闭环来更换提交戳。
- 18仓静态盘点还补齐Java/Rust NM模板的KURA2格式，19项Java/18项Rust相关检查通过，原nonce/实例/epoch/摘要/一次导出与UNKNOWN约束保留。成功夹具已删除；失败合成Main夹具确认仅只读源码输入且Root操作/启动为空后已删除，原请求 `rec-cae4c810-5bcf-4189-92d1-55d53bde348d` 编号/摘要及未接受状态保留，没有推断成功或重发。三个原网关未动。详见 [本轮检查点](managed-core-start-checkpoint.json) 与 [provider盘点](key-provider-inventory.md)。完整agent准入/privileged broker、原UNKNOWN恢复、GUI、全provider及正式发行继续pending。
- 此前 Core 本地源码为 `f718cbf799`（agent仍`499bc64`）。真实PID1凭据联调发现并修复了Java把systemd的ACL掩码`0440`误判为普通组可读的问题：复用现有原生精确ACL校验，只回传文件身份/元数据及摘要，再由Java核对；传统私钥文件权限未放宽。新增配对Crypto/managed-loader能力与native proof v1声明，旧组件即使刷新外层文件hash也不能混入。80项相关检查通过，其中一项真实systemd测试含2种权限/32次JVM场景，涵盖旧新密文、缺钥、错误代集合与失效inode/version/digest回执。新ARM64包45项载荷及提取JAR通过，未部署；主Kura受控启动/CM应用、agent准入/privileged broker、GUI和完整provider仍pending。详见 [managed-credentials-checkpoint.json](managed-credentials-checkpoint.json)。
- 此前 Core/agent `0d77355a0d` / `499bc64` 新增 Core 保管密钥的只读快照验证，agent 只传快照只读 FD；新私有证明绑定配置、原钥/代集合及实际包指纹。104项相关有限检查通过，包括一次全新 systemd/实际 Rust 客户端/Root worker 联调；原/新代密码及外层密文可验证，`kurad` 所有的目标文件可用，agent 私有目录隔离和管理备份拒绝通过，Root 未运行也可加载 agent 配置。当时的 Java 受控凭据路径仍缺少真实ACL适配，由上条修复补齐；没有把直接私有文件的单元结果当成受控Main已通过。准确原范围见 [core-validation-checkpoint.json](core-validation-checkpoint.json)。
- 旧密钥迁移、旧加密数据连续性、独立 Root 密钥服务、普通 GWT 加密备份/严格原钥恢复及冷恢复的有限范围结果保留；闭环写操作不重放。
- Core12/GWT6 的独立只读恢复资料 RPC 与登录分支已修复，既有 303 个服务端/共享/RPC policy 条目保持原接受边界；本轮没有再执行旧写验收。
- 独立 PostgreSQL 合成 schema9 库的 103 条历史密文任务，真实 dump/restore、9→26 迁移、rekey 与旧备份再次恢复验证通过，17 条 UNKNOWN 保持；这不是客户备份或完整 GUI 数据库恢复验收。
- 新增多代密钥读取、新密文认证代号、严格凭据校验和认证后才返回明文的流；解密输出的暂存文件只保存密文。已在一次合成隔离Core中完成发布/独立重启；三个保留网关及客户数据未换钥，旧钥未退役。具体合同见 [key-generation-contract.md](key-generation-contract.md)。
- 此前core检查点 `2282426e4782093aa978b0ada7231823dd35082f` 已修复安装器删除代模板的问题：模板移到持久 `share/key-management/`；发行资源排除宿主Python缓存。195项原生检查、2种权限配置的真实systemd/JDK21锁交接，以及1项实际Core安装/停机/换钥/独立重启闭环通过。真实Crypto、配置审计facade及快照reader已参与验证；旧密文原字节保留且重启后可解密，新密文代号匹配新钥。当时ARM64包43项载荷核对通过，333项非缓存载荷与实际运行过的29890a9包完全一致，仅删除5项宿主缓存。这是原Core闭环的历史范围；新增委托候选通过上条列出的独立检查，没有重放原闭环，也不把旧包一致性结论套在新改动上。详见 [key-generation-contract.md](key-generation-contract.md)。
- 离线和Java受控reader支持显式多代指纹绑定，修复logical-V1形似密文的合法密码恢复值错误；70测试及macOS/Linux实际打包JAR的原范围保留。Core保管下的agent委托已有上条候选，完整运行时准入/cohort/正式单元及GUI仍为内部pending；不得让agent直接读取Root原钥或把原UNKNOWN换ID重做。
- 最新 JDK21 加解密 46 项、配置/快照边界 27 项、CI 入口 5 项通过；两个故意退回旧行为的反例分别产生预期断言失败。`kura.snapshots.encrypt=false` 继续写明文 XML，`kura_custom.properties` 的覆盖语义不变。
- JDK 最低要求是 **21，允许更高版本**，编译目标保持 Java21，GWT 保持既有 Java11 字节码目标。CI 候选入口已去除“恰好21”限制。用户要求后不再追加27验证；当时已经发起的27进程已终态。个人 `~/.mavenrc` 仍默认21，没有改动；旧的仅设置 JAVA_HOME=27 的 Maven 记录实际为21，不得误用。更高版本全平台兼容不据此宣布完成。

## 保留现场及禁止重发

运行现场在 Docker 自身数据层/卷中，见 [runtime-preservation.json](runtime-preservation.json)。三个网关、七个进程、38 个私有文件完整元数据和六个旧数据文件内容在收尾前均精确。保留必要 live bind 输入及密钥/加密数据。不要用旧脚本重建、重启或重放现场。

| 现场 | 保留的进程 |
| --- | --- |
| `yofc-g034-key-service-lifecycle-20261115` | Core10430 / key10576 |
| `yofc-g034-key-cold-install-20261115` | Core5174 / key4735 / recovery4893 |
| `yofc-g034-rpc-isolation-20261115` | Core3792 / key4229 |

必须保留的原 UNKNOWN：管理操作 `767863a9-fe6b-4135-9783-ddcb131030af`；startup.stop 意图 `a87d94d52839e316845b7c831b265daf928f0bfd5fb0a883fb49b721cba5f63c`。只读原记录；不重发、不换 ID、不手动 stop、不重导出，不从 PID、耗时、空日志或新环境推断结算。

以下闭环 ID 永远只读：`965a1c8b-5acb-40ce-854e-b5592df80239`、`75d62ca7-cec5-47de-b136-9415e59dc226`、`fd78f9ad-d41d-4651-98bd-7a664f167255`、`5a7dfd2d-51bf-4431-bb67-7995643b0c4b`、`8fee5e2b-343e-4d71-b09d-7388af348ed9`、`f4115360-1598-4e2d-98ff-29978639936c`、`57d19ee1-a8e8-4fc6-8dba-36cd4cc6bcd9`、`10296cdc-4a25-46e9-b4b4-8993fb31bac6`。新 RPC 现场的空 journal 不能结算旧现场原记录。 本轮新隔离Core闭环的4个原ID也只保留历史：preparation `2d77ed82-8dd7-453c-88f8-cd1806d66393`、verification `6ef48b02-7d5c-430e-9162-871783efce9d`、publication `ab04d2bc-be23-4ff0-85f1-6a6abe03000e`、startup `4bbdae4a-df31-4296-a280-dcd5f1dd7332`。该一次性夹具已删除，不恢复/重发，不在其他Root状态中查询来推断结果。V36v1 不做 live query/inverse/reissue，既有 GWT 三次保存等旧 CLOSED 验收不重放。

新增实际EBUSY/原发布续接场景也已清理，仅保留原3个已结算ID：prepare `7a0e15dd-2de9-4dba-a2b7-653d2348b47c`、verify `8109c566-5ea9-416a-9943-f528aaeae3e3`、publish `54634dff-3e8d-420a-92ba-87710631f789`。原发布仅分发一次；故障仍在时的本地续接被阻止，经只读核对原状态、解除同一夹具故障后完成剩余文件。没有重发Stop/Start，也没有用新ID绕过。不要重放整轮来替换首次自动化失败的记录，或在其他Root中查询这些ID来推断结果。

新增native交接身份场景已清理：prepare `93d3a62f-4f02-4c51-a02f-7710946e1213`、verify `9b241ea6-af70-4981-ac19-c2b29598b94a` 已结算；handoff `c4d6f1cd-c728-4ee1-8eca-059bf12573c8` 仅签发测试授权。预选publication `c3486dcc-a89a-43bc-8280-e664afd40c93`、start `cd6f4621-8415-4011-9478-adf510b4a5fa` 从未分发，不存在相应操作记录。不要重建或在其他Root查询来推断结果。

实际900秒首次启动续期场景也已闭环并删除专用环境：handoffId `160c7873-7e4a-4d89-bdb8-bf0ab9097d76`、preparationId `180b21c4-be8c-47f9-a2f8-9097b3ca1b46`、publicationId `9fc45ff6-ac19-4b3a-8f02-27f76a52e5f6`、startOperationId `d2f39ad9-39b2-4f82-b425-ef5d670385ac`、verificationId `e2f4f6f2-d2f3-4928-a764-c2e33e06c0c9`，续期操作 `061d187d-8c17-4702-9df1-fd716c228010`。只保留原结果历史，不重建、重发或在其他Root状态查询来推断结果。

## 从干净环境继续

1. 读取工作空间 AGENTS.md、工作空间知识库最新交接和本目录台账；使用官方 Obsidian CLI，仍为父工作空间，不建重复 vault。
2. 核对 18 仓当前 Git 分支/提交、源码与保留的实际运行状态。Git 是源码权威，旧大型证据链不再是启动前提。
3. 按需重新构建缓存与制品；保留个人 `~/.kura-dev` 的现用配置、密钥与数据，以及正在运行的 `target/runtime`。2026-11-16 已按用户授权删除确认停用的 migration 调试目录和旧日志；不需要还原。旧现场只做必要原记录读取。不要重放已闭环验收来“恢复证据”。
4. 先做源码全盘 review，继续完整台账。需要重构时可从 WIP Git 检查点回退并重做；不能把未验收项标为完成或降为 deferred。

本次历史规模、报告计数和封签仅用于说明边界，不要求恢复同名旧日志。后续验证直接以新 Git 源码、实际环境及相应需求为依据。

## 构建目录控制

后续直接从 Git 检查点继续。隔离源码使用 Git worktree 或明确只含源码的导出，禁止递归复制旧 `target/`、`.phase*-retained`、`node_modules`、Maven/Cargo 缓存和历史运行库到新工作区。相同用途复用一套构建输出；阶段结束及时清理确认停用的编译缓存与运行库。按用户最新要求，确认可丢弃的调试文件直接删除，不为它们另建备份；真实密钥、加密数据、未确认个人数据、实际运行输入和原 UNKNOWN 记录按需保留。源码以本地 Git 保存，不再依赖旧证据链启动任务。

界面随相关功能逐步应用 `modern-web-guidance`；按用户最新要求，不把全站样式/组件改造作为主任务。GWT已有原预留核对/关闭，独立HTTPS页也已接原预留的严格原授权关闭和浏览器显式确认；使用forms和dialog指南，已补严格匹配私有启动与冷恢复意图/提前pending的原未准入结算；Root INTAKE、破损/缺命名空间和旧目录的合法恢复、实际安装Main/GWT联合验收及完整授权材料流程仍pending。它属于密钥主线，备份＋口令的原钥恢复权限未扩大。

## 2026-11-16 空间清理结果

本轮实测工作空间约483.49 GB，清理后父目录约96.39 GB；按磁盘空闲读数，本轮净增加约306.02 GB，可用约424.96 GB。目录统计会重复计算 APFS 共享区段，不能把387.10 GB的目录下降量等同于实际物理释放量。详见 [cleanup-status.json](cleanup-status.json)。已删除确认停用的重复运行库、Cargo/Maven/Equinox缓存、旧构建输出及确认可丢弃的备份/证据副本，没有为本轮废弃物重新备份。

Docker从170个容器减为6个，删除132个无用卷、保留17个数据卷和必要场景；个人 `.kura-dev` 删除3个停用迁移调试目录及日志，保留25个现用/迁移前真实文件。3个当前网关的7个进程、38个私有文件元数据和6份旧数据内容保持。另有60份来源尚未确认的旧卷档案约1.48 GB暂留，不能误称为已确认废弃物。

剩余96.39 GB仍包含未完成审查的候选副本、原UNKNOWN上下文、现用安装输入和旧数据，并非全是纯源码，也不意味着必须长期保留。后续逐项确认与Git/现场的关系后直接删除无用副本，不依赖旧target证据链恢复任务。用户最新要求：清理完成后继续目标，不在本阶段自行暂停。
