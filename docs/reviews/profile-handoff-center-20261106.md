# 普通配置中心交接与只读意图续接（2026-11-06，11dcfedf）

完整管理体系目标保持 **active**。本轮接通当前已批准 completion → 固定 ordinary profile 的中心 status/prepare/execute 任务，补齐浏览器丢失提交响应后只读找回原任务的路径。没有交付 restart/reboot 或完整 Kura 业务恢复。接续 cdd8ab4e，不重放此前一次性入口；原 boot 命令核定合同保持。

合同：[ordinary-profile-center-v1.md](../ordinary-profile-center-v1.md)；[机器证据](../evidence/profile-handoff-center-20261106.json)。日志、冻结输入、候选与全部失败记录位于 `target/profile-handoff-center-20261106/`。

## 已实现的业务链

- 新增 v1 operation 20–22、独立 `OrdinaryHandoffAction` 和 migration 0017；旧编号、wire 字段与 migration 不改。只有 status20 可读重试；21/22 使用现有加密准入/回执/幂等链，UNKNOWN 不重新 dispatch。
- agent 必须有本机固定 ordinary grant，并明确 `AGENT_RECOVERY_ORDINARY_REMOTE=1` 才声明 `recovery.ordinary.v1`；默认0，已有远程通道开关仍独立控制连接。中心不能传命令、路径、unit、父决定或检查内容。prepare 只选当前本机已经提交的 completion，再由既有 native 事务核验其不可变证据；execute 仍需新的 handoff UUID/确认令牌。
- 长交接共用在线写互斥和原生系统变更锁。只排除同一完整 Protobuf SHA、operation、幂等键且仍 RUNNING/result NULL 的自身准入；其他 RUNNING/UNKNOWN 写仍阻挡。没有把整个 executions 闸门关掉，本机 UDS 也不能提供这种排除身份。
- prepare 返回目标/profile、授权/源 journal/快照摘要、组件/检查名单和期限。确认后由 native 事务持久 issued、停止旧恢复 unit、只启动固定 ordinary 一次，复用真实 schema2 provider/业务验证。状态为 `ORDINARY_ACTIVE_APPROVED_CHECKS`，scope仅 `administrator-selected-checks`，`normalBootVerified/kuraRecoveryVerified=false`。
- 中心三操作均要求 admin/CSRF/当前归属及新鲜 capability；dispatch 再查。独立检查成功回执范围、schema2 就绪、非零 PID 与原 execute 的 handoff ID。prepare token 继续加密保存，viewer/operator 响应脱敏。
- 中英文面板审阅后必须明确勾选执行；刷新只从认证 API 恢复原 prepare，令牌不进浏览器存储。原任务跨租约时有界 GET 续读；HTTP 丢失响应先保留未知意图，再用 `GET /gateways/{id}/task-intents/{key}` 按同一 actor/tenant/gateway/key 找回原 ID。404 继续挡写，不能当作未准入。另一管理员不能认领该意图，不创建替代交接。

普通 Kura 仍不依赖 agent 启动，也不携带 recovery request。NM 保留、Java 后端生成私钥/CSR、默认 admin/admin 与首次设置后台闸门、账号/agent 身份独立、配置/PID 审核与 firewall 默认关闭等核定保持。

## 当前验证及实际范围

| 层次 | 最终证据 |
| --- | --- |
| agent | 宿主175 passed/8 ignored；Linux并行175 passed/9 ignored；两处严格 Clippy、fmt。新增真实加密准入→固定交接、原回执重复读取、不匹配 wire/其他未决写拒绝；其余原生失败/中断/不重放合同随当前源码回归 |
| 中心/协议 | console19、protocol10；实际 PostgreSQL8，其中 handoff/intent1按最终修订复验；17 migration 实际应用。迟到原回执、错误 handoff/过度业务声明拒绝、一次回执审计、加密存储和四维意图隔离通过 |
| HTTP | HTTPS准入/固定字段/CSRF/角色/capability16；后续viewer/operator token脱敏、另一admin意图404、实际Parallels能力缺省与三操作准入拒绝7；默认admin首次设置对新路由的真实拒绝通过 |
| 包装 | 宿主 Homebrew Python56、Linux root56；候选源码/制品验证，不安装或升级原包 |
| 浏览器 | 最终 mock6，包括明确确认、双语、刷新只读、UNKNOWN、丢失响应→404继续挡写→找回同一原任务；实际 HTTPS→中心→mTLS→生产agent→真实Kura1。typecheck/build通过，既有大chunk/AntD提示保留 |
| IDEA | 新02f55966完整284 inventory；IDEA UI启动seed92641、普通93283。真实DS/密码/普通snapshot9、provider缺失后就绪/显式observer1；原GWT admin/admin中文Count9；observer退出后同JVM。已正常停止 |

Java/Core/API/GWT生产源本轮未改；历史 Java429/bridge53、Core guard和Docker只按原版本保留，不计本轮新通过数。

## Orb真实中心交接

新夹具 `/opt/yofc-profile-handoff-center-20261106-2d741695`，网关 `orb-profile-handoff-be9f6c60`。新夹具通过真实本机受控恢复/补偿/快照/normal/health/completion 准备本轮前置决定；不复用旧 journal，不重发旧请求，不重复历史浏览器写11验收。普通期选定预期为9。

**ordinary unit未由脚本直接启动。** 中心 prepare `7d1fd892-a4a6-4f18-b663-668a1c709626`、execute `aeee8483-7c19-4938-9805-0222a185d3eb` 各一次，经过重新加载与明确勾选，后台启动真实普通Kura PID556444。真实DS/密码/普通快照和选定provider通过，旧三恢复unit PID0。原命令幂等键只读回查通过；本次实际流程没有观察到 UNKNOWN，也没有 reconciliation 审计，不能算作新的实际进程退出核定。

随后只重开新agent，普通Kura PID/InvocationID及全部原生 journal/receipt SHA不变；公开管理UDS与原GWT admin/admin中文Count9通过。最终五个新增unit全inactive/PID0，普通unit从未enable，原生终态/密钥/快照/备份/脚本全部保留。原Orb四服务仍PID244516/244911/286620/268，原console26/agent20包；未改主机Core marker、原服务或其他VM。

## Parallels默认关闭及真实沙箱

仅复用原Ubuntu VM，新增 `/opt/yofc-profile-handoff-default-20261106-6fc4f68c`；当前deb只解包，生产main PID2639运行。没有配置ordinary远程开关或grant，没有Java，没有创建Kura/primitive目标unit或伪造completion。仅为这个全新夹具明确建立初始远程通道，以验证真实Hello不声明handoff能力。

实际 ProtectSystem=strict、ProtectHome=yes、PrivateTmp、NoNewPrivileges、空CapabilityBoundingSet、AF_UNIX/INET/INET6、固定RW state/run；进程CapEff=0/NoNewPrivs=1。agent实际mount namespace下固定输入写入返回EROFS且SHA不变。严格TLS连同一隔离中心，中心对20/21/22均在准入前拒绝UNSUPPORTED，无handoff任务记录。此项不验证Java/Kura业务。新agent正常停止，Ubuntu经guest正常poweroff，Ubuntu/Windows最终均stopped。

## 源码、制品与保留记录

267冻结输入、37项agent staged manifest逐值匹配。新protocol artifact SHA `702d86ff09e8b9ca94392fe22084d70219188ca6ea5b41a04251ee2f7088fe1d`，按SHA独立vendor路径构建；Linux独立源码/复制缓存，旧cache未改。agent ELF `4600afd5873daf3110688fdb622e5828b27b4549baf420aa8ab8cc3bbc43e233`；review deb `0.1.0~review20261106.profilehandoff1_arm64` SHA `9407fc23f789871c4b9c5b53a3d937d407d04908dcb3ab9cb0ab088198d62b11`；manifest SHA `c697ccf087d5f822e324cd5d8bc081a249de663a10a3858efad9b51a05ad12c0`。仅review候选，未正式发行。

丢失响应只读查找在首宿主中心零任务时加入：先停本轮PID92140，保留其ELF，再用当前ELF启动PID98170，同CA/DB/key。此前没有远端任务，不重发业务。agent构建输入未变；最终真实中心流程使用新版HTTP/前端。

保留的失败：系统Python3.9缺tomllib，改用已有Homebrew Python后56通过；最初Task数字枚举尚止于19导致类型检查失败，扩展后通过；测试误调用私有helper已改走真实公开入口；HTTP未知字段的framework拒绝为422，测试原先只接受400，修正为400/422且严格INVALID_ARGUMENT；首次最终台账错误假定默认admin租户dev。其HTTP首次设置拒绝断言已通过，后续仅重开同一私有PG只读核对，证实租户为default且must_change_password=true，再正常停止。未修改账号、任务或业务结果。

真实子仓图谱已更新，相关Rust/TS/proto路径metadata_match仅best-effort；0017 SQL parser unusable，已全文直读并由真实迁移/PG验证。未索引父目录。

## 收尾和下一项

新IDEA JVM/observer、Orb新增unit、Parallels新增agent、宿主中心/PG/Vite/keepawake均停止；两Parallels VM stopped；Orb VM与原四服务继续运行。全部state/PKI/key/数据库/失败目录/旧夹具保留。18仓HEAD/branch及原dirty保留，无reset/clean、提交/推送/远端覆盖，未接触192.168.2.125、个人~/.kura-dev或其他运行runtime。

**内部pending：** 普通交接native决定与原远端命令的独立业务核定resolver（进程丢失execution receipt时仍UNKNOWN；不从当前unit推断）、其余未决计划运维处置；restart/reboot远程生命周期；Core完整安装/bootstrap/升级、明确策略交接/非原生入口退役；完整Kura在VM/网关reboot后的全业务；完整组件/metatype/domain providers、部分/空CM与外部PID；NM协同、防火墙适配/NAT/远程/早启；多轮key、数据库/制品一致备份恢复、旧迁移及正式发行。

**external deferred：** Parallels完整Java/Kura、真实NM/cloud-init/物理有线及硬件、amd64、磁盘满/硬断电和原外部条件。两历史evidence gap保留。scope false flags不能被本轮选定检查替代。
