# 部分失败 CM 来源的独立补偿核定（2026-11-07，b9eb0ad3）

完整目标保持 **active**。本轮承接 d1994c05，只闭环“Java 受控应用已经部分修改 CM、未形成 CM_APPLIED 时，允许显式授权的独立 inverse”。原始空 CM 的既有验收不重做。本轮不是完整网关恢复、正常启动、健康完成或正式发行。

## 产品变化

agent 的独立 inverse 固定配置新增 `allowPartialSource`，默认 false。此授权不来自 RPC 请求；原启动配置、validator/key、unit、请求路径与固定文件仍需逐项固定。原 full-source 分支继续要求完整 CM_APPLIED 证据，历史 journal 的 source_application 序列化保持兼容。

部分来源分支要求：原 controlled 请求已经持久发出、状态 RECOVERY_REQUIRED、原 unit 停止、候选及原始权限/owner 不变；原 Java 失败 receipt 精确为 schema/restore.id/status/fullRecoveryVerified/businessHealth 五项；完整 plan 绑定原 snapshot、AES backup 和 AES intended；bundle ledger、原请求及固定 profile 也必须完整且未漂移。缺失、重复、unsafe、权限错误、符号链接、hash 漂移或撤销 opt-in 均拒绝准入/发布。

新证据存为 source_partial，与 source_application 严格二选一。它没有 applied hash 或虚构的动作计数。prepare 与 status 明确返回 sourceStatus=RECOVERY_REQUIRED；即使 inverse 成功，原 cmApplication 仍为 null。执行前和重开时重核原证据，不重发失败的正向请求。inverse 仍只到 AWAITING_COMPENSATION_COMPLETION，普通写闸门不释放，全部原始及补偿材料保留。合同见 agent `docs/cm-compensation-v1.md`。

Rust 产品改动限 controlled_startup / recovery 及新来源模块；新增合同和显式忽略的真实 IDEA 测试。Java 产品、GWT/React、协议、身份/首登默认策略未改。本轮隔离 Kura 的原控制台测试继续使用专用测试 home 的既有首登例外，不更改产品默认闸门。

## 验证与实际业务

- 宿主全量 198 passed / 0 failed / 9 ignored；随后增加的显式 IDEA 测试另行执行 1 passed，未以更早全量次数冒称重跑。最终源码 fmt / 严格 Clippy 通过。
- Linux 新独立源码树离线 release 构建、5 个部分来源合同及严格 all-targets Clippy 通过；108 个构建输入与当前工作树匹配。没有复用旧 ELF 代表新产品变化。
- 新默认合同覆盖 default-off、不推断应用成功、独立身份及持久重开不重放、停止来源、精确 checkpoint 的十类拒绝、prepare 后漂移、撤销授权和旧 full-source 形状兼容。
- 实际顺序为本机合同/原浏览器 → IDEA 完整隔离 Kura → OrbStack → 现有 Parallels。每处 inventory 284、实际 bundles 283；每处正常新 home 原 GWT admin/admin 登录并保存初始证据。

故障注入只在全新 apply JVM 中对真实 Equinox CM 的 updateIfDifferent 做测试插桩：第一项实际写入并以 CM API 回读 generation=2，第二项再抛出受控 IOException。没有以伪造 Java 回执代替真实写入。原 bootstrap 报失败，SCR/业务启动级别保持关闭，agent 将该 JVM 停止，RECOVERY_REQUIRED 持续挡写，原 cmApplication=null，无 cm-applied 侧车、无 apply 业务激活。

独立 inverse 的真实 AES-CM backup 进一步证明原现场为“业务 generation=2、另一 count=9”的新旧混合。原 CM backup 两项均为 9；补偿 applied dump 两项恢复为 9。三处都用真实 mapper/crypto 解密审计；完整 CM 原记录精确验证、真实 DS 使用原秘密解密并激活、foreign 非 Kura 配置 value77 保留、普通 snapshot/update/rollback 拒绝、原 GWT 再登录读回 9/密码脱敏均通过。

| 环境 | seed / inverse PID | 固定 profile | 停服后现场文件 pins |
|---|---|---:|---:|
| IDEA | 89297 / 90002 | 310 | 699 |
| OrbStack | 717845 / 718350 | 311 | 696 |
| Parallels | 2598 / 3688 | 495 | 883 |

IDEA failed apply PID 为 89774；两 VM 的失败 apply 已由 agent 确认停止，未额外声称采集其瞬时 PID。

IDEA 三个完整 JVM 均由 IDEA UI 启动，Rust 直接生产引擎由显式测试适配器协调实际 PID；没有启动本机 agent daemon，不冒称生产二进制 daemon 验收，也没有修改生产 30 秒 unit 命令窗口。两 VM 使用本轮新 ELF、实际 main UDS/systemd 和原 30 秒命令合同。正向及 inverse 各只执行一次；每次 agent 重开均核对原 journal、来源证据与实际 Kura PID，不重放。

Linux ELF SHA256：`539244e2c73a36a6ccc8c8fd200311d87bf4ed61f3aae4fc4263498bcad80689`。macOS release SHA256：`89a55d60e17a2b53bd5fd1cc511eb5c8869edd24bf0974772e6ad5c24a18cad2`。两者均为开发构建，未安装到原系统或发布新 deb。

Parallels 在末次 agent 重开后再次实测 CapEff=0 / NoNewPrivs=1 / EROFS30，含私有 JRE 184 文件逐项校验。Orb 全局 LXC override 仍导致 readOnlyEnforced=false，不把 requested systemd 属性当有效只读隔离。

## 保留现场和最终状态

证据根为 `target/partial-cm-source-20261107/`，机读核定为 `docs/evidence/partial-cm-source-compensation-20261107.json`。

- Native：`.phase1-dev/partial-cm-idea-20261107-13a3bba7`；全部新增 JVM/测试 driver 已结束。
- Orb：`/opt/yofc-partialcm-orb-20261107-fd48c3cf`；本轮 seed/apply/inverse/agent 四单元全停，原四服务原 PID 244516/244911/286620/268 及 console26/agent20 包保持 active。
- Parallels：`/opt/yofc-partialcm-parallels-20261107-a8b5c144`；本轮四单元全停；Ubuntu 正常关机、Windows 保持 stopped。
- 三处 journal 都保留 AWAITING_COMPENSATION_COMPLETION，fullRecoveryVerified / kuraRecoveryVerified / normalBootVerified 均 false；没有发出 normal/health/completion/写释放。

最初最小故障合同的 wildcard boot delegation、单元对 prepare 后漂移的错误期望、审计 helper 泛型/collector 对 optional 字段及 JVM 短暂退出状态的假设、独立 Linux 源码 staging 缺 sibling workspace 等测试辅助问题已纠正并保留记录。没有由此重复运行任何 seed、正向或 inverse 请求，也没有改产品放宽测试。本轮未新增中心/PG/Vite/防休眠进程。旧夹具、state、密钥、备份、quarantine、卷和个人运行目录全部保留；18 HEAD/branch/旧 dirty 保留，无提交推送。

## 继续交付边界

内部 pending：空/部分原 CM 及部分失败来源之后的 normal/业务健康/明确完成策略；完整生产组件和外部 PID 恢复；0/±1 标量、其他类型/元数据/图标及完整 GWT 编辑；其他未决恢复运维、Core 安装升级/bootstrap/启动策略交接和旧入口退役；NM/firewall/NAT/forward/远程/早启；一致 DB/制品/key 备份恢复、轮换、迁移和正式发行。只关闭本轮“部分失败来源的独立 inverse 准入”，不关闭这些业务。

外部 deferred 继续单列真实 NM/cloud-init/物理网络硬件、amd64 实际目标、磁盘满/硬断电；本轮 CM IOException 不是这些故障的验收。两项历史 evidence gap 保留。Parallels Java 已具备，不用缺环境代替软件 pending。下一批先核对最终 SHA 与 18 仓完整 status，不执行本轮 target 中任何既有一次性入口。
