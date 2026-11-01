# 网络事务未决结果核定 v1

日期：2026-11-01。接续执行前日志及中心任务加密存储。适用新的 Rust agent；
中心/agent 的注册及 mTLS 身份继续独立于 Kura Web 认证。

## 证据与状态

原任务仍由 HTTPS admission、Protobuf 命令、SQLite 加密计划与执行前 RUNNING 日志关联。
agent 重启把没有回执的 RUNNING 保留为 UNKNOWN。中心只发送已有的 `TaskQuery` 查询，
不再分发原副作用命令；查询不调用 prepare/confirm/rollback，也不调用 Netplan 或 ip。

对已有 UNKNOWN，agent 认证并解密原执行计划，核对命令 UUID、完整请求 SHA、操作码和幂等键。
再读取本机网络事务当前日志，或按原 prepare 命令/事务 UUID 定位的终态归档。
确认和显式回滚在持久化业务决策前同时写入对应命令 ID；其他 actor 的完成不能冒充原命令回执。
系统 writer 正忙时查询保留 UNKNOWN，不等待或抢占写锁。

| 原操作 | 可核定条件 | 任务回执 |
| --- | --- | --- |
| prepare / takeover.prepare | 原命令 ID、完整候选/接管请求 SHA 一致，事务 COMMITTED 且有确认决策 | SUCCEEDED，携带已记录 COMMITTED |
| prepare / takeover.prepare | 同上请求绑定，事务 ROLLED_BACK 且没有确认决策 | FAILED / CONFLICT，携带 ROLLED_BACK |
| confirm | 事务 ID、令牌、持久化确认命令 ID 一致，事务 COMMITTED 且有确认决策 | SUCCEEDED |
| rollback | 事务 ID、持久化回滚命令 ID 一致，事务 ROLLED_BACK 且没有确认决策 | SUCCEEDED |

非终态、RECOVERY_REQUIRED、缺日志/计划、请求不符、旧 action 日志没有命令标记，均不猜测成功。
损坏密文或不可验证日志不会转成终态，也不会自动重放。旧确认/回滚标记缺失的历史仍需后续人工
可审计恢复流程；此版本没有“强制成功”、删除执行记录或直接解除写保护的产品入口。

prepare 被启动恢复回滚后以失败核定；这与正常及时收到的“请求处理回执”区分。
确认/回滚只有对应命令被持久化接受才可用该路径证明成功。
按事务 UUID 的终态归档保证下一笔网络事务不会隐藏上一笔的结果。

## 存储、审计及界面

核定结果带 `reconciliation` 元数据：`source=network.journal.v1`、命令 ID、事务 ID、操作码、
业务终态、所读日志 SHA256 和观测时间。它证明历史事务的已记录结果，不承诺当前网络未被外部改动。
原完整网络日志及确认令牌不复制到审计事件；原授权结果 API 按既有权限返回回执。

agent 在同一个 SQLite 事务内将 UNKNOWN 转成加密终态回执并追加 `execution_resolutions`。
该表仅留命令/请求 SHA、来源、日志 SHA 和时间等审计元数据；并发查询只记录第一次核定。
原加密计划不变，未决保护继续生效；历史核定元数据保留，普通卸载不清理。
中心继续验证租户、enabled、当前连接 epoch 与 dispatch epoch，原子更新任务并追加
`task.reconcile.network` 审计。其他迟到回执的核定另有 `task.reconcile.receipt`。

React 详情同时提供中文、英文核定摘要及“未再次执行操作、当前状态请另行刷新”说明。
切换语言保留已选任务，回执更新沿原 SSE 机制刷新。
agent 新增 `task.network-outcome.v1` 能力，原 `task.reconcile.v1` 和线协议不变。

## 验证边界

本增量实际结果、失败记录及准确环境范围见 `phase2-network-outcome-validation-20261101.md`。

单元及协议故障中的人工日志/回执注入须明确标记，不能称为真实电源故障或完整系统崩溃矩阵。
具体运行结果与环境证据按本增量验证报告记录；阶段2–5、配置写入和快照写入仍未交付。
