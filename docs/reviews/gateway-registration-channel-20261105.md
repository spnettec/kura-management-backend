# 中心登记与本机开关 mTLS 闭环（2026-11-05）

用户明确恢复继续，整体 active。此增量完成并收尾，不是整体完成或再次暂停。18仓 HEAD/branch 未变，保留全部工作树，无提交推送。当前完整源状态见 target/resume-center-auth-20261105/worktree-after.json。

## 已交付增量

中心新增管理员 HTTP/React 双语公开叶证书登记，精确会话租户/操作者、Origin/CSRF和首改密码后台闸门，严格有界材料；纯INSERT拒绝替换/重复，全局ID和指纹唯一，登记/审计/事件同一事务。CLI复用相同规则。页面核对指纹后确认；未知结果冻结，显式读取ID/名称/指纹/代次一致才能核定，无自动重发。详见 gateway-registration-v1.md。

实际系统联调发现：Java/SunEC 合法的 P256 PKCS8 可省略冗余公钥点，OpenSSL接受而ring拒绝；先前离线校验和opaque制品不证明真实TLS。修复仅在内存对原私钥做确定性格式适配，ring及独立公钥等价核对，不能生成新密钥或重写磁盘材料；有界OpenSSL执行在blocking worker，原身份指针/回执/私钥/证书均保留。

## 验证范围

- 中心宿主/Orb各17、协议各8；9条环境依赖Rust测试默认ignored，实际新PG HTTP合同1另显式通过。ADMIN/tenant/Origin/CSRF/首改闸门、额外字段/损坏PEM、并发仅一条记录/审计/事件、重复证书跨租户拒绝、原停用/高代次不覆盖、审计失败回滚通过。
- React构建/类型及28项适用浏览器通过；宿主及Orb各1真实HTTPS中英登记，在服务器201持久提交后丢弃浏览器响应，显式读取找回精确记录，全程各只发1次POST，真实会话registration-admin审计入库。
- agent宿主串行/Orb各114，nft各1 ignored；真实Tonic/ring构建器的SunEC原编码拒绝/适配接受/公钥不变/原件不变回归通过；严格Clippy/fmt通过。宿主Python packaging7通过/2skip，未构建新deb或Docker。
- IDEA原生启动现有完整282项隔离Kura，原admin/admin中文、无agent隐藏菜单和快照页通过并停服；agent修复后再次完整启动原登录通过并停服。第二次后续快照RPC遇到同localhost并行Kura页签cookie范围下会话过期，不计为通过，Java/GWT本轮未改。未重放prepare/密钥生成/首次身份交付。
- Orb用此前真实Kura/JCA生成并已接收的同一身份，原GWT明确启用→真实mTLS连接新中心→agent.status/配置列表/快照列表3项实际中心HTTP/gRPC/UDS/OSGi只读任务成功；原GWT明确关闭后agent仍运行，实际重启保持OFF/revision，中心变离线。36项身份/源材料摘要不变；重复登记409，原名称/指纹/代次不改。临时明确Kura开关授权仅限新测试unit，原agent-env中的grant0不改。
- Parallels最终两个ELF复验：现有VM、新/opt夹具、本机新PG18（未安装软件），精确SunEC同型测试编码由夹具在agent外生成；中心HTTP登记→native首次交付默认OFF→明确开启真实mTLS及agent.status→关闭/重启保持OFF，源和managed身份/系统配置不变。agent有效ProtectSystem=strict/ProtectHome/PrivateTmp/NoNewPrivileges/零Cap及实际mount namespace源只读/state可写通过。仍无Java，actualKura=false，不能算Kura业务恢复。

## 精确制品及运行状态

中心Linux arm64 SHA256 `00245f703d0d11c0e643980e8436221bb84c9c659196a33d8a2171150e353f8e`；agent `c5f2329bf0973e051263b8719f8c7eb229b7bfa340848f5782c2e234d8fb5761`。生产输入核对一致；编译后仅前端测试适配及OpenAPI生成类型补422变化，最终独立source stage另保留，未冒充正式发行。

宿主新console/PG/Vite已停，IDEA两次进程68726/71419均停；Orb新Kura/agent/center/PG均停，既有身份保持OFF，所有材料及失败现场保留。原Orb console/center2/agent及phase2-agent active，原包console26/agent20。Parallels新agent/center/PG已停，Ubuntu正常stopped，Windows stopped。新夹具：Orb中心/var/tmp/yofc-center-registration-system-20261105-5460741a；原身份夹具/var/tmp/yofc-gwt-enrollment-system-20261105-a75c8253继续保留；Parallels/opt/yofc-registration-native-be4b4f9e。

## 失败记录及剩余边界

所有初始日志保留。首轮错误root Rust环境自动安装了官方1.95重复工具链后缺离线依赖，切到已有开发用户缓存完成；未移除该目录。Parallels父目录权限/未复制web/failed瞬态unit重建属于夹具问题，在首次身份交付之前续接同一请求，无删除/重放。宿主并行agent旧恢复时序断言受并发构建调度影响，串行全量通过，未改恢复断言。误选旧live-backend浏览器用例的失败不计产品通过；其常规target/phase1-login.png可能被再生成，未触及旧state/key/备份/quarantine/卷。

pending：托管CA签发产品（本轮为运维签发兼容）、身份故障运维核定/归档/轮换/转移、完整组件适用面、Java XML/加密/factory/核心离线恢复及真实业务核验、防火墙适配/NAT/远程/早启、restart/reboot任务、一致备份恢复及正式发行。实际GWT身份交付的提交后响应丢失仍未验，本轮类似证明是中心登记，二者分开。

deferred：Parallels完整Java/Kura、真实NM/cloud-init/物理有线/硬件、amd64、磁盘满/硬断电。NM和Netplan保持用户已核定现状，不移除NM，不以本轮mTLS/身份验证代替业务恢复。不连接192.168.2.125，不清理个人或运行中runtime。

机器证据 docs/evidence/gateway-registration-channel-20261105.json；详细原始日志/截图/源stage/18仓状态均在target/resume-center-auth-20261105。下一轮按完整台账推进未交付业务；不要重做本次已闭环的一次性登记/交付。

知识库最新全文：`Experience/2026-11-05-Kura中心证书登记与本机开关mTLS闭环续接-40feee51.md`；Tasks/Session Summaries及前两份交接已用官方CLI追加覆盖核定。
