# Agent 接入受控 Kura 恢复事务（2026-11-05）

整体目标 active。本轮已接通 agent 的固定受控启动请求与真实 CM 应用回执；事务准确停留在 AWAITING_COMPLETION，不宣称完整业务恢复，kuraRecoveryVerified=false。

## 实现

新增可选 `AGENT_RECOVERY_STARTUP_CONFIG`，只接受本机管理员私有固定 JSON grant；必须同时使用已审核 Java validator。API 不接受 executable/profile/key/timeout/receipt 任意路径。prepare 固定 unit、原校验 key/mode/policy、目标快照及 hash、全新恢复 ID、请求内容 hash、profile 指纹和唯一回执路径。

执行前后重新核对输入与指纹，停止 allowlisted unit、恢复前快照备份流程沿用原实现（快照制品按原字节备份，CM API 备份由 Java AES-GCM 产生）。落盘目标后先持久记录 controlled_issued 意图，再 create-only 发布请求，启动固定 unit；核对 Java CM_APPLIED 的 ID/hash/动作列表、备份 hash、bundle 级别账本以及请求/目标/实际 unit 稳定性。

成功只进入 AWAITING_COMPLETION，保存 CM 证据指纹，不写终态成功回执，不删上传件、请求或释放全局写闸门。agent 重启核对原证据，不启动/重放 Kura。受控意图一旦发出，普通快照补偿不能恢复 CM：执行失败、显式 rollback、崩溃或待完成证据/unit 漂移停 Kura，保留 RECOVERY_REQUIRED 和所有现场，等待后续 CM 级补偿/明确完成。

实际完整运行集暴露 UDS 通用25秒超时缺陷：后台 blocking 恢复不能随客户端 future 取消。已将 recovery.prepare/execute/rollback 独立设为600秒等待，超时返回 OUTCOME_UNKNOWN，只查询原事务、禁止重发；其他 RPC 时限不变。新真实 UDS execute 实际73.042秒成功返回，覆盖原25秒缺陷。

合同与限制见 `../../../kura-management-agent/docs/controlled-startup-v1.md`。profile 现行核对指定文件，不是 systemd 全部有效配置的远程证明；祖先路径/动态 unit override 仍由本机管理员控制。完整395MB运行集需要256KiB grant、单文件256MiB/总2GiB界限，覆盖实际119MB脚本及67MB Web2 bundle。正式安装生成 profile、独立服务 UID/key 授权尚未交付。

## 验证

- Rust 宿主和 Orb 各125通过、3显式 opt-in ignored，严格 Clippy均通过。4项新增恢复故障合同覆盖非终态/闸门/重启不重放、缺失/错误回执、profile漂移/发布后崩溃、CM备份漂移；既有测试无回归。
- 显式 native 集成1项通过（136.23秒），真实 Java校验/完整284清单Kura/283 bundle及DS业务；由原生IDEA启动，macOS test-only unit adapter只协调实际PID，不冒充systemd。原GWT admin/admin中文登录、Count=2、agent落盘snapshot_1通过。unit adapter不是发行部件；后续UDS超时修复由实际Orb验证。
- Orb v2 实际UDS + systemd 新隔离完整Kura +新agent通过。旧值keep/remove=9先种植，恢复后业务事件仅 business.keep:2:true，remove不激活，真实Crypto解密成功，旧单例属性清除、核心snapshot写挡住。Java校验26配置/7factory/5认证Password；CM回执保留fullRecoveryVerified=false/businessHealth=UNVERIFIED。原GWT登录/Count=2/snapshot_1通过。
- 两处26条旧CM AES-GCM备份经真实Java mapper/crypto只读解密审计，旧keep/remove=9及stale属性都保留。无明文临时文件。
- agent实际重启后Kura PID不变、upload继续挡住；执行期间实际状态查询耗时0.00045秒。随后显式停止尚未完成恢复的Kura，再重启agent，真实进入RECOVERY_REQUIRED；8个目标/请求/备份/账本/上传文件逐字节不变，业务事件不变，无重放。所有新进程最终停止。

## 保留现场与精确运行状态

IDEA fixture `agent-cm-idea-20261105-58de8ea8` 已停止，私有journal保留AWAITING_COMPLETION；不能普通重启假装完成。Orb首次`/opt/yofc-agent-cm-20261105-851934c7`遇25秒TIMEOUT，最终journal VALIDATED，新进程均停，未重发；修复后全新v2为`/opt/yofc-agent-cm-20261105-3cf24043`，最终RECOVERY_REQUIRED符合故障断言，新进程均停。

实际systemd状态：两个新agent unit inactive/dead、MainPID=0；两个新Kura unit failed/failed、MainPID=0（JVM SIGTERM处理后退出143，夹具未配置SuccessExitStatus=143，失败标志保留，未reset-failed）。不把此标志写成正常退出通过。当前测试不证明正式Kura发行unit的停止合同或完整系统沙箱。原Orb agent20/console26三服务及phase2-agent仍active、原包未升级。Ubuntu/Windows VM stopped；Parallels缺Java/Kura的受影响验证继续deferred。

首套native grant边界不足（83KB grant/真实大bundle）在发布请求/建立事务前拒绝；界限已据真实运行集修正，失败日志保留。初次Linux构建缺workspace manifest属暂存夹具问题，已补并在全新v2构建复验。旧夹具/state/key/backup/quarantine/卷均未删；无提交推送reset/clean/远端覆盖、无192.168.2.125连接。

证据 `../evidence/controlled-recovery-transaction-20261105.json`，完整日志、前后18仓状态、源码/制品SHA、两套Linux源/构建、native结果、原GWT截图在 `target/controlled-recovery-transaction-20261105/`。

## 继续实施（pending）

下一必要工作是 CM公开API补偿与明确完成合同：证明桥接/实际业务健康，核对相同恢复ID/快照/备份/应用回执，显式完成后才能释放普通写；失败须通过CM补偿完整恢复，不能仅恢复快照或改journal。继而接入外部PID当前Rust网络/防火墙权威和全组件适用面/工厂/属性合同。

完整原目标继续保留：防火墙适配/NAT/远程流程/早启、restart/reboot闭环、一致备份恢复、身份轮换运维和正式发行。真实NM/cloud-init、物理有线/硬件、amd64、磁盘满/硬断电等外部deferred单列，不能把未实现产品功能改成缺环境。
