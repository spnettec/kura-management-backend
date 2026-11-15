# YOFC Kura 阶段性源码检查点 — 2026-11-16

本检查点按用户最新要求提交、推送并清理可重建中间产物，取代此前“不提交推送”的阶段限制。它保存未完成的工作，不是正式发行或完整验收。后续从这批 Git 源码和 [未验收台账](OPEN-TASKS.md) 开始，不需要恢复旧 `target/`、旧 Maven/Cargo 缓存、证据清单或源副本。

18 仓共同使用**本地**分支 `wip/management-close-20261116`；按用户最后确认，未达到主线合并条件的代码只留本地，**没有推送远程 WIP 分支**。每个有修改的仓先保存原工作树，再保存完整管理候选；可通过提交历史回退重构。干净仓也保留同名本地分支。447路径中仅2个独立CI文件已合入自己的远程 `spnettec/kura` 的 `heyoulin-kura` 主线，提交 `dc788aa2361d2c14971c0b45533848e83f17aadd`；其余445路径保留本地，见 [候选归并审查](CANDIDATE-REVIEW.md)。远程提交从原远程主线单独建立，没有顺带推送本地主线中另3个未推送的检查点提交。最终本地提交与远程读回见 `.management-close-20261116/`。

管理候选共 447 个路径，涉及 core、networking、GWT、agent、console、Docker。用于历史隔离构建的 3 个生产树替代文件与 4 个小 reactor 单独保存在 [build-fixtures](build-fixtures/)，没有覆盖正式 reactor/生产清单。新增密钥代号代码、配置边界测试和 JDK 最低版本修复一并保存。此处的 WIP 源码归档不替代原“完整 guarded apply / 正式发行”验收门禁；G034 未封签，最后接受 G033/V61、最后完整代码封签 V51。

## 已保存的最新工作

- 旧密钥迁移、旧加密数据连续性、独立 Root 密钥服务、普通 GWT 加密备份/严格原钥恢复及冷恢复的有限范围结果保留；闭环写操作不重放。
- Core12/GWT6 的独立只读恢复资料 RPC 与登录分支已修复，既有 303 个服务端/共享/RPC policy 条目保持原接受边界；本轮没有再执行旧写验收。
- 独立 PostgreSQL 合成 schema9 库的 103 条历史密文任务，真实 dump/restore、9→26 迁移、rekey 与旧备份再次恢复验证通过，17 条 UNKNOWN 保持；这不是客户备份或完整 GUI 数据库恢复验收。
- 新增多代密钥读取、新密文认证代号、严格凭据校验和认证后才返回明文的流；临时文件只保存密文。尚未发布 Root 轮换凭据，没有实际换钥。具体合同见 [key-generation-contract.md](key-generation-contract.md)。
- 最新 JDK21 加解密 46 项、配置/快照边界 27 项、CI 入口 5 项通过；两个故意退回旧行为的反例分别产生预期断言失败。`kura.snapshots.encrypt=false` 继续写明文 XML，`kura_custom.properties` 的覆盖语义不变。
- JDK 最低要求是 **21，允许更高版本**，编译目标保持 Java21，GWT 保持既有 Java11 字节码目标。CI 候选入口已去除“恰好21”限制。用户要求后不再追加27验证；当时已经发起的27进程已终态。个人 `~/.mavenrc` 仍默认21，没有改动；旧的仅设置 JAVA_HOME=27 的 Maven 记录实际为21，不得误用。更高版本全平台兼容不据此宣布完成。

## 保留现场及禁止重发

运行现场在 Docker 自身数据层/卷中，见 [runtime-preservation.json](runtime-preservation.json)。三个网关、七个进程、38 个私有文件完整元数据和六个旧数据文件内容在收尾前均精确。保留必要 live bind 输入及密钥/加密数据。不要用旧脚本重建、重启或重放现场。

| 现场 | 保留的进程 |
| --- | --- |
| `yofc-g034-key-service-lifecycle-20261115` | Core10430 / key10576 |
| `yofc-g034-key-cold-install-20261115` | Core5174 / key4735 / recovery4893 |
| `yofc-g034-rpc-isolation-20261115` | Core3792 / key4229 |

必须保留的原 UNKNOWN：管理操作 `767863a9-fe6b-4135-9783-ddcb131030af`；startup.stop 意图 `a87d94d52839e316845b7c831b265daf928f0bfd5fb0a883fb49b721cba5f63c`。只读原记录；不重发、不换 ID、不手动 stop、不重导出，不从 PID、耗时、空日志或新环境推断结算。

以下闭环 ID 永远只读：`965a1c8b-5acb-40ce-854e-b5592df80239`、`75d62ca7-cec5-47de-b136-9415e59dc226`、`fd78f9ad-d41d-4651-98bd-7a664f167255`、`5a7dfd2d-51bf-4431-bb67-7995643b0c4b`、`8fee5e2b-343e-4d71-b09d-7388af348ed9`、`f4115360-1598-4e2d-98ff-29978639936c`、`57d19ee1-a8e8-4fc6-8dba-36cd4cc6bcd9`、`10296cdc-4a25-46e9-b4b4-8993fb31bac6`。新 RPC 现场的空 journal 不能结算旧现场原记录。V36v1 不做 live query/inverse/reissue，既有 GWT 三次保存等旧 CLOSED 验收不重放。

## 从干净环境继续

1. 读取工作空间 AGENTS.md、工作空间知识库最新交接和本目录台账；使用官方 Obsidian CLI，仍为父工作空间，不建重复 vault。
2. 核对 18 仓当前 Git 分支/提交、源码与保留的实际运行状态。Git 是源码权威，旧大型证据链不再是启动前提。
3. 按需重新构建缓存与制品；不要清理个人 `~/.kura-dev` 或正在运行的 `target/runtime`。旧现场只做必要原记录读取。不要重放已闭环验收来“恢复证据”。
4. 先做源码全盘 review，继续完整台账。需要重构时可从 WIP Git 检查点回退并重做；不能把未验收项标为完成或降为 deferred。

本次历史规模、报告计数和封签仅用于说明边界，不要求恢复同名旧日志。后续验证直接以新 Git 源码、实际环境及相应需求为依据。
