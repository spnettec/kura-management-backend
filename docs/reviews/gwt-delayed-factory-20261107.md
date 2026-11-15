# GWT 延迟工厂就绪与首次消费验证

2026-11-07，031c0ed0，续接 c3819568。整体目标 active，未暂停、未完成。合同 `../configuration-plan-local-v1.md`，证据 `../evidence/gwt-delayed-factory-20261107.json`，原始记录 `target/gwt-delayed-factory-20261107/`。

## 产品行为

通用创建已经先完成 Core typed plan、真实 CM 与 snapshot；随后只读观察注册和 SCR，不获取目标服务对象。之前所有 DS 必须 ACTIVE 会让正常延迟服务一直 UNKNOWN。本轮明确为注册就绪：当前 ComponentConfigurationDTO 的 component.id、ServiceReferenceDTO.id 必须与匹配服务引用一致，当前 description 必须存在；ACTIVE 可就绪；SATISFIED 仅在当前声明非 immediate、无 ComponentFactory 标识且确有 serviceInterfaces 时就绪。immediate SATISFIED、缺必需引用、FAILED_ACTIVATION、缺描述或旧 service ID 仍不可就绪。不会为了成功提示提前激活延迟服务。非 DS 仍只证明注册，不代表业务健康。

依据 [OSGi DS 8.1 延迟组件生命周期](https://docs.osgi.org/specification/osgi.cmpn/8.1.0/service.component.html)：延迟服务满足依赖时可先注册，取得服务时再激活。这里的成功表示该合同的配置与注册就绪；首次消费者业务结果另行验证，不把注册成功冒充所有业务健康。

新增 ServiceReferenceDTO 字段访问暴露实际打包遗漏：web2 未导入 org.osgi.framework.dto，首次实际运行在 Core 已创建后抛 NoClassDefFoundError，客户端正确保留 UNKNOWN。已在真实 MANIFEST.MF 增加 `[1.8,2.0)` 导入，冻结 Equinox 的实际 export 为 1.8。测试 probe 自身也补同一 import，未更改产品认证或放宽错误处理。

## 回归与失败保留

6项就绪合同覆盖延迟 SATISFIED 不实例化、immediate 仍等 ACTIVE、采用当前 DTO 描述、缺依赖/失败、不匹配注册与缺 DTO、ComponentFactory/无服务声明。旧 observer 三项测试的首个夹具此前未设置 immediate 却要求等待 ACTIVE，现明确 immediate=true，保留其真实超时回归意图。

就绪红证据为6项中2项行为失败、0error；实现后受影响20项通过。新增1项实际打包字节码链接合同：从产出 web2 JAR 加载真实 observer/util，按其声明 org.osgi imports 限制依赖可见性，再实际访问 SCR DTO 字段。修改 manifest 前复现缺 org.osgi.framework.dto 的 NoClassDefFoundError，修复后通过。此合同是所选链接闭包，不替代完整 OSGi resolver 验证；实际冷启动也验证最终 JAR。

最终 GWT156 全部通过，生产 login/denali 各6 permutations 编译安装成功。Core130/Wire21未改，沿用，不重跑。UI 真实子仓库图谱已重索引，三个新/改 Java 路径准确返回；MANIFEST 非代码文件不在 File 查询结果，直接读取实际打包声明和运行 export 验证，没有把图谱缺项当不存在。

## IDEA 原界面与真实业务

首套 `gwt-delayed-factory-ac1714fe` / PID79995 在原中文页面只创建一次 delayed.lazy。Core CM changeCount=1与snapshot已经写入，随后缺 DTO import 导致 UNKNOWN；未重发、未删除。probe 也因自身相同缺包中断，末份 state-latest 为过时初态，不能作为服务未注册的证明。日志、原截图、快照、运行集全部保留。收集脚本曾误用 home/log 路径，随后按实际 home/logs 归档并停止原PID；这是归档脚本错误，不是第二次产品执行。

修复后的全新 v2 `gwt-delayed-factory-v2-474818c3` / PID80616：原 admin/admin 控制台登录，未装agent；测试 profile 的旧首次密码闸门设置仅限夹具，未改产品安全默认。原中文 Services+ 提交 delayed.lazy，Count=1、名称/描述与磁盘值一致，成功页面可读取配置；创建及元数据读取后 SCR=SATISFIED、serviceRegistered=true、业务事件0。明确的首次消费者取得服务后 SCR=ACTIVE、业务事件1、answer=1，CM实例/service ID和changeCount=1保持。

同一运行集中调用实际共享 GWT helper 创建真实生产 `org.eclipse.kura.core.db.H2DbService`，只使用全新独有内存数据库URL；创建后 SATISFIED，首次普通消费者激活并通过真实 JDBC 建表/插入/第二连接查询 marker=first-consumer-proof。H2未经过浏览器创建RPC；fixtureBusinessEvents仅统计测试Business，H2的0不代表未激活，真实SCR与SQL才是该证明。

原中文页面另只创建一次 delayed.missing，缺必需 DS 引用，60秒观察后实际显示 UNKNOWN。原 CM changeCount=1、Integer1与快照保留，SCR=UNSATISFIED_REFERENCE，无服务注册/业务事件。随后只注册缺失依赖，不重发创建，原CM实例变为 SATISFIED且业务事件0；首次消费者取得原服务后 ACTIVE、answer1、事件1，CM仍changeCount1、同一service.pid。原始回执与截图固定。最终283 inventory全部哈希匹配，PID正常停止确认消失，现场保留。

## Linux 所选范围

Orb 与现有 Parallels 使用相同最终283项产品/探针运行集，探针无启动自动业务写。每个新请求具有独立 admitted 标记和原回执：lazy创建Count7先SATISFIED/事件0、首次消费ACTIVE/answer7；真实H2先SATISFIED再首次SQL；缺依赖创建Count9实际等待UNKNOWN，原CM/快照保留，补依赖仅令原实例SATISFIED，再首次消费ACTIVE/answer9。未走浏览器RPC、agent/中心传输或系统网络控制写。

所有服务操作均针对本轮新UUID单元；原Orb四服务和原26/20包不重启。Orb原全局覆盖导致 ProtectSystem=no/NNP=no，不能当严格沙箱；Parallels 实际 strict/CapEff0/NoNewPrivs1，外部写 EROFS30，私有JRE184。各处 nft hash、IPv4/IPv6 forwarding 入出场一致。Orb新单元 PID752036、Parallels PID2574 的缺依赖观察分别为60.057秒与60.025秒。两单元均已停止并确认 MainPID=0/inactive，所有现场保留；Ubuntu随后正常停止、Windows保持停止。准确沙箱属性与停止回执见各原始 result.json 与最终manifest。

## 继续台账

本轮闭环的是通用延迟注册就绪合同、所选真实H2首次业务和缺依赖晚就绪。上一轮69生产factory声明中12延迟DS仅H2获得本轮生产业务实测；不能宣布全部12、未匹配DS的一项或动态provider/所有组件健康通过。通用 displayed-baseline revision条件删除与原PID-only删除、raw updateProperties、asset/driver/channel/CSV/import/clone/dirty-form专用路径优先继续；正式最低API/包版本与安装兼容、持久creation/丢回执核定、进程重开、部分工厂派发/snapshot失败完整恢复仍内部pending。

生产/外部PID和空/部分/不一致CM恢复、NM/NAT其他writer归属、加密中心防火墙与中英远程/受影响管理链路、confirmed-only实际早启/Core正式安装交接、一致DB/制品/key备份恢复轮换迁移与发行，保持全台账。fullFirewall/fullKuraRecovery/fullComponentCoverage均false。external deferred单列：真实NM/cloud-init/物理硬件、amd64、disk-full/hard-power；不把Java可用重新记为缺环境。

历史初始CONFLICT确切锁持有者未证实、旧3份Surefire XML原文缺失和两旧evidence gap分别保留，不声称补齐。入场18仓及591source/2710artifact/3425evidence全部匹配；变更前归档30项旧源码/可变制品，旧evidence字节保留。不提交推送/reset/clean，不清理现场，不连192.168.2.125，不重放任何旧一次性验收。UNKNOWN始终不重发。
