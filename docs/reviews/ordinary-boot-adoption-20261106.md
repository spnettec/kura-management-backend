# 显式普通启动策略接管与 Core 维护互锁（2026-11-06）

完整目标继续 **active**。本轮实现本地 root 管理员审核的原生 systemd 启动策略接管，以及 Core 安装维护的整阶段互锁。它不启动、停止或重启 Kura，不代表中心任务或完整 Kura 重启业务已交付。见 [机器证据](../evidence/ordinary-boot-adoption-20261106.json) 与 agent `docs/ordinary-boot-adoption-v1.md`；私有完整日志在 `target/ordinary-boot-adoption-20261106/`。前轮 372d897d 的已闭环一次性入口不重放。

## 实际变更及边界

agent 新增 root-only `ordinary-boot prepare/execute/status/cancel/reconcile` CLI，先于 identity/DB/UDS 初始化；尚未接到 UDS 或中心 API。prepare 校验独立 seal 授权、完整固定输入、实际有效 unit/drop-in、目标 active/MainPID 和旧入口 inactive/PID0，返回可审阅 before/proposed actions 与新 ID/token。只支持原生 enabled/disabled、精确 `WantedBy=multi-user.target`，旧 static 可无 Install；别名、Also、间接触发/依赖、generated/SysV/runtime/masked、未审核 drop-in/重载漂移及 agent 启动依赖拒绝。非原生入口没有被静默退役。

execute 在持久计划和每一步意图后，先 disable 声明旧入口，再 enable 固定目标，均无 `--now`。回读实际状态、同一 PID、原生 wants 链接及目录同步后才写 create-only receipt、独立 confirmed policy 和终态。任何未知/部分结果保留 RECOVERY_REQUIRED 挡住受管写，不自动补偿或重发；reconcile 只观察已有结果。仅未发出动作的计划可取消。

当前 transaction 与 confirmed policy 分离：取消后续新计划保留原确认策略；后续计划必须继续包含先前退役入口，换目标须明确退役前目标。缺 journal 却有历史/回执/confirmed 不是新安装。终态历史不因后来合法停服而倒算失败。status 当前返回最新事务，尚无独立 confirmed-policy 的组合响应。`BOOT_POLICY_COMMITTED` 仅 `administrator-reviewed-native-boot-entries`，`bootVerified=false/kuraRecoveryVerified=false`。

Core 私有可选 `/etc/kura/ordinary-boot-policy.json` 只登记真实 state root；Kura 运行不依赖 agent。prepare/execute/cancel/reconcile 先持有固定 `/run/lock/kura-ordinary-boot.lock`，再持有对应系统变更锁。Core preinst/postinst 在真正改动前以同一顺序加锁，由监督进程持有整个维护阶段，重入子脚本一次执行；close-on-exec 防止后台孙进程泄漏锁。保留 abort/failed-upgrade 路径。未启用策略的独立 Core 安装仍可用；未决/损坏策略挡住维护，确认其他普通 unit 后拒绝静默重新启用 `kura.service`，取消后续计划也不能抹掉此保护。

agent writer 与包装维护同步识别上述状态，校验私有精确匹配回执/confirmed，孤立残留证据挡写；有 boot policy receipts 时 purge 拒绝，显式验证交接仍 pending。默认 NM、admin/admin 及首次设置后台闸门、中英文、配置审核 PID、firewall 默认关闭、独立中心归属没有改变。

## 本轮验证

| 层次 | 当前候选结果与精确范围 |
| --- | --- |
| Rust | 宿主 166 passed/6 ignored；Linux 166 passed/7 ignored；两处严格 Clippy、宿主 fmt；新 boot 8 项包含状态/漂移/断点/核定/取消/确认策略保留 |
| Python/Core | agent 包维护宿主及 Linux root 各 56；Core 宿主 15（12 新 guard/监督锁 + 3 启动器），Linux guard 12；真实锁持有、一次执行、无后台继承、私有/损坏/孤立证据与取消后确认策略 |
| 原生 IDEA/GWT | 新 `boot-adoption-idea-20261106-34ef49fb`；完整隔离 Kura 经 IDEA UI 启动，独立冷 ordinary PID57572、真实 DS/密码/普通 snapshot/Count9；当前 agent observer 显式 1 passed，schema2 provider 缺失后可用；原 GWT admin/admin 中文读9，observer 退出同 JVM。macOS 不执行 Linux boot adoption |
| Orb | 新 `/opt/yofc-boot-adoption-orb-20261106-9f5a3fee`；实际 Core deb 启动脚本和新 review ELF，完整普通 Kura 冷启动，真实 provider/DS/密码/snapshot 与原 GWT Count9；实际 CLI disable 旧/enable 普通，PID475556 和 InvocationID 均不变；prepared/committed/后续取消三阶段 Core 包 guard 均在副作用前拒绝；observer 退出无 Kura 重启 |
| Parallels | 新 `/opt/yofc-boot-adoption-prl-20261106-41652913`；当前 review deb 仅解包，不安装替换。实际 root CLI 在 ProtectSystem=strict、ProtectHome、PrivateTmp、NoNewPrivileges、空 capabilities、AF_UNIX 下完成接管/取消与 Core guard；输入 EROFS/hash 不变。正常重启后 boot ID 改变，目标仅启动一次、旧入口未启动，历史 transaction/confirmed 字节不变；目标是明确非 Java/Kura 的 sleep stand-in，只证明原生启动策略/OS reboot 原语 |

Orb 使用私有 mount namespace 把 `/etc/kura` 路由到新夹具，真实系统管理器与原生 unit 链接仍参与；Parallels 用 systemd 私有 BindPaths。两处宿主 `/etc/kura/ordinary-boot-policy.json` 均保持原先不存在。没有对现网 Core 运行 dpkg 安装/升级，完整 Core 生命周期验收仍 pending。Parallels 没有 Java 或已安装 agent 包，只有本轮解包的 review ELF，完整 Kura 业务仍 deferred。

本轮无中心/前端协议变化，不重复旧首设、TLS、迁移、恢复完成和 Docker 一次性验收；Java/bridge 生产代码未变，前轮 Java429/bridge53 等是历史证据，不充作本轮重新执行计数。

## 制品与证据保留

| 制品 | SHA256 |
| --- | --- |
| agent review deb `0.1.0~review20261106.boot1_arm64` | `c62ebac34b3e1cb8f732f5f4949c8afdd0071585dca7117ec51c31c90fdf72c5` |
| Linux agent ELF（maxGLIBC2.39） | `9539e67ae605fe09132b066e65f08017b64bbb494f2a6cb30340e3596088bde2` |
| Core review deb `6.0.0~review20261106.boot1-1_arm64` | `c07ce2fc8f7260d6a3b2f8c98473775619beaac3822f7f4a40b57303878e25ab` |
| Core payload preinst / postinst | `6134308edbb448c01aaa4f01bf01ee58ca9ecdb28c116a496f6d384230978636` / `00c6b518848ea5485de6f38ce89c1386b2416022cd9d881a75163d9f5073e3e1` |

199 冻结输入与最终现场完全匹配；原协议 crate SHA 不变。Core 包在隔离源码副本由 Maven 构建，payload controls 与真实当前源一致，启动脚本过滤值已核对；仅 review 制品，未升级原 Core/agent。最终 docs/README 的范围说明在构建后补充，不是 ELF 输入。Core 复制构建的辅助 `.changes` SCM 标签沿用外层 console hash，不能当 Core HEAD；明确 deb 版本及 payload SHA 为核定依据。

中间诊断/失败日志保留：Core 第一锁测试字符串夹具转义已修正并通过最终15；首次 Parallels 重启前只读 inventory 因未安装 agent 包中止，尚未发 reboot，随后如实记录缺包；最终记录器重复 status 键保留 raw 并单独派生 v2 字段修正，未重放动作。Git 核对改用与 baseline 相同的 untracked-files=all，避免目录折叠误报文件缺失；199输入及所有原dirty路径均实际保留。

## 最终运行状态

全部新原生 JVM/observer/driver 已停止。Orb 新 seed static/inactive/PID0，ordinary 与 retired disabled/inactive/PID0，全部夹具和回执保留；Parallels 新目标/旧入口同为 disabled/inactive/PID0，探针已停，Ubuntu 与 Windows 均正常 stopped。测试 ordinary 在成功策略证明之后由本轮编排显式停止并禁用；这不是自动回滚，历史 confirmed 仍 COMMITTED、最新独立跟进事务是 CANCELLED。不要把跟进取消说成原策略已取消。

Orb 四原服务仍 active：console244516、center2 244911、agent286620、phase2-agent268；原包 console26/agent20 未变。18 仓 HEAD/branch 与全部旧 dirty 路径保留；无 reset/clean/提交推送、无远端覆盖、无旧夹具/个人运行目录/密钥备份清理，无旧一次性入口重放。Core 与 agent 子仓索引已刷新，相关路径 metadata_match；best-effort 并非全解析覆盖。

## 剩余交付边界

**pending**：完整安装/bootstrap/发行升级与显式策略交接、非原生冲突启动入口迁移；中心 ordinary/boot/restart/reboot 任务和双语交互；实际完整 Kura VM 重启后的业务核定；全组件/metatype/真实 domain providers、部分/空原 CM 与外部 PID；NM 协同、防火墙适配/NAT/远程/早启；多轮 key、整库与制品一致备份恢复、旧系统迁移及正式发行。

**外部 deferred**：Parallels 完整 Java/Kura、真实 NM/cloud-init/物理有线与硬件、amd64、磁盘满/硬断电。两处旧 evidence gap 保留原核定，不制造替代历史证据。当前 unit/opaque/stand-in proof 都不等于广义 Kura 业务恢复完成。
