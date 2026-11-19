
## 2026-11-20 BIND 文件已发布且无回执的原编号核定

Core 1ae9a34b3e 在原意图记录真实已 fsync 且通过 named-checkconf 的暂存文件身份。文件发布采用同一原 inode。受控核定先核 Root 和原 ID，只采信同一次 Linux 启动、原 named invocation、原暂存 inode 和发布摘要均匹配的文件操作。核定同步父目录并单独生成受保护的回执，确认仍需原 ID。启停和激活如果缺少完成证据，继续保留原 UNKNOWN 围栏。

Debian 13 ARM64 从 Git 构建并实装。在专用隔离容器以只用于验收的 Linux ptrace 证实实际 actor 的 rename 成功返回，用 PIDFD 中断该进程。真实 Main 的 Java 调用报告 UNKNOWN，此时配置已换成原意图指向的 inode，原结果文件缺失。继任 Main 在同次 Linux 启动、同 Root 上先只读查询 UNKNOWN，再核定并明确确认原编号。仅增加回执和确认文件，未重新发布配置，named PID 和 invocation 保持，原三份保护文件和发布配置的 inode、字节及元数据精确保持。现场禁用 Main 并正常停放。[Linux ptrace syscall-info 接口](https://github.com/torvalds/linux/blob/master/include/uapi/linux/ptrace.h)

Linux Core 121 项、Networking b3af558 Maven 2211 项通过。实际现场抑制 NM 后台重配，生产包权限未加入 tracer。其他 BIND 未决服务阶段、Root 重挂、旧配置合法纳管、NM 实际后台触发仍内部待办。其余 brokers、完整 Core/OSGi 和六组件/provider/launcher/安装升级移除 purge 与全部中断恢复矩阵保持。原 7fd 保持 41 输入/16 回执，冻结父项最终闭环 0/12，external deferred 仅原三项。见[检查点](actual-core-bind-publication-settlement-checkpoint.json)。



## 2026-11-20 BIND 已提交丢回复与同 Root 继任 Main 核定

Core c41514a055 让写请求提交前绑定当前 Root scope。提交后原结果一旦缺确认即保留后续写围栏。只读查询沿原 ID，先核 Root，再核对意图与回执。Networking b3af558 在原 JVM 确认收到结果后使用同一 ID 确认。Root 记录验证后单独持久化确认。兼容以前已闭环的旧回执，原写请求走独立派发入口。

Debian 13 ARM64 从当前 Git 以 JDK 21 实装。以 PIDFD 在专用现场暂停 named，真实 systemd 已受理原重启任务后终止一次实际 Main 客户端，随后释放 named。原 Java 调用报告 UNKNOWN，系统完成已受理的单次操作并保留一个 APPLIED 回执。继任 Main 同 Linux 启动和同 Root，只读核定原编号。随后以不同动作的独立 stop 验证围栏，未派发 job、未创建该动作意图。明确确认原编号时只新增一份确认文件，原四份文件的字节和元数据、named PID 和 invocation 均精确保持。现场禁用 Main 并正常停放。Linux Core 118 项，Networking Maven 2211 项通过。

实际验收抑制 NM 后台重配。未落终态的系统中断、Root 重挂后合法转换、已装历史配置纳管及完整 NM 触发仍内部待办。其余 brokers、完整 Core/OSGi、六组件/provider/launcher/全部发行与中断恢复矩阵保持原范围。原 7fd 保持 41 输入/16 回执，冻结父项最终闭环 0/12，external deferred 仅原三项。见[检查点](actual-core-bind-outcome-recovery-checkpoint.json)。



## 2026-11-20 BIND 类型化配置发布与实际 DNS 验证

Core 0b4f42576f 在 Main 将 /etc/bind 挂为只读，通过配对 actor 接受 IPv4 转发器、CIDR 与启用状态。先核对发行包原配置或同 Root 上次已提交字节，再由实际 named-checkconf 验证固定模板。在写意图后原子发布并核对启用状态，任一不确定的变更保留原编号。站点自改主配置或 options 在变更前拒绝。禁用时恢复原发行包字节。保留站点 zones 等 include。Networking 8c522a2 的 NM 监视器调用同一 OSGi 配置入口，由发行证明约束对应 JAR。[ISC BIND 配置参考](https://bind9.readthedocs.io/en/v9.20.15/reference.html)

Debian 13 ARM64 实装当前 Git 以 JDK 21 构建。合法 Main 内的已装 OSGi provider 实际发布并激活，新的 named invocation 转发允许源 127.0.0.1 的查询并返回测试地址 192.0.2.42，拒绝另一源 127.0.0.3。随后实际禁用，原 named.conf 字节恢复，四份 include 或启动输入保持，两个原 ID 均留同 Root 回执。现场抑制 NM 后台重配以单独验证 provider，正常停放并移出专用探针。Linux Core 116 项与 Networking Maven 2211 项通过。

历史 Java 生成及站点自改 BIND 配置的显式纳管、丢回复与跨 Main 核定、Root 重挂恢复及实际 NM 后台触发仍为内部待办。其余 brokers、完整 Core/OSGi 和六组件/provider/launcher 生命周期矩阵保持原范围。原 7fd 保持 41 输入/16 回执，12 父项最终闭环 0/12，external deferred 仅原三项。见[检查点](actual-core-bind-configuration-checkpoint.json)。



## 2026-11-20 BIND 直接 polkit 授权配对撤回

Networking b54b360 在发行包发布已构建的 DNS provider 指纹。Core d0e9774b13 在 Debian 换版和 Main 启动前、在原 modem 配对基础上核对 BIND provider 的包所有权和 JAR 指纹，撤下 kurad 对 named/bind9 的直接 systemd 操作授权。Main 将 polkit 配置挂为只读。原 polkit 文件按指纹归档，Chrony 专用账号与 unit 的权限合同保持。

原 BIND 验收夹具重挂时文件系统设备号与记录不同，原受保护文件和三份回执字节匹配，按原 Root 校验边界停放，未向它派发包操作或 BIND 操作。随后从 Git 构建前后版本，在新的隔离 Debian 13 ARM64 Root 实装。实际 dpkg 先拒绝缺少 BIND 发行证明的旧 Networking，依序升级 Networking 与 Core 后，真实 polkit 对同一 kurad 身份、两个 unit 名、三种操作的六项授权由允许变为拒绝。合法 Main 内实际 OSGi 只读状态仍通过 broker。系统 named 保持同一个 PID 和 invocation，未派发 BIND 变更，两份原凭据和空台账保持，夹具正常停放。

Linux Core 110 项、polkit 合同 14 项、Networking Maven 2211 项通过。Core 1dedc117f0 单独补齐先前 Chrony 配置激活已有的 restart 规则测试。测试现场抑制了 NM 后台重配。BIND 配置发布、真实丢回复、跨 Main 核定及 Root 设备改号后安全恢复仍内部待办。其余 typed brokers、完整 Core/OSGi 和六组件/provider/launcher/生命周期矩阵继续实施。原 7fd 保持 41 输入/16 回执，父项 0/12，external deferred 仍仅原三项。见[检查点](actual-core-bind-policy-cutover-checkpoint.json)。



## 2026-11-19 实际 OSGi BIND 进程委派

Core 05ad0dbf35 安装 BIND socket actor，核验固定 Main、Networking drop-in、直接子进程及 peer pidfd。actor 本身无 Linux capabilities，仅接受固定 named.service 的状态与启停请求，校验已装 Debian BIND unit 和二进制。变更前同步同 Root 意图，结果不明确时保留原编号并阻断后续写。Main 将系统代码、unit 目录和 dpkg 元数据挂为只读，独立 BIND 台账挂为不可访问。Networking 1809304 从正式 OSGi DnsServerService 接到此路径，运行中结果不明确时保留原 ID。

Debian 13 ARM64 正式实装后，同一合法 Main 内的业务完成停止、启动和重启，各有一份配对回执。启动和重启后的本地 UDP version.bind 查询成功。停止时的 MainPID=0、新启动及重启的不同 invocation、BIND 配置原字节、Main 名字空间实际只读和不可访问路径均已核对。同 UID 外部进程被拒且无回执。该隔离现场用现有持久所有权围栏抑制 NM 后台重配，实际业务由已装 OSGi 服务发起。Main 正常停止、禁用，原探针保留并移出加载目录，三份回执保留。Linux Core 109 项与 Networking Maven 2211 项通过。

当前保留旧包 BIND polkit 兼容授权。跨包配对撤权、实际丢回复与跨 Main 恢复及 BIND 配置发布仍内部待办。已定位 DHCP/dnsmasq、PPP 和 Wi-Fi 等余下进程管理路径，保留完整 Core/OSGi 和六组件/provider/launcher 发行与中断恢复矩阵。原 7fd 41 输入/16 回执保持，父项 0/12，external deferred 仅原三项。见[检查点](actual-core-typed-bind-service-checkpoint.json)。



## 2026-11-19 Main 撤下 CAP_SYS_TTY_CONFIG，实际 OSGi 串口验证

Core a47c41ba97 从 Main 的 ambient 和 bounding 集合撤下 CAP_SYS_TTY_CONFIG。串口仍由安装器登记的 dialout 组与 Core CommConnectionFactory 提供。此变更依据实际调用及 [Linux 虚拟终端 ioctl](https://github.com/torvalds/linux/blob/master/drivers/tty/vt/vt_ioctl.c) 与 [jSerialComm 2.11.4 POSIX 实现](https://github.com/Fazecast/jSerialComm/blob/v2.11.4/src/main/c/Posix/SerialPort_Posix.c)。

JDK 21 从当前 Git 构建，Debian 13 ARM64 隔离 Root 正式 dpkg。容器 PID 1 的 bounding set 保有该位，标准 Equinox Java Main 的五类 capability 掩码均无该位。实际 Main 中的 OSGi factory 使用随包的 jSerialComm 2.11.4，真实打开 root:dialout 0660 伪终端。读回 38400 8N1，逐字节校验双向各八字节，包括 0x00。另用无有效 capability 的 kurad 进程确认组权限足以打开同一设备。现场停放，Main 禁用，探针原字节移出自动加载目录。Linux Core 104 项通过。

伪终端结果覆盖 Core 串口配置与流 I/O。物理串口、RS-485 与实际 USB 设备继续沿用原硬件边界。其余 typed brokers、完整 Core/OSGi、六组件/provider/launcher 和发行中断恢复矩阵继续内部待办。原 7fd 保持 41 输入/16 回执，冻结父项 0/12，external deferred 仅原三项。见[检查点](actual-core-serial-main-capability-checkpoint.json)。



## 2026-11-19 Main 撤下 CAP_SYS_MODULE

Core 1ea5e52b9d 在 Debian 包换版前和 systemd Main 启动前核验 provider 及其发行指纹。Networking fbcebfc 的标准和 firewall-only 包均核验已装 Core modem broker，发布实际 OSGi JAR 指纹。无 Networking 的 Core 可独立启动。Main 的 AmbientCapabilities 和 CapabilityBoundingSet 均撤下 CAP_SYS_MODULE。

在上一轮实际 modem 的同一 Debian 13 ARM64 隔离 Root，先验证带 broker 的上一版 Networking 缺少发行证明时，候选 Core 的 dpkg preinst 拒绝换版。随后依序实装有证明的 Networking 与 Core，启动标准 Equinox Java Main。实际 JVM 的五类 capability 掩码均无 CAP_SYS_MODULE，modem socket 正常监听。同 Root 的三份历史业务文件、锁文件和两份凭据精确保持。两份夹具业务输入与输出在 Core 升级后换了 inode，字节保持。上一轮夹具 bundle 原字节移出自动加载目录，旧业务未重放。现场重新禁用 Main，正常停放。

另一套 Debian 13 ARM64 隔离现场成功启动独立 Core，随后正式安装并启动 firewall-only 配对 Main，两次实际 Java Main 均无 CAP_SYS_MODULE。Core 凭据保持，现场正常停放。Linux Core 99 项与 Networking Maven 2211 项通过。

历史 schema 1 各合法转换沿用已验收证据。实际内核模块装卸和动态注册仍按原硬件边界处理。安装中断及配对恢复、其余 typed brokers、完整 Core/OSGi 和六组件/provider/launcher/安装升级移除 purge 矩阵继续内部待办。原 7fd 保持 41 输入/16 回执，父项 0/12，external deferred 仅原三项。见[检查点](actual-core-modem-capability-cutover-checkpoint.json)。



## 2026-11-19 USB modem 类型化委派与实际 Main 准入

Networking 8da8a33 将模组装卸及 Option 动态注册接到 Core 的本机 broker。Core 核验有效 Main、固定 Networking drop-in、直接子进程和内核 pidfd。模块范围为 cdc_acm、sierra 与 option，配对 USB ID 来自当前支持表。加载前实查接入设备；卸载仅接管有原收据且仍属于原内核实例的模块。内核写入前同步原意图，未结结果阻止同一模块换号派发。通用 Java executor 与直接 new_id 写入已移出该业务路径。

独立 Debian 13 ARM64 正式安装 Core e879266d5a、Networking 8da8a33 及带签名索引校验的 Debian kmod。实际 Main 内已装 OSGi 调用得到未接入拒绝和模块已不存在的无操作成功，后者保留原编号的 Root 收据。同 UID 外部进程被拒，实际内核变更为 0。缺少实物的条件只用于拒绝及无操作结果的实际验收。实际正向加载、动态注册、卸载需原硬件验收。

真实 kmod 发现必须按 argv[0] 选择 modprobe 与 rmmod 模式。Core 73ebb0d0b3 修正固定二进制调用并延长客户端等待上限，真实 kmod 的 modprobe dry-run 与 rmmod help 均识别兼容模式。随后同一隔离 Root 正常 Core 升级，原三份操作文件、原两份 Root 凭据及原 Networking JAR 精确保持，已核原 Root 和挂载配对。Main 保持 disabled，升级后未再次派发旧业务。现场正常停放。Linux Core 95 项，Networking Maven 2211 项通过。

Main 的 CAP_SYS_MODULE 仍为旧 Networking 升级交接保留。该权限撤回、合法历史和中断恢复、全 provider/launcher、完整 Core/OSGi、其余 broker 和完整六组件矩阵仍内部 pending。原 7fd 41 输入/16 回执和原 UNKNOWN 保持。父项 0/12，external deferred 仅原三项。见[检查点](actual-core-typed-modem-driver-checkpoint.json)。



## 2026-11-19 Main 撤下 CAP_SYSLOG 与实际大日志归档下载

Core 7b9bb19324 从正式 Main 单元的 ambient 和 bounding 配置撤下 CAP_SYSLOG。独立 Journal worker 的只读权限和代码配对保持。同一隔离 Debian 13 ARM64 Root 正常升级 Core/GWT，原两份 Root 凭据完整指纹一致，Main 保持 disabled 且由显式启动。实际已装单元、systemd 有效配置及运行进程的五类 capability 位图均核对。旧能力在旧 vendor 单元中有声明，本轮记录的是当前运行位图。

GWT 08712fb 在每个请求自身的私有目录先完成 ZIP，再分块写响应。生成失败在下载标头和完成 cookie 之前返回 503。正常响应发送准确的 Content-Length，请求结束清理暂存。root scope 仍由原 Journal broker 和实际 Main 配对。受限 JVM 测试在 32 MiB 堆下压缩不可压缩的 40 MiB 文件；实际 GWT 管理员另下载 48 MiB 文件和两份 Journal，ZIP 完整且输入 SHA-256 不变。真实响应 Content-Length 与 ZIP 文件大小一致。两个 worker 实测非 Root、五类 capabilities 为零、NNP 1。Main 正常停止后 socket 已撤下，同一隔离现场已重新停放。

Linux ARM64 75 项包装回归、GWT Maven 12 项通过。前一轮的旧 watchdog 测试单独修正，按正式 helper 验证回调返回及 check/disarm。模拟的归档生成失败通过测试；真实磁盘写满继续在原外部边界内。原三个现场、原 7fd 的 41 输入/16 回执、原 UNKNOWN 和已闭环业务保持。其余 Main 权限、typed brokers、provider/launcher、完整 Core/OSGi 与六组件矩阵内部 pending。12 父项关闭 0/12，external deferred 仅原三项。见[检查点](actual-core-journal-bounded-archive-checkpoint.json)。



## 2026-11-19 Core/GWT typed Journal 下载与实际 Main 准入

Core 54fc736fee 与 GWT 862eaa0 通过同机 Unix socket 提供固定的 Kura 与 system journal 查询。GWT 保留原设备权限和 XSRF 检查，有界调用固定客户端，使用每次请求独立暂存并在归档后清理。Broker 核对已装 SHA 清单、实际 vendor 单元、Main PID/cgroup、直接子进程参数与内核 pidfd；仅接收已定义日志范围和字段名。读取每项最多 32 MiB；异常结果不会作为成功条目归档。

实际 Core 安装器会清理模板暂存目录，留存模板已移至正式配对目录。隔离 Debian 13 上动态 UID 的消息总线查询被拒，实装按正常升级改为专用静态系统账号。打包的 worker 源由独立 Root 目录提供，运行时隐藏 Core 与密钥目录，只给 systemd-journal 附加组。最终 Core 原位升级保存两个原 Root 凭据的完整指纹，Main 无自动启动。版本配对安装后显式启动实际 Main，socket 自动就绪。

同 UID 非 Main 进程被实际拒绝，返回零日志字节。GWT 管理员从正式 System Logs 入口发起一次下载，HTTP 200，ZIP 含 22 项。两份 journal 经实际 worker 输出且各自作用范围已核对。普通日志文件保持原字节，请求暂存清理。观测到的三个处理进程全部为专用非 Root UID、五类 capabilities 为零、NNP 1。正常停止 Main 时 socket 撤下，现场 disabled 并正常停放。

macOS 和 Linux ARM64 各 5 项定向合同，GWT Maven 10 项通过。Linux 74 项相关包装选择中一项旧 watchdog 夹具在两种成功分支失败，基线 7d4789fe1d 已复现相同失败。继续收敛 Main 权限、其余日志 provider/launcher、完整 Core/OSGi、typed brokers 与全六组件矩阵。原 7fd 保持 41 输入/16 回执。12 父项关闭 0/12，external deferred 仅原三项。见[检查点](actual-core-typed-journal-export-checkpoint.json)。



## 2026-11-19 已绑定 Agent 同 Root 后继发行重装及真实设备号变化

Agent `f956952` 扩展原后继重装，沿初始化和本次移除原编号核对旧 Kura peer、材料授权、原安装/来源链及新版 archive。新版 Rust 检查器在相同非 Root UID 下认证暂存 DB/WAL 副本，并逐项比较旧 inspector 的结果。configure 只发布已审核的原生、Kura peer、材料公开及私有中继代码绑定。原 grant、角色、权限、Core/Networking 配对、保留任务和 OFF 状态继续严格核对。源码回归覆盖私有中继发布后中断，沿唯一初始事务续接。

上一轮隔离 Debian 13 ARM64 的原绑定和一次成功的 `kura.snapshot.list` 延续到新的升级目的。已完成的原移除回执保持。实际新版经单次 remove、unpack、configure 安装。remove 后同一 OverlayFS backing 重新挂载，设备号从 1048815 变为 1048827，94 项原输入保持。解包状态继续拒绝启动。configure 发布 4 份经准入的代码绑定，90 份其他原输入完全保持，含停机 SQLite/WAL 和原任务。原编号回执为 REINSTALLED，未自动启动。

显式启动后核对真实运行的非 Root Rust 指纹、零 capabilities、NNP 1、Unix socket，及 Main 的 HTTPS 监听。Agent 保留原任务 ID、操作和终态，PostgreSQL 保留原 ID 及原输入和结果密文指纹。三处现场正常停放，中心容器保持原停放状态。Linux ARM64 提交版本 174 项通过，33 项定向回归已包括上述中断和业务漂移拒绝。其余绑定代、已有非空来源历史的实际发行、其余中断时点、全部 provider/launcher 及完整六组件矩阵继续内部 pending。保留原 7fd 41 输入/16 回执；12 父项 0/12，external deferred 仅原三项。见[检查点](actual-native-agent-bound-successor-checkpoint.json)。



## 2026-11-19 已绑定 Agent 保留加密任务和 WAL 的发行生命周期

Agent 2017a39 加入 Root `prepare-retained-enrolled-removal`。原 Kura peer、材料授权、已绑定凭据及控制通道均经现有实现校验，实际非 Root Rust inspector 认证同字节 SQLite 及 WAL 的副本。运行或 UNKNOWN 任务在写入维护意图前拒绝。准入后，包文件缺席时按原 Core loader/有效单元、原角色、来源链及逐项快照核对，同时保留跨重挂绑定。允许同版本、同发行包原位重装。已绑定后继版本还需联合 Kura 和材料来源转换。

专用 Debian 13 ARM64 网关经真实 GWT 首次绑定并由独立 CA 签发。开启 mTLS 后只提交一次 `kura.snapshot.list`，同 ID 为 SUCCEEDED，中心任务输入、结果均保留密文。显式关闭通道后停止并禁用 Main、Agent 和中继，正常停机后的 SQLite WAL 仍非空。全程使用原数据库和 WAL，独立读取仅在临时副本进行。实际 Root 保存 94 项，包含 Core 原 CSR/证书材料、Agent 原绑定、授权及 SQLite/WAL；Rust 回执确认一个 terminal 任务及零未决记录。

正式 dpkg 各一次 remove、purge、unpack、configure 均成功。remove 后同网关重挂，内核复用同一设备号，本轮证据覆盖真实重挂及 94 项严格指纹保持。unpack 后仍拒绝启动，configure 后原回执 REINSTALLED。显式启动后实际非 Root Agent 零 capabilities，GWT 只读核对同一绑定回执和原 OFF，中心密文及本地任务原 ID 保持。真实 purge 未出现旧缓存目录遗留告警，重装后的程序目录无 Python bytecode 缓存。三处新隔离现场正常停放。

Linux ARM64 原生权限和发行回归 172 项通过。其余绑定代、已绑定后继版本、绑定现场设备号变化、未决业务和完整六组件矩阵保持内部待办。原三现场、原 7fd 41 输入/16 回执和原 UNKNOWN 保持。冻结 12 父项仍 0/12，external deferred 仅原三项。见 [检查点](actual-native-agent-bound-business-removal-checkpoint.json)。



## 2026-11-19 Agent 同 Root 后继重装及两次 OverlayFS 重挂

Agent b8a84db 为新移除意图保存内核 OverlayFS 底层指纹。参与的状态必须处于同一原 overlay，拒绝已验证路径的独立挂载。设备号可随相同 backing 重挂变化，其余逐文件指纹保持严格比对。858628a 以原维护编号审核版本严格更高的同 Java/Core bridge 后继包，发布前核对资产、角色、原初始化与来源链。来源链记录绑定原移除和审核摘要。回归覆盖公开代码身份已发布但来源指针未发布时的原编号 configure 续接。

首个原生现场的移除、同 overlay 重挂和后继 unpack 通过。第二次重挂实际改变设备号，早期初始化证明曾固定旧设备号，configure 退出 1。保留原九项、原准入和 `install ok half-configured`，来源链尚无发布，正常停放。9760bb2 使新初始化回执记录 overlay 证明。Python Root 检查、Rust 实际启动与保留状态检查使用相同 backing 绑定，保留原数据库/密钥 inode、计划密钥摘要和原初始化拒绝条件。历史无证明回执继续使用设备号严格比对。97e8eb7 直接编译自身配对的安装控制器源文件，部署验证中隔离 Root 助手没有生成 Python bytecode 缓存。

独立 Debian 13 ARM64 实际将 9760bb2 原包移除，原 Root 同一底层 overlay 重挂后审核 97e8eb7 后继，再在 unpack 状态进行第二次重挂。三个设备号真实不同，overlay 摘要保持，七项停机状态的其余原字段及三个 Core 文件一致。首份同分钟构建的后继版本按 Debian 排序早于原包，写意图前拒绝。按秒构建排序正确的同一 Git 版本，沿原初始化和原移除意图继续。正式 remove、unpack、configure 各一次 exit 0，原移除意图/回执摘要不变。最终 dpkg installed，状态 REINSTALLED 和同编号来源链 APPLIED，仅批准公开代码身份推进，六份其余输入精确。显式启动实际非 Root Rust Agent，零 capabilities、NNP 1、本地 Unix socket 就绪，原初始化、连续性文件和计划密钥保持。Agent 此后正常停机，Core 保持 inactive，两个专用现场均正常停放。

Linux ARM64 Python 170 项及 Rust 状态连续性 1 项通过。有历史无证明的设备重挂、已绑定/业务状态、其他 provider/launcher 及故障时点仍内部待办。保留原现场、原 7fd 41 输入/16 回执及全部冻结 12 父项。最终 0/12，external deferred 仅原三项。见 [检查点](actual-native-agent-successor-overlay-reinstall-checkpoint.json)。



## 2026-11-19 已准入的 Agent postrm 原编号续接

Agent 943a27f 增加 Root `prepare-retained-postrm-recovery`，只接受已准入、负载已不存在、dpkg 为 `deinstall ok half-installed` 的原请求。原初始化、状态完整指纹、版本、Java/Core bridge、原 `postrm` 和被审核归档均核对。旧脚本先不可变保存，新脚本与原意图、原/新两份脚本摘要绑定。准备只更新该 maintainer 文件，dpkg 状态仍保持 half-installed。实际选择 Linux ARM64 定向 48 项通过，包含原设备号变化拒绝、不同脚本拒绝及原回执续接。扩展原生权限和状态回归 167 项通过，两组选择重叠。

原 9ccb 现场只读重挂后，15 个原条目仅设备号变化，原 `postrm` 摘要、原准入记录、Core 三文件和停机状态保持。历史意图没有跨重挂证明，当前严格配对仍拒绝这类变化。原现场原编号保持半安装并已正常停放，未派发 dpkg 或更改 maintainer。

在独立 Debian 13 ARM64 测试现场，4b6271a 首次真实 remove 在 postrm 退出 1，包负载已不存在，保留原准入和七份文件。保留同一 Boot、原 Root、原维护 UUID。用当前 Git 与当前协议构建的正式 943a27f ARM64 deb 提供经过核验的修复脚本。实际 dpkg 续接的进程跟踪只有 `postrm remove`，未再执行 prerm。退出 0，包状态变为 `deinstall ok config-files`，原维护回执 REMOVED。原准入文件的 inode、元数据和摘要、七份 Agent 数据及三份 Core 文件保持，现场正常停放。

较早包的后继发行重装、历史意图跨重挂核定、Python bytecode 残留、已绑定或有业务状态及其他中断阶段仍为内部待办。原 7fd 41 输入/16 回执和原 UNKNOWN 保持，完整 Core/OSGi、typed brokers、六组件矩阵及冻结 12 父项继续原范围。父项 0/12，external deferred 仍仅原三项。见 [检查点](actual-native-agent-postrm-only-recovery-checkpoint.json)。



## 2026-11-19 Agent 已初始化状态的同 Root 移除、purge 与重装

Agent `a500c3d` 增加 Root `prepare-retained-removal` 和 `retained-removal-status`。准备绑定原初始化 UUID、原版本、审核归档、固定二进制/单元/maintainer 摘要、原 UID/GID、Core loader 和停机状态。持久意图保持启动门禁，五个阶段回执核对同一意图摘要。只允许已停用、未绑定且无业务写入的已初始化状态通过本条路径。归档检查排空有界 tar 零填充，避免较大的 maintainer 脚本使 `dpkg-deb` 阻塞。

首个独立 Root 的归档宿主属主和镜像排除的五份文档示例分别在发布意图前触发拒绝。`4b6271a` 兼容这五份固定示例缺席，继续严格核验程序、单元和 maintainer。真实 dpkg 以一次正式源码更新沿原初始化升级，六份原文件精确，公开代码身份按原升级回执更新。唯一 remove 已通过 `prerm` 而后在 `postrm` 因组校验读取已不存在的 package-v2 标记退出 1。原现场为 `deinstall ok half-installed`，15 个已核条目及 Core 文件保持，原准入回执保留，正常停放，无再次派发。

`34d3b5f` 在移除负载后用已配对的原 GID 校验现存控制组。46 项 Linux ARM64 定向回归通过。独立 Debian 13 ARM64 正式包单次 `remove`、`purge`、`unpack`、`configure` 均 exit 0，阶段回执共享同一意图摘要。原包已删除后用同版本、同摘要归档分步重装。unpack 后仍保持启动门禁，configure 后原回执为 REINSTALLED，正式检查通过。七份原数据和初始化回执的内容及元数据保持，三个 Core 文件一致，整个过程没有自动启动。专用现场已正常停放。dpkg purge 仍报告库目录非空，已确认残留 Python bytecode，清理单独待验。

原半安装 remove 的核定续接、具有身份或业务记录的同 Root 维护、跨启动和其余故障阶段均为内部待办。完整 Core/OSGi、其余 typed brokers、六组件矩阵与冻结的全部 12 父项继续原范围。原 7fd 41 输入/16 回执和原 UNKNOWN 保持。最终父项 0/12，external deferred 仍仅原三项。见 [检查点](actual-native-agent-retained-removal-checkpoint.json)。



## 2026-11-19 Agent 原生 remove/purge 和已初始化边界

两个独立、无外网的 Debian 13 ARM64 Root，均安装配对的 Core 7d4789fe1d 和 Agent 3547424 正式 deb。未初始化的现场使用实际 dpkg 各执行一次 remove 和 purge，均 exit 0。remove 后为 `deinstall ok config-files`，purge 后包从 dpkg 清单消失。Agent 负载和控制器移除，专用账号保留，Agent 单元停止，Core 仍安装，三个原文件摘要一致。第一次探针用了错误的服务账号名，在任何 remove/purge 派发前纠正，沿同一次安装继续。

另一个现场单次正常初始化后保持 Main、Agent 和两个 socket inactive。真实 dpkg remove 在 `prerm` 拒绝并返回 exit 1，dpkg 记录为 `deinstall ok installed`。原 11 个状态条目、其中 7 个文件的内容及元数据保持，原初始化回执、已装二进制、控制器、maintainer、Core loader 与服务状态均一致。未再次派发 remove，未派发 purge。两个现场均正常停放。macOS maintainer 契约 8 项通过。

已初始化数据的合法移除、跨中断续接、purge 和同 Root 重装仍为内部待办。原三现场、原 7fd 41 输入/16 回执及已有 UNKNOWN 保持。冻结父项 0/12，external deferred 仍仅原三项。见 [检查点](actual-native-agent-remove-purge-boundary-checkpoint.json)。



## 2026-11-19 原生首次绑定、mTLS 与实际 Core 只读业务

在相互隔离的 Debian 13 ARM64 网关、中心及独立 PostgreSQL 验证包级联调。网关安装 Core 7d4789fe1d、GWT 1205092、Networking 4ee16c9 和修复首次绑定的 Agent 3547424。原生中心安装和显式启动均通过，标准安装器为 READY。中心单次初始化 APPLIED，独立 PostgreSQL 应用角色受限，公开 schema 由 0 表增加到 14 表。

实际 GWT 管理员各派发一次 CSR、暂存和绑定，中心单次只登记公开叶证书。两个本机 broker 各接受一次。网关正式身份指针、候选回执和完成意图字节一致，10 份源文件和独立副本摘要一致。首次绑定后通道仍为 OFF，中心无心跳。受控停止本机 Main 和 Agent 后，正式控制器授权 Kura 远程开关并配对独立 gRPC 端口；两份原回执均 APPLIED，身份和初始化保持。此前绑定 UNKNOWN 的现场保持停放。

通道显式开启一次，实际 gRPC mTLS 心跳的叶证书、generation、连接 epoch 和网关启动标识与中心记录一致，中心确认 Kura 可用。匿名客户端收到 TLS 要求客户端证书的拒绝。中心各提交一次配置列表和 ClockService 类型化读取，两个原任务通过本机 Agent 和真实 OSGi 返回 SUCCEEDED，PostgreSQL 的输入及结果均为密文。GWT 启用回包时仍显示 disconnected，显式只读刷新确认 connected，随后关闭一次，中心恢复 offline。

实际停独立 PostgreSQL 后，readiness 由 200 变 503，liveness 仍为 200。同一数据库容器和卷恢复后 readiness 为 200，中心 PID 和 InvocationID 保持。数据库标识、原登记、两条任务及其密文摘要保持，使用原 ID 只读核对，无新派发。另单独正常停止并启动 Agent，保留 OFF 版本、绑定回执和两条 SQLite 执行记录，Core 和中心进程保持。GWT 重新登录后只读读回同一绑定回执及 OFF 状态。

本轮专用网关、中心和数据库正常停放，服务已禁用，数据保留。原三现场及原 7fd 的 41 输入/16 回执保持。其余 Core/OSGi 业务、typed brokers 和完整六组件矩阵仍为内部待办。冻结父项 0/12，external deferred 仍仅原三项。见 [检查点](actual-native-positive-mtls-and-business-checkpoint.json)。



## 2026-11-19 当前原生中心、数据库与首次绑定边界

在独立 Debian 13 ARM64 Root 当前 Core/GWT/Networking/Agent 同装。中心生产输入和上轮已验证发行包精确，console HEAD 之后仅变更检查点文档。标准安装器后端实际安装，原回执成功且无自动启动。显式启动后标准状态为 READY，测试宿主提供了读取进程摘要所需的进程可见性，中心服务本身为零 capabilities、NNP 1。独立 PostgreSQL 16.15 以密码认证，应用角色无 superuser、createdb、createrole，初始化前 0 表、之后 14 表。采用独立测试 CA 签发的中心服务证书，中心 API 单次初始化后从正式认证入口读回原 APPLIED，发布后证书和同一中心 PID/InvocationID 验证成功。Cockpit 后端已验证，未走 Cockpit 浏览器页面。

Agent 的版本化 Networking 模板实际为 root:kura-key-control 0640。Agent 3edb9b8 兼容精确模板布局，vendor 生效文件仍只认 root:root 0644。旧本机授权在创建意图及 gpasswd 派发前明确 NOT_FOUND。Agent 6dc9ae3 解决原生包历史 maintainer 复核和新包头输出，用原已装版本、二进制、bridge 和已装源码核实历史包头。已验证新包的维护控制器在受保护 Root 目录准备原操作，真实 dpkg 在同一个初始化现场升级。早期旧控制器、世界可写祖先及非 Root 归档的拒绝均留证，发生在发布维护意图之前。沿原维护编号完成升级，原初始化回执保持。沿已证实未派发的原本机授权编号完成一次 gpasswd，原材料授权随后完成，Main、Agent 和两个只读中继实际启动，远程保持 OFF。Python ARM64 Linux 42 项通过。

实际 GWT 管理员首次改密后，同一 Java CSR 只生成一次，私钥保持在网关。外部 CA 签回后中心只登记公开叶证书，GWT 单次阶段写入成功。首次绑定请求只派发一次，GWT 返回 CONNECTION_FAILURE，原结果按 UNKNOWN 保留。两个 socket 各接受一次，10 份独立源/候选文件摘要相同，Agent 生成 6 份校验派生文件，正式身份、接受回执、提交标记和待提交指针均未出现。中心原登记仍 offline，无首次心跳。保留原 CSR、候选、中心数据库、初始化和 OFF 决定。两个新现场正常停放，无重发、换号或跨 Root 查询。

Agent 3547424 修复候选经过校验后读取尚未发布回执的故障。首次绑定用待提交回执的字节构造连接配置，重新加载已提交身份仍使用原磁盘回执字节。原二进制语义在独立 ARM64 回归复现 ENOENT；修改后 36 项 ARM64 Linux 定向通过。已构建这个修复提交的正式 ARM64 deb。原 UNKNOWN 现场继续按原版本停放，完整首次绑定、原位核定、mTLS、业务读取和发行矩阵继续内部待办。原三现场、个人文件及原 7fd 41 输入/16 回执保持。冻结父项 0/12，external deferred 仍仅原三项。见 [检查点](actual-native-current-center-and-first-bind-boundary-checkpoint.json)。



## 2026-11-19 Core HTTP OSGi keystore 生命周期与默认 Docker 原卷重启

Core 50a9028f0b 实现正式 SCR 描述声明的 `unsetKeystoreService`。解绑仅清理当前服务实例，已换绑后的旧实例通知不影响新绑定。对应 Jetty 刷新在单一执行器串行完成，关闭旧 TLS 和双向 TLS 监听，只运行配置里原有的 HTTP 端口。合法 provider 返回时按原配置恢复两类 TLS。未绑定的启动等待仍为 12 秒。组件停机终止后续刷新，等待 Jetty 收尾时释放组件锁。JDK 21 正式候选 HTTP manager 15 项通过，包括 HTTPS、mTLS、CRL、晚到绑定和旧实例解绑。

隔离原生 Debian 13 ARM64 安装从当前 Git 生成的正式 Core 包。实际 Felix SCR 暂停并启用文件 keystore provider，同一 Main PID 与 systemd InvocationID 内由服务 ID 89 变为 176。本机 443 和 4443 实际拒绝连接后恢复，443 的原证书指纹与 Core 凭据指纹保持。标准 systemd 停机用时 1.55 秒，两端口关闭。夹具禁用并停放。原生探针验证 4443 监听，mTLS 正向握手由 Jetty 测试覆盖。

默认 Docker 首次运行配对同一 Core DEB，四份预装 YOFC 部署包成功，SIGTERM 正常结束。Log4j 在 Jetty 收尾阶段已退出，保留首次退出状态与日志。同一容器的下一次启动在 Java 前被明确拒绝。只读原卷复核定位到 Core 的 `Store.stable()`，初读批准的部署包长度上限在二次核对退回默认 64 KiB，实际归档长度 10,528,481 字节。Core 7d4789fe1d 让二次核对重用各文件的原上限，保留原设备和 inode、属主、权限、摘要与换文件拒绝。ARM64 Linux 的 Core 和 Docker 合计 65 项通过。

从修复提交重建并配对的默认 ARM64 Docker 续接这个原失败卷，保持原容器失败状态。启动及修复后的同一容器实际重启均通过。四份归档均从已持久化记录装载并核对原摘要，凭据指纹、原卷与证书保持。JVM 为 UID 122、零 capabilities、NNP 1，默认 Gogo 关闭。两次 SIGTERM 停机都实测清理了原正在使用的 Jetty 工作目录，父级 Equinox 目录仍可读，均无此前的 SCR 解绑缺失错误。所有新夹具已停放。

原三现场及原 7fd 41 输入/16 回执保持。完整 Core/OSGi、其余 typed brokers、全六组件矩阵与全部 12 父项继续原范围，最终 0/12，external deferred 仍仅原三项。参见 [检查点](actual-core-http-ds-and-docker-restart-checkpoint.json)。



## 2026-11-19 当前六组件 ARM64 联合发行与默认 Docker 预装包

从当前 Git 用 JDK 21、Maven 3 和 Linux ARM64 Rust 构建 Core、GWT、Networking、Agent、中心及其安装器、Docker，同时构建发行所需的 Deployment、Wires 和四份 YOFC 部署包。Agent 使用与中心同提交打出的协议 crate，打包复核 Networking Java bridge 与 Core loader 摘要。

Debian 13 ARM64 专用 Root 用真实 dpkg 装 Core、Networking、GWT、Deployment、Wires 和 Agent。默认 Cockpit 安装器包内正式 `installer.py` 完成中心的原上传、安装、回执和摘要验收，原操作回执为 APPLIED，安装时服务未自动启动。测试最初调用了供日常配置使用的 `kura-console-setup`，命令解析阶段拒绝，之后从正式入口执行，中心安装只派发一次。Agent 在 Main 前独立初始化并启动。Main 实际 OSGi 激活 Core、GWT、NM、agent 包中外置的 bridge、Deployment 与 FileMove；Agent 和中心进程的 capabilities 均为零，NNP 为 1。中心显式启动后，同服务 UID 所见在运行二进制摘要，与包内摘要匹配；已装安装器的固定证书 HTTPS 探测确认本机 18443。当前容器宿主侧 `/proc/PID/exe` 访问被拒，安装器的标准状态仍报告 UNVERIFIED。Cockpit 浏览器入口、中心数据库、注册和 mTLS 留待联验。

默认 Docker 镜像首次启动实际发现四份预装 YOFC `.dp` 安装事件均失败。非 root Deployment Agent 试图在持久化后删除镜像自有的只读源。Deployment 5a6025b 只对启动注册表中的本地归档保留原源，将副本同步后原子发布到持久目录；主动安装和远程下载保留原行为。JDK 21 全仓 240 项通过。

重建的默认 Docker 镜像用新卷收到四份实际 OSGi 安装成功事件。另一次使用已明确失败的原 Docker 卷，以新镜像进行受控升级，同样收到四份成功事件。原卷凭据、附带文件指纹和 HTTPS 证书准确保持，两次均核对镜像只读源、私有持久副本和注册表。默认 Gogo 端口关闭，JVM UID 122、无 supplementary group、capabilities 为零、NNP 为 1。原失败容器、新运行容器和 Debian 验证现场已正常停放。修复后的两个 Docker JVM 均在停止时记录 Core HTTP DS 缺失 `unsetKeystoreService`，列入内部待办。

全六组件跨布局及安装、升级、移除、purge、中断矩阵，完整 Core/OSGi 业务及其余 typed brokers 继续内部待办。原三现场及原 7fd 41 输入/16 回执保持，历史 UNKNOWN 沿原记录。冻结父项 0/12，external deferred 仍仅原三项。见 [检查点](actual-current-six-component-arm64-release-checkpoint.json)。



## 2026-11-19 Deployment 换包门禁跨系统管理器重启

Deployment fddf031 用持久、事务自有的 systemd generator 建立早期 Main mask。generator 只向 systemd 的 early 输出目录发布 mask，启动时无需已装 Deployment 代码或读取 /opt。Debian 维护只有在同一 Root、原事务文件、generator 及锚点的 inode 和摘要都核对后才续接；配置完成只清理自有 generator。管理员 linked unit、vendor 文件及 Root 原配对保持。旧格式回执保留同 boot 校验。

Debian 13 ARM64 一个独立 Root 以管理员 linked 布局真实 unpack。权限夹具造成实际 half-configured，gate 保持。原测试最初把 FragmentPath 当作解析后的目标，随即按 systemd 实际返回的管理员链接路径验证，未重复创建链接或包操作。实际 systemctl soft-reboot 将计数 0 增到 1，dbus invocation 变化，内核与原 Root 指纹保持。原 generator、回执、链接及目标文件精确。重启前后真实 Main start 均被拒绝。随后对相同字节的异 inode generator 验证拒绝释放，恢复本次自有链接并修正测试权限后，原 dpkg configure 成功。只移除本次 generator 和生成的 mask，管理员链接、Core 凭据和 vendor unit 保持。

另一独立 Debian 13 ARM64 vendor 现场在实际 dpkg unpack 后正常停止并重新启动整个容器。新的 PID 1 出生时间和容器 StartedAt 已核对，仍用原内核，Root 设备号与 inode 匹配。新 systemd 启动时原 gate 再生成 mask，实际 Main start 被拒绝。原 configure 成功，保持既有布局。两个现场均无 Main 执行，正常停放。JDK 21 全仓 239 项通过。

实际硬件冷启动、换设备后的 Root 恢复、旧格式跨 boot、其他 launcher 和完整六组件矩阵仍待验证。本次管理员 linked 布局用于验证换包保护与布局保留，未在该布局启动业务。原三现场、原 7fd 与历史 UNKNOWN 保持。全部 12 父项仍 0/12，external deferred 仅原三项。参见 [检查点](actual-deployment-main-gate-restart-checkpoint.json) 和 [systemd generator 文档](https://www.freedesktop.org/software/systemd/man/latest/systemd.generator.html)。



## 2026-11-19 Deployment 正式 Main 换包准入与半配置续接

Deployment a32aa7e 将 Main 生命周期接入 Deployment 的 Debian 换包。活动 Main 或仍有启动作业时拒绝替换。Main 停止后，维护脚本持久绑定同一 Root、boot 与本次 runtime mask 的 inode，并验证 mask 生效，再允许替换。configure 成功才释放本次自有 mask。既有管理员 mask、Core vendor unit 和 Root 配对保持。preinst/postrm 内嵌同一段正式检查源码。未配置的新包可按正常 dpkg 移除，前提是未创建 FileMove 业务状态、容纳它的服务保持静止。

独立 Debian 13 ARM64 从已装 Deployment 8b6fcd0 和真实活动 Main 开始。在实际 dpkg upgrade 入口拒绝更换，旧 JVM PID/出生时间、实际 OSGi provider、包版本和已装两份 JAR 完整保持。旧 prerm 的 socket 暂停经 dpkg abort 回到原 socket 状态。用标准 systemd 停止专用旧 Main 后实际 unpack 成功，新的临时 mask 与记录精确，真实 Main start 被拒绝。用独立测试目录权限冲突造成实际 dpkg half-configured，现有 gate 与 inode 保持。仅修复测试权限后正常 configure 原包，释放相同 inode 的 mask。正式候选两份 JAR、gate、broker、unit 均与包精确；新 Main 真正加载 FileMove provider。Core canonical key、provisioned.json 和 vendor Main unit 保持。Maven/JDK 21 全仓 239 项通过。

另一个新隔离 Debian 13 ARM64 Root 用标准 dpkg unpack 后直接 remove，保留独立管理员 runtime mask 的完整所测指纹，Core 保持安装，没初始化 FileMove 状态或启动 Main。两个新现场均正常停放。上一轮业务动作及 UNKNOWN 保持。当前覆盖 vendor 布局和同 boot 续接；跨 boot、其他合法布局/launcher、其余 provider 和完整六组件矩阵仍内部待办。冻结父项 0/12，external deferred 仅原三项。见 [检查点](actual-deployment-running-main-cohort-checkpoint.json)。



## 2026-11-19 受保护 FileMove MOVE 原源清理和实际丢回复

Deployment 8b6fcd0 将目标发布与源清理分别记录。合法 Main 在提交前同步原源路径、设备号、inode、时间与摘要，只把原意图摘要交给 Root actor。目标同步并提交后，actor 生成只读证明，绑定同一个 Root、作业、源意图以及已发布目标的 inode 与摘要。Main 核对目标与源的原身份再清理，并独立确认原清理回执。源未确认时维护保持待办，同一受保护目标由原作业占用。旧协议的受保护 MOVE 需要更新 provider，旧原记录留存可读。

同一 Root 可用已安装 FileMove JAR 的 `org.eclipse.kura.internal.hook.file.move.ProtectedFileMoveSource JOB`，由非 Root 部署属主沿原作业完成源清理。若目标证明还没写，先由 Root 用 `broker.py reconcile JOB` 核定原发布。随后本机 Root 可用 `broker.py ack-source JOB` 核对持久源结果。两个恢复入口均沿原记录。替代 inode、未确认目标和缺失清理证据分别拒绝。

Debian 13 ARM64 用当前 Git/JDK 21 正式安装并核对两份 OSGi JAR、broker 和 unit。实际 Main 对 71660 正常 MOVE，原目标和源清理确认齐全。对 71661 使用 fixture-only systemd drop-in，加载已装的原 broker，在原目标和只读证明落盘后退出 79。原 Main 观察到真实丢回复，源保持，Root 查询为 DESTINATION_APPLIED_SOURCE_PENDING，过早确认与实际 dpkg remove 被拒绝。第一份测试替代文件带宿主属主并被拒；Root 所有、同字节的另一 inode 再被独立拒绝。用专用夹具构造源 inode 已进入原暂存名的边界，并在原路径建立独立新文件。正式非 Root Java 恢复只清原 inode，新文件与目标精确。Root 确认原结果后同作业读回 ALREADY_APPLIED。零目标重派，原 Main 正常停机，Main 与 socket 禁用，独立现场正常停放。Maven/JDK 21 全仓 239 项通过。

源重命名边界由专用进程构造；真实丢回复由实际 Unix socket 上的原 broker 进程退出触发。跨启动或挂载配对、协议不同时的包/运行中 JVM 联合切换、初装半配置及实际云通知仍待验。其他受保护位置、完整 Core/OSGi、其余 typed brokers 与全部六组件发行矩阵继续原范围。冻结父项 0/12，external deferred 仅原三项。见 [检查点](actual-deployment-protected-filemove-source-checkpoint.json)。



## 2026-11-19 FileMove 原发布核定和 Deployment 包维护

Deployment bd3fbec 保存同一原批准和暂存文件的设备号、inode。原领取、候选发布记录及终态均用同目录原子创建。Root 可用 `/usr/bin/python3 -I -B /usr/libexec/kura-filemove/broker.py reconcile JOB` 核定。只有正式目标可证为原审阅副本时才发布原结果。若原无覆盖动作停在硬链接建立后，核对正式目标与原暂存链指向同一 inode，只移除确切的暂存名。旧领取没有候选记录或当前可见文件无法配对时保持 UNKNOWN。此动作只核定目标发布。

Debian 13 ARM64 当前 Git/JDK 21 正式 deb 标准安装，核对已装两份 OSGi JAR、actor 和 unit。两个专用 Linux 进程加载正式已装发布代码，分别在覆盖目标已同步但结果尚未写入，以及无覆盖硬链接已创建但尚未移除暂存名时退出。两个原作业均为 UNKNOWN。以相同字节的替代 inode 做负对照，保持 UNKNOWN 且原领取、发布证据精确。回到原路径后在同一 Root 沿原作业明确核定。覆盖目标的完整指纹保持，无覆盖目标保留 inode、字节、属主、mode、mtime，移除原暂存硬链接时 nlink/ctime 变化。零重发或跨 Root 查询。

未决时，实际 dpkg remove 和 upgrade 都在建立维护围栏前被拒绝，原部署包、socket、actor 进程与回执保持。核定后，实际标准升级保留原已禁用、inactive 的 socket；标准 remove 与 purge 退出 0，正式 broker 和 unit 移除，两个目标、Root 审批及发布回执精确保留。Main 本次未启动，最终仍禁用，现场正常停放。Maven/JDK 21 全仓 239 项通过。

本轮未重复上一检查点的真实 Main FileMove 业务。单独的真实 Main 业务证据在上一检查点。MOVE 源清理跨中断恢复、真实客户端丢回复、较早未带维护门禁版本的完整迁移、初装半配置及全 provider/launcher 继续内部待办。最初的原 71456 停放现场保持。完整 Core/OSGi、其余 typed brokers 和全部六组件矩阵保留。冻结父项 0/12，external deferred 仅原三项。见 [检查点](actual-deployment-protected-filemove-recovery-checkpoint.json)。



## 2026-11-19 FileMove 受保护路径和原作业回执

Deployment 2a63783 为 `/opt/eclipse/kura-deployment-files/` 的直接子项使用独立的 systemd Unix socket 和 actor。普通目的地的 COPY/MOVE 保留现有入口。受保护请求使用 DEPLOY-V2 解析得到的原作业号。actor 校验内核 peer credentials、pidfd 和 systemd 记录中的同一活动 Main。Root 审批绑定本地 Root、原作业号、目标名、动作、是否覆盖、长度及 SHA-256。可用已装 `/usr/bin/python3 -I -B /usr/libexec/kura-filemove/broker.py authorize JOB COPY|MOVE OVERWRITE NAME SIZE SHA256` 审批，OVERWRITE 为 0 或 1，`status JOB` 原地只读。actor 不解析源路径，只接收有界载荷。正式目标以原子操作发布。原领取记录先于修改落盘，终态回执后于目录同步。MOVE 只在当前 actor 明确确认后移除同身份源，疑似已提交结果沿原作业保留。

隔离 Debian 13 ARM64 上从当前 Git/JDK 21 正式安装。包中及实际已装的两份 OSGi JAR、broker 和两份 systemd unit 核对。合法 Main 内实际 FileMove OSGi hook 完成一次原子覆盖 COPY 和一次新目标 MOVE。普通旧 MOVE 保留。Main 直接写保护目标、同 UID 的外部进程、无签发和错摘要均被拒绝。实际 actor 的能力位图均为零、NNP 为 1。对第 71456 号原 MOVE 用目录内挂载的合规普通文件制造 rename EBUSY。原领取已落盘，目标和源精确，客户端得到 UNKNOWN，零重派。Main、actor 与 socket 正常停止，原 Root 同作业读回仍为 UNKNOWN，两个旧目标源完整保留。现场 Main 与 socket 禁用后正常停放。Maven/JDK 21 全仓 239 项通过。

本轮是固定受保护目录、正式 Main 内 hook、Debian 套件安装及受控 rename 失败的验收。真实云通知、进程丢回复、目标发布后中断、MOVE 源清理恢复、保留 socket 的禁用状态、未决维护准入及 Deployment 的升级/移除/purge 仍待验证。其他受保护位置、完整 Core/OSGi、其他 typed brokers 和完整六组件矩阵继续原范围。冻结父项 0/12，external deferred 仅原三项。见 [检查点](actual-deployment-protected-filemove-checkpoint.json)。



## 2026-11-19 历史 SystemUpdate 原文件纳管和无 Verifier 发布

Deployment 3985bfa 增加两步纳管。仅在核实原作业从未派发后，本机 Root 用已装 JAR 的 `org.eclipse.kura.core.deployment.download.SystemUpdateExecution --adopt-undispatched` 签发。参数依次为用于纳管的 UUID、原作业号、当前配置下载目录下的规范脚本绝对路径、Root 审阅副本路径和摘要，成对时再传 Verifier 审阅副本路径和摘要。使用同一个 UUID、原作业号、规范路径，由非 Root 部署存储属主调用 `org.eclipse.kura.core.deployment.download.SystemUpdateLegacyMigration`。配对核对同一个 Root、路径及每份摘要，纳管只登记原文件身份。发现原派发文档、用过的领取记录或现有其他发布回执时保留现场。仅凭旧文件存在或缺少派发文档无法认定原作业未执行。

正常无 Verifier 下载也持久记录原作业及发布 UUID。其授权只包含批准的脚本；同位点或授权出现意外 Verifier 时拒绝。单文件更新成功后报告该原作业 COMPLETED。成对位点保留 Verifier 合同和相关授权。两类更新均经原派发记录。

独立 Debian 13 ARM64 Root 先标准安装上一正式包，在旧包上准备历史文件和对应未派发证明，再标准升级到从当前 Git 构建的包。实际已装的发布、审批和纳管入口保持历史文件字节、inode 与所测元数据。Root 执行纳管按角色被拒绝，存储属主登记成对和单文件两个原作业。带旧派发文档的第三个位点拒绝纳管，原文档和载荷精确，无审批和执行。实际 Main 中两个旧作业各执行一次原主动作，成对作业再执行一次 Verifier。通过同 Root loopback HTTP 下载另一无 Verifier 的新作业一次，按原发布 UUID 授权并执行。所有实际子进程的 CapEff 为零，NNP 为 1；原三作业号各获一次本地探针记录的 COMPLETED。Main 正常停止、禁用，现场正常停放。Maven/JDK 21 全仓 239 项通过。

未知历史结果保持当前判定。同一 Root 启动切换、实际云端通知及硬断电仍待验证。完整 Core/OSGi、保护目标的 FileMove、其他 typed brokers 和完整六组件矩阵继续内部待办。冻结父项 0/12，external deferred 仅原三项。见 [检查点](actual-deployment-system-update-legacy-admission-checkpoint.json)。



## 2026-11-19 SystemUpdate 原授权签发续接

Deployment 05be6d2 为本机 Root 签发增加原作业锁和持久签发意图。校验审阅载荷后先同步原授权意图，再逐份以原子替换发布首次出现的定稿副本。后续只核对已存在副本，按原摘要补齐缺失项。两份均精确后，以原意图发布正式授权。路径或原 UUID 不符时拒绝续接。显式创建并设置授权、定稿和目录权限，适用 Root umask 000 和 077。已完成的授权再次核对返回 ALREADY_APPROVED。

在全新的隔离 Debian 13 ARM64 Root 用当前 Git、JDK 21 正式 deb 及已装 JAR 验证。第 71237 号在 umask 077 下正常签发，正式回执 0444、定稿 0555。为第 71238 号构造审阅件目的地类型冲突，实际签发器先保留原意图和第一份定稿，正式授权未出现。不同 UUID 被拒绝且原内容、元数据精确。保留测试阻挡物后，沿同一 Root、同一 UUID 只补第二份并发布正式授权。第一份定稿的 inode 和元数据精确，原意图 inode 成为正式授权。两组原部署回执及载荷保持，零脚本执行。此 Root 禁用 Main 并正常停放。Maven/JDK 21 全仓 236 项通过。

本次覆盖正常签发和中途文件系统拒绝后的原地续接。未对本次发行重复运行上一检查点的 Main 及脚本场景。硬断电、跨启动或跨挂载、历史无回执脚本纳管、保护目标的 FileMove 委派、完整 Core/OSGi 业务及六组件矩阵继续原范围。冻结父项 0/12，external deferred 仅原三项。见 [检查点](actual-deployment-system-update-approval-recovery-checkpoint.json)。



## 2026-11-19 SystemUpdate 本地执行授权与原派发回执

Deployment ee339f3 在安装和 Verifier 入口核对已提交的原发布回执。本机 Root 明确审阅两份载荷，并用已装 Deployment JAR 的 `org.eclipse.kura.core.deployment.download.SystemUpdateExecution` 授权。参数依次是原发布 UUID、作业号、规范化的已发布脚本绝对路径、两份 Root 所有的已审阅副本的绝对路径及各自 SHA-256（每份路径紧跟该份摘要）。授权绑定本地 Root、原 ID、作业、路径和两份摘要。Main 从 `/opt/eclipse/kura-deployment-approvals/<作业号>` 的只读边界启动定稿载荷。固定 setpriv 销权并设置 NNP，取消通用特权命令服务依赖。两种动作都在启动前持久化原领取记录。Verifier 只有在同一更新成功且通知作业、文件与发布者匹配时可运行。未定态保留原领取记录并拦截重派、退役和同位点替换。

Debian 13 ARM64 同一隔离 Root 以 JDK 21 当前 Git deb 正常升级。第一次候选受 Root 命令 umask 影响产生 0666 授权，实际 Main 拒绝且零派发。按原摘要逐份核对后，只修正原授权 mode 为 0444，原 UUID、文件字节与 inode 保持，再用标准 dpkg 升级到显式指定文件权限的最终候选，核对正式包和已装两份 OSGi JAR。未授权对照零执行。实际 Main 中第 71192 号原作业的更新和 Verifier 各执行一次，UID 为 kurad，CapEff 为零，NNP 为 1，仅向探针提供的通知端一次报告 COMPLETED。第 71193 号脚本在原领取记录落盘并输出一次后停住，精确核对 PID、出生时间及 Main 的父子关系后仅终止这一个进程。原记录回报 UNKNOWN，零 Verifier、零重发，原数据和授权完整保留。Main 正常停止并禁用，新 Root 正常停放。236 项 Maven/JDK 21 测试通过。

本轮验证使用受控惰性载荷、实际 Main 和本地观察通知。最终候选读取了沿用的两份授权，显式 mode 的新签发全链尚未重测。无成对发布回执的历史脚本现在保留文件并阻止执行，其原地合法纳管、授权中断恢复、跨启动或跨挂载配对、运行中脚本所需的粗粒度权限替代及真实云端通知交接继续内部待办。受保护 FileMove、完整 Core/OSGi、其余 typed brokers、六组件/provider/launcher 与完整安装升级移除 purge/中断矩阵仍在原范围。冻结父项 0/12，external deferred 仅原三项。见 [检查点](actual-deployment-system-update-execution-admission-checkpoint.json)。



## 2026-11-19 原 SystemUpdate 混合态本地恢复

Deployment 11f4c37 增加仅本地部署存储属主可运行的恢复入口。逐份验证原 ID、文件副本、当前文件及待保留候选，先持久化 ROLLING_BACK 决定，再将原 inode 恢复到对应位置。中断后同一入口只补仍缺的恢复步骤，原候选和回执保留。若两份候选已与原发布精确配对，沿原提交核定；恢复完成的位点留有 ROLLED_BACK，继续拒绝下载复用、安装、Verifier 及替换。恢复入口不负责重新签发原位点执行授权。

重新启动前一轮保留的实际 Debian 13 ARM64 Root，原混合态、第 42 号已提交和第 43 号已退役位点均精确，Main 仍禁用。用 JDK21 从当前 Git 构建并正常 dpkg 升级 Deployment 到 11f4c37，Core 套件、两个已安装 JAR 和三份 Core 保护文件核对。UID0 拒绝且无状态变化。由 kurad 调用正式包的本地恢复入口一次，沿第 41 号原发布 ID 将旧 Verifier inode 恢复到正式路径，内容、mode、属主、mtime 保持，重新硬链接使 ctime 变化。被替换的候选以原 inode 保留，旧更新文件及另两个位点完整精确。此前的部署请求、更新脚本、Verifier 及 HTTP 均未重发或执行。

实际已装 Main 复核旧内容、恢复后禁用执行和下载复用、其他已提交位点仍可准入。随后正常停止，Main 保持禁用，原容器正常停放。全仓 JDK21 构建通过 236 项。恢复过程再次中断在单元测试覆盖；实际硬断电及发生设备号变化的跨挂载恢复尚未验。恢复后的再次准入、受保护 FileMove 与 SystemUpdate 执行授权、完整 Core/OSGi 和六组件发行矩阵继续内部待办。冻结父项 0/12，external deferred 仍仅原三项。见 [检查点](actual-deployment-pair-original-root-rollback-checkpoint.json)。



## 2026-11-19 SystemUpdate 成对发布与同 Root 中断核定

Deployment 62a0d36 对更新文件和 Verifier 先持久化同一发布意图、原文件副本位置和指纹，再更新两个正式文件。两次 rename 和终态回执各自同步到持久目录。下载读取、安装和 Verifier 共享 per-slot 进程及文件锁。原发布意图未核定时，拦截执行和重新下载。当前路径的两份内容及 inode 均精确符合原意图时，可用原 ID 确认提交。正常显式退役也有持久记录，受控删除两份载荷。早期无回执旧文件沿既有入口处理。

从当前 Git 和 JDK21 构建 Deployment，Debian 13 ARM64 正式安装 Core 2a3b588e63 和 Deployment 62a0d36。已装 Deployment/FileMove JAR 均与正式包及本次构建精确。两个专用进程调用已安装的 prepare 后，分别在第一份、全部两份文件实际原子替换后直接退出，保留同一 Root 的原记录和旧文件。合法 Main 读取第一个场景时拒绝安装及 Verifier 的 executor，调用为 0，请求也未重发到 HTTP。第二个场景使用原 ID 写 COMMITTED，两份已落盘文件的字节、inode 及元数据精确，无再次替换。另一个正常请求实际通过两次 loopback HTTP 完成成对发布，显式退役后留有 RETIRED 回执且两份正式载荷均消失。脚本执行为 0，Main 正常停机、禁用，现场已停放。Maven/JDK21 全仓通过 233 项。

本轮过程故障由专用探针触发。只完成一份的旧位点仍受控保留，明确的恢复动作、跨启动/跨挂载身份核定、后续执行授权、受保护 FileMove 目的地委派及完整发行矩阵继续内部待办。冻结父项 0/12，external deferred 仅原三项。见 [检查点](actual-deployment-paired-publication-interruption-checkpoint.json)。



## 2026-11-19 Deployment 校验暂存与实际 FileMove 交接

Deployment 715f48e 与 efa0ca9 将下载和 verifier 的文件名限制为受配置目录下的直接子项，拒绝符号链接和非普通文件。下载先写同目录暂存、校验和同步文件，再分别原子替换正式路径。传输或校验失败留下已有文件，成功发布后才调用 postDownload。异步完成通知等到发布与 hook 结束，同步 pending 查询在传输结束后仍报告进行中。独立安装请求检查它实际指定的产物路径。

当前 Core 2a3b588e63 与 Deployment 715f48e 正式 deb 在同一个 Debian 13 ARM64 隔离 Root 首装，然后标准 dpkg 升级 Deployment 到 efa0ca9。包中、当前 Git 构建产物与已安装的 Deployment/FileMove 两个 OSGi JAR 精确。首个探针缺少 KuraPayload 导入，首轮无发布，保留原诊断和文件。修正探针后在当前合法 Main 实际走同容器 HTTP 和已装 FileMove 服务，共六次请求。新内容校验后复制；校验失败保留原字节、inode 且不调用 hook；verifier 失败保留两份旧文件；两份均校验成功时分别发布。两条 SystemUpdate 路径均未执行脚本。五份原资料指纹精确，Main 正常停机、禁用，现场已停放。Maven/JDK21 全仓通过 229 项。

当前两份文件各自原子发布；需要进一步实现跨 verifier 和更新文件的持久提交、覆盖中断核定、SystemUpdate 执行授权及受保护 FileMove 目的地的窄委派。完整 Core/OSGi、其余 typed brokers、六组件联合发行和全生命周期矩阵继续内部待办。冻结父项 0/12，external deferred 仅原三项。见 [检查点](actual-native-deployment-staged-publication-checkpoint.json)。



## 2026-11-19 watchdog Debian 维护与正式 Main 停机准入

Core 2a3b588e63 将 Debian preinst、postinst、prerm 和当前 systemd ExecStopPost 的 watchdog 动作接入同一固定 helper。helper 有界读取旧 marker，核对文件权限与身份，并只接受 Linux sysfs 证实的 watchdog 字符设备。固定保活和 Magic Close 动作通过核对后的设备描述符写入。Maven 将 helper 精确嵌入三份脚本，正式 unit 与已装 helper 入 generation 清单。

当前 Git/JDK21 ARM64 deb 在实际 Debian 13 隔离 Root 中验证。普通文件类型的测试记录导致实际 preinst 拒绝，原测试文件及状态精确。移除人工测试记录后完整解包，由于基础镜像缺少 chrony 依赖而留在 unpacked。用 Debian 发行包补齐依赖，在同一 Root 执行 configure，Chrony 保持 inactive。正式 Main 和 watchdog OSGi bundle 实际 ACTIVE。

实际 prerm 再对人工普通文件类型记录拒绝，原 JVM PID/出生时间、文件与配对载荷精确，reboot/key/clock 三围栏未建立。清除测试记录后标准 Main 停机成功，正式 helper pulse 的 ExecStopPost 退出 0，原凭据和正式载荷保持，现场禁用 Main 并正常停放。macOS 36 项，Linux ARM64 14 项通过。

该 Root 无真实 watchdog 设备，本次实际硬件写入为 0。正向实机设备保活、Magic Close、nowayout、热插拔、旧已安装 unit 和 tar 安装器转换继续内部待验。完整 Core/OSGi、其余 typed brokers、六组件联合发行与全生命周期矩阵仍在原范围。冻结父项 0/12，external deferred 仅原三项。见 [检查点](actual-arm64-watchdog-maintenance-admission-checkpoint.json)。



## 2026-11-19 Docker ARM64 非 Root launcher 与原卷升级

Docker 34fabd8 的正式镜像入口把 Root 限于配对检查、密钥准备和对固定应用目录交接。持久 canonical key 和两份回执保持 Root 所有，JVM 仅读取本次独立运行目录下同摘要的 0400 凭据。默认 Gogo TCP 端口关闭。实际 PID 1 为 UID/GID 122 的 JVM，无 supplementary groups，全部五类 capability 位图为零，NNP 生效。普通日志和审计日志在 Root 保护父目录下由 kurad 创建，实测均为 0600。实际请求 `/admin/auth` 返回 200。初始 Root 使用 CHOWN、DAC_OVERRIDE、SETPCAP、SETUID、SETGID 五项测试启动能力，JVM 持有原卷的同一实例锁。

从当前 Git 经 Maven/JDK21 构建正式 ARM64 镜像，四份正式 deb 与此前同 Git 产物指纹精确，macOS 与实际 Alpine 各 26 项通过。空卷、已中断的原卷、此前 UID0 正常运行过的旧卷三条实际路径通过，三个正向容器均在停机超时前退出。空卷的第二实例在原 JVM 运行时被拒绝，原进程凭据与 HTTPS 证书保持。正常旧卷升级保留了 canonical key 和两份回执的完整指纹，TLS keystore 字节、inode、mode、mtime 与已服务 TLS 证书保持，TLS 文件按运行边界转交 kurad，属主和 ctime 变化。

原中断卷保留了两次入口拒绝。先修正 OverlayFS 父子目录交接顺序，再修正运行凭据先设 mode、再移交的次序。随后沿原卷完成 JVM 真正启动，canonical key 和两份回执自第一次失败保持。该卷首次 JVM 启动前的 TLS keystore 字节在加载期间变化；之后最终镜像保留了该卷实测的已服务证书及同份 keystore。不同进程阶段的证据均已保留。

本次只覆盖显式跳过 YOFC DP 且外网络隔离的 ARM64 Docker 入口。持久卷的其他布局、权限边界、其余 provider/launcher、完整六组件联合安装升级移除 purge、完整 Core/OSGi 和其余 typed brokers 继续内部 pending。冻结父项 0/12，external deferred 仍仅原三项。见 [检查点](actual-arm64-docker-nonroot-launcher-checkpoint.json)。



## 2026-11-19 当前 Git 的 Docker ARM64 发行与同卷续接

Docker 6a8a719、a989d3c、fae4711 强制检查四份 deb 的包名及架构、把实际输入指纹写入镜像，并在运行引导前校验 Root 所有的 Core 密钥策略模块及对应清单。缺失依赖、创建账号、解包和配置失败直接终止构建。实际错包构建在解包前拒绝且未生成镜像。可显式指定运行层 Alpine 镜像源和候选镜像名。YOFC DP 默认继续纳入；本次验证显式使用 `-Ddist.dp.skip=true`，过期 DP 不会随该模式混入。

从当前 Git 用 JDK 21 构建 Core fafc5b3a6b、GWT 1205092、Deployment c6e1fd2 和 Wires 93f318b，Maven 产出的 ARM64 镜像内四份 deb 记录均与实际构建包精确。两次隔离容器启动实际观测 Core、GWT、Deployment 和 Wires 对应 bundle 均 ACTIVE，HTTPS `/admin/auth` 均返回 200。第二次沿同一个数据卷启动，canonical key、两份 marker、HTTPS keystore 的内容及元数据精确，实际服务 TLS 证书摘要相同。两次标准 Docker stop 均在超时前退出且退出码 143；明确核对了启动配置中的关闭钩子，未另测每个框架停用回调。另保留首个缺少自身 hostname 映射的隔离容器和卷，该容器未提供 HTTPS，退出码 137。

macOS 和实际 ARM64 Alpine 各通过 24 项契约检查。两个正向容器使用零 capabilities、NNP 和无外部网络的测试参数，当前镜像 JVM 仍以 UID0 运行。networking 普通和 firewall-only 变体本次仅完成构建；native Agent、console 联合配对、其余 Docker 布局和 YOFC DP、其他 provider/launcher、dpkg 安装升级移除 purge、真实中断核定、完整 Core/OSGi 和其余 typed brokers 仍属内部 pending。冻结父项 0/12，external deferred 仅原三项。见 [检查点](actual-arm64-docker-current-core-gwt-cohort-checkpoint.json)。



## 2026-11-19 Chrony 发布中断与 Debian 提前准入

Debian 13 ARM64 隔离 Root 从当前 Git 正式安装 Core 969c016ced。在实际 OSGi 配置更新中，于发布程序创建暂存文件时挂起原 clock actor。发布程序将候选和原副本落盘后，只终止原 actor。Chrony PID 及进程出生时间不变，实际 daemon 未加载候选。原 namespace 与请求只保留原 intent、prepare、commit-intent 及 sender pending/issued。原请求读回 UNKNOWN，未派发重载或重发。已安装的时钟 quiesce 返回 UNRESOLVED_CLOCK_OPERATION，原现场写入保持关闭。

Core fafc5b3a6b 为 preinst 和 prerm 增加配对只读检查，先核已存在的原时钟记录，再进入 reboot 或 key 维护。早期检查既不建立 namespace，也不停止进程。现构建 ARM64 候选包的两个控制脚本均与源码精确。直接运行候选包的正式 prerm，原中断 Root 拒绝卸载准入，Main 与 Chrony 原 PID、原文件和证据精确，三个维护围栏与 retained/preserve 均保持未创建。此次验证没有调度候选 dpkg 事务。

标准 Main 停机后原 UNKNOWN 与已落盘文件保持，试验 bundle 撤下，现场禁用并正常退出。macOS 与 Debian ARM64 各 58 项通过。发布中断之后的原地核定、提前只读检查与最终 quiesce 之间的并发边界、其他 provider/launcher、完整 Core/OSGi、其余 typed brokers、六组件及发行矩阵继续内部待办。冻结父项 0/12，external deferred 仍仅原三项。见 [检查点](actual-chrony-publish-interruption-and-early-maintenance-admission.json)。



## 2026-11-19 Main 有序停机与实际 Chrony stop

Core 969c016ced 的正式启动配置启用 Equinox 关闭钩子。已有 `KillMode=control-group` 密钥发布门禁保留。配对同步 ExecStop 仅向 systemd 确认的原 Main pidfd 发一次终止信号，等待 JVM 停用 OSGi 并退出，再由 systemd 清理进程组。Root 配对清单核对固定停机程序，Core 有序停机期间保持时钟 broker 可用。

Debian 13 ARM64 从 Git 和 JDK 21 构建并安装，核对正式 JAR、配置模板、vendor 单元和停机程序四份载荷。从已运行 Chrony 进入无文件变更的 Core profile，单次标准 `systemctl stop` 耗时约 3.0 秒。原 JVM 内实际 bundle 的 stop 回调发生，Core cgroup 清空，Chrony 正常退出，原 namespace 下只有一份终态 CHRONY_STOPPED，sender 和 actor 回执精确。六份原文件、快照和 clock actor 进程均保持，零改时及持久配置更新。

专用测试 bundle 撤下，现场禁用且正常退出。真实停机中断与未决结果核定、其他 provider/launcher 的停机行为、完整 Core/OSGi、其余 typed brokers、完整六组件和发行矩阵继续内部待办。冻结父项 0/12，external deferred 仍仅原三项。见 [检查点](actual-core-framework-typed-stop-checkpoint.json)。



## 2026-11-19 Chrony 持久化配置与活动 daemon 重载

Core fd3ff3b4d1 让运行中的成对 Chrony 在新文件发布后沿同一配置事务重载一次。只允许原 clock actor 对经过配对的 Chrony vendor 单元发出固定 restart，随后核对新进程出生时间、原单元和已发布内容。发布后无法确认则保留原提交意图。非空配置让 Chrony 使用其自身的调时策略，Core 不另外派发 makestep。旧权限规则仅在原维护记录和旧 actor 成对时迁移。

Debian 13 ARM64 同一新 Root 使用现构建的 244c3068a6 与 fd3ff3b4d1 标准升级，七份原文件元数据及原历史精确，13 份载荷配对。合法 systemd Main 的实际 OSGi ConfigurationService 更新一次，当前配置与新快照回读精确。已运行的 Chrony 由 PID 1914 成为 PID 2784，实际 chronyc 查询确认候选离线时间源。一次 CHRONY_CONFIGURED，原配置字节及回执保留，两次 daemon 启动均显式禁用改时，零 step 请求。

Main 正常停止后快照、已发布配置和原回执原样。Chrony 仍保持同一运行进程，未观察到 typed stop，停机交接继续内部待办。专用测试 bundle 移除后现场禁用并正常退出。发布后中断的实际原地核定、其他 Chrony 布局、完整 Core/OSGi、其余 typed brokers、六组件/provider/launcher 和全发行矩阵继续。冻结父项 0/12，仅原三项 external deferred。见 [检查点](actual-chrony-configuration-service-active-reload-checkpoint.json)。



## 2026-11-19 Chrony 类型化配置发布

Core 244c3068a6 将 Chrony 配置从 JVM 的直接文件操作迁移到配对的发布单元。固定 Debian 配置路径由零 capability、NNP 的独立 Root 服务写入，调用者必须是已配对的 clock actor。NTP 源和支持的校时参数由 broker 类型化检查，原系统路径、权限和其他 Root 管理指令保持。变更先用实际 chronyd 校验，按原请求 ID 保留原字节，再原子发布。相同配置只读确认，发布结果不明时沿原 ID 查询。

从当前 Git 用 JDK 21 构建正式 ARM64 deb，在 Debian 13 与 Chrony 4.6.1 实际安装并核对 13 份配对载荷。标准 systemd Main 内实际 OSGi 读取 Clock 配置，使用已装 provider 的初始化入口。一次配置发布 APPLIED，相同内容零新请求，额外 include 产生确定 EPERM。原文件副本字节精确，默认 chronyd 解析读回实际配置，直接 Root 读 socket 也被拒绝。Chrony 没有启动和改时。Main 正常退出后两份原回执与文件精确，测试 bundle 撤下，新现场禁用停放。

本轮验证止于实际 provider 初始化、发布及解析。通过 ConfigurationService 持久化业务更新、active Chrony 的后续加载、其余发行布局和真实中断核定仍是内部 pending。完整 Core/OSGi、其余 typed brokers、六组件/provider/launcher、安装升级移除 purge 与全部 12 父项维持原范围。冻结父项 0/12，external deferred 仍仅原三项。见 [检查点](actual-typed-chrony-configuration-checkpoint.json)。



## 2026-11-19 Core 安装器内阶段续接

Core 5e4f795fdd 把布局、broker、运行文件、HTTPS 身份与权限设置分成五个固定步骤。每步先持久化意图，成功后持久化回执。回执绑定原 Root、维护 ID、包版本、控制脚本与原恢复副本。标准 configure 读取已完成回执，仅继续尚未派发的步骤。

在 Debian 13 ARM64 实际安装与当前 Git 配对的 deb。布局已完成和 HTTPS 已完成时分别中断，同一维护 ID 最终安装完成。五对步骤回执、原 intent、prepared、ready 和唯一 package-history 齐全。先前回执未重写，原密钥与 HMAC 秘密、时钟账户和已生成 HTTPS 身份保留。

独立现场在 keytool 已启动且仅有 HTTPS 意图时中断。标准 configure 返回 1，原围栏、意图及文件观察值保持，未重复派发。两个新现场均禁用 Main 并正常停放。macOS 与 Debian ARM64 分别通过 113 项包契约测试。

步骤内未确认结果的原地核定仍是内部 pending。完整 Core/OSGi、其余 typed brokers 及六组件/provider/launcher、剩余发行与中断矩阵继续。冻结父项 0/12，external deferred 仍为原三项。见 [检查点](actual-core-inner-installer-recovery-checkpoint.json)。



## 2026-11-19 原旧包半配置现场同 Root 恢复与实际 Main

原 43156466fe 半配置现场在同一 Debian 13 ARM64 容器内，仅在重新引导时临时屏蔽 Main，随后恢复原启动选择。原维护 ID、原文件指纹与现场连续。overlay 重新挂载导致设备号一致映射，其余八项原元数据逐项核对。Core cbf7df7fa6 正式 deb 经一次标准 unpack 和 configure 升级恢复，原 install 目的保留。同一原 ID 最终一份 package-history 和三份阶段回执。7 份原运行文件联合核对通过，原业务请求、密钥及 clock 账户保持。

安装与当前 Git 配对的正式 GWT 包后，用标准 systemd 在同原 Root 启动 Main。实际 JVM 内 OSGi ConfigurationService 获取 Clock provider 配置，CryptoService 完成内存加解密，GWT 3.0.0.1205092 处于 ACTIVE。实际 HTTPS 回环经原证书固定校验，认证资源返回 200，未认证 console 请求返回 302。Main NNP1、无 CAP_SYS_TIME 与 CAP_SYS_BOOT。正常停止后原回执和运行文件再次精确核对，移除了专用测试 bundle，Main 禁用，容器正常退出。

旧包半配置恢复与此 Main/GWT 组合通过。安装器内部未解决结果、其余 Core/OSGi、typed brokers 和完整六组件/provider/launcher、剩余发行生命周期与中断点继续内部待办。冻结父项 0/12，external deferred 仍为原三项。见 [检查点](actual-legacy-half-configured-same-root-recovery-checkpoint.json)。




## 2026-11-19 Core 实际 postinst 同原维护目的恢复

Core cbf7df7fa6 为 Debian configure 写入绑定包版本、控制脚本、原 Root 及维护 ID 的 intent、prepared、ready 回执。阶段回执先持久化，再清理已执行的安装脚本或已恢复的备份。READY 可沿用原 package-history，围栏释放后继续激活原 broker。仅有 intent 的安装器结果仍需独立核定。

从 Git 和 JDK 21 构建的 ARM64 包实际安装。Debian 13 对 dpkg 配置进程组执行四处有明确触发点的中断。首次配置在 key 围栏释放前后分别中断，最终原 ID 只保留一份 package-history，原密钥、TLS、快照及阶段回执精确。随后独立 remove/保留数据重装在 READY 后、清理备份前中断，恢复数据标记与原阶段回执精确。另一个独立新现场从 43156466fe 跨版本升级，在相同提交点中断和续接，旧历史、密钥和数据标记保留。恢复完成后两个新现场的 Main 均禁用且正常退出。原旧包失败现场仍保持停放。macOS 和 Debian ARM64 定向 Python 各 79 项通过。

仍需实现安装器内部中断及无阶段回执的旧半配置包恢复。完整 Core/OSGi/合法 Main、剩余 typed brokers、六组件/provider/launcher、剩余安装升级移除 purge 及中断点继续内部待办。冻结父项 0/12，external deferred 仍为原三项。见 [检查点](actual-core-postinst-phase-recovery-checkpoint.json)。




## 2026-11-19 发行包 Git 来源与实际 postinst 中断

13 仓的 14 个实发项目 deb 已按各自 detached worktree 的实际 HEAD 构建，主工作树 HEAD 与其不同。逐包核对文件名、Debian control Version 和 SHA256。Core 43156466fe 保持包脚本过滤在 AntRun 嵌入前完成，JDK 21 JVM 23 项通过。按新模板生成并构建了一个 addon deb，环境变量指向主工作树时仍正确取 detached HEAD。

独立 Debian 13 ARM64 新现场先解包 Core，再在真实 dpkg postinst 进入 restore_retained_sender、释放 key-service 原维护围栏前终止同一配置进程组。原现场为 half-configured。一次标准 dpkg configure 续接因安装器已被前一阶段删除而返回 1。原维护 ID、密钥、clock 账户与 TLS 材料指纹精确，Main PID 为 0，原业务记录保持。该现场保留原数据并正常退出。此点的产品恢复仍内部待办。

历史 Main 转换和已有现场沿用原检查点。完整 Core/OSGi、其余 typed brokers、完整六组件/provider/launcher 和所有发行中断点继续内部待办。冻结父项 0/12，external deferred 仍仅原三项。见 [检查点](worktree-package-provenance-and-interrupted-postinst-checkpoint.json)。




## 2026-11-19 ARM64 Core 时钟 cohort 的正常发行生命周期

新 Debian 13 ARM64 隔离现场使用已安装但停用的 Chrony，Agent 缺席。实际从 e952f82 的四操作时钟 cohort 装机启动，运行中的 Main 通过正式 Debian 生命周期升级到 ef78c11 的五操作 cohort。新 Main 进入 Java，保持 NNP1 且无 CAP_SYS_TIME，relay 保持未激活。实际完成 remove、reinstall 和 purge。升级和 remove/reinstall 保留同一数据标记，purge 后隔离数据标记及暂存目录消失。原加密凭据与时钟账户全程保持同字节、属主、模式和 inode，未创建时钟 namespace。新现场正常退出。

明确核对缺 Chrony 的正式 deb 仍由 APT 拒绝，Core 和时钟状态未创建。历史 detached worktree 的默认包版本错误选用主工作树提交号，native Git 插件开关也未纠正。该产物未安装。历史包在独立构建目录中用实际 Git 头明确固定版本后重新组装，提取的 8 项载荷与原 cohort 匹配。

范围为单个 ARM64 Core profile、Chrony 停用和 Agent 缺席。完整六组件的 provider/launcher、其他包组合和中断恢复继续内部待办。冻结父项 0/12，external deferred 仍为原三项。见 [检查点](actual-clock-cohort-package-lifecycle-checkpoint.json)。




## 2026-11-19 Chrony makestep 类型化入口与真实 UNKNOWN

Core ef78c110fe 移除 Chrony provider 的通用命令执行器。经原时钟事务的准备、提交意图和结果台账，固定 v6 Unix 数据报只派发一次。systemd 监听由 PID 1 持有。独立 relay 为 _chrony、cap0、NNP1，仅向原 clock actor 开放。对端 PID/UID/GID、实际 Chrony 单元、进程出生和 socket 指纹参与绑定。包 cohort 同时核对两份 vendor unit、原助手及新版 Java bundle。原 cohort 仅沿原维护合同核对。

从 Git 构建 JDK 21 和 ARM64 正式包，10 份载荷精确。Debian 13 实际安装后，Main 通过 OSGi ConfigurationService 变更 provider，真实 Chrony 4.6.1 记录一次改时尝试。隔离环境禁止系统改时，原提交回读 UNKNOWN 并关闭写入准入。启动依赖在新现场产生一份 START 终态。正常停 Main 后两份原输入、START 回执和 STEP pending 字节精确，新现场 disabled、正常退出。Linux ARM64 90 项、macOS 89 项通过且 1 项 Linux 专属测试跳过、JDK 21 JVM 23 项通过。

历史 schema 1 static、linked 和 linked-runtime 沿用已验收证据。原三现场、13 个运行中容器和原 7fd 禁用正常停机 41 输入/16 回执已复核。真实正向改时、Chrony 配置写、完整 Core/OSGi/合法 Main、其余 typed brokers 和六组件矩阵继续内部待办。冻结父项 0/12，external deferred 仍为原三项。见 [检查点](actual-typed-chrony-makestep-checkpoint.json)。




## 2026-11-19 原 OBSERVED Root 的合法包升级与运行准入

原 Root 由已装 Agent f054676 升至 7c03712。Debian 先执行旧 prerm，再由已核定新包的 failed-upgrade 和 preinst 分别验证。同一个原 OBSERVED ID、精确候选与已装 bridge、原始租约、已确认的物理头和进程身份保持。Core 328378、NetworkManager 78 未重启，43 份原保护记录及 12 份防火墙文件精确。错误观察 ID 被拒，未发布准入或停机记录。

Agent b5e24b1 让系统 writer 准入直接使用已经验证的 imported 防火墙历史，0f48c7d 增加有限阶段诊断。正式发布 0f48c7d 后，在原 OBSERVED 上独立采样六次，旧状态表拒绝计数从此前 31 条到当前 0 条。实际 Core、NM、原回执及规则保持。ARM64 两个装机版本各 17 项，宿主 94 项和 Debian ARM64 105 项 Python 包回归通过。

同原 JVM 的独立 IPv6 候选完成一次 prepare、apply、显式 rollback，原查询读回历史 ROLLED_BACK。真实 IPv6 候选撤回，之前 IPv4 规则句柄保持，逻辑 revision 2。临时写授权已撤回，中心远程管理关闭。原三现场和原 7fd 禁用正常停机 41 输入 / 16 回执已复核。完整 Core/OSGi、合法 Main、其余 typed brokers 和六组件安装升级移除 purge / 中断恢复矩阵继续内部实施。冻结父项 0/12，external deferred 仍为原三项。见 [检查点](actual-observed-firewall-package-continuation-checkpoint.json)。




## 2026-11-19 原 COMMITTED Root 实际防火墙提交与原 ID 到期恢复

正式 Core JVM 与已安装双族 provider 下，原 IPv4 回环操作 `6b3acbf5` 保持三个原请求 ID，依序完成 PREPARED、WAIT_CONFIRM 与 COMMITTED。独立 outcome 核对原输入、实际内核规则句柄和 revision 2。此前仅等待放行的过期探针缺少发送意图。

Agent `a15fb7f` 将原已确认 Core 的有界读取时限用于同注册 epoch 的 roles 与 writer drain。Debian 13 ARM64 正式 Agent 升级保留同字节 Java bridge，Core/NM 连续，43 份原保护记录和原 UNKNOWN 精确。ARM64 定向 118 项通过。Agent 6de7668 为原 outcome 增加三秒有界排他等待。定向 20 项通过。Agent 0d05072 对 source observation 保留原排他准入且有界等待。定向 15 项通过。

Agent f054676 与 Networking aad948f 对齐 imported 权限证明的固定连接时限，正式配对升级后当前 Core PID 328378，原 NM、43 份记录和实际规则保持。JDK 21 bridge 162 项、ARM64 transport 18 项通过。Agent f8bedd7 普通 imported 排他准入已构建，ARM64 13 项通过。已安装包升级预检拒绝终态 OBSERVED，停服务和安装均未发生。

第二个独立 IPv6 回环候选通过预审和 prepare，apply 只派发一次。调用超时后读取到原 WAIT_CONFIRM 和实际内核效果。现有事务经自动到期回收，由同 JVM 用原请求独立读回历史 ROLLED_BACK，IPv6 回环候选已撤回。IPv4 原规则句柄和历史 COMMITTED 回执、结构化策略与 revision 2 均精确。加载新版 bridge 的正式 Core JVM 查回原 source observation 请求的历史 OBSERVED，实际规则字节和逻辑 revision 2 保持。显式 rollback 的正向验收继续内部待办。临时本地写授权已撤回，当前 Agent PID 349643，撤权期间 Core 与 NetworkManager 进程连续。

历史 schema 1 static、linked 与 linked-runtime 沿用已验证检查点。原三现场实时复核 7 进程、38 私有文件元数据和 6 运行数据文件。原 7fd 保持禁用正常停机 41 输入/16 回执。正常 typed Core 业务、完整 Core/OSGi/合法 Main、RTC/Chrony 等其余 typed brokers 和六组件/provider/launcher/发行生命周期矩阵继续内部待办。12 父项闭环 0/12，external deferred 仍仅原 3 项。见 [检查点](actual-committed-typed-firewall-outcomes-checkpoint.json)。




## 2026-11-19 原 COMMITTED Root 正式 OSGi 防火墙业务预审通过

Agent `3c76d94` 从原密封租约验证历史 CM 源及其原始角色，现行权限使用同一实际 Core JVM 的完整外部 handler 证据。原操作、计划、模板、当前两族确认 revision/digest、原 writer drain、进程/注册、网络和内核代校验保留。旧 CM 捕获与初次迁移逻辑保持原合同。

从 Git 构建 Debian 13 ARM64 Agent。通过同原 Root 已安装的标准包生命周期更新一次 Agent，配对 bridge 来自现场，其字节与已安装 `3723ab2` bridge 一致。Core PID 143427 与 NM PID 78 连续，Agent 为 193534。原 43 份记录、原快照、未结算 UNKNOWN、原 lease / startup ticket 和 native APPLIED 保持，远程关闭。

安装态同 JVM 实际双族 provider 与原 revision 1 的确认源一致。真实 imported 预审返回 REVIEWED，确认同原布局从 114 条到 115 条的回环接口候选。预审的 writeEnabled、dispatchEnabled 和 ownerGrant 均为 false，没有新事务或内核派发。ARM64 三组共 405 项通过、22 项忽略，JDK 21 探针实际运行。

历史 schema 1 static、linked 与 linked-runtime 的合法转换沿用已验证检查点。原三场景 7 进程、38 私有元数据、6 运行数据文件匹配，原 7fd 禁用正常停机 41/16；18 仓同分支干净。实际 forward/confirm/inverse/source observation、完整 Core/OSGi/合法 Main、其余 typed brokers、六组件/provider/launcher/安装升级移除 purge/中断恢复继续内部待办。冻结 12 父项闭环 0/12，仅原 3 项 external deferred。见 [检查点](actual-committed-typed-firewall-business-review-checkpoint.json)。




## 2026-11-19 原 owner 正式 Agent/Java bridge 配对升级与业务源授权定位

在原 COMMITTED Root 从 Git 构建并正式安装 Debian 13 ARM64 Agent `951da5c` 与 Networking `3723ab2` 的 Java bridge。原 Root 原 owner 的维护记录收齐五个独立结果，实际执行一次 Core 正常停止、Agent 停止、同包升级、Agent 启动及 Core 启动。Core PID 143427、Agent PID 143112、NM PID 78。确认实际 OSGi bridge 已加载，43 份原记录指纹完全一致，原 lease、native APPLIED、快照及未结算 UNKNOWN 保留，远程关闭。

实际 JVM 读取双族 typed provider，确认原 revision 和 source digest。布局与绑定原计划的双入口均通过编译预审，正式 imported review 和 source review 均返回 UNSUPPORTED。真实 JVM 内可验证已安装 Java executable，外部 procfs 仍拒绝读取。现行 roles、fence、owner 回调返回 OK。原两族 ConfigAdmin PID 均不存在，Core 的旧 stored revision 接口拒绝已外部保留的 PID。当前 imported 源授权仍调用旧存储路径，需接通现行外部 typed 权限及原始密封源证明。现场没有产生新防火墙事务。

ARM64 Rust 62 项通过、1 项忽略，JDK 21 bridge 161 项通过。原三现场及原 7fd 禁用停机 41 输入/16 回执保留。完整 Core/OSGi/合法 Main、其余 typed brokers 和六组件/provider/launcher/安装升级移除 purge/中断恢复继续内部待办。冻结父项闭环 0/12，external deferred 仅原三项。见 [检查点](actual-imported-firewall-paired-upgrade-checkpoint.json)。




## 2026-11-19 原 owner 的双族 typed 业务读取与 JVM 可执行身份

在原 `COMMITTED` 现场，通过同一实际 JVM 的正式 OSGi 防火墙 provider 读取 IPv4/IPv6 typed snapshot、schema 4 candidate 和同一确认源。正式 imported review 在 Java 探针修正本地时限参数后到达已装 Agent，返回 INTERNAL，现场无防火墙 transaction。实测外部 Root 与完全相同 UID/GID 均无法读取 JVM 的 `/proc/<pid>/exe`。原 lease 与实际 native 回读仍可验证，Agent 本地和远程写授权均未开启。

Networking `3723ab2` 复用已有普通启动回调的扩展查询，由真实 JVM 读取自己的内核 executable 身份。旧请求和注册清单保持兼容。Agent `951da5c` 仅在外部 procfs 权限拒绝时调用，核对 Unix 对端 PID 与注册 epoch，独立绑定实际进程出生及 Root 保护安装路径、device/inode/size/ctime，并保留原 unit、profile、租约和启动证据复核。纯所有权 status 使用既有有界共享锁。ARM64 Rust 62 项通过、1 项忽略；JDK 21 bridge 161 项通过。

实际同 JVM 测量及独立 Root 校验一致。候选桥与 Agent 尚待正式同 Root 配对部署及真实业务写入验收。原三现场、现有进程、原 lease、密钥、快照、原 UNKNOWN 与 7fd 禁用停机 41 输入/16 回执保持。完整 Core/OSGi/合法 Main、拆分 broker 与六组件矩阵仍内部待办，冻结 12 父项 0/12，external deferred 仅原三项。见 [检查点](actual-imported-firewall-executable-observation-checkpoint.json)。




## 2026-11-19 原初始化 Root 受控停止、普通 owner 启动与实际 OSGi 确认

Agent `530f999`、`f027f33`、`3edd3f5` 将 schema 5 接到正式已装 Root 入口。操作使用同一原 ID、实际已装包链、固定 systemd 单元、原 Root 和原始授权。停机前重新采样当前 Agent Unix 对端、旧 JVM 注册、原 Netfilter 和已提交初始化；停机后仍核对同一 Root/NM/Agent，按原序列发布和启动。被隔离的 Agent 通过正式 Root reader 只读核对两道原系统 fence。新 JVM completion 要求实际已确认启动及新的 Java/OSGi 证据。

从 Git 构建正式 Debian 13 ARM64 deb 并按原计划升级，原 JVM 与 NM 在 Agent 替换期间保持。原 ID 实际完成一次 `STOPPED`、一次 `FILES_PUBLISHED` 和一次 `STARTED`，均独立读取同一原回执。新 JVM 通过实际普通 profile 与 OSGi writer 核验，正式同 ID 所有权为 `COMMITTED`。独立复核实际 JVM 加载已发布参数文件，NetworkManager 后端 active，原 native Netfilter 仍 `APPLIED`，实际终结和 lease 摘要一致，原启动参数字节和 inode 保留。原快照、UNKNOWN 回执、原三现场及 7fd 禁用停机 41 输入/16 回执保持。

ARM64 startup 与 continuity 共 61 项通过、1 项忽略。此次完成同 Root 合法 owner 切换，完整 Core/OSGi/合法 Main、其余 typed brokers、六组件/provider/launcher/安装升级移除 purge/中断恢复继续内部待办。冻结 12 父项最终闭环仍 0/12，external deferred 仅原三项。见 [检查点](actual-original-legacy-startup-activation-checkpoint.json)。




## 2026-11-19 原旧 JVM 启动子事务的类型化证据与现场准入

Agent `bcedd4c` 增加启动子事务 schema 5，固定旧 JVM 回执的文件设备号、inode 与摘要，核对父计划、systemd JVM 身份、已签发 profile、原角色和 stop policy。schema 5 的 stop intent 区分旧 JVM 证据与普通 Java report。现有停止、停止后文件发布和未结算结果的门禁复用。隔离回归验证只停止一次、按原记录发布文件；丢失停机应答保留 `STOP_OUTCOME_UNKNOWN`，禁止从后续 inactive 状态合成回执或重新派发。`a7367c2` 恢复历史 schema 2 在任何 unit 探测前拒绝；原字节保持。

原 Debian 13 ARM64 现场从 Git 构建候选二进制，实际核对已安装 Agent Unix 对端、同原 ID 注册、原 JVM/NM、已提交初始化、原 Netfilter、签发 profile 与停机定义。现场返回 `LEGACY_STARTUP_REVIEWED`，给出后续 JVM 参数和 profile 摘要以及 1 个保留、2 个待创建的 fence。错误原操作 ID 被拒。未写入有时效的 proposal 或任何 startup intent，未安装候选；原 Core/NM/Agent 和 17 份原记录、原快照与 UNKNOWN 保持。

最终 ARM64 startup 回归 59 项通过、1 项忽略，continuity 2 项通过。实际 Root 受控执行入口、Core 停止、停止后发布、owner 启动和 OSGi adoption 继续内部待办。完整 Core/OSGi/合法 Main、其余 typed brokers、六组件/provider/launcher/安装升级移除 purge/中断恢复继续。原三现场和 7fd 禁用停机 41/16 保持。冻结父项闭环 0/12，external deferred 仅原三项。见 [检查点](actual-original-legacy-startup-child-review-checkpoint.json)。




## 2026-11-19 原初始化 Root 正式 Agent 升级与实际旧 JVM 绑定

Agent `5a3f852` 支持同一 Root 的原 PREPARED 升级证明续接到 FENCED、native APPLIED 和初始化 COMMITTED。续接重新校验原 plan/加密回执、双 fence、已结算原应用批次与当前真实 Netfilter；首次候选在记录和停止 Agent 前明确拒绝。现场回读定位到旧 JVM 和 NetworkManager 的 procfs inode 分配变化。 `76bacc9` 保留单次采样的文件身份核对；跨期比较要求原证明有同一 PID 命名空间，再核对实际 PID、启动时刻、凭据、systemd InvocationID、cgroup 和 unit。没有命名空间的历史凭证仍用原严格比较。

原 Debian 13 ARM64 Root 以同一个原 ID 通过正式 `dpkg` 升级，随后显式启动一次已安装 Agent。实际 Unix 对端通过同 JVM 注册核验。经 Root 本地入口对当前 JVM、已密封的 12 份普通启动 profile 输入、原角色和原初始化出具独立 `LEGACY_JVM_BOUND` 回执。原 ID 读回与受保护文件摘要一致。错误 token 在写入前拒绝。现行 Core 与 NetworkManager 进程保持，原 17 份记录、旧包证明、数据、原快照和 UNKNOWN 回执均保持，远程维持关闭。

本次未发出 Core 停机或 owner 启动。后续需在原 startup 子事务内消费独立旧 JVM 证明，落实原 Root 的停机、文件发布和实际 owner 启动。完整 Core/OSGi/合法 Main、其余 typed brokers、六组件/provider/launcher/安装升级移除 purge/中断恢复继续内部待办。ARM64 定向 19 项通过、2 项忽略。全 handover 套件 254 项通过、16 项忽略。Debian 打包源码与 115 项已验证版本一致。原三现场及原 7fd 禁用停机 41 输入/16 回执保持。冻结父项闭环 0/12，external deferred 仅原三项。见 [检查点](actual-original-legacy-jvm-startup-binding-checkpoint.json)。




## 2026-11-19 原初始化 Root 普通启动 profile 签发

Agent `936ddd4` 识别现行 Core 的受密钥服务管理的 packaged-background 启动链，并将密钥包装器及凭据读取脚本纳入固定输入。普通参数文件仅接受附加 JVM 参数。新 Root 本地签发入口核对原 FENCED 计划、该 Root 的实际升级/停机记录、发布凭证、已提交初始化文件、当前真实 native 回执和原 JVM/NM 进程，再密封指定已装 Core 单元、实际有效定义、stop policy 和输入文件。`64d1eb9` 以现有全局锁作有界排他等待。首候选因锁争用在签发前已知拒绝，无原 profile 文件生成。

原 Debian 13 ARM64 Root 以原 ID 实际布置仅包含三个附加 JVM 选项的受保护参数文件和一份 systemd drop-in。一次 daemon-reload 后，采用新 Git 构建 deb 所提取的精确签发组件发布 Root 私有 `nm-ownership-profile.json`。签发绑定现行凭据管理启动链、12 份固定文件及原实际 Netfilter/初始化回执。独立读回 systemd 定义和所有文件一致。原 Core/NM/Agent PID、快照和原 UNKNOWN 记录保持。当前运行 Agent 包仍为 `ed4a2d5`，本轮候选为单独 Root 签发工具。

当前真实 JVM 启动时没有普通 profile 标识，后续实际启动需用原合法 JVM/Root 证明单独处理旧态路径。受控停止、发布、owner 启动、完整 Core/OSGi/合法 Main、其余 typed brokers 和六组件/provider/launcher/安装升级移除 purge/中断恢复继续内部待办。ARM64 定向 15 项、Debian 打包回归 115 项。原三现场和原 7fd 禁用停机 41 输入/16 回执保持。冻结父项最终闭环 0/12，external deferred 仅原三项。见 [检查点](actual-original-nm-live-startup-seal-checkpoint.json)。




## 2026-11-19 原 PREPARED 同 Root 完成双 fence、原生批次和初始化

Agent `9cbdba3` 将后续 Debian 包候选的凭证按实际二进制摘要存放。正式升级核对当时已安装的前任版本、其私有计划和停机回执、原 JVM/NM 与原 Netfilter 来源。历史记录保持原字节。Root arm 保留原计划中的 Agent 注册，另记录实际新 Agent 注册与原升级摘要。正式 Root publisher 校验安装包和原计划、两个注册绑定之后，才写两道物理 fence。`ee3046e` 在原 FENCED 计划的后续 native/初始化入口继续严格核对同 Root、原 JVM 的当前 Unix 注册、真实 Java writer 排空、原 CM/NM 和不可变计划。`ed4a2d5` 在续接预检使用既有共享锁作有界等待读取。首个候选在非原 ID 的预检遇到锁争用，只增量落盘候选文件，原操作未进入停机或派发。

原 PREPARED Debian 13 ARM64 Root 使用 `ed4a2d5` 正式生成并安装 deb。链式凭证与旧 `87bbf02` 记录精确匹配。受控启动后通过标准 UDS 核验原 JVM 注册。按同一原 ID 从正式 systemd 单元派发一次，获得真实双 fence、schema 2 的 arm 和完全相同的唯一 dispatch。随后同一原操作 Netfilter 一次批次返回 APPLIED，初始化一次返回 COMMITTED。另以原 ID 经两个正式只读入口核对原内核回执和实际初始化文件摘要。旧计划和六份加密输入/回执、原快照、Core/NM 进程、配置和原配置 UNKNOWN 回执均保持。远程仍关闭。另一原 FENCED Root 仅读取核对原计划、内核和 JVM/NM 身份，保留其已安装版本和现有状态。

`ee3046e` ARM64 相关四组 79 项通过，`ed4a2d5` ARM64 定向 16 项、相同打包源码 Debian Python 115 项通过。当前已初始化 Root 的普通启动还没有正式安装启动 profile。后续普通 Core/OSGi owner 启动、签发、完整合法 Main、其余 typed brokers 与完整六组件/provider/launcher/安装升级移除 purge/中断恢复继续实施。原三现场、7fd 禁用停机 41 输入/16 回执和全部 12 父项保持。最终闭环 0/12，external deferred 仅原 3 项。见 [检查点](actual-publication-and-initialization-of-original-nm-checkpoint.json)。




## 2026-11-19 原 NM 计划期间的正式 Agent 包升级和同 JVM 注册

Agent `99b46b0` 将 Root 固定原操作、当前 Agent 与候选二进制和原版本的绑定接入 Debian 正式升级入口。旧 prerm 对活动守护继续拒绝。新包仅在原 Root 私有停止记录可验证、其余生命周期门禁满足的情况下处理 failed-upgrade。Stop intent 持久化一次，preinst 核对原进程实际已停，同一 JVM/NM、全量 Netfilter 源、密文、计划与 fence 再校验，然后允许解包。实际 postinst 保持守护停止，后续以原正式 systemd 单元显式启动。启动后的 Root 本地核对使用实际 Unix 对端 PID、实时 CM 角色、stored firewall 与完整 writer 模型。

原 FENCED 的 Debian 13 ARM64 现场正式升级到 `99b46b0` 并启动。原 Core/NM、12 份原记录、数据库身份与远程关闭保持，正常安装版 UDS 同 ID 回读 FENCED。新注册证明返回 CONFLICT。已在其真实 Root 对比证实 procfs 在受保护 systemd 沙箱内重挂后，同一进程的 procfs device/inode 与宿主视角不同。保留这次正式安装及第一次启动。原现场此次没有应用修正版。

Agent `87bbf02` 在同一视角保留对 procfs 文件身份的核对，跨挂载使用实际 PID 命名空间、systemd InvocationID/cgroup、PID、启动时刻和凭据，既有共享锁仅在有争用时作有界等待读取。另一原 PREPARED Root 沿其原 ID 用修正版 ARM64 正式 deb 升级并显式启动，实际 Agent 的内核对端 PID、原 CM 角色、存储防火墙和 writer 模型均通过核验。同 Root 读回 PREPARED，原 Core/NM、8 份原文件、快照、数据和原 UNKNOWN 回执保持。ARM64 Rust 18 项、Debian 包测试 115 项通过。两边均未派发原生或 owner 写入。

原 FENCED 的修正续接、原 PREPARED 的后续 Root 派发、签发、初始化、实际 owner 启动继续内部待办。完整 Core/OSGi/合法 Main、其余 typed brokers 与六组件/provider/launcher/安装升级移除 purge/中断恢复及全部 12 父项维持原范围。原三现场和原 7fd 禁用停机 41 输入/16 回执保持。最终闭环 0/12，external deferred 仅原三项。见 [检查点](actual-paired-nm-package-upgrade-checkpoint.json)。




## 2026-11-19 两个原 NM 现场的 Agent 续接准入检查

Agent `ba636ed` 新增 Root 本地命令 `inspect-original-nm-continuity`。在已有共享锁下校验原 ID、Root owner、计划、加密传输完整回执与原来源。PREPARED 须无原派发记录且 fence 仍保持原样。FENCED 须有与原计划严格绑定的 arm/dispatch 和两道真实 fence。已派发未决的动作、startup 子事务及矛盾结算记录拒绝。检查同时取原 Kura、Agent、NetworkManager 的 systemd InvocationID 与内核进程出生凭据，以原计划及当前真实 nftables JSON/raw dump 校验全量源并生成本代候选 batch 摘要。入口只读。

从 Git 构建 ARM64 deb，以包内二进制在两个原 Debian 13 Root 各执行原 ID 检查。原 FENCED 经过配对 systemd Root 只读 socket，原 PREPARED 读取本地实际 fence 缺席。两边真实 Netfilter generation 与计划验证通过，各拒绝随机非原 ID，分别保持 12 和 8 份原文件、原进程及日志。定向 Rust 原生套件通过 10 项，另 2 项需专用夹具。候选仅增量落盘用于检查，原已安装守护保持，未派发新原生批次。

仍需正式 Agent 受控替换、以实际新注册核验原 JVM/CM/NM 绑定，再继续签发、初始化和实际 owner 启动。完整 Core/OSGi/合法 Main、其余 typed brokers 与六组件/provider/launcher/安装升级移除 purge/中断恢复继续内部待办。原三现场及 7fd 停机 41 输入/16 回执保持。冻结 12 父项仍 0/12，external deferred 仅原三项。见 [检查点](actual-nm-root-continuity-review-checkpoint.json)。




## 2026-11-19 原 FENCED 的配对 Root 只读证明

Agent `c835d15` 新增 legacy Debian 的 socket-activated Root 读取单元。文件路径固定，socket 为 root:root 0600，并检查对端凭据。每次读取核对包来源、实际 systemd unit、Core loader 与 Java bridge，拒绝别名、硬链接和变更中的文件，只返回两道 marker 的设备号、inode、SHA-256。沙箱中的 Rust 读取方核验 PID 1、socket 身份和原计划摘要，共享已有执行锁，并保持原 Root 的路径和写入权限。

在原 Debian 13 ARM64 `FENCED` Root 上从候选 deb 仅增量放置四份读取单元文件。正式 socket 经 systemd 启动但未 enable。以候选二进制和正式只读挂载验证同 Root 同 ID 返回 `FENCED`。无 socket、外来 ID 和非授权 UID 均被拒。12 份原 marker、计划和加密回执的身份与摘要、快照、旧版 Agent 以及原 Core/NM 进程保持。实际 JVM 只读网络角色和完整 writer 模型匹配原计划。现装 `bc8e1ea` 主守护仍返回 `CONFLICT`。尚未把新读取逻辑部署到该守护，没有派发原生 netfilter 写批次。另一原 Root 用原 ID 仍可读出 `PREPARED`，五份加密文件指纹匹配，原两道 fence 仍不存在。

ARM64 Rust 定向通过 19 项，另需隔离挂载或原 Java writer 夹具的 2 项保持忽略。Debian 包 Python 定向 95 项通过。正式守护在原操作期间的安全续接、签发、初始化和实际 owner 启动继续列为内部待办。完整 Core/OSGi、合法 Main、其余 typed brokers、六组件/provider/launcher/安装升级移除 purge 及中断恢复矩阵继续实施。原三现场和 7fd 停机 41 输入/16 回执保持。父项 0/12，external deferred 仅原 3 项。见 [检查点](actual-paired-root-fence-readback-checkpoint.json)。




## 2026-11-19 正式 Root 双 fence 发布及 Java writer 排空

Agent `bc8e1ea` 发行独立 `kura-management-nm-fence@.service`。Root 发布单元核对包内 loader、bridge、已装二进制、控制脚本、来源及实际 unit；用受保护 UDS 和进程凭据绑定当前 Agent，并比对双方运行二进制摘要。原注册的 JVM/CM/NM/原生来源重新核验，在同一事务内按原 ID 留下 arm 和唯一发布意图。Root 从固定路径发布两道原 fence，用真正文件设备/inode/摘要结算。无共享 Root-only 数据目录重新映射，无自动启用单元。

新隔离 Debian 13 ARM64 以 Core `e952f82bb9`、Networking `180ed6e`、Agent `bc8e1ea` 验证。JDK21 正式 Core 双族 CM 一次落盘，原 Java 加密准备一次进入 PREPARED。实际发布单元先拒绝同 Root 的全新非原 ID，原记录保持；随后正式 Root 单元对原 ID 启动一次，成功 FENCED。实际两道原 marker 权限、文件身份、计划和加密回执核对通过。真实 OSGi writer 证实双族防火墙及 NM writer 排空。原生内核 handle 保持，尚无初始化或首个 netfilter 写批次。Agent 自己继续在只读 /etc/kura 沙箱，查询该 FENCED 结果返回 CONFLICT，待接独立配对的只读证明。ARM64 Rust 15 项、Debian Python 包 92 项通过。

此前独立 PREPARED 原现场仍运行，数据、原编号和两道空 fence 保留。配对只读证明、该旧原计划的安全续接、签发、初始化、真实启动、其余 typed brokers、完整 Core/OSGi/合法 Main 及六组件矩阵继续实施。原三现场与 7fd 41 输入、16 回执保持，12 父项仍 0/12，external deferred 仅原 3 项。见 [检查点](actual-typed-root-fence-publisher-checkpoint.json)。



## 2026-11-19 双 fence 派发前准入及原 NM 维护记录保全

Agent `232945a` 将真实目录只读检查和外来 network marker 冲突移到 FENCE_INTENT 前。原 actor、token、注册和双族来源见证继续核对。真实 ARM64 systemd 测试用 `TemporaryFileSystem=/etc/kura:ro`，确认拒绝时原 PREPARED 计划及两个空 fence 位置保持。首次探针因 /run 本身 noexec 返回 203，随即从可执行的隔离路径通过；原操作未派发。ARM64 handover 247 项、独立 bind mount 1 项、systemd 1 项；macOS 外来 marker 1 项通过。

Agent `7be3275` 将原 NM ownership 和加密模板流水接入 legacy 包维护准入。未结算或孤立原记录拒绝；正常结算仍核对双侧的原 ID、计划、收据和租约摘要。purge 前保护原交接证据。在原隔离 Root 上只读执行新 ARM64 包提取的真实 preinst 检查，原 PREPARED 拒绝，8 份原计划/分片/回执的内容和元数据精确。当前安装仍为 `8033e43`，原 Core/NM/Agent 进程保持。Debian ARM64 Python 包测试 91 项通过。

配对 Root publisher 和原计划续接、实际两道 fence/签发/初始化/启动、其余 typed brokers、完整合法 Main/OSGi 业务及六组件生命周期仍为内部 pending。原三现场、个人配置、运行中 runtime、原 7fd 停机 41 输入/16 回执继续保全。冻结 12 父项仍 0/12，external deferred 仅原 3 项。见 [检查点](paired-root-fence-admission-checkpoint.json)。



## 2026-11-19 原 JVM 加密导出与单次实际 PREPARED

同一隔离 Debian 13 ARM64 Root，沿执行库正式升级 Agent `8033e43`。保留已注册 Java 桥及 Core loader、同一 Core 和 NM 进程、两份快照及原配置收敛 UNKNOWN 记录。通过实际 OSGi CryptoService 确认原部署密钥可用，仅读取布尔结果。私有 NM 配置显式授予该隔离现场的本地 Root 执行器，接口范围只含 lo，远程关闭。

Agent 在升级后重新配对实际 Java 桥，从新预览沿原双族存储和 114 条原生规则绑定，执行一次正式 prepare。真实 Java 生成 1 个 1025 字节自定义密钥 AES-GCM 密文分片。Agent 返回 PREPARED。同 Root 同 ID 原始查询仍为 PREPARED，分片、manifest、原回执和两侧计划文件权限及指纹精确。实测已安装 legacy Agent 的 /etc/kura 挂载为只读，两道原 fence 均保持不存在。后续必须由配对的 Root 发布路径继续原计划，不能改变沙箱权限归属。

原三现场、C/D/E、个人配置、数据和 7fd 停机 41/16 保持。完整 Core/OSGi/合法 Main、其余 typed brokers、六组件/provider/launcher/安装升级移除 purge/中断及全部 12 父项保持内部台账。最终闭环 0/12，external deferred 仅原 3 项。见 [检查点](live-encrypted-firewall-prepare-checkpoint.json)。



## 2026-11-19 双族正式 CM 基线与原生防火墙准备预览

Networking `07b00a6` 在桥连接注册时声明原 writer 读取能力，读取时核对当前 provider。Agent `20724a5` 在无原 writer 模型时拒绝为未托管防火墙准备委派，`8033e43` 保留配对防火墙读取的 CM CONFLICT。实际 Debian 13 ARM64 以 JDK 21 从 Git 构建 Core `e952f82bb9`、Networking 和 Agent。正式 Core 条件批处理仅调用一次，紧随其后的读回遇到 CM 收敛门禁，原结果保留 UNKNOWN。同一 Root 独立只读证实两族 stored CM 与实际 writer 一致、原快照保留且新增一份快照。该调用未重发。

原生只读诊断定位到 IPv6 filter input-kura 模型序列歧义。实际威胁防护另提交了与内置规则完全相同的 38 条 IPv6 ICMP ACCEPT，两份均进入内核。Networking `180ed6e` 在内置 ICMP 启用时省去完全相同、位于其后的附加规则。未启用内置 ICMP 时仍刊布附加规则，其他威胁规则顺序保留。真实包在同一隔离 Root 升级，38 条内核规则均由两份变为一份。Core 按正式单元启动，Agent、NetworkManager 进程及执行库 device/inode 保持。前后两份快照、原日志与两族规则精确，存储 revision 随 Core 重启重新计算。

实际 Agent 原源审查返回 `legacy.raw.source.v1`，同源准备预览返回 `legacy.native.source.v1`。114 条原生规则唯一匹配，43 条未选规则保留。两个读入口配对同一 stored CM 和分类摘要。状态为 UNMANAGED，远程关闭，原布局保留，后续需要初始化转换，尚无签发、fence、Java 加密导出或所有权派发。Networking JDK21 定向 99 项，Agent 原安装版与冲突回归共 30 项通过。

原三现场、C/D/E、个人配置、数据及 7fd 停机 41/16 按原指纹保持。后续签发及启动、其余 typed brokers、完整 Core/OSGi/合法 Main 和六组件矩阵为内部 pending。完整 12 父项 0/12 最终闭环，external deferred 仅原 3 项。见 [检查点](stored-dual-firewall-preparation-checkpoint.json)。



## 2026-11-19 真实 JVM/NM 旧源只读审查

Agent `09c349d` 将本地 ownership status 和原源审查接入 NM `writeEnabled=false`。沿用原 Root 目录、同一系统锁、已注册 JVM 和现有 CM/内核核对。观察路径不建写事务目录。prepare、原动作结算、cancel、fence、activate 和 Core 启动动作仍要求独立写执行器。相关 ARM64 Rust 23 项通过。

新隔离 Debian 13 ARM64 在 Core `e952f82bb9`、Networking `d949ec1` 和正式 legacy-profile Agent deb 上验证。沿包自动启动已建立的执行库 device/inode 配置观察模式，实际 Java 桥注册，NetworkManager D-Bus 可读，ownership status 为 UNMANAGED。初次原源只读在 NM DS 就绪前为 NOT_FOUND。同一 Core JVM 的真实 NM writer ACTIVE 后，返回 `legacy.kernel.source.v1`，见证两个固定 PID 的 stored CM 投影和真实内核双族 iptables。该现场两族 PID 尚无原存储配置，审查标记 preparationReady=false，单独的准备预览返回 INTERNAL 原码。均为只读采集，未派发 ownership 建计划、加密导出、持久 fence 或委派启停操作。观察现场继续保留运行。

真实正向原源 prepare、两族合法原存储、原签发与启动、其余 typed brokers、完整 Core/OSGi 和六组件矩阵继续内部 pending。原三现场、C/D/E、个人配置、数据和原 7fd 停机 41 输入/16 回执保持。冻结父项最终 0/12，external deferred 仅原 3 项。见 [检查点](live-readonly-nm-source-checkpoint.json)。

## 2026-11-19 typed firewall provider 进入正式 Networking 包

Networking `d949ec1` 将 typed facade 作为 Root 所有的 0644 bundle 安装到既有 `5s`。Core loader 和原 Agent Java 桥配对指纹保持。可选 Java 包导入允许独立 Core 解析该 bundle。选择 typed profile 时，两族服务需要真正的 Java 桥服务和 NM writer observer。已安装的旧 bundle 要经过两项原 fence、实际 writer drain 和独占 facade 检查。

同一个新 Debian 13 ARM64 现场从 Git 构建 Core `e952f82bb9`、Networking `d949ec1` 和 Agent `ff36191`。未装 Agent 时及装入精确配对 Java 桥但 Agent 停机时，真实 OSGi 两族 legacy facade 均 ACTIVE，typed facade 留在未满足状态。后者确认 typed bundle 真实导入该配对桥。隔离测试临时选择 typed profile，原所有权未签发且无 fence，两族均在 NM writer 检查因找不到 durable network owner fence 拒绝激活，legacy 同时被抑制。测试选择文件已移除。

正式移除和 purge 清除 typed bundle，Core 钥、快照及配对桥原指纹精确，NetworkManager 原进程保持。其后实际 Core 和配对桥再启动，旧和 typed firewall bundle 均不存在。测试现场正常退出保留。JDK21 相关 66 项通过。原三现场、C/D/E、个人配置和 7fd 41 输入/16 回执保持。见 [检查点](packaged-firewall-provider-checkpoint.json)。

原真实 JVM/NM 签发与正向所有权过渡、完整 Core/OSGi 业务、Native 其余 typed brokers、六组件/provider/launcher/安装升级移除 purge 和中断恢复矩阵继续内部 pending。冻结 12 父项最终 0/12，external deferred 仍仅原 3 项。

## 2026-11-19 真实 Core、Networking、Native Agent 本地配对

新 Debian 13 ARM64 使用 JDK 21，从 Git 构建并实际安装 Core `e952f82bb9`、Networking `4b58a55` 和 Native Agent。首个本地配对被旧守卫拒绝。只读核定原编号为 NOT_FOUND，组成员未变化、两单元均停止。正式安装布局将 Networking 包内 Root 模板复制到 vendor drop-in。有效 Main 原已有该模板声明的两项网络能力。

Agent `9abe64a` 和 `ff36191` 对该布局增加 schema 2 原授权。Root 预启动核对 Networking 包归属、已配对 Core 版本目录里的原模板、实际 vendor 文件和 systemd 有效能力。仅接受这一份已审查的额外声明，要求两项能力都已在原 Core 单元生效，拒绝其他 drop-in、变化字节和新增能力。Native 进程校验可见的实际 vendor 文件，无需读取 Core 私有目录。原 schema 1 独立 Core 仍按原合同读取。

同一新 Root 用原初始化编号完成一次受控源码升级到 `ff36191`。原初始化、continuity、执行库及 plan key 指纹精确。新配对具备独立原记录，随后正式单元一次启动完成实际 Java/OSGi 桥注册，NetworkManager 保持原 PID。Native Agent 零 capabilities、NNP1、远程关闭。Core Main 的 systemd 声明为 9 ambient、11 bounding、NNP0。此 Docker 现场受外层 capability 围栏与 NNP 约束，Java 进程实测 5 ambient、7 bounding、NNP1。最终版相关 Rust 16 项和 Python 40 项通过。运行中 OSGi 确认 Core 配置、NM、Bridge 三 bundle 均 ACTIVE，实际 NetworkWriterStateService 已绑定，NM 配置 DS 组件 ACTIVE。现场继续用于后续真实业务。

完整 ordinary Core/OSGi 业务、真实 JVM/NM 所有权签发和过渡、可选 typed firewall provider 发行映射、其余 typed brokers、已授权 v1 现场后装 Networking 的明确联合变更及六组件矩阵仍内部 pending。原三现场、C/D/E、个人配置和原 7fd 停机 41 输入/16 回执保持。冻结 12 父项最终 0/12，仍仅原 3 项 external deferred。见 [检查点](installed-core-networking-native-peer-checkpoint.json)。

## 2026-11-19 imported 共享防火墙链早启与事务链重绑

Agent `c53279c` 和 `3e3fb75` 增加共享布局的正式早启恢复。原 schema 5 配对、Root、租约、原证据和逻辑版本继续逐项验证。Agent 先核对独立服务恢复的全部声明、外部规则顺序和原始 Netfilter 表达式。schema 3 意图同时固定实际外部基线和单次派发批次，内核世代作为写入围栏。只使用本次启动的外部 handle 插入或追加原 typed 规则。读回重新计算外部基线摘要，独占布局仍要求完整独占镜像。

新 Debian 13 ARM64 systemd 在同一专属 Root 启动。独立 nftables 服务先恢复 6 表、58 链和 2 条 `numgen` 规则。正式 Agent 单元在 network-pre 前补回 144 条 Kura 规则，单次内核世代 2→3。全部声明与 2 条外部规则保留物理 handle，外部规则两侧的 Kura 规则按原序恢复。正式重绑产生 `BOOT_RESTORE` 链接并保持逻辑版本 1、原配对文件和完整规则，现场正常停放。原初始化的 JVM、NM 和源内核见证使用测试夹具。此现场部署正式 ELF 和 vendor unit，完整发行安装仍待验证。

真实 JVM/NM 签发、原共享恢复缺失或负回执核定、共享重绑后普通 typed 更新、完整 Core/OSGi、其余 typed brokers 和六组件矩阵继续内部 pending。原三现场、C/D/E、个人配置与 7fd 停机的 41 输入/16 回执保持。冻结 12 父项最终 0/12，external deferred 仍仅原三项。见 [检查点](firewall-imported-shared-boot-checkpoint.json)。
## 2026-11-19 imported 早启回执进入事务链

Agent `43c1ebc` 增加固定 Root 的 `firewall-rebind-imported-boot`，Agent 启动恢复也走同一绑定实现。它从原已完成 schema 2 早启回执验证 schema 5 租约、原源摘要、初始化锚、实际 boot/netns、内核世代和完整原生读回。`BOOT_RESTORE` 链接以物理文件身份和摘要固定原回执及前序，保留既有逻辑版本，发布新的物理源摘要和已验证 handle。已有事务历史及原回执保留。新事务不得重用初始化操作 ID。Agent `4aec38a` 保留旧版逐命令 ACK 的精确摘要校验，原负回执必须与其原派发意图使用同一编码。新派发仅使用批次结束 ACK。

新 Debian 13 ARM64 systemd 容器从固定 Git ELF 签发真实格式的原租约，正式早启恢复独占 `ip`/`ip6` 布局。用正式命令接入事务链后逻辑版本仍为 1，原规则、原授权文件及早启回执的指纹精确。然后同一 Root 一次完成新转发端口的 typed prepare、内核 apply 和 confirm。实际 Netfilter 世代从 2 到 3，形成新规则回执及逻辑版本 2，重绑链接和早启回执的物理文件身份和内容都保持。初始化、JVM/NM 见证和后续 Core 注册授权为测试夹具，正式读回、重绑、事务持久化及内核效果真实。新现场正常停放。88 项 Agent 包测试通过。固定 ARM64 release ELF 的并行全量为 776 通过、2 失败、49 跳过。两项失败都触发原启动执行锁冲突，沿用同一 ELF 的串行定向复测两项及旧编码合同均通过。

真实 JVM/NM 签发与正式安装态业务、共享链先恢复原所有者基线、缺原或负回执的早启核定、完整 Core/OSGi 及六组件矩阵继续内部 pending。原三现场及 C/D/E、个人配置、7fd 停机 41 输入/16 回执保持。冻结 12 父项仍 0/12，仅原 3 项 external deferred。见 [检查点](firewall-imported-boot-lineage-checkpoint.json)。

## 2026-11-19 imported 防火墙正式早启派发与内核回执

Agent `6ad4667` 将独占 imported `ip`/`ip6` 布局接到正式 `firewall-restore-confirmed` 和既有可选 systemd 单元。固定 Root、原 schema 5 配对租约、原结算策略头、初始化回执摘要、重新编译的镜像摘要及实际新 boot/netns 进入 schema 2 意图。空命名空间和所需转发状态在原锁内核对。落盘后只派发一个世代围栏批次。收到批次结束 ACK 并完成声明、规则顺序和原生 payload 读回后，写唯一原回执，保留完整 canonical 内核观测。schema 1 可继续读取。独立零 capabilities CLI 夹具证实缺回执的 schema 2 原意图保持 UNKNOWN，派发和确认均拒绝，锁及原数据精确。

本次在新 Debian 13 ARM64 容器，用当前 Git 构建的正式 Agent ELF 和 vendor unit，沿专属持久卷的同一 Root。用测试见证签发正式 schema 5 租约的采集命令超时。未重发原命令，进程退出后用正常只读命令在原 Root 核定已提交租约、原 native APPLIED、初始化 COMMITTED 和启动完成回执，再显式启用单元。随 PID 1 正常退出及重启一次，单元在 `network-pre.target` 前恢复。读回为 6 表、58 链、144 规则，内核世代 1→2。唯一原回执保留全部 144 个 raw 规则指纹及载荷。十份原授权文件的内容和物理文件身份精确。末次正常停放首次观察超时，沿原退出任务确认 PID 1 结束且退出码为 0。15 项定向 Rust 回归及 88 项 Agent 包测试通过。

旧 JVM、NetworkManager 见证以及初始化时的 kernel 源数据为测试夹具。启动恢复使用正式入口及真实内核。本次直接放置正式 unit 与 ELF，未在该现场安装 deb。新物理 epoch/handle 接入后续正常事务、共享链基线、原始无回执或负回执核定、真实 JVM/NM 签发、完整 Core/OSGi 和六组件矩阵继续内部 pending。原现场、C/D/E、个人配置及 7fd 原停机 41 输入/16 回执保留。冻结 12 父项最终 0/12，仅原三项 external deferred。见 [检查点](firewall-imported-early-actor-checkpoint.json)。

## 2026-11-18 imported 防火墙早启预检与真实内核批次

Agent `086dbd2` 增加固定 Root 的 `firewall-restore-imported-review`。预检取已提交的 schema 5 配对租约、原运行态物理 epoch 和结算后的原策略头，按 typed 模型生成 `ip`/`ip6` 的 filter、nat、mangle 表及规则。只读共享锁下复核原租约和文件身份，命令输出镜像指纹及对象计数。共享链中的未授权规则、额外对象、base hook 漂移、缺失原证据及未决日志均阻止生成整表计划。镜像不创建历史回执表或历史规则标记。正式 release CLI 在 capabilities 为 0 的独立空现场拒绝缺租约和 Root 参数，原锁及目录指纹精确。另一个 capabilities 为 0 的夹具在查找 owner 前保留待确认原意图的 UNKNOWN，原意图和锁指纹精确。

Agent `557f7b3` 修正 Netfilter 大批次的成功 ACK 请求。批次结束时请求一个成功 ACK，内核继续主动发送命令 NACK。首个旧编码夹具在读取 ACK 时得到 UNKNOWN，该意图及日志保留，未再次派发。在全新 IPv6 OUTPUT DROP 策略夹具中，真实 Linux 一次原子创建 6 表、58 链和 144 规则，内核 generation 只递增一次，完整读回符合 typed 模型。另一全新夹具用 EEXIST 原始负回执核定原子拒绝，内核保持空规则集。两个测试进程均正常返回 0。外层保持命名空间的容器在完成采集后因停止超时退出 137，测试和采集已先完成。最终固定 ARM64 ELF 的全量 Rust 套件为 778 通过、0 失败、47 跳过。

真实内核验证的输入是从仓内测试数据生成的独占布局，并未经过正式 systemd imported 恢复入口。正式 dispatch 与新物理 epoch/handle 接管、共享链先恢复原所有者基线、真实 JVM/NM 签发和后续合法更新仍内部 pending。完整 Core/OSGi、其余 typed brokers 和六组件矩阵继续保留。原三现场、C/D/E、个人配置与 7fd 停机 41 输入及 16 回执保持。冻结父项 0/12，仅有原 3 项 external deferred。见 [检查点](firewall-imported-boot-review-checkpoint.json)。

## 2026-11-18 配对租约的正向防火墙早启验证

Agent `a497845` 增加独立 Linux 集成夹具。正式防火墙事务在真实 nft 内核上完成 prepare、apply 和 confirm，形成 revision 1 确认策略和原提交日志。正式 coordinator 生成并核验原 schema 4 配对租约、双标记和启动完成回执。NetworkManager D-Bus 和 Kura JVM 见证使用替身。两项启动控制器回归通过。

在独立 Debian 13 ARM64 容器用当前 Git 构建的 Agent release ELF 和原 vendor unit。显式启用后经一次正常 PID 1 退出和启动，单元在 `network-pre.target` 到达前写入唯一 `RESTORED` 回执。正式只读命令从同一 Root 返回原回执摘要。原确认策略、提交日志、双标记、启动完成回执、租约、ownership 指针和原锁的摘要及文件身份逐项精确。内核读回除重新分配的表 handle 外完全相同。普通 Agent 单元未安装。现场已正常停止，保留两个专属持久卷。仅夹具存储选型使用实测设备号和 inode 恒定的持久卷，生产 Root 配对条件保持原样。

此前实际旧态现场保持停止 UNKNOWN。实际 JVM/NM 原见证的签发链、imported ip/ip6 正向早启、缺原回执时的独立运行态核定、剩余 typed brokers 以及完整 Core/OSGi 与六组件矩阵继续内部 pending。原三现场、C/D/E、个人配置和 7fd 停机 41 输入及 16 回执保持，冻结父项仍为 0/12，仅原三项 external deferred。见 [检查点](firewall-paired-positive-boot-checkpoint.json)。

## 2026-11-18 防火墙早启原结果只读查询与回执确认

Agent `116e8e0` 增加固定 Root、本地管理员专用的 `firewall-restore-confirmed-status` 与 `firewall-restore-confirmed-acknowledge`。两条入口均不接受调用方 Root、boot ID 或规则。查询共享锁下读原待确认意图及配对回执；没有待确认意图时，只读当前 boot 已有的回执。确认用同一原共享锁的排他模式，只在原回执及身份完整时清理待确认标记。原回执和原锁保留，缺回执继续 UNKNOWN。复制到另一 Root 的意图在查原回执前拒绝，两条入口均无 nft 后端调用。

Rust 1.95 从当前 Git 构建 Linux ARM64 release ELF。11 项定向 Rust 合同和 88 项包维护测试通过。两处有效 capabilities 为 0 的隔离 Linux 使用正常 ELF：历史待确认意图没有回执时保持 UNKNOWN；真实包维护 guard 拒绝。合成原意图配有精确回执时，只清理标记并放行包维护检查，原回执、状态文件和锁精确。并发占锁、调用方指定 Root/ID 及复制跨 Root 均拒绝。当前提交固定 ELF 的完整 ARM64 Rust 套件为 776 通过、0 失败、44 跳过。

已关闭的现场和既有停止 UNKNOWN 保留，未重新派发。原三现场、C/D/E、个人配置和 7fd 停机 41 输入/16 回执保持。实际已提交 owner lease 的正向 systemd 早启、imported ip/ip6 布局、缺原回执的运行态证据核定、剩余 typed brokers、完整 Core/OSGi 与六组件生命周期矩阵继续内部 pending。冻结父项 0/12，仅原 3 项 external deferred。见 [检查点](firewall-confirmed-original-outcome-checkpoint.json)。

## 2026-11-18 已确认防火墙恢复的可选早启发行单元

Agent `44e023b` 只在 legacy-root-v1 包内增加可显式启用的 vendor oneshot 及其入包摘要。默认不启用、不运行。明确启用后进入多用户启动事务，拉起并排序在 `network-pre.target` 前。单元只有 NET_ADMIN，开启 NNP，只有原 firewall 子树可写，沿原锁 inode 只读打开并排他锁定。包维护核对 vendor 身份并拒绝正在执行的 actor，移除时撤销原启用链接。native-nonroot-v1 包没有该单元或 manifest。

由 Git 以 Rust 1.95 和 JDK 21 实构 Linux ARM64 release ELF、正式格式的本地协议 crate、Java bridge、Core deb 与两种 Agent deb。9 项 Rust 定向、88 项打包、160 项 Java bridge 通过。上一提交的 816 项全量 Rust 记录为 769 通过、3 失败、44 跳过；3 项因并行构建替换其子进程 executable，已在最终 Rust 源码的固定 ELF 副本单独复测通过。当前完整 suite 未重跑。

独立 Debian 13 ARM64 用同一次 Core 安装，首装 legacy Root，普通 Agent 实际运行、远程关闭，早启单元 disabled，站点 nft 完整。显式 enable 后正常停止 PID 1，续接同一容器；实际 systemd 早启调用一次，因缺少原 owner lease 报拒绝，未创建意图或内核表，同一个 Root 锁、配置及 Core 文件精确。实际 remove/purge 保全独立站点 nft 和 Core，撤销启用链接。随后实装 native 包，早启仍无出口，普通 agent 和 Core 均未启动，夹具正常停机。

实际已提交 owner lease 的正向 boot、imported ip/ip6 布局、明确 UNKNOWN 的原编号核定、其余 typed brokers 和完整六组件/provider/launcher/生命周期矩阵继续内部 pending。原三现场、C/D/E、个人配置、7fd 停机 41 输入/16 回执保持。冻结父项 0/12，external deferred 仅原 3 项。见 [本轮检查点](firewall-confirmed-early-unit-checkpoint.json)。

## 2026-11-18 已确认 typed firewall 策略的 Root 恢复执行器

Agent `24d8a47` 提供固定入口 `firewall-restore-confirmed`。只承认固定 Root 的原已提交 schema 4 配对租约，以及私有布局中与原已结算日志一致的 V3/V4 确认策略。入口从 typed policy 重新编译规则，只允许在目标表不存在时独占创建，遇到其他写入方或现有表漂移则拒绝。先同步带 boot ID、Root 与网络命名空间身份的原意图，再单次派发。无原回执保持 UNKNOWN，跨 boot 禁止再派发；复制到其他 Root 的原意图在读取原回执路径前拒绝。打包生命周期继续保全未决意图和历史回执。

从当前 Git 用 Rust 1.95 构建 Linux ARM64。最终提交 5 项恢复合同、3 项共享锁合同及 86 项打包测试通过。两处隔离真实 nft 命名空间验证：仅创建一次，原回执再次调用只读，异配命名空间在内核前拒绝；外来表拒绝且整个规则集精确。正向内核场景用合成已确认状态及租约见证，只证明真实执行原语。正常 ELF 的生产 CLI 拒绝缺原 lease 和调用者指定其他 Root，实际内核无变更。

没有启用生产早启单元。正式 systemd 启动、实际原 owner lease 的整合、原生 imported ip/ip6 布局早启、显式故障核定与其余 typed brokers 继续内部 pending，完整 Core/OSGi、六组件及全部 12 父项范围保持。原场景、个人配置及原 7fd 停机 41 输入/16 回执保持。冻结父项 0/12，external deferred 仍仅原三项。见 [本轮检查点](firewall-confirmed-boot-actor-checkpoint.json)。

## 2026-11-18 firewall-only 移除、purge 与跨变体恢复

Networking `4b58a55` 移除 firewall-only 时先停止已加载 provider 的 Main，仅停用配对的 vendor 早启单元，保留站点单元或 stop 覆盖。卸载不清表、不重设内核默认策略，postrm 在文件移除后刷新 systemd。两变体共用严格校验的能力配置助手，各自保留独立 Root 回执和配置段。切换到 firewall-only 时保全 Networking 的原段，返回 Networking 时按配对记录分别结算。10 项包脚本合同、171 项 Linux 模块测试通过。

旧停机夹具只读确认过往脚本把 Networking 原配对段内的 firewall=false 改成了 true。新助手对这一损坏格式和非标准 dpkg Root 均拒绝，原文件 inode 和字节保持。历史损坏文件的显式修复仍内部 pending。

JDK 21 从当前 Git 构建 Core ARM64 deb、两种 Networking deb 和部署包。新 Debian 13 ARM64 夹具实际安装 Networking，经管理员选择 net=true/firewall=false 后，切换 firewall-only。原 Networking 回执逐字节保留。普通 Main 实际激活两族服务，移除钩子让活跃 Main 正常停止。remove 和 purge 前后，IPv4/IPv6 的 filter/nat/mangle/raw、8 个独立哨兵链、规则文件、快照及两份 Root 回执精确。Networking 解包后停机，续接同一容器时仍为 unpacked，配对记录和文件完整。一次 configure 结算两回执并恢复管理员原配置，Main 保持 disabled，夹具正常退出。

typed firewall broker、确认态早启与显式内核交接、历史损坏拒绝记录的迁移、旧站点文件迁移、NAT/远程及完整六组件/provider/launcher 继续内部 pending。原三现场、C/D/E、个人配置和 7fd 停机 41 输入/16 回执保持。冻结父项 0/12，external deferred 仍仅原 3 项。见 [本轮检查点](networking-firewall-only-removal-checkpoint.json)。

## 2026-11-18 Debian 普通 Main 防火墙规则保存路径

Networking `2fa2fa8` 为 Networking 与 firewall-only 两种 deb 安装独立的 `/var/lib/kura-legacy-firewall/rules`。外层由 Root 管理，内层为 `kurad:kurad/0700`。初始化按地址族保留已有站点文件，已存在的同族包管理文件优先。规则发布同步临时文件及目标目录。实际 Linux 普通 UID 路径检查覆盖历史文件、混合地址族、保留重配及链接拒绝。Linux 防火墙模块 171 项通过。

JDK 21 从当前 Git 构建 Core ARM64 deb、两种 Networking deb 和部署包。隔离 Debian 13 ARM64 首装 Networking 后，规则目录为空、`/etc/sysconfig` 未创建，安装前后的两族内核表精确。实装普通 Main 写入两族内核表和各自的 `0600` 文件。随后实际移除且 purge Networking，再安装冲突变体 firewall-only。安装路径全程保留文件内容、inode、权限和内核规则。夹具正常停机并在同一容器使用新的网络命名空间续接，普通 Main 激活两族 OSGi provider，重新写入内核及各自规则文件，保留独立哨兵。早启单元保持 disabled，夹具再次正常停机。

typed firewall broker、确认态早启加载、旧站点文件的显式迁移、NAT/远程及 firewall-only 移除的内核清理继续内部 pending。其余 Core/OSGi、broker、六组件/provider/launcher、完整生命周期矩阵和全部父项保持原范围。原三个现场、C/D/E、个人配置和 7fd 停机 41 输入/16 回执保全。冻结 12 父项仍 0/12，external deferred 仅原三项。见 [本轮检查点](networking-debian-rule-state-checkpoint.json)。

## 2026-11-18 Networking 旧防火墙规则检查、应用与保存

Networking `cb871ef` 在修改 IPv4/IPv6 内核规则前预写同目录临时文件，检查原目标身份和所需内核表。命令全部成功后才原子替换规则文件。原文件的 owner、group 和权限需要保持，目标为链接或并发改动时拒绝。所需命令失败时保留原保存文件，使尚未确认的新模型不可读。Networking Linux 防火墙模块 168 项通过。

JDK 21 从当前 Git 构建 Core ARM64 deb、Networking deb 和部署包，核对九个 sibling JAR 与防火墙实际入包 JAR。独立 Debian 13 ARM64 使用无外网的网络命名空间和夹具限定的 NET_ADMIN。使用实装普通 systemd Main，缺少旧 `/etc/sysconfig` 时，两族内核规则逐字节一致，保留独立哨兵链。夹具提供 `kurad:kurad/0700` 旧目录后，两族实际生效并发布各自的 `0600` 规则文件，独立哨兵仍在。Main 已 disabled 并正常停止，PID 1 正常退出。

标准 Debian 首装的保存路径、独立 typed firewall broker、分步写入途中内核状态恢复、NAT/远程/早启和完整安装矩阵继续内部 pending。原三现场、C/D/E、个人配置和原 7fd 停机 41 输入/16 回执保持。冻结 12 父项仍 0/12，external deferred 仅原三项。见 [本轮检查点](networking-legacy-firewall-publication-checkpoint.json)。

## 2026-11-18 Networking 保留重装与 OSGi provider

Networking `2ce5250` 在移除前保留独立 Root 0600 回执，以该回执绑定的临时配置段禁用两个应用能力。重装只撤回配对的临时配置段，保留原设置和管理员后来追加的属性。旧版本的无回执 `false` 在实际跨版本升级时保持原字节和 inode。四项源码检查覆盖行边界、原始组合、重试以及修改/异配的拒绝。

按原检查点续接同一独立 Debian。JDK 21 从 Git 构建并核对完整 Networking deb 和部署包。夹具管理员显式设置两项初始值，移除后追加无关属性，解包并正常重启夹具 PID 1 后仍为 unpacked，回执和配置一致。配置一次后恢复原 `true` 值及追加属性，删除已结算的回执。七份已核私有文件、站点 NM 文件和 Core 策略保持。

该夹具用原 systemd Main 实际启动，Core 读取两项 `true` 和新增属性，`org.eclipse.kura.nm` 注册状态和配置元数据服务，实际 NM 状态查询返回 1 项。IPv4/IPv6 防火墙 provider 均注册，启动时保存规则报 IO_ERROR，实际防火墙规则写入仍内部 pending。已正常停止 Main，保全只读验收 bundle 到自动加载目录之外，原夹具再次正常停机。

原三现场、C/D/E、个人配置、7fd 停机 41 输入/16 回执和原 UNKNOWN 已复核。完整 Core/OSGi、typed brokers、六组件/provider/launcher、全部生命周期及 12 父项继续实施。冻结父项仍 0/12，external deferred 仅原 3 项。见 [本轮检查点](networking-capabilities-and-provider-checkpoint.json)。

## 2026-11-18 Networking 旧 NM 配置跨 purge 保全

Networking `167ea8b` 在 dpkg 仍登记旧 `99-kura-nm.conf` 时，先落 Root 0600 记录，再以不改名的临时 diversion 将 dpkg 的清除目标指向受保护的空路径。purge 或重装确认旧登记消失后解除。外部 diversion、危险路径和被占用的目标拒绝处理。桥接构建器 `ef19810` 在宽松 umask 下仍生成受保护的归档目录和 conffiles 元数据。

JDK 21 从 Git 构建完整 Core ARM64 deb、Networking deb 和部署包。独立 Debian 从最小历史普通载荷夹具进入当前 bridge 和正常包，得到实际 obsolete conffile。移除后保留 `config-files`，管理员再修改活动文件。原容器正常停机并续接后，实际 purge 清除了 dpkg 登记和临时 diversion，重装也保留该文件。已核文件内容、inode、所有权、模式和时间精确。五项独立 dpkg 钩子试验通过，涵盖直接 purge、结算前后已知失败再续接、保留目标占用及其他 diversion。

Networking `bc90625` 保全 Root 注册表的组和模式。实际 bridge→正常包升级保留字节、顺序和原 inode，移除与重装仅在成员变化时原子替换，保持 Core 的 `0:992/0640`。重装后的两个应用能力标志仍是移除时的 `false`，恢复原能力待实施。Main disabled，七份已核私有文件精确，夹具正常停机。

原三现场、C/D/E、个人配置、原 7fd 停机 41 输入/16 回执已复核。完整 Core/OSGi、其余 typed brokers、六组件/provider/launcher/生命周期与全部 12 父项继续内部 pending。冻结父项仍 0/12，external deferred 仅原 3 项。见 [本轮检查点](networking-legacy-nm-purge-checkpoint.json)。

## 2026-11-18 Core 与 Networking 的 systemd 授权顺序

Core `e952f82bb9` 把共享规则发布到 `51-kura-systemd.rules`，让 Core 在旧 Networking 规则之前决定 `kurad` 的 systemd 与 logind 授权。新版规则发布后，原 `kura.rules` 与可见的旧 wildcard 规则停用并保留原文件归档。Networking `2fe49eb` 的 `52-kura-networking.rules` 仅保留 NetworkManager、ModemManager 和 wpa_supplicant 授权。

从 Git 用 JDK 21 构建两代 Core deb、两代 Networking deb 和部署包。独立 Debian 实装旧组合后，只升级 Core 并保留旧 Networking。两项 Chrony 管理动作和一项越界 named 动作的实际 polkit 决策转为拒绝。允许的 named/bind9 动作及 NetworkManager 授权仍有效。旧规则按原字节归档为 Root 0600。Main 保持 disabled，实际服务、电源和业务写入派发均为 0。

新版 Networking 解包后容器停止超时，Docker 发送 SIGKILL 并以 137 退出。同一容器恢复后仍为 unpacked，原 Core 授权、现场 NM 文件及七份已核私有文件保留，Docker 重挂的设备号单独核对。继续原 configure 一次成功，九个 JAR 和注册顺序保持。注册表内容相同，原子重写后 inode 变化，属组及模式从 0:992/0640 变为 0:0/0644。实际移除成功，撤下 Networking 的规则与 drop-in，独立站点 NM 文件和 Core 授权保留。普通包移除后已为 not-installed，随后 purge 无待处理对象。夹具最终由 systemd 正常退出。

旧 NM obsolete conffile 的实际 purge、保留重装的应用标志、注册表权限连续性和完整运行矩阵仍内部 pending。原三现场、C/D/E、个人配置与 7fd 停机 41 输入/16 回执已复核。冻结父项仍 0/12 最终闭环，external deferred 仅原 3 项。见 [检查点](core-networking-polkit-package-checkpoint.json)。

## 2026-11-18 普通 Main 重启权限撤回

Core `6a8a19c8ac` 从普通 Main 的 ambient 和 bounding 集合移除 `CAP_SYS_BOOT`。升级根据同 Root 的包历史核对保存的原软件包和原单元，再采用入包的配对 vendor 单元。保留重装复用已有 Main 回执，实际加载配置在解除维护围栏前核对。Core 拒绝 `kurad` 的 logind 电源动作并归档旧规则。Networking `dc8cd36` 的兼容脚本停止创建或删除共享规则，保留 NetworkManager 等专用规则。

JDK 21 从隔离 Git 构建 Core ARM64 deb、Networking deb 和部署包，核对实际入包文件。macOS 合同 250 项通过、24 项跳过。Linux ARM64 的普通 UID 合同 242 项通过、18 项跳过。Core `c77d6a0357` 只修复两套既有测试的输入，另有 21 项生成代包合同和 6 项历史日志合同通过。

独立 Debian 实际升级一次。重启 broker 的新版本只替换 cohort 中的 Main 单元。原五份密钥及快照文件和原八份 broker/sender 记录保持，旧 logind 规则按原字节归档为 Root 0600。实查 19 项 logind 电源动作均拒绝，旧规则时期其中 6 项曾直接放行。普通 `kurad` Main 实际启动，OSGi SystemService 与 Jetty 就绪，JVM 五个 capability 集合均无 `CAP_SYS_BOOT`。原 UNKNOWN 和 sender 待办各一条，实际派发 0，夹具保持 disabled 并正常停止。

保留重装权限变更的实装交叉矩阵、Networking 实际安装升级移除 purge、真实 reboot/boot 和业务恢复、完整 Core/OSGi、其余 typed brokers 与六组件/provider/launcher/安装升级移除 purge/中断恢复继续内部 pending。原三现场、C/D/E、个人配置、停机 7fd 的 41 输入/16 回执及原 Chrony UNKNOWN 均复核。冻结 12 父项仍 0/12 最终闭环，external deferred 仅原 3 项。见 [检查点](core-main-reboot-privilege-cutover-checkpoint.json)。

## 2026-11-18 Core 与部署重启入口接线

Core `4ee120cf53` 将 Linux `SystemAdminService.reboot()` 和 watchdog 切到安装态 broker 的固定 helper，维持公开 API。客户端只对确定未派发的情况提供可重试结果，超时、断连、无效回执及 UNKNOWN 保持不确定。watchdog 保留先 sync、停机取消和硬件兜底。deployment `bca16d3` 改用已有 `SystemAdminService`，安装及移除的原业务结果与重启请求分开报告，缺服务时没有命令回退。

macOS 与 Linux ARM64 的 JDK 21 检查每端 Core 系统模块 42 通过、2 跳过，watchdog 114 项、部署 123 项均通过。两份 Core bundle 与从隔离 Git 构建的 ARM64 deb 逐字节配对。deployment deb 的 bundle 和 OSGi 服务引用也已核对，部署包仍待实装。

独立 Debian 从 `120853ec99` 正常升级一次至新完整包，重启 broker 的 Python 和 systemd cohort 相同，Main 的 disabled 状态保持。旧夹具探针保留在自动加载树之外。以正常 Main 内的 OSGi 探针调用一次公开重启入口，Root actor 实际启动，Java 记录 `UNKNOWN or unavailable`，watchdog 的新包导入实际解析。原 Root 恰一条合成 UNKNOWN，sender 恰一条原待办，八份文件在升级、调用和停机间完整精确。实际重启派发 0，夹具正常停机。

保留 Main 单元的 CAP_SYS_BOOT 与旧 logind 授权迁移、实际重启与业务恢复、deployment 实装及硬件 watchdog、跨 broker cohort 升级、完整 Core/OSGi、其余 typed brokers、六组件/provider/launcher 和全部生命周期矩阵继续内部 pending。原三现场、个人配置、C/D/E、7fd 停机 41 输入/16 回执及旧 UNKNOWN 已复核。冻结 12 父项仍 0/12 最终闭环，external deferred 仅原 3 项。见 [检查点](core-reboot-callers-checkpoint.json)。

## 2026-11-18 Core 重启 broker 自动安装与 purge 后续接

Core `120853ec99` 将重启 broker 接入 dpkg。首次配置先保留唯一安装标记，再发布配对 Root、发送端和已校验的 systemd 单元。升级和移除在已配对日志锁内设置围栏。卸载与 purge 保留独立 Root 及 sender，已建立配对缺文件时拒绝重新初始化。配置核对实际单元与 socket 启用后才解除维护围栏，原 UNKNOWN 仍阻止新派发。

macOS 43 项重启合同通过、2 项 Linux 专属跳过。Linux ARM64 45 项均通过，两平台各 86 项时钟回归通过。JDK 21 从 Git 构建的完整 ARM64 deb 在新的独立 Debian 13 上实际首装、卸载、purge、解包、配置各一次。解包后正常重启夹具 PID 1，dpkg 仍为 unpacked，socket 未加载，原围栏与八份记录不变。跨该次重挂仅设备号改变，文件原内容、inode 和其余已核元数据精确，内核 boot ID 保持。配置重新启用配对 socket。

单条 UNKNOWN 由夹具的派发替身建立，实际重启派发为 0。恢复配置后，普通 Main 通过 OSGi 探针和生产 helper 查回同 Root 的原 ID，返回 `currentRequest=false`，Root 仍单条 UNKNOWN、发送端仍单条待办。八份原记录在查回及正常停机后保持，Main 已 disabled。此证据覆盖同包卸载重装、purge 与解包后中断，跨版本实际升级、脚本内更多中断点、产品调用接线、旧权限撤回与实际重启、内核 boot 和业务恢复仍内部 pending。

完整 Core/OSGi、其余 typed brokers、六组件/provider/launcher 和全部生命周期矩阵继续实施。原三现场、个人配置、C/D/E 和 7fd 停机 41 输入/16 回执已复核，旧 UNKNOWN 留在原 Root。冻结 12 父项 0/12，external deferred 仅原 3 项。见 [检查点](core-reboot-package-lifecycle-checkpoint.json)。

## 2026-11-18 Core 重启 broker 实装 socket 与 Main 准入

Core `47b3906027` 将固定 reboot 协议接入 systemd 激活的 Unix socket。客户端验证 PID 1 持有的监听端。Root actor 用内核 pidfd、实际 MainPID、InvocationID、父子关系、账号和 cgroup 核对固定 helper。actor 的 capability 集合为空，NNP 为 1。包保留三份单元副本，核对实际加载单元和源码 SHA。Root 核对并接受密钥服务预留的固定 Root 0700 空目录，活动覆盖仍拒绝。派发器固定调用 systemctl reboot，受理和实际 boot 变化仍分开。

macOS 35 项重启合同通过、2 项 Linux 专属跳过。Linux ARM64 37 项均通过，两平台的 86 项时钟合同均通过。JDK 21 从隔离 Git 构建的完整 ARM64 deb 在新 Debian 13 现场完成一次首装和两次纠错升级。最终普通 Main 内 OSGi 探针通过真实 helper 到达 Root 维护围栏，返回 `ADMISSION_CLOSED`。同 UID 的 Main 外进程在未发送应用帧时由已配对监听端返回 `UNAUTHORIZED`。外进程直接走完整 helper 时仅得到通用不可用错误。Root 原绑定与围栏跨最终升级和停机逐字节精确，重启日志、发送 pending 和实际派发均为 0。独立现场已正常停机。

夹具手动配置 Root 配对和启动 socket。自动安装激活、Root 与 sender 的发行保全、实际重启及 boot/应用恢复、产品 Main/watchdog/部署调用和原权限撤回继续内部 pending。完整 Core/OSGi、其余 typed brokers、六组件/provider/launcher 和全部生命周期矩阵继续实施。原三现场、个人配置、C/D/E 和 7fd 停机 41 输入/16 回执已复核，旧 UNKNOWN 留在原 Root。冻结 12 父项 0/12，external deferred 仅原 3 项。见 [检查点](core-standalone-reboot-socket-checkpoint.json)。

## 2026-11-18 Core 重启请求与发送回执持久化

Core `e113ccb0f0` 实现独立的固定类型重启协议。Root 核对预期 namespace 和当前代码 cohort，在调用派发器前同步保存与 boot ID 绑定的原意图。受理与后续观察到的 boot ID 分开，原记录只读。对同一次启动只允许一条已受理的重启，未知结果会跨启动继续封住派发。发送端先保存原 ID，恢复时先核 Root 配对并仅查原记录。原 Root 无记录时仍保留 UNKNOWN，其他 Root 收不到旧 ID。

macOS 与 Linux ARM64 各通过 23 项重启协议和 86 项既有时钟合同。两个独立进程退出用例覆盖 Root 意图已持久化但无回执，以及发送端丢失已受理的回执。Linux 以普通 UID、无 capability 和无网络执行，boot 变化用独立 ID 输入模拟，实际重启派发为 0。

生产 socket/peer 和 systemd 固定动作、Main 与部署组件调用切换、CAP_SYS_BOOT 和旧 logind 授权撤回、发行安装与中断恢复均内部 pending。完整 Core/OSGi、其余 typed brokers、六组件/provider/launcher 及全部 12 父项继续实施。原三现场、个人配置、C/D/E 和停机 7fd 的 41 输入/16 回执仍沿用，旧 UNKNOWN 留在原 Root。冻结父项 0/12 闭环，external deferred 仅原 3 项。见 [检查点](core-standalone-reboot-journal-checkpoint.json)。

## 2026-11-18 watchdog 重启结果与配置变更

Core `ce1f4f7ec2` 将确定未启动的重启和已启动但结果不明的重启分开。无硬件 watchdog 时，仅确定未启动进入软件重试。已启动后非零退出、超时或等待中断均保留 UNKNOWN。设备更换保留原待重启状态、原因和硬件兜底截止时间。SystemAdminService 同步修正命令失败日志判断。

macOS 与 Linux ARM64 的 JDK 21 模块回归每端 watchdog 116 项、core.system 33 项通过，core.system 每端 2 项跳过。Linux 合同使用普通 UID、空 capability 集合及模拟进程和设备。完整 ARM64 deb 从该提交构建，两份包内 OSGi bundle 与 reactor 产物逐字节一致。验证范围为源码合同和包内容。本轮未部署该包，实际重启派发为 0。

跨 JVM 或系统重启的持久回执、原 Root 配对的 typed reboot broker、Core/watchdog/部署组件调用切换、Main CAP_SYS_BOOT 和旧 logind 权限撤回仍内部 pending。完整 Core/OSGi、Chrony 剩余动作、其余 typed brokers、六组件/provider/launcher 及安装升级移除 purge 和中断恢复矩阵继续实施。原三现场、个人配置和 C/D/E 已只读核对。原 7fd 停机状态与 41 输入/16 回执沿用。原 Chrony UNKNOWN 现场仍正常停放。冻结 12 父项 0/12 最终闭环，external deferred 仅原 3 项。见 [检查点](core-watchdog-reboot-outcomes-checkpoint.json)。

## 2026-11-18 Core sender 回执跨 purge 保全

Core `5f719e43b3` 在卸载时把固定 sender 文件经原 Root 核对后原子写入 `/etc/kura` 的 Root 0600 文件。副本绑定原 clock namespace、维护 ID 和两侧 UID/GID。重装在 key 和 clock 准入开放前恢复或核对原记录。接收方尚无意图的 pending 路径通过合同验证，保留原字节且派发计数为 0。macOS 和 Linux ARM64 各 86 项 clock 合同及 23 项 JDK 21 检查通过。

新源码完整 ARM64 deb 在原独立 Debian 13 夹具上实际升级、移除、purge、同包重装各一次，均返回 0。purge 后保留目录和可执行安装树已删除，Root 副本和原围栏跨夹具 PID 1 正常重启完整精确。重装恢复五份 sender 的原内容、权限及 mtime，inode/ctime 随恢复变化。原 Root 两条 APPLIED、namespace、账号和 14 份外部文件完整指纹精确。key 包历史沿原 remove ID 完成，写入准入恢复。Main 和 Chrony 启动次数均为 0。purge 后重装采用默认 enabled，验收后独立夹具调回 disabled 并正常关停。

由旧软件包移除后直接执行旧 `postrm` 的历史 purge、其它中断点、Chrony 剩余写入和运行权限、完整 Core/OSGi、其余 typed brokers 和六组件发行矩阵仍内部 pending。原 7fd 停机状态及 41 输入/16 回执精确，原三现场、个人配置、C/D/E 与旧 UNKNOWN 已核对。冻结 12 父项仍为 0/12 闭环，external deferred 仅原 3 项。见 [检查点](core-clock-purge-sender-checkpoint.json)。

## 2026-11-18 Chrony 回执随保留重装跨解包中断续接

在原独立 Debian 13 ARM64 夹具上，使用与 Core `0ed95fa4fe` 完全配对的完整 deb，实际 `dpkg --remove`、`--unpack`、`--configure` 各一次均返回 0。停在 `unpacked` 后正常关闭并重启夹具 PID 1。重启前后状态均为 `unpacked`，Main 与 clock 单元未加载，Chrony 的 polkit 规则缺席，写准入关闭，原 clock 和 key 维护 ID 及保留副本精确。配置完成后规则和 socket 恢复，两个围栏解除，key 包历史按原 remove ID 追加一条回执。

全流程未启动 Main 和 Chrony。原 Root 两条 Chrony `APPLIED` 只读查回，14 份文件完整指纹精确，原 key 历史两条保持；五份 sender 内容、权限和 mtime 精确，常规复制更新 inode/ctime。最终 Main/Chrony 仍 disabled，夹具正常退出。该证据覆盖 `--unpack` 完成至 `--configure` 之间的 PID 1 重启。

源码核对发现 `postrm purge` 会删除整棵 `/var/lib/kura-retained`，其中包含 clock sender 的 `issued` 和 `completed`，而 actor 原 Root 日志保留于独立路径。具有 sender 历史的 purge 需要补齐持久保全及同 Root 续接；其实际验收、dpkg 脚本执行中的中断、Chrony 配置/`makestep`、完整 Core/OSGi、其余 typed brokers 和六组件矩阵均继续内部 pending。原 7fd 为 41 输入/16 回执，原三现场与旧 UNKNOWN 停放状态已复核。冻结 12 父项仍 0/12 最终闭环，external deferred 仅原 3 项。见 [检查点](core-chrony-retained-interruption-checkpoint.json)。

## 2026-11-18 Chrony 校时事件的 typed 读取

Core `0ed95fa4fe` 的 Chrony provider 经原配对 clock broker 读取本次启动、当天的最近校时事件。查询固定 Chrony 单元、真实可执行路径和消息格式，响应只有 schema 和两个时间戳。provider 用单调时间去重，系统壁钟退回后仍能识别后续事件。`systemd-journal` 读取组只配置于独立 actor 单元，`kura-clock` 系统账号本身没有该组成员资格。日志不可读和有权限但无匹配分别处理。

macOS 和 Linux ARM64 各 83 项 clock 合同通过，12 项 polkit 合同和 23 项 JDK 21 检查通过。已更新旧 JVM 测试的空结果 mock。JDK 21 从 Git 和独立缓存构建的完整 ARM64 deb 在原独立 Debian 13 夹具上单次升级成功。实际 Main 经 OSGi 生产 client 查询，排除了普通 UID 写入的同名日志，同 UID 的 Main 外进程返回 `UNAUTHORIZED`。真实校时事件的时间戳通过代码合同验证，实装夹具验证空结果和权限边界，真实 clock step 仍待验。

原 Root 两条启停回执、namespace 和账号连续。17 份文件内容、属主、模式、链接数、大小及 mtime 精确，随标准升级复制的五份 sender 文件改变 inode/ctime，另外 12 份完整指纹精确。Main 和 Chrony 均 disabled，夹具退出 0。原 7fd 仍沿用停机 41 输入/16 回执，旧 UNKNOWN 原现场停放状态精确。Chrony 配置、`makestep`、Main 停机阶段与旧总线兼容，以及完整 Core/OSGi、其余 typed brokers、六组件/provider/launcher 和全生命周期矩阵继续内部 pending。冻结 12 父项仍 0/12 最终闭环，external deferred 仅原 3 项。见 [检查点](core-chrony-step-journal-checkpoint.json)。

## 2026-11-18 Debian 13 ARM64 实装 Chrony 正向启停

Core `9949693770` 从 Git 隔离检出并以 Maven 3.10.1、JDK 21、独立缓存构建完整 ARM64 deb。22 项 JDK 21 检查通过。Debian 13 原生 systemd 257、D-Bus 1.16、polkit 126 和 Chrony 4.6.1 上首装成功。正常 Main 中的 OSGi 探针通过实际 ClockService 的生产 client 启停固定 `chrony.service`，原 Root 分别持久返回 `APPLIED/CHRONY_STARTED` 和 `APPLIED/CHRONY_STOPPED`。系统总线观察到 StartUnit 和 StopUnit 各派发一次，polkit 分别检查各自的可信 bus name。日志显示同一实际 chronyd PID 启停各一次。

Main 的五类能力均无 CAP_SYS_TIME，独立 actor 的五类能力均仅 CAP_SYS_TIME，NNP 为 1。夹具只有 loopback，使用发行版 `/etc/default/chrony` 的 `-x`，原厂单元保持精确，seccomp 拦截壁钟和 RTC 写入，实际两类请求均为 0。单次正常停机后两条业务的 12 份记录哈希精确，Main 与 Chrony 均 disabled，容器退出 0。

Ubuntu 26.04 ARM64 原生总线也返回 `ProcessFD`，原厂 Chrony 单元在 Docker 下条件为 false，该夹具在调用前正常停机。Ubuntu 24.04 的旧请求仍停放于原 Root，未重发。旧总线兼容、按单元条件安全准入、deactivating Main、Chrony 配置/`makestep`/事件访问、RTC 正向写入与热插拔、完整 Core/OSGi、其余 typed brokers、六组件/provider/launcher 及全生命周期矩阵继续内部 pending。原三现场和个人配置保全核对通过，7fd 仍为 41 输入/16 回执。原 12 父项仍 0/12 最终闭环，external deferred 仅原 3 项。见 [检查点](core-chrony-positive-service-checkpoint.json)。

## 2026-11-18 Chrony 启停事务与系统总线身份准入

Core `a0eb7e7da8` 将 Java ClockService 的 Chrony 启停接入原 Root 的 prepare/commit 日志。actor 只接受实际配对的固定单元和启停请求，按最终单元状态写终态回执。一次 ARM64 整包升级成功，Main 配置仍为 disabled，Chrony 配置和原锁指纹精确。实机 polkit 检查确认旧 `kurad` 的直接启停授权已撤回，服务外的 `kura-clock` 同 UID 也被拒绝。

正常 Main 的首次 `start-chrony` 留下 UNKNOWN。实际 D-Bus 1.14 凭据无 `ProcessFD`，polkit 无法验证调用者的服务单元。Chrony 未启动，未发送 stop。沿原 Root 只读查回原 ID 的 `NO_DURABLE_RESULT`，正常停机前六份记录哈希精确，无重发。旧探针原文件留存。夹具重启时恢复了本地 loopback 主机名解析，整包未作现场修改。

Core `9949693770` 将 `ProcessFD` 准入检查前移至 prepare 和派发前，两平台各 80 项 clock 合同通过，另有 12 项 polkit 合同和 22 项 JDK 21 检查通过。首次 Linux 合同运行使用了 Root，触发 sender 身份拒绝，按产品身份要求以普通 UID 重验通过。新检查在实机总线上返回 `EOPNOTSUPP`。新提交仍待整包实装，完整成功启停和 deactivating 生命周期仍内部 pending。原 7fd 停机状态实时一致，沿用 41 输入和 16 回执。

Chrony 配置、`makestep`、事件访问授权、RTC 正向写入与热插拔、完整 Core/OSGi、其余 typed brokers、六组件/provider/launcher 和安装升级移除 purge/中断恢复仍内部 pending。冻结 12 父项 0/12 最终闭环，external deferred 仅原 3 项。见 [检查点](core-chrony-service-control-checkpoint.json) 及 [polkit 124 源码](https://github.com/polkit-org/polkit/blob/124/src/polkitbackend/polkitbackendjsauthority.cpp#L652-L675)。

## 2026-11-18 Chrony 状态接线与 Main 时钟能力移除

Core `9e59cb6004` 用现有已配对的 clock broker 查询 Chrony 单元状态。请求不接受服务名或操作 ID，也不创建时钟日志。Chrony 事件读取改用固定 `journalctl` 参数。Main 的 ambient 和 bounding 集合均移除 CAP_SYS_TIME，clock actor 保留独立身份和仅有的 CAP_SYS_TIME。

同组 72 项 Python 在 macOS 和 Linux ARM64 各通过，另有 21 项 JDK 21 检查通过。新 ARM64 deb 实装后，正常 Main 中的实际 Chrony provider 初始化一次选中 `chrony.service`。同 UID 的独立进程被拒绝为 UNAUTHORIZED。实测 Main 五类能力均无 CAP_SYS_TIME，actor 五类均仅该项，Root 和 sender 均无 clock 事务。旧安装器在 Main 启动前停用夹具自启的 Chrony，原配置指纹精确。新夹具已正常停机。

真实 ARM64 `chronyc tracking` 能由 cap0 的 Chrony UID 在固定 Unix socket 执行。无应答的独立 socket 探针测得默认 3 包和零重试 1 包。正式 `makestep` 仍需单次发送并单独处理协议版本回退。Chrony 配置、启停、校时写与事件授权，以及 RTC 正向写入和热插拔仍内部 pending。完整 Core/OSGi、其余 typed brokers、六组件/provider/launcher、安装升级移除与中断恢复及 12 父项继续实施。最终 0/12，external deferred 仅原 3 项。见 [检查点](core-chrony-status-capability-checkpoint.json) 和 [Chrony 4.5 客户端文档](https://chrony-project.org/doc/4.5/chronyc.html#_client_commands)。

## 2026-11-18 RTC 准备记录绑定实际设备

Core `29c92f0427` 在 prepare 保存内核设备和投影节点的身份指纹，commit 在打开设备前核对原指纹。别名改指、节点换代或准备时设备缺失均无法转成对其他设备的写入。旧未提交的无绑定请求保持 UNKNOWN，旧终态回执按原格式读取。

同组 67 项合同在 macOS 和 Linux ARM64 各通过。ARM64 实际缺投影探针验证 prepare 保存 ENOENT，重新实例化 actor 后只完成同一请求的确定失败，RTC/壁钟执行均为 0。Chrony 实机只读核对确认 `chrony.service` 状态可由 cap0 账户读取。配置写入、启停、`makestep` 与校时事件读取各自的 typed 接线仍内部 pending。

从 Git 隔离重建的 ARM64 包在原独立 RTC 夹具上单次实装升级成功。新代码同 Root 读回旧 FAILED/EPERM，7 份原记录精确。升级期间 Main disabled 且未启动，actor 未启动，新增 clock 请求为 0。夹具正常停机。原 7fd 仍保持 41 输入/16 回执及原停机状态。

详见 [检查点](core-rtc-prepared-device-checkpoint.json)。RTC 正向写入、热插拔和剩余包生命周期、完整 Core/OSGi、其余 typed brokers、六组件/provider/launcher 和全部 12 父项继续实施。冻结台账仍为 0/12，external deferred 仅原 3 项。

## 2026-11-18 RTC 生产接线及实装拒绝路径

Core `228233f2d0` 将 Java ClockService 的 RTC 写入切换到已配对的 actor。独立启动单元以 CAP_MKNOD/CAP_CHOWN 生成 Root 所有的临时 RTC 投影，保留宿主设备原权限。actor 保留私有 `/dev`，以只读 bind 和 `char-rtc r` 使用投影，仍只有 CAP_SYS_TIME。RTC 调用沿用同一个 pending、原 Root 和不重发规则。

同一组 Python 61 项在 macOS 与 Linux ARM64 各通过。JDK 21 的 18 项 Java 定向检查通过。新 Git 完整 ARM64 deb 实际首装后，正常 Main/OSGi 单次提交 `/dev/rtc0`，实际投影为 254:0，actor 生效挂载只读。同容器直接只读 open 也得到 EPERM，生产回执为 FAILED/OS_ERROR/errno1，未达 ioctl。原请求、issued、回执及 ClockEvent 实际核对通过。新夹具已正常停机，7 份记录保持，原 7fd 仍为 41 输入/16 回执。

RTC 正向设备打开/写入、热插拔与新投影升级移除恢复继续待验。完整 Core/OSGi、Chrony/其余 typed brokers、六组件/provider/launcher 与整个安装升级移除、purge 和中断矩阵仍内部 pending。冻结 12 父项继续 0/12，external deferred 仅原 3 项。详见 [检查点](core-rtc-installed-cutover-checkpoint.json)。

## 2026-11-18 RTC typed 协议与 ARM64 验证

Core `4512ac8573` 已实现独立 RTC 入参、回执和 Linux ioctl 边界。RTC 与壁钟沿同一 Root、同一 pending/issued 日志准入。旧壁钟 APPLIED 和 UNKNOWN 按原格式读回，跨版本夹具的 9 份文件逐字节精确。两个平台各通过同一组 60 项合同。ARM64 系统头文件验证 ioctl ABI。隔离 Linux 缺设备得到 ENOENT 确定失败，ioctl 次数 0。

生产 actor 私有设备授权和 Java ClockService 切换仍内部 pending。完整 Core/OSGi 业务、Chrony/其余 typed brokers、六组件/provider/launcher、安装升级移除与 purge、中断恢复及全部 12 父项继续保留。原 7fd 停机基线 41/16 沿用。最终 0/12，仍仅原 3 项 external deferred。详见 [检查点](core-rtc-typed-protocol-checkpoint.json)。

## 2026-11-18 历史 static、linked 与安装态 OSGi 业务

Core `77d53eda4a`。历史 schema 1 static 的原 manifest 由同一 Root 最后已完成包历史及原 HMAC/UID/GID/围栏配对。真实缺清单拒绝一次，配对后沿旧围栏重装成功，24 输入保留。受保护永久和 runtime linked 实际迁移为 disabled，25 输入保持。从旧 40268c05c3 的 linked 经新包控制入口迁移并升级成功，23 输入连续，原源文件保留。74 项定向中 73 通过、1 个 macOS/Linux-only 跳过。

独立安装态 Main 探针已过 65,329 字节 AES 流、两个实际 CM 存储视图及一个逻辑视图，实际 Root lease 和原数据精确。探针只读续验，既有 Root 业务和已通过的 AES 步骤保持一次。原 7fd 保留停机状态和 41/16 原基线。完整 Core/OSGi 业务、RTC/Chrony/其余 typed brokers、六组件/provider/launcher 和通用中断继续内部 pending。原 12 父项完整保留、0/12 最终闭环，仅原 3 项 external deferred。见 [审查](../../reviews/core-historical-main-migration-review-20261118.md) 和 [检查点](core-historical-main-migration-checkpoint.json)。

## 2026-11-18 配对 static 转换与原 7fd 实际 Core 启停

Core `40268c05c3` 为经过精确包定义及实际有效单元配对的 static 保存原 Root HMAC 回执，标准包重装沿原移除 ID 转为 disabled。ARM64 实际首装/移除/保留重装 0/0/0，23 旧输入保持。独立后续 purge 及沿原围栏重装通过，15 份外部输入继续保留。50 定向通过，另 69 项相邻回归中 68 通过、1 项 Linux renameat2 在 macOS 跳过。

同一原 7fd 从 e99 单次正常升级，39 原输入与新回执连续，真实普通 Main / Equinox / writer lease / ClockService 通过。首次预检失败与首次 Main 停机超时留存，隔离 fixture 的 `/run` 传播及本机名解析修正后真实启停通过。最终 41 份输入（含新增原配置分派锁）、16 正常回执，Main disabled 及正常停放。原现场和亲缘 Root 的业务意图保留。

继续历史 schema 1 static、linked 审核迁移、完整 Core/OSGi/合法 Main 的实际业务、RTC/Chrony/其余 typed brokers、完整六组件/provider/launcher/purge/通用中断恢复及下列冻结台账的全部范围。12 父项仍 0/12 最终闭环，external deferred 仍为原 3 项。详见 [检查点](core-paired-static-retained-checkpoint.json) 和 [实际审查](../../reviews/core-paired-static-retained-review-20261118.md)。

## 2026-11-18 用户要求本轮收尾后暂停

用户明确要求本轮完成后停止目标、从新会话续接。phase33代码/新包/实际拒绝路径/保全/官方父知识库已完成收尾，未开启下一轮实施。最新[完整交接](../../reviews/continuation-paused-20261118-main-layout-admission.md)及[暂停检查点](main-layout-pause-checkpoint.json)；交接备妥后将目标工具置paused，最终状态以回执目录pause-goal-result.json为准，不把暂停称为目标完成。包含交接提交的确切18仓HEAD/tree/status在pause-final-repositories-and-ledger.json；新会话仍实时核对。

Core d49e65824f（代码e99df17793）、GWT18c1b5eed4，console本轮实施3fff782后仅追加暂停交接提交。原7fd完整安装e99、39输入/15回执、disabled且Main从未启动、正常停止；原3/个人/C/D/E和两VM状态保留。完整12父项及仅3 external deferred不变，0/12最终闭环；继续权限、旧态/完整六组件/provider/launcher/安装恢复矩阵及全部原范围。已授权现有积分继续，新会话无需重复询问积分许可。历史active条目保留为当时记录。

## 2026-11-18 Core Main 布局维护前拒绝

e99df17793把linked/linked-runtime等不支持Main选择的拒绝前移到initialize、新维护围栏和stop之前；schema2/static还核固定vendor与原manifest的unitSha256，不认领新路径、不读取key、不放宽Root配对。缺Main不引入完整reader依赖，有原围栏不重新解释unpack中间态。66宿主选择65过/1既有Linux renameat2在macOS skip。新Git/private缓存/JDK21完整ARM64包6.0.0~git202611180255.e99df17-1/SHAf99692a6…ddb0a1从原44正常升级一次exit0，38输入保持、新增1正常回执，总39/15，Main disabled未启。

三项完整实际dpkg拒绝观察linked-observed、linked-runtime、未配对static各一次exit1：原39数据及所选unit/link完整指纹、5辅助单元属性/活跃进程ticks精确，无新维护围栏/retained/upgrade stage或业务。首次另有linked采集不完整：socket属性和脱敏消息断言错误，实际动作已知拒绝但调用前helper快照未持久化；不计完整通过、不回填。原失败留存、39数据精确且无新ID后只恢复夹具布局，新linked-observed独立观察，非UNKNOWN重发或旧UUID业务重做。总4次已知拒绝、本轮完整3次；不算成功移除/通用中断恢复。

观察完成后才恢复夹具vendor布局和dpkg install选择；最终key完整generation及clock canonical/actor/effective配对通过，39输入/15回执、原UID/clock目的/密钥/.data历史保持，原7fd完整安装e99、disabled、从未启动Main，正常停止exit0。03:03UTC原3/个人25/C/D/E原身份/文件/进程/ON保持，两VM stopped；E Main仍11/13caps/NNP0。见[检查点](core-main-layout-admission-checkpoint.json)、[审查](../../reviews/core-main-layout-admission-review-20261118.md)。

合法旧static转换/linked审核迁移、purge/通用中断及完整Core/OSGi/当前合法Main/RTC/Chrony/其余typed brokers、六组件/provider/launcher/资源/平台矩阵和原14组/API/R017/GUI/网络/密钥/CA/客户DB/上游/正式发行12父项继续内部pending。仅原3external deferred，冻结全文SHA不变，最终0/12、无加权百分比；G034/协议未封签发布。goal active/现有积分继续，无push/JDK27/活动runtime清理；UNKNOWN/丢失CSR不重发换号跨Root查、闭环密钥/归档/900秒等不重放。

## 2026-11-18 Core 实际 Main 四状态

97dbbc06e9修复普通disable遗留/run enable链接，追加--runtime且失败前不删vendor，补cron/logrotate/tmp失败传播；凭据准备只认固定Root/单链接/受保护父目录/精确/dev/null Main mask，必须原canonical+回执完整、拒绝bootstrap/普通override/其他目标和mask变更。44b4281923修复clock安装拒绝合法Main masked：明确inactive/PID0/control0+固定Rootmask才接受，clock service/socket masked及原代码/有效unit检查仍拒绝，不读key。135宿主选择134过/1既有Linux-only归档用例在macOS skip不算通过，7实际LinuxRoot key/mask、6shipped preRemove/systemd条件通过，不累加旧JVM/crypto。native首夹具umask000被正确拒绝，失败保留新独立umask077夹具通过；生产检查未放宽。

18仓/官方父知识库初始与前轮精确。7fd重挂overlay只st_dev变化，原30文件inode/uid/gid/mode/nlink/size/mtime/ctime/SHA九字段全同，原失败和旧记录留存、当前boot基线另记，产品Root/namespace检查未改；无GWT binding/clock namespace/业务。packages/log扩展覆盖原dpa.properties总31。新Git/private缓存/JDK21完整97包及44包分别单次升级通过；最终44包6.0.0~git202611180229.44b4281-1/SHAe3f8d8fa…02c407。

原同现场enabled-runtime/enabled在97各自单次正常remove/reinstall 0/0，同目的68bd1736…、16c8ecb7…完成，32→33→34库存。97 masked-runtime remove0、reinstall在clock拒绝exit1/half-configured，原2041f9fd…/mask/全部34外部与retained完整指纹保持；只发布新Git完整44的独立8文件clock code cohort、key manifest及原维护不动，单次configure同原目的完成，库存35。该有界修复不算通用/未修改产品通过；随后完整44正常masked-runtime升级0/库存36，真正未改产品脚本masked-runtime/masked各自remove/reinstall 0/0，3793c1ea…/7b160323…同目的完成、36→37→38，原mask inode/属主/模式/链接/mtime/ctime保持。原外部完整/8Core数据逻辑及各副本精确，Rootcanary/.data历史/原UID/钥保持，无Core/key/clock业务，Main从未启动。每次仅正常package-history追加。

四profile验证后单独恢复夹具原disabled停放选择，非产品修复；最终ExecMainStartTimestampMonotonic0、key完整generation及clock canonical/actor/effective配对通过，actor protocol324441c5…/manifest文件SHAfb0da9f0…分别记录。原7fd健康安装完整44、38输入/14正常回执、停止exit0；独立命令探针也停止，不重初始化或重放本轮已完成目的。02:39UTC原3/个人25/C/D/E原数据/身份/配对/进程/ON保持，两VM实时stopped、旧+1ms未重放；E Main仍11/13caps、NNP0。见[检查点](core-main-boot-choice-matrix-checkpoint.json)、[审查](../../reviews/core-main-boot-choice-matrix-review-20261118.md)。

继续static/linked实际Core/legacy转换、purge/通用中断/半配置恢复、全部六组件/provider/launcher/资源/平台矩阵、完整Core合法Main/RTC/Chrony/其余typed brokers/API/R017/GUI/网络/密钥/CA/数据库/上游及发行12父项，不缩目标。仅原3external deferred、冻结完整12父项SHA不变，最终0/12、无加权百分比，G034/协议未封签发布。goal active/现有积分继续，无push/JDK27/活动runtime清理，旧UNKNOWN/丢失CSR不重发换号跨Root查，闭环密钥/900秒等不重放。

## 2026-11-18 Core 正常保留重装与 NM 授权撤销

代码38a983787e。原7fd的1fab失败维护先在原Root/ID/代码/副本/身份/27输入完整核对后做自有夹具局部续接，从未到达账号回调开始，不重做key/clock移除和数据复制；第一次只读预检因dpkg暂态half-configured拒绝，另次发布因dpkg改脚本755拒绝，账号回调仍0。指纹上下文最终移Root0600；之前该夹具脚本短暂可读的仅测试元数据，无密钥载荷。原日志不改，此修复不作通用中断恢复。新Git/private缓存/JDK21的14f单次正常保留重装exit0，原1fab由产品同ID完成，总28输入；原身份/密钥/data/Main disabled保持。

发现并修复Core残留kura-nm.rules网络授权：固定原精确字节/Root父目录/属主/单链接/稳定读，未知site规则不删不采纳；独占完整发布，安装前check，卸载用Root bin清规则，补错误传播。63宿主回归、12真实LinuxRoot条件过/无skip；最初自查readlink改atime的测试失败留存、未回填时间。新Git完整38a ARM64包6.0.0~git202611180130.38a9837-1/SHAe365e1ab…a1c2caf在同一原现场正常升级/移除/保留重装各单次exit0，未改产品脚本。原29输入22外部完整/7Core逻辑及retained副本精确，Root0/0/0600 canary和历史.data保持，新正常remove维护4f52e089…由产品同ID完成，总30。Main始终disabled/inactive/PID0/NRestarts0、最终ExecMainStartTimestampMonotonic=0，clock namespace/operations/key业务/startups0，原UID/clock目的保持。正常package本地key副本不等于重放Root备份。

真实polkit、同一UID997/cap0/NNP1/PID1574/ticks22949568 subject对无handler fixture action，移除前0/移除后1/重装后0，daemon原PID/ticks/NRestarts0，0网络写。subject/action清理，私有上下文0600；正常原7fd健康停止exit0，安装38a/30输入保留，不能重初始化。1fab/4f52已完成，不重放这两条验收。见[检查点](core-retained-reinstall-policy-checkpoint.json)、[审查](../../reviews/core-retained-reinstall-policy-review-20261118.md)。

01:39UTC原3/个人25/C/D/E原身份/数据/进程/ON保持；两VM实时stopped、旧+1ms未重放；E Main仍11/13caps、NNP0。文档前18仓干净同分支仅Core38a变更，冻结12父项完整SHA不变。继续purge/legacy/通用中断/其他boot profile、完整Core/当前合法Main、六组件/provider/launcher/资源/安装升级矩阵、RTC/Chrony/其余typed broker/API/R017/GUI/网络/密钥/CA/数据库/上游发行及全部12父项；仅原3external deferred，最终0/12、无加权总百分比。目标active/现有积分继续，无push/JDK27/活动runtime清理；旧UNKNOWN/丢失CSR不重发换号跨Root查，闭环密钥/900秒不重放。

## 2026-11-18 Core 保留卸载候选与实际失败

4b8171c新增.data独立保留、卸载回调wait/失败传播、原remove fence/HMAC/unit摘要/UID绑定的Main boot选择保留，以及保留数据时不回收kura UID。52项当前定向回归过，含后续安装目录删除后仍执行已安装Root bin helper。新Git/private缓存/JDK21完整构建4b ARM64包6.0.0~git202611180104.4b8171c-1/SHA2014aec5…f1e2a1，同原7fd8…唯一一次升级exit0；原26输入19外部完整/7 Core逻辑字段保持，新增1正常package-history，Main仍disabled/PID0、clock namespace及业务0、key业务及Core启动0。

唯一一次真实remove exit1：已保留完整数据/disabled选择并移除clock/key listeners，新增prerm误调postinst已删除的install回调，尚未进入账号回调。这是本轮引入错误，源码b509d5d3f7恢复Root bin路径并加回归；新修正尚未重建或安装。失败不记通过、不重发remove。原27输入20外部完整/7 Core逻辑字段及对应retained副本再次精确，产品原Root/fence/HMAC/身份验证通过，原账号/目的保持。原remove维护1fab699a…与retained/canonical双份状态及clock维护均保留，同容器正常停止exit0，不能重初始化/改ID/清副本；先独立核对已完成步骤并做可审查精确续接，普通升级不能制造第二份preserve。

01:09UTC原3/个人25/C/D原Main3135及E原身份/配对/全部进程/ON保持，VM未启动、旧+1ms未重放。见[检查点](core-retained-remove-checkpoint.json)、[审查](../../reviews/core-retained-remove-review-20261118.md)。实际保留重装、修正版remove、purge、中断/legacy/所有boot状态与授权清理仍内部pending，完整Core/六组件/provider/launcher/安装升级矩阵及全部12父项未缩减。最终0/12闭环，无可靠加权总百分比；时间只作数周、可能超过一月的粗估。仅原3external deferred，G034/协议未封签发布；目标active/现有积分继续、无push/JDK27/活动runtime清理，旧UNKNOWN/丢失CSR及闭环密钥/900秒不重发换号跨Root查或重放。

## 2026-11-18 当前 actor 实际权限与 Core 升级数据保留

Core b49eb6cb20修复升级强制enable，e1c40469d9修复历史.data恢复及数据属主，合同aa8f086b50。原VM按机器/原namespace/APPLIED/sender/未启probe核对后，沿原维护1284ae68…只归档发布新Git a54的7代码/unit/JAR及manifest，产品install/resume不重身份、不复制真实数据/重建namespace。实际actor cohort07212c1f…，PID4181/ticks41703，UID992/GID970/组970/五cap仅SYS_TIME/NNP1/NRestarts0，通过当前有效unit自配对；一次无ID的Root context被UNAUTHORIZED。原namespace/operation/sender精确、操作数仍1/新clock0，合法Main业务未验。10生产文件与当前逻辑相同；正常quiesce后停机维护4322e5d4…保留同namespace/当前cohort，VM正常poweroff回stopped，旧probe/+1ms未重放。非完整deb/OSGi安装。

Main新逻辑保留已有标准位置unit的enable/mask链接，fresh reload后enable且错误传播，消息不再强制承诺自启；5合同过。原7fd8…同现场从9066到b49新Git/private缓存完整重建包6.0.0~git202611180028.b49eb6c-1/SHAca68e097…669fd3唯一一次升级exit0，Main仍disabled/PID0无后补disable；17外部完整指纹、4权威user逻辑字段精确，2历史副本mtime刷新实录。其源恢复new默认优先/快照强刷及chown-R风险已修：旧.data优先、既有历史快照/链接不重写、缺失填充失败传播、真实数据属主不递归改，Root代码独立libexec/code-only；5合同过，不重跑/累加旧49Python/57JVM。

在该自有夹具归档两原副本、明确追加公开历史标记和Root0600数据canary，权威快照/密钥未改；原始失败/时间不回填。b49到e1新Git完整包6.0.0~git202611180039.e1c4046-1/SHA5df08c91…db66d5唯一一次升级exit0：原25输入，18外部dev/inode/uid/gid/mode/nlink/size/mtime/ctime/SHA精确、7Core数据历史uid/gid/mode/nlink/size/mtime/SHA精确，Root0/0/0600及2不同历史内容保留；Core复制inode/ctime差异实录，NOATIME自查不声称atime一致。每次新增1正常package-history旧回执留存。Main一直disabled/inactive/0，clock actor未启/namespace及operations未创建/业务0；原账号目的2a388683… UID995/GID988保持。最终e1 actor cohort a16faa5e…/manifest文件1798b28d…分开。新7fd8…正常停止保留全部数据/标记/属主/回执，不重当空主机、重初始化或误启Main；私有日志指纹仅本地0700/0600。

00:44UTC原3/个人25/C9及进程计数/D29原Main3135、E原配对/身份20/Core6/local7/material5/switch9/transport7及原全部进程/ON保持。文档前18仓仅预期Core提交，冻结完整12父项SHA不变。见[检查点](clock-actor-core-upgrade-preservation-checkpoint.json)、[审查](../../reviews/clock-actor-core-upgrade-preservation-review-20261118.md)。当前联合包合法Main/完整CoreOSGi、六组件/provider/launcher/模式资源平台、e1首装/其他enable状态/保留数据重装移除purge恢复中断、RTC/Chrony/其余typed broker/clock UNKNOWN容量及全部API/R017/GUI/网络/密钥/CA/数据库/上游/发行12父项仍内部pending；E实际Main11/13caps/NNP0。仅原3external deferred、最终0/12不等于实现0%、无加权总百分比，G034未封存/协议未发布。目标active/现有积分继续、无push/JDK27/活动runtime清理；旧UNKNOWN/丢失CSR不重发换号/跨Root查，闭环密钥/900秒/原clock+1ms不重放。

## 2026-11-18 Clock 有效 unit 与完整 Core 实际首装

Core9066ce7996/测试817ff3cfa2/合同a54af33949：从PID1真实UnitPath核对完整Root加载目录，拒绝control/attached/generator、前缀/类型drop-in、替代fragment与依赖目录；权限拒绝不当不存在。加载Id/Names/LoadState/FragmentPath/SourcePath/DropInPaths/NeedDaemonReload/Transient须配对，异常缺失/重复关闭准入。安装前先查全部unit输入；resume保留原维护gate直到产品reload、加载配对、启socket及复核成功。原密钥Root配对不改。49项Python全过/新增11方法，Java/manifest/DS未变，前57项不重跑/不累加。新systemd255.4 ARM64夹具60条件通过/37拒绝23接受恢复，含12实际加载目录、真实set-property/global/prefix/stale；cap0/NNP1 sleep仅After引用用于保留inactive fragment，无SYS_TIME/clock namespace/请求，actor/socket未启动，夹具停止保留。

新Git/private Maven缓存完整重建support/Core/tools/distrib，ARM64包6.0.0~git202611172300.9066ce7-1/SHA c6a53072…bb918e、203543854字节。manifest文件SHA57b7f686…7e442c与协议actor cohort cc0a32d5…43efaf7分开，JAR28ea41f1…65b11b。Root7载荷/guard/维护shell/Main依赖通过；第二全新依赖齐全Ubuntu ARM64/JDK21现场完整包唯一一次dpkg首装exit0/installed，实际固定别名/两套代码/账号/已加载unit配对通过。UID995/GID988仅自己组，状态0700/空lock、无namespace/operations；Main/actor inactive/PID0/NRestarts0，socket listening/连接0。原维护目的5e33127e…实际quiesce后control override使resume明确拒绝，原gate字节及指定元数据/目的精确；仅撤销自建输入后，产品自身reload沿同一目的恢复，23份Core/密钥/账号文件及原进程保持（NOATIME自查、不断言atime），未发clock/密钥业务请求。新7fd8c6a0…现场随后仅为停放禁用Main并正常停止保留，不能重当空主机；普通boot/完整CoreOSGi/当前actor实际权限进程仍未验。

原PID1/复制属主及777/stale/宿主os导入失败均留记录；os异常发生在dpkg前，核实实际分发0后沿原容器/包单次执行，无重身份或盲改真实数据。私有安装日志/指纹不进Git/知识库。23:16UTC原3/个人25/C9及进程计数/D29原Main3135、E原配对/身份20/Core6/local7/material5/switch9/transport7及原全部进程/ON revision保持，Parallels未启动/原+1ms未重放。文档前18仓仅预期Core提交、冻结完整12父项SHA不变。见[检查点](clock-effective-unit-install-checkpoint.json)、[审查](../../reviews/clock-effective-unit-install-review-20261118.md)。

完整Core/OSGi/实际非Root actor、六组件/provider/launcher/模式/资源平台、升级移除purge恢复中断、RTC/Chrony/其余typed brokers、clock UNKNOWN容量及API/R017/GUI/网络/密钥/CA/数据库/上游/发行全部12父项仍内部pending；E实际Main仍11/13caps、NNP0。最终0/12闭环不等于实现0%，无整体可信加权百分比；仅原3external deferred，G034未封存/协议未正式发布。目标active/现有积分继续、无push/JDK27/活动runtime清理；旧UNKNOWN及丢失CSR不重发换号/跨Root查，闭环密钥和900秒不重放。

## 2026-11-18 Java NTP 独立时钟 actor 与新 ARM64 包

Core实现b72fa0de7f、固定安装别名修复0f409edde9、合同43204a916a：Java NTP壁钟写改用独立非Root kura-clock/仅CAP_SYS_TIME/NNP1 typed actor，不加入密钥组；Root精确配对4helpers/2units/实际Clock JAR与专用只读代码，双方状态隔离。socket只准当前Main固定helper直接子进程；只传偏移，原namespace/UUID prepare/issued/commit/result持久化、最多一次commit，UNKNOWN/UNSEEN不重发换号/跨Root查，不回落date。按当前realtime补偿RPC耗时，同boot/30秒样本门禁；失败/旧恢复样本不发当前事件或写RTC。独立账号/安装目的、维护guard与保留日志路径已接入；固定canonical别名只接受Root单跳/同父匹配版本真实Root目录，不放宽其他链接/原Root配对。新前缀显式755，既有目录不自动改属主/权限。

新源码JDK21/Maven3.10.1重建，57项JVM/Core33+Clock24无跳过、当前Python38项和Linux固定别名8项通过；重叠不求和。既有Parallels独立新组件实际ClockServiceImpl：JVM cap0/NNP1，actor仅SYS_TIME/NNP1；一次guest-only +1ms，namespace a23fb4e9…、原请求c35584c9… APPLIED/errno0/ClockEvent1。Root/非Main peer、额外代码/标准unit override拒绝，公开canary直达与proc别名EACCES、未读真实钥；后续实际观察事务数1/原文件精确，首canary固定计数不作证明。组件quiesce正常停机，原数据/维护目的保留，VM正常poweroff回stopped，不重放该请求。新真实Git0f409检出/private Maven缓存完整重建support/Core/tools/distribution，ARM64包6.0.0~git202611172245.0f409ed-1/SHAe88f8302…a6313，7载荷/父目录Root权限摘要、内嵌guard/维护shell/Main socket和遮蔽核对通过；两生产类与VM字节相同，但完整JAR/cohort不同、完整包未安装/运行。b72旧包缺固定别名修复，仅历史保留。

22:45UTC原3/个人25/C9及进程计数/D29原Main3135精确；E原配对/身份20/Core6/local7/material5/switch9/transport7与原native29323/Core29609/key4714/recovery4640/NM4668/ON revision5e6e429f…保持，未部署候选。文档前18仓仅预期Core提交/新合同，冻结完整12父项SHA不变。原预检/GECOS/父目录、Git/工具target和检查器错误均留原失败，未重身份/放宽门禁。见[检查点](core-clock-broker-checkpoint.json)、[审查](../../reviews/core-clock-broker-review-20261118.md)。

完整Core/OSGi/有效unit-profile/安装升级移除恢复与真实故障矩阵、RTC/Chrony和其余typed brokers/provider/launcher、clock UNKNOWN管理/容量、六组件/资源平台/R017/GUI与全部12父项仍内部pending；实际Main仍11/13caps、NNP0。最终0/12闭环不等于实现0%，整体无可靠加权百分比，仅原3 external deferred，G034未封存/协议未正式发布。目标active/现有积分继续、无push/JDK27/活动runtime清理；旧UNKNOWN和丢失CSR意图不重发换号/跨Root查，闭环密钥及900秒不重放。

## 2026-11-18 Core 同身份普通命令前置

Core本地代码84d6a58新增仅限同一OS非Root身份、进程/当前线程身份及附加组一致、五cap0/NNP1的直接普通命令路径，不信任user.name；Root/保留身份/不同用户/NNP0/非零cap/缺失异常字段仍走原su。固定env/timeout/argv、显式shell和同步异步语义保留，不自动改command user。JDK21/Maven3.10.1从Git与明确差异导出全新源码并重建test-support/API/util/Core，33项JVM全过/无跳过，含9新方法；8条Linux ARM64/JDK21真实条件通过，含实际同步/异步/参数环境/超时及Root、NNP0、CapBnd、附加Root组、用户名伪造/不符拒绝。五类cap逐项为JVM合同检查，不外推全部真实cap组合。首次只读rootfs复制夹具失败发生在Java前，保留并修正为从未运行的暂存容器封存测试镜像。

原3/个人25/C9进程计数/D29原Main3135再次精确，E原配对/身份20/Core6/local7/material5/switch9/transport7及原native29323/Core29609/key4714/recovery4640/NM4668保持，原ON revision5e6e429f…精确。只读观察器误用Root授权reader读取UID997状态的拒绝保留，按真实属主独立观察后通过，未放宽产品检查。文档前18仓同分支干净，仅Core预期提交，冻结完整12父项SHA不变。

见[检查点](core-unprivileged-executor-checkpoint.json)、[审查](../../reviews/core-unprivileged-executor-review-20261118.md)与Core合同。本次尚未打deb/部署/降低service权限；实际Main仍11/13caps、NNP0。完整typed broker及全部provider/不同command user/launcher、六组件/安装升级移除恢复/平台资源/R017/GUI和全部12父项仍内部pending，最终0/12闭环不等于实现0%，整体无可靠加权百分比。仅原3external deferred，G034未封存、协议未正式发布；目标active/现有积分继续、无push/JDK27/活动runtime清理，旧UNKNOWN和闭环密钥/900秒不重发换号重放或跨Root查。

## 2026-11-18 同E业务保留实际升级与配置读取

原E已实际安装代码eab6821/包0.1.0~native202611171954.eab6821-1（SHA0dc111eb…3f5b0/source0a5eb472…d157c/ELFc03bbb72…3691）。新维护OFF目的bda873d9…在原ON bc29b319…已核后写前留存；第一次会话过期时实际RPC发送0，同目的恢复认证后单次HTTP200//OK，OFF revision03991e78…。source **b479b232-38d2-436d-9566-3f0e33c25bb3** 各一次prepare/install APPLIED，不自动启动。原加密SUCCEEDED业务及72停止态资产精确，原身份20/材料/Core6/local7/material5/switch9/transport7和所有权限范围保留，不重签/重绑定/重授权/重配。安装基线无WAL/SHM，未补造/截断/checkpoint；不能外推非空WAL或UNKNOWN安装。

显式恢复后native29323/ticks21272446 UID/GID997/groups996/997/五cap0/NNP1/NRestarts0，Core29609/ticks21272461仍11/13caps/NNP0内部待办；key4714/recovery4640/NM4668原ticks不变。原socket enabled/listening/NConnections0/NAccepted4、6精确。新恢复ON目的2aa54a8d…单次GUI确认/HTTP200//OK，当前 **ON revision5e6e429f-13e4-4066-a61e-2bb218c51cc1**、CONNECTED/activeRequests0，同证书3681eb86…/generation1/11能力，新epoch771630d7-7f22-4617-a1db-3a56fc26f8ce。原Source/grant结果remoteEnabled=false只描述原动作，不能误判当前OFF。

GUI一次配置清单：原keyda951403-bdcb-42e7-8733-57bd3b31349c、task2841b777-1f58-43ba-a5ef-b949c4993062，SUCCEEDED/19 PID。选ClockService时工具超时自动重置浏览器，详情purpose/dispatch/ack尚不存在，PG/Native仅原status/list2条，未重发清单。恢复原认证会话后，独立受控API一次get：key97cd0e4b-55ae-4c97-af85-6e88f10f9dbd、taskd2fc3e47-1ec4-4458-bbdc-b1f0c48eef94，SUCCEEDED/13字段/英文元数据，结果在任务GUI查看；**GUI表单提交未验**。配置选择未跨客户端恢复，继续内部pending。writable=true是组件支持，实际updateEnabled=false且不宣告update capability，未写配置；两UNKNOWN字段因NO_METATYPE隐藏，不外推所有secret/provider。

PG/API/Native对齐三条原ID/op/key/SUCCEEDED/generation1。Native真实加密库存认证后，仅专用UID997/cap0/NNP1在副本读取元数据；原status7e30af94… hash78ab76b2…、加密114/153字节不变。当前真实WAL32992字节SHA4ea7fd8f…e7960承载合法新读取，schema70be30f7…/logicald1a5a1e2…，不导出载荷/密钥、不prune/recover/重放，也不把安装后WAL当安装验收。20:37:56UTC原3/个人25/C9原进程计数/D29原Main3135再次精确；文档前18仓与phase24完全一致干净、冻结12父项SHA不变，本轮只改文档，无实现/旧测试重跑。

见[实际检查点](native-retained-business-installed-checkpoint.json)、[审查](../../reviews/native-retained-business-installed-review-20261118.md)。继续真实非空WAL/RUNNING/UNKNOWN/renewal/legacy source、全部provider/launcher/安装升级/移除恢复/R017/typed brokers/Core最小权限和GUI及全部12父项。仅原3 external deferred，其他内部pending；G034未封存/协议未正式发布。无push/reset/clean/JDK27/活动runtime清理，旧UNKNOWN和C丢失CSR ID不重发/换ID/跨Root查，闭环备份/归档/轮换/恢复/900秒不重放。新浏览器保留默认拒绝写入保护，私密恢复状态不进Git/知识库，Node不要reset，定位器超时短于工具超时。

## 2026-11-18 真实业务只读检查与安装维护候选

agent eab6821已接入独立保留业务source事务，文档0755565；原空业务入口继续拒绝非空表/WAL。新路径仍检查原初始化、完整Root代码/unit/source配对、原local/material权限、首次完整身份、remote OFF、停止/排空及writer锁；RUNNING/UNKNOWN只保留、不重发。所有公开source prepare/status/complete增加必需--initialization-id，原pending不允许同包另换ID。只复制加密DB/WAL到新/run夹具，原钥不复制；准入/发布绑定原库存、输入摘要和全部状态指纹。续期/多候选/legacy/未知provider文件等内部pending保持。

真实UID997夹具32,992字节WAL及SUCCEEDED/RUNNING/UNKNOWN各1认证通过，原目录及5文件字节和含atime元数据精确；Root/错误UID/非零cap拒绝。旧atime失败保留并修正原库SQLite打开，没有回填时间戳。当前Python284选择/282过/2历史导出合同跳过，含18新方法；选择重叠不累加，模拟检查器的事务夹具不等于真实安装。当前Git新包0.1.0~native202611171954.eab6821-1（SHA0dc111eb…3f5b0/source0a5eb472…d157c/ELFc03bbb72…3691）及4控制脚本通过，协议本次重新从声明Git1c5842a打包；新核验使用真正拥有方crates/management-protocol，旧错误路径回执不改。

19:56 UTC同E完整候选配对后的实际Root Python/非Root sealed memfd检查通过：原库69,632字节、executions1/SUCCEEDED1、其余6业务表0、generation1，原字节及含atime元数据精确，无source intent/安装/服务/任务/remote写入。检查前已无WAL/SHM，不归因原因，也不冒充非空WAL现场升级；原12,392字节WAL旧观察保留。新代码仅暂存/opt/kura-retained-business-inspection-f6e198ad-6752-4d90-88f0-c6074b8787f1，未分配source request。E仍24552f6/native23882/Core24168/NRestarts0，原已知ON bc29b319…未改。19:58 UTC原3/个人25/C9原进程计数/D29原Main3135再次精确；18仓同分支，冻结完整12父项SHA不变。

见[检查点](native-retained-business-checkpoint.json)、[审查](../../reviews/native-retained-business-review-20261118.md)。下一步继续同E实际source维护：独立已知ON后维护OFF目的写前留存、单次GUI确认；停止排空后重取基线，原source目的各一次prepare/install，UNKNOWN原ID/原Root查询及发布续接、不重装。随后继续完整六组件/provider/launcher/安装升级矩阵和全部12父项。最终仍0/12闭环，不等于实现0%，整体无可靠加权百分比。仅原3 external deferred；其余内部pending。闭环备份/归档/轮换/恢复/900秒不重放，无push/JDK27/活动runtime清理，界面改进仍次要，本轮未改client JS。

## 2026-11-18 Netplan能力范围与真实状态任务

agent实现5f95a7e/测试修正24552f6：Hello清单复用network固定接口范围，无后端不宣告network.*；有后端/空范围仅inspect、transaction、takeover.plan；显式非空范围才宣告四个既有写能力。实际空范围E旧Hello误宣告写能力但执行门禁仍拒绝，本次只修宣告，不授新权限或认领数据。transport18过/5显式故障夹具ignored、network35过/0ignored，含4新方法；首次未引入tempfile编译失败保留，改现有临时目录方式无新增依赖。Python/Root相对3e28a44逐字节不变，沿用已验266/264过/2历史排除证据，本轮未重跑、不累加。

当前Git自包含构建新包0.1.0~native202611171836.24552f6-1，SHA239eac99…41f0fc/source44a72498…2ead9/ELF03b87f80…7baa9，协议本轮从已核验声明Git检出重新打包、bridge只读当前Root配对E。已知ON后的维护OFF目的ecddaf54…一次HTTP200//OK，revision790a3e0b…；source8a93b64a-0c41-4816-b092-29eb683f783e各一次prepare/install APPLIED、不自动启动，72停止态资产精确，原switch49e58164…的9证明、transport2031e3d3…的7证明及原身份/材料/env保留，不重授权或配置。显式恢复后native23882/ticks20724103 UID997/五cap0/NNP1；Core24168/ticks20724118仍11/13caps/NNP0内部待办，key4714/recovery4640/NM4668不变。

恢复目的a2cb7bad…确认后一次HTTP200//OK，当前ON revision bc29b319-608b-4920-bef3-9a161b0f5a11/actor kura-uid:996，同证书3681eb86…/网关native-e-a60111a6/generation1，实际新epoch7787b274…；中心Hello15→11，四个未授权网络写能力消失，读取/核定保留。中心Query status只发1次，原key4f46051a-eeff-4aef-ae9d-f8debfd82a51、task7e30af94-6bcd-40cb-a0e4-af475360cab6，202入队后仅读原ID；中心/API/网关现行executions均SUCCEEDED，agent/kura=true。中心1网关1任务，native executions1/旧results及其余5表0，加密plan114/result153字节未导出；WAL12392字节合法保留，不截断/checkpoint/清数据凑准入。后续source必须先有保留真实业务的显式维护，当前空业务门禁不放宽。

初次只读核验误用Fetch.status方法、jsonb长度函数和旧results表，拥有方journal.claim/finish及正确只读查询核对后通过，失败回执保留，无任务重发或产品日志修补。两端UI连接/任务Completed，CSR/stage/bind/登记总各1，页面错误0。原身份20/材料32/Core6/local7/material5/native selected5及grant9/transport7精确；18:46:34UTC原3/个人25/C9进程计数/D29原Main3135再次精确，文档前18仓干净、冻结12父项SHA不变。

见[检查点](native-capability-status-checkpoint.json)、[审查](../../reviews/native-capability-status-review-20261118.md)及agent Netplan能力合同。只证明agent.status任务，继续实际configuration等受限读取、真实业务/续期/TLS/UNKNOWN/source及升级移除恢复、typed brokers/Core最小权限、完整能力/Mode原目的UNKNOWN/地址GUI、六组件/provider/launcher/R017和全部12父项，G034未封存，仅原3 external deferred。目标active/现有积分继续、无推送、不加27、不清active runtime，旧UNKNOWN不重发/换号/跨Root查，闭环备份/归档/轮换/恢复/900秒不重放；仍0/12父项最终闭环，不折算实现0%或编造整体百分比。

## 2026-11-18 同一原身份的首次 native mTLS 通道

agent 3e28a44 新增独立 Root gRPC 连接地址配置：15字段/原安装-peer-result-角色/同网关及原回执/最低generation、同HTTPS host严格端口；原材料18443保留，连接18444。Root全局原意图先存、原号补全、result/公开/current/ACK完整，停机排空/远程OFF/业务0/原完整首次身份及全部Root配对准入保持，不改.data或重身份。首次u16类型编译错误及65536端口解析缺陷修复后，266 Python中264过/2历史导出夹具排除（16新方法）；干净source Rust身份15/15、enrollment/renewal30过/4显式Root/cross-UID夹具ignored，未算通过、不累加。原失败保留。

当前Git重新导出/协议重新从声明Git打包、bridge只读当前配对E；新包0.1.0~native202611171817.3e28a44-1，SHAbb3230eb…3c13/source0cfe4cdf…72135/ELFf53f7994…1bdaa，四控制脚本编译。同E source a12f9c21-c02c-4c7a-a63e-99cbfc4a6c8f仅一次准备/安装APPLIED、不自动启动；独立Root地址配置2031e3d3-a90b-4ad8-8445-f5a5138ecb0f一次APPLIED/READY。65停止态资产精确，原grant49e58164…的9证明与新配置7证明启动后精确；原init/CSR/stage/bind/register/身份材料及env保持。本地记录文件名冲突在任何Root分发前O_EXCL拒绝并保留，未覆盖旧证据。

native20323/ticks20606914 UID/GID997、五cap0/NNP1/NRestarts0；Core20608/ticks20606930仍11/13 caps/NNP0内部待办，key4714/recovery4640/NM4668不变。原身份20/材料32/Core6/local7/material5精确，7业务表/Root业务启动0。沿原目的8808df94…与OFF原revision148047bd…，先核Root namespace/source/route、写前留存及单次guard，再真实GWT一次开启，HTTP200//OK；当前ON revision c693589f-03e5-4dc1-be19-56bd87363ea8/actor kura-uid:996。agent真实CONNECTED，中心同证书3681eb86…/同网关/generation1、epoch4b8e3040…、18:25:27 UTC心跳seq16在线且Kura可用，1网关/0任务。两端UI在线/截图，CSR/stage/bind/登记总各1、页面错误0；不把独立TLS探针当真实流。Root授权/config回执false仅指其动作不启服/开通，当前ON另有remote决策与live证据。

见[检查点](native-first-mtls-checkpoint.json)、[审查](../../reviews/native-first-mtls-review-20261118.md)及agent地址配置合同。18:25:51 UTC原3现场/个人25/C9及进程计数/D29原Main3135只读精确，文档前18仓干净、冻结12父项SHA不变。实际受限只读任务、能力宣告与实际授权、开关跨刷新原目的/UNKNOWN和地址GUI完整生命周期、保留业务/续期/TLS/UNKNOWN/source/旧态迁移、typed brokers/Core最小权限、六组件/provider/launcher/安装升级移除恢复/R017及全部12父项仍内部pending，仅原3 external deferred。全部12父项最终闭环0/12不等于实现0%，没有可靠加权总百分比；目标active/现有积分继续，无推送、不加27、不清active runtime，旧UNKNOWN及闭环密钥/900秒不重发重放、不跨Root查询。

## 2026-11-18 独立本机开关授权与主机入口

agenta7e5d3c新增 Root 原号/原意图/前后环境/公开证明/ACK/current 与同号补全，Rust严格验证13字段及环境一致，先核本机namespace再查旧号；原Root代码/peer/material/source检查保持。consolea88ad2f新增固定主机bridge及双语表单、发送前公开原号留存/Web Lock、跨Root/双tab/存储失败/确认后identity-source变化保护。250 Python中248过/2历史夹具排除，含23新方法；当前Git source Rust身份10/10；console打包129/129；最终浏览器60/60含13新场景与双语390px图。原失败均保留，计数重叠不求和。

新包0.1.0~native202611171716.a7e5d3c-1，SHA7175a667…184cb/sourcefb2a9b26…10476a/ELFdb55b68b…5c92ba；协议从声明Git重新打包SHA61163caf…e70c2、与现源码/清单/锁一致，bridge只读当前Root配对E，不用旧中间产物。原E source a8226dd5-fe5e-4631-a9ea-76bb89ee3130一次准备/安装APPLIED、不自动启动；56停机资产/原身份及权限精确。显式恢复后独立本机开关授权49e58164-08a1-4511-8b61-ab0e585c2467由固定Root控制器一次APPLIED/READY，仅env本机开关权限0→1，通道仍OFF，原grant/初始化/CSR/stage/绑定不重做。授权停机58对象中57原样/9新证明启动后精确；新增空WAL/32KiB SHM为原协议支持，固定数量及只读观察器脚本/目录/context失败均保留，未改产品准入或重发授权。

最终native16554/ticks20298790 UID/GID997、五cap0/NNP1/NRestarts0；Core16836/ticks20298805仍11/13 caps/NNP0待办，key4714/recovery4640/NM4668不变。原身份20/材料32/Core6/local7/material5/native selected6精确，agent/kura=true，7业务表/Root业务启动0，两socket监听/连接0。原GWT现用密码登录读回：开关确认可用、未勾选/未开启，remote原revision148047bd…OFF，CSR/stage/bind/登记各总1次，页面错误0。主机installer新包529f4699…541dd/20 critical files/无maintainer脚本仅构建；Cockpit/PAM实机及更轻网关依赖仍内部pending，110新增包/1 Perl升级仅预演未安装，不外推夹具为实机。

见[检查点](native-kura-switch-checkpoint.json)、[审查](../../reviews/native-kura-switch-review-20261118.md)与agent授权合同。继续先核GWT通道变更的原revision/发送前留存/UNKNOWN，再真实mTLS/心跳/只读任务；完整保留业务/TLS/UNKNOWN/source/旧态迁移、typed broker/Core最小权限、六组件/provider/launcher/安装升级移除恢复/R017与全部12父项均内部pending，仅原3 external deferred。目标active/现有积分继续，无推送、不加27、不清active runtime，旧UNKNOWN/闭环密钥/900秒不重放、不跨Root查询。

## 2026-11-18 原首次身份与材料的联合source维护

agent代码fd771f1/81c1e9b新增独立enrolled-material source动作，仅原授权/Core-native停机/两socket停用排空、远程关、业务0及单一完整首次身份。Core上下文/CSR/stage、native指针/候选/完成回执/独立副本严格只读盘点，类型/权限/链接/哈希/key-CSR-证书与到期一致；不重身份、不认领修补或清理数据。原摘要/完整指纹入不可变source意图/结果，准入核对原意图，丢摘要拒绝；控制入口锁争用立即BUSY。227 Python中225过/2历史夹具排除，含17新方法，前次计数重叠；实际只读先核32文件，不外推完整运行故障矩阵。

当前Git锁定离线新包0.1.0~native202611171614.81c1e9b-1，SHAc5744ad7…b0a22d/source0ce25b5d…9de9da；协议从声明Git1c5842a重新打包且与当前源码/清单/锁一致、SHA61163caf…e70c2，未正式发布；未变bridge直接读当前配对E，非旧中间产物。ELF334a3fb8…53877c/unit1a1579c7…736ca不变，4脚本编译。首次OCI参数过长在Python启动前拒绝，后续只读缺helper失败保留；确认目录缺省/Main不变，同原source df26b4ab-9b0a-434f-a8d3-1168cc63f954改stdin流，准备/分发各一次APPLIED、不自动启动。56停机资产/原身份20/Core6/local7/material5精确、原init/grant仍APPLIED/READY，仅代码绑定推进，权限/身份/原回执不改。

显式恢复后native12359/ticks19860202 UID/GID997、五cap0/NNP1/NRestarts0；Core12644/ticks19860220仍11/13 caps/NNP0待办，key4714/recovery4640/NM4668不变。身份20/材料32及现用私有精确、agent/kura=true、7业务表/Root业务启动0、remote原revision148047bd…仍OFF。现用密码GWT重登录及原CSR7c3361b3…/stage adf0acc7…/绑定回执读回；CSR/stage/bind/登记各总1次，页面错误0，不重放身份或闭环密钥验收。

见[检查点](retained-enrollment-source-checkpoint.json)及[审查](../../reviews/retained-enrollment-source-review-20261118.md)。本次后原3/个人25/C9进程计数/D29原Main3135只读精确，18仓文档前干净、完整冻结12父项SHA不变。继续独立native Root远程开关授权及产品UI、实际mTLS/心跳/只读任务、保留续期/TLS/业务/UNKNOWN/旧态迁移、typed broker/Core最小权限及完整六组件provider/launcher/安装升级移除恢复/R017/平台资源矩阵与全部12父项，均内部pending，仅原3 external deferred。目标active/现有积分继续，无推送、不加27、不清active runtime、不重发旧UNKNOWN或跨Root查询。

## 2026-11-17 同E真实材料FD接收及中心公开证书登记

Core50855bc/agent代码a3b13ea/GWT6353614保持。原E CSR7c3361b3-3073-4394-be1c-e18d359525df公开下载、外部CA一次签发，CA钥/证书/序列文件精确；两次OpenSSL4输出格式预检在签发意图/分发前失败并保留。真实GWT公开材料暂存一次、首次接收一次，叶证书3681eb86…79f1a与stage adf0acc7…76ca8匹配。实际cap0/NNP1 native通过paired relay/publisher，双方socket各新增1接收，10独立副本哈希同源/inode不同；生产Rust完成PKCS8/CSR/证书校验。候选42e96468-bcf9-4940-9b81-0bc4fe70d13f及20私有文件997:997/0600/0700，指针/完成/原回执匹配，真实操作者admin。Core原CSR/上下文/stage及Core6/local7/material5/current2/native其他5精确，仅产品remote原revision正常推进，enabled仍false。预检观察器误查未用目录的历史字段已显式更正、不改原回执。

Core10205/ticks19592759、native9001/ticks19348899均未为此流重启，native五cap0/NNP1/NRestarts0；Core11/13 caps/NNP0义务保留。native7业务表/Root业务启动0。原D界面只登记公开叶证书一次201，数据库1条gateway.register成功审计、generation1、无bundle钥/token；列表1网关离线、无心跳/boot/epoch/任务。中心enabled不等于本机远程开关，后者仍关闭且因缺独立native Root授权禁用。当前source维护仍拒绝真实stage/身份；下一步显式保留身份/证书的联合source维护与独立远程开关授权，不改env绕门禁、不重初始化/重grant/换请求或身份。

见[检查点](native-enrollment-fd-checkpoint.json)及[审查](../../reviews/native-enrollment-fd-review-20261117.md)。本次后原3/个人25/C9进程计数/D29原Main3135只读精确，18仓文档前干净、冻结完整12父项SHA不变。真实mTLS/只读任务/通道、typed broker/Core最小权限、保留TLS/业务/UNKNOWN/旧态迁移、完整六组件provider/launcher/安装升级移除恢复/R017与全部12父项继续内部pending，仅原3 external deferred。目标active/现有积分继续、无推送；旧UNKNOWN及闭环密钥/900秒不重放。

## 2026-11-17 CSR发送前原ID留存与同E真实恢复

GWT6353614新增发送前Web Lock内公开原ID/上下文/网关UNKNOWN记录及读回，缺存储/锁/安全API或损坏记录拒绝新写；超时/丢回复/刷新不重发换号，先核对Core上下文再scope读取，匹配公钥/CSR后OBSERVED并显式确认。Core原目录下0600单链接上下文仅首次授权CSR创建；读不创建、不同Root不查旧ID。native v2菜单单标记/Root保护检查，不降低原Root配对。JDK21新隔离构建、Java11 GWT目标，47 JVM和11浏览器场景通过/页面错误0，覆盖双tab/失败/超时/晚回调/中文390px；初次fixture导航及指纹换行失败保留。浏览器夹具不冒充Root验收。

新GWT包3.0.0~git202611171528.6353614-1在同E仅一次安装，2 JAR Root0644/3 libexec Root0755、5脚本解析；不自动启动。显式Core停启后Main10205/ticks19592759，native9001/ticks19348899原进程保留，cap0/NNP1；Core11/13 caps/NNP0待办仍开。bundle218/DS56 ACTIVE/TLS1.3。首次登录及仅新密码/确认强制改密各一次成功；错误响应端点超时只读确认，没有重发。WAL观察器初次要求文件存在失败保留，仅探针改为缺省或空，不改产品准入。

独立E CSR原7c3361b3-3073-4394-be1c-e18d359525df、上下文43d34f65-3d6c-4f98-af9d-0fd4d6e3e465在POST前已留UNKNOWN；真实公钥52a2f8af…7b6eb2/CSR验签匹配、新文件Core996:989/0700或0600，UI转OBSERVED。真实reload只读scope恢复、总CSR分发1次，原C保留请求不动。Core6/local7/material5/current2/native6私有精确，原grant/init仍APPLIED/READY、远程关。

见[检查点](remote-csr-original-id-checkpoint.json)及[审查](../../reviews/remote-csr-original-id-review-20261117.md)。本次后原3/个人25/C9及进程计数/D29原Main3135只读精确，18仓文档前干净、冻结完整12父项SHA不变。正向真实native证书/FD/crypto/通道、typed broker/Core最小权限、保留态/业务/TLS/UNKNOWN/旧态迁移、完整六组件provider/launcher/安装升级移除恢复/R017及全部12父项继续内部pending，仅原3 external deferred；目标active、现有积分继续、无推送，旧UNKNOWN和闭环密钥/900秒不重放。

## 2026-11-17 保留原材料授权的source联合演进

agenta3b13ea/文档3acb8b0新增独立material联合source动作：Core/native及两材料socket停用并排空、初始空业务/无stage，原env/权限/组/Core/JAR/loader保持；普通/local-only/未装Core动作仍拒绝材料能力。Source前后绑定锚定原local及material结果SHA，原local7/material5证明资产不重写；仅当前native/public与私有relay代码指纹推进。丢材料链拒绝readiness，原fate保留。原号complete只补已准入发布，不重装/重授权/启动；5项初次fixture发布失败及修正保留。210 Python中208过/2历史夹具排除，含13联合方法；Rust未改，既有7/1/30按原范围、4提权互操作未执行，计数重叠。

当前Git自包含锁定离线新包0.1.0~native202611171450.a3b13ea-1，SHAd4077f2b…79f8fa、4脚本编译。宿主回执名冲突在Root分发前拒绝，失败保留；同E原source76dc94bb-fc6f-4ee3-9990-0f8809437fd3一次准备/分发APPLIED，16停机资产/Core6/local7/material5/两socket启用链接含元数据精确。原material13d50bed…结果SHA4247575e…593dbc及peer25abd835…/init24560864…仍APPLIED/READY，权限/env/数据/启用决定不变、不自动启动。

显式恢复后native9001/ticks19348899 UID/GID997、groups996/997、五cap集合0/NNP1/NRestarts0，Core9276/ticks19348922保留11/13 caps/NNP0待办；key4714/recovery4640/NM4668不变。两socket启用监听/连接0，agent/kura=true、bundle216/DS53 ACTIVE；native6/Core6/local7/material5/current material2精确，7业务表及Root业务/启动0、stage缺省/远程关。现场未注入发布故障，不外推fixture到完整故障验收。

见 [检查点](native-material-source-continuity-checkpoint.json)及 [审查](../../reviews/native-material-source-continuity-review-20261117.md)。本次原3/个人25/C9与进程计数/D29及原Main3135只读精确，18仓文档前干净，冻结完整12父项SHA不变。继续GWT发CSR前耐久原ID、正向native材料/通道、typed broker/Core最小权限及真实材料/TLS/业务/UNKNOWN/旧态迁移、联合升级移除恢复/R017与完整六组件provider/launcher矩阵和全部12父项；均内部pending，仅原3 external deferred。原C保留CSR/UNKNOWN及闭环密钥验收不重发重放；目标active、现有积分继续、无推送。

## 2026-11-17 原生材料授权与实际socket拒绝

agent材料实现3ed06c2、Core版本链接修正269cfb3/文档71f5c2e：独立Root原UUID材料授权、严格固定UID/路径/代码配对、Rust保护能力校验；publisher隐藏/run和/proc，仅只读stage绑定。通用拒绝链接规则不变，只专门校验Root拥有的Core版本入口；data/stage拒绝链接及错误属主/权限，不创建认领/修改.data。197 Python中195过/2历史夹具排除，Rust身份材料7/连续性1/注册续期30过、4提权互操作未执行，计数重叠不累加。

同E两次不同代码目的source66c36d19…/d620ab81…各准备/分发一次APPLIED，16停机资产/Core6/原local grant7精确，权限/env/初始化不变、不自动启动。首个材料预检因真实Core链接拒绝，授权前修正并保留失败。新包0.1.0~native202611171436.269cfb3-1从当前自包含Git源码锁定离线新构建，SHA277883c7…675f1，四脚本编译；JAR/loader未变。

材料原13d50bed-dab2-412c-90ed-73d634258fb9一次授权APPLIED/READY、远程关、不自动启动，无CSR/钥/材料源/身份/通道。随后显式启用两socket及启动native/Core：native7887/ticks19257379 UID/GID997、groups996/997、五cap集合0/NNP1/NRestarts0；Core8162/ticks19257403保留11/13 caps/NNP0待办，key4714/recovery4640/NM4668原进程不变。agent/kura=true、bundle216/DS53 ACTIVE，native6/Core6/原local grant7私有精确、Root业务/启动0。实际socket合法UID缺材料REFUSED/no FD、错误UID拒绝、直达私有publisher EACCES13；探针解析/健康路径两次错误保留，同只读号续接，没有业务重发。

见 [检查点](native-material-authority-checkpoint.json)及 [审查](../../reviews/native-material-authority-review-20261117.md)。本次原3/个人25/C9与进程计数/D29及原Main3135只读精确，18仓文档前干净、冻结完整12父项SHA不变。正向真实Core材料/crypto/通道未验；已封材料能力的原证明联合source维护、broker/Core最小权限、保留态/旧态/UNKNOWN迁移、联合升级移除恢复/R017及六组件provider/launcher完整矩阵和全部12父项均内部pending，仅原3 external deferred。旧CSR/UNKNOWN及闭环密钥验收不重发重放；目标active、现有积分继续、无推送。

## 2026-11-17 保留原本地授权的source联合演进

agent代码85784f2/文档8bc267b新增显式local-peer联合source动作，只接受两Main停止、原env/空业务运行态/Core unit/JAR/loader/组不变；普通source及未装Core动作仍分开拒绝。Source结果记录前后代码绑定并锚定原授权结果SHA，原UUID/权限/回执/current指针/发布回执不改；丢链拒绝当前readiness，原fate不变。185 Python中183过/2历史夹具排除，含8联合方法，focused8额外核对原回执元数据；210编译输入与上一新构建精确，Rust未改，4脚本编译，计数重叠不累加。

同一E source原862826c6-67a3-4cf8-baf8-70d964f50cd6一次准备/分发APPLIED，不自动启动；16停机资产/Core6私有/原授权7资产含指针字节及元数据精确。原授权25abd835…仍APPLIED/READY，原结果SHA806ae290…25147e与全部权限/env不变，仅当前native身份哈希8609c04e…→6394f9b9…推进；原初始化24560864…APPLIED/READY，无加组/重授权/重初始化。

显式启动后native6574/ticks19080145、UID/GID997、groups996/997、五cap集合0/NNP1/NRestarts0；Core6849/ticks19080165、UID996/GID989、groups20/987/989/997，既有11/13 caps/NNP0义务仍开。Key4714/recovery4640/NM4668原身份不变；首次只读status agent=true/kura=true，native当前5/Core6/原授权7私有指纹精确，Root业务/启动0。此范围仅本地初始空业务授权态，不代表TLS/材料/真实业务/UNKNOWN迁移全覆盖。

见 [检查点](native-local-authority-source-checkpoint.json)及 [审查](../../reviews/native-local-authority-source-review-20261117.md)。18仓文档前干净、冻结完整12父项SHA不变，其他现场未改；原3/个人/C在phase14、D在phase13只读精确。下一步独立材料/relay与通道Root授权，原本地grant不重写/远程不开；broker/Core最小权限/完整保留态迁移/联合升级移除恢复/R017及六组件provider/launcher全矩阵和12父项均内部pending，仅原3外部deferred。原CSR/UNKNOWN/闭环密钥验收不重发重放；目标active、现有积分继续、无推送。

## 2026-11-17 固定Kura UID授权与原生真实注册

agent代码d50ac96/文档6a28741新增固定Root本地peer原UUID授权、单次gpasswd与最终发布回执；回复丢失只读原状态、完成动作不重发加组，缺标记/历史阻止新号与启动，闭环fate与当前readiness分离。Root与Rust校验精确账号/组、Core unit/loader/bridge、native身份及原初始化，Core/GWT Root门禁未放宽。177项Python中175过/2历史夹具排除，含9项授权；focused9、Rust身份5/连续性1/注册续期30过，4提权互操作未执行，计数重叠不累加。

当前Git自包含新构建包已在同一E以source原号3c04982b…一次升级APPLIED，16停机资产精确，Core期间继续运行；安装保持native停止。再以peer原号25abd835-cf55-4871-929d-87abee746024一次授权APPLIED/READY、不自动启动/远程仍关；固定Kura UID996/GID989仅加control997，env只改UID，原初始化24560864…不变。授权env之外15停机资产/新Core6私有精确，无重新初始化/CSR/材料/通道。

显式先native后Core启动，native Main5719/ticks18960007、UID/GID997、groups996/997、五cap集合0、NNP1、NRestarts0；Core Main5994/ticks18960028、UID996/GID989、groups20/987/989/997，既有11 ambient/13 bounding/NNP0义务仍开。Key4714/recovery4640原身份不变；Root业务/启动计数0，native4原文件/Core6私有精确。agent.status实际agent=true/kura=true，bundle216/DS53 ACTIVE；无关UID65534即使有control997也在任何frame前EOF拒绝。普通source对已授权peer仍拒绝，下一步必须实现保留原授权的显式联合source/authority演进，不删grant或撤销重做绕过。

见 [检查点](native-kura-peer-registration-checkpoint.json)及 [审查](../../reviews/native-kura-peer-registration-review-20261117.md)。原3现场/个人25/C9及进程计数本次后精确，D29/原Main3135前次精确且未动；18仓文档提交前干净，完整冻结12父项SHA不变。材料/通道与提权broker、Core最小权限、保留态/旧态/UNKNOWN迁移、联合升级移除恢复/R017及全部矩阵/12父项继续内部pending，仅原3外部deferred。原CSR与闭环密钥验收不重发重放；目标active、现有积分继续、无推送。

## 2026-11-17 原运行态盘点、loader配对及当前Core实际组合

agent2b2ed3c（文档6ee303f）已将原禁用remote、空构造目录、零字节锁/WAL等按含义盘点；非空WAL、原remote丢失/改变、真实恢复/网络记录、凭据及仍持写锁均拒绝，不删改/认领数据。168项Python中166通过、2历史夹具排除，含21项source方法；Core50855bc的v2可选加载及严格单标记/Root配对10项Linux通过。计数重叠不累加。

同一E两次不同代码目的原操作a0365464…和a06216e8…各只准备/分发一次，均APPLIED；16停机资产与原初始化精确，安装不自动启动。第二次为Core尚未安装的独立loader指纹变化，Java JAR不变；包残留/真实Core数据/已装unit或阶段间Core出现均拒绝，普通source不放宽Java/Core门禁。前后配对进入Root不可变历史，Core安装后该窄动作已拒绝。

当前Git Core50855bc已从隔离检出构建并安装到同一E，233归档JAR/loader Root、四维护脚本通过解析。初次导出缺Git元数据失败、宿主复制501:20与改属主超时、同原安装意图改用Root600校验流、NM缺file警告、WIP哈希版本dpkg降级警告均保留，不删现场/降检查。实际控制版本6.0.0~git202611171018.50855bc-1；后续候选用单调时间戳版本。真实生成配置位于Main隔离/tmp；adapter bundle216/DS53 ACTIVE，ConfigurationService/revision/CryptoService已绑定。

Core Main4976/ticks18777309，UID996/GID989、11 ambient/13 bounding、NNP0仍为内部待办；native Main1562/ticks18708735，UID/GID997、groups996/997、五cap集合0、NNP1、NRestarts0。Root key4714/recovery4640 cap0/NNP1；NM4668随Core启动，不外推接管。新Core6私有及native5原文件精确，Root业务/启动计数0。显式Kura UID/control组授权仍缺，本地status agent=true/kura=false，未发CSR/材料grant/通道激活。见 [检查点](native-runtime-inventory-core-loader-checkpoint.json)及 [审查](../../reviews/native-runtime-inventory-core-loader-review-20261117.md)。

原3现场7进程/38私有/6旧数据、个人25、C9私有及进程/零计数、D29私有及原进程精确；18仓本次文档提交前干净，冻结12父项SHA不变。全部12父项、完整六组件/provider/launcher/安装升级移除/恢复/R017及原生授权broker均内部pending，仅原3 external deferred。原CSR/UNKNOWN及已闭环密钥验收不重发重放。用户已明确“使用现有积分继续”，替代下文历史额度仅收尾/暂停文字；目标active，无推送。

## 2026-11-17 同现场 source 维护及非Root Main 实际续接

agent e06139b 修复固定PID1挂载/root上下文的Root预检，Main隔离不放宽；原source操作26ea376c-81ff-4cdd-b8b9-3ebfcc7c4105在同一E a60111a6…44414只准备/分发一次，实际dpkg升级后APPLIED、未自动启动。旧prerm与完整源配对后私有保留；数据/配置/额外控制包项提前拒绝，原DB/计划钥/初始化回执等6输入字节及记录元数据精确，原初始化24560864…仍APPLIED/READY，没有改写/重做。

原首次启动目的沿配对修复续接：Main795/ticks18465663，UID/GID997、groups996/997、五cap集合0、NNP1、NRestarts0，本地只读status可用。实际Main挂载命名空间中，合成Root0600公开文件直达隐藏、经Root/cap0进程根别名EACCES、Root0600合成IPC EACCES；未读真实Core钥/HMAC。JDK21仅在agent独立启动后安装供实际已装JAR的peer校验：专用user/group接受，冲突Root属性拒绝，无业务frame/OSGi注册。Core尚未安装，不外推真实Core/provider/OSGi验收。

231源码文件自包含离线构建，201 Rust输入未变且实际重建ELF同指纹；四脚本可编译，158项Python中156通过/2历史夹具排除（包含11项source维护，选择重叠不累加）。运行已创建自有禁用remote记录、空network/firewall/recovery目录及零字节锁/WAL；7业务表0、native generation1，后续须按含义盘点维护，不删数据凑准入。见 [检查点](native-source-maintenance-main-checkpoint.json)及 [审查](../../reviews/native-source-maintenance-and-main-review-20261117.md)。

原3现场7进程/38私有/6旧数据、个人25、新C9私有/进程/零业务、中心29私有/原进程、新E原4私有与现用进程再次精确；18仓检查时干净，冻结12父项SHA不变。Native授权/提权、当前Core联合、完整运行态/迁移升级移除/恢复及R017 GUI与全部矩阵/12父项继续内部pending，仅原3 external deferred。旧Root现场同UID反例仍保留；原CSR与闭环密钥验收不重发重放。目标active、现有积分继续、无推送。

## 2026-11-17 原生 peer、状态连续性及首包原失败续接

networking d226b61 的精确 native peer 校验已通过160项JVM及8项Linux/JDK21检查，Root进程即使伪装socket属主仍拒绝；Core/GWT Root transport未放宽。agent 3f7c5b6 增加显式非Root包、固定账户、代码/unit/source配对、原UUID初始化及DB/计划密钥结构连续性；145项Python通过、2历史导出夹具未执行，后续32项为重叠子集；Rust身份3/连续性1/注册续期30通过。

当前Git自包含230文件源码构建的ARM64 native包已在新E a60111a6…44414安装，18运行载荷精确；5份文档由基础镜像既有dpkg策略排除。原初始化24560864-f974-46aa-940a-0a97ad926118唯一完成，APPLIED/宿主READY、远程关闭、7业务表0。首次Main启动在Root预检失败，Main从未运行；24次自动预检后仅停止该新unit。只读诊断证实systemd255.4的“+”未消除TemporaryFileSystem遮蔽，Root env/admin不可见；Root helper为完整容器caps/NNP0，不能称Main cap0/NNP1已验。保留同一E、原DB/计划钥/初始化回执与失败，不重新初始化、换号、换环境或改写历史回执；继续Root检查上下文修复和有原意图的精确source/pin维护。空Root0755挂载点与真实旧数据须分别识别。

见 [首包检查点](native-agent-first-package-checkpoint.json)及 [审查](../../reviews/native-agent-peer-and-first-package-review-20261117.md)。原3现场7进程/38私有/6旧数据、个人25文件、新C9私有与进程/零业务计数、中心29私有及原进程再次精确；18仓检查时干净，冻结台账SHA不变。Native实际Main/隔离、授权/提权broker、迁移/升级移除/恢复、R017及全部六组件矩阵和冻结12父项继续内部pending；仅原3 external deferred。原CSR及闭环密钥验收不重发/重放，目标active、现有积分继续、无推送。

## 2026-11-17 专用非 Root agent 与窄委托源码续接

agent b893964 新增 native-nonroot-v1 的固定身份/路径/Root程序与unit及源码指纹门禁，daemon 必须 cap0/NNP1；Root 提权后台未接入时明确拒绝。新 runtime、控制 socket、Root管理/配置与公开身份分开；不复用旧目录，不让 StateDirectory 递归更改持久数据属主。只读材料经不同端点的 Kura UID/cap0 → Root/cap0窄委托 → 专用 agent UID/cap0 传递，Core Root key transport 不放宽。

3项Rust身份、8项Linux窄委托、9项原生预检夹具及30项受影响注册/续期检查通过；另一次显式三进程 parent/child 以真实 UID995/cap0/NNP1完成独立证书校验、创建自有0600候选，源字节/完整元数据含atime精确，没有发布身份/未决指针。其程序/源码元数据明确为文件系统夹具，不能称原生发行配对验收。既有新现场的UID0反例仍适用，任何运行服务未替换。

见 [源码检查点](native-agent-identity-source-checkpoint.json) 和 agent 的 docs/native-agent-identity-v2-candidate.md。新文件尚未纳入deb映射，未构建/部署；账户创建、Root授权/恢复、Core peer映射、旧数据/凭据/UNKNOWN迁移及状态丢失保护、真实systemd与GUI仍内部pending。原CSR意图及闭环密钥验收不重发/重放；完整六组件/全部冻结12父项继续内部pending，仅原3 external deferred；目标active、现有积分继续、无推送。

## 2026-11-17 注册材料 FD 与 Root 同 UID 反例续接

agent 75ebf3c/9a25c91/36fff96/8a7c03e 保存只读 FD 发布/消费、精确包/源码/unit 配对、原 UUID 授权与 purge 保留候选。30 项 Rust 常规检查及一次显式 Root 跨 UID parent/child 通过；扩展 Python 116 通过/2 历史导出夹具未执行，后续 purge 相关 14 通过（重叠不累加）。未构建、安装或启用新包，未结算任何业务 CSR。

公开临时探针确认：既有新组合现场中，UID0/NET_ADMIN/NNP1 可通过 Root key-service 的 /proc/PID/root 绕过直接路径遮蔽。未读任何钥/HMAC/凭据/日志，探针已移除，/var/lib 目录时间戳变化保留。此前“agent 看不到原钥/HMAC”只代表直接路径观察，不能外推完整隔离；Root-actor 候选暂停部署，继续专用非 Root 身份、窄委托及准确 peer/发行包配对。此项仍是内部工作，不新增 external deferred。

见 [检查点](enrollment-custody-runtime-identity-checkpoint.json)与 [审查](../../reviews/enrollment-custody-runtime-identity-review-20261117.md)。原3现场7进程/38私有/6旧数据及个人25文件精确，新C9私有/进程/零Root业务计数、新中心29私有/原进程保持。combination-c-ebdbf564 不重生成/换号/换Root；R017 GUI、完整六组件/provider/launcher/安装升级移除与全部冻结12父项继续内部pending。仅原3 external deferred；现有积分继续、目标active、无推送。

## 2026-11-17 组合网关、首改密码及正式数据目录续接

同一新 agent-first 现场新增 networking5d27fea/GWT 并实际普通启动，Core/agent/networking/GWT configured，NM/GWT/bridge ACTIVE；health 为惰性 SATISFIED，不宣称业务健康已验。GWT0d6dba8 修复首改只填新密码/确认及事件会话锁，46a2d92 改用 Core SystemService 数据目录并拒绝旧 override 冲突。最终 20 项选择检查、两模块12 GWT变体通过；当前 GWT 原生包已在同一新现场升级，9私有完整元数据/agent进程保留，Root operations/startups与agent执行表0。唯一密码更新200/服务端成功审计，现有新密码实际登录通过；一次门禁探测先使测试登录失效，XSRF403时密码写0，记录保留。

CSR 自动化意图 `combination-c-ebdbf564` 原响应/客户端UUID未恢复，无申请目录/受理审计；新只读列表为空。异步错误弹窗可解释点击未触发，但不据缺失或推断结算；不再生成、不换ID/标识、不中转到其他Root重做。身份目录错误已实际修复，原意图和 GWT 发出前持久保存请求编号的缺项继续内部 pending。本机未授权的首次接收/通道按钮保持禁用；R017 本机授权/维护/独立agent GUI 未关闭。

见 [检查点](combined-gateway-runtime-checkpoint.json)及 [审查](../../reviews/combined-gateway-runtime-review-20261117.md)。原3现场7进程/38私有/6旧数据、个人25文件精确；新中心29私有及原进程身份精确。方案A与后备CLI停机顺序文档已校正。完整六组件/provider/launcher/升级移除和全部冻结12父项继续内部pending，仅原3 external deferred，目标 active、现有积分继续；旧UNKNOWN及已闭环密钥验收不重发/重放，无推送。

# 未验收与剩余实施台账

## 2026-11-17 继续：中心原生 GUI 引导与独立工具更新

新空主机实际 Cockpit/PAM 发现并修复 sudo 必需依赖；9 项 Root 打包/源码检查通过。中心包由 GUI 唯一上传/安装，原实际 dpkg＋postinst 回执与 9 个 Root 载荷核对，安装保持停止；之后 GUI 显式启动、可信凭据/实际 TLS 核对、中心生成 CSR、外部测试 CA 签发、GUI 专用空库初始化及管理员登录通过。TLS 切换后的原初始化 8d038ae0…仅在同一中心登录后只读取得 APPLIED/observedConfiguration=true，没有重发换号。独立工具在运行中更新，7 私有元数据和 PID/ticks 保持；中英文不同主机名帮助及无障碍关联实际验证。

见 [检查点](native-console-empty-host-checkpoint.json)及 [审查](../../reviews/native-console-empty-host-review-20261117.md)。最终中心 PID3135、cap0/NNP1、29 私有文件，原 7 bootstrap 元数据精确；外部 PG16.15 应用角色无 superuser/createdb/createrole。OS 图形安装/GTK/人工证书信任、gateway mTLS 与完整六组件/provider/launcher/安装升级移除及全部冻结 12 父项仍内部 pending，原 networking inode/ctime 偏差继续保留；仅原 3 外部边界。原 3 现场/7 进程/38 私有/6 旧数据及个人 25 文件再次精确，目标 active，无推送。

## 2026-11-17 继续：旧 NM 配置保留与原失败续接

原 networking 失败现场经显式 conffile 过渡及修复后正常包续接，Core＋12 sibling 全部 configured，64 个 Root/不可由 group/other 写入的 JAR，12 注册项无重复；Main 启动/Root operations/startups 0。活动 NM 文件原完整元数据、网络服务 PID/ticks 保持。首次过渡暴露空 configured-version 误判首装，给该新测试现场属性追加两标志；已严格按原前缀/已知后缀指纹撤销，字节/属主/模式/mtime 恢复，但 inode/ctime 不可恢复，偏差保留。preinst 持久安装模式及损坏准入拒绝已修复；24 项 sibling、9 项 dpkg 检查通过。最新修复版过渡归档已组合，尚未另作完整实际 Kura 路径验收。

见 [检查点](legacy-nm-policy-continuation-checkpoint.json)及 [审查](../../reviews/legacy-nm-policy-continuation-review-20261117.md)。resolver 仅 ctime 变化已记录，不宣称全部系统元数据精确。原 3 现场/7 进程/38 私有元数据/6 旧数据及个人 25 文件再次精确。完整六组件/provider/launcher/升级移除、obsolete conffile purge、R017 GUI 与全部冻结 12 父项继续内部 pending，仍仅原 3 外部边界，目标 active。以下 half-configured 描述是各历史检查点当时状态。

## 2026-11-17 继续：agent 先装与精确 loader 配对

Java adapter 已移到 agent 包自有路径，Root 保护的元数据/loader/JAR 精确绑定；agent-first 不再先占用 Core 未初始化目录。动态父目录隔离修复了启动时不存在密钥路径的漏洞，公开 CLI 支持 kurad。新原生 ARM64 agent/console/installer 已组合；同一 agent-first 失败现场原维护记录续接成功，Core 配对升级及一次普通 Main 首启通过。OSGi bridge ACTIVE、三项 Core 服务绑定，实际 kurad 只读 UDS 得到 kuraAvailable=true；6 份私有文件元数据保持，agent 看不到新原钥/HMAC，Root operations/startups 仍为 0。Main 仍 11/13 caps、NNP0；无 networking 的 health DS 和中心/安装器未实际验收。

见 [检查点](agent-first-paired-loader-checkpoint.json)及 [审查](../../reviews/agent-first-paired-loader-review-20261117.md)。原失败/命令前端/parser 记录全部保留，不重新 provision、不绕过 inactive guard 或 Root 初始化检查。原 3 现场/7 进程/38 私有元数据/6 旧数据及个人 25 文件再次精确。完整六组件/provider/launcher/安装升级移除、旧活动 NM 配置迁移及冻结 12 父项继续内部 pending，仍仅原 3 项 external deferred，目标 active。

## 2026-11-17 继续：默认 NM、Core 保留与源码 cohort

默认 networking 已取消自动系统接管/快照改写，旧活动包配置的 obsolete-file 转换会提前拒绝，安全保留迁移仍 pending。22 项 sibling、9 项变体、8 项 Core 文件系统检查通过；同一原失败夹具的一次实际 Core 升级保留 56 份 Root addon 代码、12 份外部私有完整元数据与 7 份用户文件内容/属主/权限，Main/Root 业务请求 0；networking 仍 half-configured。agent/中心源码绑定和默认 Core 显式授权已修复（12/10/5 项各有限检查），独立 ARM64 Rust/前端构建完成，实际原生组合尚未完成。

见 [检查点](default-nm-core-code-cohort-checkpoint.json)与 [审查](../../reviews/default-nm-core-code-cohort-review-20261117.md)。原 3 现场及个人数据再次精确；不重放旧密钥验收或重发 UNKNOWN。用户已授权现有积分继续，目标 active。完整六组件/provider/launcher/安装升级移除及冻结 12 父项继续内部 pending，仅原 3 项 external deferred；以下 e016 及早期场景均为历史有限范围。

## 2026-11-17 继续：兼容变体及普通启动

firewall-only 已补齐4个JAR Root权限、升级阶段/agent ownership保护、受限注册表与失败传播，9项独立源码/Linux原生检查通过。对应新实际systemd profile的Core＋12 sibling全部配置成功，一次普通Main启动/HTTPS通过；59个sibling JAR为Root所有且不可由group/other写入，6份凭据/TLS/HMAC元数据保持，Root业务及日志化启动请求0。Main仍11/13 caps、NNP0；MQTT默认clientId与历史额外JAXB边界分别记录，完整provider/broker没有关闭。Netplan兼容适配只完成源码及5-JAR包核对，未部署/切换NM。

见 [兼容变体检查点](networking-variants-code-ownership-checkpoint.json)及 [审查](../../reviews/networking-variants-code-ownership-review-20261117.md)。原普通networking失败现场仍half-configured，继续原现场处理；新profile不替代该失败。完整六组件/provider/launcher/升级移除矩阵及冻结12父项继续内部pending，仍仅3外部边界。

## 2026-11-17 当前续接：sibling 代码权限与安装链路

用户已明确选择在周额度 100% 时使用现有积分继续，目标 active；不兑换 reset。11 个剩余主 sibling 的 49 处 JAR 映射、提权脚本路径及失败传播已修复，GWT 补齐卸载遗漏。19 项源码/Linux 原生 test methods 通过；实际 12 个新包的 64 个 Root/0644 JAR、56 份代码/模板及 54 条 API 声明已核对。从当前源码构建，不依赖还原旧证据。

新的独立 systemd fixture 已安装 Core 并完成 11 个 sibling 配置；Networking 因真实 udev 控制通道缺失而失败。新候选已在同一失败现场验证提前拒绝，19 份私有/用户文件完整元数据保持，Main 启动/Root 密钥请求均为 0。该现场继续保留，不重复 provision；此项仍内部 pending。历史 Networking 包的旧 prerm 首次升级路径、完整六组件/provider/launcher/变体/安装升级移除矩阵及全部 12 父项仍内部 pending，仅原 3 项 external deferred。原 3 现场的 7 进程/38 私有元数据/6 旧数据及个人当前 25 文件复核一致；配置/密钥/数据及运行中 target/runtime 保留；UNKNOWN 和已闭环密钥验收不重发、不换号、不重放或跨 Root 查询。

见 [sibling 检查点](sibling-code-ownership-checkpoint.json)及 [本轮审查](../../reviews/sibling-package-security-review-20261117.md)。Core 新增仅为通用安装夹具的测试入口；生产包仍与 e016 源码一致。全部更改仅本地 WIP，无推送；冻结 12 父项原文不变。

## 上一阶段：实际 Main 联合检查点（历史）

2026-11-17 当前 Core 源码 `de5f4e6901`（生产包来自 `e016f60c03`，其后只追加测试/说明）、GWT `f135710e00`。实际 Ubuntu24.04 ARM64 / systemd / Main / Root UDS / HTTPS / ADMIN/XSRF 联调已通过一个受限新环境：正常首次登录改密，一次新随机测试钥的加密备份下载，原 LEGACY 和 ADMISSION 各一次元数据归档，独立恢复 HTTPS API 只读解密核验。Root 原密钥操作记录原字节/元数据保留；6份测试凭据/TLS/HMAC文件及现用Main PID不变，没有恢复、发布、换钥或日志化启动。浏览器由Playwright驱动；恢复页健康态按合同禁用材料，没有重放冷恢复验收。

本轮修复发行包旧Crypto API消费者混入（增加打包前源码契约门禁）、GWT隐藏页签初始化过早、插件JAR/提权脚本归属错误及并发历史读取BUSY。181项相关JVM、7项包输入、7项安装器检查通过，双GWT模块12变体构建通过；实际两次页签选择的8个只读调用全成功。新Core包15个Crypto契约和47个包内文件核对，GWT实际包内JAR及Root所有权核对。原3现场/7进程/38私有元数据/6旧数据保持。已删除本轮完成容器、测试备份/口令及重复包等712,958,966字节文件；容器可写层逻辑大小1,294,621,968字节不能等同物理释放量。无备份、无推送。

见[实际Main联合检查点](installed-main-key-archive-checkpoint.json)和[本轮审查](../../reviews/installed-main-key-archive-review-20261117.md)。当前仅闭环上述新测试子场景；完整12父项继续内部pending，G034未封签，仅3外部边界不变。其余11个sibling包发现同类daemon所有代码/安装器映射，优先按功能和数据目录逐项修复、验证。界面按modern-web-guidance随功能渐进处理，仍是次要任务。历史阶段细节留在Git及各原检查点，以下冻结台账原文不能被局部联调结果替代。

## 冻结完整台账

1. **完整密钥生命周期**：Root 可恢复轮换事务、原意图/回执及 UNKNOWN 门禁；原/新代双侧持久保留；多代加密备份与严格原钥恢复；安全的停机/启动交接；全部 provider 数据盘点、逐项原子重加密、原文件保留、重启核对和 GUI 下载/核对/确认。当前已有新 reader、Root多代备份、严格原集合缺失文件恢复与显式启动门禁的未部署候选；离线及Java受控reader现已支持显式指纹绑定的多代文件，修复logical-V1形似密文的合法密码恢复值错误（70测试及macOS/Linux实际JAR通过）。Root已增加不切换现用钥的代准备、不可变暂存历史及原加密备份上传核验；普通单钥/多代共同Root启动门禁已加入source默认unit和3个实际启动入口，原UNKNOWN即使单钥完好仍拒绝启动。reader/launcher/Core单元包文件配对、绑定包指纹的JVM运行期锁、核验原备份后的原生发布事务及独立generation-core-start授权已有候选。此前228检查点的Linux195项原生检查、2种权限的真实systemd/JDK21锁交接及1次实际Core闭环通过，当时ARM64 deb的43个实际文件及提取JAR通过；主Kura损坏不拖住冷恢复基础服务。原生故障测试继续使用明确的systemd/进程替身；另已通过一次实际发行包安装后的Core停机/发布/独立重启：真实Crypto与配置审计facade、快照reader就绪，旧密文保留且仍可解密、新密文代号切换。安装后模板清理缺陷已修复，195项原生+2种权限的systemd锁+1实际Core闭环通过；当时228包非缓存载荷逐项等同于实际验收包，不适用于后续已改变的候选。此为单个Ubuntu24.04 ARM64独立Core/合成数据profile，完整provider/runtime盘点、原子重加密及更多模式仍pending。维护后多代启动授权、原发布UNKNOWN恢复以及取消/已提交历史的GUI处置仍pending；原生历史/取消的当前有限范围见页首。agent已具备Core保管下的只读FD委托候选，完整运行时准入/权限仍pending，不能扩大agent原钥读取权。完整中断发布恢复（含缺原停止凭据/跨boot）、完整恢复/启动界面、provider重加密和正式六组件安装单元配对尚未完成，不能启用实际轮换或自动删除旧钥。旧弱钥仅迁移保留尚未完成安全轮换。
2. **CA custody**：保持用户选定方案A、外部签发；中心不默认保存 CA 私钥。多人保管/审批、quorum、批量管理仍需实施和验证，不能把“同一人持钥”改称已解决。
3. **最小权限与完整发行**：完整 privileged broker、各 provider/launcher/模式/资源/平台/purge 组合；Main 仍11 ambient/13 bounding caps、NNP0。当前三运行场景使用已安装 Core12 等旧组合；最新候选ARM64 Core deb已包含reader及包文件配对，已在全新隔离环境完成单个Core profile安装/换钥，但尚未正式发行或完成全平台安装验收。完整当前源码六组件发行、包配对、安装升级卸载、凭据连续性和全平台门禁仍待完成。
4. **原14组及 API**：原1–6核心/编辑语义、7与12直接 CA/非DS ownership及生命周期、8与9广义配置恢复、10与11 NM/远程管理、13一致备份/迁移/发行、14历史边界全部保留；七个 API2、R017、全部消费者与实际 provider 验证未因局部修复关闭。配置 API 历史运行1.2/构建1.8、后续1.9的正式配对仍需明确。
5. **完整实际接管链**：合法原记录恢复；registered CM/runtime/source→一次加密导出→paired fences→真实 Java drain→native/realStore 初始化→startup/Core owner→ordinary boot/recovery→原 forward/confirm/inverse/source observation/stopped/expiry。已经发生的 UNKNOWN 不可绕过以重做整轮；未执行/未证明的阶段保持 pending。
6. **网络矩阵及历史**：ordinary/recovery、NetworkManager/CM/runtime、双族、NAT true/false、多WAN/modem/DHCP/DNS/Wi-Fi/VLAN/packet/fragment/unsupported；boot/netns/handle 重建后的合法 restore；初始化先退役再原 native inverse；子记录结算、退役、历史、重新接管。原 forward/confirm/inverse、observation、stopped/expiry 的实际完整接受不以模型/临时存储验证替代。初始 INTERNAL 生命周期根因和并行 maintenance-lock flake 保留。
7. **中心与部署 GUI**：中心地址、数据库、CSR legacy、半凭据、首次 TLS UNKNOWN、容量与归档；完整主机/实例/容器用户流程及错误恢复。已完成的首次配置、TLS联合切换、CSR、续期和只读恢复资料各自有限范围保留，不能外推整套正式交付。
8. **数据库与协议**：客户旧加密任务库及完整 GUI 数据库备份/恢复；协议0在线升级、正式发布的校验和协议 crate、跨组件 joint generations。已通过的是合成 schema9 的真实PG备份恢复/当前迁移/rekey，不能重放或替代客户场景。
9. **R017 及文档/技能**：单机部署中安装器已启动 agent 与要求 inactive 的 import/CSR 顺序；统一方案A、工作空间知识库交接；完整 TLS reload/bulk/证书下载上传及平台体验的父项验收。后续用户操作从界面完成，不能以设置环境变量/运行脚本代替产品流程。
10. **G021/G022**：实际 LDAP present/absent；Go31 vet 债务、三个非Java模板 POM、可选生成与未执行.NET范围。保持内部 pending。
11. **上游逻辑同步**：按实际 fork 与依赖关系继续，非fork不直接 merge。保留11 fork/7 nonfork关系与已审查游标，不能因 WIP 提交/推送而前移。PLC4X 两个新增逻辑组、shared SLMP/.NET 三提交等剩余范围继续 review；不盲目批量 merge，不恢复 YOFC 明确删除的模块。
12. **正式源码/发行门禁**：WIP 已保存完整447候选并不等于原 guarded apply/正式发布通过。继续以新源码执行必要范围验证和全盘 review，不把旧报告、green manifest 或已闭环写操作重新跑一遍当作证明。

## 仅有的 external deferred

- 实际 NM/cloud-init/物理硬件交接。
- amd64。
- 磁盘满/硬断电。

其余缺项均为内部 pending。现有内存/磁盘/构建环境问题、产品流程未完成、provider 缺失及 UNKNOWN 都不能自动移入 external deferred。

## 下一步建议顺序

1. 先审查其余11个sibling发行包的Root代码所有权、提权安装脚本路径及失败传播。GWT已修；不得把真实运行数据改成代码权限，也不能降低Root配对检查来绕过。网络包有不同的安装布局，需单独分析。
2. 继续当前源码完整六组件、全部provider/launcher/模式及安装升级移除矩阵，完成密钥全生命周期、原文件保留和GUI流程。新环境Core＋GWT通过不代表全部provider重加密或正式发行通过。
3. 按以下完整12父项继续网络/API/中心部署与数据库/CA custody/上游逻辑同步等工作。只读查询短暂排队；写入仍立即拒绝争用，跨进程内核锁非阻塞，不重发Root请求。

本轮关闭的测试备份编号 `cd2ee5cd-1c6c-4e6f-9e82-c545231c50d8`，LEGACY归档 `eb3a5a16-256f-4626-a8b4-8ee7af425b9c`、ADMISSION归档 `47ecb515-0524-4b6f-82b5-1eaed22ac737` 仅保留历史；专用容器和测试秘密已删除。不能重建、重发或在其他Root查询这些编号。原UNKNOWN `767863a9-fe6b-4135-9783-ddcb131030af` 及原startup.stop仍保留在原现场，绝不重发或换ID。

以Git源码和当前环境继续，不依赖旧中间产物或恢复旧证据链。保留原3现场、个人现用配置/密钥/数据及正在运行的target/runtime；确认停用的产物直接删除。JDK最低21，不追加27。周额度耗尽时按用户要求保存必要交接并暂停，只为必要收尾使用少量credits；当前接口周用量显示100%（ordinaryUsageAllowed仍true），按用户额度边界完成必要收尾后暂停；额外credits仅可用于必要交接，不开启新任务。
