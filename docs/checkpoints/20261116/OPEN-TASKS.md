# 未验收与剩余实施台账

此台账与 Git WIP 源码一起保留，完整目标未完成。内部 pending 与外部 deferred 严格区分。历史继承范围另保留于 [legacy-scope-v51.md](legacy-scope-v51.md)；其中旧路径、PID、计数、旧“禁止提交清理”等是历史，按 README 的当前状态和用户最新授权执行，不要求恢复旧证据。

## 内部 pending

最新GWT本地WIP `8a2544a42e` 已为Core所有的旧密钥、generation及handoff三个发送日志增加共同准入：持锁覆盖核对其他发送方、私有意图落盘和首次Root调用；跨操作人UNKNOWN也阻止新写，已有ID仍可只读原结果。修复了打开并关闭同一锁inode的第二个FD会释放POSIX进程锁的缺陷，以及普通Root错误＋NOT_FOUND被误结算为终态拒绝的缺陷；旧弱ROOT_PREFLIGHT记录保留原字节并按UNKNOWN查询，缓存成功不能替代新准入时核对Root原件。旧密钥RPC补HTTPS门禁。90项相关单元检查、7组真实GWT组件/HTTPS/RPC浏览器检查和其中6项Linux/JDK21独立进程内核锁检查通过；Root回复与人类会话仍为替身。双模块12变体打包通过，33,991,757字节，29个服务端类匹配且为Java11，login/denali各6份主脚本。原准备/加密备份下载上传/核验/签发流程保留，Core仍a277；3原现场、7进程、38私有文件和6旧数据均未改，无远程推送。跨Unix UID独立恢复、全部日志升级门禁、实际data目录/Root命名空间绑定、准入拒绝后的持久未发送回执与界面恢复仍pending；尚不能作为完整共同协调或实际Main联合验收。详见 [GWT交接检查点](gwt-key-handoff-checkpoint.json)。

最新Core本地源码 `a2770c21e9` 已实现独立HTTPS固定操作页、浏览器与接收端各自的持久发送意图，已有意图只读原结果，不因NOT_RECORDED重新发送。发布与启动分开确认，原发布续接须匹配认证回执；不确定续接不能再次转发。原钥冷恢复与交接写操作互斥；包生命周期准入除本地完成回执外，还必须核对Root原发布/启动均已成功。27项新检查在两种权限模型通过（同一27例，不计为54种），22项原恢复回归、121项Linux/JDK21模型和9项真实Chromium/HTTPS测试通过；Root回复仍为替身，新包未部署。旧8b原生身份、7e实际发布续接和4c6实际Main证明不套给此次已改的恢复/lifecycle代码。授权材料下载上传、实际安装Main/GWT/HTTPS联合验收、原授权续期/撤销/归档、过期首次启动及不确定续接后的合法恢复仍pending。详见 [HTTPS交接检查点](https-key-handoff-checkpoint.json)。以下12个父项和3个外部边界不变。

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

先继续补跨Unix UID独立恢复与三类GWT发送方的共同准入、全部发送日志升级门禁，以及实际配置data目录/Root命名空间在重配置、JVM/bundle重启和升级中的绑定。Core所有的三GWT发送方已具备共同门禁，不等于独立恢复身份已接入；Java FileLock与Python flock也不能视为同一种锁。跨发送方准入在本地意图创建前拒绝时，仍须补持久未发送原回执和明确界面恢复，不能用Root原件缺失推断终态拒绝或发新ID。

随后继续加密授权材料下载/上传和实际安装Main人类会话/ADMIN/XSRF/GWT/HTTPS与独立页联合核验；现有7组浏览器Root/人类会话替身检查不替代它。原凭据续期/撤销/归档、过期后首次启动和不确定续接后的合法界面恢复仍须补齐，不以脚本替代。UNKNOWN只读原结果，NOT_RECORDED不构成重发许可；不复用旧单钥DTO或扩大备份口令授权。runtime/binary/cohort、完整privileged broker、缺原停止凭据/跨boot/旧锁重建、维护启动、全部provider原子重加密/退役及12父项继续推进；安装agent仍阻止代发布。界面随功能逐步应用modern-web-guidance，JDK最低21，暂不追加27；不重放README闭环ID，不依赖旧大型证据/缓存继续。
