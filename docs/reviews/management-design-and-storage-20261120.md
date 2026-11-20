# YOFC Kura 管理改造：设计、工期与磁盘复盘

2026-11-20。复核对象为会话《YOFC Kura 管理体系改造》、父工作区暂停交接、12 项冻结台账、实际八包安装与 Wires 重装检查点、两次清理回执和当前 Git 引用。

## 工期花在哪里

最初的多网关管理中心已经扩展到网关操作系统、原 Kura 本地控制台、密钥保管、发行包维护及故障后的业务连续性。同一张 Timer→Logger 图能在首次安装时送达，只证明运行时可用。要证明客户升级可用，还需在原安装上升级 Core，确认原图只提交过一次；要证明 Wires 可独立拆装，又需经过 config-files、unpacked、installed，查看注册行并观察原图再次送达。后三项已有现场记录。其余组件及中断点不能借用这个结果。

历史方案确有过渡。旧中心使用 Java/Spring Boot、Vue 和 MQTT，新中心采用 Rust 和 React。早期 Netplan 接管原型在 11 月 5 日调整为保留 NetworkManager。两次变化都留下代码、包、旧状态和夹具。另有一类长期设计成本来自同一安装身份上的安全恢复。写操作派发后丢回执可能已经生效，产品需要识别原编号、原 Root 和维护围栏；旧版本与新版本都需要理解可恢复的中间状态。

再看 Git，从北京时间 11 月 16 日零时到本次交接 HEAD，沿提交历史排除 merge 后，中心仓有 267 次提交，242 次只修改 `docs/`，占 90.6%。同期 Core 有 185 次提交，其中 178 次涉及 `docs/` 外的路径。中心仓承担了跨仓验收台账，Core 则在持续改实现。这能解释为什么看中心提交数量会高估功能迭代次数，也能看出逐场景单独交接的维护成本。统计按提交计数，不能换算工时。

**工程判断**：按每个小改动建立完整发行目录和私有依赖副本，已经产生可量化的重复成本。独立低权限进程、限制写权限和保留未决操作各有实际业务理由。现有数据足以支持合并构建工作目录和缩小回归输入，尚不足以证明可以合并这些授权边界。工期没有逐项工时记录，不能可靠地计算返工比例。

单次完整测试也有可量出的等待成本。11 月 21 日的 JDK 21 Core/OSGi 构建从头跑完 82 个模块，用时 11 分 42 秒。仅证书吊销列表的 13 项集成测试就用了 424 秒，约占总时长的 60%；这些用例要观察下载、刷新和到期的时序。2128 项测试通过、2 项跳过。调整验证频率时，可以把这类整套时序验收与局部维护脚本的定向验证分别估时。[完整构建记录](../checkpoints/20261116/actual-rtc-hotplug-and-addon-upgrade.json)

## 当前部署与权限

| 位置 | 当前职责 | 已验证的边界 |
| --- | --- | --- |
| 浏览器和中心 | React 经 HTTPS API/SSE 连接中心。Rust 中心保存任务、审计和结果；agent 主动建立双向 gRPC/mTLS 通道。方案 A 使用外部 CA。 | 中心、安装器和六个网关包已在一套 ARM64 夹具联合首装。多人 CA 保管、客户旧库及完整部署 GUI 仍待验。 |
| 网关本地 | Kura/OSGi 支持独立运行，保留 NetworkManager。可选 agent 常驻，远程开关控制中心通道。原 GWT 在安装 agent 后显示远程菜单。Core 生成私钥和 CSR。 | 原运行现场与新候选分开核验。当前六组件为 Core、Networking、GWT、Deployment、Wires、agent；加中心和安装器共八个 deb。 |
| 需要系统权限的操作 | Main 调用按操作类型限定的本机服务。peer 校验调用身份，actor 执行获准的系统动作，安装器校验成套代码和单元。 | 六组件夹具的正式 Main 已撤下 DAC_READ_SEARCH。上次 Wires 验收读到 DAC_OVERRIDE、NET_BIND_SERVICE、NET_ADMIN。旧现场有自己的权限集。 |
| RTC 热插拔 | 草稿把启动时的设备投影改成请求时重验节点及别名，再向时钟 actor 传递只读 FD。 | 复盘时为草稿。报告送达后，已完成 ARM64 热插拔、只读 FD 和新包的两条原 Root 升级路径。随后在原 Parallels 组件完成正式 actor 经 relay 接收 FD 并写入虚拟 DS1307，实测回执 APPLIED。正式 Main 已取得设备缺失的 ENOENT 回执，并在 Core purge、重装后读回原记录。完整 Main 正向写入虚拟 DS1307 和物理 RTC 仍待验。 |

### 六个网关包已经验证到哪一步

截至 11 月 21 日，首装和组件能启动已有联合现场。下面按实际业务区分维护证据，便于续接时直接选取尚缺的模式或中断点。每行只覆盖所列现场，冻结父项仍为 0/12。

| 网关包 | 已取得的现场证据 | 仍需区分的条件 |
| --- | --- | --- |
| Core | 正式 Main 取得缺失 RTC 的 ENOENT，Core 移除、purge、重装后原回执和密钥仍在。[时钟实装记录](../checkpoints/20261116/actual-registered-main-clock-and-purge-checkpoint.json)。三类网络发送端在派发前保存的原编号已通过正式 Core purge、重装及 Java 读回。[网络发送端记录](../checkpoints/20261116/actual-core-network-sender-purge-checkpoint.json) | 完整 Main 对虚拟 DS1307 的成功写入。 |
| Networking | 启用 DNS 的六组件现场经过 Core 升级及 Networking 移除、purge、重装。正式 Main 恢复原模型，IPv6 客户端收到合成 IPv4 上游的回答。[启用 DNS 实装记录](../checkpoints/20261116/actual-enabled-dns-snapshot-upgrade-checkpoint.json)。DHCP 停用丢回执后，经 Networking purge、配置中断及重装，正式 Main 按原编号取回结果。[DHCP 实装记录](../checkpoints/20261116/actual-registered-main-dhcp-maintenance-checkpoint.json) | 本次网络设备由夹具准备。真实 NM/cloud-init 和物理硬件交接仍归原延期项。 |
| GWT | 单独移除后观察到 unpacked、half-configured，原地续配成功，随后 purge；核对了自定义图标和 Wires 文件。[中断回滚记录](../checkpoints/20261116/actual-passive-addon-dpkg-rollback-checkpoint.json) | 维护脚本通过后，仍需对照各项实际页面流程单独验收。 |
| Deployment | FileMove 发布后丢回执，系统服务管理器重启后仍按原记录完成源端收尾。[FileMove 续接记录](../checkpoints/20261116/actual-deployment-filemove-manager-reboot-checkpoint.json) | 这次是服务管理器重启，Linux boot ID 保持。 |
| Wires | 保留式移除和分阶段重装后，正式 Main 读回原 Timer→Logger 图，Logger 再次收到消息，原图仅提交一次。[原图重装记录](../checkpoints/20261116/actual-native-wires-retained-reinstall-checkpoint.json) | 联合配置中断有另一套现场及原 Root。[六组件转换记录](../checkpoints/20261116/actual-standalone-position-six-conversion-checkpoint.json) |
| Agent | 同 Root 实际经过移除、purge、解包及重装，另有已绑定和未绑定场景。[保留式维护记录](../checkpoints/20261116/actual-native-agent-retained-removal-checkpoint.json) | 续接前要确认原注册身份、业务记录和 launcher 模式。 |

这次验证启用中的 DNS 复用了原六组件容器、已有 Core 工作树和 Maven 缓存。修复后的 Core deb 为 204,224,080 字节，逐项核对了运行时和恢复工具中的两份 XML provider JAR。收尾从测试容器移除了已核对宿主原件的两个临时 deb，共 213,605,954 字节。[本轮安装与空间记录](../checkpoints/20261116/actual-enabled-dns-snapshot-upgrade-checkpoint.json)

## 磁盘增长的实测解释

以下采用十进制 GB。11 月 16 日清理时工作区从 483.49 GB 降至 96.39 GB。11 月 20 日上次会话中途量到 491.68 GB，清理完成时为 335.18 GB，宿主可用空间净增 154.45 GB。两次读数之间有新构建，APFS 共享块也会影响目录计量，不能把两次节省量相加。

| 11 月 20 日清理后的目录 | 占用 | 对工期和保全的意义 |
| --- | ---: | --- |
| 工作区 `.management-close-20261116` | 241.07 GB | 各阶段的源码副本、私有依赖、发行包、故障现场及回执混放。其中单个 source-cohort 目录约 27.07 GB。 |
| 中心仓 `target` | 78.76 GB | 包含跨阶段的现场引用、保全基线与构建结果。整目录删除会破坏续接。 |
| 两者合计 | 319.83 GB | 约占工作区 95.4%。这是开发与验收的目录占用，不能用来推算客户网关的安装容量。 |

清理实际辨认出 266 份重复组装输出、134,424 个字节相同的 Maven 文件、20,787 个编译输出目录，以及 271 个同摘要缓存归档。Docker 曾积累 270 个容器，清掉经核定的 181 个后，可写层从 102.3 GB 降至 39.84 GB。Docker VM 与宿主目录使用不同计量口径。以上数字来自保留的清理回执，本轮未执行清理。

## 本次续接采用的简化

同一版本只建立一份工作检出，RTC 先跑脚本与 unit 定向回归，需要验证包实际安装时再组装当前版本。对必须安装的候选保留 Git tree、包摘要、成套文件清单和原始维护回执。涉及原 Root 的业务沿原现场继续；可重建的编译输出单独放置。这项选择削减构建副本，保留已冻结的 12 项验收范围。

11 月 20 日复用 Core 构建目录时，第二次打包因旧的时钟配对清单已存在而退出。将八个生成目录及已过滤的 Debian 控制脚本移到暂存位置（合计约 1.46 MiB）后，同一源码检出和原 Maven 缓存成功产出 204.21 MB 的 ARM64 deb。这个例子明确区分了必须重建的包输入与可以复用的源码和依赖。

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
