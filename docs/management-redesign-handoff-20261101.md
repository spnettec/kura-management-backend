# YOFC Kura 管理体系会话交接台账

更新：2026-11-01，Asia/Shanghai。本轮完成后按用户要求暂停目标，等待明确续接。
整个改造未完成，不能将goal标记complete。所有迁移与实施改动仍未提交/推送，禁止reset/clean。
权威方案为 `management-redesign-plan.md`，目录整合事实见 `project-consolidation-20261031.md`。

## 已确认且继续有效的决策

- 中心Rust/Axum/Tonic、PostgreSQL，React+TypeScript+Vite；同源HTTPS API/SSE、浏览器渲染。
- 独立Rust agent开机自启，不进入Kura JVM；agent主动建立gRPC双向流/Protobuf/mTLS。
  原Java/Vue/MQTT代码和历史保留，不要求兼容旧JSON信封，不重复前端合并/目录改名。
- console/agent注册认证完全独立于Kura Web认证。中心预登记证书并绑定网关，支持禁用和轮换；
  另一个中心普通注册不能抢占现有归属。生产注册/换中心的完整产品流程仍须按方案落实，不能“后注册踢前面”。
- agent与Kura用UDS和OS身份，Java桥接直接访问OSGi，不依赖Kura Web账号；原接口保留。
- Ubuntu24.04+、Netplan/systemd-networkd、有线；nftables、系统控制、离线恢复按后续阶段实施。
- console持续中英文；所有新文案同步翻译。多VM采用现有一台Parallels26.04（2CPU/4GB）配OrbStack，
  不增建重VM；凭据不写交接正文。普通卸载保留快照、密钥、用户数据，purge须明确范围。
- 代码发现先图谱，选择真实子仓库root_path，禁止索引父工作空间。知识库显式父工作空间scope=ws。
  不用子代理；验证顺序为单元/契约与浏览器→IDEA完整Kura及原控制台登录→OrbStack。
  不连接192.168.2.125，不清理运行target/runtime和~/.kura-dev；缺失环境不记通过。

## 本会话已经落地的主线

| 增量 | 状态与证据入口 |
| --- | --- |
| 阶段0/1 | Rust中心/agent、固定工具链/协议/权限/任务错误模型、登录/网关/API/SSE、mTLS心跳重连、UDS、Kura缺席仍管理；`phase0-contract.md`、`phase1-validation-20261101.md` |
| 双语与多VM | React中英切换、选择保持、业务码本地化；Parallels26.04只读受限验证；`console-i18n-20261101.md`、`phase2-parallels-validation-20261101.md` |
| 网络阶段2原型 | 单明确networkd来源转交/回退、确认、writer竞态防护、Kura配置owner、旧网络PID适配、原REST/GWT事务UI、显式none/netplan运行变体、dummy真实事务；相关phase2网络报告 |
| 显式发行入口 | core/netplan/networking候选包、独立agent包和身份导入/UID授权/维护防护已有受限证据；不代表完整发行矩阵；`phase2-netplan-package-validation-20261101.md`、`phase2-agent-package-validation-20261101.md` |
| 执行与敏感存储 | agent执行前密文持久日志、UNKNOWN查询、断流隔离、中心input/result加密及旧数据原子迁移；`phase2-task-journal-validation-20261101.md`、`phase2-task-storage-validation-20261101.md` |
| 网络UNKNOWN | 绑定原命令/认证计划和业务日志的网络终态核定、一次审计与双语；Linux真实dummy动作后的回执损失注入；`phase2-network-outcome-validation-20261101.md` |
| 配置读取/核心基础 | typed get、脱敏投影、opaque revision/密码keep-set/核心条件写、并发与CM ABA拒绝、失败挡写、container范围兼容修正；`phase2-configuration-read-validation-20261101.md`、`phase2-configuration-revision-validation-20261101.md` |
| 本轮受限远程配置更新 | typed Operation12、agent可选能力/持久UNKNOWN挡写、中心admin/CSRF/租户/加密/幂等、Java审核PID开关与editable、双语React明确确认；`configuration-remote-update-v1.md`、`phase2-configuration-update-validation-20261101.md` |

历史报告只证明各自当时版本和环境，不累计为当前全架构验收。尤其旧Java/Vue17项冒烟、
旧agent revision5、旧26.04只读沙箱不能替代最新配置写入/发行验证。

## 本轮最终验证

中心8、共享协议7、显式DB2和mTLS3均通过；宿主及Linux agent各60通过；Java bridge26通过/install。
React类型和生产构建通过，约1005kB chunk提示保留。最终默认浏览器23，0skipped/unexpected/flaky。
独立协议故障契约22通过，但使用测试Python桥接，真实Java写入0。

IDEA真实完整Kura：原GWT登录首次和重启后各4通过；真实React双语写入1流程通过，
中文修改Long.MAX_VALUE并保留密码，英文显式设置测试密码。只读观察器initial/keep/set/rejected/restart
各9项；两次写入CM各增1，密码对象/快照无明文/0600均核对。HTTPS拒绝15、重启11通过。
IDEA写JVM40357、重启41238已退出；新agent40778已停止，gateway已disabled，数据保留。

随后OrbStack Ubuntu24.04 arm64：独立新agent ELF、完整282项运行集SHA匹配，真实HTTPS写13、拒绝15、
重启11通过；五阶段各9项观察器。Kura UID993/CapEff0，agent CapEff0/NoNewPrivs1。
既有OrbStack全局service覆盖使Kura NoNewPrivs0，完整systemd沙箱deferred，未改全局覆盖。
新两unit均停止、gateway禁用，原VM agent8391仍active，8项旧fixture/Netplan SHA不变。
最终只读审计 `target/configuration-update-final-audit-20261101.json`；源码/制品/证据SHA清单
`docs/evidence/configuration-update-20261101.json`。旧revision清单保留历史，不能用来核对最新源码。

失败全部保留：首次IDEA私钥0644被正确拒绝，确认零任务/无SQLite后仅收紧0600恢复；
首次Linux编译缺协议workspace根，补齐派生成员清单；首次Linux status错误要求受全局覆盖的Kura NNP，
改为真实报告deferred后只读续接，未重放start。最终浏览器回归曾被typed429拦截1项，
完整限流窗口后串行23项通过；不放宽生产限流，不倒推更早未知原因的登录失败。

## 停止时环境与保留数据

| 环境 | 当前状态/不可重放入口 |
| --- | --- |
| 宿主开发中心 | HTTPS18443/gRPC18444、独立PG仍运行；原host agent27974保留（旧运行二进制、未开启配置写）。密码/密钥只在.phase1-dev，禁止输出 |
| 本轮IDEA | `.phase1-dev/configuration-update-idea-20261101`，stopped-data-retained；新身份disabled；不要重放prepare/browser/write或删除marker |
| 本轮Linux | `yofc-management-network-20261101:/opt/yofc-configuration-update-20261101`，stopped-data-retained；两个新unit均停止、新身份disabled；不要重放prepare/start/write/restart |
| 协议夹具 | `.phase1-dev/configuration-update-contract-reviewed-20261101`停止、身份disabled；最初失败目录保留 |
| 之前夹具 | revision/read/outcome等stopped-data-retained；包夹具purged/restored/失败归档保持既有状态，不能删除state重试 |

图谱聊天MCP旧通道仍Transport closed，daemon/官方CLI可用，不能把CLI成功写成旧通道恢复。
本轮full刷新：core28671/109541、networking9347/43499、console4876/11199、agent735/3160（节点/边）。
核对1/4/14/4路径：全部metadata_match；console migration0008为unusable，已直接读三行SQL并核对
真实PG operation上限12。其余22路径无记录缺口；core15partial/1unusable、console3partial/4unusable。
ready不是完整解析，图谱同名误匹配不能代替运行证据。未索引父工作空间。

## 续接顺序与剩余问题

1. 先核对dirty worktree和本台账、最新知识库全文，保留迁移与全部未提交改动。核对SHA而非重跑旧验收。
   如需验证新写入，建立新名字的隔离夹具，不能删旧状态重放。
2. 完善配置元数据/组件适用性（尤其SelfConfiguringComponent、带约束字段），保持默认关闭和审核PID范围。
   核心直接ConfigAdmin写者不共享monitor，当前不承诺对它们原子CAS；不要扩大通用配置编辑器声明。
3. 按方案推进snapshot save/rollback、配置UNKNOWN持久业务结果核定；当前查询只取原回执，
   不能凭当前配置相同判断原动作成功，更不能换key重发。删除/reset/factory仍未实现。
4. 正式中心包、密钥轮换/恢复、最新协议agent发行包及Linux真实Kura包组合、旧revision5升级、
   新版26.04回归、完整网络provider/旧字段映射与注册/换中心产品闭环继续pending。
5. 推进nftables、系统控制、离线恢复与剩余阶段2–5。amd64、完整systemd文件系统沙箱、物理有线、
   磁盘满/逐点崩溃/硬断电等仍deferred。继续双语，按单元/浏览器→IDEA→OrbStack顺序验证。

没有提交、推送、强推或更改两套旧远端。暂停只是本轮停止点，不是整体目标完成。
