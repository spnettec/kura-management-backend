# V51：中心任务与界面原操作闭环（代码验证，真实注册整轮仍待完成）

同一R013 shadow2112源码+4fixture；98existing+123new−3fixture=218生产/测试路径未整体应用。18仓工作树、index、refs、未跟踪源码、原1986/20/519present+89absent/credentials141+5/API522已完整复核。未提交推送或局部apply。V50及G023封存和所有历史失败/验收证据保留。

新增38–48及独立typed payload，公共请求不接收ownerKey/token/UID/path/unit/native命令。中心生成能力密钥并用原TaskCipher加密，幂等复用原密文计划。agent加密索引绑定原command、中心/gateway/代次及唯一operationId；原元数据查询与新写授权分开，可在写授权关闭/JVM不可用时查询已归属原操作。新调用直接进入SAME imported producer，不借用旧private firewall的reapply路径。

打包服务实际User=root，已补record5 REMOTE_ORIGINAL审计及不可反序列化的RemotePrincipal；保留真实euid，包括0，不伪造正UID或代用Kura UID。旧record1–4保留字节兼容，local入口仍要求正UID并拒绝remote保留域。origin本身不能成为新的Core/runtime/owner授权。应用、确认、回退和取消在意图前保存各自实际admitted command审计；自动expiry保持独立范围，不取得remote inverse命令。

中心原结果追认同时校验同actor/gateway/代次、目标和parent两级原command hash、提案UUID及精确阶段命令marker/ACK/decision/history。命令A不能继承命令B的成功；不可逆的竞争阶段证明A未进入时核定FAILED/CONFLICT，并只写一次审计。证据不足保持UNKNOWN，不能排队、改ID、重发或重导出。schema5缺失或改变审计不能降级为local旧版本。

应用后的确认要求新的真实mTLS连接：首个原forward ACK后保存加密且不可覆盖的barrier，绑定原prepare及forward命令。连接witness由agent在welcome后本地产生，并记录当前共享witness，已在效果前建立的替代连接不能冒充新连接。同连接、过早/未来时间、重用nonce/epoch、过期worker和时钟回退均拒绝。agent自动重连，用户从界面查询原操作；只有新连接和有效窗口满足才显示可确认倒计时。确认命令自己的加密witness先持久化，再进入SAME fresh Core/NM授权；查询不能从当前策略重建缺失证明。source observation没有native或确认权限。

React双语表单接入source/version/revision/cohort/显式destination prefix审阅和prepare/apply/confirm/rollback/cancel/query/source observation。浏览器只存无值原关联，不存能力密钥/规则草稿。UNKNOWN不重发，终态/恢复/不确定/过期停止倒计时；应用后自动重连→查询→确认的点击流程已验证。GUI部署、证书下载上传、TLS reload和真实注册整轮仍是独立内部待办。

本轮当前源码验证：宿主693通过/29忽略，Linux aarch64 normal/test ELF、严格Clippy、174选定通过/11忽略（没有全量Linux）；Linux按生产服务UID0验证远程origin。中心50+协议24通过/38忽略，新增私有PG3通过，独立连接存储2通过，真实mTLS4条通道worker验证通过，浏览器mock14通过，前端类型/生产构建通过。mTLS用缓存原结果且0系统任务，只证明真实worker重连；存储/数据库/mock不替代真实Core/NM/native验收。Java1802输入、2493历史测试、193报告、9JAR及privateutil1.3逐项精确复用，没有重跑。

phase1旧源码/二进制及685/28、Linux166/11证据单独保留；它们没有证明root账户与新连接门槛。本次新增失败归档：root-v10断言Debug编译问题已修；归档工具先猜217，实测增加mtls现有文件后为218，未通过更改源码迎合计数。其余此前所有失败快照和原日志保留。

用户明确确认Docker曾清理/重建，按当前现场继续。旧normal center/PG、60owned容器、Orb gw-001健康/运行只作历史，不重建旧fixture或重放旧验收。新V51私有PG、两Linux验证容器停止并保留，真实mTLS证书/数据库/结果保留。没有正常服务重启或新nft/NM写场景。

下一关口为新的实际注册source→单次加密export→paired fences→Java drain→native/realStore初始化→startup/Core owner→ordinary boot/recovery→原forward/confirm/inverse/observation/stopped/expiry整轮，再进行218整体guarded apply。原14组、7API2、R017、合法restore、完整CM/runtime/网络矩阵、初始化与子记录退役/历史重接、API1.2 runtime/1.8 build配对、GUI部署/TLS/bulk/正式协议crate/joint generations、G021/G022及G023新增SSL/Crypto兼容cohort等全量内部台账保留。G023游标不提前推进，不bulk merge非fork。只有原三组external deferred：NM/cloud-init/物理硬件、amd64、disk-full/hard-power。方案A外部CA及custody决策不变。目标ACTIVE，代码验证不等于整体完成。

续接先读本组件current-state.json、final-binding.json、verification-summary-v51.json、remaining-transfer-implementation-v51.md、phase2-source-manifest.json、working-candidate-phase2-v51.json及证据。完整V50台账保留在prior-complete-ledger-v50.md，V49/V48继续原样留存。UNKNOWN不重发，V36v1不做live query/inverse/reissue，所有CLOSED场景不重放。
