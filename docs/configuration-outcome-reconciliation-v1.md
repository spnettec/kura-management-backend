# 配置 UNKNOWN 持久业务结果核定 v1

日期：2026-11-01。接在显式快照保存/回滚之后。它只核定受限配置更新（Operation 12）的
“Kura 是否已接受该次提交”，不核定组件异步健康，也不能凭当前配置相同推断历史动作成功。

## 记录

成功执行 `ConfigurationRevisionService.updateRevision` 后，Java bridge 在
`${kura.data}/management-outcomes` 写一个命令专用记录。
正式 Main 使用 Core `SystemService` 的数据目录。部署可用
`-Dkura.management.configuration.outcome.dir` 显式覆盖。缺少可定位的存储时，带原命令编号的请求在调用 Core 前被拒绝。
记录只含 source、commandId、operation、pid、previousRevision、submitted 和
`paramsSha256`，不含属性值、密码或当前配置。目录 0700，记录 0600。
发布前同步临时文件，以原编号原子创建记录，成功前同步目录。
`paramsSha256` 使用与 Rust agent 共享的 canonical vector，覆盖 PID、revision 和全部提交属性，
已用 Java/Rust 相同 golden 向量核对。它绑定原请求，但不用于展示配置内容。

如果 `updateRevision` 成功而记录失败，bridge 返回 OUTCOME_UNKNOWN；查询缺记录时保持 UNKNOWN。
写前拒绝不建记录。损坏、符号链接、来源/命令/操作/PID/revision/params hash 不匹配都保持 UNKNOWN。
查询只发送 `kura.configuration.outcome` 读取原记录；不会重发 Operation 12，也不读取当前配置。
agent 在同一 SQLite 事务内将 UNKNOWN 转为终态并追加 `execution_resolutions`，source 为
`configuration.journal.v1`。并发查询保留第一次核定；原加密计划不变。

## 回执与界面

成功回执只含 `pid`、`submitted:true`、`previousRevision` 和 reconciliation 元数据。
它证明 Kura 已按原配置写入语义接受提交和快照策略，不证明组件已经应用成功，也不证明
当前值仍相同。中心校验 reconciliation 绑定后记 `task.reconcile.configuration` 审计。
React 对网络日志核定和配置提交日志核定分开显示，并继续提示“未再次执行写入，当前值请另行刷新”。

agent 宣告 `task.configuration-outcome.v1`。旧 bridge 未登记 `kura.configuration.outcome`
时保持 UNKNOWN，不降级为当前值核对或重放。

## 边界

本机制不能撤销写入，也不是删除/reset/factory。若本机特权方篡改记录及其元数据，本版本
不能提供超出本地完整性假设的防伪。历史 IDEA/OrbStack 结果见
[早期验证](phase2-configuration-outcome-20261101.md)。

2026-11-20 在已安装的 Debian 13 ARM64 环境中，正式 Main 发布新命令的原编号回执后，
Agent 在持久终态前中断。重启时撤回配置写权限。回执缺位期间原任务保持 UNKNOWN，
恢复同一 Root 中原回执后按原编号核定为 SUCCEEDED。已核对原加密计划、PostgreSQL 审计和唯一派发。
该验收覆盖提交回执，独立的异步业务健康仍需验证。
[正式包检查点](checkpoints/20261116/actual-native-configuration-unknown-outcome-checkpoint.json)
