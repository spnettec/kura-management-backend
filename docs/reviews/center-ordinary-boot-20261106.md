# 中心原生启动策略任务与真实沙箱核定（2026-11-06，770604cf）

完整管理体系目标保持 active。本轮在 42b31189 本地启动策略合同上，实际交付可选管理 UDS、中心任务及中英文审核界面；没有重做迁移或重放旧验收。最新证据根为 `target/center-ordinary-boot-20261106/v2/`，同级父目录保留首候选、失败、数据库和构建日志。机器台账见 [对应 evidence](../evidence/center-ordinary-boot-20261106.json)。

## 实际变化与授权边界

- 新协议操作 15–19 分别是 status、prepare、execute、cancel、reconcile；仅 status 是可重试读。operation 原编号及 protocol-major 1 保留。迁移使用新增 `0016_ordinary_boot_tasks.sql`，未覆盖旧迁移。
- agent 只有配置私有 `AGENT_ORDINARY_BOOT_REQUEST` 才声明 `ordinary.boot.v1`。固定文件指定已审核 unit、封装授权和退役入口；中心/UDS 不能传路径、unit、命令或 grant。prepare/execute 再查当前输入和原生状态。默认 sandbox 和远程开关语义不变，示例 drop-in 不自动安装。
- 管理 UDS 允许 root/admin 首帧进入上述五方法；已注册 Kura 角色即使与 admin 同 UID 也一律拒绝。变更共用在线写互斥及 Core/系统变更锁，status 不等待长写锁；同步领域操作在 bounded blocking worker 内执行。四种变更沿用长写超时与 UNKNOWN 处理。
- 中心全五操作要求 admin、CSRF、当前归属及新鲜 capability，dispatch 再查。execute/cancel 的载荷只有规范 `boot-UUID`、确认 UUID；其余操作拒绝 action。任务输入/回执走现有加密、持久准入和幂等链，非 admin 任务视图隐藏 prepare token。
- 双语面板先读状态，再显示目标、退役、动作、grant digest 和期限并明确勾选执行。刷新只保存原 prepare task ID，授权从认证 API 读取，token 不落浏览器存储。UNKNOWN 不自动重发或创建替代计划；取消后续未执行计划与先前 confirmed 策略分开显示。
- agent 的 default-off/unbound 本机运行、Kura 无 agent 独立使用、Java 后端生成密钥/CSR、保留 NM、admin/admin 与首次改密后台闸门、中英文、配置审核 PID 和 firewall 默认关闭均保持。

`BOOT_POLICY_COMMITTED` 只证明管理员审核的原生启动 enablement；不 start/stop/restart Kura，`bootVerified=false`、`kuraRecoveryVerified=false` 始终保留。后续测试显式停禁测试目标属于收尾，历史 confirmed 策略仍保留，不称回滚。

## 最终候选与验证

| 层次 | 当前证据及精确范围 |
| --- | --- |
| Rust agent | 最终 v2 宿主 170 passed/6 ignored，Linux 并行 170 passed/7 ignored；宿主和 Linux 严格 Clippy、fmt；真实首帧 default-off 与注册 Kura 拒绝回归 |
| 中心/协议 | console 17、protocol 9；真实 PostgreSQL 显式 6；HTTP/TLS 准入合同 8；严格 Clippy；迁移 0016/读写重连分类/token 加密与脱敏/UNKNOWN 写闸门 |
| 浏览器 | React typecheck/build；mock 4；最终真实 HTTPS→中心→mTLS→agent 1，双语、明确确认、保留 confirmed、原幂等 ID 与 reload 只读 |
| IDEA | 新 `.phase1-dev/center-boot-v2-idea-20261106-debdf663`，284 inventory；通过 IDEA UI 启动 seed11049、普通11974；实际 DS/密码/普通 snapshot9、schema2 provider 缺失后就绪、observer1，退出 observer 后同 JVM，原 GWT admin/admin 中文 Count9；均停止保留 |
| OrbStack | 新 `/opt/yofc-center-boot-v2-orb-20261106-f836fe29`，实际完整 Kura 普通包启动 PID514361；DS/密码/snapshot9、独立观察后启动实际 agent；中心任务 execute1、follow-up prepare/cancel、原 task 幂等回读、首次管理 UDS；agent514647→515172 后 Kura PID/Invocation、journal/confirmed 字节不变；原 GWT Count9 |
| Parallels | 新 `/opt/yofc-center-boot-v2-prl-20261106-39c831c8`，当前 deb 仅解包；真实 main/UDS 在 ProtectSystem=strict、ProtectHome=yes、PrivateTmp、NoNewPrivileges、空 capability、AF_UNIX 下验证 default-off/unbound、固定字段拒绝、同 UID 注册 Kura 五方法拒绝、真实 mount EROFS、策略/取消/agent重启持久。目标明确非 Java/Kura sleep stand-in，不替代完整 Kura 验收 |

本轮 Java/Core/API 无新增生产改动，不重跑既闭环的 Java429/bridge53/Core/包装验收；这些只按原版本 carry-forward，不能记成本轮新计数。没有新增 Docker 改动或重复镜像验收。

最新真实中心网关 `orb-center-boot-ba0bd098` 共 6 项任务全部 SUCCEEDED：status2、prepare2、execute1、cancel1；execute ID `7a6a20e8-dbb3-4601-a4e9-a6b83cf00975`，prepare ID `d55364f7-ad39-4387-83df-38dadf78b22d`。浏览器只有一次 execute 请求，重放同幂等键返回原 ID。独立 `dev-gateway` 的准入合同 UNKNOWN 是未 dispatch 的保留夹具，不能当成真实系统未知执行或删除它来获得通过。默认 admin 记录仍 `must_change_password=true`；测试使用独立 review-admin。

## 发现、失败和保留证据

第一候选的真实 Parallels UDS 首帧返回 EOF：方法分派已接通但连接初始 allowlist 漏了前缀。失败在任何 boot 动作前，目标从未 start/enable。已修 initial admission 和长写分类，以新 `centerboot2` ELF、全套并行测试、新 IDEA/Orb/Parallels 夹具复验；旧 339ededf/候选1全部停止保留，不重试。

首次浏览器辅助程序再次登录同一 admin，触发正常单会话撤销，只留下未执行 prepare。改为原浏览器会话核对 admin、辅助程序只登录 viewer/operator；明确取消原计划后新建。随后 Node request 需要显式 fixture CA，失败未产生新动作；仅给测试子进程 `NODE_EXTRA_CA_CERTS`，未关闭证书验证。旧日志均保留。Python严格 TLS 的旧 leaf 缺 AKI/SKI，另建 server-v2 leaf，原证书保留，未跳安全告警。

第一隔离数据库曾因新增迁移误用已有 0010 编号而失败；改用真正新 0016，并新建 management_boot_v2，失败 management 数据库保留。无旧库修复或 drop。旧 async 单元固定 50ms 观察在并行负载下误读 VALIDATED，改为有界等待真实 STOPPING且每次 status≤200ms；最终 Linux 未串行化。

intentional PREPARED 期间周期 network.expire 的共享闸门会记录 CONFLICT，未执行网络变更；日志保留。这一提示可在后续改善，不能用关闭写闸门消音。

## 制品与源码对应

| 制品 | SHA256 |
| --- | --- |
| protocol 0.1.0 本地 crate | de2780ad9edd164528a2e921e58fe4106bac96880558b2e81cc488692276dcb9 |
| agent Linux arm64 centerboot2 ELF | 466c3e8c927b87ab9470bd5cf5506f7039bc2ff460b72796ea87f901d761df63 |
| agent `0.1.0~review20261106.centerboot2_arm64` deb | c79bc7469ab1e35ec8738e9af1d308cde339557ab1d54469ae15d6b025f5922a |
| agent staged source manifest | edb6f71947f7357be84a0b119576590098f1520f3afa6ac2605a267e4776f74b |
| 实际宿主中心 immutable ELF | 8d6829efcf92a50aa3a7b7c8a681e854ecdb3d5eb79b84094803994c0021cdc4 |

冻结输入 258 项与当前源码逐值一致；agent source manifest 37 项（包含单独 staged Cargo）逐值一致；Linux构建源独立包含按 SHA 命名的 protocol vendor，无 sibling 源依赖。Core 包脚本沿用已核定输入，实际隔离启动没有升级原 Core。仅 review 制品，未发布 registry/deb 或替换原服务。

18 仓 HEAD/branch、原 dirty 路径全部保留，增量见 worktree-before/after.json。agent/console 真实子仓重索引；相关 Rust/TS/proto metadata_match 为 best-effort；0016 SQL parser unusable，已直接读源并用真实迁移/PG验证。不索引父工作空间。

## 最终运行状态

- 宿主中心 PID84715、独立 PG cluster、Vite 和 keep-awake 已正常停止；新数据库、失败数据库、PKI、密钥与 `.phase1-dev/center-boot-20261106-9a6d19a0` 全部保留。
- 两套新 IDEA JVM/observer 均停止；两套新 Orb seed/ordinary/agent 停止，测试 ordinary/retired disabled，journal/confirmed 原字节保留。Orb VM继续 running。
- Orb 原四服务仍 active，PID244516/244911/286620/268，原 console26/agent20 包。新策略 Core marker 仅位于 mount 隔离目录；主机 `/etc/kura/ordinary-boot-policy.json` 原样不存在。
- Parallels 第一失败与第二通过夹具均停止保留，当前新三 unit 均 PID0，target/retired disabled；Ubuntu 和 Windows 最终均 stopped。prlctl 正常关机等待期间未开始系统 shutdown，观察升级已结束后请求 guest systemctl poweroff，最终 prlctl stop 返回成功；未强杀或 reboot。
- 没有 reset/clean/提交/推送/覆盖远端，未访问 192.168.2.125，未清理旧夹具、备份、quarantine、卷、个人 `~/.kura-dev` 或运行 runtime。

## 尚未交付：pending 与 external deferred 分列

**内部 pending，不能降为环境 deferred：**

1. boot 领域决定已持久、加密 execution receipt 尚未持久时进程退出，尚无绑定原 command 的业务核定 resolver。当前保留 UNKNOWN；policy reconcile19 本身不解除原中心任务，不能从当前 enablement 推断原任务成功。下一轮优先补这个故障窗口与公开核定链。
2. 完整 ordinary-profile 交接中心任务、restart/reboot 远程闭环、完整 Core安装/bootstrap/升级、明确策略交接及非原生启动入口退役；实际完整 Kura VM/网关 reboot 后的业务核验。
3. 完整组件/metatype/domain providers、部分/空 CM 与外部 PID 的业务适用面；NM 协同、防火墙适配/NAT/远程流程/早启。
4. 多轮 key、数据库和制品一致备份恢复、旧系统迁移、正式发行；本轮没有用 opaque 制品/unit/所选 DS 检查替代全业务恢复。

**外部 deferred：** Parallels 缺完整 Java/Kura；真实 NM/cloud-init、物理有线和硬件、amd64、磁盘满/硬断电及其他原外部条件。旧两项 evidence gap（console-deb-20261102-binding-generation/agent-tests.log、transfer-ui-browser.json）原样保留，不能造替代历史证据。

最新知识库交接为 `Experience/2026-11-06-Kura中心启动策略任务与真实沙箱续接-770604cf.md`，Tasks/Session Summaries 同步核定。下一轮从本核定继续，不重放任一已使用的 seed/prepare/execute 或旧一次性入口。
