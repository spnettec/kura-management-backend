# 2026-11-06 固定普通配置后台交接核定

完整改造目标保持 **active**。本轮交付的是：管理员独立授权后，由 agent 后台事务停止受控恢复的旧 unit，启动预先配置的固定普通 Kura unit，并核对真实 JVM 与选定业务。上一轮由测试脚本直接启动 ordinary 的方式，已由本轮后台 prepare/execute 路径替代。安装器预配置、开机启用/接管、中心任务及后续 restart/reboot 生命周期仍待实现。

合同见 agent [ordinary-profile-handoff-v1.md](../../../kura-management-agent/docs/ordinary-profile-handoff-v1.md)，精确结果见 [evidence](../evidence/ordinary-profile-handoff-20261106.json)，日志/冻结输入/保留二进制位于 `target/ordinary-profile-handoff-20261106/`。

## 实现与边界

- 本地管理员新增 `recovery.ordinary.prepare/execute`，只收 transactionId/confirmToken。独立私有 grant 默认关闭，绑定父 completion 决定、固定第四 unit、JVM 参数、runtime 文件、有效 unit/drop-in 定义和短时授权。
- 普通 JVM 参数没有恢复请求，独立 profile ID，清除继承的 JVM 注入环境。原恢复请求/回执只保留，不删除、不重用。Java 格式/密钥策略由独立 ordinary grant 显式选择，不自动套用 normal/legacy 策略。
- 先持久 issued 意图，停旧三 unit，再校验源并只启动固定 ordinary 一次。新默认关闭 `kura.ordinary.verify` 读取实际 JVM PID、无恢复请求属性、框架等级、选定组件和真实 health providers；不增加 agent 或恢复 bootstrap 的 Kura 启动依赖。
- approved snapshot hash 是请求上下文及文件校验绑定，Java 原样返回此 hash **不证明该文件被加载**。Java 短时格式校验、源文件保留与实际业务断言分别核定。启动未就绪允许只读等待，ready 后业务失败立即失败，不能轮询到成功。
- 观察先持久化，create-only receipt 后进入 `ORDINARY_ACTIVE_APPROVED_CHECKS`，仅 `administrator-selected-checks` 范围；`kuraRecoveryVerified/normalBootVerified=false`。writer 与包装生命周期闸门识别该终态。
- issued 后中断不得自动重启；只有匹配持久终态回执可只读核定，否则 RR、停四 unit、持续挡写并保留现场。终态后的合法快照、配置升级及普通停服，不追溯否定历史决定。

## 当前输入的验证

| 范围 | 当前结果 |
|---|---|
| agent 主机 | 154 passed / 5 ignored；严格 Clippy、fmt |
| agent Linux ARM64 | 154 passed / 6 ignored；严格 Clippy，隔离源码/target 构建 |
| Java bridge | 52 passed，含默认关闭、严格字段、独立 profile、未就绪、重复/缺失/抛错/挂起 provider |
| 包装/生命周期 | 55 passed，含普通交接的待定状态挡写和新终态 |
| 显式 native | 1 passed，经 IDEA UI 启动完整隔离 Kura；真实原 GWT admin/admin 中文登录/9→11写入、新 JVM 重新登录保留11 |
| Orb | 真实 UDS + systemd 固定 unit/有效 drop-in + 完整 Kura，后台交接、DS/密码/快照写及 agent 重启不重放通过 |
| Parallels | 两 VM 实时核对为 stopped；Ubuntu 仍缺 Java/Kura，完整业务 deferred；不重放已闭环挂载/fsync入口 |

本轮没有改 Core/API；429 项旧 Java reactor 证据按版本保留，没有当成本轮重跑。React/原 GWT 代码和 Docker 构建未变，不重放旧一次性验收。193 个冻结输入与当前树一致，两仓图谱刷新成功；ready 不代表所有格式均有解析覆盖。

成功 native：`.phase1-dev/ordinary-handoff-v3-idea-20261106-c5f78735`。Count=11、工厂9、真实 DS/密码及普通快照写通过，test agent 退出后同一普通 JVM 继续运行，最终正常停止。原生 unit 协调器只负责 IDEA 运行的测试交接，不冒充 systemd。

成功 Orb：`/opt/yofc-ordinary-handoff-20261106-6d67ec3f`，284 项 runtime 文件 hash 验证。原控制台先9改11，随后 **仅通过本轮后台 ordinary.execute 启动**普通 unit，真实只读业务报告和 agent 重启后 PID 不变通过；新 JVM 原 GWT admin/admin 登录显示11。最后五个新 unit 全部 inactive/MainPID0/Result success，快照、state、授权、密钥、备份和 unit 均保留。Linux agent SHA256：`a46b8d94547cb8d06b9b3f139831ba3f78ab06215ebfeac67e306c702c479dcb`。

保留两次 native 未通过尝试：首套 `b793b387` 因未及时在 IDEA 启动进入 RR；v2 `fa5d1645` 的测试协调器被已结束工具会话的进程组回收，停在 STARTING。两份 Kura JVM 均已停止、请求/现场保留，无重发；v3 改用持续工具会话完成。不将失败夹具或轮询等待超时当业务通过。

## 收尾与剩余业务

18 仓 HEAD/branch 未改，原 dirty 与夹具保留；无提交/推送/reset/clean。原 Orb console26/agent20 四服务持续 active；Ubuntu/Windows stopped。本轮所有主机 JVM/driver 和 Orb 新 unit 已停止。

**pending**：普通 profile 的安装预配置/开机接管/中心任务与 restart/reboot 闭环；全组件/metatype/真实 health provider 适用面、部分/空原 CM 与外部 PID 权威；NM 协同、防火墙 NAT/forward/原 UI/远程流程/早启；key 多轮轮换、整库/制品一致恢复、旧系统迁移及正式发行。

**deferred**：Parallels 完整 Java/Kura、真实 NM/cloud-init 交接、物理有线与硬件、amd64、磁盘满/硬断电。两项历史缺失证据继续为 evidence gap。NM 保留、Kura 无 agent 独立可用、密钥/CSR 由 Kura 后端生成、远程/配置/防火墙默认关闭、UNKNOWN 不自动重发等既有核定保持。
