## 2026-11-26 OPC UA 与 Position 秒级时间戳后继同代换版

补齐此前未入场的两笔同代换版，均改用已入场 API2 配对（基线为 `wip/api2-admission-20261125` tip：OPC UA `62c5514-1`、Position `284058a-1`，Core `c65c047` 七包组合与 `api2-opcua-purge` 一致）。两个 API2 tip 均无后继，各自新建本地分支 cherry-pick 时间戳修复构建后继（`4f2fbd1`、`462c8df`，均未推送）。

**OPC UA** `2.0.0+api2~git20261126090823.4f2fbd1-1`：VM `yofc-opcua-tsfix-20261126`（Root `204:257`）unpack 返回 0 无降级警告，一次 configure 后 installed、audit 空，注册表 `204:22970` 与 SHA `5f9a669f…` 保持，合法 Main（PID 802）探针确认 `org.eclipse.kura.driver.opcua.provider 2.0.0.202611260908` ACTIVE×251，VM 停放。

**Position** 首次构建嵌入过期缓存 spi（API1 comm `[1.0,2.0)`），现场 Main 拒绝（证据 stale-spi-attempt-0912/）。恢复配对 spi（与旧包内嵌逐字节一致）重建为 `2.0.0+api2~git20261126091631.462c8df-1`：VM `yofc-position-tsfix-20261126`（Root `193:257`）unpack 返回 0 无降级警告，一次 configure 后 installed、audit 空，注册表 `193:22968` 与 SHA `cfef5eee…` 保持，合法 Main（PID 831）探针确认 `org.eclipse.kura.linux.position 2.0.0.202611260916` ACTIVE×249，VM 停放。

补父项 #3/#6/#12。两轮无业务派发，构建 target 已清理。完整目标 active，冻结 **0/12**。外部延期仅真实 NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。

[OPC UA 交接](../../../../.management-close-20261116/core-rtc-hotplug-20261120/opcua-tsfix-samegen-20261126/HANDOFF.md) · [Position 交接](../../../../.management-close-20261116/core-rtc-hotplug-20261120/position-tsfix-samegen-20261126/HANDOFF.md)

## 2026-11-26 秒级 Networking 后继包实装验证与错误 lineage 修正

上节"尚未执行"已完成。e4ae919 属 heyoulin-kura 主线而非 firewall lineage 后继（与 98ce414 的 merge-base 为 81534f1），实装被 Core cohort 围栏正确拒绝（Generation-reader package is not paired，证据 wrong-lineage-e4ae919-refusal/）；误用 c65c047 基线的首现场同样被拒（baseline-c65c047-refusal/），两现场删除留证。修正为 15ebcda + cherry-pick = 8f921f8（本地分支 wip/net-tsfix-samegen-20261126，未推送），构建 3.0.0~git20261126084746.8f921f8-1（SHA-256 94fb00f2…，已归档）。

隔离 ARM64 VM yofc-net-tsfix-20261126（Root 203:257，machine-id SHA b37e9294…）六包首装（旧 Networking 98ce41482f，Core 6341906）后：unpack 返回 0 无降级警告（dpkg 真实判定为升级），一次 configure 后 installed 新版、audit 空，注册表 203:22941 与 SHA de4cff48… 保持，合法 Main（PID 3436）探针确认 org.eclipse.kura.nm 2.0.0.202611260847 ACTIVE、bundle 245，探针撤下，Main/Agent disabled，VM 正常停放。前两次观察因旧探针 jar 硬编码路径未落盘，记录保留。清理：现场构建 target 约 44MB 删除，worktree 保留。本轮无业务派发。补父项 #3/#12。完整目标 active，冻结 **0/12**。外部延期仅真实 NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。

[本轮交接](../../../../.management-close-20261116/core-rtc-hotplug-20261120/debian-snapshot-timestamp-20261126/HANDOFF.md)

## 2026-11-26 秒级 Networking 后继包构建与版本比较

Networking e4ae919 按 Git/JDK21 构建为 3.0.0~git20261126081718.e4ae919-1，SHA-256 308ff7aa18dd6eba8b6ea84fa64be6b6d1155ac835c33a589cec1e9c22b0b4fb。Debian dpkg 确认它大于 14 位旧版 20261125044831.98ce41482f 和 12 位旧版 202611251924.15ebcda。OrbStack VM 启动等待超时，正式 unpack/configure 尚未执行；原现场未动。

完整目标 active，冻结 **0/12**。外部延期仅真实 NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。

[构建回执](../../../../.management-close-20261116/core-rtc-hotplug-20261120/debian-snapshot-timestamp-20261126/build-receipt.json)

## 2026-11-26 Debian 发行快照改为秒级时间戳

Networking 98ce414 到 15ebcda 的同代替换已停放，不重跑。dpkg 把后继判成降级，因为 12 位 `202611251924` 的数值小于历史 14 位 `20261125044831`。各 sibling 的 debugBuild 发行快照改为 `yyyyMMddHHmmss`，与 Core 发行包一致。OSGi 的 `kura.build.qualifier` 仍是 12 位，本轮不改 bundle 限定符宽度。定向测试 2 项通过。新包尚未实装。本地提交未推送。

补父项 #3/#12。完整目标 active，冻结 **0/12**。外部延期仅真实 NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。

## 2026-11-26 Networking 同代直接后继换版

隔离 ARM64 VM yofc-net-samegen-20261126，ID 01M8BCDBHMTT8S0RGGMR84DWBT，Root 192:257。Core 6341906 上首装 Networking 98ce414，Git 直接后继 15ebcda 一次 unpack 返回 0。dpkg 警告这是降级，因为后继时间戳 202611251924 的数值小于前版 20261125044831。正常关机后新 boot 只 configure 一次，返回 0，installed 3.0.0~git202611251924.15ebcda-1，审计空。注册表 inode 192:22941 和 SHA-256 de4cff488db2c46543d27bfe8a0697562c2ed02d35e3d1dd1a8abf6b838466cd 保持。合法 Main PID 933 只读确认 org.eclipse.kura.nm 2.0.0.202611251924 ACTIVE，bundle 数 245。探针撤下，Main/Agent disabled/inactive，VM 正常停放。业务派发 0。

补父项 #3/#6/#12。完整目标 active，冻结 **0/12**。外部延期仅真实 NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。

[Networking 交接](../../../../.management-close-20261116/core-rtc-hotplug-20261120/networking-same-generation-upgrade-20261126/HANDOFF.md)

## 2026-11-26 Triton 同代直接后继换版与确认不用的缓存清理

隔离 ARM64 VM yofc-triton-samegen-20261126，ID 01M8BB6BGSY628KECK8GMBJSPD，Root 191:257。API2 Core c65c047 不 Breaks Triton。旧包 1b6d15b 一次首装后，直接后继 bea8871 一次 unpack 返回 0。正常关机后新 boot 只 configure 一次，返回 0，installed 2.0.0~git202611211325.bea8871-1，审计空。注册表 inode 191:22964 和 SHA-256 fef68eb9955076dafc321e3b4dae1278af8a6a925fed9c4adf2f5ab2d7241896 保持。合法 Main PID 824 只读确认 org.eclipse.kura.ai.triton.server 2.0.0.202611201423 ACTIVE，bundle 数 246。探针撤下，Main/Agent disabled/inactive，VM 正常停放。业务派发 0。

同日已闭环但此前未入官方台账的两笔不重跑：Deployment Root 142:257 后继 8da586b，Main PID 1534，deployment.agent ACTIVE；Artemis Root 145:257 后继 1bcdc5d，Main PID 1749，artemis.xml ACTIVE。两台均已停放。

确认不用项只删除无挂载容器 unruffled_beaver，并 prune 未使用 Docker 构建缓存。60 台 VM、115 个数据卷、个人 MariaDB/Redis、证据容器和镜像保留。docker builder prune 报告 5.66GB。宿主可用空间从 158496616448 增至 163672588288，实际增加 5175971840 字节。

补父项 #3/#12。完整目标 active，冻结 **0/12**。外部延期仅真实 NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。

[Triton 交接](../../../../.management-close-20261116/core-rtc-hotplug-20261120/triton-same-generation-upgrade-20261126/HANDOFF.md)

## 2026-11-26 正式恢复输入跨Core独立purge与原围栏冷续配

前轮3624摘要cold通过后，复用独立ARM64 Root141:257、VM yofc-core-published-upgrade-20261126、ID01M8B2WK54GHEFX4ZDDDGJY2BF，新增VM为0。正式Git/JDK21七包保持，生产源码改动0。准入核对264文件、59份Core HMAC、17份凭据/history和原binding完整stamp。

复制真实23712字节factory snapshot到Root私有目录，经已装Core正式Root UDS及只读FD预检。唯一rec-3a502d0d-0895-4979-93d6-90d2e0eb5fac只执行一次正式stage，返回RECOVERY_INPUT_STAGED。原receipt.json、snapshot和源副本双侧归档。模式plain、策略legacy，恢复应用请求0。

五个依赖各一次remove，Core一次remove返回0到config-files。原data inode25472停放并保持原文件完整stamp，当前packages无完成回执，正式钩子复制dpa.properties至retained。原维护fb6a4f6e-2ff6-48c0-a57b-cacccd48a090及packageTag HMAC配对本Root。正常关机cold后，第二boot沿原围栏一次独立purge返回0，再次正常关机cold。第三boot同Root同原围栏一次unpack/configure正式结算，五依赖一次联合重装，七包installed且audit空。

三个阶段的恢复输入原件、回执和源副本完整stamp保持。重装后仅同Root同rec号original只读返回原STAGED回执。新增19份Core HMAC，累计78份含READY/configured。17份凭据/history及binding inode22731完整stamp保持，原published launcher/drop-in字节恢复。6份应用文件在维护前与依赖移除后各双侧归档，共12份应用归档及19份retained归档保留。

合法Main4443读取原legacy和发布代密文，以原活动钥写读新KURA2，10provider/JAR、Cloud transport、原client/job/name/requester/publisher作用域和GET原1412601的NOT_FOUND通过。原COMPLETED回执继续从历史双侧归档冷读，实际EXEC lifetime仍为1。本轮业务派发0、key mutation0，原四个key UUID仅只读SUCCEEDED。UID995、NNP1、五cap集合0x400、真实FD9 writer租约及HTTPS302通过。Main正常stop并PIDFD退出，Main/Agent disabled/inactive，VM正常停放。最终293保护文件、78份Core HMAC和74份companion/provider资产与控制纳入cold。

准备阶段all-clean断言遇到三个仓库九份Finder元数据，逐份记录并保留。首次Agent setup inventory在stage前返回FileNotFoundError，后继使用已装Core正式Root验证协议完成预检。Core remove成功后继承观察器误要求无完成回执的packages也进入holder，依据正式capture_history/retained_trees/park源码改为独立只读核验真实原围栏和复制原件，包动作重放0。原错误、日志和内容寻址控制器保留。

复用封存JDK21探针，新增编译及字节码缓存0。本轮可删除构建产物0，目录减少0，宿主可用空间独立计量见cleanup.json。18份既有输出按原消费者保留，18主仓按真实初态复核，完整57台身份/状态及原三网关running保持。原Root1048585/1048587/1048588的不可变cold执行，原452文件/154终态/103HMAC及既有业务闭环保持。所有原现场、个人配置/密钥/数据、运行集、六组件/rtc0和7fd最后disabled、正常停机、41输入16回执封存保留。本地台账提交，推送0，父vault双笔记官方CLI追加并精确全文读回。

补父项#1/#3/#9/#12，覆盖已发布代、七包共装下plain/legacy恢复输入跨Core remove、独立purge与原围栏冷续配保全。六组件自身实际换版、更多进程中断点、其他恢复模式、完整typed brokers/provider/launcher与十二父项继续内部待验。Networking managed model初始化和真实接管链仍内部待验。完整目标active，冻结 **0/12**。外部延期仅真实NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。

[本轮正式恢复输入保全及停机入口](../../../../.management-close-20261116/core-rtc-hotplug-20261120/core-staged-recovery-input-maintenance-20261126/HANDOFF.md)。

## 2026-11-26 发布代携带完成Deployment历史的Core独立purge与原围栏冷续配

前轮3370摘要cold通过后，复用独立Debian13 ARM64 Root141:257、VM yofc-core-published-upgrade-20261126、ID01M8B2WK54GHEFX4ZDDDGJY2BF，新增VM为0。正式Git/JDK21 Core6341906、Networking15ebcda、Deployment8da586b、Cloud329d420及同批UI/Wires/Agent七包输入保持，生产源码改动0。本轮新增业务派发0、key mutation0，原业务1412601和四个key UUID保持历史闭环且仅按原号读取。

准入核对原251保护文件、40份Core HMAC、17份凭据/history及binding完整stamp。10份user/data/packages原件在维护前和五个依赖移除后各双侧归档。五个依赖各一次remove返回0，Core一次remove返回0到config-files，原data inode22487、packages inode21242及其原文件完整stamp保持。原维护800bae04-70bd-43d8-90b1-24310fe89ab7及packageTag HMAC绑定本Root和原目录。末尾观察器误要求已删vendor unit的UnitFileState为disabled而退出1，原Core动作已成功。修正后独立只读观察要求空UnitFileState、vendor缺席、PID0/inactive，完整核验原围栏/记录和27份retained双侧归档，包动作重放0。

正常停机cold通过后，新boot沿原Root、原维护号和原围栏一次独立purge返回0，正式清除该合成夹具应用树及产品完成历史。captured/completed两份HMAC配对原publication6f2e56ca、三个launcher及drop-in cohort，17份凭据/history、原40份Core签名和Root binding保持，全部应用原件双侧保留。第二次正常停机cold通过后，第三boot同Root原围栏一次unpack/configure返回0，正式结算原维护号，五个依赖一次联合重装，七包installed且audit空。新增19份Core签名，累计59份含READY/configured，原published launcher/drop-in字节恢复。

后继合法Main4440通过原legacy及发布代密文读取，以原活动钥写读新KURA2。10provider ACTIVE且JAR逐项对正式包，实际UID995/NNP1/五cap集合0x400/真实FD9共享writer租约及HTTPS302通过。GET-only观察器逐项对原client E6:5F:9F:FC:F5:D8、job/name/requester/publisher，GET原1412601返回NOT_FOUND且Felix原业务缺席，原COMPLETED和returned-phase完成凭证从双侧归档读取。原四个key UUID只读SUCCEEDED，原实际EXEC lifetime仍为1。Main正常stop并PIDFD确认退出，观察bundle及runtime按原归档后退役。

最终264保护文件、59份Core HMAC、74份companion/provider资产与控制脚本、20份应用归档副本及27份retained归档停机cold通过。17份凭据/history及binding inode22731完整stamp保持。Main/Agent disabled/inactive，VM正常停放。完整57台身份/状态保持，原三网关running，18主仓台账前与初态一致，原Root1048585/1048587/1048588的不可变cold全部执行，原452文件/154终态/103HMAC及既有真实换版/默认完成历史purge保持。

初始host准备调用macOS缺席的dpkg-deb，在guest产品动作前失败，改用ar/tarfile继续。首次cold误在returned-phase原凭证查找COMPLETED文本，已修正为分别校验原凭证字段和封存正式GET。三处观察/准备失败、原输出与源码均保留，实际业务和包动作没有因观察失败重派。

仅清理本轮1个观察类及3份正式classpath输入的4文件编译目录，逐文件对保留JAR/正式输入、lsof及VM挂载核验。目录分配减2322432字节，宿主同期可用增2318336字节，清理后157377024000字节。18份既有输出按原消费者保留，原现场、个人配置/密钥/数据、运行集、六组件/rtc0及原7fd最后disabled、正常停机、41输入16回执封存保持。本地台账提交，推送0，父vault双笔记经官方CLI追加并精确全文回读。

补父项#1/#3/#9/#12，本轮闭环发布代携带完成Deployment历史的Core独立purge和跨两个正常停机点的原围栏重装。六组件自身实际换版、更多进程中断点、完整typed brokers/provider/launcher及十二父项继续内部待验。Networking managed model初始化及真实接管链仍内部待验。Agent共装、remoteOFF且无旧恢复引用的重加密已在Root136及替换后中断Root137闭环，已有恢复引用等余下组合按原证据继续。完整目标active，冻结 **0/12**。外部延期仅真实NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。

## 2026-11-26 发布代和完成Deployment历史跨Core真实换版

前轮3105摘要cold通过后，新建独立Debian13 ARM64 Root141:257，VM yofc-core-published-upgrade-20261126，ID01M8B2WK54GHEFX4ZDDDGJY2BF。复用Git/JDK21 Core99ffac0→直接后继6341906、Networking15ebcda、Deployment8da586b、Cloud329d420及同批UI/Wires/Agent七包，生产源码改动0。两版generation manifest对象和provider输入相同，实际Debian/Git版本迁移已验证，其他manifest/ABI组合继续内部待验。

合法Main11385沿新现场唯一业务号1412601执行一次真实EXEC并完成。只读Main12105确认原COMPLETED、Felix DP/payload ACTIVE和实际Cloud transport，逐项对原回执client/job/name/requester/publisher。prepare1dc999e4、verifyd70e0956、publish6f2e56ca和explicit-start64abce90各一次且SUCCEEDED。发布正式排空Main12105并PIDFD确认，后继Main13050读取原密文、原业务及活动代KURA2。敏感原响应和口令仅留Root私有目录，四个原号后续只读。

10份原user/data/packages文件及metadata双侧归档，17份凭据/history、原启动器和registry建基线。一次Core unpack返回0，签名capture绑定Root、旧/新版本、新preinst和原data/packages身份。原维护7a167f17-bc43-42bf-9353-6d2c8ed35559保持。正常关机cold核验50保护文件、19份HMAC、原围栏和3105摘要。新boot同Root、同原围栏一次configure返回0，正式恢复发布代launcher并结算维护，新增21份HMAC含READY/configured，累计40份。

后继合法Main3154读回原密文和原发布代密文，使用原活动钥写读新KURA2，原1412601只读COMPLETED。10provider ACTIVE且JAR逐项对包，Cloud transport、原连接身份、UID995/NNP1/五cap集合0x400/真实FD9共享writer租约及HTTPS302通过。原data inode22487、packages inode21242及其文件完整stamp保持，17份凭据/history和binding inode22731完整stamp保持。原10文件字节/UID/GID/mode/mtime均保持，4份user文件经正式COPY改变inode/ctime，差异留档。三个发布代launcher和drop-in原字节恢复，registry原字节保持。

最终251保护文件、40份Core HMAC及74份companion/provider资产和控制脚本停机cold核验，七包installed、audit空。Main/Agent disabled/inactive，VM正常停放。完整57台准入后执行原不可变Root1048585/1048587/1048588 cold，原452文件/154终态/103HMAC、前轮真实换版及完成历史purge保持。首版builder局部变量重名及只读适配argv误传的失败均保留，分别在实装和产品操作前修正，实际包动作和业务各一次。

补父项#1/#3/#9/#12，完整目标active，冻结 **0/12**。发布代携带完成历史的Core purge、六组件自身实际换版、更多中断点及完整typed brokers/provider/launcher矩阵继续内部待验。Networking未初始化managed model的FW/NM警告保留，真实网络接管链继续内部待验。外部延期仍仅真实NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。

仅清理本轮3个观察类、3个正式classpath输入及fixture.dp的7文件编译目录，逐文件对保留JAR/正式输入、lsof及VM挂载核验。目录分配减2347008字节，宿主同期可用增2334720字节，清理后157392654336字节。18份既有输出按消费者保留，18主仓台账前保持，原56台身份/状态及原三网关running保持。所有原现场、个人配置/密钥/数据、运行集、六组件/rtc0和原7fd封存保留。本地台账提交，推送0，父vault双笔记官方CLI全文精确回读。

## 2026-11-26 独立Core带完成业务历史的purge与原围栏冷续配

前轮2742摘要联合cold通过后，补矩阵新建Debian13 ARM64 Root1048588:257，VM yofc-core-history-purge-20261126，ID01M8B1FAENWET0FKVTK7HW3SMX。复用封存Git/JDK21 Core6341906、Networking15ebcda、Deployment8da586b、Cloud329d420及同批UI/Wires/Agent七包，生产源码改动0。

原业务号10485882601唯一一次实际EXEC完成，独立同Root GET确认COMPLETED，Felix DP/payload ACTIVE、Cloud transport及10provider/JAR配对。首版探针在EXEC前失败，原日志、JAR/源、go及/tmp载荷保留。原线程终止、两个严格先于doExec的持久标记均缺席及发布/完成回执缺席，构成同Root正式非准入依据。正常stop/PIDFD退出后补齐listener导入，沿原号和原host意图继续首次实际EXEC，随后仅GET，原失败保持留档。

10份user/data/packages原文件及metadata双侧归档。移除五个Core依赖包后一次Core remove返回0，原data inode22487、packages inode21242及完成回执完整stamp保持。正式remove记录的packageTag验签，原维护5ccc483a-b898-4e1c-8842-723610ad5809绑定本Root和两个目录。正常关机cold通过58保护文件及19份HMAC。

第二boot沿原Root/原围栏一次独立purge返回0，正式删除此新现场的应用树与完成历史，双侧原件及4份canonical/journal凭据和Root binding保持。再次正常关机cold后，第三boot同Root、同原围栏一次unpack/configure完成正式结算，恢复七包installed、audit空。新增19份签名，累计38份含READY/configured。

后继合法Main4418解回原密文，10provider ACTIVE且JAR对包，UID995/NNP1/五cap集合0x400/真实FD9 writer租约和HTTPS302通过。补充只读Main5152从实际DataTransportService读取clientId，逐项对原完成回执的client/job/name/requester/publisher，GET原号返回NOT_FOUND，Felix原业务缺席。原号维持历史闭环，原COMPLETED从双侧归档冷读。两个Main均正常stop并PIDFD退出。

最终213保护文件、38份HMAC、79份配对companion/provider资产及控制脚本、原10份应用原件和前Core remove归档各双侧核验。原4份凭据及kernel-fh-v2 binding inode22731完整stamp保持。Main/Agent disabled/inactive，VM正常停放。原55台身份/状态保持，原三网关running，18主仓台账前与初态一致。只读准入核验完整56台后执行原不可变Root1048585和1048587 cold，原452文件/154终态/103HMAC及原完成历史、前轮真实换版保持。

仅清理本轮5个观察类及3份输入等9文件的编译目录，先逐类对保留JAR、输入对正式包、lsof无占用及VM挂载核验。目录分配减2367488字节，宿主同期可用增2359296字节，清理后158788354048字节。18份既有输出按实际消费者保留。所有原现场、个人配置/密钥/数据、运行集、六组件/rtc0和原7fd封存保留。本地台账提交，推送0，父vault双笔记经官方CLI追加及精确全文回读。

补父项#3/#9/#12，本轮闭环独立默认Core凭据模式、六组件加Cloud共装时的完成历史purge及跨remove/purge两个正常停机点续配。发布代携带业务历史的purge、实际换版、更多中断点和完整typed brokers/六组件/provider/launcher矩阵继续内部待验。Networking初始managed model未初始化的FW/NM警告已留档，网络接管矩阵继续内部待验。完整目标active，冻结 **0/12**。外部延期仍仅真实NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。

## 2026-11-26 独立Core真实Git换版与原维护号冷续配

前轮2563摘要cold通过后，新建独立Debian13 ARM64 Root1048587:257，正式安装Git99ffac0包，再一次dpkg --unpack升级到直接后继6341906。两个真实Debian版本、控制脚本、初始化CLI、配对manifest和相同provider/JAR均预先对原Git包核验，JDK21观察bundle编译通过。完整目标active，冻结0/12。

初始合法Main9879经真实CryptoService/ConfigurationService产生独立样本后正常stop并PIDFD退出。7份原应用文件双侧完整归档，凭据与journal key共4份建完整stamp基线。正式preinst签发原维护9a2c8aae-e485-46c4-9c93-21dc2339f6b8，签名capture绑定Root、旧/新版本及新preinst摘要。解包后正常关机，中间cold核验34文件、19份Core签名及原capture/围栏。

同Root新boot仅一次configure返回0，正式结算原围栏并产生21份新签名，累计40份含READY/configured。原7文件字节/UID/GID/mode/mtime保持，5份经正式COPY改变inode和ctime，data inode22487保持，packages目录inode21242→22986据实记录。原4份凭据及kernel-fh-v2 binding inode22732完整stamp保持。初始化固定CLI在旧版返回INITIALIZED，新版沿同域返回EXISTS，原Scope3994f2043ec1913100345e4a3cea0c41d630326563c592a8ab63f527864dca34保持。

后继合法Main3076解回原密文，两个Core provider ACTIVE且JAR对包，实际UID995/NNP1/五cap集合0x400/FD9 writer租约通过，随后正常stop且PIDFD确认退出。143保护文件、40份Core HMAC、原签名capture、两包四控制/完整manifest/units/native cohort及控制器私有源快照已停机cold通过。Core installed、audit空，Main disabled/inactive、VM正常停放。显式key/FW/Deployment任务均0。

创建新VM首次等候镜像索引约九分钟，原create随后成功。原Root1048585的旧cold直接执行因全局清单54→55数量断言退出，失败保留。INVENTORY-ADMISSION.md及只读适配先核验完整55台身份/状态，再执行原Root全部检查，452保护文件、154终态、103HMAC及原完成业务历史通过。原检查器/回执保持。18主仓台账前与初态一致，原54台身份/状态保持，原三网关running。所有原现场、个人配置/密钥/数据、运行集、六组件/rtc0及7fd最后disabled、正常停机、41输入16回执封存保留。

仅清理对原包/JAR且lsof确认闲置的本轮probe编译目录，目录分配减2215936字节，宿主同期可用增2195456字节。18份既有构建输出按原消费者保留，全部原包/源码/档案/控制器保留。本地台账提交，推送0。

补父项#3/#9/#12，覆盖独立Core默认凭据模式及两个provider的99ff→634真实换版和unpacked后正常停机续配。发布代换版、六组件实际换版、完成Deployment历史的Core purge、更多中断点及完整typed brokers/provider/launcher矩阵继续内部待验。完整目标active，冻结 **0/12**。外部延期仍仅真实NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。

## 2026-11-26 完成Deployment历史跨独立purge与原门禁冷续配

固定ARM64 Root1048585:257、原machine-id及kernel-fh-v2 Scope，先通过前轮2383摘要cold。复用Git Core6341906、Networking15ebcda、Deployment8da586b和原七包/JDK21探针。原业务10485852601只读GET，本轮显式FW apply/inverse及业务派发均0。

合法Main973读原COMPLETED/Felix DP/payload/真实Cloud transport、原密文和10provider后正常stop且PIDFD确认。已装Deployment四控制及11载荷对原包。实际dpkg remove一次返回0到config-files，独立purge一次返回0到not-installed，原data inode22539、packages inode25555及完成回执/归档/Job原件保持，FileMove原remove围栏跨purge保持。

同包一次unpack返回0，产品原Main transition锚点90442139525a420596b7f8fa54a56b64和generator inode30367保持，真实Main启动被断言拒绝。正常关机中间cold通过417文件、146终态、103HMAC及原transition/FileMove围栏。新boot同Root核验原记录完整stamp，再次真实断言拒绝后，一次configure返回0并正式结算原门禁和FileMove围栏。后继合法Main1497 GET原业务仍COMPLETED，原payload/transport、原密文/快照、10provider/JAR、kurad/NNP1/五cap集合0x400/FD9租约/HTTPS302通过。

418份原基线中417份完整stamp保持。注册表原字节归档，正式remove删除deployment行，configure标准追加到末尾，最终顺序变化据实记录。两boot自动新增16组（4 APPLIED/12 UNCHANGED），最终154终态（38 APPLIED/116 UNCHANGED）、452保护文件及103份Core HMAC停机cold通过。首次两次只读prepare误用归档权限和kurad主组而失败，均在Main/包动作前。后继按正式Core代码权限root/kura-key-control、0750/0640核验，原失败及全部cohort保留。

Main/Agent disabled/inactive，VM正常停放。新编译/字节码缓存0，18份既有构建输出按原消费者保留，删除0、目录减少0。宿主同期可用变化-16384字节。18主仓台账前保持，54台VM身份/状态恢复初态，原三网关running。所有原现场、个人配置/密钥/数据、运行集、六组件/rtc0及7fd最后disabled、正常停机、41输入16回执封存保持。

补#3/#6/#9/#12，完整目标active，冻结 **0/12**。本轮补齐当前v2组合的Deployment完成历史独立purge。Core带完成业务历史的purge、实际版本转换、更多中断点及完整typed brokers/六组件/provider/launcher矩阵继续内部待验。外部延期仍仅真实NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。

## 2026-11-26 原IPv6规则实际报文与联合冷启动

固定ARM64 Root1048585:257及原kernel-fh-v2 Scope，前轮2259摘要cold通过后，仅一次正式firewall restore及合法Main925。原IPv6号1f17114d-09cf-4e80-bf62-c8bd4796c4ff、IPv4号886f19b4和Deployment10485852601均已闭环，本轮显式apply/inverse与业务派发均0。七份Git包和JDK21探针复用。

原IPv6 UDP49528/source fd7a:d0c0:128::2/128/interface yfd0，经自有无默认路由netns/veth完成正常DAD及ND。两笔唯一载荷分别得到指定源ECHO、其他源TIMEOUT，私有尾部drop计数增1，server仅收到指定源载荷。原接受规则没有counter，该计数不可观测。首服务tentative时bind失败、原脚本要求接受计数1以及只读观察器state语法差异三处失败全部留档。后继只继续缺失服务与只读核对，额外报文0。

Main925读原密文、原IPv6快照，READY、10provider/JAR、kurad/NNP1/五cap集合0x400/FD9租约/HTTPS302联合通过。GET-only只读原业务10485852601为COMPLETED，原Felix DP/payload及真实Cloud transport ACTIVE。原401文件及130组终态完整stamp保持，自动启动新增8组，为2 APPLIED/6 UNCHANGED。最终418保护文件、138终态（34 APPLIED/104 UNCHANGED）及103份Core HMAC停机cold通过，完整manifest/units/控制/native cohort/Cloud JAR和各执行源码配对。

自有进程PIDFD确认退出，私有drop/table/netns回收，原nft结构及默认路由保持。Main/Agent disabled/inactive，VM正常停放。18份既有构建输出按原消费者保留，新编译/字节码缓存0，删除0、目录减少0，宿主同区间可用变化+65536字节。18主仓台账前保持，54台VM身份/状态恢复初态，原三网关running。全部原现场、个人配置/密钥/数据、运行集、六组件/rtc0及7fd最后disabled、正常停机、41输入16回执封存保持。

补#3/#6/#9/#12，完整目标active，冻结 **0/12**。实际版本转换、更多中断点、完成Deployment历史purge及完整typed brokers/六组件/provider/launcher矩阵继续内部待验。外部延期仍仅真实NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。

## 2026-11-26 IPv6真实未决发布的包维护拒绝与原号结算

固定隔离ARM64 Root1048585:257，先通过该Root前轮1832摘要/373文件/117终态/103HMAC停机cold，继承独立Core purge轮2072摘要。复用Git Core6341906、Networking15ebcda、native0.1.17及原七包/JDK21探针。13份原user/data/packages文件完整归档，原IPv4模型及Scope保持。

本boot合法Main924经原ConfigurationService只触发一次IPv6 UDP49528/source fd7a:d0c0:128::2/128/interface yfd0。正式worker完成1条真实Netlink命令和ACK、sync原staged后，在rename入口中断。原号1f17114d-09cf-4e80-bf62-c8bd4796c4ff、原staged inode29915及原sender/意图四字段配对。真实dpkg remove、purge kura-networking各一次，由已装prerm返回1，日志均为原发布pending。393保护文件、同Main/Invocation、内核规则及七包实际installed保持，dpkg desired选择变化据实记录。

一次正式outcome沿同Root/Scope/原号发布原staged inode和精确原字节为APPLIED，trace无Netlink写。后继终态查询无Netlink及rename。恢复install选择后，原Main正常stop且PIDFD退出，同boot后继合法Main2452经已装IPv6构造器结算原sender并发布原candidate。原密文、READY、10provider/JAR、kurad/NNP1/五cap集合0x400/FD9租约/HTTPS302通过。

原ConfigurationService调用记录Provider read deadline expired，原failure保持。早期观察器在成功续接后比较内部bootModel，以及恢复后压缩IPv6字符串匹配的两次断言失败均留档。后继只读完成对外字段合同和IPv6网络地址类型核对，包动作、任务和Main启动各一次。所有执行cohort按SHA配对到Root私有0400快照。

最终130全终态为32 APPLIED/98 UNCHANGED，原117组完整stamp保持，新增12组自动启动为2 APPLIED/10 UNCHANGED。401保护文件中旧373基线仅正式IPv6模型改变，其余372份stamp保持。旧IPv6原文由真实sender.previous完整留存，摘要对原基线。103份Core HMAC及完整manifest/units/控制/native cohort/10JAR/应用归档停机cold通过。Main/Agent disabled/inactive、VM正常停放。显式配置1、inverse0，IPv6报文数据面继续待验。

仅清理32字节自测输出宿主重复副本，guest原观察副本保留。目录分配减4096字节，宿主同区间可用变化-4096字节。18主仓台账前保持，54台VM身份/状态恢复初态，原三网关running。个人配置/密钥/数据、原现场、运行集、六组件/rtc0及7fd最后disabled、正常停机、41输入16回执封存保持。

补#3/#6/#9/#12，完整目标active，冻结 **0/12**。实际版本转换、更多中断点、完成Deployment历史purge及完整typed brokers/六组件/provider/launcher矩阵继续内部待验。外部延期仍仅真实NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。

## 2026-11-26 独立 Core 已发布代跨 remove、purge 与原围栏冷重装

前轮1832摘要cold先通过。按完整矩阵新建独立Debian13 ARM64 Root1048586:257，仅正式安装Git Core6341906，JDK21。初始合法Main9885经真实CryptoService/ConfigurationService产生原密文。prepare77b3aafc、verify967f32b1、publish6f1534ec、explicit-start3ebb6bbb各一次，真实SUCCEEDED。发布代Main10660解回原密文，发布原文及敏感响应留Root私有目录。

新现场七份应用文件在维护前完整归档，17份canonical凭据/history建基线。Main运行时一次dpkg remove返回0，PIDFD确认退出，Core为config-files。原维护32a1fe66-0fe8-4c18-82f9-6fd31e215508、firstAction=remove及retained签名保留。正常停机cold后，第二boot沿同Root/围栏一次独立purge返回0，正式captured/completed HMAC对绑定原publication、retained receipt及三个launcher/drop-in cohort。应用HOME/retained实际清除，原应用归档及17份凭据/history完整stamp保持。

再次正常停机cold后，第三boot沿同Root/原围栏一次unpack及configure均0，Core installed且audit空，真实完成purge签名驱动原发布代恢复。后继合法Main3092解回原密文，两个Core provider ACTIVE且JAR对包，三个实际JVM的kurad/NNP1/五cap集合0x400/FD9租约通过。四个原key UUID仅只读SUCCEEDED。三份launcher及generation drop-in字节与原签名cohort保持。

观测修正完整留档，涉及镜像缺Python、重复JAR扫描、默认enabled/inactive、purge签名目录及平台全局drop-in。平台manager NoNewPrivileges=no，正式exec-java以prctl设置NNP，实际三个JVM均NNP1。所有失败控制器、issued/returned及日志保留，包动作及任务各一次。最终79保护文件、38份Core持久HMAC及另存purge签名对cold通过，Main disabled/inactive，VM正常停放。显式key任务4、FW0、Deployment0。

仅清理配对原包且lsof无占用的本轮build，目录分配减2215936字节，宿主同期可用增2211840字节。18主仓台账前保持，既有53台VM身份/状态保持，唯一新增VM已stopped。原三网关running，Root1048585及所有原现场、个人配置/密钥/数据、运行集、六组件/rtc0及7fd最后disabled、正常停机、41输入16回执封存保持。

补父项#1/#3/#9/#12，目标active，冻结 **0/12**。本轮覆盖独立Core两个provider。实际版本转换、更多purge中断点、完成Deployment历史purge、IPv6真实未决中断及完整typed brokers/六组件/provider/launcher矩阵继续内部待验。外部延期仅真实NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。

## 2026-11-26 原完成 Deployment 历史跨 Core remove 与原围栏冷重装

固定隔离 ARM64 Root1048585:257，保留原 kernel-fh-v2 绑定及七份 Git 正式包。原 FW886f19b4、业务10485852601均已结算，本轮显式业务派发0，仅 GET 原业务号。实际 APT 模拟及 installed 控制配对后，Networking、Deployment、Wires、GWT、Cloud 各一次正式 remove，Networking prerm 停止合法 Main 且 PIDFD 确认退出。一次 Core remove 返回0，进入 config-files，原维护号 d87f0c17-a1ce-410b-86ef-c102956ecf60、签名 removal-home 和 Main-state 原件保留。

原 data inode22539、packages inode25555 经正式 holder 停放，318文件原始基线及移除态359文件归档保留。移除后正常停机 cold 通过。新 boot 先验同 Root/Scope/原围栏及原文件，再沿该围栏一次 unpack、一次 configure，均返回0。只读 inotify 捕获产品临时 package-history-handoff 原件及签名，HMAC、captureTag 与原 packages 身份核对通过。累计103份持久 Core phase HMAC，含21份新增及 READY/configured。前轮将临时 package-history 签名计入持久 phase 组的 prose 错误在本轮单独澄清，原封存证据保持。

五依赖各一次正式 install 恢复原注册顺序。Cloud 包配置追加36字节，为4个LF及 kura.have.cloud.connection=true 加LF。原888字节是现924字节的精确完整前缀，原末值与新增值均true，UID/GID/mode保持。其余317份基线文件字节及属主/模式保持，data/packages目录和原发布文件完整stamp保持。差异及原文件留档，无手工改写custom。七份文件随正式复制/重建更换inode，custom/registry/drop-in三份mtime变化均记录。

后继合法 Main4699 以 GET-only JAR 读原10485852601为COMPLETED，Felix原DP1.0.0、payload及真实Cloud transport ACTIVE。原密文/49528快照、10provider/JAR、UIDkurad/NNP1/五cap集合0x400/FD9 writer租约/HTTPS302联合通过。维护前、依赖移除后、Core removed、恢复后四阶段实测原UDP规则指定源ECHO、其他源TIMEOUT。两boot新增16组自动启动记录，为4 APPLIED/12 UNCHANGED，最终117终态（29 APPLIED/88 UNCHANGED），原101组保持，显式FW apply/inverse均0。

原观察器的custom全文件相等断言、共享模块导入及首版native runner语法失败均保留。后继执行源以完整SHA配对到Root私有0400快照，只推进未发起阶段，全部包动作和任务均未重跑。最终停机cold核验373保护文件、117终态、103持久HMAC及另存的真实临时签名、完整manifest/units/控制/native cohort、Cloud八JAR和七包installed，audit空。Main/Agent disabled/inactive，VM正常停放，自有报文资源及探针磁盘载荷按归属收回留档。

本轮复用正式包，无新构建或字节码缓存可清，删除0、目录减少0。宿主计量区间可用变化+151552字节，末次162557583360字节，单列观测。18主仓台账前保持，其他VM身份/状态与初始一致，原三网关running。原现场、个人配置/密钥/数据、运行集、六组件/rtc0及7fd最后disabled、正常停机、41输入16回执封存保持。

补#3/#6/#9/#12，完整目标active，冻结 **0/12**。Core purge、实际换版、IPv6真实未决中断及完整typed brokers/六组件/provider/launcher矩阵继续内部待验。外部延期仅真实NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。

[Core remove retained-history evidence](../../../../.management-close-20261116/core-rtc-hotplug-20261120/firewall-deployment-history-remove-20261126/HANDOFF.md)

## 2026-11-26 原完成 Deployment 历史跨 Core 正式冷续配

固定隔离ARM64 Root1048585:257、原machine-id和kernel-fh-v2绑定保持。原真实未决防火墙号886f19b4已经结算，本轮沿原模型被动验证。补齐Git329d420的正式Cloud API2包后，原生七包组合中的真实Deployment handler经JDK21探针仅派发一次新测试号10485852601，Felix安装fixture.corehistory.1048585 1.0.0，产品完成通知、归档、注册表和.done均实际生成，无pending。

251份完整维护前基线覆盖user/data/packages及原文件。运行Main时一次Core6341906正式unpack返回0、PIDFD证明Main退出。原维护号3dd1059f-c7b7-4f1c-b82e-5b49d14076fe的签名capture声明原data及packageHistoryDirectory。283文件/93终态/61份Core签名中间cold通过后，同Root新boot沿同围栏一次configure返回0，新增21份签名含READY/configured，累计82份。

后继合法Main3365以独立GET-only JAR只读原业务号COMPLETED，Felix原DP及payload bundle ACTIVE，实际Cloud MQTT transport provider ACTIVE且JAR对包。data inode22539、packages inode25555保持，原归档/完成回执/注册表/Job文件的inode、摘要、属主、模式保持。251份原文件的字节/UID/GID/mode保持，其中HOME下14份维护前mtime保持，六份配置/keystore/快照文件随正式复制更换inode。

原密文、49528快照、10provider/JAR、kurad/NNP1/五cap集合0x400/FD9共享writer租约/HTTPS302联合通过。维护前、unpacked、恢复后三阶段原UDP规则实测指定源ECHO、其他源TIMEOUT。两boot及一次前置Main重启新增22组自动启动记录，4APPLIED/18UNCHANGED，最终101终态（25APPLIED/76UNCHANGED），原79组保持，显式FW apply/inverse均0。

初次Cloud前置误选ABI版本标记更新前归档，被Core Breaks拒绝配置，后继使用正式+api2包完成标准升级。初版unpack观察遗漏ctimeNs字段及cold使用Cloud逻辑路径的错误均保留，修正观察后只继续未执行阶段。一次业务派发、一次Core解包、一次原围栏configure均有原始日志。无补造成功回执。

最终cold核验318保护文件、101终态、82份HMAC、完整manifest/units/控制/native cohort和Cloud八JAR、七包installed且audit空。Main/Agent disabled/inactive，VM正常停放。清除已对原包/JAR且lsof确认闲置的本轮编译输出，目录分配减2338816字节，宿主同区间可用增2322432字节。18主仓台账前状态相同，其他VM状态与原三网关running保持。原现场/个人配置/密钥/数据/运行集/六组件/rtc0及7fd最后disabled、正常停机、41输入16回执封存保持。

补#3/#6/#9/#12，目标active，冻结 **0/12**。实际换版、完成Deployment历史的Core remove/purge、IPv6真实未决中断及完整typed brokers/六组件/provider/launcher矩阵继续内部待验。外部延期仅真实NM/cloud-init/物理交接、amd64、磁盘满/硬断电。

[原完成历史、正式围栏和最新停机入口](../../../../.management-close-20261116/core-rtc-hotplug-20261120/firewall-deployment-history-core-20261126/HANDOFF.md)

## 2026-11-26 已结算防火墙跨 Core remove 与原数据冷重装

固定 ARM64 Root1048585:257，复用 Git Core6341906、Networking15ebcda 和 native0.1.17。前轮620hash cold通过，实际依赖核验四包后，Networking/Deployment/Wires/GWT 的移除作为 Core remove 前置。合法 Main945 经正式 Networking prerm 同步停止，PIDFD确认退出。一次 Core remove 返回0并进入config-files，正式原维护号235ad522-4ce2-44ff-abf6-8cef8c2708bc、removal-home 和 Main-state 原件保留。

192份维护前完整基线覆盖 user/data/packages 与原191文件，原data目录按正式holder停放，inode22539保持。228文件/71终态/原40份Core签名中间cold通过后，同Root新boot沿原remove围栏一次unpack和一次configure均返回0，新增21份正式签名含READY/configured及retained-data交接，累计61份。四依赖按原注册顺序各安装一次，六包installed，audit空。

后继合法 Main4543 读回原密文、49528快照，10provider/JAR对包、UID/NNP1/五cap集合0x400、FD9 writer租约和HTTPS302通过。维护前、依赖移除后、Core removed、恢复后四阶段原UDP规则实测指定源ECHO、其他源TIMEOUT。192份原文件最终字节/UID/GID/mode全部匹配，data身份保持，packages按正式COPY从inode24034到25555，dpa.properties逐文件原字节保持。custom/registry/Networking drop-in的正式原子重建使inode和mtime更新，差异完整留档。

两boot新增16组正式自动启动回执，4APPLIED/12UNCHANGED，最终79终态（21APPLIED/58UNCHANGED），原63组精确保持。显式业务配置和撤回任务均0，原886f19b4及8a9ab5ee保持封存。Core移除期间身份观察使用同包原ELF仅检查当前Root内核身份。初版观察器遗漏包属drop-in正常删除及共享脚本首次导入SyntaxError均原样留档，后继脚本经12份宿主/guest编译与摘要配对，只继续尚未派发阶段。

247保护文件、61份Core签名、完整manifest/units/控制/native cohort及10JAR已最终停机cold核验。Main/Agent disabled/inactive，VM正常停放。复用正式包，无新构建/字节码缓存可清，删除0，目录减少0、同区间宿主可用变化0，末次162594590720字节。18主仓台账前保持，原三网关仍running，其他原VM和Core219/Deployment225等原场景保持。个人配置/密钥/数据、运行集、六组件、rtc0及7fd最后disabled/正常停机/41输入16回执封存保留。

补父项#3/#6/#9/#12，完整目标active，冻结 **0/12**。本轮完成Core remove的保留数据冷重装。Core purge、实际换版、完成Deployment历史的完整维护、IPv6真实未决中断和完整typed brokers/六组件/provider/launcher矩阵继续内部待验。外部延期仍仅真实NM/cloud-init/物理交接、amd64、磁盘满/硬断电。

[原号冷重装与最新停机入口](../../../../.management-close-20261116/core-rtc-hotplug-20261120/firewall-core-remove-retained-20261126/HANDOFF.md)

## 2026-11-26 已结算防火墙跨 Core 同包解包与原维护号冷续配

固定隔离ARM64 Root1048585:257，在前轮516hash/47终态/136文件基础上，复用Git Core6341906、Networking15eb和native0.1.17。合法Main913实际运行时，一次dpkg --unpack同版Core634包返回0，PIDFD确认Main退出。正式preinst生成原维护号8a9ab5ee-d53c-4e7e-87fb-03f411b20920、Root私有围栏及签名capture，原136文件在正式.old-home或原外部路径按inode/摘要精确核对。55终态及156文件中间cold通过后，同Root新boot只一次configure返回0。

新维护号有21份真实签名含READY/configured和runtime-data交接，原19份保持，累计40份HMAC。后继合法Main3386读回原密文和原快照，10provider/JAR与正式包配对，UID/NNP1/五cap集合0x400、FD9 writer租约和HTTPS302通过。原规则UDP49528在维护前、解包后和冷续配后三阶段实际验证指定源回包、其他源超时。原47组原生意图/回执和模型/绑定精确，两个boot新增16组正式启动记录，最终63终态（17APPLIED/46UNCHANGED）。显式配置及撤回任务均0，原886f19b4未重发。

原data目录inode22539保持。六份HOME文件含custom、两份keystore、两份快照和registry随正式复制更换inode，字节/属主/模式及mtime保持。控制器初次把packages目录也要求inode连续，configure成功后在观察阶段退出。原签名capture仅声明runtimeDataDirectory，源码仅对已有完成Deployment发布哨兵声明packageHistoryDirectory，当前按正式COPY路径使packages inode21303→24034、UID/GID/mode保持。原失败/源码/补充依据保留，只继续未派发的Main。本控制器未为packages建立新的维护前逐文件基线，该目录逐文件保持与完整已有Deployment历史的Core维护仍按矩阵待验。

最终停机cold核验191保护文件、63终态及原47组、40份Core签名、完整manifest/units/Core与Networking四控制/native cohort、10JAR和六包installed，audit空。自有UDP/site/netns按归属回收，Main/Agent disabled/inactive、VM正常停放。当前最新入口为firewall-core-unpack-20261126/cold.py。前轮Networking及真实pending的cold入口均转为各自历史状态。

本轮复用Git包，无新构建/字节码缓存可清，删除0，真实目录减少0、宿主同区间可用变化0，末次162657546240字节。18主仓台账前保持，原三网关仍running，Core219、Deployment225/57225、137和旧229保持停放。个人配置/密钥/数据、运行集、六组件夹具、rtc0及原7fd最后disabled/正常停机/41输入16回执封存保持。

补父项#3/#6/#9/#12，完整目标active，冻结 **0/12**。本轮为同版本dpkg upgrade路径及unpack后正常停机续配。实际换版、Core remove/purge、其他中断点以及全部typed brokers、六组件、provider/launcher完整矩阵继续内部待办。外部延期仍仅真实NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。

[同包解包、原围栏冷续配与最新停机入口](../../../../.management-close-20261116/core-rtc-hotplug-20261120/firewall-core-unpack-20261126/HANDOFF.md)

## 2026-11-26 已结算防火墙跨 Networking remove、purge 与冷续配

固定隔离 ARM64 Root1048585:257 先通过前轮398hash cold，两次复机均核验原 machine-id、kernel-fh-v2 绑定及原文件。沿原 Git Core6341906、Networking15ebcda和native0.1.17，仅使用被动probe，显式配置/撤回任务均0。原号886f19b4-e32c-48b8-a795-88eceb090342及原31组终态的意图、回执、模型、绑定和sender原文归档保持。

合法 Main912 实际运行时一次 dpkg remove 成功，原控制器的PIDFD确认该Main退出，Networking进入config-files。独立purge成功到not-installed，同一正式包一次unpack后正常关机。中间停机cold核对114保护文件、39终态及19份Core签名。新boot沿同Root一次configure成功，原capability token正式退役，custom原字节/属主/模式恢复，唯一networking注册项和配对vendor drop-in恢复。custom原子编辑inode23025→23438→23578，三个Networking provider JAR重装inode更换，原摘要/属主/模式保持。

只建本VM虚拟yfd0/yfd1与无默认路由namespace，复用原UDP49528规则。维护前、remove后、purge后、unpack后及冷续配后五阶段均实测指定源198.18.28.2回包、其他源198.18.28.3超时，原模型/回执和site规则保持。自有UDP服务、site规则和netns均按归属回收。原Main停止后，观察器曾把IPv6路由expires倒计时误作配置变化，原失败与源码留档；只修正只读规范化，再继续尚未签发的purge/unpack，没有重跑remove。

冷续配后的合法Main1121读回原密文和快照，10个provider/JAR对包、UID/NNP1/五cap集合0x400、FD9 writer租约及HTTPS302通过。两boot正式启动新增16组记录，为4 APPLIED/12 UNCHANGED，最终47组全部终态（13 APPLIED/34 UNCHANGED），原31组完整元数据保持。136保护文件、19份Core HMAC含READY、完整manifest/units/四控制/native cohort及六包installed已停机cold核对，audit空。Main/Agent disabled/inactive，VM正常停放。当前冷读入口为firewall-networking-retained-20261126/cold.py。

本轮复用Git正式包，无新增构建/字节码缓存可清，清理复核删除0，真实目录减少0、宿主同区间可用变化0，末次162664779776字节。原始含审计写入的计量另行保留。18主仓台账前与原状态一致，原三网关仍running，Core219、Deployment225/57225、137和旧229保持停放。个人配置/密钥/数据、运行集、六组件夹具、rtc0及原7fd最后disabled/正常停机/41输入16回执留档保留。

补父项#3/#6/#9/#12，目标active，冻结 **0/12**。本轮正常停机边界为Networking unpack后冷续配。Core维护、IPv6真实未决中断及其他typed brokers、六组件、provider/launcher完整矩阵继续内部待办。外部延期仍仅真实NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。

[原生维护、报文与最新停机冷读](../../../../.management-close-20261116/core-rtc-hotplug-20261120/firewall-networking-retained-20261126/HANDOFF.md)

## 2026-11-26 真实未决防火墙发布的原生拒绝与原号结算

原隔离ARM64机器复机先核验Root仍1048585:257及原kernel-fh-v2绑定。沿已装Git Core6341906、Networking15ebcda和配对native0.1.17，正式Main924经原ConfigurationService只触发一次IPv4 UDP49528。worker1849实际完成1条Netlink命令并生成真实ACK/staged后，在结果rename入口中断，原号886f19b4-e32c-48b8-a795-88eceb090342及sender pending保留。

真实dpkg remove、purge kura-networking均由installed prerm退出1，日志为原发布pending。81保护文件及原staged/意图/sender保持，Main924和InvocationID保持active。dpkg desired选择变为deinstall/purge的实际记录保留，随后只恢复install选择，六包实际installed保持。

正式worker沿同Root/Scope/原号/摘要将原staged inode23295及原字节发布为APPLIED，trace无Netlink写。随后终态只读查询无Netlink/rename。原Main正常退出后，同boot合法Main2835经已装构造器只查询原pending号、恢复原candidate并完成sender。新增6个启动记录均UNCHANGED/0命令。10provider/JAR配对、原密文、writer租约、HTTPS302通过，最终31终态（9APPLIED/22UNCHANGED）。本轮无报文流量实测，原ConfigService读取超时及控制器比较断言错误均原样留档，未补写成功文件或重派任务。

Main/Agent disabled/inactive，VM正常停放。cold核验94保护文件、31原生记录、19份Core签名含READY、完整manifest/units/控制/native cohort和10JAR、六包installed。当前机器的最新冷读改用firewall-pending-maintenance-20261126/cold.py，前轮62文件/16记录入口属于历史。清理32字节可再生自测文件，真实目录分配减4096字节，宿主清理区间可用增4096字节，符号链接du的原计量错误和修正均留档。

原三网关继续运行，18主仓台账前保持，Core219、Deployment225/57225、137和旧229未重放。原7fd沿最后disabled、正常停机、41输入/16回执封存，个人配置/凭据/数据、运行集、六组件和rtc0保留。补#3/#6/#9/#12，完整12父项及typed brokers、六组件、provider/launcher全维护矩阵继续，目标active，冻结 **0/12**。独立config-files、其他中断点及平台仍按原范围待验，外部延期仍仅真实NM/cloud-init/物理交接、amd64、磁盘满/硬断电。

[真实未决、原号结算与最新冷读](../../../../.management-close-20261116/core-rtc-hotplug-20261120/firewall-pending-maintenance-20261126/HANDOFF.md)

## 2026-11-26 正式 Core/Networking 新 v2 Root 联合链

隔离Core6341906、Networking15ebcda按Git/JDK21构建并实装同一ARM64 Root1048585:257。Core固定新域CLI校验Core/actor资产和PID1配对idle Main，持dpkg/admission/exclusive writer围栏、继承FD3；运行/启动中Main不能借缺席锁被绕过。两包控制与helper锁定同一native0.1.17/F11归档。36项Linux定向通过，旧F11的207hash冷读保持，未推送。

Core19份HMAC含READY验核，Agent安装前正式初始化v2域，绑定跨后继五包APT保持。合法Main实际Crypto/配置及10provider/JAR与正式包配对，UID/NNP1/0x400/FD9和HTTPS302通过；运行Main时初始化入口实际拒绝且绑定保持。正确firewall引导后同Scope产生8组正式记录。正常停机冷启先核验原绑定与当前内核对象，再同Root原8号只读确认，43文件inode/摘要/属主/模式保持。冷Main读回原密文，10原provider/JAR保持；最后16组native全终态（6APPLIED/10UNCHANGED），原8号保持。device仍1048585，未计为重分配实证。

Main/Agent disabled/inactive、测试JAR移出后留档，VM正常停放，62保护文件、19签名、完整manifest/units/控制及10JAR可冷读。控制器的APT前/Main前及只读观察错误均原样留档，只继续未发起阶段，没有补造成功回执。仅清理确认闲置的本轮缓存和编译组装输出，目录减1,860,972,544字节，宿主清理区间可用增1,424,924,672字节；源码/归档/夹具/共享缓存保留。

本轮显式apply/inverse任务为0，真实未决FW下的原生remove/purge及中断仍待，不能用本轮全终态或前轮合成标记替代。原Core219、Deployment225/57225、137、229均未重放；18主仓台账前保持，原三网关仍运行，个人数据/运行集/六组件/rtc0及7fd封存保留。补#3/#6/#9/#12，完整目标active，冻结 **0/12**；12父项、typed brokers、六组件、provider/launcher全维护矩阵继续。外部延期仍仅真实NM/cloud-init/物理交接、amd64、磁盘满/硬断电。

[正式新域联合链及冷读](../../../../.management-close-20261116/core-rtc-hotplug-20261120/firewall-root-domain-core-20261126/HANDOFF.md)

## 2026-11-26 新现场持久 Root 域 native 候选

隔离Agent工作树本地f11d66f生成0.1.17 source crate，标准Cargo归档的VCS及全部Git源码核对，复用同一ARM64缓存。新格式只在空历史、固定Core FD3排他writer围栏下显式初始化，绑定当前Root/publication内核对象；原v1公式、opaque wire和已签发意图保持，不接受调用方Root或旧号。原暂存字节以RENAME_NOREPLACE发布，普通构造器在操作路径前检查未完/外来绑定；独立Python包维护门禁不依赖worker。

Git归档完整15项crate回归和11项ARM64合成文件系统场景通过，含实际renameat2 EINTR与原暂存inode续接、共享租约拒绝、非空v1拒绝及worker缺席的remove/purge helper门禁。没有真实业务派发或成功回执。新原机146:257正常停机重启后Scope/绑定/合成哨兵保持；整机克隆保留machine-id和inode但内核身份不同，无操作号的scope/inspect及独立门禁均拒绝，trace未读哨兵。两台已正常停放并cold核对。原机device未变，不计为同源重分配实证；OverlayFS显式拒绝且未写marker，持久支持仍内部待验。

正式Core/Networking仍是原pin，本轮未实装新格式。下一步另开源码与全新Root，完成正式入口、配对包及Core/OSGi、READY、合法Main/provider和真实回执联合链。原229不追认、不迁移、不跨Root查询；Core219和Deployment225已闭环场景不重跑。旧e155观察夹具冷读保持，原18仓除台账外17仓保持，原三网关仍运行，个人数据/运行集/六组件/rtc0和原7fd封存记录保留。

仅清理确认闲置的本轮构建副本，目录减少1,122,304字节，宿主清理区间实际可用增加1,146,880字节；源码、归档、ELF、夹具与共享缓存保留。首轮FD3 harness、tmpfs模式和完整workspace离线打包失败均留档，后继验证通过。补#3/#6/#9/#12，完整目标active，冻结 **0/12**。其余12父项、typed brokers、六组件、provider/launcher维护矩阵继续；外部延期仍仅真实NM/cloud-init/物理交接、amd64、磁盘满/硬断电。

[新格式、合成边界与停机冷读](../../../../.management-close-20261116/core-rtc-hotplug-20261120/firewall-root-domain-v2-20261126/HANDOFF.md)

## 2026-11-26 RootScope 持久身份审查与只读观察候选

正式 worker 的v1 Scope绑定machine-id及Root/publication的device/inode，原号比较在操作路径读取前。Root229复机变78会改变真实Scope，仍禁止查询原号、重派或换号。原现场保持停放；当前观测不能追认旧意图的授权。

Agent隔离工作树本地 `e155065` 增加 --inspect-root-identity，只持目录锁读取Root和publication FD的filesystem type/fsid/不透明句柄，不读操作文件、不派发Netlink；legacy-v1 Scope公式和wire保持，输出authorizesMigration=false，不接受调用方Root或句柄。15项完整crate回归通过，按Git构建与测试源码及ELF一致；复用同一ARM64缓存。正式Core包仍沿原worker，持久Scope协议尚未启用。

新建两台无Kura业务的隔离ARM64元数据夹具。克隆保持同一machine-id、Root inode257及私有控制目录inode16076，内核文件系统身份不同；原机正常停机重启后观察保持。两台另建的publication目录与独立C探针一致，cap清空、NNP1；合成mode000哨兵证明观察不读操作文件、错Scope在哨兵前拒绝。它不是真实待结算回执，业务派发0。Docker OverlayFS的C和native观察均明确不支持、无替代身份。本次正常启动未复现device重分配，不能记成该边界已验，也不影响原三网关以制造条件。

两台均正常停放并cold复核，原始观察、合成哨兵、源码和ELF保留；清除确认闲置、可从Git和原锁文件重建的测试副本，目录计量及清理区间宿主可用均改善282,624字节。原三网关、六组件夹具、原219/223/224/225/137、个人数据和共享rtc0保留，7fd沿用最后disabled/正常停机/41输入16回执封存。

补#3/#6/#9/#12的有限源代码与内核证据。完整12父项及typed brokers、六组件、provider/launcher矩阵继续，目标active，冻结 **0/12**。下一步是版本化持久Scope的正式合同、未知/克隆/对象替换/不支持边界以及配对Core/Networking打包与Core/OSGi、READY、合法Main/provider联合实装，不能把观察工具记作续接修复。OverlayFS覆盖仍内部pending；外部延期仅真实NM/cloud-init/物理交接、amd64、磁盘满/硬断电。

[源代码、观察边界与冷读交接](../../../../.management-close-20261116/core-rtc-hotplug-20261120/root-scope-persistence-20261126/HANDOFF.md)

## 2026-11-26 有效 typed firewall 跨 Core API1→API2 联合升级

独立 Debian 13 ARM64 Root `143:257` 用保留的 Git/JDK21 正式包建立六组件 API1 基线，再实装 Core `18380316cc-2`、Deployment `8da586b`、GWT `9f226c0`、Wires `f0a95c6` 的四包 API2 联合升级。Networking `98ce414`、Agent `f78610a` 保持。维护号 `b75f24fc-5c43-4b9c-a776-72089b1018ba` 有 21 份新签名含 READY/configured，原 19 份保持，总 40 份 HMAC 冷验通过。合法 Main 的 UID/NNP/0x400/FD9 writer 租约、九个实际 JAR 与包、完整 manifest 与控制脚本已核对，HTTPS302，六包 installed，audit 空。

实际 ConfigurationService 各发布一次双栈新用途 UDP49526 放行，原号 `340c8f43-164b-438f-be70-37580475c7e7`、`8fd400c0-b408-4f87-975e-89f1f535f0ce` 只有一次 apply。探针误判 IPv6 压缩/展开文本后停 Main，保留原 failure 及缺失结果文件；同 Root 原号 native outcome 均为 APPLIED，实际批准源 ECHO、其他源 TIMEOUT。按正式依据只继续未派发的维护和撤回。新 Main 恢复原快照 `1795627476348`，有效模型文件身份和正反向流量跨升级保持；两个新撤回各 APPLIED，四源全拒绝，模型内容恢复基线，总 24 组意图/结果齐全，原十号保持。

首次 APT 的 --no-download 本地路径解析在解包前明确返回100，原包及签名组保持；随后标准绝对路径 APT 实装成功。本轮升级从 inactive/disabled Main 开始，不计为包维护停止运行中 Main 的因果证据。原四份用户/安全/初始快照文件在包复制后 inode 更换，字节/属主/模式/mtime 保持；原数据目录在配置完成时核验保持。收尾即时 veth 检查失败后仅补只读观察确认已消失，业务步骤没有重跑。Main/Agent disabled/inactive，VM 正常停放，清理后冷读一致。

原机器 `229:257` 复机预检发现实际 Root 为 `78:257`，机器 ID 和 machine-id 摘要保持。已在业务前停止，零业务派发、零原号查询，正常停放后原 22 组记录及冷读覆盖的 8 份升级签名保持。持久 Root 身份与正式续接依据仍内部 pending；停机字节审计不能替代运行态 Root 准入，不迁移旧回执或重标 Root。新143没有借用原密钥、配置、意图或回执。

仅清理确认闲置的重复别名和编译输出，目录计量减少 738,529,280 字节，宿主清理区间实际可用增加 363,655,168 字节；95份未配对的OS缓存包保留。原三网关、其他原现场及原7fd封存记录保持。补 #3、#4、#6、#9、#12，完整目标 active，冻结 **0/12**；其余 typed brokers、六组件、provider/launcher 与12父项全矩阵继续内部待办，外部延期仍仅真实 NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。

[正式联合链、失败边界和停机冷读](../../../../.management-close-20261116/core-rtc-hotplug-20261120/api2-firewall-live-model-20261126/HANDOFF.md)

# 2026-11-24 冻结父项复核

## 2026-11-26 原发布代跨 Core 原生解包、冷续配与 provider 恢复

原 ARM64 Root `137:257` 将正式 Core 从 `a047e63a77` 原生升级至 Git `18380316cc` 的 `-2` 包。构建对已审查的 service 摘要对加入精确单向兼容关系，仍需实际原 manifest pin，其他 service 改动、逆向迁移及稳定 reader/unit/protocol 差异拒绝。111 项 ARM64 Linux/JDK 21 定向通过，按 Git 构建成功。原生 unpack 后旧 Main PIDFD 确认退出，14 份文件在实际暂存树保持；正常停机冷读后同 Root 新 boot 沿 `2f4990fc-f063-4970-ba5f-de7133698bc0` 一次 configure，新增 21 份签名含 READY/configured，旧 19 份保持，七包 installed，dpkg audit 空。

实装代码在 Main 首启前建立 Root 0600 空 writer 锁，后继合法 Main FD 9 持有同一 inode，维护独占申请被阻止。只读探针经真实 CryptoService 解出原 KURA2 样本，经原 ConfigurationService 恢复原快照；原证书 TLS 1.2 双向握手、两种 TLS 拒绝、错误 MQTT 口令拒绝及新主题 QoS 1 送达通过。原重加密及四个密钥号仅只读保持原 SUCCEEDED。包维护更换 site、snapshot_0、原业务快照三份文件 inode，字节/属主/模式/mtime 保持；原 intent/archive/completed 和凭据保持。Main/Agent disabled/inactive，VM 正常停放，40 份 Core HMAC 和保护文件停机核对。仅移除确认重复且无占用的候选输入硬链接，清理后读回与终态逐字节一致。

Root 137 最新入口改为本轮 `cold.py final`，旧 `api2-artemis-rewrap-interrupt` 的入口属于升级前历史，不能对当前 Root 重跑。18 仓盘点发现 Camel、Container、Wires 有未提交内容，本轮未修改或提交这些仓。补 #1、#3、#4、#9、#12，冻结 **0/12**，目标 active。其余 typed brokers、六组件、provider/launcher 及原 12 父项全矩阵继续内部待办；外部延期仅真实 NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。

[原生联合链、精确兼容边界与停机冷读](../../../../.management-close-20261116/core-rtc-hotplug-20261120/api2-artemis-core-upgrade-20261126/HANDOFF.md)


## 2026-11-26 Artemis 快照实际替换后中断与原号冷续办

隔离 Debian 13 ARM64 Root `137:257` 以原 Core `a047e63a77`、Agent `f78610a-1`、Artemis `1bcdc5d-1` 等七包在 JDK 21 下实装。正式重加密 `81095c06-4449-44ea-905d-68de5022ca69` 仅 apply 一次，实际快照替换成功后 SIGKILL，原暂存 inode 已发布、完成回执缺席。观察器 PIDFD 五秒等待失败后仅补观察，原签名、pending 和归档保持，正常停机冷读 UNKNOWN。首次新 boot 续办在准入阶段因 `/run/kura-key-service/writers` 缺失拒绝，trace 无 Java 和 rename，原号保持。

Core 本地 `d016e78cbe` 修复独占维护锁的冷启动初始化，108 项 ARM64 Linux/JDK 21 定向通过。Git 候选与原包的 50 份 Python 文件仅 service 文件不同。保留原安装包及 cohort，按正式续接依据使用候选兼容 CLI 沿同一 Root、同号结算 SUCCEEDED。零 Java 重加密、零快照替换，一次正式完成回执；原 snapshot/intent/archive inode 和摘要保持。两份 43,412 字节 XML 逐字节相同，新件仅凭新活动钥读取。合法 Main 用原证书和用户完成 TLS 1.2 双向认证及 MQTT QoS 1。原 Agent 状态、锁 inode、profile、远程 OFF 保持，19 份 Core 签名保持，七包 installed，Main/Agent disabled/inactive，VM 正常停放。清理后冷读与终态留档逐字节一致。

仅清除七份确认重复且无占用的包硬链接，目录计量减 390,250,496 字节；宿主清理区间可用变化 −40,960 字节，原证据与候选保留。本轮候选仅用于兼容 CLI 续办，正常 dpkg 升级、READY/Main/provider 联合实装仍待。补父项 #1、#3、#4、#9、#12，冻结 **0/12**。原 Deployment `225:257` 的 `57225` 已闭环证据保持，本轮不重放。其余 typed brokers、六组件与 provider/launcher 全矩阵继续内部待办，外部延期仅真实 NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。

[实际中断、Git 修复、正式续接与停机冷读](../../../../.management-close-20261116/core-rtc-hotplug-20261120/api2-artemis-rewrap-interrupt-20261125/HANDOFF.md)


## 2026-11-25 Deployment 225:257 定向复核

父项 #3、#9、#12。保持同 Root `225:257` 正常停机，只读核对原号 `57225` 的业务归档、终态回执、已安装载荷及同号 GET `COMPLETED` 留档。冷读输出逐字节一致，原生 remove 与 purge 的真实拒绝覆盖已安装包的 `prerm`。独立 config-files 的 `postrm` 仍使用原合成标记证据。冻结 **0/12**。 [停机冷读](../../../../.management-close-20261116/core-rtc-hotplug-20261120/three-generation-deployment-receipt-20261125/current-stopped.json)

## 2026-11-25 API2 Artemis 与运行中 Native Agent 的快照重加密

隔离 Debian 13 ARM64 Root `136:257` 实装 Git 构建的 Core `a047e63a77`、Agent `f78610a-1`、Artemis `1bcdc5d-1` 等七包，实际 JDK 21。Core 持 dpkg 后端锁、Agent 初始化锁及原 `system-change.lock`，校验正式 unit、运行进程、挂载视图并检查恢复引用。新签名意图绑定原 Agent profile、状态目录及锁 inode，保留独立 Core 旧意图续办。60 项 ARM64 合同测试通过。正式发布代的快照重加密 `faf94ff5-74f5-46a1-998d-d3fcfedc10f9` 返回 SUCCEEDED，Agent PID、invocation 和两项 inode 保持，原密文归档为 Root 0400。两份 43,412 字节 XML 逐字节一致，新件仅靠新活动钥读取。

Artemis 原生 remove、独立 purge、同包解包及正常停机后，同 Root 新 boot 一次 configure。合法 Main 通过原 TLS 1.2 双向证书及原 MQTT 用户表完成 QoS 1 送达，原四个密钥号和重加密号均只读回原终态。七包 installed，19 份 Core 签名保持，Main 与 Agent disabled/inactive，停机冷读在清理前后一致。本轮实装覆盖配对 Agent 运行、远程 OFF、无旧恢复引用的模式。已有恢复引用的拒绝和重加密中断续办由合同测试覆盖，仍待进一步原生故障验证。

Deployment 原 Root `225:257` 仅停机读回，原归档和终态回执摘要与留档一致。实测 `c012586-1` 的真实待结算发布使原生 remove、purge 均由 installed prerm 退出 1。同 Root 原号 `57225` 的只读 GET 为 COMPLETED。原包和回执保持，后继 `c012586-2` 的四份钩子、门禁 helper 和四个 JAR 字节一致。独立 config-files 的 postrm 拒绝证据使用合成标记。补父项 #1、#3、#4、#9、#12，冻结 **0/12**。外部延期仅真实 NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。

[正式实装、合同测试和停机冷读](../../../../.management-close-20261116/core-rtc-hotplug-20261120/api2-artemis-agent-rewrap-20261125/HANDOFF.md)

## 2026-11-25 typed reboot 跨 Core API1 → API2 联合升级

隔离 Debian ARM64 Root `1048593:257` 在 Git 发行 API1 Core `df5ae00aac` 上由 JDK 21 正式 OSGi Main 单次触发重启。首个新 boot 取得原号 `738c16f4-5178-4141-a330-4d1ee33d88ef` 的 ACCEPTED/MANAGER_ACCEPTED，原 intent、pending 和 actor 回执封存。OrbStack 命令通道恢复后在同 Root 的后续 boot，以正式 Main 只读结算同号，sender 落盘 completed 并清除 pending。

正式 APT 一次换版 Core `f8c61e4e27`、Deployment、GWT 和 Wires。旧 Deployment 的 FileMove `prerm` 退出 1，候选 `failed-upgrade` 接管。六包 installed，dpkg audit 空，原 19 份记录及新维护号 `aed2b23e-f432-4ee0-8b10-df4afc34973e` 的 21 份记录含 READY/configured 核对。后继正式 Main 实际四项 OSGi API 均为 2.0.0。原 actor intent、actor receipt、sender completed 的 inode、摘要保持，只有一笔原 actor 操作且无 pending。Main 和 Agent disabled/inactive，VM 正常停放，清理前后冷读一致。

Deployment 原 Root `225:257` 的原号 `57225` 再次停机读回与留档一致。真实 remove、purge 的拒绝覆盖已安装包的 prerm，同 Root 同号 GET COMPLETED；独立 config-files 的 postrm 证据使用合成标记。补父项 #3、#4、#9、#12，冻结 **0/12**。外部延期仅真实 NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。

[正式实装和停机冷读](../../../../.management-close-20261116/core-rtc-hotplug-20261120/reboot-api2-upgrade-20261125/HANDOFF.md)

## 2026-11-25 API2 Artemis XML 独立清除与原加密快照恢复

隔离 ARM64 Root `1048585:257` 正式实装 Core `c65c04741e` 和 Artemis `1bcdc5d-1`。JDK 21 合法 Main 只提交一次 XML broker 设置，原加密快照 `1795595635374` 保留用户表。首次探针在 QoS 1 读回后的监听字面断言处退出。只读观察核对 IPv4 映射回环地址和 Main 的 socket inode，原发布标记保持。

正式 remove、独立 purge 后，合法 Main 仍能读取原加密快照，Artemis XML bundle 和监听缺席。同包解包并正常停机，同 Root 新 boot 一次 configure。后继 Main 用原用户表拒绝错误口令，两个 Paho 客户端实际完成恢复主题 QoS 1。有效配置接线 1.9.0，同 JVM 的 message API 2.0.0。七包 installed，原快照和 19 份 Core 签名保持，Main 与 Agent disabled/inactive，VM 正常停放并重复冷读一致。

补父项 #1、#3、#4、#9、#12，冻结 **0/12**。Artemis 跨代换钥、TLS/其他协议及其余 typed brokers、六组件与 provider/launcher 矩阵继续内部待验。外部延期仅真实 NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。 [正式证据和冷读](../../../../.management-close-20261116/core-rtc-hotplug-20261120/api2-artemis-xml-purge-20261125/HANDOFF.md)

## 2026-11-25 API2 Cloud Raw MQTT 独立清除与跨停机恢复

隔离 ARM64 Root `1048584:257` 以正式 Core `c65c04741e` 和 Cloud `329d420-1` 实装，JDK 21。合法 Main 只创建一次 Raw MQTT 工厂，保留原工厂和 broker 设置快照。独立订阅端通过隔离 Mosquitto 实际收到一条 QoS 1 消息。原生 remove 到 config-files，独立 purge 到 not-installed。缺包时正式 Main 仍读得原快照，Raw MQTT bundle、endpoint 和工厂服务均缺席。

同包解包后正常停机，重复冷读一致。同 Root 新 boot 一次 configure，七包 installed 且 dpkg audit 为空。后继正式 Main 经 OSGi message API 2.0.0 接线，以原工厂和原 DataService、DataTransport PID 向恢复主题实际送达一条。两份原快照、原标记、19 份 Core 签名和两份凭据保持，站点属性精确保全。Main 和 Agent disabled/inactive，隔离 VM 正常停放，清理前后冷读一致。

Deployment Root `225:257` 的 `57225` 原生 installed prerm 对 remove、purge 的拒绝与同号 GET COMPLETED 证据再次冷读一致，原文件和回执保持。补父项 #3、#4、#9、#12，冻结 **0/12**。本轮覆盖 Raw MQTT 回环 QoS 1，其他 Cloud provider、TLS MQTT 及原矩阵继续内部待验。外部延期仅真实 NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。 [正式证据和停机冷读](../../../../.management-close-20261116/core-rtc-hotplug-20261120/api2-cloud-mqtt-purge-20261125/HANDOFF.md)

## 2026-11-25 API2 Camel 独立清除、注册锁超时与跨停机原路由恢复

隔离 ARM64 Root `1048582:257` 用正式 Core `c65c04741e` 和 API2 Camel `b999d44-1` 实装，JDK 21。合法 Main 只创建一次 XML 路由工厂 `fixture.api2.camel.retained`，原快照 `1795594257048`，Timer→File 实际送达一条。原生 remove 进入 config-files，独立 purge 进入 not-installed。Camel bundle 和工厂缺席时，合法 Main 保存新快照 `1795594289434`，六个配置字段保持，消息文件无新增。

同包解包后在实装注册锁上出现实际等待和约 30 秒超时，dpkg 为 half-configured。正常停机，两次冷读一致。新 boot 同 Root 核对原锁和注册表后只成功续配一次，八包 installed，dpkg audit 为空。正式 Main 恢复原路由，经实际 OSGi message API 2.0.0 接线送达第二条。两份快照、原提交、78 份 Core 签名和此前 OPC UA 的原快照及实装载荷保持，Main 和 Agent disabled/inactive，VM 正常停放。

补父项 #3、#4、#9、#12，冻结 **0/12**。本次覆盖 XML Timer→File 路由。其余 provider/launcher 和六组件矩阵继续内部待验。外部延期仅真实 NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。 [正式证据和冷读](../../../../.management-close-20261116/core-rtc-hotplug-20261120/api2-camel-purge-20261125/HANDOFF.md)

## 2026-11-25 API2 OPC UA 独立清除、跨停机续配与原工厂恢复

隔离 ARM64 Root `1048582:257` 的正式 Core `c65c04741e` 与 API2 OPC UA `62c5514-1` 在 JDK 21 下实装。合法 Main 只创建一次工厂 `fixture.api2.opcua.purge`，原快照 `1795593886179`。一次原生 remove 到 config-files，独立 purge 到 not-installed。缺包时正式 Main 的 ConfigurationService 能读原快照，provider bundle 与 Driver 服务缺席。

同包解包正常停机，冷读两次一致。新 boot 同 Root 一次 configure 恢复七包 installed。后继合法 Main 恢复原工厂，实际 OSGi channel 接线 API 2.0.0，经隔离 Milo 回环读取原产品名及更新后的服务器 UTC。原快照、两份提交标记、78 份 Core 签名、两份凭据及两个应用归档保持。Main 和 Agent disabled/inactive，隔离 VM 正常停放，清理前后冷读一致。

补父项 #3、#4、#9、#12，冻结 **0/12**。签名加密和订阅仍内部待验。外部延期仅真实 NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。 [正式证据和冷读](../../../../.management-close-20261116/core-rtc-hotplug-20261120/api2-opcua-purge-20261125/HANDOFF.md)

## 2026-11-25 执行器与 ownership 跨正式 Core 移除、独立 purge 和原号重装

隔离 ARM64 Root `1048582:257` 的正式 Core `c65c04741e` 真实 OSGi 执行器运行一条长命令时进行原生 remove。返回时原 Main、命令、执行器 worker、ownership reader 的四个 PIDFD 均确认退出，原回调为 `EXECUTOR_RESULT_UNAVAILABLE`，三个 socket 已撤下。旧命令仅实际派发一次。随后独立 purge、正式解包与正常停机，同 Root 新 boot 沿 `33b4116b-6983-46f2-b665-712be604df31` 一次续配。六包 installed，新增 19 份验签，59 份既有签名、两项凭据和原应用数据归档保持。 后继合法 Main 的正式 OSGi 执行器 `id -un` 返回 `kura`。维护前后 ownership 两个只读请求均返回缺席，同 UID 非 Main 均被拒。Main 与 Agent disabled/inactive，VM 正常停放，两次冷读一致。补 #3、#9、#12，冻结 **0/12**。外部延期仅原三项。 [正式证据](../../../../.management-close-20261116/core-rtc-hotplug-20261120/icmp-core-maintenance-20261125/HANDOFF-EXECUTOR-PURGE.md)

## 2026-11-25 typed ICMP 跨正式 Core 独立 purge 与同号重装

隔离 Debian ARM64 Root `1048582:257` 保持 Agent installed，按正式依赖顺序移除四个组件，Core `c65c04741e` 原生 remove 后独立 purge。实际 ICMP 单元和客户端缺席。新围栏 `a31a36df-5991-4fed-a374-9fe2338cb414` 跨 purge、原生解包及正常停机保持，原 40 份签名和两项凭据保留。新 boot 同 Root 一次续配，19 份新记录含 READY/configured 验签，六包恢复 installed。 正式 Core ConfigurationService ACTIVE，实际 Main 通过 writer lease 和双族 ICMP，同 UID 外部进程被拒。原应用数据 tar 保持，重装后逐项比对见证据。Main/Agent disabled/inactive，VM 正常停放，重复冷读一致。补父项 #3、#9、#12，冻结 **0/12**。外部延期仅原三项。 [正式证据](../../../../.management-close-20261116/core-rtc-hotplug-20261120/icmp-core-maintenance-20261125/HANDOFF-PURGE.md)

## 2026-11-25 typed ICMP 跨 Core 原生解包与新 boot 原号续配

隔离 Debian ARM64 Root `1048582:257` 实装 Core `f8c61e4e27` 等六包。JDK 21 正式 Main 活跃时原生解包后继 Core `c65c04741e`，旧 Main PIDFD 证实退出，ICMP worker 和 socket 回收。正常停机后，同 Root 新 boot 沿同一围栏 `f91e8d96-24ea-4ed6-8157-cbbf7ee63276` 一次续配。六包 installed，dpkg audit 为空，21 份新记录含 READY/configured 验签，19 份原签名与两项凭据 inode 和摘要保持。 前后正式 Main 的 Core ConfigurationService 均 ACTIVE，FD 9 匹配实际 writer lease，独立非阻塞独占申请被拒。双族回环通过，同 UID 非 Main 均拒绝。worker UID 986、NNP 1、仅 `CAP_NET_RAW`。Main 和 Agent disabled/inactive，VM 正常停放，重复冷读一致。补父项 #3、#9、#12，冻结 **0/12**。外部延期仅原三项。 [正式证据](../../../../.management-close-20261116/core-rtc-hotplug-20261120/icmp-core-maintenance-20261125/HANDOFF.md)

## 2026-11-25 API2 Position 正式 HTTPS 与 Core REST bundle 修复

父项 #3、#4、#9、#12 补证。ARM64 Root `1048581:257` 一次升级到 Core `c65c04741e`，补齐 REST bundle 对 Commons IO function 的导入。JDK 21 下 150 项通过，21 份新记录含 READY/configured 验签，七包 installed。合法 Main 实测三路匿名 401、登录后未授权 403、授权后定位及毫秒 UTC 200、撤销身份后旧会话 401，定位锁失效后的三路状态与实装合同一致。四份原快照保持原字节和权限，Core 升级更换其 inode。原运行时数据目录和 19 份历史记录保持，隔离 VM 已正常停放。冻结 **0/12**。 [正式证据和冷读](../../../../.management-close-20261116/core-rtc-hotplug-20261120/api2-position-gpsd-maintenance-20261125/HANDOFF.md)

## 2026-11-25 API2 Position 独立 purge、跨停机续配及 GPSD 定位恢复

父项 #3、#4、#9、#12 补证。隔离 ARM64 Root `1048581:257` 将正式 API2 Position `284058a-1` 独立 remove、purge，再解包和跨正常停机一次续配。缺包时 Core 仍读原快照。后继合法 Main 通过实际 gpsd 和伪终端取得新定位，毫秒 UTC、定位锁失效和四条实际 API 2.0.0 接线通过。原快照 inode、摘要、Agent READY/OFF 配对和 19 份 Core 记录保持。七包 installed，VM 正常停放并重复冷读。冻结 **0/12**。 [正式包和冷读](../../../../.management-close-20261116/core-rtc-hotplug-20261120/api2-position-gpsd-maintenance-20261125/HANDOFF.md)

## 2026-11-25 typed Journal 跨 Core API1 → API2 原位升级

父项 #3、#4、#9、#12 补证。隔离 ARM64 Root `1048580:257` 一次正式 APT 联合升级 Core、Deployment、GWT、Wires。正常停放后同 Root 新 boot，正式 Main 经低权限 typed Journal 读回原 boot 的唯一消息，原归档、inode、摘要和 cursor 保持。四项实际 OSGi 导出均为 2.0.0。原 19 份与新 21 份 Core 记录、两份凭据均核对，六包 installed 且正常停放，冷读重复一致。冻结 **0/12**。 [正式证据](../../../../.management-close-20261116/core-rtc-hotplug-20261120/journal-api2-upgrade-20261125/HANDOFF.md)

## 2026-11-25 typed Journal 跨 Core 独立 purge 与同号重装

父项 #3、#9、#12 补证。隔离 ARM64 Root `1048579:257` 原生 remove 配对依赖与 Core 后，独立 purge Core，再沿同一围栏解包和跨正常停机一次续配。正式后继 Main 只读取得原 journald 消息，原日志 inode、SHA-256 和 cursor 保持。40 份既有和 19 份新签名记录、两份凭据及原应用数据备份均通过核对。六包 installed 且停放，重复冷读一致。冻结 **0/12**。 [正式证据](../../../../.management-close-20261116/core-rtc-hotplug-20261120/journal-core-maintenance-20261125/HANDOFF-PURGE.md)

## 2026-11-25 typed Journal 跨正式 Core 解包与新 boot 续配

父项 #3、#9、#12 补证。隔离 ARM64 Root `1048579:257` 的正式 typed Journal 原生归档和唯一消息跨同版 Core unpack、正常关机及同 Root 原号一次 configure 保持，前后合法 Main 实际低权限读回，非 Main 同 UID 请求被拒。40 份 Core 签名记录、六个实装载荷及原两份凭据通过核对。六包 installed，VM 正常停放并重复冷读。冻结 **0/12**。 [正式证据](../../../../.management-close-20261116/core-rtc-hotplug-20261120/journal-core-maintenance-20261125/HANDOFF.md)

## 2026-11-25 Deployment 原号 57225 停机复核

父项 #3、#9、#12。ARM64 Root `225:257` 已在正式 `c012586-1` 验证真实待结算发布使原生 remove、purge 均由 installed prerm 拒绝。原号 `57225` 只派发一次，随后同 Root 同号只读 GET 为 COMPLETED。本轮冷读与留档一致，原 DP 和回执摘要保持，六包 installed，VM 保持正常停放。独立 config-files postrm 拒绝仍沿用合成标记证据。冻结 **0/12**。 [原生实测与停机证据](../../../../.management-close-20261116/core-rtc-hotplug-20261120/three-generation-deployment-receipt-20261125/HANDOFF.md)

## 2026-11-25 API2 六组件内 GWT 与 Wires 独立清除和原图续配

新隔离 Debian ARM64 Root `1048578:257` 实装 Core `f8c61e4e27`、GWT `9f226c0`、Wires `f0a95c6` 等六份正式包。Core READY/configured 共 19 份记录验签，Agent 原配对号 READY/OFF。JDK 21 正式 Main 只提交一次 Timer→Logger 图，原快照 `1795589035000` 实际送达。

GWT 与 Wires 各自原生 remove 到 config-files，再独立 purge。GWT 缺席时 HTTPS 404，Wires 原图仍实际送达。GWT 先重装后 HTTPS 200，Wires bundle、工厂和图服务缺席且无送达，Core 仍读到原快照。四个 GWT conffile 在 remove 后保留、purge 后撤下、重装后与原摘要一致。Wires 解包后正常停机，同 Root 新 boot 核对注册锁和原图，只 configure 一次。六包恢复 installed，实际 Main 和两类 provider 恢复，GWT 与 Wires 五条选定导入接线 API 2.0.0，原图再次送达。原快照和提交标记、14 份保护条目、19 份 Core 记录以及 Agent 原配对保持，VM 正常停放，重复冷读一致。

补父项 #3、#4、#9、#12，冻结 **0/12**。外部延期仅真实 NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。 [本轮交接与冷读](../../../../.management-close-20261116/core-rtc-hotplug-20261120/api2-six-gwt-wires-20261125/HANDOFF.md)

## 2026-11-25 ARM64 Alpine API2 镜像内 YOFC DP 与同卷换容器

从 Git 构建 Docker `ad4b77f`，使用正式 Core `f8c61e4e27`、Deployment `8da586b` 及配对 GWT、Wires 四份 deb。默认四份镜像内 DP 为 YOFC runtime、api-comp、apps-comp、apps-opcuaserver。首次启动四个 bundle 均 ACTIVE，归档与构建输入同摘要。原卷 Root `39:10239737` 正常停放后换容器，Felix DeploymentAdmin 再次读到原四份 DP，五条 channel／driver 接线均为 API 2.0.0。

复核容器 PID 1 为 JDK 21 Java、UID 122、NNP 1、CapEff 0，回环 HTTPS 200，原卷实例锁在运行时拒绝独立非阻塞取锁，停机后可取得。原注册表、TLS keystore 和凭据共七份文件的 inode、权限与摘要保持，两容器正常停放，重复冷读一致。默认镜像的 Cloud handler 和所需 MQTT transport 均未注册，此处覆盖本地 DeploymentAdmin 与镜像内 DP，远程 provider 和依赖现场配置的 DS 行为继续内部待验。

补父项 #3、#4、#9、#11、#12，冻结 **0/12**。外部延期仅真实 NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。 [本轮交接与冷读](../../../../.management-close-20261116/core-rtc-hotplug-20261120/api2-alpine-dp-20261125/HANDOFF.md)

## 2026-11-25 API2 Triton 独立 purge、跨停机续配与原工厂配置恢复

隔离 Debian ARM64 Root `249:257` 实装 Core `f8c61e4e27` 和正式 Triton `5abdd65-1`，使用 JDK 21。合法 Main 通过 ConfigurationService 只创建一次 `fixture.api2.triton.remote`，原快照 `1795588222040`。同 Root 原生 remove 到 config-files，独立 purge 到 not-installed。缺包期间 Core 仍读到原快照，Triton bundle 与原实例服务缺席，回环测试端点无新增 RPC。

同包原生 unpack 后正常停机，新 boot 核对同 Root 和原快照、提交标记后一次 configure。恢复 14 包 installed，已装 JAR 与正式 deb 成员同摘要。合法 Main 自动恢复同一实例，实际 InferenceEngineService 对隔离回环 gRPC 的三项读回与维护前一致。21 份 Core 记录摘要和三份保护文件的 inode、权限与摘要保持。Main、Agent disabled/inactive，VM 正常停放，重复冷读一致。

首次启动测试端点因夹具目录权限失败，修正前未调用配置 API 或执行包维护。覆盖 Triton 的包生命周期和回环 gRPC，NVIDIA 服务及 GPU 推理继续内部待验。补父项 #3、#9、#12，冻结 **0/12**。外部延期仅真实 NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。 [本轮交接与冷读](../../../../.management-close-20261116/core-rtc-hotplug-20261120/api2-triton-purge-20261125/HANDOFF.md)

## 2026-11-25 API2 Deployment 原生 remove、独立 purge 与原围栏重装

隔离 ARM64 Root `250:257` 对已装正式 Deployment `8da586b-1` 执行一次 remove 到 config-files，再独立 purge 到 not-installed。其余 13 包保持 installed。缺包时合法 Main 的 Core ConfigurationService ACTIVE，四份 Deployment bundle 和正式 handler 均缺席。原四个 DP 归档、终态回执和数据指纹保持。

同包解包到 unpacked 后，正常停机前后均由同一原锚点和 PID 1 正式断言拒绝 Main。新 boot 核对原围栏后一次 configure 恢复 14 包 installed，dpkg audit 为空。四个发行 JAR 与原 deb 同摘要，实际 Deployment Core 和 REST provider ACTIVE。后继 Main 只读查询 `25025001` 至 `25025004` 均为 COMPLETED，四个 YOFC bundle 均 ACTIVE 并实际接线 API 2.0.0。原归档和回执 inode 与摘要保持。Main、Agent disabled/inactive，VM 正常停放并重复冷读。

补父项 #3、#4、#9、#11、#12，冻结 **0/12**。本次覆盖已结算 DP 的 Deployment 独立 purge。其余包维护矩阵继续内部待办，外部延期仅原三项。 [本轮交接与冷读](../../../../.management-close-20261116/core-rtc-hotplug-20261120/api2-dp-deployment-purge-20261125/HANDOFF.md)

## 2026-11-25 API2 Core 原生解包、跨停机续配与四份 YOFC DP 保全

隔离 Debian ARM64 Root `250:257` 在四份正式 YOFC DP 已实装的 Main 运行中，原生解包同版 Core `f8c61e4e27-1`。实际旧 prerm 有序停服，返回 0 时原 Main PIDFD 已退出。原维护号 `ce41c531-999b-4aed-b13e-70b71bdabc98` 停在 unpacked。正常停机冷读确认原围栏及注册表、四份 DP、四份终态回执均保持。

同 Root 新 boot 核对原围栏 inode 和摘要后只续配一次。同号 21 份记录验签，40 份前序记录保持，14 包 installed 且 dpkg audit 为空。原 packages 和 data 目录及受保护文件 inode 保持。合法 Main 只读查得 `25025001` 至 `25025004` 均 COMPLETED，四个 bundle 均 ACTIVE 并接线 API 2.0.0。Main、Agent disabled/inactive，VM 正常停放，官方冷读重复一致。

补父项 #3、#4、#9、#11、#12，冻结 **0/12**。含正式 DP 的独立 remove/purge、其他中断和剩余包维护矩阵继续内部待办，外部延期仅原三项。 [本轮交接和冷读](../../../../.management-close-20261116/core-rtc-hotplug-20261120/api2-dp-core-maintenance-20261125/HANDOFF.md)

## 2026-11-25 四个 YOFC API2 DP 正式实装与同 Root 只读复验

隔离 Debian ARM64 Root `250:257` 在已配对的 14 包正式 Main 下，从 YOFC Git `a652e29910` 构建的 api-comp、apps-abplc、apps-comp 和 apps-opcuaserver 四个 DP 各派发一次。首号 `25025001` 的早到 GET 返回 NOT_FOUND，有原归档和已落盘终态回执后沿原号只读确认。其余三号分别为 `25025002` 至 `25025004`。四个 Felix bundle 均 ACTIVE，四条 channel 和三条 driver 接至实际 API 2.0.0。

同 Root 正常停机后重新启动，原四号只读均为 COMPLETED，四个 bundle 仍 ACTIVE。21 份 Core 签名记录、四份归档和四份回执摘要保持，归档和回执 inode 保持。Main 与 Agent disabled/inactive，隔离 VM 再次正常停放并重复冷读。原 `225:257` 的 `57225` 仅做停机复核，原实测拒绝和回执保持。

补父项 #3、#4、#11、#12，冻结 **0/12**。含 DP 的 Alpine、依赖现场配置的 DS 行为及剩余包维护矩阵继续内部待办。外部延期仅原三项。 [本轮交接和冷读](../../../../.management-close-20261116/core-rtc-hotplug-20261120/api2-yofc-dp-20261125/HANDOFF.md)

## 2026-11-25 API2 正式发行准入与原位联合升级补证

父项 #3、#4、#11、#12 补充真实 ARM64 Debian 和 Alpine 发行证据。八包的实际 Debian 控制档采用 `+api2`。隔离 Root `249:257` 的初次联合升级在维护钩子拒绝后跨正常停放按原 Root 续配。隔离 Root `250:257` 用 Deployment `8da586b` 一次完成旧包到 API2 的联合升级，原两份 `deconfigure` 围栏保留至成功关闭。

两个 Debian Root 各有 14 包 installed、21 份配置验签、合法 Main 和 30 个实载 API2 直接消费者及九个选定 provider 证据。旧 Core 反向解包及旧 Wires configure 被 dpkg 拒绝。旧 Wires 未配置载荷在 PID 1 正式 Main 启动时被 launcher 拒绝，原 Equinox 配置保持，恢复匹配包后 Main 通过。两个 Root 均正常停放并冷读。Alpine 双向混装构建拒绝与同代无 DP 运行通过，YOFC DP 实装和完整场景矩阵继续内部待办。冻结 **0/12**，外部延期仅真实 NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。 [当前交接](../../../../.management-close-20261116/core-rtc-hotplug-20261120/api2-admission-20261125/HANDOFF.md)

沿用 11 月 22 日暂停时的 12 项合同。复核以当时的完整冻结台账、可读检查点、实装版本和新增终验证据为依据。按子场景确认已有工作，整个父项满足原合同后才记闭环。

| # | 父项 | 已验收的范围 | 仍待的证据或实现 |
| --- | --- | --- | --- |
| 1 | 完整密钥生命周期 | 多代备份及恢复、原 READY 续配与归档、原 Main provider 验签、受控快照恢复及写释放中断均有原号证据。跨格式发布冷读已恢复。保留重装的原目录交接、联合续配和正式 Main 冷启验签保全原 PKCS12、加密口令记录和三份快照。 当前 Core 在原已初始化六组件 Root 保留原 PKCS12 与数据 inode，跨正常关机续配后实际 provider 冷启验签。 新增 PKCS#11 SoftHSM 原 XML 快照及不可导出私钥的正式保留重装、缺席拒绝和同 inode 归位后验签。 当前同 Root 实测新 keyring 发布、原号快照重加密及实际 Main 读回三份快照、同一软件令牌验签。原 XML 当时仅对 PIN 作 Base64 编码，本轮启用整文件加密。 Core `8ff7e6eaaf` 在另一个独立 ARM64 Root 的正式 LXC 换代自动发布三个凭据副本，原发布经中断和密钥服务重启同号完成，真实 Main 新旧代读回通过。 已发布 Root `218:257` 跨正常关机沿原维护完成受审查的后继 cohort 升级，Main 从原数据读回两代密文。 原 `218:257` 保留重装原号已由严格配对的仅控制脚本后继续配，正式 Main 从原数据读取两代密文。 | 各 provider 的原子重加密与跨包连续性，更多已发布 cohort 的受控升级和保留恢复，令牌运行期缺席或替换、其他 launcher、完整 GUI 管理和旧弱钥安全换代。 |
| 2 | CA custody | 方案 A 外部签发。本轮生产 TLS 激活已受 quorum 约束。独立 PostgreSQL 与真实 HTTPS/mTLS 测试覆盖重复管理员、错误租户／版本、禁用审批人、两人通过和原号回执。 正式激活在 PostgreSQL 持锚定锁直至发布，已验并发策略写或禁用、v2 原号审批证明及 mTLS 切换。 0029 与正式 console 接入当前保管名单、门槛和原提案管理。现行两人批准后发布三人策略，旧策略下的待激活提案失效，名单外同租户管理员无法计票。三个有效成员使原号 APPLIED。批量登记绑定生效 CA 代际、保管策略和公开证书指纹。实际两名管理员从真浏览器批准后两台 mTLS 客户端获 Welcome；唯一键冲突整批回滚并保留原号拒绝回执。批量公开 CSR 经自签名与主体校验，以原号精确导出。独立无网络测试签发器签回，中心核对原公钥及清单后，原号双人批准 APPLIED，两台客户端实际 mTLS。批量续期在原号锁定旧证书与代际，复用原单证书历史及未决写门禁，隔离正式 ARM64 中心两台换代后接纳新证书、拒绝旧证书，并对代际冲突保留整批拒绝。 两台 Debian ARM64 正式 Agent 导入各自 v2 原收据，原证书和代际经两个本机 journal 与中心一一核对。 | 跨人实体保管、外部签发机构及恢复交接的真实多人证据；GWT 联合导入与独立端口配置、人员管理部署和完整发行矩阵仍待验。 |
| 3 | 最小权限与完整发行 | 既有 Core Main、broker 和部分六件套维护有单独的 ARM64 证据。本轮 3 台 OrbStack 与 1 台 Parallels 用同版 Core+agent 首装和入网成功。独立 ARM64 实测当前完整六组件首次 APT 安装，agent 原号分段升级后合法配对，Wires 解包后原配置续接，Main 前后实载 8 个 bundle。另在当前配对下实测 Networking remove、purge、half-configured 续配，原配对失配时拒绝 agent 启动，续配后原号 READY。已补当前受审 drop-in 的后继入口，现行六组件用相同载荷的后继 Debian 修订实际联合换版，原配对和数据连续。修复后的 Core 与 Networking 在新的已绑定 ARM64 网关合并实装，原号 APPLIED、原目录的签名交接、原 SQLite/WAL 和原任务保全通过。实测 agent UID 499、CapEff 0、NoNewPrivs 1。Deployment 启动断言修复 ARM64 原号联合续配，Core 先配置后实际拒绝 Main 启动，四 addon 续配及六组件 purge 均通过。Core 密钥服务覆盖校验已收紧到准确的 LXC 适配，Agent-first 的正式六包首装、Root 密钥状态和 Main/agent 启停通过。 Core 898e456 通过 99 项 ARM64 定向单测及新正式包合并实装，实际 Main 和 agent 的 NoNewPrivs 均为 1，Main 的 Root 写租约 FD 与内核锁一致。Core 8dc92ab 的时钟及重启代理接受准确 LXC 配对、拒绝实际加载的未登记通用覆盖，撤回后重新配对。Core 2140aa5 让实际重启代理在 OrbStack 以 NoNewPrivs 1 和零能力运行；同 Root 与已配置 Networking 配对，真实 socket 拒绝不同进程的同 UID 客户。Core 806ad2e 在隔离 ARM64 LXC 正式 OSGi 验收中单次派发重启，首次跨 boot 的原 actor 回执为 ACCEPTED，随后同 Root 正式 Main 沿原号完成结算。原版另一 Root 的回执缺失按原号 UNKNOWN 封存。Core e119ff5 为十个正式 typed worker 保留实际 NoNewPrivs，获准的适配仍按准确路径、内容和运行时配对；同一正式 JVM 实验验证所有权标记、回环 ICMP 和日志导出，原重启终态保持。 Core ff1364d 保持 BIND 服务和 DHCP actor 的完整配对，正式 Main 只读 BIND 状态并完成实际 DHCP 启停及回执确认。 同一正式 Main 的生产 BIND/DHCP 代理做了真实 LAN 配置发布和配对回执。 Core 91cbb30 与 8dbf60c 已实装，modem readiness 和虚拟网口拒绝已确认。实际时钟 actor 内核 NNP 1 和 CAP_SYS_TIME 独立核对。 Core 0823d14 的正式新包使系统服务管理授权与实际时钟代理内核约束一致。独立 Root 单次 Chrony STOP 完成，原号和两端回执跨 Core remove、purge 与重新解包续配保持。原另一 Root 的 UNKNOWN 继续封存。 当前正式 Core 的同一 Main 完成 Chrony 发布、启动、热重启、停止和原配置恢复，五笔正向操作冷读一致。两次 daemon 运行均 -x 且实际无 CAP_SYS_TIME。 未绑定且 OFF 网关用 Agent `6c1977c` 完成六组件首装及同 Root 的 Core `0823d14779` 与 Networking 联合升级，正式 Main/Agent 实测 NoNewPrivs 1。 Docker `766590c` 修复首次 HTTPS 凭据暂存中断，正式 ARM64 四包四 DP 镜像在同卷移除容器和重部署后保留原密钥库与部署记录。 原 Root 215:257 保留未绑定 peer 后经正式后继 Agent 完成 remove、purge、解包和同原号重装，六包恢复 installed。 已初始化六组件在原 ARM64 Root 跨正常关机保留重装，Core 保留数据，Agent 和四个 addon 实际 purge 后同版续接。 两台新 ARM64 装 Core 可正常启动 Main 和正式 Agent，均实际观测 NoNewPrivs=1，Agent 无有效能力，Main CapEff=0x400。 当前正式 Core 0823d14 和 Deployment 2bdb66d 在独立 Debian ARM64 按原号完成静态 Main 保留重装，vendor 发布前已有断言，Core READY 后仍拒绝启动，Deployment 完成才允许显式启动。 Main 实际正常停机 disabled，原数据保留。 在现有 ARM64 Root 按独立原号真实移除和续装 Core＋Deployment，原 PKCS#11 配置及数据保留，三个正式 Main 以 NoNewPrivs 1 运行后正常停放。 Core `87328bbb6d` 启动并读回原号重加密后的三份快照，正式 Main UID 995、NoNewPrivs 1，正常 disabled 停放。 密钥服务保持实际五组 capability 为空，Root 所有的换代副本只授权 kurad 读，其他本机账号实际拒读，正式 Main 唯一显式授权后启动并正常停放。 同一 Root 实装新 cohort 后，正式保留移除成功，重装在原签名 intent 后、layout 前停点，原数据 inode 和 25 个文件保持。 原停点由新签名续接回执接到原 intent 和精确后继 postinst，原 Root 正式续配完成，25 份文件和原数据 inode 保持，Main 正常停放。 新 Root143 六组件基线及四包 API1→API2 联合升级、40签名和9个provider已实装；本场景从停用Main开始。 隔离候选e155065的只读Root观察、15项crate回归及cap0/NNP1内核对照通过，不改变正式包或Legacy授权。  新格式native0.1.17按Git锁定，15项回归及11项ARM64合成围栏/原暂存续接场景通过，尚无正式Core包消费。  Core634/Net15eb已在新ARM64 Root实装同一v2域，19份READY配置签名、六包及10实际JAR配对，冷Main合法租约和原密文读回通过。 本轮同一正式包在真实FW pending下remove/purge均由installed prerm拒绝，81保护文件和运行Main保持，原号正式结算后六包installed及合法Main/provider恢复。 | 当前六组件中余下的升级场景，各 provider 和 launcher 的完整升级、移除、purge、中断及平台矩阵，以及各角色的完整原生隔离验收。旧已绑定现场的原维护 UNKNOWN 继续封存。早期 layout 只有原意图时仍未验证一般恢复，原现场保持停放。 Chrony STOP 原号 UNKNOWN 按原 Root 封存。 |
| 4 | 原 14 组及 API | 原 14 组、API2、1.2/1.8/1.9 演进记录可追溯。已有局部构建、消费者及 provider 证据。七份正式包的九个 API2 解析对照及四个 YOFC Git 重建产物已核对。隔离 ARM64 新装 API2 组合的 30 个实际 Debian 消费者均 ACTIVE 且接线 2.0.0，四个 YOFC 直接消费者已解析。 实际 ConfigurationService 在跨代后恢复同一防火墙快照，所选9个provider ACTIVE并与归档配对。 | 原 14 组逐项实际发布、完整包维护、YOFC DP 在其他 launcher 的实装和现场配置行为。ARM64 的混装拒绝、原位联合升级、四个 YOFC DP 正式实装和同版 Core 续配已补证。 |
| 5 | 完整实际接管链 | 已核对正式 Main 的恢复输入、原 CM 回执、固定启停、受保护启动与写释放。 | 在同一原现场沿原授权覆盖原 forward／confirm／inverse、source observation、停机/到期全链。 |
| 6 | 网络矩阵及历史 | 保留 NM；已有实际 DHCP、部分 IPv4/IPv6 防火墙、重启与原号读回证据。 隔离 ARM64 实测 BIND 状态保持同一系统实例，专用 DHCP 只读配置投影与两代启停回执停机冷读通过。 当前正式包组合完成 LAN 范围的 BIND 发布、真实客户端租约和 DHCP 来源 DNS 转发，随后恢复原配置并冷读五笔终态。 当前正式 Main 识别 Networking 管理的 veth 并在 modem 作业前拒绝，未留下新意图。 独立 ARM64 审查副本通过真实 ModemManager 对回环虚拟 modem 单次 Reset，后继正式 Main 沿原号查询及确认，原 intent 和 result 保持。 新 Root 的单次真实 Reset 保留实际待结算原号并跨 Core/Networking 联合 remove、purge、解包、正常关机和续配，设备缺席时按原号读回 APPLIED 并确认送达。 新用途双栈规则跨四包升级保持真实允许/拒绝，原2笔放行仅只读、2笔新撤回恢复模型，24组原生意图/结果可冷读。 已确认v1 Scope的device依赖；同machine-id/控制目录inode的btrfs克隆可由内核对象区分，OverlayFS明确拒绝。  新v2元数据在正常重启保持，绑定后的同machine-id/inode克隆被native及独立维护门禁拒绝；同源device重分配未复现。  正式kernel-fh-v2跨正常停机冷启保持Scope，原8号只读保持，最终16组终态；device重分配未复现，原时点真实未决FW维护尚待，本轮已补证。 本轮Root1048585:257的真实IPv4发布只派发一次，ACK后staged中断，原号886f19b4在同Scope结算APPLIED，原inode23295保持，sender由同bootMain恢复，最终31终态。 | 完整网络组合、回退、退役及原历史继续核验。真实 NM/cloud-init/物理硬件交接按原延期项。 其他 modem 动作与六组件联合维护矩阵继续待验。 |
| 7 | 中心与部署 GUI | 隔离部署验证正式中心登录、网关列表、TLS 设置及四网关状态；最终 ARM64 包的 React 载荷已用真实 Chrome 读取已发布的 CA 保管策略原提案。批量登记页完成公开 PEM 选择、两人原号审批、终态读取及 Chrome 截图。真实 Chrome 完成批量 CSR 整理、原公开包下载、签回上传和原号审批读回。真实 Chrome 实测批量续期、两份原 v2 公开回执下载和代际冲突原号读回。 本轮正式 HTTPS 原批次与子回执、两笔任务读回，使用已验证的正式 ARM64 中心载荷；实际图形沿用上轮证据。 | 主机、实例、容器端到端流程，容量与归档、首次 TLS UNKNOWN、半凭据和历史 CSR 的全部 GUI 错误恢复。 |
| 8 | 数据库与协议 | 合成旧 schema 的真实 PG 备份/恢复/rekey，正式中心新库的迁移 0028。隔离 ARM64 PostgreSQL 以受限角色完成 0029，保存新策略与旧激活提案的原版本。0030 在受限 PostgreSQL 角色下实测批量登记与同事务事件／审计、冲突回滚和原号拒绝回执。0031 经受限角色实测以原号、租户与清单摘要关联 CSR 和登记回执。0032 实测整批续期及跨网关原子回滚，原子保存各网关原 v2 收据与身份事件。 当前三个发行包的新隔离联合验收跨 PostgreSQL 原批次和两个网关本机原收据，连接纪元及两笔成功任务均有数据库确认。 | 客户旧加密库的真实恢复、完整 GUI 数据库流程、协议 0 在线升级及发布互操作与校验和。 |
| 9 | R017 与文档/技能 | 方案 A 和父库交接已更新；本轮 agent-first 安装成功，并单独保留授权顺序与配对记录。当前六组件修复 Networking 有效 drop-in 校验，实装 agent 在原号完成本地配对且保持 OFF，已验收匹配 Networking drop-in 经移除和中断续配后的原号 OFF 与 READY。已实际验证当前配对下 Core 与 Networking 后继联合维护仍保留该状态。已绑定场景保留四份原授权 READY，并用同一证书重新连通、读回原任务，正常切回 OFF。实装绑定材料由夹具准备。 未绑定且 OFF 的原远程开关授权、材料授权和本地 peer 经联合维护在原号保持 READY，冷启后仍 OFF / UNBOUND。 已发放本机 peer、材料与远程开关授权的未绑定网关，三份原授权跨 Agent purge 和重装保持 READY，实际启动仍 OFF。 六组件同时缺席或待配置后，按 Core、四 addon、Agent 顺序恢复，三份原授权同号 READY，远程 OFF。正式 ARM64 React 已跑通外部签回公开证书的批量登记与冲突原号读取。 当前正式 Core/Agent 的本机直连端口联合链覆盖两个 Root 的显式 OFF、原 v2 导入、显式启用、新会话、任务和 OFF 停机。 原229复机设备号变78在业务前停止，原22组记录和升级签名停机保全；新143正常停机冷读。 原229保持封存，新的观察输出不授权迁移；两台纯元数据夹具正常停机冷读。  新格式仅空历史显式初始化，原229不追认；两台无Kura元数据夹具正常停放，未派发业务号。  正式入口在Agent安装前初始化，校验PID1 idle Main并继承原FD3；Main活动时拒绝，原229等旧v1记录不迁移。 同Root/原号/原围栏的正式结果发布与终态只读已区分，原pending原文留档，旧v1原现场保持封存。 | R017 的产品化安装、import/CSR 顺序和所有 UI 操作路径，全套 TLS reload、GWT 实际批量续期回执导入、独立端口配置和跨平台体验。中心侧批量 CSR、签回登记与续期有隔离 ARM64 证据。 原Root设备号变化后的持久身份与正式续接依据继续内部待办，禁止携旧号跨Root操作。 新v2正式入口及本轮真实未决原号续接已有上述证据。旧v1续接、未验存储和launcher继续内部待办。 |
| 10 | G021/G022 | G021 27 个描述符，5 个状态周期。G022 的三个非 Java POM、.NET 三协议生成、11 个 nupkg 和 Go 31 条历史诊断已核对。 | 原检查点仍保留 Go 默认 vet 债务、KNX 数据漂移 review 与多语言生成规则。 |
| 11 | 上游逻辑同步 | 11 fork／7 nonfork 的策略和历史审查游标可追溯，PLC4X 局部修复有独立证据。 | 按冻结游标完成剩余逻辑组、shared SLMP/.NET 范围及跨仓来源审查。 |
| 12 | 正式源码／发行门禁 | 484c4c843c 的全量 ARM64 Equinox，后续构建记录及本轮 ARM64 安装和当前 Console 测试分别可核对。当前六组件选定 8 个实际加载的 JAR 与正式 deb 内容逐个验同，本轮 Networking 中断续配及当前两包后继联合升级后保持相同证据。包含 fd67 的修复 Core 制品保留原 JDK 21 运行时载荷，已绑定联合维护合并验证新控制脚本、agent 包及原任务连续性。后继 f9cebce 的维护脚本和原数据交接已实装，配对 Deployment 1642074 后同号联合重装、正式 PKCS12 冷读及六组件 purge 通过。四个 Deployment JAR 字节保持。Core 0737f63 的两份维护策略和清单已实装，233 个运行时 JAR 与基线逐项一致。 后继 Core 898e456 的 Python 启动与租约策略、制品校验和、ARM64 首装及实际 JVM 写租约均可冷读复核。源和发行包配对下重跑原生合同，1265 项 ARM64 通过，补验 7 项生成后 polkit 合同。8dc92ab 的四项改动载荷、正式 ARM64 包和实装策略均可冷读。Core 2140aa5 恢复九组 broker 的源码副本和配套清单，实装逐项核对 58 个变更成员。后继 806ad2e 通过 74 项 ARM64 定向测试、40 个配对成员包审计及隔离 restart 与同号实装结算。Core e119ff5 关联的 77 个更新成员在原 Root 和 boot 实装逐项验同，233 个运行时 JAR 字节不变。 Core ff1364d 在 ARM64 同 Root 标准升级后核对 17 个更新成员，233 个运行时 JAR 字节保持。 原正式 ARM64 的 Core ff1364d 与 Networking 83f237e 不换包，在同一 Main 实装验证 BIND 和 DHCP 的真实客户端全链。 Core 91cbb30 与后继 8dbf60c 的 ARM64 配对载荷分别实装核对 42 和 9 项。两版 233 个 JAR 字节相同。 Core 0823d14 独立 ARM64 首装、实际 PIDFD Polkit 有限授权、单次正式 Main 回执和同 Root purge 后重装均通过。 已验 0823d14 正式包的 Chrony 热配置入口和实际两次 loopback 数据面应答，原 STOP 回执保持。 当前联合 Core 0823d14 全量 ARM64 Core/OSGi 已验，82 模块和 2173 项通过，正式包 41 个 Core 插件、176 个第三方插件和独立 recovery 的实际输入均可复核。 Agent `6c1977c` 的 ARM64 定向 88 项和六组件首装、同号续办及 8 个实际 ACTIVE bundle 的制品一致性均有冷读证据。 当前 Core `0823d14779` 的正式 ARM64 Docker 镜像核实四个 deb 和四个 DP，30 个启动器单测与四个实载业务 bundle 可冷读。 Agent `e473e02` 的 ARM64 107 项与实装新版归档、原号阶段签收和停放证据可冷读。 当前六份正式 deb 配对，同 Root 两维护围栏与关机前后四项摘要经冷读，实际原 PKCS12 签名通过。 Console CA 持锁与原证明通过 ARM64 11 项单测和两项实装链，Git 对应 Linux ARM64 release 二进制可冷读。CA 名单管理增量通过 13 项 ARM64 单测，原号链已使用同 Root 的 ARM64 正式打包入口载荷补验，最终 deb、对应源清单和真实 Chrome 截图可冷读。批量登记候选的 259 项源码与本地 Git 匹配，ARM64 16 项单测及同一正式包的 Chrome／PG／mTLS 整链可冷读。CSR 批量交接通过 7 项 ARM64 单测，正式 deb 的源清单 263 项与 Git 对应，保留真实浏览器下载 tar、同号签回审批、mTLS 原证据。批量续期候选通过 10 项 ARM64 单测。原两个终态与两份 v2 子回执、在用 ELF、最终正规 deb 和真实 Chrome 可冷读。 最终中心 ARM64 deb 与 Core 0823d14、Agent e473e02 组成一次本机批量续期全链，可由冷读匹配三份原制品。 新 Deployment 正式后继定向重组及四个 JAR 保持、同 Core 完整实际保留重装、拒绝与首次显式启动可冷读。 当前正式 ARM64 Core 内 Keystore JAR 和本机已安装 JAR 逐字节核对，SoftHSM2 发布包来源与实际摘要已录。 Core `87328bbb6d` 正式后继包、81 项定向单测及同 Root 原准备、验证、正式发布和旧代快照重加密的实装可冷读，233 个 JAR 摘要未变。 正式 Core `8ff7e6eaaf` 的 705 个成员审计、233 个 JAR 字节比较、99 项 ARM64 单测和实际同号回执可冷读。 正式 Core `9de1c6201e` 的发布清单配对、79 项定向和跨关机续配可冷读。保留布局源码 `361457aa9a` 另有 72 项定向和真实清单的只读接受证据。 Core `2cbb34996b` 后继 deb 与原键和运行载荷字节一致，63 项 ARM64 单测、原号签名和正式 Main 实装均可冷读。 4份Git正式包、在装控制档与完整manifest、9个JAR、40份HMAC及24组防火墙记录冷验；见actual-api2-firewall-live-model-20261126-checkpoint.json。 原锁文件和共享ARM64缓存下完成15项回归、Git同源码同ELF验证，见actual-root-identity-observation-20261126-checkpoint.json。  f11d66f的0.1.17标准Cargo source归档与全部Git文件匹配，锁定缓存构建和合成Linux门禁通过；Core/Networking新pin联合实装仍待。  Git6341906/15ebcda正式发行消费同一F11归档，36项Linux定向通过，实际manifest/控制/native cohort与10JAR及冷读配对；全矩阵不闭环。 现装634/15eb控制、native cohort及10JAR保持，真实pending的两次dpkg拒绝、同号原inode结算及94文件/31记录/19HMAC停机冷读已补，见actual-firewall-pending-maintenance-20261126-checkpoint.json。 | 按当前联合发行版本完成原 guarded apply、正式发布标签与制品校验，并验收整套发行合同。 |

## 2026-11-25 普通执行器在正式 Core 解包期间回收

隔离 Debian ARM64 六组件 Root `232:257` 实装 Core `df5ae00aac-1`。正式 Main 通过 OSGi 普通执行器派发一条长运行命令，实际命令和 worker 同属 `kura`、NNP 1、能力全零。命令运行中原生 `dpkg --unpack` 同一正式包，真实旧 prerm 管理停服，返回 0 时两个 PIDFD 均就绪，两个 socket 已撤下，原回调为 `EXECUTOR_RESULT_UNAVAILABLE`。沿 `8450a592-51b2-4980-90cc-095d4b706793` 正常关机并在同 Root 新 boot 一次续配。READY/configured 验签，新 Main 经同一 API 运行独立的 `id -un`，返回 `kura` 和退出 0。六包与可选 SoftHSM installed，Main、Agent disabled/inactive，VM 正常停放，停机读回一致。

25 项受保护条目逐项核对。两份既有 `token.object` 在两次 Main 运行阶段内容摘要变化，文件身份、属主和权限保持。四份快照随包恢复复制更新 inode，内容及权限保持。原长命令只派发一次。补父项 #3、#9、#12。冻结 **0/12**，外部延期仅原三项。 [正式取证与停机读回](../../../../.management-close-20261116/core-rtc-hotplug-20261120/executor-core-unpack-20261125/HANDOFF.md)

## 2026-11-25 RTC 原成功回执跨 Core 移除、清除和重装

隔离 Parallels ARM64 子 Root `2049:263807` 的正式 Main 使用已装 ClockService 客户端单次写入虚拟 DS1307。原号 `1d6f9778-af64-4dcb-ae6d-13641aa732ad` 已为 APPLIED / RTC_SET_RETURNED，本地保留 pending。正式 Core `df5ae00aac-1` 原生 remove、purge、同包 unpack，子机与外层 VM 正常停机。新 boot 校验同 Root、两套原维护围栏、原 sender 归档和原 actor 回执后，一次 configure。后继 Main 在试验 RTC 缺席时结算原号，五份原 actor 文件保持，仍只有一笔操作。两份子 Root 原凭据、外层 27 份保护文件和共享 rtc0 保持。Main disabled/inactive，子机及 VM 正常停放，停机冷读一致。

补父项 #3、#9、#12。冻结 **0/12**，外部延期仅原三项。 [实装与停机证据](../../../../.management-close-20261116/core-rtc-hotplug-20261120/rtc-applied-purge-20261125/HANDOFF.md)

## 2026-11-25 正式 RTC 成功回执跨 Core 解包与完整内核重启

隔离 ARM64 ext4 Root `2049:2` 的正式 Main 用已装 OSGi ClockService 的客户端单次写入虚拟 DS1307。原号 `1bff9494-6fce-4c24-949a-e347024cf9a4` 已为 APPLIED / RTC_SET_RETURNED，sender 保留 pending。Core `df5ae00aac-1` 同版原生解包后正常关机，同 Root 新 boot 核对两套原维护围栏后一次续配。后继 Main 在试验设备缺席时只结算该原号。actor 仍只有一笔操作，五份原 actor 文件保持。20 项保护文件核对通过，OCI 快照由包恢复复制后 inode 更新。四包 installed，Main disabled/inactive，VM 正常停放，重复冷读一致。

补父项 #3、#9、#12。正向 RTC 回执的 remove/purge 组合已在独立子 Root 补验，见上节。冻结 **0/12**，外部延期仅原三项。 [实装与停机证据](../../../../.management-close-20261116/core-rtc-hotplug-20261120/rtc-applied-package-maintenance-20261125/HANDOFF.md)

## 2026-11-25 FileMove 跨完整内核重启的原号结算与包维护

隔离 Parallels ARM64 ext4 Root `2049:2` 实装 Deployment `c012586-2`。实际 Main 通过正式 OSGi provider 单次派发 MOVE `120252948901`。原目标与回执落盘后丢失回复，Main 收到 UNKNOWN，源文件仍在。实际 remove 与 purge 均由原 prerm 退出 1。正常关机并重新开机后，新 boot ID 下原目标 inode、源文件和回执保持。原号经包内低权限恢复与本机确认到 ALREADY_APPLIED。随后独立 remove、purge、同包解包与配置通过，14 项原号文件、另 6 份保护文件及 rtc0 复核通过。Main 和 FileMove socket disabled/inactive，VM 正常停放。

补父项 #3、#9、#12。冻结 **0/12**，外部延期仅原三项。 [实装与停机证据](../../../../.management-close-20261116/core-rtc-hotplug-20261120/filemove-kernel-reboot-20261125/HANDOFF.md)

父项 #3、#12 补证。Core `22e54511df` 的 ARM64 定向回归与 Git 重建候选通过。Debian ARM64 dpkg 实际验证 postinst 替换与 Version 登记之间中断后的原包续接；Core 在精确签名验证后接受这组状态。当前停点的正式实装 Core／Main 整链仍待。 [中断证据与候选](../../../../.management-close-20261116/core-rtc-hotplug-20261120/retained-layout-dpkg-interruption-20261124/HANDOFF.md)

**冻结闭环 0/12。** 外部延期仅真实 NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。其余按原合同内部待办。

父项 #2、#3、#9、#12 补证。同一 ARM64 Root `245:257` 以正式中心和 PostgreSQL 17 接通第 1 代，双管理员审批真实签发批次。现装 GWT 对中心原样子回执的浏览器拒绝已复现。后继 `cafc758` 的 JDK 21 共 539 项测试与 ARM64 实装通过，实际 GWT 原样提交一次。Agent 与 GWT 对原号 `880c92cd-0c33-4d8e-a31c-d0fb0eaea028` 只读查得 APPLIED，提交时远程 OFF。随后第 2 代在原独立端口与正式中心 mTLS 成功，真实 `agent.status` 同号查得 SUCCEEDED。再次明确 OFF，原证书与回执保留，Main、Agent、中心、PostgreSQL 及 VM 正常停放，冷读重复一致。 [正式浏览器、中心与停机证据](../../../../.management-close-20261116/core-rtc-hotplug-20261120/gwt-bulk-renewal-live-20261125/HANDOFF.md)

同一 ARM64 Root `245:257` 由正式浏览器生成原 CSR、暂存、首次绑定，Agent 原回执的 actor 为 `kura-uid:995`。原号分端口配置 APPLIED/READY。实际 Agent 重启后连接地址为 41744，原登记地址保持 41743。GWT `6501d1b` 补齐正式无 schema 的 v2 API 回执，539 项回归通过，候选原位升级。正式浏览器已暂存后继 CSR 与证书，原身份第 1 代、远程 OFF，续期尚未派发。六包 installed、Main/Agent/socket disabled/inactive，VM 正常停放并冷读。 补父项 #2、#3、#9、#12。新中心回执的浏览器提交及真实 mTLS 继续内部待办。冻结 **0/12**，外部延期仅原三项。 [实装、浏览器与停机证据](../../../../.management-close-20261116/core-rtc-hotplug-20261120/gwt-enrollment-transport-20261125/HANDOFF.md)

中心原生 v2 APPLIED 回执含 `rejectionCode: null`，旧包解析两份真实回执均拒绝。GWT `3ffb80e` 接纳该字段为 null，仍拒绝非 null 或额外字段。JDK 21 共 537 项测试通过，正式 deb 实装隔离 ARM64 Root `245:257`，实际 Main 的 ACTIVE web2 bundle 对包并读入两份原回执，没有续期写入。六组件 installed，两服务 disabled/inactive，正常停放及重复冷读通过。 补父项 #2、#3、#9、#12。真实浏览器、分端口与 mTLS 联合验收继续内部待办。冻结 **0/12**，外部延期仅原三项。 [正式回执、实装与停机证据](../../../../.management-close-20261116/core-rtc-hotplug-20261120/gwt-center-renewal-receipt-20261125/HANDOFF.md)

既有隔离 Parallels ARM64 Root `2049:2` 使用 Docker 29.1.3、委派的 systemd 用户服务和实际 runc。原系统级 rootless 对象因无用户 cgroup 明确拒绝并保留。新独立对象经正式 Container `9a7cda7-1` OSGi provider 一次启动，原 PID 和启动时刻跨 Main 停服、provider remove/purge、缺席启动和正式重装保持。后继实际 Main 沿原容器号和快照查回，单次 TERM 停止、exit 0。原五份保护文件、快照及网络保持，临时引擎、用户及 AppArmor 配置清理。VM 正常停放，停机档案读回一致。 补父项 #3、#9、#12。冻结 **0/12**，外部延期仅原三项。 [实际 OCI、包装维护与停机核对](../../../../.management-close-20261116/core-rtc-hotplug-20261120/container-oci-positive-parallels-20261125/HANDOFF.md)

Root `232:257` 正式从 `-4` 原生解包 `-5`，正常关机冷读为 unpacked。同 Root 新 boot 只续配一次并达到 installed。实际 `build-origin` 权限由 0666 收紧至 0644，模块字节保持。正式 Main 用原两张证书验签，四份快照、16 项令牌的文件身份、发行版库和试验库、原站点配置与六组件保持。Main 和 Agent disabled/inactive，正常停放，重复冷读一致。 补父项 #1、#3、#12。冻结 **0/12**，外部延期仅原三项。 [实装和停机读回](../../../../.management-close-20261116/core-rtc-hotplug-20261120/softhsm-debian-mode-upgrade-20261125/HANDOFF.md)

从 Git 构建 ARM64 双库过渡镜像，完整 CTest 7/7。独立 Compose Root `39:10180694` 从 Alpine SoftHSM `2.7.0-r0` 生成并保存两个原文件令牌。正式 Main 经双库镜像将两个原配置合批切换至可选库，仅可选镜像同卷重开并用原证书验签。再经双库镜像一次合批切回发行版库，原镜像同卷重开及原证书验签通过。两份新快照与 19 份原文件保持，14 个令牌文件的 inode 和权限保持，只有两份 `token.object` 字节随库重开变化。五次正常停机，最终原版容器停放，冷读一致。 补父项 #1、#3、#12。冻结 **0/12**，外部延期仅原三项。 [实际 Compose、原卷与停机核对](../../../../.management-close-20261116/core-rtc-hotplug-20261120/softhsm-alpine-stock-migration-20261125/HANDOFF.md)

Core `f5f978f491` 和 Docker `ff4ed22` 用同一份 Git 固定源码构建可选原生模块。ARM64 Alpine CTest 7/7，原正式镜像入口和四份 deb 清单保持。独立同卷 Root `39:10164771` 在正式 PID 1 JVM 用原快照和原证书验签。目标令牌缺席时原槽拒绝，另一槽持续可用。同 inode 归位后原、新句柄在同 JVM 均恢复。换回基础镜像时两槽明确返回 `SERVICE_UNAVAILABLE`，切回可选镜像后两份原证书验签。19 份文件指纹、14 个令牌文件的 inode 和权限保持，四个容器正常停放，冷读一致。Debian 新修订 `-5` 的 CTest 7/7，和原版对照的成员字节相同，provenance 归档权限由 0666 收紧至 0644，新修订待实装。 补父项 #1、#3、#12。冻结 **0/12**，外部延期仅原三项。 [镜像、原卷和停机核对](../../../../.management-close-20261116/core-rtc-hotplug-20261120/softhsm-alpine-release-20261125/HANDOFF.md)

Core `424175c489` 的可选 SoftHSM 文件令牌模块从 Git/JDK 21 构建出 ARM64 `-3`、`-4` 两版，CTest 均 7/7。隔离 Root `232:257` 初次实装的低权限 Main 无法遍历 `/etc/kura`，修正到独立配置目录后沿原两份迁移快照只读续验。实际 dpkg 升级、移除与重装完成。移除后独立 purge 确认包已缺席，两个原 OSGi 配置均返回 `SERVICE_UNAVAILABLE`。重装后正式 Main 用原两份证书验签，16 个令牌条目、四份快照、原试验库和发行版库保持。六组件 installed，Main 与 Agent disabled/inactive，VM 正常停放，冷读通过。 补父项 #1、#3、#12。冻结 **0/12**，外部延期仅原三项。 [构建、实装、冷读和清理](../../../../.management-close-20261116/core-rtc-hotplug-20261120/pkcs11-module-release-20261125/HANDOFF.md)

SoftHSM 2.6.1 原测试的两个 fork 子进程在父令牌目录继续执行套件。同环境上游和候选未修正版各输出四份 77 项报告。上游 `c1b7273` 和候选 `670e97d` 只改同一份测试，等待并核对子进程退出。同条件下两边各两轮 CTest 均 7/7，p11test 每轮 77 项通过，运行库摘要在测试修复前后保持。复刻构建的运行库与 Root `232:257` 正式 Main 实装候选逐字节相同，这次完整 CTest 也通过。100 次原生初始化和结束后，候选每次释放所有四个令牌描述符。原正式现场仍停放并冷读。 补父项 #1、#10、#12。SoftHSM 正式发行接入继续内部待办。冻结 **0/12**，外部延期仅原三项。 [回归、原制品和冷读](../../../../.management-close-20261116/core-rtc-hotplug-20261120/pkcs11-native-regression-20261125/HANDOFF.md)

SoftHSM 原生候选 `b6388cf372` 固定 Linux 原目录与 token.object，缺席和同字节异 inode 替换均拒绝，原件归位后同 JVM 新取钥及旧句柄均恢复验签，另一槽持续可用。隔离 Debian ARM64 Root `232:257` 正式六包 Main 以 UID 995、NNP 1 和 CapEff 0x400 实测同一 JVM、同一 OSGi 服务及原证书。两份配置各创建一次，两份快照和先前 Deployment 三份保护材料保持。候选仅在夹具实装。上游与候选完整 CTest 仍有 p11test 失败和差异，四个隔离差异用例均通过。Main 和 Agent disabled/inactive，VM 正常停放、重复冷读。 补父项 #1、#3、#12。冻结 **0/12**，外部延期仅原三项。 [Git、原生候选和正式 Main](../../../../.management-close-20261116/core-rtc-hotplug-20261120/pkcs11-native-hot-return-20261125/HANDOFF.md)

隔离 Debian ARM64 Root `232:257` 实装六包，Core `df5ae00aac-1`、Deployment `c012586-2`。持真实 sibling 注册表锁，原生 remove 已删除 Deployment 载荷，postrm 锁超时使 dpkg 退出 1 并保留 half-installed。原 postrm、注册表、围栏和锚点保持，Main 被断言拒绝。正常关机冷读后，同 Root 新 boot 仍由原断言拒绝 Main。一次续办只触发原 postrm，进入 config-files，独立 purge 到 not-installed。同一归档原生解包、配置后六包 installed，两个发行 Deployment bundle ACTIVE，REST 服务由原 provider 注册。三份保护材料保持，Main 与 Agent disabled/inactive，VM 正常停放、冷读重复一致。 补父项 #3、#9、#12。冻结 **0/12**，外部延期仅原三项。 [原围栏、实际钩子和停机冷读](../../../../.management-close-20261116/core-rtc-hotplug-20261120/deployment-postrm-interrupt-20261125/HANDOFF.md)

隔离 ARM64 Root `2049:2` 同 boot 实测旧 Core 两族 PREPARE 的预期 CONFLICT 触发 Ubuntu Python Apport 路径，两个发布实例触及 64 MiB cgroup 限额并被内核终止。Core `df5ae00aac` 将预期拒绝收口为固定日志和退出 1。3 项定向测试通过，Git/JDK 21 构建候选的 233 个 JAR、原 Rust worker 和单元保持。原生解包、续配后实际 Main 两笔 SCOPE 成功，两族同类 CONFLICT 均正常拒绝，四实例保持 64 MiB，内核无 OOM。直接峰值仅采到一个实例。无新发布意图或回执，原 modem 三文件及两份凭据保持。两包 installed，Main disabled/inactive，VM 正常停放并重复冷读。 补父项 #3、#6、#10、#12。冻结 **0/12**，外部延期仅原三项。 [正式候选、运行日志和冷读](../../../../.management-close-20261116/core-rtc-hotplug-20261120/firewall-worker-memory-20261125/HANDOFF.md)

隔离 Parallels ARM64 Root `2049:2` 正式 Main 以 kurad、NNP 1 和 CapEff 0x400 持有真实 Linux 软件 watchdog。同包原生解包的设备调用实证保活及 Magic Close，Core 进入 unpacked，正常关机后在原 Root 一次续配。实际关键组件超时使 Main 进入 REBOOT_REQUESTED，代理不可达返回 UNKNOWN，300.8214 秒后停止保活，内核软件 watchdog 到期且按设置抑制重启。另测 nowayout=1 时原生 dpkg 拒绝解包，原 Main PID 和五份文件保持。两包 installed、原 modem 三份文件和两份凭据保持，Main disabled/inactive，VM 正常停放，停机冷读重复一致。Core `64316e9c7b` 补齐现行 Main recovery 条件的严格测试期望，7 项定向通过。 另发现正式 firewall 初始模型 CONFLICT 后，发布 worker 在两次 Main 启动均触及 64 MiB cgroup 限额，保留实际内核与 systemd 日志，列为 #6、#10 内部待查。 补父项 #3、#12。冻结 **0/12**，外部延期仅原三项。 [实装、清理和停机冷读](../../../../.management-close-20261116/core-rtc-hotplug-20261120/watchdog-formal-device-20261125/HANDOFF.md)

父项 #1、#3、#12 补证。Core `5a31199e23` 支持 Ubuntu SQLCipher SDK，其 Debian JNI 与原发行件一致。Core `21b4a09f8c` 用发行限定符和 JNI 摘要形成独立驱动版本，使正式 Felix 热更新替换运行中的 JDBC bundle。ARM64 Root `230:257` 六包实装，在各自同一 JVM 完成引擎双向切换，移除 .so.1 后用 .so.2 冷启读回原库及页完整性。原两库、三份快照与原 AES 凭据指纹保持，正常停机冷读通过。 [实装与发行](../../../../.management-close-20261116/core-rtc-hotplug-20261120/sqlcipher-abi-maintenance-20261125/HANDOFF.md)

父项 #3、#6、#9、#12 补证。隔离 Parallels ARM64 Root `2049:2` 的正式 Main 通过真实 ModemManager 执行虚拟 modem 单次 Reset。原号 `0d00e0b2-e0f3-464b-a6e3-500c0f341206` 的 UNKNOWN 发送记录和已提交结果跨正式 Core `b75eb19368-1` 与 Networking `98ce41482f-1` 的共同 remove、purge、解包和同 Root 跨正常关机续配。正式 Main 在虚拟设备缺席时只查原结果为 APPLIED 并确认送达，原 intent 和 result 完整指纹保持，设备端只有一次 Reset。两包 installed、Main disabled/inactive，VM 停放及冷读均已通过。 [实装证据](../../../../.management-close-20261116/core-rtc-hotplug-20261120/modem-joint-purge-20261125/HANDOFF.md)

父项 #3、#6、#9、#12 补证。独立 Parallels ARM64 同一审查副本 Root `2050:2` 安装正式 Core `b75eb19368-1`、Networking `98ce41482f-1`，原 Agent 的 43 份文件保持。真实 ModemManager 经正式 Main 接入回环虚拟 CDC ACM，派发一次 Reset 后原号返回 UNKNOWN。后继正式 Main 只读取该原号 APPLIED 并确认送达，保留原 intent/result 的 inode、权限和摘要。新 boot 核对临时设备已释放、原网络接续、原回执及三个包后正常停放。表中 #6 的虚拟目标实际 Reset 与原号恢复已有证据，真实未决回执下的原生包维护矩阵仍待。 [正式配对、原号及冷读](../../../../.management-close-20261116/core-rtc-hotplug-20261120/modem-installed-virtual-20261125/HANDOFF.md)

父项 #3、#6、#9、#12 补证。隔离 Root `229:257` 的 Networking 普通 Depends 候选原生解包后旧 Core 拒绝 configure，正式 Main 在 JVM 前拒绝。 `fa4a4e9afc` 将初始化能力移至 Pre-Depends。旧 Core 实测在解包前拒绝，随后正式 APT 按 Core configured、Networking 解包的顺序完成升级，Core 八个阶段验签。八笔原 firewall 回执和模型跨实际 remove、purge、unpack、同 Root 新 boot 续配保持，新 boot 两份 ACK 读回。修复无变化发布更换文件 inode 的 `98ce41482f` 通过 88 项 JDK 21 定向测试，正式后继 Main 六笔零内核命令保持模型原 inode 和时间戳。九个 ACTIVE bundle 对包、22 笔冷读通过，六包正常停放。 [包维护、原号及停机读回](../../../../.management-close-20261116/core-rtc-hotplug-20261120/firewall-capability-upgrade-20261125/HANDOFF.md)

父项 #3、#6、#12 补证。Networking `0ff9d17989` 与 Core `b75eb19368` 配对首装 firewall 发布。86 项 JDK 21 和 15 项 ARM64 测试通过。隔离 Root `224:257` 正式六包首装、Main 双族首次发布，veth 每族只允许指定来源的 UDP。沿同 Root 保留两笔首发回执，仅实施两笔撤回，基线规则及站点保护核对通过。正常关机后同 Root 新 boot 由正式 firewall 服务恢复两族原模型，Main 停用，18 笔旧回执保持，最终 20 笔冷读通过。原 `223:257` 保持停放。 [双栈实装、撤回、启动恢复及冷读](../../../../.management-close-20261116/core-rtc-hotplug-20261120/firewall-managed-initial-model-20261125/HANDOFF.md)

父项 #3、#6、#12 补负向证据。隔离 ARM64 Root `223:257` 的正式 Core `78b75b44ad-1` 与 Networking `83f237e-2` 上，独立 veth 双族各两个来源均未通过基线 UDP。正式 Main 的一次 IPv4 规则在 typed broker PREPARE 被拒绝。首装仅配置链框架，Java 仍用内存默认规则作为预期前态。没有新 receiver 意图或结果，原配置快照和内核基线保留，VM 正常停放。首次发布及允许、拒绝、撤回的实际流量继续待验。 [正式拒绝与停机冷读](../../../../.management-close-20261116/core-rtc-hotplug-20261120/firewall-typed-packets-20261125/HANDOFF.md)

父项 #1、#3、#12 补证。Docker `766590c` 从 Git 用 JDK 21 和 Maven 重建正式 ARM64 前驱与候选，Core 由 `dc1b0cbbf4-3` 升到 `78b75b44ad-1`，GWT、Deployment、Wires 摘要保持。隔离同卷 Root `39:10057850` 的首个 PID 1 Main 保存两份 PKCS#11 原快照。候选 Main 同卷读回，注册缓存失配和同名占用均被拒绝，另一槽及原槽重取得到原证书验签。Alpine SoftHSM `2.7.0-r0` 的目标令牌缺席、同 inode 归位后，原 JVM 的取钥均为 `NO_ENTRY`，旧句柄均失败，另一槽持续验签。同一候选镜像新进程通过只读 OSGi 探针再次用两份原证书验签。两份快照、TLS 和 16 份持久文件完整指纹保持，8 个 ACTIVE bundle 对包通过。 同 JVM 热恢复仍待。 [Docker 联合验收与冷读](../../../../.management-close-20261116/core-rtc-hotplug-20261120/docker-pkcs11-current-20261125/HANDOFF.md)

父项 #1、#3、#9、#12 补证。隔离 Debian ARM64 Root `222:257` 先以六组件正式装入 Core `dc1b0cbbf4-3`，在正式 Main 保存两个 PKCS#11 工厂配置、写入两份原快照并按原证书完成验签。Core 候选 `78b75b44ad-1` 一次解包并正常关机，停机读回 `unpacked` 及原围栏，新 boot 沿 `368dc18b-cb86-450d-8666-57069b270e97` 一次续配，8 份阶段记录验签通过。正式 JDK 21 Main 读回原两份快照，同名 JCA 注册占用和缓存失配均返回 `SERVICE_UNAVAILABLE`，外部占用保持，独立同库槽的旧句柄与新取钥验签通过。释放占用后原 OSGi 服务继续用原钥验签。10 个 ACTIVE bundle 与正式包匹配，AES 往返、Main 写租约、HTTPS 302、Agent 原号 READY/OFF 均通过。 两份原快照和原令牌数据均复核，六包正常停放。热恢复仍待。 [联合实装与冷读](../../../../.management-close-20261116/core-rtc-hotplug-20261120/pkcs11-formal-main-20261125/HANDOFF.md)

父项 #1、#3、#12 补证。Core `78b75b44ad` 校验 JCA 进程级注册的实际对象。名称已被占用时返回 `SERVICE_UNAVAILABLE`，缓存发现注册被替换时拒绝读取，停用只释放仍归属本服务的对象。101 项 Keystore 单测通过。独立 Debian ARM64 / JDK 21 的两个 SoftHSM 槽实测旧版接受同名冲突并移除了原注册。修复版拒绝，原句柄和新取钥均继续验签，同库另一槽与受控替换后的注册保留，当前所有者可以正常释放。正式 ARM64 候选 `78b75b44ad-1` 由 Git 构建，Keystore 35 个类与动态实测逐字节一致，其余 232 个 JAR、原 7 个受保护 JAR 和 13 个原生载荷不变。 候选实际 Main 系统级验收和 provider 热恢复继续内部待办。 [候选制品、原生对照与冷读](../../../../.management-close-20261116/core-rtc-hotplug-20261120/pkcs11-provider-registration-20261125/HANDOFF.md)

父项 #1、#3、#12 补证。独立 Debian ARM64 以签名索引取得的 SoftHSM `2.6.1-3` 与 JDK 21，使用两个临时不可导出 RSA 令牌。目标目录短暂缺席期间仅读同库另一槽时，目标原 inode 归位后原 JVM 可继续验签。缺席时访问目标则复现槽仍呈 present、`C_GetTokenInfo` 返回 `CKR_GENERAL_ERROR`、证书枚举为零。同 inode 归位后原 JVM 保持该状态，新建 SunPKCS11 provider 初始化同样失败。原 JVM 仍运行时，子进程可按原证书公钥验签，原 JVM 随后仍失败，另一槽各阶段持续验签。匹配的 Debian 源码表明文件目录刷新失败后 OSToken 保留无效状态。原 Root `207:257` 全程停机，上一轮正式 Main 冷读仍逐字节通过。 产品热恢复与替换仍待。 [独立探针、发行源码与冷读](../../../../.management-close-20261116/core-rtc-hotplug-20261120/pkcs11-cache-boundary-20261125/HANDOFF.md)

父项 #3、#9、#12 补充停机复核。停机复核确认 `57225` 原归档和终态回执摘要保持。当前在装的是 `c012586-2`，四份维护脚本、发布门禁 helper 和四个 Deployment JAR 与实测 `c012586-1` 一致。六包仍为 installed，保留原停机状态。 [当前停机读回](../../../../.management-close-20261116/core-rtc-hotplug-20261120/three-generation-deployment-receipt-20261125/current-stopped.json)

父项 #1、#3、#12 补证。原 ARM64 Root `207:257` 的正式 Core `87328bb-1` 中，实际 Main 在三份原快照及发布凭据下通过原 SoftHSM 不可导出私钥验签。保持同一 JVM、调用和 OSGi 服务，移存软件令牌目录时重新取钥为 NO_ENTRY，旧句柄签名返回 JCA ProviderException。同 inode 归位后原 JVM 仍为 NO_ENTRY，正常重启一次后用原证书恢复签名验签。原 data 与令牌 inode 保持，20 份保护记录、三份快照和原密钥、重加密回执保持。六份令牌文件完整指纹保持，token.object 摘要变化并留证。Main disabled/inactive，VM 正常停放，provider JAR 与原发行包一致并通过冷读。 [原令牌、同 JVM 观察与停机证据](../../../../.management-close-20261116/core-rtc-hotplug-20261120/pkcs11-runtime-absence-20261125/HANDOFF.md)

父项 #4、#12 补证。隔离 ARM64 Root `225:257` 的正式 Core `dc1b0cbbf4-3` 导出配置 API `1.9.0`。从原 Git 1.2/1.8 配置 API 子集编译两个 Java 11 目标消费者，分别核对 59/106 个公开 JVM 描述符，再在同一正式 Main 实接线读取原 Wires 快照和两项修订。1.9 消费者经正式 provider 核对存储图、逻辑读及配置边界拒绝。实录 45 个发行消费者和三份观测 bundle 的正式包接线，含当前 GWT、Networking、Wires、Deployment 和 Agent bridge。六包、原 69 份保护文件及 Agent 原号阶段保持。VM 正常停放，停机核对原 120 项及匹配正式包的 API JAR。 [历史编译、正式接线和停机证据](../../../../.management-close-20261116/core-rtc-hotplug-20261120/installed-config-api-compatibility-20261125/HANDOFF.md)

父项 #1、#3、#9、#12 补证。三代凭据 ARM64 Root `225:257` 的正式 Agent `f78610a-1` 未授权 remove 被原 prerm 拒绝。随后沿 `e2e8e0ef-3344-4ceb-bfc9-70864901183c` 保留式 remove/purge，Agent 缺席时正式 Main 读回三代原密文和九个 ACTIVE bundle，Wires 原图继续送达，HTTPS 302。同一原归档解包 INSTALL_ADMITTED 后正常关机，停机读回 120 份文件。新 boot 同 Root 一次续配至 REINSTALLED。55 条 Agent 状态路径和另外核对的 69 份保护文件保持，原本机 peer、材料、远程开关三授权仍 READY，实装 Agent 保持同代次 OFF/UNBOUND、五组能力为零。六包 installed，VM 正常停放并通过终态冷读。 [原 Agent 维护、独立 Main 和停机证据](../../../../.management-close-20261116/core-rtc-hotplug-20261120/published-generation-agent-retained-20261125/HANDOFF.md)

父项 #3、#6、#12 补证。隔离 ARM64 Root `225:257` 用当前三代 Core `dc1b0cbbf4-3`、Networking `83f237e-2` 的正式 OSGi provider 发布 BIND 双栈 LAN。named 实际仅监听获准的两个 LAN 地址，IPv4/IPv6 客户端四次 TCP/UDP 查询都到达同一独立 IPv6 上游并取得预期 A/AAAA 响应。通过正式 provider 撤回并恢复原 named 配置，内容、权限和属主保持，替换更新主配置 inode。三组原 intent/result/delivered 同 Root 停机读回。原六份 DHCP 文件、两笔 Deployment 回执等 56 份保护文件保持。六包 installed，Main 和 Agent disabled/inactive，VM 正常停放。 [BIND 实装、DNS 查询与冷读](../../../../.management-close-20261116/core-rtc-hotplug-20261120/published-generation-bind-dual-20261125/HANDOFF.md)

父项 #3、#6、#12 补证。隔离 ARM64 Root `225:257` 的当前已发布三代 Core `dc1b0cbbf4-3` 与 Networking `83f237e-2` 验证 DHCP 原号交接。原 apply `91d07fe8-82cd-4d77-a13f-59471b18614e` 仅派发一次。正式 Main 正常停服前后，原 dnsmasq 的 PID、invocation、启动 ticks、配置保持，隔离客户端两次实际 DORA 均获 `192.0.2.83`。后继 Main 的通用 compare 在未确认送达阶段返回 UNKNOWN，pair 查询后原号确认一次，原意图与结果保留原 inode 和摘要，未重发。独立 inverse 还原配置及 DHCP inactive。两次正式 OSGi Main 均只读核对同一 named 实例，未写 BIND。六包 installed，50 份保护文件保持，VM 正常停放，官方冷读原 89 项、两笔新操作的六份文件和十个实装载荷。 [DHCP 原号、两次租约和停机证据](../../../../.management-close-20261116/core-rtc-hotplug-20261120/published-generation-dns-dhcp-20261125/HANDOFF.md)

父项 #1、#3、#9、#12 补证。三代凭据 ARM64 Root `225:257` 用 GWT `b881a3a-2`、Wires `277954b-2` 正式联合升级。两个维护修订的业务载荷与 Git 来源正式归档逐字节一致。联合解包后 GWT 配置成功，Wires 的真实 postinst 注册锁等待 30 秒超时，留下半配置。正常关机冷读 89 项后，同 Root 新 boot 仅续配 Wires 一次。正式 Main 读取三代密文，九个 ACTIVE bundle 对包，原 Wires 快照 `1795544897896` 再送达，GWT 回环 HTTPS 302。原注册表、三快照、四个皮肤 conffile 和 43 份保护记录保持，含两笔 Deployment 原归档和回执。六包 installed，VM 正常停放，终态冷读 89 项。 [联合升级及冷读证据](../../../../.management-close-20261116/core-rtc-hotplug-20261120/three-generation-gwt-wires-upgrade-20261125/HANDOFF.md)

父项 #3、#9、#12 补证。隔离 ARM64 Root `225:257` 的正式 Core `dc1b0cbbf4-3` 中，实际 Java API `UnprivilegedExecutorService` 核对 `kura` 身份、工作目录、环境、字面参数与标准流。12 秒超时返回 143、`timedOut=true`，原命令与 worker 两个 PIDFD 均确认退出。实际 Main 正常停服回收另一异步命令及 worker，两个 socket inactive，同 UID 独立进程保持，回调返回 `EXECUTOR_RESULT_UNAVAILABLE`。四份正式执行器文件对包，六包 installed，原 Deployment 两笔归档与回执所在的 43 份保护记录及三个快照保持。VM 正常停放并通过冷读。 [Java API、原进程和冷读证据](../../../../.management-close-20261116/core-rtc-hotplug-20261120/typed-executor-lifecycle-20261125/HANDOFF.md)

父项 #1、#3、#9、#12 补证。Core `dc1b0cbbf4` 使用 Git、JDK 21、Maven 与原 Bundle-Version 限定符 `202611230517` 生成 ARM64 `-3`。7 个受保护 JAR 和全部 233 个运行时 JAR 逐字节匹配，标准发行清单绑定 `225:257` 原前驱，专用布局计划为空。实装沿 `db47e1ec-2991-40b7-97ab-fd4d64b75b7a` 解包、正常关机、停机读回和同 Root 新 boot 续配，8 份阶段记录含 READY/configured 验签通过。数据和 Deployment 目录原 inode、43 份保护记录保持，三份快照内容和权限保持且 inode 随恢复复制更新。正式 Main 读回三代密文，9 个 ACTIVE bundle 对包，原 Wires 图继续送达，HTTPS 302，Agent 原号 READY/OFF。六包 installed，VM 正常停放，停机读回 73 项。 [原位升级、验签和停机证据](../../../../.management-close-20261116/core-rtc-hotplug-20261120/core-standard-published-successor-20261125/HANDOFF.md)

父项 #3、#9、#12 补证。Core `dc1b0cbbf4` 经 Git、JDK 21 与 Maven 正式生成通用 `dc1b0cbbf4-2` ARM64 deb，两处维护钩子的专用续接计划为空。新隔离 Root `221:257` 一次 APT 首装当前六包，dpkg 审计为空。Core 凭据 READY，合法 Main 和 Agent 实际 NoNewPrivs 1，Main 的 FD 9 与原生写租约锁配对。九个选定 OSGi bundle ACTIVE 且实装 JAR 对包，AES 往返和 HTTPS 302 通过。Agent 新原号配对 READY/OFF，六包保持 installed，正常停放。233 个 JAR 中 188 个与基线字节相同，其余 45 个仅 Bundle-Version 字段变化。通用首装包的已发布代合同有 7 个 JAR 摘要差异，原 `225:257` 保持原配对及版本。原 `219:257` 保持停放。 [通用发行、首装和停机冷读](../../../../.management-close-20261116/core-rtc-hotplug-20261120/core-standard-release-20261125/HANDOFF.md)

父项 #3、#12 补证。Core `dc1b0cbbf4` 从 Git 导出到无网络 Debian ARM64/JDK 21 夹具，84 个原生合同模块共 1,294 项通过，macOS ARM64 生成规则另有 7 项通过。需独立 systemd 或宿主编排的 12 个入口逐项保留。相较已验 Core `0823d14779`，Git 差异仅在发行目录，Core 打包 POM 仅新增两个 systemd 单元映射。原 `219:257` 专用前向包及 `225:257` 三代正式包与 82 模块、2,173 项既有 Core/Equinox 基线的配对发行包有 233 个逐字节相同的 JAR。当前 Git 的 13 项运行时改动在 `225:257` 实装全部匹配。原 `219:257` 停机读回确认保留专用续接载荷。两处保持停放。 [原生合同、发行一致性及停机证据](../../../../.management-close-20261116/core-rtc-hotplug-20261120/core-current-native-and-osgi-parity-20261125/HANDOFF.md)

父项 #1、#3、#9、#12 补证。隔离 ARM64 Root `225:257` 使用正式 Wires `277954b`。正式 Main 只提交一次 Timer→Logger 测试图并实际送达，原图快照 `1795544874310`。原生 purge 后两个 provider bundle、工厂服务和图服务缺席，Core 一次保存新快照 `1795544897896`，图摘要、Timer 间隔、Logger 设置保持，缺席期间零送达。VM 正常关机后读回 Wires `not-installed` 和新快照。新 boot 同一 Root 重装同一正式归档，只读显示从新快照恢复原节点与 `0→0` 边，Logger 实际收到事件，两份实装 provider JAR 与 deb 一致。原快照、33 份保护记录、两笔 Deployment 原回执及 Agent READY/OFF 保持。六包 installed，正常停放。 [业务、快照与停机冷读](../../../../.management-close-20261116/core-rtc-hotplug-20261120/three-generation-wires-absent-snapshot-20261125/HANDOFF.md)

父项 #3、#9、#12 补证。隔离 ARM64 Root `225:257` 使用 Git `b881a3a` 正式 GWT 包，实际 remove、purge、解包。GWT 缺席时正式 Main 的 Core、Deployment、Networking、Wires 八个选定 bundle 保持 ACTIVE，回环 HTTPS 返回 404。重装在注册锁处中断 postinst，正常关机并读回 `half-configured`。同 Root 新 boot 仅续配一次，其他注册行顺序及元数据保留，GWT 唯一注册行恢复。正式 Main 九个选定 bundle ACTIVE，Web2 实装 JAR 与正式归档摘要一致，HTTPS 302 至 `/admin/console`。33 份保护记录、两笔 Deployment 原归档和回执、原数据 inode 与 Agent READY/OFF 保持。六包 installed，VM 正常停放并通过停机冷读。 [实装与停机冷读](../../../../.management-close-20261116/core-rtc-hotplug-20261120/three-generation-gwt-maintenance-20261125/HANDOFF.md)

父项 #3、#9、#12 补证。三代凭据六组件 Root `225:257` 的新业务号 `57226` 仅派发一次。Java 正式归档落盘后保留 `phase=returned`，待发布注册表仍未交换。实际 `dpkg --unpack` 候选 `c012586-2` 在旧版 `prerm upgrade` 和候选 `prerm failed-upgrade` 均拒绝。旧版 `-1` 保持 installed，17 份已装文件、四份发布材料及 33 份保护记录通过原位核对。正式 Main 启动恢复后，同 Root 同号仅 GET 确认 `COMPLETED`，同一 `-2` 归档随后正式升级至 installed。原 `57225` 的归档和终态回执保持。Main、Agent 和六包夹具正常停放，停机冷读通过。独立 ARM64 Debian 以合成存在标记实测 `config-files` 的 `postrm` 拒绝直接 purge，撤除标记后原生 purge 通过。 [正式验收与冷读](../../../../.management-close-20261116/core-rtc-hotplug-20261120/three-generation-deployment-upgrade-20261125/HANDOFF.md)

父项 #3、#9、#12 补证。三代凭据六组件 Root `225:257` 正式升级 Deployment 至 `c012586`。实际 Java 发布原号 `57225` 已写入 `phase=returned`，归档已提交，注册表仍待交换时中断该 JVM。原生 `dpkg --remove` 和 `dpkg --purge` 各一次均由 Deployment 钩子退出 1，包负载仍 installed，四份现场文件及 33 份保护记录保持。显式启动正式 Main 后，启动恢复将原回执和待发布注册表以原 inode 落位。只含 GET 的 OSGi 探针对同一原号查得 COMPLETED，Felix 读回原包版本。Main 和 Agent 正常停放，六包 installed，停机读回通过。 [正式拒绝与同号只读结算](../../../../.management-close-20261116/core-rtc-hotplug-20261120/three-generation-deployment-receipt-20261125/HANDOFF.md)

父项 #1、#3、#12 补证。在独立 Root `226:257` 通过正式包联合验收后，Core `dc1b0cbbf4` 已在原 Root `219:257` 沿 `b372aa65-d749-4af0-9a28-eb934b4ed978` 和原围栏正式 unpack、configure 到 `ii`。原三份签名的字节和 inode、23 份保护文件、原数据 inode `219:23851` 保持。追踪确认原 layout 零重派，原 returned／complete 缺席，历史结果 UNRESOLVED，独立效果证明、READY 和 configured 验签通过。正式 JDK 21 Main 读回两代密文，三个 OSGi bundle ACTIVE，回环 HTTPS 404。Main disabled、VM 正常停机，停机文件复核了版本、postinst 和七份记录。 [原号联合验收及停机冷读](../../../../.management-close-20261116/core-rtc-hotplug-20261120/issued-layout-forward-20261125/HANDOFF.md)

父项 #3、#12 补证。Core `dc1b0cbbf4` 增加 Root、原号、原三份签名、旧控制与候选控制绑定的 forward 包路径。新效果证明保留原 layout 结果为 UNRESOLVED，原 returned／complete 保持缺席。108 项 ARM64 测试及原生小包 dpkg 状态探针通过，正式候选数据归档与 `22e54511df` 完全一致。原 `219:257` 停放，原包续接和 READY、Main、OSGi、provider 联合验收仍待。 [同号正式候选与实证范围](../../../../.management-close-20261116/core-rtc-hotplug-20261120/issued-layout-forward-20261125/HANDOFF.md)

父项 #3、#12 补证。Core `55c973efc2` 在发布配对 vendor unit 并刷新 PID 1 后核验 LoadState，再允许签发布局意图。106 项 ARM64 定向测试通过。原 `219:257` 停机实读仍 iF，正式 postinst 和 14 个 vendor unit 对包匹配，六个 shell 文件仍是 0644，`.data`、`data` 缺席。原号、原围栏和原历史保持。前向效果核定的准入与允许写入已列出，续接包和原号 READY、Main、OSGi、provider 待验。 [停机原包实读与原号前向续接设计](../../../../.management-close-20261116/core-rtc-hotplug-20261120/issued-layout-review-20261125/HANDOFF.md)

父项 #3、#12 新增正式 ARM64 证据。Root `219:257` 的候选 `22e54511df` 真实复现 dpkg 发布顺序中断，按原号续解包至 iU，原签名与 inode 保持。后续 iF 停点已归因于 PID 1 中同一 vendor recovery unit 的 EBUSY，刷新后 loaded，原签名 layout 意图待受控续接，READY／后继 Main 仍待。 [正式原号、验证边界和停放](../../../../.management-close-20261116/core-rtc-hotplug-20261120/native-dpkg-control-resume-20261124/HANDOFF.md)

相关记录：[冻结台账](OPEN-TASKS.md) · [G021](actual-g021-service-pid-runtime-checkpoint.json) · [G022](actual-g022-plc4x-validation-checkpoint.json) · [新版四网关部署](../../e2e-deployment-guide-20261122.md) · [当前六组件证据](../../../../.management-close-20261116/core-rtc-hotplug-20261120/six-current-native-20261124/HANDOFF.md) · [Networking 中断续配证据](../../../../.management-close-20261116/core-rtc-hotplug-20261120/six-networking-purge-20261124/HANDOFF.md) · [当前配对后继联合升级](../../../../.management-close-20261116/core-rtc-hotplug-20261120/six-companion-current-update-20261124/HANDOFF.md) · [已绑定联合维护实现](../../../../.management-close-20261116/core-rtc-hotplug-20261120/companion-retained-enrollment-20261124/HANDOFF.md) · [已绑定实装与原目录修复](../../../../.management-close-20261116/core-rtc-hotplug-20261120/companion-bound-full-chain-20261124/HANDOFF.md) · [修复后已绑定联合升级合并验收](../../../../.management-close-20261116/core-rtc-hotplug-20261120/companion-bound-fixed-20261124/HANDOFF.md) · [Core 保留重装现场](../../../../.management-close-20261116/core-rtc-hotplug-20261120/core-retained-provider-20261124/HANDOFF.md) · [Deployment 门禁与原号续配](../../../../.management-close-20261116/core-rtc-hotplug-20261120/deployment-assertion-resume-20261124/HANDOFF.md) · [Core 密钥服务配对](../../../../.management-close-20261116/core-rtc-hotplug-20261120/core-unit-admission-20261124/HANDOFF.md) · [正式 Main NoNewPrivs 与写租约](../../../../.management-close-20261116/core-rtc-hotplug-20261120/core-main-no-new-privs-20261124/HANDOFF.md) · [时钟与重启代理配对](../../../../.management-close-20261116/core-rtc-hotplug-20261120/core-native-current-contracts-20261124/HANDOFF.md) · [重启代理启停与 Networking 配对](../../../../.management-close-20261116/core-rtc-hotplug-20261120/reboot-projection-20261124/HANDOFF.md) · [重启持久回执及跨 boot](../../../../.management-close-20261116/core-rtc-hotplug-20261120/reboot-systemd-accepted-20261124/HANDOFF.md) · [typed broker 共用配对与实装](../../../../.management-close-20261116/core-rtc-hotplug-20261120/broker-common-pairing-20261124/HANDOFF.md)

[未绑定且 OFF 网关的六组件联合维护证据](../../../../.management-close-20261116/core-rtc-hotplug-20261120/unbound-switch-companion-20261124/HANDOFF.md)

[当前 Docker HTTPS 首装恢复与同卷重部署](../../../../.management-close-20261116/core-rtc-hotplug-20261120/docker-tls-bootstrap-recovery-20261124/HANDOFF.md)

[未绑定 peer 的 Agent 保留清除和重装证据](../../../../.management-close-20261116/core-rtc-hotplug-20261120/unbound-peer-retained-removal-20261124/HANDOFF.md)

[已初始化六组件跨正常关机保留重装](../../../../.management-close-20261116/core-rtc-hotplug-20261120/six-cohort-retained-peer-20261124/HANDOFF.md)

[CA 联合激活持锁与原审批证明](../../../../.management-close-20261116/core-rtc-hotplug-20261120/ca-quorum-publication-20261124/HANDOFF.md)

[CA 保管名单、策略审批、同 Root 全链补证与 ARM64 发行](../../../../.management-close-20261116/core-rtc-hotplug-20261120/ca-custodian-policy-20261124/HANDOFF.md)

[CA 原号批量登记、冲突回滚及实际 mTLS](../../../../.management-close-20261116/core-rtc-hotplug-20261120/ca-bulk-enrollment-20261124/HANDOFF.md)

[批量公开 CSR 交接、签回原号与 mTLS](../../../../.management-close-20261116/core-rtc-hotplug-20261120/ca-csr-signing-handoff-20261124/HANDOFF.md)

[批量续期原号、两份 v2 收据与代际冲突回滚](../../../../.management-close-20261116/core-rtc-hotplug-20261120/ca-bulk-renewal-20261124/HANDOFF.md)

[两台 ARM64 正式 Agent 的原 v2 导入、OFF 和任务冷读](../../../../.management-close-20261116/core-rtc-hotplug-20261120/ca-native-pair-renewal-20261124/HANDOFF.md)

[静态 Main 的早期断言、正式联合重装及原未决停点](../../../../.management-close-20261116/core-rtc-hotplug-20261120/deployment-early-main-assertion-20261124/HANDOFF.md)

[PKCS#11 软件令牌跨保留重装的缺席和恢复证据](../../../../.management-close-20261116/core-rtc-hotplug-20261120/pkcs11-retained-software-token-20261124/HANDOFF.md)

[本轮原号、只读证据和停放](../../../../.management-close-20261116/core-rtc-hotplug-20261120/pkcs11-generation-rewrap-20261124/HANDOFF.md)

[本轮正式包、同号续办和停放](../../../../.management-close-20261116/core-rtc-hotplug-20261120/container-generation-credentials-20261124/HANDOFF.md)

[原号升级、保留移除与 iF 停点](../../../../.management-close-20261116/core-rtc-hotplug-20261120/published-cohort-successor-20261124/HANDOFF.md)

[原签名意图、正式续配和只读证据](../../../../.management-close-20261116/core-rtc-hotplug-20261120/retained-layout-control-repair-20261124/HANDOFF.md)

父项 #1、#3、#12 补证。Core `419d1e5578` 在已配对 vendor unit 持久发布后、layout 意图签发前执行 PID 1 刷新。ARM64 定向 76 项通过，正式 deb 按 Git 重建且原数据归档逐字节一致。隔离 ARM64 Root `225:257` 沿独立原号 `085bb890-38af-45c3-8243-a2f20ac116a3` 完成真实保留重装。追踪核对 vendor 发布、刷新返回 0、签发和一次派发的先后，dpkg ii、READY/configured 验签。显式 Main PID 15472 实载 Crypto、配置 facade、快照 provider 并读回两代密文。原数据 inode、23 份凭据和历史记录保持，Main disabled，VM 正常停放。

原 Root `219:257` 的 `b372aa65-d749-4af0-9a28-eb934b4ed978` 仍为 iF，原 layout 意图和围栏保留。现有签名缺可信阶段返回，原号前向续接待审查。完整 typed brokers、六组件和 provider/launcher 的维护矩阵仍待。冻结 **0/12**，外部延期仍仅原三项。 [本轮正式联合链和冷读](../../../../.management-close-20261116/core-rtc-hotplug-20261120/retained-layout-pid1-20261124/HANDOFF.md)

父项 #1、#3、#12 补证。Core `1202b49461` 允许带原 purge 完成签名、两份受审前序 manifest 和同一发布原启动回执的后继 reader。ARM64 81 项通过。705 个发行载荷成员只有 manifest 列表变更，233 个 JAR 保持。隔离 Root `225:257` 沿新号 `327316f8-83dc-4afc-b3be-b5f2af93687d` 完成真实 remove、purge、后继解包配置和 READY。合成数据先归档，再正常走 purge。22 份原凭据及历史、三个原 launcher 保持，正式 Main 实际读回原两代密文，正常 disabled 停放。

原 `219:257` 已签发意图仍缺可信返回，保持原号未决。跨代保留重装与其它 typed brokers、六组件、provider/launcher 尚待。冻结 **0/12**，外部延期仍仅原三项。 [跨 reader manifest 的正式实装与冷读](../../../../.management-close-20261116/core-rtc-hotplug-20261120/cross-cohort-purge-20261124/HANDOFF.md)

父项 #1、#3、#12 补证。Core `4b7531d437` 把已核验的双前序 manifest 和原启动回执门禁接入保留重装，ARM64 104 项通过。隔离 ARM64 Root `225:257` 的被移除 reader 与原发布 reader 摘要不同，后继正式包沿新号 `f4f9930d-70ca-4751-a222-785d7ea37090` 解包后正常停机，再沿原围栏续配至 ii。原保留签名、数据 inode 225:25464、四份原数据文件、23 份保护记录及三个 launcher 保持。实际 READY/configured 验签，正式 OSGi Main 读回两代密文，Main disabled，VM 已停放。已修正通用首装尾注对自动启动的错误承诺。

原 `219:257` 签发后的布局停点仍缺可信返回，保留原号、数据和围栏。typed brokers、六组件及 provider/launcher 继续内部待验。冻结 **0/12**，外部延期仍仅原三项。 [跨 reader manifest 保留重装、原围栏和冷读](../../../../.management-close-20261116/core-rtc-hotplug-20261120/cross-cohort-retained-20261124/HANDOFF.md)

父项 #3、#9、#12 补证。Core `4b7531d437` 的已发布代 Root `225:257` 正式加装五组件。Agent `f78610a` 加入配对 Core 0400 凭据 drop-in 核验，ARM64 定向共 150 项通过。正式 Agent 新维护 APPLIED，原初始化保持，原配对号检查 NOT_FOUND 后落盘 READY，三份本机授权保持 OFF。Deployment `1642074 → 2bdb66d` 独立升级，解包时实际断言拒绝启动，配置后退役。升级前后正式 OSGi Main 各核对 8 个 ACTIVE bundle，23 份原保护记录、原 data inode 和授权保持，Main/Agent 停放。

同一后继 JVM 的 ownership、journal 和双族回环 ICMP 读请求因共用单元核定拒绝。原 219:257 的布局意图仍缺可信返回。冻结 **0/12**，typed brokers、六组件及 provider/launcher 其余矩阵继续内部待验，外部延期仅原三项。 [六包、Deployment 断言和 typed broker 拒绝](../../../../.management-close-20261116/core-rtc-hotplug-20261120/six-published-companion-20261124/HANDOFF.md)

Core `07a5b74b8f` 严格核验 Root 0400 凭据声明，`de68644b3b` 允许已发布代凭据预检配对实际 Networking 的 vendor 依赖声明。ARM64 定向 87 项及发布/恢复 58 项通过。隔离 Root `225:257` 的首候选在生成升级副本和签发布局前明确拒绝，旧版仍 installed。同 Root 沿 `2fcbddca-4a83-4bf4-b659-0255b7450aea` 由后继正式包解包配置，READY、configured 和原数据交接验签。原 data inode、四份数据、23 份保护文件和 Agent READY/OFF 授权保持。正式 OSGi Main 读回两代密文，ownership 通过。低权限 journal 和 IPv4/IPv6 回环 ICMP 仍拒绝，Main/Agent 与 VM 正常停放。

补父项 #1、#3、#9、#12。原 `219:257` 布局意图保持未决。typed brokers、六组件及 provider/launcher 的完整矩阵仍待，冻结 **0/12**，外部延期仍仅原三项。 [同围栏原位升级、ownership 与冷读](../../../../.management-close-20261116/core-rtc-hotplug-20261120/broker-published-main-20261124/HANDOFF.md)

Core `c8e04fa570` 通过 ARM64 定向 87 项。PID 1 将专用 Root 0600 核验连接交给低权限 worker。Root 绑定 PIDFD、实际 worker 身份、vendor unit、cgroup 和 Main invocation，核对正在使用的私有声明。隔离 Root `225:257` 沿 `89d10916-8b4f-45c9-a79f-2b586508b041` 原位升级，六包 installed，READY/configured 及原 data 交接均验签。正式 OSGi Main 读回两代密文，ownership、Journal、IPv4/IPv6 回环与普通执行器有实际通过证据。普通执行器的私有工作目录被内核拒绝，使用其可遍历的 `/usr` 补证成功。专用 socket 拒绝直接 kurad 和非配对 Root，Main/Agent 与验证 VM 已停放。

补父项 #1、#3、#9、#12。独立 reboot broker 的发布代 actor 和 client 配对仍拒绝。原 `219:257` 已签发布局仍未决。冻结 **0/12**，外部延期仅原三项。 [低权限 broker 正式证据与停机冷读](../../../../.management-close-20261116/core-rtc-hotplug-20261120/typed-generation-proof-20261124/HANDOFF.md)

Core `b992c87a39` 对发布代 Main 核对 Root 0400 私有声明和 systemd 加载状态。ARM64 118 项通过。独立 Debian ARM64 Root `225:257` 正式升级六包共存场景，READY/configured 和原数据交接验签。正式 Main 的同 UID 外部进程先被拒绝，随后正向只派发一次重启。首个新 boot 冷读原号 `6b29f751-8c48-474d-b3b1-70f4584ccf4f` 为 ACCEPTED。正常恢复 OrbStack 命令通道后，正式 Main 只读结算原号，原 actor intent 和回执的 inode、字节未变，pending 清除。Main disabled，VM 正常停放。

补父项 #3、#9、#12。原 `219:257` 仍为签发后未决，原号和原围栏保留。其余 typed broker、provider/launcher 与完整六组件矩阵仍待。冻结 **0/12**，外部延期仅原三项。[正式已发布代重启、原号结算和停放](../../../../.management-close-20261116/core-rtc-hotplug-20261120/reboot-published-layout-20261125/HANDOFF.md)

Core `9e962dc502` 接入正式 Networking 依赖声明，`7a176edd6a` 接入现代 Agent 包中准确且未启用、未配置的旧服务文件。ARM64/JDK 21 最终 60 项通过。隔离 Root `225:257` 两次正式原位升级，六包 installed，READY/configured 和原数据交接验签。原准备与验证两号跨后继版按 inode、摘要保持。正式新发布 `47b3d57c-b80c-4496-90a2-36a4c80e8d84` 在首份 LXC 投影已落盘后中断，经原签名检查点沿原号恢复，保留原两代条目并增至三代。独立启动门禁和实际 OSGi 新旧密文读回通过。Agent 保持原 READY/OFF，原数据与 reboot 回执保持，Main 与 VM 正常停放。

补父项 #1、#3、#12。原 `219:257` 的布局意图仍缺可信返回，原号和围栏保留。完整 typed brokers、provider/launcher 及六组件矩阵仍待。冻结 **0/12**，外部延期仅原三项。 [同 Root 原号发布、正式启动及冷读](../../../../.management-close-20261116/core-rtc-hotplug-20261120/generation-networking-20261125/HANDOFF.md)

正式 Core `7a176edd6a` 的三代凭据 Root `225:257` 以原 Networking `83f237e-2` 正式 remove、purge、解包和中断续配。Networking 缺席时实际 Agent 依原配对拒绝，独立 Main 读回三代密文并返回 HTTPS 302。精确终止注册锁等待者后 dpkg 为 half-configured，同 Root 一次续配恢复原 vendor、唯一注册表行及原号 READY/OFF。33 份原记录、原 data inode 和第三代均保持。恢复后 OSGi 实装 8 个 ACTIVE bundle 逐一与正式 deb 匹配，Agent 实际无能力且 OFF/UNBOUND。两服务及 VM 正常停放。

补父项 #1、#3、#9、#12。原 `219:257` 官方停机回读实证 iF，原三份签名和围栏在，仍无可信返回。其余 typed brokers、provider/launcher 与六组件矩阵仍待。冻结 **0/12**，外部延期仅原三项。 [三代现场 Networking 原生维护与停机读回](../../../../.management-close-20261116/core-rtc-hotplug-20261120/three-generation-networking-maintenance-20261125/HANDOFF.md)

## 2026-11-25 活跃 Chrony 的正式 Core 包维护

隔离 Debian ARM64 Root `214:257` 的正式 Core `0823d14779-1` 实测 chronyd 正在接收回环 NTP 时同版原生解包。原 PID 1622 的 PIDFD 未退出，有效回包从 1 增至 13。沿原维护围栏 `383addd3-fbb6-498c-beb5-d3783a76d22d` 一次 configure 返回 0，已装包的 broker 配置阶段有序停止 chronyd 并维持 disabled。21 份原号阶段记录验签通过。后继正式 Main 通过原 ClockService 恢复配置，三笔新操作均 APPLIED。38 份旧文件的字节与身份保持。Core 和 Chrony installed，两服务 disabled/inactive，VM 正常停放，冷读 84 项重复一致。

补父项 #3、#9、#12。冻结 **0/12**，外部延期仅真实 NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。 [实装与停机冷读](../../../../.management-close-20261116/core-rtc-hotplug-20261120/chrony-running-core-maintenance-20261125/HANDOFF.md)

## 2026-11-25 API2 正式归档解析与 YOFC 消费者

以七份正式 deb 为输入，隔离 JDK 21 Equinox 的九个选定消费者在发行组合全部解析，仅替换 API2 实验 JAR 后全部解析失败。核定四个受影响的包导出及两类、七个公开描述符的删除，记录 28 个正式归档 JAR 的相关导入和引用检查。Position REST 实装 JAR 直接导入 message，Comm 导入缺席。

从 YOFC Git `a652e29910` 构建四个 JAR 及四个 DP。javac 实际加载的 API JAR 与正式归档同摘要，四个 channel 导入为 `[1.4,2)`，其中三个另有 driver `[1.0,1.1)`。这四个 JAR 的精确常量池和 DS XML 检查未发现已删项的直接引用。四个 Core、OPC UA、Position Comm、Wires 共七个冻结变更的正式联合切换、完整消费者和 provider 激活继续内部待验。关联父项 #4、#11、#12。

停机复核再次核对 Deployment `225:257` 原号 `57225` 的归档和回执。真实 remove、purge 拒绝覆盖 installed 的 prerm。独立 config-files 的 postrm 拒绝证据使用合成存在标记。当前正式钩子在建立围栏前和锁内检查发布回执，尚无受支持的现场转换可将同一真实未结算发布带入 config-files。保留该组合的证据边界。冻结 **0/12**，外部延期仅真实 NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。 [核对与冷读](../../../../.management-close-20261116/core-rtc-hotplug-20261120/api2-release-resolver-20261125/HANDOFF.md)

## 2026-11-25 API2 联合 Debian 实装与混装门禁缺口

Core `ab25377373` 及十二个 sibling 的候选包和重建的 YOFC 产物，在隔离 Equinox 中解析 286 个 bundle、34 个 API2 直接消费者。首轮 Position 因旧快照 Comm SPI 被拒绝，使用配对 Core 发行输入重新组装通过。JDK 21 Core 宿主报告 2177 项（2 项跳过），ARM64 2177 项（零跳过），均零失败、零错误，包含 81 项 Failsafe。

独立 Debian ARM64 Root `251:257` 一次正式 APT 安装 13 个候选 deb 加 Agent，14 包 installed。Core `32c25d68-cf3a-4ecc-96cf-5e3566f270c4` 的 READY、configured 等 19 份记录验签。正式 Main PID 12200、UID 995、NNP 1、内核独占写租约通过。30 个在装消费者 JAR 对包且均 ACTIVE，实际接线至 API 2.0.0，九个选定 provider ACTIVE，配置 API 1.9 仍有 65 个消费者。四个 YOFC 直接消费者已有构建和解析，实际 DP 安装激活待验。Main 与 Agent disabled/inactive，VM 正常停放，两次冷读一致。

真实归档的 APT 模拟分别放行 API1 Core 配 API2 siblings、API1 Wires 配 API2 Core。旧 ABI Core 的 Debian 版本排序更高。双向混装拒绝、原位换版、跨停机续配、remove/purge 仍待。原 Deployment ARM64 Root `225:257` 停机复核一致，沿用原号 `57225` 真实的 installed prerm 拒绝和同 Root 同号 GET COMPLETED，原归档、终态回执保留。

补父项 #3、#4、#11、#12。冻结 **0/12**，仅原三项外部延期。 [实装、APT 门禁对照与停机冷读](../../../../.management-close-20261116/core-rtc-hotplug-20261120/api2-cohort-20261125/HANDOFF.md)

## 2026-11-25 API2 Artemis XML 跨正式发布代与原生续配

隔离 Debian ARM64 Root `1048590:257` 实装 Core `c65c04741e`、Artemis `1bcdc5d-1` 及其余五包。JDK 21 的合法 Main 只提交一次 XML broker 配置，生成加密快照 `1795596044567`。正式密钥发布 `fe7085bd-d44b-448a-8058-192857b02242` 排空原 Main 并要求显式启动。另行批准启动后，正式 OSGi CryptoService 写读新活动钥的 KURA2 样本，Artemis 仍可凭原加密快照中的原用户表运行。

包内快照重加密入口对原号 `dbbe0e5f-bfb1-444e-b2fe-c7b55dc0c3a8` 返回 `AGENT_GENERATION_UNAVAILABLE`。同 Root 同号只读返回 `NOT_FOUND`，无操作目录，原快照保留原 inode 和摘要。保留共装 Agent 和原意图，后续包维护使用原密文。一次原生解包后正常停机，同 Root 新 boot 一次 configure，七包 installed，dpkg audit 为空。正式 Main 在首装、密钥发布后及续配后三次均拒绝错误口令，原用户以不同主题分别完成一次实际 MQTT 3.1.1 QoS 1 送达。19 份 Core 签名保持，四个密钥操作在新 boot 仅按原号查询且均为 SUCCEEDED。两服务 disabled/inactive，VM 正常停放，清理前后冷读一致。

Deployment 原 Root `225:257` 继续停放。原号 `57225` 的真实 installed prerm 对 remove、purge 的退出 1 和同 Root 同号 GET COMPLETED 已冷读复核，原归档、原回执和六包保持。原号未重发。独立 config-files 的 postrm 拒绝仍限于合成标记。

补父项 #1、#3、#4、#9、#12，冻结 **0/12**。Artemis 与 Agent 共装的快照重加密、TLS/其他协议及余下 typed brokers、六组件与 provider/launcher 矩阵仍为内部待办。外部延期仅真实 NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。 [正式证据和停机冷读](../../../../.management-close-20261116/core-rtc-hotplug-20261120/api2-artemis-generation-20261125/HANDOFF.md)

## 2026-11-25 API2 Artemis XML 双向 TLS 跨原生清除与冷续配

隔离 Debian ARM64 Root `1048591:257` 实装 Git 构建的 Core `c65c04741e`、Artemis `1bcdc5d-1` 和配对五包。JDK 21 下合法 Main 只提交一次 XML broker 配置，保存加密快照 `1795596595869`。实际 TLS 1.2 双向认证核对原服务端和客户端证书指纹，服务端拒绝缺客户端证书，客户端拒绝不受信任的服务端，正确双向 TLS 下 MQTT 仍拒绝错误口令。正确凭据经独立 Paho 客户端在回环完成一条 QoS 1 送达。

正式 remove 到 config-files，独立 purge 到 not-installed。缺包时合法 Main 仍读原加密快照，Artemis XML bundle 与监听均缺席。一次同包解包并正常停机，同 Root 新 boot 一次 configure。后继 Main 的 XML provider ACTIVE，恢复原快照、原 TLS 配置和原用户表，重复验证双向证书及 MQTT 鉴权，并在恢复主题送达一条新消息。原快照、两套密钥库和信任库、原提交及 19 份 Core 维护签名保持。七包 installed，dpkg audit 为空，Main 和 Agent disabled/inactive，VM 正常停放，重复冷读一致。

补父项 #1、#3、#4、#9、#12，冻结 **0/12**。本轮覆盖 Artemis XML 的 MQTT 3.1.1 与双向 TLS 1.2。共装 Agent 的快照重加密、其他协议及余下 typed brokers、六组件与 provider/launcher 矩阵保持内部待办。外部延期仅真实 NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。 [正式证据和停机冷读](../../../../.management-close-20261116/core-rtc-hotplug-20261120/api2-artemis-mtls-maintenance-20261125/HANDOFF.md)

## 2026-11-25 API2 Artemis 双向 TLS 跨密钥发布、快照重加密和包维护

隔离 Debian ARM64 Root `1048592:257` 以 Git 构建的 Core `c65c04741e`、Networking `98ce41482f-1` 和 Artemis `1bcdc5d-1` 实装独立 Core 模式。合法 Main 只提交一次 Artemis XML 设置，原加密快照 `1795596925259`。正式密钥发布 `86421b1e-13f4-44e2-8472-511cfb4e4d24` 排空旧 Main，另行批准后 OSGi CryptoService 用新活动钥写读 KURA2，原用户表与双向 TLS 仍可用。

Main 停止后，正式快照重加密 `17a89193-b45a-425b-8972-dec0d9ba6570` 发布原快照的新代 KURA2 密文，原密文归档 `original.save` 为 Root 0400，原摘要保持。独立 Java reader 使用原 ring 读归档，只用新活动钥读重加密件，两份 42,662 字节 XML 逐字节一致。原生 remove、独立 purge 后，合法 Main 仍读到该快照，Artemis XML bundle 和监听缺席。同包解包正常停机，同 Root 新 boot 一次 configure，三个包恢复 installed。后继合法 Main 的实际 TLS 1.2 证书指纹和 MQTT 用户表与原件相符，验证两种 TLS 拒绝及错误 MQTT 口令，恢复主题实际收到新 QoS 1。原四个密钥号和重加密号均只读回原终态，原密文及 19 份 Core 维护签名保持。Main disabled/inactive，VM 正常停放，重复冷读一致。

补父项 #1、#3、#4、#9、#12，冻结 **0/12**。本轮覆盖独立 Core 模式的 Artemis XML 双向 TLS 和加密快照。Agent 共装时的安全重加密、其他协议及余下 typed brokers、六组件与 provider/launcher 矩阵仍为内部待办。外部延期仅真实 NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。 [正式证据和停机冷读](../../../../.management-close-20261116/core-rtc-hotplug-20261120/api2-artemis-mtls-rewrap-20261125/HANDOFF.md)
