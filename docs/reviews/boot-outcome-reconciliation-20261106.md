# 原命令启动策略核定与回执故障闭环（2026-11-06，cdd8ab4e）

完整管理体系目标保持 active。本轮实际补齐 770604cf 中“boot 决定已落盘、execution receipt 未落盘时缺少原命令核定”的交付项。没有重做迁移、重放旧夹具或替换原 Orb 服务。证据根为 `target/boot-outcome-reconciliation-20261106/`；[机器 evidence](../evidence/boot-outcome-reconciliation-20261106.json) 和 agent `docs/ordinary-boot-task-outcome-v1.md` 规定精确范围。

## 原命令证据链

1. 对已认证、加密持久准入的完整 ReadCommand Protobuf 求 SHA256，绑定原 command UUID、operation、幂等键、期限及 typed action；管理 UDS 不能传入这种远端绑定。
2. 原生事务原子写入 `commandDecisions`，将私有 create-only 命令决定文件的 SHA 关联到该事务。execute 在任何动作前持久化 `issuedCommand`，并把它写入不可变原生策略 receipt；显式 reconcile 可以同时记录原 execute 与自身决定。单独 sidecar 不足以证明成功。
3. 原 UNKNOWN receipt 查询认证原计划，仅查当前/历史原生事务、命令关联、精确结果与 receipt。缺失、旧格式、孤儿、权限错误、篡改或未提交一律继续 UNKNOWN；不查询当前 unit、不修复原生事务、不调用 systemctl、不重发。
4. SQLite 原子落加密终态与一条核定记录；中心独立从加密 input 与原期限/幂等键重构 wire command，精确校验摘要，拒绝错误任务/载荷或 boot/business 过度声明。中心只为一次 UNKNOWN→终态记录 `task.reconcile.ordinary_boot`。

仅 `commandDecisions` 属于传输决定元数据，不改变已确认内核策略的相等性；`issuedCommand` 保留在策略 receipt 和其他相等比较内。agent lifecycle、Core preinst/postinst 只接受明确可选字段，并验证私有决定文件、摘要和身份。旧字段缺省时保持原 receipt 字节兼容；原先没有命令关联的 UNKNOWN 不被推断为成功。每事务最多64条决定，容量不足拒绝新准入，不清旧证据。缺失主 journal 但有 command evidence 时也不能作为新安装。

部分动作仍为 RECOVERY_REQUIRED，阻挡后续写入。只有显式 policy reconcile 可以只读核定已发出的动作，完成原生决定；随后原任务的独立 receipt query 才可核定。此项不承诺自动处理任意存储故障或所有 pre-commit 孤儿状态。`bootVerified=false`、`kuraRecoveryVerified=false` 保持。

## 界面变化及发现的租约窗口

中英文任务详情明确标注“已依据原命令的启动策略决定核定”，说明没有再次执行命令，也未证明实际开机/完整 Kura 业务恢复。实际 Orb prepare 读取大包输入时跨过中心租约，先 UNKNOWN 后原回执成功；原面板在首次 UNKNOWN 停止读取，未显示已准备计划。本轮已改为在有界时间内继续 GET 同一任务，显示不确定提示并挡住写入，不再发送 POST。终态到达可完成当前显示；超界则保留原 ID 与未知闸门。

首次浏览器只创建了 prepare `c7d03102-23be-49f5-a863-6a57da1e03a2`，没有 execute。修正及 mock 复验后，通过原 task ID、认证 API 和只读 status 载入同一准备单，明确勾选后只下发一次 execute；未取消重建或替换该计划。

## 当前验证

| 层次 | 最终范围 |
| --- | --- |
| Rust agent | 宿主173 passed/8 ignored；Linux并行173 passed/9 ignored；两处严格 Clippy、fmt；四种变更均以独立子进程在原生决定后退出，重开后并发原 receipt 查询只核定一次，且不产生任何 unit 查询/动作 |
| 反例与补偿边界 | RR 查询保持 UNKNOWN；显式 policy reconcile 后再证明原命令；缺失/暴露/篡改/跨命令/无原子关联证据不因当前 enablement 成功；后续本机计划归档后仍能读取历史命令证明 |
| 中心/协议 | console18、protocol9；实际 PostgreSQL7，含四种操作原加密 input、wire字段篡改拒绝、过期后核定、幂等一次审计及既有认证/转移/存储回归；无新增 migration/wire编号 |
| Python/Core | agent包装宿主/Linux root各56；Core guard宿主/Linux各13，含有命令元数据时的兼容、私有/摘要拒绝和 confirmed 策略保护 |
| 前端 | typecheck/build；最终 mock8，含 UNKNOWN→原终态持续只读、无替代 prepare/execute 和双语/错误证明拒绝；Orb实际 live1、Parallels实际 live1 |
| IDEA | 新 `boot-outcome-idea-20261106-76a3007b`，284 inventory；IDEA UI启动 seed49177、普通49859；真实DS/密码/普通snapshot9、schema2 provider缺失后发布/observer1、原GWT admin/admin中文Count9；observer退出后同JVM。已停止保留 |

Java/API/GWT 生产源没有变化，不把历史 Java429/bridge53 或 Docker 验收记成本轮新通过数。Core本轮仅改维护 guard，未进行完整 dpkg 安装/升级。React大 chunk及既有AntD弃用提示保留，未为警告改变业务边界。

## Orb 实际原生决定与真实 Kura

最终夹具 `/opt/yofc-boot-outcome-orb-20261106-1eafee4e`，网关 `orb-boot-outcome-8ef22c26`。实际普通包 Kura PID534120，在独立 observer 完成真实DS/密码/snapshot9后，由测试专用 transport 可执行文件接受真实 mTLS 命令。它在 native policy COMMITTED 后、SQLite execution receipt 写入前实际进程 exit92：原 admission仍 RUNNING/result NULL，中心原任务已 UNKNOWN。

只启动生产 ELF（PID534918）后，现有 mTLS receipt 查询核定原 execute `02402c81-4781-4535-9699-1e6db0688456` 为 SUCCEEDED，`task.reconcile.ordinary_boot` 仅一次；浏览器 execute POST仅一次、替代prepare零次。随后新 agent再重启为534956，Kura PID/InvocationID、原生 transaction/confirmed 字节不变；公开管理 UDS及原GWT登录 Count9通过。Core guard在该agent实际mount namespace下接受命令证据，并明确拒绝重启用另一Kura启动入口，未到达测试副作用。

浏览器第一次 prepare 的 UNKNOWN 最终由已有加密回执核定，审计 source 是 `task.reconcile.receipt`；execute 故障由新的原命令业务证据核定，source是 `task.reconcile.ordinary_boot`，两者没有混算。

## Parallels 实际严格沙箱与同一中心链

复用原 Ubuntu VM，夹具 `/opt/yofc-boot-outcome-prl-20261106-ba3e77ef`，网关 `prl-boot-outcome-accaaeed`。当前 deb仅解包，明确无Java，target是sleep stand-in。先用生产main实际验证default-off/unbound；已有OFF决定不会被 env=1或测试helper的initial=true覆盖，随后通过公开UID管理UDS以原revision明确ON，保留该决定。

测试transport和生产main均运行于实际 ProtectSystem=strict、ProtectHome=yes、PrivateTmp、NoNewPrivileges、空capabilities、受限AF_UNIX/INET/INET6及固定读写路径。实际mount写固定request返回EROFS。为现有共享宿主IP签发隔离leaf v3，保留旧leaf/同CA/key，正常重启本轮隔离中心；未关闭TLS验证或修改VM网络/DNS。

原 execute `1896c9fa-10ab-4a90-a3b1-1572023da4fc` 经真实mTLS/native COMMITTED→测试进程exit92→中心UNKNOWN→单次启动生产main3793→原命令证明SUCCEEDED，审计仅一次。生产main再重启后 native证据、target PID3199/InvocationID与唯一start记录保持；新Coreguard在真实mount中通过元数据校验后拒绝不同目标。此为系统沙箱及任务闭环，不能当作Java/Kura业务恢复。共享helper原始字段名 `sameKuraProcessBeforeProduction` 已在独立 `proof-scope.json` 明确解释为stand-in进程，原始证据保留。

## 失败、制品与源码

首Orb夹具0c095e78缺少隔离启动脚本，只有seed已完成并停止，未创建boot事务/普通unit；保留失败目录与日志，不重跑该目录。补齐与当前未变Core启动模板一致的两个输入后，以1eafee4e新夹具继续。第一次Linux包装测试只复制packaging而缺Cargo/src，56中2个制品用例因此报输入缺失；完整源的新夹具56通过，原失败保留，Core13原已通过未重跑。前端初期类型/label/模拟SSE夹具与严格Clippy发现均已修正并复验，原日志保留。

| 制品 | SHA256 |
| --- | --- |
| agent arm64 `bootoutcome1` ELF | 9bd97cd495d98c656d0234ae12b767e81b32185e4be3704dd3f15fc82c67e3c3 |
| agent `0.1.0~review20261106.bootoutcome1_arm64` deb | 50c1ce921bab4d5528da36e4d491e7cf11312cf5670c91f65735dcfe60f4317d |
| agent staged manifest | fea046b2944738effbb4a2f2b163618439d3fc47236a4fee93253f8f7a9d890d |
| 实际宿主console immutable ELF | 17379cb0a81a4d86f776628d190aaa9c8c8cd074ca6e742312f85e1d04618ffe |
| 未变protocol crate | de2780ad9edd164528a2e921e58fe4106bac96880558b2e81cc488692276dcb9 |

故障开关只在 `cfg(test)`；生产ELF查验不含两个测试环境标记。本轮明确是进程退出注入，不是真实硬断电。262冻结输入及agent37 manifest逐值匹配；后续仅前端原任务读取/测试元数据改变，后端构建输入未漂移，并以当前UI完成两个真实live流程。仅review制品，未发布或升级原包。真实子仓graph已更新，相关Rust/Python/TS路径metadata_match只是best-effort；Core shell源直接核对并用宿主/Linux及实际namespace验证。

## 最终运行状态与后续

全部新IDEA JVM/observer、宿主中心（先51683、换隔离leaf后61389）/PG/Vite/keepawake已停止；数据库、全部PKI、密钥、失败目录及证据保留。新Orb和Parallels正常agent/target均停止，测试target/retired disabled；两个故障unit保留预期failed/exit92、MainPID0，不reset-failed抹平现场。首次收尾断言误要求故障unit inactive，实际停服/disable已完成；后续仅只读记录真实failed状态，未重复停止或重发动作。历史 confirmed COMMITTED不因测试收尾伪称回滚。

Orb原四服务保持原PID244516/244911/286620/268和console26/agent20包active，VM继续running；Ubuntu/Windows最终均正常stopped。两Linux主机的全局Core marker仍不存在；marker仅在新agent mount隔离目录。18仓HEAD/branch/旧dirty保留，无reset/clean/提交推送/远端覆盖，不触碰192.168.2.125、旧一次性入口、个人~/.kura-dev或运行runtime。

**内部pending：** 完整ordinary-profile中心交接、restart/reboot远程闭环；完整Core安装/bootstrap/升级及明确策略交接/非原生入口退役；真实完整Kura VM/网关reboot后的业务；全组件/metatype/domain providers、部分/空CM与外部PID；NM协同、防火墙适配/NAT/远程/早启；多轮key、数据库和制品一致备份恢复、旧迁移与正式发行。任意存储错误或pre-commit孤儿的管理恢复不能由本轮有限证明自动替代；保留证据与UNKNOWN/RR。

**external deferred：** Parallels完整Java/Kura环境、真实NM/cloud-init/物理有线硬件、amd64、磁盘满/硬断电及原外部条件；两历史evidence gap继续保留。旧F01–F19与一次性验收不重放。最新知识库 `Experience/2026-11-06-Kura原命令启动策略核定与回执故障闭环-cdd8ab4e.md`、Tasks/Session Summaries与continuation同步；整体目标active。
