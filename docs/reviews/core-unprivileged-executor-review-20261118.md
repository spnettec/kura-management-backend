# Core 同身份普通命令与最小权限前置

2026-11-18，Core 本地代码提交 `84d6a58f3cc2b40073af2475de7ecf0b6a63f944`。原带权限 JVM 仍依赖 su 丢弃执行权限；未来专用非 Root/cap0/NNP1 JVM 则不能再依赖 setuid su。当前为后一种精确条件增加同 UID 直接执行分支，完整契约见 Core `docs/management-unprivileged-executor-20261118.md`。

## 实现边界

命令用户名须与 OS 的 `ProcessHandle` 用户一致，且进程 leader 与当前线程的 UID/GID/附加组/五类 capability/NNP 完全相同；四种 UID/GID 均非零且同一身份、附加组无 Root、五 cap 全零、NNP1，才进入固定 env/timeout/argv 路径。可更改的 `user.name` 无法提供准入。字段缺失/重复/异常、读取失败、不同用户、Root、NNP0、非零 cap 均保留旧 su 路径。没有自动更改 command user。

目标用户名在一次构造开始时固定，避免与可修改字段的后续读取不一致。环境及普通 argv 仍为目标 payload 数据，显式 shell/超时/同步异步语义保留。该分支不提供跨 UID 委派，也不实现 Root 通用命令代理。

## 实际验证

从 Git 基线 50855bc 和精确暂存补丁导出全新目录，JDK21.0.13/Maven3.10.1 重建 test-support/API/util/Core 并打包，没有复用旧 target。33 项 JVM 检查全部通过、无跳过；新增 9 个方法。Core JAR SHA `262076b9349aa423a372cc6a5ebd2fb0bf73b41316196856ba990d1c4a236e98`、新类 major65。构建 source tree 与最终 Java 源码的 5 个受影响文件逐字节一致；后续仅 Linux 驱动和文档改变。

新建网络关闭、只读 rootfs、cap/PID/内存受限的 Linux ARM64/JDK21.0.12.1 容器，8 条条件通过。同 UID 非 Root、cap0/NNP1 两条含实际命令执行（正常及 user.name 伪造）；覆盖身份、参数/空参数/环境字面量、工作目录、显式 shell、超时和一次异步完成。Root、Root 伪造用户名、NNP0、CapBnd=0x400、Root 附加组、目标用户不符均拒绝新分支。真实非零 cap 只验证 bounding 条件；五类 cap 逐项拒绝另有 JVM 合同检查，不能合称全部真实 cap 组合已验。

首次 Linux 夹具因只读 rootfs 无法复制 payload，Java 未运行，失败容器/回执保留。修正为在从未启动的专用容器暂存公共测试文件并封存本地镜像，再创建只读运行容器。运行使用新编译 Core/API 和声明的 commons-exec1.4.0、commons-io2.21.0、slf4j-api2.0.11。第一次失败前列入的 slf4j2.0.18 未进入 JVM，最终实际条件使用声明的2.0.11。没有任何 Kura 安装或运行数据挂载，全部测试容器已退出；staging 容器保持 created。

回执目录为父工作空间 `.management-close-20261116/source-cohort-20261117/core-unprivileged-executor-20261118/`，检查点见 [core-unprivileged-executor-checkpoint.json](../checkpoints/20261116/core-unprivileged-executor-checkpoint.json)。

## 保留与剩余

21:08:26 UTC 原三现场/个人25/C9及原进程计数、D29私有与原Main3135只读精确。21:09:59 UTC E 的原 Root 配对、原初始化命名空间及 source b479b232… APPLIED、身份20/Core6/local7/material5/switch9/transport7精确；native29323/ticks21272446/cap0/NNP1、Core29609/ticks21272461及 key4714/recovery4640/NM4668均保留，原 ON revision5e6e429f…精确。观察器初次误用仅接受 Root 授权文件的 reader 读取 UID997 的远程状态，被原规则拒绝；改为验证其真实 UID/GID997、0600、单链接及读前后身份的独立只读观察，保留拒绝回执，产品 Root 检查未改变。

文档前18仓同分支且干净，仅 Core 有本轮预期代码提交；冻结完整12父项 SHA `4e1aaca54eb94c3fc36266506c87a0781498aa797f0c18a01036f74a21584cc7` 不变。

该候选未打 deb 或部署；`kura.service` 未修改，实际 Main 仍11 ambient/13 bounding caps、NNP0。完整 typed privileged broker、网络/时钟/模块/日志/文件移动/系统更新及其余 provider、不同 command user/launcher、六组件/安装升级移除恢复/资源平台/R017/GUI和全部12父项继续内部 pending。没有用局部前置替代完整最小权限。0/12父项最终闭环不等于代码实现0%，缺少加权任务拆分时不编造整体百分比或完成日期。仅原3 external deferred，G034未封存，协议未正式发布。

目标 active、现有积分继续；不推送、不加27、不清活动 runtime，不重发/换号/跨Root查询旧UNKNOWN，不重放闭环密钥及900秒验收。下一步继续实际 provider 的窄权限委派与 launcher/Root代码配对后，再收敛 Main 权限。
