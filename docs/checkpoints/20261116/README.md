## 2026-11-17 原生 peer、状态连续性及首包原失败续接

networking d226b61 的精确 native peer 校验已通过160项JVM及8项Linux/JDK21检查，Root进程即使伪装socket属主仍拒绝；Core/GWT Root transport未放宽。agent 3f7c5b6 增加显式非Root包、固定账户、代码/unit/source配对、原UUID初始化及DB/计划密钥结构连续性；145项Python通过、2历史导出夹具未执行，后续32项为重叠子集；Rust身份3/连续性1/注册续期30通过。

当前Git自包含230文件源码构建的ARM64 native包已在新E a60111a6…44414安装，18运行载荷精确；5份文档由基础镜像既有dpkg策略排除。原初始化24560864-f974-46aa-940a-0a97ad926118唯一完成，APPLIED/宿主READY、远程关闭、7业务表0。首次Main启动在Root预检失败，Main从未运行；24次自动预检后仅停止该新unit。只读诊断证实systemd255.4的“+”未消除TemporaryFileSystem遮蔽，Root env/admin不可见；Root helper为完整容器caps/NNP0，不能称Main cap0/NNP1已验。保留同一E、原DB/计划钥/初始化回执与失败，不重新初始化、换号、换环境或改写历史回执；继续Root检查上下文修复和有原意图的精确source/pin维护。空Root0755挂载点与真实旧数据须分别识别。

见 [首包检查点](native-agent-first-package-checkpoint.json)及 [审查](../../reviews/native-agent-peer-and-first-package-review-20261117.md)。原3现场7进程/38私有/6旧数据、个人25文件、新C9私有与进程/零业务计数、中心29私有及原进程再次精确；18仓检查时干净，冻结台账SHA不变。Native实际Main/隔离、授权/提权broker、迁移/升级移除/恢复、R017及全部六组件矩阵和冻结12父项继续内部pending；仅原3 external deferred。原CSR及闭环密钥验收不重发/重放，目标active、现有积分继续、无推送。

## 2026-11-17 专用非 Root agent 与窄委托源码续接

agent b893964 新增 native-nonroot-v1 的固定身份/路径/Root程序与unit及源码指纹门禁，daemon 必须 cap0/NNP1；Root 提权后台未接入时明确拒绝。新 runtime、控制 socket、Root管理/配置与公开身份分开；不复用旧目录，不让 StateDirectory 递归更改持久数据属主。只读材料经不同端点的 Kura UID/cap0 → Root/cap0窄委托 → 专用 agent UID/cap0 传递，Core Root key transport 不放宽。

3项Rust身份、8项Linux窄委托、9项原生预检夹具及30项受影响注册/续期检查通过；另一次显式三进程 parent/child 以真实 UID995/cap0/NNP1完成独立证书校验、创建自有0600候选，源字节/完整元数据含atime精确，没有发布身份/未决指针。其程序/源码元数据明确为文件系统夹具，不能称原生发行配对验收。既有新现场的UID0反例仍适用，任何运行服务未替换。

见 [源码检查点](native-agent-identity-source-checkpoint.json) 和 agent 的 docs/native-agent-identity-v2-candidate.md。新文件尚未纳入deb映射，未构建/部署；账户创建、Root授权/恢复、Core peer映射、旧数据/凭据/UNKNOWN迁移及状态丢失保护、真实systemd与GUI仍内部pending。原CSR意图及闭环密钥验收不重发/重放；完整六组件/全部冻结12父项继续内部pending，仅原3 external deferred；目标active、现有积分继续、无推送。

## 2026-11-17 注册材料 FD 与 Root 同 UID 反例续接

agent 75ebf3c/9a25c91/36fff96/8a7c03e 保存只读 FD 发布/消费、精确包/源码/unit 配对、原 UUID 授权与 purge 保留候选。30 项 Rust 常规检查及一次显式 Root 跨 UID parent/child 通过；扩展 Python 116 通过/2 历史导出夹具未执行，后续 purge 相关 14 通过（重叠不累加）。未构建、安装或启用新包，未结算任何业务 CSR。

公开临时探针确认：既有新组合现场中，UID0/NET_ADMIN/NNP1 可通过 Root key-service 的 /proc/PID/root 绕过直接路径遮蔽。未读任何钥/HMAC/凭据/日志，探针已移除，/var/lib 目录时间戳变化保留。此前“agent 看不到原钥/HMAC”只代表直接路径观察，不能外推完整隔离；Root-actor 候选暂停部署，继续专用非 Root 身份、窄委托及准确 peer/发行包配对。此项仍是内部工作，不新增 external deferred。

见 [检查点](enrollment-custody-runtime-identity-checkpoint.json)与 [审查](../../reviews/enrollment-custody-runtime-identity-review-20261117.md)。原3现场7进程/38私有/6旧数据及个人25文件精确，新C9私有/进程/零Root业务计数、新中心29私有/原进程保持。combination-c-ebdbf564 不重生成/换号/换Root；R017 GUI、完整六组件/provider/launcher/安装升级移除与全部冻结12父项继续内部pending。仅原3 external deferred；现有积分继续、目标active、无推送。

## 2026-11-17 组合网关、首改密码及正式数据目录续接

同一新 agent-first 现场新增 networking5d27fea/GWT 并实际普通启动，Core/agent/networking/GWT configured，NM/GWT/bridge ACTIVE；health 为惰性 SATISFIED，不宣称业务健康已验。GWT0d6dba8 修复首改只填新密码/确认及事件会话锁，46a2d92 改用 Core SystemService 数据目录并拒绝旧 override 冲突。最终 20 项选择检查、两模块12 GWT变体通过；当前 GWT 原生包已在同一新现场升级，9私有完整元数据/agent进程保留，Root operations/startups与agent执行表0。唯一密码更新200/服务端成功审计，现有新密码实际登录通过；一次门禁探测先使测试登录失效，XSRF403时密码写0，记录保留。

CSR 自动化意图 `combination-c-ebdbf564` 原响应/客户端UUID未恢复，无申请目录/受理审计；新只读列表为空。异步错误弹窗可解释点击未触发，但不据缺失或推断结算；不再生成、不换ID/标识、不中转到其他Root重做。身份目录错误已实际修复，原意图和 GWT 发出前持久保存请求编号的缺项继续内部 pending。本机未授权的首次接收/通道按钮保持禁用；R017 本机授权/维护/独立agent GUI 未关闭。

见 [检查点](combined-gateway-runtime-checkpoint.json)及 [审查](../../reviews/combined-gateway-runtime-review-20261117.md)。原3现场7进程/38私有/6旧数据、个人25文件精确；新中心29私有及原进程身份精确。方案A与后备CLI停机顺序文档已校正。完整六组件/provider/launcher/升级移除和全部冻结12父项继续内部pending，仅原3 external deferred，目标 active、现有积分继续；旧UNKNOWN及已闭环密钥验收不重发/重放，无推送。

# 管理体系阶段检查点

## 2026-11-17 继续：中心原生 GUI 引导与独立工具更新

新空主机实际 Cockpit/PAM 发现并修复 sudo 必需依赖；9 项 Root 打包/源码检查通过。中心包由 GUI 唯一上传/安装，原实际 dpkg＋postinst 回执与 9 个 Root 载荷核对，安装保持停止；之后 GUI 显式启动、可信凭据/实际 TLS 核对、中心生成 CSR、外部测试 CA 签发、GUI 专用空库初始化及管理员登录通过。TLS 切换后的原初始化 8d038ae0…仅在同一中心登录后只读取得 APPLIED/observedConfiguration=true，没有重发换号。独立工具在运行中更新，7 私有元数据和 PID/ticks 保持；中英文不同主机名帮助及无障碍关联实际验证。

见 [检查点](native-console-empty-host-checkpoint.json)及 [审查](../../reviews/native-console-empty-host-review-20261117.md)。最终中心 PID3135、cap0/NNP1、29 私有文件，原 7 bootstrap 元数据精确；外部 PG16.15 应用角色无 superuser/createdb/createrole。OS 图形安装/GTK/人工证书信任、gateway mTLS 与完整六组件/provider/launcher/安装升级移除及全部冻结 12 父项仍内部 pending，原 networking inode/ctime 偏差继续保留；仅原 3 外部边界。原 3 现场/7 进程/38 私有/6 旧数据及个人 25 文件再次精确，目标 active，无推送。

## 2026-11-17 继续：旧 NM 配置保留与原失败续接

原 networking 失败现场经显式 conffile 过渡及修复后正常包续接，Core＋12 sibling 全部 configured，64 个 Root/不可由 group/other 写入的 JAR，12 注册项无重复；Main 启动/Root operations/startups 0。活动 NM 文件原完整元数据、网络服务 PID/ticks 保持。首次过渡暴露空 configured-version 误判首装，给该新测试现场属性追加两标志；已严格按原前缀/已知后缀指纹撤销，字节/属主/模式/mtime 恢复，但 inode/ctime 不可恢复，偏差保留。preinst 持久安装模式及损坏准入拒绝已修复；24 项 sibling、9 项 dpkg 检查通过。最新修复版过渡归档已组合，尚未另作完整实际 Kura 路径验收。

见 [检查点](legacy-nm-policy-continuation-checkpoint.json)及 [审查](../../reviews/legacy-nm-policy-continuation-review-20261117.md)。resolver 仅 ctime 变化已记录，不宣称全部系统元数据精确。原 3 现场/7 进程/38 私有元数据/6 旧数据及个人 25 文件再次精确。完整六组件/provider/launcher/升级移除、obsolete conffile purge、R017 GUI 与全部冻结 12 父项继续内部 pending，仍仅原 3 外部边界，目标 active。以下 half-configured 描述是各历史检查点当时状态。

## 2026-11-17 继续：agent 先装与精确 loader 配对

Java adapter 已移到 agent 包自有路径，Root 保护的元数据/loader/JAR 精确绑定；agent-first 不再先占用 Core 未初始化目录。动态父目录隔离修复了启动时不存在密钥路径的漏洞，公开 CLI 支持 kurad。新原生 ARM64 agent/console/installer 已组合；同一 agent-first 失败现场原维护记录续接成功，Core 配对升级及一次普通 Main 首启通过。OSGi bridge ACTIVE、三项 Core 服务绑定，实际 kurad 只读 UDS 得到 kuraAvailable=true；6 份私有文件元数据保持，agent 看不到新原钥/HMAC，Root operations/startups 仍为 0。Main 仍 11/13 caps、NNP0；无 networking 的 health DS 和中心/安装器未实际验收。

见 [检查点](agent-first-paired-loader-checkpoint.json)及 [审查](../../reviews/agent-first-paired-loader-review-20261117.md)。原失败/命令前端/parser 记录全部保留，不重新 provision、不绕过 inactive guard 或 Root 初始化检查。原 3 现场/7 进程/38 私有元数据/6 旧数据及个人 25 文件再次精确。完整六组件/provider/launcher/安装升级移除、旧活动 NM 配置迁移及冻结 12 父项继续内部 pending，仍仅原 3 项 external deferred，目标 active。

## 2026-11-17 继续：默认 NM、Core 保留与源码 cohort

默认 networking 已取消自动系统接管/快照改写，旧活动包配置的 obsolete-file 转换会提前拒绝，安全保留迁移仍 pending。22 项 sibling、9 项变体、8 项 Core 文件系统检查通过；同一原失败夹具的一次实际 Core 升级保留 56 份 Root addon 代码、12 份外部私有完整元数据与 7 份用户文件内容/属主/权限，Main/Root 业务请求 0；networking 仍 half-configured。agent/中心源码绑定和默认 Core 显式授权已修复（12/10/5 项各有限检查），独立 ARM64 Rust/前端构建完成，实际原生组合尚未完成。

见 [检查点](default-nm-core-code-cohort-checkpoint.json)与 [审查](../../reviews/default-nm-core-code-cohort-review-20261117.md)。原 3 现场及个人数据再次精确；不重放旧密钥验收或重发 UNKNOWN。用户已授权现有积分继续，目标 active。完整六组件/provider/launcher/安装升级移除及冻结 12 父项继续内部 pending，仅原 3 项 external deferred；以下 e016 及早期场景均为历史有限范围。

## 2026-11-17 继续：兼容变体及普通启动

firewall-only 已补齐4个JAR Root权限、升级阶段/agent ownership保护、受限注册表与失败传播，9项独立源码/Linux原生检查通过。对应新实际systemd profile的Core＋12 sibling全部配置成功，一次普通Main启动/HTTPS通过；59个sibling JAR为Root所有且不可由group/other写入，6份凭据/TLS/HMAC元数据保持，Root业务及日志化启动请求0。Main仍11/13 caps、NNP0；MQTT默认clientId与历史额外JAXB边界分别记录，完整provider/broker没有关闭。Netplan兼容适配只完成源码及5-JAR包核对，未部署/切换NM。

见 [兼容变体检查点](networking-variants-code-ownership-checkpoint.json)及 [审查](../../reviews/networking-variants-code-ownership-review-20261117.md)。原普通networking失败现场仍half-configured，继续原现场处理；新profile不替代该失败。完整六组件/provider/launcher/升级移除矩阵及冻结12父项继续内部pending，仍仅3外部边界。

## 2026-11-17 当前续接：sibling 代码权限与安装链路

用户已明确选择在周额度 100% 时使用现有积分继续，目标 active；不兑换 reset。11 个剩余主 sibling 的 49 处 JAR 映射、提权脚本路径及失败传播已修复，GWT 补齐卸载遗漏。19 项源码/Linux 原生 test methods 通过；实际 12 个新包的 64 个 Root/0644 JAR、56 份代码/模板及 54 条 API 声明已核对。从当前源码构建，不依赖还原旧证据。

新的独立 systemd fixture 已安装 Core 并完成 11 个 sibling 配置；Networking 因真实 udev 控制通道缺失而失败。新候选已在同一失败现场验证提前拒绝，19 份私有/用户文件完整元数据保持，Main 启动/Root 密钥请求均为 0。该现场继续保留，不重复 provision；此项仍内部 pending。历史 Networking 包的旧 prerm 首次升级路径、完整六组件/provider/launcher/变体/安装升级移除矩阵及全部 12 父项仍内部 pending，仅原 3 项 external deferred。原 3 现场的 7 进程/38 私有元数据/6 旧数据及个人当前 25 文件复核一致；配置/密钥/数据及运行中 target/runtime 保留；UNKNOWN 和已闭环密钥验收不重发、不换号、不重放或跨 Root 查询。

见 [sibling 检查点](sibling-code-ownership-checkpoint.json)及 [本轮审查](../../reviews/sibling-package-security-review-20261117.md)。Core 新增仅为通用安装夹具的测试入口；生产包仍与 e016 源码一致。全部更改仅本地 WIP，无推送；冻结 12 父项原文不变。

## 上一阶段：实际 Main 联合检查点（历史）

2026-11-17 周用量已显示100%，本阶段必要交接完成后暂停目标；以下记录保留已接受的受限范围。

2026-11-17 当前 Core 源码 `de5f4e6901`（生产包来自 `e016f60c03`，其后只追加测试/说明）、GWT `f135710e00`。实际 Ubuntu24.04 ARM64 / systemd / Main / Root UDS / HTTPS / ADMIN/XSRF 联调已通过一个受限新环境：正常首次登录改密，一次新随机测试钥的加密备份下载，原 LEGACY 和 ADMISSION 各一次元数据归档，独立恢复 HTTPS API 只读解密核验。Root 原密钥操作记录原字节/元数据保留；6份测试凭据/TLS/HMAC文件及现用Main PID不变，没有恢复、发布、换钥或日志化启动。浏览器由Playwright驱动；恢复页健康态按合同禁用材料，没有重放冷恢复验收。

本轮修复发行包旧Crypto API消费者混入（增加打包前源码契约门禁）、GWT隐藏页签初始化过早、插件JAR/提权脚本归属错误及并发历史读取BUSY。181项相关JVM、7项包输入、7项安装器检查通过，双GWT模块12变体构建通过；实际两次页签选择的8个只读调用全成功。新Core包15个Crypto契约和47个包内文件核对，GWT实际包内JAR及Root所有权核对。原3现场/7进程/38私有元数据/6旧数据保持。已删除本轮完成容器、测试备份/口令及重复包等712,958,966字节文件；容器可写层逻辑大小1,294,621,968字节不能等同物理释放量。无备份、无推送。

见[实际Main联合检查点](installed-main-key-archive-checkpoint.json)和[本轮审查](../../reviews/installed-main-key-archive-review-20261117.md)。当前仅闭环上述新测试子场景；完整12父项继续内部pending，G034未封签，仅3外部边界不变。其余11个sibling包发现同类daemon所有代码/安装器映射，优先按功能和数据目录逐项修复、验证。界面按modern-web-guidance随功能渐进处理，仍是次要任务。历史阶段细节留在Git及各原检查点，以下冻结台账原文不能被局部联调结果替代。

本检查点按用户最新要求保存本地Git提交并清理可重建中间产物，取代此前“不提交推送”的阶段限制。它保存未完成的工作，不是正式发行或完整验收。后续从这批 Git 源码和 [未验收台账](OPEN-TASKS.md) 开始，不需要恢复旧 `target/`、旧 Maven/Cargo 缓存、证据清单或源副本。

18 仓共同使用**本地**分支 `wip/management-close-20261116`；按用户最后确认，未达到主线合并条件的代码只留本地，**没有推送远程 WIP 分支**。每个有修改的仓先保存原工作树，再保存完整管理候选；可通过提交历史回退重构。干净仓也保留同名本地分支。447路径中仅2个独立CI文件已合入自己的远程 `spnettec/kura` 的 `heyoulin-kura` 主线，提交 `dc788aa2361d2c14971c0b45533848e83f17aadd`；其余445路径保留本地，见 [候选归并审查](CANDIDATE-REVIEW.md)。远程提交从原远程主线单独建立，没有顺带推送本地主线中另3个未推送的检查点提交。最终本地提交与远程读回见 `.management-close-20261116/`。

管理候选共 447 个路径，涉及 core、networking、GWT、agent、console、Docker。用于历史隔离构建的 3 个生产树替代文件与 4 个小 reactor 单独保存在 [build-fixtures](build-fixtures/)，没有覆盖正式 reactor/生产清单。新增密钥代号代码、配置边界测试和 JDK 最低版本修复一并保存。此处的 WIP 源码归档不替代原“完整 guarded apply / 正式发行”验收门禁；G034 未封签，最后接受 G033/V61、最后完整代码封签 V51。


## 本轮已闭环场景（禁止重放）

本轮新容器已删除。备份 `cd2ee5cd-1c6c-4e6f-9e82-c545231c50d8`，LEGACY归档 `eb3a5a16-256f-4626-a8b4-8ee7af425b9c`、ADMISSION归档 `47ecb515-0524-4b6f-82b5-1eaed22ac737` 均已正面完成。只保留历史，不重建或在其他Root查询这些编号；未来必要验证从当前源码和另外有理由的新环境开始。

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

界面随相关功能逐步应用 `modern-web-guidance`；按用户最新要求，不把全站样式/组件改造作为主任务。GWT已有原预留核对/关闭，独立HTTPS页也已接原预留的严格原授权关闭和浏览器显式确认；使用forms和dialog指南，已补严格匹配私有启动与冷恢复意图/提前pending的原未准入结算；Root INTAKE、破损/缺命名空间和旧目录的合法恢复、其他GUI模式/人工体验、完整六组件/provider联合验收及完整授权材料流程仍pending；本次实际Main/GWT子场景见页首。它属于密钥主线，备份＋口令的原钥恢复权限未扩大。

## 2026-11-16 空间清理结果

本轮实测工作空间约483.49 GB，清理后父目录约96.39 GB；按磁盘空闲读数，本轮净增加约306.02 GB，可用约424.96 GB。目录统计会重复计算 APFS 共享区段，不能把387.10 GB的目录下降量等同于实际物理释放量。详见 [cleanup-status.json](cleanup-status.json)。已删除确认停用的重复运行库、Cargo/Maven/Equinox缓存、旧构建输出及确认可丢弃的备份/证据副本，没有为本轮废弃物重新备份。

Docker从170个容器减为6个，删除132个无用卷、保留17个数据卷和必要场景；个人 `.kura-dev` 删除3个停用迁移调试目录及日志，保留25个现用/迁移前真实文件。3个当前网关的7个进程、38个私有文件元数据和6份旧数据内容保持。另有60份来源尚未确认的旧卷档案约1.48 GB暂留，不能误称为已确认废弃物。

剩余96.39 GB仍包含未完成审查的候选副本、原UNKNOWN上下文、现用安装输入和旧数据，并非全是纯源码，也不意味着必须长期保留。后续逐项确认与Git/现场的关系后直接删除无用副本，不依赖旧target证据链恢复任务。2026-11-16当时要求清理完成后继续；后续周额度耗尽暂停指令仍适用，当前100%暂停交接见页首。
