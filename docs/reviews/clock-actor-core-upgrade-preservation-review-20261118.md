# 当前 actor 权限与 Core 升级数据保留

2026-11-18，Core服务选择修复 `b49eb6cb204f2e7b4ac00322b45187354bf44847`、数据恢复修复 `e1c40469d9807c0518a34b1f9e24b033df2d5d98`、合同 `aa8f086b50a994354d6cc877085d8c9839cab64f`。继续原全目标，未关闭完整 Core/OSGi、六组件矩阵或12父项。

## 当前非 Root actor

在原 Parallels 组件现场先实时核对机器、static且未启动的旧Java probe、原 namespace `a23fb4e9-1621-4d82-9d53-7b9a1d94e391`、唯一原APPLIED和全部sender文件。沿原维护目的 `1284ae68-2e9e-4f28-8e1b-c62fc1119cf1`，只归档/发布新Git完整包a54中核过的7个clock代码/unit/JAR和manifest，由产品install归档原actor，再resume原目的。没有重NSS身份、复制真实数据、重建namespace或触发旧dispatch。

新实际actor cohort `07212c1f…9129304f`、JAR `6a67ff22…d9c05cc8`。只发1次不携带namespace/ID的Root context拒绝请求，得到UNAUTHORIZED；启动及请求处理已通过当前代码/已加载unit配对。实际PID4181/ticks41703，UID992/GID970四类一致、附加组仅970、五cap均`0000000002000000`、NNP1、NRestarts0，固定真实python入口符合unit。10份helpers/units/Java/manifest/DS与e1当前逻辑逐字节一致，后续仅安装恢复行为变更。

原namespace/operation/sender字节及指定元数据精确，操作数仍1、新clock业务0，旧+1ms未重放，没有向其他Root发送旧ID。此为真实actor身份/权限、有效unit自配对与未授权拒绝，**不是当前合法Main写业务或完整Core验收**。之后正常quiesce，停机维护目的 `4322e5d4-a9bb-4bdb-ab43-a853fad78fb0` 保留同一namespace/当前cohort，units inactive/PID0，正常guest poweroff回stopped。完整a54 deb未在VM安装，仅clock代码cohort续接。

## 两处实际升级修复

Main原安装脚本无条件enable。b49改为在标准vendor/admin/runtime位置已有unit时保留enable/mask链接，避免改写disabled或enabled-runtime选择；全新服务reload后enable，reload/enable错误传播，升级消息不再承诺强制自启。5项临时文件系统/模拟systemctl合同通过，实际disabled profile见下文；其他实际状态/新模式仍待验。

实际b49升级核对又发现，`.data`新默认副本优先于保留副本并无条件刷新恢复快照。当前默认夹具只刷新两副本mtime，字节/属主/模式相同；校验原失败保留，没有回填时间戳。源逻辑可能覆盖不同历史内容，因此e1改为旧addon-data优先，已有历史snapshot不重写/不跟随链接，只填缺失副本且传播失败；可信addon code仍由独立Root libexec配对。移除恢复后user/data/packages/log递归chown，保留原数字属主/模式，再做code-only核对。5项历史内容/缺失/链接/失败/不chown合同通过。

所有包均从新真实Git/JDK21/Maven3.10.1/private Maven缓存重建support/Core/tools/distrib，不复用旧target、不写个人共享项目缓存。生产clock逻辑/Java未变，前49项Python/57项JVM不重跑或累加。

| 同一原7fd8…现场 | 实际包/指纹 | 原输入验证 |
| --- | --- | --- |
| 9066→b49单次dpkg，exit0/installed | `6.0.0~git202611180028.b49eb6c-1` / SHA `ca68e097…669fd3` | 外部17完整指纹、4份权威user文件逻辑字段一致；2历史副本mtime刷新如实记录 |
| b49→e1单次dpkg，exit0/installed | `6.0.0~git202611180039.e1c4046-1` / SHA `5df08c91871b3f922811b7f14bf226ea964b9b9bbe856196c72afff0ebdb66d5` | 原25输入：外部18完整指纹、Core数据/历史7份逻辑字段一致，Root/Root0600数据canary及2历史标记保留 |

第二次前明确在自有新夹具归档原两副本、追加公开历史标记、添加Root0600公开数据canary，权威user快照和密钥未改。这不是客户数据或旧证据链重建。外部完整字段dev/inode/uid/gid/mode/nlink/size/mtime/ctime/SHA；Core逻辑字段uid/gid/mode/nlink/size/mtime/SHA，复制产生的inode/ctime差异保留、自查NOATIME、atime未断言。每次新增1份正常package-history，旧回执保留。

Main一直disabled/inactive/PID0/NRestarts0，升级后没有手动补disable；clock actor未启动，namespace/operations不存在、业务0。原clock账号目的2a388683…、UID995/GID988不变。最终e1 actor cohort `a16faa5e…5578ac4`、manifest文件SHA `1798b28d…aa623c`（二者不混用）、JAR `90ed7ecb…07be57`。同一容器 `7fd8c6a0967a58a95c123785e5feff68cd0ce74e370d00ee875f9e215ac08d7e` 已正常停止，全部数据/标记/Root属主/回执保留；不能重当空主机或误启Main。私有日志/指纹仅本地0700/0600，不进Git/知识库。

回执目录为父工作空间 `.management-close-20261116/source-cohort-20261117/core-clock-actor-current-runtime-20261118/`，见[检查点](../checkpoints/20261116/clock-actor-core-upgrade-preservation-checkpoint.json)。Core合同为 `docs/management-clock-broker-20261118.md` 与 `docs/management-core-upgrade-preservation-20261118.md`。

## 保留与继续

00:44UTC原3/个人25/C9及原进程计数/D29原Main3135再次精确；E配对/身份20/Core6/local7/material5/switch9/transport7，native29323/Core29609/key4714/recovery4640/NM4668及ON revision5e6e429f…保持。E实际Core仍11/13caps、NNP0。文档前18仓同分支干净、仅预期Core提交，冻结12父项SHA `4e1aaca54eb94c3fc36266506c87a0781498aa797f0c18a01036f74a21584cc7` 不变。

继续完整Core/OSGi/当前联合包合法消费者业务、六组件/provider/launcher/模式资源平台、最新e1首装/其他enable状态/保留数据重装移除purge恢复中断、RTC/Chrony/其余typed brokers/clock UNKNOWN容量及API/R017/GUI/网络/密钥/CA/数据库/上游/正式发行和全部12父项。仅原3external deferred；其他内部pending，最终0/12不等于实现0%、无可信加权总百分比。G034未封存/协议未发布，目标active/现有积分继续，无push/JDK27/活动runtime清理，旧UNKNOWN和丢失CSR不重发换号/跨Root查，闭环密钥/900秒/原+1ms不重放。
