# 代码质量与逻辑架构审查及重构

日期：2026-11-04。用户在缺陷 Review 后明确要求补充代码质量/架构审查：若不合理，先重构并修复，再全部验证；验证另加入现有 Parallels VM。

## 审查判断

整体进程和信任边界合理：Rust 中心、独立 Rust agent、PostgreSQL/SQLite 分工、mTLS 与 UDS、Kura OSGi 桥接和 React 同源界面继续保留。没有证据要求重新迁移技术栈或合并仓库。

实现层需要重构，不能只逐行修补 F01–F14：

| 维度 | 现状及影响 | 实施方向 |
| --- | --- | --- |
| 持久化一致性 | 网络已有 fsync 和独占；新 firewall/recovery 重复实现较弱的 write+rename，元数据/权限与错误路径不同 | 复用安全文件和有界进程基础，显式记录文件/目录持久顺序及元数据 |
| 系统变更协调 | 网络、firewall、recovery 独立锁；配置写锁又在 transport；包维护只认识旧网络日志 | 统一变更准入和未决检查，在模块边界同时维护恢复、并发、生命周期规则 |
| 状态机 | 字符串散落、提前终态、确认决定分文件且不可恢复；人工恢复状态可覆盖 | 枚举化状态和持久决定；先意图、执行/回读、再终态；未知/漂移挡写并保留现场 |
| 故障隔离 | 新模块同步锁及 Command.output 在 async 路径阻塞；超时外包装不能取消同步阻塞 | 有界 blocking 工作与受限子进程；查询不等待长写锁；失联不取消已准入副作用 |
| 中心职责 | main/http/grpc 包含大量数据库业务和重复权限/转移逻辑 | 抽取可测试的任务生命周期与转移业务，CLI/HTTP 作为入口适配，保持事务与审计一起提交 |
| 安装与状态所有权 | 实例布局、初始化、activation、维护枚举存在不同目标；新模块未接入 guard | 单一实例布局/状态校验，首装和保留状态维护分开，全部状态模块纳入维护 |
| 测试有效性 | fake 暴露私有 journal 令牌、预创建目录、依赖外部 umask；成功计数未覆盖真实合同 | 通过公开接口验收、故障窗口和漂移测试、明确命令目标与生产文件权限，修复夹具的环境依赖 |
| 前端边界 | 按 feature 分模块、生成类型和双语框架可保留；部分 API 错误和包版本文档未同步 | 不为行数拆组件；补契约、双语、准入不明和版本适用性回归 |

## 本轮交付范围

对已实现功能完成上述重构和 F01–F14 修复，补齐同范围回归与安装入口/证据问题。未实施的 NAT、完整 Java 离线快照工具/核心恢复路径等继续单列 pending，不以本次重构声称完成整个阶段 2–5。

所有已有未提交改动先记录 SHA/HEAD，按现有内容增量编辑；不 reset/clean、不提交推送、不覆盖远端。旧状态/一次性 token/夹具不重放，真实写验证使用新名字。

## 验证顺序和完成证据

1. Rust、Java、Python 单元及契约；真实 PostgreSQL/mTLS 隔离契约；React 类型/生产构建和浏览器。
2. IDEA 新隔离完整 Kura、原 GWT 实际登录与配置/快照受限路径。
3. 已有 OrbStack：Linux 构建、安装/维护合同、受控隔离系统动作和重启恢复。
4. 已有 Parallels：当前 Ubuntu/架构实查、实际沙箱、相同候选受控回归；不借用旧只读结论。

本文件先记录审查与实施决策；实际改动、每一层结果和 deferred 在完成后追加，未执行不得记通过。

## 2026-11-05 实施检查点（系统层验证进行中）

已实现 F01–F14 的修复及共享 durability/process/system_change 基础：网络、配置写、防火墙、恢复共用准入锁；防火墙记录确认决定与回滚阶段；恢复补偿先停服务、核对完整备份/现场、保留漂移并恢复 uid/gid/mode；RECOVERY_REQUIRED 持续挡写。线程池固定 2 个异步 worker / 4 个 blocking worker。防火墙新增 `AGENT_FIREWALL_WRITE=0` 默认关闭；恢复仍通过固定 unit allow-list 显式启用。

中心抽出了 task_lifecycle，CLI/HTTP 共用转移审批事务。重连只作符合实际副作用的状态转换，租约/回执与连接变更统一 gateway → task 锁顺序；审计在加密前决定 reconciliation source。Java 约束补 exact string options、boxed type 和 cardinality 上限。实例维护枚举保留目录，首装可在没有 env 时检查；init/activate 保持选择的实例。Docker TLS 脚本及 build context 已修正，使用独立新 PKI 通过严格服务端/客户端用途与重复执行不覆盖检查。

系统回归又发现并修复：

- F15：确定时间戳的协议 crate 复用旧 Cargo target 时可能沿用旧生成类型。agent vendor 路径加入制品 SHA；在相同旧缓存上已重新编译正确的新 RPC，并通过 Linux Rust 套件。旧失败 stage/cache 均保留。
- F16：部分 OpenSSL 实现 `pkey -pubin` 会接受私钥；注册/检查签名方增加严格 Ed25519 公钥 PEM/SPKI 检查，拒绝私钥及证书。测试采用真正的另一把公钥，并修复 macOS/Linux/umask 依赖。
- 同范围质量补充：psql 维护命令把 URI 密码移至环境并加超时；新生成的凭据从 open 起就是目标权限，使用 fd chmod 并同步父目录；本机管理请求拒绝未支持字段，避免部分执行 NAT/恢复等尚未支持的参数。

用户再次确认默认账号密码 **admin/admin**、首次登录必须设置密码。该初始化属于既定需求，不列为缺陷，不删除/绕过种子账号。首次验证成功后只填新密码和确认密码，成功后进入控制台；普通改密仍校验当前密码，首次设置持行锁并撤销其他会话。已同步双语、OpenAPI 和类型。

本地证据根：`target/quality-refactor-20261104/`。目前通过 Java configuration 全模块 78、bridge 33、agent Rust 87（随后新增管理参数断言，最终计数待汇总）、console/protocol 全单元、PostgreSQL 5（含首次密码并发及转移审计回滚）、mTLS 3、浏览器 30、两仓严格 Clippy。Python 当前宿主 agent 42、console 7+2 root skipped；Linux agent 42、console 9 root 用例均可执行。真实 nftables 的隔离 netns 测试通过，不涉及宿主全局表或流量/NAT验收。

IDEA 新 fixture：`.phase1-dev/quality-review-20261105-c9fa7a09/`，全 282 项 inventory 校验；实际通过 IDEA UI 启动（PID 15161，现已停止），原 GWT 实际 admin 登录，HTTPS→Rust→UDS→真实 Kura 的 string options、越界拒绝、snapshot save/rollback 和脱敏共 10 项通过。新 fixture 已停止并保留，网关已禁用；旧 fixture 未重放。

OrbStack 当前复制目录 `/var/tmp/yofc-quality-review-20261105-1/`；仅复制旧编译缓存，不修改旧缓存。原 console26/agent20 和 center2 继续 active。尚需完成最终候选 deb、安装/维护及公开 UDS 系统事务，再进入既有 Parallels 的实际沙箱/权限验证。这里不把未执行项记通过。


## 2026-11-05 本轮最终核定

本轮从 47efca29/de6eed61 交接开始，审查至 2026-11-04 后续台账及当前 dirty 源码；原 Review 的 14 条发现已修复。整体进程/仓库/信任边界保留，重构集中在持久状态机、准入协调、有界阻塞工作与中心事务职责。没有重新迁移 Rust/React，也没有把原型扩大为完整阶段 2–5。

| 发现 | 当前处理与证据 |
| --- | --- |
| F01 重连误过期 | 抽取 task_lifecycle；同代次 QUEUED 保留、已交付写转 UNKNOWN；gateway→task 锁顺序；真实 PG 重连/结果合同 |
| F02 STRING options | 精确比较选项；boxed 类型、cardinality 与边界；Java 78 和 IDEA 真实 safe/4/同长非法值 |
| F03/F06/F07 恢复补偿、未决覆盖、元数据 | 停服务并确认 inactive/MainPID0；完整备份与 manifest；先核对全现场再写；漂移保留 RECOVERY_REQUIRED；原 uid/gid/mode、完成 receipt 决定与显式 rollback |
| F04 生命周期漏检 | 停服、网络/firewall/recovery/SQLite UNKNOWN 一起准入；未解决或仍持有受管状态时拒绝维护；保留备份/回执 |
| F05 防火墙持久顺序 | typed 状态、确认决定先持久、回滚意图先落盘、内核回读后终态；重启完成决定或补偿 |
| F08 nft 注入 | 真正 IP/CIDR 解析；未知外部表拒绝；语义 JSON 回读；无表必须由成功规则集查询证明 |
| F09 token 不可达 | 只从获授权的公开 prepare 响应交付；UDS/真实 nft 闭环不读取私有 journal |
| F10/F11/F14 首装/命名实例 | 空首装与保留数据分开；loaded+保留目录枚举；init 设置属组，activate 使用 self.unit；真实 PG guard 与 Parallels 实装 |
| F12 Docker TLS | shell/扩展路径、具名 EC 曲线、CA/leaf 约束、严格用途及不覆盖；根 context allow-list；当前源码镜像/新卷验证 |
| F13 核定审计 | 在 seal 前固定 source；真实 PG 验证网络审计和一次提交 |

系统回归新增并修复：F15 协议 crate 的固定时间戳导致温缓存误用旧 wire 类型（vendor 路径绑定完整 SHA）；F16 OpenSSL 部分实现接受私钥作为公钥（严格 Ed25519 PEM/SPKI）；F17 nftables 将 /32、/128 输出为标量地址导致合法 host rule 误报漂移（按内核 JSON 合同规范化）；F18 并发 fork 在 exec 前继承 flock 文件描述导致关闭后短暂误报 writer 冲突（最后一个协调者显式解锁，dup 模拟该窗口）。F18 修复后 Linux 全套 90 通过，未用串行测试掩盖竞态。F19 为 macOS 默认 OpenSSL 给 demo CA 使用 SHA1，默认安全级别客户端拒绝；TLS 生成所有签名显式 SHA256，新增两项真实默认策略双向 TLS/摘要与不覆盖回归。弱签名旧 PKI/卷保留，新建 v2 PKI 与独立卷完成复验。

其他同范围质量改进：共享 durable/process/system_change；固定 worker 上限；CLI/HTTP 共用审批事务；psql 密码离开 argv；凭据创建即私有并 fsync；拒绝未实现的本机管理字段。默认 firewall 写关闭；配置仍按审核 PID 显式开放。console/agent 身份独立于 Kura，普通其他中心登记不能改变 agent 本地归属。

### 首次登录的产品要求

用户明确要求 **admin/admin、首次必须设置新密码**，并选择更直接的首次设置流程。该默认账号属于需求，不能当成缺陷删除。认证成功后只填写新密码与确认；服务端闸门仍生效，刷新不能绕过，成功直接进入控制台。普通改密仍验当前密码；并发首次会话持数据库行锁，撤销其他会话。OpenAPI、生成类型、中英文与真实安装流程保持一致。

### 当前验证矩阵

证据根 `target/quality-refactor-20261104/`；本节计数替代上方进行中检查点的临时计数。

| 层次 | 当前结果与范围 |
| --- | --- |
| Java | API 构建安装；configuration 全模块 78，bridge 33；真实完整运行集 282 项 hash 校验 |
| Rust | agent 宿主/Linux 各 90 passed + 1 ignored；真实 nft 用例另以 root/private netns 1 passed；console 16、protocol 8 |
| 数据库/认证 | 隔离 PostgreSQL 5，真实 mTLS 3；含首设并发、转移审批审计回滚、重连/UNKNOWN/审计 |
| Python/静态 | agent host/Linux root 各 42；console host 7+2 root skipped、Linux root 9；两仓 Clippy -D warnings、fmt、React typecheck/build |
| 浏览器 | 默认套件 30；OrbStack 当前候选、Parallels 实装包、Docker 新镜像各 1 条真实初始设置全流程（双语/闸门/旧密码失效） |
| IDEA | 经 IDEA UI 启动独立完整 Kura，原 GWT admin 登录；10 项实际配置/快照/脱敏检查，11 个中心任务；夹具停止并保留 |
| OrbStack | 真实公开 UDS+private netns+低 UID reader 16 项；命名中心/未加载实例三状态维护与私钥 12 项；旧三服务保持 active/原包 26/20 |
| Parallels | Ubuntu 26.04 arm64；当前 agent ELF 实际严格沙箱 17 项；中心干净首装、模板直接启动与实际沙箱、remove/reinstall/purge 保留 8 项 |
| Docker | 当前源码镜像构建、新独立 PKI/卷、真实首设双语、普通重启后凭据保留；具体回执见 docker-validation.json |

真实恢复测试的低 UID reader 读取每个快照，并在故障场景制造未知文件；验证属主权限、停止后补偿和漂移保留。其内容仍是 opaque fixture，回执明确 `verification=artifact-and-unit-only, kuraRecoveryVerified=false`。Parallels 的 probe user 为现有 parallels UID1000；没有伪称安装了完整 Linux Kura。

OrbStack 的系统沙箱受既有全局覆盖影响，本轮不修改该覆盖；沙箱核定来自 Parallels 的真实有效设置。所有内核防火墙实验均位于专用 network namespace，未修改 VM 主网络或宿主规则集。Parallels 原 agent 身份、unit、Netplan/NM/networkd 来源和默认路由比较不变。新装 PostgreSQL 18 留存数据库，停止并禁用自动启动；新中心包最终 purge，凭据/state 保留；VM 最终正常 stopped（关机曾等待 unattended-upgrades 完成）。

### 制品与源码对应

| 制品 | SHA256 |
| --- | --- |
| console deb `0.1.0~review20261105-1_arm64` | `dc78f03c8688b3c6f748488acc32a43964cf525cc3507cf9214029da2540f763` |
| console ELF | `0f5c44b024f9a958f0eed389d5a726e4e2734370a20fba25cc0818b9c4d36e87` |
| agent deb `0.1.0~review20261105-2_arm64` | `c0a680fc683ae32a92aac1159a792a62a2a88b2ad2b10a549f612232f214b158` |
| agent ELF | `34465966d761fbd06dcdbc7645f1584995657a6a51c879e9d8daa92684c9fc9c` |
| protocol crate | `07a78a3563a3bdc57e6e41f59419b32463145dbdac10ba3a5977c80714cbda98` |
| Docker image | `sha256:ac6016f3142b97a52b3400acecee6b4cc37547e061cf6542366bfd060ba92b95` |

源码 stage 为 console-v2、agent-v5。与实际当前生产输入比对 console 80、agent 29 条全部一致；当前 live 测试和文档另存工作树，不当作 ELF 输入。先前失败 stage/包 v1 保留，不能当最新候选。制品在 `target/quality-refactor-20261104/native-packages/`；原 VM 三服务未用候选替换。

### 保留状态和后续边界

六仓 HEAD 与本轮开始一致，所有原 dirty 文件仍在，修改前副本位于私有 baseline-files；未提交、未推送、未 reset/clean。最终 worktree-after.json 记录本轮增量。旧 fixture、一次性凭据和远端均保留。codebase-memory 使用真实子仓库官方 CLI 重索引；代码路径覆盖 metadata_match，SQL/配置解析缺口保留文本核对，不能把 ready 当全覆盖。

**待实现**：完整 metatype/组件适用面、NAT/forward 与 Kura 防火墙 API/适配/原 UI/远程协议和早启加载；完整 Java XML/加密/factory/核心恢复工具与业务验收、制品远程管理；重启/主机 reboot 远程任务闭环；key 多次轮换、整库/制品一致备份与恢复；旧系统迁移和正式发行。

**deferred**：真实 NM/cloud-init 交接、物理有线和硬件、amd64、磁盘满/硬断电等。本轮 kernel/沙箱、正常 VM 关机不能替代这些条件。React 生产大 chunk 警告仍在；未为压行数拆分无实质收益组件。两个旧证据缺文件（console-deb-20261102-binding-generation/agent-tests.log、transfer-ui-browser.json）仍标 evidence gap，不制造替代历史证据。

详细下一会话入口：`continuation-prompt-20261105.md`。本轮按用户要求收尾后暂停，整体改造目标未完成。


### 2026-11-06 续接台账覆盖：受保护普通启动

目标已恢复并保持active，上述历史暂停只对应原质量轮次。CM逆向及文件集后的受保护普通snapshot/CM加载、独立旧格式授权已在新IDEA/Orb核验，见 [最新核定及原业务pending表](guarded-normal-startup-20261106.md)。此项不等于无闸门普通启动、桥接/全业务健康或完成释放；原完整组件/部分与空原CM、NAT/远程/早启、restart/reboot、一致恢复及正式发行仍pending，外部deferred分列。原F01–F19和旧验证矩阵按其版本保留，不重放。最新知识库6537c595及continuation入口已更新；新夹具全部停止、原Orb原包active，两VMstopped。

### 2026-11-06 最新覆盖：实时所选业务健康

整体继续 active。[实时健康核定及剩余业务](recovery-health-20261106.md) 替代上一节“桥接健康未接线”的部分：可选 API1.6/默认关闭桥接/独立健康授权和真实 DS/CM/密码检查已在新 IDEA 与 Orb 全流程通过。宿主/Linux各145，Java reactor428、bridge46，native显式1；源26条逐值一致，显式自配置Console追加1条，合计27/193。typed失败观察保留/RR挡写和成功后的agent重开无Kura重放已核验。仍 AWAITING_HEALTH_COMPLETION、普通写挡住、normalBootVerified/fullRecoveryVerified=false。明确完成/释放/无闸门启动及全组件/外部PID/部分空CM等原业务仍pending；Parallels缺Java/Kura及外部边界单列deferred。全部新夹具停止保留，原Orb原包26/20四服务active，两VMstopped。

### 2026-11-06 最新覆盖：明确批准范围完成与释放

整体active。[完成/双闸门释放与独立普通启动核定](recovery-completion-20261106.md) 替代上一节相应未实现项：默认关闭API1.7 completion授权及一次写、持久Java回执/agent核定已接通。终态COMPLETED_APPROVED_CHECKS仅是管理员所选检查范围，广义kuraRecoveryVerified仍false。新IDEA/Orb原GWT实际9改11及新snapshot、无恢复请求普通启动/DS密码/普通snapshot写、完成后agent重开通过；Java429+bridge48、host/Linux149、packaging54/native1及Parallels新只读同步合同1通过。正式固定普通unit/profile交接及生命周期、全组件/providers/部分空CM/外部PID等原业务继续pending；缺Java/Kura及外部系统硬件条件deferred分列。新JVM/Orb5unit/探针已停，原Orb原包active，两VMstopped；最新知识库dfcf15f7。


### 2026-11-06 最新覆盖：固定普通配置后台交接

整体active。[固定普通配置交接核定及原业务pending](ordinary-profile-handoff-20261106.md) 替代上一节“后台固定ordinary unit交接未实现”的部分：独立默认关闭grant、本地prepare/execute、固定第四unit/runtime/有效systemd/drop-in和只读真实JVM/provider观察已接通。只启动一次、持久回执后ORDINARY_ACTIVE_APPROVED_CHECKS仅所选范围，广义恢复仍false，UNKNOWN/RR规则保留。host/Linux154、bridge52、packaging55/native1及193输入核对通过；新IDEA v3/Orb实际GWT9改11、后台普通交接后DS/密码/快照写及agent重启不重放通过。失败/中断native现场停止保留不重发。所有新JVM/driver/Orb5unit已停，原Orb原包26/20四服务active，两VMstopped。Parallels缺Java/Kura完整业务deferred，不重放旧探针；Core/API/Docker未改。安装预配置/开机接管/中心任务与restart/reboot、全组件/外部PID/部分空CM、防火墙NAT远程早启、一致恢复与正式发行继续pending，外部deferred分列。最新知识库5951e763；详见对应evidence和日志target/ordinary-profile-handoff-20261106。


## 2026-11-06 普通包启动与冷启动就绪增量核定 372d897d

完整目标active。详见 [本轮核定](ordinary-package-launcher-20261106.md) 与 [机器证据](../evidence/ordinary-package-launcher-20261106.json)。Core独立可选argfile、agent本地安装封装、schema2 provider就绪已实际实施；单元/IDEA/Orb/Parallels精确范围及最终状态见核定。boot接管/冲突入口退役、中心restart/reboot及原全量剩余业务仍pending；Parallels完整Java/Kura与硬件/外部边界deferred。既有F01–F19和一次性验收不重放，旧失败现场停止保留。


## 2026-11-06 显式启动策略与Core维护互锁增量核定 42b31189

完整目标active。见 [最新核定](ordinary-boot-adoption-20261106.md) 与 [机器证据](../evidence/ordinary-boot-adoption-20261106.json)。本地root原生unit启动策略已实施，独立confirmed/取消保留/只读核定/UNKNOWN挡写及Core整阶段锁持有已接通，尚无中心UDS/API。host/Linux各166、包维护各56、Core host15/Linux12及新IDEA/Orb真实普通Kura/GWT与实际CLI通过；Parallels严格沙箱和OS reboot原语仅非Kura stand-in，完整Java/Kura仍deferred。新JVM/units停且保留、两VMstopped，原Orb26/20四服务active。完整Core安装/bootstrap/升级、非原生入口退役/策略交接、中心restart/reboot、真实Kura重启和原全量剩余业务继续pending，外部deferred及历史gap分开。最新知识库 Experience/2026-11-06-Kura显式启动策略接管与Core维护互锁续接-42b31189.md；旧一次性验收不重放。


## 2026-11-06 中心启动策略任务与真实沙箱增量核定 770604cf

完整目标active。见 [最新核定](center-ordinary-boot-20261106.md) 与 [机器证据](../evidence/center-ordinary-boot-20261106.json)。固定本地授权的可选管理UDS、ops15–19、admin/CSRF/归属/capability、加密/幂等/UNKNOWN闸门及双语审核界面已实施；cancel后续计划保留独立confirmed。实际Parallels发现并修复初始UDS首帧allowlist及长写分类，以centerboot2、新IDEA/Orb/Parallels夹具重验，旧失败全部保留不重试。host/Linux各170（6/7ignored）、console17/protocol9、PG6、HTTP8、mock4/live1、严格Clippy/fmt，258冻结输入及agent37 manifest一致。IDEA完整隔离Kura/原GWT Count9、Orb真实普通包Kura PID514361及中心一次execute/agent重启同PID/Invocation/GWT9通过。Parallelsstrict沙箱/实际main UDS/default-off/同UID角色拒绝/EROFS通过，但目标明确非Java/Kura stand-in；完整Java/Kura仍external deferred。bootVerified/kuraRecoveryVerified保持false。

新增明确内部pending：boot领域决定已持久、execution receipt未持久时，绑定原command的业务核定resolver尚缺；保持UNKNOWN，policy reconcile19不自行解除原中心task。完整ordinary中心交接、restart/reboot、安装bootstrap/升级/非原生退役/策略交接、完整Kura reboot及原全量业务pending保留；外部系统硬件deferred和历史两gap单列。全部新JVM/units及宿主中心/PG/Vite/keepawake停止，测试目标显式停禁且历史confirmed保留；原Orb26/20四服务active原PID，两VMstopped。18仓HEAD/branch/旧dirty保留，无reset/clean/提交推送；知识库最新770604cf。


## 2026-11-06 原命令启动策略核定与回执故障窗口 cdd8ab4e

完整目标active。见 [最新核定](boot-outcome-reconciliation-20261106.md) / [机器证据](../evidence/boot-outcome-reconciliation-20261106.json) 及agent ordinary-boot-task-outcome-v1合同。原生commandDecisions/issuedCommand、精确wire hash及私有决定已接线，查询认证原UNKNOWN计划且只读current/history/receipt，不重放/不查live unit；缺失/legacy/孤儿/漂移保UNKNOWN，RR仍需明确policy reconcile。中心从加密input重构原请求并原子一次audit。双语详情与BootPanel跨租约后持续有界GET原任务已补，写闸门保留。此项替代770604cf缺resolver的部分，不替代完整方案。

宿主173/8ignored、Linux并行173/9ignored、console18/protocol9、PG7、包装两处56、Coreguard两处13、mock8、Orb live1/Parallels live1及严格Clippy/fmt/typecheck/build通过；262冻结输入/agent37 manifest一致。新IDEA76a3007b完整Kura/原GWT Count9及DS密码snapshot9/observer1；Orb1eafee4e真实Kura PID534120，测试transport exit92后native已commit而execution无receipt，生产ELF原query核定02402c81、单次audit/无替代写/GWT9/重启后原生字节；PRL ba3e77ef实际strict sandbox/default-off保留到公开UDS明确ON/EROFS及真实mTLS原任务1896c9fa闭环，但target3199是sleep stand-in，完整Java/Kura仍external deferred。生产ELF无test fault标记，review制品未升级原包。

全部新运行进程停、target/retired disabled，故障unit保留预期failed/exit92 MainPID0；两VMstopped，原Orb26/20四服务active原PID，所有旧dirty/fixture/state保留。完整ordinary中心交接/restart-reboot、Core完整安装bootstrap升级/策略交接/非原生退役、完整Kura reboot业务和原全量pending继续；任意storage error/precommit orphan保持证据与挡写，external deferred及历史gap分列。知识库最新cdd8ab4e；不reset/clean/提交推送，不重放旧入口。


> **最新核定（2026-11-06，11dcfedf）：普通配置中心交接与只读意图续接。完整目标保持 active。** 先读 `profile-handoff-center-20261106.md`、对应evidence、`../ordinary-profile-center-v1.md`，以及知识库 `Experience/2026-11-06-Kura普通配置中心交接与只读意图续接-11dcfedf.md`、Tasks/Session Summaries最新核定。默认关闭的20–22 status/prepare/execute已接通；只选当前本机批准completion和固定ordinary grant，不接受命令/unit/path。自身准入排除需完整wire SHA/operation/key且RUNNING；其他UNKNOWN/RUNNING挡写。admin/CSRF/归属/新鲜capability与dispatch重查、加密token/非admin脱敏、中英审阅明确勾选/刷新原任务均通过；丢失提交响应可按同actor/tenant/gateway/key只读找回ID，404仍挡写、不创建替代计划。
> 宿主/Linux175、console19/protocol10、PG8（最终handoff/intent1复验）、包装两处56、mock6、实际中心→mTLS→完整Kura1通过。新IDEA02f55966原GWT admin/admin中文Count9及真实DS/密码/snapshot/observer1已停。新Orb2d741695的普通unit仅由中心execute aeee8483启动，PID556444；agent重开不重启Kura，native/receipt字节不变，原GWT9；实际本次没有UNKNOWN，不套用前轮进程退出验收。PRL6fc4f68c当前main实际strict/CapEff0/NoNewPrivs1/EROFS和真实Hello不含默认关闭capability、三操作准入前拒绝通过，明确无Java/Kura且未建目标unit。所有新进程/units/中心PG/Vite/keepawake停，Ubuntu/Windows正常stopped；原Orb26/20四服务原PID继续active，全部现场保留、18 HEAD/旧dirty不动，无提交推送。
> 本条仅覆盖旧台账“ordinary-profile中心三操作未接线”；cdd8ab4e原boot命令resolver保持。普通交接native原命令核定/未决计划运维、restart/reboot、Core完整安装升级/交接、完整reboot业务、全组件/metatype/provider/部分空CM/外部PID、NM/firewall/NAT/远程/早启、一致备份/key/迁移/发行仍pending；Parallels完整Java/Kura、真实NM/cloud-init/物理/硬件、amd64、磁盘满硬断电external deferred单列。scope及normalBootVerified/kuraRecoveryVerified=false保持。日志 `target/profile-handoff-center-20261106/`。



> **最新核定（2026-11-06，9104c438）：普通交接原命令核定与真实回执丢失闭环；完整目标继续 active。** 先读 `handoff-outcome-reconciliation-20261106.md`、对应 evidence、agent `docs/ordinary-handoff-task-outcome-v1.md`，以及知识库 `Experience/2026-11-06-Kura普通交接原命令核定与真实回执丢失验证-9104c438.md` 与 Tasks/Session Summaries 最新核定。ops21/22 的私有决定及完整原 wire SHA 绑定，execute 的 issued binding 在启停前落盘；只读 query 不调用 systemctl、live observe、repair 或 replay，后续恢复覆盖前归档原 journal。缺失/漂移/legacy/未提交继续 UNKNOWN，RR 持续挡写，孤儿命令证据挡 agent/Core 安装维护。中心验证原加密输入重构的 wire，审计一次 `task.reconcile.ordinary_handoff`，双语详情限定选定检查。并发查询在另一查询已核定时重读原回执，消除过期 UNKNOWN 返回。
> 最终宿主/Linux agent177、console20/protocol10、PG9、包装两处57、Core guard两处14；浏览器既有12回归及新3通过。IDEA新8e1d0e40种植27701→ordinary28198（284 inventory）、observer1、真实DS/密码/snapshot和原GWT admin/admin中文Count9通过并已停。Orb新95413f3a：prepare 733a8f5a-3f32-4b3c-a87c-74287ebd53d4 仅一次，execute 804337e9-5b0a-4c5a-9de5-de4acfcce3f5 仅一次；native提交后测试进程exit92、中心实际UNKNOWN，正式v3 ELF只读核定成功，Kura PID616673不变，native/receipt/command字节不变、SQLite resolution和中心审计各一次；原GWT9。Node只信任本轮CA后从原prepare续接，未重发准备。PRL新e046063c当前v3 deb仅解包，实际strict/CapEff0/NoNewPrivs1/EROFS及默认关闭Hello/三操作准入前拒绝通过；无Java/Kura目标或伪造completion。
> 宿主新中心/PG/keepawake、IDEA/observer及Orb新运行单元全部停止，故障单元failed/92且PID0保留，原瞬态agent已自动卸载；Ubuntu与Windows均stopped。原Orb26/20四服务原PID保持active。18 HEAD/分支/旧dirty路径保留，无提交推送。当前 review deb `0.1.0~review20261106.handoffproof3`，日志 `target/handoff-outcome-reconciliation-20261106/`。历史mock脚本的旧截图路径曾被本轮回归写入，审计已单列，不作为旧一次性业务场景重验。
> 仅闭环普通交接的原命令核定；未决计划运维、restart/reboot任务、Core完整安装升级/交接、实际重启后完整Kura业务、全组件/metatype/provider/部分空CM/外部PID、NM/firewall/NAT/远程/早启、一致备份/key/迁移/正式发行仍pending；PRL完整Java/Kura、真实NM/cloud-init/物理/硬件、amd64、磁盘满/硬断电等外部deferred单列。normalBootVerified/kuraRecoveryVerified仍false；保留NM/Kura无agent独立运行等既定方向。



> **最新进度（2026-11-06，a09e95d7）：restart/reboot 已实施并通过宿主首层验证，完整目标继续 active。** 先读 `restart-reboot-lifecycle-20261106.md`、对应 evidence、agent `docs/system-control-v1.md` 与知识库 `Experience/2026-11-06-Kura重启任务实施与宿主浏览器验证-a09e95d7.md` / Tasks / Session Summaries。ops23–28、固定本机grant/唯一reboot timer、原命令纯查询、连接epoch队列过期/UNKNOWN防重放、包维护guard和中英UI已接通。宿主整套agent186/9ignored（竞态补丁前）+最终native10，console24/protocol11、新PG2、包装65/Coreguard17、mock浏览器10及严格静态门通过；系统调用fake/包装sealer stub，**尚无本轮真实IDEA/Orb restart/Parallels reboot或Linux制品**，内部pending不能算通过或external deferred。新PG/Vite已停保留；未启新中心/agent/JVM/VM或发布真实任务；原Orb四服务原PIDactive，两Parallels stopped。18HEAD/旧dirty保留，无提交推送。下一步先真实HTTP负例，再按新IDEA完整Kura/GWT→新Orb固定Kura restart→现有PRL严格sandbox实际reboot，原Orb不reboot、旧入口不重放。完整业务/其他发行目标仍pending，normalBootVerified/kuraRecoveryVerified=false。



## 2026-11-07 固定系统任务真实验证核定 4230b9b8

**最新核定（2026-11-07，4230b9b8）：固定 Kura restart 与主机 reboot 已完成真实系统任务和选定业务验证，完整目标继续 active。** 先读 `restart-reboot-system-20261107.md` / 对应 evidence、agent `docs/system-control-v1.md`、知识库 `Experience/2026-11-07-Kura固定服务与主机重启真实业务验证-4230b9b8.md` 与 Tasks / Session Summaries 最新核定。a09e95d7 的 HTTP/IDEA/Orb/Parallels 内部 pending 本轮已落实：systemd255 空 ExecStop 兼容及 COMPLETED status 原 outcome 刷新保持已修，host native11、Linux v3 188/10ignored、真实HTTP/PG1、Linux包装65/Coreguard17、4个新增实际浏览器场景和严格检查通过；原轮宿主全量/mock数字不冒称重跑。新IDEA863073cc 284 inventory、真实DS/密码/快照/observer及原GWT admin/admin中文Count9通过并停。Orb新3c13a447仅prepare a009016e/execute 6cf7ebc4一次，Kura658127→658378，选定业务通过；v3 agent重开/只读中英与刷新/原GWT续验无新增写，native字节不变。原浏览器在实际restart成功后语言选择器失败，续验仅GET原任务，严禁重发原动作。PRL新8f68773c私有自包含Java184文件+284bundle共472 pins，默认关闭能力/GWT独立、实际strict/CapEff0/NNP1/EROFS30通过；只授权reboot，restart拒绝。prepare73138bcc/execute1cb4f6ab各一次，真实boot143ab4e5→6307ae2e，Kura1533/agent1532自动启动，原任务UNKNOWN→SUCCEEDED、SQLite核定与中心reconcile审计各1，真实schema2选定DS/密码/snapshot和中英/原GWT9通过。不是sleep stand-in、不是Core deb安装升级、不是全部业务恢复；normalBootVerified/kuraRecoveryVerified仍false。
所有本轮宿主中心/PG/keepawake、IDEA/observer、Orb新unit已停；PRL新agent/普通Kura停并disabled，seed停，两VM stopped。原Orb原26/20四服务原PID244516/244911/286620/268保持active。最终review candidate systemcontrol3 ELF ac61f5fc / deb9893692e、44record manifest5257a174已读回；全部旧失败/请求/state/key/backup保留，18HEAD/旧dirty路径不动，无提交推送。日志target/restart-reboot-system-20261106。旧“PRL无Java”对本轮验证的环境障碍已用私有环境解决，不追认历史其他scope。内部pending继续：未决交接运维/Core完整安装升级与策略交接/全组件适用面与离线恢复业务/NM-firewall-NAT远程早启/一致备份-key-迁移-发行；外部真实NM-cloudinit-硬件/amd64/磁盘满硬断电和旧两个evidence gap单列。下一批按未交付台账继续，勿重放已闭环入口。


## 2026-11-07 普通交接未执行计划取消 f2ea1b07

**最新增量（2026-11-07，f2ea1b07）：普通交接未执行计划取消已实现并完成宿主合同/浏览器，完整目标active。** 先读 `pending-plan-operations-20261107.md` /对应evidence、agent `docs/ordinary-handoff-cancellation-v1.md`，及知识库 `Experience/2026-11-07-Kura普通交接未执行计划取消与宿主合同验证-f2ea1b07.md` /Tasks/Session Summaries。op29、双cap与默认关闭、admin/CSRF/归属/首登及当前epoch/30秒TTL/dispatch复核；只有未issued Prepared可显式取消，过期不自动停健康parent。持久Cancelling→Cancelled、原parent/历史保留、prepare/cancel完整wire只读核定、丢响应原intent、agent/Core纯guard均已接线。取消/恢复证据查询不调用系统或业务动作，issued/RR不解除，缺失证据持续挡写，normalBootVerified/kuraRecoveryVerified=false。最终host192、console24/protocol11、显式HTTP/PG3、包装68pass2skip+native合同5、Core17、受影响浏览器10及最终新4、严格静态门通过。原生单元是mock，**本批新IDEA→Orb→现有Parallels系统验证仍内部pending**，没有本批Linux候选；旧systemcontrol3仅ops1–28，不能当本轮版本。下一需新crate/ELF与全新获批parent/ordinary计划验证实际取消/JVM/业务不变，不能复制旧事务或重播旧seed/恢复入口。
本轮新PG16650/Vite19701已停，夹具/schema/key/185条旧源码副本/截图保留；未启新中心/agent/JVM/VM或发真实系统任务。原Orb四服务原PID244516/244911/286620/268原26/20包active不动，两VM stopped，18HEAD/旧dirty路径保留，无提交推送。日志target/pending-plan-operations-20261107。4230b9b8的restart/reboot闭环继续有效，不重做；其他未决运维/Core发行/全组件/离线恢复/NM-firewall-NAT/一致备份迁移发行仍pending，external deferred和旧evidence gap分开。


## 2026-11-07 未执行交接取消实际系统验证 d91353d6

> **最新核定（2026-11-07，d91353d6）：未执行普通交接取消的实际系统验证已闭环，完整目标继续 active。** 先读 `handoff-cancel-system-20261107.md` /对应evidence、agent cancellation合同，以及知识库 `Experience/2026-11-07-Kura未执行交接取消真实系统验证-d91353d6.md` /Tasks/Session Summaries。覆盖 f2ea1b07 本批新IDEA→Orb→现有Parallels内部pending。新IDEA8924490c完整284 inventory、真实CM/DS/密码/快照/Java释放/GWT9，PID53819自然过期recover仍Prepared，两本地UDS取消无unit调用/无快照业务变化及历史归档通过并已停。Orb171c3eed、PRL45ec6e0e均从全新真实恢复链建立approved parent（PID702406/4568），两份不同计划各一次中心cancel、无execute；过期后仅agent重开、原journal与四unit/PID/Invocation/快照/DS事件不变，双语/历史原task/GWT9及实际guard各5通过。PRL私有Java184文件、483 pins，实际CapEff0/NNP1/EROFS及重开后再次验证。中心15任务全SUCCEEDED：20=7、21=4、29=4；8写结果及取消输入加密，默认admin首登flag仍true。原ID续接修正CA/等待/storage测试夹具，未补发任何原prepare/cancel。候选handoffcancel1 ELF bd85f7b9/deb9bf88fff，45 source manifest ba24cd7c读回一致；host192+最终native1/Linux192、严格检查通过，旧console/protocol/mock数沿用不冒称重跑。
> 本轮新JVM/中心/PG/keepawake和两处新五unit均停、两新gateway disabled、Ubuntu/Windows正常stopped，原Orb26/20四服务原PID244516/244911/286620/268继续active，全部旧/新现场保留。review deb未安装原系统；不是Core完整安装/升级或全部业务恢复，normalBootVerified/kuraRecoveryVerified仍false，未追加op29真实UNKNOWN故障注入。其他未决运维/Core完整安装升级/策略退役/全组件与Java离线恢复完整业务/NM-firewall-NAT远程早启/一致备份key迁移发行继续内部pending；真实NM-cloudinit/物理硬件/amd64/磁盘满硬断电external deferred及两旧gap分列。本批PRL完整所选Java/Kura不再缺环境。日志target/handoff-cancel-system-20261107；18HEAD/旧dirty保留，无提交推送。下一按未交付台账继续，禁止重放本轮与既有一次性入口。



## 2026-11-07 部分失败CM来源独立补偿 b9eb0ad3

完整目标继续active。见 [本轮核定及剩余业务](partial-cm-source-compensation-20261107.md) / [机读证据](../evidence/partial-cm-source-compensation-20261107.json) 和 agent cm-compensation 合同。新增 inverse allowPartialSource 默认关闭，精确失败 checkpoint/停止来源/不可变授权及独立 source_partial 接线；原 cmApplication 不伪造、原失败请求不重发。宿主198+新增真实IDEA1、Linux5和严格检查通过；全新IDEA→Orb→现有Parallels真实CM先写2后IOException、混合2/9认证、独立inverse原CM/实际DS秘密解密/原GWT9及普通写挡住通过。新ELF539244e2、VM真实UDS/systemd、PRL末次strict/Cap0/NNP1/EROFS30，Orb旧override限制保留。三处仍AWAITING_COMPENSATION_COMPLETION、恢复总标记false，未发normal/health/completion。新JVM/两处新四unit全停、Ubuntu正常stopped/Windows仍stopped、原Orb四原PID原26/20包active，18HEAD/旧dirty及全部失败现场保留，无提交推送。只关闭部分失败来源inverse准入；空/部分CM后续策略、全生产组件/外部PID、其他原软件台账继续pending；真实NM/cloud-init/硬件/amd64/磁盘满硬断电external deferred与两历史gap分开。禁止重放本轮一次性入口。


## 2026-11-07 部分失败来源所选业务完成 ff72eb79

完整目标active。见 [本轮核定](partial-source-completion-20261107.md) / [机读证据](../evidence/partial-source-completion-20261107.json) 与最新知识记录。产品代码/ELF未改，新增2合同及真实IDEA helper；host/Linux各7、严格检查及IDEA真实1通过。新IDEA→Orb→现有Parallels部分CM失败→独立inverse→完整snapshot-set→guarded normal9→实际UDS whole-CM/秘密业务健康→独立明确批准→一次GWT10/真实snapshot→无恢复请求ordinary11全通过。原cmApplication始终null/source_partial保留；COMPLETED_APPROVED_CHECKS仅管理员所选范围，广义恢复标记false。新JVM/测试服务和VM各6unit全停、两VMstopped、原Orb四原PID原26/20包active，18HEAD/旧dirty/全部现场保留，无提交推送。只关闭非空且原baseline一致的部分失败来源所选链；空/部分原CM其他策略、全生产组件/外部PID、原运维/Core发行/网络/一致备份等继续内部pending；外部真实NM/cloud-init/硬件/amd64/磁盘满硬断电deferred和两历史gap分列，不重放旧入口。
