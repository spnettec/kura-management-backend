# 01e7f75c：真实三成员栈迟到回复对照

2026-11-08，前序5661efcd。本轮progress，目标active，整体改造未完成。生产代码/产物未改，不重新构建或重跑已钉单元测试；补齐前序云栈迟到视图守卫的原生证据。

## 对照边界

先核对前序所有指纹、18仓及原运行状态。两个依次运行的IDEA完整新隔离实例，各283inventory/282实际bundle+同源码探针，无agent。before-guard PID6971/HTTP18599使用5661efcd intermediate-first-build中留存的UI JAR（已有读回超时、尚无isReadPending云视图守卫）；after-guard PID7153/HTTP18600使用当前最终UI JAR。其余生产bundle逐一SHA相同，探针源码相同；只在私有运行集使用历史JAR，没有回退源码、覆盖旧产物或重放旧闭环入口。

各自原生Raw工厂创建stack.wait三成员，原英文GWT读取DataService重连间隔60。探针取得唯一实际RawMqttCloudConnectionFactory，在其configurationService字段安装ConfigurationService+ConfigurationRevisionService双接口委托：所有调用先交给原ConfigurationServiceAuditFacade，只有真实readRevisionViews完成后，保存该次实际脱离值/版本的返回Map再延迟返回。没有伪造配置、版本、工厂或Core业务结果；保持原工厂唯一性、实际Core写逻辑和三成员注册。

一次UI保存60→61及快照成功确认后，五次整栈读回分别捕获实际61及三成员版本并挂住，客户端进入saved/unavailable。随后只开放新读取，旧五次仍挂住；关闭警告但不切换选择/不改变代次，以独立旧式Core写者把DataService推进13（每实例一次受控并发写，非重试）。释放旧五次，它们仍返回真实旧Map61。此顺序避免用显式刷新产生新选择代次而掩盖同代次迟到副作用。

## 结果

| 阶段 | 无守卫对照 | 当前守卫 |
|---|---|---|
| 原UI写入61 | 1次写/1次确认，CM count2 | 相同 |
| 五次整栈挂住 | saved/unavailable、字段禁写 | 相同 |
| 独立Core写者13 | CM count3、原值13 | 相同 |
| 旧61回复返回 | 字段重新可编辑且显示61，实际Core仍13 | 字段继续禁用，New Pub/Sub与Delete继续禁用，不重建旧视图 |
| 显式新读取 | 没有提交过期写，保留错误现场 | Refresh回到默认连接后，重新选择stack.wait才取得实际可编辑13 |
| 独立新UI意图62 | 未进行 | 保存/读回成功；最终UI写/确认2/2，CM count4 |

两个实例的三成员serviceIds保持；每实例root与transport版本不变，只有DataService版本按写入推进。Core uncertainty为空。原UI条件写计数器不包含那次独立旧式Core写入；不能把最终2/2说成全部CM写入。没有点击Connect，auto-connect一直false，无MQTT投递、无过期写入尝试、无UNKNOWN重发。

## 证据与保留

机器验证脚本target/cloud-late-readback-20261108/verify-native.py，已执行通过；native-result.json；contrast-inventories.json；两个实例*.result.json和原始admissions/receipts；held-stack-read-{1..5}.json记录实际旧版本/61，returned文件证明实际旧请求结束；原AX/PNG；scene-before-stop日志/状态及快照观察原字节。当前UI源码及210测试证据来自5661efcd、未重跑；Core150/helper12/Camel8等前序结果同样保留。

两个新PID6971/7153均已停止，原目录/scene保留。原Orb四服务package26/20、Parallels两VM stopped及18仓在清单封口实时核对。清单docs/evidence/cloud-late-readback-20261108.json。图谱按UI/cloud真实子仓，五个相关路径coverage metadata_match，仍为best effort。

## 完整台账继续

关闭“选定真实Raw三成员栈：五个挂起读回、同代次旧全栈回复、独立写者推进、守卫不重建视图、显式新读及独立新保存”的子项。前序14项pending逐项继承并缩小已证范围，原26项disposition链保留；外部deferred独立原样保留。

仍未建立服务器请求取消、真实provider执行/队列/Core锁有界性；两个夹具中的旧读靠测试门控显式释放，不能泛化为生产取消。restricted挂起恢复、其他provider/Linux、全部dirty/并发/程序化选择/成员变化、所有字段/密码/空值、专用业务计划/工厂健康/所有权/审计、durable UNKNOWN/lost-reply裁决/重启、空Core/外部PID恢复、NM/防火墙与受影响管理链/早启动/备份恢复密钥迁移发行未整体闭环。历史初始CONFLICT精确持有者及既有证据缺口仍未证实，不补造。

下一步按完整台账审计真实Core读取→SelfConfiguring provider→同步锁路径，落实服务器端执行/排队/资源上限；不要把客户端超时或此只读委托当成服务器取消能力。源码未改，无提交推送/reset/clean/个人运行集清理/192.168.2.125连接。
