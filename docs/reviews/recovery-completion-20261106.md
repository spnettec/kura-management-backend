# 2026-11-06 明确完成、写入释放与独立普通启动

完整管理体系目标保持 **active**。本轮交付管理员明确批准范围的完成协议，接通 agent/Kura 两层写入闸门，并在真实完整 Kura 验证释放后的原 GWT 写入及独立普通启动。范围固定为 `administrator-selected-checks`，所有回执继续 `kuraRecoveryVerified=false`；全组件、外部 PID、正式 profile/发行等仍按下表保留。

最新知识库交接 `Experience/2026-11-06-Kura明确完成与双闸门释放及普通启动续接-dfcf15f7.md`、Tasks.md、Session Summaries.md 已经官方 Obsidian CLI 保存并完整读回核对。

## 实现

configuration API 导出版本为 1.7.0，新增独立可选 `RecoveryCompletionService`；bridge 导入下限为1.7。`kura.management.recovery.completion.enabled=true` 才注册/声明写能力，agent 另需私有固定 `AGENT_RECOVERY_COMPLETION_CONFIG`，均默认关闭。无恢复请求时 bootstrap 仍无操作，Kura 可不装 agent 独立启动。使用合同见 agent `docs/recovery-completion-v1.md`。

管理员 `recovery.completion.prepare/execute` 只接受独立 ID/token。授权精确绑定 health grant SHA、所选检查范围、健康最大年龄和完成有效期。准备阶段核对完整 normal/health/sidecar/source/活跃 MainPID，保存私有源日志及原授权；执行先持久化意图和不可覆盖决定，再发送一次桥接写。Java 在原准入 JVM 内重新核验 CM 全集、组件和真实业务，核对源/实际/属性计数与自配置追加集合后，先持久化固定私有释放回执，再开放核心普通写。清除系统属性不能绕过闸门；重复尝试不能覆盖回执。

agent 核对 typed reply、Java 回执、owner/mode/精确字段，并独立同步文件及父目录、重读字节与元数据。响应丢失只核定既有持久决定，不重发释放；缺失/不一致证据保持 RR 并停止事务的三个 Kura unit。Java 决定已提交而 agent 终态未写的崩溃窗口可只读核定并补齐 agent 完成回执。完成后合法 CM/快照变化、普通停服不再被当成未完成恢复漂移。

终态是 `COMPLETED_APPROVED_CHECKS`：`writeReleaseCommitted=true`、`agentWritesBlocked=false`，原健康/full/normal 的广义验证标志不升级。`kuraReleaseReplyVerified` 单独记录是否获得可信成功响应，回执核定不伪造这个观察。终态表示历史批准决定，不表示永久当前健康。普通 agent writer/包维护识别该终态，旧证据仍保留。

## 当前验证

证据根 `target/recovery-completion-20261106/`；[核定 JSON](../evidence/recovery-completion-20261106.json)。所有系统夹具均为本轮新建，未重放上一轮健康、恢复或 mount 入口。

| 层次 | 结果与实际范围 |
| --- | --- |
| Java | Maven reactor 429 passed（configuration90、offline tool30），bridge另48。五个独立 JVM 验证成功、发布失败、业务失败、计数边界错误、过期；严格参数、默认关闭和可选引用通过。 |
| Rust/Python | 宿主149 passed/5 ignored，Linux149 passed/6 ignored，双端严格Clippy及宿主fmt通过；宿主packaging54。显式完整native测试另1 passed。ignored不计通过。 |
| IDEA | 只经 IDEA UI 启动完整284 inventory/实际283 bundle新运行集；正向2→逆向9/密码→文件集→受保护普通加载→所选健康→明确完成全部通过。原GWT admin/admin中文登录，释放后实际将Count9改为11并生成新快照，agent重开维持完成状态。随后独立无恢复请求、无agent测试进程的普通Kura启动，真实DS/密码及普通snapshot写入通过，原GWT重新登录显示11。 |
| OrbStack | 现有VM内新systemd/UDS完整同流程通过；原GWT实际写11，完成后两次真实agent重启分别核对原JVM和新的独立普通JVM，均无Kura重放。普通启动保留11、真实DS密码和snapshot写入通过。现有Orb沙箱全局覆盖限制未改变。 |
| Parallels | 现有Ubuntu26.04 ARM64新探针1项通过：实际 `ProtectSystem=strict`/只读application挂载，O_RDONLY文件与目录fsync成功，写入被EROFS拒绝；原字节SHA不变、unit停止保留。这是新系统调用合同；实际确认仍无Java，完整Kura业务保持external deferred。 |

当前185条冻结输入与验证源码一致。Linux ELF SHA256 `bb336a66b078f6a67532fc190227b160e4dab9285d17820f586591f875311a47`；宿主release及test ELF与Linux ELF均保留精确hash。三个真实子仓库重索引，所查七条代码路径metadata_match，仍为best-effort覆盖。

本轮测试准备纠正了测试快照路径/调用计数断言、两个Clippy冗余转换；系统Python3.9缺tomllib后改用现有Homebrew Python，packaging54通过。Orb首次收尾脚本在服务停止后出现Path写法错误，已用新只读核对脚本修正，未重新执行任何业务事务；初始日志保留。上述均不冒充业务失败或已通过的旧证据。

## 收尾状态及普通启动边界

native `.phase1-dev/recovery-completion-idea-20261106-e7beb134` 的所有新JVM和测试驱动已停止；最后journal为 `COMPLETED_APPROVED_CHECKS`。Orb `/opt/yofc-completion-20261106-66d9dcbd` 新agent/forward/inverse/normal/ordinary五unit均inactive/MainPID0/Result=success。新构建夹具 `/var/tmp/yofc-completion-build-20261106-0fbd94c7` 保留。原Orb console、center2、agent及phase2-agent仍active，原console26/agent20包未升级。

Parallels新探针 `/opt/yofc-completion-sync-prl-20261106-d6065b11` 及其state/原文件保留，探针inactive/MainPID0/Result=success；Ubuntu与Windows最终正常stopped。18仓HEAD/branch不变，原全部dirty/untracked保留，无提交/推送/reset/clean。旧state、请求、key、备份、quarantine、卷和个人运行集未清理。

本轮普通启动使用**新的普通profile/unit**，没有删旧请求、没有重用旧normal入口。IDEA UI和本轮测试编排显式启动这个普通profile；agent完成协议本身不自动改写生产unit或激活普通profile。正式的固定普通unit交接、启停/重启/reboot策略、维护行为和产品发行仍需交付，不能将本轮测试编排当成这些产品接线。

## 未交付台账

| 分类 | 范围 |
| --- | --- |
| pending，下一续接 | 正式固定普通unit/profile交接与生命周期接线，完成后的可运维启动策略；全组件/metatype适用面、业务health provider产品接线及覆盖。 |
| pending | 部分/空原CM、外部PID权威恢复；Kura防火墙适配、NAT/forward、原UI/中心远程流程/早启；restart/reboot远程任务闭环。 |
| pending | key多次轮换、数据库/制品一致备份恢复、旧系统迁移及正式发行。 |
| external deferred | Parallels缺Java/Kura的完整业务；真实NM/cloud-init、物理有线/硬件、amd64、磁盘满/硬断电。两处历史evidence gap保留。 |

保留NM/Netplan当前方向、agent常驻及Kura独立性、Kura生成私钥/CSR、默认admin/admin和首次设置后台闸门、console中英文、配置/firewall默认关闭、身份独立和普通注册不抢占归属、UNKNOWN不重发及RR挡写保留现场等既定约束。下一轮先读最新知识库、Tasks/Summaries、continuation顶部覆盖和本文件，实时核对工作树后继续未交付业务。
