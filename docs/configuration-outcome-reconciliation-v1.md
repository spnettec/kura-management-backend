# 配置 UNKNOWN 持久业务结果核定 v1

日期：2026-11-01。接在显式快照保存/回滚之后。它只核定受限配置更新（Operation 12）的
“Kura 是否已接受该次提交”，不核定组件异步健康，也不能凭当前配置相同推断历史动作成功。

## 记录

成功执行 `ConfigurationRevisionService.updateRevision` 后，Java bridge 在
`${kura.data}/management-outcomes`（可用
`-Dkura.management.configuration.outcome.dir` 显式覆盖）写一个命令专用记录。
记录只含 source、commandId、operation、pid、previousRevision、submitted 和
`paramsSha256`，不含属性值、密码或当前配置。目录 0700，记录 0600，临时文件原子改名。
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
不能提供超出本地完整性假设的防伪。真实 IDEA/OrbStack 运行验证见
`phase2-configuration-outcome-20261101.md`；未执行前不得宣称真实 Kura 闭环。
