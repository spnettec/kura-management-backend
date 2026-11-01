# 配置提交结果核定验证

日期：2026-11-01。契约见 `configuration-outcome-reconciliation-v1.md`。IDEA 完整 Kura、
原控制台登录和 OrbStack 尚未对本增量复验，不能记为真实运行闭环。

## 已通过

| 范围 | 结果 |
| --- | --- |
| Java bridge | `ConfigurationOutcomeTest` 3 项；同次 bridge 选择共 33 项通过。覆盖 canonical hash、记录不含属性值、拒绝写入不建记录、读取不触当前配置 |
| agent | 65 项全部通过。覆盖 Java/Rust golden hash、匹配记录把 UNKNOWN 变成功、PID 不匹配/缺记录保持 UNKNOWN、查询不重发写入、结果不回显 secret |
| 中心 | workspace 9+8 通过。覆盖 `configuration.journal.v1` 绑定校验、缺少属性值、审计分类测试 |
| React | 类型检查和生产构建通过；默认浏览器 26 项通过，含配置提交日志核定与网络事务核定分离的双语界面 |

chunk 约 1013 kB，原有体积提示保留。PostgreSQL 与 mTLS 显式集成测试仍按环境要求 ignored。

## 未执行

未在真实 IDEA Kura 中制造断流后查询、核对 `${kura.data}/management-outcomes` 文件权限、
审计事件和原 GWT 登录回归。当时 OrbStack 也未复验；下方同日晚间增量已复验正常成功路径。不能把这些单元/浏览器结果扩大为完整系统验证。
删除/reset/factory、组件异步健康核定和正式发行仍 pending。

## 2026-11-01 IDEA 真实运行增量

真实受限配置写入两次成功后，`${kura.data}/management-outcomes/` 各生成一条 0600 记录；
字段只含 source、commandId、operation、pid、previousRevision、submitted 和 `paramsSha256`，
断言不含密码、count 值或当前配置。命令 ID 分别为 `5c90caa0-b8b1-45ae-9ecd-c5e2df7dd8d6`
和 `f2475073-79b5-48a8-b2a3-5213a75c8f99`。React 成功提示与原记录绑定，无重发。
本流程没有制造断流 UNKNOWN，因此没有新的 `task.reconcile.configuration` 运行审计；
UNKNOWN→原记录核定的单元/浏览器证据仍是上文测试，OrbStack 断流核定继续 deferred。


## 2026-11-01 OrbStack 真实运行增量

Linux 完整 Kura 中两次受限写入各生成一条 `configuration.journal.v1` 记录，文件 0600，字段不含密码、
count 或当前配置。命令 ID 为 `6672f66d-dacf-41c8-b050-fb7350002196` 和
`8a106b06-06cb-4beb-8337-b56291465b20`。本流程未制造断流 UNKNOWN；真实断流后的
UNKNOWN 原记录核定继续 deferred，不能由正常成功记录替代。


## 2026-11-02 IDEA 真实 UNKNOWN 回执损失核定

在 `configuration-outcome-snapshot-idea-20261101` 中执行一次真实 Operation 12：
`43d44fee-3e9c-4fc6-b6d8-842c3fc7dd2a`。观察到 outcome 文件原子落盘后立即暂停并终止 agent，
模拟本地回执丢失；重启 agent 后中心先得到持久 `UNKNOWN`，再次查询只用原记录核定为
`SUCCEEDED`。核定 proof 绑定 `configuration.journal.v1`、原 PID/revision、
`paramsSha256=02113bd0fad9ea577e8c09ca594961b76d77ed4e206fc4dbb78cd45e57584d87` 和
`journalSha256=8102e0ef6457d5a8f5ea590f548cfaeaf211ccb1720ac79a8355e62b9a8d22b4`。
outcome 文件 0600、不含属性值/密码；审计依次为 `task.uncertain/UNKNOWN`、
`task.result/SUCCEEDED`、`task.reconcile.configuration/SUCCEEDED`。没有重发写入。

该真实故障注入发现并修复一个存储缺陷：`execution_resolutions.source` 原被硬编码为
`network.journal.v1`；现按 proof 保存 `configuration.journal.v1`，并新增
`configuration_resolution_preserves_its_own_journal_source` 单测。修复后 agent 66 项通过，
release SHA `2eb0ccb30320f557192fb0fee02fb238658f511d894fa4f34981ad4b4a823f7a`。
证据见 `phase2-configuration-unknown-idea-20261102.json`。旧 resolution 行保留错误 source，
作为发现前证据，不篡改。夹具已 `stopped-data-retained` 且 gateway 禁用。
