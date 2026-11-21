# YOFC Kura 管理改造：设计、工期与磁盘复盘

2026-11-20 初稿，2026-11-21 更新。复核对象为会话《YOFC Kura 管理体系改造》、父工作区暂停交接、12 项冻结台账、实际八包安装与 Wires 重装检查点、两次清理回执和当前 Git 引用。

## 目前的判断

**有过渡设计，也有可以直接压缩的验证成本。** 旧 Java/Vue 中心与早期 Netplan 接管留下了代码和夹具。现行路径是 Rust/React 中心、可选常驻 agent、可独立运行的 Kura 与 NetworkManager。涉及系统权限的写操作由本机 broker 核对调用者、设备和原安装，再保存操作编号与回执。对已经派发的操作，这些记录决定能否安全续办。

11 月 21 日 09:19 起按同一口径复测，工作区 **346.43 GB**，其中历史工作目录 **252.32 GB**，中心 target **78.76 GB**。Core 打包目录有 36 个 ARM64 deb，文件长度合计 7.35 GB。较 07:06 增加 5 个包，共 1.02 GB；同期工作区目录计量增加 1.07 GB。已登录 HTTP 下载复用了安装包和测试卷。修复旧 PID 接管后另建了一个 204.24 MB 的 Core 包。宿主卷当时可用 141.79 GB，APFS 共享块和虚拟机卷分别计量。[目录与发行件复测](../../../.management-close-20261116/core-rtc-hotplug-20261120/main-pid-handover/storage.json)

11 月 21 日 07:06 起按工作区目录计量，工作区为 **345.36 GB**，较 05:08 增加 1.48 GB。历史工作目录占 251.25 GB，中心 target 单独复测为 78.76 GB，两者约占 95.6%。Core 打包目录现有 31 个 ARM64 deb，共 6.33 GB。较上次多出的 7 个包合计 1.43 GB。两种计量数值接近，APFS 共享块仍影响实际物理增长。本轮 Linux 编译容器为普通构建用户复制了 0.67 GB 缓存，源码与临时缓存已随容器回收，六组件卷另行保留。Docker 的销毁事件已核对。清理程序未及时保存可写层总量，因此无法量化此次容器实际释放的空间。[目录与发行包计量](../../../.management-close-20261116/core-rtc-hotplug-20261120/linux-arm64-core-osgi/storage.json) · [临时构建回收记录](../../../.management-close-20261116/core-rtc-hotplug-20261120/linux-arm64-core-osgi/builder-removed.json)

11 月 21 日 05:08 前后，工作区按 `du -k` 实测 **343.88 GB**，宿主可用 **147.51 GB**。其中历史工作目录 **249.77 GB**，中心仓的 `target` **78.76 GB**，两者合计约 **95.5%**。中心 `target` 中，网络权限与注册现场占 49.12 GB，上游逻辑同步目录占 17.73 GB。当时 Core 打包目录留有 24 个 ARM64 deb，文件长度合计 4.90 GB。APFS 可共享数据块，上述目录计量不能直接换算为可释放空间。[本次容量明细](../../../.management-close-20261116/core-rtc-hotplug-20261120/pkcs12-file-writer-identity/storage.json)

这次还定位到一种可以避免的重组装。API、Crypto 和 Configuration 三个 JAR 在运行时与冷恢复目录的内容逐项相同，只有清单中的 OSGi `Bundle-Version` 不同。例如 API 的构建后缀分别为 `202611202101` 和 `202611202040`。打包正确拒绝该组输入，从同一缓存重建恢复工具后通过。正式 `build-all.sh` 会为一次构建固定 `KURA_BUILD_QUALIFIER`。拆开运行 Maven 时，同一批候选也应显式传递相同的 `-Dkura.build.qualifier`，并让运行时和恢复工具使用同一批依赖。从当前 Git 只取 API 模块和构建父 POM，用同一个固定后缀连续执行两次干净打包，606 个文件条目以及整个 JAR 的 SHA-256 均相同。该验证复用了 Maven 缓存，临时源码和编译目录随即回收。[两次构建摘要](../../../.management-close-20261116/core-rtc-hotplug-20261120/pkcs12-file-writer-identity/qualifier-proof.json) · [实际差异](../../../.management-close-20261116/core-rtc-hotplug-20261120/pkcs12-file-writer-identity/unpaired-build.json) · [正式构建入口](../../../.management-close-20261116/core-rtc-hotplug-20261120/wip-core/build-all.sh)

## 工期花在哪里

最初的多网关管理中心已经扩展到网关操作系统、原 Kura 本地控制台、密钥保管、发行包维护及故障后的业务连续性。同一张 Timer→Logger 图能在首次安装时送达，只证明运行时可用。要证明客户升级可用，还需在原安装上升级 Core，确认原图只提交过一次；要证明 Wires 可独立拆装，又需经过 config-files、unpacked、installed，查看注册行并观察原图再次送达。后三项已有现场记录。其余组件及中断点不能借用这个结果。

历史方案确有过渡。旧中心使用 Java/Spring Boot、Vue 和 MQTT，新中心采用 Rust 和 React。早期 Netplan 接管原型在 11 月 5 日调整为保留 NetworkManager。两次变化都留下代码、包、旧状态和夹具。另有一类长期设计成本来自同一安装身份上的安全恢复。写操作派发后丢回执可能已经生效，产品需要识别原编号、原 Root 和维护围栏；旧版本与新版本都需要理解可恢复的中间状态。

再看 Git，从北京时间 11 月 16 日零时到本次交接 HEAD，沿提交历史排除 merge 后，中心仓有 267 次提交，242 次只修改 `docs/`，占 90.6%。同期 Core 有 185 次提交，其中 178 次涉及 `docs/` 外的路径。中心仓承担了跨仓验收台账，Core 则在持续改实现。这能解释为什么看中心提交数量会高估功能迭代次数，也能看出逐场景单独交接的维护成本。统计按提交计数，不能换算工时。

**工程判断**：按每个小改动建立完整发行目录和私有依赖副本，已经产生可量化的重复成本。独立低权限进程、限制写权限和保留未决操作各有实际业务理由。现有数据足以支持合并构建工作目录和缩小回归输入，尚不足以证明可以合并这些授权边界。工期没有逐项工时记录，不能可靠地计算返工比例。

单次完整测试也有可量出的等待成本。11 月 21 日的 JDK 21 Core/OSGi 构建从头跑完 82 个模块，用时 11 分 42 秒。仅证书吊销列表的 13 项集成测试就用了 424 秒，约占总时长的 60%；这些用例要观察下载、刷新和到期的时序。2128 项测试通过、2 项跳过。调整验证频率时，可以把这类整套时序验收与局部维护脚本的定向验证分别估时。[完整构建记录](../checkpoints/20261116/actual-rtc-hotplug-and-addon-upgrade.json)

这轮 Linux 全量构建还解释了重复验证中的一部分。一次从恢复模块开始的续跑，68 项 Equinox 测试全过；随后完整构建在异步注册组件时暴露了配置服务枚举 PID 的并发错误。先用一个确定性测试在旧实现上复现，再用原六组件卷中的正式 Main 并发枚举和注册。最终这次完整构建耗时 11:40 min，2151 项通过、2 项跳过。一个临时容器和同一份缓存支撑了各次构建。回归覆盖和重复存储可以分别安排。[完整构建及原卷验证](../checkpoints/20261116/actual-native-wires-retained-reinstall-checkpoint.json)

## 当前部署与权限

| 位置 | 当前职责 | 已验证的边界 |
| --- | --- | --- |
| 浏览器和中心 | React 经 HTTPS API/SSE 连接中心。Rust 中心保存任务、审计和结果；agent 主动建立双向 gRPC/mTLS 通道。方案 A 使用外部 CA。 | 中心、安装器和六个网关包已在一套 ARM64 夹具联合首装。多人 CA 保管、客户旧库及完整部署 GUI 仍待验。 |
| 网关本地 | Kura/OSGi 支持独立运行，保留 NetworkManager。可选 agent 常驻，远程开关控制中心通道。原 GWT 在安装 agent 后显示远程菜单。Core 生成私钥和 CSR。 | 原运行现场与新候选分开核验。当前六组件为 Core、Networking、GWT、Deployment、Wires、agent；加中心和安装器共八个 deb。 |
| 需要系统权限的操作 | Main 调用按操作类型限定的本机服务。peer 校验调用身份，actor 执行获准的系统动作，安装器校验成套代码和单元。 | 原六组件卷的正式 Main 已撤去 DAC_READ_SEARCH 和 DAC_OVERRIDE。Core b1b8060be2 两次启动均只保留 NET_BIND_SERVICE 和 NET_ADMIN。旧安装仍按各自版本核对。 |
| RTC 热插拔 | 草稿把启动时的设备投影改成请求时重验节点及别名，再向时钟 actor 传递只读 FD。 | 复盘时为草稿。报告送达后，已完成 ARM64 热插拔、只读 FD 和新包的两条原 Root 升级路径。随后在原 Parallels 组件完成正式 actor 经 relay 接收 FD 并写入虚拟 DS1307，实测回执 APPLIED。正式 Main 已取得设备缺失的 ENOENT 回执，并在 Core purge、重装后读回原记录。在原组件上，独占打开的 DS1307 使实际 relay 返回 EBUSY，Root 与发送端保存同一笔失败。当前 Core 安装器只读核对后拒绝此静态组件单元。随后在同一 Parallels 虚机的独立新安装，正式 Main 通过已注册的 ClockService 向专属 DS1307 成功写入一次，设备读数及两端回执均已核实。物理 RTC 仍待验。 |

从当前六组件停机安装逐文件核对了八组 broker 的发布清单，覆盖 26 个 systemd 单元。以 RTC 为例，读取设备的服务使用 root 身份，能力集为空，设备策略为 closed；时钟 actor 使用 kura-clock 身份，仅获 CAP_SYS_TIME，并启用私有设备目录。当时的 Main 以 kurad 运行，实测保留网络管理、绑定低端口和 DAC_OVERRIDE 三项能力。分开这两个时钟服务，使接触设备路径和调整时钟成为两个可以分别核验的权限。后续 b1b8060be2 已在同一安装撤去 DAC_OVERRIDE。[实装声明](../../../.management-close-20261116/core-rtc-hotplug-20261120/linux-arm64-core-osgi/typed-unit-declarations.json) · [正式 Main 进程实测](../../../.management-close-20261116/core-rtc-hotplug-20261120/linux-arm64-core-osgi/observed.json)

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

Deployment 的本地和云端入口各自维护同一份业务 DP 注册表。旧云端代码一次写入失败，就把 5963 字节的表截成 1024 字节，并向调用方返回成功。修复后两个入口编译同一份文件更新代码，各自随原 bundle 发行。相同限制下新版报告失败，原表完整。同一现场的两个 JVM 并发登记 60 项后，原有的 180 条无关记录仍可读。这轮复用原六包卷，仅新建约 0.22 MB 的 Deployment deb。随后发现注册行完整时，旧版仍可能在替换同名归档时先删旧文件。新版先完成同目录暂存与同步，再原子替换。两个实装入口在跨文件系统复制失败时均保住原文件，正式 Main 再次从原记录加载业务包。本次 Deployment deb 为 218,572 字节，复用了原卷和 Maven 缓存。Felix 安装、归档替换和注册表更新仍分步执行。[版本对照和实际 Main](../checkpoints/20261116/actual-native-wires-retained-reinstall-checkpoint.json)

H2 文件库提供了一个只有跨升级才看得清的风险。旧版遇到首次打开失败会删除数据库重试，凭据不匹配也可能丢失业务行。六组件的旧 Main 建库、写入一次业务记录，原地升级后的 Main 在拒绝错误凭据后恢复原凭据，再次读回同一行。整个场景复用 Wires 中断恢复的原容器和卷。[H2 实装与停机读回](../checkpoints/20261116/actual-native-wires-retained-reinstall-checkpoint.json)

这次验证启用中的 DNS 复用了原六组件容器、已有 Core 工作树和 Maven 缓存。修复后的 Core deb 为 204,224,080 字节，逐项核对了运行时和恢复工具中的两份 XML provider JAR。收尾从测试容器移除了已核对宿主原件的两个临时 deb，共 213,605,954 字节。[本轮安装与空间记录](../checkpoints/20261116/actual-enabled-dns-snapshot-upgrade-checkpoint.json)

SQLCipher 的现场还验证了两个包管理层如何共同恢复。在同一六组件安装上登记 SQLite 部署包后，共同清除了 Debian 的 Deployment 包和原生 SQLCipher 库。只解包 Deployment 时关机，其维护围栏阻止 Main 提前启动。原地恢复两包后，正式 Main 从原注册行加载部署包，读回原明文和加密业务行，两份数据库内容摘要均保持。此次复用了原部署包和两个 deb，驱动和 provider 由部署包交付。[联合维护记录](../checkpoints/20261116/actual-native-wires-retained-reinstall-checkpoint.json)

正式 Main 安装过两个版本的加密 SQLite 部署包。升级后注册表指向新版，卸载按当前登记地址删除了新版。冷卷上还留有旧版的 294,611 字节归档，SHA-256 与首次安装的发行件一致。DeploymentAgent 启动时遍历注册表，因此该旧文件不会仅凭位于 packages 目录就被重新安装。这解释了运行卷上的一处小额留存，完整构建目录和多版 deb 的占用要大得多。[旧归档和注册行](../../../.management-close-20261116/core-rtc-hotplug-20261120/sqlite-provider-distribution/archive-identities.json) · [启动和卸载代码](../../../.management-close-20261116/core-rtc-hotplug-20261120/kura-deployment/bundles/org.eclipse.kura.deployment.agent/src/main/java/org/eclipse/kura/deployment/agent/impl/DeploymentAgent.java)

## 磁盘增长的实测解释

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
