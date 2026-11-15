# 固定 Kura restart 与主机 reboot 的真实系统验证

核定 `4230b9b8`，2026-11-07。完整管理体系目标保持 **active**。本轮完成 a09e95d7 的真实 HTTP、IDEA、OrbStack 和现有 Parallels 验证；固定服务重启及主机重启任务，在本机明确授权和选定业务范围内闭环。不是全部组件、全部 Kura 业务、Core 安装升级或正式发行完成。证据根 `target/restart-reboot-system-20261106/`，机读证据 `docs/evidence/restart-reboot-system-20261107.json`。旧夹具和一次性恢复入口未重放。

## 实施修正与合同

- 真实 systemd 255 会省略空 ExecStop；固定声明校验现在把这个可选空字段规范为空串，仍必须存在并校验 ExecStart，其余有效声明漂移继续拒绝。增加真实格式回归。
- COMPLETED 的 service.status 返回已持久提交的完整 outcome，保留进程/主机/选定业务证明，使刷新或 agent 重开后仍显示原结果。没有重新检查业务、再次执行系统动作或暴露 confirmToken；两次读取的 journal/命令/receipt 字节相同。
- 新真实 HTTP/PG 契约覆盖 admin、租户/归属、初始密码、CSRF、非法字段、独立 capability、过期/离线 epoch、TTL、输入加密/非 admin token 脱敏、原意图隔离与 UNKNOWN 防重发。
- 保留 NM、Kura 不装 agent 独立运行、agent 常驻与独立远程开关、Kura 后端生成密钥/CSR、默认 admin/admin 及首次设置闸门、配置/防火墙默认关闭。系统授权仍默认不存在，restart/reboot 分别明确开放；不接受 caller unit/path/shell。UNKNOWN 不重发，RR 保留并挡写。

## 已验证

宿主最终受影响 native **11 passed**，两仓严格 Clippy/fmt、React typecheck、Git diff check 通过。新真实 HTTP/PG **1 passed**；它是包含多项准入断言的一个集成测试，不与前轮 PG 两项混算。Linux 最终 v3 **188 passed / 10 ignored**、严格 Clippy、优化测试构建及 ELF/deb 构建通过。Linux 包装 **65 passed**、Core 纯维护 guard **17 passed**；没有执行原系统包升级。a09e95d7 的宿主整套 186/9ignored、mock 浏览器10和其他测试仍是原轮原版本的证据，没有伪称本轮重跑。

本轮成功的新增真实浏览器场景为四个：新 IDEA 原 GWT；Orb 原任务只读中英/刷新与 GWT 续验；Parallels 默认关闭/完整 Kura 独立登录；Parallels 实际一次 reboot/中英与 GWT。浏览器启动只信任本轮 CA，未全局跳过 TLS。

### IDEA 完整隔离 Kura

新 fixture `.phase1-dev/system-control-idea-20261106-863073cc`，284 inventory；经 IDEA UI 启动 seed PID3669 一次、ordinary PID3893。真实 DS 激活、密码解密、快照写入、schema2 ordinary observer（唯一 provider/选定组件与检查）及原 GWT admin/admin 中文 Count9 通过。两 JVM 和 observer 均已停，短 UDS 文件保留。个人 ~/.kura-dev 和其他运行集未清理。

### OrbStack 一次固定 Kura restart

新 fixture `/opt/yofc-system-control-20261106-3c13a447`，seed637608 一次且已停。真实中心 HTTPS → mTLS gRPC → v2 agent → 固定完整 Kura unit：prepare `a009016e-7658-4ff2-8606-37718853bd83`，execute `6cf7ebc4-1b08-41f0-8f98-b24eba929179`，各一次。Kura PID658127 → 658378、InvocationID 换代，boot ID 不变；原任务 SUCCEEDED，选定 DS/密码/快照及 schema2 provider/check 通过。

之后仅重开该私有 agent 为最终 v3，PID678182，Kura PID658378 不变；已提交 native 文件 SHA 不变。只读续验新增系统写 POST=0，确认中英文结果及刷新保持，原 GWT admin/admin Count9 通过。Orb 的主机 reboot 本机未授权，prepare 被拒绝；没有重启 Orb VM。Orb 全局 zzz-lxc-service.conf 隔离限制仍保留，不能据请求属性宣布有效 strict sandbox。

### Parallels 实际 reboot 与完整 Kura 的选定业务

只用现有 Ubuntu26.04 ARM64（2CPU/4GiB）VM，Windows 未启动。系统仍未全局安装 Java；在新 fixture `/opt/yofc-system-reboot-20261106-8f68773c` 内解开 Orb 既有 Java21 的自包含副本，184 文件逐一 SHA 校验、无外部 symlink。新 ordinary profile 同时固定全部 Java 文件、284 bundle 和启动配置，合计 **472 文件**。这是完整隔离运行集的 direct-java 普通启动，不是 sleep stand-in，也不是已安装 Core deb 的升级测试。

seed2767 一次通过后停止；ordinary3034 完成默认关闭验收。默认无 system.control/restart/reboot capability，UI 不显示入口，本地 status/prepare 返回 UNSUPPORTED，无 native 事务；原 GWT仍 admin/admin Count9。安装封存要求停止 ordinary，故仅停止普通服务以封存，然后启动普通 PID4247；没有重播 seed。随后仅明确 allowReboot=true，allowRestart=false；本地 restart.prepare 返回 FORBIDDEN，中心按钮同样禁用。

通过中心 prepare `73138bcc-67f2-44f2-a8d9-a934b7f5b12b` / execute `1cb4f6ab-9dd0-4a07-8e52-42b3effd70d4` 各一次。实际 boot ID 从 `143ab4e5-f96c-427c-838a-298dcd12d01b` 变为 `6307ae2e-bad3-4940-8b55-4ee989798428`；固定唯一 timer `yofc-system-reboot-eeb60062-f399-45c6-8487-e77eb11682fc.service`，系统重启后 agent1532 和 Kura1533 自动启动。原任务实际经历 QUEUED → RUNNING → UNKNOWN → SUCCEEDED；仅核定原命令，SQLite execution_resolutions **1** 条、中心 task.reconcile.system_control **1** 次。

原任务 HOST_REBOOT_OBSERVED：processRestartVerified、hostRebootVerified、applicationVerified 均 true；schema2 报告绑定新 PID1533、原 profile/approved snapshot、唯一 provider 和选定 DS/check。新进程真实密码解密与 snapshot() 成功，原 GWT admin/admin 中文 Count9；中英文系统面板通过。重启前后均实测 CapEff=0、NoNewPrivs=1、ProtectSystem=strict、输入写入 EROFS=30。重启后两次只读 status 与已提交 outcome 完全一致，native 事务/命令/receipt 字节未改变，Kura PID不变。

所有结果仍限定 administrator-selected-checks；`kuraRecoveryVerified=false`、`normalBootVerified=false`。这两项是完整恢复承诺，不能因本轮宿主及进程换代被提升。

## 失败与现场保留

Orb 原浏览器在真实 restart 已成功后因中英 region 选择器失效中止；没有重发准备/执行。只读续验首次又遇登录完成前即 GET 的测试时序问题；修正等待后从同一原 execute ID 完成只读验证。原失败日志和截图均保留，未将原失败用例改报为通过。

前置夹具准备错误均在实际系统任务发出前：ordinary sealer 的派生摘要/全局 drop-in pin、Parallels 默认关闭响应预期、停止 ordinary 才能封存、system request 必填摘要占位字段。按原 fixture 已完成阶段续接，旧请求和诊断保留；没有重放 seed，没有伪造结果。v1/v2 制品和全部失败目录保留。最终 v3 的生产修正仅为上述空 ExecStop 兼容及已提交状态呈现。

## 制品、停止状态与后续

最终候选 agent `0.1.0~review20261106.systemcontrol3_arm64`；ELF SHA256 `ac61f5fc38f0fe30c09596bec9d4e06d82ebb7e177464eb0e9a43f8d5363cfa6`，deb `9893692e48110895412b20bff74029828e3ac38143e2d2149e4c712284f8f3f0`。source manifest `5257a174e75a8ef67c7afeb6d6d8569b5c45adced1d974405aaf11d9ff1a76a1`，43 原输入与独立 staged Cargo.toml 共44记录已读回一致。协议 crate `daa2e75af24bed80018269afe86e7c2fb5d75ee1b8f5890856b73b2d48fb10ff`。不是正式发行包，未安装/升级原包。

本轮宿主 console4681/私有PG16648/keepawake6927、IDEA JVM/observer、Orb 私有 agent/Kura均已停止；Orb瞬态v3 agent停止后自动卸载，文件/日志仍保留。Parallels 新 agent/Kura正常停止并 disabled，seed inactive；Ubuntu及Windows均 stopped。原Orb四服务仍 active 原PID：console244516、center2 244911、agent286620、phase2-agent268；原26/20包未动。18仓HEAD/分支与全部原dirty路径保留，无commit/push/reset/clean。

本轮替代“restart/reboot 尚无真实系统闭环”和“Parallels 因无Java不能做本轮完整Kura选定检查”的旧描述。私有Java环境已可用，但不追认历史任务或全组件恢复通过。仍为内部 pending：未决交接运维、Core完整安装/升级/策略交接和退役、全组件/metatype/provider/空CM/外部PID适用面、Java离线恢复完整业务边界、NM/firewall适配/NAT/forward/远程流程/早启、一致DB/制品/密钥恢复与多轮key/旧系统迁移/正式发行。外部 deferred：真实NM/cloud-init/物理网络/硬件、amd64、磁盘满/硬断电等；两个旧 evidence gap 继续单列。

下一轮优先核对完整台账中未决运维与Core包交接的真实可执行缺口，或继续全组件业务适用面；不重做已闭环系统动作。新增实现仍按单元/契约与浏览器 → 新IDEA完整Kura及原GWT → Orb → 现有Parallels顺序验证。
