# 2026-11-06 受保护普通启动后的实时业务健康核验

完整原目标保持 **active**。本轮交付可选的实时健康观察及真实 Kura/UDS 验证，仍未实现明确完成、写入释放或无闸门普通启动。前轮 `guarded-normal-startup-20261106.md` 的历史验收不重放；本轮用全新夹具验证新增接线。

最新知识库交接 `Experience/2026-11-06-Kura实时业务健康与自配置追加授权续接-4723acd6.md`、Tasks.md 与 Session Summaries.md 已经官方 Obsidian CLI 保存并逐份完整读回核对。

## 实现及边界

核心新增可选 `RecoveryVerificationService`、`RecoveryHealthCheck`，configuration API 导出版本为 1.6.0；原配置服务和 Kura 启动仍不依赖 agent。恢复 bootstrap 仅在经过认证的普通加载准入后注册验证器。桥接方法 `kura.recovery.verify` 默认不声明，需显式设置 `kura.management.recovery.verification.enabled=true`。agent 另需私有固定 `AGENT_RECOVERY_HEALTH_CONFIG`，默认关闭。详见 agent `docs/recovery-health-v1.md`。

本机管理员使用独立 `recovery.health.prepare/execute` ID/token。原正常启动证明、完整源日志及制品指纹、实际 MainPID、nonce、agent 实例和桥接 epoch 均被核对。CM 完整集合在业务检查前后比较；所选源 PID 必须注册，所选业务检查必须唯一注册且在单个有界 worker 的总计一秒期限内通过。缺失、重复、抛错、挂起、旧会话响应和来源漂移均不能获得成功。匹配身份的类型化失败观察保留 `verified=false`；不确定/失败执行停止三个恢复 Kura unit 并保持 RECOVERY_REQUIRED，不自动重发。

首套真实普通启动发现源集合 26 条配置之外新增 `org.eclipse.kura.web.Console`。没有放宽全局比较或修改旧数据库。CM-only 诊断只克隆 framework storage，经公开 ConfigurationAdmin API读取，原 318 个文件 SHA 不变；26 条原配置逐值一致，新增记录含六个实际属性。随后增加默认空的 `additionalSelfPids` 明确授权：新增 PID 必须是权威适用面接口确认的 SELF_CONFIGURING、视图归组件所有、不是 factory，且完整 CM 记录与实时自配置视图一致；其余新增/未知配置或旧值漂移仍失败。没有默认 Console 例外，也不忽略自配置记录。

成功只生成 `SELECTED_BUSINESS_VERIFIED` 私有不可覆盖回执并进入 **AWAITING_HEALTH_COMPLETION**。`selectedChecksVerified=true`；`ordinaryWritesBlocked=true`、`normalBootVerified=false`、`kuraRecoveryVerified=false`、`awaitsExplicitCompletion=true`。有效期字段描述过去观察，不表示永久实时健康。重开只核对既有证据和同一运行 JVM，不重新调用检查或重启 Kura。

## 本轮验证

证据根 `target/business-health-completion-20261106/`；成功 native 为 `v3/`，Linux 当前输入/制品与结果为 `v2/`。前两套失败现场及 v1 输入保留，不能当成成功或重放入口。

| 范围 | 结果 |
| --- | --- |
| Java | 当前 Maven reactor 428 passed；含 configuration 90、offline tool 29；桥接另 46 passed。验证严格字段、默认关闭、可选引用、CM 漂移、自配置显式授权、业务检查缺失/重复/异常/挂起及持续挡写。 |
| Rust | 宿主 145 passed / 5 ignored；Linux 145 passed / 6 ignored；两端严格 Clippy 通过，宿主 fmt check 通过。未将 ignored 记作通过。 |
| IDEA | 只通过 IDEA UI 启动全新隔离完整运行集，inventory 284、实际 283 bundles。正向 Count=2、CM 逆向两个工厂实例 Count=9/密码解密、快照集合及受保护普通加载通过；原 GWT admin/admin 中文登录显示 Count=9。显式 live 测试 1 passed。 |
| 实时健康 | native 与 Orb 均经实际 Java UDS bridge 核验 3 个源 PID 和真实 DS/ConfigurationService/CM/密码检查。26 条源配置逐值一致；显式允许且完整视图匹配的 Console 1 条，实际合计 27 条/193 属性。框架级别 5、MainPID/nonce/来源一致，成功后 upload/普通 Guard 仍拒绝。 |
| OrbStack | 现有 VM 内全新 systemd Kura/agent，完整正向→逆向→文件集→普通启动→健康流程通过。agent 实际重启后 AWAITING_HEALTH_COMPLETION 证据核对通过，Kura MainPID 不变，upload 仍阻挡。既有系统沙箱覆盖限制不变，不将请求的沙箱属性当成有效严格沙箱验收。 |
| 失败保留 | 首套 native 健康严格比较失败，持久化失败报告并停止所有新 JVM，保持 RECOVERY_REQUIRED。native v2 等待 IDEA 启动超时，未启动恢复 JVM，保留请求/RR。v3 使用新独立夹具和更长测试协调窗口，未重发两套旧 execute。 |

当前 176 条组合生产输入 hash 与验证输入一致；API/bridge/bootstrap JAR 随 native/Orb inventory 固定。Linux ELF SHA256 `74bd7c3b5fc938912130d80d2667d69d6623cc5f6fd06c66c662f0050d0c6092`。完整指纹及精确状态见 [证据 JSON](../evidence/recovery-health-20261106.json)。三个真实子仓库已重索引；所查六条新增/受影响代码路径 metadata_match，仍是 best-effort 覆盖。

## 收尾状态

成功 native `.phase1-dev/business-health-v3-idea-20261106-8b7b5cd2` 的新 JVM 已停止；最后 journal 为 AWAITING_HEALTH_COMPLETION。失败 native `business-health-idea-20261106-66c5dca1`、`business-health-v2-idea-20261106-919a8c68` 均停止并保留 RR。旧请求/token/密钥/备份/日志/CM storage 均保留。

Orb 新夹具 `/opt/yofc-business-health-20261106-b8f5273d` 的四个 unit 均 inactive/MainPID0/Result=success；最后 journal 为 AWAITING_HEALTH_COMPLETION。主动停服发生在成功重开核对之后，不声称停服后仍能通过 active-unit reopen。构建夹具 `/var/tmp/yofc-business-health-v2-build-20261106-63a28f89` 保留。原 console、center2、agent 及旧 phase2-agent 仍 active，原包26/20未升级。Ubuntu/Windows Parallels 均 stopped。

18 仓 HEAD/branch 不变，原所有 dirty/untracked 路径保留；没有提交、推送、reset/clean。未连接禁止的网关，未修改主网络，未清理旧运行集、个人目录或卷。

## 继续实施台账

| 分类 | 未交付范围 |
| --- | --- |
| pending，下一步 | 明确完成协议、最终核验与写入释放/回执崩溃窗口、无闸门普通启动的独立安全路径；不能把本轮所选健康检查当作释放依据已完整交付。 |
| pending | 全组件适用面、部分/空原 CM、外部 PID 权威恢复；真实业务检查 provider 的产品接线与完整覆盖。 |
| pending | Kura 防火墙适配、NAT/forward、原界面/中心远程流程/早启；restart/reboot 远程任务闭环。 |
| pending | 多次 key 轮换、数据库/制品一致备份恢复、旧系统迁移及正式发行。 |
| external deferred | 现有 Parallels 缺 Java/Kura，未重放已闭环旧 mount primitive；真实 NM/cloud-init 交接、物理有线/硬件、amd64、磁盘满/硬断电。已有两处历史 evidence gap 保留。 |

保留 NM/Netplan 当前方向、agent 常驻和 Kura 独立性、默认 admin/admin 与首次设置闸门、console 中英文、默认关闭配置/firewall、UNKNOWN 不自动重发及 RR 挡写等既定要求。下一轮先读本文件、最新知识库/Tasks/Summaries及 continuation 顶部覆盖，实时审查 HEAD 和全部工作树，再从未交付业务继续。
