# Clock 有效 unit 配对与完整 Core 首装

2026-11-18，Core 实现 `9066ce7996f14332c34e21649bce235cadd0edba`、测试驱动 `817ff3cfa2d0ecdd9ef794012498e7baadaca521`、合同 `a54af33949f6d379af887b01bd9b53c623eaffca`。本次修复 clock 配对仅查部分目录的缺口，并在新隔离场景实际安装当前完整 Core 包、验证 clock 维护失败及原目的恢复。完整 Core/OSGi、当前 actor 的实际权限进程及六组件全矩阵仍未完成。

## 实现与真实检查

按 [systemd 加载规则](https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html)和 [PID1 属性](https://www.freedesktop.org/software/systemd/man/latest/org.freedesktop.systemd1.html)，从实际 UnitPath 检查每个 Root 保护加载目录，拒绝替代 fragment、control/attached/generator 输入、名称前缀/类型 drop-in 与 wants/requires/upholds；权限拒绝不当成不存在。已加载 unit 必须原 Id/Names、loaded、固定 vendor FragmentPath、无 SourcePath/DropInPaths、非 transient、无需 reload；超时、错误或缺失/重复属性关闭准入。原 Root 密钥配对检查未修改。

安装器在账号/状态/代码变更前检查 unit 输入。新 fragment 尚未 reload 时只做文件核对；resume 保留原维护 gate，直到产品 daemon-reload、加载配对、socket 启动及复核全部成功，再删除 gate。49项 Python 检查通过，新增11个方法；生产 Java/manifest/DS 未改变，没有重跑或累加前轮57项 JVM。

全新 Ubuntu24.04 ARM64/systemd255.4 容器，60个真实条件通过：37拒绝、23接受/恢复，覆盖12个实际加载目录、真正 set-property --runtime、全局/前缀 drop-in、替代 fragment、依赖目录和改回原字节但尚未 reload。首次 inactive unit 被 GC/重载，没有建立 stale 条件；加入 UID65534/cap0/NNP1 sleep holder，仅 After= 引用、不启动 clock unit 后通过实际 stale 检查。容器没有 SYS_TIME、网络或宿主/runtime 数据挂载；clock namespace/请求0，actor/socket未启动，holder已核验停止并移除，容器正常停止保留。

## 完整包实际首装与维护

从 Git9066 新真实检出，以 JDK21/Maven3.10.1/private Maven 缓存重建 support、整个 Core/tools/distribution。ARM64 包 `6.0.0~git202611172300.9066ce7-1`，203543854字节，SHA `c6a53072155e9ee89c1fe2ecad85901474962112b0cd92d7eba5422376bb918e`。7个载荷及父目录 Root 权限、摘要、内嵌 guard/维护脚本/Main 依赖遮蔽通过；两生产类与原实际 VM 字节相同。manifest 文件 SHA `57b7f686…f97e442c` 含最终换行；协议 actor cohort `cc0a32d5…43efaf7` 是 canonical JSON 摘要，二者不能互换。Clock JAR `28ea41f1…d65b11b`。静态回执记录“未安装”发生在以下首装之前，保留原回执，不改写历史。

第二个全新 Ubuntu ARM64/JDK21 容器按实际 Depends/Pre-Depends 准备，Root600校验流复制包，唯一一次 dpkg 首装 exit0、install ok installed。实际版本别名指向 `/opt/eclipse/kura_6.0.0-SNAPSHOT_linux`，两套代码、账号和有效 unit 配对通过。clock账号 UID995/GID988、仅自己组、不在密钥组；状态0700且仅空 admission.lock，namespace/operations不存在。Core Main/actor 均 inactive、MainPID0、NRestarts0；socket listening、NConnections0/NAccepted0。此为完整发行包实际首装，**不是完整 Core/OSGi 或 actor 进程实际 SYS_TIME 权限验收**。

实际 quiesce 原维护目的 `5e33127e-7433-4070-8f77-fe60c3f77480`、namespace null。加入本次自建 control override 后，resume 一次 UNPAIRED_CODE/exit1，原 gate 字节及 dev/inode/uid/gid/mode/nlink/size/mtime/ctime/SHA 精确、socket保持inactive。撤销核过原字节的自建输入后，同一原目的 resume 成功，由产品自己 reload/核对/启 socket 后删除 gate，没有手动 reload 或换目的。Core/密钥/账号23文件的上述字段精确（自查NOATIME、未断言atime），Core/密钥/NM原PID/ticks保持；clock namespace/请求0，actor和Core Main从未启动，夹具未发送密钥业务请求。

完成后仅为停放该新夹具禁用 Main，再正常停止容器并保留全部数据；首次默认安装的 Main 原本 enabled，普通 boot 未验。该新现场为 `7fd8c6a0967a58a95c123785e5feff68cd0ce74e370d00ee875f9e215ac08d7e`，后续不能当作新空主机重新初始化、删除旧数据凑准入或误启 Main。安装日志/23文件私有指纹仅留本地0700/0600目录，不进Git/知识库。

首次 PID1 exit255（载荷前）、复制501:20和新父目录777被拒绝、stale条件未建立、宿主安装驱动缺os导入均留原失败。os异常在实际dpkg前，核实 package/state均不存在、实际分发0后，沿原容器/包执行唯一一次dpkg。复制修复只处理核过封闭集合的本次新代码，不修改真实运行数据。回执在父工作空间 `.management-close-20261116/source-cohort-20261117/core-clock-effective-units-20261118/`；结构化记录见[检查点](../checkpoints/20261116/clock-effective-unit-install-checkpoint.json)，合同见 Core `docs/management-clock-broker-20261118.md`。

## 保留与完整范围

23:16UTC原3/个人25/C9及进程计数/D29原Main3135再次只读精确。E原配对、身份20/Core6/local7/material5/switch9/transport7、native29323/Core29609/key4714/recovery4640/NM4668与ON revision5e6e429f…保持；实际E Core仍11/13caps、NNP0。Parallels未启动，原+1ms/namespace/请求未再使用。18仓同分支、文档前仅预期Core提交，冻结完整12父项SHA `4e1aaca54eb94c3fc36266506c87a0781498aa797f0c18a01036f74a21584cc7` 不变。

继续当前 actor 的真实非Root/SYS_TIME sandbox准入、完整Core/OSGi、升级/移除/purge/恢复/中断、六组件/provider/launcher/模式/资源/平台、RTC/Chrony及其余typed broker、clock UNKNOWN/容量，以及全部API/R017/GUI/网络/密钥/CA/数据库/上游/发行/12父项。只有本次单个Core首装/clock维护profile通过，未替代完整矩阵。最终0/12闭环不等于实现0%，无可信加权总百分比。仅原3external deferred，G034未封存/协议未正式发布；目标active/现有积分继续，无push/JDK27/活动runtime清理，旧UNKNOWN和丢失CSR意图不重发换号/跨Root查，已闭环密钥及900秒不重放。
