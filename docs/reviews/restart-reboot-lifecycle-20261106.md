# 固定 Kura 重启与主机 reboot：实施及宿主验证

核定 ID `a09e95d7`，2026-11-06。完整改造目标保持 **active**。这是 restart/reboot 的本地实现和首层验证进度，**不是系统交付闭环**；不得替代后续 IDEA → OrbStack → 现有 Parallels 的真实验证。此前 `9104c438` 普通交接回执丢失证据保留，不重放。

## 已实施

- 协议追加 23–28：固定服务 status、restart.prepare/restart、reboot.prepare/reboot、cancel；仅执行/取消接受严格 `SystemControlAction`（字段 14），禁止 caller unit/path/shell。原 1–22 保持。
- 本机 root 安装授权：固定有效 systemd 声明、unit/drop-in/env 文件 SHA；限定 Kura unit、system.slice、control-group 和有限 stop timeout。restart/reboot 独立授权，远程默认关闭。可选固定 ordinary profile + private snapshot 提供选定业务检查；没有该配置则明确 process-only。
- `seal-system-control` 和包内 `configure-system-control`：只读封存实际声明、create-only 新 grant；保留旧 env/grant/审计，不自动启动或启用 unit、远程通道。中断 marker 挡启动/维护。示例两个动作均关闭且文件 SHA 为占位值。
- native 持久 PREPARED/ISSUING/AWAITING_BOOT/VERIFYING_APPLICATION/COMPLETED/CANCELLED/RR。issued binding 先于固定 restart 或唯一 reboot timer；受理不算成功，断连不算成功。重开仅观察新 boot ID、固定 PID/InvocationID/cgroup，不重发系统动作。
- 可选只读 schema 2 ordinary 业务观察：固定 profile/nonce/approved snapshot、当前进程、start level、选定 DS/唯一 provider/check。尚未 ready 有界等待；ready 的业务失败即 FAILED，不重复检查或 restart。只报告选定检查，`kuraRecoveryVerified/normalBootVerified=false`。
- 私有原命令决定、原配置、完成回执和后续 history，完整 protobuf wire SHA 绑定 ID/operation/key/TTL/action token。query 不运行 systemctl/observer/repair/replay；缺失/损坏/孤儿继续 UNKNOWN 并挡写。并发核定 SQLite resolution 只一次。
- 观察器取得 OS 锁后重读 journal，避免将锁前旧 ISSUING 覆盖并发完成的事务。注册 Kura 即使同 UID 也不获得管理员系统操作；网络等未决事务及其他 UNKNOWN/RUNNING 写阻止新系统动作，cancel 只结清自身未执行计划。
- agent/Core 包维护检查同一份纯证据 guard（精确函数一致性另有记录），不要求运行 agent；未结清或损坏拒绝，purge 保留系统任务现场。
- 中心 migration 0018、admin/CSRF/归属/首次设置闸门、fresh capability、独立重启权限和派发复核。24–28 绑定当前连接 epoch，未派发旧队列在换连接后 EXPIRED；已派发变 UNKNOWN，仅查询原任务。执行固定 TTL 5 分钟，其余 30 秒，过期仍可核定已完成事实。input/result 加密，prepare token 非 admin 脱敏，核定审计一次。
- 中英文面板不依赖 Kura 在线，审阅固定 unit、快照/检查和启动标识，明确勾选；进程、主机、业务分开呈现。仅保存意图 key/operation/task ID，丢失提交响应按只读 intent 找回；404/UNKNOWN 保持挡写，不创建替代 POST。详情原命令证明标签同样限定 scope。

详细使用合同：agent `docs/system-control-v1.md`；OpenAPI 和生成 TS 类型已更新。证据根为 `target/restart-reboot-lifecycle-20261106/`。

## 已验证与局限

- agent 整套宿主测试在观察器竞态增补前为 **186 passed / 9 ignored**；最终受影响 native 套件 **10 passed**，包括实际 SQLite 关闭/重开、模拟 boot 身份改变、并发原回执仅一次、确认/漂移/权限/孤儿与已 ready 失败检查。系统调用均为明确 fake fixture，**没有真实重启主机或 Kura**。不将两个数字合并成一次整套结果。
- console **24 passed / 11 ignored**，另 mTLS fixture 3 ignored；protocol **11 passed**。本轮新 PostgreSQL 两项实际通过：旧 epoch 未投递队列过期、已投递只核定；过期原命令输入/结果加密、proof 绑定和一次审计。全部历史 PG 套件本轮未重跑。
- 包装宿主 **65 passed**、Core pre/post guard **17 passed**。新 configure 测试为 sealer stub；真实 root sealer/systemd/sandbox 仍须 Linux 实证。Core 两脚本内的 guard 与 agent 函数精确相同，见 `guard-parity.json`。
- 浏览器新 **10 passed**，仅本轮新 mock 场景：中英、明确确认、Kura 不可用、提交响应丢失后原 intent、reload 后 UNKNOWN、cancel、独立权限/RR、原命令详情正反例。截图只写本轮证据根。未重放旧 mock 的固定截图输出或旧一次性业务入口。
- 两仓严格 Clippy、fmt、UI typecheck/build、Git diff check 通过；Vite 仍提示既有 bundle 体积。新源码尚无 Linux ELF/deb；前轮 v3 制品不能代表本轮协议/实现。

## 精确运行状态

本轮仅启动新的私有 PostgreSQL 和 Vite mock 服务，均已停止，数据、密钥、schema/日志和截图保留。PG fixture `.phase1-dev/restart-reboot-pg-20261106-d6c28151`，私有 socket `/tmp/yofc-sc-pg-9284f24f`，port 16646；Vite 19663 无 listener。本轮没有启动新的中心、agent、IDEA JVM 或 VM，没有发布真实系统控制任务。

OrbStack 已恢复命令响应。原四服务保持 active/PID：console 244516、center2 244911、agent 286620、phase2-agent 268；未升级、重启或修改原包。Ubuntu `8061d57e-0120-42ec-a9a9-5799b436c4eb` 与 Windows `f077f2f9-0a5c-4347-8037-04214e00ca9b` 都 stopped。前轮所有旧夹具、故障 failed/92、state/key/backup/quarantine/volume 保留。18 仓 HEAD/branch 和旧未提交改动保留，无 commit/push/reset/clean。

## 下一步与未完成范围

1. 补真实 HTTP 准入/脱敏/意图及 capability/epoch 负例，检查本轮最终冻结源码；以新路径生成 protocol crate/agent 候选，禁止复用旧 crate SHA 冒充。
2. 新完整隔离 IDEA Kura/原 GWT admin/admin、真实 DS/密码/快照与只读 ordinary 观察；启动 JVM 必须经 IDEA UI，独立运行集，不复跑旧 seed/恢复入口。
3. 既有 OrbStack 新私有 fixture：真实中心 → mTLS → 固定完整 Kura restart 一次，进程/InvocationID 换代、真实选定业务和原 GWT，必要的本轮回执丢失原命令核定。原四服务不动，**不 reboot Orb VM**。
4. 既有 Parallels 严格 sandbox 下新本机授权/默认关闭及实际一次 reboot task/new boot/agent 重连核定；该 VM 无完整 Java/Kura 时只能证明 system/process，不能借 stand-in 宣布 Kura 业务恢复。最后正常停止两 VM 和本轮私有进程。
5. 每个系统阶段完成才追加证据/台账。当前这些是内部 **pending**，不是环境 deferred，也不是已通过。

完整目标其余 pending 仍包括未决交接运维、Core 完整安装升级/策略交接、完整组件/metatype/provider/部分空 CM/外部 PID、Java 离线恢复与真实业务适用面、NM/firewall/NAT/远程流程/早启、一致 DB/制品/密钥恢复、迁移和正式发行。外部 deferred 单列：Parallels 完整 Java/Kura、真实 NM/cloud-init/物理网络/硬件、amd64、磁盘满/硬断电等。保留 NM、Kura 无 agent 独立运行、身份归属、默认账号/首次设置、配置/防火墙默认关闭及 UNKNOWN/RR 合同不变。
