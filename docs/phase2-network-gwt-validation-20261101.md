# 原 GWT 网络事务与双语回归增量

2026-11-01，父工作空间 `/Users/heyoulin/iot-kura-develop/git`。
整体 goal 继续 active，阶段 2–5 未完成。现有迁移与本增量均保持未提交；未 reset/clean、提交、推送或修改远端。
本增量哈希见 `evidence/network-gwt-20261101.json`；前一 REST 增量证据保留为历史快照。
父工作空间交接为
`Experience/2026-11-01-Kura管理体系Rust-React原GWT网络事务与双语回归交接-f7d68943.md`，
已通过官方 Obsidian CLI 全文读回，Tasks 其他任务保留。

## 交付范围

原 GWT 在显式 Netplan 模式下通过公共 `NetworkTransactionService` 编辑受限有线配置，
同一 view 捕获 revision/sourceDigest/canonical/display 基线，随机 edit ID 按 HTTP 会话隔离。
允许 DHCP4/手动、主 IPv4/掩码、单一默认网关、IPv4 DNS、MTU；其余附加地址、routes/metric、
IPv6 DNS 顺序、searchDomains、optional、dhcpMetric 和企业属性保留原值。
不支持的字段禁用或隐藏，服务端仍拒绝篡改。硬件观测值不回填写请求。

prepare 仅表示待确认，页面两秒观测状态；确认前必须明确勾选管理路径重新连接。
勾选是操作者声明，不是系统自动连通性证明。确认令牌只留在创建事务的服务端 HTTP 会话，
其他会话可观测、不能确认/回退；查询不能恢复令牌。写入失败不自动重试，丢回执保持不确定状态。
原 Kura 用户、NETWORK_ADMIN、写操作 XSRF 继续适用；中心/agent 注册认证仍独立于 Kura。

新增 15 组英文/中文消息键；原 GWT 保留 locale cookie 和编译排列。
React 双语检测、即时切换、选择记忆、错误翻译继续通过回归，见 `console-i18n-20261101.md`。
完整行为契约见 `network-transactions-gwt-v1.md`。

## 已执行验证

| 检查 | 最终结果与边界 |
| --- | --- |
| GWT 完整构建 | login/denali 的 zh_CN/en/ja × 两浏览器排列编译通过；Java 11 生产目标、Java 21 测试 |
| 服务端测试 | 62 项通过，0 failures/errors/skipped；含新增 13 项，不把原有 49 项算作新增 |
| 原控制台运行 | IDEA 完整隔离 Kura 最终 PID 4775，实际登录及 GWT RPC 成功 |
| GWT 浏览器 | 4 项通过，0 skipped/unexpected/flaky；明确 test-only OSGi network owner |
| React 回归 | 类型检查/生产构建成功，默认浏览器 11 项通过，0 skipped/unexpected/flaky |
| 恢复 | IDEA 停止 JVM 后，原 API/web2 两 JAR、config.ini、jvm.args、snapshot_0 SHA 一致；临时 fixture JAR 移出运行集 |
| 产品边界 | 生产 web2 JAR 不含 test-only fixture/probe 类；本增量系统网络写入为 0 |
| Parallels | 收尾只读核对 Ubuntu 26.04 ARM64 仍 stopped；没有启动或增开 VM |

GWT 四项覆盖双语/只读控件；prepare、路径勾选和同会话确认；另一独立 HTTP 会话只能观测；
明确回退；INVALID_ARGUMENT 后允许修正；已接受但 TIMEOUT 的丢回执只观测且不恢复 token。
使用大于 JavaScript 安全整数的 revision，验证没有把新 revision 套用旧表单。
最终 fixture 记录 prepares=4、confirms=1、rollbacks=1、revision=9007199254740994、WAIT_CONFIRM；
最后一笔是丢回执模拟。confirmed MTU 1400、candidate MTU 1302，其他字段保留。

这些是实际 IDEA/Kura/GWT RPC 加测试 owner 的证据。fixture 不调用 UDS、Netplan 或系统网络命令，
不能替代真实 Rust agent、Linux Java provider、断网重连或超时系统回退验收。
本增量没有修改 agent/Linux 源码，没有重跑一次性接管或 live 网络写入专项。

主要证据：

- `target/gwt-network-full-build-form-final.log`：最终完整构建与 62 项测试。
- `target/gwt-network-browser.json`、`target/gwt-network-browser-fourth.log`：最终 4 项浏览器结果。
- `target/gwt-network-{readonly-zh,confirmed-en,lost-receipt-en}.png`：实际编译页面截图。
- `target/gwt-network-fixture.txt`：无令牌的测试状态与保留字段。
- `target/gwt-network-restoration.json`、`target/gwt-network-restore-check.json`：恢复与独立 SHA/进程/产物检查。
- `target/console-i18n-current-build.log`、`target/console-i18n-current-browser.json`：React 本轮回归。

## 失败与修正

最初 GWT Messages 占位符生成 String 参数，传 int 导致编译失败；最终显式 String.valueOf，
没有改变 i18n 生成器。原状态页直接内部调用 servlet 时没有 HTTP request，首次运行 NPE；
列表允许临时只读基线，事务控制仍要求真实会话，并补回归。
新服务端测试最初缺 FileUpload2 runtime 类；仅补测试依赖，最终 62 项通过。

前两次浏览器英文断言因旧 locale cookie 的 path 冲突失败，第二次编辑因 cwd 错误未生效；
最终测试先清同名 cookie。第三次因旧硬件只读值/隐藏企业属性被客户端回填写入，prepare 被拒；
修正为保留基线、只提交可写值后第四次 4 项通过。
失败构建、运行和三次浏览器报告保留在 `target/gwt-network-*`，不记为通过。
一次性 helper 的原备份、两次测试产物修订和新增快照保存在
`.phase1-dev/network-gwt-profile-backup/`；state 已 restored，不可删除 state 重新 prepare。

## 图谱和剩余事项

官方 CLI/permanent daemon 刷新真实 management-ui 和 console 子仓库，并核对关键路径覆盖。
原聊天旧 MCP 通道仍 Transport closed，CLI 成功不表示该旧通道已恢复。
management-ui CSS partial/unusable、console SQL/配置解析缺口保留，覆盖信号不代表完整解析。
本轮图谱与覆盖输出为 `target/gwt-network-graph-*`、`target/gwt-network-coverage-*`。

完整旧字段写入、正式 provider/profile、真实 NM/cloud-init 交接、注册转交/轮换、基础 nftables、
系统控制、离线恢复和正式发行仍 pending；另一 console 不得自动抢占。
Linux 完整 Kura/真实写入、DHCP/resolver、24.04/26.04 × amd64/arm64 发行矩阵及物理有线/断电验证
缺环境部分保持 deferred。React 主包约 963 kB 的拆包仍 pending。
GWT 终态仍可能显示原 deadline 的剩余秒数，后续应改进终态文案；不影响待确认控制规则。
