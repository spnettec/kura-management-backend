# YOFC Kura 续接交接：Main 布局维护前准入

2026-11-18。用户明确要求“这轮完成后，停止目标，然后给我一段交接的提示词，我从新的会话开始”。本轮 phase33 已完成代码、实际验证、文档和父知识库落盘；收尾准备好后将目标置 `paused`。最终工具状态见回执目录 `pause-goal-result.json`，不把暂停记为目标完成。不要在本会话开启下一轮实施。

## 工作空间与读取顺序

工作空间固定 `/Users/heyoulin/iot-kura-develop/git`，18个独立Git仓，无umbrella Git。共同本地分支 `wip/management-close-20261116`，必要本地WIP提交已获授权，不推送、不reset/clean，不清理个人配置/密钥/数据或运行中的 target/runtime。JDK最低21，不追加27。用户已授权使用现有积分继续，不再把先前“周额度尚未恢复”当作等待条件或再次征询积分许可。

新会话先全文读本文件及根/受影响仓库 AGENTS.md，再通过官方 Obsidian CLI 读取父工作空间最新暂停交接、Tasks.md、Session Summaries.md；不要混入全局或按子仓建重复知识库。manager：`python3 ~/.local/share/workspace-knowledge/kb.py context --dir=/Users/heyoulin/iot-kura-develop/git`；已有成功摘要不重复。`vaults.py ensure --project-root /Users/heyoulin/iot-kura-develop/git` 当前返回 selector `f9b864007dcd8a44`，每次使用 manager 当前返回值与精确 `path=`，CLI失败不得退回直接读写知识库Markdown。

然后全文读取：

- `kura-management-console/docs/checkpoints/20261116/OPEN-TASKS.md`、`README.md`。
- `kura-management-console/docs/checkpoints/20261116/core-main-layout-admission-checkpoint.json`。
- `kura-management-console/docs/reviews/core-main-layout-admission-review-20261118.md`。
- `kura/docs/management-main-layout-admission-20261118.md`、`management-main-boot-choices-20261118.md`。
- 原完整交接 `kura-management-console/docs/reviews/continuation-paused-20261117-installed-main-archive.md`；后续结果按日期和本文件覆盖范围使用，旧证据不重建。

代码发现优先 codebase-memory-mcp，按真实子仓 root_path。Core项目 `Users-heyoulin-iot-kura-develop-git-kura` 已对本轮代码更新索引，03:06UTC状态 succeeded；新会话仍核当前覆盖，不索引父目录。非代码脚本/配置/字面量或覆盖不足时再用文本检索。

## 最新 Git 与完整台账

本轮实施文档检查点 Core `d49e65824f8efa27e8b83685906960d2f012995c`，tree `a11ba8b4c65e5e66e12a8f91181728ec039fb884`；GWT `18c1b5eed4bf1483fb19756f6ea7ca817bd2732e`。console在暂停交接提交前为 `3fff782baae4849da2d2ee837b2b86cb3cfec48c`，随后仅追加本暂停交接文档提交。**包含暂停提交的确切18仓HEAD/tree/status见本地 `pause-final-repositories-and-ledger.json`，并以新会话实时Git核对为准。** 不回退到最初用户给出的de5/f135/4fc旧检查点。

本轮最后实施回执 `phase33-final-repositories-and-ledger.json` 已确认18仓干净同分支，其他16仓与phase32完全一致。冻结12父项块从“1. **完整密钥生命周期**”到“## 下一步建议顺序”之前，SHA256 `4e1aaca54eb94c3fc36266506c87a0781498aa797f0c18a01036f74a21584cc7`，全文未缩减。

最终12父项闭环仍0/12，无可信整体加权百分比。完整密钥生命周期/provider、CA custody、最小权限/完整发行、原14组/API、真实接管链、完整网络矩阵、中心与部署GUI、客户数据库/协议、R017/文档技能、G021/G022、上游逻辑同步和正式源码发行全部保留。G033/V61为最后接受、V51为最后full seal；G034未封签、协议未正式发布。

仅3项 external deferred：真实NM/cloud-init/物理硬件；amd64；磁盘满/硬断电。合法旧态转换、provider/launcher缺项、产品恢复、UNKNOWN、资源/构建问题等均为内部pending。11个其余sibling的有界代码所有权/提权脚本路径/失败传播修复已落地，不能外推为完整六组件/provider/runtime矩阵通过。完整范围以OPEN-TASKS全文为准。

## 本轮实际完成

代码 `e99df17793799a6e506ef37b5c1a82e7140193ba`：不支持的Main UnitFileState在initialize、新维护围栏和stop之前拒绝。schema2/static还核固定vendor unit与原manifest generation.unitSha256；不认领override、外链或新key来源，不放宽Root配对。缺Main仍保留独立冷恢复基础服务；已有原围栏时不把unpack中间态重新解释成新准入。

66相关宿主回归选择，65通过、1个既有Linux renameat2不可变Root归档用例在macOS skip。新Git/JDK21/Maven3.10.1/private缓存顺序构建support、Core/tools、distrib均exit0。新ARM64包 `6.0.0~git202611180255.e99df17-1`，SHA256 `f99692a65cd768936864f4705b68f2e508a54d58ca57969fe134a1e659ddb0a1`，203550910字节。39 key/generation文件、3 key单元、Main SHA、精确内嵌lifecycle/clock guards与Root载荷父目录已核对。

原7fd从完整44正常升级一次exit0，原38输入保持，新增1正常package-history，总39/15。该次维护 `1afecd19-704e-403d-87ba-7da5b39a39ee` 已完成，不重做。三项完整真实dpkg拒绝为 linked-observed、linked-runtime、未配对static；各一次remove exit1，39原数据及所选代码/链接完整指纹、5辅助单元属性/活跃进程ticks精确，无新围栏、retained、upgrade stage或key/clock业务。它们是**拒绝路径**证据，未完成合法旧static转换或linked审核迁移。

首次linked另有不完整观察：socket MainPID采集先于dpkg失败，修正续接后实际remove已知拒绝，但CLI脱敏消息断言失败，前helper快照未持久化。原记录保留、不补写、不计完整通过；只读核39数据/代码/无新ID后仅恢复夹具布局，再做独立新观察。合计4次已知拒绝、3次完整观察，没有UNKNOWN或Root UUID业务重发。所有验收之后才单独恢复夹具vendor布局和dpkg install选择，不把夹具操作算产品恢复/迁移。

## 当前可续用隔离 Core 现场

- CID `7fd8c6a0967a58a95c123785e5feff68cd0ce74e370d00ee875f9e215ac08d7e`，image `sha256:3b563783bf35e04b48a0495e7b84d7975ccea991d3d05dc5eec7420269b9cdfb`，Ubuntu24 ARM64/JDK21，network none，无host/runtime bind，无SYS_TIME。
- 完整安装e99包，正常停止exit0，Main disabled/inactive/PID0/NRestarts0/ExecMainStartTimestampMonotonic0；Main和actor从未启动，clock namespace不存在、key/clock业务/startup0。
- 39原输入/15正常回执。账号kura1001/1001、kurad997/991、recovery996/990、clock995/988；clock原账号目的 `2a388683-4a21-4616-95bd-41efbf631da0` 保持。原钥、HMAC、历史.data两标记、Root0600数据canary、packages/dpa.properties均保留。
- key generation代码cohort `a00fc81927ab5e2ad4b7c5781cddb7d39c64271489f09bff3d317f38dab42a30`；clock canonical/actor/effective unit配对通过，actor protocol cohort `f9ae58a642d5a28039aa8c2ecf86c4428d37ec6d933dfb794b0b9764a4cd2822`；clock manifest文件SHA `fbb6052c25d3a3b1f827606a8d205e747879a4c6e1b6b4a74ae55cb031eba5f9`。勿混用两类SHA。
- 当前39份完整指纹基线在回执 `private/final-healthy-current-boot-observation.json`（外层returnCode/stdout，stdout为JSON）。不可重新初始化、清数据、改Root/账号目的或重做已完成profile目的。
- 本轮重挂overlay使38原文件仅st_dev从1048667变1048652，其余九字段精确；旧观察保留，当前boot独立分类。以后重启须独立核实实际差异；不得把忽略device或重绑持久Root当通用规则，尤其不得对有业务/namespace的原现场套用。
- `/fixtures/phase33-core-main-layout.deb`是Root0600的实际流式发布包。已安装包是现状，不要求依赖旧build target；后续改动按Git和当前环境构建。不要重建已闭环证据链。

## 必须保留的运行现场

03:03UTC实际只读复核通过，详见本轮 `phase33-preservation-after.json` 与 `phase33-e-readonly.json`。后续变动前再次实时核对。

| 原现场 | CID | Core PID / start ticks | 已有业务计数 |
| --- | --- | --- | --- |
| 原RPC | `863ea5ecfeb0558eabe4c110a5973efd26fbad58e076c48080b2e9bba5a40f0a` | 3792 / 4469038 | operations0/startups0 |
| 原cold | `a04bfb08590b93fa3351447ded42c24b576c676699216f3636ff3ff658190c07` | 5174 / 4319777 | operations1/startups2 |
| 原primary | `5f6b45125fa35903f4c725aeadfdaa22025c4f2b62e33d156309fe09baf629bd` | 10430 / 3504160 | operations4 |

个人25文件、C9及原进程计数、D29及Main3135/ticks16649159保持。C `ebdbf56457231cb710ae7eab759d254d8ba54d321480a0d82c0af5415b600f4a` 的丢失CSR/UNKNOWN保留，不重生成/重签/重绑/换ID/跨Root查。D `5a4b2b32d7a5cfb1c87d56f2486364f86d47155c7f068179d70955c87c50ca39`为原中心，勿重启或更改数据库闭环任务。

E `a60111a62e828e5b50535252a98e3eb3b219fc95f65a1565cdd1fd8dd0944414`：native29323/ticks21272446、Core29609/ticks21272461、key4714、recovery4640、NM4668及原身份20/Core6/local7/material5/switch9/transport7、ON均保持。实际native代码eab6821/文档b6d29aa，Core已部署50855bc，不能拿新候选当已部署。原source `b479b232-38d2-436d-9566-3f0e33c25bb3` APPLIED，Root `24560864-f974-46aa-940a-0a97ad926118`，ON revision `5e6e429f-13e4-4066-a61e-2bb218c51cc1`。现用Main仍11 ambient/13 bounding caps、NNP0，最小权限仍pending；原生DB/WAL是活数据，不truncate/checkpoint/prune/恢复旧副本。

原 UNKNOWN `767863a9-fe6b-4135-9783-ddcb131030af` 及原startup.stop保留，不重发、不换ID、不在其他Root查。已闭环备份、归档、轮换、恢复、900秒续期及本轮之前正常profile目的不重放，完整已完成ID列表沿原交接与phase32检查点读取。

两台Parallels均stopped：Ubuntu `8061d57e-0120-42ec-a9a9-5799b436c4eb`，Windows `f077f2f9-0a5c-4347-8037-04214e00ca9b`。仅复用现有Ubuntu，不增开重型VM。原clock namespace `a23fb4e9-1621-4d82-9d53-7b9a1d94e391`，唯一+1ms `c35584c9-c3ba-45b6-8305-673e7b576353`已APPLIED，原maintenance `4322e5d4-a9bb-4bdb-ab43-a853fad78fb0` 已quiesced；不要启动旧JavaProbe、重建/run dispatch或重放+1ms。完整当前合法Main/OSGi clock业务仍pending。

## 下一轮实施

新会话从当前Git和完整原现场继续，先核18仓/分支/HEAD/tree和上述现场。优先处理合法旧static包的配对保留转换与其他旧态，再推进完整Core/OSGi、当前合法Main业务、RTC/Chrony和其余typed brokers、完整六组件/provider/launcher/安装升级移除purge/通用中断与半配置恢复、资源/平台矩阵及全部12父项。不要把本轮拒绝路径或先前四状态局部通过外推为完整矩阵，更不能把内部pending移为external deferred。

GUI仍次要；实际HTML/CSS/clientside JS任务前先执行modern-web-guidance。保留NetworkManager，agent始终运行、远程开关仅控制中心通道；Kura可无agent独立运行，原GWT仅装agent才显示远程菜单，Core自己生成私钥/CSR。方案A外部CA，不把中心默认改成保管CA私钥。

## 本地回执与知识库

本轮回执根：`/Users/heyoulin/iot-kura-develop/git/.management-close-20261116/source-cohort-20261117/core-main-static-linked-20261118/`。其中实施总结、暂停后18仓最终指纹、目标工具暂停结果分别为 `phase33-final-repositories-and-ledger.json`、`pause-final-repositories-and-ledger.json`、`pause-goal-result.json`。私有日志/指纹只在0700/0600本地目录，不进Git/知识库，不复制密钥载荷。

实施知识笔记 `Experience/2026-11-18-Kura-Main布局维护前拒绝-d49e658-3fff782-e99df17-b73e7453.md` 已官方CLI全文精确readback，Tasks/Session对应标记各1。随后以用户暂停要求追加最新暂停交接；确切path和Tasks/Session最终SHA见 `kb-pause-readback.json`。新会话读最新暂停交接，不把历史active记录当作当前目标状态。
