# 公共网络事务 API 与原 Kura REST 增量

2026-11-01，父工作空间 `/Users/heyoulin/iot-kura-develop/git`。
整体 goal 继续 active；阶段 2–5 未完成。所有改动保持未提交，未 reset/clean、提交、推送或改远端。
前一增量及其 manifest 仍是历史快照：`phase2-netplan-adapter-validation-20261101.md`、
`evidence/netplan-adapter-20261101.json`。
本增量源码、产物和证据哈希见 `evidence/network-rest-20261101.json`。
父工作空间交接为
`Experience/2026-11-01-Kura管理体系Rust-React公共网络事务与原REST交接-d8db9d36.md`，
已通过官方 CLI 全文读回；Tasks 的其他任务保留。

## 交付

核心 API 新增 `org.eclipse.kura.net.transaction.NetworkTransactionService` 与稳定异常类型，
只使用 JDK 值，不依赖 networking sibling 或 Gson。Netplan provider 同时提供公共服务和原内部服务；
`NetworkValues` 保留整数精度、分离返回值，拒绝不支持类型、非整数和超深/超大结构。

原 networking REST 新增独立 optional dynamic DS resource，提供 view/prepare/query/confirm/rollback，
沿用原网络权限。严格外层字段和 ID，令牌只在 body，类型化错误不泄露详情，响应禁止缓存。
prepare 202 明确表示待确认；缺 owner 返回503，超时不重发、不自动改写配置。
原接口与旧字段拒写策略不变。完整契约见 `network-transactions-rest-v1.md`。

## 验证证据

| 检查 | 结果及实际范围 |
| --- | --- |
| core API 构建/安装 | 成功；不是新增 API 的单元测试数声明 |
| Java Netplan provider | 15 项（原9＋公共 facade 6），0失败/跳过 |
| Java 原 REST provider | 19 项（原12＋事务7），0失败/跳过 |
| 既定浏览器回归 | 10项通过，`target/phase2-network-rest-browser-final.json` |
| 新默认浏览器集合 | 11项通过，0 skipped/unexpected/flaky，`target/phase2-network-rest-browser-default.json` |
| IDEA 完整隔离 Kura | 最终 PID99968；公共 API/原 REST resource 实际注册，原 GWT 控制台登录成功 |
| 真实 HTTP→公共 API→Java→Rust UDS | 21项通过，`target/phase2-network-rest-http.json`；候选未应用 |
| 隔离环境恢复 | 原3个JAR＋config.ini＋jvm.args及snapshot0 SHA匹配；新增快照归档；最终JVM停止 |
| OrbStack 只读核对 | agent active、revision3、事务ROLLED_BACK、yofc-test0恢复192.0.2.2/24、Kura委托禁用 |
| Parallels | Ubuntu26.04保持stopped；没有重跑/增开VM |

HTTP 在本轮独立 `.phase1-dev` Kura 上执行。原 REST policy 为允许443/4443；test-only
probe 先将实际18080 listener限制到127.0.0.1，才临时允许REST18080。结束时先恢复原policy，
再恢复listener host；`target/phase2-network-rest-runtime.json` 证明 restored。
probe只进入测试JAR，产品JAR不含探针；未为了验收修改生产HTTP server。
恢复脚本在IDEA停止JVM后还原3个原runtime JAR与配置，原快照不覆盖。
证据为 `target/phase2-network-rest-restoration.json` 和
`target/network-rest-original-login-final.jpg`。

真实HTTP覆盖五端点匿名拒绝、原配置路径及原身份、schema1、确认态与候选分离、
no-store、未知字段拒绝、空Kura接口委托拒写、未知事务和明确token要求。
宿主没有写接口授权；成功候选/确认/回退状态的HTTP映射只有单元/契约证据。
未执行真实非管理员角色矩阵，不能把注解检查等同运行时越权验收。

Linux没有升级Java或重跑一次性角色/接管脚本。只读证据
`target/phase2-network-rest-linux-observe.json` 确认前次误跑network-write已回退，
持久revision仍3、管理默认路由仍eth0，禁用委托及原synthetic角色UID恢复。
agent ELF仍为 `c95b06bd147ffccc4c1fcb720a8869da71043d0ba225d82670722209085ffddc`。
这是现有fixture状态证据，不能扩大为本增量的Linux Java provider验收。

## 失败与修正

- 初次REST单元有重复Mockito stubbing异常，改为doThrow后19项通过；首次与最终日志均保留。
- 首次默认浏览器误选live专项，14项中12通过、2失败：停止的Parallels观测断言失败，
  已接管来源的首次接管测试超时。该报告不是成功验收。network-write当次通过并回退，
  后续已实际只读核对。默认配置现在排除3个live文件，显式
  `KURA_BROWSER_LIVE_PROFILE=network|parallels|takeover` 选择专项；未知值拒绝。
- test-only probe前三次在启动时序检查失败：公共UDS动态引用未就绪、REST配置尚未注册、
  HTTP server holder尚未初始化。补充有界只读等待，三个失败探针和结果均归档到
  `.phase1-dev/network-rest-profile-backup/failed-probe-{1,2,3}/`，没有绕过恢复校验。
- 第四次HTTP最后一条断言将空transaction当作必有JSON键；原REST Gson会省略null字段。
  改为缺省/空值都表示无事务后21项通过，生产序列化器未改。

主要输出：`target/phase2-network-rest-api.log`、`phase2-network-rest-java.log`、
`phase2-network-rest-java-final.log`、`phase2-network-rest-probe-build-final.log`、
`phase2-network-rest-smoke-attempt{2,3,4}.log`、`phase2-network-rest-smoke-final.log`。
首次探针失败结果保留在上述failed-probe-1目录。

## 图谱与剩余范围

只刷新真实core/networking/console子仓库，使用官方CLI与permanent daemon；原聊天原生MCP
仍Transport closed。新增Java/Python/TS路径按当前generation检查metadata_match；manifest
出现not_tracked时直接核对，不宣称完整解析。core历史partial/unusable和console的SQL/配置
解析缺口仍保留。输出为 `target/network-rest-graph-*`、`target/network-rest-coverage-*`。

React中英文已交付并保留，新的界面文案仍要求双语。**原GWT网络事务交互、完整无损旧字段写入映射**
仍pending：需把编辑baseline、revision、sourceDigest一起捕获，明确展示候选/确认/回退；
不能提交时读新revision后套用旧表单，不能丢失未展示routes/IPv6/未知字段。
正式provider/profile、NM/cloud-init交接、注册转交/轮换、基础nftables、系统控制、离线恢复和发行仍开放。
缺环境的Linux完整Kura、24.04/26.04×amd64/arm64发行矩阵、DHCP/resolver、物理有线及硬断电验证
保持deferred。另一console不能自动抢占网关；正式转交尚未完成。
