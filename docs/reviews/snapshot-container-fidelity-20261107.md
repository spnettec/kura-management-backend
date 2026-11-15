# 快照容器保真与本机业务恢复验证

核定 `ff350488`，2026-11-07。接续 `d91353d6`，完整目标保持 **active**。本轮推进完整组件适用面中的 Java 类型保真缺口，实际修改核心快照、恢复及密码集合转换。没有重放旧恢复、交接取消或 restart/reboot 入口。**本轮新增变更的 OrbStack、Parallels 系统验证仍为内部 pending。**

## 实现范围

历史适配器会丢弃空数组，基本类型数组、Vector/Collection 无法忠实进入快照；密码集合在 CM 字符串转换及加解密链中也缺少支持。本轮保留旧标量/对象数组格式，增加仅在 `array=true` 时有效的 `container=PRIMITIVE_ARRAY|VECTOR|LIST`，同步 DOM、流式编解码与独立 Java 校验器。

- 基本类型数组保留 component type；空数组保留为数组，区别于空值标量。
- Vector 的 XML 往返保留 Vector；其他同质集合规范化为有序 ArrayList，保持顺序、重复值和空集合。CM/SCR 可规范化集合的具体实现，因此业务核验针对 Collection 语义；数组仍要求精确类型。
- 密码集合经过与现有 Password/Password[] 相同的 AES、CM String 转换、类型重建和解密。恢复准入深复制集合与其中 Password，返回视图不再与准入现场共享可变集合。
- 完整 CM 备份/恢复允许这些可表示类型；任意对象、嵌套容器、异质元素、null 容器元素在变更前拒绝，快照写出也显式拒绝未表示对象，不再留下缺字段的“成功”表示。

这与 [OSGi CM 的集合顺序及同质值合同](https://docs.osgi.org/specification/osgi.cmpn/8.1.0/service.cm.html#service.cm-configuration.properties)一致。新属性需要配套更新的 mapper/validator；仅承诺新读取器继续读取旧快照，不承诺旧读取器正确处理新容器快照。正式发行兼容与升级策略仍 pending。未改变审核 PID、默认关闭写入/firewall、独立归属、UNKNOWN/RR 或首次密码后台闸门。

## 实际证据

证据根目录 `target/component-applicability-20261107/`，机读 [evidence](../evidence/snapshot-container-fidelity-20261107.json)。

| 层次 | 本轮结果 |
|---|---|
| 缺陷复现 | 初始23项测试为21断言失败、2测试参数错误；参数错误修正后独立保留原日志。最终24项容器测试全过 |
| Java reactor | API23、util136、core111、configuration91、crypto28、inventory1、XML4、offline55，共449项，0失败/错误/跳过 |
| CM 契约 | 真实 Equinox CM：XML→CM→framework restart→verify→inverse，包含基本类型、空容器、集合及实际 AES 密码集合；属于 API/持久化证据 |
| 桥接/浏览器 | bridge53通过；新 mock 浏览器1项核对中英文空数组/未设置区别及密码集合脱敏；React typecheck通过，未将 mock 记成真实远程恢复 |
| IDEA 完整 Kura | 全新 `containers-idea-v2-20261107-3b43ad12`，284项 inventory；现场 `getBundles()` 为283。原生 UI 依次启动 seed59428、apply59554、inverse59614，8类基本类型数组、4空容器、普通集合及密码集合均被 DS 回调和 ConfigurationService 核对 |

本机种植9→正向2→逆向9。候选 `snapshot_1793988564781.xml` SHA `3a83adbaf0dc55b29945f17d998c053914c85a67a35aeb697e029c6d080019a2`，26配置/7工厂/225属性，6个密码值全部认证、legacy=0。正向 `rec-a52185ff-40ba-4665-aa5c-3e36a04b3cb1` 的真实 CM_APPLIED 回执保留备份/意图/实际应用证据；逆向使用原加密 CM 备份及精确 source receipt/plan 绑定，恢复25记录、220属性，删除1条正向新增记录。两个受控 JVM 的普通 snapshot 写入均实际被闸门拒绝。

原 GWT 在 seed/apply/inverse 后均实际使用 admin/admin 登录；apply 后既有标量组件 Count1、密码脱敏表单正常。该 Count1 是保留的原配置，属于未改动服务的兼容观察，不与独立容器组件的9/2/9混淆。手工 Java bootstrap 的已核定 helper 路径只证明本轮所选 CM/DS 数据恢复，不是新的 agent/中心完整事务，也不释放写闸门。`fullRecoveryVerified=false` 和回执 `businessHealth=UNVERIFIED` 保留，实际 DS 核对另记探针证据。

## 保留的失败与新适用面问题

首个全新夹具 `containers-idea-20261107-4c2e8c07`、PID59299，因为测试探针错误要求 DS 收到的空 Vector 仍具有 Vector 具体类而失败。实际 SCR 交付 ArrayList；保留全部状态及异常日志，停止后另建 v2，没有重新运行该 seed。v2 修正集合断言，仅数组要求具体类型。第一次 v2 staging 在旧 JVM 停止过程中因端口尚未释放失败，发生在新目录/入口建立前；随后确认停止再准备，不重放任何业务入口。

另外发现并保留内部 pending，尚未修复或记通过：

1. `ComponentUtil.readMetadata` 按 `OSGI-INF/metatype/<pid>.xml` 查找，标准允许的任意 XML 文件名以及动态 provider 的完整支持尚需落实。v1探针原用 `business.xml`；v2 按现有 Kura 约定命名，未把夹具调整冒称产品已支持全部 metatype。
2. 原 GWT 的负基数 Password 集合组件显示空 form。登录及旧标量 form 正常不等于该新增表单通过，需要诊断 UI/RPC/cardinality 链。
3. 本轮没有扩展通用 Collection 的远程 revision/投影/编辑合同；已有 Vector 专用路径与真实 CM 集合实现之间的适用面仍需核对，不能自动放开 PID。

## 现场与后续

全部本机四个新 JVM已停；Vite59074及其子进程已停，15173/18453/18875/18876均关闭。两处 native 夹具、快照、CM数据库、请求、回执、加密备份和失败材料均保留，所有 seed/apply/inverse 均禁止重跑。原 Orb 四服务仍为244516/244911/286620/268及原console26/agent20包；本轮没有新建VM服务、中心或网关。Ubuntu、Windows仍 stopped。18仓 HEAD/branch/原dirty保留，无reset/clean/提交推送，不触及192.168.2.125、活动target/runtime或个人~/.kura-dev。

接续先以本轮固定源码/制品建立**新的** OrbStack 与现有 Parallels 夹具，完成新增类型恢复/补偿及原 GWT 登录验证；仅复用静态 bundle/JRE 位元输入，不能复制旧成功 state/receipt 或重跑本机入口。然后继续上述 metatype/GWT/集合适用面，以及原总台账的其他未决恢复运维、完整组件/provider/部分或空CM/外部PID离线及业务恢复、Core安装升级/bootstrap/策略交接与旧入口退役、NM/firewall/NAT/forward/远程流程/早启、一致DB/制品/key备份恢复与正式发行。外部 deferred 继续单列物理NM/cloud-init/网络硬件、amd64、磁盘满/硬断电等；两个历史 evidence gap独立保留。新类型的 VM验证是可执行的内部 pending，所选 Parallels Java环境已具备，不能记为缺环境。
