# 集合配置、原 GWT 表单与 XML 元数据适用性核定

2026-11-07，核定 **bde06c22**，接续 01258353。整体目标 active，完整管理体系未交付。本轮有产品源码改动；此前快照容器、恢复补偿、restart/reboot、交接取消的一次性验收保持原证据，没有重放。

## 实现与范围

- Core revision、约束校验、深复制和值比较支持一般 Collection，允许 CM 将 Vector 规范化为 ArrayList；顺序、重复、元素类型与密码比较仍参与检查，嵌套/循环和超限容器保持拒绝。既有 Vector 的 canonical tag 不变。
- bridge 读取一般集合，密码集合保持脱敏；版本化在线更新保留已有基本类型数组的具体类型。审核 PID 白名单不变，配置默认关闭。
- 原 GWT 的三处元数据值投影统一到同一转换函数，支持一般集合和基本类型数组。负 cardinality 正常生成字段，已有超过十项的值全部显示；服务端容器上限 256。原有 0/±1 的标量兼容约定未迁移。
- GWT 密码集合按索引保留占位符对应的已有密码，深复制后回填，保留空密码与首尾空格，不修改请求 DTO。没有对应原值的纯占位符不落盘；混合未解析占位符与新值拒绝。已有空字符串元素与字符串首尾空格在本轮保存路径中保留。属性回填按稳定 ID 查找，不按显示名称。
- XML 元数据从 bundle 顶层 `OSGI-INF/metatype/*.xml` 读取；通过 Designate 的 pid/factoryPid 与 Object.ocdref 找到对应 OCD，不再要求文件名、PID、OCD ID 相同。保留本地化；重复资源的同一 PID 和悬空引用不任意选择默认值。MetaTypeService 暂缺时返回空结果。

XML 发现规则参考 [OSGi Meta Type 8.1](https://docs.osgi.org/specification/osgi.cmpn/8.1.0/service.metatype.html)。本轮没有实现动态 MetaTypeProvider 的完整注册、修改、移除生命周期，不能据此宣称所有合法 provider 和全部组件已适用。

## 受影响验证

最终 Core/API/crypto/XML/离线工具依赖 reactor **459** 项、bridge **55** 项、GWT **87** 项全部通过，无失败/错误/跳过。GWT 主代码以 Java 11 目标编译、六个 permutation 构建通过。Rust agent/console 与 React 本轮未改，不重复其历史验收。

随后依序执行新的 IDEA 完整隔离 Kura、OrbStack、现有 Parallels Ubuntu ARM64。每个成功运行集 **283 inventory / 实际 282 bundles**，全新 home/CM，无新 agent/中心：

| 环境 | 新现场 | JVM | 最终状态 |
|---|---|---:|---|
| IDEA v4 | `.phase1-dev/applicability-idea-v4-20261107-eb434e31` | 65968 | stopped |
| OrbStack | `/opt/yofc-applicability-orb-20261107-0818f485` | 706619 | 新 unit inactive/MainPID=0 |
| Parallels | `/opt/yofc-applicability-parallels-20261107-acfe1456` | 2597 | 新 unit inactive/MainPID=0，VM 正常 stopped |

各运行集从新工厂配置种植 1，调用**实际注册 bridge 的直接 frame 合同**版本化更新到 2，再通过浏览器 admin/admin 登录原 GWT，仅将 Script 改为 `gwt-verified` 并确认保存一次。没有声称这是 agent/UDS/mTLS/远程任务验收。每个现场实际 DS 记录 `1 → 2 → 2/form=true`，typed ConfigurationService 回读同时核对 int[] 类型、12 项有序字符串集合（含空元素及首尾空格）、Long.MAX_VALUE 与两项密码（含空密码、首尾空格）。界面中的密码持续为占位符，秘密未回显。任意 XML 文件名/shared OCD/factoryPid 与中英本地化由实际加载链覆盖。新运行集不安装 agent，Kura 独立启动与登录通过。

本轮是普通配置链路验证；`fullRecoveryVerified=false`，没有 normal/health/completion/写释放，没有新的恢复、补偿、重启或重开旧入口。新元数据范围在受控恢复下的实际业务仍需后续新场景验证。

## 保留的失败与纠正

单元基线先复现一般 Collection revision 不支持与 GWT 投影错误。首次 IDEA 发现详情服务仍走重复 Object[] 转换：v1/v2 均未保存表单，停止保留。统一全部 RPC 转换并加入实际 RPC 属性生成方法回归后，v3 显示正确，但保存时客户端 trim 破坏字符串空格；快照已变、DS 不满足业务校验，**未记通过、未重发保存**。修正客户端字符串提交后，独立 v4、Orb、Parallels 通过。四套 IDEA 的 launch marker、home、CM、日志与失败现场全部保留。

旧测试曾固定“用显示名查密码”的实现行为，本轮改成属性 ID 与显示名不同的实际合同。最终 87 项通过；此前失败日志保留。浏览器首次早于 Jetty 就绪出现连接拒绝，保留现场并在同一成功 JVM 就绪后使用新 tab；没有因此重启该 JVM或重发写。

## 收尾与剩余工作

18 仓 HEAD/branch 未变，无提交/推送、reset/clean、远端覆盖。原 Orb 四服务仍为原包及原 PID：console 244516、center2 244911、agent 286620、phase2 agent 268。四套 IDEA JVM 和两个本轮新 VM unit 全停；Ubuntu 与 Windows 均 stopped。没有启动新 PG、console、agent 或防休眠进程；所有 fixture/state/key/snapshot/backup/quarantine/卷保留。

内部 pending：动态 provider 生命周期及校验合同；0/±1 标量历史兼容与完整组件适用面；GWT 新增/删除空元素等完整编辑语义及其余类型边界；本轮元数据的受控恢复实际业务、生产组件业务、部分/空 CM/外部 PID 恢复；其他未决恢复运维和 Core 安装升级/bootstrap/启动策略交接、旧入口退役；NM/firewall/NAT/forward/远程流程/早启；一致 DB/制品/key 备份恢复、轮换、旧系统迁移和正式发行。已闭环的旧一次性验收不重跑。

外部 deferred 单列：真实 NM/cloud-init/物理网络硬件、amd64、磁盘满/硬断电。两条历史 evidence gap 继续单列。Parallels Java 环境可用，软件工作不能算外部 deferred。保留 NM、Java key/CSR、agent 常驻与独立通道开关、首登后台闸门、中英文、独立归属、UNKNOWN 不重发和 RECOVERY_REQUIRED 挡写保现场等核定不变。

机读：`docs/evidence/configuration-collection-metatype-20261107.json`；原始证据、源码前像/差异、全部构建日志、运行集描述符与截图：`target/component-applicability-v2-20261107/`。后续从最新知识库核定续接，不把历史提示中的 pending 自动当成当前状态。
