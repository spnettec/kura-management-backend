# 代码图谱连接恢复记录

2026-11-01，工作空间 `/Users/heyoulin/iot-kura-develop/git`。

当前聊天原 MCP 通道报 `Transport closed`。配置中的程序路径有效，版本为
0.12.1；排查时没有运行中的图谱进程。服务日志记录 `stop_requested` 和
`service_stop`，未据此判断是谁停止了进程。CLI 仍能读取原有数据库，未发现需要
删除缓存或重建全部仓库的证据。

已执行 `codebase-memory-mcp daemon start`，服务为 permanent 模式，短暂 CLI/MCP
客户端结束后仍保持 active。本轮只刷新两个实际子仓库，均为 full、persistence=false：

| 实际仓库 | 节点 | 边 | 结果 |
| --- | ---: | ---: | --- |
| kura-management-agent | 528 | 1983 | indexed |
| kura-management-console | 2430 | 5866 | indexed |

没有索引父工作空间，没有删除既有缓存，也没有更改四工具的共享配置。

使用配置中的绝对程序路径新建真实 stdio MCP 会话，完成 initialize、tools/list、
list_projects、两仓库 search_graph、get_code_snippet、trace_path 与
check_index_coverage。17 项工具可发现，能定位新增 `Engine.verify_writers` 及中心
`run_stream`，并追到 transaction_sources/commit 调用链。验证的 Rust 文件
metadata_match/no_recorded_issue；这是 best-effort 覆盖信号，不能推导全部源码可解析。
console 仍有两项 SQL partial 和两项 SQL/配置 whole-file 解析缺口，应结合文本核对。

证据：`target/codebase-graph-repair-20261101.json`。后续源码更新仍须刷新实际子仓库，
本快照不代表后续版本。

当前聊天已关闭的旧客户端通道不能靠 daemon start 恢复；本轮没有可调用的客户端
reload 入口。仍需在 MCP 设置重新连接。服务端与新会话已恢复，在此期间可通过
官方 `codebase-memory-mcp cli` 使用同一图谱；未将 CLI 成功声称为旧通道已恢复。

后续生成器与首次接口恢复修正后再次刷新真实子仓库：agent 530 节点/1993 边，console
2434 节点/5873 边。daemon 仍为 active permanent（PID 78194），本轮实际使用 CLI
完成源码定位；原聊天原生 MCP 重试仍报 Transport closed。原修复快照保留，证据 JSON 的
followUp 记录此增量，SQL/配置解析缺口保持显式说明。

2026-11-01 中英文及 26.04 native configure 修正后再次按真实子仓库刷新：console
2691 节点/6422 边，agent 532 节点/1999 边。CLI 实际定位 I18nProvider 与 native_generator，
新语言模块、浏览器测试及 Rust writers 覆盖均为 metadata_match/no_recorded_issue。
packaged unit 为未跟踪文件，覆盖 freshness=not_tracked，已直接读取核对 AF_NETLINK，
不据此宣称完整解析；既有 SQL/配置缺口未改变。
当前聊天原 MCP list_projects 再次返回 Transport closed；官方 CLI 可用，不混同两种结果。
新增证据为 `target/i18n-graph-console.json`、`target/i18n-graph-agent.json`。

2026-11-01 配置归属/双向 UDS 增量后再次只刷新四个真实子仓库：core 28398 节点/108186 边、
networking 9023/41664、console 2929/6744、agent 552/2117。
关键新 API、core 协调/探针、Java client、Rust 本机传输和测试脚本已查询覆盖。
Java/Rust/Python/TS 证据路径为 metadata_match/no_recorded_issue；三个 manifest 为 not_tracked，
已直接读源码核对。核心记录 15 partial/1 unusable，console 仍为 2 partial/2 unusable；
networking/agent 无记录的解析缺口，并不构成完整性证明。
索引输出为 `target/provider-graph-*.txt`，覆盖为 `target/provider-coverage-*.json`。
后续仅文档变动不代表图谱已包含全部最新笔记；结构事实仍按真实源码和覆盖信号判断。

2026-11-01 公共网络事务/原 REST 增量再次刷新真实 core/networking/console：
28415/108291、9233/42964、3402/7506（节点/边）。新公共 API、provider facade、REST resource、
test-only probe、运行集 helper、HTTP smoke 和 Playwright 配置已核对覆盖，Java/Python/TS 为
metadata_match/no_recorded_issue；API manifest 为 not_tracked，已直接读取。
agent 未改源码，相关3个Rust路径仍 metadata_match。
core仍15 partial/1 unusable，console仍2 partial/2 unusable；原聊天MCP通道仍关闭，CLI可用。
证据为 `target/network-rest-graph-*`、`target/network-rest-coverage-*`。

2026-11-01 原 GWT 网络事务/双语回归增量：真实 management-ui full 刷新为
13386 节点/54898 边，console 为 3737/7940。核对 management-ui 本增量全部 16 路径、
console helper/浏览器及双语相关 8 路径：Java/Python/TS/XML/properties 均为
metadata_match/no_recorded_issue；web2 manifest 为 not_tracked，已全文读取核对公共事务导入。
management-ui 仍有 1 CSS partial/1 CSS unusable；console 仍 2 partial/2 unusable。
没有索引父目录，也未删除历史缓存。原聊天关闭通道状态未改变，使用官方 CLI/permanent daemon。
输出见 `target/gwt-network-graph-final.txt`、`target/gwt-network-graph-console.txt`、
`target/gwt-network-coverage-{ui,console}.json`；详细验收见
`phase2-network-gwt-validation-20261101.md`。后续仅补交接文档，不把本快照声明为全部文档实时索引。

2026-11-01 Netplan 类型/双语终态修正后，只刷新真实 management-ui 和 console：
13392 节点/54962 边、3998/8465。6 个 UI 路径与 5 个 console helper/浏览器路径
覆盖均为 metadata_match/no_recorded_issue。UI 仍 1 CSS partial/1 CSS unusable，
console 仍 2 partial/2 unusable；不据此声称完整解析。
原聊天 MCP list_projects 再试仍为 Transport closed，官方 CLI/permanent daemon 可用。
输出为 `target/netplan-ui-graph-{ui,console-final}.txt`、`target/netplan-ui-coverage-{ui,console}.json`。

2026-11-01 Netplan deb 增量只刷新真实 core/networking/console 子仓库（full、persistence=false）：
28563/108698、9288/43041、4055/8606 节点/边。8/3/2 个关键路径已核对覆盖，12 个为
metadata_match/no_recorded_issue；core unit 为 not_tracked，已直接全文读取并解最终 deb 检查。
core 仍有 15 partial/1 unusable，console 仍 2 partial/2 unusable；networking 无记录解析缺口。
初次 console inspector 覆盖为 metadata_changed，刷新后 metadata_match，初次报告保留。
原聊天 MCP list_projects 仍 Transport closed，官方 CLI/permanent daemon 可用。
证据为 `target/netplan-package-graph-*-final-20261101.json` 和
`target/netplan-package-coverage-*-20261101.json`。后续仅文档补记不视为图谱实时刷新，
也不据 ready 或路径无缺口推断完整解析；没有索引父工作空间或删除其历史缓存。
