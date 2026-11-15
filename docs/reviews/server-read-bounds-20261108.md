# 5ab34951：Core 自配置 provider 读取隔离

2026-11-08；前序01e7f75c。本轮progress，目标active，整体改造未完成。先核对前序全部指纹、18仓及原运行状态。

## 发现与实施

Core readRevision、整栈读取、条件写接纳及快照遍历会调用SelfConfiguringComponent.getConfiguration；同步调用不响应时可能持续占住Core monitor。新增真实Core对象回归在旧实现上失败：3.5秒后仍不能得到拒绝，原报告保留。

新增内部ProviderConfigurationReads，接入原Core自配置provider读取，不修改公共API或导入sibling实现：

- 每个Core实例固定最多2个daemon工作线程、2个排队任务，闲置30秒回收线程；所有工作仍交给实际provider。标准provider读取名义2000ms期限，包含排队/服务获取/回调/验证，非硬实时保证。
- 按服务引用抑制未结束的顶层重复读取。超时请求尝试interrupt，但不把Future取消当成回调已经终止；忽略中断的工作继续占原容量，不能通过重复请求生成无限工作线程。
- 实际getService/回调/ungetService都在同一工作体内；等待超时或组件deactivate不提前调用unget。只在回调实际结束后的finally释放。排队超时任务移除，不得稍后执行。
- 只读provider聚合可在同一个工作线程嵌套，最多8层并拒绝循环；不额外分配工作线程。多个组件遍历/过滤读取、readRevisionViews及条件更新的接纳前遍历共享既有读预算，不逐provider重置。
- provider不可用不能生成可写版本。readRevision及接纳元数据检查映射typed CONFLICT，尚未派发的读取不宣称写结果UNKNOWN；普通读取/快照中传播配置错误，避免将超时当成缺失配置静默保存。派发后的现有uncertain保护保留。
- rollback读取定义新增受检错误后纳入原causes集合，不能静默跳过失败定义。Core deactivate关闭该实例的新读工作。

旧getConfiguration回调改为工作线程执行。已检查BaseAsset和NM代表读取，但没有穷举所有第三方provider：依赖调用线程、调用方持有的monitor、ThreadLocal或从回调重入同步Core条件API的实现仍需兼容性验证。此实现保留Core接纳锁内的既有原子性，不冒称已移除全部锁/回调交叉依赖。

## 测试与原生证据

最终Core configuration模块JDK21/Maven3.10显式缓存install通过，155 tests/0failures/0errors/0skipped（旧150加5项新回归）。新回归覆盖Core monitor释放、服务不提前释放、组件deactivate、接纳元数据CONFLICT、嵌套/循环、共享预算、工作线程/排队上限及取消排队、原异常保留。focused5及第一轮完整155报告/JAR独立留存；中间受检异常传播接线的编译失败日志保留，最终修正通过。原始negative focused1失败（TimeoutException）报告/旧源码与旧Core可变产物存before目录，不重建历史字节。

新IDEA完整隔离Kura PID9360/HTTP18601，283inventory（282实际bundle+探针），无agent，启动业务写入0。探针注册真实OSGi SelfConfiguringComponent stack.provider，由实际Core自动跟踪、实际Core审计门面调用。provider平时返回私有只读配置，挂起后刻意忽略interrupt；未用伪造Core接口或延迟代理替代此次服务器实现。

| 步骤 | 实测 |
|---|---|
| 首次挂起并readRevision | 2006ms后typed CONFLICT；回调仍未返回，interrupt被忽略1次 |
| 同时从另一线程读取真实Clock Core版本 | 成功，证明provider没有继续占Core monitor |
| 对同一仍运行provider再读 | 约1ms CONFLICT，heldCalls仍1，没有启动第二个回调 |
| 工作池 | maximum2、size2、active1、queued0 |
| 显式释放原provider | 实际原回调返回，active0/queued0，随后新版本读取成功 |

Core uncertainty始终为空。没有配置更新/快照业务写命令，没有UNKNOWN重发。服务注册ID保持。OSGi服务引用精确释放调用在Core集成单元回归验证；原生场景证明实际回调未返回/返回和资源占用，不把Future取消称为实际停止。

PID9360已停止，原runtime/home、命令admission/receipt、日志、状态及快照观察原字节保留。原Orb四服务package26/20、Parallels两VM stopped与18仓在清单封口实时核对；没有新Linux夹具，没有重放旧一次性入口。UI210、cloud helper12、Camel8等前序测试保留而未重跑。

证据根target/server-read-bounds-20261108；清单docs/evidence/server-read-bounds-20261108.json；verify-native.py/native-result.json/test-counts.json；各阶段日志/报告、idea-launch截图、scene-before-stop。Core子仓图谱更新，6个关键路径coverage metadata_match，best effort；初始跨仓实现检索仅scout，不声称全provider枚举。

## 范围与继续台账

关闭选定Core自配置provider读取的等待/线程/排队/重复接纳与原回调释放后恢复子项。所有限制均为单个Core服务实例；忽略中断的provider仍可能占用工作线程，不能保证停止第三方Java代码，也不能保证跨bundle/classloader反复重启的累计线程上限。组件deactivate的引用保留不等于framework撤销BundleContext后仍能延续服务生命周期。

尚未覆盖整个服务器操作期限：普通ConfigurableComponent/ConfigurationAdmin/密码服务/外部owner、元数据source与AD.validate、marshaller/文件I/O/实际写provider、完整plan/rollback全部路径，及等待其他Core/source monitor的时间仍需审计。某个SCC不可用时完整快照会失败关闭，不能宣称其他配置写一定可提交；派发后快照超时的业务UNKNOWN裁决仍在原台账。原生真实生产NM/硬件/provider全兼容性未验，不能据此升级通过。

完整14项pending逐项继承并缩小已证范围，原26项disposition链及外部deferred独立保留。其余专用计划/字段/工厂健康/审计/所有权、durable UNKNOWN/lost reply/重启、空Core/外部PID恢复、NM/防火墙管理链/早启动/备份恢复密钥迁移发行未整体闭环。历史初始GWT CONFLICT精确持有者与既有证据缺口不补造。

下一步优先审计ProviderDefinition.withSource/withValidatedProperties与SelfConfiguring读取/快照的锁交叉、回调线程兼容性和接纳前后超时分界，补完整plan与rollback读预算及实际派发后provider超时证据，再推进其余完整台账。不要把本轮SCC局部读取隔离当成全服务器取消/全Kura恢复。

不提交推送、不reset/clean、不清理个人运行集、不连接192.168.2.125；UNKNOWN不重发。
