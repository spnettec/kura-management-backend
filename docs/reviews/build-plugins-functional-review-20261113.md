# G022：PLC4X 上游构建与语言绑定功能审查（2026-11-13）

## 结果与范围

`apache/plc4x` 的已审查游标从 `671e086ce85ae9c0722ceb031cb7377a66185d99` 续接至 `ea42d3db707839f3d2030d05c929a42a867c5d2f`，新增3提交按3个功能处置：Java构建插件、Go依赖、.NET SLMP多块读取。没有执行整分支合并、提交或推送。原工作树只应用5个已核对前后SHA的文件；其余源码、Git HEAD/分支/index/暂存区/refs/remotes均保留。改前文件保存在隔离证据目录。

证据根为 `target/upstream-logical-sync-20261112/build-plugins-functional-v17/`，权威入口为 `functional-review.json`、`applied-to-working-tree.json`、`preservation-final.json`、`final-binding.json`。此前V15和管理体系V46绑定均保持原文，不将历史绑定解释为新工作树的全部指纹。

| 功能 | 实际处置 | 验证边界 |
| --- | --- | --- |
| Maven插件 | exec 3.6.4→3.7.0、build-helper 3.7.0→3.8.0、xml 1.3.0→1.4.0；只更新父POM的3个版本 | 三个确切版本的官方JAR/POM/plugin descriptor均留存SHA；均要求Maven3.9/JDK8，实际使用Maven3.10/JDK21 |
| Go依赖 | `go.mod`、`go.sum`逐字取上游；维持私有缓存、只读module模式 | build通过；vet真实退出1，与改前4461个Go源码的31条诊断完全相同，无新增/消失诊断；不能称vet通过 |
| .NET SLMP | 生产文件及上游新增回归文件逐字取上游；不向Java移植 | 多块/字位混合、原请求顺序、错误传播、120块/960点拆分5项上游测试意图已审查；本机无dotnet，未运行.NET构建或测试 |

本地非Java绑定遵照 `plc4x-yofc/AGENTS.md` 的“非Java语言绑定一律取上游”政策。候选v1仅有一个摘要注释差异；全部原任务终态后恢复为上游原字节并保存v2指纹，v1文件及指纹仍保留。没有扩展自定义PLC4X测试、运行真实PLC测试main或改全局配置。

## Java与消费者

保留Maven model4.0.0、Maven3/IDEA兼容入口、Java21、Jackson3及YOFC `2.0.0.B-SNAPSHOT`。`yofc-iot` 的PLC依赖POM声明仍只消费Java `plc4j-*`，本批没有改Java源码或artifact坐标。Go和.NET不在该Maven消费链内；不能由Java构建推断其他语言或网关已验收。

隔离候选使用原脏工作树的11929个精确输入，独立Git元数据与APFS私有Maven/Go缓存。串行 `install -Pwith-java -pl '!website' -DskipTests --fail-at-end`：97模块通过、2470构建目标完成。现有Java全量 `test ... --fail-at-end`：97模块通过、575个JUnit报告、11764用例、0失败、0错误、102跳过（11662未跳过用例）。无源码漂移，未向共享Maven缓存安装自己的产物，原共享Kura API JAR仍为SHA `52205637a2742e99d052bff835ca68b41d83b774bbcb7d76403e2c35316114ed`。

build-helper3.8的add-source和plc4x-server附加产物目标实际执行。exec目标属于未选择的website/Python/.NET路径；OPCUA/KNX的XML transform只在 `update-generated-code` profile内，本批未开启。因此本批普通构建/测试通过不代表代码重新生成、IDEA实际UI导入、website或非Java构建已通过。

POM盘点还发现3个既有非Java生成器测试资源POM的起始project标签不合法（`modelVersion`被插到标签属性之间），不是Velocity解析问题；它们不在本批所选Java reactor中，源码未改。该问题和未执行的可选生成流程单列内部待办，不能改记为通过或新增外部deferred。

## 现场与后续

新增验收没有遗留测试容器；正常console/PG的ID、StartedAt、healthy状态与G022开始前一致，Orb网关保留运行、Parallels两VM保留停止，54个既有R013任务容器仍停止。V41历史运行中断仍保留，不宣称整个长期目标从未中断。

新的原源码权威为 `target/upstream-logical-sync-20261112/applied-source-pins-after-build-plugins-v17.json`：509个存在路径（更新原4项、新增1项）加89个保留删除路径。管理体系仍是同一V46 shadow，2081源码+4fixture、180路径未整体应用；R013代码或正常agent均未部署。后续保留校验必须先读G022新权威，不能再对原PLC工作树要求V15的508项旧字节。

上游同步继续按阶段执行：11fork检查指定上游游标并按功能、前置和消费者处置；7nonfork跳过直接merge，使用现有327条Maven加1条Cargo关系及源迁移记录分析影响。G021已证明Kura5943的27个DS配置等价（含nonfork kura-cloud的6项）；CM有/无配置时PID标量的实际运行断言仍随新的联合Kura验收推进。

下一步回到R013：在同一已settled owner/current/transaction/history域内实现foreign/NM物理generation变化的只读source续接；保持逻辑revision及Kura-owned raw/handles/声明/receipt来源，不能伪造ACK或靠名称认领。随后仍须处理owned身份变化的合法恢复、Kura-off authority、自动到期/原调度、UI/task/protocol、完整真实registered生命周期及180路径整体guarded apply。原14组、7API2、R017、配置API1.2/构建1.8运行配对、界面部署/TLS reload/bulk/protocol发布校验等完整内部台账保留。

仅3项external deferred：真实NM/cloud-init/物理硬件、amd64、disk-full/hard-power。方案A外部CA/custody保持。所有CLOSED一次性场景禁止重放，UNKNOWN不重发、不换ID绕过、不重导出。周额度实际耗尽时仅做必要收尾并按已授权暂停；不使用额度重置券。
