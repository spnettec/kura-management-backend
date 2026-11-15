# 空密码严格认证与纯动态 provider 的实际恢复

核定 `f7a8438c`，2026-11-07，接续 `239a42f7`。完整管理体系目标继续 **active**。本轮检查新动态元数据范围的真实配置应用、补偿和后续运维，不重放既有迁移、restart/reboot、交接取消或其他旧一次性验收。

## 实际发现及修复

新夹具只通过运行时 MetaTypeProvider 提供工厂定义，bundle 没有 metatype XML。首次上传预检发现：新的候选虽已规范化其他密码，空密码仍被判成 legacy，严格认证不能通过。真正的原因是 `CryptoServiceImpl.decryptAes(char[0])` 为兼容历史读取会成功返回空数组，而 `ComponentUtil.isEncrypted` 把这个成功返回误当成密文认证。因此空 Password 没有调用 AES-GCM 加密。

修复集中在 `ComponentUtil.isEncrypted`：输入是实际空密码时返回未加密，交由真实 AES-GCM 加密；已有有效密文仍保持原样。CryptoService 的兼容解密语义、XML 格式、上传策略、密钥选择及认证失败边界没有放宽。调用者的 Password 对象不被改写。

新增真实 Crypto/mapper/validator 合同覆盖标量、Password 数组、Vector、List 的空密码加密、认证、类型及重复处理不二次加密；错误密钥下的空密码密文在 authenticated 和 legacy 两种策略都拒绝。修复前重现失败，修复后最终 Core/API/crypto/XML/inventory/离线工具依赖 reactor **466**、bridge **55**、GWT **87**，共 **608**，零失败、错误和跳过。代码图谱按真实核心与 agent 子仓库查询，核心修改后已刷新；coverage 只按 best effort 解释。

真实 Kura 原始基线快照的审计报告含 3 项已认证及 3 项 legacy 密码值。基线读取仍由独立、显式的 legacy grant 授权，不自动修改上传策略。修复后的新上传候选为 26 配置、7 工厂、215 属性、6 项密码 **6/6 认证、legacy 0**。预检失败候选、原始基线和全部回执均保留，没有发布后重发或覆盖旧候选。

## 业务场景及证据边界

选择的真实 DS 工厂组件使用纯动态元数据，保存 int[]、有序字符串集合（含空字符串、首尾空格、重复值、逗号、反斜线、中文）、Long.MAX_VALUE 集合、XML/CDATA 字符串，以及两项密码集合（含实际空密码与首尾空格）。每次 DS activate/modified 验证实际值和密码解密，ConfigurationService typed API 再核对；不以文件存在或 opaque 制品当业务恢复。

受控 apply、inverse、guarded normal 各阶段的 provider 都额外发布带默认值的新属性和单例元数据，之后再修改服务属性刷新来源。真实 ConfigurationAdmin change count、DS 事件、typed view 和不存在的 guard 单例都核对：元数据可读，原值保持，默认属性不补种，新单例 CM 不创建。四种普通配置/工厂/快照操作在恢复闸门下拒绝，CM 不变。

流程为新 seed 9 → 受控 CM apply 2 → 独立 CM inverse 9 → 原始快照文件集完整还原 → 独立 guarded normal 9 → 实际 UDS 所选业务健康 → 独立授权显式完成 → 一次原 GWT 写入 → 无恢复请求的独立 ordinary JVM 写到 10、保存快照。三处 GWT 写入均由新增 DS 回调和实际 Kura 快照再次读回，检查类型、秘密值与 `secondDefault=gwt-after-release`；普通启动后重新核对真实 DS、typed 配置和秘密，并生成新快照，Native 另对普通启动快照做独立读回。ordinary provider 回到初始动态定义，原 GWT 不展示仅在受控阶段追加的 secondDefault，不能用普通启动截图代替其快照验证。原 GWT 用 admin/admin 登录，中文名称、PASSWORD 输入占位符和空默认字段均有 DOM/截图证据。

健康验证使用可运行的真实 RecoveryHealthCheck，检查实时 DS 和当前 typed view；标准 verifier 在回调前后比较整个 managed CM。仅明确授权的 Console self-owned singleton 例外按既有合同检查，不能忽略任意新增组件。显式完成通过新身份、新 nonce、固定健康/完成 grant 和 Java create-only 写释放回执；agent 核定 `COMPLETED_APPROVED_CHECKS`。完成前持续挡写，完成后合法的 CM/快照变更和正常停止不被误判成未完成恢复漂移。

这里证明的是 **administrator-selected-checks** 场景。`fullRecoveryVerified=false`、`kuraRecoveryVerified=false`、`normalBootVerified=false` 等全网关标记保持原合同；没有扩大为所有生产组件、硬件、外部 PID 或正式发行。独立 ordinary 使用另一个不带 `kura.recovery.request` 的 profile，不复用一次性恢复请求，不代表生产自动安装/切换 profile 已交付。

## 实际环境

| 环境 | 新现场 | 已证明范围及状态 |
|---|---|---|
| IDEA v4 | `.phase1-dev/dynamic-recovery-idea-v4-20261107-f4add841` | 284 inventory / 实际 283 bundles；seed74249→apply74510→inverse74868→normal75322→ordinary75636；完整所选流程、GWT 一次写、typed/DS/实际快照和无 agent 协调服务的独立 ordinary 已通过；310 profile pins，全部 JVM 已停 |
| OrbStack | `/opt/yofc-dynrec-orb-20261107-ad637e78` | 生产 ELF / 真实 systemd / 管理员 UDS；seed707874→apply708171→inverse708394→normal708771→ordinary709420；完整所选流程及 GWT 已通过；agent 多次重开不重启 Kura、不重发写；313 profile、737 现场文件 pins；新增6单元全部停止，journal保持 COMPLETED_APPROVED_CHECKS |
| Parallels | `/opt/yofc-dynrec-parallels-20261107-5540df7e` | 现有 Ubuntu26.04 ARM64，生产 ELF / 真实 systemd / 管理员 UDS；seed2613→apply3070→inverse3426→normal4426→ordinary6300；完整所选流程、GWT 一次写及独立 ordinary 10 已通过；497 profile pins 含私有 JRE184文件、922 现场文件 pins；新增6单元全部停止，journal保持 COMPLETED_APPROVED_CHECKS；Ubuntu正常停止、Windows仍停止 |

每处都是新 home/CM/state/请求/事务身份，只复用经过 SHA 验证的静态 bundle、工具、私有 JRE 和生产 agent ELF。VM 的远程通道、配置远程写及 firewall 均关闭，无新中心、gateway 或 mTLS 注册。Native 为真实 Rust Recovery engine 与 Java UDS 健康/完成链路，unit 由 test-only IDEA 协调器使用真实进程 PID 管理，完整 Kura 只从 IDEA UI 启动；它不是宿主 systemd 验收。隔离 home 延续 `kura.need.password.change=false` 以检查配置业务，生产 admin/admin 及首次设置密码后台闸门未修改，本轮也不新增首登验收主张。

Native 协调需人工/桌面工具启动 JVM，生产固定 unit 命令的 30 秒窗口不足。最终使用独立测试源码副本，只将该协调窗口改为 120 秒、调整该副本的协议 crate 路径并声明独立 Cargo workspace；原 agent 仓库和 VM 生产 ELF 均未修改，VM 仍验证生产 30 秒命令合同。差异、原始输入 SHA 和构建输出在 `native-harness-source-proof.json` / `native-harness-diff.patch`。Native 用该副本的一个真实集成测试通过，不把它算作未改动的生产二进制测试。

Parallels 初始及末次重开的 agent 都已实际核对 CapEff=0、NoNewPrivs=1，进入其 mount namespace 写专用只读探针返回 EROFS30，严格只读沙箱通过。Orb 原有 LXC 全局覆盖仍使探针可写，明确 readOnlyEnforced=false，不能称其完整严格沙箱通过。两处 JVM 的 systemd 资源约束不等于 agent 的严格沙箱。

Orb 浏览器使用已核对的既有 IPv6 地址，没有修改宿主或 VM 路由。原四服务继续保持原包 console26 / agent20、原 PID 244516 / 244911 / 286620 / 268。无新 PG、中心、Vite、防休眠进程；不连接192.168.2.125，不清理个人 ~/.kura-dev、其他活动 target/runtime 或任何旧 state/密钥/备份/quarantine/卷。18仓 HEAD/branch 和工作树收尾另存完整核对，无提交、推送、reset、clean 或远端覆盖。

## 失败现场与续接规则

最初编译失败的 `9b5ba317` 目录从未启动。第一套 `c4aa7f71` 的 seed71743 通过后，候选严格认证失败，未发布恢复请求，已停服保留。

v2 `d7f1a7c3` seed72975→apply73521：夹具少了 OCD 包导入，同时 UI 比30秒窗口晚约4秒。Java后来真实 CM_APPLIED/DS2，但 engine已 RECOVERY_REQUIRED，不记通过，全部停服保留。v3 `0a9c93d3` seed73679→apply73777→inverse74097：Java正/逆向及元数据闸门证明通过，补偿首次桌面点击没启动 JVM；核对无新 PID/phase marker 后才再次点击，实际启动仍晚于30秒窗口。原事务保持 RECOVERY_REQUIRED、全部停服保留，未重新执行其 prepare/execute 或清除闸门。

调试构建曾在403MB固定输入的只读指纹初始化耗时；CPU采样证明 SHA256 软件路径。冻结后核对只有空目录，无上传、计划、请求、unit命令或 socket，才停止该初始化，换优化构建。该证明及旧日志完整保留，不将观察超时当任务终止。最终v4使用全新夹具、改进后的 test-only 协调时限，未重放上述任一 seed/请求/入口。所有 token 只留私有 target/fixture，公开 evidence 不复制 token。

## 未交付范围

本轮关闭所选纯动态 provider 的实际受控恢复、补偿、所选健康/显式完成/独立普通启动范围；不是完整组件适用面完成。内部 pending 仍有历史0/±1标量兼容、其他类型/元数据边界含图标、GWT新增/删除/空元素完整编辑；生产组件、部分/空 CM、外部 PID 恢复及其他未决恢复运维；Core安装升级/bootstrap/启动策略与旧入口退役；NM/firewall/NAT/forward/远程流程/早启；一致 DB/制品/key 备份恢复、轮换、迁移及正式发行。

外部 deferred 仅真实 NM/cloud-init/物理网络硬件、amd64、磁盘满/硬断电；两历史 evidence gap 单列。既有 restart/reboot 和交接取消验收不重开；Parallels Java可用，软件 pending 不改写成缺环境。保留 NM、Kura 无 agent 独立运行、Java 独立 key/CSR、agent常驻及可选远程通道、中英、独立归属、UNKNOWN不自动重发、RECOVERY_REQUIRED挡写保现场等核定不变。

机读证据 `docs/evidence/dynamic-provider-recovery-20261107.json`；完整原始证据 `target/dynamic-provider-recovery-20261107/`。收尾更新父工作空间 Obsidian 知识库及 Tasks / Session Summaries / Knowledge Index，以官方 CLI 精确读回；完整目标保持 active。
