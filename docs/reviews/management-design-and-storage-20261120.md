# YOFC Kura 管理改造：设计、工期与磁盘复盘

2026-11-20 初稿，2026-11-22 更新。复核对象为会话《YOFC Kura 管理体系改造》、父工作区暂停交接、12 项冻结台账、实际八包安装与 Wires 重装检查点、两次清理回执和当前 Git 引用。

## 11 月 22 日保留重装及 purge 的实际成本

从保留重装 preinst 到部分删除恢复，五个 Git 修订分别生成了完整 ARM64 Core deb。五份文件的逻辑长度合计 1,025,533,520 字节，逐份摘要已与发行记录核对，密钥读者清单相同。这一轮约 1.03 GB 的宿主发行件来自每个包钩子修订都重新打整包。两处 purge 实测使用独立的业务历史，结束时卷内分别为 69,936 KiB 和 69,940 KiB。这些读数不能换算为 APFS 或虚拟磁盘的实际增量。[逐包与卷内读数](../../../.management-close-20261116/core-rtc-hotplug-20261120/core-purge-partial-tree/storage.json)

本轮删除恢复只需保存原目录身份。Core 在递归删除前给原目录改名，以原移除编号和 inode 校验私有名称，续办仍使用原维护围栏。4096 个测试文件没有产生逐文件进度记录；实际中断后从剩余的 4093 个文件继续清理。[ARM64 中断与恢复](../../../.management-close-20261116/core-rtc-hotplug-20261120/core-purge-partial-tree/HANDOFF.md)

## 已发布密钥代移除时的保留成本

11 月 22 日的 ARM64 原生移除复用了已停放的同一测试卷。三份启动脚本和绑定移除编号的回执合计 9,533 字节。包含新维护脚本的整包为 205,092,922 字节，宿主保留一份发行件；传入来宾的临时副本在校验后删除。这些数值分别是文件长度，此次没有测量 APFS 物理块增量。[移除与冷读实证](../../../.management-close-20261116/core-rtc-hotplug-20261120/retained-generation-launchers/HANDOFF.md)

## 11 月 22 日工作区复测与容器发行包

北京时间 02:55，按同一文件系统的 `du -kx` 计量，父工作区合计 **364.70 GB**。其中历史工作目录 269.74 GB，整个中心仓 81.46 GB，两处合计 96.3%。这些是目录块使用量，APFS 共享块与虚拟机磁盘另计。下面 11 月 21 日的 95.5% 使用了中心仓内 target 的较窄口径。[本次分目录读数](../../../.management-close-20261116/core-rtc-hotplug-20261120/container-engine-maintenance/workspace-kib.txt)

本轮的两次发行修订各有现场原因。正式 Main 先遇到类缺失，补齐依赖后又发现旧适配器错误连接 TCP。直接在已有 ARM64 内存系统对比两个适配器并复用同一快照，最终从专属 Unix socket 读到原容器。三份新 deb 合计 48.88 MB，引擎输入 37.52 MB，停机可写层 4.58 MB，新增宿主持久卷为零。发行件还存在可量化的重复打包：16 个相同 JAR 同时位于编排 bundle 根目录与 lib/，第二份在 ZIP 内占 8.19 MB，占整个 bundle 的近一半。这个数字针对该 bundle，不能外推到整个工作区。[发行包明细](../../../.management-close-20261116/core-rtc-hotplug-20261120/container-engine-maintenance/embedded-size.json) · [本轮现场](../../../.management-close-20261116/core-rtc-hotplug-20261120/container-engine-maintenance/HANDOFF.md)

## 目前的判断

**有过渡设计，也有可以直接压缩的验证成本。** 旧 Java/Vue 中心与早期 Netplan 接管留下了代码和夹具。现行路径是 Rust/React 中心、可选常驻 agent、可独立运行的 Kura 与 NetworkManager。涉及系统权限的写操作由本机 broker 核对调用者、设备和原安装，再保存操作编号与回执。对已经派发的操作，这些记录决定能否安全续办。

截至 11 月 21 日 13:40，宿主工作区目录占 **356.56 GB**。历史工作目录占 **261.63 GB**，中心仓的 target 占 **78.76 GB**，两处合计 **95.5%**。单个 Core 打包目录有 48 个 ARM64 deb，文件长度合计 9.81 GB，本次两代完整包共 410.03 MB。宿主目录使用 du 计量，Parallels 虚机另计。[目录与包实测](../../../.management-close-20261116/core-rtc-hotplug-20261120/main-firewall-xt-state/storage.json)

共享 ARM64 Cargo 的 debug 目录现占 5.50 GB，相对较早的同目录读数 0.60 GB 增加 4.89 GB。12:17 到 13:40 的工作区目录读数增加 5.90 GB，同期 Core 完整包从 42 份增到 48 份，新增文件长度 1.23 GB。Cargo 的早次读数取自另一检查点，不能据此把两个增量相加。[Cargo 原读数](../../../.management-close-20261116/core-rtc-hotplug-20261120/startup-execution-release/storage.json) · [本次分类实测](../../../.management-close-20261116/core-rtc-hotplug-20261120/main-firewall-xt-state/storage.json)

**可以压缩的是重复组装、独立缓存和逐场景重建安装的成本。** 目前三个主要部署位置各有明确职责，中心管多网关，网关 Core 和本地控制台可独立运行，可选 agent 负责远程通道。已派发写操作必须能在原安装查回结果，这项约束影响停机、升级和恢复的验收安排。此次沿用原六组件卷升级 Core，并把旧、新 worker 的 IPv4、IPv6 交接放在一次隔离内核测试中。临时安装的根设备号改变时在升级前被拒绝；这类必须跨停机核对的场景，需要能保持根目录身份的安装介质。[六组件、原回执与升级](../checkpoints/20261116/actual-installed-firewall-action-receipt-checkpoint.json)

截至 11 月 21 日 15:27，同一打包目录已累计 54 个 ARM64 Core deb，文件长度合计 11.04 GB。最近一次启动恢复改动复用了现有卷与编译缓存，产出一个 Core 和两个 Networking 变体，合计 214.93 MB，其中 Core 一包占 205.05 MB。即使不复制虚拟机，只要每次修改都保留整包，历史目录仍会增长。[本次包长度与数量](../../../.management-close-20261116/core-rtc-hotplug-20261120/firewall-confirmed-boot-service/storage.json)

后续一次只涉及 Networking 启动脚本的修订，沿用上轮 Core、原测试卷及两份编译缓存。标准包和 firewall-only 包的文件长度合计 9.88 MB，较上轮三包的 214.93 MB 少 205.05 MB。原安装上的卸载、purge、解包后停机及重启续配使用同一份标准包。[包长度与测试范围](../../../.management-close-20261116/core-rtc-hotplug-20261120/firewall-bind-shutdown/packages.json)

这次六组件联合升级直接复用了 Core 和 Networking 两份已有 deb，输入文件合计 214,464,472 字节。三个只读观察 JAR 共 12,898 字节，新建发行包、卷及虚机快照均为零。每次变更都生成整包会扩大磁盘占用，读取旧数据库和验证安装器自动保留本地组件则可以直接使用已有包。本次仅修改包维护提示的 Core 提交运行到 Maven `prepare-package`。这些数字统计产物长度，未计日志、数据库写入和编译缓存的实际块变化。[联合升级检查点](../checkpoints/20261116/actual-native-wires-retained-reinstall-checkpoint.json)

防火墙 worker 修订及随后仅改升级提示的两份实装 Core deb 共 410,102,854 字节。逐项读取 deb 成员，两份压缩载荷 `data.tar.gz` 的长度均为 204,847,072 字节，SHA-256 完全相同。第二次构建的差异在控制脚本和包元数据。归档成员的数据可用于判断哪些历史包可共享存储；此处计的是文件长度，APFS 独占块未测。[两份包的成员摘要](../../../.management-close-20261116/core-rtc-hotplug-20261120/firewall-boot-receipt-admission/storage.json)

11 月 21 日的 Felix 验收复用了 Deployment 发行件与已有 Core 插件。三份独立业务包及其 Equinox 存储共占 324 KiB（宿主 du），没有新增 deb、Docker 数据卷或虚机。三次进程中断后，业务 JAR 字节全部保留，其中两次 Felix 也能列出包版本，但 Equinox 冷启动均未登记业务 bundle。排查时可以先对照既有包清单与运行时清单。只有涉及正式 Main 和安装维护的结论才需要回到对应安装上验证。[逐项结果和适用范围](../checkpoints/20261116/actual-native-wires-retained-reinstall-checkpoint.json)

正式 Main 的 IPv4 试验显示，配置快照可以在内核拒绝规则时保存成功。临时安装的站点规则缺少保存模型声明的两条入站规则。核对两份清单就能定位这一差异，单纯重装或重启会使结论更难判断。为改善定位，这次仅重建了 50,482 字节的源 crate 和 ARM64 worker，复用原安装、构建缓存及已有 deb。测试快照保留在原目录，恢复了原三份启动候选。[IPv4 现场与磁盘范围](../checkpoints/20261116/actual-installed-firewall-action-receipt-checkpoint.json)

IPv4 回环端口转发说明了保留原现场的另一项成本。正式 Main 已发布规则，原生内核读回正确；重启后旧版恢复也取得了内核回执，但站点启动器依赖的 iptables-save 无法列出 NAT 表。为保存这笔恢复的原命名空间，临时安装保持运行，Main 已停。验证修订只使用已有编译缓存和短时独立网络命名空间。最终 ARM64 Core deb 为 205,064,424 字节，其中替换的是防火墙 worker。这次包含实际运行时行为变更，保留了完整发行件。[端口转发、启动中断与两族实测](../checkpoints/20261116/actual-installed-firewall-action-receipt-checkpoint.json)

## 工期花在哪里

最初的多网关管理中心已经扩展到网关操作系统、原 Kura 本地控制台、密钥保管、发行包维护及故障后的业务连续性。同一张 Timer→Logger 图能在首次安装时送达，只证明运行时可用。要证明客户升级可用，还需在原安装上升级 Core，确认原图只提交过一次；要证明 Wires 可独立拆装，又需经过 config-files、unpacked、installed，查看注册行并观察原图再次送达。后三项已有现场记录。其余组件及中断点不能借用这个结果。

历史方案确有过渡。旧中心使用 Java/Spring Boot、Vue 和 MQTT，新中心采用 Rust 和 React。早期 Netplan 接管原型在 11 月 5 日调整为保留 NetworkManager。两次变化都留下代码、包、旧状态和夹具。另有一类长期设计成本来自同一安装身份上的安全恢复。写操作派发后丢回执可能已经生效，产品需要识别原编号、原 Root 和维护围栏；旧版本与新版本都需要理解可恢复的中间状态。

交接本身也逐轮变长。11 月 21 日 11 时许，父库的 Tasks.md 与 Session Summaries.md 经官方 CLI 全文读出分别为 881,441 和 886,889 字节，Git 未结台账约 482 KB。全文读取会增加每次续接的工时，这三份文档合计约 2.25 MB。从磁盘数字看，更大的开销集中在历史工作树、包与构建目录。

再看 Git，从北京时间 11 月 16 日零时到本次交接 HEAD，沿提交历史排除 merge 后，中心仓有 267 次提交，242 次只修改 `docs/`，占 90.6%。同期 Core 有 185 次提交，其中 178 次涉及 `docs/` 外的路径。中心仓承担了跨仓验收台账，Core 则在持续改实现。这能解释为什么看中心提交数量会高估功能迭代次数，也能看出逐场景单独交接的维护成本。统计按提交计数，不能换算工时。

**工程判断**：按每个小改动建立完整发行目录和私有依赖副本，已经产生可量化的重复成本。独立低权限进程、限制写权限和保留未决操作各有实际业务理由。现有数据足以支持合并构建工作目录和缩小回归输入，尚不足以证明可以合并这些授权边界。工期没有逐项工时记录，不能可靠地计算返工比例。

单次完整测试也有可量出的等待成本。11 月 21 日的 JDK 21 Core/OSGi 构建从头跑完 82 个模块，用时 11 分 42 秒。仅证书吊销列表的 13 项集成测试就用了 424 秒，约占总时长的 60%；这些用例要观察下载、刷新和到期的时序。2128 项测试通过、2 项跳过。调整验证频率时，可以把这类整套时序验收与局部维护脚本的定向验证分别估时。[完整构建记录](../checkpoints/20261116/actual-rtc-hotplug-and-addon-upgrade.json)

这轮 Linux 全量构建还解释了重复验证中的一部分。一次从恢复模块开始的续跑，68 项 Equinox 测试全过；随后完整构建在异步注册组件时暴露了配置服务枚举 PID 的并发错误。先用一个确定性测试在旧实现上复现，再用原六组件卷中的正式 Main 并发枚举和注册。最终这次完整构建耗时 11:40 min，2151 项通过、2 项跳过。一个临时容器和同一份缓存支撑了各次构建。回归覆盖和重复存储可以分别安排。[完整构建及原卷验证](../checkpoints/20261116/actual-native-wires-retained-reinstall-checkpoint.json)

## 当前部署与权限

| 位置 | 当前职责 | 已验证的边界 |
| --- | --- | --- |
| 浏览器和中心 | React 经 HTTPS API/SSE 连接中心。Rust 中心保存任务、审计和结果；agent 主动建立双向 gRPC/mTLS 通道。方案 A 使用外部 CA。 | 中心、安装器和六个网关包已在一套 ARM64 夹具联合首装。多人 CA 保管、客户旧库及完整部署 GUI 仍待验。 |
| 网关本地 | Kura/OSGi 支持独立运行，保留 NetworkManager。可选 agent 常驻，远程开关控制中心通道。原 GWT 在安装 agent 后显示远程菜单。Core 生成私钥和 CSR。 | 原运行现场与新候选分开核验。当前六组件为 Core、Networking、GWT、Deployment、Wires、agent；加中心和安装器共八个 deb。 |
| 需要系统权限的操作 | Main 调用按操作类型限定的本机服务。peer 校验调用身份，actor 执行获准的系统动作，安装器校验成套代码和单元。 | 原六组件卷上，Core 71eb1087ae 的正式 Main 以 UID 997 运行，NoNewPrivileges 为 1，有效能力只保留 NET_BIND_SERVICE（0x0400）。NET_ADMIN、DAC_READ_SEARCH、DAC_OVERRIDE 均已撤去。旧安装仍按各自版本核对。 |
| RTC 热插拔 | 草稿把启动时的设备投影改成请求时重验节点及别名，再向时钟 actor 传递只读 FD。 | 复盘时为草稿。报告送达后，已完成 ARM64 热插拔、只读 FD 和新包的两条原 Root 升级路径。随后在原 Parallels 组件完成正式 actor 经 relay 接收 FD 并写入虚拟 DS1307，实测回执 APPLIED。正式 Main 已取得设备缺失的 ENOENT 回执，并在 Core purge、重装后读回原记录。在原组件上，独占打开的 DS1307 使实际 relay 返回 EBUSY，Root 与发送端保存同一笔失败。当前 Core 安装器只读核对后拒绝此静态组件单元。随后在同一 Parallels 虚机的独立新安装，正式 Main 通过已注册的 ClockService 向专属 DS1307 成功写入一次，设备读数及两端回执均已核实。物理 RTC 仍待验。 |

从当前六组件停机安装逐文件核对了八组 broker 的发布清单，覆盖 26 个 systemd 单元。以 RTC 为例，读取设备的服务使用 root 身份，能力集为空，设备策略为 closed；时钟 actor 使用 kura-clock 身份，仅获 CAP_SYS_TIME，并启用私有设备目录。当时的 Main 以 kurad 运行，实测保留网络管理、绑定低端口和 DAC_OVERRIDE 三项能力。分开这两个时钟服务，使接触设备路径和调整时钟成为两个可以分别核验的权限。后续 b1b8060be2 已在同一安装撤去 DAC_OVERRIDE。[实装声明](../../../.management-close-20261116/core-rtc-hotplug-20261120/linux-arm64-core-osgi/typed-unit-declarations.json) · [正式 Main 进程实测](../../../.management-close-20261116/core-rtc-hotplug-20261120/linux-arm64-core-osgi/observed.json)

普通 Main 的调用线程从 0x1400 降至 0x0400 后，已通过本机服务读取 IPv4、IPv6 内核表，并在 Core b1c664aaeb 上用正式发布 worker 写入两条回环转发规则。iptables-save、逐条匹配与保存文件均已核对。主线程仍持有 NET_ADMIN 与 NET_BIND_SERVICE。两包共同 purge、重装和冷启动中断后，原编号仍可查询。Core bba8b1d1c8 使包维护期间的开机任务在派生进程前失败，原维护程序可以直接续配。[实装状态与恢复记录](../checkpoints/20261116/actual-installed-firewall-action-receipt-checkpoint.json)

进一步实测表明，Main 中单个线程撤去绕过文件权限的能力后，仍能读取原业务包，并让日志 broker 导出 23,647 字节。冷启动还有具体的文件权限依赖：PID 目标在 root 所有的 /run，已有三个日志可由 kurad 续写，但该账号不能在 /var/log 创建首份日志或新的归档。现已修复两个后台启动入口在 PID 文件打不开时仍派生 JVM 的问题。整项服务撤权还需要覆盖首次创建、滚动归档及 JVM 错误转储。权限实测复用了已装包；随后为两个启动器打包一次，耗时 8.478 秒，新 deb 为 204.24 MB。此时 Core 目录内共 32 个 ARM64 deb，合计 6.54 GB。[权限与启动器实装记录](../checkpoints/20261116/actual-native-wires-retained-reinstall-checkpoint.json)

整台前台 JVM 的日志实测说明，单看文件可写会漏掉容量故障。两个 ARM64 进程均以 kurad 和有效能力 0x1400 运行，并恢复原业务 DP。使用 Debian 默认日志布局时，应用和审计日志写入成功，强制轮换后都没有压缩归档，仍向原 inode 追加。换成 Docker 已用的 /var/log/kura 私有目录，两个 Log4j appender 各生成一份含旧标记的压缩档，旧日志路径仍能读到新文件。试验复用已装包，只替换两个前台进程看到的日志挂载。原三份宿主日志的完整指纹保持。第三次启动只将归档目录指向 /var/log/kura/archives。两个活动文件均保持原路径和 inode，各自产生一份含旧内容的压缩档，并继续接收新行。已有普通活动日志可原地保留。正式发行仍需覆盖首次创建、历史归档下载及保留。[同一安装上的两种日志布局](../checkpoints/20261116/actual-native-wires-retained-reinstall-checkpoint.json)

Core 910cb9bd9a 已将活动日志首次创建交给 Debian 安装器。用发行包的配置阶段在 ARM64 实测，三个文件均以 kurad、0600 创建；同一配置步骤保留已有站点文件、别名及目标暂时离线的别名。原六组件卷真实升级后，三份旧日志的内容、inode、权限和时间戳一致。后台启动器先打开控制台，避免日志失败时误写 PID。归档布局、GC 和错误转储仍待正式发行验证。此轮只组装一个现有 bundle 集合的 Core 包，耗时 8.685 秒。[首建与升级实装](../checkpoints/20261116/actual-native-wires-retained-reinstall-checkpoint.json)

Core 5aa5aad4f1 将专属归档目录接入 Linux 发行包，沿用原活动文件和 inode；因此升级无需搬迁旧日志。JVM 转储和错误报告使用安装目录已有的私有 log 子目录，调试入口的 GC 日志也写入其中。对三个启动器分别用隔离的 12 MiB 测试堆触发诊断，临时文件随测试容器回收。GWT 6fd074c 的服务端 ZIP 同时收取旧路径的日志和新目录中的应用、审计归档。原六组件卷装入两包后，对三条 ZIP 记录逐项核对了 SHA-256。正式管理页的已登录 HTTP 下载已在同一安装验证。[升级、诊断与 ZIP 核对](../checkpoints/20261116/actual-native-wires-retained-reinstall-checkpoint.json)

同一六组件卷随后通过正式管理页完成首次改密并重新登录，从系统日志标签下载了 3,666,808 字节的 ZIP。浏览器收到 HTTP 200，99 个条目通过 CRC；两份新目录归档和一份旧路径归档的 SHA-256 与输入相符。原卷实际开放的是 HTTP 80，验证期间连接仅经过宿主和夹具回环。八份原快照完整，新快照保留原账号。此次复用了已安装的 Core 和 GWT，两份 systemd journal 也随 ZIP 交付。[登录下载与冷读](../checkpoints/20261116/actual-native-wires-retained-reinstall-checkpoint.json)

原六组件安装升级到 Core b1b8060be2 后，正式 Main 已撤去 DAC_OVERRIDE。systemd 在 fork 前为 kurad 准备历史 PID 路径，进程运行时只保留管理网络与绑定低端口两项能力。两次正式启动的五组能力掩码均为 0x1400，两次停机后 PID 文件都由 systemd 移除。同卷的启动中断实测中，生成配置途中停机留下 424 字节临时文件。再次启动后，正式 JVM 取得写入锁，完整配置为 22,494 字节。这次复用了现有安装和发行包。旧 PID 的实测又发现，root 遗留的记录会使首次启动返回失败。Core 8b275f0ece 在同一卷核对已退出进程后接管原 inode，首调成功，正式 JVM 维持原能力集。[原安装的两次启动及停机](../checkpoints/20261116/actual-native-wires-retained-reinstall-checkpoint.json)

普通命令的独立服务还影响两个用户可见的行为。在原六组件卷上停止一个正在运行的命令实例，Main 和监听 socket 均继续运行，下一条命令成功。对于 Main 私有数据目录，worker 可以相对读取已知文件，调用 getcwd() 则返回 EACCES。站点另设的目录给命令组穿越权后，worker 可以取得绝对路径并读指定文件，仍无法列举目录。两次实测复用已安装的 Core，没有新增 deb 或构建镜像。[普通命令与目录验证](../checkpoints/20261116/actual-core-command-worker-lifecycle-checkpoint.json)

### 六个网关包已经验证到哪一步

截至 11 月 21 日，首装和组件能启动已有联合现场。下面按实际业务区分维护证据，便于续接时直接选取尚缺的模式或中断点。每行只覆盖所列现场，冻结父项仍为 0/12。

| 网关包 | 已取得的现场证据 | 仍需区分的条件 |
| --- | --- | --- |
| Core | 正式 Main 取得缺失 RTC 的 ENOENT，Core 移除、purge、重装后原回执和密钥仍在。[时钟实装记录](../checkpoints/20261116/actual-registered-main-clock-and-purge-checkpoint.json)。三类网络发送端在派发前保存的原编号已通过正式 Core purge、重装及 Java 读回。同一发行系列的真实首装在 PREPARED 中断后，原 17 份回执经拒绝替换与正常续配保持一致。[网络发送端记录](../checkpoints/20261116/actual-core-network-sender-purge-checkpoint.json) | 虚拟 DS1307 正向写入已在独立首装的正式 Main 实测。现行正式单元已验证两次撤权后的启动和停机，配置生成途中停机已在原地恢复，已验证常规 root 所有的过期 PID 文件可在首次启动时接管。 |
| Networking | 启用 DNS 的六组件现场经过 Core 升级及 Networking 移除、purge、重装。正式 Main 恢复原模型，IPv6 客户端收到合成 IPv4 上游的回答。[启用 DNS 实装记录](../checkpoints/20261116/actual-enabled-dns-snapshot-upgrade-checkpoint.json)。DHCP 停用丢回执后，经 Networking purge、配置中断及重装，正式 Main 按原编号取回结果。[DHCP 实装记录](../checkpoints/20261116/actual-registered-main-dhcp-maintenance-checkpoint.json)。在 Core 与 Networking 同时 purge、重装的现场，重新提供同一停用测试配置后，正式 Main 也只确认原停用编号。BIND 的停用回执也已跨两个包的共同 purge；出厂快照启动后只新增原回执。[共同 purge 实装记录](../checkpoints/20261116/actual-core-network-sender-purge-checkpoint.json) | 本次网络设备由夹具准备。真实 NM/cloud-init 和物理硬件交接仍归原延期项。 |
| GWT | 单独移除后观察到 unpacked、half-configured，原地续配成功，随后 purge；核对了自定义图标和 Wires 文件。[中断回滚记录](../checkpoints/20261116/actual-passive-addon-dpkg-rollback-checkpoint.json)。六组件联合移除中实际得到 GWT half-installed、Wires config-files，续办移除、共同 purge 和原版重装后，正式 Main 加载两者并恢复 HTTPS。 | 正式 Main 已验证 agent 安装、离线、移除和 purge 时的远程标签及手动状态提示；Cloud purge 后保留 true 时，页面显示无可用连接。已登记 agent 的远程业务及其他页面流程仍待分别验收。[浏览器与包回执](../checkpoints/20261116/actual-native-cohort-gwt-ready-and-purge-checkpoint.json) |
| Deployment | 单次安装的业务 DP 在 Deployment 移除和 purge 后保留了归档与注册表。重装在 half-configured 状态正常关机，原 Root 续配后，正式 Main 自动恢复 ACTIVE bundle 并读出资源。[业务包维护](../checkpoints/20261116/actual-native-wires-retained-reinstall-checkpoint.json) | 仅装余下五包时，部署服务缺席，Felix 未列出该 DP。FileMove 发布丢回执另有原地恢复实证。[FileMove 恢复](../checkpoints/20261116/actual-deployment-filemove-manager-reboot-checkpoint.json) |
| Wires | 保留式移除和分阶段重装后，正式 Main 读回原 Timer→Logger 图，Logger 再次收到消息，原图仅提交一次。[原图重装记录](../checkpoints/20261116/actual-native-wires-retained-reinstall-checkpoint.json) | 联合配置中断有另一套现场及原 Root。[六组件转换记录](../checkpoints/20261116/actual-standalone-position-six-conversion-checkpoint.json) |
| Agent | 同 Root 实际经过移除、purge、解包及重装，另有已绑定和未绑定场景。[保留式维护记录](../checkpoints/20261116/actual-native-agent-retained-removal-checkpoint.json) | 续接前要确认原注册身份、业务记录和 launcher 模式。 |

标准 Networking 的原包即使未被替换，也曾因后继包拒绝安装而降为 unpacked。原六组件实测定位到旧包自身的回滚脚本。修订后，升级、依赖停配和冲突移除三种撤销都保住了原包和组件加载顺序。真正移除才注销组件。本次复用了原 Core 包、原卷和 Java 构建结果。[原包状态与 dpkg 回调](../checkpoints/20261116/actual-installed-firewall-action-receipt-checkpoint.json)

标准版与 firewall-only 共用的安装状态让换型能够复用原卷。11 月 21 日在原六组件安装上验证，firewall-only 解包后停机，重新配置后两族 OSGi 防火墙服务可读；移除运行中的 provider、purge 并装回标准版后，9 份原始快照的内容和元数据均保持。此次复用已安装的 Core，两种 Networking deb 合计 9.82 MB。[换型及冷卷记录](../checkpoints/20261116/actual-installed-firewall-action-receipt-checkpoint.json)

防火墙确认流程中有两个编号。适配器用动作派发编号定位丢失回包的那次调用，GWT 会话用原准备编号读取整笔事务。新版在派发前收回按钮对应的服务端权限，连续读到旧的等待状态也不会再次派发。原六组件卷中，正式 Main 装载两个升级包，模拟确认、回滚各丢失一次回包，分别在第三次只读查询取得终态。两个新 deb 合计 43.52 MB，复用原 Core 和测试卷。[实装验证与回执](../checkpoints/20261116/actual-installed-firewall-action-receipt-checkpoint.json)

Deployment 的本地和云端入口各自维护同一份业务 DP 注册表。旧云端代码一次写入失败，就把 5963 字节的表截成 1024 字节，并向调用方返回成功。修复后两个入口编译同一份文件更新代码，各自随原 bundle 发行。相同限制下新版报告失败，原表完整。同一现场的两个 JVM 并发登记 60 项后，原有的 180 条无关记录仍可读。这轮复用原六包卷，仅新建约 0.22 MB 的 Deployment deb。随后发现注册行完整时，旧版仍可能在替换同名归档时先删旧文件。新版先完成同目录暂存与同步，再原子替换。两个实装入口在跨文件系统复制失败时均保住原文件，正式 Main 再次从原记录加载业务包。本次 Deployment deb 为 218,572 字节，复用了原卷和 Maven 缓存。Felix 安装、归档替换和注册表更新仍分步执行。[版本对照和实际 Main](../checkpoints/20261116/actual-native-wires-retained-reinstall-checkpoint.json)

H2 文件库提供了一个只有跨升级才看得清的风险。旧版遇到首次打开失败会删除数据库重试，凭据不匹配也可能丢失业务行。六组件的旧 Main 建库、写入一次业务记录，原地升级后的 Main 在拒绝错误凭据后恢复原凭据，再次读回同一行。整个场景复用 Wires 中断恢复的原容器和卷。[H2 实装与停机读回](../checkpoints/20261116/actual-native-wires-retained-reinstall-checkpoint.json)

这次验证启用中的 DNS 复用了原六组件容器、已有 Core 工作树和 Maven 缓存。修复后的 Core deb 为 204,224,080 字节，逐项核对了运行时和恢复工具中的两份 XML provider JAR。收尾从测试容器移除了已核对宿主原件的两个临时 deb，共 213,605,954 字节。[本轮安装与空间记录](../checkpoints/20261116/actual-enabled-dns-snapshot-upgrade-checkpoint.json)

SQLCipher 的现场还验证了两个包管理层如何共同恢复。在同一六组件安装上登记 SQLite 部署包后，共同清除了 Debian 的 Deployment 包和原生 SQLCipher 库。只解包 Deployment 时关机，其维护围栏阻止 Main 提前启动。原地恢复两包后，正式 Main 从原注册行加载部署包，读回原明文和加密业务行，两份数据库内容摘要均保持。此次复用了原部署包和两个 deb，驱动和 provider 由部署包交付。[联合维护记录](../checkpoints/20261116/actual-native-wires-retained-reinstall-checkpoint.json)

正式 Main 安装过两个版本的加密 SQLite 部署包。升级后注册表指向新版，卸载按当前登记地址删除了新版。冷卷上还留有旧版的 294,611 字节归档，SHA-256 与首次安装的发行件一致。DeploymentAgent 启动时遍历注册表，因此该旧文件不会仅凭位于 packages 目录就被重新安装。这解释了运行卷上的一处小额留存，完整构建目录和多版 deb 的占用要大得多。[旧归档和注册行](../../../.management-close-20261116/core-rtc-hotplug-20261120/sqlite-provider-distribution/archive-identities.json) · [启动和卸载代码](../../../.management-close-20261116/core-rtc-hotplug-20261120/kura-deployment/bundles/org.eclipse.kura.deployment.agent/src/main/java/org/eclipse/kura/deployment/agent/impl/DeploymentAgent.java)

## 六个网关包的维护实证

下表按最近复核过的 ARM64 检查点列出能复用的结果。每行的安装版本、Root 和维护编号见该行记录。六包跨不同版本的记录不能拼成一次同版本联合验收。

| 网关包 | 已实测的维护与恢复 | 业务或运行时核对 |
| --- | --- | --- |
| Core | 与 Networking 联合升级时，两包解包后停机，重启沿原编号配置。独立安装另验 remove、purge 和移除过程中断续办。[联合升级](../checkpoints/20261116/actual-native-wires-retained-reinstall-checkpoint.json) · [移除](../checkpoints/20261116/actual-passive-addon-dpkg-rollback-checkpoint.json) | 正式 Main 分别读回明文 SQLite、SQLCipher 与 H2 原业务行。 |
| Networking | 已在六包安装上升级、remove、purge 和重装；防火墙专用 Root 另验 purge 后重装、解包停机及重启配置。[BIND](../checkpoints/20261116/actual-bind-main-recovery-and-six-component-networking-checkpoint.json) · [防火墙](../checkpoints/20261116/actual-installed-firewall-action-receipt-checkpoint.json) | 正式服务保留原 DNS 模型，防火墙核验原 IPv6 启动回执。正式 Main 授权的 IPv6 转发已在无站点文件时跨正常停机恢复。Main 按原配置撤销规则后，下一次启动双族转发均为 0。另经 ConfigurationService 保存一条 IPv6 业务规则，完整 Main 重启后读回原快照和两条内核规则，显式撤销产生一份新快照。 |
| Management UI（GWT） | 配置遇注册锁中断后原地续配。与 Wires 联合移除时，GWT 在 postrm 中断，随后续办、共同 purge 并重装。[dpkg 阶段](../checkpoints/20261116/actual-passive-addon-dpkg-rollback-checkpoint.json) | 已检查保留的图标、正式 HTTPS 入口，以及安装和移除 agent 时菜单的变化。[页面实证](../checkpoints/20261116/actual-native-cohort-gwt-ready-and-purge-checkpoint.json) |
| Wires | 原安装上升级进入 half-configured，正常停机后续配。另验与 GWT 共同移除、purge 和重装。[原图与升级](../checkpoints/20261116/actual-native-wires-retained-reinstall-checkpoint.json) · [联合移除](../checkpoints/20261116/actual-passive-addon-dpkg-rollback-checkpoint.json) | 正式 Main 读回原 Timer→Logger 图并再次送达，原图只提交过一次。 |
| Deployment | 业务 DP 随包移除、purge、同包解包和恢复。postinst 在注册锁处中断，跨停机仅续配原包。[DP 生命周期](../checkpoints/20261116/actual-native-wires-retained-reinstall-checkpoint.json) | 原 DP 自动恢复并返回业务资源。注册的 SQLCipher DP 和原生库联合拆装另有实证。 |
| Management Agent | 已初始化与已绑定分别验收。已初始化样本有原编号 postrm 续办；已绑定样本以同一业务状态完成 remove、后继版本 unpack、configure 与显式启动。[postrm](../checkpoints/20261116/actual-native-agent-postrm-only-recovery-checkpoint.json) · [已绑定样本](../checkpoints/20261116/actual-native-agent-bound-successor-checkpoint.json) | 保留原任务及远程开关决定。已绑定现场的后继换版不能代替其他绑定代和来源历史。 |

多目录恢复又核对了一个中断点。在隔离 ARM64 进程中，正式配置器发布第一个 sibling 目录后收到 SIGKILL；新进程沿原围栏恢复第二个目录，首个目录中的 JAR inode 保持，未发布暂存已撤下。该次使用文件系统夹具，同组 14 项通过。完整 dpkg 现场已有单个 JAR 写至一半的中断记录。后续选择覆盖点时，可据此区分文件复制、目录发布和业务恢复三个阶段。[多目录回归](../checkpoints/20261116/actual-native-wires-retained-reinstall-checkpoint.json) · [单包实装中断](../checkpoints/20261116/actual-core-prepared-torn-copy-checkpoint.json)

## 磁盘增长的实测解释

防火墙两代包的恢复实测共用一个持久 Root。新建的具名卷为 61,256 KiB，最终冷读为 293,460 KiB。设备号在宿主 overlay 中改变时，持久 Root 和两份原凭据仍保持身份连续。修正 unit 的 Core deb 长 205,016,000 字节，当前组装目录有 49 份 ARM64 deb，逻辑长度共 10,015,844,740 字节。此轮复用 Maven 缓存、ARM64 worker 和 Networking 包。[双包 purge、中断与分项计量](../checkpoints/20261116/actual-installed-firewall-action-receipt-checkpoint.json)

Core 自身的升级也会产生临时复制。安装脚本在替换旧目录前把用户配置、数据、部署包、日志和 sibling 目录暂存，再恢复仍由原包持有的内容。这次六组件停机卷中，这些目录的常规文件合计 164.05 MB，其中 sibling 占 163.54 MB（99.7%）。升级前后抽查的 40 份文件内容均一致，36 份运行文件换了 inode。这里量到的是复制范围的逻辑大小；文件系统是否共享物理块、升级瞬间的可用空间峰值尚未计量。该现场说明，小型业务库也要承担同一次 Core 升级中 sibling 发行文件的暂存成本。[升级、文件与目录计量](../checkpoints/20261116/actual-native-wires-retained-reinstall-checkpoint.json)

相邻两次 Core 发行更直接地说明了磁盘为什么随迭代增长。修正包维护对正式 Main 的识别后，两个 ARM64 deb 各约 204.24 MB。逐个解包计算内容摘要，540 个常规文件中有 538 个完全相同，占新版常规文件总字节的 99.93%。变化集中在维护脚本和记录其摘要的清单。以最终 deb 为基准试做差分，88.03 MB 即可逐字节恢复 204.24 MB 的中间 deb，SHA-256 与原包一致。这是一次临时测量，两个发行原件仍保留，未计为已回收空间。[包内逐文件比较](../../../.management-close-20261116/core-rtc-hotplug-20261120/package-writer-quiescence/payload-difference.json) · [差分还原实测](../../../.management-close-20261116/core-rtc-hotplug-20261120/package-writer-quiescence/delta-estimate.json)

网关运行时也会积累少量旧发行件。本次 SQLite 可选 provider 直接用现有部署入口交付，两个版本各约 0.295 MB，沿用同一六组件卷。升级及卸载后，旧版本的 294,611 字节仍在 packages/，摘要与首次安装包完全一致。部署服务启动时按 dpa.properties 恢复，卸载时只删除当期登记的归档；停机时注册表只剩原业务 DP。这个实例解释了单个旧文件的来源，不能据此推算工作区数百 GB 的总量，也不足以决定其他历史包的删除策略。[现场归档与摘要](../../../.management-close-20261116/core-rtc-hotplug-20261120/sqlite-provider-distribution/archive-identities.json)

以下采用十进制 GB。11 月 16 日清理时工作区从 483.49 GB 降至 96.39 GB。11 月 20 日上次会话中途量到 491.68 GB，清理完成时为 335.18 GB，宿主可用空间净增 154.45 GB。两次读数之间有新构建，APFS 共享块也会影响目录计量，不能把两次节省量相加。

| 11 月 20 日清理后的目录 | 占用 | 对工期和保全的意义 |
| --- | ---: | --- |
| 工作区 `.management-close-20261116` | 241.07 GB | 各阶段的源码副本、私有依赖、发行包、故障现场及回执混放。其中单个 source-cohort 目录约 27.07 GB。 |
| 中心仓 `target` | 78.76 GB | 包含跨阶段的现场引用、保全基线与构建结果。整目录删除会破坏续接。 |
| 两者合计 | 319.83 GB | 约占工作区 95.4%。这是开发与验收的目录占用，不能用来推算客户网关的安装容量。 |

清理实际辨认出 266 份重复组装输出、134,424 个字节相同的 Maven 文件、20,787 个编译输出目录，以及 271 个同摘要缓存归档。Docker 曾积累 270 个容器，清掉经核定的 181 个后，可写层从 102.3 GB 降至 39.84 GB。Docker VM 与宿主目录使用不同计量口径。以上数字来自保留的清理回执，本轮未执行清理。

一次六组件 Wires 升级中断验证量出了另一项可避免的成本。将新测试安装的根目录、dpkg 数据库、业务数据和凭据放在同一具名卷，/usr 沿用该容器自己的可写层。初始化卷约 62.7 MB，完成安装和验证后约 457.8 MB。外层挂载设备号从 2097164 变为 2097165，systemd、dpkg、恢复工具和 Main 始终看到同一 Root `39:6611544`。因此这次能用同一份已保存的图和包数据库完成冷启动后的续配。该布局可供后续新场景复用；原有夹具仍按原配对条件续接。[两次启动和磁盘计量](../checkpoints/20261116/actual-native-wires-retained-reinstall-checkpoint.json)

11 月 21 日 04:05 再次采样，当前续接目录用 `du -sk` 计得 7.91 GB，含私有 Maven 缓存 0.69 GB。单个 Core 打包目录已保留 20 个版本，deb 文件长度合计 4.08 GB。本次在同一六组件卷上升级 Core 并实测事件，停机卷从 447,028 KiB 增至 448,176 KiB。完整包留存与现场复用的成本可以分别核算；APFS 克隆的实际独占块没有单测。[目录计量](../../../.management-close-20261116/core-rtc-hotplug-20261120/clock-sync-confirmation/event-order/disk.json) · [停机卷验证](../checkpoints/20261116/actual-native-wires-retained-reinstall-checkpoint.json)

两条文件数据库路径继续复用同一六组件卷。H2 修改属于网关默认 Core，验证升级时保留了一份 204,227,890 字节的新 deb。SQLite 属于可选 OSGi provider，在同一 Main 内实际安装、更新、卸载与重装，直接使用 provider 和 JDBC bundle。两次正常停机的卷占用分别为 450,540 KiB 和 465,856 KiB。SQLite 验收沿用 H2 已安装的 Core 包。[实装和离线读回](../checkpoints/20261116/actual-native-wires-retained-reinstall-checkpoint.json)

## 本次续接采用的简化

验证未决改密时，六组件升级和单包移除共用一个 205,079,022 字节的 Core deb。独立单包容器正常移除后保留 6 份已核对的合成业务文件，Docker 报告该停机容器的可写层为 29,324,301 字节。包长度与可写层属不同口径。此处未创建整机快照。[原始发行长度](../../../.management-close-20261116/core-rtc-hotplug-20261120/pkcs12-package-admission/core-release.json) · [单包冷读与可写层计量](../../../.management-close-20261116/core-rtc-hotplug-20261120/pkcs12-package-admission/core-only/cold.json)

PKCS12 随机口令修订复用原六组件卷。205,077,978 字节的新版 Core deb 中，只有一个 keystore provider JAR 改变。中断前后的多次 JVM 冷读共用每组临时密钥库，只在原网关部署一次后继包。正式 Main 使用原业务钥验签，原库与口令文件的指纹保持。[包差异与大小](../../../.management-close-20261116/core-rtc-hotplug-20261120/pkcs12-random-recovery/provider.json)

启动器同步修订复用原六组件卷和缓存。新 ARM64 Core deb 长 205,072,788 字节，621 个常规载荷文件中仅发布器和配对清单内容变化，233 个 JAR 均与上版相同。三个入口的故障注入和正式 Main 启动共用这一包。此处计包文件长度，未测 APFS 独占块。[包内逐文件核对](../../../.management-close-20261116/core-rtc-hotplug-20261120/launcher-writeback/package-difference.json)

11 月 21 日晚复核 Core 时，两个已有 ARM64 deb 已足够回答 Java 载荷有无变化。实装的 233 个 JAR 与两版发行件逐项相同，51 个源码改动均在发行目录。复用只读 Maven 缓存和短时容器，补跑了变更的 189 项发行测试，以及先前因缺专用文件系统而跳过的 2 项。全程新增 deb 数为零，原六组件保持停机。[核对范围与实测](../checkpoints/20261116/actual-native-wires-retained-reinstall-checkpoint.json)

同一版本只建立一份工作检出，RTC 先跑脚本与 unit 定向回归，需要验证包实际安装时再组装当前版本。对必须安装的候选保留 Git tree、包摘要、成套文件清单和原始维护回执。涉及原 Root 的业务沿原现场继续；可重建的编译输出单独放置。这项选择削减构建副本，保留已冻结的 12 项验收范围。

11 月 20 日复用 Core 构建目录时，第二次打包因旧的时钟配对清单已存在而退出。将八个生成目录及已过滤的 Debian 控制脚本移到暂存位置（合计约 1.46 MiB）后，同一源码检出和原 Maven 缓存成功产出 204.21 MB 的 ARM64 deb。这个例子明确区分了必须重建的包输入与可以复用的源码和依赖。

Core `6bfda3c94c` 已在正式打包入口处理这项重复工作。它先核对八个 broker 输出目录的结构，再用当前输入重建各自的文件与配对清单。十项包测试通过。实测在八个旧目录各留一份废弃输入，原工作树直接打包，八份废弃输入均被剔除。固定为已实装的 `d0f5bed692` 包版本，与原 ARM64 deb 全文件 SHA-256 相同，第二次构建用时 7.98 秒。两份用于同版本对照的临时包在比对后移除，保留一份正式原件。[打包回归与归档摘要](../checkpoints/20261116/actual-rtc-hotplug-and-addon-upgrade.json)

本轮开始、创建新工作树前，宿主按相同 du 口径实测工作区 335.17 GB（327,315,988 KiB），卷可用约 173.11 GB。

本次核实 18 仓的本地 WIP 引用及 tree 全部匹配交接，14 个主目录由用户手动切换到 heyoulin-kura。Core 主目录当前不含 RTC 草稿，交接补丁摘要精确匹配。最初三个现场的七个进程、38 份私有文件、六份运行数据与原基线一致。7fd 与六组件夹具均停机 exit 0。7fd 的 disabled、41 输入和 16 回执取自停机记录。冻结闭环仍为 **0/12**。外部延期仅为真实 NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。

依据见[最新 Wires 检查点](../checkpoints/20261116/actual-native-wires-retained-reinstall-checkpoint.json)、[冻结台账](../checkpoints/20261116/OPEN-TASKS.md)、[网络决策](../management-network-nm-decision-20261105.md)、[首次清理回执](../checkpoints/20261116/cleanup-status.json)及[最近清理回执](../../../.management-close-20261116/core-rtc-hotplug-20261120/session-handoff/cleanup-final.json)。

本轮实际续做复用了同一工作树与 Maven 缓存。第一次组装发现运行时 JAR 与独立恢复工具中的 JAR 摘要不同，发行检查拒绝了该候选。保留失败目录、在空组装目录重新生成后，14 份时钟配对文件均已对照新 deb。正式 Debian 六组件现场和较旧 Ubuntu 时钟现场分别停在各自已验的版本，详细范围见[本轮检查点](../checkpoints/20261116/actual-rtc-hotplug-and-addon-upgrade.json)。

北京时间 22:25 再次以 `du -sk` 计量，当前续接目录连同原交接共 3.48 GB，项目私有 Maven 缓存约 0.54 GB，八个可选 sibling 的工作树、测试与包合计约 0.54 GB。后两项包含在续接目录内。此次复用主机依赖作为只读来源，代码仍能按 Git 版本生成独立 deb。

### 单独验证 Core deb 的实测成本

已构建同一提交的 Core bundle、密钥工作器和离线恢复工具时，可在 `kura/distrib/kura-core` 模块执行 Maven 的 `prepare-package org.vafer:jdeb:1.14:jdeb@gateway-deb`。沿用同一个 JDK 21、依赖缓存、Git 版本和包修订号。本次两条命令分别从干净的八个配对输出目录和过滤控制脚本目录开始。源码或 JAR 有变化时，仍需先构建受影响模块。

| 同一提交 `8947d37475` | Maven 用时 | deb 之外的输出 |
| --- | ---: | --- |
| 完整发行 package | 30.215 秒 | ZIP、独立安装脚本、catalog 共 408,304,863 字节 |
| 只组装 deb | 8.585 秒 | 上述三份保留件的内容、inode、mtime 均保持 |

两份 deb 的整体 SHA-256 相同。另逐项比对了 7 个控制条目和 595 个载荷条目的内容与元数据。比对用的临时 deb 已删除，正式包及回执保留。在新验收目录只需 deb 时，本例可少生成约 408 MB 的附带产物。客户安装器和 ZIP 交付的验证仍采用完整发行构建。[构建对照回执](../../../.management-close-20261116/core-rtc-hotplug-20261120/deb-only-comparison/verified.json)

jdeb 在模块的 target 中写入 Debian 发行清单。临时输出若命名为 `validation.deb`，清单也会引用这个名字。对照打包时应在独立目录沿用正式包名。已用此方法重新生成清单，并以保留的正式包核验文件名、大小、MD5、SHA-1 和 SHA-256，原包 inode 和修改时间均保持。[发行清单核对回执](../../../.management-close-20261116/core-rtc-hotplug-20261120/deb-only-comparison/canonical-manifest.json)

### 一次小改动实际新增了多少包数据

本次确认文件修复提供了一个可逐字节复核的样本。Core 从 `db514f2a57` 变为 `bdf7690c8e`，ARM64 deb 从 204,214,708 字节变为 204,216,986 字节。流式比较这两个包，538 份普通文件中有 525 份内容完全相同，占新包未压缩普通文件字节数的 **99.97%**。其余 13 份分别是同一 Python 模块的 7 份部署副本，以及三个 broker 在两个安装位置的 6 份配对清单。JAR 和其他业务资源沿用原内容。保留这次验收用的完整 deb，目录里就会多一个约 204 MB 的文件。

北京时间 23:57 抽样，本轮续接目录约 4.83 GB，其中 Core 组装目录约 2.26 GB；该目录已有八个 ARM64 deb，文件长度合计 1.63 GB。私有 Maven 缓存单独计量约 0.54 GB。目录可能共享文件块，上述数字不能直接相加。这组实测解释了两种不同成本：每次留存完整包是主要增量，运行时为权限隔离而部署的 helper 副本只占这次修改涉及的少量字节。[两包逐文件比较](../../../.management-close-20261116/core-rtc-hotplug-20261120/typed-delivery-interruption/package-delta.json)

还对这两个原始 deb 做了一次临时归档实验。以完整旧包为基准，Zstandard 1.5.7 生成的增量为 91.35 MB，比再留一份完整新包少 112.87 MB（55.3%）。用旧包和增量流式还原，新 deb 的整包 SHA-256 完全相同。即使未压缩载荷有 99.97% 的字节相同，直接存放已压缩 deb 的二进制差异仍要约 91 MB。这个方法适合已有可信基准包的历史归档；实装和续接仍使用可直接交给 dpkg 的完整包。本次只计量并校验，原包均保留。[整包还原实测](../../../.management-close-20261116/core-rtc-hotplug-20261120/typed-delivery-interruption/archive-delta-measurement.json)

### 为什么保留了容器仍可能接不上原操作

六组件夹具和独立 BIND 夹具各出现一次同类情况。独立夹具先在原 Root 完成 Core 与 Networking 联合升级，并从正式 Main 验证锁竞争。正常停机再启动后，BIND 目录 inode 仍为 4928376，内核报告的设备号却从 1048815 变为 1048582。现行配对将机器标识、设备号和 inode 一起纳入 Root 身份，因而在 Position 装包前停下。保留容器和磁盘字节足以保住证据，仍可能缺少继续执行所需的原挂载身份。

这揭示了另一项工期成本：若验收需要跨重启继续写同一份台账，选用夹具前就值得验证其文件系统身份能否跨停机保持。对这两个现存夹具，复制目录、重算身份或凭相同 inode 放行都会改变验收条件。此项观察限定在这两次 ARM64 容器续接，尚无物理网关上的同类结论。[实装及停放记录](../checkpoints/20261116/actual-rtc-hotplug-and-addon-upgrade.json)

11 月 21 日，独立 Core 夹具出现第三例。修正版 deb 在连续运行的同一 Root 上完成了三类 Java 记录的 purge 和恢复。重启容器后，机器标识、根目录 inode 6481777 和三个 actor 目录的 inode 均保持，挂载设备号从 1048815 变为 1048582。本次在读取待确认编号之前就结束了续接。团队可以用这个例子估算测试安排的代价：一组需要同一 Root 的维护步骤放在一次开机窗口内，可复用同一安装和同一份包；跨停机身份连续性则须单独验证。[本次原 Root 核对](../checkpoints/20261116/actual-core-network-sender-purge-checkpoint.json)

### 四个发行包更新后的 Docker 层

11 月 21 日用当前 Core、GWT、Deployment、Wires 四个 deb 重建 Alpine ARM64 镜像。Docker 报告旧镜像为 677.77 MB，新镜像为 679.04 MB。两者各有六个文件系统层，前两层的内容标识相同。整树复制的 `/opt/eclipse` 层分别为 369 MB 和 370 MB。因此，即使新旧镜像的总大小只差 1.27 MB，保留两个版本仍会留下不同的运行时层。宿主实际新增的物理块还受压缩、共享和构建缓存影响，未在此次单独计量。

本轮复用现成的四个正式 deb；核对原件后移除构建上下文的四份 APFS 克隆，逻辑文件长度合计 358.09 MB。升级和回滚还复用了原数据卷与空的 /load 卷，保存两套数据副本的开销在这一步可以避免。[镜像层、发行包与原卷检查点](../checkpoints/20261116/actual-arm64-docker-nonroot-launcher-checkpoint.json)

四个 YOFC 可选 DP 合计 25,075,532 字节。这次 ARM64 实装将四包安装在同一卷上，回滚到基础镜像后，DeploymentAdmin 仍读到四个原版本，四个 bundle 均 ACTIVE。核对宿主原件后移除了构建上下文中的四个 deb 和四个 DP，合计 383,170,410 字节逻辑长度。这些上下文文件是 APFS 克隆，未据此推算物理磁盘释放量。[可选包、数据卷与日志实装记录](../checkpoints/20261116/actual-arm64-docker-nonroot-launcher-checkpoint.json)

11 月 21 日 03:32，单独复用的续接目录约 7.50 GB。Core 组装目录保留 18 个 ARM64 deb，逻辑长度合计 3.68 GB。日志轮换修复使 provider JAR 增长 841 字节，整个新 Core deb 为 204.23 MB，比旧包大 956 字节。版本间增量很小，保留每个可独立安装的完整包仍有固定成本。[实际包及目录计量](../../../.management-close-20261116/core-rtc-hotplug-20261120/log-provider-rotation/size.json)

在随后验证 Docker 加载目录时，同一份 Core deb、三个 sibling deb 和四个 DP 都沿用原件。镜像组装后移除了已对摘要的上下文副本，逻辑长度共 383.17 MB。两个临时 Main 已完成对照，回收其 Docker 可写层共 692.60 MB，原数据卷与最后的停机实例保留。Docker 可写层大小和 APFS 文件逻辑长度分开计量。[启动器实测与回收记录](../checkpoints/20261116/actual-arm64-docker-nonroot-launcher-checkpoint.json)

### 时钟包与串口定位的实际成本

11 月 21 日的时钟修复让 bundle 从 43,064 字节增至 43,144 字节。旧、新 Core deb 分别为 204,226,474 和 204,226,478 字节。为比较停用后的网络请求，保留两份正式包要再计约 204 MB 逻辑长度。文件系统实际新增物理块未单测。[发行包和 UDP 计数](../checkpoints/20261116/actual-registered-main-clock-and-purge-checkpoint.json)

六组件串口验收把 11 个已有安装包以只读方式直接挂入夹具，复用了 374,893,280 字节输入。一次开机、一次定位配置覆盖 Position 升级、独立 purge 重装、配置中断恢复三个阶段，每次以新的串口报文核对坐标和 UTC。旧包在端口打开后立即因空闲超时退出，完整流程因此发现了单查服务注册看不到的客户故障。[串口实装、原 Root 与安装恢复](../checkpoints/20261116/actual-standalone-position-six-conversion-checkpoint.json)

读到定位后，进一步实测了空闲串口的负载。第一版超时修复在 1.20 秒内进行了 1,324,688 次读取；加入 25 毫秒等待后，同一窗口内为 41 次。两次在随后输入报文时均交付了定位。此处计数来自实际伪终端的读取调用，未测整机功耗。同期当前 Core 全量构建耗时 11 分 43 秒，其中证书吊销集成用例占 424.4 秒。[串口负载](../checkpoints/20261116/actual-standalone-position-six-conversion-checkpoint.json) · [Core 完整构建](../checkpoints/20261116/actual-registered-main-clock-and-purge-checkpoint.json)

仅复用中的 Core 打包目录就保留了 17 个不同版本的 ARM64 deb，合计 3,471,719,790 字节逻辑长度。逐文件对照其中首版 `66cce5e` 和末版 `3fe46ee`，末版有 539 个常规文件，443 个路径和内容完全相同，合计 93,289,145 字节。另有 46 个同路径 JAR 的整包摘要改变，其中 42 个的直接 `.class` 条目逐一同名同内容，而完整解包后所有文件条目仍相同的仅有 1 个。仅凭编译类相同来清理历史包会遗漏配置、资源或内嵌依赖的变化。此次对照流式读取原包，保留了全部 17 个历史版本。[发行包逐文件计量](../../../.management-close-20261116/core-rtc-hotplug-20261120/position-idle-poll/package-storage.json) · [JAR 内容对照](../../../.management-close-20261116/core-rtc-hotplug-20261120/position-idle-poll/jar-storage.json)

### Watchdog 维护的一次增量

这次复用了已有 Parallels VM 和六组件卷。Linux 的两个设备入口指向同一只软件 watchdog，独立进程持有设备、Java 服务退出和安装器接手都可在同一轮观察。它揭示了旧版在设备不允许停用时仍放行包维护的问题。新版 Core 的完整 ARM64 deb 为 204,236,454 字节。打包时曾在源码提交前生成一份带旧提交号的候选；核对正式包和原安装包后移除了该未安装候选的 204,236,452 字节文件。两者是文件长度，实际释放的 APFS 块数未计量。[发行包、Java 与 Linux 交接记录](../checkpoints/20261116/actual-arm64-watchdog-maintenance-admission-checkpoint.json)

11 月 21 日，复用已构建的 ARM64 Core 包，在现有 Parallels 虚机内另建 Debian 安装，完成正式 Main 写虚拟 RTC 的实测。保留该安装及原回执占用 1,149,644 KiB。核对摘要后移除了宿主的 308,849,518 字节传输归档和虚机内的 204,535,994 字节安装包副本。两处分别计量，未测量 APFS 实际回收量。[独立安装、回执与存储记录](../checkpoints/20261116/actual-registered-main-clock-and-purge-checkpoint.json)

## 11 月 21 日较早的计量与验证记录

11 月 21 日 12:15–12:17（上海时间），宿主工作空间实测 **350.66 GB**。历史工作目录占 **255.70 GB**，中心的 `target` 占 **78.76 GB**，合计 **95.4%**。与 10:30 的同口径读数相比，工作空间增加 3.70 GB，中心 `target` 的读数相同。Core 构建目录的 ARM64 deb 从 38 份、7.76 GB 增至 42 份、8.58 GB，解释了其中约 0.82 GB。计量采用目录占用，deb 采用文件长度，Parallels 虚拟磁盘另计。[12:17 目录实测](../../../.management-close-20261116/core-rtc-hotplug-20261120/firewall-flooding-publication/storage.json)

12:44 再数同一 Core 打包目录，已有 44 个 ARM64 deb，文件长度合计 8,990,793,594 字节。最新两份完整包共 409,999,816 字节。两轮修复共用已有 worker 和 VM，分别在原六组件卷和防火墙专用安装验证。后者通过只读挂载使用 deb，冷停机时占用 1,161,408 KiB。这里没有重测整个宿主工作空间。[两次实装、版本和冷读](../checkpoints/20261116/actual-installed-firewall-action-receipt-checkpoint.json)

随后修复了从注释误判转发请求的问题。只重建两个 Networking deb，文件长度共 9,864,350 字节，复用现有 Core。同一组站点输入在 ARM64 上分别运行旧、新启动器，旧版将两族转发都开启，修正后只有含真实转发链规则的 IPv6 开启。Linux 文档说明改变 IPv4 的全局转发开关会调整主机与路由器的默认参数。[新旧包与内核读回](../checkpoints/20261116/actual-installed-firewall-action-receipt-checkpoint.json) · [Linux 内核转发参数](https://docs.kernel.org/networking/ip-sysctl.html)

12 个 sibling 仓需要各自携带安装和卸载用的注册表维护脚本，同一项权限修复因此涉及 13 个 deb。它们的总长度是 259,283,908 字节，比上一组只多 5,034 字节。逐包解包后，仅 Wires 和 Position 各有一个额外变化的 JAR，差异都是 OSGi 版本后缀。此次直接复用已构建的 Core 完成原六组件升级，八个可选包的旧、新发行件也以只读方式挂入同一测试环境。[两组发行件与实装对照](../checkpoints/20261116/actual-passive-addon-dpkg-rollback-checkpoint.json)

最后一个 sibling 卸载时，dpkg 会按数据库中的目录归属处理安装链接。真实 Core 首装、八包联合移除，以及 Core 自身正常和中断卸载共用了一个 ARM64 环境。此次新增一个 205,003,262 字节的 Core deb；13 个输入包共 300,839,316 字节，以只读方式挂载，其中已包括该 Core 包。停机时容器可写层为 32,163,461 字节。三组数字的口径不同，不能相加作为宿主净增量。[包归属、卸载回执与计量](../checkpoints/20261116/actual-passive-addon-dpkg-rollback-checkpoint.json)

在已有 Networking 的安装上，Core 即使把公共目录写进了 deb，条目的顺序也会影响升级。前版先遇到暂时悬空的旧链接，被 dpkg 拒绝。仅调整打包顺序后，修正版比前版小 42 字节，但仍需一个 205,003,220 字节的完整包。修正包先在与失败现场相同版本的前任上完成升级，再用于原六组件卷。当前 Core 打包目录有 46 份 ARM64 deb，逻辑长度合计 9,400,800,076 字节；此处未重新测量整个工作空间。失败的 Parallels 安装保留其原始维护记录。[升级现场及两份正式包](../checkpoints/20261116/actual-passive-addon-dpkg-rollback-checkpoint.json)

普通 Main 的转发写入复用了既有 socket、两次批准校验和原操作日志。新协议同时核对网络命名空间、协议族和开关原值。隔离安装完成正向调用后又用于 Networking 移除、purge、重装，三次均从该 Root 查回同一结果。该安装正常停机时容器可写层为 262,439,298 字节；新 Core 包、两个 Networking 变体和共享编译缓存分别计量。[实装与维护数据](../checkpoints/20261116/actual-installed-firewall-action-receipt-checkpoint.json)

防火墙这一轮说明了两种不同的工期来源。正式 Main 遇到现场实际保存的洪泛防护规则，原 worker 在批准前拒绝。补齐 TCP flags、MSS 和 IPv6 扩展头后，ARM64 内核收下了针对这些规则的变更。随后发现 Cargo 复用了旧二进制，即使源归档已更新。构建现在核对成品自报的源码摘要。这些问题都影响交付。另一部分工期和磁盘来自为每个中间改动生成完整的约 205 MB Core 包、再逐轮安装。可将同一交付的修改合并组包，日常验证直接运行变更的安装钩子或只构建 worker，到真实跨组件验收时再保留完整包。[实际调用与发行件](../checkpoints/20261116/actual-installed-firewall-action-receipt-checkpoint.json)

当前 Java 防火墙模型继续服务已有调用方。规则发布经本地 systemd socket 交给按请求启动的 worker，独立运行的 Kura 可直接使用它。原六组件 Root 留存的现场规则与本次冷启动内核状态不同，现有证据只证明了 Main 配对、拒绝和内容保全。另建的 Parallels 防火墙验收 Root 以只读挂载复用安装包，正常停机时占 1,161,068 KiB。它在安装阶段留下缺少成功回执的原操作。该 Root 计入本轮验证成本。[原六组件拒绝记录](../../../.management-close-20261116/core-rtc-hotplug-20261120/firewall-publish-worker/installed-refusal.json) · [新安装停机冷读](../../../.management-close-20261116/core-rtc-hotplug-20261120/firewall-flooding-publication/fresh-parked.json)

获准用于防火墙验收的另一临时安装占 1,161,212 KiB。它复用同一份基础镜像和只读 deb，实际安装了 Core 和标准 Networking。正式 Main 通过 OSGi 完成一次仅限回环的规则增加和移除，两笔均有内核回执。新安装与前一中断现场在 Parallels 客体内合计约 2.38 GB。计量是客体目录占用，PVM 在宿主的实际分配量另计。[正向回执、六表及冷读](../checkpoints/20261116/actual-installed-firewall-action-receipt-checkpoint.json)

10:30 的宿主目录计量为 **346.96 GB**，其中历史工作目录 **252.85 GB**、中心 target **78.76 GB**，两者占 95.6%。较 09:58 增加 259.7 MB；同期保留的 Core ARM64 包从 37 份增至 38 份。这一轮防火墙发布只重建两个 Networking 变体，复用同一 ARM64 镜像和六组件卷。[10:30 目录实测](../../../.management-close-20261116/core-rtc-hotplug-20261120/firewall-configuration-publication/storage.json)

09:58 的目录复测为 **346.70 GB**，其中历史工作目录 **252.59 GB**、中心 target **78.76 GB**，合计 95.6%。较 09:40 增加 227.0 MB。两个读权限实现随原包交付，本次另存一个 204.31 MB 的 Core 完整包。37 份 Core ARM64 deb 的文件长度合计 7.56 GB。上述统计按宿主目录计量，虚拟机卷另计。[最新目录计量](../../../.management-close-20261116/core-rtc-hotplug-20261120/main-ownership-markers/storage.json)

本轮发现，一份 ConfigAdmin 配置原先分三类先删除再追加，遇到 NAT 拒绝时已经修改其他规则并发出了变更通知。两种地址族都复现了四次发布。现在先准备完整配置，转发开关不满足时执行零条内核命令，成功时每族发布一次。一次 Java 发布在当前后端仍涉及多条命令，跨表提交的中断恢复仍有单独成本。[配置与内核实测](../checkpoints/20261116/actual-installed-firewall-action-receipt-checkpoint.json)

11 月 21 日 10:20 在原六组件卷测得，正式 Main 的调用线程撤去 NET_ADMIN 后，仍可通过独立读取服务核对 IPv4、IPv6 各三张内核表。服务和调用线程的有效能力分别为 0x1000 和 0x0400。内核规则发布仍走旧路径。这次沿用同一卷和 Maven 缓存；核对最终 Core deb 后移除了本轮未安装的中间包，逻辑长度 204.37 MB。[实装读数与包记录](../checkpoints/20261116/actual-installed-firewall-action-receipt-checkpoint.json)

Core 和 Networking 已在原六组件卷修正归属标记的读取路径。普通 Main 遇到目录拒绝时向本机只读服务查询两个固定标记；服务核对正式 Main 并返回当时的存在性。原卷保留 0700，三次查询共 33 ms。这个进程使用 root 身份遍历原目录，Linux 能力集为空。内核防火墙的读写权限仍在旧执行路径上。[正式 Main 实测](../../../.management-close-20261116/core-rtc-hotplug-20261120/main-ownership-markers/reader.json)

11 月 21 日的两种 Networking 发行包复用同一份防火墙 JAR，共 9.81 MB。两个 ARM64 网络命名空间也复用同一镜像。实测发现旧代码清空一族规则时重复发布五次，现已合并为一次。原六组件只升级标准包，正式 Main 又暴露出一个跨组件约定：Core 把 /etc/kura 设为 root 专属的 0700，Networking 却要求普通 JVM 能判断其下两个归属标记。这使独立运行的 Main 在修改规则前拒绝操作。[发行件](../../../.management-close-20261116/core-rtc-hotplug-20261120/firewall-forwarding-admission/releases.json) · [原卷权限实测](../../../.management-close-20261116/core-rtc-hotplug-20261120/firewall-forwarding-admission/ownership-refusal.json)

11 月 21 日 09:40 完成的目录复测为 **346.48 GB**，其中历史工作目录 **252.37 GB**、中心 target **78.76 GB**，两者合计 95.6%。Core 打包目录保留 36 个 ARM64 deb，文件长度合计 7.35 GB。这轮只构建有代码变动的 GWT 和 Networking，两个新 deb 共 43.52 MB。SQLCipher 联合恢复和 Main 权限实测均复用了现有包和原安装卷。[目录计量](../../../.management-close-20261116/core-rtc-hotplug-20261120/main-net-admin-preflight/storage.json)

11 月 21 日 09:19 起按同一口径复测，工作区 **346.43 GB**，其中历史工作目录 **252.32 GB**，中心 target **78.76 GB**。Core 打包目录有 36 个 ARM64 deb，文件长度合计 7.35 GB。较 07:06 增加 5 个包，共 1.02 GB；同期工作区目录计量增加 1.07 GB。已登录 HTTP 下载复用了安装包和测试卷。修复旧 PID 接管后另建了一个 204.24 MB 的 Core 包。宿主卷当时可用 141.79 GB，APFS 共享块和虚拟机卷分别计量。[目录与发行件复测](../../../.management-close-20261116/core-rtc-hotplug-20261120/main-pid-handover/storage.json)

11 月 21 日 07:06 起按工作区目录计量，工作区为 **345.36 GB**，较 05:08 增加 1.48 GB。历史工作目录占 251.25 GB，中心 target 单独复测为 78.76 GB，两者约占 95.6%。Core 打包目录现有 31 个 ARM64 deb，共 6.33 GB。较上次多出的 7 个包合计 1.43 GB。两种计量数值接近，APFS 共享块仍影响实际物理增长。本轮 Linux 编译容器为普通构建用户复制了 0.67 GB 缓存，源码与临时缓存已随容器回收，六组件卷另行保留。Docker 的销毁事件已核对。清理程序未及时保存可写层总量，因此无法量化此次容器实际释放的空间。[目录与发行包计量](../../../.management-close-20261116/core-rtc-hotplug-20261120/linux-arm64-core-osgi/storage.json) · [临时构建回收记录](../../../.management-close-20261116/core-rtc-hotplug-20261120/linux-arm64-core-osgi/builder-removed.json)

11 月 21 日 05:08 前后，工作区按 `du -k` 实测 **343.88 GB**，宿主可用 **147.51 GB**。其中历史工作目录 **249.77 GB**，中心仓的 `target` **78.76 GB**，两者合计约 **95.5%**。中心 `target` 中，网络权限与注册现场占 49.12 GB，上游逻辑同步目录占 17.73 GB。当时 Core 打包目录留有 24 个 ARM64 deb，文件长度合计 4.90 GB。APFS 可共享数据块，上述目录计量不能直接换算为可释放空间。[本次容量明细](../../../.management-close-20261116/core-rtc-hotplug-20261120/pkcs12-file-writer-identity/storage.json)

这次还定位到一种可以避免的重组装。API、Crypto 和 Configuration 三个 JAR 在运行时与冷恢复目录的内容逐项相同，只有清单中的 OSGi `Bundle-Version` 不同。例如 API 的构建后缀分别为 `202611202101` 和 `202611202040`。打包正确拒绝该组输入，从同一缓存重建恢复工具后通过。正式 `build-all.sh` 会为一次构建固定 `KURA_BUILD_QUALIFIER`。拆开运行 Maven 时，同一批候选也应显式传递相同的 `-Dkura.build.qualifier`，并让运行时和恢复工具使用同一批依赖。从当前 Git 只取 API 模块和构建父 POM，用同一个固定后缀连续执行两次干净打包，606 个文件条目以及整个 JAR 的 SHA-256 均相同。该验证复用了 Maven 缓存，临时源码和编译目录随即回收。[两次构建摘要](../../../.management-close-20261116/core-rtc-hotplug-20261120/pkcs12-file-writer-identity/qualifier-proof.json) · [实际差异](../../../.management-close-20261116/core-rtc-hotplug-20261120/pkcs12-file-writer-identity/unpaired-build.json) · [正式构建入口](../../../.management-close-20261116/core-rtc-hotplug-20261120/wip-core/build-all.sh)

这次防火墙修复复用系统已安装的 `iptables-restore` 做语法核验。发行包生成的 49 条 IPv4、85 条 IPv6 规则均能装入隔离内核；使用系统的 nft 文本翻译器时，25 条 ICMPv6 规则只留下了注释，其中有针对具体类型、代码和链路本地来源的限制。这次发现促成了后面的共享规则库。这轮只构建 Networking 和 firewall-only 两个 deb，合计约 9.83 MB，复用了原 Core 包、原 VM 和原六组件卷。[规则逐项对照](../../../.management-close-20261116/core-rtc-hotplug-20261120/firewall-native-policy-compiler/kernel-observation.json) · [两份发行包](../../../.management-close-20261116/core-rtc-hotplug-20261120/firewall-native-policy-compiler/releases.json)

原布局的防火墙解析和内核规则编码现已抽为一个 Rust 库，agent 和独立调用方共用。Linux ARM64 实测将 134 条规则放入一个 40,848 字节的事务，其中 40 个 ICMP 匹配的类型、代码范围和取反位都从内核读回。最后一条规则被拒绝时，六张表一起保持原值。独立读取程序随后从混有现场规则的六张表找回了全部原句柄。它与 agent 共用的 Rust 源码包是 21,944 字节，ARM64 能单用这个包离线构建。原六组件卷升级使用的 agent deb 为 7,048,836 字节。正式 Main 接入写服务和按原操作编号恢复仍需实装。[规则归属及源码包](../checkpoints/20261116/actual-installed-firewall-action-receipt-checkpoint.json) · [内核接受回执](../../../.management-close-20261116/core-rtc-hotplug-20261120/standalone-firewall-policy/kernel-valid.json) · [内核拒绝回执](../../../.management-close-20261116/core-rtc-hotplug-20261120/standalone-firewall-policy/kernel-invalid.json)

这轮两次实际升级复用同一六组件卷，各保留约 7.0 MB 的 agent 安装包。构建现场的 macOS debug 目录为 594 MiB，Linux debug 与 release 分别为 574 MiB、497 MiB。它们供多轮构建共用。这解释了为什么小范围的规则变更也会伴随较大的目录占用。两份 deb 的文件长度已知，APFS 独占块及本轮缓存净增量没有单独计量。[本轮目录和包计量](../../../.management-close-20261116/core-rtc-hotplug-20261120/startup-execution-release/storage.json)

11 月 21 日 14:02 的 Networking 修复复用了现有 NetworkManager 扫描实现。旧 OSGi 接口还依赖每个接入点的完整速率列表和能力标志，直接返回新状态 API 的最高速率会丢失数据。因此保留原解析器，主动扫描由 NetworkManager 负责。标准 Networking deb 为 9.41 MB，本轮复用原六组件卷和 Core 发行件。完整 Networking 构建通过 2266 项测试，原卷上的正式 Main 已绑定新版 provider。该次 JVM 的有效能力仍为 0x1400。[发行件和运行证据](../checkpoints/20261116/actual-nm-typed-wifi-scan-checkpoint.json)

11 月 21 日 14:58 的两次实装暴露了另一笔可量化的构建成本。撤去 Main 的网络管理能力、恢复 Core 单独升级时丢失的 Networking 启动器，以及修正恢复后的文件权限，共产生三个 ARM64 Core deb，文件长度合计 615.06 MB。两处实装均沿用现有 Root。实际发布时，正式 Main 的 134 个线程均只保留低端口绑定能力，内核写入由独立 worker 完成。未提供站点策略时，默认启动仍有 17 条已保存防洪规则未被 Java 模型还原，这一差异单独保留。[安装、进程能力和包长度](../checkpoints/20261116/actual-installed-firewall-action-receipt-checkpoint.json)

Networking `a7dfc51` 的双族冷读修复合并了网络运行时和防洪 provider 重复定义的 38 条 IPv6 规则。只组装标准和 firewall-only 两份 deb，共 9.88 MB，并复用原 Core 与六组件卷。ARM64 验证后的 9,949,754 字节临时 JAR 均已逐一核对包内摘要并回收。两种发行件均还原了上轮保留的规则文件；没有站点策略时的内核冷恢复继续单独验收。[发行件、冷读与实装](../checkpoints/20261116/actual-installed-firewall-action-receipt-checkpoint.json)

Core `ad2a586c31` 在已有发布回执中保存经确认的每族规则摘要，继续使用同一份安装日志。隔离内核恢复 83 条 IPv6 命令时沿用了正式发布 worker 的差异计算和内核匹配。worker 源码包为 46,569 字节，更新后的 ARM64 Core deb 为 205,042,346 字节。实装复用既有防火墙临时卷，六组件卷本轮保持停机。[原安装冷读及恢复试验](../checkpoints/20261116/actual-installed-firewall-action-receipt-checkpoint.json)

此次转发恢复复用了同一临时安装和编译缓存。Core deb 占 205,062,620 字节，标准 Networking 与 firewall-only 两个 deb 合计 9,880,928 字节。三包文件长度合计 214,943,548 字节；宿主物理占用增量没有单独测量。此次两次启动将费用对应到一项具体风险：防火墙服务先于 Main 恢复授权后，Main 仍会按自身保存的配置重新应用规则。随后沿业务配置保存入口的验收补齐了这一点，结果见下段。[发行件与正式 Main 验收](../checkpoints/20261116/actual-installed-firewall-action-receipt-checkpoint.json)

本次跨完整 Main 重启验收复用了原防火墙安装、编译缓存和三份既有发行件，没有新建卷或组装 deb。正式配置快照与注册服务的读回有一处格式差异：同一规则写入快照时保留末尾分号，provider 在返回规则时省略该分号。修正观察器后，核对原快照和实际内核规则即可完成验收。[配置保存与恢复检查点](../checkpoints/20261116/actual-installed-firewall-action-receipt-checkpoint.json)

Deployment 的一次进程中断解释了为什么成功验证原子替换单个文件后，仍要检查两份文件是否一致。原实现先替换同名归档，再等待注册表锁；此时终止进程，目标归档已变，注册表仍保留原行。新版复用既有注册表锁，等待时终止进程，两者均保持。现场沿用原六组件卷和其他五个网关包，只组装 Deployment。两个实装版本分别为 228,300 和 228,322 字节。第二版保留首次创建包目录的兼容行为，两条安装入口均实测。归档和注册表各自重命名，随后对两次重命名之间的进程退出做了实测。[锁竞争、首次创建与业务读回](../checkpoints/20261116/actual-native-wires-retained-reinstall-checkpoint.json)

这两次重命名的恢复复用原暂存文件与注册表锁。两个 ARM64 样本新增的待办记录分别为 1,093 和 1,094 字节；登记完成后删除。服务进程在新一次启动中凭文件身份和内容摘要补完登记，无需再复制业务包。为验证中断与恢复权限，原六组件卷先后安装了 239,302 和 240,140 字节的 Deployment deb，复用了 Core、其余组件和编译缓存。真实 Felix 在返回结果前中断仍需独立证据。[两次重命名、权限与冷读](../checkpoints/20261116/actual-native-wires-retained-reinstall-checkpoint.json)

原六组件卷联合升级复用了 214,477,656 字节的两个现成发行包，安装完成后已移除卷内传输副本。本轮新增零个 deb、虚拟机和 Docker 卷。当前 Core 打包目录的 58 个 ARM64 deb 文件长度合计 11,861,225,310 字节；APFS 共享块使该值与实际可释放容量有别。实际验收只编译了一个 Java 21 只读观察器，由同一次正式 Main 启动核验部署包和 H2。保留两个历史测试目录的 inode 额外需要同卷改名，它们的历史证据才依赖这种身份；正式业务通过包注册、bundle 资源和数据库查询验证。[安装与磁盘计量](../checkpoints/20261116/actual-native-wires-retained-reinstall-checkpoint.json)

本轮在生成 Core 安装脚本后，先用 ARM64 小夹具证明旧脚本会放行一份仍依赖 inode 的回执，再合并组装最终的 Core 和 Deployment。两个新增包分别为 205,064,976 和 242,858 字节。真实联合升级发现两个组件对同一个 Main 停机状态的识别方式不同，必须先用正式包管理流程移除 Deployment，才能沿原编号恢复。这是跨包维护合同带来的工期。默认 Core 打包目录当前有 59 份 ARM64 deb，逻辑长度 12,066,290,286 字节。[实际维护顺序与成本](../checkpoints/20261116/actual-native-wires-retained-reinstall-checkpoint.json)

上次实装需要移除 Deployment 才能继续 Core 配置。本次保留同一个 systemd 生成器及原有维护机制，由时钟安装脚本核对其原始记录。原卷在解包后正常停机，重启后一次 dpkg 配置完成两个包。新增 Core 包 205,065,436 字节，Deployment 包 242,858 字节；后者复用相同实现，四个 JAR 摘要未变。Core 打包目录现有 60 个 ARM64 包，逻辑长度合计 12,271,355,722 字节。本次复用原卷和已有镜像。[同卷中断恢复及组包计量](../checkpoints/20261116/actual-native-wires-retained-reinstall-checkpoint.json)

本轮共同移除和重装沿用原六组件卷及四份既有 deb，合计 163,316,262 字节。验证只临时把这四份包传入原卷，重装读回后删除临时包，新增发行包 0 字节。两次正常停机分别保留了四包 purge 后和重装后的证据。[四包维护记录](../checkpoints/20261116/actual-native-wires-retained-reinstall-checkpoint.json)

在同一业务卷验证六包保留式卸载与重装时，Core 的卸载脚本漏存站点 provider 顺序，修复后新增一个 205,066,056 字节的 Core deb。其余五包沿用原件。此时 Core 打包目录保留 61 个 ARM64 deb，长度合计 12,476,421,778 字节。一次正式 Main 启动同时检查原部署归档和三种数据库的原始记录。[包计量](../../../.management-close-20261116/core-rtc-hotplug-20261120/core-remove-sibling-order/storage.json)

站点自装的两个 OSGi provider 随 Core 保留式重装恢复。本轮仍复用同一卷、五份网关包和两个既有 provider JAR，仅因 Core 卸载程序变动新增一个 205,066,338 字节的 deb。当前同目录 62 个 ARM64 Core deb 合计 12,681,488,116 字节。[包长度](../../../.management-close-20261116/core-rtc-hotplug-20261120/site-provider-retained-remove/storage.json)

Docker 启动器的三版真实 OSGi 工厂更新复用了原镜像、原卷及三个已经构建的 Cloud deb。仅提取的三个工厂 JAR 合计 29,471 字节，新增 deb 和镜像均为零。缺少 Cloud 基础 bundle 时工厂无法注册；提供依赖后基础 bundle 也被激活，既有 MQTT 配置参与了启动。仅凭启动级别或部署目录无法判断这类副作用，需读取 OSGi 服务及运行日志。[服务更新与依赖实测](../checkpoints/20261116/actual-arm64-docker-nonroot-launcher-checkpoint.json)

本轮 ARM64 启动器复用四份现成 deb 和四份原 DP，仅构建一个本地镜像。八个临时 APFS 克隆的文件长度合计 384,044,900 字节，构建完已移除。新旧镜像的逻辑大小分别为 704,587,864 和 704,121,619 字节，相差 466,245 字节。六个最终层中有两个摘要相同。Docker 对新镜像报告共享 308.9 MB、独占 395.7 MB，其中复制整个 `/opt/eclipse` 的层约 396 MB。因此，同时保留两版的存储成本要看独占层大小；镜像总大小之差只能说明两版各自的文件规模接近。宿主 APFS 的物理增量未单独测量。原业务卷和 235 个在镜像内交付的 JAR 均已核对。[镜像层和包输入](../../../.management-close-20261116/core-rtc-hotplug-20261120/docker-current-gateway-release/storage.json)

已绑定 agent 的回执恢复修正只改变 Python 维护代码及测试。将新旧发行清单逐项比较后，复用同一份 ARM64 Rust 二进制和网关原 bridge，本轮新增一个 7,061,640 字节的 deb。四个进程中断检查使用同一包；原网关、中心和数据库保持停放。[构建输入与包计量](../../../.management-close-20261116/core-rtc-hotplug-20261120/bound-agent-receipt-publication/release.json)

2026-11-21 的容器启动器修复复用了四份已构建的当前 deb 和四份原 DP。新镜像比前一版仅多 12,523 字节，两版各约 705 MB，但最终六层只有两层摘要相同。Docker 报告新版共享 308.9 MB、独占 395.7 MB。八个构建输入使用 APFS 克隆，文件长度合计 384,058,352 字节，镜像验完已逐个核对并撤下。构建日志还显示，Dockerfile 在复制启动脚本之后才为 builder 安装 JRE 等依赖；这次启动脚本变化使该步骤重新下载包，单是 headless JRE 这一步就跨过数分钟。没有单独测量宿主物理空间增量。[镜像大小及构建输入](../../../.management-close-20261116/core-rtc-hotplug-20261120/docker-launcher-config-publication/storage.json)

2026-11-21 的独立正式 Main 改密试验，三次启动共用一份合成业务卷和一个容器。停机时该卷占 24,788 KiB，Docker 报告容器可写层为 346,448,373 字节。只读统计容器内 /run/kura-container/configuration/org.eclipse.osgi，177 个常规文件的逻辑长度合计 345,975,616 字节。这一现场的主要写层开销来自 Equinox 框架缓存。应用数据卷、镜像独占层、容器可写层分别计量，宿主物理增量未单独测量。[容器、业务卷与缓存实测](../../../.management-close-20261116/core-rtc-hotplug-20261120/pkcs12-container-resume/storage.json)

本次完整 Core 构建用时 11:37 min，其中 13 项证书吊销列表集成测试用时 424.0 秒。Docker 在回收前报告临时构建容器可写层 1,012,961,093 字节，容器内源码及编译输出占 711,884 KiB，JDK 占 302,284 KiB。单列 Maven 缓存发现 478 个依赖文件被复制，合计 669,839,936 字节，另有 131 个使用链接。实际以构建用户 UID 1000、零有效能力在同一容器上读取原只读目录，609 份文件摘要全部匹配。再用空的可写仓库、依次挂接原缓存和宿主镜像，Maven 离线取到了与当前 deb 相同的 keystore JAR；宿主镜像中同坐标的 JAR 摘要不同。后续同类构建可先验证原缓存的实际可读性，再决定是否创建这份约 670 MB 的副本。修订夹具后已撤下首轮缓存副本，用原只读缓存、空的可写仓库及全新源码目录跑完完整 reactor，临时容器已回收，宿主 APFS 的实际增减未单独测量。[临时构建计量](../../../.management-close-20261116/core-rtc-hotplug-20261120/current-core-osgi-arm64/storage.json) · [离线解析记录](../../../.management-close-20261116/core-rtc-hotplug-20261120/current-core-osgi-arm64/direct-maven.log)

本次旧版容器恢复沿用原卷和原 Main，只在停机期间挂入维护镜像。新 Core deb 为 205,082,404 字节，比上一包多 3,252 字节，解开比较时只有 keystore provider JAR 的内容改变。两次镜像构建使用同一组四个 deb 和四个 DP，各次输入克隆的文件长度合计 384,061,662 字节，验证后已撤下。第二次构建是为维护子进程保留原实例锁。中间镜像已核实没有容器引用后移除标签，保留最终维护镜像和原业务镜像。此处为文件或镜像逻辑大小，宿主物理增量未测。[包、镜像与回收计量](../../../.management-close-20261116/core-rtc-hotplug-20261120/legacy-keystore-links/storage.json)

## 2026-11-22 ARM64 Docker 发行输入与启动器分层

过去只改容器启动器也会让四包网关重新组装。Docker 643d460 将四个 deb、选中的 DP 和组装工具作为安装层的输入，最终镜像单独复制运行脚本。固定发行包，只切换上一代和当前启动器，镜像六层中三层相同，395,674,305 字节的 /opt/eclipse 层直接命中缓存。首次构建 33.23 秒，含下载构建依赖，随后只换脚本用时 2.21 秒。这组时间描述同一环境的首次与缓存构建。镜像分层摘要可独立复核。

非清空构建目录也暴露了正确性问题。Maven 原本会跳过比 staging 文件更旧的同坐标 deb 和启动器，构建成功仍可能发布错误版本。Docker 896b976 按本次解析的包、脚本和 DP 名称组装，同目录回退和恢复均已从镜像读回核对。此方案仍有复制成本。单组八个输入的逻辑长度为 384,061,662 字节，其中 DP 占 25,075,532 字节。本轮回收 17 个隔离副本，逻辑长度合计 768,346,549 字节。APFS 克隆、Docker 层共享和缓存使宿主实际空间变化需要单独计量，本轮只记录了文件长度和镜像层大小。[构建与回收记录](../../../.management-close-20261116/core-rtc-hotplug-20261120/docker-release-inputs/HANDOFF.md)

## 2026-11-22 相同业务卷上的容器可写层

同一业务卷回退、再升级时，密钥库和三份快照均原样保持。最终容器可写层为 346,298,536 字节，持久卷中的实际分配量为 25,223,168 字节。停机读取该容器新增的 /run/kura-container/configuration/org.eclipse.osgi，文件逻辑长度合计 345,939,055 字节。其中 100 份解出的 bundle classpath 文件占 294,664,596 字节，另 74 份框架状态文件占 51,274,459 字节。每个保留的容器都可能单独积累这份运行缓存，即使指向同一业务卷。发布时复用镜像层解决的是构建开销，运行时这部分发生在容器可写层。

本轮核对后移除了两个临时容器，Docker 报告其可写层合计 692,674,238 字节。保留最后的容器与原业务卷供同 Root 续接。全量 Core 构建使用两个只读 Maven 缓存和隔离写目录，全程 11:39 min。其中 13 个文件型 keystore 证书吊销列表集成用例共用 424.3 秒。构建容器已回收。本轮未单测宿主 APFS 物理增减。[现场与空间计量](../../../.management-close-20261116/core-rtc-hotplug-20261120/core-provider-reactor-796e0ba/HANDOFF.md)

## 2026-11-22 隔离部署测试的运行缓存

本次正式 Main 的临时 Equinox 配置目录 /run/kura-container 挂在上限 512 MiB 的 tmpfs。业务密钥、快照、DP 和注册项继续留在独立持久卷。Main 完成真实 DP 安装和卸载，停机后 Docker 报告可写层为 450,042 字节。上一轮采用容器普通可写层存放 Equinox 缓存，停机时为 346,298,536 字节。两轮运行内容不同，本次结果适用于这个隔离测试配置。tmpfs 空间计入运行时内存，本轮没有测峰值，也没有修改部署默认值。本次构建的 16 份包副本逐项核对后移除，逻辑大小合计 768,124,304 字节。[现场与计量](../../../.management-close-20261116/core-rtc-hotplug-20261120/cloud-deployment-maintenance-admission/HANDOFF.md)
## 2026-11-22 未决业务回执的原生包维护

Docker 发行路径验证成对发布时，复用原镜像的独立测试卷停机时占 24,648 KiB。Debian 实装路径需保留 dpkg 数据库、systemd 生成器和可跨停机识别的 Root。本次原生测试在同一个 Root 完成后继升级、解包后停机续配、remove、purge 和重装。初始卷为 61,256 KiB，最终为 286,516 KiB，容器可写层为 4,629,167 字节。205,916,532 字节的 deb 均以只读方式挂入。本轮无新发行镜像或 deb 构建，未单独测量 APFS 净增量。[原生现场与计量](../../../.management-close-20261116/core-rtc-hotplug-20261120/native-pending-dpkg/HANDOFF.md)

## 2026-11-22 Core 原位升级后，正式 Main 沿原目录读回已完成的部署任务

这次实测解释了为什么有些历史需要保留原卷。部署回执绑定 packages 目录的设备号和 inode。解包后备份内容相同，目录身份却已改变。Core 现在通过原维护记录把原目录交给后继版本，正式 Main 据此查回同一个任务。实装复用了已有 ARM64 镜像，单次 Maven 发行打包耗时 8.725 秒，产生一个 205,083,694 字节的 deb；实测入口以硬链接复用它。233 个发行 JAR 与来源基线逐字节一致。停机时隔离系统卷占 285,824 KiB，容器可写层为 4,629,167 字节。这些数值分别描述不同存储对象，不能相加作为宿主磁盘新增量。[实测和存储明细](../../../.management-close-20261116/core-rtc-hotplug-20261120/native-completed-core-upgrade/HANDOFF.md)

## 2026-11-22 移除重装的实际存储增量

沿用 48001 的原容器和原卷，Core 移除时先把原目录保管在同一安装文件系统，重装后将它交还给正式 Main，清除临时保管目录。系统卷从 285,824 KiB 变为 286,804 KiB，两次正常停机之间相差 980 KiB。本轮另外生成一个发行 deb，通过硬链接挂入原现场。发行物、保留卷和宿主 APFS 空闲量使用不同计量口径。[实测明细](../../../.management-close-20261116/core-rtc-hotplug-20261120/native-completed-core-reinstall/storage.json)

## 2026-11-22 带挂载的 Core purge 续办

这次 purge 验收使用独立卷，保留了 48001 的原业务回执。复用此前构建的 Core 和 Deployment deb 及 ARM64 镜像，只编译测试用的 OSGi 观察组件。新卷初始 61,256 KiB，postrm 因业务包仍挂载而拒绝时为 70,932 KiB，解除挂载并完成 purge 后为 69,968 KiB。测量对象是测试卷。[实测记录](../../../.management-close-20261116/core-rtc-hotplug-20261120/native-completed-core-purge/HANDOFF.md)

## 2026-11-22 移除中断后复用已写好的暂存目录

以前 prerm 在发布失败后会删除已完成的暂存副本，进程退出后重试也会重新分配目录。现在按原维护号找回同一个目录，完成一次改名。实测在中断冷读时记录 284,580 KiB，保留重装、业务读回并停机后为 285,956 KiB，始终复用同一测试卷。单次发行打包耗时 10.684 秒，复用已有 ARM64 镜像。首次夹具未命中中断点的普通移除单独记账。[实测记录](../../../.management-close-20261116/core-rtc-hotplug-20261120/native-core-removal-publication/HANDOFF.md)

## 2026-11-22 Camel 两种包维护现场的磁盘用量

这两种验收只要求跨 JVM 保留同一 Root，无需跨整机关机继续。将系统目录放在 tmpfs 后，原快照、注册表和 dpkg 数据在各次 Main 重启间保持，整机退出时释放。复用两版 Camel 和现有 Core deb，两个停止容器的可写层各占 4,582,826 字节；新增持久系统卷和镜像均为零。[计量记录](../../../.management-close-20261116/core-rtc-hotplug-20261120/camel-provider-upgrade/storage.json)

## 2026-11-22 OPC UA 两种安装入口的成本

正式 Main 走 Debian 插件路径时发现 Milo 的 OSGi 导入导出不全和属性默认值失效，为验证修复生成两份 deb，各约 5.6 MiB。验证 Deployment 路径时复用同一修复版本的 5,827,108 字节部署包、已有镜像和 Core、Cloud、Deployment 发行包。本次隔离系统使用内存根目录，停机后容器可写层为 4,629,167 字节，持久卷为零。这个数值只计停机后的容器可写层。两次测试分别检验驱动实际读节点，以及交付包在 Felix 中的登记、重启保留与卸载。[驱动实装](../../../.management-close-20261116/core-rtc-hotplug-20261120/opcua-provider-main/HANDOFF.md) · [部署包实装](../../../.management-close-20261116/core-rtc-hotplug-20261120/opcua-deployment-main/HANDOFF.md)

## 2026-11-22 为何这次升级需要修改 Milo 发行件

上一轮四个补丁 fragment 在新装系统均能解析。同名热升级时，已在运行的宿主保持原版本，结果只有一个 fragment 解析成功，驱动拿到完成回执后读节点失败。修订后的发行件直接在四个 Milo 宿主中声明所需依赖，并使用新的 OSGi 版本让 Felix 更新宿主。上游代码和资源逐字节相同。DP 的 bundle 数从 11 减至 7，Debian 包内 JAR 从 10 减至 6。新 deb 和 DP 各约 5.6 MiB，另保留一份上一轮精确测试过的 5,827,108 字节 DP。两个容器停机后可写层合计 4,826,915 字节，持久卷为零。[升级与业务证据](../../../.management-close-20261116/core-rtc-hotplug-20261120/opcua-deployment-upgrade/HANDOFF.md)

## 2026-11-22 Triton 管理模式与完整运行库的成本

远程 Triton 只需要 gRPC，却曾被本地特权执行器和解密服务的 SCR 引用阻止启动。移除两项依赖后，ARM64 现场用原配置完成端点停启和冷启动。容器模式则保留编排和加密服务。另一个耗时原因是同一 Maven SNAPSHOT 坐标可以留着旧 Crypto API 的导入声明。两个下游包按原 Git 重建后，在同一套其余输入上恢复 ACTIVE。三次 macOS 启动各生成约 325 至 340 MiB 的 Equinox 缓存。测试结束后连同本次临时组装一起回收，删除前计量约 2.03 GiB。保留用于配对的两个下游 JAR 合计约 13.7 MiB。[交接与计量](../../../.management-close-20261116/core-rtc-hotplug-20261120/triton-remote-maintenance/HANDOFF.md)

## 同一 agent 提交为何生成不同的六组件包

六组件和绑定网关所用的两个 bridge 有八份 class 和两份 OSGi 元数据不同，虽然其 Core loader 相同。直接复用绑定网关的 deb 会将六组件打包输入中的 bridge 从 11 月 20 日版换回 11 月 19 日版。本轮复用同一已配对 ARM64 二进制与源码清单，保留六组件原 bridge，用独立 Debian 版本发布 7.07 MB 的包。同一安装在解包后关机，继续 configure 并再次关机。新增持久卷为零。[原包比较及六组件源码对照](../../../.management-close-20261116/core-rtc-hotplug-20261120/six-agent-cohort/HANDOFF.md)

## 六个实装包与当前源码的差异

六组件 Core 由 d97260d 升至 e99caab 后，五个包直接匹配当前提交，Deployment 的后续提交仅改文档。复用了已有的 205.09 MB ARM64 包和同一原卷，停机前移除了临时上传副本。这次从实装而非版本标签比对六包 755 项，264 个 JAR 全部匹配；归档路径存在差异的 77 项中，61 项为安装后撤下的模板。实际发现的一项配置问题是没有以太网时板型仍为占位符。[逐项核对与运行结果](../../../.management-close-20261116/core-rtc-hotplug-20261120/six-core-cohort/HANDOFF.md)

## 2026-11-22 无以太网时的 Core 板型修复的磁盘实测

两个安装脚本更新后，新的 Core deb 为 205,087,792 字节，比前一版大 90 字节。其 233 个 JAR 全部复用原内容。两次独立 ARM64 安装顺序使用内存盘，退出后各留 4,582,826 字节 Docker 可写层，没有持久卷。本轮的主要新增大文件来自保留整份可安装 deb。该大小没有扣除宿主文件系统共享的块。证据在 core-board-install/release.json 和两个 parked.json。

## 2026-11-22 Container 编排依赖去重与正式 Main 升级

Container 编排 bundle 原先按根目录和 lib/ 两套布局各存一份依赖，OSGi 类路径只使用根目录。Maven 停止复制第二份，PDE 的根目录输入与 Maven 配齐。旧、新 deb 实测分别为 16,454,006、8,262,616 字节，每次留存新的完整发行件可少占 8,191,390 个文件字节。已有历史包保留。独立 ARM64 新旧包均由正式 Main 从实际引擎读到镜像，15 个类的所属 bundle 及摘要相同。数据见 container-bundle-layout/release.json。

## 2026-11-22 六组件原卷与 Core 发行件的增量

Core 构建输出当前保留 85 个 deb，文件逻辑大小合计 17.40 GB。六组件原卷 e99caab 与候选 89dfd6a 的 233 个 Core JAR 相同。逐项比较普通文件、链接和维护脚本，仅五个载荷文件和四个钩子内容有差异。两个版本各携带完整的约 205 MB 发行件。全量打包和多版本留存共同放大了测试存储成本。这个计量只覆盖 Core 输出目录，APFS 实际可回收空间需另测。

原 Root 的签名回执、维护号和密钥连续性需要保留。分析当前版本改动、匹配安装文件权限和验证账号可读性都可以直接复用归档和停放卷。本轮新增发行件与持久卷均为零。详见工作区 six-package-permissions 下的 audit.json 和 storage.json。

## 2026-11-22 跨停机验证两代快照的存储成本

本次重装复用了 205,112,358 字节的现成 Core deb。为在解包后正常停机、再沿原文件系统身份续配，需要保留同一安装卷。该 ARM64 卷初始为 61,256 KiB，保留两代快照和已安装的 Core 后为 286,076 KiB（279.37 MiB）。这是卷内 `du -skx` 的分配量。宿主 APFS 的物理增量未测。后续 provider 与 purge 可沿这一卷验证。[测量与配对依据](../../../.management-close-20261116/core-rtc-hotplug-20261120/published-generation-current-reinstall/final-check.json)

## 2026-11-22 两代快照卷完成 purge 后的占用

同一测试卷完成保留重装时，卷内分配量为 286,076 KiB。其后经过 provider、升级和最终清除验收，停机实测为 78,680 KiB（76.84 MiB），其中保留区私有压缩归档的文件大小为 125,276 字节。两个数值取自同卷不同停点的 du -skx，两次测量之间还执行过升级和业务验证。宿主 APFS 独占空间未测。[最终归档与卷内测量](../../../.management-close-20261116/core-rtc-hotplug-20261120/published-generation-final-purge/storage.json)
