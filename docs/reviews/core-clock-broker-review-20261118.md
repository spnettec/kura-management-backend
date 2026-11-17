# Java NTP 独立时钟 actor 与 ARM64 包

2026-11-18，Core 实现 `b72fa0de7f02ac36c1832c22ba06a4d37df954d5`、固定别名修复 `0f409edde9d33929c4de650765c86f4aab004d0a`，合同/证据文档 `43204a916a4dfffe36f369784e475cb884751e1d`。本轮推进最小权限父项的 Java NTP 写壁钟路径；RTC/Chrony、其他 provider 与生产 Main 权限尚未收敛。

## 实现与限制

`ClockServiceImpl` 通过 `SystemService` 取得正式数据目录，固定 helper 只接受 typed correction，不接受 shell/argv/路径。独立 `kura-clock` 账号仅 CAP_SYS_TIME、NNP1，不加入密钥组；Root 配对的四 helper、两 unit 和实际 Clock JAR 复制到该 actor 的专用只读树。actor 看不到 Core/密钥/agent 状态，Main 看不到 actor 私有状态和专用代码；密钥服务继续 cap0。

Root/PID1 创建固定 socket。actor 只接收当前 `kura.service` MainPID 的固定 helper 直接子进程，校验真实 UID/GID 与 argv，拒绝 Root 和相同 UID 的其他进程。原 Root 配对检查保持。固定 canonical 安装别名仅容许 Root 所有单跳链接，目标须是同父目录匹配版本名的真实 Root 目录、不可由 group/other 写入；不接受其他链接或目标。

sender/actor 分别保存原 namespace/UUID、prepare、issued/commit-intent 和不可变结果。prepare 不写时钟；commit 最多一次。回复丢失只在同一原 Root 查原号；已 issued 或 UNKNOWN/UNSEEN 不重发、不换号、不回落 `date`。仅本地证明从未 commit 的原 PREPARED 可完成第一次 commit。actor 在 syscall 前以当前 realtime 加偏移补偿 RPC 耗时，同 boot/30秒样本、边界溢出明确失败。失败、UNKNOWN 或恢复的旧样本不冒充当前 ClockEvent，也不继续写 RTC。

维护 guard 先核原安装代码；未决事务拒绝维护。独立账号及分步安装目的不认领未知已有身份；remove/purge 保留原 namespace、日志、维护凭据和 sender 数据。新代码前缀显式755以适配 umask077，既有目录不自动 chmod/chown。完整有效 unit/profile、升级中断和 UNKNOWN/容量管理仍待实施/验收。

## 有限验证

从明确 Git/补丁导出的新源码，JDK21.0.13/Maven3.10.1，Core33＋Clock24项 JVM 检查全过，无跳过。4份生产 Java/manifest/DS 文件与当前源码逐字节一致。当前 Python 合同/包装38项通过；8项 Linux 固定别名文件系统检查通过，后者没有 namespace/请求/校时。重叠测试运行不求和。

已有 Ubuntu26.04 ARM64 Parallels VM 的独立新组件现场加载真实生产 `ClockServiceImpl` 和新 Clock JAR。JVM UID993/GID971/五cap0/NNP1，actor UID992/GID970/仅CAP_SYS_TIME/NNP1，NRestarts0。一次显式 guest +1ms，ClockEvent1、APPLIED/errno0：namespace `a23fb4e9-1621-4d82-9d53-7b9a1d94e391`，原请求 `c35584c9-c3ba-45b6-8305-673e7b576353`，请求摘要 `d0552dafee426a3e0b60d0f6700b0a8664c396d4ea135d40c2729f1c2f7259cb`。实际 cohort `e2d5a266…e46881e`、Clock JAR `b3cba7f6…e1b36a`。

Root/非Main peer 均 UNAUTHORIZED；额外代码与标准目录 unit override 均 UNPAIRED_CODE。公开 canary 的直接和 `/proc/PID/root` 路径均 EACCES，没有读取真实钥或 canary 字节；此为具体拒绝边界，不外推整个文件系统。随后实际观察确认事务数1、原 namespace/事务文件精确；首次 canary 回执的固定计数不作为证明。仅撤销本次自建且核对原字节的测试文件。组件正常 quiesce/停机，原日志和维护目的保留，guest poweroff 后 VM 回到 stopped。没有修改宿主或 OrbStack 共享内核时钟，也没有重放原校时。

从 `0f409edde9` 的新真实 Git 检出，以独立 Maven 缓存重建 build-support、整个 Core/tools 和 ARM64 distribution。完整包 `6.0.0~git202611172245.0f409ed-1`，203543034字节，SHA `e88f83022083f033b155a76c8db00d8cc2cfe02e8cfaa0e21ceae6ba99aa6313`；包内 cohort `7b87879b…f27b3f`、Clock JAR `a0889289…a35b438`。7份载荷/父目录 Root 权限、摘要、preinst/prerm 内嵌 guard 完整字节、四维护脚本解析、Main 的 socket 依赖与遮蔽通过。两个生产类与实际 VM 用类逐字节一致、major65；完整包/cohort 不同，**未实际安装或运行该完整包**，不能外推完整 Core/OSGi 验收。早期 b72 包缺固定别名支持，已由本包替代。

初始预检/GECOS/新父目录不可遍历、导出缺 Git 元数据、本 checkout 工具 target 缺失和检查器对版本目录/换行/systemd 多行的错误假设均保留原失败；沿原目的继续，无重身份或放宽产品检查。原始回执在父工作空间 `.management-close-20261116/source-cohort-20261117/core-clock-broker-20261118/`，结构化记录见 [检查点](../checkpoints/20261116/core-clock-broker-checkpoint.json)，完整合同见 Core `docs/management-clock-broker-20261118.md`。

## 保留与继续

22:45 UTC 原3现场/个人25/C9及进程计数/D29与 Main3135再次只读精确。E原配对、身份20/Core6/local7/material5/switch9/transport7，native29323/Core29609/key4714/recovery4640/NM4668及原ON revision5e6e429f…保持。18仓同本地分支，文档前只有本轮预期 Core 改动；冻结完整12父项 SHA `4e1aaca54eb94c3fc36266506c87a0781498aa797f0c18a01036f74a21584cc7` 不变。

原现场/E未部署本候选，实际 E Core 仍11 ambient/13 bounding caps、NNP0。下一步继续完整 Core/OSGi、安装升级移除恢复、有效 unit/profile 与故障矩阵、RTC/Chrony、其他 typed provider/broker/launcher，以及六组件/API/R017/GUI/网络/密钥/CA/数据库/上游/发行和全部12父项。最终闭环0/12不等于实现0%，无可靠加权整体百分比。仅原3 external deferred，G034未封存、协议未正式发布。目标 active/现有积分继续；无push/JDK27/活动runtime清理，旧UNKNOWN及丢失CSR意图不重发换号/跨Root查，已闭环密钥和900秒不重放。
