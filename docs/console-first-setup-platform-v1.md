# 首次配置的平台入口候选

2026-11-17 续接：默认 Cockpit 工具包已显式依赖 sudo，覆盖不安装推荐依赖的空主机；不新增免密码 sudo/polkit 规则。中心地址的不同主机名要求现在持续显示在地址帮助中，并与输入框的无障碍说明关联。以下 V58 及更早段落保留各自历史边界，实际安装/初始化验收见最新 reviews/checkpoints。

2026-11-14，V58 工作候选，同一 R013 shadow，生产源未应用、未封存，未提交推送。

## 操作系统图形化安装和桌面入口

使用操作系统支持的图形化 DEB 安装器打开发行的安装工具包，核对包名、版本、来源及依赖后授权安装。Ubuntu 提供的 GDebi 是一种可以查看并安装本地 DEB 的图形入口（见 https://packages.ubuntu.com/noble/gdebi）；所需依赖由发行版包管理器从已授权软件源解析，缺少软件源或离线依赖时应保留具体拒绝结果，不调用脚本兜底。

默认工具包依赖 Cockpit 网页必需的 cockpit-ws、cockpit-bridge、cockpit-system 与 xdg-utils，不默认引入元包推荐的存储、网络或软件管理插件；本机引导窗口另依赖发行版提供的 Python/GObject、GTK3、pkexec 与 dbus-user-session。后者提供主机管理登录所需的用户会话总线，不以单独的 X11 会话总线代替。若组件刚安装而当前桌面会话仍未就绪，窗口提示退出桌面并重新登录，不替用户重置会话或重发原操作。安装后在应用菜单显示「YOFC 管理中心安装设置 / YOFC Console Setup」。点击后由普通桌面用户打开引导窗口，不打开终端、不运行 root 浏览器。窗口可读取主机状态；安装页面尚未启用时，用户可显式点击启用并完成系统管理员授权。固定权限入口只启动 cockpit.socket，不 enable、restart 或启动管理中心，不更改防火墙或系统证书信任。使用其他管理员电脑时通过其浏览器访问主机的 Cockpit 地址。

主机启用请求在授权前保存原 UUID，在实际系统命令前另写 root 保护的 intent；只有真实命令成功返回才记录匹配回执。窗口关闭、丢响应或命令超时保留原请求，刷新只读核对。当前 active 状态不能替代回执；原未确定请求阻止同编号或新编号再次启用。主机台账与本地请求损坏、丢失或包含不安全文件时关闭写入口。只有固定 helper 与 pin 未变、没有对应 root intent、且系统授权程序明确未授权时，才把请求保留为授权前拒绝，允许下一次显式申请。权限动作遵循 [polkit 的管理员认证策略](https://www.freedesktop.org/software/polkit/docs/latest/pkexec.1.html)，不增加免密码规则或保留授权。

引导窗口在启用后只握手读取固定 loopback HTTPS peer 公共证书，不发送 HTTP 或凭据，并与 root 保护的 Cockpit 证书文件核对。两种 loopback 地址均可访问时必须呈现相同证书。窗口显示可复制的 SHA-256，提供标准文件对话框保存公开证书，然后单独打开普通用户的默认浏览器；保存内容重新编码为单一公开证书，即使源文件混有私钥也不会导出私钥。旧系统限制公开证书文件读取时，可显式通过系统管理员授权只提取公开证书。匹配原主机文件与当前 peer 后才开放保存/打开，证书改变须重新读取。

公开证书保存对话框默认使用当前桌面用户的下载目录，目录不存在时使用主目录，不继承启动目录；确认保存时再次核对当前主机证书与原请求。已经存在的文件或符号链接不被覆盖。浏览器自己的证书检查继续生效。用户须将浏览器证书详情与窗口提供的主机指纹核对后再决定是否信任该主机；窗口不关闭浏览器安全检查，不自动信任自签证书、不安装 CA，也不传递配对令牌。此处的主机公共 TLS 证书与中心/agent 的外部 CA 管理分别处理，生产 CA 私钥保管边界不变。

桌面文件、引导模块、固定 helper 和 polkit policy 是 root 保护的工具包 provenance 的一部分，bootstrap 协议1共18个必需工具负载。显式 headless 工具变体不提供桌面引导或相关 Cockpit/GTK/pkexec 依赖；旧14文件无桌面及V57的15文件桌面 provenance 保持兼容。声明新协议的包必须完整校验18个固定文件。该增量的实际系统图形包安装、权限提示、证书公开下载和桌面启动验收须以最新审查为准，单元/归档校验不代替实际界面。

## 空主机的程序核对与引导顺序

本机页面按运行方式、软件安装、启动设置服务、领取凭据、打开设置页排序。先点击读取核定版本及主机状态；主机以固定 dpkg-query 读取核心包状态，并核对 root 保护的程序、源码摘要和必需安装文件，不调用中心程序作探测，不连接数据库或创建安装身份。程序尚未安装时提示先安装批准包；安装信息不全、摘要或架构不符、无法读取时保留为尚未核实，启动、服务状态和凭据读取按钮保持禁用。已有数据和原 UNKNOWN 请求不受这个软件前置检查改变。

唯一软件安装有实际 APPLIED 回执后，页面只读刷新主机安装信息。来源核验通过才开放显式启动；安装、刷新和恢复原回执都不自动启动或初始化中心。完整包回执、HTTPS存活、首次业务初始化分别有自己的依据，来源核验通过不等于服务就绪。Docker 模式继续使用自己的代际与就绪核对；切换模式不触发安装或启动。

包回执把必需运行文件与说明文档分别记录。完整 deb 摘要仍绑定所有文件；主机的 dpkg 文档省略策略不会让新包误判运行负载不全。程序、unit、来源清单、标记和全部 React 运行资源继续逐项严格核对。已经受理的旧请求仍按其原始文件 pin 核定，不追改原目录或原请求，也不重发安装。

本节是 V55 之后的源改动。操作系统软件安装入口、完整空主机交付、正式平台/沙箱、跨版本数据库和多管理员的最终验收状态仍须查最新审查及完整台账，不以契约测试替代真实证据。

## 本机发行包的界面安装与更新候选

默认 `kura-management-console-installer` 发行包把 Cockpit、Python、systemd、PostgreSQL 客户端、账户工具、OpenSSL 和公共 CA 证书纳入依赖。管理员先通过操作系统受信任的软件安装入口安装该 deb，再从主机管理页进入 `YOFC Console Setup`。高级维护者可显式构建不依赖 Cockpit 的 headless 工具包；常规交付以带图形入口的变体为准。工具包依旧不含中心程序、中心 unit、私有状态或维护脚本，更新工具不会停止或启动中心。既有 Cockpit 的禁用策略和系统信任设置不被工具包重置；空主机的完整图形化 bootstrap 仍须按总台账单独验证。

本机模式新增「安装或更新本机中心软件」。读取主机状态后选择核定版本，上传对应的 `.deb`，审阅该主机所有默认及命名实例的影响，再显式确认安装。已运行的中心必须先通过下方现有维护入口冻结、核对任务和停服；软件安装自身不会隐式停服或启动。安装结束后中心仍保持停止，由用户另行点击启动设置服务。

允许的版本、架构、完整 deb 大小和 SHA-256、程序摘要、源码清单及全部包负载文件 pin 来自工具包自带的 root 保护发行目录，不接受浏览器自行批准的路径、摘要、软件源或命令。目录由构建者依据实际 core deb 及其校验回执生成；它不作为 core deb 自身的编译源，避免把包自身摘要放进其源清单形成循环。工具的独立 provenance 同时保护这个目录。二进制仍须是对应架构的普通 Linux ELF，并满足 Ubuntu 24.04 的 glibc 下限。

上传在私有目录内完成，接收后复制到另一个受校验 inode，后到的上传句柄或路径替换不能修改已受理字节。派发前保存原请求及 worker 意图，由固定 systemd 任务执行一次安装。只有实际 worker 进程可提供 PENDING；缺进程和缺回执保持 UNKNOWN。浏览器只保存公开的原 UUID、版本目录键和请求步骤，刷新只核对原请求。

安装持有原主机维护冻结，包自身 postinst 只能记录目标文件已验证，不能在安装命令回执之前解除冻结。实际 `dpkg --install` 成功返回后，还要核对已安装版本、架构、每个负载文件和原 postinst 回执，才能记录安装成功并解除冻结。没有安装成功回执时，不因文件已经存在或 postinst 已完成而补记成功、不重发安装、不自动回滚。已经有真实成功命令回执但后续核验未完成时，可在同一原编号显式核定；核定只检查原回执与文件，不再执行安装。成功记录已落、解除冻结未完成时，另有完成已核定冻结解除的入口，不安装、不启动中心。

尚未受理安装的上传请求可以显式取消；root 在同一排他锁内先保留原 UUID 的退役记录，后到的上传准备或安装请求被拒绝，上传文件和历史不删除。此取消不用于已有安装意图或缺回执的 UNKNOWN。协议0旧包在线升级、跨版本数据库和非空加密任务兼容、恢复和多人审批继续保留为内部待办。

以下 V52 段落是历史依据；其中关于「推荐 Cockpit」「没有本机包上传入口」的范围由本节替代。实际新界面、标准包及 PAM 验收状态须读取最新审查和对应组件证据，不能把契约或假文件系统测试写成实际主机安装通过。

## Docker 冻结启动与代际升级候选

界面新增所选 Docker 中心的冻结、停服、原编号观察、完成维护并启动、升级所选镜像并启动。先导入支持维护启动的目标镜像，再进入维护。旧镜像缺少此启动能力时只能升级到受支持的新镜像，不能先解冻后启动来替代升级保护。

根维护台账把实际守护进程动作及应用冻结/完成动作的意图、成功回执分别保存。冻结时先禁用旧容器的自动重启；只有原冻结及成功停服回执、实际停止和PID0都匹配，才可创建下一个容器。旧容器、原记录和原数据卷保留，不自动降级或回滚。缺回执不因容器状态变化而补记成功，UNKNOWN不重发、不更换编号绕过。

服务器以容器PID1运行并处理SIGINT。管理入口单次发送该信号，然后只读核对停止/PID0/正常退出；等待超时保留原操作，不升级为强制信号。后续受管容器也显式设置默认停止信号为SIGINT。信号发送成功回执本身不等于进程停止回执。

新镜像的固定内部命令 `serve-maintenance ORIGINAL_UUID` 只接受已有持久安装和匹配的私有冻结；启动阶段持有独占写入锁，继续初始化已配置的运行时。它跳过bootstrap向导，不能创建新安装或绕过本机native主机维护。监听就绪后仍保留原冻结，普通HTTP API/gRPC/CLI写入继续被拒绝。后端核实实际程序及进程摘要、原冻结、精确TLS叶证书和数据库健康，才选择新代际，最后通过同一原编号完成冻结并恢复自动重启。后续主机/守护进程正常重启只接受该编号的精确完成回执；新的冻结会再次阻止旧命令启动，残留或损坏回执要求原操作核定。

浏览器在每步派发前只保存原公开维护UUID、安装ID和请求步骤，刷新只观察原操作。已确认停服之前不能选择启动/升级；请求结果不明时不再次停服、启动或解冻。契约测试不代替实际配置实例、真实Docker/Cockpit、冻结期间拒绝业务写入、同卷升级和最终恢复的证据；实际验收状态以reviews/registered-runtime-review-20261114.md为准。工具bootstrap、跨版本数据库兼容性/正式发行、恢复/备份/多人审批等完整台账仍保留。

若原启动已经有真实成功回执，仅启动后的核验失败，可显式核验该已启动代际并完成原维护。要求全部原变更都有匹配回执、尚无完成意图、原选中代际仍停止、新容器仍持有同一冻结；实际运行程序必须在该容器内唯一匹配预期argv、低权限UID、程序摘要与进程开始时间。旧批准入口可能让shell等待于PID1，因此不能把包装进程当作服务。停止通过该已核定服务进程的pidfd单次发送SIGINT，防止PID复用；新入口直接exec降权程序。核验阶段不再创建、启动或停止原容器，只有全部检查通过后才首次选择新代际并完成原冻结。缺少原启动回执、仍有不确定变更或已发完成意图时拒绝此入口，不是通用UNKNOWN重试/重置。

## Docker 界面部署增量候选

Linux Cockpit 页面新增核定镜像列表、离线文件上传和导入、实例名称及 HTTPS/gRPC 发布端口、专用卷创建和首次启动。安装包保护的镜像目录同时绑定精确 image ID、架构、普通程序摘要、离线文件大小和 SHA-256；不接受任意 registry/tag、环境变量、宿主挂载或命令。发行维护者应连同工具包分发目录内的精确 tar.gz 文件；升级工具包以变更核定目录。上传使用 [Cockpit 官方文件 API](https://docs.cockpit-project.org/cockpit-guide/main/guide/cockpit-file.html) 的 binary/root 通道，目录由固定 root 动作分配；再次核实文件后才能单独点击导入。

镜像目录分别绑定配置摘要和 OCI manifest 摘要，兼容旧 Docker 镜像存储与新 containerd 存储的不同 ID。导入前校验原压缩文件和有界 OCI 元数据之间的摘要关系。若原 load 已有真实成功回执，仅后续标识核对未通过，可在同一原编号下显式核定：要求原文件完全一致、原目录字段仍一致、OCI manifest 与配置相互绑定且实际镜像匹配；核定不调用任何 Docker 写操作，保留先前 UNKNOWN 和核定证据。没有原 load 成功回执时，镜像存在本身不足以核定，仍不重发。

已接受的导入或部署由固定 systemd 临时任务独立执行，浏览器只保存原公开 UUID、动作类型和请求步骤。主机把每个 Docker 变更的意图先落盘，实际成功返回后才记回执。只有原固定 worker 的实际活动进程可记 PENDING；进程消失且无回执、观察超时或派发结果不明均保留原操作，不重启任务、不重发命令，也不换编号逃过不确定性。另一浏览器配置也受同一 root 台账限制。镜像、卷和失败容器不自动删除或接管。

创建固定使用已核定的不可变镜像、专用命名卷、受限内存/CPU/进程数、只读根文件系统和降权入口。就绪查询先验证实际镜像、卷、发布端口、root provenance 和普通程序摘要，再以固定低权限只读命令取得叶证书指纹；核对已发布端口的精确 TLS peer 后才 GET /health/live。运行中不等于 HTTPS 就绪，打开中心页仍须单独主机名并由用户核对发布地址的证书。

此增量不表示 Docker 已配置实例的完整升级/冻结/停止/解除冻结、未知结果人工恢复、安装工具本身的图形化 bootstrap、macOS/Docker Desktop 或正式发行矩阵已验收；这些仍为内部待办。真实 Docker Engine、认证 Cockpit 和受限契约测试的证据分别记录于 reviews/registered-runtime-review-20261114.md，不能互相替代。

## 默认启动

systemd 默认及模板 unit 不再以 console.env 为启动条件；环境文件可选，固定私有 StateDirectory，`launch-check` 只核对安装负载和私有状态，不要求预先提供数据库或密钥。旧 `check` 与密钥 CLI 为高级维护入口。Docker 默认只挂状态卷，旧三文件只读 bootstrap 挂载移至显式 legacy profile；默认不接收弱口令数据库或固定 TLS/key 环境值。入口不提前 key-init/migrate，初始化由 HTTPS 向导拥有。空或全空白环境值统一视作未配置，非空值不裁剪，非UTF8值失败且不输出其内容。

初始服务提供公开 `/health/live` 200 与 `/health/ready` 503；正式服务存活200，readiness保持数据库校验。存活不证明证书信任或完成配置。实际 Docker health 仅访问固定 loopback，不传凭据。

## 可信主机页面

真实主机联调补充：Cockpit处于Limited access时，`access-denied`是应用命令执行前的已知拒绝，页面提示先在主机顶部开启管理权限，不把它归为UNKNOWN，也不自动重发。中心发布地址由用户填写，不猜测主机管理地址或用内部监听端口覆盖发布地址；打开时要求不同主机名，避免主机管理与中心会话Cookie混用。多个中心也应使用不同主机名。

包提供 Cockpit `YOFC Console Setup` 页面及固定 root helper。Cockpit 为推荐的主机管理依赖，中心运行不依赖其常驻；当前不自动启用 Cockpit 或停止其他服务。页面双语，可选默认/命名实例、首次监听端口，显式启动设置服务；可以读取状态并下载已有安装凭据。读取状态保留用户正在输入的端口。命名实例只在尚无状态目录时创建独立目录，首次端口创建私有启动配置，不能覆盖已有设置。已 active 的服务不重启；有Job或正在变化的状态拒绝再次start。启动丢响应后观察原状态，不自动重发。

首次启动发生默认端口冲突时，仅允许通过受权界面为已知inactive/failed、尚未配对且没有任何操作记录的bootstrap选择端口。原证书、私钥、安装ID和令牌保留，使用已验证的只读命令核对原身份；状态目录必须恰好只有已知bootstrap文件和允许的空目录。有session、accepted/intent/receipt、generation、未知文件或符号链接均拒绝。此入口不修改已有启动设置，也不处理已受理或UNKNOWN初始化。

页面使用已登录主机通道的 `cockpit.spawn(...,{superuser:'require',err:'message'})`，没有低权限或公开HTTP令牌后备。root helper 只接受固定动作、经过校验的实例名/端口/完整容器ID；无任意路径、命令、restart/reset接口。Docker目标必须带指定标签且正在运行，只执行固定低权限 console 用户下的凭据读取。根据 [Cockpit官方进程API](https://cockpit-project.org/guide/latest/cockpit-spawn.html)，require 通道需具备相应主机权限，不能降级为try或公开接口。

调用应用前先核对 root 保护的本地命令 provenance、实际二进制摘要和声明能力；native 包还校验 source manifest 摘要。Docker 固定本机 socket，不继承远程 context，记录与程序须 root 所有且不可被组/其他用户改写。旧制品缺记录时在 stat/cat/摘要读取阶段拒绝，绝不把未知命令当作能力探测，避免旧版本先走数据库启动逻辑。

下载通过受权通道进入浏览器Blob，令牌不显示在DOM、不写localStorage/sessionStorage或URL、不记日志；目标变化清除页面内凭据。用户将浏览器安全详情的证书SHA与主机工具的可信指纹核对后，在中心页面上传凭据。CLI只读导出校验已有私有目录/文件和证书私钥匹配；不创建身份/配对/重置或受理初始化。安装配置完成后拒绝导出。

## 维护保护和实际边界

新只读 `installation-maintenance-check` 不迁移、不恢复任务密钥、不造管理员或停止服务。首次安装记录没有匹配回执、TLS intent没有精确匹配原ActivationRequest的回执、目标数据库安装绑定不符或任务QUEUED/RUNNING/UNKNOWN时拒绝维护。包维护先检查所有实例，再请求停止，并保留停止后的检查。完整并发维护排他/写入冻结、子记录结算和失败恢复仍须实施与验证，不能把读前检查冒称已闭环的升级事务。

主机页面把进程active与HTTPS就绪分开。新的固定只读installation-status命令不连接数据库、不创建身份、不获取服务租约、不发布TLS选择，也不导出令牌、密码或私钥。它核对已选不可变TLS代际的叶证书指纹；待审批材料不会成为就绪依据。安装器先验证已安装程序及命令provenance、实际active进程的程序摘要，再在本机核对TLS peer的精确指纹，随后只GET固定/health/live。进程摘要只读取固定/proc/数字PID/exe内核路径；控制台配置/状态路径继续拒绝符号链接。返回受限的成功存活状态才标记主机HTTPS就绪；这不能代替浏览器对发布地址/域名/证书的核对。旧制品没有声明能力时拒于应用命令调用前。

如果宿主管理员没有读取运行进程程序的权限，安装器以固定的 yofc-console 账号和组启动一次只读摘要进程，清空附加组与 capabilities 并启用 NoNewPrivileges。摘要必须与已安装程序的受保护记录一致。TLS 存活探测成功后再次核对原 systemd InvocationID 和 MainPID，再返回 READY；读取被拒、摘要不符或进程在观察期间变更时保持未就绪。

启动后的界面最多追加15次只读状态查询，不重发start，不转换成restart；切换目标取消旧观察。尚未核实HTTPS时不打开本机中心页面。新安装默认尝试IPv6通配双栈，仅自动首次设置在IPv6不可用时回退IPv4；端口占用不触发回退。显式已有地址不扩大地址族。命名实例由界面创建只含CONSOLE_SETUP_PORT的私有内部启动文件；这不是需要用户填写的环境变量。去掉unit强制监听地址后，已有安装继续使用原不可变设置；高级legacy环境文件保留。向导根据实际HTTPS监听地址族和端口，初始化代理通道为同地址族的下一端口（65535时取65534），后续保留用户填写的值。

验收状态和制品SHA以docs/reviews/registered-runtime-review-20261114.md及对应target组件为准；源码版本、普通release、标准包、真实Cockpit和完整多阶段构建证据分别核对，不能用模拟桥替代主机认证，或用运行阶段镜像替代完整构建。Docker Rust构建固定Bookworm基础，限制两个编译job，保留normal/non-test release的编译消息。维护并发冻结/排他、完整平台GUI、macOS/Docker Desktop交付、凭据恢复续期及正式沙箱仍须按完整台账交付。

仅原3组external deferred；原网关UNKNOWN和完整owner/网络/API/上游台账未缩减。方案A外部CA/custody不变。源与证据在 `target/nm-role-authority-20261112/registered-runtime-v52/console-first-setup-platform-v1/`。


## Host maintenance protocol 1 (candidate)

The authenticated host installer can freeze writes across every natively installed console instance, observe the original operation, stop frozen centers, and verify the installed package before releasing the freeze. It never starts a service as part of completion. HTTP, gRPC, initialization jobs, startup migrations and mutating CLI commands share a file lease until their durable commit. The host coordinator takes the exclusive lease while checking all retained instances and publishing a root-owned durable fence under `/var/lib/yofc-console-maintenance`. Busy writers and queued/running/UNKNOWN tasks refuse maintenance before any stop. Initialization and TLS journals without final authoritative receipts also refuse maintenance.

A stop intent is persisted before invoking systemd. Only an actual successful return plus inactive/failed, MainPID=0 and empty Job produces a stop receipt. An intent without its receipt remains uncertain even if the process later disappears. It cannot be retried under either the original ID or a fresh operation. Preserve the original journal for explicit review. Root-owned fences survive package-script processes, service restart and host reboot. Native units skip automatic startup while fenced; the executable also rejects writes before connecting to PostgreSQL.

Debian prerm/preinst/postinst carry the same operation across stop and verified package installation. Protocol-0 packages cannot be upgraded online safely: dpkg invokes the old prerm before the new preinst. The candidate therefore refuses unsupported live participants. Explicit offline adoption is available only after every legacy center and job is already stopped; it masks the default and template unit without issuing a stop, verifies the retained inventory again, and retains all masks/fences on an uncertain result. A general GUI delivery/bootstrap path for upgrading these old helpers is still pending. Do not describe the new preinst as repairing an already executed old prerm.

Standalone/Docker installations use a private `.lifecycle` directory in their selected state volume. The local maintenance begin/status/complete commands preserve the original UUID and fail closed after uncertainty. Docker deployment selection and complete GUI maintenance remain pending. Offline cryptographic utilities without an installation state selection coordinate in their output directory; installation maintenance requires the explicit matching state selection. CA custody remains external (scheme A).


## Independent host tools and cancellation (candidate)

`kura-management-console-installer` is an independently updatable, architecture-independent package containing the Cockpit page, root helpers and their separate checksummed provenance. It contains no center executable, unit, state directory or maintainer script, and updating it does not stop/start a center or run migrations. Its `Replaces` declaration transfers the helper files from older monolithic prototype packages; subsequent center packages depend on the matching tools version and no longer own those files. The center executable/source provenance remains separate. A complete installer/bootstrap delivery UI and formal release matrix remain pending.

Before every maintenance mutation the browser persists only the original public UUID and requested step. Credentials, passwords, private keys and cookies are never put in this record. Reloading cannot turn an uncertain stop or completion into another request. An absent server receipt is unobserved and does not certify non-delivery. A completed operation keeps its original identity and result visible.

When the original server journal has no native stop intent, an authorized administrator can explicitly cancel that maintenance. Under the same host/exclusive writer locks, the coordinator retires the original UUID before removing the fence; any delayed original begin/stop is rejected. An unseen request receives a durable cancellation tombstone without changing another operation's fence. The receipt deliberately does not claim transport non-delivery. Existing COMPLETE readers remain compatible through an explicit cancellation outcome. A native stop intent, receipt uncertainty, offline adoption/mask, changed participants or non-running service forbids this cancellation. Cancellation issues no stop/start/restart. If the cancellation receipt is durable but pointer removal failed, the UI can finish that verified cancellation as a separate action; it does not repeat an unknown stop. Original initialization/remote-write UNKNOWN states are outside this host-maintenance operation and are never reset by cancellation.
