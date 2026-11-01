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
审计事件和原 GWT 登录回归。OrbStack 也未复验。不能把这些单元/浏览器结果扩大为完整系统验证。
删除/reset/factory、组件异步健康核定和正式发行仍 pending。

## 2026-11-01 IDEA 真实运行增量

真实受限配置写入两次成功后，`${kura.data}/management-outcomes/` 各生成一条 0600 记录；
字段只含 source、commandId、operation、pid、previousRevision、submitted 和 `paramsSha256`，
断言不含密码、count 值或当前配置。命令 ID 分别为 `5c90caa0-b8b1-45ae-9ecd-c5e2df7dd8d6`
和 `f2475073-79b5-48a8-b2a3-5213a75c8f99`。React 成功提示与原记录绑定，无重发。
本流程没有制造断流 UNKNOWN，因此没有新的 `task.reconcile.configuration` 运行审计；
UNKNOWN→原记录核定的单元/浏览器证据仍是上文测试，OrbStack 断流核定继续 deferred。
