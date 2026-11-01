# Netplan 原控制台类型与双语终态修正

2026-11-01。本增量修正上一轮真实 Linux 开发运行集发现的两项原 GWT 问题。
React console 的中文/英文已经交付；本轮没有改动其生产实现。
整体 Rust/React 改造仍 active，正式发行 profile 和阶段 2–5 继续开放。
父工作空间交接为
`Experience/2026-11-01-Kura管理体系Rust-React双语与GWT终态修正交接-4618ad03.md`，
经官方 Obsidian CLI 写入并全文读回；Tasks 首项更新，其余任务保留。

## 代码与行为

`NetplanNetworkEdit` 明确设置 Ethernet 硬件类型。既有 builder 的 `forType` 只决定构建子类型，
不从空 legacy properties 推导 `hwType`；因此之前原状态页调用 `getHwTypeEnum()` 时会记录异常。
本次只补 Netplan 模型，包含受管条目和仅观测的有线接口，不修改通用 builder 行为。

`NetplanNetworkSession` 只为 `WAIT_CONFIRM` 返回剩余确认秒数，其余状态为 0；
终态仍删除会话中的确认令牌。`NetworkButtonBarUi` 只在等待确认时显示倒计时，
其余状态使用新增的中英文 `netplanTransactionState` 摘要。

新增两项契约回归，分别覆盖配置/观测接口的类型、带未来 deadline 的非等待及终态行为，
并检查终态不能再次使用原确认令牌。原等待态断言继续验证正常倒计时。

## 验证

| 检查 | 本次实际结果 |
| --- | --- |
| 修正前回归 | 定向 16 项中新增 2 项失败，分别复现类型 null、PREPARING 错误倒计时 60 秒；0 errors/skipped |
| 修正后完整 GWT | login/denali 编译、正常 Maven install；65 项服务端测试通过，0 failures/errors/skipped |
| React 浏览器 | 默认 11 项通过，0 unexpected/skipped/flaky；包含真实 HTTPS 登录/SSE 和明确标注的模拟表单契约 |
| IDEA | 完整隔离 Kura PID 10178，原控制台 admin 登录、状态页 Ethernet 内容可见；本次日志 HwType 错误 0 次 |
| 原 GWT 浏览器 | 4 项通过；英文确认摘要、英文/中文回退摘要无倒计时；等待态仍显示倒计时，确认门槛和跨会话隔离保持 |
| 收尾 | IDEA Stop，独立进程核对已退出；原配置/JAR/初始快照 SHA 恢复，新增快照归档保留 |

顺序为服务端单元/契约、React 浏览器 → IDEA 完整隔离 Kura及原控制台登录/GWT 浏览器。
本轮没有启动 Parallels 或重放已 restored 的 Linux fixture，没有执行系统网络写入。
本次两项修正的 Linux 新版本回归为 pending，不能把宿主 test-only 结果记作 Linux 网络写入验收。
上一轮真实 Linux 事务证据保持其原版本语义。

## 可重复验证与证据

`scripts/phase2-network-gwt-runtime.py` 新增显式 `--run-id`，为新一轮验证分配独立状态、备份和输出。
原默认 GWT state 保持 restored，新命名运行还会校验旧恢复基线的 SHA；原 `prepare` 防重放规则保持。
本次 run-id 为 `ui-regression-20261101`，状态现为 restored，同名不能再次 prepare。
浏览器配置和截图接受同一 `KURA_GWT_RUN_ID`，不会覆盖旧报告。
fixture 是独立 test-only OSGi owner，系统网络写入为 0，不含真实 agent 事务执行。

证据哈希见 `evidence/network-ui-regression-20261101.json`；旧 manifest 不改写。

- `target/netplan-ui-regression-before.log`：修正前两项缺陷的实际失败。
- `target/netplan-ui-regression-build.log`：完整构建及 65 项通过。
- `target/netplan-ui-react-browser-final.{json,log}`：11 项 React 回归及独立 `netplan-ui-current-*` 截图。
- `target/gwt-network-ui-regression-20261101-browser.json`、`netplan-ui-gwt-browser.log`：4 项原 GWT。
- `target/gwt-network-ui-regression-20261101-{confirmed-en,rolled-back-zh,readonly-zh,lost-receipt-en}.png`：双语真实 GWT 交互。
- `target/netplan-ui-{idea.png,status-en.png,idea-kura.log}`：IDEA、原状态页和本次启动日志。
- `target/gwt-network-ui-regression-20261101-{fixture.txt,restoration.json}`：test-only 计数与恢复。
- `target/netplan-ui-final-check.json`：报告、恢复、初始快照、日志和防重放的只读核对。

首次 React 回归通过，但执行时漏设 `EVIDENCE_PREFIX`，默认截图别名被本轮更新；
已存在 manifest 中的证据文件 SHA 核对未改变，不能把这些默认别名当作旧截图的不可变归档。
补齐一处仍硬编码的事务截图前缀后，以 `netplan-ui-current` 独立命名再次执行并保留最终报告；
首次报告也保留为 `netplan-ui-react-browser.{json,log}`，不掩盖此证据管理问题。

正式 core/networking 包仍使用旧依赖和生命周期；仅新增 Netplan sibling 无法消除 core 包中的 NM 依赖。
下一增量继续实施正式 package/profile、服务权限和卸载保留规则，不能将当前开发运行集当作正式发行。
登记/转交、完整旧字段、nftables、系统控制、离线恢复等仍 pending；缺少硬件/架构环境的验证仍 deferred。
