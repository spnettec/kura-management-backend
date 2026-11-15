# V50：停机授权与原操作到期调度（代码验证，真实注册验收待完成）

同一 R013 shadow2097源码+4fixture；88existing+108new−3fixture=193生产/测试路径仍未整体应用。相对V49修改13个现有shadow路径、新增6个Rust文件。Root保持V49/G023，未提交推送、未重启正常服务、未清理现场。

已接入独立停机运行证明：原完整committed owner/启动receipt、selected profile文件身份、固定ordinary unit实际停止/空cgroup、stop policy、原physical epoch及前后复核。running路径仍校验当前Core三PID/CM/drain/registration/runtime；不把通信失败转换为停机。运行证明、Verified/Observed均不能从请求反序列化。历史lease/startup/baseline0不改，停机记录保留原Core owner，另外绑定当前confirmed source/revision和停机runtime，未伪造新Core projection。

新事务schema4只允许显式stopped-confirmed audit或original expiry decision；旧1/2/3读写兼容且不能取得新审计/到期权限，Live owner JSON逐字一致。Source observation可用停机证明，但不能带expiry/native/confirmation权限。准备/执行前为两次intent保留较大的停机审计及expiry元数据空间；容量占位符不会持久化或取得权限。

内部调度使用不可反序列化、不可Clone、无公开构造器的Scheduled原操作凭据，绑定原uid/request/id、active inode/hash、parent和已到deadline。未发送的准备只做无副作用取消；完整forward ACK后才允许原inverse一次。inverse意图先持久化，丢回复保留UNKNOWN，不自动观察/重发/换ID。确认决策、未知forward、未决observation不变成inverse。poll只有元数据路径；已存在精确历史可修原marker，缺失记录不重建。取消、inverse与原人工操作共用同一producer/current/history，不恢复明文owner key。

Nm.expire先处理imported原操作，pending时等待；private Firewall.expire跳过imported域，未借用会reapply的旧commit/rollback。现有一秒调度任务独立于remote transport，远程通道关闭不关闭agent调度。功能未配置/写能力关闭的边界和实际服务/网络矩阵仍须真实验收，不能由临时目录夹具概括。

验证：210关联输入/132Rust；宿主681通过、28忽略；新增11项（停机2、审计兼容4、expiry5）均通过，V49全部698项原结果保持。Linux normal/test ELF、严格Clippy、162选定通过/11忽略；没有全量Linux或新真实registered owner验收。Java1802输入/2493测试/193报告/9JAR/privateutil1.3逐项精确复用，无Java重跑。19个变更/新增路径图谱metadata一致、无Rust解析缺口；fixtures/Cargo.lock排除保留说明。

失败证据保留：types-v2 accessor位置与限定路径错误；Clippy-v7私有enum体积/参数数目，改为box和OriginalAction，无lint抑制；checkpoint helper先猜12个变更、实测13（漏算initialization.rs域路由），在任何checkpoint输出前失败，按真实清单修正，源码及job未重启。所有原日志、源码归档和旧seal均保留。

下一交付关口集中接通中心任务、typed protocol、agent原操作查询与界面完整流程。现有30–37是旧firewall/NM协议，新producer仍只接本地已注册Kura UID入口；不能借用旧remote firewall的reapply流程。UNKNOWN只能查询原ID，界面不能提供换ID重试；终态/UNKNOWN/恢复状态停止确认倒计时。随后执行新的真实注册整轮验收及193整体guarded apply/部署收尾。原14组/7API2/R017、合法restore、矩阵、退役、API1.2runtime与1.8source配对、GUI部署/TLS/bulk/protocol crate等全量台账继续保留。方案A外部CA/custody不变，仅原3组external deferred。

读取本组件 current-state.json、final-binding.json、verification-summary-v50.json、remaining-transfer-implementation-v50.md、source/candidate清单和证据；完整V49台账原样保留在 prior-complete-ledger-v49.md。Root519present/89absent、1986=1982历史精确+4已知G023覆盖、management20、credentials141+5、共享API522不变；正常console/PG原StartedAt健康，Orb运行、两Parallels停止、60owned验证容器停止。目标active。
