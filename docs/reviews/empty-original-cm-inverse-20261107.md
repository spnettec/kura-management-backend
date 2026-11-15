# 原始空 CM 集合的认证补偿准入

核定 `d1994c05`，2026-11-07，接续 `f7a8438c`；完整目标继续 **active**。本轮关闭“底层能清空 CM，但实际 bootstrap 无法接纳原始空备份”的缺口，不重放既有动态 provider、restart/reboot 或其他一次性验收。

## 缺口与实现

ConfigurationRestore.compensate 已能恢复原始空 managed 集合，且已有真实 Equinox CM 持久化合同；实际路径仍在三个位置拒绝空值：SnapshotTool.decode 的普通上传限制、ConfigurationRestore.expected 的意图投影、ControlledRecovery.admit 的非空准入。直接放宽普通上传会混淆一次性补偿与新的空目标恢复，因此使用独立的受控路径。

Bootstrap 现在先认证原请求的 source receipt、独立 source plan、原始 backup 及 intended 的固定 SHA、restore 身份和 AES-GCM；仅取得该次真实非空应用意图后，才通过包内 decodeRetainedCmBackup 读取原始空备份。普通 validate/decode、普通 apply、普通准入仍拒绝空目标，没有新增可由上传者提供的 allowEmpty 开关。expected 只投影已验证的 CM 意图，允许表示空集合，不授予写权限。

补偿继续由 ConfigurationRestore.compensate 逐项核对当前 CM 是否来自原值/已应用值，保存加密的补偿前、意图、应用后副本，再精确移除本次新建的 Kura managed 记录。无 Kura 标记的其他 CM 记录保留。专用 admitCompensation 保留源文件最高编号和摘要检查；Core 对 Optional.of(emptyList) 读取为空，不回退其他快照，不种植默认值，不解锁普通写。

没有改动 Rust agent 生产源码、协议或默认授权，也没有改 UI/中心认证逻辑。默认 admin/admin、首次密码后台闸门及双语不变；本轮隔离 home 沿用此前 test-only need.password.change=false，不能扩大为生产首登验收。远程通道、远程配置写和 firewall 均关闭；只在新夹具内授予本机管理员固定恢复单元和目录。

## 测试与实际核验

最终受影响 Maven reactor **474** 项，零失败/错误/跳过，其中包含本次 reactor 的 6 项构建测试支持合同。新 forked-JVM 合同证明普通空准入拒绝、错误摘要拒绝、空补偿一次接纳、实际 Core 快照加载不回退、普通写持续拒绝、较高未授权快照及请求属性变化挡住后续读取。真实 crypto/mapper/validator 合同证明认证的原始空备份可读取，authenticated/legacy 普通上传均拒绝，普通 apply 在备份或 CM 调用前拒绝。

每个完整场景先在全新 home 正常启动，原 GWT admin/admin 登录后生成完整新快照；受控 apply 使用另一个仅复制静态配置文件的全新 Equinox storage。真实加密 CM 原始备份经独立短时 Java 工具解密确认为 **0**；候选包含完整配置及新 DS 业务，apply 为 **26**，实际 DS generation2、秘密解密、普通写闸门与原 GWT 读回通过。独立 inverse 使用该次 create-only 源回执和原备份，真实 CM API 与加密应用后副本均为 **0**，非 Kura record value77 保留，业务只在 apply 激活一次，inverse 未误激活。

| 环境 | 新现场及范围 | 状态 |
|---|---|---|
| IDEA v2 | `.phase1-dev/empty-cm-idea-v2-20261107-3d842a53`；seed78626→apply78742→inverse78803；284 inventory / 实际283 bundles；307 profile pins | 完整 Kura 仅由 IDEA UI 启动；直接 Java bootstrap 独立请求，不冒称本机 agent 事务；空原备份与空 inverse、原 GWT、非管理 CM、持续挡写通过，全部 JVM 已停 |
| OrbStack | `/opt/yofc-emptycm-orb-20261107-82cfd77b`；seed710029→apply710366→inverse710643；生产 agent ELF / 真实 UDS/systemd | 完整所选空原备份/26→0补偿、原GWT、非管理CM及挡写通过；312 profile、914现场文件pins；新增四单元全停，journal保留 AWAITING_COMPENSATION_COMPLETION |
| Parallels | `/opt/yofc-emptycm-parallels-20261107-198548a1`；seed2615→apply3045→inverse3533；现有 Ubuntu26.04 ARM64、私有JRE184文件、生产ELF/真实UDS/systemd | 同一所选场景通过；496 profile、1097现场文件pins；新增四单元全停，journal保留 AWAITING_COMPENSATION_COMPLETION；Ubuntu正常stopped、Windows仍stopped |

VM 场景使用未改动的生产 Linux ELF，固定命令仍为生产30秒预算，Java receipt timeout 为固定 profile 的120秒；不会使用上轮 Native 派生测试副本。每阶段真实 agent 重开只读核对原 journal/当前 PID，既不重启 Kura，也不重发 execute。空 inverse 只到 AWAITING_COMPENSATION_COMPLETION，普通写继续阻挡。

收尾再次进入末次 agent 的 mount namespace 核对：两处 CapEff0 / NoNewPrivs1；Parallels 写专用只读探针返回 EROFS30，严格只读生效。Orb 原有 LXC 全局覆盖仍使探针可写，readOnlyEnforced=false，不称完整严格沙箱通过。停止时先停观察 agent，再停 inverse/apply/seed，保留原 journal SHA 不变；不把停服后的 pending journal 当作仍有 active unit 的重开验证。原 Orb 四服务仍原包 console26 / agent20、原 PID244516/244911/286620/268 active；无新 PG/中心/Vite/防休眠进程，18仓HEAD/branch及原dirty保留，无提交推送。

## 保留现场与限制

两个初始辅助类编译失败目录 878b0075 / 8ca3964c 从未启动或发布恢复请求，全部保留；新 v2 staging 曾在读取静态来源路径时失败，尚未创建现场。第一套 1eee4f2a 实际 apply78461 已证明原始备份0、12项配置和实际 DS，通过后发现静态初始快照缺少原 GWT 所需完整配置，HTTP404，不能记完整 Kura 验收。已正常停止、保留回执/CM/密钥/快照，无 inverse 请求；后续另建 v2 正常种植及全新 storage，没有重放该入口。

本轮证明原始空 managed CM 的独立认证逆向恢复。**没有发出空 CM 的 normal/health/completion 或普通写释放**，不把已清空、尚未配置业务的 Kura 当健康网关；补偿后的原控制台可用性也不以 apply 的登录结果冒充。fullRecoveryVerified/kuraRecoveryVerified/normalBootVerified 仍 false。部分失败 CM 的来源准入仍需独立核定，不因底层接受 old/new/absence 就记为 agent 已支持。

内部 pending：空/部分原 CM 后续正常启动、业务健康和明确完成策略；部分失败来源准入；完整生产组件/外部 PID 的恢复；0/±1 历史标量兼容、其他类型/元数据含图标及完整 GWT 编辑；其他未决恢复运维、Core 安装升级/bootstrap/启动策略交接与旧入口退役；NM/firewall/NAT/forward/远程流程/早启；一致 DB/制品/key 备份恢复、轮换、迁移和正式发行。外部 deferred 仍仅真实 NM/cloud-init/物理网络硬件、amd64、磁盘满/硬断电，两历史 evidence gap 单列。

源码/制品/原始证据的最终 SHA、18仓 HEAD/branch/全部工作树、准确停服与原 Orb 四服务状态见 `../evidence/empty-original-cm-inverse-20261107.json`；完整原始现场记录 `target/production-cm-recovery-20261107/`。不删除任何旧/新夹具、state、请求、密钥、备份、quarantine 或卷；无 reset/clean、提交推送或远端覆盖，不连接192.168.2.125。
