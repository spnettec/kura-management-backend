# 普通交接原命令核定及真实回执丢失验证（2026-11-06，9104c438）

> **最新核定（2026-11-06，9104c438）：普通交接原命令核定与真实回执丢失闭环；完整目标继续 active。** 先读 `handoff-outcome-reconciliation-20261106.md`、对应 evidence、agent `docs/ordinary-handoff-task-outcome-v1.md`，以及知识库 `Experience/2026-11-06-Kura普通交接原命令核定与真实回执丢失验证-9104c438.md` 与 Tasks/Session Summaries 最新核定。ops21/22 的私有决定及完整原 wire SHA 绑定，execute 的 issued binding 在启停前落盘；只读 query 不调用 systemctl、live observe、repair 或 replay，后续恢复覆盖前归档原 journal。缺失/漂移/legacy/未提交继续 UNKNOWN，RR 持续挡写，孤儿命令证据挡 agent/Core 安装维护。中心验证原加密输入重构的 wire，审计一次 `task.reconcile.ordinary_handoff`，双语详情限定选定检查。并发查询在另一查询已核定时重读原回执，消除过期 UNKNOWN 返回。
> 最终宿主/Linux agent177、console20/protocol10、PG9、包装两处57、Core guard两处14；浏览器既有12回归及新3通过。IDEA新8e1d0e40种植27701→ordinary28198（284 inventory）、observer1、真实DS/密码/snapshot和原GWT admin/admin中文Count9通过并已停。Orb新95413f3a：prepare 733a8f5a-3f32-4b3c-a87c-74287ebd53d4 仅一次，execute 804337e9-5b0a-4c5a-9de5-de4acfcce3f5 仅一次；native提交后测试进程exit92、中心实际UNKNOWN，正式v3 ELF只读核定成功，Kura PID616673不变，native/receipt/command字节不变、SQLite resolution和中心审计各一次；原GWT9。Node只信任本轮CA后从原prepare续接，未重发准备。PRL新e046063c当前v3 deb仅解包，实际strict/CapEff0/NoNewPrivs1/EROFS及默认关闭Hello/三操作准入前拒绝通过；无Java/Kura目标或伪造completion。
> 宿主新中心/PG/keepawake、IDEA/observer及Orb新运行单元全部停止，故障单元failed/92且PID0保留，原瞬态agent已自动卸载；Ubuntu与Windows均stopped。原Orb26/20四服务原PID保持active。18 HEAD/分支/旧dirty路径保留，无提交推送。当前 review deb `0.1.0~review20261106.handoffproof3`，日志 `target/handoff-outcome-reconciliation-20261106/`。历史mock脚本的旧截图路径曾被本轮回归写入，审计已单列，不作为旧一次性业务场景重验。
> 仅闭环普通交接的原命令核定；未决计划运维、restart/reboot任务、Core完整安装升级/交接、实际重启后完整Kura业务、全组件/metatype/provider/部分空CM/外部PID、NM/firewall/NAT/远程/早启、一致备份/key/迁移/正式发行仍pending；PRL完整Java/Kura、真实NM/cloud-init/物理/硬件、amd64、磁盘满/硬断电等外部deferred单列。normalBootVerified/kuraRecoveryVerified仍false；保留NM/Kura无agent独立运行等既定方向。


## 实现与证据边界

`recovery/ordinary/outcome.rs` 独立校验不可变 source/grant/decision/native receipt，完整绑定 command UUID、operation、idempotency key、原 expiry 和 typed action。prepare/execute 各至多一个决定；旧本机交接缺省字段不改变原回执格式。prepare 历史决定可在后续 RR 中核定，但不解除 RR；execute 必须已 committed、schema2 ready 且所选 checks 全部通过。后续正常恢复替换 journal 前建立 create-only 归档，旧查询不依赖当前 unit 或 grant。主 journal 缺失而 command evidence 留存时拒绝新采用和其他 writer/包维护。

中心从原加密 input 构造完整 wire 并验证 request/journal/decision digest、原 handoff 与 selected scope，原 UNKNOWN 只生成一次核定审计。UI 只说明原任务的持久决定；仍不证明完整业务、开机或 reboot。协议1–22及默认开关未扩展，console/agent归属独立、默认admin/admin与首次密码后台闸门、中英及配置/firewall默认关闭均保留。

## 验证

| 范围 | 最终结果 | 证据 |
|---|---|---|
| agent宿主 | 177通过/9忽略，严格clippy通过 | agent-full-v3.log / agent-clippy-v3.log |
| agent Linux当前源码 | 177通过/10忽略，严格clippy通过 | orb-v3-tests.log / orb-v3-clippy.log |
| console/protocol/实际PG | 20 / 10 / 9通过 | console-unit-v2.log / protocol-test.log / postgres-tests-v2.log |
| 安装与Core脚本 | 两平台各57 / 各14通过 | packaging-host.log / packaging-linux-root-v2.log / core-guard-host.log / core-guard-linux-root-v2.log |
| HTTP边界 | 16+7及首次密码新路由闸门通过 | http-contract.json / post-http-proof.json / first-password-gate.json |
| 浏览器 | 旧12回归、新handoff proof3、真实故障流程1通过 | browser-mock.json（3个旧定位失败已修）、browser-mock-v2.json / browser-live-v5.json |
| IDEA完整隔离Kura/原GWT | 284 inventory、observer1、真实DS/密码/snapshot、同JVM原GWT9通过 | native/final.json / native/observer.log / native/gwt-verification.json |
| Orb真实故障 | 原native提交→测试agent exit92→中心UNKNOWN→正式agent核定原命令；Kura PID及持久证据均不变 | orb-before-production.json / orb-post-proof.json / center-final-audit.json |
| 现有Parallels | 当前正式main严格沙箱、实际EROFS、默认关闭capability及三操作拒绝；无Java | parallels-final-state.json / post-http-proof.json |

本轮仅改Rust/前端及安装闸门，没有Java/Core业务/GWT生产变更；历史Java429/bridge53与Docker业务验收为carry-forward，未冒充本轮新验收。Core脚本14为本轮新验证，未发行新的Core deb。当前ARM64 review agent deb仅测试解包及隔离运行，尚非正式发行。

## 调试修正与现场保留

浏览器最初用错中文按钮定位，修正测试后通过。故障测试最初采用单线程运行方式，未上线且中心零任务，调试/优化两种旧测试进程均在准入前停止；改为与生产一致的多线程测试后连接成功。并发回归捕获到既有的查询竞态：首次读UNKNOWN后，另一查询已落终态导致pending lookup为空；现在重读原receipt。此前失败日志和独立v1/v2源码、二进制及制品保留；只有v3是当前交付制品。

真实流程的prepare已成功后，Node request context未信任临时CA导致只读取任务失败。设置仅该测试进程的NODE_EXTRA_CA_CERTS，按同一prepare ID和key只读续接；无第二个prepare。当前确认执行仅一次，完整日志含私有测试会话信息，因此本轮证据目录为0700。首次密码测试最初误写错误码期望，按实际PASSWORD_CHANGE_REQUIRED修正测试，账号未更改。收尾发现旧瞬态agent已自动卸载，改为先核对状态，仅停止仍运行的本轮单元；未重建或启动旧瞬态单元。

旧boot mock脚本自带固定截图路径，本轮回归曾写回 `target/boot-outcome-reconciliation-20261106/browser-proof-en.png`；未找到该图的历史SHA记录，已单列 `historical-mock-image-check.json`，没有将其声称为原一次性业务验收或补造旧hash。以后旧mock回归先隔离其输出路径。旧业务夹具、状态、密钥、备份、quarantine和卷保留；没有重放旧一次性入口。

## 当前运行状态与下一步

运行/停止状态以 `final-runtime-state.json` 为准。所有新夹具进程已停，故障unit failed/92/PID0保留，原四服务PID与26/20原包不变；Ubuntu/Windows均正常stopped。未reset/clean/提交/推送/覆盖远端，18仓HEAD、分支和已有dirty路径保持。

下一轮先核对实时HEAD和dirty、读取本交接及Tasks/Session Summaries。优先推进未决计划的显式运维与restart/reboot任务，后续完整组件适用面、Core安装/交接、NM/firewall/NAT/早启、一致备份和正式发行仍按总台账实施。外部deferred不得与pending混合或由unit/opaque artifact代替。禁止重跑本轮任何已完成prepare/execute、旧恢复入口或故障夹具。
