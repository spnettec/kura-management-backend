# 中心服务器 CSR 界面与外签回传审查

## 2026-11-14 G026 / V54：中心 CSR 界面与外签上传已实现，正式交付和完整改造继续 ACTIVE

同一 R013 shadow 本批修改13已有源路径、新增10路径，共23；当前 **2191source+4fixture**，相对最后完整 CODE V51 为115输入增量。候选 **121 existing+189 new=310记录，含3fixture-only，所以307 production/test路径整体UNAPPLIED**。这是路径计数，不是剩余任务数量。当前源码检查点 `target/nm-role-authority-20261112/registered-runtime-v52/console-installer-bootstrap-v1/source-checkpoint-server-csr-v54.json` SHA `8c1c1c718990002f89fc4f9b07d29500d3583cf882d58bef6993452f1f4f12fb`；限定 CSR 组件 `final-binding.json` SHA `26730f1433c0d378319a2624091070fe34f67005f0c95b29937662672e8ba754`。V54为源码及独立应用/界面检查点，注册现场仍V52、最后完整CODE seal仍V51。

首次设置配对后及管理员证书设置页，默认点击生成P-256服务器私钥和CSR、下载公开CSR、外部CA签发后上传服务器证书；服务器私钥不进入HTTP响应/DOM/浏览器存储，CA私钥不上传、不放进中心VM挂载。保留已有服务器证书/私钥导入选项。签回校验密钥匹配、DNS/IPv4/IPv6、有效期、serverAuth和非CA约束。明确验证拒绝发生在导入intent之前，可更正同一CSR的证书；缺响应/缺回执则保留原编号UNKNOWN，只查询，404不清本地编号。新编号不能绕过未结CSR。只对已确认且未使用的CSR提供保留历史的停用，已安装/已暂存/UNKNOWN不能停用。初装旧NONE输入的序列化字节与HMAC不变；初装消费仅由原APPLIED回执及精确证书/密钥pin只读推导，缺初装回执不冒称已消费。

签回和暂存不改变当前TLS，仍走既有联合TLS显式确认；缺CSR暂存回执时，旧上传入口及activation也被阻断。服务生成专用OpenSSL配置，明确named-curve/PKCS8，修复本轮实测LibreSSL空配置和EC编码兼容失败；早期失败日志保留。

**103单元通过/39独立集成测试默认ignored，另在最终V54源码上1个新建私有PG+真实HTTPS CSR集成通过（20项未登录/非管理员/CSRF/跨Origin拒绝）；13中英文浏览器契约通过，typecheck/生产前端及Darwin ARM64普通非test debug构建通过。** 浏览器契约使用公开API夹具，不冒称当前Linux包或真实生产CA审批验收。首次HTTP夹具的故意UNKNOWN仍原字节保留；最终源码变更后使用不同schema/应用目录的新夹具验证，没有重试旧UNKNOWN。仅本批私有PG已正常停止，数据库、两个HTTP夹具和失败记录保留。当前普通Darwin可执行SHA `7dee2515932110213d3eb1827809d0a7d16d03dfe2b081a514917cacb7de8dbb`。V54 Linux/容器镜像、正式DEB、完整sandbox/polkit和端到端交付未认证，不以旧V52产物充当前源码通过。

18原仓HEAD/branch/index/staged/refs/remotes及原工作树继续按本批preservation核对；G025的40原仓功能改动、580上游pin/89absent和9个shadow回补均保留。G025/G024/G023/V52Docker/完整V51旧seal共26924个绑定文件只读核验精确。共享API522未改/安装。原767863a9仍唯一startup.stop UNKNOWN/intent a87d94d5，原boot4db3、2799/3635/81713/82137/59158及agentaffa64e2精确，无stop receipt/publication/start/complete/activate/cancel/re-export；owned OrbStack仍stopped。用户已确认清理的旧中心/PG/60容器不重建，旧首次设置/TLS/GWT3保存/V36v1不重放。无提交、推送、reset、clean。

**下一实际实施仍优先installer/bootstrap界面与正式交付**，再按完整台账推进中心地址/DB和证书异常核定、agent独立身份/CSR续期/通道能力、凭据部分恢复、多人外部CA custody quorum/bulk、跨版本DB/非空加密任务/备份恢复、正式sandbox/polkit/协议0升级、完整owner/ordinary/recovery/原forward-confirm-inverse-observation-stopped-expiry、合法boot/netns/handle恢复/全CM-runtime-network-NAT矩阵、初始化先退役再原inverse/子结算退役历史、14组/7API2/R017/307整体guarded apply/正式API与协议校验/joint generations、G021 LDAP、G022 Go31vet/3非JavaPOM/regen/.NET。上述均内部pending；只有原NM-cloud-init-physical、amd64、disk-full-hard-power三组external deferred。未因CSR组件通过缩减总目标。周额度26%已用、ordinary allowed，credits54476.099936及2reset未使用，目标保持ACTIVE。

