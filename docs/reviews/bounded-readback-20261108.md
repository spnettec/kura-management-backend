# 5661efcd：挂起读回的客户端期限与迟到回复

2026-11-08，前序1f93dd7c。本轮progress，目标active；整体改造未完成。

## 从实际证据调整方向

先核对前序全部指纹、18仓和运行状态。新IDEA原生实例PID5424，通过原英文GWT创建stack.first/stack.second SSL实例，只挂起stack.second的真实版本读取；普通点击和Up键均未将选择切回first。截图显示RequestQueue等待遮罩，读取释放后仍选中/显示second，CM及服务状态不变。未复现前序源码风险所假定的普通用户选择竞态，未修改restricted选择代码、未记修复；程序化clear/refresh、内部异步选择及全部竞态仍pending。等待图已保留，不为得到失败绕开UI或伪造选择。

## 实际实施

ConfigurationSave旧实现只有五次返回失败后的重试上限；读取永不回调时永远停在第一次读取。新增回归在旧实现上实际失败（focused11项/1失败，expected5reads实际1），原报告及日志保留。

现在每次已确认写入后的只读尝试有2000ms客户端期限，失败后200ms再读，最多五次。全部读无回复时名义期限10800ms，浏览器事件循环/后台节流影响实际调度，非实时保证。只读超时只报告“已保存但无法读取”，不触发writeFailed、不合成UNKNOWN、不重发写入。未确认写入没有启动观察计时器，仍按现有UNKNOWN合同处理。

过期/重复回调不得采用新基线。为有视图副作用的reader提供isReadPending，CloudConnectionConfigurationsUi在迟到全栈回复重建标签前检查；超时或已终止的回复不得重建视图或恢复权限。泛型/受限直接读回仍由同一callback守卫。新增三项测试覆盖无回复终止、旧回复不得覆盖已就绪基线，以及未确认写入不由观察计时器裁决；虚拟时钟验证五次无响应名义10800ms。

## 原生挂起RPC验证

最终构建后新IDEA完整隔离Kura PID6469，HTTP18598，283inventory/282实际bundle+探针，无agent，Clock emulator不会同步系统时间。探针仅在真实Core写入和快照返回后阻塞该PID的原生版本读取，所有原调用保持，只延迟只读回复，不伪造值/版本。

- 原GWT一次false→true保存，真实write dispatch/ack1/1，CM count2，Core配对实际值true，serviceId154保持，uncertainty空。
- 五个实际读回RPC全部挂住、没有一个回复，原界面仍进入“Changes were saved, but the current configuration could not be read...”禁写终态，所有输入/写按钮禁用。读请求起始间隔约2207/2202/2203/2203ms。
- 只释放读门控，五个旧回复实际返回。界面仍禁写，无写入、无新baseline自动采用。随后显式重选才读取实际true及新版本并恢复输入。
- 独立新用户意图true→false一次保存成功，最终dispatch/ack2/2、CM count3、实际false、serviceId154，uncertainty仍空。第二次保存不是重试，UNKNOWN未重发。

两个新PID5424/6469均已停止，原现场和scene-before-stop日志、admission/receipt、快照观察原字节保留。没有新Linux夹具、没有重放旧闭环入口。原Orb4服务/package26/20、Parallels两VM stopped及18仓在封口实时核对。

## 构建和证据

JDK21/Maven3.10，显式本机Maven缓存。最终UI reactor install及GWT login/denali各六排列通过；210 tests/0errors/0failures/0skipped。第一次210成功报告/JAR另存intermediate-first-build；旧1f93 UI可变原字节存before；expected-negative失败报告另存unanswered-read-negative-reports。Core150/helper12/Camel8等前序测试未重跑。图谱真实UI子仓，七个材料路径coverage metadata_match，best effort；不声称全图无缺口。

证据根：target/restricted-selection-race-20261108（沿用本轮最初目的的目录名，未清理或重命名）；清单：docs/evidence/bounded-readback-20261108.json；native-result.json、test-counts.json、bounded-readback/*、before-fix/*与PNG/AX/原生回执。

## 明确边界及台账

此实现限制客户端观察等待及尝试次数，不取消已发出的服务器请求、不打断真实provider，不建立服务器线程/队列/同步Core锁的有界性。最坏允许五个旧读请求仍在服务器中；生产provider执行期限/隔离/取消仍pending。真实云栈迟到全栈回复只加了守卫，尚未完成原生挂起实测；restricted读回超时和所有provider/Linux同样未泛化。本轮单实例是通用Clock emulator读回，不等于全系统或硬件验收。

前序14项pending逐项继承并只缩小客户端无回复子项，原26项disposition链保留；外部deferred独立原样继承。其他dirty/并发/选择/成员变化、所有字段/密码/空值、专用计划/工厂健康/所有权/审计、durable UNKNOWN/lost-reply裁决/重启、空Core/外部PID恢复、NM、防火墙与受影响管理链、早启动、备份恢复密钥迁移发行仍未整体闭环。历史初始CONFLICT精确持有者及既有证据缺口不补造。

下一步：真实云栈/受限迟到回复与provider边界。优先在新隔离实例对实际Raw工厂注入仅延迟readRevisionViews的ConfigurationService+RevisionService委托（保留原Core写逻辑、工厂唯一性），验证旧全栈回复不重建视图，再推进服务器有界性。不要把已验证普通等待遮罩问题重复当成可复现选择错误。

不提交推送、不reset/clean、不清理个人runtime、不连接192.168.2.125；UNKNOWN不重发。
