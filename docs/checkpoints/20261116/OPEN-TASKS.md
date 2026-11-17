# 未验收与剩余实施台账

## 2026-11-17 继续：兼容变体及普通启动

firewall-only 已补齐4个JAR Root权限、升级阶段/agent ownership保护、受限注册表与失败传播，9项独立源码/Linux原生检查通过。对应新实际systemd profile的Core＋12 sibling全部配置成功，一次普通Main启动/HTTPS通过；59个sibling JAR为Root所有且不可由group/other写入，6份凭据/TLS/HMAC元数据保持，Root业务及日志化启动请求0。Main仍11/13 caps、NNP0；MQTT默认clientId与历史额外JAXB边界分别记录，完整provider/broker没有关闭。Netplan兼容适配只完成源码及5-JAR包核对，未部署/切换NM。

见 [兼容变体检查点](networking-variants-code-ownership-checkpoint.json)及 [审查](../../reviews/networking-variants-code-ownership-review-20261117.md)。原普通networking失败现场仍half-configured，继续原现场处理；新profile不替代该失败。完整六组件/provider/launcher/升级移除矩阵及冻结12父项继续内部pending，仍仅3外部边界。

## 2026-11-17 当前续接：sibling 代码权限与安装链路

用户已明确选择在周额度 100% 时使用现有积分继续，目标 active；不兑换 reset。11 个剩余主 sibling 的 49 处 JAR 映射、提权脚本路径及失败传播已修复，GWT 补齐卸载遗漏。19 项源码/Linux 原生 test methods 通过；实际 12 个新包的 64 个 Root/0644 JAR、56 份代码/模板及 54 条 API 声明已核对。从当前源码构建，不依赖还原旧证据。

新的独立 systemd fixture 已安装 Core 并完成 11 个 sibling 配置；Networking 因真实 udev 控制通道缺失而失败。新候选已在同一失败现场验证提前拒绝，19 份私有/用户文件完整元数据保持，Main 启动/Root 密钥请求均为 0。该现场继续保留，不重复 provision；此项仍内部 pending。历史 Networking 包的旧 prerm 首次升级路径、完整六组件/provider/launcher/变体/安装升级移除矩阵及全部 12 父项仍内部 pending，仅原 3 项 external deferred。原 3 现场的 7 进程/38 私有元数据/6 旧数据及个人当前 25 文件复核一致；配置/密钥/数据及运行中 target/runtime 保留；UNKNOWN 和已闭环密钥验收不重发、不换号、不重放或跨 Root 查询。

见 [sibling 检查点](sibling-code-ownership-checkpoint.json)及 [本轮审查](../../reviews/sibling-package-security-review-20261117.md)。Core 新增仅为通用安装夹具的测试入口；生产包仍与 e016 源码一致。全部更改仅本地 WIP，无推送；冻结 12 父项原文不变。

## 上一阶段：实际 Main 联合检查点（历史）

2026-11-17 当前 Core 源码 `de5f4e6901`（生产包来自 `e016f60c03`，其后只追加测试/说明）、GWT `f135710e00`。实际 Ubuntu24.04 ARM64 / systemd / Main / Root UDS / HTTPS / ADMIN/XSRF 联调已通过一个受限新环境：正常首次登录改密，一次新随机测试钥的加密备份下载，原 LEGACY 和 ADMISSION 各一次元数据归档，独立恢复 HTTPS API 只读解密核验。Root 原密钥操作记录原字节/元数据保留；6份测试凭据/TLS/HMAC文件及现用Main PID不变，没有恢复、发布、换钥或日志化启动。浏览器由Playwright驱动；恢复页健康态按合同禁用材料，没有重放冷恢复验收。

本轮修复发行包旧Crypto API消费者混入（增加打包前源码契约门禁）、GWT隐藏页签初始化过早、插件JAR/提权脚本归属错误及并发历史读取BUSY。181项相关JVM、7项包输入、7项安装器检查通过，双GWT模块12变体构建通过；实际两次页签选择的8个只读调用全成功。新Core包15个Crypto契约和47个包内文件核对，GWT实际包内JAR及Root所有权核对。原3现场/7进程/38私有元数据/6旧数据保持。已删除本轮完成容器、测试备份/口令及重复包等712,958,966字节文件；容器可写层逻辑大小1,294,621,968字节不能等同物理释放量。无备份、无推送。

见[实际Main联合检查点](installed-main-key-archive-checkpoint.json)和[本轮审查](../../reviews/installed-main-key-archive-review-20261117.md)。当前仅闭环上述新测试子场景；完整12父项继续内部pending，G034未封签，仅3外部边界不变。其余11个sibling包发现同类daemon所有代码/安装器映射，优先按功能和数据目录逐项修复、验证。界面按modern-web-guidance随功能渐进处理，仍是次要任务。历史阶段细节留在Git及各原检查点，以下冻结台账原文不能被局部联调结果替代。

## 冻结完整台账

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

1. 先审查其余11个sibling发行包的Root代码所有权、提权安装脚本路径及失败传播。GWT已修；不得把真实运行数据改成代码权限，也不能降低Root配对检查来绕过。网络包有不同的安装布局，需单独分析。
2. 继续当前源码完整六组件、全部provider/launcher/模式及安装升级移除矩阵，完成密钥全生命周期、原文件保留和GUI流程。新环境Core＋GWT通过不代表全部provider重加密或正式发行通过。
3. 按以下完整12父项继续网络/API/中心部署与数据库/CA custody/上游逻辑同步等工作。只读查询短暂排队；写入仍立即拒绝争用，跨进程内核锁非阻塞，不重发Root请求。

本轮关闭的测试备份编号 `cd2ee5cd-1c6c-4e6f-9e82-c545231c50d8`，LEGACY归档 `eb3a5a16-256f-4626-a8b4-8ee7af425b9c`、ADMISSION归档 `47ecb515-0524-4b6f-82b5-1eaed22ac737` 仅保留历史；专用容器和测试秘密已删除。不能重建、重发或在其他Root查询这些编号。原UNKNOWN `767863a9-fe6b-4135-9783-ddcb131030af` 及原startup.stop仍保留在原现场，绝不重发或换ID。

以Git源码和当前环境继续，不依赖旧中间产物或恢复旧证据链。保留原3现场、个人现用配置/密钥/数据及正在运行的target/runtime；确认停用的产物直接删除。JDK最低21，不追加27。周额度耗尽时按用户要求保存必要交接并暂停，只为必要收尾使用少量credits；当前接口周用量显示100%（ordinaryUsageAllowed仍true），按用户额度边界完成必要收尾后暂停；额外credits仅可用于必要交接，不开启新任务。
