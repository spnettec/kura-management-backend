# 网络 UNKNOWN 业务结果核定增量

日期：2026-11-01。接续中心任务加密存储。阶段 2–5、配置写入及快照写入仍未完成。
契约见 `network-outcome-reconciliation-v1.md`；本增量不改变注册归属或 Kura Web 认证。

## 已实现行为

agent 查询已有 UNKNOWN 时认证解密原执行计划，核对命令 UUID、操作、幂等键及完整请求 SHA，
再读取网络事务日志证明历史结果。prepare/takeover 只有请求绑定且已提交才能成功；启动恢复回滚的
prepare 核定为 FAILED/CONFLICT。confirm/rollback 还要求业务日志中持久化的原接受命令 ID，
其他 actor 或没有命令标记的旧日志不能冒充回执。writer 正忙、日志缺失/损坏、非终态及
RECOVERY_REQUIRED 继续 UNKNOWN。查询不再次调用 prepare/confirm/rollback。

终态按事务 UUID 另行归档，避免下一笔事务遮蔽旧结果。SQLite 在同一事务中更新加密回执并追加
`execution_resolutions`；中心保留租户、enabled、当前 epoch/dispatch epoch 校验，并追加一次
`task.reconcile.network`。普通迟到回执另记 `task.reconcile.receipt`。
React 增加中英文历史核定说明，并核对任务、操作与业务终态绑定；说明当前状态须另行刷新。
能力新增 `task.network-outcome.v1`，Protobuf 和 Java 生产代码未改。

## 本轮验证

证据均相对本仓库；新清单为 `docs/evidence/network-outcome-20261101.json`。
清单包含86项源码/配置/文档、7项制品及58项证据，已逐项SHA读回核对；旧清单保持历史。

| 范围 | 实际结果 | 证据 |
| --- | --- | --- |
| macOS agent | 57 项通过 | `target/network-outcome-agent-unit-final2-20261101.log` |
| console / protocol | 8 / 6 项通过，默认数据库 2 / mTLS 3 ignored | `target/network-outcome-console-unit-20261101.log` |
| 显式 PostgreSQL / mTLS | 2 / 3 项通过；独立数据库 schema 清理 | `target/network-outcome-postgres-20261101.log`、`network-outcome-mtls-20261101.log` |
| Rust fmt / 两端构建 | 通过 | `target/network-outcome-{agent,console}-{fmt,build}-20261101.log` |
| React 类型/生产构建 | 通过，约 970 kB 主包仍有大 chunk 提示 | `target/network-outcome-react-build-final-20261101.log` |
| 默认浏览器 | 17 通过，0 skipped/unexpected/flaky | `target/network-outcome-browser-final-20261101.json` |
| IDEA / 原控制台 | 新 PID28460，实际 admin 登录并见设备状态；HTTP31、React live1 双语读取通过 | `target/network-outcome-idea-gate-20261101.json` |
| Linux 独立 agent | 57 项及离线独立构建通过 | `target/network-outcome-linux-agent-{unit,build}-20261101.log` |
| Linux 真实结果核定 | 最终 23 项通过，保留前序失败和执行记录 | `target/network-outcome-linux-live-20261101.json` |

新单元覆盖请求/计划绑定、准备后恢复回滚、确认决策恢复、回滚 actor 隔离、终态历史归档、接管请求 SHA、
损坏密文拒绝、SQLite 并发原子结案，以及组合查询不重放。浏览器补充成功、失败 prepare、错误任务注记
不展示及中英文切换；截图 `network-outcome-{uncertain,reconciled}-ui-20261101.png` 已查看。
这些界面桩不冒充真实 Linux 任务；真实 IDEA 图片 `network-outcome-idea-react-20261101.png` 也已查看。

IDEA 从 Run 启动完整隔离 Kura，原控制台重新实际登录；HTTP 和 live 浏览器顺序执行。
Stop 后 ps 确认退出，当前 `kura-runtime/inventory.json` 281 项 SHA 全部匹配，没有替换/清理运行集。
本轮没有改 Java，不将历史 Java 测试算作新执行。

## Linux 现场及故障范围

IDEA 门槛之后，既有编译 VM `kura-package-validation-20261029` 使用独立 staged agent 源码及
带 SHA 的 protocol crate；13 项 staged 输入与清单匹配，不依赖 sibling source，57 项及 build 通过。
新 Linux arm64 debug agent SHA 为 `8db9e0d900e1b38a3941c9627ab499d8e5d875da0f59668153e8b7afd62ec18a`。
源码、协议及制品核对在 `target/network-outcome-linux-build-20261101.json`。
本段未重打 agent deb，也未部署新的 Linux 中心。

实际联调使用既有 `yofc-management-network-20261101`，新身份/夹具
`/opt/yofc-network-outcome-20261101`、gateway `network-outcome-gateway-20261101`。
只复制已确认网络元数据作为新夹具基线，仅授权 dummy `yofc-test0`，Kura 缺席。
原 agent、配置、身份、网络业务日志和已结束的旧包夹具均保留。

真实临时 MTU 候选从 1500 到 1468。停止新夹具后，仅移除其已完成任务的 SQLite 回执并置 RUNNING，
原加密计划及业务日志保持；重启由正常启动恢复执行真实回滚，SQLite 留 UNKNOWN。
恢复结束后才将中心对应任务注入 UNKNOWN，真实 gRPC TaskQuery 核定该 prepare 为 FAILED/CONFLICT。
随后新命令再次准备临时 MTU 并明确回滚；同样注入回执损失，查询核定原 rollback 为 SUCCEEDED。
这叫“真实动作后的回执损失注入”，不是硬断电或精确崩溃点测试。

独立 Linux ptrace 验证工具 `scripts/trace-exec.c` 记录所有被跟踪线程/子进程的 execve/execveat 尝试，
不记录参数或环境；它不是发行内容。正常退出、错误退出、fork 子进程、失败 exec 的 4 项探针通过。
启动恢复与后续查询用不同观测窗口；两次结果查询窗口的 exec 尝试均为 0，网络业务日志 SHA 不变。
最终 10 条原执行记录完整保留（9 SUCCEEDED / 1 FAILED），两条核定审计各一次；重连没有追加执行或核定。
只读管理仍可用，两项中心任务 input/result 继续为密文。

完整日志/窗口及最终独立检查在 `target/network-outcome-linux-receipts-20261101/` 和
`target/network-outcome-linux-final-check-20261101.json`。最终新夹具 stopped-data-retained、gateway disabled、
运行候选消失，dummy MTU1500，初始文件、地址、路由及原 agent PID8391 全部匹配。
Parallels Ubuntu26.04 与 Windows 均 stopped，没有新开 VM。

## 保留的失败及检查修正

- 最初 agent 新子模块方法可见性和 console fmt 未整理，已修复；另一项测试错误要求并发查询立即同为终态，
  已按 writer 忙时允许 UNKNOWN、随后必须终态的真实契约修正。生产锁保护未减弱。
- 首轮浏览器纯 UI 桩未触发事件流 open，16 通过/1 失败；修正桩后完整 17 通过。误复制的
  `network-outcome-stale-report-copy-20261101.json` 明确保留为旧报告，排除证据清单。
- 源码 staging 首次系统 Python 缺 tomllib；使用已安装 Python3.15 重试成功。
- strace 官方包下载停滞、备用下载失败，未安装；改用本地构建且经过上述 4 项探针的 exec 观测工具。
- Linux 首轮宿主 Python3.15 拒绝旧开发证书的缺失 AKI，未发网络任务；新夹具停服/disabled。
  保留身份和空日志，显式有条件续接并使用此前通过校验的系统 Python，没有关闭证书验证。
- 第二轮检查过早把暂时 UNKNOWN 当作普通请求终态。跟踪开销下网络请求晚到成功；检查改为等待明确终态，
  首批 3 条记录和正常恢复回滚均保留，未删 state 或重放原命令。
- 后续首次全基线比较捕获 eth0 原 global IPv6 SLAAC 地址延迟返回；该窗口 exec=0，所有业务日志未变，
  后续完整初始基线恢复。原失败、before/after 与此差异保留，**不声明该窗口的整个系统状态始终不变**。
  后一窗口先等待 networkd/SLAAC 收敛再观察，全基线保持。Netplan apply 对系统网络的收敛影响仍是后续
  发行/物理网络验收边界。最终继续精确保留 6 条已有执行及第一条核定，再完成剩余验证。

对应日志为 `target/network-outcome-linux-live-{first,resumed,continued,final}-20261101.log`。
两次前序恢复有正当系统写入，不能把整轮称为零写入；只有明确标记的结果查询窗口没有 exec。

## 图谱与剩余范围

只刷新真实 console/agent，聊天 MCP 仍 Transport closed，官方 CLI 可用。agent 724 节点/3095 边，
4 个新增/修改关键 Rust 路径均 metadata_match、无记录解析缺口。console 的实际刷新与逐路径覆盖见
`target/network-outcome-graph-console-final-20261101.txt`、`network-outcome-coverage-console-20261101.json`。
SQL/配置仍有 3 partial / 3 unusable，不能据 ready 宣称完整解析。
静态 trace 的宏/同名方法候选边存在过度匹配（含并非本路径执行的 commit/execute），不能作为
“无执行”的证明；精确源码、单元和上述实际 exec 窗口才是本次依据。

确认提交及 takeover 的业务核定目前有单元证据，本轮 Linux 仅覆盖 prepare 恢复回滚与明确 rollback。
旧无命令标记日志、RECOVERY_REQUIRED 和证据不足的 UNKNOWN 仍需后续人工可审计恢复流程。
配置 update、snapshot save/rollback、key 轮换/正式中心包及阶段 2–5 继续 pending。
新 agent 包真实 Linux Kura 组合、旧 revision5 升级、26.04 新版回归尚 pending；
完整 OrbStack 文件系统沙箱、amd64、物理有线和硬断电保留 deferred。
未 reset/clean、提交、推送或修改旧远端，未动原 target/runtime 或个人 ~/.kura-dev。
