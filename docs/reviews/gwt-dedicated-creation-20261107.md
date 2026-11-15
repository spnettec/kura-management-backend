# GWT 通用工厂创建与专用路径审查

2026-11-07，c3819568，续接6ce49a2e。整体 active，完整目标未完成。合同 `../configuration-plan-local-v1.md`，证据 `../evidence/gwt-dedicated-creation-20261107.json`，日志 `target/gwt-dedicated-audit-20261107/`。

## 改动

通用 GWT factory helper 改用一次 Core typed creation plan，检查包括 dormant CM 在内的 PID 缺席，原 Core 准入、原生参数验证和最终 snapshot 保持同一计划。未走 legacy create/delete 回退。等待激活超时或派发后异常一律 UNKNOWN，不删除配置、不重放、不补偿 snapshot；准入前 typed conflict/unsupported/invalid 分流。

共享新实例参数转换仅包含类型化参数、明确 name/description 和 factory 属性；不复制 UI datamap。`providesService` 使用转义的字面 PID filter，另逐项精确匹配实际 kura.service.pid；`*` 不再匹配并授权其他服务。该共享方法的 restricted/driver/asset/pub-sub 调用受此修复影响，但不表示这些专用页面已全量验收。

## 实际发现后修复的两处漏洞

首次隔离 IDEA 实际延迟工厂发现：ServiceUtil.waitForService 的默认 ServiceTracker.open 会同步 getService 并激活目标，60秒计时尚未开始便可阻塞80秒。线程栈定位到 RPC线程→ServiceTracker→DS activate/sleep；首套并未取得 UNKNOWN，不能计为超时通过。新 FactoryActivationObserver 只读 registry 和 SCR DTO，DS须 ACTIVE，不获取目标服务对象，单调时钟有界轮询。非DS只证明实际注册，不声称完整业务健康；原全局ServiceUtil其他调用未在此修改或审计。

v2 新运行集证明激活等待有界后，原页面却仍把所有创建错误显示成“名称可能已被占用”。原截图保留；EntryClassUi 创建回调改为类型化错误处理。明确服务端错误码保留；传输中断/坏回执/无类型错误核定UNKNOWN，未自动重发；401/403保留正常登录处理。使用现有中英文 UNKNOWN 文案，不暴露私有异常详情。

## 测试范围

新增11项共享创建/授权/属性合同，3项无需取得目标对象的 activation observer 合同，3项生产客户端写错误策略合同；原132项保留，共149项通过。生产客户端因POM设计不进入JVM服务器class，策略合同直接编译同一份生产client源，再运行其纯错误分流；没有复制实现。生产 GWT login/denali 各六排列编译并安装通过。Core130与Wire21本轮未重跑。

旧产品有效红证据为 wildcard授权、UI元数据混入以及创建仍走legacy/超时自动删除/错误错误码等；首份红日志中ServiceLocator静态夹具未初始化的7个errors不作为产品红。修正夹具后8个行为失败单列；另一个Mockito matcher内部错误不作产品红。客户端合同首次试图直接引用未编进服务器JAR的client类导致编译错误，后改成编译实际client源；非产品缺陷。所有失败日志和报告保留。

## 原界面实际结果

原始服务创建面板（Services +）在首套与v2的正常创建中验证 Count=1、common name/description、真实CM/DS与磁盘值；无UI元数据混入。首套 PID76718 的慢工厂等待不受60秒限制，停止保留；v2 PID77683 的服务端观察修复已生效，但原创建回调掩盖UNKNOWN，截图与状态保留，已停止。

最终新 v3（IDEA Run `Kura Dedicated Creation v3 20261107`，PID78371）加载最终283 inventory/282 OSGi bundles，原admin/admin正常登录，未装agent；本轮不改生产安全设置。两个不同新PID `creation.slow.en` / `creation.slow.zh` 各经原英文/中文页面只提交一次。80秒测试DS激活尚未完成时，原页面正确显示“结果未知、不要再次提交、检查当前配置与恢复状态”，截图固定。后续只读观察均见原CM实例始终changeCount=1、无删除重建、DS最终只激活一次；Integer Count=1及工厂/common字段在原磁盘快照中一致。中英切换使用原登录页，没有重发英文请求。

该证明涉及真正的非幂等工厂创建已经发生、随后激活观察超时，并非凭空模拟写失败。但它没有中断创建本身或snapshot写，也没有证明进程重开/丢回执的持久结果核定。非DS分支仅实测单元层注册观察，不声称全部programmatic factory业务健康。截图时间在DS激活前，native proof没有冒称精确60秒的完整RPC硬期限。原ServiceUtil对其他调用的无界激活风险未全局修改。

v3完整proof、原观察时间线、CM实例ID、快照与283 inventory已核对；JVM已正常SIGTERM并确认消失，旧现场保留，不重新启动。v1/v2中间运行JAR和报告均冻结；最终源码包含之后的observer/client修复，不把中间制品当最终制品。

## Linux 实际结果

最终产品JAR复制到新的Orb `/opt/yofc-gwt-creation-orb-97a5ba2a` / PID750943 与现有Parallels新单元/PID2694。仅fixture probe额外包含两次共享GWT helper调用与独立admitted/receipt；没有重新调用旧测试入口，也没有在Linux冒称实际浏览器RPC。每处fast Count=2成功；slow Count=3在60.017秒/60.045秒返回实际CONFIGURATION_OUTCOME_UNKNOWN，原DS约80秒后完成。每个CM changeCount=1、DS事件=1，原CM实例不删除重建、磁盘Integer和factory/common值一致；所有283 inventory hash核对，Parallels另184私有JRE文件固定。

Orb实际ProtectSystem=no/NNP=no（原全局覆盖），不称严格沙箱；Parallels实际strict/CapEff=0/NoNewPrivs=1，nsenter外部写EROFS=30。两处nft hash、IPv4/IPv6 forwarding均与入场相同。新增单元均已停止并确认MainPID0/inactive，Ubuntu随后正常停止。辅助PRL python -c只读观察因包装层shell引号解析失败，没有派发动作；继续查询同一个原verify handle得到成功结果，未重启或重发。

这是共享GWT helper→真实Core/CM/DS/快照验证，没有新agent、中心传输或真实网络控制写，也没有证明跨所有native factory的业务健康或持久UNKNOWN核定。正式新deb未构建/安装，原Orb四服务原PID与26/20包保持。

## 工厂适用面台账

从最终实际283项运行集固定JAR离线解析OSGi声明：71项factory metatype Designate，其中69项生产/2项fixture；69项中12项匹配到未声明immediate的延迟DS，1项未匹配DS，XML解析错误0。完整表 `declared-factory-scope.json`。这是声明级清单，不是实时SCR/激活状态或动态provider覆盖证明；12个延迟DS的业务契约须优先继续，不能用本轮immediate验证替代。全组件覆盖仍未完成。

## 当前未闭环范围

观察策略对DS要求ACTIVE；已有 `kura-container/.../OSGI-INF/containerinstance.xml` 未声明immediate，属于延迟服务，不能凭本轮immediate测试工厂推断其创建行为完整。delayed/SATISFIED工厂的明确准入/激活契约与真实业务验证继续内部pending，须优先处理，不能将长期UNKNOWN当最终产品行为。通用 delete 仍是 PID-only RPC，需要客户端显示 revision 与条件删除计划；raw updateProperties、driver/asset/channel/CSV/import/clone/dirty-form、所有native factory/metatype/实际端口/组件面仍需继续。正式最低API/包版本与安装升级兼容、持久creation请求回执/丢回复核定及进程重开、非幂等工厂部分派发和snapshot失败的完整恢复均未证明。

原生产/外部PID、空/部分/不一致CM恢复、NM/NAT writer归属、加密中心防火墙与远程中英编辑/受影响管理路径、实际早启/Core正式交接、一致DB/制品/密钥备份恢复轮换迁移发行，完整台账保持。fullFirewall/fullKuraRecovery/fullComponentCoverage均false。外部deferred：真实NM/cloud-init/物理硬件、amd64、disk-full/hard-power，单独列示。初始历史CONFLICT确切锁持有者、旧3份XML原文缺失及两项旧evidence gap不改变。

入场18仓完整HEAD/branch/porcelain与585 source/1854 artifacts/3189 evidence全部匹配；旧源码/可变制品先归档。所有旧请求/状态/快照/密钥/卷和工作树保留，不提交推送/reset/clean，不连接192.168.2.125。本轮各实际运行集均为新UUID，不重放已闭环的一次性入口。
