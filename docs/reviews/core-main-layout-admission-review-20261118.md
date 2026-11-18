# Core Main 布局拒绝的维护前验证

2026-11-18。源码 `e99df17793799a6e506ef37b5c1a82e7140193ba`。本轮把不支持的 Main 布局检查前移，实际验证拒绝之前不改变原数据和辅助服务；不把拒绝验收称为这些布局已支持迁移。上一 goal turn 为状态答复和只读检查，分类 no progress；本轮新增代码、回归、新源码发行包及实际系统证据，分类 progress。目标保持 active。

## 实现与检查

原 Lifecycle.enter 只检查 key 辅助单元覆盖，在维护围栏和 stop 之后，retain_main_state 才查 Main boot choice。现在没有原围栏、有既存 Main/systemd 时先查 UnitFileState；不支持的 linked/linked-runtime/alias/indirect/generated/transient/空值或查询失败在 initialize、新围栏和 stop 之前拒绝。对 schema2/static，固定 vendor 单元还必须匹配已有 Root manifest 的 generation.unitSha256；手工移除 Install 的 unit 不是已支持 static 包。没有引入 override 认领、外链 key source 或额外 key 读取，也未减弱 Root/代码/UNKNOWN 门禁。

缺 Main 时独立冷恢复基础服务不依赖完整 Core reader；原围栏存在时不重新解释 unpack 中间态，保留原维护意图及后续校验。旧合法 static 转换仍待实施，不能凭新拒绝分支关闭旧态矩阵。

宿主相关选择66项，65通过、1项既有 immutable Root archive/renameat2 在 macOS skip。6个新测试方法覆盖新准入拒绝、查询失败、未配对 static、原六选择、缺 Main 的冷基础服务维护、原围栏 reentry。不同选择重叠不累加，不重放此前 Linux 归档或业务。

新真实 Git 检出、JDK21/Maven3.10.1/private Maven 缓存依次构建 support、Core/tools、distrib，均 exit0；不是旧 target 或共享项目缓存输入。包 `6.0.0~git202611180255.e99df17-1`，203550910字节，SHA256 `f99692a65cd768936864f4705b68f2e508a54d58ca57969fe134a1e659ddb0a1`。39个 key/generation 文件及3个 key unit、Main generation SHA、精确嵌入新 lifecycle、clock guards、控制脚本语法和载荷父目录 Root 权限逐项检查。没有再次启动原 VM 或运行旧 clock +1ms 业务。

## 原现场与一次正常升级

18仓初始全部与 phase32 完整 HEAD/tree/status 一致，同本地 wip 分支；冻结12父项全文 SHA `4e1aaca54eb94c3fc36266506c87a0781498aa797f0c18a01036f74a21584cc7` 保持。父知识库使用 manager selector f9b864007dcd8a44，最新交接/Tasks/Session 全文 readback 与前轮 SHA 一致，未写全局。

原7fd健康安装44/38输入后正常重启隔离 PID1。38份文件仅 st_dev 从1048667变成1048652，其余 inode/uid/gid/mode/nlink/size/mtime/ctime/SHA九字段精确；同CID/image/无bind，Main未启动、无 GWT sender binding、无 clock namespace 或 key业务/startup。旧观察不改，本 boot 独立分类并记录新基线；产品持久 Root/namespace 检查不变。

完整 e99 包在同现场唯一一次正常升级 exit0，原38输入保留：30外部完整指纹、8Core数据逻辑字段精确，正常 package-history 从14到15，总39文件。Core复制产生的 inode/ctime变化保留，不声称跨升级全元数据或 atime 一致。原 clock 账号目的2a388683-4a21-4616-95bd-41efbf631da0、UID995/GID988和其他数字身份保持；历史 .data 和 Root0600数据 canary保留，未当作代码执行或统一 chown。

## 实际布局拒绝

在本 boot 的升级后39份完整指纹基线上，夹具分别选择下列布局。linked 将原 Root vendor 文件移动到自有固定 `/fixtures/phase33-main-layout/<case>/kura.service` 并由真实 systemctl link 设置；未配对 static 仅在夹具临时 vendor 单元移除 Install，原包 manifest 不改、原 vendor 另存。没有借此启动 Main、修改 key 或初始化新网关 Root。

| 独立完整观察 | 单次真实 dpkg remove | 拒绝期间证据 |
| --- | --- | --- |
| linked-observed | exit1，维护准入拒绝 | 原39数据及所选代码/链接完整指纹、5辅助单元属性/活跃进程ticks精确 |
| linked-runtime | exit1，维护准入拒绝 | 同上 |
| unpaired-static | exit1，维护准入拒绝；只读配对检查明确 unit 与 manifest 不符 | 同上 |

每次在动作前持久保存 Root0600 observation，动作结束立即保存 exit code，随后核对；Main均inactive/PID0/ExecMainStartTimestampMonotonic0，未创建维护围栏、retained、upgrade stage、clock namespace或业务/startup。CLI保持脱敏拒绝消息，具体原因由已配对 Root helper 的只读 layout 检查观察，不修改产品日志策略。

各拒绝观察完成之后才单独恢复夹具原 vendor 布局，恢复数字属主/模式/nlink/size/mtime/字节并重验完整 generation；代码文件 rename 引起 ctime 变化是夹具选择/恢复，不冒充原始元数据不变。dpkg 因 operator 尝试 remove 把 selection 置 deinstall，这是实际结果；全部观察完成后另记意图恢复夹具原 install 选择，未再次调用产品安装或移除。

## 首次 linked 观察的保留限制

首次选择 linked 后，采集器错误地要求 socket 也有 MainPID，尚未调用 dpkg，原失败保留。修正采集后精确续接同一尚未 dispatch 的动作，唯一一次 dpkg remove exit1、无围栏/retained，但采集器误要求 CLI显示具体 Refused原因；CLI本来输出脱敏消息。调用前辅助服务快照只在内存，未持久化，不补写、不把该次计完整通过。

随后只读确认39份原文件完整精确、代码字节配对、原拒绝日志和无新维护ID，单独恢复夹具布局。修正采集和消息断言后的 linked-observed 是独立新观察，保留第一次不完整记录；没有重发 UNKNOWN 或任何已签发 Root UUID 业务。合计4次已知拒绝的本地 remove尝试，其中3次完整观察；不是4次成功移除，也不是通用中断/产品恢复通过。

## 最终状态与完整目标

最终完整 key generation cohort `a00fc81927ab5e2ad4b7c5781cddb7d39c64271489f09bff3d317f38dab42a30`；clock canonical/actor/effective unit配对通过，actor protocol cohort `f9ae58a642d5a28039aa8c2ecf86c4428d37ec6d933dfb794b0b9764a4cd2822`，clock manifest文件SHA `fbb6052c25d3a3b1f827606a8d205e747879a4c6e1b6b4a74ae55cb031eba5f9`。两类SHA分别记录。39输入/15正常回执、原身份/密钥/历史数据，Main disabled且从未启动、actor未启动，key/clock业务0；原7fd完整安装 e99、正常停止exit0。下一轮沿新 final-healthy-current-boot-observation 继续，不重初始化或重做此前已完成维护目的。

03:03UTC 原3/个人25/C9与原进程计数、D29/原Main3135、E原身份20/Core6/local7/material5/switch9/transport7及原进程/ON配对均保持；两VM实时stopped。本轮未部署到原运行场景；E实际Main仍11/13caps、NNP0，不能把候选修复当已降低现用权限。

合法旧 static/其他旧态迁移、linked审核迁移、purge、通用中断/半配置恢复，完整 Core/OSGi、当前合法 Main、RTC/Chrony/其余typed brokers与全部六组件/provider/launcher/资源/平台组合继续内部pending。原14组/API/R017/GUI/网络/密钥/CA/客户数据库/上游逻辑同步/正式发行及完整12父项全部保留，最终0/12、无可靠加权百分比。仅原3 external deferred，G034未封签、协议未正式发布。现有积分继续、goal active，无push/JDK27/活动target/runtime清理；UNKNOWN/丢失CSR不重发换ID跨Root查，已闭环备份/归档/轮换/恢复/900秒不重放。GUI仍次要，本轮未改clientside JS。

回执：父工作空间 `.management-close-20261116/source-cohort-20261117/core-main-static-linked-20261118/`。见[检查点](../checkpoints/20261116/core-main-layout-admission-checkpoint.json)。Core合同 `docs/management-main-layout-admission-20261118.md`。
