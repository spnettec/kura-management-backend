# 普通 Kura 包启动、安装授权及冷启动就绪核定（2026-11-06）

完整目标仍为 **active**。本轮交付 Core 包启动器的独立可选参数入口、agent 本地安装授权封装，以及普通启动观察协议 schema 2。它们通过新 IDEA、实际 Orb 包脚本及 Parallels 沙箱核验；不代表全部 Kura 业务恢复或正式发行完成。对应 [机器证据](../evidence/ordinary-package-launcher-20261106.json)，私有完整日志在 `target/ordinary-package-launcher-20261106/`，最新修正与最终状态在其 `provider-readiness-v2/`。

## 产品变化

Core 的 `start_kura_background.sh` 接受可选 `KURA_JVM_ARGS_FILE`。不设置时保留原 JVM 参数；设置时必须为可读、绝对、非符号链接的普通文件，并以一个 Java `@argfile` 参数传递。错误输入在生成配置、启动 Java 之前失败。这个入口没有 agent 引用或启动依赖；普通参数文件应属于 Kura 安装输入并在卸载 agent 后保留。字面量 shell `$@` 与 Maven `@...@` 占位符分行，避免打包资源过滤吞掉内存参数。

agent 保留默认 `direct-java`，另需显式选择 `packaged-background`、固定脚本路径及 `KURA_JVM_ARGS_FILE`。脚本、相邻 generator、JVM 参数、unit 和有效 drop-in 全部固定哈希；只允许 `/bin/sh 固定脚本`，不能由请求供应命令或额外 shell 参数。原三项 JVM 环境注入清除要求继续保留。

新增 `seal-ordinary-profile --request PRIVATE_JSON --output NEW_PRIVATE_GRANT`。本地管理员预审请求不含 unit 摘要，CLI 从实际 loaded/inactive/PID0 的 unit 导出有效定义摘要，重新核对输入并持久创建私有授权。它在身份、数据库和 UDS 初始化之前执行，只调用有界 `systemctl show`；没有 start/enable/reload 或自动激活授权。回执的 `unitStarted=false/unitEnabled=false` 仅说明安装器未执行这些动作，**不认证原有启用状态或整个开机依赖图**。boot adoption、冲突启动入口退役及中心任务仍 pending。合同见 agent `docs/ordinary-profile-handoff-v1.md`。

## 冷启动发现与修正

第一次实际包脚本的全量冷启动中，框架已达 5、三个 PID 已在 ConfigurationService 登记，但 DS 的业务实例尚未激活。旧观察 schema 1 立即调用 provider，第一次返回 false；agent 正确转入 `RECOVERY_REQUIRED`，停止四个 Kura unit，现场未重发。独立新只读诊断在相同冷启动条件下确认 `live-keep` 尚无实际值，而非类型、密码或回调超时问题。旧失败和诊断夹具均停止保留。

新版 Java 报告 schema 2 明确列出每个选定 provider 的 `available/unique`。只有框架、组件登记与全部 provider 均可用才调用业务检查；缺服务属于有界只读等待。provider 必须在自身应用初始化完成后发布服务，组件名称不能代替业务就绪。重复 provider、异常、超时、false 或不一致的通过元数据立即失败，不把业务失败轮询成成功。测试 provider 仅等待实际应用属性与其 CM 期望相符后发布，之后一次性核验既定业务值，没有等待“业务检查通过”再掩盖失败。

新 execute 必须使用 schema 2。schema 1 只为已经持久完成的历史回执保留精确兼容，不可授权新交接。就绪失败仍保留观察与日志、RR 挡写；UNKNOWN 不重发。snapshot 哈希回显仍只是批准上下文，并非独立加载证明。最终普通终态仍仅 `administrator-selected-checks`，`kuraRecoveryVerified/normalBootVerified=false`。

## 验证与制品

- 宿主 agent **158 passed / 6 ignored**；Linux **158 / 7 ignored**；两端严格 Clippy、宿主 fmt 通过。负例覆盖缺 provider 后就绪、重复服务、非法通过、旧 schema 新执行拒绝、历史 schema 1 回执无重启核定以及原有不重试/中断合同。
- Java bridge **53**；Core 启动脚本 **3**，含不启用、带空格的单参数 argfile、非法文件在副作用前拒绝。Python 生产包装代码未改，旧 **55** 项作为 5951e763 历史证据沿用，未冒充本轮重跑。
- IDEA schema 1 的新全量恢复/补偿/普通交接先已通过；修正后另建完整 284 项冷 OSGi 配置运行集 `.phase1-dev/readiness-v2-observer-idea-20261106-ebd1d315`，显式只读 observer 测试 **1** 项通过：已登记 PID 且 provider 缺失被观察到，随后首次 ready 检查通过，未重试业务失败。真实 DS、密码解密、普通快照写入通过；原 GWT admin/admin 中文登录读取 Count9。observer 退出后同一 Kura PID8286 存活，随后正常停止。该 native 修正验证不宣称重跑整条恢复事务或真实 shell。
- Orb 新 `/opt/yofc-packaged-readiness2-20261106-ff7c356e` 使用实际 CLI 封装 295 个固定输入，完整正向、逆向、文件集、受控普通、健康、明确完成和原 GWT 9→11 通过，再由真正 `Type=forking /bin/sh` Core 包脚本冷启动普通 JVM456939。schema 2 的三个组件及一个真实业务 provider 通过，独立业务断言、密码和新快照写入通过。原 GWT 重新登录并实际读取11，停止测试 agent 后同一 Kura PID/InvocationID/回执保持，页面仍可用。启动后初次页面菜单加载未完成，普通只读 reload 后成功，不曾重启 Kura 或重发事务。期间宿主休眠造成命令通道等待，恢复后接续同一事务。
- Parallels 新 `/opt/yofc-packaged-seal-prl-20261106-196b1b31` 只解包当前 review agent deb，不升级原安装。在实际严格 systemd 沙箱中运行 CLI：私有创建、重复目标原样保留、未知字段拒绝、输入写入 EROFS、全部输入 SHA 与目标 unit 状态不变。`ProtectSystem=strict/ProtectHome=yes/PrivateTmp=yes/NoNewPrivileges=yes/CapabilityBoundingSet空/AF_UNIX` 生效；目标始终 static/inactive/PID0、未启动，探针已停。Java 不存在，完整 Kura 业务保持 deferred。首个宿主驱动因 prlctl 长命令参数限制未创建 guest 夹具，host plan/log 保留；v2 使用已有共享 Home 中的脚本执行成功，未重放旧 mount/fsync。

Core review deb：`6.0.0~review20261106.ordinary3-1_arm64`，SHA `8de420ff8fcbf2294b8f3f7e3ba898d4ac1a723d315833b47f7237e33ff9b867`。从隔离源副本 Maven 离线构建，仅验证提取的实际启动 payload，**未在原网关 dpkg 安装整个 Core 包**。首份内存占位符未替换的候选及自动 SCM 标签候选均保留，不作正式发行。

agent review deb：`0.1.0~review20261106.ordinary2_arm64`，SHA `817833f107fc31b690f22002175a5030ea594ddf0792f067c987002731c5b6e9`，ELF SHA `4c8b1d4144d34ad4c8ecea008cc16bc58e15e67e389da381f0a2df9c18a906f8`，最高 GLIBC2.39。自包含源 manifest SHA `fe3d7705ca5fb2be8548997e7f0cd8d990744ef42c81678e21d9c41c995669d0`；协议 crate SHA `07a78a3563a3bdc57e6e41f59419b32463145dbdac10ba3a5977c80714cbda98`。实际源与 stage 逐条匹配，最终 195 条受审输入在测试/联调后保持一致。早期 schema 1 agent review 包及所有 build 源、target 保留。

## 最终运行状态与边界

新 native JVM、observer、driver 均停止。Orb 新五 unit inactive/PID0/Result success（临时 agent 已被 systemd 收集，LoadState not-found）；ordinary UnitFileState disabled，未启用。终态 journal 在主动停服后字节不变；原候选 snapshot 按文件集合同位于 `.recovery-retained/`，原请求/回执/加密 CM 证据哈希不变。四个原服务继续 active，原 console `0.1.0~package20261102-26`、agent `0.1.0~package20261103-20` 未升级。Parallels Ubuntu/Windows 均 stopped。

旧 Orb `e6ee2ec6` 为建模阶段失败、无事务；`192b3b09` 为 schema 1 冷启动失败、RR；只读诊断 `0f4bfc4c` 已停止，均保留且不重发。此前 5951e763 的失败/成功夹具仍保持原核定。

18 仓 HEAD/branch 与全部旧 dirty 路径保留；无 reset/clean、提交、推送或远端覆盖；未连接禁用网关、未清理个人或运行中 runtime。Core/agent/networking 图谱已按各自真实根目录刷新。admin/admin 与首次改密后台闸门、中英文、NM 保留、独立身份、配置与 firewall 默认关闭均不改变。

下一 pending：普通安装/bootstrap/显式 boot 接管与冲突入口退役、中心 ordinary/restart/reboot 生命周期；完整组件/metatype/真实 domain providers、部分/空 CM 与外部 PID；NM 协同、防火墙 NAT/远程/早启；多轮 key 轮换、一致备份恢复、旧系统迁移、正式发行。deferred 单列 Parallels 完整 Java/Kura、真实 NM/cloud-init、物理有线/硬件、amd64、磁盘满/硬断电。两个历史 evidence gap 不改写为通过。最新知识交接：`Experience/2026-11-06-Kura普通包启动授权与冷启动就绪验证续接-372d897d.md`。
