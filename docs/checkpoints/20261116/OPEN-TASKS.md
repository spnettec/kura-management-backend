## 2026-11-22 已发布密钥代的 purge 签名与同围栏续装

Core 82b242fa33 在独立 Debian ARM64 正式实装中补齐已发布密钥代的 purge 后合法重装。应用数据已删除、完成签名未写入时 SIGTERM，正式 incoming reader 拒绝；随后沿原维护号 5c743dde-a024-496d-a266-a8a7f0cedc74 续 purge、解包和配置。六份小型原凭据 cohort 文件共 11148 字节，19 份配置签名与五个 vendor 单元配对；HTTPS 原恢复与归档保持。显式 Main 从工厂应用状态写入并读回原发布代加密的新快照，27 份保护文件指纹保持，系统正常停机。91 项 ARM64 测试、217 个 JVM 插件路径字节核对通过。旧半配置和 purge 后拒绝两现场继续冻结，12 父项冻结 0/12。冷读入口 generation-recovery-purge-cohort/cold.py。

## 2026-11-22 原 READY 签名续配、原归档与双代 Main

Core 78623395ce 的 ARM64 原 Root 2097298:1 已核实 READY 后的进程中断。沿原维护号 46749956-5e81-4ae5-a1e4-6110f01957a9 续配一次，18 份签名保持并新增第 19 份完成回执，五份配对单元核验通过。HTTPS 只读查回原恢复 SUCCEEDED 和原归档 ARCHIVE_RECORDED。随后显式启动正式 Main 读回原两代快照 24101／24102，确认原启动并正常停机，27 份保留文件完整指纹保持。旧版半配置与 purge 后拒绝的两处原现场保持冻结。冻结 0/12。 冷读入口 generation-recovery-retained-current/cold.py。

## 2026-11-22 正式浏览器多代密钥恢复与原号续查

Core 08da5e88d4 的正式 ARM64 恢复页显示双摘要并在原生确认框核对。Chromium 实际发送一次恢复。观察器丢失结果后，核对同一 Root 的原回执，再在新浏览器会话沿原号续查。模拟错误 Root 标识时原号查询为零。显式启动后 Main 读回两代 OSGi 值 24101／24102，清结原启动交接并正常停放。88 项 ARM64 测试、64 项浏览器回归及 217 个正式 JVM 插件路径核对通过。冻结 0/12。 冷读入口 generation-recovery-browser/cold.py。

## 2026-11-22 独立 HTTPS 双凭据恢复及显式启动

Core 2cc010cfd6 的独立非 Root HTTPS 服务在 Debian ARM64 正式实装中核验原多代加密备份，恢复两份缺失凭据。HTTPS 服务重启后原恢复号读回 SUCCEEDED，Main 仍停机。另经显式 HTTPS 启动使正式 Main 读回两代 OSGi 配置 24101／24102，完成原启动交接并正常停放。归档预览验证两份摘要配对。188 项 ARM64 回归通过，其中三项以普通账号补验，217 个插件路径与既有全量验证包一致。浏览器页面及其端到端验收继续待办。冻结 0/12。 冷读入口 generation-recovery-https/cold.py。

## 2026-11-22 双凭据恢复中断后沿原检查点续办

Core f31e4b965c 在隔离 Debian ARM64 正式 deb 实测双凭据恢复。两份凭据缺失，keyring 落盘后测试进程退出。正式 Root worker 重启后沿原号 cf2da0b0-eace-4c58-9667-a5613357d3c4 和签名检查点仅补齐活跃钥。正式 Main 显式启动后经 OSGi 读回两代配置，非 Root agent 验证同两份密文。128 项 ARM64 回归和 217 个插件路径核对通过，系统正常停放。历史无证明 UNKNOWN 保持冻结。冻结 0/12，恢复界面和完整 provider 密钥生命周期待验。 冷读入口 generation-restore-continuation/cold.py。

## 2026-11-22 缺失 keyring 的 ARM64 正式恢复与两代读回

ARM64 Core dc479d3266 通过 22 项发布测试，正式 deb 的 217 个插件与 484c4c843c 字节一致。隔离 Root 2097259:1 保留原活跃钥，只按原生成备份恢复缺失 keyring。正规启动的 Main 经 OSGi 读回两代配置 24101／24102，非 Root agent a998ebe 验证同两份密文，原活跃钥、两份回执和旧 keyring 原件保留。该系统正常停放。另一个 Root 2097235:1 的发布 UNKNOWN 因缺原停机证明沿原号冻结，Main 正常停止，未重发。冷读入口 generation-publication-stop-diagnostic/cold.py。冻结 0/12。

## 2026-11-22 正式 ARM64 非 Root agent 的快照 FD 正向验收

正式 ARM64 agent a998ebe 经本地 Root 证实可向 Core 2ad663286a 交付只读 FD。独立 Root 从 266adf5 沿原初始化与维护号解包、续配并读回 APPLIED。正式命令通过明文、AES-GCM、超过 1 MiB 和恰好 32 MiB 四份快照；截断、尾随、错摘要、额外 unit 参数和 broker 停用均拒绝。四份 Core 保护文件保持，无恢复事务或启动记录，Main disabled，系统正常停放。冷读入口 native-snapshot-validation-request/cold.py。冻结 0/12。

## 2026-11-22 ARM64 独立快照校验 socket 与 Core 维护

Core 2ad663286a 与 agent 266adf5 接入独立只读快照 socket。ARM64 83 项定向 Python 测试和宿主 3 项 Rust 测试通过。两个正式 deb 在隔离 Root 实装，原 Core 217 个插件路径字节一致。实际非 Root agent 发布同一 Root 的启动凭据，同 UID 同组的独立 peer 被拒绝。Core remove、purge、同版重装沿原维护号完成，agent 调用标识保持，新 socket 恢复。Main 全程无 PID，重装后显式恢复 disabled，系统正常停放。正式非 Root 业务调用与只读 FD 正向验收仍待完成。冷读入口 native-snapshot-validation-broker/cold.py。冻结 0/12。

## 2026-11-22 正式 ARM64 非 Root agent 的快照校验准入

正式 ARM64 Core 484c4c843c 与 native-nonroot-v1 agent 6910532 在隔离 Root 实装。固定 loader、bridge 和 18 个 agent 载荷、4 个钩子及 19 个 Core 资产核对通过。正式 agent 在 Core socket 停用时以非 Root、cap0 启动。恢复 socket 后，同身份探针收到 EACCES，现有权限保持；kurad 交付合成只读 FD，由正式 worker 校验通过。四份受保护文件保持，Root 操作及启动表为空，系统正常停放。非 Root 正向校验仍待专门授权入口。冷读入口 native-agent-validation-admission/cold.py。冻结 0/12。

## 2026-11-22 ARM64 systemd 下的只读快照 FD 验证

Core 快照只读 FD 的 ARM64 systemd 专项通过。Core 484c4c843c 源码和正式 deb 提取件共 19 个执行资产逐字节匹配，复用与 agent 6910532 相同 Rust 源码的 ARM64 测试二进制。实测测试客户端在无 Root socket 时可初始化，Root worker 起后验证明文、新代及旧代密文三份快照。直接读取凭据与备份请求均拒绝，密钥指纹一致，Root 操作及启动表均为空。Core bb2e8b24a0 仅修复 systemd 测试夹具。系统正常停放，入口 core-snapshot-fd-systemd/cold.py。冻结 0/12。

## 2026-11-22 密钥准备取消跨 Core purge 与同版重装

隔离 Debian ARM64 Root 2097235:1 以 Core 484c4c843c 正式安装。local-admin 各派发一次准备、校验和取消，原取消回执跨实际 Root worker 重启读回。真实 remove、purge、unpack、configure 各执行一次。13 份历史与凭据文件指纹一致，重装后按原编号读回三笔 SUCCEEDED 和 CANCELLED 历史。正式 Main 的 CryptoService 成功解开移除前生成的同一份合成密文，系统正常停放。冷读入口 cancelled-generation-package-lifecycle/cold.py。冻结 0/12。

## 2026-11-22 Core ARM64 原生发行合同与 broker 配对

Core 484c4c843c 的 Debian ARM64 原生合同测试完成汇总。79 个测试模块中 1220 项在 Linux ARM64 通过，7 项 polkit JavaScript 在 macOS ARM64 补验通过，另通过真实 Unix peer 归档入口。Core 50c02de8bc 只更新归档测试夹具，覆盖 writer 持锁拒绝及正常释放后同一历史续接。正式 deb 的 9 份 broker 清单共 103 项文件及 21 份 systemd 单元副本，摘要与配对全部通过。独立 systemd 和宿主编排的 11 个场景入口单独登记。构建容器已回收。冷读入口 core-native-contracts-484c4c8/cold.py。冻结 0/12。

## 2026-11-22 Container 正向 OCI 验证的 OrbStack 环境边界

Container 9a7cda7 的 OCI 正向验证定位了 OrbStack 的内核限制。Docker 内的独立系统在运行时选用无会话 keyring 模式后收到 pivot_root 拒绝。另在 OrbStack 隔离 Debian ARM64 系统重建独立 systemd 运行环境，正式 Core 484c4c843c 的 Main 成功通过凭据准入，但 runc 无法附加 BPF_CGROUP_DEVICE，进程仍为 PID 0。两笔明确失败均已封存，provider 已 purge，两套系统均停放。实际 OCI 启动、跨 Main 重启和 TERM 停止仍待验。冷读入口 container-oci-lifecycle/cold.py。冻结 0/12。

## 2026-11-22 正式 Main 传递 Container 命令失败状态

Container 9a7cda7 保留 OS_COMMAND_ERROR，补齐操作、对象及可用的 Docker HTTP 状态，并保留异常原因。JDK 21 共 133 项测试通过。ARM64 独立系统使用 Core 484c4c843c，同一 Root 从 d060603 解包并续配至 9a7cda7。正式 Main 按原快照从独立 Docker 26.1.5 引擎收到启动 HTTP 404 和移除镜像 HTTP 409，两个错误均准确返回。原夹具容器与镜像保持，provider 已 remove、purge，系统正常停放。冷读入口 container-command-errors/cold.py。冻结 0/12。

## 2026-11-22 ARM64 原构建容器续跑完整 Core/OSGi 验证

Core 484c4c843c 在原 ARM64 构建容器解冻后完成离线验证。82 个模块成功，259 份 XML 与日志均为 2161 项通过，其中真实 Equinox 68 项，失败、错误、跳过均为零。Equinox 实际打开的配置与 keystore bundle 均与本轮模块输出逐字节一致。正式 deb 的 41 个 Core 插件共 1208 个 class 完全一致，全部条目差异仅为 Bundle-Version 时间限定符；其余 176 个依赖路径均匹配只读 Maven 输入。证据入口 core-reactor-484c4c8/cold.py。只读复核入口 core-reactor-484c4c8-readback/check.py，已按原 ID 核对构建器回收，重新读取正式 deb 全部插件路径并复算 Equinox 文件记录。冻结 0/12。后续按用户指定的暂停入口再次只读复算，结果与原证据逐字节一致。绑定网关及六组件仍停放，本次无包操作。见 core-reactor-484c4c8-resume-audit/HANDOFF.md。

## 2026-11-22 OPC UA 网关验证服务端证书撤销

OPC UA 网关客户端可通过 server.crl.path 验证服务端证书撤销。独立 ARM64 正式 Main 用 d8dcc4f、原固定叶证书和快照 1795300935890 成功加密读节点。原位 remove、unpack、configure 后仅更新服务端 CRL，同一快照的新 Main 拒绝原证书并记录 Bad_CertificateRevoked。当前 cf0677e 的 14 项测试通过，六个包内 JAR 的 3171 个 class 条目逐项等同于实装版。OPC UA 已 purge，系统正常停放。只读入口 opcua-server-crl/cold.py。冻结 0/12。

## 2026-11-22 六组件原卷 Core 升级与独立 Main 异常退出接管

六组件 ARM64 原 Root 39:6611544 已将 Core fc17697eba 升至 484c4c843c。正式 Main 读回原业务包、九个快照，FD 8 与升级后 data 目录一致，FD 9 与原生 writer lease 一致。停机冷读 50 笔维护、911 条签名阶段。另在独立内存系统确认 JVM 被 SIGKILL 后原锁释放，systemd 自动启动后继 Main 并接管同一目录。六组件入口 six-native-launch-upgrade/cold.py final；独立进程证据 six-native-launch-upgrade/cold-process.py。冻结 0/12。

## 2026-11-22 原生启动器并发与正常接管

Core 484c4c843c 修复原生启动器的并发窗口。ARM64 正式 deb 的四种启动方式遇到持锁者均返回 1；正式 Main 两次从自身读到数据目录的同一 FD 8 inode，独立进程在运行时取锁失败、正常停机后成功。运行中三个直接入口均被拒，原 config.ini 与 PID 文件指纹一致。两个临时系统正常停机，现场入口 native-launch-exclusion/cold.py。六组件原卷仍使用 fc17697eba。冻结 0/12。

## 2026-11-22 Docker 启动器同卷争用、接管和进程中断恢复

ARM64 正式 Docker 启动器在同一持久卷上只允许一个 Main 持锁。后继 5be1a75 在卷占用时明确说明原因并返回 1。正常交接和 JVM 被 SIGKILL 后均沿同一卷、同一锁 inode 恢复，HTTPS 证书及 11 份文件完整指纹一致。三个实例最终均正常停放。当前入口 docker-instance-lock-handoff/successor.py cold。冻结 0/12。

## 2026-11-22 本地卸载先进入 Felix 时，云端安装等待原注册表

独立 ARM64 正式 Main 实测本地卸载先进入 Felix，云端安装 74002 的虚拟线程等待同一注册表的 dispatch。释放后通知与原号 GET 均为 COMPLETED，业务包资源可读。另一笔云端卸载 74001 也按原号读回 COMPLETED，其等待位置未采到。停机冷读两份完成回执、四个出厂包及一个业务包。当前入口 deployment-local-first-dispatch/cold.py。冻结 0/12。

## 2026-11-22 未决安装回执跨 Deployment 后继升级、直接 purge 与重装

ARM64 原 Root 39:7997146 上，Deployment 由 1b043a4 升至 6c0da58。解包停机后沿原围栏续配，随后直接 purge、停机并沿原 FileMove 围栏重装。升级和重装后的正式 Main 均只读查回 47001 COMPLETED、47002 UNKNOWN。12 份原文件、19 份 Core 配置历史和业务目录身份保持，最终正常停放。冷读入口 pending-install-deployment-upgrade/cold.py restored。冻结 0/12。

## 2026-11-22 六组件原卷 Deployment 后继升级与解包停机恢复

六组件 ARM64 原 Root 39:6611544 已由 Deployment 1b043a4 升至 6c0da58。解包后正常停机，沿原 Main 和 FileMove 围栏仅续配一次。正式 Main 读回原业务 DP 资源，七个实装类摘要与发行件一致。观察时云端 handler 未注册。停机核对 49 笔维护、892 条签名阶段及 319 项可变文件。当前冷读入口 six-deployment-delay-upgrade/cold.py final。冻结 0/12。

## 2026-11-22 Deployment 执行前校验显式重启延迟

Deployment 6c0da58 在下载、安装、卸载的 EXEC 入口拒绝显式重启的负延迟。262 项 Maven 测试通过。原 ARM64 Root 原位升级后，正式 Main 对三个新请求均返回 BAD_REQUEST，两笔原部署任务经 GET 仍为 COMPLETED。原重启 UNKNOWN 保留，正常停机。当前冷读入口 deployment-reboot-delay-admission/cold.py final。冻结 0/12。

## 2026-11-22 原部署完成回执跨 Cloud 与 Deployment 联合 purge、停机和重装

同一 ARM64 Root 的 Cloud 与 Deployment 经 purge、重新解包和停机后，沿原围栏续配。正式 Main 只读查询 71001 安装与 71002 卸载，两者均为 COMPLETED；业务包保持已卸载，原重启 UNKNOWN 保留。两份部署回执与原注册表身份、元数据、摘要一致。当前停机入口 cloud-deployment-receipt-reinstall/cold.py final。冻结 0/12。

## 2026-11-22 Deployment 的安装、卸载及后续重启判定

独立 ARM64 正式 Main 的 Deployment 安装任务 71001 与卸载任务 71002 均为 COMPLETED。两次部署后重启调用均只读原 UNKNOWN，原 Root 39:8337304 的 actor 始终只有同一编号。正常停机后核对两份部署回执、原九份请求相关文件、三份凭据和 11 个发行文件。当前冷读入口 deployment-original-reboot-receipt/cold.py final。冻结 0/12。 实测通过 JVM 内已注册的请求处理器。真实传输和内核重启后的业务恢复仍待验。

## 2026-11-22 watchdog 超时读取原 UNKNOWN 与配置恢复

ARM64 正式 Main 的 watchdog 在设备缺席宽限期后沿原编号读取 UNKNOWN。启用期间和恢复原禁用配置后分别观察 65.1 秒、65.1 秒，均未再读原回执，日志无软件重试。原 Root 39:8337304 正常停机，九份请求相关文件、三份凭据均与基线一致。停机入口 watchdog-original-retry-window/cold.py final。冻结 0/12。 打开设备后的关键组件超时、实际后继包接管原 pending 仍待验。

## 2026-11-22 公开 OSGi 系统管理服务核对原 UNKNOWN

独立 ARM64 正式 Main 通过 OSGi SystemAdminService.reboot() 核对原重启请求。原 actor 回执在调用期间被读取，SystemAdminServiceImpl 记录 UNKNOWN 告警，void 方法返回。原 Root 39:8337304 正常停机，九份请求相关文件和三份凭据完整指纹相符。当前冷读入口 reboot-system-admin-unknown/cold.py final。绑定网关 6910532 停机核对五笔维护、三代来源、原任务 SUCCEEDED 和远程 OFF。冻结 0/12。 watchdog 超时入口和后继实现接管原 pending 待实测。

## 2026-11-22 正式 Main 的重启配对修复与 UNKNOWN 跨 purge 恢复

Core 2cecea4bad 修复 /etc/kura 为 0700 时正式 Main 无法读取重启配对的问题。独立 ARM64 原 Root 39:8337304 中，真实 actor 在屏蔽重启目标时记录 UNKNOWN。经历 remove、purge、解包停机与续配后，第二个 Main 仅查询原号 63ec28ee-ad3b-4302-92ac-33b7be6a5083，pending 保留，九份原文件和三份凭据指纹一致。冻结 0/12。 当前停机入口 reboot-original-unknown-cohort/cold.py final。

## 2026-11-22 OPC UA 多级客户端证书跨重装与撤销

ARM64 正式 Main 以三证书链建立加密 OPC UA 会话，服务端仅信任根 CA。保留重装后从原快照 1795295949685 再次读节点；撤销原客户端叶证书后第三个 Main 被拒绝，成功会话仍为二。旧 Milo 0.6.16 的多级链会话未通，Milo 1.1.2 已实测。最终 purge、正常停放。冻结 0/12。 见 opcua-ca-maintenance/HANDOFF.md。

## 2026-11-22 六组件原卷升级后用原业务 PKCS12 验签

六组件原 Root 39:6611544 从 89dfd6a 升至 fc17697eba，维护号 199ef4aa-04e9-48ff-aec1-336f088d38d4。正式 Main 从原快照 1795223609366 取到业务口令，打开原 PKCS12 并用原证书验签。最终停机冷读 49 笔维护、892 条签名阶段，六包 264 个 JAR 匹配，319 项可变文件按原记录核对一致。冻结 0/12。 当前停机入口为 six-keystore-cohort-upgrade/cold.py final。

## 2026-11-22 已发布两代密钥的原卷在移除停机后沿原号完成 purge

Core fc17697eba 在原 Root 39:8285453 完成最终 purge。正式 remove 后正常停机并冷读，重启后沿 990fac5c-9991-4e8b-b46e-d013e45366ae 清除，dpkg 审计为空。最终停机冷读 33 份受保护文件及三笔历史的 57 条签名记录，卷内 root 私有归档保留原测试快照、证书库和启动脚本。冻结 0/12。 原卷当前只读入口 published-generation-final-purge/cold.py final。

## 2026-11-22 自设 SSL 信任库口令跨密钥换代及已发布代的清除

Core fc17697eba 的两个正式 ARM64 Main 从换代前后两份加密快照读回站点自设 SSL 口令，均能打开同一 JKS，两张信任证书及 HTTPS 签名证书匹配。该临时安装沿同一维护号完成 remove 和 purge，17 份凭据与密钥服务记录在停机前核对一致，容器正常退出。原两代卷仍停放，冻结 0/12。 原两代停放卷的最终 purge 仍待验。见 ssl-trust-generation-purge/HANDOFF.md。

## 2026-11-22 两代快照与站点自设 HTTPS 口令跨 Core 升级

Core 075ea8b733 至 fc17697eba 的原位升级经两个 ARM64 安装实测。原两代卷解包停机后沿 d15469c5-fae4-4bcc-9cd1-809cdbf06522 续配，正式 Main 读回两代值、原 TLS 身份与两张信任证书。独立安装通过配置服务设置测试 HTTPS 口令；升级后从原加密快照用该口令打开 PKCS12、验签并核对 TLS。两卷均正常停机，本轮新增发行包与持久卷均为零。冻结 0/12。 原两代卷当前停机入口为 published-generation-keystore-upgrade/cold-read.py final。

## 2026-11-22 HTTPS 身份库与 SSL 信任库的出厂快照口令

Core fc17697eba 移除两份工厂模板中绑定旧 AES 钥的密文。旧 Root 39:8285453 的两代快照均保留相同的出厂字段，两份库实际依靠 CryptoService 的默认口令打开。修正版独立 ARM64 安装的两个正式 Main 从同一加密快照直接打开 HTTPS PKCS12 与 SSL JKS，验过原签名、回环 TLS 和两张信任证书。两个现场正常停机，旧快照迁移仍待验。冻结 0/12。 当前原两代卷冷读入口 published-generation-keystore-providers/run.py cold。

## 2026-11-22 已发布密钥代在五种直接启动方式下的实读

Core 075ea8b733 在原 ARM64 安装 39:8285453 通过五种直接启动方式读回两代原加密快照。四种 start_kura.sh 参数组合及专用调试脚本各有独立 JVM，账号均为 kurad，有效能力均为 CAP_NET_BIND_SERVICE，writer 登记与原发布 cohort 匹配。正常停机后 42 份文件、19 份原签名记录及三个启动脚本核对一致。复用原卷和已装包。冻结 0/12。 当前停机入口为 published-generation-direct-launchers/remaining.py cold。

## 2026-11-22 已发布密钥代的保留重装与两代快照读回

Core 075ea8b733 的 ARM64 保留重装在独立 Root 39:8285453 通过。解包后正常停机，dpkg 创建的空目录由包自行恢复为链接。沿原移除编号 448acb95-e358-4c69-af41-f1ff27c25a9d 验过 19 条签名记录，三个启动脚本与发布时一致。正式 Main 读回两代加密快照 24101 和 24102，随后正常停放。原半配置卷仍在原编号下停放。冻结 0/12。 见 published-generation-current-reinstall/HANDOFF.md。

## 2026-11-22 Chrony 调用方的原 pending 跨 Core purge 和重装

单独 ARM64 安装在 Chrony STOP 的 actor 回执已为 APPLIED、调用方仍有 pending 和 issued 时实际 remove、purge、unpack、configure。944 字节私有档跨 purge 保持原 inode 和摘要，重装按原 Core 维护号恢复三份调用方文件。正式 Main 沿原时钟编号完成结算，actor 仍只有一笔操作，系统正常停机。冻结 0/12。 详见 clock-pending-purge-reinstall/HANDOFF.md。

## 2026-11-22 正式 Main 持久化 Chrony STOP 原回执

Core 075ea8b733 已由三个正式 ARM64 Main 续办一次 Chrony STOP。Root actor 在 d3cf526a-d20c-4e09-97a0-e4197aa54676 写入 APPLIED/CHRONY_STOPPED。第一进程遇到本地完成目录权限拒绝，第二进程已落盘完成回执、pending 文件仍为挂载点，第三进程核对原回执后清除 pending。本地完成 inode 4974 和四份 actor 文件指纹相同，Chrony 从未启动。冻结 0/12。 详见 chrony-main-sender-recovery/HANDOFF.md。

## 2026-11-22 重启与时钟回执同步失败后的原号恢复

Core 075ea8b733 修复重启与时钟调用方遇到回执同步失败后过早清除 pending。独立 ARM64 正式安装核对 39 个包内副本。两种调用方各在两个进程的同步点先后退出，第三个进程沿原号只读查询后清除 pending，回执 inode 和内容保持，夹具动作各一次。148 个不同定向用例通过，原六组件仍装 89dfd6a，冻结 0/12。 实测采用进程退出与测试动作端，详见 typed-sender-receipt-sync/HANDOFF.md。

## 2026-11-22 六组件原卷升级后沿解包围栏恢复

六组件原 Root 39:6611544 已由 e99caab 升至 89dfd6a。解包后正常停机，冷读 47 笔历史、854 条签名阶段，沿 8adedbad-895a-4f04-9743-4d4f75bd0796 续配成功。正式 Main 同时读到 generic-device、九份原快照及原部署包资源。最终停机读回 48 笔、873 条签名阶段，六包 264 个 JAR 相符，319 项可变文件与原记录完全一致。冻结 0/12。 六组件停机入口改用 six-core-board-resume/cold.py final。

## 2026-11-22 六组件原卷文件权限与后继包差异

六组件原 Root 39:6611544 停机核对 755 个包文件和链接。以原 kurad 账号的 UID、GID 和附加组在只读挂载上打开 263 个 /opt 下的 JAR，摘要全部匹配。Core e99caab 至候选 89dfd6a 的文件与钩子共九处内容变化，233 个 Core JAR 相同。绑定网关 6910532 的五笔维护已结束，原任务 SUCCEEDED、OFF 保持，冻结 0/12。 详见 six-package-permissions/HANDOFF.md。

## 2026-11-22 OPC UA 加密会话与原证书保留重装

OPC UA c963584 修复 Milo 客户端证书链缺失，支持显式选择 SignAndEncrypt。正式 ARM64 Main PID 2944 与保留重装后的 PID 3795 从同一快照 1795292258810 成功读节点。服务端两次均报告 Basic256Sha256 / SignAndEncrypt，双方证书摘要一致。11 项测试通过，最终 purge、正常停放。示例服务端有终端证书 KeyUsage 检查被抑制的告警，严格用途与撤销验证仍待验。冻结 0/12。 详见 opcua-secure-maintenance/HANDOFF.md。

## 2026-11-22 正式 Main 的 OPC UA 订阅恢复与保留重装

正式 ARM64 Main PID 1882 通过既有 OPC UA db7cedb 订阅服务器时间。Milo 端点退出并重启后，原监听收到新的时间，未再次注册。移除、解包和续配后，新 Main PID 3184 从原快照 1795291901848 再次收到订阅回调，两次取消注册后均观察到计数稳定。最终 purge，Main 正常停放。未新增持久卷。签名加密及其他 provider/launcher 仍待验，冻结 0/12。 见 opcua-subscription-maintenance/HANDOFF.md。

## 2026-11-22 Core purge 删除部分文件后的原编号恢复

Core 89dfd6a53d 的正式 purge 在删除 4096 个测试文件中的 3 个后被 SIGTERM 中断。ARM64 Root 39:8230388 正常停机，冷读 HMAC 和原目录 inode 后沿移除编号 71230014-dfd7-4eb8-b195-dcfeb014645d 续办。最终包已清除、审计为空、四份凭据与维护文件指纹一致。原发布密钥代的半配置 Root 39:8195025 继续停放。冻结 0/12。 见 core-purge-partial-tree/HANDOFF.md。

## 2026-11-22 Core purge 保留目录挂载拒绝与原编号续办

Core e7b690954a 完成保留 user/ 内有挂载时的正式 purge 验证。ARM64 Root 39:8221140 在 purge 删除前拒绝，正常停机检查八份原文件；重启后沿 022bd80f-97eb-4319-b0ed-b37f2df7604f 续办成功，包已清除，四份凭据和维护文件指纹相同。原半配置 Root 39:8195025 继续停放，空目录尚待用户确认移存。递归删除中途退出仍待验，冻结 0/12。 见 retention-purge-mounts/HANDOFF.md。

## 2026-11-22 Core 保留重装的解包与配置停点

Core 3b3fc8df46 在 ARM64 原 Root 39:8195025 沿移除编号 69b222b6-002f-46d5-aaa7-03a99b67e906 完成实际重装解包。configure 意图已签名，layout 尚未派发。dpkg 创建了归 kura-core 独占的空目录 39:8216097，原 postinst 拒绝继续。现场已正常停机，冷读 42 份文件、原 38 份阶段记录及本次意图。后继 3654dc4f8b 已构建，40 项 ARM64 测试通过，已在原卷只读识别该目录。原编号续配、Main 冷读和 purge 待验，冻结 0/12。 详见 retained-generation-reconfigure/HANDOFF.md。

## 2026-11-22 已发布密钥代的 Core 原生移除

Core 1590e368c1 复用 Root 39:8195025 完成实际 dpkg remove。包状态为 config-files，移除编号 69b222b6-002f-46d5-aaa7-03a99b67e906。保留的三份启动脚本与签名回执共 9533 字节，停机核对原发布记录、两代快照和 42 份文件。现行 preinst 仍要求已移除的 /opt/eclipse/kura，保留重装与 purge 待验。最新冷读入口为 retained-generation-launchers/cold.py。冻结 0/12。 详见 retained-generation-launchers/HANDOFF.md。

## 2026-11-22 已发布密钥代跨包升级后的正式 Main 冷读

Core b57e5bccfe 在 ARM64 Root 39:8195025 从 09851262d5 升级。两包密钥读取器清单相同。解包停机后沿 20ea89bb-deb1-4809-bb3a-f285cf967417 续配，三份启动脚本按发布回执恢复，19 份维护记录 HMAC 均通过。正式 Main 冷启动读回两代快照 1795290300532、1795290311731，值为 24101 和 24102。现场 disabled、正常停机。旧版已派发布局且缺成功见证的原围栏，以及不同 cohort 的原围栏，各按原入口保留。冻结 0/12。 证据见 published-launcher-reinstall/HANDOFF.md。

## 2026-11-22 已发布密钥代的包维护通过解包，在续配时发现启动脚本差异

Core 09851262d5 以原发布回执、已装 cohort 和入包清单校验维护准入。ARM64 定向测试 89 项通过。在新安装 Root 39:8183920，同版本 deb 解包成功，正常停机，再沿原编号 4a4f366b-cedb-45df-8edf-d4a871ef3312 续配。布局阶段发现三份启动脚本的初始堆由发布时 256m 回到包内 1024m，配对拒绝。现场 half-configured、Main disabled、正常停机，42 个原文件冷读指纹一致。旧 Root 39:8169098 只读核验，仍须跨 cohort 迁移。冻结闭环 0/12。 证据见 generation-package-admission/HANDOFF.md。

## 2026-11-22 已发布密钥代的 Core 配置快照和包维护

ARM64 独立 Root 39:8169098 的正式 Main 在密钥代切换后读回旧快照 1795289323248 与新快照 1795289334086。随后同版本重装的 prerm、preinst 均拒绝 Root 生成的 systemd 凭据配置。dpkg 仍为 installed，原维护围栏 f791ff07-6fa2-4a19-97bf-cd2b39dc1757 处于 MAINTENANCE。现场 disabled、正常停机，42 个原文件的冷读指纹一致。包维护恢复待实现，冻结 0/12。 证据见 snapshot-generation-field/HANDOFF.md。

## 2026-11-22 Container 编排依赖去重与正式 Main 升级

Container d060603 取消 16 份 lib/ 重复 JAR，deb 从 16,454,006 降为 8,262,616 字节。JDK 21 的 133 项测试通过，PDE 目录布局与 ARM64 实装均加载 15 个类。原 Root 升级后保留快照，正式 Main 查询到原镜像及新增镜像，remove、purge、正常停机完成。进程正向运行仍为内部待验。冻结 0/12。 证据见 container-bundle-layout/HANDOFF.md。

## 2026-11-22 Core 旧安装的用户设备名与容器启动拒绝后的恢复

同一 ARM64 Root 上 e99caab 升级 a16b65c 后，包内 device_name 已修复为 generic-device，前后两次正式 Main 均读到原用户设备名；用户属性和原快照的内容及元数据通过核对。独立 Container 3ea6634 现场的 OCI 引擎在会话 keyring 阶段拒绝进程启动，随后 remove、解包、续配、冷启动 Main 查回原 ID、purge 均已核实。正向运行仍为内部待验，原六组件和绑定网关只读。详见 core-board-upgrade/HANDOFF.md 与 container-process-maintenance/HANDOFF.md。冻结 0/12。

## 2026-11-22 无以太网时的 Core 板型修复

Core a16b65c 已完成两种 ARM64 新安装的正式 Main 验证。仅 lo 时 SystemService 返回 generic-device、aarch64、eth0；有两个 dummy 有线接口时返回 enxboard0。原六组件仍装 e99caab，旧安装升级后的属性与用户覆盖仍待验。绑定网关 6910532 原五笔维护均已结束，停机冷读与上轮一致。见 core-board-install/HANDOFF.md。冻结 0/12。

## 2026-11-22 六组件原卷完成当前 Core 升级与合法 Main 实读

原 Root 39:6611544 已装 e99caab，六包运行源码与当前 WIP 对齐，Deployment 仅差说明文档。正式 Main 读回原九份快照及原业务包资源。47 笔 Core 维护的 854 条签名阶段和 319 个可变文件均核实。六包 264 个 JAR 全部配对。无以太网时安装器漏掉板型替换，kura.device.name 留下 device_name，须继续修正。当前六组件冷读为 `six-core-cohort/cold.py final`，见 `six-core-cohort/HANDOFF.md`。冻结 0/12。

## 2026-11-22 原六组件沿本现场 bridge 升级 agent

原 Root 39:6611544 的 agent 已由 5be64b7 升至 6910532，沿用原 bridge 532c124e 和 loader。解包后正常停机，原 Root 续配一次后再次正常停机。已核对原业务、18 个实际发行文件、4 个包钩子、46 笔 Core 历史及 835 条签名记录。新的六组件冷读入口为 `six-agent-cohort/cold.py final`。当前六组件与 WIP 的剩余产品源码差异集中在 Core d97260d → e99caab，Deployment 的后继仅改文档。详见 `six-agent-cohort/HANDOFF.md`。冻结 0/12。

## 2026-11-22 正式 Main 的容器编排与引擎离线停用

Container 3ea6634 修复专属 Unix socket 连接和离线客户端清理。ARM64 实装从原快照读回真实引擎内的停放容器，引擎离线时停用编排，恢复后原服务及冷启动 Main 均维持停用。133 项测试通过，remove、purge、正常停机已核对。容器进程执行与其他业务维护组合仍待验。详见 `container-engine-maintenance/HANDOFF.md`，冻结 0/12。

## 2026-11-22 Triton 远程模式解除多余本机服务依赖

ARM64 实装前版在无匹配执行器和解密服务时无法满足远程实例引用。7de1cc0 移除这两项，容器模式也移除未使用的执行器引用。正式 Main 沿同一快照完成真实回环 gRPC 健康和模型状态查询、端点停启、冷启动、remove 和 purge。119 项模块测试通过。macOS 独立 SCR 验收通过；按同一 Git 重建两个过期 YOFC 缓存输入后，两个下游 bundle 均为 ACTIVE，43 项下游测试通过。实际模型推理及容器/原生模式的正向执行仍待验。详见 `triton-remote-maintenance/HANDOFF.md`，冻结 0/12。

## 2026-11-22 同名 OPC UA 部署包热升级

旧包 52001 因缺旧 crypto 导出无法启用驱动，52002 安装完成后仍有三个 Milo fragment 未接入，实读失败。db7cedb 将重定位依赖直接声明在四个新版宿主上。52004 同名热升级和 Main 冷启动均通过原工厂读到 Milo 产品名，三个原安装号查询均为 COMPLETED。52003 卸载后保留两份历史 DP 归档。随后原配置跨 Debian 包升级再次实读，purge 与正常停放完成。PDE 依赖入口已统一到 9f914b4。详见 `opcua-deployment-upgrade/HANDOFF.md`。其余 provider/launcher 及六组件矩阵继续待验，冻结 0/12。

## 2026-11-22 OPC UA 部署包正式投递与卸载

在隔离 ARM64 上安装正式 Core、Deployment 和 Cloud。实装处理器安装 51001 后，Felix 登记 11 个 bundle，7 个 ACTIVE、4 个 fragment RESOLVED。Main 停启后沿原编号读回 COMPLETED，存档、提交标记、bundle ID 和驱动工厂均已核对。卸载 51002 完成，Felix 和 Kura 注册表均已移除该包，系统正常停放。详见 `opcua-deployment-main/HANDOFF.md`。同名部署包跨版及其他 provider/launcher 的维护组合继续待验，冻结 0/12。

## 2026-11-22 OPC UA 正式 Main 的节点读取与恢复

ARM64 实装修复 Milo 1.1.2 重定位依赖缺少 OSGi 声明，以及通道省略 attribute 时默认值未生效。WIP `107da42` 通过 10 项模块测试。原 Main 的同一 PreparedRead 在服务器停止时返回 ConnectionException，服务重启后读到新的时间。remove、解包和续配后，新 Main 用原工厂和同一份快照再次读取产品名与时间，随后 purge、正常停放。匿名回环读已验，签名加密、订阅、实际部署包投递及其他 provider/launcher 仍待验。见 `opcua-provider-main/HANDOFF.md`，冻结 0/12。

## 2026-11-22 正式 Camel 路由跨包维护和 provider 缺席时的快照

真实 dpkg 配置后继包时等待注册表锁超时，状态为 half-configured；原 Root 续配后，正式 Main 从同一编号恢复原路由并再次送达。另一 Root 在 Camel 已 purge、配置工厂缺席时由 Main 保存新快照，重装后从该新编号恢复既有路由并送达。每个 Root 的路由只提交一次。两版六个 JAR 相同，详见 `camel-provider-upgrade/HANDOFF.md`。其他 provider/launcher 和跨代换钥继续待验，冻结 0/12。

## 2026-11-22 Core 快照转换中断续办

Core `e99caab526` 修复了历史中已有密文时跳过最新明文的问题。ARM64 JVM 在编号 1 发布后被 SIGTERM 终止；旧发行 JAR 在同一目录返回 null，修复版保留编号 1 的完整指纹并恢复编号 2。新的 ARM64 deb 在独立内存系统通过正式 Main 读回运行与快照值 12002。模块及跨模块合计 389 项通过。详见 `snapshot-conversion-resume/HANDOFF.md`。50001 本轮仅冷读；跨代换钥及其他安装、恢复组合仍待验，冻结 0/12。

## 2026-11-22 Artemis XML 用户表快照与冷启动

原包关闭整份快照加密，0600 XML 中可直接匹配测试口令。临时开启整份快照加密后，新快照为密文。两种设置下均经正常关机、按原编号读回和 MQTT QoS 1 收发验证。开启加密时 Core 自动转换了原夹具 snapshot_0；已从同 Root、同摘要副本恢复字节、权限和 mtime，inode 与 ctime 已改变。当前停机读回为 `artemis-xml-snapshot/encrypted/cold.py`，证据见 `artemis-xml-snapshot/HANDOFF.md`。跨代换钥与历史转换的中断恢复仍待验，冻结 0/12。

## 2026-11-22 Artemis XML 配置热更新

在 50001 的 ARM64 卷复用既有发行包，正式 Main PID 1381 经 XML provider 在回环端口 22883 收发认证 MQTT，更新 brokerXml 后原端口关闭、22884 收发第二条消息；两次均在订阅端核验 QoS 1 和载荷。恢复原配置，两端口关闭，remove、purge 后正常停机。该卷的当前停机入口为 `artemis-xml-main/cold.py`，记录见同目录 HANDOFF.md。XML 用户表的持久化与跨代密钥处理仍列入 provider 盘点，冻结 0/12。

## 2026-11-22 正式 Main 上的 Artemis Simple MQTT

复用 50001 的 ARM64 卷和已有发行包。Artemis 七个 JAR 与包配对，正式 Main PID 6448 通过两个客户端完成两轮认证 MQTT 3.1.1 QoS 1 收发，中间停启 Simple MQTT bundle，端口随之关闭、重开。恢复原配置后端口关闭，实际 remove、purge 后正常停机。原业务目录、快照、凭据、维护回执已冷读。详见工作空间 `artemis-provider-main/HANDOFF.md`，该目录的 `cold.py` 是 50001 的当前冷读入口。XML provider 的自定义配置及其业务行为仍待实装验证，12 父项冻结 0/12。

## 2026-11-22 Core 移除见证已写入、保留目录尚未发布时的原位续办

Core `fb9b6760a4` 修复 prerm 在写完移除见证后重新分配暂存目录的问题。新钩子从原移除围栏定位见证，逐项核对 Root、原安装目录、暂存目录和移除记录；有部署历史时还校验签名及私有保管目录。发布采用禁止替换目标的改名，随后核对正式路径并同步父目录。ARM64 定向测试通过 79 项，涵盖内容相同但 inode 不同的替代目录，以及维护编号变更时的拒绝。

在独立 Debian ARM64 系统的正式 Main 上，业务包安装任务 `50001` 只提交一次。第一次故障注入因夹具命令位于 noexec 的 /run 而未命中，dpkg 使用系统 mv 并正常移除。这笔对照维护 `32e3fa84-b3f5-440e-9e3e-f906d1a0681b` 已按原围栏完成保留重装，单独归档。

随后同一 Root `39:8063827` 发起独立移除，编号 `000bc1d0-d35f-4572-94de-0d85f0e25a30`。在已写完见证、即将改名时，夹具对该笔 prerm 发送 SIGTERM。dpkg 记录子进程因信号终止，执行 abort-remove，Core 状态为 `deinstall ok installed`。正常关机后冷读确认：正式保留路径尚未出现，原暂存目录为 `39:8071960`，业务目录为 `39:8068535`，见证、暂存副本、原业务文件和两份凭据均可按原指纹核对。

原 Root 重启后，沿此编号续办，原暂存目录直接成为正式保留目录，设备号和 inode、移除记录及暂存业务副本均一致。沿同一笔移除围栏重新安装 Core，再装回同版本 Deployment。只读观察组件经正式 Main 得到 `GET install/result 50001 COMPLETED`，Felix 查到 `fixture.history.staged`。最终 Core、Deployment 均 installed，Main disabled/inactive，系统退出 0；停机读回四个原业务文件和两份凭据的身份、摘要与时间戳。

本项验证了移除见证已落盘至正式目录发布之间的进程中断。全套 provider、launcher、typed broker 与六组件联合生命周期仍有内部缺项。12 父项冻结闭环 0/12，外部延期仍限真实 NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。 [实装检查点](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-22 Core 原生 purge 遇到业务包挂载时拒绝，停机后沿原维护编号续办

独立 Debian ARM64 系统复用 Core `8894de2ca2` 和 Deployment `1b043a4` 的发行包。正式 Main 受理新任务 `49001` 一次，原 handler 返回 `GET install/result 49001 COMPLETED`。Root 为 `39:8041092`。停止 Main，依次移除 Deployment、Core。Core 进入 `deinstall ok config-files`，原业务目录由维护编号 `4e937f6d-923f-479c-bfb8-40c0631035ff` 保管。

在单次 dpkg 进程的私有挂载命名空间内，把已保管的业务 DP 自绑定为只读挂载。已安装的 postrm 识别到该挂载并返回 1，dpkg 状态为 `purge ok config-files`，审计输出为空。正常关闭系统后，独立只读挂载核对原目录及其四个文件、常规备份、移除记录、两份凭据和三份密钥服务文件，身份与内容均符合移除时基线。

再次启动同一容器和原系统卷，核实挂载已解除、维护编号及全部指纹仍匹配，再执行一次原 purge 请求。dpkg 返回 0，Core 状态为 `unknown ok not-installed`，Deployment 为 `config-files`。原保管目录、保留副本和 Core 的 postrm 已清除。密钥服务仍保存同一笔 remove 围栏、日志密钥和前代包历史；两份凭据亦保持原 inode、摘要和时间戳。系统第二次正常退出 0，最终状态已冷查。

本次覆盖从 config-files 发起 purge、postrm 拒绝、正常停机和原 Root 续办。业务任务的完成回执在 purge 前核对；成功 purge 按包语义清除该业务目录。此项不覆盖删除目录进行到一半时的进程中断。原 48001 现场仍按其保留重装交接冷查。冻结 12 父项、闭环 0/12，外部延期仍限真实 NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。 [实装检查点](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-22 Core 移除后保留重装，48001 沿原目录读回 COMPLETED

沿上一轮的 Debian ARM64 原卷和容器续办，Root 为 `39:8007036`。先装入 Core `8894de2ca2`，用发行包核对四个实装 dpkg 钩子。因 Debian 包依赖先移除 Deployment，然后移除 Core。dpkg 返回成功，Core 进入 `config-files`。移除维护编号为 `753e06e8-3932-4a1d-ba7b-39ec7f7f5f95`。原 packages 目录在 Core 安装文件系统的私有目录里保持 `39:8011751`，常规保留副本为 `39:8037268`。业务 DP、注册表、锁和完成回执的文件身份、内容与时间戳逐一匹配。

沿这一笔移除围栏执行原生 unpack 和 configure，Core 依次为 unpacked、installed。配置恢复了原 packages 目录，随后重新安装原 Deployment `1b043a4`。显式启动正式 Main 后，只读观察组件从已注册的 handler 得到 `GET install/result 48001 COMPLETED`，并从 Felix 查到原业务包 `fixture.history.sealed`。48001 仍只有首次安装的一次派发。

Main 最后为 disabled/inactive，系统正常退出 0。冷查确认两包均为 installed、原维护编号已进入历史、围栏和私有保管目录已清除。四个业务文件及两份凭据的摘要、inode、属主、权限、链接数及时间戳均与上一轮升级前的基线吻合。

ARM64 上 78 项定向测试通过，涵盖交换目录后注入中断、按原维护号续办，以及 purge 时拒绝替换目录。本现场执行了正常移除和保留重装。原生 dpkg 在上述交接点中断后的恢复、带完成业务回执的原生 purge 和非标准注册表布局仍待验证。停放的 48001 保留供原编号只读查询。原 7fd 维持正常停机和 41 输入／16 回执。12 父项冻结闭环 0/12。外部延期仍限真实 NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。 [实装检查点](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-22 Core 原位升级后，正式 Main 沿原目录读回已完成的部署任务

在 Debian ARM64 的正式 Main 上，安装任务 `48001` 只提交一次。原生 dpkg 将 Core 从 `ac9b8bb` 升到 `76f0c6b`，Deployment 为 `1b043a4`。解包时，Root 为 `39:8007036`，原 packages 目录为 `39:8011751`，暂存副本为 `39:8016294`。配置完成后，正式 packages 路径仍指向原目录。显式启动新 Main 后，原 handler 返回 `GET install/result 48001 COMPLETED`，Felix 查到原业务包 `fixture.history.sealed`。

云端部署回执绑定注册表目录的路径、设备号和 inode。此前的升级代码复制目录内容后清退原目录。新版本用已有签名维护记录绑定原目录，在同一 Core home 内原子交换目录；历史中的同名文件优先，新增的发行文件先校验再补入。ARM64 上 52 个定向测试通过，其中一项在实际目录交换后注入异常，验证沿原维护记录续办和目录被替换时拒绝继续。本次原生 dpkg 实装走正常升级路径，云端回调由本机夹具接收。

Main 已正常停止，保持 disabled/inactive，容器退出 0。只读挂载冷查确认业务 DP、完成回执、注册表、锁及两份凭据的内容和文件身份与升级前一致。下一处缺口是有完成回执时 Core 移除后保留重装能否沿原目录查回同一任务。非标准目录布局也尚待实装验证。冻结 12 父项、闭环 0/12，外部延期仍限真实 NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。 [实装检查点](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-22 未决安装回执下的 Core 移除、直接 purge 与 dpkg 选择状态

独立 Debian ARM64 系统的 Root 为 `39:7997146`，实装 Core `796e0ba` 与 Deployment `1b043a4`。正式 Main 接受一次 47001 并完成业务包安装。独立的 47002 也只提交一次。在 Felix 同步发出其目标 bundle 的 INSTALLED 事件时，测试探针核对原 job 回执并中断 JVM。重启后原安装服务读回 47001 COMPLETED、47002 UNKNOWN，后者的私有暂存 DP 和原未决回执保留。

正常停止 Main，移除 Deployment，消除了 Debian 对 Core 的包依赖。分别执行原生 `dpkg --remove kura-core` 和 `dpkg --purge kura-core`，二者都进入已安装 Core 的 prerm，因原部署发布未决返回 1，并调用 postinst abort-remove。Core 实装状态仍为 installed，没有生成 /var/lib/kura-retained，dpkg 审计为空。两次失败后的完整选择状态分别为 `deinstall ok installed` 和 `purge ok installed`。

沿原 Root 重装同一 Deployment，正式 Main 按原编号再次读回 47001 COMPLETED、47002 UNKNOWN。重装 addon 后 Core 的期望状态仍是 purge。取消本夹具的测试移除请求，将 Core 的选择恢复为 install。19 个 Core 配置历史文件的 inode、mtime、摘要逐一相同。Main 随后保持 disabled/inactive，容器正常退出 0。冷读验证 dpkg 选择及安装状态均为 install installed，已完成业务归档、未决输入副本、两份回执、注册表、凭据等 12 个文件的身份、时间戳和内容均与维护前相同。

本轮证明的是已安装 Core 在安装类业务回执未决时的拒绝路径；两次调用都在 prerm 返回，未执行成功删除。下一步应检查只有已完成业务回执时，Core 维护成功后能否保留回执绑定的注册表目录身份并读回原结果。该问题应独立核对；本现场保持停放。原六组件、绑定网关及历史未决分别从各自原入口只读核对，7fd 保持停机、41 输入／16 回执。冻结 12 父项、闭环 0/12。外部延期仅原三项。 [实装检查点](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-22 原生 dpkg 在云端卸载未决时升级、移除、purge 和恢复 Deployment

在独立的 Debian ARM64 systemd 安装上，Core `796e0ba` 和 Deployment `c77e34a` 的正式 Main 接受业务包安装 46001，随后只接收一次卸载 46002。Felix 同步发出该业务 bundle 的 UNINSTALLED 事件时，探针记录原回执摘要并终止 JVM。旧 handler 在同一 Root 读回 46002 UNKNOWN。失去完成回执的卸载保持未决。Root 为 `39:7987236`。

在 Main 停止后，dpkg 实际尝试重新解包同版本 Core，旧 prerm 和新 preinst 均报告必须先处理原发布回执，dpkg 返回 1，Core 仍为 installed。Deployment 的后继包 `1b043a4` 解包成功。保持 unpacked 状态正常关闭网关，冷读确认原业务回执和原 replace 围栏。原 Root 重启后，生成器仍将 Main 置于 masked-runtime，沿原围栏配置一次，随后 dpkg 审计为空。真实 remove 进入 config-files，purge 进入 not-installed，启动顺序中的 deployment 行已撤去。沿同一 Root 重新安装后继包，恢复一行注册和四个发行 JAR。

再次显式启动正式 Main，后继 handler 按原来源和编号读回 `GET uninstall 46002 UNKNOWN`，新旧 handler 的类摘要分别匹配各自 deb。Felix 中已查不到目标，磁盘业务归档和注册项均保留。各阶段核验原卸载回执、DP、注册表、锁、两个凭据文件和三份派发／中断标记，共 9 个文件的摘要、inode、权限、链接数及时间戳。最后 Main 为 disabled/inactive，容器正常退出 0；最终包状态及上述文件已只读核对。

本轮覆盖业务卸载回执未决时的同版本 Core 拒绝和 Deployment 生命周期。未执行 Core 移除或 purge，也未在原生 dpkg 上验证安装类未决回执。46002 保留原编号，后续仅停机读回。六组件、原绑定网关、42002、44002 和 45001 各自在原现场核对。原 7fd 保持 disabled、停机、41 输入／16 回执。冻结 12 父项、闭环 0/12。外部延期仍限真实 NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。 [实装检查点](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-22 云端安装 45001 在归档发布后中断，原 Main 安装现场重启恢复

隔离 ARM64 网关的正式 Main 通过已注册的云端 RequestHandler 接受一次安装，编号为 `45001`。回执已持久化为 `phase=returned` 后，测试夹具在同一工作线程发布归档、同步目录、即将替换注册表时暂停 JVM。归档摘要与提交的 DP 相同，当前注册表与受理时的 inode 和摘要相符，待发布注册表已包含新包。用该 JVM 的 UID 中断进程，退出码为 137。中断后原容器保持运行；JVM 已退出。实装 Core 的 `--check-stopped-writers` 在这时返回 1，指出必须先处理原发布回执。执行前后的磁盘指纹相同。

沿原容器、文件系统 Root 和数据卷 `39:7983752` 重启正式 Main，按原编号读取 `GET install/result`，结果为 `COMPLETED`。原 pending 回执的 inode、摘要和 mtime 与最终完成回执相同，原暂存注册表也原样成为正式注册表。发布过的归档和四个出厂归档指纹一致。启动日志记录本地 DeploymentAgent 按新注册表装载业务 DP，Felix 读回 `fixture.returned.pair 1.0.0`。云端 EXEC 只提交一次。Main 收到 TERM 后停止，容器正常退出 0，原卷已只读复核。

本次验证了已落盘的成对发布回执及 Core 停机写入检查。带未决业务回执运行原生 dpkg 的升级、移除、purge 及相应维护恢复仍待实测。六组件停机入口继续使用 six-deployment-receipt-upgrade/cold.py，绑定网关继续使用 bound-agent-packaged-receipt-field/cold-business.py。先前的 42002 和 44002 分属各自原 Root，本轮仅停机核对。原三现场、原运行进程及 7fd 的 41 输入／16 回执已核对。冻结闭环 0/12。external deferred 仅真实 NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。 [实装检查点](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-22 原六组件 ARM64 网关完成 Deployment 升级，正式 Main 读回原业务包

原 Root `39:6611544` 从 Deployment `de17bd5` 升至 `1b043a4`。本次直接使用上轮已核对源码的 deb。正式 dpkg 解包后，Main 处于 masked-runtime、inactive，Deployment 的 replace 围栏和生成器硬链接对应 inode 7978837。沿该围栏配置一次后，dpkg 审计为空，Main 和 agent 恢复 disabled、inactive。其余五个组件版本和 Core 的 46 笔原维护编号均与升级前一致。

显式启动一次正式 Main，探针通过 OSGi 服务读回 Felix 中的原业务包、活动 bundle 及其资源 `retained application resource`。本地和云端两个发行 bundle 各有三份类字节与 deb 相符。原业务 DP、注册表及启动顺序均已核对。随后正常停止 Main 并关闭原容器，退出码为 0。冷读核对 15 个实装文件、原 82 个保护文件、两个凭据文件指纹及 319 项可变文件。319 项的内容、身份和修改时间均与升级前相符，原 46 笔 Core 历史的 835 个阶段文件通过签名校验。

后续从本目录的 cold.py 复核六组件。原容器将 /usr 从自身可写层绑定到系统 Root，停机卷单独读取不到其中的四个文件。冷读器已按原容器完整 ID 读这四项，其余 11 项取自原卷；修正后在容器停机状态完成核对。

本轮六组件现场没有待处理的云端安装或卸载。仍待验证真实待处理回执存在时 dpkg 的维护准入与原回执保留，以及成对发布回执已落盘时的中断恢复。42002 和 44002 分属两处已停放的 Root，继续按各自原编号只读续查。绑定网关此次只读核对，6910532 的五笔维护、三代来源、原任务 SUCCEEDED 和原 OFF 均已确认。原 7fd 停放，41 输入、16 回执。冻结闭环 0/12。外部延期仍限真实 NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。 [实装检查点](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-22 云端安装在 Felix 接单后退出，按原 Root 和编号读回结果

隔离 ARM64 正式 Main（UID 122，CapEff=0，NNP=1）通过本机探针调用实际注册的 RequestHandler。测试卷 `39:7974636` 上，44001 正常安装并取得完成通知及完成回执。独立的 44002 只发一次。同步收到目标业务 bundle 的 INSTALLED 事件后，夹具锁住注册表锁文件。待 Felix 已能查询到目标包，磁盘尚无对应注册项，Linux /proc/locks 又显示同一个 Main PID 正在该 inode 上等待写锁时，夹具令原 JVM 退出 77。在同一挂载中启动另一个 JVM，只读查回 44001 COMPLETED 和 44002 UNKNOWN。Felix 此时查不到后者，四个出厂包和 44001 均可用。停机后原完成回执、原未决回执、待发布的 DP 副本、注册表及五个归档的身份和摘要与中断时相符。读回 JVM 收到 TERM 后退出 143，容器退出 0。正式发行包四个 JAR 与实装逐字节核对，JDK 21 全仓 259 项测试通过，零失败、零跳过。

本次退出点在 Felix 已返回、原工作线程尚未取得发布锁。原编号对应的成对发布回执已经写入后再中断的恢复路径，当前由定向测试验证。本轮使用本机通知接收器，并在同一个容器挂载上重启 Main。原六组件仍装 Deployment de17bd5。下一步沿原维护围栏核对后继 Deployment 包能否在六组件现场升级并保留业务 DP，以及持有安装回执时各维护入口的行为。42002 和 44002 均保留 UNKNOWN，只能沿各自原 Root 和编号读回。绑定网关 6910532 维持停放，五笔维护、三代来源、原任务 SUCCEEDED 和 OFF 已核对。原三现场与原 7fd 已保全，7fd 保持 disabled、正常停机、41 输入和 16 回执。冻结 12 父项、闭环 0/12。external deferred 仅真实 NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。 [实装检查点](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-22 同一 Main 内的云端安装和本地卸载等待注册表发布

Deployment 本地提交 `ea8da2c` 将同一 Main、同一注册表的本地及云端 Felix 变更串行到注册项和归档处理结束。读回和云端通知仍在各自原路径。异步队列中的出厂归档在取得执行权后重新核对注册 URL。本地卸载入队遭拒会释放本次包名，避免后续请求一直显示已在卸载。

在独立 ARM64 发行镜像的正式 Main 中（PID 1、UID 122、有效能力为零、NNP=1），本机探针通过已注册的云端 RequestHandler 提交安装 `43001`。夹具持有注册表的 POSIX 文件锁，等到 Felix 已能查询新包、磁盘注册表仍只有四个原包时，通过独立的 DeploymentAgentService 请求卸载同一个包。线程栈捕获到本地 DeploymentAgent 在 DeploymentPackageRegistry.dispatch 上处于 BLOCKED，此时安装尚未发出完成通知。释放文件锁后，43001 返回 COMPLETED，本地卸载也完成；Felix 包、注册项和归档均消失。原卷 `39:7966465` 正常 TERM 停放，退出码 143，四份出厂归档与发行输入摘要逐份匹配，注册表没有 pending。正式 deb 的四个 JAR 与安装文件逐字节一致，JDK 21 全仓 257 项通过，零失败、零跳过。

此次实测顺序为云端安装先进入 Felix、本地卸载随后到达。反向次序、本地与云端跨进程，以及云端安装在 Felix 接单后进程退出的持久判定均不能从此结果推出。后续先接云端安装的同编号只读判定和对应的实装中断点。原 42002 保持 UNKNOWN，沿 felix-uninstall-receipt/cold.py 只读续接。原绑定 agent 6910532 的五笔维护、三代来源、原任务 SUCCEEDED 和 OFF 已停机冷读。原六组件及原三现场已核对，7fd 维持 disabled、正常停机、41 输入及 16 回执。12 个父项冻结 0/12；external deferred 仅真实 NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。 [实装证据](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-22 Felix 已执行云端卸载后，Main 重启按原编号返回 UNKNOWN

隔离 ARM64 正式 Main 在原测试卷 `39:7959247` 安装了测试 DP（42001）。42002 仅派发一次卸载。在同步收到目标业务 bundle 的 UNINSTALLED 事件时，夹具令 JVM 以 76 退出。沿同一容器挂载、原卷和来源启动另一 JVM，通过实际注册的 RequestHandler 只发 GET，读回 `42002 UNKNOWN`。此时 Felix 查不到原包，磁盘上原归档和注册项仍在；注册项、五份 DP 及未决回执的 inode、权限、mtime 和摘要均与中断时一致。四个出厂 DP 可从 Felix 查询到，日志显示未决包的自动安装被拒绝。两次 Main 均以 UID 122、零有效能力、NNP=1 运行。正式 deb 的四个 JAR 与实装逐字节核对，运行 handler 类匹配。JDK 21 全仓 254 项测试通过，零失败、零跳过。

本项覆盖云端卸载在实际 Felix 事件与应用层完成回执之间中断，以及同一挂载上的 Main 重启。云端安装在 Felix 已接单、应用尚未登记完成时的持久判定，以及跨本地/云端入口的并发派发，仍需单独验证。42002 封存为 UNKNOWN，仅沿原 Root、原编号读取。冷读检查保留卷的原始文件，不在其他 Root 执行业务查询。绑定网关 6910532 的五笔维护、三代来源、原任务 SUCCEEDED 和 OFF 已在停机状态复核。原六组件沿原围栏停放，原 7fd 继续 disabled、正常停机、41 输入及 16 回执。保留 12 父项，冻结闭环 0/12。external deferred 仍仅真实 NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。 [实装证据](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-22 Deployment 云端安装和卸载在原任务结束前保持占用

Deployment `d8ce761` 修正了两个排队问题。旧版云端卸载在提交给执行器后就清除了占用标志；安装入口也会先覆盖共享参数，再检查能否受理。当前 handler 在检查包和调用 hook 前取得本次维护的占用，排队和执行使用本次请求的参数与卸载对象，直到执行和通知返回才释放。下载完成后的自动安装也检查占用。组件停用时取消尚未执行的任务。GET install 在卸载期间保持 IDLE。

在独立 ARM64 发行镜像中，正式 Main 以 UID 122、有效能力为零、NNP=1 运行。通过 /load 提供来自 Cloud 正式 deb 的 base provider，满足 Deployment 对 MQTT DataTransport 的引用。探针在本机调用实际注册的 RequestHandler.doExec，以本地通知接收器在回调返回前保持工作线程。41001 安装真实 Felix 业务 DP，读回包内资源并看到原包名的 IN_PROGRESS；41002 卸载与 41003 安装被拒绝。41004 卸载原 DP，等待通知时 41005 卸载被拒绝。仅 41001、41004 返回 COMPLETED。四个出厂 DP 及注册项恢复，业务 bundle 和已安装归档均已移除。进程正常 TERM 停放，原卷冷读核对四个 DP 的发行摘要。发行包四个 JAR 与容器内的文件逐字节一致，运行中的 handler 类与 deb 内容匹配。JDK 21 全仓 251 项测试通过，零失败、零跳过。

本轮验收的是单个已注册 handler 的并发受理、状态和生命周期。实装中的 41003 使用未暂存的包名，另有定向测试在输入文件已准备好时验证排队中的安装拒绝竞争请求。云传输通知由本地夹具接收；进程重启后对已派发 Felix 操作的持久判定仍待实施，先前的 UNKNOWN 均按原编号封存。六组件原卷的 Deployment 继续为 de17bd5。绑定网关继续停放，五笔维护、三代来源、原任务 SUCCEEDED 与 OFF 已冷读。7fd 保持 disabled、正常停机及 41 输入／16 回执。12 父项冻结 0/12，external deferred 仅真实 NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。 [正式 Main 检查点](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-22 当前 Core 全量验证与 HTTPS keystore provider 的同卷回退、升级

Core `796e0bacf5` 在 Debian ARM64、UID 1000、JDK 21 和 Maven 3.10.1 上离线执行 `-Posgi-it verify`。82 个模块构建成功，2158 项测试通过，其中真实 Equinox 68 项，失败、错误、跳过均为零。包含本版新增的旧式双硬链接恢复用例。与本版正式 deb 对照，41 个 Core 插件的 1,208 个 class 内容相同；Equinox 用到的配置和 keystore JAR 均与该次模块输出逐字节相同。

在同一正式容器 Main 中，瞬时停用 keystore bundle 后，HttpsKeystore 服务注销，回环 HTTPS 拒绝连接。重新启动同一 bundle，服务编号从 84 变为 248，Main 的 PID 与启动时间保持。重新取得的服务用 localhost 私钥签署随机挑战，原证书验签成功，新的 HTTPS 连接出示同一证书并返回 200。

专属卷 Root `39:7909345` 随后通过两个完整 ARM64 发行镜像从 796e0ba 回退至 ac9b8bb，再升回 796e0ba。两版均从真实服务验证签名，并通过回环 HTTPS 核对证书指纹。各版运行中的 bundle 实际返回的 PendingStorePassword.class 与本版镜像对应的正式 provider JAR 匹配，两个 class 的摘要不同。三份原快照、HTTPS 密钥库和两份身份文件的 inode、内容、权限、属主、大小和 mtime 在各阶段一致。各 Main 为 UID 122，NNP=1，有效能力为零，全部正常 TERM 停机。 原绑定网关及六组件停放，冻结 0/12。[对应运行记录](actual-arm64-docker-nonroot-launcher-checkpoint.json)

## 2026-11-22 ARM64 Docker 发行输入与启动器分层

Docker 896b976 在 Maven 复用同一工作目录时按所选输入发布。旧版和当前 Core 双向切换后的 deb、provider JAR、启动器与各自来源配对；带遗留 DP、禁用 DP 和缺失指定 DP 三种边界已验证。固定包只换启动器时 395,674,305 字节的安装层命中缓存。四包 235 个 JAR 与发行输入配对。原绑定 agent 的 6910532 升级沿既有编号完成；本轮停机冷读核对五笔维护、三代来源、SUCCEEDED 和 OFF。完整父项冻结 0/12。[Docker 实装检查点](actual-arm64-docker-nonroot-launcher-checkpoint.json)

## 2026-11-22 旧版 Docker 的 PKCS12 双硬链接恢复

旧版 Core d97260d035 的正式 Main 在独立 ARM64 卷 39:7855626 上写入随机口令配置时中断，留下两条同 inode、0600 的加密记录。原容器普通启动退出 1，停机核对五份原快照、业务库和密钥均保持。

维护镜像使用 Core 796e0bacf5 和 Docker 393b089。显式维护入口取得原实例锁，子进程以 kurad 身份核对配置路径、实际库的 inode 和内容指纹、业务 PID 及密文摘要。改用其他 Root、PID 或摘要均被拒绝。原输入成功拆分两个名称并同步目录，两份密文仍与原记录相同；同输入再次执行，两份记录的 inode 都保持。普通启动器原有单链接检查继续生效。

随后显式启动原容器和原 Main。原业务钥签名通过原证书验证，第六份快照记录已恢复的口令，待办记录均已退出。原业务库 inode 7856657 与中断时相同，最终正常 TERM 停机。另在独立 ARM64/JDK 21 进程用同一份已打包 provider JAR，在维护临时文件同步完、改名前以及改名后、目录同步前分别终止 JVM，沿同一组输入完成恢复。该试验为进程终止。ARM64 keystore 单元测试 97 项、容器启动器测试 28 项通过。四个包中的 235 个镜像插件 JAR 与发行输入配对。

本次覆盖旧版两条完整名称、同一私有 inode 的改密记录。单一名称、额外硬链接或其他未配对布局继续保留原门禁。原六组件 d97260d035、原绑定网关 6910532、原 7fd 均停放。12 父项冻结闭环 0/12，外部延期仍限真实 NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。 [实装检查点](actual-arm64-docker-nonroot-launcher-checkpoint.json)

## 2026-11-21 当前 Core 在 ARM64 上的完整 OSGi 验证

Core 原发行源码 ac9b8bb80c 的首次全量验证在跨模块配置测试中出现 5 项失败。测试仍期望旧式双流加密调用。Core `37613e040c` 仅修订测试夹具和原有的 6 处调用断言，沿用 XML 对照、快照编号、文件权限和非法目标拒绝检查。该测试类 82 项定向通过，随后在隔离 Debian ARM64 上以 UID 1000、Maven 3.10.1、JDK 21.0.12.1 离线完成 `-Posgi-it verify`。82 个模块构建成功，2157 项通过，含 68 项真实 Equinox 集成测试，失败、错误、跳过均为零。上轮完整构建跳过的两个 SystemService 路径用例本次使用专属 tmpfs 执行。明文和加密快照写失败、keystore 第二个记录名发布失败的断言均通过。

Equinox 测试目录中的配置和 keystore JAR 分别与本次模块产物逐字节相同。与当前 ARM64 deb 对照，两组件的 122 个 class 全部相同，各归档的成员内容差异只有清单中的 Bundle-Version 时间后缀。另将 deb 插件目录中全部 41 个 org.eclipse.kura.* JAR 与同名模块产物比较，1,207 个 .class 文件逐个匹配，归档内的其他文件内容也只在清单版本后缀上有差异。第三方插件另计。旧双硬链接恢复记录在既有容器的正式启动器上仍有准入问题，六组件及各 provider/launcher 的其余安装维护组合继续按原回执验收。

原绑定网关保持 6910532，停机冷读沿同一笔升级编号 7a1e4339-aaad-4ee0-94fc-8db58592881f 确认三代 source-pairs、五笔维护及原任务 SUCCEEDED、远程 OFF。六组件原 Root 39:6611544 仍装 d97260d035，冷读 46 笔维护和 835 条签名。7fd 保持正常停机，历史 41 输入和 16 回执。冻结闭环 0/12，外部延期仍为原三项。 [构建与逐包对照](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-21 正式容器 Main 在随机改密快照写入中断后恢复原业务钥

Core ac9b8bb80c 把同一份加密恢复记录分别同步并原子发布到前后两个指纹名称。两个名称的 inode 独立，权限 0600。原 PID、配置路径、实际路径、文件身份和内容指纹继续配对。旧版使用硬链接，在旧配置和 store.save 均打不开已换口令的 PKCS12 时，实装容器启动器的状态检查会拒绝启动。新版有 96 项 keystore 测试通过，和上一版 Core deb 对照，仅 keystore provider JAR 变化。新镜像四包的 235 个插件 JAR 与 deb 配对。

在独立 ARM64 测试卷 39:7766080，正式入口三次启动同一容器的 Java PID 1，使用同一业务 PID 和证书。首次用正式工厂创建 RSA 业务钥，注册随机改密，读回配置快照并正常停机。第二次启动，测试 OSGi 观察器在真实配置写线程同时经过 FilesystemKeystoreServiceImpl 和 ConfigurationServiceImpl.writeSnapshot、出现暂存文件时使测试 JVM 退出 76。停机卷上的五份原快照保持，暂存文件 0 字节，两份恢复记录各自只有一个链接，后继业务库 inode 为 7767200。第三次沿原卷启动，Main 生成第六份正式快照。快照口令与 store.save 一致，与此前快照口令不同；可打开原业务库，原钥签名通过原证书验签。已发布 PKCS12 的内容和 inode 保持，两份恢复记录均退出。测试卷正常 TERM 停放，退出 143；中断产生的空暂存文件仍在且未参与快照枚举。

旧双硬链接记录另经新版 provider 在独立同 Root 进程读回、恢复和验签，换 PID、同字节换 inode 均拒绝。已有双链接未决的旧容器在正式启动器这一关仍会停下，本轮未实现旧容器自动迁移。中断实测发生在文件已创建、尚未写入内容的时点。该试验使用新合成业务钥，测试卷独立于原 Docker 业务卷和原六组件卷。

当前测试卷停机读回 pkcs12-container-resume/joint.py parked。原 Docker 业务卷继续沿 docker-launcher-config-publication/runtime.py cold。原六组件仍为 d97260d035，沿 snapshot-publication/cold.py 读回 46 笔历史、835 条签名。绑定网关仍为 6910532，五笔维护封存、原任务 SUCCEEDED、开关 OFF。单包移除号仍只沿 pkcs12-retained-root-readback/check.py cold。12 个父项冻结闭环 0/12，外部延期仍限原三项。 [实装检查点](actual-arm64-docker-nonroot-launcher-checkpoint.json)

## 2026-11-21 Alpine 启动器在 Equinox 配置不完整时停止

旧容器启动器在模板已存在但读失败时仍发布了 18,816 字节的 bundle 列表。Docker a71f3cf 改为先复制模板，再让已安装的生成器追加 bundle 列表；文件同步成功才改名，目录同步成功才继续启动。27 项容器回归通过。旧版和当期 Core 的隔离 ARM64 夹具都验证了未改名时文件同步报错保留旧配置，以及改名后目录同步报错时完整配置已可见。撤去故障后正常发布得到相同摘要。两处 EIO 由独立 Python 驱动定向注入。

四份当前发行包与镜像输入配对，235 个插件或 sibling JAR 逐一匹配。原卷 Root 39:6566861 在本轮同一容器中经历拒绝和恢复。故障只设在该容器可写层的模板，保留原模板，正式启动入口退出 1 后冷读不到运行配置。原卷复核后启动同一容器，入口夹具恢复经摘要配对的模板并执行正式入口。PID 1 为 UID 122 的 Java，有效能力为零、NNP=1，HTTPS /admin/auth 返回 200。四个原业务 bundle 为 ACTIVE，原四个部署包已登记。Java 实用的 21,004 字节配置与单独生成结果逐字节一致，属主 122:122，权限 0600。正常 TERM 停放退出 143，旧容器保持原停放状态。停机冷读原卷的三份快照、四份身份和原业务资料，并核对新容器层的模板、启动器和运行配置摘要。

最新 Docker 原卷冷读入口为 docker-launcher-config-publication/runtime.py cold。原六组件继续沿 snapshot-publication/cold.py。原绑定网关五笔维护和三代来源已冷读，原任务 SUCCEEDED、开关 OFF。单包移除围栏继续沿 pkcs12-retained-root-readback/check.py cold。正式 Main 随机改密与快照联合中断仍待验。本轮保留 12 父项，冻结闭环 0/12，外部延期仅原三项。 [实装检查点](actual-arm64-docker-nonroot-launcher-checkpoint.json)

## 2026-11-21 Core 配置快照完整写入后原子发布

Core d97260d035 将快照写入同目录私有暂存文件。加密模式直接向 AES 流写 XML。完成认证标签和文件同步后原子发布，在 POSIX 文件系统同步父目录。ARM64、JDK 21 上分别中断明文和加密写入，覆盖已有编号和创建新编号共四种场景。中断前后独立读取进程都能用原快照口令签名，完成发布的后继快照也能签名。配置组件 304 项、离线恢复工具 74 项测试通过。

原六组件 Root 39:6611544 经维护号 9ff19a93-2f26-46c6-9039-0bcaf8f3c282 升级。正式 Main 在 9 份原快照、保留上限 10 的现场通过注册的 ConfigurationService 写入 1795273664995，读回口令打开原业务 PKCS12，并以原证书验签。Main 停机后，仅撤下本轮快照和探针，9 份原快照、原业务库和 store.save 的指纹保持。六组件正常停机，46 笔维护、835 条签名阶段通过冷读。

最新六组件停机读回使用 snapshot-publication/cold.py。单包移除仍沿 pkcs12-retained-root-readback/check.py cold，绑定网关沿 bound-agent-packaged-receipt-field/cold-business.py。独立写进程在发布前被终止时，可留下权限受限且不被快照枚举的暂存文件。正式 Main 本轮验证正常写读；业务库随机改密、其配置快照写入与 Main 进程中断连为同一笔的恢复仍待验。保留 12 父项，冻结闭环 0/12，外部延期仅原三项。 [实装检查点](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-21 单包 Core 移除后的 Root 配对核验

原单包 ARM64 夹具的移除围栏与 removal-home.json 均指向 5201d2d4-df9f-4387-a82b-787e7e194c8b。启动原容器后，machine-id 摘要及 Root inode 7724412 匹配，设备号从 1048716 变为 2097163。核验在包操作前结束，容器正常关机，退出 0。停机前后的两次冷读均确认包状态为 config-files、原移除围栏仍在、保留的六个文件逐一匹配移除时的 SHA-256。

这个编号的保留重装和重装后验签仍待原 Root 配对成立。当前单包停机只读入口为 pkcs12-retained-root-readback/check.py cold。原六组件继续使用 pkcs12-package-admission/cold.py。绑定网关继续使用 bound-agent-packaged-receipt-field/cold-business.py，其 6910532 后继升级已封存，原任务 SUCCEEDED、开关 OFF。冻结闭环 0/12，外部延期维持原三项。 [检查点](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-21 Core 包维护在复制前保留原 PKCS12 改密文件

Core c5381c2e55 已在原六组件 ARM64 Root 39:6611544 实装。PKCS12 已经替换、store.save 尚未更新时，真实 dpkg 升级在复制前拒绝。原六包保持 installed，正常停机后核实原文件身份，重启后在原测试库恢复同一随机口令并验签。沿旧 prerm 已建立的维护号 4fede114-d501-4969-b465-f6b8c1e67749 完成后继升级。独立 Core 单包容器中，真实 dpkg 移除在建立围栏前拒绝；恢复后正常移除，config-files 与保留区 6 份文件已停机冷读。

85 项 ARM64 定向回归通过；另有两项夹具核实围栏建立后才出现恢复记录时仍拒绝复制并保留原编号。六组件最新冷读为 pkcs12-package-admission/cold.py，45 笔维护、816 条签名阶段。新单包现场沿 pkcs12-package-admission/core-only/cold.py。正式配置快照与维护并发、包装钥跨代继续内部 pending。冻结 0/12，外部延期维持原三项。[包维护和冷读检查点](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-21 PKCS12 随机改密中断后沿原文件恢复

Core 9450cd883f 已修复 PKCS12 替换后、随机口令尚未写入 store.save 即退出可能导致的原钥不可读。ARM64 独立临时库验证替换前、口令写入前后、配置确认后退出，以及配置起初已过期时的连续两次进程中断。恢复沿原文件与 PID，同字节不同 inode 和其他 PID 在两份常规口令均不可用时均被拒绝。配置回调使用测试接收器，正式 Main 另行验证原注册服务、实装类和原业务私钥签名。95 项 keystore 测试通过。

原六组件已升级、正常停机，44 笔维护的 797 条签名阶段通过冷读。最新入口为 pkcs12-random-recovery/cold.py。在未决改密期间进行 Core 包维护，以及真实配置快照跨中断和包装钥跨代，继续内部 pending。冻结闭环 0/12，外部延期仍限原三项。[发行与实装检查点](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-21 三个实装启动入口处理配置同步失败

Core 09691dad70 已在原六组件 Root 39:6611544 升级。三个实装启动器使用正式凭据，在私有临时单元的 fsync 返回 EIO 时均退出 1，保留原配置和 writer 状态。恢复正常同步后，正式 Main 以 UID 997 运行，生成的配置与独立发布器摘要一致，HTTP /admin/auth 为 200。15 项 ARM64 启动回归通过。目录同步失败后已改名的新文件和下一次启动的处理另有定向测试。

Core 升级后，原加载顺序保持，锁由正式注册表检查重新建立。最终六包 installed，Main 和 agent 均 disabled、inactive，正常停机后核对 43 笔维护、778 条签名阶段、319 份运行文件和 101 个夹具对象。最新只读入口为 launcher-writeback/cold.py。冻结闭环 0/12，外部延期维持原三项。[实装与启动检查点](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-21 当前 Core 实装与 ARM64 全量 OSGi 基线核对

上次完整构建 `ba84bdcfb4` 的 ARM64/JDK 21 日志摘要与原回执一致，82 模块、2151 项通过、2 项跳过。当前实装 Core `1af242d3e8` 相对该版的 51 个 Git 路径改动全部位于 kura/distrib。两个正式 ARM64 deb 的 233 个 JAR 路径与 SHA-256 完全相同，原六组件 Root `39:6611544` 停机只读挂载后，dpkg 登记的 233 个 Core JAR 也逐一匹配当前 deb。

对 123 个发行载荷差异和六份 dpkg 控制文件逐项冷读。118 项原字节匹配。三个启动脚本只有安装时堆参数不同；八份安装暂存输入已按 postinst 的清理步骤退出，其七份对应 systemd 单元在同一停机容器的 /usr 层核对通过。安装脚本本身随暂存目录撤下。

在新的断网 ARM64 容器按模块运行本次变更的 15 个发行测试模块，共 189 项通过。cohort 测试首次通过通用发现入口运行时缺少仓库参数，22 项初始化失败；改用源码规定的 --repository 入口后 22 项全部通过，计数只取此次结果。上次全量构建跳过的两个 SystemService 方法已在原脚本要求的 /opt/eclipse/kura 专用 tmpfs 内，用当前源码、Maven 3.10.1 和 JDK 21 单独执行，2 项通过、0 跳过。历史全量与本次定向结果分别保留。

六组件一直停放，未触发包操作或 Main 启动。原绑定网关仍为 SUCCEEDED、OFF；7fd 保持正常停机、41 输入及 16 回执。12 个父项冻结闭环 0/12，外部延期仍限原三项。 [构建、发行差异与实装配对](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-21 Cloud 配置发布中断后复用同一暂存文件

ARM64 旧发行包 `e683e71` 在应用配置文件已同步、尚未发布时退出，即使续办成功，应用用户目录仍留有一份完整副本。Cloud `d969371` 按原文件代次和本次目标内容识别暂存文件。续办验证已写入的字节，只补余下的部分。已有文件和首次创建各测发布前、发布后退出，修订版四种情况均完成，暂存副本为零。内容冲突时保留现场；确认属于本次写入的短文件可沿原 inode 续完。独立 Debian ARM64 上的八种站点设置输入也已通过。

原六组件 Root `39:6611544` 进行了两次停机续办。第一次尝试截停 dpkg 子进程时，测试程序所在 /run 挂载为 noexec，配置正常完成，只记普通安装。后续用实际目录锁使正常 dpkg 钩子分别留下 half-installed 和 half-configured。在各自中断状态下，以 kurad 单独运行取自实装 postrm 或 helper、经 SHA-256 配对的 Python 程序，并在指定行使进程退出。该工作进程的定点退出由测试驱动执行。两次重启后的续办均使用正式 dpkg 命令。

移除时 false 已发布、目录同步尚未执行。停机读回无暂存文件，续办后配置 inode 7680583、内容及完整指纹均保持。安装时 true 已写入暂存 inode 7681023，正式配置仍为 false，Cloud 尚未登记。停机重启后同一 inode 成为正式配置，八个业务 JAR 与旧版相同。最终正常 purge 并复原原配置。六包 installed，Main 与 agent disabled、inactive，dpkg 审计清洁，停机退出 0。319 份运行文件仅属性文件 mtime 改变，42 笔维护的 759 份签名、101 个夹具对象和原凭据已读回。冻结闭环 0/12，外部延期仍限原三项。 [发行包及两次停机证据](actual-passive-addon-dpkg-rollback-checkpoint.json)

## 2026-11-21 Cloud 原生卸载撤回云连接能力，原 Root 中断续办与正式菜单已验证

Cloud `e683e71` 修复了原生包卸载后正式 GWT 仍显示 Cloud Connections 的问题。原六组件 ARM64 Root `39:6611544` 从旧版 `1caad20` 升级。新 deb 的八个业务 JAR 均与旧包一致，postrm 内含与安装路径相同的配置更新脚本。维护以 kurad 写入能力值，保留独立站点属性、文件权限和扩展属性。purge 从 config-files 继续时保留卸载后新增的站点设置。

持有原注册表锁时，真实 dpkg 移除 Cloud 载荷后返回失败。正常停机并只读核对，Cloud 为 half-installed，Core 有两个待处理路径，Cloud 注册行、旧启用值和已安装 postrm 均在。重启后从该状态只续办移除，注册行退出，能力值为 false，Cloud 为 config-files。处理原 Core 触发器后审计清洁。随后 purge、重装当前版本、直接 purge，均按预期切换同一设置。正式 Main 中同一管理员可访问 Device 和 System Logs，Cloud Connections 已隐藏。包关闭标志后才进行浏览器验收，夹具原 854 字节配置在验收后复原。

正式包在隔离 ARM64 上验证八种属性文件输入，覆盖空文件、缺文件、续行、CRLF、旧版文件与站点追加配置。独立属性测试用日志输出接收器代替 systemd 日志；原六组件的 dpkg 运行使用真实 systemd。最终六包 installed，Main 和 agent 均 disabled、inactive，正常停机退出 0。原 319 份运行文件仅属性文件 mtime 变化，42 笔维护、759 份签名、101 个夹具对象及两份凭据通过停机核对。冻结闭环 0/12，外部延期仅原三项。 [修复、实装与停机冷读](actual-passive-addon-dpkg-rollback-checkpoint.json)

## 2026-11-21 确认 Cloud 包清除后残留标志会显示并打开正式 GWT 云连接菜单

原 ARM64 六组件 Root `39:6611544` 保持 Cloud 包已清除，直接复用既有卸载回执。以 kurad 在原 854 字节配置后补回此前记录的 34 字节，文件摘要与原 purge 后的摘要完全相同。显式启动正式 Main，Chromium 使用已改密的原夹具管理员登录 /admin/console。Device 正常可用，Cloud Connections 菜单可见，点击后显示云连接页面。两次观察均限制在原回环端点。

停止 Main，核对文件身份、原前缀和完整后缀后撤回这 34 字节。重启同一安装并用同一账号登录，Device 和 System Logs 可用，Cloud Connections 菜单已隐藏。GWT 的 denali.init 读取 SystemService 实际属性，EntryClassUi 同时检查云管理权限和会话中的能力标志。Core 默认值为 false。当前 Cloud deb 的 postrm 只删除注册表行，未在实际卸载时撤销安装写入的 true。修复目标是 Cloud 包的原生移除路径，须覆盖升级、卸载失败续办、保留站点配置和 purge 后重新安装。

本轮两次启动均未创建业务任务或改密。六包最终 installed，Main 和 agent disabled、inactive，正常停机退出 0。319 份运行文件逐项对照，仅原属性文件的修改时间变化。42 笔维护的 759 份签名记录、101 个夹具对象与凭据均由停机卷读回。冻结闭环 0/12，外部延期仍限原三项。 [浏览器和停机核验](actual-passive-addon-dpkg-rollback-checkpoint.json)

## 2026-11-21 八个可选包在原六组件卷联合升级，解包停机后续配并清除

原 ARM64 Root `39:6611544` 使用 16 份已核对 SHA-256 的 deb，依次安装八包前版、解包共同分支八包后继版本。八包为 Artemis、Camel、Cloud、Container、OPC UA、Position、Triton 和 YOFC Runtime。解包后正常停机，八包均为 unpacked，Core 有两个待处理路径触发器。原加载顺序、注册表锁、35 个新版 JAR 和 59 项待修正的包代码权限均已只读核对。

重启后在原 Root 配置八包，八包同时等待 Core；处理触发器后十四包均 installed，35 个 JAR 与发行件逐一匹配，权限清单无差异。随后移除八包，八包均进入 config-files，再执行 purge。最终原六包 installed，八个可选包的载荷目录、维护脚本和 dpkg 登记均已清除。原四行加载顺序恢复，原注册表锁保持，dpkg 审计清洁，Main 和 agent 全程 disabled、inactive，正常停机退出 0。

Cloud 安装在原应用配置末尾增加了 34 字节。原生 remove 和 purge 后 `kura.have.cloud.connection=true` 仍在。夹具复原以 kurad 核对原 854 字节摘要、文件身份和完整后缀后，只撤回这 34 字节。319 份运行文件的内容、权限和属主均与开始时一致，仅该配置文件的 mtime 变化。此次覆盖真实包生命周期，卸载后正式 GWT 的 Cloud 菜单仍待核验。42 笔维护的 759 份签名记录、101 个夹具对象及两份凭据停机读回通过。12 个父项冻结闭环 0/12，外部延期仍为原三项。 [包版本及两次冷读](actual-passive-addon-dpkg-rollback-checkpoint.json)

## 2026-11-21 原六组件在 firewall-only 和完整 Networking 之间切换后续办 Core 触发器

原 ARM64 Root `39:6611544` 使用 Networking 和 firewall-only 的同一构建 `83f237e`，Core 为 `1af242d3e8`。从完整 Networking 切到 firewall-only 后正常停机。停机读回确认 Core 等待处理三个路径触发器，firewall-only 等待 Core。两份 Root 回执与配置尾部逐字节对应，原配置前缀摘要一致。原卷核对了启动脚本，原停机容器的 /usr 层核对了 systemd 单元。重启后只处理本次积压的触发器，15 项包代码权限恢复；启动脚本为 root:root 0700，kurad 和 kura-key-recovery 均无法读取或执行。

清除 firewall-only 后按原版本装回完整 Networking，处理触发器后 44 项代码权限符合 Core 安装策略。两个包分别在自己生效期间独占原启动文件。原 sibling 加载顺序和锁恢复，两个能力回执均已退出，六包均 installed，dpkg 审计清洁。319 份运行文件的内容、权限和属主与开始时一致，仅 kura_custom.properties 的修改时间变化。Main、agent 和 firewall.service 均 disabled、inactive，六组件正常停机退出 0。停机复核了 42 笔维护的 759 份签名、101 个夹具对象及原凭据。六组件隔离网络的 IPv4、IPv6 规则导出始终为空。另一个已配置防火墙现场的 44 笔发布和 3 条规则已单独只读核对。

原绑定网关仍停放，6910532 对应编号 `7a1e4339-aaad-4ee0-94fc-8db58592881f`，原任务 SUCCEEDED、远程 OFF、三代来源与上次停放一致。原三现场与 7fd 停机证据已复核，7fd 为 41 输入、16 回执。12 个父项冻结闭环 0/12；外部延期仍为真实 NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。 [维护与停机核验](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-21 原六组件完整移除后沿同一编号恢复 Core 皮肤

原 ARM64 Root `39:6611544` 已完成 Core `1af242d3e8` 的整包保留数据重装。使用与已安装版本一致的六个正式 deb。移除 GWT、Wires、Deployment、Networking、agent 和 Core 后，六包均为 config-files，原运行目录已撤下。Core 的保留副本内有原 GWT conffile 登记、定制图标与非包站点图片。四组续接夹具按原围栏临时保管，101 个对象均已逐项核对。

六包联合解包后正常停机。只读挂载确认六包均为 unpacked，四个发行图标均以 .dpkg-new 等待处理。两份测试文件的原内容位于保留副本中。移除回执和重装围栏使用同一编号 `1b8d270e-0a1d-45f7-b02a-13c5512a04a4`。该状态下核验了 740 份既有签名记录和保留副本内的 254 份运行文件。

重启后沿该编号先配置 Core，再按原顺序配置 sibling 和 agent。使用 dpkg 的 confold 保留本地配置选择。Core 恢复了定制图标和站点图片，GWT 配置后另三个原先删除的图标继续缺失。两种运行账号读取的摘要均与输入一致。已核实并移除两份测试输入，恢复原有的四个配置图标缺失状态和四组续接夹具。最终六包 installed，dpkg 审计清洁，Main 和 agent 均 disabled、inactive，正常停机退出 0。42 笔维护共 759 份签名记录，319 份运行文件的内容、属主和权限与开始时一致，只有 kura_custom.properties 的 mtime 改变。原加载顺序、两份凭据和 85 项代码条目均已冷读。

原绑定网关仍停放，五笔维护封存，原任务 SUCCEEDED、远程 OFF。原三现场和 7fd 的停机状态、41 输入与 16 回执已复核。后续继续核对 firewall-only 与完整 Networking 切换时的独立配置、Root 启动入口和待处理代码权限触发器。仅保留原三项外部延期，12 个父项冻结闭环 0/12。 [移除、解包与重装核验](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-21 Core 升级保留 GWT 定制图标与站点皮肤

原六组件 Root `39:6611544` 已实装 Core `1af242d3e8`。升级前在原本为空的 console/skin 放入一个有别于发行默认值的有效 PNG 和一份独立站点图片。正式 dpkg 解包后，两份原件均可从同一维护编号的恢复副本中读回，GWT 的四项 conffile 登记与原包版本配对。配置后，两份文件的字节、属主、组、权限和 mtime 均与输入相同；kurad 和 kura-key-recovery 分别读出了原摘要。其余三个原先缺失的图标在升级中继续缺失。

Core 的三个维护钩子现在携带同一份皮肤保留脚本。它记录捕获时的 GWT 配置文件登记，恢复时核对当前登记，保留已到位的新包文件，并把缺失的站点文件逐项恢复。若站点的两个版本冲突且缺乏 dpkg 决定依据，原副本继续保留。ARM64 的 53 项定向测试通过。另从新 deb 提取脚本，配合具有真实 dpkg 配置状态的轻量 GWT 夹具，验证保留数据重装、联合解包后按 confold 配置、remove 至 config-files 和 purge 四种情况。联合解包保住了用户修改和删除选择；purge 后仅恢复非包站点图片。完整 Core 保留数据重装中的皮肤恢复仍待同 Root 整包验收。

已逐项核实并移除两份测试输入，夹具恢复到四个已登记 favicon 均缺失的原状。原加载顺序保持，六包 installed，Main 和 agent 均 disabled、inactive，正常关机退出 0。冷读验过 41 笔维护的 740 份签名记录、319 份运行文件、两份凭据、101 个夹具对象及 85 项代码条目。原绑定网关仍为 6910532，五笔维护封存、三代来源配对、原任务 SUCCEEDED、远程 OFF。原三现场和 7fd 均已复核。冻结闭环 0/12，外部延期仍限原三项。 [新包、原编号及停机核对](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-21 GWT 独立 purge、原皮肤配置与 Core 保留缺口

原六组件 Root `39:6611544` 对已装 GWT 单独执行 purge。开始时 dpkg 登记的四个 favicon 配置文件均缺失。测试在空的 console/skin 内放置了一份可单独识别的站点文件。GWT 清除后，其包脚本和 JAR 目录已删除，Wires、Deployment、Networking 三个 sibling 目录和站点文件逐项保持。Core 收到 console、siblings 和 libexec/addons 三条触发路径，一次处理完成。

同版 GWT 重新安装时恢复四个默认图标，并把 management-ui 追加到注册表末尾。Core 触发器自动修复 12 项权限。为恢复夹具，取得原锁后逐项核对四个新默认图标的指纹、测试文件和两份注册表，再恢复原注册顺序和四个配置文件缺失的原状。该复原是现场维护步骤。四个 conffile 的 dpkg 登记与维护前完全相同。最终六包 installed、pending 为空，Main 与 agent 均 disabled、inactive，正常停机退出 0。冷读核对 85 个已装代码条目、319 份运行文件、101 个夹具对象、两份凭据及 40 笔维护的 721 份签名记录。

另在一次性 ARM64 环境中，从 Core `b4ba0e06b9` 正式 deb 的 preinst、prerm、postinst 提取出相同的保留脚本。分别沿升级和保留数据重装的路径调用该脚本，维护回调在两例均恢复，console/skin 下的测试站点文件和 favicon 均未进入恢复副本。证据覆盖包内脚本，其范围不含整包 dpkg 的历史缺失归因。下一项优先修复 Core 对现有皮肤内容的保留与恢复，并核对管理员删除的配置文件、联合解包时 dpkg 新文件和已清除组件的边界。完整台账保留 12 个父项，冻结 0/12，外部延期仅原三项。 [安装回执与保留脚本覆盖](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-21 三个 sibling 的移除、purge 与停机后触发器续办

原六组件 Root `39:6611544` 保留 GWT，将 Wires 和 Deployment 移除到 config-files，随后 purge 两包并直接 purge 已安装的 Networking。三包卸载后，Core 的 dpkg 记录仍有 siblings、libexec/addons 和 bin/firewall 三条待处理路径。正常关机后，以只读方式确认三包已清除、GWT 原目录保持、原注册表只剩 management-ui。重启后执行一次正式 dpkg 触发器，Core 回到 installed，三个已清除的组件目录保持缺席。处理前后 Main、agent、密钥 socket 和恢复服务的 systemd 活动时间及调用标识一致。

Networking 移除时在原配置末尾追加 126 字节能力停用块。读回其 Root 私有回执，核对停用块与回执编号配对，剥离该块后的原配置字节与卸载前 SHA-256 一致。沿原顺序装回三个正式包后，配置文件恢复原摘要，停用回执已撤销。Core 触发器自动修正 77 项重装产生的代码权限，注册表恢复原字节与顺序，锁仍为 inode 7632058、root:root 0600。

最终六包 installed，pending 为空，Main 和 agent 均 disabled、inactive，正常停机退出 0。两次冷读均核对 319 份运行文件、两份凭据、101 个夹具对象与 40 笔维护的 721 份签名记录。最终还核对了 77 项重装条目和 7 项保留条目。在停机卷上分别切换到 kurad 和 kura-key-recovery 的实际 UID/GID 和补充组，两者均可打开三包的 28 个 JAR，逐包内容与正式 deb 相符，Root 专用防火墙启动入口均拒绝读取。此项验证文件读取权限，OSGi 激活沿用原有独立证据。

原绑定网关继续停放，五笔维护封存、三代来源可读。原三现场、7fd 的正常停机状态及 41 输入、16 回执均已复核。新触发器的 GWT 独立 purge、firewall-only 模式及其余 provider/launcher 组合继续留在内部台账。冻结闭环 0/12；外部延期仅原三项。 [包状态、触发路径和冷读记录](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-21 原六组件 ARM64 包维护自动恢复 70 项代码权限

原六组件安装的 Core 已升至 `b4ba0e06b9`，Networking 已升至 `83f237e`。Core 根据 dpkg 文件清单核对 sibling JAR、libexec、console 及防火墙入口。完成 Core 安装时记录带签名的 configured 回执，绑定安装 Root、原维护编号和包脚本。触发器失败后，dpkg 以普通 configure 续办时先核对这份回执，再单独恢复权限。

在原 Root `39:6611544`，GWT、Wires、Deployment 的包操作留下 31 项差异。持有原权限锁时 dpkg 触发器返回 1，Core 进入 half-configured。释放锁后只续配 Core 一次，31 项全部恢复。Main、agent、密钥服务 socket 和恢复服务的 systemd 调用标识及活动时间均与续办前一致。

联合维护时旧 Networking preinst 因 Core 已有待处理触发器，在解包前退出。新 Networking preinst 识别四个已登记的权限触发路径，继续核对四组 broker 的归属、脚本、socket、service 和协议。在 Core 已挂起 siblings 触发器时，原卷成功解包并配置 Networking 后继版，余下 39 项由正常触发器恢复。Networking 与 firewall-only 两个 deb 的业务载荷、权限和属主逐项等同上一版。本轮在原卷安装 Networking，firewall-only 只核对发行包。

最终六包 installed，Main 和 agent 均 disabled、inactive，正常关机退出 0。冷读核对 70 项相对于各自解包后的 inode、字节和 mtime，注册表锁在两次权限恢复中保持 inode 7632058、root:root 0600。40 笔 Core 维护的 721 份签名记录、319 份运行文件、两份凭据和 101 个夹具对象通过核验。ARM64 两组 Core 定向测试共 39 项通过，Networking 另有 7 项包内准入用例通过。

绑定网关沿原编号 7a1e4339-aaad-4ee0-94fc-8db58592881f 完成的 431dedc → 6910532 升级保持封存，五笔维护和三代来源回执已停机复核，原任务 SUCCEEDED、远程 OFF。7fd 保持正常停机，41 输入、16 回执。完整台账保留 12 个父项，冻结 0/12。后续仍需核对新触发器在移除、purge 和其余组件组合中的实际行为。外部延期仍限真实 NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。 [发行包与原卷冷读](actual-native-wires-retained-reinstall-checkpoint.json)


## 2026-11-21 排队的 sibling 注册表请求在 Core 切换目录后退出

12 个 sibling 仓的注册表脚本现在以原 framework 目录为工作目录。取得 flock 后同时核对公开目录和锁文件与原目录、打开的锁描述符是否配对。13 个正式 deb 的载荷变化均限于注册表脚本，每个 postrm 的内嵌副本已逐包核对，原业务 JAR 字节均保持。

ARM64 上的 install、remove、check 在原目录替换和同目录锁替换两种情况下均拒绝旧请求。使用已发行 Core `8f2952d302` 的 preinst 目录切换代码和新 Networking 包的脚本，在隔离 Root 确认旧请求进入内核等待队列。旧目录切换后，旧 remove 返回 1，两代注册表均保持，新请求在新锁释放后返回 0。这个对照在夹具中同步了进程调度。

原六组件 Root `39:6611544` 实际联合升级 GWT、Wires、Deployment 和 Networking。四包解包后核对了 Deployment 的原 Root 过渡凭据和 Main 的临时 mask。正式配置返回 0，注册表和锁 inode `7613316` 保持。配置期间的观察未取得等待进程记录；随后在同一 Root 单独确认四个已安装脚本的 check 同时排在该 inode，释放后全数成功。

停机发现四包覆盖了 70 项 Core 代码权限，涉及 30 个 sibling JAR、38 个 libexec 条目及两个 console 目录。已单独按已安装 Core 的权限规则修复这些条目，保住原 inode、文件内容、时间戳中的 mtime 和原注册表锁。包操作自动恢复这些权限仍为内部待办。最终六包 installed，Main 和 agent 均 disabled、inactive，冷读核对 319 份运行文件、两份凭据、101 个夹具对象及 39 笔维护的 702 份签名记录。新入口为 `sibling-registry-waiter/cold.py`。冻结 0/12。 [发行包、内核等待与原卷冷读](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-21 Core 升级遇到持有 sibling 注册表锁的进程时保留原安装

Core `8f2952d302` 在分配暂存目录、发布恢复副本和切换原 Core 目录时取得现有注册表的独占锁，并在发布和切换前核对登记文件的内容、权限、属主、组和修改时间。ARM64 两代正式包对照中，旧版在独立进程持锁时仍通过这三步，新版均拒绝；登记顺序在复制后改变时也拒绝发布。41 项定向测试通过。

原六组件 Root `39:6611544` 的正式 dpkg 在锁 inode `7599906` 被独立进程持有时拒绝解包。原 Core `46e5c9543a` 仍为 installed，原目录和注册表保持，新增暂存副本为零。释放锁后正常停机，冷读原 38 笔维护的 684 份签名记录。重启沿原维护号 `8608465c-37ba-4776-864b-51f0edf6e993` 解包并配置成功；原锁随旧目录进入恢复区，正式 Networking 检查在新目录创建锁。最终锁 inode 为 `7613316`，四行注册表及权限保持。

最终六包 installed，Main 和 agent 为 disabled、inactive。停机核对 319 份运行文件、两份凭据、101 个夹具对象以及 39 笔维护的 702 份签名记录。最新停机入口为 `sibling-registry-core-lock/cold.py final`。本次覆盖已持锁的写入者和复制后的登记变化；已打开旧锁并排队的进程跨目录换代尚未验收。冻结 0/12，external deferred 仅原三项。 [正式包对照与原卷冷读](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-21 原六组件在 sibling 写回失败后沿原维护号恢复

Core `46e5c9543a` 在复制 sibling 前打开暂存文件系统，四个目录发布前用 `syncfs` 检查写回结果。原六组件卷 `39:6611544` 升级 Core 时，现场只对该维护号暂存目录的 syncfs 注入一次 EIO。dpkg 返回失败，冷读 Core 为 half-configured，其余五包 installed。GWT、Wires、Deployment 和 Networking 四个目录均未发布，原恢复副本完整，当前编号已有 17 份签名记录。重启沿 `232e8e77-373d-4bb9-b826-5203f862dc3a` 续配一次后，四个目录的 30 个文件恢复，原 37 笔历史的 666 份记录逐文件一致。

Core 单包升级后，sibling 注册表保留四行和原权限，原锁文件缺失。正式 Networking 脚本执行一次 check 后按设计新建 root:root 0600 空锁，注册表文件的 inode、时间和内容均保持。锁 inode 已变化。旧锁被持有时跨 Core 换版的互斥连续性仍待验证。

最终六包 installed，Main 与 agent 均 disabled、inactive。停机读回 319 份运行文件、两份凭据、101 个夹具对象及 38 笔维护的 684 份签名记录。当前只读入口为 `sibling-restore-syncfs/cold.py final`。此次为定点注入写回错误；磁盘满与硬断电沿用原外部延期。 [实装包、原编号及冷读](actual-native-wires-retained-reinstall-checkpoint.json)
## 2026-11-21 原绑定网关在完成回执已有两个硬链接时恢复后继包

原绑定网关由 `431dedc` 升至 `6910532`。沿经授权的维护编号 `7a1e4339-aaad-4ee0-94fc-8db58592881f`，正式 dpkg 在完成回执建链后，只对删除暂存名的 unlink 返回了一次 EBUSY。停机读回 half-configured、原 pending 和同一份 0600 回执的两个硬链接，启动检查拒绝。重启后沿原 Root、原编号只续配一次。包恢复 installed，状态为 REINSTALLED，pending 和暂存名均已清除。完成回执保留 inode 7585379、mtime 和原摘要，链接数由 2 回到 1。

第三代来源回执指回第二代，第二代指回第一代。100 个原对象中，94 个完整指纹保持，5 份授权或指针文件按计划更新，另一个变动是 source-pairs 父目录。四个已安装 dpkg 钩子的摘要均匹配新包。两次停机冷读的五笔维护 JSON 摘要全部一致；原任务 SUCCEEDED，原 OFF 版本保持。网关正常停放，数据库和中心全程停放。此轮验证后继包维护，Main、agent 保持停用。建链前中断及目录 fsync 失败仍只有隔离测试证据。最新只读入口 `bound-agent-packaged-receipt-field/cold.py final`。冻结 0/12。[实装检查点](actual-native-agent-bound-business-removal-checkpoint.json)

网关停机后，原 SQLite（69,632 字节）和 WAL（12,392 字节）的 SHA-256 均与该笔维护意图一致。在 ARM64 隔离内存盘以只读连接查询两份副本，SQLite 快速完整性检查为 ok，执行表只有原任务一行，状态为 SUCCEEDED。三代来源回执逐代匹配各自的意图、后继包、身份和前代指针，四个目录均为 root:root 0700，三份回执均为 root:root 0600。复核入口为 `bound-agent-packaged-receipt-field/cold-business.py`。

## 2026-11-21 已绑定 agent 恢复重装回执的硬链接与目录同步中断

旧版在回执写完但尚未建立最终硬链接时续配遇到 FileExistsError；链接已建但暂存名仍在时，由于文件有两个链接而拒绝。另一个缺陷是首次目录 fsync 失败后，第二次配置会跳过该同步并清除 pending。agent `6910532` 只在原安装、原意图和 install-admitted 阶段匹配时继续：核实 Root 持有的 0600 完整回执，确认暂存和最终名称的 inode 及链接数，将原文件发布并同步目录，再清除 pending。内容不完整或两个名称对应不同文件时保留现场。

ARM64 原生包回归 144 项通过。从新 deb 提取实际 postinst 内嵌类，分别在同版本和后继版本夹具的建链前、建链后退出子进程。四个现场均沿各自原 Root、原编号恢复为 REINSTALLED，保留原回执 inode 和字节；systemd 和业务检查使用发行测试夹具。原绑定网关仍装 `431dedc`，保持正常停放，四笔已闭环维护冷读一致。该网关的只读入口仍为 `bound-agent-package-interruption/cold.py final`。冻结 0/12。[包与恢复检查点](actual-native-agent-bound-business-removal-checkpoint.json)

## 2026-11-21 已绑定 agent 在重装回执落盘后中断，沿原编号续配

原绑定网关的三笔维护均已 REINSTALLED。本次经授权为同版本维护单独登记 `ae9f4d4e-dad1-4be4-80ca-e6634b86748c`，复用实装包 `431dedc` 和原初始化。真实 dpkg 在私有挂载命名空间内写成重装回执，随后 unlink 原 pending 文件遇到 EBUSY。停机冷读为 half-configured，回执和 pending 同时存在，状态查询为 INSTALL_ADMITTED，启动检查拒绝。

原 OverlayFS 重挂后设备号从 1048716 变为 2097163，原 backing 摘要匹配。沿该编号续配一次，dpkg 回到 installed，pending 清除，查询为 REINSTALLED。完成回执的 inode、mtime、ctime 和摘要保持；跨两次停机读回，四笔维护的全部 JSON 文件摘要一致。100 个原对象指纹、两代来源回执、原任务 SUCCEEDED 和原 OFF 版本均通过。网关正常停机，Main 与 agent 停用；中心与 PostgreSQL 全程保持原停放状态。已有 ARM64 发行包无须重建，定向回归 5 项通过。最新只读入口为 `bound-agent-package-interruption/cold.py final`。冻结 0/12。[实装检查点](actual-native-agent-bound-business-removal-checkpoint.json)

## 2026-11-21 已绑定网关第二代来源回执与正常停放

原网关在保留第一代来源回执的情况下安装了 agent `431dedc`。沿用原初始化和本次维护编号 `65ed1487-76c1-499c-8eac-4c9995c15c55`，暂停后从 installed 状态只读续验。当前指针指向第二代，第二代按编号和摘要指回第一代，第一代无前项。98 个原对象中，92 个完整指纹保持，5 份代码授权或指针文件按计划更新。`source-pairs` 父目录增加第二代子目录，原 inode、属组、权限和链接数保留。两代各有一份完整回执。

正式 Main、agent 显式启动，各自重启计数为零。回环 HTTPS 返回 200，agent UID 991、有效能力零、NNP=1，运行二进制匹配发行包。原任务 `523ec79a-731b-4bb6-b274-9008a750b34d` 在网关与 PostgreSQL 均为 SUCCEEDED，中心仍只有原来一笔任务，原输入与结果密文指纹一致。原 OFF 决策及版本保持。网关和数据库正常停机，中心全程停放；停机后按原容器冷读两代回执、当前指针和 OFF 版本。ARM64 release 定向测试 26 通过、0 失败、3 忽略，另有 820 项未在此筛选范围运行。最新只读入口为 `bound-agent-second-successor/cold.py`。冻结闭环 0/12。[两代来源与停机证据](actual-native-agent-bound-successor-checkpoint.json)

## 2026-11-21 原防火墙启动事务阻止包维护

在原临时安装 Root `39:7320076` 上执行当前 Core、Networking 与 firewall-only 发行包共同包含的只读维护检查。四种钩子共 12 种参数，九项拒绝继续，三种 postinst abort 返回保留。Networking 的打包提交与现分支仅差一份测试文件。这里实际运行的是包内检查，未启动 dpkg。第一次探测位于外层容器，结果已剔除；最终检查在 chroot 内先断言 Root 身份。前后均核实原 IPv4 回执 APPLIED、IPv6 未派发、原 44 对发布记录及三条规则保持。开机恢复和完整 Main 冷读仍待办，冻结 0/12。[原请求及包维护检查](actual-installed-firewall-action-receipt-checkpoint.json)

## 2026-11-21 当前四个网关包在原 Docker 业务卷启动

ARM64 镜像使用共同分支的 Core `71eb1087ae`、GWT `9de7a7b`、Deployment `8f5927e`、Wires `4f13cf1` 及原四份 DP。镜像内四包的版本、架构与 SHA-256 全部匹配，随四包进入插件目录的 235 个 JAR 逐一对上原 deb。原卷 `39:6566861` 启动后，正式 Java Main 为 UID 122、有效能力零、NNP=1，HTTPS 返回 200。DeploymentAdmin 查到四个原版本，四个业务 bundle 均为 ACTIVE。正常停机后冷读，四份 DP、注册表、四份身份文件和三个快照均与启动前一致。最新 Docker 卷读回入口为 `docker-current-gateway-release/runtime.py cold`。六组件、原防火墙 Root、完整安装维护矩阵沿各自检查点续接。冻结 0/12。[镜像与原卷实测](actual-arm64-docker-nonroot-launcher-checkpoint.json)

## 2026-11-21 Alpine 启动器热更正式 Cloud 工厂，并在停机换版后重新注册服务

原 Docker 业务卷 `39:6566861`、现有 ARM64 镜像及只读 `/load` 目录验证了三个正式 Cloud deb 中的工厂 JAR。单独投放首版时，FileInstall 已安装 bundle；OSGi 缺少 `org.eclipse.kura.core.data.util`，没有注册工厂。补入同包的 Cloud 基础 bundle 后，第一版真实服务可调用。在同一 Main、同一 OSGi bundle ID 上替换第二版，先收到旧服务 244 注销，再收到新服务 245 注册。停机发布第三版，重开同一容器后如期注册；撤下后收到注销，并连续两次查到零个服务。三版的工厂实现类字节相同，限定版本不同，分别从原发行包逐字节提取。每次调用均由预期 bundle 返回原工厂 PID。

测试给基础 bundle 配置了第 4 级，发行包本身位于第 1 级；两者均无显式 start 标记。实测基础 bundle 仍为 ACTIVE，注册了三个服务，已有 MQTT 组件因配置无效报错。容器只有回环网络，未提交业务配置。原四个 DP、注册表和四份身份文件的指纹核对通过，三个原快照时间均早于本轮。原卷上仅三个目录的时间发生变化。正式 Main 为 UID 122、有效能力为零、NNP=1，HTTPS 为 200；夹具已正常停机，测试挂载目录已清空。最新读回 `docker-load-service-update/cold.py`。六组件继续沿 `site-provider-retained-remove/cold.py final`。G021 的 27 个 OSGi 过滤查询沿用原检查点，冻结 0/12，外部延期仍仅原三项。[启动、服务变更和原卷实证](actual-arm64-docker-nonroot-launcher-checkpoint.json)

## 2026-11-21 站点自装的 OSGi provider 随 Core 保留式重装恢复

旧 Core 的 prerm 只保存 sibling 维护脚本，站点没有自带维护脚本时，即使保住顺序表也会失去 JAR。Core `71eb1087ae` 把受保护的 sibling 代码目录保存到同一卸载暂存区，沿用原有配对和 postinst 恢复。ARM64 旧版回归实际缺少目录，修复后 48 项定向测试通过。

在原六组件 Root `39:6611544` 将 SQLCipher JDBC 驱动与 SQLite provider 分别登记为两个站点目录。共同移除后，保留目录中有这两份原 JAR 和两行登记。六包重新解包即正常停机，冷读六包均为 unpacked、两份源 JAR 仍在，另有 253 份保留文件、两份凭据、101 个夹具对象和 126 条签名记录。重启后直接 configure 六包，沿原卸载维护号恢复，两个站点 provider 的相对顺序保留。正式 Main 按两个还原路径加载，明文和加密库原行以及加密页完整性均通过。

测试登记和额外副本已退役，原四行顺序恢复。原夹具文件归位后正常停机，六包均 installed，Main 和 agent 均 disabled。停机冷读通过 319 份运行文件、两份凭据、101 个夹具对象、八代 144 条签名记录。最新入口 `site-provider-retained-remove/cold.py final`。其他 provider/launcher、完整 Core/OSGi、已绑定 agent 维护和其余父项照原范围继续；冻结 0/12。[包、维护号与两个实际 OSGi 路径](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-21 原六组件共同移除、停机、原编号重装及三种数据库读回

Core 升级能保存 provider 加载顺序，保留式卸载曾漏掉 `framework/sibling-install-order`，现场自定义顺序会在重装时丢失。Core `8414993dc9` 将普通注册文件放入原恢复目录。ARM64 回归覆盖站点末行无换行、属组、权限、mtime、扩展属性、没有注册文件的旧布局及链接拒绝；相关 42 项测试通过。

原网关 Root `39:6611544` 实装修正版后，Core、agent、GWT、Wires、Deployment 和 Networking 一起移除，全部进入 `config-files`。正常停机后冷读 252 份保留文件、两份凭据、101 个暂存夹具对象及五代 90 条签名记录。重启后六包一次重装成功，沿原卸载维护号完成 reinstall。252 份文件内容全部恢复；Networking 维护改写的 `kura_custom.properties` 字节和权限恢复，mtime 有变化。两份本地 provider JAR 和两处旧测试目录按原 inode 归位。agent 此前尚未初始化。

正式 Main 同时核对原活动业务部署、H2、SQLite、SQLCipher 三条原始业务记录和加密页完整性。再次停机后六包均 installed，Main 和 agent 均 disabled。82 份重点文件中，仅 H2 数据库经实际打开后字节有变化。冷读 318 份运行文件、两份凭据、101 个夹具对象和六代 108 条签名记录通过。六组件入口改为 `core-remove-sibling-order/cold.py final`。本轮为保留式移除；已初始化 agent 的受控归档及其余 provider/launcher 组合按原台账续接。冻结 0/12。[原编号、包指纹与实际 Main](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-21 四个 sibling 在原业务卷共同 purge，停机后按原顺序重装

原六组件 ARM64 Root `39:6611544` 共同移除 GWT、Wires、Deployment 和 Networking。四包均进入 `config-files`，随后共同 purge。正常停机后的只读挂载确认 Core、agent 和原 SQLCipher 库仍在，注册表为空，原锁、82 份业务文件、两份凭据、四代 72 条签名记录及 101 个夹具对象均通过核对。

原系统重启后，从四份原始 deb 按站点顺序重装，恢复了四行注册表、0640 和属组 990。正式 Main 中两个 Deployment bundle 均读到原业务归档，原业务 bundle 为 ACTIVE。再次正常停机，Main 与 agent 均 disabled，82 份文件连同 inode、时间和内容均与本轮开始时一致。最新冷读入口是 `four-addon-business-purge/cold.py final`。Core 与 agent 的联合保留式移除、其余 provider/launcher 组合继续，冻结 0/12。[四包维护回执](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-21 Core 与 Deployment 联合解包，正常停机后直接配置成功

Core `e8d57815e0` 在 systemd 报告 Main 已屏蔽时检查实际 FragmentPath。对 Deployment 的 early generator，核对同一 Root 的 schema 2 替换记录、生成器与原锚点的 inode 和内容、指向 /dev/null 的屏蔽、MainPID、ControlPID 及队列任务。时钟服务和 socket 继续沿各自原围栏核对。ARM64 用原 Deployment 生成器验证配对；时钟回归 97 项通过，4 项跳过。

原六组件卷 `39:6611544` 先解包 Core，再解包 Deployment，正常停机并重新启动同一容器。Main 仍被原生成器屏蔽。85 份维护期文件和编号 `1eecebe3-aabe-4833-8dc8-09e92e9a722f` 保持。一条 `dpkg --configure kura-core kura-deployment` 返回 0，生成器由 Deployment 正常退役。此前需要移除再安装 Deployment 的直接配置故障在该顺序已修复。正式 Main 以 UID 997 启动，两个真实 bundle 读回原业务注册。最终正常停机，Main 和 agent 均 disabled。四代 72 条 HMAC、82 份保护文件及两份凭据已冷读；两个历史测试目录和两份本地 JAR 仍按同卷暂存归位，101 个对象保留原身份。Deployment 用相同 Git 源码重新组包，版本推进，四个 JAR 摘要与前任相同。

六组件只读入口 `deployment-generator-joint/cold-maintenance.py`，独立防火墙仍沿 `firewall-ipv4-loopback-port/retained.py` 保持原启动命名空间。其余联合维护顺序、provider/launcher 及内部父项按原台账继续，冻结 **0/12**，外部延期仍仅原三项。[联合配置和冷读检查点](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-21 部署回执阻止 Core 复制原文件，六组件按原维护号恢复

Core `ef207249bf` 在维护准入和 Main 停止后的写入者检查中识别标准包注册表的未结发布。Deployment `8f5927e` 提供停机续办入口，原存储属主持原回执 SHA-256，在注册表锁内完成剩余重命名。ARM64 的发行 JAR 实测同一安装、原始两份暂存文件；错摘要与 Root 调用均拒绝，原输入只提交一次。该隔离中断使用测试 DeploymentAdmin；原六组件实装后的正式 Main 另行核对两个真实 bundle 的注册表读入和原活动业务包。

Core 与 Deployment 同时解包时，Deployment 建立了配对的 systemd early generator 屏蔽。原 Core 配置在 `setup-layout` 结束后因时钟脚本 `UNSAFE_UNIT` 拒绝，Deployment 又依赖已配置的 Core。通过正式 dpkg 移除 Deployment、沿原编号 `c21c3730-171a-42b6-9f49-0a21d545f746` 完成 Core 配置，再装回同一个 Deployment deb。原四条阶段记录保持，原六组件 Root `39:6611544` 现已正常停机，冷读 54 条 HMAC、82 份保护文件及两份凭据。两份本地测试 JAR 和两个依赖 inode 的历史目录按同卷暂存后归还。最新六组件入口 `deployment-core-maintenance/cold-maintenance.py`，独立防火墙继续沿 `firewall-ipv4-loopback-port/retained.py`。Core deb 使用本地时间版本 `6.0.0~git20261121170344.ef20724-1`，后续安装先用 dpkg 比较版本。直接联合 configure 的生成器配对仍待修复。冻结 **0/12**，外部延期仍为原三项。[发行件、原阶段及停机续办证据](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-21 原六组件卷联合升级 Core 和 Networking，正式 Main 读回原业务

Root `39:6611544` 通过 dpkg 从 Core `5bd7710`、Networking `783a594` 升至 `00c3f8fe08`、`671a90f`。其余四包版本相同。正式 Main 以 UID 997、有效能力 `0x400` 启动，读取已注册的 `fixture.retained.business` 1.0.0 包、活动 bundle 的原资源和 H2 原业务行。随后正常停机，Main、agent 均 disabled。只读续接入口为 `six-nat-worker-upgrade/cold-maintenance.py`。

正式升级脚本保全了业务包、数据库、快照。启动前核对 82 份保护文件的原内容及权限。两个历史部署测试目录和两份本地 SQLCipher 测试 JAR 在同卷暂存归还，冷读核对 101 个对象的身份与内容，包括旧版遗留的 pending 文件。旧、新两次升级各有 18 条已核实的 HMAC。独立防火墙安装仍沿 `firewall-ipv4-loopback-port/retained.py` 保持原启动命名空间和 IPv4 回执。其余 provider、launcher 和包维护矩阵继续，冻结 **0/12**，外部延期仍为原三项。[联合升级与两代回执](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-21 正式 Main 发布 IPv4 回环端口转发，旧版在开机读回 NAT 表时停止

防火墙 Root `39:7320076` 的旧站点文件有四条入站规则。历史记录显示，Main 应用 TCP 22、443 后，IPv4 保存模型和当时的内核均为六条。此次从原管理员测试输入重建六条规则并核对历史摘要，只在隔离安装首次启动的 firewall.service 中临时只读挂载。原站点文件及上一笔封存的 NAT 快照均保留。正式 Main 经 ConfigurationService 提交独立的 IPv4 端口转发一次，读回新快照与三条内核规则。UDP 49170 转到 127.0.0.2:49171，原生 NAT 查询可见。

正常关机后在原 Root 无站点配置启动，IPv4 原生恢复回执 APPLIED，转发开关为 1。旧 worker 写出的 DNAT 形式使 iptables-save 无法列出它负责核对的 NAT 表，启动器停在已派发 IPv4 的读回阶段。IPv6 尚未派发，Main disabled、未启动。**临时安装当前开机保留原命名空间**。以 `firewall-ipv4-loopback-port/retained.py` 只读核对，不使用上轮停机入口。原六组件仍沿 `deployment-paired-archive-commit/access/cold-maintenance.py`。

worker `c09b059` 为原 iptables 模型写出对应的 xtables NAT 动作。Core `00c3f8fe08` 已构建 ARM64 deb。隔离内核分别验证两族五种规则的发布、开机恢复和撤销；最终 deb 的 worker 能替换并移除旧版已确认的 DNAT 且保留旧回执。13 项 crate 测试通过。原临时安装没有升级或重发；其启动回执及完整 Main 冷读仍待续办。冻结 **0/12**，外部延期仍为原三项。[原编号、包摘要及验收范围](actual-installed-firewall-action-receipt-checkpoint.json)

## 2026-11-21 正式 Main 的 IPv4 配置因现场规则不同而被拒绝

临时防火墙安装 `39:7320076` 的正式 Main 通过 ConfigurationService 提交了一次回环 IPv4 NAT。快照保存成功，worker 在 PREPARED 前拒绝发布。原保存模型的 filter/input-kura 链有六条，内核有四条，缺 TCP 22、443。未新增内核业务规则。原发送端无 pending，本次操作前后的 36 对发布记录相同。配置服务保存与内核生效需分别验收。该请求保留原件，未重发。

停 Main 后将本次独有快照在原目录改名留存，inode 和内容一致，原三份启动快照保持。夹具正常停机、Main disabled。新版 worker 在冲突日志中提供地址族、表、链和模型中的规则序号；ARM64 原生测试 12 项通过，另用发行 crate 对照六条与四条，定位到第 4 条。Core 正式 worker 构建已校验源归档，现场仍装上版。本轮未用拒绝代替 IPv4 正向验收，正式 Main 正向仍待有配对的 IPv4 站点模型。六组件沿原入口冷读，防火墙改用 `firewall-ipv4-configured-main/cold.py`。冻结 **0/12**，外部延期仅原三项。[快照、现场差异和源码版本](actual-installed-firewall-action-receipt-checkpoint.json)

## 2026-11-21 真实 Felix 安装中断后的包索引与业务 bundle

用 Deployment `ba4bc90` 发行包中的本地、云端安装方法，在 ARM64 Equinox 3.24.100 中调用实际注册的 Felix 0.9.10。三份独立业务 DP 分别只提交一次。安装命令执行后、索引替换前中断时，原 Felix 临时目录保留，冷启动查询不到包。索引替换后、方法返回前中断，以及 Felix 返回给本地代码、Kura 尚未登记时中断，冷启动均查到包版本 1.0.0。三次 Equinox 枚举均缺少业务 bundle，磁盘上的 bundleFile 仍与原输入内 JAR 逐字节一致。原 Kura 注册表其他行保留，未发布新的业务归档。三次结果维持 UNKNOWN。现有云端 GET PACKAGES 读取 Felix 声明，GET BUNDLES 枚举运行时，两者可分别核验。

这组测试使用真实发行类和最小 Equinox，在同一容器、各自原存储目录中恢复。完整 Main 的已配对归档恢复见下一节。尚需解决在调用 Felix 前保存原请求身份、按原件查询和阻止重复派发的问题。原六组件与防火墙停机冷读入口沿用上次。冻结 **0/12**，外部延期仍为原三项。[三处中断、存储身份与结果](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-21 Deployment 两次重命名之间停机后，正式 Main 补回原注册表

原六组件 Root `39:6611544` 的新版 Deployment 在 DeploymentAdmin 成功返回后，把归档和注册表两份暂存文件与原安装配对。实装对照在归档已发布、注册表尚未改名处终止 ARM64 测试 JVM。旧版无配对凭据。新版两条入口保住原暂存；停机并沿原卷重启后，正式 Main 内的本地和云端读取代码分别补完登记。注册表 inode 与各自暂存一致，归档、输入和一次调用记录均保持。故障注入使用单独的试验文件和受控 DeploymentAdmin；原业务 DP 另由真实 Felix 读回 ACTIVE bundle 资源。首次观察把目录别名和规范路径当成不同归档，后续按文件身份核实已完成的本地结果，只续办云端待办。

最终安装 `ba4bc90` 另验两种状态：凭据可被组写时拒绝发布，恢复权限后沿原文件提交；两份文件均已发布时只退役凭据。JDK 21 构建 247 项通过，最终 deb 240,140 字节。六组件已正常停机，Main、agent 均 disabled。最新六组件冷读入口 `deployment-paired-archive-commit/access/cold-maintenance.py` 同时核对 82 份文件、两份凭据、原 Core 18 份 HMAC 和本次四组结果。防火墙沿 `firewall-configuration-snapshot/cold.py`。Felix 内部结果不明及原配对凭据形成前的中断仍待验。12 父项冻结 **0/12**，外部延期仅原三项。[检查点](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-21 Deployment 在注册表锁竞争时保住原业务归档

原六组件 Root `39:6611544` 上，旧版本地和云端安装入口都在取得注册表锁前替换同名业务归档。等待锁时终止测试 JVM，注册表和候选源仍是原值，目标归档已变化。Deployment `eaec433` 让两处归档替换与注册表写入共用锁。实装后，两入口在等待处遭 SIGKILL 均保住原归档和 inode；在另一次验收中释放锁，原进程完成发布，其他注册行保留。竞争试验用可控的 DeploymentAdmin 返回值。正式 Main 另行核对两份已加载的类、原业务 DP 的 ACTIVE bundle 及资源。

后继 `d53e20b` 保留首次使用时创建共同目录的行为。最终 deb 为 228,322 字节，JDK 21 完整构建 242 项测试通过。最终版本在原卷上通过本地和云端入口首次创建目录、归档及注册行。冷读核对六包、82 份文件、两份凭据和原 Core 维护的 18 份 HMAC；Main 和 agent 均 disabled、正常停机。最新六组件入口 `deployment-registry-publication-lock/first-use/cold-maintenance.py`，防火墙入口 `firewall-configuration-snapshot/cold.py`。Felix 返回后，归档与注册表两次重命名之间的中断仍待验。冻结 **0/12**，外部延期仅原三项。[两版对照与实装证据](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-21 ConfigurationService 保存的 IPv6 规则跨完整 Main 重启保留

临时防火墙安装 `39:7320076` 的正式 Main 通过注册的 ConfigurationService 保存一条回环 IPv6 转发配置。正常停机后沿原 Root 启动，在没有站点规则文件的新网络命名空间中，firewall.service 先恢复规则；完整 Main 启动后，注册 provider 仍返回原规则，原快照 `1795248549312` 的文件指纹保持，两条内核规则均逐条通过。随后同一配置 API 撤销规则，新快照 `1795248649849` 中 NAT 为空，内核规则和开机授权均已移除；当时 IPv6 转发开关仍为 1。观察器修正了末尾分号的比较，两次配置写入各提交一次。

复用既有 Core 和 Networking deb。临时安装正常停机，最终冷读 159 份文件、34 对发布记录、11 份启动记录。防火墙续接入口 `firewall-configuration-snapshot/cold.py`，原六组件仍用 `six-local-sqlcipher-upgrade/cold-maintenance.py`。其余 provider/launcher 及六组件维护矩阵继续。冻结 **0/12**，外部延期仅原三项。[快照、正式 Main 与内核证据](actual-installed-firewall-action-receipt-checkpoint.json)

## 2026-11-21 正式 Main 授权的 IPv6 转发经正常停机后恢复

防火墙临时安装 `39:7320076` 升级至 Core `7f04a990f3` 和 Networking `671a90f`。正式 Main 经注册的 IPv6 provider 加入两条回环规则。正常停机再启动，标准 firewall.service 看到两个地址族均无站点文件，依原 Root 的 Main 回执执行 85 条内核命令，两族转发从 `[0,0]` 变为 `[0,1]`。逐条核对两条内核规则通过。正式 Main 随后按原 OSGi 配置自动移除临时规则和启动授权，当时 IPv6 转发仍为 1。再次正常停机启动后两族均为 0、两条规则均已消失。隔离 ARM64 内核另验双族授权、旧请求、撤销和中断续办。IPv4 正向尚限隔离 worker；业务配置经 ConfigurationService 保存并跨完整 Main 重启仍待验。

临时安装最终正常停机，Main disabled，冷读 136 份文件、25 对发布记录及 9 份启动记录。防火墙续接入口 `durable-firewall-forwarding/cold.py`，原六组件沿 `six-local-sqlcipher-upgrade/cold-maintenance.py`。12 项父任务全部保留，冻结 **0/12**。外部延期仍仅真实 NM/cloud-init/物理硬件交接、amd64、磁盘满/硬断电。[安装、内核规则与冷读证据](actual-installed-firewall-action-receipt-checkpoint.json)

## 2026-11-21 暂存防火墙回执在内核和原历史均核对后发布

worker `6b26fdd` 在开机续办中先核对原编号、完整历史、内核规则与世代，再发布已暂存的回执。正式 ARM64 worker 在隔离内核中实测三种边界。内核世代改变时，七个原条目指纹保持，下一族没有派发。普通查询遇到启动器的暂存回执会要求沿原启动编号续办；另一个普通发送方仍能核对并完成自己的暂存回执。条件吻合时两个地址族沿原启动记录完成，双族转发均为 0。当前明确转发的 Main 请求只绑定当次操作与命名空间，无站点时的持久转发授权继续待验。

防火墙临时安装 `39:7320076` 已升级 Core `76986fa98f`，原 18 次发布保留。其间首包 `4ec4b02ed4` 暴露了 Maven 过滤 postinst 时把 Networking 版本写死为 Core 项目版本的问题。修正后从最终 deb 提取脚本，验证混合 dpkg 状态、独立 Core 两种情况；原安装实际输出 Networking `3.0.0~git202611210733.783a594-1 installed`。Main 为 disabled，正常停机冷读 99 份文件。最新防火墙入口 `firewall-boot-receipt-admission/cold-model.py`，六组件入口仍为 `six-local-sqlcipher-upgrade/cold-maintenance.py`。冻结 **0/12**，外部延期仍为原三项。[安装与内核证据](actual-installed-firewall-action-receipt-checkpoint.json)

## 2026-11-21 原六组件联合升级自动保留本地 SQLCipher provider

在原六组件 Root `39:6611544` 上，将夹具的 SQLCipher driver 与 provider 放入启动器支持的本地 `siblings/` 目录。Core `5bd7710ed8` 与 Networking `783a594` 均解包后正常停机，重启沿原维护编号恢复。两个 JAR 由安装脚本自动放回，正式 Main 从恢复目录加载它们，读回原明文及加密 SQLite 业务行，SQLCipher 页完整性通过。首版观察程序未识别 Equinox 相对地址，只更换了观察程序。验收后两只原文件按原 inode 放回夹具。另由正式 Main 通过原 H2 服务只读核验唯一业务行。相较升级前，82 份受保护文件中，仅 H2 数据文件内容指纹有变化。

冷读核对全部六包、SQLCipher、82 份最终文件、两份原凭据、注册表与锁，以及同一维护编号的 18 个 HMAC。Main、agent 均 disabled 并已正常停机。六组件最新入口 `six-local-sqlcipher-upgrade/cold-maintenance.py`。防火墙安装沿用 `firewall-boot-package-recovery/cold-model.py`。Core `bb1aa463cc` 修正联合升级期间的错误提示，已通过生成脚本及独立 dpkg 状态测试，现场安装仍为 `5bd7710ed8`。其余 provider、launcher、组件中断矩阵与无站点转发授权仍未结。冻结 **0/12**，外部延期仍为原三项。[联合升级和业务读回](actual-native-wires-retained-reinstall-checkpoint.json)

补验两个 sibling 目录先后发布时的进程中断。在隔离 ARM64 测试进程发布第一个目录后发送 SIGKILL，恢复沿原围栏，只补第二个目录并保留首个 JAR 的 inode，同组 14 项通过。六个包各自的实装和中断证据已列入[设计与磁盘报告](../../reviews/management-design-and-storage-20261120.md)。

## 2026-11-21 防火墙暂存回执的启动配对与 Networking 包恢复

Networking `783a594` 把暂存回执的发布移到完整启动配对之后。ARM64 实际启动器在包模式改变、另一个地址族出现站点策略、出现站点扩展三种情况下均保留原暂存回执、意图与操作编号。恢复原条件后，两族按原编号续办。Networking 2268 项测试及启动归属 17 项通过。

临时防火墙安装 `39:7320076` 已实测 Networking 移除、purge、解包后停机、重启原地配置。原 IPv6 启动操作在各阶段一直按同一编号取得 APPLIED。两次冷读均核对 63 份文件，含 6 份启动记录，18 次发布的意图与结果完整。最终 Main disabled、停止，最新入口 `firewall-boot-package-recovery/cold-model.py`；六组件仍从 `firewall-cold-model/six/parked-readback.py` 续接。此前 BIND 关机等待另在同一 Root 查清：仅提供 IPv6 站点时，IPv4 回环受缺省入站策略限制，正式 rndc 连接 127.0.0.1:953 超时；named 随后自行退出，系统退出码 0，从发出关机信号到停止为 60.25 秒。启动时无站点的转发授权、其余 provider/launcher 与六组件维护仍待验。冻结 **0/12**，外部延期仍为原三项。[检查点](actual-installed-firewall-action-receipt-checkpoint.json)

## 2026-11-21 正式防火墙启动器按原安装回执恢复 IPv6

临时安装 `39:7320076` 已升级 Core `5bd7710ed8`、Networking `03917ad`，worker 源码为 `6a80919`。正式 systemd 启动时，运行时挂载使服务看到空的站点目录，两个原文件的内容及元数据保留。启动器以该 Root 已确认的规则恢复 IPv6，83 条内核命令取得 APPLIED，IPv4 的 22、443 未放行。双族转发均为 0。此次 Main 保持 disabled、停止。恢复通用服务定义后停机；BIND 的 rndc 停止客户端等待约 50 秒，经单独发送 TERM 后系统退出码为 0。无干预停机仍缺本次现场证据。

隔离 ARM64 内核验证已落盘和待发布的回执均可沿原编号续办；缺回执、转发开关或内核状态漂移、保存摘要变化均得到拒绝。站点 IPv4 与回执 IPv6 并存时各自生效。停机后冷读 57 份文件，核验 18 笔原编号的终态历史与本次启动回执。现场入口 `firewall-confirmed-boot-service/cold-model.py`，六组件仍用 `firewall-cold-model/six/parked-readback.py`。无站点时的主动转发授权、其余 provider/launcher 和组件维护仍待验。冻结 **0/12**，外部延期仍为原三项。[发行、启动与中断结果](actual-installed-firewall-action-receipt-checkpoint.json)

## 2026-11-21 原 Root 回执确认开机规则，隔离内核恢复 IPv6

Core `ad2a586c31` 实装 worker 0.1.10（源码 `5781b22`）。成功或无需变更的发布在既有回执中记录该族候选规则摘要，并绑定同一 Root 下的完整终态历史。旧 worker 后续发布会使此前的证明失效，拒绝的变更保留此前确认值；缺少回执仍为 UNKNOWN。新旧 worker 已在 ARM64 隔离内核共同读回原编号，降版、重新升级和未完成回执的边界均有实测。

原防火墙临时安装 `39:7320076` 已升级至该 Core 与 Networking `a7dfc51`。启动前的 15 次旧发布没有可用规则摘要。正式 Main 启动后新增两份 IPv6 UNCHANGED 回执；停机冷读独立重算全部 17 次发布，确认的 IPv6 摘要与保存文件完全一致。IPv4 保存值仍是上轮最后使用的配置，含 SSH 22 和 HTTPS 443；站点启动输入不含这两条规则，内核逐条核对均未命中，原 IPv4 文件保留。正式 Main 的 128 个线程能力上限为 0x0400。现场已正常停机，55 份文件指纹通过，最新读取入口 `boot-firewall-policy-proof/cold-model.py`。原六组件续接入口 `firewall-cold-model/six/parked-readback.py`。

另在仅有回环的 ARM64 内核中，由包内 worker 确认 IPv6 规则后清空该测试族规则，运行正式启动脚本建立基础链。按同一 Root 确认的摘要、以新的恢复编号提交 83 条内核命令，回执 APPLIED。随后逐链匹配完整目标，额外配置的独立链保留；转发维持关闭。本次由测试程序连接启动脚本与 worker，正式无站点策略的自动启动接入、转发授权和中断续办仍待实施。其余 broker 及组件维护继续，冻结 **0/12**，外部延期仍为原三项。[发行、原安装及内核恢复记录](actual-installed-firewall-action-receipt-checkpoint.json)

## 2026-11-21 正式六组件绑定共同防洪规则，双族保存模型冷读恢复

Networking `a7dfc51` 修复了保存文件没有标记防洪规则来源时的 Java 冷读。读取器用发行件定义和已解析的业务规则重建模型，仅在整份输出与原始字节一致时恢复防洪设置及 ICMP 状态。IPv6 的 38 条共享规则现在只有一处定义。IPv4、IPv6 的防洪及 ICMP 开关八种组合、残缺规则和额外内容已纳入回归。完整 Networking 构建 2268 项通过。标准和 firewall-only 两个 deb 的发行类分别在无网络、零能力的 ARM64 JDK 21 进程冷读上轮原样输入，两族均逐字节匹配。

原六组件 Root `39:6611544` 已实际升级标准 Networking 包。正式 Main 的两个 OSGi 消费者都通过包连线取得同一规则类，实测有效能力 0x0400。正常停机后六包、SQLCipher、82 份文件、两份凭据、注册表与锁通过冷读。最新入口 `firewall-cold-model/six/parked-readback.py`。防火墙临时安装继续用 `main-process-network-capability/cold.py`。

无站点策略时的内核冷恢复仍待实施。原临时安装上只读核对已配对发布意图与回执，其中保存的是当次变更的内核批次与请求摘要。原号没有重发。其余 broker 和维护矩阵继续，冻结 **0/12**，外部延期仍为原三项。[两种发行件、OSGi 绑定和原卷证据](actual-installed-firewall-action-receipt-checkpoint.json)

## 2026-11-21 正式 Main 整进程撤去 NET_ADMIN 与 Networking 启动器恢复

Core `e660ca6d67`、Networking `d0a80e0`。原防火墙临时 Root `39:7320076` 先拒绝不兼容的旧 Networking。升级后定位到 systemd 203/EXEC：Core 替换了 bin 目录，dpkg 登记的 Networking 启动器随之消失。保留及恢复逻辑现核对已登记提供包、校验和及代码权限，联合解包时保留新版本，移除的提供包不恢复。正式 Core 单包升级后，dpkg 校验通过，启动器为 root:root 0700。ARM64 维护与权限测试 59 项通过。

在同一临时安装，用当前 Networking 发行件的默认配置生成 IPv4 和 IPv6 站点规则，两份摘要分别与该安装原保存值相同。正式防火墙服务启动后，非 Root Main 发布一条仅接受回环 TCP 19743 的规则。原编号 738a021b-a7a3-4857-b709-7b746dd22df8 回执 APPLIED；应用前、发布后和逆操作后分别检查 151、134、133 个线程，能力上限均为 0x0400。内核逐条核对成功；撤回后两族 filter 表和保存值均恢复。先前转发业务的原号仍可在原 Root 查询 APPLIED。正常停机后冷读 51 份文件。临时安装保留两份测试站点策略，续接用 `main-process-network-capability/cold.py`。

原六组件 Root `39:6611544` 单独升级同一 Core 后，系统启动器权限正确。正式 Main 的 132 个线程均已撤去 NET_ADMIN，SCR 绑定 Networking 的 NM 扫描 provider。按原夹具流程，两个可选 SQLCipher bundle 在升级时暂存并放回，未计作无人值守保留验收。停机后六包、SQLCipher、82 份业务文件、两份凭据和注册表冷读通过，续接用 `main-process-network-capability/six/parked-readback.py`。

**仍待实施**：没有站点策略的默认启动现场，保存模型有 17 条防洪 mangle 规则，重启后的 Java 模型缺少这些规则。匹配检查拒绝发布，未产生新操作。该路径继续作为内部恢复任务；其余 broker、组件和 provider/launcher 维护矩阵继续。冻结闭环 **0/12**，外部延期仍为原三项。[整进程权限、回执、升级和停机证据](actual-installed-firewall-action-receipt-checkpoint.json)

用 JDK 21 对当前发行类做离线冷读，IPv4 和 IPv6 的附加规则均还原为零。用防洪 provider 的默认配置独立补回 IPv4 的 17 条 mangle 规则，以及 IPv6 的 38 条 filter 和 26 条 mangle 规则后，两个模型均逐字节匹配各自输入。启动器在没有站点策略时仅建基础链，内核和 Java 模型的冷恢复均需验收。原卷的两个 SQLCipher 测试 JAR 由夹具注入，正式部署包的登记恢复已有独立验收。[双族读数](../../../../.management-close-20261116/core-rtc-hotplug-20261120/main-process-network-capability/parser-readback/result.log)

## 2026-11-21 正式 NetworkService 的 Wi-Fi 扫描与原六组件升级

Networking `d0a80e0` 将旧 OSGi API 的扫描请求交给 NetworkManager，nl80211 和 WEXT 读取驱动缓存，保留旧接口的每个 BSSID、支持速率和能力标志。完整 Networking 构建 2266 项通过。ARM64 无网络、零有效能力的 JVM 使用私有 D-Bus 无线服务和受控驱动输出，生产调用链发出一次 RequestScan，仅执行接口查询和缓存读取。原六组件 Root `39:6611544` 经 dpkg 升级后，正式 Main 的 SCR 确认 NetworkService 为 ACTIVE，绑定的服务来自新版 NM bundle。该进程实测 0x1400。停止后，六包、SQLCipher、82 份业务文件、两份凭据及注册表和锁冷读通过。原六组件续接入口为 `main-network-service-scan/six/parked-readback.py`。防火墙 Root `39:7320076` 仍用 `main-firewall-reduced-caller/cold.py assert-final`。

整进程撤去 NET_ADMIN、其余网络直接写入消费者与维护矩阵继续。真实无线与 AP 模式的设备行为尚待实证。冻结闭环 **0/12**，原三项外部延期照旧。[发行件、ARM64 调用与正式绑定](actual-nm-typed-wifi-scan-checkpoint.json)

## 2026-11-21 正式 Main 的低权限调用及共同 purge 后的 Core 配置恢复

在仅有回环的 ARM64 安装 `39:7320076`，Core `b1c664aaeb` 的正式 Main 调用线程从 0x1400 降到 0x0400，经 0.1.9 worker 写入两条 UDP 规则。iptables-save 导出了状态规则，两条均由 iptables -C 核对。IPv4 转发从 0 变为 1，IPv6 保持 0，移除临时规则后两族 filter 表和保存文件恢复。主线程仍为 0x1400。原号 `9726c608-97e8-4f85-9930-8d87593af441` 的结果为 APPLIED。

共同 remove、purge Core 和 Networking 后，原 14 份防火墙记录及两份凭据保持。重装在 Core 的 PREPARED 阶段实际中断，冷盘为 half-configured 和 unpacked。旧版首次续配被开机自动重试的 Main 挡住。停止经过配对核对的该项 systemd 任务后，沿原维护号完成续配。修正版 Core `bba8b1d1c8` 用 AssertPathExists 在服务进程创建前拒绝开机任务。随后在同一 Root 对修正包的升级配置实施中断；冷启动看到 Main 仍 enabled、inactive，MainPID、ControlPID、重试数均为 0。直接完成原配置，17 份阶段记录完整，原业务回执仍是 APPLIED。正式 Main 随后成功启动并正常停止。停机入口为 `main-firewall-reduced-caller/cold.py assert-final`。原六组件继续从 `main-firewall-xt-state/six/parked-readback.py` 续接。ARM64 定向 50 项通过。整进程撤去 NET_ADMIN 及其余 provider/launcher 矩阵继续，冻结 0/12。[普通 Main、两个维护号及停机冷读](actual-installed-firewall-action-receipt-checkpoint.json)

## 2026-11-21 连接状态规则经新旧发行包交接，原六组件升级到 Core b1c664aaeb

防火墙 worker 0.1.9 保留 Java 规则中 state、conntrack 及取反的原意。ARM64 发行件在 IPv4 和 IPv6 上写入后，iptables-save 与 ip6tables-save 均输出完整 filter 表，六条源规则逐条通过 -C。另在同一隔离 Root 用 0.1.8 写入两族规则，换 0.1.9 后按各自原编号取回 APPLIED，再各用一条内核命令改为新状态条件。新规则可导出和匹配，原四份意图和结果文件的摘要一致。ARM64 Rust 工作区 807 项通过，50 项忽略。

六组件原 Root `39:6611544` 已安装新 Core。正式 dpkg 升级返回 0，审计为空。冷读确认六包、SQLCipher、82 份受保护文件、两份凭据和注册表。旧锁随 framework 重建移除，原 inode 无持锁者。续接入口为 `main-firewall-xt-state/six/parked-readback.py`。前版正式 Main 已有转发正向记录；复用该临时安装时 Root 由 `2097164:7248502` 变为 `2097166:7248502`，升级前即拒绝，未查原编号，已正常停机。新版正式 Main 的正向调用仍待验。冻结 0/12。[双版本、原回执及冷读](actual-installed-firewall-action-receipt-checkpoint.json)

## 2026-11-21 普通 Main 的受限转发和原回执跨 Networking 包维护

ARM64 实装 Core `668c7c596e`、Networking `71f30b6`、worker 源 `1200e75`。只含回环接口的独立 Root `2097164:7248502` 上，正式 Main 以 kurad 经 systemd socket 发布两条 UDP 转发规则。内核先接受规则，再从固定控制文件读回 IPv4 为 1，IPv6 保持 0。原号为 `af4cfded-7405-49a3-b558-8d9ff1389bbc`。临时规则移除后，原保存文件摘要恢复。Networking 实际 remove、purge、重装均成功，每次都用原号查回 APPLIED；14 份历史文件及两份凭据核对一致。Main 的有效能力掩码仍为 0x1400，NNP 为 1。原六组件已升级两包，冷读入口为 `main-firewall-forwarding/six/parked-readback.py`。

测试还发现 iptables 1.8.11 无法显示这条原生 conntrack 状态表达式；实际规则由 libnftables 读回，旧工具的可读性继续处理。协议同时验证了错误命名空间、无对应转发规则、错误批准和批准前开关漂移。Rust ARM64 工作区 806 项通过、50 项忽略；Networking reactor 2260 项通过，启动归属 15 项通过。其余 provider/launcher 和完整维护矩阵继续，冻结 0/12。[原 Main、内核回执和维护](actual-installed-firewall-action-receipt-checkpoint.json)

## 2026-11-21 原六组件已升级公共目录修正，Parallels 保留一次失败的升级

原六组件 Root `39:6611544` 已升级到 Core `e14d06eae1` 和 Networking `e5422d2`。dpkg 为公共安装目录登记了 Core、Networking、Deployment、Wires 和 GWT 五个所有者。六包、SQLCipher、84 份保护文件以及注册表和锁已停机冷读。最新六组件入口是 `six-registered-directory-upgrade/parked-readback.py`。[正式包和原卷核对](actual-passive-addon-dpkg-rollback-checkpoint.json)

在原防火墙临时安装 `2050:2271189`，上一版 Core `92ea89ba15` 的 deb 先声明了公共目录。preinst 暂存原物理目录后，dpkg 遇到了由 Networking 登记、此时目标暂缺的链接，拒绝解包。随后 Networking 解包将逻辑路径变成了目录，原捕获的链接 inode 已不匹配。Core 留在 `5884bf5a6a`，Networking 为 `half-configured`。该 Root 已正常停机；原物理目录、原升级凭据、18 份防火墙记录和两份密钥文件均保留。冷读入口为 `firewall-exact-forwarding-chains/park-failed.py cold`。修正版将公共目录条目排到物理安装文件之后，另一个安装同版前任的 ARM64 Root 及原六组件升级均通过。前述失败现场仍按原围栏保留。

旧 Networking 会把站点自建的 `forward-kura-site` 链当作开启转发的请求。正式启动器对照中，旧包将 IPv4 和 IPv6 都设为 1，修正包保留 IPv4 为 0，并为实际使用 Kura 链的 IPv6 开启转发。两条规则均在内核读回。普通 Main 的受限转发授权和其余维护矩阵继续，冻结 0/12，external deferred 仅原三项。[双族内核值和原回执](actual-installed-firewall-action-receipt-checkpoint.json)

## 2026-11-21 正式 Core 保住最后一个 sibling 的安装链接，并从实际卸载中断续办

Core `92ea89ba15` 在 deb 中登记 `/opt/eclipse/kura`。独立 ARM64 安装 `2097163:7204836` 上，八个可选包全部移除并 purge 后，原链接和物理目录的 inode 保持一致，注册表准确保留站点末行。随后正常移除 Core 保留了应用数据及原凭据，显式 purge 清除普通保留目录，两份凭据仍一致。

同一 Root 按正式入口重装后，在原物理目录已改名时终止 postrm 清理进程。dpkg 返回 1，状态为 `half-installed`。沿原维护编号续办后进入 `config-files`，三份保留文件和两份凭据保持一致，dpkg 审计为空，现场正常停机。另有四项 Linux 路径替换及中断回归通过，测试提交为 `db963f956e`。原六组件仍实装 Core `5884bf5a6a`，六包及 84 份保护文件冷读一致。其余正式 Main、typed broker 和六组件维护继续，冻结 0/12。[发行件、正常卸载与中断回执](actual-passive-addon-dpkg-rollback-checkpoint.json)

## 2026-11-21 六组件升级保留站点注册表权限及八个 provider 续配

原 ARM64 六组件 Root `39:6611544` 已升级至 Core `5884bf5a6a`、Networking `448909b`、GWT `9de7a7b`、Deployment `ef50b67`、Wires `4f13cf1`，agent 保持 `5be64b7`。Core 复制保留了注册表的四行、0640 及属组 990，重建 framework 后旧锁不在，原 inode 无内核锁。随后四个 sibling 联合升级，注册表的 inode、权限、属组、时间和内容全部相同。六包、SQLCipher 和 84 份保护文件均已停机冷读。最新入口为 `sibling-registry-maintenance/parked-readback.py`。

12 仓的 13 个 deb 统一保留站点注册表的属组、访问 ACL 和末行。修复前，已登记或没有登记的组件在维护时都能把 0640 改成 0644。另以真实 dpkg 和仅提供依赖的 Core 验证八个可选包升级：锁争用使 Artemis 半配置，放锁后在原 Root 续配，全部八包通过 dpkg 校验，原 ACL 和 inode 保持。移除八包返回 0，但最后一个包的移除使逻辑安装链接消失，物理表残留 yofc-runtime；该现场已停止，未发起 purge。另冷读正式 Parallels Root 2050:2271189 的 dpkg 数据库，逻辑安装链接仅由 kura-networking 登记，已安装的 Core 没有登记它。对照实测表明，Core 声明共同目录可保住最后一个 sibling 移除时的链接，但 Core 自身的 postrm 此时也要能识别原物理目录。正式 Core 修复和最后一个 sibling 的完整卸载仍待验。冻结 0/12。[发行件、原六组件与八包中断记录](actual-passive-addon-dpkg-rollback-checkpoint.json)

## 2026-11-21 防火墙启动时从实际规则识别转发需求

正式启动器曾把注释里的 `-A forward-kura` 当作开启网关转发的请求。在只有回环接口的两个 ARM64 网络中，分别运行旧版和修正版的标准发行包启动器。两边均成功装入 IPv4 注释规则和 IPv6 转发链规则，站点文件内容一致。旧版把两族转发都由 0 设为 1；Networking `bd8ccdd` 保留 IPv4 为 0，仅将 IPv6 设为 1。两种 deb 共用相同修正，15 项测试通过，沿用 Core `5884bf5a6a`。此修正尚未装入两个停机保留现场；转发 broker 及其余维护矩阵继续，冻结 0/12。[发行件和实际内核结果](actual-installed-firewall-action-receipt-checkpoint.json)

## 2026-11-21 实装防火墙启动恢复和原编号双族续接

防火墙专用 Parallels Root `2050:2271189` 已升级至 Core `5884bf5a6a`、Networking `51c157c`。在其私有网络内建立一条会被 iptables-save 静默省略表达式的原生规则，再由正式 firewall.service 按标准配置启动。独立进程从内核重算双族规则和转发状态，匹配原回执，外部规则和原有 17 份文件内容均保留。Main 保持 disabled，现场正常停机，冷读入口为 `firewall-family-readback/installed-boot.py cold`。

发行包中的 worker（源码 `5e4be5d`）还验了两类中断。IPv4 回执落盘后，外部集合或同句柄规则的表达式改变，续办均在写 IPv6 前拒绝。现场恢复原值后，仅补做 IPv6，沿用各自原编号。两个 Networking 变体的恢复脚本一致。ARM64 离线工作区 806 项通过、50 项忽略，启动归属及恢复测试 14 项通过。

原六组件卷 `39:6611544` 实装 Core `88cc7f2ad0` 和 Networking `2a58770`，六包、SQLCipher 和 84 份保护文件已冷读。当前入口为 `firewall-native-boot-readback/parked-readback.py`。同表整体替换时保全被旧工具静默省略的原生表达式、普通 Main 的转发授权、其余 provider/launcher 安装维护继续。冻结 0/12，external deferred 仍为原三项。[发行件、启动回执和冷读](actual-installed-firewall-action-receipt-checkpoint.json)

## 2026-11-21 防火墙临时安装的正式 Main 正向回执

获准的独立 ARM64 Root `2050:2271189` 安装了 Core `857015576f` 和标准 Networking `2a2deca`。正式 Main 以 kurad 通过 OSGi 加入且随后移除仅限 `lo`、来源 `127.0.0.1/32` 的 TCP 19897 规则。每笔操作单独获得内核 ACK 和 APPLIED 回执。六张表的规则内容和原保存文件恢复，独立进程按原编号查回两份结果。Main disabled，临时安装正常停机并冷读。该结果补齐上一条记录中的正式 Main 防火墙正向写入。旧六组件卷和此前中断的新装 Root 仍各按原围栏续接。

安装现场的 iptables-save 和 ip6tables-save 对 mangle 返回 0，却没有输出这张表。另用隔离 ARM64 内核测得两份不同原生规则产生同一旧摘要。Networking `ce26379` 的两个构建包均在创建恢复意图前拒绝不完整输出，内核规则保持原样，13 项归属及恢复测试通过。此修正已打包，两个已停机的安装保留各自实装版本。原生规则的完整启动恢复、转发和 Main 剩余网络权限继续。冻结 0/12。[原操作、回执与冷读](actual-installed-firewall-action-receipt-checkpoint.json)

## 2026-11-21 正式 Main 的防火墙调用与安装中断

原六组件卷上的正式 Main 已完成配对并调用独立 worker。原文件含洪泛防护规则，内核没有原 Kura 链，防火墙服务为 disabled。批准前拒绝，发送端和 Root 均无发布意图。修正后的 Core `a87db8ac33` 与 Networking `180ea17` 已装回原卷，六包和 84 份文件内容经停机冷读。隔离 ARM64 另验了原 Java 生成的 43 条洪泛防护规则、双族变更和原编号回执。正式 Main 的防火墙正向写入仍待验。

独立 Parallels 新装 Root `2050:2265822` 在配置 brokers 时丢失 D-Bus，保留原编号和五份记录，已正常停机。当前候选 Core `857015576f` 在登记步骤前检查系统总线。Networking `2a2deca` 与 worker 源 `5ebb3c9` 还修复了待办仍在、回执目录缺失时的包维护放行。三个候选 deb 共 18 个维护调用通过，另验了旧版原回执的冷查询。候选包尚未装入原六组件卷。正式 RTC 写入和夹具账号已登录下载沿用原通过记录。冻结 0/12，external deferred 仍为原三项。[实装、候选与续接证据](actual-installed-firewall-action-receipt-checkpoint.json)

## 2026-11-21 原防火墙命令和内核回执

Agent `5be64b7` 与独立调用程序共用 Netfilter 发送和回执校验。ARM64 上四份真实拒绝回执只匹配各自的原命令和发送格式。独立程序以一笔事务将两个协议族的端口范围从 18000–18010 改为 18100–18110，保留另外 138 条规则。生成号在派发前变化时仅发送一次，内核拒绝，140 条规则保持原值。ARM64 工作区测试 806 项通过、50 项忽略。原六组件卷只升级 agent 后正常停机，84 份保护文件冷读一致。最新冷读入口为 `firewall-original-command-receipt/parked-readback.py`。正式 Main 到独立写服务的配对及原操作编号恢复继续，冻结 0/12。

## 2026-11-21 防火墙增量规则发布

Networking `8025d56` 修复了暂存失败后 Java 提前记入规则、下一次调用却跳过写入的问题。ARM64 上用前后两版发行包分别核对 IPv4 和 IPv6，修复后拒绝时内核写入为零，原模型仍可读。恢复暂存目录后同一条合法规则实际进入内核和保存文件，重复添加无额外写入，其他软件的规则链保留。标准包和 firewall-only 均构建通过，2245 项测试通过。原六组件卷只升级标准 Networking，Root、84 份保护文件的内容和元数据冷读一致。最新停机卷冷读入口为 `firewall-java-original-model/parked-readback.py`。正式 Main 对独立写服务的调用、原操作编号持久回执和恢复继续，冻结 0/12。

## 2026-11-21 独立防火墙规则归属核对

Agent `86bfc22` 与独立调用方共用内核规则匹配。ARM64 现场 140 条中准确定位原 134 个句柄，另外 6 条保留。等价条目有两个位置时拒绝，即使只匹配到一条，带有原 Java 模型未声明的现场标注也拒绝。两份历史语料的规则分类、原操作绑定、批次内容和发送字节摘要均与已实装版本一致。原六组件卷已从 `a1f1ebe` 升级至 `86bfc22`，Main 和 agent 均为 disabled，84 份保护文件内容和元数据一致。ARM64 离线工作区测试 806 项通过、50 项忽略。最新冷读入口为 `firewall-rule-binding/parked-readback.py`。正式 Main 的受限写服务及原编号结算继续。冻结 0/12。[原句柄、源码发行件与兼容性核对](actual-installed-firewall-action-receipt-checkpoint.json)

## 2026-11-21 双族六表原子提交和启动操作锁

Agent 共用原规则编译库。在 ARM64 隔离网络中，134 条 Java 规则一次提交成功，六表同批的最后一条被拒时完整规则集保持。原六组件 Root 先升级共享库版 2096a04，再升级启动锁修正版 a1f1ebe。写者已结束但子进程仍保有描述符时，下一阶段现在可接回原操作。两次停机均冷读六包、SQLCipher 及 84 份保护文件。最终 ARM64 离线测试通过 802 项，显式忽略 50 项。最新入口为 `startup-execution-release/parked-readback.py`。正式 Main 的受限写服务、同编号结果恢复及其余组件维护继续，冻结 0/12。[发行件和验证记录](actual-installed-firewall-action-receipt-checkpoint.json)

## 2026-11-21 正式 Main 在清链前核验候选规则

Networking `cecf20f` 用发行系统自带的 IPv4、IPv6 解析器检查暂存文件。无效的附加规则原先会改写已运行的链，新包在隔离 ARM64 上拒绝后保留规则、磁盘文件和可读模型。原六组件卷标准版升级后，正式 Main 调用两次解析器均返回 2，六张表一致。原 Root 正常停机，最新冷读入口是 `firewall-native-policy-compiler/parked-readback.py`。跨表原子写入、已配对独立写服务和原操作结果恢复继续，冻结 0/12。[发行件、解析器和停机记录](actual-installed-firewall-action-receipt-checkpoint.json)

## 2026-11-21 原配置服务按地址族一次发布开放端口、转发及 NAT

Networking bd5fb9b 将同一次 ConfigAdmin 更新的三类规则一起准备，保留独立维护的自动 NAT。ARM64 上真实旧包面对只读且关闭的转发开关，IPv4/IPv6 各发布四次，改写规则文件并发送一次变更事件。新包两族均在第一条内核命令前拒绝，原模型、表、规则文件保持，事件数为零。转发已开时，两族均一次发布、一次通知，原有其他软件的规则链保留。2240 项 Networking 测试通过。

原六组件卷原位升级标准 Networking。正式 Main 的 OSGi 实际解析到 iptables 包版本 1.6.0，双族配置回调在只读夹具上均无部分提交。六包、SQLCipher 和 84 个原内容摘要已冷读。最新停机入口 firewall-configuration-publication/parked-readback.py。另在隔离网络命名空间量得 iptables-restore 的分表提交可留下先前的修改；nft 单批两张测试表在拒绝时均保留。受限写入 broker、原编号结算和 Main 整体撤权继续，冻结 0/12。[内核对照、发行件与原卷](actual-installed-firewall-action-receipt-checkpoint.json)

## 2026-11-21 正式 Main 在调用线程撤去 NET_ADMIN 后读取双族防火墙表

原 ARM64 六组件 Root 联合升级 Core 002b0b545e 与 Networking 6fa4232。正式 Main 通过独立 socket 对每一族检查 filter、nat、mangle。读取服务配对已加载的 Main 并验证调用进程，只接受族别、返回状态。测试线程及其普通子进程的有效能力由 0x1400 降为 0x0400 后，两族查询均成功。仅读取服务使用 0x1000，Main 主线程仍为 0x1400。六张表前后摘要相同，六包、SQLCipher 和 84 份文件冷读通过，84 份文件的内容与升级前一致。Core 后续 f956cc3314 只修订组装测试，最新停机入口 main-firewall-table-reader/parked-readback.py。规则写入、其他 provider/launcher 与整体撤权继续，冻结 0/12。[实装进程、发行件和测试范围](actual-installed-firewall-action-receipt-checkpoint.json)

## 2026-11-21 firewall-only 在 dpkg 撤销时保留启用链接

原 ARM64 六组件卷上，旧版 7a57c4f 碰到替换包 preinst 拒绝后仍显示 installed，但 firewall.service 由 enabled 变为 disabled。Networking 40a6a79 将注销移到成功移除之后。三次真实的 abort-upgrade、abort-deconfigure、abort-remove 均保留原启用链接、注册文件的 inode 与顺序，dpkg audit 为空。随后成功 remove、purge 并装回标准 Networking。原卷正常停机，六包、SQLCipher 和 84 份保护文件已冷读；最新停机入口 firewall-only-dpkg-unwind/parked-readback.py。Main 的内核防火墙权限与其他 provider/launcher 组合继续，冻结 0/12。[回调及发行件记录](actual-installed-firewall-action-receipt-checkpoint.json)

## 2026-11-21 Networking 的 dpkg 事务撤销与真实移除

在原六组件 Root 上，一个故意返回 42 的测试包触发了真实 dpkg 升级撤销。旧标准版 96cc512 的 postinst 也报错，原包从 installed 降为 unpacked。Networking 7a57c4f 原位升级后分别触发 abort-upgrade、abort-deconfigure 和 abort-remove。三次测试均由 dpkg 拒绝候选，原标准包已配置，dpkg audit 为空；原属性和 systemd drop-in 内容一致，组件注册顺序文件内容、inode 和时间戳精确，84 份受保护文件相符。

真实移除成功后，postrm 删除了 Networking 的注册行，其他行保持原序；再次安装同版并结算原移除记录后六包、SQLCipher 和 84 份文件已冷读。firewall-only 新包的 abort-upgrade 和 abort-deconfigure 两个脚本分支另在 ARM64 直接核实。最新停机入口 networking-dpkg-unwind/parked-readback.py。Main 与 agent 在本次包回滚中均停用。冻结 0/12，旧内核防火墙权限及其余 provider/launcher 矩阵继续。[dpkg 回调、发行件与冷读](actual-installed-firewall-action-receipt-checkpoint.json)

## 2026-11-21 标准 Networking 切换 firewall-only 后中断续配，再恢复标准版

原六组件 Root 有 9 份快照：最新配置已有防火墙服务，原始 snapshot_0.xml 未配置。此前 firewall-only 会把跨包换型当作首次安装，给原始快照补写默认值。Networking 96cc512 使用既有 Root 能力记录区分首次引导与保留式安装；preinst 和 postinst 使用同一判定。20 项包测试通过，真实 deb 内读取器在 ARM64 验证了初装、两种保留记录和记录损坏的拒绝。

移除标准版，解包 firewall-only 后正常关机。冷卷上 firewall-only 仍为 unpacked，84 份文件精确，Main 包配对检查拒绝这个中间状态。原地配置后，正式 Main 从 firewall-only 的 OSGi provider 读回 IPv4 和 IPv6 配置，归属读取器在空能力集下服务。对活跃 provider 执行 remove 正常停 Main，随后 purge 并装回标准版。原属性内容、9 份快照完整指纹保持，六包与 SQLCipher 冷读相符。最新续接入口 firewall-variant-reader/parked-readback.py。独立防火墙内核路径仍需 NET_ADMIN，冻结 0/12。[发行件、包状态和 Main 观测](actual-installed-firewall-action-receipt-checkpoint.json)

## 2026-11-21 正式 Main 读取系统防火墙及网络归属

原六组件 Root 一次 dpkg 调用联合升级 Core 67c6a50045 与 Networking 6d4cc37，按原指纹暂存和归还两个可选 SQLite 测试 JAR。agent 停用，/etc/kura 仍为 root:root 0700。新的 systemd 读取服务以 UID 0、空能力集运行，按包的单元摘要和内核 pidfd 核对正式 Main，仅对两个固定标记返回一次最新存在性。正式 Main 及 NM 读取路径实际得到缺失，三次查询共 33 ms。IPv4/IPv6 继续进入转发预检，各自在执行任何内核命令前拒绝不可写的关闭开关。

同一套 Networking 发行测试 2238 项通过。ARM64 读取器实测悬空标记出现及移除、非文件标记和父目录缺失。正常停机后读取服务退出、socket 消失。六包、SQLCipher 及 84 份文件冷读相符。续接入口 main-ownership-markers/parked-readback.py。旧内核读表和规则发布仍需 NET_ADMIN；正式 Main 撤权及其余包维护矩阵继续，冻结 0/12。[实装读数及发行件](actual-installed-firewall-action-receipt-checkpoint.json)

## 2026-11-21 独立 Kura 防火墙转发与归属检查

Networking 3494a9e 的标准包及 firewall-only 包通过 2238 项测试。在隔离 ARM64 内核中，当 IPv4 或 IPv6 转发处于关闭且开关只读时，该族在发出第一条内核命令前拒绝，已有三张表和保存文件保持原样。转发已开启时，两族均可在开关只读的条件下发布及撤销 NAT。清空全部规则由五次发布减为一次，并保留其他软件的规则链。

原六组件原位升级一次标准包。正式 Main 在更早的所有权检查收到 FIREWALL_OWNERSHIP_UNREADABLE。root 观察到标记缺失，kurad 遍历 root 所有的 0700 目录 /etc/kura 时收到 EACCES。网络和防火墙两处均需能在保留原目录权限的条件下判断归属。原卷已正常停机，六包、SQLCipher 及 84 份文件冷读相符。续接入口 firewall-forwarding-admission/parked-readback.py。冻结 0/12。[实装及内核对照](actual-installed-firewall-action-receipt-checkpoint.json)

## 2026-11-21 独立运行时的防火墙读表权限

在原六组件 Root、agent 停用的正式 Main 中，Networking 已安装的 IPv4 和 IPv6 实现分别通过其真实注册的 PrivilegedExecutorService 执行读表前置检查。测试线程持有 0x1400 时，两族均通过。只撤去该线程的 NET_ADMIN 后，直接子进程和执行服务创建的子进程均为 0x0400，两个前置检查均返回 IO_ERROR。Main 主线程保持 0x1400。测量前后两族的 filter、nat、mangle 六张表逐项摘要相同。

独立运行 Kura 的旧防火墙路径仍依赖 NET_ADMIN。迁移需覆盖读表、规则发布及已有所有权交接。原六组件正常停机；续接入口 main-net-admin-preflight/parked-readback.py，冷读六包、SQLCipher 和 84 份文件。冻结 0/12。[调用链与现场读数](actual-installed-firewall-action-receipt-checkpoint.json)

## 2026-11-21 防火墙确认及回滚回包丢失后的原编号查询

原六组件安装共同升级 GWT 8870575 与 Networking 9a5007a。正式 Main 装载两个发行包的类，用内存传输端分别模拟确认和回滚的畸形回包。两个场景均只派发一次操作，适配器报告该次派发的原始关联编号。服务端按准备编号查询三次，前两次旧状态保持 UNKNOWN 和禁止操作，第三次分别取得 COMMITTED 和 ROLLED_BACK。真实 agent 未启动，内核防火墙未改动。GWT 两个模块合计 535 项、Networking 构建 2235 项通过。

六组件在原 Root 正常停机，冷读核对六个包、SQLCipher 和 84 份文件。续接入口为 firewall-action-receipt/parked-readback.py。真实防火墙 actor 与完整权限及包维护矩阵继续按原台账验收，冻结 0/12。[发行与实装记录](actual-installed-firewall-action-receipt-checkpoint.json)

## 2026-11-21 已登记 SQLCipher 部署包跨 Deployment、原生库共同维护

原六组件安装通过公开 Deployment 服务登记 SQLCipher 部署包一次。包内 provider 读到原明文和加密业务行后停机。共同 remove、purge Deployment 和 libsqlcipher1，只解包 Deployment。其安装器在原 Root 留下配对生成器和 Main 运行时 mask。正常关机后的只读挂载证实 Deployment 待配置，libsqlcipher1 未安装，90 个卷内路径指纹吻合。

在原 Root 恢复后解包原生库并配置两包。新启动的正式 Main 自动恢复原登记，两个数据库由部署包内的 provider 读回，SQLCipher 版本及加密页完整性均通过。注册表、归档和首次派发凭据的完整指纹跨停机保持。退役测试 DP 后，原业务 DP 注册行和历史旧归档保留，两个原数据库内容摘要不变，六包和 84 份文件已冷读。最新版停机入口为 registered-sqlcipher-joint-maintenance/parked-readback.py。冻结 0/12。[实装与恢复记录](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-21 独立启动器旧 PID 与正式 Main 接管

同一六组件 Root 上，用已退出子进程的编号构造 root 所有的常规旧 PID 文件。旧包首次 systemctl start 返回 1，随后服务自动起来。Core 8b275f0ece 在发行清单中配对新的 PID 准备程序，只向这个短时进程授予 CHOWN。实际 dpkg 原位升级后，正式服务首调返回 0，旧 PID 文件保持 inode 并由 kurad 接管，文件中的新编号和 systemd 相同。JVM 取得 writer lease，能力掩码 0x1400，管理页返回 200，重启计数为 0。正常停机后 PID 文件消失。原维护回执相邻，原八份快照和获准改密后的第九份均保留，最新冷读入口 main-pid-handover/parked-readback.py。

六包、SQLCipher 和 84 份文件已冷读。升级期间暂存的两个测试用可选 SQLCipher JAR 已按原摘要归位。其余 provider、启动模式、包维护中断与 12 父项仍按原台账续接，冻结 0/12。[实装与原版本对照](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-21 原六组件管理页登录与日志下载

正式 Main 的 GWT 页面在已获准的原夹具账号上完成首次改密，使用新密码再次登录后，经 Device → System Logs → Download 取回 Kura_Logs.zip。隔离安装的 80 端口提供该页面，宿主只监听回环；443 未监听。响应为 200、application/zip，Content-Length 与 3,666,808 字节实际下载一致，完成 cookie 已收到。99 个 ZIP 条目 CRC 全过，新旧目录三份样本逐项匹配 SHA-256，另有两份非空 journal 和三份 Kura 活动日志。

原八份快照保持精确。新增快照中仍只有原管理员，其首次改密标志已清除，另外 15 个配置条目只更新了 service.pid。冷读共 84 份文件，六组件已正常停机。续接入口 http-download-session/parked-readback.py。旧 PID 导致首次服务启动失败、其他 provider/launcher 维护模式继续。冻结 0/12。[浏览器、快照与冷读记录](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-21 正式 Main 在配置生成途中停机后恢复

原六组件卷沿用 Core b1b8060be2。正式启动器生成 Equinox 配置时执行 systemd 停机。当时 PID 文件为空，JVM 尚未启动；424 字节临时配置在停机前后 inode 和摘要相同。单元正常退出，PID 文件消失，写入登记未变。原地启动一次后临时文件消失，完整配置为 22,494 字节。JVM 进程号与 PID 文件相符，其写入登记匹配进程启动时刻和锁文件，独占取锁探测证实内核锁已被持有。再次正常停机后冷读核对六个包、SQLCipher 和 83 份文件，续接入口为 main-launch-cancel/parked-readback.py。

这次覆盖配置生成中的正常服务中断。独立启动器遗留 PID、其余包维护和已登录日志下载仍待验。原卷最新快照只有出厂管理员，首次登录要求改密。冻结 0/12。[实装检查点](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-21 正式 Main 撤去 DAC_OVERRIDE

Core b1b8060be2 在原六组件 Root 原位升级。新包先核对旧单元与原包配对，升级后冷读的维护链把两份实际发行清单直接相连。正式单元在每次 fork 前准备原 /run/kura.pid；固定 tmpfiles 规则只为新文件设置 kurad:kurad 和 0600。setpriv 将准备进程的 capability bounding set 限定为 CHOWN 和 FOWNER。原 Java 调用方及 systemd 继续读取同一普通 PID 文件。

两次独立启动的正式 JVM 均为 kurad，CapInh、CapPrm、CapEff、CapBnd、CapAmb 均为 0x1400，NoNewPrivs=1。每次均核对 Java 读取的 PID、systemd MainPID、写者 lease、原业务 DP 内容和 agent bridge；每次正常停止后 PID 文件消失。六包、SQLCipher 和 83 份文件已冷读，续接入口 main-without-dac-override/parked-readback.py。独立启动器的遗留 PID 状态、启动准备中断、已登录 HTTP 下载及其余维护矩阵仍待验。冻结 0/12。[发行包、进程和维护回执](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-21 发行包日志轮换、JVM 诊断及 GWT 下载

Core 5aa5aad4f1 与 GWT 6fd074c 已在原六组件 Root 实际升级。安装器创建 kurad 私有归档目录，Linux 默认 Log4j 在其中保存应用和审计归档。已安装的前台 JVM 以 0x1400 运行，使用发行包原 XML 和私有日志挂载，两个活动文件均保持 inode，旧标记各出现在一份 gzip，新标记继续写入。正式 Main 读回包中的堆转储和错误日志参数。隔离 ARM64 JDK 21 用三个启动器的实际参数分别产生堆转储、错误报告，两个调试入口也写出 GC 日志。

GWT 服务端随 /var/log 收集固定格式的 Kura 轮换文件，逐级使用已打开的目录且拒绝跟随链接。正式 Main 从已激活的 web2 bundle 加载 LogServlet，组装来自旧目录及新目录的三条 ZIP 记录，逐项 SHA-256 核对。此项覆盖服务端 ZIP 生成。当前安装的已登录 HTTP 下载、PID 生命周期及正式单元撤去 DAC_OVERRIDE 继续待验。六组件已正常停机，83 份文件已冷读；续接入口 packaged-main-diagnostics/download/parked-readback.py。冻结 0/12。[实装检查点](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-21 正式 Main 的首份日志与后台 PID

Core 910cb9bd9a 在发行包配置阶段为 kurad 创建缺少的应用、审计和控制台日志。实际 ARM64 安装脚本首建三份 0600 文件，零能力账号均可续写；已有站点文件、别名和目标暂时缺失的别名均保留。原六组件卷完成真实 dpkg 升级，正式 Main 启动前，三份旧日志的完整指纹与升级前一致。

后台启动器改为先打开控制台日志，再打开 PID 文件及派生 Java。旧脚本在控制台被拒时覆盖了原 PID。新包在原卷遇到只读日志时以 EROFS 退出，未留下 PID、JVM 或新增 writer；正常启动时，两条控制台输出流、真实 PID、writer lease、原业务 DP 和 agent bridge 已核对。六组件已停机，83 份文件及 SQLCipher 已复核。停机入口为 packaged-main-log-layout/parked-readback.py。归档目录的发行接入、JVM 诊断文件及正式单元撤权仍待验。冻结 0/12。[实装检查点](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-21 正式 Main 向隔离虚拟 RTC 写入、六组件卷日志归档

在现有 Parallels 内获准另建 ARM64 安装。发行包安装的 Main 以 kurad 启动，通过注册的 ClockService 将系统时间同步到专属虚拟 DS1307。初始设备读数为 EINVAL；唯一一次回调后，设备秒值与 actor 写入时刻相符，发送端和 Root 的同一编号均为 APPLIED，errno 为 0。新安装已正常停机并保留围栏。续接入口为 parallels-registered-main-rtc/parked-readback.py。原 Parallels 组件的 21 份文件、3 个操作和原围栏已复核。物理硬件仍在原延期项内。[时钟实装检查点](actual-registered-main-clock-and-purge-checkpoint.json)

六组件原卷还实测了第三种日志布局。仅将归档写到专属目录，应用和审计两个活动文件均保持原路径和 inode，各自压缩旧内容并继续写入。六组件停机入口更新为 direct-log-archive-only/parked-readback.py。正式包首次建日志、诊断输出、历史归档下载和正式单元撤权仍待验。冻结 0/12。[六组件实装记录](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-21 整个前台 JVM 撤去 DAC_OVERRIDE 后的日志轮换

六组件原 Root 上，两次使用已安装的前台脚本、原凭据和 writer lease，整个 JVM 的有效能力为 0x1400，原业务 DP 均为 ACTIVE。两种私有挂载分别模拟 Debian 默认目录及 Docker 已用的 Kura 专属目录。前者的应用和审计日志都可续写，但强制轮换后 inode 保持，压缩归档为零；后者两条日志均完成压缩并换新 inode，旧路径可读到轮换后的新行。原宿主三个日志完整指纹相同。六包已正常停机，83 份文件完成只读复核。当前入口为 direct-main-log-permissions/parked-readback.py。正式单元整体撤权、历史日志升级迁移和完整维护仍待验。冻结 0/12。[实装对照](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-21 正式 Main 的文件权限与启动 PID

原六组件 Root 上，正式 Main 的独立线程撤去 DAC_OVERRIDE 后，原业务 DP 可读，typed journal 导出 23,647 字节。ICMP actor 返回不可达，本机 root 直接 ping 也丢包。Main 主线程继续持有原权限。原启动器无法写 PID 时仍会派生 JVM；Core 7f91ef9024 调整了后台及分离启动顺序。新包原位升级后，两个入口在 PID 被拒绝时均无 Java 进程、无新 writer，正常 Main 的实际 PID、writer lease、业务 DP 和 agent bridge 已核对。启动所需的 PID 与日志所有权仍待收敛。当前停机核对入口为 launcher-pid-admission/parked-readback.py。冻结 0/12。[实装检查点](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-21 Deployment 同版本归档替换

原六组件卷上，旧版两条安装路径遇到跨文件系统复制失败时均丢失原归档，云端通过目录别名安装还会在返回成功后删除归档。Deployment 15c50ca 通过 240 项测试，同样条件下三次均保留原注册行、文件内容和 inode。故障对照使用实装 bundle、独立普通用户 JVM 和替代的 DeploymentAdmin；随后正式 Main 通过实际 Felix 读回原业务资源。六包正常停机，83 份文件已核对。当前只读入口为 deployment-archive-publication/parked-readback.py。冻结 0/12。[实装记录](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-21 中断普通命令实例及工作目录实测

六组件原 Root 的正式 Main 通过 OSGi 启动普通命令。确认 worker 实例、子进程及其 cgroup 后，仅停止这个实例。调用收到退出码 1、空 stdout 和 EXECUTOR_RESULT_UNAVAILABLE。原子进程及 cgroup 退出，Main 和监听 socket 的 InvocationID 保持。下一条独立请求以 UID 1000 成功执行。

对于 Main 的私有数据目录，普通命令能相对读取已知文件，Python 的 getcwd() 实际返回 EACCES。另建的站点测试目录和子目录归 kurad:kura，模式 0710，样本归同一用户组、模式 0440。正式 Main 经同一 worker 得到完整工作路径，按相对和绝对路径均读到同一内容，父目录和子目录都拒绝列举。两阶段都已正常停机。私有目录的透明绝对路径兼容及其他命令账号继续待验。最新冷读入口为 command-site-directory/parked-readback.py，核对六包、SQLCipher 和 83 份文件。冻结 0/12。[实例、目录及停机证据](actual-core-command-worker-lifecycle-checkpoint.json)

## 2026-11-21 Linux ARM64 全量 Core/OSGi 与原卷升级

以 ba84bdcfb4 在隔离 Debian ARM64、非 root、JDK 21 下运行完整 osgi-it 构建。同一六组件卷先安装 cbaadd1984，随后原地升级到与全量构建相同的 ba84bdcfb4。共 82 模块，2151 项通过、2 项跳过。修正独立解压进程的类文件及目录权限，并修正 TLS 显式本地地址被远端地址覆盖。正式 Main 上实际 TLS 客户端与服务端均见 127.0.0.2 源地址，普通命令用户 UID 1000 解压出正确内容。

Equinox 异步注册期间曾在生产代码复现 ConcurrentModificationException。修正版在同一个注册锁下复制 PID 集合，再查询 provider。新版正式 Main 上实测注册并注销 64 次，另一线程完成 32 次枚举，早先取得的 PID 集合在注销后仍保持原值且不可修改。已正常停机，续接从 config-pid-snapshot-arm64/parked-readback.py 核对六包和 83 份文件。OrbStack 只有共享 rtc0，Parallels 原组件围栏和 21 份文件仍匹配。正式 Main 的正向 RTC 写继续待验。停机时另按实装 deb 的八份 broker 清单核对卷上 102 个文件及容器 /usr 中 105 个文件，其中包括 26 个 systemd 单元。摘要、root 所有权和组及其他用户无写权限均通过。

该版本的系统 SQLCipher 包声明 activate-noawait ldconfig，控制归档中没有包自身的维护脚本。原卷实测暂缓触发器后，库为 installed，libc-bin 为 triggers-pending。停机冷读原缓存及 83 份文件，重新启动后仅处理该触发器，动态链接器再次解析到原版库。链接器缓存换了 inode，内容摘要相同，dpkg 审计为空。本次 Debian trixie ARM64 软件源索引只列出 4.6.1-2，跨版本升级仍待验。冻结 0/12。[完整构建与实装检查点](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-21 系统 SQLCipher 库 purge 与解包后续配

六组件原 Root 停服后实际 purge libsqlcipher1，链接器缓存和已安装库文件均已核对。重新解包原 4.6.1-2 后，在 configure 之前正常停机；卷上冷读确认该包为 unpacked，六组件为 installed，原 83 份文件精确匹配。同 Root 续配并执行 libc-bin 触发器，dpkg 审计为空。正式 Main 经原夹具 provider 读回两个数据库和加密页。首次观察在取服务前等待 lazy bundle 进入 ACTIVE 超时；复测证实 STARTING 时已有服务引用，经正常入口即可读回，全程无需显式启动 bundle。停机只读入口更新为 sqlcipher-native-maintenance/parked-readback.py。不同系统库版本升级和维护脚本内部被中断仍待验。冻结 0/12。[dpkg、冷卷与正式 Main 记录](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-21 加密 SQLite provider 的发行包与维护

Core 0ffa1325cd 生成可选 Linux ARM64 部署包，在一个 DP 内交付 provider 和按原 Xerial 源码编译的 SQLCipher JNI。两次相同构建的 DP 逐字节一致。六组件原 Root 上，正式 Main 通过部署服务安装、升级，随后冷启动恢复。三个阶段均由 DP 所属 provider 读回原明文库与加密库业务行，加密页完整性检查通过。首次升级观察时框架已发布 DeploymentAdmin，原 DP 仍在异步恢复，等到指定版本后才派发一次升级。移除后两个 bundle 消失，只剩原业务 DP 登记。停机只读追溯到未登记的旧版本归档 294,611 字节，保留待核查历史用途。当前只读入口为 sqlite-provider-distribution/parked-readback.py。系统 SQLCipher 包的移除、purge 与中断恢复仍待验。冻结 0/12。[发行、实装与归档记录](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-21 正式 Main 和直接启动 JVM 跨包维护

六组件原 Root 上，直接启动的 JVM 持有写者锁时，已安装 prerm 和传入 preinst 均拒绝重装，原进程、密钥及历史保留。首版依赖读取另一 UID 的 FD，在实装中被拒。最终 Core 3e1bc49711 使用现有 Root 租约登记识别正式 Main；停服后仍须取得内核独占锁。正式 Main 运行中用 dpkg 重装成功，手动再启动后，原业务 DP 和 agent bridge 均 ACTIVE。六组件已正常停机，续接入口为 package-writer-quiescence/parked-readback.py，复核六包和 83 份文件。[实装与续接证据](actual-native-wires-retained-reinstall-checkpoint.json)

Parallels 原组件 Root 上，专属虚拟 modem 拔出和重接后，发行包两次拒绝旧 NM 对象。同一 USB 端口重新出现时，内核 inode、NM 和 MM 对象全部变更，新对象可以只读解析。设备、路由和服务调用实例已复核恢复。原发送端 UNKNOWN 继续保留，正式 Main 已派发的 modem 操作跨双包 purge 仍待验。冻结 0/12。[两次设备试验及原始结果](actual-modem-sender-restart-and-upgrade-checkpoint.json)

## 2026-11-21 Equinox 启动配置中断恢复

旧版前台、后台、调试启动器会在模板缺失时截断上次的 config.ini 并继续。原六包 Root 第一次升级后，三个入口实测均保留旧文件并在申请 writer 前退出。正式 Main 和两个直接入口均读到 ACTIVE 业务 DP 与配对 agent bridge，但进程组中断可留下 17 字节暂存文件。最终 Core 12518cff92 把暂存限制为一个文件，用同目录的非阻塞锁协调三个入口。实装中，后台生成到 1 MiB 时，前台启动被拒绝，原 config.ini 和 writer 登记保持。随后中断后台，实际 --detached --debug 启动回收暂存，发布与原来逐字节相同的 22,579 字节配置，取得原 Root 的 writer lease 并恢复两个 ACTIVE bundle。六包与 83 份保留文件已停机只读复核。新入口为 launcher-config-recovery/parked-readback.py。冻结 0/12。[发行与现场记录](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-21 Watchdog 的设备交接与原六包升级

旧包对 Linux softdog 的两种 nowayout 设置均允许进入维护。新版在系统明确报告不可停用时拒绝维护。Parallels 上用同一 ARM64 deb 的 Java provider 和 JNA，以 kurad 运行，读到 300 秒超时并观察到两次保活。Java 服务停用并关 FD 后，内核仍在计时。发行包的维护程序接手续喂，再用 Magic Close 使设备回到 inactive。原六组件卷另以 dpkg 升级 Core，正式 Main 读到 watchdog 为 DISABLED，原业务 DP 再次 ACTIVE，正常停机后六包及 83 份文件只读核对。原卷下一次从 watchdog-softdog-maintenance/parked-readback.py 续接。正式 Main 持有真实设备跨包中断的场景仍待验。冻结 0/12。[ARM64 设备与实装证据](actual-arm64-watchdog-maintenance-admission-checkpoint.json)

## 2026-11-21 原 Parallels RTC 独占打开竞争

原组件 Root 2050:2 的独立进程持续只读占用虚拟 DS1307。实际 ClockService、时钟 actor 与 systemd relay 对一次请求完成 prepare，交付 FD 时遇到 EBUSY。Root 与 Java 都保存了同编号 FAILED / OS_ERROR / errno 16。解除占用后可以只读打开，观察窗口内维持原三笔操作，设备日期仍未设置。停用后原 15 份及新增六份文件已核对，八个时钟单元 inactive。此处取代先前两笔操作和旧停机围栏作为本 VM 的续接基线。正式包的 Main 布局检查已在原 Root 只读执行，拒绝无配套清单的静态 Main。冻结 0/12。[当前围栏、证据及只读入口](actual-registered-main-clock-and-purge-checkpoint.json)

## 2026-11-21 Deployment 注册表部分写入

同一 ARM64 六包 Root 先后安装的两个版本在相同 1024 字节进程文件上限下各处理一份合成注册表。旧版返回成功并把 5963 字节截成 1024 字节。新版返回 I/O 失败，原摘要、长度及 inode 完整。两个 JVM 分别加载已安装的本地和云端 bundle，并发新增 60 条后，原有 180 条无关记录仍在。正式 Main 从原登记读出原业务 DP 的 ACTIVE 资源，两份实际 bundle 的注册表类摘要均匹配发行包。六包正常停机，83 份文件只读复核。冻结 0/12。[实装记录及下次只读入口](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-21 原业务 DP 跨 Deployment 移除、purge 和中断重装

六组件正式 Main 通过 DeploymentAgentService 一次安装合成业务 DP，收到成功事件，从 ACTIVE bundle 读出资源。Deployment 包普通移除后进入 config-files，purge 后包缺席。以余下五包启动正式 Main，部署服务缺席，Felix 未列出该 DP，磁盘上的原注册表和归档仍在。重装同版 deb 时在 postinst 等待注册锁处中断，dpkg 进入 half-configured。正常停机、以新的 systemd PID 1 启动后，原 Root、Main 围栏、FileMove 维护文件、注册表及归档均匹配。直接续配，第三次正式 Main 从原登记自动恢复同版本 ACTIVE bundle，资源读回成功。注册表和 DP 的内容、inode 与元数据在各阶段一致。六包、83 份文件已停机只读复核。冻结 0/12。[三次 Main 与续接入口](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-21 PKCS12 文件发布中断、原业务钥和数字 UID

Core `6621e1fa64` 将文件密钥库的保存改为同文件系统暂存并原子发布。ARM64 上在旧库替换前、首次建库发布后分别终止独立写进程，冷读者均能用原口令签名，下一写者收回暂存。另对真实 PKCS12 序列化施加单文件大小限制，原文件、元数据及签名钥保持。原六组件正式 Main 拒绝已锁文件，释放后保存一个新的受信任证书。冷启动以及再次原地升级到 `8a70d2e518` 后，均能读回该证书并用原业务钥验签。发行包支持只有数字 UID 的写者。JDK 21 密钥库模块 94 项通过，完整 82 模块 2148 项通过、2 项跳过。夹具正常停机，按最新回执只读核对六包与 72 份文件。跨口令记录与库文件的随机改密中断、网关包装钥跨代、其他 provider/launcher 与维护矩阵继续待验。冻结 0/12。[实装和续接入口](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-21 虚拟 GSM 设备通过 modem broker 的目标配对

在原 Parallels 组件 Root 上用只匹配测试序列号的临时 udev 规则声明主 AT 端口。ModemManager 的 generic 插件枚举出虚拟 modem，NetworkManager 报告 ttyACM0 为 disconnected GSM。首次运行发行包的只读目标核验时，两个守护进程的 systemd 定义均待重载，核验拒绝。核对磁盘单元和二进制归属及校验和后正常重载管理器定义，原进程及 InvocationID 保持。再次接入时，同一发行包的 target() 两次核对真实系统总线、原包、设备路径及 sysfs inode 通过。临时规则和设备已撤，原时钟围栏、模块、RTC、网络和默认路由均复核。此次没有派发 reset，正式 Main 的已派发 modem 跨双包 purge 恢复继续待验。冻结 0/12。[目标核验与复原记录](actual-modem-sender-restart-and-upgrade-checkpoint.json)

## 2026-11-21 PKCS12 业务私钥跨 Core 升级和冷启动

Core `55ce9dfec7` 修复通用配置服务更新只读属性映射时的异常。旧 Main 通过正式工厂在 PKCS12 生成一对 RSA 业务钥并验证签名，随后在口令属性加密阶段拒绝更新。旧源码的定向用例在同一方法报错。六组件原地升级后，正式 Main 用原口令和原证书验证签名，以只读映射更新同一工厂的口令，再次验证签名并确认旧口令被拒绝。第三次 Main 启动仍用原证书验签。正常停机后 59 份文件及六包完成只读核对。配置模块 301 项、完整 Core/OSGi 82 模块 2142 项通过、2 项跳过。文件库发布中断及包装钥跨代维护仍待验证。冻结 0/12。[实装与测试回执](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-21 SQLite 加密配置与正式 Main 冷启动

Core `f6c57008b7` 在打开持久库前核对原生 SQLite 引擎支持的加密和换钥指令。Git 指定的 Xerial 驱动在 ARM64 请求加密时拒绝操作，业务文件和凭据记录均未创建。SQLite 模块 83 项通过。同一六组件 Root 的正式 Main 加载测试用 SQLCipher 4.6.1 驱动后，实际通过配置工厂创建加密库、写入一行业务数据并换钥。第二次 Main 冷启动读回同一行，通过加密页完整性检查，两次均读回此前普通 SQLite 库的原行。正常停机后只读核对六包、SQLCipher 库包与 40 份文件。加密版 JDBC 的正式交付及网关包装钥轮换仍待验。冻结 0/12。[构建输入、两次 Main 与停机回执](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-21 SQLite 可选 provider 安装、升级、卸载与原文件重开

Core 源码 `2bc1de1a76` 的 SQLite provider 保留打开成功后的数据文件。保存凭据和换钥失败时直接返回错误。缺省配置保留不可读文件，主动启用重建的旧配置仍可在没有候选加密钥且 SQLite 明确报告文件格式错误时重建。82 项模块测试通过，包括真实 SQLite 文件打开成功、随后保存凭据失败，恢复后读回原行。在同一 ARM64 六组件 Main 上实际安装 JDBC 和旧 provider、创建一次工厂配置、写入一行。原地更新 provider、卸载并从相同 bundle location 重装后，原服务重新注册并分别读回同一行。该验证采用可选 OSGi 安装方式，持久库使用普通 SQLite。冻结 0/12。[维护阶段与停机状态](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-21 六组件正式 Main 保留 H2 文件库及原凭据

Core `5e80d67bd6` 修复 H2 provider 首次打开文件库失败时删除原库的问题，并正确取消重新配置前的 checkpoint 与 defrag 定时任务。原六组件 Main 创建独立 H2 文件库，通过 Kura 配置工厂提交一次凭据，写入一条记录。Core 原地升级期间关闭的数据库 SHA-256 保持。新 Main 先读回原记录，临时重开凭据被拒绝后恢复原配置，再次读到相同业务行。仅校验当前 H2 文件 provider。该 Root 的六包、13 份文件已在正常停机状态只读核对。JDK 21 本模块 42 项、Core/OSGi 完整 82 模块 2139 项通过、2 项跳过。冻结 0/12。[业务读回与最新停机现场](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-21 正式 Main 的 NTP 确认时间与 Wires 包跨容器停机续配

Core `abefdec533` 修复公开 ClockService 在调时尚未确认时就报告同步时间的问题。回环 UDP 回归复现旧行为，新实现只在当前 provider 的系统时钟得到确认后推进时间。ARM64 正式 Main 收到一包有效 NTP 回复，时钟 actor 返回 FAILED / OS_ERROR / EPERM。公开读数仍为空，2.4 秒内无第二个请求，原停用配置恢复。JDK 21 时钟模块 35 项，完整 Core/OSGi 82 模块共 2138 项通过、2 项跳过。Parallels 原静态 Main、15 份历史文件及八个停用单元的围栏已核对。完整 Main 对虚拟 DS1307 的正向写入仍待验。[时钟实装和原 Root 核对](actual-registered-main-clock-and-purge-checkpoint.json)

随后 Core `d0f5bed692` 将确认时间写入提前到 OSGi 事件发布之前。复用同一六组件安装升级后，正式 Main 收到一次无需改动系统时钟的回环样本。事件回调和随后公开 API 读到同一毫秒，actor 操作数为零。该提交的时钟模块 35 项、Equinox 集成 67 项通过。原图、凭据、快照和六包已在正常停机卷上只读核对。[事件、安装与停机证据](actual-registered-main-clock-and-purge-checkpoint.json)

Core `6bfda3c94c` 更新构建脚本。相同安装版本在复用目录重建八个 broker 配对清单后，整个 ARM64 deb 与实装原包逐字节等价。冻结 0/12。[构建回执](actual-rtc-hotplug-and-addon-upgrade.json)

六组件独立安装从 Wires 89d5f57 解包升级到 5605b6b，在真实 postinst 等待注册锁时中断。带着 half-configured 状态正常停机，再启动后外层设备号变化，PID 1、dpkg 和 Main 的原 Root 均精确匹配。仅续配原 Wires 后，dpkg audit 为空，正式 Main 读回已提交一次的原 Timer→Logger 图并再次收到消息。两个启动共用同一内核 boot ID。原凭据、图回执与快照保持，六包已在停机卷上只读复核。冻结 0/12。[Wires 升级和停机恢复](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-21 六组件中 GWT 与 Wires 联合移除在 postrm 中断

实际 dpkg 在 GWT 的文件已移除、postrm 正等待原注册锁时收到定向中断。GWT 留在 half-installed，原注册行仍在。该次 dpkg 继续完成 Wires 移除，Wires 进入 config-files，注册行已移除。仅续办 GWT 的普通移除后，两包均到 config-files，四个 GWT 图标保留。共同 purge 移除四个图标，再从相同的两个 deb 安装，六包恢复原版本，dpkg audit 为空。正式 Main 中七个目标 bundle 均为 ACTIVE，HTTPS 登录入口返回 200。两份凭据、原快照和另外四个包经过全部阶段复核。Main disabled，夹具正常停机。

原 GPSD 夹具重新启动的挂载设备号变动，在业务读取前正常停机。联合维护在独立六组件 Root 上完成。冻结 0/12。[dpkg 阶段、实装 Main 与原现场核对](actual-passive-addon-dpkg-rollback-checkpoint.json)

## 2026-11-21 Docker 挂载目录中的 bundle 热加载和移除

旧镜像中的 FileInstall 已解析但未启动，普通 Main 可以读到 /load 的 JAR，观察包始终没有激活。Docker `9f9aad6` 将原 FileInstall 放入镜像的 6s 启动目录，原四个 deb 及四个 DP 的输入均保持。相同只读挂载、同一数据卷上的新镜像自动激活原观察包。另一次正式 Main 从空目录运行，在宿主发布观察包后取得 ACTIVE 及其 OSGi 回调，撤下该文件后两次查询均已无该 bundle，HTTPS 返回 200。两次都复核了原四个 DP、注册表、回执与密钥。冻结 0/12。[正式镜像和 OSGi 状态](actual-arm64-docker-nonroot-launcher-checkpoint.json)

## 2026-11-21 正式日志 provider 跨应用及审计日志轮换

在同一 ARM64 数据卷、原快照与 HTTPS 身份上，实际让 Log4j 的应用和审计 appender 各轮换一次。旧版两个监听器能收到轮换前的标记，新文件已经写入标记，两路仍收不到。Core `f4b364bdf5` 识别文件替换、排空旧文件并重开原路径，支持已观察到的截短，等待未写完的 UTF-8 行。正式修复包实测两个新 inode 上的标记各送达一次。两版 Main 均以 UID 122、零有效能力运行，原四个 DP、注册表和凭据精确复核。四项模块回归通过；JDK 21 完整 Core/OSGi 共 82 个模块、2137 项通过、2 项跳过。冻结 0/12。[新旧包、日志 inode 和监听回执](actual-arm64-docker-nonroot-launcher-checkpoint.json)

## 2026-11-21 Parallels 上的 USB 串口 modem 识别实测

在原 ARM64 虚拟机上，发行内核通过仅回环可达的 USB/IP 接入 Linux CDC ACM gadget。AT 应答器在接入前就绪。内核创建 ttyACM0，udev 标为候选，40 秒内无 AT 探测。ModemManager 记录没有插件支持该设备，NetworkManager 没有建立 GSM 或 CDMA 对象。卸载夹具后，原 USB 设备、内核模块、RTC、网络连接、默认路由及两个 daemon 的运行身份均匹配，原时钟围栏保留。此次未派发 modem 业务，原操作跨双包 purge 仍待验。冻结 0/12。[设备和服务端记录](actual-modem-sender-restart-and-upgrade-checkpoint.json)

## 2026-11-21 ARM64 Docker 四个 YOFC 部署包与保留快照的日志服务

JDK 21 从源码构建 Runtime DP、组件 API、业务组件和 OPC UA Server，应用模块 43 项测试通过。正式镜像首次启动收到四项安装成功事件。保留同一数据卷，回滚到不内置这四包的基础镜像，再切回新版，DeploymentAdmin 两次均列出四个原版本，四个 bundle 连续采样均为 ACTIVE。四个 DP、注册表、两份回执、加密密钥与 TLS 文件的内容、inode、权限、属主及 mtime 保持。

旧镜像启动时两个日志 provider 均报告找不到文件。Docker `448cdd0` 在镜像内提供两个 Root 所有的兼容链接。原快照继续使用原路径，应用日志和审计日志的 OSGi 监听器各实收一条指定测试消息。实际文件模式 0600、目录 0700，Main 以普通账号运行。首次放在 /load 的观察器没有产生样本；后续回滚和恢复使用只读挂载到正常 6s 启动目录的观察器。夹具经正常 SIGTERM 停机，保留数据卷。日志滚动后的连续读取、非空 /load 的自动加载及其他生命周期组合继续内部待验。冻结 0/12。[镜像与实装记录](actual-arm64-docker-nonroot-launcher-checkpoint.json)

## 2026-11-21 正式 GWT 的 agent 条件菜单和 Cloud 移除后页面

在隔离 Debian ARM64 的正式 Main 上，未装 agent 时进入设置，旧 GWT 的隐藏面板仍读取续期状态。修复后只查询包是否存在。实装 agent 后选择远程标签，离线状态在面板内显示，Core 的 CSR 控件可用；手动读取状态仍会提示连接失败。普通 dpkg 移除并 purge 未初始化的 agent 后，同一个 Main 上的远程标签立即隐藏，快照标签可用。GWT 包 `6b7442f` 以 JDK 21 构建，532 项测试通过。

同一 Root 的六组件另装 Cloud 后，网页列出一个断开的 CloudService。移除并 purge Cloud 后，八份 JAR 消失，客户属性中的 true、文件内容和元数据保留。正式 Main 再启动，菜单仍可进入，显示无可用连接；列举连接与工厂的 RPC 均成功。未提交远程控制、登记或云连接写操作。原两个快照和两份凭据保持，夹具 disabled、正常停机。冻结 0/12。[正式包、页面和维护回执](actual-native-cohort-gwt-ready-and-purge-checkpoint.json)

## 2026-11-21 GPSD 正式定位跨 daemon、socket 与 Position 包升级

Debian ARM64 六组件现场通过真实 gpsd 和伪终端输入取得定位。旧包能在 gpsd 服务重启后恢复坐标，但两次都把报文中的 .375 秒截成整秒。Position `eafa1d3` 保留 UTC 小数，在还没有时间时返回空值；74 项 Position 和 8 项 REST 测试通过。同一 Root 升级后直接沿用原配置，gpsd 服务和 socket 在包升级期间保持运行。随后停止并重建两者，同一个正式 Main 再次从新的坐标和 UTC 取得锁定，输入停止后按配置失锁。两快照、两份凭据与其他六包均保持，夹具 disabled、正常停机。物理接收机仍在原硬件边界，冻结 0/12。[GPSD 实装检查点](actual-standalone-position-six-conversion-checkpoint.json)

Parallels 另实测了一种 modem 输入：发行内核的 `wwan_hwsim` 产生两个 AT 端口，udev 都标为 ModemManager 候选；35 秒观察中 MM 无 modem、NM 无 GSM 设备。原两个服务持续运行，测试后模块、端口、连接和默认路由已核对复原。此次没有派发 reset，已派发 modem 双包 purge 继续待验。[平台实测](actual-modem-sender-restart-and-upgrade-checkpoint.json)

## 2026-11-21 串口 Position 从升级、重装和配置中断中恢复定位

旧包在串口空闲时退出接收线程。六个网关包联合安装后，正式 Main 仅提交一次 Position 配置。将 Position 从 `fd742d9` 升至 `b908c47`、单独移除并 purge 重装、在 postinst 等待注册锁时中断并直接续配，三次均在同一 Root 从伪终端读到各自的 NMEA 定位和 UTC 毫秒。其他六包、两个快照和两份凭据均保持可核对。夹具正常停机，物理 GPS 仍在原硬件边界内。随后用同一 Core 串口实现、两版正式 Position 包和 ARM64 伪终端定量复核。`3af724e` 在无字节时等待 25 毫秒，实测空闲读取从每秒约 110 万次降至 34 次，仍能收到有效定位。该版本 Position 与 REST 共 80 项测试通过。[六组件串口与 dpkg 记录](actual-standalone-position-six-conversion-checkpoint.json)

## 2026-11-21 NTP 单次任务在 provider 停用后退出重试

Core `3fe46eeefb` 将失败重试绑定到发起它的那次启动。ARM64 实际发行包使用 JDK 21 调度器和回环 UDP，第一次回复后的测试回调报错。旧包在停用后再发 1 次，修复后为 0；重新启用时从 2 次降至当次启动的 1 次。一直保持启用的任务失败后仍重试 1 次并成功。当前 Core 的 82 模块完整构建用时 11 分 43 秒，2,133 项通过、2 项跳过，含 67 项真实 Equinox 集成测试。冻结父项 0/12。[正式包和 UDP 计数](actual-registered-main-clock-and-purge-checkpoint.json)

## 2026-11-21 正式 Main 丢弃已退役 NTP provider 的迟到回复

Core `4c43461ff3` 修复 RTC 配置切换时的旧采样竞争。旧实现将每个 provider 的回调交给同一个监听器，一次真实回环 UDP 回归在切换后触发了模拟时钟写入。修复后，时钟服务在发布新配置前等待已受理的回调结束，后续只接收当前 provider 的采样。JDK 21 时钟模块 31 项通过。独立 Debian ARM64 正式 deb 装入完整 Main，Configuration Admin 将 RTC 选择从 /dev/rtc1 改到 /dev/rtc2 后，旧 Java NTP provider 收到并成功解析切换前请求的 UDP 回复。原 actor 目录只保有安装锁，发送端目录没有创建。出厂禁用配置已恢复，Main disabled，容器正常停机。上一 RTC 现场因挂载设备号变化在查业务记录前停机。虚拟 DS1307 上完整 Main 的正向写入仍待验，冻结 0/12。[源码、包配对与正式 Main 实测](actual-registered-main-clock-and-purge-checkpoint.json)

## 2026-11-21 Alpine 启动器用原卷升级四个发行包并回滚

以当前 Core、GWT、Deployment、Wires 四个 deb 构建 ARM64 镜像。原停放容器保留，候选镜像和旧镜像先后挂载原数据卷和空的 /load 卷启动。两个实例均提供原 HTTPS 证书，登录入口返回 200，PID 1 为 UID/GID 122 的 Java，五组能力集均为零。每次正常停止和移除临时容器后，两卷根 inode 与四份受保护文件的内容、属主、权限、inode 和时间戳均一致。候选 JVM 实际打开的 109 个发行 JAR 中，有 16 个与旧镜像同路径文件摘要不同，覆盖全部四个包。此项覆盖基础镜像的升级、回滚及保留卷式移除。冻结 0/12。[启动与回滚检查点](actual-arm64-docker-nonroot-launcher-checkpoint.json)

## 2026-11-21 正式 Main 的 BIND 配置回执跨 Core 与 Networking 共同 purge

原 DHCP 夹具重启后挂载设备号改变，未查询业务操作，正常停机。独立 BIND 原现场另按自己的机器、Root 与 actor 目录配对成功，同 Root 升级至 Core `fcf1206435` 与 Networking `4741b5e`。正式 Main 先为隔离 IPv6 LAN 发布 DNS 配置，再从原出厂快照停用 DNS。服务端提交后，等待回复的客户端被中断，Main 记录 UNKNOWN，named 已停机。两个正式包共同 remove、purge、重装，发送端读回同一编号和原字节。重装后正式 Main 直接使用出厂快照，15 份既有服务端记录和两份凭据逐项一致，只新增原编号的送达回执，无新增 DNS 操作。保留测试前的运行数据及旧证据，删除两份已核实宿主原件的 deb 副本，213,607,914 字节。现场 disabled、正常停机，冻结 0/12。[完整实装检查点](actual-core-network-sender-purge-checkpoint.json)

## 2026-11-21 正式 Main 的 DHCP 已生效回执跨 Core 与 Networking 共同 purge

复用上一次完整 Core 首装现场，重新核实原挂载身份后安装 Networking，使用隔离虚拟 LAN。正式 Main 启用专用 dnsmasq。随后的停用已由 actor 提交，终止等待回复的客户端后 Main 记录 UNKNOWN。普通 dpkg 共同移除、purge、重装 Core 和 Networking；原六份服务端记录与两份凭据逐项一致。发送端恢复原编号、字节、权限、属主、链接数和长度。明确 purge 后仅从夹具备份重新提供原停用测试快照。新 Main 取得原结果，服务端只新增该编号的送达回执，本地 pending 消失，服务保持 inactive。收尾撤除虚拟接口与运行目录中的测试快照，保留恢复证据，Main disabled、正常停机。BIND、modem 的已派发组合仍各自待验，冻结 0/12。[正式包与原操作证据](actual-core-network-sender-purge-checkpoint.json)

## 2026-11-21 正式 Core 首装在 PREPARED 中断后沿原围栏完成

在独立 Debian ARM64 首装现场，完整 `fcf1206435` deb 完成五个安装步骤并发布 PREPARED 后，夹具终止了仅承载本次 dpkg 配置的 systemd 单元。包停在 half-configured。再次解包同一份正式 deb 被维护围栏拒绝，原 postinst 的 abort-upgrade 回调让 dpkg 保持 unpacked。直接执行 `dpkg --configure kura-core` 完成首装，原 17 份意图及回执保持内容、inode 和其他元数据；只新增 ready.json 和同围栏编号的包历史。两份凭据保持、dpkg audit 为空。该现场始终未启动 Main，停放时 disabled、exit 0。五份 deb 用只读挂载复用。冻结 0/12。[原围栏、回执与包状态](actual-core-network-sender-purge-checkpoint.json)

## 2026-11-21 纠正 Core 旧配置尚未完成时 dpkg 误报 installed

Core `fcf1206435` 在 abort 回调中核对活跃围栏与对应原 configure 目录。两个隔离 ARM64 dpkg 事务直接调用正式 deb 内的完整 postinst。替换解包被拒后，原 configure 未完成的一组由 half-configured 转为 unpacked，普通已安装的一组保持 installed，两个原围栏内容均一致。另有 29 项 configure 与普通启动回归通过。两个测试的 configure 入口由载体构造。完整 Core 现场重新启动后，挂载设备号由 1048815 变为 1048582，根目录与三处 actor 的 inode 保持。配对拒绝后已正常停机，未启动 Main、查询原编号或发起包替换。完整原 Root 上的同类中断续接仍待验。冻结 0/12。[包维护检查点](actual-core-network-sender-purge-checkpoint.json)

## 2026-11-21 三类网络发送端的原编号经过 Core purge

Core 原先在 purge 时删除运行数据保留副本，DHCP、BIND 和 modem 的 Java 待确认文件也会随之消失。Core `4857c3128c` 在移除时按各自的机器标识、actor 目录设备号和 inode 核对记录，重装时在 Main 停止的条件下以原 kurad 身份发布。Debian ARM64 的正式 Core 包原地升级、移除、purge、重装通过。发行版 Java 发送端读回三条原记录，编号、字节、属主、权限、链接数和长度匹配，三个 actor 目录均保留且没有写入作业。凭据一致，dpkg audit 为空。该现场验证派发前的持久化。正式 Main 已生效、未确认的 DHCP 经 Networking purge 的证据见下一条。

首个候选降权后无法穿越版本安装目录。它保住原归档并停在 half-configured。一次替换升级被围栏拒绝，旧 abort 回调成功返回导致 dpkg 报 installed，实际原配置阶段仍为 PREPARED。该次故障用修正版 deb 的保留工具及原包已核对的 postinst 在同一围栏内局部修复。后续独立的修正版正常维护路径通过；旧 abort 回调的状态误报继续跟踪。61 项 ARM64 定向回归通过。新隔离现场 Main disabled、正常停机。冻结 0/12。[实装检查点](actual-core-network-sender-purge-checkpoint.json)

## 2026-11-21 正式 Main 的 DHCP 停用回执跨 Networking purge 和配置中断恢复

六组件 Debian ARM64 现场用原 Root 启动正式 Main。第一次启用请求启动了专用 dnsmasq。观察器误读结果格式，启用已正常确认。随后独立的停用请求使服务停止，观察器核对意图摘要及 APPLIED 后中断等待回复的客户端，Main 记录 UNKNOWN。该笔 pending 和六份服务端记录完整经过 Networking 移除、purge、解包、真实 postinst 中断及原包配置续接。重启正式 Main 后，仅新增原停用编号的送达回执，pending 清除，服务维持 inactive。两个凭据一致，现场正常停机。Core 自身 purge 时三类 Java 发送端的未确认记录仍待处理。冻结 0/12。[检查点](actual-registered-main-dhcp-maintenance-checkpoint.json)

## 2026-11-21 启用中 DNS 跨 Main 重启和包维护，修复普通启动漏读快照

六组件 ARM64 现场复现了普通快照解析按字面 `esf:` 查找配置的问题。同一命名空间换用另一前缀后，旧 Main 加载快照却漏掉 LAN 配置，DNS 监视器执行停用。用发行前缀表达同一棵 XML 树，旧 Main 即发布 IPv6 LAN 模型。Core `c3a6b4f549` 按命名空间和父子关系读取普通快照；受控恢复继续执行原有严格格式校验。实际原地升级后，正式 Main 用原混合前缀文件恢复已启用的模型，原 BIND 操作与密钥保留。

Networking 单独 remove、purge、重装后，五个其他包保持 installed，dpkg audit 为空。已启用的 named 全程保持 PID 16251 和原启动时间。正式 Main 运行时，独立 IPv6 客户端经仅允许一个来源、接口和端口的规则访问 named；合成 IPv4 上游实际收到一次请求，客户端读回 198.51.100.27。测试链路由夹具准备，NetworkManager 上报 unmanaged。收尾用正常配置撤销测试规则和 DNS 模型，保留历史，恢复原解析器文件内容，仅保留原启动快照，容器退出 0。JDK 21 完整 Core/OSGi 82 模块 2129 项通过、2 项跳过。冻结 0/12。[实装检查点](actual-enabled-dns-snapshot-upgrade-checkpoint.json)

## 2026-11-21 正式 Main 的 RTC 缺失回执跨 Core purge 恢复

六组件原 Root 的正式 Main 通过 ConfigAdmin 选择不存在的 `/dev/rtc1`，触发一次生产 ClockService 回调。该现场的 PID 1 缺少 CAP_SYS_TIME，时钟进程五次启动均记录 UNSAFE_ACTOR；发送端只创建了锁。再次启动 Main 读回默认配置，原 BIND 四份记录及两份凭据逐项一致。

独立 Debian 13 ARM64 安装使用同一 Core 包，隔离网络和设备，并在容器层禁止修改系统时钟和 RTC 写入。正式 Main 选择相同的缺失设备，生产 actor 准备阶段记录 errno 2，最终结果为 FAILED / OS_ERROR / ENOENT，发送端保存同一编号。真实 dpkg remove、purge 和重装均成功。五份 Root 文件及两份凭据的完整指纹一致，三份 sender 文件恢复内容、属主、权限、链接数、长度和 mtime。重装后的 Main 再次取得已注册的 ClockService，原操作仍为唯一一笔。两夹具均正常停机，删除了已核对宿主原件的五份临时 deb，共 204,519,238 字节。Parallels 保留原围栏与两笔操作，完整 Main 正向写入虚拟 DS1307 仍待验。冻结 0/12。[检查点](actual-registered-main-clock-and-purge-checkpoint.json)

## 2026-11-21 正式 Main 的 BIND 丢回执恢复及六组件 Networking 维护

真实 BIND 服务在正式 Main 调用 stop 后停机。测试在 actor 提交结果后终止了等待回复的客户端，第一 JVM 返回 UNKNOWN。第二个正式 Main 核对原 Root、原编号及动作后完成确认，原意图和结果文件保持，named 没有再启动。该次启动另出现一笔记录，未采集它的动作。

Core `5a47412895` 增加只读核对，Networking `4741b5e` 在普通启动时先读取已发布的 DNS 模型。独立 Debian 13 ARM64 实装六个网关 deb，同一 Root 先后启动三个正规 Main。第一次保存 disabled 模型；Networking 升级、单独 remove、purge 和重装后，后两次启动均恢复原模型。三次都只看到原编号、同一套 BIND 意图、结果和回执；文件内容和元数据、named 状态及密钥一致。其余五包保持 installed，dpkg audit 为空。Networking 重装后按启动器规则移至注册顺序末尾。JDK 21 全 Networking 2249 项、ARM64 actor 48 项通过。两个现场正常停机，核实宿主原包后共移除 17 份夹具临时 deb，598,030,778 字节。启用中 DNS、完整 Main/RTC、其余组件和 provider/launcher 的完整维护与中断矩阵仍待验。冻结 0/12。[检查点](actual-bind-main-recovery-and-six-component-networking-checkpoint.json)

## 2026-11-21 DHCP 调用方跨 JVM 续接原确认

Core `61aac0f8ce` 支持 BIND 和 DHCP 按原动作及配置查回执。Networking `e06eb2b` 的 DHCP 调用方派发前持久化原请求，与 modem 复用文件锁和原子发布。Debian 13 ARM64 上三个普通用户 JVM 先后运行。第一个让真实 dnsmasq 停机一次，确认只写入 32 字节。第二个遇到不同 Root，只查询 scope。第三个回到原 Root，用同一编号和配置将原 inode 补完到 201 字节。测试使用真实 deb 的 Java 与 socket actor，systemd 调用身份及服务调度使用适配器。

两个临时 ARM64 安装分别对同一 Networking deb 执行实际 dpkg 解包，旧 Core 拒绝，新 Core 准入。首次打 Core deb 时跳过 assembly 也跳过了资源刷新，四份脚本仍旧；包在安装前被检查拒绝。按发行 POM 重新准备资源并单独打 deb，核对包内代码及成套摘要后验收。Networking 全 17 模块 2245 项、ARM64 actor 46 项通过。BIND 的 Java 跨进程续接、DHCP 完整 Main 及维护矩阵仍在进行。冻结 0/12。[检查点](actual-modem-sender-restart-and-upgrade-checkpoint.json)

## 2026-11-21 正式 Main 在重启及升包后保留未决调制解调器操作

旧 Networking 的 JVM 在确认中断后丢失原操作。Core `54cc64dfe2` 和 Networking `30f67a722f` 现在核对原 Root、编号、动作、设备路径及设备标识。ARM64 的三个独立 JVM 场景覆盖 32 字节部分确认、确认已写完但回复丢失，以及虚拟调制解调器已收到 Reset 而结果不明。每个场景仅收到一次 Reset。Networking 全 17 模块 2245 项、Core actor 62 项通过。

实际 dpkg 在旧 Core 上拒绝新版 Networking。同一隔离安装升级 Core 后接纳相同的 deb。前一版正式 Main 用探针在派发前写入 GPS 记录，新版正式 Main 请求 reset 时返回原 GPS 编号的 UNKNOWN，actor 无新记录。Core 升级重建了发送记录、站点属性和快照的 inode，三者内容保持，密钥和发放记录的元数据也一致。另一个已验证跨 JVM 重启的现场因再次启动时挂载设备号改变，在 dpkg 和操作查询前停止。两现场均正常停机。物理调制解调器及其余内部验收仍开放，冻结闭环 0/12。[检查点](actual-modem-sender-restart-and-upgrade-checkpoint.json)

## 2026-11-21 独立 Core 的 Position 升级、六组件转换中断与拆回

在新的 Debian 13 ARM64 测试安装上，Core `bdf7690c8e` 用发行版 systemd 单元独立启动。旧 Position 的串口 provider 缺少连接工厂，原地升级至 `fd742d9` 后，正式 Main 内的串口 provider 与 Core 工厂都为 active，绑定指向 `org.eclipse.kura.core.comm`。remove 后再次启动 Main，定位 provider 消失；purge 完成。gpsd socket 的 invocation 保持。此次观察了真实服务绑定，未打开物理串口。

同一 Root 将独立 Core 转为六个网关包。五个新增包同时解包，Networking 先完成配置，随后在四包联合配置中，向等待注册表锁的 Wires 子进程发出 SIGTERM。发送前已核对其进程祖先及锁的设备号和 inode。dpkg 返回 1，Wires 留在 half-configured，GWT、Deployment、agent 均完成配置。只续配原 Wires 后六包 installed，dpkg audit 为空。正式 Main 加载 Core、Networking、GWT、Deployment、FileMove、Wires 和 agent bridge 七个指定 bundle。此时 agent 从未初始化，仍 disabled/inactive。

五包联合 remove 全部进入 config-files，联合 purge 后全部退出安装。正式 Main 随后独立启动，两次观察各 216 个 bundle，Core active，六个附加 bundle 均已消失，Networking drop-in 已撤下。Core 密钥、发放记录、原快照的内容及元数据一致。Networking 安装时在原 799 字节属性后追加两个 true，卸载后最终值均为 false。现场以 exit 0 停机，核实宿主原件后移除 12 份夹具内重复 deb（375,009,702 字节）。原六组件现场此前的 Root 配对拒绝继续保留；沿原 Root 的绑定业务与剩余 provider、launcher、typed brokers 和完整 Main/RTC 仍待验。冻结 0/12。[实装检查点](actual-standalone-position-six-conversion-checkpoint.json)

## 2026-11-21 十个附加包的 dpkg 回滚与 GWT 配置中断恢复

在同一隔离 Debian 13 ARM64 Root，旧 GWT 与 Wires 遇到后继包 preinst 拒绝后，dpkg 将原本 installed 的包留在 unpacked。两包原地装入修订版恢复。十个缺少 prerm 的附加包现已能完成 abort-upgrade、abort-remove、abort-deconfigure，实测 30 次回调。后继包仍被拒绝，原包均保持 installed。八包联合验证中，81 份文件指纹在每类回滚前后吻合。构建复用现有 Java 产物，只重新组装发行包。

GWT 的 remove 保留四份图标配置；配置中断后，注册表未变，dpkg 进入 half-configured。原包继续 configure 后恢复唯一注册行和用户图标，显式 purge 移除四份配置。相邻 Wires 的 14 份 JAR 及独立业务数据保持。Cloud 在合成现场因上级目录属主不符而首次配置失败，调整夹具后仅续配原包；已为 true 的属性文件字节相同，inode 和时间戳有变动。该现场仅用最小 Core 依赖包，正式 Main、原六组件与 provider/launcher 全维护矩阵继续待验。冻结 0/12。[实测检查点](actual-passive-addon-dpkg-rollback-checkpoint.json)

## 2026-11-21 BIND Java 已观察成功后的确认中断恢复

Networking `e01f6ec` 保留原 BIND 请求和 JVM 已实际收到的成功令牌。送达确认失败时，下一次调用或原 DNS 监视器先核对作用域，再只送原编号的确认。actor 用原 Root、意图、结果及令牌核验。相同动作在恢复后直接返回，IPv4 及双栈模型在确认后发布。原回复未知、令牌无效和 Root 改变仍维持围栏；监视器遇到有未决请求的旧 provider 时继续保留原配对。

JDK 21 Networking 全 17 模块、2242 项测试通过。旧 deb 的 Java 合同复现确认后无法前进，新包通过了 Root 变化、无有效原回复和模型发布检查。隔离 Debian ARM64 用实际 BIND provider、固定 socket 客户端及 Core `bdf7690c8e` 包内 actor，在普通用户 JVM 发出单笔 stop。真实 named 仅停一次，首次确认内核只写了 32 字节，再核对 scope 并以原编号补完到原 inode 的 201 字节，原 Root、意图、结果保持。peer 的正式 systemd Main 身份、目标 unit 观察及 stop 调度由测试适配器提供。本证据覆盖同一 JVM 的已观察成功路径。完整 Main 实装、跨 JVM 未决交接及完整维护矩阵继续内部待验。原三现场、六组件、BIND/DHCP 原续接现场与 7fd 停放复核，冻结 0/12。[检查点](actual-rtc-hotplug-and-addon-upgrade.json)

## 2026-11-21 DHCP 正式 Java 调用方沿原编号确认送达

Networking `22a47d6` 修复后台调和一旦遇到未决回执便永久停在 JVM 内的问题。下一次调和先用原编号读取作用域，仅在原 Root 核对原结果；取得原 APPLIED 的有效令牌才确认并释放编号。Root 改变、NOT_FOUND、UNKNOWN、坏令牌及确认失败均保留原操作。相同目标在本次恢复中不再执行 apply；目标已变则确认成功后才重新比较。上述覆盖同一客户端存活期间的恢复。跨 JVM 的原编号恢复和全部安装维护继续待验。

JDK 21 的 Networking 17 模块构建及 2242 项测试通过。用新 deb 内实际调用类、Core `bdf7690c8e` 包内固定 Unix socket 客户端及 actor 进行了隔离 Debian ARM64 联调。普通账号 Java 发出一次 apply，独立 dnsmasq 只停止一次。内核将首次送达文件截于 32 字节，Java 下一次调和以原编号 scope、outcome、acknowledge，经原 inode 补完为 201 字节，原意图、结果、Root 元数据及内容保持。peer 的 systemd Main 归属、unit 观察和 stop 调度使用夹具适配器；此项没有在正式 Main 实装。已 purge 的 DHCP 原夹具 `206dcf6f` 停机状态与回执吻合，但重启后设备号由 1048825 变为 1048582，inode 5031285 保持；未查询历史、未派发包或启动 Main，已正常停机。三原现场、六组件和 7fd 复核通过，冻结 0/12。[检查点](actual-rtc-hotplug-and-addon-upgrade.json)

## 2026-11-21 当前 Core 单次完整 reactor 验证

在 Core `bdf7690c8e` 使用 JDK 21.0.13、Maven 3.10.1，从根 POM 执行无模块筛选的 `-Posgi-it verify`，82 个模块一次完成，11 分 42 秒。Maven 各测试阶段共报告 2130 项，2128 项通过，SystemServiceTest 跳过 2 项，失败和错误均为零。其中跨模块单测 127 项、真实 Equinox 集成测试 67 项全部通过。13 项 CRL 集成测试用时 424 秒。预先调用 build-all 使用的测试模块选择器只选中 23 个模块，已停止该次调用并以本次完整构建计数。原三现场在完整构建后核对一致。修复版在原六组件或完整 Main 的设备、安装维护和中断矩阵继续内部待验，冻结闭环 0/12。[测试汇总](actual-rtc-hotplug-and-addon-upgrade.json)

## 2026-11-20 同一编号续完 BIND、DHCP、modem 的送达确认

Core `bdf7690c8e` 修复结果已经 APPLIED、送达确认文件只写了一部分时无法确认的问题。原 Root、编号、意图、结果及确认令牌配对后，只在原 inode 上补完与应有记录完全相符的前缀，并同步文件及父目录。只读查询不改送达记录，错误令牌或无关字节继续拒绝。实际操作不重发。此路径要求已有完整的 APPLIED 结果，缺损的操作意图或结果继续 UNKNOWN。

原实现在 32 字节截断的 BIND 确认回归上失败。修复后 ARM64 回归 61 项通过。JDK 21 单独组装 Core deb 用时 9.186 秒；从包取出的 DHCP、modem actor 分别在第 32、64 字节被内核中断，两次均能只读查到原结果；最后沿原 inode 补完 201 字节并确认。意图、结果和 Root 记录保持。此处使用合成的已提交记录和测试用授权入口。修复版正式 Main 实装及全组合维护仍待验，冻结 0/12。[检查点](actual-rtc-hotplug-and-addon-upgrade.json)

## 2026-11-20 原 BIND Root 上的联合包升级与正式 Main 锁竞争

在独立 ARM64 夹具 `20b36ea5`，原 Root 的设备号、inode、机器及作用域吻合。实际 dpkg 联合升级 Core 至 `db514f2a57`、Networking 至 `0789499`；原 BIND 五份文件、凭据、密钥及既有包回执逐项吻合，新增一份本次包回执。13 份原配置及测试数据在 Core 恢复复制后 inode、ctime 改变，内容、所有权、模式、大小及 mtime 保持。升级期间 Main 保持 disabled、inactive，named 的 PID 及 invocation 连续。

随后启动该安装的正式 Main，经 OSGi DNS 服务发出一次独立的原目录锁竞争请求。原锁由独立进程实际持有，原调用返回 REFUSED，同 Root 查询该新编号返回 NOT_FOUND。未写入新意图；named 的 invocation 和原五份记录保持。停机后探针已归档。另一次启动准备验证修复版 Position 时，原目录 inode 仍为 4928376，设备号由 1048815 变为 1048582。配对校验拒绝，未查询操作编号、未启动 Main、未发出 dpkg，夹具正常停机。Position 修复版完整安装验收继续待办。原三现场 7 个进程、38 份受保护文件和 6 份运行数据复核通过，冻结闭环 0/12。[实装检查点](actual-rtc-hotplug-and-addon-upgrade.json)

## 2026-11-20 BIND、DHCP 原意图写入途中保留操作编号

Core `db514f2a57` 修复写入失败后的回执分类。旧实现将同 Root 下无法解析的部分意图报告为 REFUSED，BIND Java 调用者会据此清除原编号。BIND、DHCP 和 modem 现在共用同一处理逻辑。只有在确认原 Root、固定原目录并查明该编号的意图文件缺席后，才报告写入被拒绝。原 Root 的只读查询遇到可安全读取但无法解码的意图或结果，返回 UNKNOWN。Root、文件所有权、权限以及可解析记录的配对校验继续生效。

旧代码上的新增回归复现了误判。Debian ARM64 定向回归 59 项通过，三个 broker 的成套文件与当前 Core deb 逐项配对。用包内脚本分别派发一次合成 BIND 和 DHCP 请求，内核实际在意图的第 128 字节截停写入。两次原请求和同编号只读查询均得到 UNKNOWN，原文件保持，无服务作业、结果或送达记录。DHCP 使用发行版 dnsmasq 实际检查了配置。此项使用隔离进程、真实本机 socket 和测试用 peer／unit 条件；未在六组件正式 Main 安装本版本，未执行 modem 复位。冻结 0/12，原三项外部延期保持。[检查点](actual-rtc-hotplug-and-addon-upgrade.json)

## 2026-11-20 正式 Main 的 OSGi 引用和串口定位服务修复

在六组件原现场 `60df`，正式 Main 相隔 5 秒的两次 SCR 采样完全一致。IPv4、IPv6 本地防火墙组件均 active，对应的 agent 实现等待另一配置。Cloud Deployment 缺 MQTT 传输，ModemManager 定位缺 Position 条件。另有 20 条 require 声明没有实例，1 条 optional 声明 WebsecurityFilter 在包中默认禁用。此次没有切换防火墙配置。

在同一原现场装入 Position `e014454` 后，ModemManager 实际绑定 Position 包提供的条件，GPSD、PositionService 和 Position REST 均 active。串口定位组件独自缺少旧 OSGi ConnectionFactory。Position `cf7e292` 将 DS 引用改为 Java 方法和 Core 服务共同使用的 `org.eclipse.kura.comm.CommConnectionFactory`。新增回归在旧声明上失败，修复后 76 项测试通过。ARM64 独立 Equinox 使用两份 deb 的实际 bundle，在线更新后串口提供者绑定原 Core 工厂的同一对象，卸载后串口服务消失，Core 注册继续有效。此项未打开串口。

原现场实装的旧 Position 已 purge，原四行注册表、业务配置、密钥与历史保持，gpsd 服务及 socket 状态连续。随后准备安装修复版时，原 Root 的挂载设备号及四份包历史文件的设备号改变；文件 inode、内容和其他元数据保持。配对检查在发出 dpkg 和启动 Main 前拒绝。六组件已正常停机，后续沿该 Root 使用最新停放回执。修复版在该现场的 dpkg／完整 Main 验收仍待完成。12 项冻结闭环 0/12。[实装检查点](actual-rtc-hotplug-and-addon-upgrade.json)

## 2026-11-20 Core deb 单独构建与时钟暂存锁

Core `8947d37475` 在暂存文件被其他进程锁住时立即返回 BUSY。3 项 ARM64 Root 回归通过，含解除锁后沿原文件补完。该版已构建，原 Parallels 上实装到 `4739a24da1`。在依赖已就绪时，单独组装 deb 用时 8.585 秒，完整发行用时 30.215 秒，两个 deb 整包 SHA-256 一致。单包验收可少生成约 408 MB 附带产物。冻结 0/12。[实装检查点](actual-rtc-hotplug-and-addon-upgrade.json)

## 2026-11-20 六组件原 Root 的 Cloud 安装、升级、移除和 purge

真实 dpkg 将 Cloud `8bb2418` 装入原六组件夹具，再升级至 `b7d280f`，随后 remove、purge。每次只在 dpkg 的私有挂载命名空间覆盖应用 user 目录，使用合成站点配置；包数据库、systemd 日志和 sibling 注册表都来自原 Root。升级时末行带续行符，维护脚本正确独立追加 Cloud 开关。注册顺序从四行到五行再回到原四行，原八包及其他受保留文件均吻合，中心 PID 连续且 Main 保持停用。

原客户配置从本轮开始到正常停机的内容、inode 和时间戳完全一致。与更早的可选包检查点比，本轮开始前其 inode、ctime 和挂载设备号已有变化，内容及 mtime 一致。实测原生 remove 和 purge 保留了测试配置中的 `kura.have.cloud.connection=true`；本轮未验证卸载后的 GWT 菜单。六组件已正常停机，原三现场和 7fd 保持，冻结 0/12。[实装检查点](actual-rtc-hotplug-and-addon-upgrade.json)

## 2026-11-20 原时钟 actor 暂存文件写入中断

Core `4739a24da1` 支持逐文件续装。只有原维护下尚未发布的候选可补写，先校验文件类型、所有者、模式、链接数和已有字节，再从同一候选补全。旧 actor 已归档时，暂存树依然必须完整通过配对验证。Debian ARM64 以普通账号验证 96 项，另以 Root 验证 2 项暂存回归。

在原 Parallels 时钟 Root，将实际安装器单进程的文件大小上限设为 4,096 字节。安装返回 1，7,761 字节的文件实际只写入 4,096 字节，旧 actor 仍在原路径。普通安装入口随后沿原围栏在同一 inode 上补完，完整摘要与 deb 相同，原 actor 和暂存目录在发布前后均可核对。15 份原状态文件、两笔操作及静态 Main 保持；新停机围栏下八单元均 inactive。此项覆盖暂存途中中断。磁盘满及硬断电仍属原外部延期。冻结 0/12。[实装检查点](actual-rtc-hotplug-and-addon-upgrade.json)

## 2026-11-20 正式 Main 日志字段实测和原时钟 actor 发布中断恢复

六组件原 Root 启动正式 kura.service。OSGi 客户端通过已安装的 typed journal broker 分别导出 Kura 和系统日志，指定 _SYSTEMD_UNIT、SYSLOG_IDENTIFIER、PRIORITY。结果为 5,595 和 873,189 字节，分别与独立 journalctl 同范围输出前缀逐字节吻合。独立标记仅出现在系统范围。采样到 245 个 bundle、110 条 SCR 配置或描述记录；唯一处于 INSTALLED 的独立 Jackson JAXB 扩展属于已记录的发行兼容边界。记录保留了 4 条未满足引用及 21 条没有配置实例的描述。此次通过实际 Main 调用 broker，未增加浏览器验收。原六组件已正常停机。

Core `3aa64568bc` 修复时钟 actor 在代码归档和发布两次 rename 之间中断的续接。原 Parallels 组件沿原 namespace 和维护 purpose，用 JDK 21 构建的 ARM64 deb 更新两份发生变化的时钟文件。实际安装器在第二次 rename 前退出 75，活动目录缺席，旧归档和新暂存两边均可核验。经正常维护入口继续，保留了两个目录各自的 inode，原 15 份状态文件及两笔操作保持。原围栏恢复后再正常停用，八单元均 inactive。96 项 ARM64 时钟测试通过。VM 根只有一个时钟插件和静态测试单元；完整正式 Main 正向写入虚拟 RTC 仍待联调。冻结 0/12。[实装检查点](actual-rtc-hotplug-and-addon-upgrade.json)

## 2026-11-20 Deployment 在管理员临时屏蔽 FileMove 时升级、移除和恢复

Deployment `3772936` 将管理员创建的临时 mask 与原 Root、boot、符号链接设备号、inode 和 ctime 配对。六组件原夹具实际从 `c93c39e` 升级。持有 sibling 注册表锁后，真实 dpkg configure 返回 1，包进入 half-configured。释放锁后，沿原 Main 与 FileMove 两份围栏完成 configure。四项注册顺序、临时 mask、中心进程以及原密钥、Agent 和中心回执均保持。JDK 21 验证通过 240 项。

在原 Deployment 单 provider 夹具 `d533a299`，装入同一包后执行 remove、purge、重新解包和配置。实测包状态依次为 config-files、not-installed、installed，原 mask 与移除围栏贯穿 purge，配置完成后原 socket 仍受管理员屏蔽。原三组发布记录和三份 Core 文件逐项吻合，Main 启动次数为零。两夹具均已撤销本次测试创建的 mask 并正常停机。历史 MOVE 夹具 `e5f3de3d` 的挂载设备号与原回执不同，仅核对 Root 后正常停机，未发包维护或业务请求。本轮冻结仍为 0/12。[检查点](actual-rtc-hotplug-and-addon-upgrade.json)

## 2026-11-20 六组件原 Root 的时钟服务和 socket 联合停机

Parallels 组件停机时，systemd 提示 RTC 服务停止后其 socket 仍可激活服务。Core `5f6be61f46` 将八个时钟服务及 socket 纳入同一个 PID 1 停止事务，并逐项核对终态。95 项时钟合同在 Debian 13 ARM64 以非 Root 身份通过；首次以 Root 运行时，模拟普通调用者的用例被准入检查拒绝。

JDK 21 构建当前 ARM64 deb，六组件原 Root 从 `66cce5e` 实际原位升级。中心进程连续，Main 维持 disabled，Agent 没有启动，原两份中心回执和 Agent 源包记录完整。实际 relay 在 accept 等待时执行新维护入口，八单元均 inactive，原 relay 进程结束。systemd 保留 socket 文件，Linux 内核连接结果为 ECONNREFUSED。沿同一围栏恢复后 socket 重新监听，relay 按需启动，新增时钟请求数为零。原六组件夹具正常停机；原 VM 已关机、三现场、13 个原运行容器及原 7fd 保持。冻结仍 0/12。[检查点](actual-rtc-hotplug-and-addon-upgrade.json)

## 2026-11-20 正式 RTC socket 与 actor 在虚拟 DS1307 上写入成功

原 Parallels ARM64 时钟组件沿原 namespace 和停机维护围栏，将 Core 包 `168681f6aa` 的时钟文件及正式 systemd 单元配对升级。沿用原测试单元、账号和 JDK 21，测试入口 `30b7af79e4` 增加仅针对内存 I²C 总线上 DS1307 的显式触发。真实 ClockServiceImpl 以零有效 capabilities 运行，时钟 actor 仅有 CAP_SYS_TIME。JVM 时钟模块 30 项通过。

虚拟 `/dev/rtc1` 首次读日期为 EINVAL。零系统时钟偏移触发一次 RTC 同步，relay 由 systemd 启动两次并均成功退出，分别处理 prepare 和只读 FD 交付。actor 实际 ioctl 返回，原 namespace 内新增一笔 APPLIED、RTC_SET_RETURNED、errno 0；设备读数与同期 guest UTC 差 0.837 秒。原校时和新 RTC 共两组 issued/completed，原校时未重发。停机前核对新操作的六个文件、原历史文件、同 namespace 的新停机围栏；移除虚拟设备后 VM 正常关机。此次启动的是历史 ClockService 组件夹具，完整 Core/OSGi Main 的 RTC 联调仍待验。物理 RTC 归原硬件边界，OrbStack 共享 rtc0 保留。冻结闭环 0/12，外部延期仍为原三项。[检查点](actual-rtc-hotplug-and-addon-upgrade.json)

## 2026-11-20 Agent 原编号升级与 Cloud 客户属性保留

Agent `e28abfa` 将源包、Kura peer、材料、远程开关和传输五种完成入口的原编号核对移入 dpkg 锁。原操作已结算时只读查询。原六组件 Root `60df52cf998b` 上，首次准入因新 deb 携带宿主属主而拒绝，原运行状态、该编号目录及 pending 均已核实。仅将暂存 deb 的属主修正为 root:root，保持摘要和操作编号，解包和配置各执行一次。原准备进入 UNKNOWN 后没有重发。实际 postinst 返回 APPLIED，Agent 和 Main 均保持停止，中心进程连续，原十份输入和两份中心回执相同，夹具正常停机。

工作树后续 `ab1cf33` 保留各入口原有的编号拒绝文案。Debian 13 和 Alpine 的 ARM64 包装回归各通过 323 项、套件跳过 2 项，另有 2 项依赖能力配置的用例未计入通过。六组件实装版本仍为 `e28abfa`。中心包装在 Debian 13 ARM64 另行复核，142 项通过。

Cloud `b7d280f` 修正安装与卸载脚本向未换行或末行续写的 Java 属性文件追加开关时串改客户值的问题。JDK 21 构建 deb，在独立 Debian 13 ARM64 解包并核对两份脚本字节，Java Properties 验证安装和卸载各 7 个输入，旧实现的两个末行缺换行样本均读错。该项覆盖包内脚本，不计为 Cloud 在原六组件上的完整安装、移除或 purge。原三现场七个进程、38 份私有文件及六份运行数据再次一致，25 份个人文件与原 13 个运行容器保持。冻结闭环 0/12，外部延期仍为原三项。[检查点](actual-rtc-hotplug-and-addon-upgrade.json)

## 2026-11-20 中心安装完成回执及同 Root 的释放中断

console `930cab3` 把安装器任务编号与中心 postinst 的 COMPLETE 回执放在同一次原子写入。历史两次写入留下的缺编号回执，只在原活动围栏、原 RUNNING 安装意图、原归档和载荷均配对时补齐。其他任务编号拒绝，已完整回执重复核验后保留原 inode。37 项定向测试和 Debian ARM64 包装套件 142 项通过。按当前 Git 构建中心与安装器，207 项编译输入与已验 ELF/Web 一致后复用相同二进制及三份静态文件。

原六组件 Root `60df52cf998b` 上先升级安装器，运行中心进程连续；正式维护停止中心，安装器使用独立的维护及安装 UUID 并仅派发一次。观察到第一份 COMPLETE 已带原安装 UUID，当时 worker 仍为 RUNNING。独立管理进程实际持有 host.lock，dpkg 成功回执与 APPLIED 已持久化，worker 退出 1，原围栏仍在、启动准入为 MAINTENANCE。系统单元日志没有 Python 堆栈。释放锁后沿同一安装 UUID 单次 native-finalize，原两份终态文件的内容及元数据均保持，围栏清除。经显式启动恢复 READY，再正常停机。原六个网关包版本、原 Core/Agent 文件与十份保留输入核对通过，原安装编号没有重发。最新停放证据在 `.management-close-20261116/core-rtc-hotplug-20261120/center-held-completion/parked.json`。上述不扩大为所有中断及已绑定业务通过；冻结仍 0/12，外部延期仍为原三项。[检查点](actual-rtc-hotplug-and-addon-upgrade.json)

## 2026-11-20 RTC 热插拔、原 Root 的 Core 及三组件升级

Core `66cce5ec64` 将时钟 actor 与按请求打开 RTC 的无能力 relay 分开。Ubuntu ARM64 虚拟 I²C 总线上，真实 DS1307 内核驱动创建、拔出、同编号重新注册，别名改指和两种打开竞争均完成观察。原绑定拒绝，重新准备可收到只读 FD。设备检查使用实际 SEQPACKET socketpair，未调用写时间 ioctl。ARM64 时钟合同 93 项通过。

已在六组件原 Debian 13 ARM64 Root 上由 `735ebf0c64` 升级新 Core。正式 socket、actor 副本、vendor 单元与 cohort 配对；同 UID 的其他进程被实际 relay 拒绝。正式 Main 的 OSGi ClockService 已注册，两份原快照均为 disabled，getLastSync 返回 SERVICE_UNAVAILABLE。保持原配置、原图和所有历史操作编号。合法 actor 经正式 socket 的正向 FD 及 RTC 写入仍待实证。

GWT `b62f67a`、Wires `89d5f57`、Deployment `c93c39e` 移除无人生产的跨包失败标记，继续传播各自安装器的真实退出码。三个新包在同一 Root 一次联合解包后均为 unpacked，Deployment 的维护 mask 生效；单次联合 configure 后八包 installed、Main 回到原 disabled。中心进程、十份保留文件和 Core 包历史在两阶段一致，现场正常停机。上述为阶段间续接；故障中断和其他 provider/launcher 场景继续内部 pending。冻结闭环仍 0/12，外部延期仍为原三项。[检查点](actual-rtc-hotplug-and-addon-upgrade.json)

随后对同一 Root 的 Wires 核对包内无 conffiles，已装文件仅含受管理的 JAR 与脚本，原图存在 Core 快照。单包 purge 后为 not-installed，原十份文件保持。原包解包后，一次真实注册表锁竞争使 configure 明确返回 1，包处于 half-configured，Wires 注册行仍缺席。释放锁后同一 Root 续配成功，八包 installed、原四行顺序和维护历史精确。原图没有再次提交。联合整机中断和其余包的故障恢复继续内部 pending。

Core 后续提交 `a356b28138` 将旧 RTC 投影退场限定为同一维护围栏、同一旧 actor 清单和原 service 的配对。Linux ARM64 设备目录试验验证保留 inode 的原子移动和重复进入；时钟合同 95 项通过。Core `168681f6aa` 修正配置测试，覆盖未标记快照的密码解密和失败回退，保留新旧 XML 两份预期，并显式选择单参数快照保存方法。JDK 21 的 Core 82 模块分初跑和续跑完成；跨模块单测 127 项、真实 Equinox 集成测试 67 项通过。初跑 SystemServiceTest 有两项跳过。此记录覆盖该 Maven profile；客户安装下的全部 provider、launcher 和故障矩阵仍在台账中。

同一 Core 工作树单独重建 ARM64 发行包，14 份时钟配对文件逐项核定。原 Ubuntu 时钟 Root 从 `29c92f0` 升级时，缺少 kmod、python3-dbus 和 iputils-ping，dpkg 明确退出 1，状态 unpacked。五份原 actor 记录及两份已移入保留目录的 sender 记录均核对。由相同基础镜像从 Ubuntu 签名仓库取得三项依赖，原围栏摘要保持，单次 configure 成功；原 FAILED 回执和七份文件精确，操作数、issued、completed 各一。新 RTC socket 已启用，Main 保持 disabled，原 Root 正常停机。旧投影在本次冷启动时不存在，实际带设备的旧投影迁移仍未覆盖。

随后排查其余 sibling，Artemis、Camel、Cloud、Container、OPC UA、Position、Triton、YOFC Runtime 各自只读取共享临时失败标记。八仓在本地 WIP 修正，JDK 21 构建八个 deb，59 份有效载荷和维护脚本已核对。Linux ARM64 的 8 个安装器均在正常注册时成功，在真实注册目录权限不符时拒绝，旁置临时文件的元数据和内容保持。该脚本回归使用日志接收替身。

六组件原 Root 上，Artemis、Camel、Container、OPC UA、Triton、YOFC Runtime 六个可选包联合实装、remove、purge；原八包、四行注册顺序、十份保留文件及客户配置原样。Position 另经真实 postrm 锁等待失败，返回 1，half-installed，载荷目录已移除、注册行保留。锁释放后在同一 Root 完成 remove 和 purge，gpsd 服务状态保持。Cloud 此版的 deb 和维护脚本已验，六组件现场的客户配置未进入它的改写流程。上述是包级验收；各 provider 的运行中业务及重加密仍待验。六组件原现场正常停机。现行原始回执目录为 `.management-close-20261116/core-rtc-hotplug-20261120/sibling-maintenance/`。

## 2026-11-20 继续实施 — 六组件内 Wires 保留式移除和分阶段重装

继续上一轮保留实际 Timer→Logger 的同一 Debian 13 ARM64 测试 Root。正常停启后核对原容器、OverlayFS 底层路径、根 inode 和 machine-id 摘要，十份被保护文件的元数据及内容在新挂载中核定。维护期间 Main disabled、未自启，Agent 保持停止且原初始化读取通过，中心原进程保持。

使用已核归档 SHA 与已装版本的正式 Wires `93f318b`。一次实际 remove 后包为 config-files，provider JAR 与注册行消失，其他三个注册行保留。以原归档一次解包，实际状态 unpacked、原 JAR 摘要配对，Wires 注册行尚未发布。随后一次正式 configure，八包恢复 installed、dpkg audit 为空，Wires 注册行准确恢复一次。三阶段内两份原快照、两份原图提交记录、两份 Core 维护历史和四份原始授权文件完整指纹保持。

显式启动后的正式 Main 从实际 Wires provider 读回两个工厂组件、两秒周期、QUIET 设定及原 0→0 连线，Logger 再次收到真实消息。探针只读配置，原图仍仅提交一次。三服务最终停止，Main disabled，容器退出 0。停机读回八包、原注册行及十份文件。

此次覆盖有实际业务的 Wires 独立保留移除和同 Root 的解包／配置续接。全套已绑定业务、其他 provider／launcher、完整 Core/OSGi、余下 typed brokers 及六组件组合生命周期/中断矩阵继续内部待办。原三现场与停机 7fd 的 41 输入／16 回执保留。冻结 12 父项 0/12，external deferred 仅原三项。见 [实装检查点](actual-native-wires-retained-reinstall-checkpoint.json)。

## 2026-11-20 继续实施 — Main 撤下 DAC_READ_SEARCH 与六组件内 Wires 保留升级

Core `735ebf0c64` 从正式 Main 的 AmbientCapabilities 和 CapabilityBoundingSet 撤下 CAP_DAC_READ_SEARCH。独立测试提交 `67d2dbe926` 让 key cohort 回归夹具带齐正式 Main 声明的辅助程序。JDK 21／Maven 3.10.1 从该生产 Git 提交构建 ARM64 deb。首次发行构建缺 Git 元数据，在同一源码树补齐后完成。定向合同与 Linux writer lease 合计 107 项通过。

沿当前六组件首装测试 Root，保留八包和原初始化/安装回执。Docker 正常停启后 root inode、machine-id 摘要和容器 ID 保持，设备号变化；在该次挂载中为本次新维护建立基线。实际正式 Main 通过 WireGraphService 的成对修订入口仅一次提交 Timer→Logger 图，确认两个工厂组件及实际消息送达后正常停止。单次真实 dpkg 原位升级 Core，八包均 installed，其他七包版本保持。新维护只增加一份原编号回执，原回执、Core AES 和 provision、Agent 初始化、中心安装记录保持。升级期间中心原进程连续，Agent 初始化的正式只读检查通过。

候选 Main 保持 disabled 和 NNP。三项剩余能力声明与五组进程掩码已核，夹具 PID 1 原本缺 CAP_DAC_READ_SEARCH，因此此次差异证明来自单元声明。新 Main 读回两个工厂配置和原拓扑，三份编辑令牌按重启语义刷新，原 Logger 再次收到实际 Timer 消息。两份快照及两份图提交记录的内容与业务属性保持，保留回拷仅改变 inode/ctime。首轮观察误用了 Logger 的 PID 格式和跨进程修订相等条件，均沿同一进程补采。dpkg 返回成功后的元数据断言同样只读补验。未重新提交图或升级。三服务正常停止，Main disabled，停机后读回八包和十份输入/记录。

本次覆盖已有 Wires 图的六组件内 Core 原位升级。已绑定业务、其他 provider／launcher、联合升级/移除/purge/中断的其余组合、完整 Core/OSGi、剩余 typed brokers 与全部 12 父项继续内部待办。原三现场、个人配置和停机 7fd 的 41 输入／16 回执保留。冻结 12 父项仍 0/12，external deferred 仅原三项。见 [实装检查点](actual-native-main-dac-read-search-wires-upgrade-checkpoint.json)。

## 2026-11-20 继续实施 — 凭据隔离 Main 与当前六组件联合实装

使用 Core `bcf8c24f5c`，从当前 Git 构建 Networking `0789499`、GWT `08712fb`、Deployment `5a6025b`、Wires `93f318b`、Agent `b9b2d3b` 与中心 `9691fe9` 的正式包，并构建中心安装器。JDK 21／Maven 3.10.1 和 Rust 1.95.0 的构建通过。Rust 首次在隔离构建中尝试同步工具链组件，在构建前退出；明确选用镜像中已安装的同版本后完成离线构建。协议 crate 从当前 Git 当次打包，进入 Agent 源码配对。

在无网络和宿主挂载的 Debian 13 ARM64 Root，六个网关包和正式安装器一次 dpkg 安装完成。中心以独立新 UUID 在正式安装器完成上传、校验、安装与 APPLIED 回执，安装阶段未自动启动中心。Agent 一次初始化，保持远程关闭。正式 Main 与 Agent、中心显式启动。Core、Networking、GWT、Deployment、FileMove、Wires 和 Agent Java bridge 七个指定 bundle 均为 ACTIVE。实际 Main 经 systemd 注入凭据完成 OSGi Crypto 加解密并持有原 Root writer lease，直接打开原始凭据记录、provision 回执与锁均拒绝。

GWT 登录入口实际返回 200，未认证控制台返回 302，两个请求用同一 TLS 证书。中心的正式安装器按原证书指纹验证本机设置页就绪。Agent 和中心的五组 capability 皆为 0，NNP 为 1。三服务正常停止，Main disabled，夹具退出 0。停机后读回八包的真实 dpkg 状态、中心安装和 Agent 初始化的原回执、Core 凭据配对以及两份程序指纹。

本次覆盖当前包的联合首装、Main 凭据隔离、指定 provider 就绪和中心设置入口。已绑定业务、其他 provider／launcher、联合生命周期与中断恢复的其余矩阵仍内部待办。原三现场和 7fd 的 41 输入／16 回执保持。冻结 12 父项仍 0/12，external deferred 仅原三项。见 [实装检查点](actual-native-masked-core-six-components-checkpoint.json)。

## 2026-11-20 继续实施 — 正式 Main 隔离原始凭据并保留加密连续性

Core `bcf8c24f5c` 将 `/etc/kura/credentials` 加入正式 systemd Main 的 InaccessiblePaths。systemd 继续加载配对凭据，Root broker 继续核对原始凭据及发放 writer lease。原配对检查保持。JDK 21／Maven 3.10.1 由 Git 构建正式 ARM64 deb，包合同和普通 launcher 相关回归 75 通过、3 跳过。

在同一 Debian 13 ARM64 隔离 Root，基准 Main 用真实 OSGi CryptoService 写入一份合成密文，正常停止后原位升级一次。升级完成的候选 Main 仍具有 CAP_DAC_OVERRIDE 和 NNP=1。进程内打开原始 AES 记录、provision 回执和锁均被拒绝，注入的凭据与原 AES 摘要相同，继承的 Root writer lease 有效。新进程用真实 OSGi 服务解密升级前的原密文，新加密往返通过。前后 HTTPS 回环请求均返回 404，服务端证书摘要相同。

首次观测因隔离系统缺 curl 超时，沿原已运行 Main 以 Python HTTPS 补采。原启动和密文写入保持一次。升级保留原凭据完整指纹、密文字节与指定属性、原包历史，仅新增一份配对维护回执。正式 Main 保持 disabled、正常停止，夹具退出 0，停机后读回 6 份文件。

本次覆盖 systemd 正式 Main 单代凭据的原位升级。多代、其他 launcher、剩余 Core 权限收敛、typed brokers、全部 provider 和六组件生命周期仍为内部待办。原三现场和 7fd 的 41 输入／16 回执保留。冻结 12 父项 0/12，external deferred 仅原三项。见 [实装检查点](actual-native-core-main-credential-isolation-checkpoint.json)。

## 2026-11-20 继续实施 — PKCS#11 共享令牌的 PIN 校验与既有会话

Core `9d490f6195` 使用独立 JVM 验证显式 PIN，PIN 通过匿名管道传递。错误 PIN 或已启动后结果未明的校验阻止后续读取，直到认证配置改变。认证前确认不可用可以重试。JDK 21／Maven 3.10.1 的 88 项 keystore 测试通过，正式 ARM64 deb 由该 Git 树顺序构建。

独立 Debian 13 ARM64 Root 一次安装正式 Core 和已校验的 SoftHSM2。正式 Main 内两个不同 OSGi provider 分别按槽号和实际槽索引访问同一令牌。先同一配置计划创建两者并将 Jetty 绑定到 B。再以一份新计划配置 A 的错误 PIN，两次读取均拒绝，只观察到一次原生 PIN 拒绝。B 的原不可导出私钥句柄继续签名。第三份合法计划恢复 A；两个 OSGi service ID、Main PID、出生时间、InvocationID、B 与 Jetty 的配置修订均保持。三个阶段均用原证书完成 TLS 1.2 和 TLS 1.3 并得到 HTTP 响应。

首次读取令牌指纹把 token.object 与证书和私钥对象等同，因元数据变化退出。核定原创建计划已提交后仅补采原观察，原创建未重派。证书与私钥对象文件精确保持，token.object 的变化单独留证。三个计划各有原四文件回执，前序快照与回执逐项保持。Main 保持 disabled 且正常停止，夹具退出 0；停机后读回 21 份文件，合成 PIN 和探针留在该 Root 私有目录。

本次覆盖显式 PIN 的共享软件令牌。受保护认证路径、令牌拔出或替换、全部 provider 重加密、完整 Core/OSGi、其余 typed brokers、六组件/provider/launcher 与生命周期矩阵继续内部待办。原三现场及 7fd 的 41 输入／16 回执保持。冻结 12 父项最终 0/12，external deferred 仅原三项。见 [实装检查点](actual-native-core-pkcs11-shared-slot-checkpoint.json)。

## 2026-11-20 继续实施 — PKCS#11 签名 CRL 的离线冷启动

复用实装 Core `4535c821c6` 的原隔离 Debian 13 ARM64 Root。原容器重新启动后先核对设备号、Root inode、machine-id 摘要、原已装包及 14 份受保护文件的完整指纹，再显式启动禁用状态下的正式 Main。回环 CRL 源端口全程拒绝连接，配置探针和临时 PIN 文件继续移出运行目录。没有配置派发、令牌重建或 CRL 发布。

新 Main 的日志记录一次原 CRL 载入。首次观察到两种协议均接受未吊销客户端时，TLS 1.2 与 TLS 1.3 都拒绝被吊销客户端、缺证书及不受信客户端。未吊销客户端验证原服务器证书并得到 HTTP 响应，普通 HTTPS 可用。沿用原计划四份持久记录、快照、令牌对象和缓存指纹。正式 Main 保持 disabled，正常停止后再次停放原容器，退出 0，停机后读回 14 份文件。

共享槽位、令牌拔出或替换、全部 provider 重加密、完整 Core/OSGi、其余 typed brokers 和完整六组件生命周期继续内部待办。冻结父项仍 0/12，external deferred 仍仅原三项。原三现场及 7fd 的 41 输入／16 回执保留。见 [离线冷启动检查点](actual-native-core-pkcs11-crl-offline-restart-checkpoint.json)。

## 2026-11-20 继续实施 — 正式 Main 的 PKCS#11 签名吊销与 Jetty 双向 TLS

隔离 Debian 13 ARM64 Root 用 JDK 21 实装 Core `4535c821c6` 与 SoftHSM2 2.6.1-3。首个 Core 安装在解包时因 ENOSPC 明确退出 1。确认未安装、Main 未启动、原凭据齐全后，磁盘有空间时将同一已核摘要的包在同 Root 续接一次，退出 0。dpkg audit 清空，已装 keystore bundle 与原 Git 配对制品摘要一致。此为本次实装故障恢复，磁盘满／硬断电完整矩阵仍按原台账处理。

从新建的合成令牌生成服务端私钥。安全官登记客户端 CA，JDK 实际读为受信条目。三个探针启动因夹具输入将换行写成字面字符而在提交前退出，记录保留。确认 journal 只有锁文件后，修正输入并沿原计划 UUID 在正式 ConfigurationRevisionService 提交一次。结果 CORE_COMMITTED，四份持久记录。真实 PKCS#11 OSGi 服务在 CRL 更新前后分别以不可导出私钥签名并验证，确认同一受信 CA 和同一服务。

实际 Jetty 使用签名校验和 CRL_ONLY。在空 CRL 生效时，两名受信客户端在 TLS 1.2 和 TLS 1.3 下均成功，缺证书和不受信证书拒绝。回环源一次发布同 CA 签名的吊销列表。两种 TLS 均拒绝被吊销客户端，另一受信客户端及普通 HTTPS 继续完成 HTTP 请求。PID、出生时间、InvocationID、OSGi 服务、原快照及四份配置记录保持。即时磁盘观察早于缓存落盘，稍后只读核对确认磁盘 DER 摘要与 OSGi 所读吊销列表一致。令牌证书和私钥对象文件指纹保持。Main 禁用并正常退出，新夹具停放后读回 14 份文件，所核对日志、快照和回执无明文 PIN。

本次覆盖单枚软件令牌的运行期 CRL 分发。PKCS#11 缓存冷启动、共享槽位、令牌拔出和替换、全部 provider 重加密、完整 Core/OSGi、其余 typed brokers、六组件/provider/launcher 和全部生命周期矩阵继续内部待办。原三现场及 7fd 的 41 输入／16 回执保留。冻结父项 0/12，external deferred 仅原三项。见 [实装检查点](actual-native-core-pkcs11-crl-checkpoint.json)。

## 2026-11-20 继续实施 — PKCS#11 同槽位 PIN 变更必须重新认证

旧 Core `96cd580d8a` 在隔离 Debian 13 ARM64 中一次提交同槽位错误 PIN，OSGi provider 仍可打开原令牌。Jetty 实际刷新后，TLS 1.2 和 TLS 1.3 仍返回原证书。OpenJDK SunPKCS11 对已登录令牌会跳过再次登录。首次观察探针因未声明 KuraException OSGi 导入失败，未派发配置，原探针归档后修正。

Core `4535c821c6` 在库、槽位或 PIN 改变时退出现有 provider 的令牌会话，退出失败时阻止加载。SunPKCS11 明确拒绝 PIN 后，保留该凭据配置的失败状态，后续消费者读取不重试原生登录。Keystore Maven 86 项通过。JDK 21/Maven 3.10.1 从 Git 构建正式 ARM64 deb，已装 keystore bundle 与构建摘要一致。

第二个独立 Root 正式实装修正版。一次原错误 PIN 配置提交后，实际 provider 返回 SECURITY_EXCEPTION。Jetty 刷新后两种 TLS 新连接均拒绝。后续消费者读取及 Jetty 刷新共只观察到一次原生 PIN 拒绝。随后一份新的合法配置通过原修订号检查，沿同一 Main、同一 OSGi 服务恢复原证书的 TLS 1.2/1.3。原错误请求未重发。两个 Root 均正常停止并保持 Main disabled。停机后读回 25 份回执和快照，配置内容只有加密 PIN 改变，Jetty 配置条目字节一致。令牌原证书和私钥对象保留。

这次实装覆盖每 Root 一个 SoftHSM 令牌与一个 provider。共享槽位、令牌拔出或替换、物理令牌状态和 CRL 分发仍待验证。完整 Core/OSGi、其余 typed brokers、六组件矩阵和所有 12 父项继续内部待办。原三现场及 7fd 的 41 输入／16 回执保持，闭环 0/12，external deferred 仅原三项。见 [实装检查点](actual-native-core-pkcs11-pin-revalidation-checkpoint.json)。

## 2026-11-20 继续实施 — 正式 Main 的 PKCS#11 令牌运行期切换

独立 Debian 13 ARM64 Root 实装 Core `96cd580d8a` 与已验签 SoftHSM2 2.6.1-3，用两个不同槽位、不同 PIN 和不同证书的独立软件令牌。真实 Main 一次创建 PKCS#11 配置并绑定实际 Jetty。TLS 1.2 和 TLS 1.3 均在 18443 完成证书校验和 HTTP 请求，首枚证书与原令牌一致。

同一 Main 内第二份正式配置计划只更新该 provider 的 slot 和 AES 包装 PIN。原配置计划保持，收到实际 keystore 变更事件。Main PID、出生时间、systemd InvocationID、OSGi service.id 和 Jetty 配置 revision 一致。两个协议新连接均验证到第二枚令牌证书并获得 HTTP 响应。两份计划各仅派发一次、各有四份持久文件，先前快照和回执完整指纹保持。两套证书与私钥对象及 Core 凭据保持，所核对快照、回执及日志无两份明文 PIN。探针在最后误用 Java 对象实例等价核对回执，随后只读比对原回执完整指纹并核对实际 TLS，未重新提交。正式 Main 与隔离现场正常停止，Main 保持 disabled。停机后读回八份回执和两份快照，只有 provider 的 slot 和加密 PIN 改变，Jetty 配置条目字节一致。

本轮使用此前完整 Git 配对构建的同一正式包。此次验证覆盖运行中进程及绑定消费者切换，TLS 读回在探针修正后采样，尚无完整切换期间的可用性测量。令牌拔出、无效 PIN、PKCS#11 CRL 分发、全部 provider 重加密、完整 Core/OSGi、余下 typed brokers 及完整六组件矩阵继续内部待办。原三现场及 7fd 的 41 输入／16 回执保持。冻结父项 0/12，external deferred 仅原三项。见 [实装检查点](actual-native-core-pkcs11-live-tls-switch-checkpoint.json)。

## 2026-11-20 继续实施 — 正式 Core Main 的 PKCS#11 令牌与配置连续性

隔离 Debian 13 ARM64 Root 从 Debian 签名索引校验 SoftHSM2 2.6.1-3 正式包。旧 Core `96ffee3741` 的一次正式配置计划成功提交，但实际 SCR 激活未获得 provider。原日志显示默认 CRL 路径读取了尚未创建的 PKCS#11 选项。首次 SoftHSM 令牌准备因 `kurad` 无法读取配置而停止。只读确认令牌目录为空后，测试夹具将该配置的读取组配给 Main，沿原 PIN 输入完成首次令牌初始化。

Core `96cd580d8a` 先加载 PKCS#11 选项再初始化或更新 CRL manager。令牌配置变化时使旧 manager 退休并发出既有 keystore 变更事件。metatype 只保留一份 CRL 路径属性。从 Git 用 JDK 21/Maven 3.10.1 生成正式 ARM64 deb，在原 Root 实际升级。已装 JAR、正式包和本次 Git 构建摘要相同。

旧 Main 正常停机后把合成 PIN 输入移到 Root 私有目录。修正版两次正式 systemd 启动都通过原配置及 JDK SunPKCS11 读取同一令牌证书和不可导出的私钥，各一次签名由对应公钥验证。KeyManager 可用。升级回拷使四份配置记录、快照和三份令牌对象的 inode 及 ctime 变化，八份字节、属主、模式、长度和 mtime 保持，Core 凭据完整指纹保持。两次启动之间快照、回执及证书和私钥对象完整指纹保持，SoftHSM 的 token.object 元数据内容在使用期间更新。原 PIN 未出现在快照、回执及所核对日志。两次 Main 正常停止并保持 disabled，隔离现场正常停放。keystore Maven 84 项通过。

活跃消费者下的令牌更新、拔出和 CRL 分发，以及全部 provider 加密归档与重加密、完整 Core/OSGi、其余 typed brokers、六组件矩阵和全部 12 父项继续内部待办。原三现场及 7fd 的 41 输入／16 回执保留，闭环 0/12，external deferred 仍仅原三项。见 [实装检查点](actual-native-core-pkcs11-keystore-checkpoint.json)。

## 2026-11-20 继续实施 — 标准防火墙启动器配置与原回执续接

标准版旧 Networking `3e15d6a` 在隔离 Debian 13 ARM64 Root 的一次正式 systemd 启动中，已经写入两族站点规则并归档了站点回执，随后创建 IPv4 `input-kura`，把父链推导为无效的 `INPUT KURA` 而退出 1。实际 IPv4 链存在但未链接，其他配置尚未执行，原现场正常停放且未再次派发。

Networking `0789499` 修正父链解析，允许将站点已经定义而尚未链接的链挂接，保留链内规则。标准配置将两族站点恢复、filter/nat/mangle 配置、转发与站点扩展纳入同一持久原编号和包数据库锁。标准阶段绑定实际 launcher、shell、工具与站点扩展，派发前写入意图，成功后记录两族规则及转发的最终证据。最终证据持久后可用同一编号补归档。

新隔离 Debian 13 ARM64 Root 从 Git 实装 Core `96ffee3741` 与正式标准 Networking。只在隔离网络命名空间开放实际两份转发 sysctl 写入，其他 proc/sys 保持只读。一次正常启动验证两族 filter/nat/mangle、站点原规则保留及实际转发改为 IPv4 0、IPv6 1，站点扩展执行一次。另一原操作在标准阶段完成证据的目录 fsync 返回后被实际中断。经正式 systemd 原 Root、原 boot、原命名空间续接，原回执摘要一致，两族规则和转发一致，站点扩展没有再次执行。

第三个独立操作在实际标准配置子进程返回 0 后、持久标准完成回执落盘前中断。正式 launcher 与真实 `dpkg --remove` 均拒绝，原编号、未决、之前回执、内核规则、转发和扩展计数保持。旧夹具正常停放。候选夹具停止超时，Docker 随后发 SIGKILL，终态 137。停机后只读核对了原 pending、两份已归档回执和扩展计数，字节一致，未重启。两个夹具的 Core Main 全程 disabled 且未启动。JDK 21/Maven 3.10.1 构建两种正式 deb，Networking Maven 2239 项，宿主和 ARM64 包装各 12 项通过。

未决记录的原证据结算、跨 boot、其余 provider／launcher 和维护范围、完整 Core/OSGi、其余 typed brokers 及完整六组件矩阵继续内部待办。原三现场和 7fd 的 41 输入／16 回执保持，冻结父项 0/12，external deferred 仅原三项。见 [实装检查点](actual-native-standard-firewall-config-recovery-checkpoint.json)。

## 2026-11-20 继续实施 — 防火墙启动恢复与 Debian 包维护互斥

Networking `3e15d6a` 为两种正式防火墙启动器增加实际 dpkg 数据库锁，并在两种 deb 的四个维护脚本中内嵌同一准入检查。检查读取原 Root 的记录目录，有未决记录时拒绝包维护。保留未决记录的 postinst abort 以无副作用返回，避免自动回退操作再次修改现场。从 Git 使用 JDK 21、Maven 3.10.1 构建两种正式 deb，Networking Maven 2239 项、宿主和 ARM64 包装各 10 项通过。

独立 Debian 13 ARM64 Root 实装 Core `96ffee3741` 和新 firewall-only。在已装启动器持有数据库 POSIX 读锁且未写入 pending 时，真实 `dpkg --remove` 退出 2。继续同一启动器，两族规则正常生效，只归档一份完成回执。

对该新 Root 的另一个独立操作，在真实 IPv6 子进程返回 0 后、持久完成回执写入前中断。用同代码较新版本的正式 deb 真实升级，旧、新 prerm 均拒绝。真实 remove、purge 也在准入阶段退出 1。原包仍为 installed，dpkg 期望状态仍可记录请求的 deinstall 或 purge。已装维护脚本、启动器及 helper、原未决和先前完成回执共八份指纹精确，站点输入、两族内核规则和 Core 保护文件精确。同期标准 deb 的原始 preinst 在同一 Root 单独执行准入检查时同样拒绝，尚待标准版完整实装正向和中断验证。Main 全程 disabled 且未启动，dpkg audit 无输出，新夹具正常停放。

未决结果的原证据结算仍待完成，无重派。完整 Core/OSGi、其余 typed brokers、provider／launcher、六组件及剩余维护矩阵继续内部待办。原三现场、个人文件和 7fd 的 41 输入／16 回执保留。冻结父项 0/12，external deferred 仅原三项。见 [实装检查点](actual-native-firewall-boot-package-guard-checkpoint.json)。

## 2026-11-20 继续实施 — 防火墙启动器保留已完成地址族并拒绝重派未决步骤

Networking `6a98ed1` 在两种正式启动器共用的站点规则恢复路径中记录原意图。私有记录绑定 Root、boot、网络命名空间、原文件指纹、实际工具和 helper。每族派发前同步意图，成功退出和内核读回后同步完成。恢复时已完成族须匹配内核读回，只派发尚无意图的族。已有派发意图而缺完成证明时保留原编号并拒绝写入。JDK 21 和 Maven 3.10.1 从 Git 构建两种正式 deb，Networking Maven 2239 项、宿主和 ARM64 包装各 8 项通过。

Debian 13 ARM64 独立 Root 实装 Core `96ffee3741` 和新 firewall-only。跟踪已装启动器，在 IPv4 完成回执所在目录的实际 fsync 成功返回后终止进程，此时 IPv6 尚未派发。由正式 systemd `firewall.service` 沿同一 Root、boot 和网络命名空间续接，两个地址族读回正确，原编号归档一份完成回执。IPv4 原规则的测试计数保持，未重派 IPv4。观测脚本起初直接比较了会随流量变化的内建计数，随后对原规则、结算文件和真实计数只读补验，未重派启动。

同一新 Root 上的另一个独立规则对，在真实 IPv6 执行进程返回 0 后、启动器写入完成回执前中断。已审阅的合成规则实际进入两族内核，产品记录保留原 IPv6 issued 状态。实际 vendor unit 拒绝续写，原 pending、之前的完成回执、两族规则和站点文件完整指纹保持。现场正常停放，Main 全程 disabled 且未启动，dpkg audit 无输出。

已派发步骤的显式原证据结算、带未决记录的包维护准入、真实重启、标准 launcher 完整正向实装和其余中断点仍待验证。完整 Core/OSGi、其余 typed brokers、provider 和六组件矩阵继续内部待办。原三现场、个人文件和 7fd 的 41 输入／16 回执保留。冻结父项 0/12，external deferred 仅原三项。见 [实装检查点](actual-native-firewall-boot-write-recovery-checkpoint.json)。

## 2026-11-20 继续实施 — 两种防火墙启动器在写入前核对双族站点规则

独立 Debian 13 ARM64 Root 安装 Core `96ffee3741` 和旧 firewall-only 正式包。用单次实际 `firewall.service` 调用复现：合法 IPv4 已写入内核，随后无效 IPv6 使单元失败，IPv6 保持原规则，两份站点文件指纹精确。该旧结果完整保留。

Networking `76825d6` 让标准和独立两种正式包共用站点规则前置检查。只从固定 Root 所有目录读取普通、非链接且无非 Root 写权限的文件。先将两族规则送入各自真实 `--test`，重核文件及防火墙归属，然后使用校验过的原字节。标准版根据同一份数据计算转发选择。JDK 21/Maven 3.10.1 从 Git 构建两种正式 deb，Networking Maven 2239 项、ARM64 包装 6 项通过。

同 Root 只升级 firewall-only，Core 凭据、能力记录、快照和站点文件保持。单独实测两次拒绝：IPv6 规则损坏，以及 IPv6 路径为符号链接。两族均未发生写入，现有文件及链接目标保持。随后合法规则对经实际 launcher 生效，两个表与站点文件匹配，独立哨兵链保留。

沿同一 Root 实际移除隔离 firewall-only 并安装同期标准 Networking。两族规则和站点文件保持，NetworkManager 进程身份及 Core 凭据保持。标准版实际 `firewall.service` 拒绝错误 IPv6 时，两族规则和转发开关精确。整个隔离现场 Main 保持 disabled 且未启动，dpkg audit 无输出，夹具正常停放。

本轮覆盖写入前拒绝。实际写入途中故障的结果核定、标准 launcher 正常完整启用、其余 provider／launcher、Core/OSGi、typed brokers 和六组件矩阵继续内部待办。原三现场、个人文件和 7fd 41 输入／16 回执保留。冻结父项 0/12，external deferred 仅原三项。见 [实装检查点](actual-native-firewall-dual-family-boot-preflight-checkpoint.json)。

## 2026-11-20 继续实施 — Core 与旧 firewall-only 原位联合升级及 launcher

Core `96ffee3741` 增加独立 `KURA_FIREWALL_ONLY_COHORT_SHA256`。同一 Root 及挂载命名空间内的实际 dpkg 仍需传入有序批准归档。单独核对已装变体、新归档包名及版本、独立 modem 证明和 JAR 指纹。混入标准 Networking、BIND 或 NetworkManager 资产会拒绝。实际两种归档分别通过自身检查，反向交叉均拒绝。69 项 ARM64 回归通过，JDK 21、Maven 3.10.1 从 Git 构建 Core ARM64 正式包。

在独立 Debian 13 ARM64 Root 实装旧 Core `2a3b588`、旧 firewall-only `4ee16c9`、GWT、Deployment、Wires。旧防火墙辅助脚本只创建私有规则目录，站点规则保持原路径。首次观测误预期会复制站点文件，按实装行为核对空私有目录和两份原站点文件后继续。两族隔离内核已各有独立哨兵链。

一次真实 dpkg 联合换版后，Core 和 firewall-only 各一次 upgrade，零 remove，dpkg audit 无输出。标准 Networking 未安装。两份站点规则、能力记录、两族 filter 表、两份 Core 凭据及旧包历史精确。快照与合成用户数据的内容和保留元数据一致，原编号新增一份回执。私有规则目录仍空，Main 未启动。实际已装 modem 证明以 kurad 核对通过。

显式启动一次实际 `firewall.service`，按两份站点文件独立加载 IPv4／IPv6 规则，原站点哨兵链保持，Main 未被连带启动。之后实际 Main 保持 NNP，Core、GWT、Linux 网络及 firewall 管理四个 bundle ACTIVE。两个 OSGi 防火墙配置 API 分别返回 IPv4 和 IPv6 配置，均由 firewall-only 的正式 bundle 提供。固定已装叶证书验证 GWT 200／302。站点规则文件和哨兵链保持，Main 正常停机并维持 disabled，夹具正常停放。

其他 provider／launcher、换版后更多中断点、实际内核重启或 Root 重挂、已绑定业务、完整 Core/OSGi、余下 typed brokers 及完整六组件矩阵继续内部待办。原三现场、个人文件和 7fd 的 41 输入／16 回执保留。冻结父项 0/12，external deferred 仅原三项。见 [实装检查点](actual-native-firewall-only-direct-upgrade-checkpoint.json)。

## 2026-11-20 继续实施 — Core 与 Networking 联合升级中断及 soft-reboot 续接

独立 Debian 13 ARM64 Root 实装旧组合。对正式 Core `910493756d` 与 Networking `6ab7101` 发起联合升级，在新版 Networking preinst 的内核 exec 停点中断。Core 已 unpacked，Networking 仍为旧版 half-installed。新版 preinst 正文未进入，原 dpkg 和该子进程均确认退出。九份新版证明尚未写入，三份 Core 原围栏、恢复副本、凭据和旧包历史保持。实际 Main 配对检查拒绝该中间状态。

在同一 Root 和同一 boot 仅解包尚未执行的原批准 Networking 归档。两包实际 unpacked，原围栏与恢复副本完整指纹保持。随后用 dpkg configure 配置两包，Core 未重新解包，原编号只新增一份回执。九份 provider 证明、两份凭据、旧包历史及用户文件和快照通过核对。两包 installed，dpkg audit 无输出。Networking 日志有两条开始记录，首条停在新 preinst 正文前。

恢复后显式启动实际 Main。Core、GWT、NM、Linux 网络及 systemd provider 五个 bundle ACTIVE，固定已装叶证书验证 GWT 200／302。合法 Main 的 OSGi 探针通过真实客户端完成 BIND status、DHCP scope、Modem ready-gps 三项只读请求。三处 broker 台账无新增文件，named 的 PID 与 InvocationID 保持。Main 正常停止且禁用，探针移出加载路径，隔离 Root 正常停放。

第二处在两包载荷已解包、新 Core postinst 正文尚未执行时中断。一次真实 systemd soft-reboot 后，Root、原围栏和十三份证明精确，Core 配置实际返回 UNIT_RUNNING。持久启用的旧 clock／reboot 监听已恢复，broker 安装阶段有原意图而缺成功证明。保留原号、原围栏和这份未结算状态，正常停放原试验 Root。

Core `58942a2843` 在发布 broker 配置意图前，用已核对的新版 helper 检查原时钟及重启维护记录，仅静默已配对的监听器。已派发的阶段仍按原成功证据处理。67 项 ARM64 回归通过，JDK 21 和 Maven 3.10.1 从 Git 构建新正式 ARM64 deb。独立 Root 实测相同 postinst exec 中断及一次 soft-reboot，D-Bus InvocationID 更新，内核 boot ID、容器启动记录、Root 设备及 inode 保持。十三份围栏和 provider 证明精确，原编号仅以 dpkg configure 完成，两份归档均未重解包。四个实际监听器在维护收尾重新监听。原凭据、快照、旧历史及九份 provider 证明核对通过，dpkg audit 无输出，Main 全程未启动，现场正常停放。

真正的内核重启或 Root 重挂、其他 provider／launcher、补充中断点、已绑定业务、完整 Core/OSGi 和其余 typed brokers 继续内部待办。原三现场、个人文件和 7fd 的 41 输入／16 回执保留。冻结父项 0/12，external deferred 仅原三项。见 [实装检查点](actual-native-direct-core-networking-interruption-checkpoint.json)。

## 2026-11-20 继续实施 — 旧 Networking 与 Core 单次 dpkg 原位联合换版

Core `910493756d` 增加显式的联合换版准入。旧 Networking 缺少未由旧包声明的证明时，仅从实际调用新 Core preinst 的同 Root、同挂载命名空间 dpkg 进程识别有序归档。核对 Core 目标版本、显式审批的 Networking SHA-256、九个 typed provider 合同及只读 ModemManager 策略。已装包声明但缺失的文件、错误证明、游离 provider 仍拒绝。Main 继续按实际已配置包及文件配对。

独立 Debian 13 ARM64 Root 实装旧 Core、Networking、GWT、Deployment 与 Wires。JDK 21、Maven 3.10.1 从当前 Core Git 建成正式 ARM64 deb。新版 Networking 归档实际只读校验通过，错误摘要实测拒绝。单次真实 `dpkg --install` 以 Core 后接 Networking 的顺序成功。日志中各有一次 upgrade、零 remove，dpkg audit 无输出。GWT、Deployment 和 Wires 保持原已装版本。

升级期间 Main 保持 disabled 且未启动。两份 Core 凭据及原包历史精确，合成用户数据和实际快照的内容、模式、属主、组、链接数、大小、mtime 保留。新增一份原维护编号的完成回执。以 kurad 身份核对已装 typed provider 成功。

实际 systemd Main 显式启动后保持 NNP，JDK 21 探针核对 Core、GWT、NM、Linux 网络和 systemd provider 五个 ACTIVE bundle。固定已装叶证书验证 GWT 认证入口 200 和未认证入口 302。原 systemd stop 成功，Main PID 退出且保持 disabled。观测器误期望 inactive 后仍保留启动计时，按同一 stop 的退出码、systemd 结果及 PID 补验，未重派启动或停止。探针移出自动加载，隔离 Root 正常停放。Debian ARM64 75 项针对回归通过。

直接联合升级的中断和跨 boot、其他已授权／已绑定业务、provider／launcher、完整 Core/OSGi、剩余 typed brokers 和完整六组件矩阵继续内部待办。原三现场、个人文件及 7fd 的 41 输入／16 回执保留。冻结父项 0/12，external deferred 仅原三项。见 [实装检查点](actual-native-direct-core-networking-upgrade-checkpoint.json)。

## 2026-11-20 继续实施 — 未绑定 Agent 的 Java bridge 换版及原生组合有序升级

历史六组件隔离现场挂载后设备号改变，原 Agent 初始化证明没有 OverlayFS 底层来源信息。已装代码拒绝续接，旧证明字节精确、三服务零启动，现场正常停放。使用独立 Debian 13 ARM64 Root 安装同版八个旧正式包，并在该 Root 同一次 boot 内合法初始化和验证。

Agent `e5cb79d` 增加显式的 `prepare-unbound-java-bridge-update`。只接受远程关闭、未绑定且停机的已初始化 Agent。原初始化编号、审批归档、bridge 身份和协议、原 Core loader、Core 单元及版本、同一停机 Main、物理目录、Root 和 boot 在准入及发布时核对。旧普通入口实测拒绝 bridge 变化，显式入口后实际 dpkg 以原编号结算为 APPLIED。两份 Core 凭据、六份 Agent 文件和三份中心文件精确。正式包源自 Agent `e5cb79d`，218 项 Rust 编译输入一致后复用已核对 ELF。针对性回归 24 项通过。独立测试提交 `b9b2d3b` 修正旧故障注入参数，Debian ARM64 包装套件 326 项中 324 项通过、2 项跳过。

直接联合 dpkg 时，新 Core 要求 typed Networking 证明，新 Networking 依赖新 Core broker。Core 保持旧版已装，旧 Networking 留在 unpacked，GWT 和 Deployment 已升级。沿同一 Root 核对原三份围栏、旧数据和历史后，实际配置并移除旧 Networking，再用原围栏完成批准的 Core 升级并安装新 Networking。原 Core 编号只增加一份关联回执，Core 凭据、旧历史及原 Agent 证明精确。合成用户文件的内容、模式、UID/GID 与原修改时间保持。

中心工具与中心沿正式维护入口升级，原中心编号 COMPLETE。八包均已安装，七包变更版本，Wires 已是匹配版本，dpkg audit 无输出，三服务均未自启。显式启动后，JDK 21 探针在实际 Main 核对六个 ACTIVE OSGi bundle，包括已审阅的 Agent bridge。GWT HTTPS 按已装叶证书固定核对 200／302，中心 READY。三个进程均启用 NNP，全部正常停止并维持 disabled，隔离 Root 正常停放。

直接原位联合换版、已绑定和认证业务、其余 provider／launcher 与中断点、完整 Core/OSGi、余下 typed brokers 和完整六组件矩阵继续内部待办。原三现场、个人文件及 7fd 的 41 输入／16 回执保留，未重派 UNKNOWN。冻结父项 0/12，external deferred 仅原三项。见 [实装检查点](actual-native-unbound-java-bridge-cohort-upgrade-checkpoint.json)。

## 2026-11-20 继续实施 — 全 purge 后联合复装与解包态重启续接

Core `015e6dce60` 移除 dpkg 首装时对 `/opt/eclipse/kura*` 的清理，独立 provider 状态由各自软件包维护。自解包入口只按原 Core 链接和历史目录名检查旧安装。同一 purge 现场先用旧安装器头部复现旧路径拒绝。JDK 21、Maven 3.10.1 从当前 Git 构建新正式 ARM64 deb 和自解包。实际新自解包保留六个 Deployment 目录和三份文件完整指纹，Core 写入原移除编号的正常包回执，保留的 AES 不变。Linux ARM64 25 项通过，macOS 24 项通过、1 项平台跳过。

随后实际安装四个 Core 依赖包及中心工具。Agent 和中心沿原移除意图同时实际进入 unpacked。单次正常停启后，系统管理器及 D-Bus 实例更新，内核 boot ID、容器和根 inode 均保持，OverlayFS 设备号改变。使用已装 Agent 的原挂载配对验证底层 OverlayFS 摘要，22 份原围栏和文件只变更设备号，其余指纹精确。三服务启动计时均为 0。现有产品配对通过后，实际 dpkg configure 完成，同一 Agent 编号为 REINSTALLED，同一中心编号为 COMPLETE。八包均为 installed，dpkg audit 无输出。

显式启动三服务后，GWT 按本次已装叶证书指纹核对 HTTPS，认证入口 200，未认证控制台 302，中心 READY。Journal socket 实际 active，FileMove socket 已装并维持 disabled。观测器初次误等 FileMove 自动启动，沿已运行进程补采，三项启动均未重派。三服务正常停机、禁用，隔离现场正常停放。合成 Core 用户数据在上一轮 purge 已清除，本轮保留的是原凭据与维护历史。

该次实装覆盖同一 OverlayFS 上完整 purge 后复装和双组件 unpacked 的正常停启续接。联合换版、已绑定和认证业务、其他中断点及 provider／launcher、完整 Core/OSGi、其余 typed brokers 与全部 12 父项仍内部待办。原三现场、个人文件和 7fd 的 41 输入/16 回执保留。冻结父项 0/12，external deferred 仅原三项。见 [实装检查点](actual-native-post-purge-reinstall-reboot-checkpoint.json)。

## 2026-11-20 继续实施 — 联合重装后的 GWT 就绪与八包完整 purge

续接前一原生组合的同一隔离 Debian 13 ARM64 Root。真实 Docker 停启后，容器 ID、底层挂载信息、根目录设备和 inode、machine-id 摘要及原四份 Agent 回执精确。原 Agent 和中心回执仍为 REINSTALLED 和 COMPLETE。按正常 systemd 入口显式启动 Agent、Main 与中心。真实 HTTPS 按已装 Core keystore 的叶证书 SHA-256 固定核对，GWT 认证入口响应 200，未认证的控制台入口响应 302。正式中心安装器返回 READY，三个进程均为 NNP。

中心仍安装时，实际 dpkg 拒绝提前 purge 主机安装器，原三个进程、安装器文件和原记录保持。沿独立的本轮合法审核目的建立中心及 Agent 围栏，沿原已初始化资料停机。真实 dpkg 按 Agent、中心、Deployment、Wires、GWT、Networking、Core、安装器的顺序完成 purge。八包均为 not-installed，dpkg audit 无输出。三服务单元、七个核对的发行文件及已装工具入口不再存在。隔离测试用例的合成 Core 用户数据按 purge 语义清除。

Core、Agent 和中心共十份保留输入、前轮五份 Agent 与中心记录、原 Core 包历史的指纹精确。三个新 Core 移除围栏、四份新 Agent 记录及原编号中心 STOPPED 回执保留。同一隔离 Root 正常停放。生产源码无变化。

本轮覆盖同一组合重装后的 GWT 未认证入口、已初始化未绑定 Agent、无 PostgreSQL 任务中心及完整八包 purge。全 purge 后联合重装、联合换版、已绑定业务和其余中断、provider、launcher、Core/OSGi、typed brokers 继续内部待办。原三现场、个人文件和 7fd 41 输入/16 回执保留。冻结 12 父项最终 0/12，external deferred 仅原三项。见 [实装检查点](actual-native-cohort-gwt-ready-and-purge-checkpoint.json)。

## 2026-11-20 继续实施 — 原生联合发行保留式移除与同 Root 重装

Debian 13 systemd 257 在未安装中心且没有命名实例时，`list-unit-files` 实际返回 1，stdout 和 stderr 均为空。原正式安装器因此拒绝进入包概览。console `6798af7` 识别这一种空清单，继续核对持久和运行期链接、已加载实例和保留实例目录，真实系统错误仍上报。Debian ARM64 包装回归 139 项通过。由当前 Git 构建正式中心及安装器，207 项编译输入精确后复用现有正式 ARM64 ELF 和 Web 文件。

沿原隔离 Root 只更换安装器，原网关包和受保护输入保持。真实入口返回唯一批准的中心归档及未安装的默认单元，上传和安装各派发一次。当前 Core、Networking、GWT、Deployment、Wires、Agent、中心安装器和中心八包齐备。实际独立 Agent、合法 Main 与默认中心同时运行。直接移除 Core 时，dpkg 因四个依赖包拒绝，三个原进程、八份输入及包状态精确，未创建 Core 围栏。

通过正式中心维护及 Agent 保留移除入口完成原围栏。Agent 第一次审核归档因位于可写的 `/var/tmp` 父目录拒绝，未创建意图。将同 SHA 归档放入隔离 Root 的私有路径后沿原编号完成审核。之后 Agent 和中心实际 remove、purge，Core 和四个依赖包实际联合 remove。Core 三个围栏、保留数据、Agent 和中心原编号、受保护文件精确。中心安装器维持安装。

沿同一 Root 从相同正式归档重装。原 Agent 回执 REINSTALLED，原中心回执 COMPLETE，原 Agent 四份回执精确。Core 保留的合成业务文件及两份密钥，另有五份 Agent 和三份中心文件核对。三类服务在重装配置后都未自启，经显式启动观察到在运行的进程，随后正常停止，隔离 Root 正常停放。

本轮覆盖无 PostgreSQL 任务和未绑定 Agent 的联合发行路径。网关组与安装器的完整 purge、联合换版、已注册生产式业务、重装后的 GWT 业务就绪和跨阶段中断、完整 Core/OSGi、其余 typed brokers、六组件矩阵和全部 12 父项仍内部待办。原三现场、个人文件及 7fd 41 输入/16 回执精确。冻结父项 0/12，external deferred 仅原三项。见 [实装检查点](actual-native-combined-package-retained-reinstall-checkpoint.json)。

## 2026-11-20 继续实施 — Debian 正式 Core 的双向 TLS、动态吊销与离线缓存续接

新 Debian 13 ARM64 无外网隔离 Root 安装当前正式 Core `f6beac005c` 的 Debian 包，当前 Core `b0ac48556b` 仅有测试变更。四份已装 Equinox、配置、HTTP 和 keystore JAR 与归档 SHA-256 精确。实际 systemd Main 由 kurad 在 NNP 下运行。正式 OSGi 配置计划仅派发一次，创建文件 keystore，切换 Jetty 到独立测试端口和严格 CRL_ONLY。配置回执为 CORE_COMMITTED。真实 TLS 使用隔离测试 CA，客户端核对服务端证书链、主机名和叶证书摘要。

初始 CRL 下，两位受信客户端均收到实际 HTTP 404。双向 TLS 拒绝无证书及非受信客户端，普通 TLS 仍可用。回环 CRL 源一次原子发布新版签名 CRL，实际 provider 加载，Jetty 在同一 Main PID、出生时间和 InvocationID 内刷新。被吊销客户端在 TLS 1.2 与 TLS 1.3 均拒绝，另一位受信客户端仍可访问。原四份配置记录与四份受保护文件精确。

新版 CRL 索引和 DER 文件落盘。正常停 Main、移出探针并关闭回环 CRL 源后，沿同一 Root 显式启动。新的 Main 从原快照配置和原 CRL 文件加载，源端口仍拒绝连接，原吊销策略继续生效。两份缓存、原回执和受保护文件精确，最终 Main 禁用、正常停机，夹具正常停放。验证期间仅调整隔离客户端对缺少 AKI 的合成证书检查方式，保留信任链、主机名与精确摘要校验。测试误建在缓存文件位置的空目录已移出保留，两次停源辅助命令在宿主参数检查处拒绝，实际只停止一次源。

本轮覆盖正式文件 keystore 与 Jetty 的 TCP、TLS 和 CRL 生命周期。其他 provider、证书到期、业务路由授权、完整 Core/OSGi、其余 typed brokers、六组件发行矩阵和全部 12 父项仍内部待办。原三现场、个人文件与 7fd 41 输入/16 回执保留。冻结父项 0/12，external deferred 仅原三项。见 [实装检查点](actual-native-core-jetty-mtls-crl-checkpoint.json)。

## 2026-11-20 继续实施 — Core Linux 自解包安装器

Core `f6beac005c` 让 Linux 自解包安装器封装同次 Maven 生成的正式 `kura-core.deb`。先在私有目录核对完整载荷 SHA-256，再走正常 `dpkg --install`。旧入口的全局 Java 终止、目录删除和原始 watchdog 路径写入已移除。现有 dpkg 围栏和配对检查照常执行。未受 dpkg 管理的历史目录，以及已解包或半配置状态，在解包前拒绝。

JDK 21／Maven 3.10.1 从 Git 构建。三处 Debian 13 ARM64 隔离 Root 用同一自解包产物验证。直接首装及标准 purge 通过，原密钥、历史及三个移除围栏保留，Main 与对应 socket 已移除。由旧正式包升级保留原数据、密钥、历史和 Main 禁用选择，标准 remove 后用同一安装器保留重装通过。三处独立 JVM 的 PID 和出生时间均精确。

缺少 `iputils-ping` 的首装实际返回 1，留在 unpacked。补齐依赖后，configure 先正确拒绝验证脚本误建的 0777 父目录。只修正该测试目录为 0755，原 Root、围栏、密钥和已装控制脚本精确，再沿原 dpkg configure 完成并增加一份正常回执。正式 Main 由 kurad 运行，NNP 生效，OSGi ConfigurationService 实际激活并加载初始快照，Main 正常停机。三个隔离 Root 均正常停放。

独立测试提交 `b0ac48556b` 补齐先前已加入 Main 的 modem cohort 断言，Linux ARM64 24 项通过，macOS 23 项通过、1 项平台跳过。本轮覆盖 Debian 自解包发行入口。既有未登记 TAR 安装的迁移、非 Debian 原生入口、实际 watchdog 硬件，以及完整 Core/OSGi、剩余 typed brokers 和六组件全矩阵继续内部待办。冻结父项 0/12，external deferred 仅原三项。原三现场、个人文件与 7fd 41 输入/16 回执保留。见 [实装检查点](actual-core-linux-deb-self-extractor-checkpoint.json)。

## 2026-11-20 继续实施 — 受保护 FileMove 跨系统管理器重启续接

在新的 Debian 13 ARM64 无外网隔离 Root 实际安装当前 Core 27a64d73f3 和 Deployment 5a6025b。按 Git 和既有正式归档 SHA 复用已构建包，从实际已装 JAR 以 JDK 21 构建单次验证 bundle。首个编译前探针同时匹配了运行库和独立恢复库，校正为正式 Main 的运行库后继续，未重新安装。

实际 Main 的 OSGi FileMove 发出一个新 MOVE。已装 actor 按正式流程发布原目标和证明后由隔离故障钩子退出 79，客户端未收到结果，原状态为 DESTINATION_APPLIED_SOURCE_PENDING。标准停机后移出探针和故障覆盖，Main 不再启动。实际 systemd soft-reboot 计数增加一次，D-Bus InvocationID 变化，内核 boot ID 与 Root 的设备、inode、machine-id 保持。同一 Root 读回原作业仍待清理，七份目标、源意图及回执字节、inode 和元数据精确。

使用正式已装 Java 入口以 kurad 清理同一原源，再由本机 Root 确认原回执。同一作业达到 ALREADY_APPLIED，原已发目标及证明未变化，一次发布请求。实际 remove、purge、同包重装后原作业仍为 ALREADY_APPLIED，原目标、原记录及三份 Core 保护输入精确，原隔离 Root 禁用停放。历史 UNKNOWN 未查询或重派。

此次范围为系统管理器软重启下同一内核、文件系统和 Root。跨设备号或内核重启、实际云通知和完整 Core/OSGi、余下 typed brokers、六组件及 12 父项仍内部待办。原三现场、个人文件和 7fd 41 输入/16 回执保持。父项 0/12，external deferred 仅原三项。见 [实装检查点](actual-deployment-filemove-manager-reboot-checkpoint.json)。

## 2026-11-20 继续实施 — Native 中心命名实例与启用状态的维护

console 3cf887b 在原 STOPPED 围栏下停用默认与命名发行单元。292412c 补齐持久和运行期 target 链接清点，避免 systemd 只列模板时漏掉未启动实例。无目录的原实例在停用前将精确单元名记入原维护记录。配置时原参与者仍须停机，原休眠实例不得新增状态。启用的管理员单元替换会拒绝变更，管理员 mask 保留。 [systemctl 单元文件文档](https://freedesktop.org/software/systemd/man/latest/systemctl.html)

原 Debian 13 ARM64 远程配置现场用正式安装器在独立 28443 首次启动 rollout-check，TLS 与原默认中心不同。观测器最初预想 named 仍 disabled，实际首次启动入口也启用；沿两条原进程观测明晰现状，测试夹具切换 named 为 runtime enabled，默认保留持久 enabled。两实例实际经同一 UUID 停机、dpkg 解包及配置，显式启动后保留原启用选择、原 TLS 与默认原任务。

另一个维护目的包含默认、rollout-check、运行期启用但未启动的 nextboot 和管理员 mask 单元。实际 list-unit-files 缺休眠实例，发行清点从 target 链接取得原名。实际 remove、purge 移除三处原启用链接，管理员 mask 的 inode、内容和元数据精确。沿原四参与者回执同一归档重装，配置后两个业务实例均 disabled，只有随后显式选择者启动。休眠实例未建立状态。

原 PostgreSQL 14 表、6 条任务、两条原会话只读结果、默认 24 项及命名 12 项私有输入保持。网关未启、没有新业务派发，中心和数据库正常停放。137 项 Debian ARM64 包装测试通过。从当前 Git 生成正式包，核对 207 项编译输入后沿用原正式包 ELF 和 Web 文件。完整 Core/OSGi、typed brokers、其他提供方／启动方式和全六组件矩阵继续内部待办。原三现场、个人文件和 7fd 41 输入/16 回执保持。冻结 12 父项 0/12，external deferred 仅原三项。见 [实装检查点](actual-native-console-named-maintenance-checkpoint.json)。

## 2026-11-20 继续实施 — Native 中心带原 PostgreSQL 任务的换版与保留态重装

在原 Debian ARM64 远程配置实验现场复用独立 PostgreSQL 与已初始化中心。原网关保持停放。中心初次显式启动时收敛一条已过期的网关在线状态，实际只多一条 offline 事件，原事件前缀、其他表与任务精确。随后把启动后已结算的 14 张表、6 个原任务和 24 项中心私有输入作为维护基线。

从当前 Git 核对发行源码，使用前轮已实装的 768b0fa-2 正式中心和安装器包。只更新安装器时中心进程不变。正式中心维护沿同一 UUID 停机、解包。原 PostgreSQL 正常停止后实际 configure 拒绝，dpkg 保留 half-configured，原 STOPPED 围栏和回执精确且没有完成记录。恢复同一 PostgreSQL 容器和数据卷，沿原 UUID configure 成功。显式启动后原 TLS 指纹及安装回执一致，两笔成功配置任务经原认证会话只读得到同一结果，原数据库标识和 14 表指纹保持。

独立维护目的在同一 Root 用真实 dpkg 完成 remove、purge 和同归档解包配置。原 PostgreSQL 全程在线，原移除围栏保持至配置结束，包负载、单元和控制文件在 purge 后实际缺席。中心仍禁用，显式启动返回 READY；14 表、6 条任务、原安装回执、24 项私有输入及两项 API 原结果再核精确。未重新初始化、登录或派发任务，原网关一直未启动。中心与原 PostgreSQL 均正常停放。135 项 Debian 包装回归通过。

更早的中断断点、跨 boot 续接、命名实例和其他 launcher/provider 继续内部待办。完整 Core/OSGi、typed brokers、六组件矩阵和 12 父项维持。原三现场、个人文件及 7fd 的 41 输入/16 回执保持。最终闭环 0/12，external deferred 仍仅原三项。见 [实装检查点](actual-native-console-postgres-maintenance-checkpoint.json)。

## 2026-11-20 继续实施 — Native 中心 postinst 完成后原编号恢复

console 768b0fa 在直接 dpkg 配置中于释放围栏前记录原维护编号、Debian 前后版本、实际已装 postinst inode 和摘要、Root 和 boot。没有活动围栏时，只有 dpkg 的同一半配置包、精确配对记录和原终态才能续接，原 stopped 回执、参与单元和已装程序来源仍核对。启动检查在 dpkg 报 installed 前拒绝。独立安装器 worker 仍需要自己的 dpkg 完成回执。

从当前 Git 生成 Debian ARM64 中心及安装器包，复核已发布包和 207 项编译输入，沿用已核实 ELF 和三个 Web 文件。首轮追踪器观察了错误的同步目录，原配置实际成功，原编号和回执保留。修正追踪器后以规格相同的下一正式 Debian 修订为独立维护目的，未重做已完成目的。

Debian 13 ARM64 原六组件隔离 Root 上，实际 postinst 第一次在原完成回执的 operations 目录 fsync 成功后、释放活动围栏前被杀停。第二次沿原编号配置，在原活动围栏移除且其父目录同步后被杀停。两次真实 dpkg 均 exit 1 并保留 half-configured。两份终态文件在第二次和随后成功配置中均未重写。半配置期间实际 launch-check 拒绝；第三次 configure exit 0，达到 installed，原护栏检查允许启动。中心未启动，原 TLS 指纹、安装回执和 13 项受保护输入精确。三个服务禁用，隔离现场正常停放。135 项 Debian 包装回归通过。

更早的持久断点、跨 boot、含 PostgreSQL 任务的中心、命名实例和其他 launcher/provider，连同完整 Core/OSGi、其余 typed brokers、六组件矩阵和全部 12 父项继续内部待办。原三现场、个人文件和 7fd 的 41 输入/16 回执保持，原 UNKNOWN 未重派。冻结父项 0/12，external deferred 仅原三项。见 [实装检查点](actual-native-console-postinst-recovery-checkpoint.json)。

## 2026-11-20 继续实施 — Native 中心运行态换版和保留态重装

console c42b025 让主机维护准入和取消共用中心就绪的受保护程序摘要。宿主直接读取被拒时，固定服务账号仍须核对运行程序与已装包的同一摘要。摘要变化时不创建围栏，原停机和 UNKNOWN 约束继续生效。Debian ARM64 包装回归 133 项通过。

原六组件 Debian 13 ARM64 现场在旧工具上只读复现维护程序摘要拒绝。新版安装器独立升级后，原中心的 PID、InvocationID 和启动时钟精确。正式入口冻结并沿同一维护 UUID 只停机一次。真实 dpkg 解包后 Main 仍 inactive，停机回执和原围栏保持。配置核对实际负载后完成同一维护。显式启动新版中心返回 READY，原 TLS 叶证书、原安装回执与 13 项受保护目录及文件精确。Core 和 Agent 保持停机。

同一 isolated Root 另用一个新维护目的完成实际 remove、purge、同版本及同归档重装。purge 回执为 exit 0，dpkg 显示 unknown ok not-installed，包控制文件与 unit 缺席；首轮观察器只接受无 dpkg 条目，沿原派发修正只读判定，未重做 purge。保留文件和原回执精确。解包期间仍有原围栏，配置完成后中心保持 disabled；显式启动后的 READY 与原 TLS 指纹一致。三个服务停机且禁用，隔离现场正常停放。

新发行包按当前 Git 生成，核对原正式中心包的 207 项 Rust、迁移和 Web 编译输入后，重用与原包摘要匹配的 ELF 及三个 Web 文件。现场中心仍为未初始化数据库的本机设置态；含真实 PostgreSQL 任务的换版、进一步配置中断、命名实例、其他 launcher/provider 与全六组件矩阵继续内部待办。原三现场、个人文件和 7fd 禁用停机及 41 输入/16 回执保持。冻结父项 0/12，external deferred 仅原三项。见 [实装检查点](actual-native-console-online-maintenance-checkpoint.json)。

## 2026-11-20 继续实施 — Native 中心安装器在受限宿主核验运行程序

console 45a973e 在管理员读取内核程序链接被拒时，以固定中心账号、空附加组、空 capabilities 和 NNP 1 核对同一数字 PID 的程序摘要。已安装包的来源证明和精确 TLS 叶证书检验仍必需，成功探测后再次核对 systemd 原 MainPID 和 InvocationID。读不出原摘要、摘要不匹配或运行代次变化均不会返回 READY。

沿已有六组件 Debian 13 ARM64 原隔离现场继续。中心因原 enablement 在启动时自动恢复，未显式再次启动。旧安装器实际返回 UNVERIFIED，管理员进程没有 CAP_SYS_PTRACE，直接读 /proc/PID/exe 确实被拒。只以真实 dpkg 升级新默认图形入口工具包一次。同一中心 PID、启动时钟和 InvocationID 上，安装器经原证书和存活端点返回 READY。直接读取仍拒绝，受控摘要命令的账号、五组能力、附加组和 NNP 已在实际系统核对。

其他七包版本和 21 项受保护输入的内容及元数据保持，原中心安装编号和 APPLIED 回执精确。Core 和 Agent 未启动，原隔离现场正常停放，中心原启用策略保持。42 项 Debian ARM64 定向包装回归通过。未触及的证书测试在 Debian 新 TLS 栈的 BrokenPipeError 由旧源码同环境复现五次；10998be 只调整测试端 TLS session tickets，保留零 HTTP 字节断言。现有整套 132 项通过。实际安装包来自 45a973e，新测试只改测试文件。

旧 HTTPS 已派发步骤仍为 UNKNOWN；沿原 Root、围栏恢复 PREPARED 回拷继续使用 67bf3c5 及 c83759e 的实装证据。原三现场、个人配置及 7fd 禁用停机、41 输入/16 回执保持。完整 Core/OSGi、其余 typed brokers、全部六组件/provider/launcher 与其余安装恢复仍内部待办。冻结 12 父项 0/12，external deferred 仅原三项。见 [实际检查点](actual-native-console-executable-proof-checkpoint.json)。

## 2026-11-20 继续实施 — Core 与 Networking 联合升级保留原只读材料授权

Agent 7d58d7b 为联合维护增加显式原材料授权编号。沿原 peer、初始化和同一 Root/boot，准备时固定公开授权、私有 relay grant、原指针和四份回执。两条原材料 socket 必须停用且连接归零。完成校验在新 Core 上重算原授权上下文，拒绝文件、权限或源目录变化，原材料授权保留。保留此前无需材料授权的流程。

从当前 Git 构建新版 Agent 正式包，从前一已核验的发行包提取 Rust 构建输入未变的 ARM64 ELF 并核对指纹。Debian 13 ARM64 实际部署旧 Core 和 Networking，分别建立原本地与材料授权。正常 dpkg 升级两个批准的正式包，沿原维护编号完成。原材料和本地授权均为 APPLIED/READY，7 份材料授权文件的字节、inode、属主与权限精确，原 SQLite、plan key、AES 等另外 12 份输入在启动前核对一致。

正式非 Root Main 和 Agent 显式启动，正常 Unix 对端身份和 OSGi 桥注册。Agent 五组 capabilities 为零，Main 为 0x1402，两者 NNP 1，远程关闭。启动后原 7 份材料文件再次核对一致。135 项 Debian Native 包装测试通过，含授权字节漂移拒绝和已有 enrollment 保留门禁。夹具正常停放。

已绑定数据、附加授权和跨 boot 仍为内部待办，完整 Core/OSGi、typed brokers、六组件和全部 12 父项保留。原三现场、个人文件及 7fd 的禁用停机与 41 输入/16 回执保持。父项 0/12，external deferred 仅原三项。见 [实装检查点](actual-native-companion-material-checkpoint.json)。

## 2026-11-20 继续实施 — 原 Native Kura peer 的同 Root 联合升级

Agent 1f74d99 增加 `prepare-kura-companion-update`。正式入口沿原初始化、原本地授权和同一 Root/boot 记录显式升级意图，固定两个实际 Debian 归档、版本、维护脚本和新 Main/drop-in。仅接受原版已审查 drop-in 到无能力声明版的过渡，Core 的能力范围只能收窄。沿用原源码维护回执链与启动准入。已绑定、材料及额外授权仍走其独立维护合同。

用 Rust 1.95.0 从当前 Git 构建 ARM64 Agent，隔离 Debian 13 上实装到原旧 Core/Networking 配对。按已审查发布的 Core 与 Networking 正式包一次性固定各自 SHA-256。真实 dpkg 只配置完 Core 时，同编号完成请求拒绝，原公开授权与 pending 保持。Networking 配置后，沿同一编号完成，结果 APPLIED，原授权为 READY。原 AES、初始化、授权、SQLite 与 plan key 等 12 份保留输入精确，只有 Core unit 与 Networking drop-in 两个授权指纹变化。

实际 Main 和非 Root Native Agent 显式启动，Unix socket 对端与正式 Agent PID/UID/GID 一致，正常 Java/OSGi 桥注册。Agent 五组 capabilities 为零、NNP 1，Main 为 0x1402、NNP 1，远程 OFF。夹具正常停放。Debian 132 项 Native 包装测试通过，含新指纹权限越界、另一编号拒绝、安装不完整及写入一部分后原编号恢复。旧拒绝夹具保持原停放记录。

已有材料、已绑定及附加授权的联合迁移、跨 boot、其余实际中断边界、完整 Core/OSGi、typed brokers 和六组件矩阵继续内部待办。原三现场、个人文件及 7fd 禁用停机和 41 输入/16 回执保持。冻结 12 父项 0/12，external deferred 仅原三项。见 [实装检查点](actual-native-companion-maintenance-checkpoint.json)。

## 2026-11-20 继续实施 — 当前 Native Agent 启动与现有 Kura peer 授权边界

从 Git 用 JDK 21、Maven 3.10.1 和 Rust 1.95.0 构建 Core 27a64d73f3、Networking 6ab7101、Native Agent bd69cc1。正式 Agent 包固定当前协议、Java bridge、Core loader 以及 ARM64 ELF 指纹。首次并行构建的 Debian 时间戳排序不符合升级顺序，派发前重新生成正式归档。旧包和首次输出保留作为证据。

在新 Debian 13 ARM64 Root 安装三包，原生初始化、本地授权各派发一次。正常 systemd 单元显式启动 Agent 和 Main，Java/OSGi bridge 实际完成注册。Unix socket 的内核对端 PID、UID、GID 与正式 Agent 进程一致。Agent 为独立非 Root、五个能力掩码均为零、NNP 1，Core Main 为非 Root、0x1402、NNP 1。远程通道 OFF。首次进程采样早于启动脚本交接 Java，经原运行进程只读补采，启动未重派。

第二个隔离 Root 从旧正式组合建立本地授权。沿原初始化，经真实 dpkg 将 Agent 一次升级到同桥配对的 bd69cc1，源码维护回执 APPLIED，原授权仍为 APPLIED/READY，12 份保留输入精确。随后 Core 与 Networking 在同一 Root、boot 正式升级。原授权记录、原状态、原 AES 和两份退役指纹保持。历史仍为 APPLIED，当前准入为 REFUSED。真实 systemd ExecStartPre 返回原 Root proof 差异，ExecMain PID 和启动计时均为 0。原两个业务编号保留，未重复派发。

Linux Rust 17 项、Python 本地授权与原源码维护 22 项通过。两轮夹具正常停放，原三现场、个人文件和 7fd 的 41 输入/16 回执保持。已有授权的 Core unit／Networking 双指纹联合迁移、材料和已绑定场景、完整 Core/OSGi、typed brokers、六组件矩阵均继续内部待办。冻结 12 父项为 0/12，external deferred 仅原三项。见 [实际检查点](actual-current-native-agent-peer-checkpoint.json)。

## 2026-11-20 继续实施 — Core 与 Networking 的 Main 配对及域名探测

旧 Networking drop-in 请求 cap_net_raw，实际 systemd 进程受 Core bounding set 限制，仍为 0x1402，但 systemd 的声明值与旧 Agent 配对条件不符。Core 5ce4a95ddf 只接受两个指纹固定的 Networking 模板，并声明新包依赖。Networking 6ab7101 改用无能力声明的依赖 drop-in，仅在升级中精确识别并替换原模板。Agent bd69cc1 核对包归属、原模板、实际 vendor 文件与 Core 已加载能力声明。两个指纹均有回归覆盖，历史规则继续生效。

Debian 13 ARM64 上从 Git 用 JDK 21 和 Maven 3.10.1 构建并实际升级 Core 和 Networking。在同一 Root 和 boot 上，原 Core 兼容的旧 drop-in 保留到 Networking 正式后置配置，随后新模板与 vendor 文件精确匹配。实际新 Main 的声明和五个进程能力掩码都没有 CAP_NET_RAW；原 AES 与原维护回执保持。Agent 守卫读取真实已安装包和 systemd 状态通过。独立挂载测试确认旧模板只能按升级路径替换，修改内容和硬链接拒绝，同字节续接不更换 inode。

域名首轮实测观察到 IP_ONLY。补齐夹具 AAAA 空响应后，单独比较反向超时，普通输出在 12 秒退出，数字输出在约 4.1 秒收到 5 个回复。Core 27a64d73f3 固定使用 iputils-ping 的 -n，仍用正向解析选择目的地址。再沿同一 Root 升级并启动，实际 OSGi SystemService 报 FULL。同一 Main 的正式 ConfigurationService 发布 IPv6 LAN，BIND 返回精确同 Root 的 APPLIED/送达回执。独立 LAN 客户端经 IPv6 访问代理，得到指定上游 IPv4 答案。防火墙只在夹具中为该客户端放通目标 UDP 53。 [Debian ping(8)](https://manpages.debian.org/trixie/iputils-ping/ping.8.en.html#n)

Networking Maven 2239 项、Core Python 10 项、Linux Agent 配对 14 项通过。夹具正常停放，原三现场与 7fd 禁用停止及 41 输入/16 回执保持。Native Agent 实际启动和已有授权迁移、其他 launcher、完整 Core/OSGi、其余 typed brokers 和六组件矩阵继续内部待办，冻结 12 父项为 0/12，external deferred 仅原三项。见 [实际检查点](actual-core-icmp-networking-pair-checkpoint.json)。

## 2026-11-20 继续实施 — Core Main ICMP 专用 worker

Core 6d768e52b3 将实际 OSGi SystemService 的联网探测交给按需启动、独立账号的 ICMP worker。只接纳来自实际 systemd Main 的直接子进程，重新核对 cgroup、PID、已加载单元与包指纹。使用固定 iputils-ping 指令和有界运行时间。正式 Main 移除 CAP_NET_RAW，普通命令 worker 的能力集仍为空。发行包显式依赖 Debian iputils-ping。

用 Git、JDK 21 和 Maven 3.10.1 构建前后两代 Core。在独立 Debian 13 ARM64 Root 关闭普通 ICMP datagram 授权。旧 Main 实测 0x3402，升级后 Main 为 0x1402 且 NNP=1，其他位保持。实际 SystemService 在两代均报告 FULL。新 worker 五个能力掩码均为 0x2000，IPv4 和 IPv6 回环均返回 0；同账号但不属于 Main 的请求被 worker 拒绝。两代软件包同 Root、同 boot 升级，原 AES 凭据、测试配置字节和已配对的维护历史保持。

正式 Main 正常停止后 socket 和实例全部退出。实际 dpkg remove 保留测试用户配置，purge 清除该保留副本，两步均保持原 AES 和两个维护回执。worker 单元、socket、包内代码已移除，独立账号闭锁。隔离 Root 正常退出。35 项 Python 回归通过，42 项 JDK 21 Core System 通过、2 项跳过。

原 HTTPS 的 UNKNOWN 核定和 PREPARED 同 Root 原围栏恢复结果沿用已有证据。Networking 组合、其他 launcher、完整 Core/OSGi、其余 typed brokers 及六组件矩阵继续内部待办，冻结 12 父项为 0/12，external deferred 仅原三项。原三现场、个人配置和 7fd 禁用停止及 41 输入/16 回执保持。见 [实际检查点](actual-core-typed-icmp-checkpoint.json)。



## 2026-11-20 继续实施 — G022 Maven 模板、可选生成与 .NET 验证

PLC4X 8bfdc46416 将 C／Go／Python 三份生成器测试 POM 的 modelVersion 移入 project 元素。c3f4747719 把 C／Python 模块的 Maven 4 专用 sources 声明换成兼容原 source root 的 Maven 3 写法。112 份仓库 POM 核对为合法 XML，三份原模板的 Maven 3 解析失败均复现。以 Git 当前构建的生成器依赖复验，三个模板的实际 Maven validate、各自的 Invoker test 和 generate-driver 均通过。Maven 坐标保持 2.0.0.B-SNAPSHOT。

当前 Git 的 JDK 21／Maven 3.10.1 可选生成依序完成 OPC UA 和 KNX 的 xml 1.4.0 变换及两套 driver 生成，34 模块完整安装通过。OPC UA 源码无变化。在线 KNX 主数据变更产生 mspec 及三份 Java 生成文件差异，隔离工作树中保留 XML 指纹和生成结果，后续需联合评审。

.NET 首轮测试中，旧本地 SLMP 测试仍等待两个独立批次，当前上游实现把显式范围合为 0x0406 读取。36e8afa8da 修正既有测试帧断言，保留原列表值和请求顺序验证。JDK 21 的 Maven 入口调用私有 .NET SDK 8.0.428，Modbus/S7/SLMP 生成物检查和全套 1,165 项通过，SLMP 定向 11 项全部通过。11 个实际 NuGet 产物元数据核对为 2.0.0-B-SNAPSHOT。[上游 SLMP 变更](https://github.com/apache/plc4x/commit/ea42d3db707839f3d2030d05c929a42a867c5d2f)

Go 1.27.4 的全模块 build 通过，默认 vet exit 1，精确重现既有 31 条诊断，包括 27 条方法签名、2 条不可达代码和 2 条移位。16 个相关文件与当次远端 develop f28cefb1d0 逐字节一致，未更新工作区 refs 或审查游标。Go vet 和 KNX 生成差异继续内部待办。原三现场、25 份个人文件及 7fd 的 41 输入/16 回执保持。其余 Core/OSGi、typed brokers、完整六组件矩阵及 12 父项继续，冻结闭环 0/12，external deferred 仅原三项。见 [实际检查点](actual-g022-plc4x-validation-checkpoint.json)。

## 2026-11-20 继续实施 — G021 实际 OSGi PID 查询与 Config Admin 状态

从当前 Git 用 JDK 21、Maven 3.10.1 构建 Core d37dfbc9de 和 Cloud 9faaec1 正式包。逐项核对原 G021 的 27 个描述、历史 DS namespace、PID 默认或显式 String 类型及当前打包字节。Core 2d5c0ab992 提交了实际 registry 与 Config Admin 测试探针，生产描述保持。[上游变更](https://github.com/eclipse-kura/kura/commit/5943c897ae0353ff1360b8b808c49fb504e98195)

在独立 Debian 13 ARM64 Root 正式安装 Core 与 Cloud，实测非 Root Main 上 25 个原生 provider 的 OSGi LDAP 过滤查询，均返回来自预期 bundle 和 component 的单个 String PID，Config Admin 原本均无匹配项。JSON、XML 和 Raw MQTT 工厂分别由真实 Config Admin 建立临时匹配项、通过包含标记的过滤器读取、移除并再次核定缺配置。三个 provider 的全部九次阶段观测均为 String，Main PID 和 invocation 保持。

两份 emulator provider 在无外网的独立 Equinox JVM 实测。首轮引导缺 Aries SPI Fly/ASM，在 resolver 阶段退出。补齐本包对应运行依赖后完成两个 provider 的缺配置查找及两次真实 Config Admin 三态验证。原生和 emulator 合计 27 个缺配置注册结果及 5 个配置存在／移除周期。两个夹具均正常停放，原生 Main 禁用，探针已移出自动加载目录。

G021 的实际 PID／配置合同已有运行证据。G022、其余业务与 typed brokers、完整六组件 provider/launcher/生命周期矩阵继续内部待办，冻结 12 父项仍 0/12，external deferred 仅原三项。原三现场、个人文件及 7fd 的 41 输入/16 回执保持。见 [实际检查点](actual-g021-service-pid-runtime-checkpoint.json)。

## 2026-11-20 继续实施 — Core dpkg 回滚硬链接中断与 Equinox reactor

在独立 Debian 13 ARM64 Root 实测 dpkg 发布完整原路径后中断，续解包在 linkat 成功后再次中断。原路径和回滚路径仅有两个名字，共用原包文件 inode。旧 a95523d46a 在只读核验中拒绝该状态，夹具保持原 dpkg 状态、Root 和围栏。

Core d37dfbc9de 只在已有原 HMAC 交接且目录 inode 匹配的载荷中，按同一包清单验证原路径和回滚路径共享的 inode。两份路径和原文件的大小、元数据及摘要必须精确相符。第三个硬链接、换成同字节但不同 inode 的回滚文件、未配对路径及不完整文件上的硬链接继续拒绝。

修正在新隔离 Root 从 Git、JDK 21 构建并实际验收。实际 dpkg 经两个中断边界后，沿同一 Root、boot、包和围栏接回原旧树。旧树核验后，同一个目录、硬链接的 inode、两份名字及内容精确交还 dpkg。实际 configure 达到 installed ok，18 份配置记录通过，原 AES、四类数据、旧包历史和相邻目录保持，仅新增原维护号回执。96 项 ARM64 Linux 定向通过。两个现场均禁用 Main 且正常停放。

Core a95523d46a 用 JDK 21、Maven 3.10.1 完成标准 Equinox reactor，先前分开验证的 67 项真实 Equinox 用例首次在同一当前源码构建中通过。后续 d37dfbc9de 只变更 Python 包装代码与其测试。其余 Core/OSGi 业务、typed brokers、六组件 provider/launcher/全生命周期及跨 boot 仍内部待办。原三现场、25 份个人文件和 7fd 的 41 输入/16 回执保持，旧 UNKNOWN 未重派。12 父项仍 0/12，external deferred 仅原三项。见 [实际检查点](actual-core-dpkg-hardlink-checkpoint.json)。

## 2026-11-20 继续实施 — Core dpkg 控制替换及新版 reinstreq 恢复

原 HTTPS 步骤仍按原编号保留 UNKNOWN。可信记录只有派发证据。已实际验证的 PREPARED 文件回拷和原围栏续接继续沿用。

Core a666c7cda9 在新版版本已由 dpkg 持久发布时，按原 Root、boot、围栏、捕获记录、两边版本及四个原维护脚本的精确指纹识别同版重装。首次实装仍在新版重装时拒绝，保留现场。实际 dpkg 在上一轮解包重试为同一候选的 434 个文件生成了 `.dpkg-tmp`。Core a95523d46a 只在同一目录 inode 已有原 HMAC 交接时接纳逐一匹配入包大小和 SHA-256 的完整备份。无原交接、未列入包清单或不一致的文件均拒绝。[Debian dpkg 手册](https://dyn.manpages.debian.org/trixie/dpkg/dpkg.1.en.html)

在独立 Debian 13 ARM64 Root 从 Git 用 JDK 21 构建并实际安装。第一次中断紧随 dpkg 替换新 postinst，数据库仍为旧版 half-installed reinstreq；沿同包补续解包，第二次中断紧随新版 unpacked reinstreq 状态发布。真实重装传给 preinst 的两个版本参数均为新版，恢复期间 dpkg 显示新版 half-installed reinstreq。旧树、同一个入包目录 inode 及 434 份备份的 inode 和内容沿原维护围栏交还 dpkg。实际安装最终为 installed ok，18 份配置记录通过 HMAC 验证，原 AES、四类数据、历史回执和相邻目录保持，只新增一份原编号维护回执。

正式 Main 装载本包 Equinox，实际非 Root 且 NNP=1。观察到 HTTPS/SSL keystore、CertificatesManager 及 Felix HTTP bridge 启动，回环 HTTPS 实际返回 404。Linux 96 项通过，macOS 90 项通过、6 项跳过。两个隔离现场均正常停放，原三现场、25 份个人文件和 7fd 的 41 输入/16 回执保持。其他 dpkg 边界、跨 boot、完整 Core/OSGi、其余 typed brokers 以及六组件 provider/launcher/生命周期矩阵继续内部待办。冻结父项 0/12，external deferred 仅原三项。见 [实际检查点](actual-core-dpkg-control-checkpoint.json)。

## 2026-11-20 继续实施 — Core 包载荷写入后沿原围栏恢复

Core ebdfbd77ca 为同一次升级持久记录 dpkg 载荷交接，只有原 Root、boot、维护围栏、版本和 preinst 配对，且 dpkg 为原版本 half-installed 时才处理已出现的载荷。包内清单对照实际 deb 的 434 个文件和 40 个目录完全一致。完整文件核对大小与 SHA-256，.dpkg-new 临时文件核对路径、类型、属主和大小上限。接回原旧树之前，半成品在同一文件系统进入专用暂存，原目录及文件 inode 保持。原前置检查通过后把同一目录交回 dpkg。已开始 configure 的围栏和文件身份歧义仍拒绝。

Debian 13 ARM64 用 JDK 21 实际构建两代 Core。第一个中断点是 dpkg 对 org.eclipse.kura.api JAR 的 .dpkg-new 文件成功写入 32,768 字节之后，读回字节与候选包前缀一致。第二个中断点是已接回原树、暂存半成品之后。第三次沿同一 Root 和围栏把同一 inode 与原字节交回 dpkg，完成解包和配置，两个续接均未重新捕获旧数据。原 AES、旧备份与维护历史、四类恢复文件及相邻安装目录保持。18 份 configure 记录通过核验，仅新增原编号维护回执。

正式 Main 使用与构建产物 SHA-256 一致的 Equinox launcher，回环 HTTPS 404。ARM64 定向 77 项通过、2 项跳过。新夹具正常停放。Core 9f8a090a35 只修正回归测试。后续 dpkg 控制信息替换等中断、跨 boot、其余 Core/OSGi、typed brokers 和六组件矩阵继续内部待办。原三现场、个人文件和 7fd 的 41 输入/16 回执保持。12 父项 0/12，external deferred 仅原三项。见 [实际检查点](actual-core-dpkg-payload-checkpoint.json)。

## 2026-11-20 继续实施 — Core 已发布副本及旧树移出后恢复

Core 4803d29dde 验证已发布副本的 HMAC、Root、boot、原围栏、源树与链接、preinst 指纹及两边版本。确认 Main 停止后，将旧物理运行树整体原子移入私有副本目录。解包前再次进入原 preinst 时，验证并接回同一树，再运行原 provider 和维护准入。已进入 configure 的围栏、外来源树、其他包、其他 boot 和无原记录的旧副本继续拒绝。包安装阶段按已配对脚本位置选择新物理目录，夹具同前缀的相邻目录保持原文件和元数据。

Debian 13 ARM64 用 JDK 21 从 Git 构建两代 Core。在同一实际 dpkg 升级中，先中断于副本发布后、旧树移出前，再中断于旧树移出完成后、dpkg 解包前。两次中断后 dpkg 都保留旧包 installed。正式副本的 inode、四份数据、原 HMAC 记录及围栏精确保持。旧树移出后，旧 prerm 因缺原 broker 代码拒绝，dpkg 切换到新包的 failed-upgrade 路径。配对 preinst 接回旧树，经正常前置检查后继续。全程只生成一次正式副本，最终解包配置成功，18 份 configure 记录通过核验。四类恢复数据、原 AES 和旧维护历史保持，仅新增原编号维护回执。

正式 Main 在 incoming 物理目录运行，回环 HTTPS 探针有实际响应。ARM64 Linux 定向 76 项通过、2 项跳过。新夹具正常停放。dpkg 已开始写入新载荷后的中断、跨 boot、其余 Core/OSGi、typed brokers 和完整六组件生命周期矩阵继续内部待办。父项 0/12，external deferred 仍仅原三项。原三现场、个人文件和 7fd 的 41 输入/16 回执保持。见 [实际检查点](actual-core-preinst-published-checkpoint.json)。

## 2026-11-20 继续实施 — Core preinst 旧树副本发布前中断恢复

Core 4f71bda2b3 将旧数据复制到私有目录。暂存记录绑定原维护围栏、Root 和旧目录身份、incoming preinst 指纹与两侧版本。原子发布 .kura-preserve 前须全部复制成功并同步。再次 unpack 只清理与同一 Root、源目录、包和围栏匹配的未发布暂存。已存在的正式恢复副本及历史保留副本仍拒绝覆盖。

在新隔离 Debian 13 ARM64 Root 从 Git 用 JDK 21 构建并实装两代 Core。data 源文件共 3,400,000 字节，真实 dpkg 的 preinst 只写出 1 MiB 前缀时中断。正式恢复目录未出现，dpkg 保留旧包 installed。原 user/data/packages/log 四份文件的内容、inode 和所测元数据，原 AES 凭据、旧维护历史及原围栏保持。沿同一 Root、boot、原包和原围栏继续 unpack。仅弃用有原 HMAC 记录的未发布暂存，从完整旧树生成全量副本，再原子发布。随后标准 configure 成功，四份文件的内容、属主、权限及时间戳复核一致，18 份 configure 记录通过 HMAC 核验，旧历史保持，仅新增同编号维护回执。ARM64 Linux 定向 65 项通过，夹具 Main 全程禁用且未启动，最终正常停放。

正式副本已发布后的中断、跨 boot、其余 Core/OSGi、typed brokers 和完整六组件生命周期矩阵继续内部待办。12 父项 0/12，external deferred 仍仅原三项。原三现场、25 份个人文件和 7fd 的 41 输入/16 回执保持。见 [实际检查点](actual-core-preinst-capture-checkpoint.json)。

## 2026-11-20 继续实施 — PREPARED 中途写入 sibling 与安装顺序恢复

Core 63b2091e35 将缺失 sibling 的回拷改为按原维护围栏暂存，写完并同步后在同一文件系统原子发布目录，保留已经由 dpkg 解包的目录。安装顺序记录回拷失败会保留 PREPARED 和原恢复副本。首次实装在 GWT JAR 实际写出 1 MiB 后中断。续接时 generation-reader 扫到私有区中的半截 JAR 并拒绝。该现场保持半配置，原副本和编号保留。Core 33a3cfef85 把暂存移出 reader 扫描范围，配对规则保持。

Debian 13 ARM64 用 JDK 21 从 Git 构建两代 Core 与 GWT。修正包在新隔离 Root 从实装旧包升级。第一次在实际 GWT JAR 只写出 1 MiB 时中断，正式 sibling 目录仍缺。第二次沿相同 Root、boot 和维护围栏恢复完整目录，安装顺序文件仅写出 4 字节时 dpkg 返回失败，源数据和原阶段回执保持。第三次恢复完整安装顺序文件，已发布 sibling 的 inode 保持，五个安装步骤均未重新执行。17 份原阶段证据精确保持，新增 READY 后的 18 份记录全部通过 HMAC 核验，旧维护历史与原 AES 凭据保持。原副本在 READY 后退休，新增一份同编号维护回执。

合法 Main 启动后经 HTTPS 读取已恢复的 GWT 登录入口。ARM64 Linux 定向 54 项通过，macOS 53 项通过、1 项 Linux 原子重命名用例跳过。两个隔离现场均正常停放。跨 boot、其他回拷路径、完整 Core/OSGi、其余 typed brokers 及六组件全生命周期矩阵仍为内部待办。原三现场和 7fd 的 41 输入/16 回执保持，旧 UNKNOWN 未重发。父项 0/12，external deferred 仅原三项。见 [实际检查点](actual-core-prepared-torn-copy-checkpoint.json)。

## 2026-11-20 继续实施 — 正式 Main 改密成功后立即停机

Debian 13 ARM64 从当前 Git 和 JDK 21 构建旧 Core 2a74ac1c38、修复版 99024da7fb 和 GWT 08712fb。旧包的 GWT 普通改密实际返回成功，Main 在 1.565 秒内正常停止。新密码在新 Main 被拒绝，原密码仍可登录。Felix 角色事件原先只排入默认五秒延迟的快照任务，HTTP 可以提前返回。

修复版由 OSGi 角色存储在返回成功前提交已有配置快照。延后事件跳过同一已提交配置，快照结果未决时停止后续写入，等待重新加载角色库。沿同一 Root 和 boot 实际 dpkg 升级，GWT、旧快照与 AES 密钥内容和所测元数据保持。新的一次 GWT 普通改密返回成功至 Main 正常停止为 1.584 秒。重启后新密码可登录、原密码被拒绝，只有一份新快照。

另一独立新装 Root 实际完成首次强制改密。原快照保留，新快照清除强制标记。正常停止并重启后新密码可登录，初始密码被拒绝。浏览器探针漏采首次改密模块的 RPC 回执，未测量该分支收到回执后立即停机的时间，原改密未再派发。三个改动相关模块共 257 项测试通过。两个隔离 Main 均禁用并正常停放。

此前原安装步骤核定与 PREPARED 回拷的同 Root、原围栏证据继续沿用。完整 Core/OSGi、其余 typed brokers、完整六组件/provider/launcher/安装升级移除 purge/中断恢复矩阵和全部 12 父项仍为原范围。闭环 0/12，external deferred 仅原三项。原 7fd 禁用且停止，保留 41 输入/16 回执。见 [实际检查点](actual-native-identity-persistence-checkpoint.json)。

## 2026-11-20 继续实施 — 正式包 Operation 12 丢回执与原编号核定

沿上轮 Debian ARM64 隔离 Root 重新启动已装的 Core、Networking、GWT、Agent、console 和原 PostgreSQL。保持原注册、凭据及上轮成功任务。对 stock 文件日志 provider 的新配置只派发一次。故障注入在 Agent 持久 claim 后持有 SQLite 写锁，注入器没有执行数据写入。正式 Java Main 按原编号发布回执时，Agent 的执行记录仍是 RUNNING，随后对该 Agent 进程发 SIGKILL。中心真实进入 UNKNOWN，Main 的 PID、启动时钟和调用代次保持。

故障夹具把新 Java 回执暂存在同一目录。新 Agent 启动前撤回配置写权限。原证书和代次的 mTLS 重连后，中心对原编号取得两次核定租约。回执缺位期间，中心与 Agent 均保留 UNKNOWN，SQLite 没有核定记录。将原文件移回原名后，原 inode、内容及 SHA-256 对齐。只读核定令同一任务进入 SUCCEEDED，并绑定原 PID、revision、参数哈希和 Java 回执哈希。SQLite 原加密计划保持，核定仅一条，PostgreSQL 输入和结果仍为密文封装。原编号只有一次 RUNNING，审计依次记接受、UNKNOWN、成功、配置回执核定。

新任务的异步 provider 健康及其他 provider/launcher 的跨中断覆盖仍需单独验证。隔离服务已禁用并正常停放。18 仓、原三现场、个人文件和 7fd 的 41 输入/16 回执复核保持。旧 UNKNOWN 未重发、已验场景未重放。完整 Core/OSGi、其余 typed brokers、完整六组件矩阵和 12 父项继续实施。闭环 0/12，external deferred 仅原三项。见 [实际检查点](actual-native-configuration-unknown-outcome-checkpoint.json)。

## 2026-11-20 继续实施 — 正式包的远程配置授权、真实提交及原回执

Agent 03b2b80 使用与原初始化绑定的 Root 配置授权。非 Root 进程核验原数据库、原密钥、初始化和 Kura peer，仍拒绝环境注入的写入开关。Networking 82dc2cd 从受保护文件读取 Java PID 名单，通过正式 SystemService 定位回执目录。缺少可定位的存储时，带编号的请求在调用 Core 前被拒绝。Java 同步落盘并按原编号创建回执。

从当前 Git 构建 Core、Networking、GWT、Agent、console 和安装器。Debian 13 ARM64 真实 dpkg 在首轮发现缺少平台依赖，补齐后只配置原 unpacked 包。首个 Agent 候选仅通过 Root 预检，非 Root 运行门禁拒绝写入开关，当时数据库零任务。保留失败授权原文件，沿同一 Root、boot、原注册和加密状态按正式 source-update 流程装入修正候选。前两次准备在目录与包文件权限预检中拒绝，原 UUID 未发布，经合规暂存后首次获准发布。原注册、初始化、SQLite 与凭据指纹保持。

新 Root 授权启用后，实际 mTLS 中心在 Java PID 名单缺失时返回 422，数据库零任务。加名单并启用正式 Main 后，Agent 进程、认证连接、证书及绑定代次保持，中心开始报告可写。名单外任务在 Java 拒绝为 FORBIDDEN，未产生 Java 回执。名单内对 filesystem-kura-log 的 Operation 12 只派发一次，Java 原编号回执、Agent 加密执行及 PostgreSQL 密封任务一致。Main 再次启动后，只读查回原回执与新的 provider 配置值。该回执证明 Core 接受提交，异步业务健康仍需单独核验。

新现场用实际 GWT 完成注册。换版后 GWT 登录复验拒绝原密码，远程开关的两次操作经正式本地 Root UDS 执行，原权限检查保持。三台隔离容器已正常停放，服务禁用，现场数据保留。Java bridge 165 项通过，Agent ARM64 Rust 身份合同 17 项通过，Agent Debian 包合同 315 项中 2 项跳过、0 失败。完整正向交付仍需覆盖其他 provider、字段、launcher 和中断，原 UNKNOWN 持久核定继续内部待办。

原三现场和 7fd 的 41 输入/16 回执保持。完整 Core/OSGi、其余 typed brokers、完整六组件及全部 12 父项维持原范围。闭环 0/12，external deferred 仅原三项。见 [实际检查点](actual-native-remote-configuration-write-checkpoint.json)。

## 2026-11-20 继续实施 — 认证心跳中的远程配置写入能力

Agent 25e295a 仅在本地 opt-in 且当前注册的 Java bridge 宣告 update 时报告写就绪。console 043e83d 在认证 Hello 协商后，通过有序心跳更新原连接的配置写能力。读取能力、证书与绑定代次、Java PID allowlist、实时派发门禁及 UNKNOWN 合同保持。没有协商标记的旧 agent 保留原 Hello 静态行为。

真实 mTLS 与独立 PostgreSQL 验证首心跳前隐藏、bridge 不可用时隐藏、同连接启用及撤回、重复序号拒绝、其他 capability 保留及旧 agent 合同。撤回后正式 HTTPS API 返回 422，数据库零任务，实际测试派发零次配置写入。Agent 740053d 另以真实 Unix socket 及原 peer 身份检查测试只读与写方法注册、脱离及重新注册。PostgreSQL 已正常停放。

Agent 首次 ARM64 Linux 全量遇到 1 个导入防火墙历史用例失败，原 Agent/console Git 配对独立复现。用例的临时 Root 在通用写入口先被正式围栏拒绝。4d7348d 在同一测试分别核对原围栏拒绝与孤立计划的 OUTCOME_UNKNOWN，保持零 backend 调用及精确原文件。修正仅涉及测试。协议 24 项、ARM64 Linux Agent 798 项和 console 115 项通过。macOS Agent 编译触及未改动的 Linux peer_pid 源码边界。macOS console 首次默认套件 1 个维护用例失败，前后版本单项复验和 Linux 套件该项通过。首个离线 Linux 构建在测试前因 Rustup 组件探测失败，显式采用镜像内已装 Rust 1.95.0 后运行。

从 Git 的 console 代码提交新打包协议 crate，核对归档 Git 提交及 SHA-256，再生成当前 Agent 自包含源码并逐文件验证 manifest。ARM64 Linux 容器只挂载只读发布源码和依赖缓存，离线通过实际 Agent 二进制目标的 cargo check。正式 deb 和实际正向配置写入仍待验。

正式包 Java 授权、真实业务写入及已核定结果交付仍待验。原三现场、7fd 的 41 输入/16 回执和旧 UNKNOWN 保持。完整 provider/launcher、其余 typed brokers、完整六组件矩阵及 12 父项继续内部待办。闭环 0/12，external deferred 仅原三项。见 [实际检查点](actual-configuration-write-readiness-checkpoint.json)。

## 2026-11-20 继续实施 — Debian 正式 Main 配置事务、真实 provider 及重启回执

Debian 13 ARM64 从 Git 和 JDK 21 构建实包。旧 981d71c393 的合法 Main 在 Core 配置准入拒绝第一次尝试。原编号只出现 request、rejected，原因 UNSUPPORTED；无 dispatch、intent、complete，独立密钥库未创建。文件日志 provider 已实现 ConfigurableComponent，但 SCR 未发布该接口。Core 2a74ac1c38 补齐正式注册，原准入和 Root 配对规则保持。采集区分正式插件目录和冷恢复目录的同名配置 JAR。

标准 dpkg 沿同一 Root 升级。两份旧拒绝记录的内容和可保留属性、启动时加密后形成的原快照、原 AES 凭据及 Root 身份保持。原拒绝编号保留，未重发。已装正式 Main 一次提交新计划，更新文件日志 provider 并创建一个独立空 PKCS12。实际日志监听收到新文件事件，实际 keystore provider 打开文件，配置与加密快照均读回合成 Password。原编号的四份记录及加密快照均未包含该合成明文。

显式停止再启动原 systemd Main 后，PID 出生时间与 invocation 改变。只读查回同一 CORE_COMMITTED 和原 REJECTED。新 Main 仍收到相同 provider 的新日志事件并打开原密钥库；回执四份、加密快照及密钥库文件指纹精确。Main 维持 NNP1，两个 Main 都未持有 SYS_TIME 或 SYS_BOOT。现场禁用、正常停放，专用探针退出自动加载。

远程授权和交付、其他 provider/launcher、其余 typed brokers、完整六组件发行与生命周期和全部 12 父项维持原范围。原 7fd 禁用、正常停止且保留 41 输入/16 回执。最终 0/12，external deferred 仅原三项。见 [实际检查点](actual-native-core-configuration-plan-checkpoint.json)。

## 2026-11-20 继续实施 — 实际 Equinox 标识化配置事务与回执

Core 3d84351215 加入真实 Equinox 联合回归。由正式 ConfigurationRevisionService 一次提交单例更新和一个 factory 实例创建，组件和加密快照均读回原 Password 类型及合成值。停用实际配置 bundle 后回执 DS 撤下，重启原 bundle 后只读查询原编号得到相同 CORE_COMMITTED、创建及更新范围。未重新提交，回执的 businessHealthVerified 保持 false。

JDK 21 和 Maven 3.10.1 在独立 Git 检出构建。新用例单次通过。另有 64 项真实 Equinox 用例通过，两个系统服务用例因夹具缺少 RebootBrokerClient 实际要求的 Gson 解析失败。Core 981d71c393 补齐同发行包版本的 2.11.0，并核对两个 package wire，两个已失败用例复验通过。三次合计 67 个不同 Equinox 用例均有成功证据。旧 pin 2603adefae 的标准 Maven 依赖范围结果为 BUILD FAILURE，已报告 1206 项、0 失败、2 错误、2 跳过。两个提交仅修改测试及夹具依赖，生产包和原现场保持。统计范围和回执见 [检查点](actual-core-osgi-configuration-plan-receipt-checkpoint.json)。

Debian 实际 Main 的该配置业务、其他 provider/launcher、远程授权和异步业务结果仍待验。其余 typed brokers、完整六组件安装升级移除 purge/中断和 12 父项维持原范围。原 7fd 禁用、正常停止且保留 41 输入/16 回执。父项最终 0/12，external deferred 仅原三项。

## 2026-11-20 继续实施 — Core 已派发步骤核定与 PREPARED 部分回拷恢复

在历史原容器保持 stopped 的情况下只读核对八份原阶段文件、围栏以及 dpkg 半配置状态。此前实际 HTTPS keytool 已派发，但无可信成功退出证据或完成回执，保持 UNKNOWN。原现场未启动，原号及所有原证据保留。

Core 2603adefae 在原安装子进程成功退出并同步之后，原子落盘同 Root/围栏/包版本/脚本/恢复副本绑定的 HMAC 成功证据。后续 configure 仅凭精确证据为原步骤补齐完成回执。结果未决的步骤继续拒绝再次执行。原意图、成功证据、完成回执和 PREPARED/READY 标记均原子发布；READY 发布前同步回拷数据。73 项相关包合同通过。

Debian 13 ARM64 用 JDK 21 从 Git 顺序构建旧版与候选。真实 dpkg 第一次在 layout 成功证据写入后、完成回执写入前中断。原围栏第二次 configure 补齐 layout 回执，同次实测只派发 brokers、runtime、HTTPS 和 ownership，layout 未重新执行。完成 PREPARED 后 user 树已回拷，在 data 树实际 cp 进程执行正文前中断。此时另外三棵树的观察文件未出现，原恢复副本及原 AES 凭据保持，实际状态为 half-configured。

最终沿同一 Root 和原围栏 configure，四棵树的观察文件内容、属主、组、模式和时间戳与升级前一致。原暂存副本在 READY 后按原配对退休，18 份 HMAC 阶段记录通过验证，旧包历史保持，新增唯一一份原维护回执。Main 一直禁用且未启动，新现场正常停放。

无成功证据的旧步骤、单文件内的部分写入、跨 boot 与其余发行中断点继续内部待办。完整 Core/OSGi、其余 typed brokers、六组件/provider/launcher/安装升级移除 purge/中断及全部 12 父项维持原范围。闭环 0/12，external deferred 仅原三项，原 7fd 保持禁用、正常停止和 41 输入/16 回执。见 [检查点](actual-core-postinst-issued-and-copy-recovery-checkpoint.json)。

## 2026-11-20 继续实施 — Core postinst 持久阶段中断与原围栏恢复

Core 8e60c9f901 先在围栏目录的同级私有暂存位置落盘完整 HMAC 意图，再原子发布原围栏目录并同步父目录。中断发生在发布之前时，正式目录仍缺失，可沿原围栏开始；当正式目录已存在但原意图缺失，继续拒绝并保留现场。两个新增用例和其余相关合同、保留布局和维护测试合计 72 项通过。macOS 全量发现与上版产生相同平台及夹具诊断；显式使用实际构建 JAR 的 generation 测试仍有 22 项与上版相同的夹具设置错误。

Debian 13 ARM64 从 Git/JDK 21 构建旧版与候选。同秒构建候选在 Debian 版本预检被拦下，派发为零；另行构建发行包后从已经安装的同一现场换包。真实 dpkg 沿同一 Root 三次在持久边界后的下一个处理进程进入正文前中断，分别位于原意图发布后、layout 和 brokers 完成后、ready 写入且四类数据已恢复后。每次实际 postinst 都由 SIGKILL 终止，dpkg 实测为 half-configured。中间续接仅执行尚无回执的阶段。

原围栏最终配置进入 installed。保留原 AES 凭据及旧包历史，形成当前围栏下唯一新维护回执。13 份 HMAC 配置记录和四类数据的原指纹在最终配置后相同，原暂存副本按原 inode 退休，Main 持续禁用且未启动。沿最后持久阶段的配置续接没有重复执行已完成安装步骤或重拷四份实际观测文件。

步骤已派发且缺完成回执的核定、PREPARED 数据回拷中断、其他写入和跨 boot 边界继续内部待办。完整 Core/OSGi、其余 typed brokers、完整六组件/provider/launcher/安装升级移除 purge/中断矩阵及 12 父项均保留，仍 0/12，external deferred 仅原三项。原 7fd 保持禁用、正常停止及 41 输入/16 回执。见 [检查点](actual-core-postinst-durable-recovery-checkpoint.json)。

## 2026-11-20 继续实施 — 普通命令目录授权、活动换版与 postinst 中断恢复

Core 88376a62d9 在正式 Main 启动命令客户端前解析规范工作路径，客户端核对实际 cwd。worker 在自己的挂载命名空间打开可遍历目录并核对原 inode；对私有目录，只在已配对 Core 的五类可写数据目录继承原描述符。实际 OSGi 验证普通路径和 Kura 私有数据的相对访问。站点私有目录及从 Kura 数据树指向它的链接均拒绝，两个请求零 stdout，worker 留下明确的目录拒绝事件。

Debian 13 ARM64 从 Git/JDK 21 构建两代 Core。旧 19d2926fd0 Main 及忽略 SIGTERM 的普通命令和子进程仍运行时，实际 dpkg --unpack 换版。Core 预安装脚本暂存数据后，从实际暂存位置读回停机回调，确认原 cgroup 两进程均在回调前退出，AES 凭据保留。初始候选与旧包同分钟，Debian 版本预检查拒绝，派发为零。Core 快照版本改用秒级时间戳并重建，沿原现场执行一次换版。新版本进入 unpacked，Main 和 socket 已停止。 [Debian dpkg 状态](https://manpages.debian.org/trixie/dpkg/dpkg.1.en.html)

同一 Root 执行实际 configure，隔离追踪器只在确认正式 postinst 指纹和参数后的 PTRACE_EVENT_EXEC 处中断该脚本。dpkg exit 1，实际 half-configured，脚本正文尚未执行。原 AES 凭据、维护记录与已安装 worker 单元保持。沿原软件包状态配置一次进入 installed，Main 保持停止；之后显式启动，新的合法 Main 实际完成四项工作目录验证，再正常停机、禁用并停放。 [Linux exec 追踪事件](https://man7.org/linux/man-pages/man2/ptrace.2.html)

postinst 已执行写入后的其他中断点、其他 launcher/命令账号及 provider、完整 Core/OSGi、其余 typed brokers 与六组件生命周期矩阵继续为内部待办。冻结 12 父项 0/12，external deferred 仅原三项；原 7fd 禁用且停止，保留 41 输入/16 回执。见 [检查点](actual-core-command-active-upgrade-checkpoint.json)。

## 2026-11-20 继续实施 — 普通命令 worker 的停机顺序与实际包生命周期

Core 19d2926fd0 在正式命令 worker 单元增加 After=kura.service。systemd 在停机时逆序清理，Main 的 ExecStop 和 OSGi 退出开始前，先停止已接受的命令 cgroup。工作进程仍为普通账号、零 capabilities、NNP 1，沿用原包和 Root 配对。 [systemd 顺序](https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html) [cgroup 停止](https://www.freedesktop.org/software/systemd/man/latest/systemd.kill.html)

从 Git 和 JDK 21 构建 Debian 13 ARM64 Core-only 实包。实际 dpkg 解包一次，停在 unpacked 后正常停止容器。沿同一容器恢复，软件包、worker/socket 单元、AES 凭据和预配置维护记录保持，再实际 configure 一次。初次采集误以为凭据在 configure 才创建，实包确认 preinst 已建立，沿原结果只读继续。configure 完成时 Main 仍停止，命令 socket 尚未创建。

正式 Main 通过 OSGi 启动一个忽略 SIGTERM 的普通命令及子进程。活动状态下执行 dpkg remove，原 cgroup 在 OSGi 的 stop 回调前已清空，回调保留两进程均已退出的记录。采集未取得针对该实例的具体信号记录，沿原 remove 回执只读核对后继续。无关的同 UID 服务保持原 PID、启动计数及 invocation。移除正常保留数据副本和 AES 凭据，撤下 socket、unit 和 worker。实际 purge 删除保留的运行时副本，原 AES 凭据及独立服务保持，现场正常停放。两阶段间中断试验覆盖 unpacked 状态，单步中的中断仍为内部待办。

完整 Core/OSGi、活动命令换版与其他中断点、其他 provider/launcher、其余 typed brokers 及六组件矩阵仍为内部待办。冻结 12 父项 0/12，external deferred 仅原三项，原 7fd 禁用且停止，保留 41 输入/16 回执。见 [检查点](actual-core-command-worker-lifecycle-checkpoint.json)。

## 2026-11-20 继续实施 — 正式 Main 启用 NoNewPrivileges，普通命令仍由独立账号执行

Core ed6db4f0eb 让已配对的正式 Main 通过 socket worker 运行普通命令。Main 保留 kurad，worker 使用 kura、零 capabilities 与 NoNewPrivileges=1。kernel pidfd、有效 Main、原 cgroup、直属客户端及 package cohort 继续配对。worker 从经核定进程读取固定启动参数，使用 SCM_RIGHTS 接收标准流，仅接收方向正确的管道或 /dev/null，以及已打开的工作目录。可遍历路径在 worker 的挂载命名空间重新打开并核对 inode；私有路径继续使用原目录描述符处理相对路径。Main 默认启用 NoNewPrivileges，并从 bounding 集合移除 CAP_SETUID、CAP_SETGID。 [systemd socket](https://www.freedesktop.org/software/systemd/man/latest/systemd.socket.html) [Unix 描述符传输](https://man7.org/linux/man-pages/man7/unix.7.html)

Debian 13 ARM64 在同一隔离 Root 从 833fc21c35 正常停机后实际升级，并实装当前 Networking。原 AES 凭据指纹和元数据保持。PID 1 的 NNP 为 0，实际 Main 为 1，五类 capability 掩码仅减少原 bounding 的两个身份位。正式 OSGi 验证独立账号下的工作目录、环境变量与字面量 argv，六个二进制字节、独立 stderr、状态码 23、显式 shell、异步命令和 SIGTERM、超时回收。外部同 UID 客户端被拒且零 stdout。Main 正常停机，socket 撤下，夹具已禁用并停放。Linux 描述符合同 7 项、Networking Maven 2236 项通过。初始夹具因 worker 无法读取受保护的客户端副本而在派发前拒绝，已保留。最终包从同一已配对的 /usr/libexec 文件启动，不改变原目录权限。

其余 Main capability、其余 typed brokers、不同 launcher、站点命令账号和 provider、私有目录下依赖绝对 getcwd 的调用、历史及完整 Core/OSGi、六组件生命周期矩阵继续内部待办。冻结 12 父项 0/12、external deferred 仅原三项。原 7fd 禁用且停止，保留 41 输入/16 回执。见 [检查点](actual-core-main-command-executor-checkpoint.json)。

## 2026-11-20 继续实施 — Main 撤下 CAP_KILL

Core 833fc21c35 从正式 systemd Main 的 ambient、bounding 集合撤下 CAP_KILL。Kernel 保留同 UID 进程的信号能力，实际系统服务继续由各专用 broker 与 systemd 管理。既有 Main 停机使用已配对的同 UID pidfd 并等待 OSGi 退出。 [Linux 信号权限](https://man7.org/linux/man-pages/man2/kill.2.html) [systemd 停机与 cgroup](https://www.freedesktop.org/software/systemd/man/latest/systemd.kill.html)

以 JDK 21 从 Git 构建两代 Core 和当前 Networking，Debian 13 ARM64 同一隔离 Root 先实装旧 Core 3ea096ae52，核对实际 CAP_KILL 并正常停止，随后实际 dpkg 升级及安装配对 Networking。升级前后的已有 AES 凭据字节、inode、UID/GID、权限、链接数、长度及 mtime 保持。PID 1 仍持 CAP_KILL，实际 Equinox Main 的五类 capability 掩码均仅少该位，ambient 共 5 项、bounding 共 7 项。上述 5/7 是单元声明。容器 PID 1 的 bounding 不含 CAP_DAC_READ_SEARCH，JVM 实测为 4/6。四个 actor 的包内 Main unit 指纹与实际安装字节相符。

同一实际 Main 的两个已装 OSGi executor 分别启动并以 SIGTERM 回收 kurad、kura 子进程，退出码均为 143。对独立 Root 测试进程的 signal 0 返回 EPERM，原进程及 invocation 保持。最终 OSGi 停机回调和正式 ExecStop 均完成，Main 禁用，夹具正常停放。Networking Maven 2236 项通过。

首个夹具继承容器的 NoNewPrivileges=1，同 UID 终止与跨 UID 拒绝通过，跨账号普通命令仍在 su 设置附加组时被拒绝。另一隔离 Root 使用正式发行的默认 NoNewPrivileges=0，完成上述两个 executor 的验证。两处均正常停放。 [Linux NoNewPrivileges](https://docs.kernel.org/userspace-api/no_new_privs.html)

其余 Main 权限及 NNP、其他 provider/launcher 与运行模式、完整 Core/OSGi、历史边界、其余 typed brokers 和完整六组件生命周期仍内部待办。父项 0/12，external deferred 仅原三项，原 7fd 禁用且停止，保留 41 输入/16 回执。见 [检查点](actual-core-main-cap-kill-checkpoint.json)。

## 2026-11-20 继续实施 — IPv6 LAN 经 systemd-resolved 分流及中断恢复

从当前 Git 用 JDK 21 构建 Core 3ea096ae52 与 Networking 846d250，实装隔离 Debian 13 ARM64。使用 Debian 正式 systemd-resolved 257.13 包。基础镜像的 policy-rc.d 拒绝安装期自动启服，核对初始 inactive 后依原厂 systemd 单元启动一次。NetworkManager 1.52.1 将两个实际活动接口的域和服务器交给 resolved。首次配置漏列两条 WAN，现行全量清单行为停用设备并删除夹具 profile，原现场和两份回执已留存。第二个隔离 Root 显式声明两条 WAN 的 IPv4 和 IPv6 均为 Unmanaged，交由 NetworkManager 管理。合法 Main 内由正式 OSGi ConfigurationService 提交静态 IPv6 LAN，正式 DNS provider 通过同 Root typed BIND broker 只监听获准 LAN，唯一转发器为 127.0.0.53。 [NetworkManager DNS 配置](https://networkmanager.dev/docs/api/1.52.0/NetworkManager.conf.html) [systemd-resolved 路由规则](https://www.freedesktop.org/software/systemd/man/latest/systemd-resolved.service.html) [Debian ARM64 包](https://packages.debian.org/trixie/arm64/systemd-resolved/download)

独立 IPv6 LAN 客户端经 TCP 和 UDP 共完成六次真实查询。企业域走企业链路，默认域走第二条链路。企业解析器切换后，默认解析保持。单独停止 resolved 时，企业查询未到达任一上游。跨一个普通监视周期，BIND 配置、进程和 Root 台账保持。按原厂单元恢复 resolved 后，NetworkManager 自动重新发布分流配置，两个域恢复。合法 Main 与 NetworkManager 的实际 invocation 保持；原 Root 共两组意图、结果和送达记录，均已复核。首次端到端查询已收到正确答案，夹具最初误设每次只能对应一个上游数据报。两份上游报文到达同一预期链路，读取已有证明后接续其余查询。夹具禁用 Main 并正常停放。Networking Maven 2236 项通过。

自动公布 IPv6 本地 RDNSS 与安全撤回、实际 PPP、完整网络/接管、其余 typed brokers、完整 Core/OSGi 和六组件发行矩阵仍为内部待办。12 父项 0/12，external deferred 仍仅原三项，原 7fd 禁用且停止，保留 41 输入/16 回执。见 [检查点](actual-core-nm-systemd-resolved-checkpoint.json)。

## 2026-11-20 继续实施 — LAN DNS 代理排除自身作为转发器

Networking 846d250 在已配对的双栈路径排除网关上由本次 BIND LAN 策略监听的地址。网段内其他主机仍可作上游，已配对的系统 loopback stub 继续遵循原分流策略。所有代理配置仍由既有同 Root 的 BIND broker 接受和确认。

隔离 Debian 13 ARM64 实装 Core 3ea096ae52 和 Networking。合法 Main 中实际 OSGi 监视器观察 NetworkManager 加入本地及 WAN 两个解析器，BIND 配置、进程和 Root 台账取样保持。独立 SLAAC 客户端查询取得 WAN 上游答案。NetworkManager 只保留本地解析器后，委派前缀与活跃连接仍在，BIND 停用并交付同 Root 回执。共 3 组意图、结果和送达记录。夹具最初误判 WAN 只能有 DHCPv6 单地址，实际仍有 SLAAC 地址。原回执已在同 Root 只读复核，随后正常停放。Networking Maven 2236 项通过。

另一隔离网络抓到 NetworkManager 1.52.1 的实际 RA。临时指定共享接口的 DNS 后，RDNSS 从上游地址换为 LAN 本地地址，有效期 900 秒。撤销后下一份 RA 恢复上游，未携带本地地址的零时限撤回。自动公布与安全撤回、完整分流/防火墙/接管及其余 typed brokers、完整 Core/OSGi 和六组件仍为内部待办。父项 0/12，external deferred 仅原三项。原 7fd 停止且保留 41 输入/16 回执。 [NetworkManager 1.52.1 RA 发送源码](https://raw.githubusercontent.com/NetworkManager/NetworkManager/1.52.1/src/core/ndisc/nm-lndp-ndisc.c) [检查点](actual-core-nm-dns-loop-checkpoint.json)。

## 2026-11-20 继续实施 — NetworkManager 委派前缀驱动 LAN DNS

Networking f9a0c3c 支持明确选择 `netIPv6MethodShared` 的 IPv6 LAN。开启该 LAN 的 DNS 代理后，只使用实际应用的 NetworkManager shared 连接和内核中仍 preferred 的地址交集，排除临时、tentative、DAD 失败或已 deprecated 的地址。委派前缀规范化后比较，同前缀忽略续租期限变化，失败观测保留已提交状态。正式 Main 仍经只接受既定 Root 配对的 typed BIND broker 写入。 [NetworkManager 1.52.1 委派处理](https://raw.githubusercontent.com/NetworkManager/NetworkManager/1.52.1/src/core/devices/nm-device.c)

隔离 Debian 13 ARM64 实装 Core 3ea096ae52 和 Networking。正式 Main 一次提交共享 LAN 与 WAN 配置。独立上游以真实 DHCPv6 和 RA 报文提供前缀，实际 NetworkManager 将 LAN 由 fd60:10::/64 改为 fd60:20::/64。独立 LAN 客户端从实际 RA 获取地址，并在两个前缀下分别获得 BIND 转发的 AAAA 响应。捕获了同前缀续租，配置、进程和账本取样一致。随后撤回委派并确认 BIND 停用，原 WAN DNS 保留。共 4 组原 Root 配对的意图、结果和送达记录。Networking Maven 2235 项通过。

夹具默认只读 sysctl 导致首次 shared 激活失败。只在该隔离夹具及 NetworkManager 的私有挂载命名空间放通 IPv6 子树，随后由实际 NetworkManager 启用已有配置。Main 和 Root 历史沿用。客户端仅放通测试地址的 UDP 53，输入默认策略保持。自动公布本地代理地址、完整转发/防火墙/接管矩阵、其余 typed brokers 与六组件仍为内部待办。外部延后仍仅原三项，原 7fd 停止且 41 输入/16 回执。父项仍 0/12。见 [检查点](actual-core-nm-delegated-dns-checkpoint.json)。

## 2026-11-20 继续实施 — 保留 IPv6 链路本地上游的 zone

Networking 9509d5a 在有效解析器观测中保留 IPv6 链路本地地址的接口名与内核索引。Core 3ea096ae52 在已配对的 BIND broker 中增加 typed `enable-lan-scoped`。Root 在准备、准入和应用前后核对接口索引，发布精确索引的 BIND 配置；同名链路重新创建时，配置模型根据索引改变。旧类型和 Root 账本规则保持。 [RFC 4007](https://datatracker.ietf.org/doc/html/rfc4007)

Debian 13 ARM64 实装 Core 和 Networking，由正常 Main 内的 ConfigurationService 与生产监视器处理配置。两个独立 WAN 命名空间使用相同的 `fe80::53`，LAN 第三个命名空间的实包只到达 NetworkManager 选中的上游，源地址核对为相应链路本地地址。替换第二条链路后，解析器文件内容保持，新内核索引经新配对回执发布，第三个实包到达新接口的上游。随后取消上游，BIND 按原 Root 配置停用。共保留 5 组意图、结果及送达记录。Maven Networking 2231 项、Linux Root BIND 20 项通过。夹具仅为单个测试客户端放通 UDP 53，输入默认策略保持。

首个配置发布到链路本地地址就绪有短暂窗口，路由观测曾显示同一 WAN 的全局源地址。待实际链路本地源地址就绪后才发出首次验证查询。重插时前台 nmcli 报告中断，原连接随后由实际 NetworkManager 恢复，未重复重插或激活。无 zone 的 IPv6 源地址选择、动态前缀、DHCPv6/RA、systemd-resolved、实际 PPP、其余 typed brokers 及六组件矩阵仍为内部待办。外部延后仍仅原三项。原 7fd 停止，41 输入/16 回执。12 父项 0/12。见 [检查点](actual-core-nm-scoped-dns-checkpoint.json)。

## 2026-11-20 继续实施 — 静态 IPv6 LAN DNS 与双栈转发

Core 948bf00af8 为已配对的 typed BIND broker 增加双栈配置。IPv4/IPv6 各自只监听批准的 LAN 前缀，上游传输地址族独立于客户端。原 Root、旧请求及历史发布匹配规则继续生效。Networking 825d5dc 新增默认关闭的 `net.interface.<接口>.config.ip6.dnsProxy.enabled`，仅适用于显式静态 IPv6 LAN。发布使用系统有效解析器，保留已支持的本地 stub 选择；IPv6 上游需要路由可达的无 scope 地址。 [BIND 配置参考](https://bind9.readthedocs.io/en/v9.20.24/reference.html)

隔离 Debian 13 ARM64 实装 Core 与 Networking，经正常 Main 的已注册配置服务验证仅 IPv6。随后沿同一账本升级 Networking，修复可选 DNS provider 的晚绑定，并让已配对的双栈配置避开旧 IPv4 解析器。取走并恢复实际 DNS DS 组件时，网络配置服务保持同一注册 ID。provider 缺席期间完成 IPv4 网络配置，BIND 文件及账本保持，恢复 provider 后由监视器发布双栈，再撤回 IPv6 监听。独立网络命名空间共五次取得 IPv6 上游的 AAAA 答案。保留三次启动禁用及三次配置共六组同一 Root 下配对的意图、结果及送达记录。隔离夹具只为测试客户端放通 TCP/UDP 53，默认输入策略保持。Networking Maven 2229 项及 Linux Root BIND 19 项通过。

上游在首次仅 IPv6 阶段记录到网关的 LAN IPv6 源地址，最终两阶段记录到预期的 WAN IPv6 源地址。IPv6 上游切换期间的源地址选择、动态 IPv6 前缀、链路本地 DNS、DHCPv6/RA、systemd-resolved、实际 PPP、完整接管及六组件生命周期仍为内部待办。外部延后仅原三项。原 7fd 保持 41 输入/16 回执，父项 0/12。见 [检查点](actual-core-nm-dual-stack-dns-checkpoint.json)。

## 2026-11-20 继续实施 — LAN 代理遵循 NetworkManager 系统 stub 的分流

Core 800ce41270 将 LAN 监听能力与 broker 文件、systemd 单元和正常 Main 一起配对。typed `enable-lan` 仅监听获准的 IPv4 LAN，排除 loopback，并限制查询与递归。保留已有 `enable` 请求、文件渲染与账本协议。Networking 0c5cf77 在能力可用时将一个已识别 IPv4 本地 stub 作为唯一转发目标，保留系统路由策略。未知或多个 loopback 仍停用旧转发。 [NetworkManager DNS 配置合同](https://www.networkmanager.dev/docs/api/latest/NetworkManager.conf.html) [systemd-resolved stub 合同](https://www.freedesktop.org/software/systemd/man/latest/resolved.conf.html)

Debian 13 ARM64 实装 Core 与 Networking，用正常 Main 中的已装 OSGi 回调接上真实 NetworkManager 管理的 dnsmasq。BIND 的 TCP/UDP 53 仅监听 LAN，dnsmasq 使用 127.0.0.1。独立网络命名空间的四个 TCP/UDP DNS 请求分别走企业域和公共域，并核对实际收到请求的上游。企业域更换上游时，BIND 进程、配置和 Root 账本保持，公共域答案保持。验证完成后恢复直接上游，共保留三组原 Root 的 intent/result/delivered。夹具只为单个测试客户端放通 TCP/UDP 53，输入默认策略保持。Networking Maven 2227 项及 Linux Root BIND 17 项通过。

真实 PPP、systemd-resolved 现场验证、IPv6 LAN 代理、完整接管与其余 Core/OSGi/typed brokers/六组件矩阵继续为内部待办。外部延后仍仅原三项。原 7fd 保持 41 输入/16 回执，父项 0/12。见 [检查点](actual-core-nm-split-dns-checkpoint.json)。

## 2026-11-20 继续实施 — LAN DNS 转发跟随 NetworkManager 的有效解析器

Networking 60618e7 只读采集实际 `resolv.conf`，包括符号链接指向的文件。读取失败或格式损坏保留最后一次已应用配置，IPv4 解析器有效撤回时停用 LAN 代理。观测到本地 stub 时停用旧转发，避免代理回环和绕过分流。LAN 网段按规范化前缀比较，配置不变时零写入。所有变更继续使用已配对的 typed BIND broker。[NetworkManager DNS 配置合同](https://www.networkmanager.dev/docs/api/latest/NetworkManager.conf.html)

隔离 Debian 13 ARM64 实装 Core 4122fafa10 与新 Networking，以正常 Main 中已安装 OSGi 监视器回调验证。真实 NetworkManager 在两个 WAN 配置间切换，独立网络命名空间内 LAN 查询收到相应上游答案。夹具仅显式放通单个测试客户端的 UDP/53，默认输入策略保持 DROP。损坏、丢失和稳定快照保留服务、文件与账本，IPv6-only 或包含本地 stub 恢复发行版 BIND 配置并停用，回到直接上游后重新启用。Agent ownership marker 置位后有效上游继续变化，代理保留原状态。共 6 组原 Root intent/result/delivered、4 次独立 DNS 数据包查询，Networking Maven 2226 项通过，夹具正常停止。

实际 PPP 拨号、支持本地 stub 的分流转发、完整双族与接管矩阵、其余 typed brokers、完整六组件生命周期仍为内部待验。外部延后仅原三项。原 7fd 维持 41 输入/16 回执并保持停止，父项 0/12。见 [检查点](actual-core-nm-resolver-dns-checkpoint.json)。

## 2026-11-20 继续实施 — modem 自动 NAT 关联已激活的 IPv4 数据接口

Networking a929074 使用 NetworkManager 已激活且带有效 IPv4 地址的 IpInterface 作为 modem NAT 端点。旧快照 pppNum 保留在历史配置用途。NAT 监测仅在配置包含相关端点时运行，成功观测到成员变化才重算，观察失败保留原规则，写结果不明暂停该监测。Agent 防火墙门禁按配置中的逻辑 LAN/WAN 关系预检，新旧配置断线时都不能规避门禁。内核转发已经开启时避免再次写 sysctl。[NetworkManager Device D-Bus 合同](https://networkmanager.dev/docs/api/1.52.0/gdbus-org.freedesktop.NetworkManager.Device.html)

隔离 Debian 13 ARM64 实装 Core 4122fafa10 和 Networking。真实系统总线读取确认现场无 modem，用独立私有 D-Bus 提供两个替身 modem 的属性，在正式 Main 内用生产配置和防火墙代码写隔离内核 iptables-nft。固定 WAN 与第二 modem 的规则保持。首 modem 的规则随 ppp47 激活出现，观测期间接口突变时保留已提交规则，稳定后改为 ppp49，IPv6-only 时撤回该 IPv4 NAT。防火墙 ownership marker 置位后不再写，独立站点链及真实 NM 进程保持。转发 sysctl 已开启且只读，夹具正常停放。

Networking Maven 2222 项通过。实测覆盖内核规则与受控 D-Bus 观测。物理拨号和数据流量仍待验收。完整 PPP 协商、DNS/双族/NAT 业务、其余 typed brokers、历史矩阵、Core/OSGi 与六组件生命周期继续内部待办，外部延后仅原三项。原 7fd 41 输入/16 回执，父项 0/12。见 [检查点](actual-core-nm-modem-nat-checkpoint.json)。

## 2026-11-20 继续实施 — PPP LCP 配置的 D-Bus 类型与实际 profile 升级

Networking 257a1b7 保留旧快照 lpcEchoInterval、lpcEchoFailure 键，将 LCP 属性编码为 NetworkManager 规范的 uint32。负数及没有发送周期却启用失败阈值的配置在写入 profile 前拒绝。PPP profile 由现行 NetworkManager Settings 生命周期管理。[NetworkManager 1.52 PPP 设置](https://networkmanager.dev/docs/api/1.52.0/settings-ppp.html)

在隔离 Debian 13 ARM64 从 Git 构建 Core 4122fafa10 和 Networking 两个版本。基线 Main 的已安装 OSGi 转换器发送有符号正值，NetworkManager 1.52.1 接受并读回 30/5。通过实包升级 Networking 后，Main 写入 D-Bus 签名 u，先在同一 UUID、同一 Settings 对象读回 CHAP 的 30/5，再更新并读回 PAP 的 45/7。负数与缺少周期两种错误在 profile 写入前拒绝。真实 NetworkManager 重启后读回保持，Root 私有 keyfile 字节与元数据保持，profile 为非自动连接且未激活。夹具正常停放。

Networking Maven 2218 项通过。实际拨号和 PPP 协商、断线重连、实际 IpInterface 与 NAT/DNS 联动仍待验证。完整 Core/OSGi、历史迁移余项、其余 typed brokers 及六组件生命周期继续内部待办，原 7fd 41 输入/16 回执，父项 0/12，external deferred 仍仅原三项。见 [检查点](actual-core-nm-ppp-profile-checkpoint.json)。

## 2026-11-20 继续实施 — 配对撤回 Main 的直接 ModemManager 控制权限

Core 4122fafa10 和 Networking b0c5fcd 将包授予的 ModemManager 通配权限收敛为 GetLocation 所需的精确 Location 权限。Core 在升级及 Main 启动前核对 Reset、GPS provider 与 Networking 只读规则的包归属和哈希。Networking 在拆包前核对已安装 Core 的 GPS actor，从新包移出旧授权脚本。旧 Core 和 Networking 写入的确切规则可以合法迁移，只有精确的历史 ModemManager pkla 退役。[ModemManager 1.24 授权定义](https://github.com/linux-mobile-broadband/ModemManager/blob/1.24.0/src/mm-auth-provider.h)

隔离 Debian 13 ARM64 从当前两个候选和两个基线 Git 版本构建并完成双包升级。新版 Core 对旧 Networking 规则、新版 Networking 对缺失 Core actor 证明、以及新版 Core 对带站点改动的旧 pkla 都在实包预检时拒绝。实际先升级 Networking，后升级 Core。新版 Core 就位后真实 pkcheck 对 kurad 拒绝 Control 和 Device.Control，允许 Location 与 NetworkManager；root 权限、polkit 进程、站点规则和凭据指纹保持。隔离数据标记按既有 cp -a 流程恢复，内容、权限、所有者与 mtime 保持，inode 变更。正式 Main 启动后由 OSGi 读到真实 NM 网卡，并通过 GPS broker 就绪检查，0 意图、0 回执，夹具正常停放。

Linux 65 项定向、Networking Maven 2215 项通过。Debian 13 验证了旧 pkla 字节退休和当前 polkit 的实际授权。旧后端的动态行为属于历史验证余项，物理 modem 沿用原外部边界。完整 PPP/Core/OSGi、历史矩阵余项、其余 typed brokers、六组件/provider/launcher/安装升级移除 purge/中断恢复继续内部待办。原 7fd 保持 41 输入/16 回执，父项 0/12，external deferred 仍仅原三项。见 [检查点](actual-core-modem-policy-cutover-checkpoint.json)。

## 2026-11-20 继续实施 — 配对 modem 启用与 GPS 配置

Core 01c158f66c 和 Networking 6e01047 将已配置与未配置的 modem 的 GPS 策略转给已配对 broker。保留原先按需 Enable(true) 与 Setup 的顺序，支持关闭、托管 Raw/NMEA 和非托管模式。在首个实际总线写入前持久化一条意图，逐步重新核对 Main、Root 和物理目标。中途响应不明时依原编号对账，后续 Setup 被阻止。GetLocation 的独立读取权限仍用于位置 provider。[ModemManager 1.24 定位接口实现](https://github.com/linux-mobile-broadband/ModemManager/blob/1.24.0/src/mm-iface-modem-location.c)

隔离 Debian 13 ARM64 从 Git 出包并实装 Core 和 Networking。正式 Main 中的 OSGi 客户端经 GPS 入口拒绝真实 NM 回环设备。临时移出定位 provider 证明时在分配写入 ID 前拒绝，证明以原 inode 与字节恢复，Main InvocationID 保持。两次检查 Root 台账 0 意图、0 回执，夹具已停用并正常停放。Linux 定向 46 项和 Networking Maven 2215 项通过。物理 Enable/Setup 沿用原硬件边界，三种模式、顺序和响应丢失为定向证据。

实际 pkcheck 记录了收敛前 kurad 仍可使用 ModemManager 的 Control、Device.Control 与 Location。配对授予撤回与历史 pkla 迁移仍属内部待办，继续完整 PPP/Core/OSGi、其余 typed brokers、六组件/provider/launcher/安装升级移除 purge/中断恢复。父项 0/12，external deferred 仅原三项。原 7fd 保持 41 输入/16 回执。见 [检查点](actual-core-modem-location-checkpoint.json)。

## 2026-11-20 继续实施 — 收敛 ModemManager 定时重置

Core 97e0464cc4 和 Networking 51646e1 将原 ModemTaskScheduler 的直接 Reset 交给已配对的 systemd socket actor。它核对 Main PIDFD、Networking provider、两个原厂系统单元、NM 对象类型与 Udi、ModemManager 的配置 ID 和物理 sysfs 身份。写入 Root 私有意图后再次核对派发目标，只调用不带参数的 ModemManager Reset。收到总线肯定答复才写回执。同一 Root 有未确认的重置时先行拒绝新 ID；进程仍持有原操作时只用原 ID 查询并确认。 [ModemManager D-Bus 接口](https://www.freedesktop.org/software/ModemManager/api/latest/gdbus-org.freedesktop.ModemManager1.Modem.html)

隔离 Debian 13 ARM64 以 JDK 21 和 Maven 3.10.1 从 Git 出包，实际安装正式 Core 和 Networking。已启动的原生 Main JVM 经安装包的 OSGi provider 调用 broker，真实 NM 回环设备被拒绝，单独运行的 kurad 被身份检查拒绝，Root 台账为零意图、零回执。已从 Main 的 mountinfo 核对私有 Root 目录。夹具 disabled，正常停放。Linux 41 项定向和 Networking Maven 2215 项通过，正向 Reset 总线响应、丢失响应、旧 ID 与 Root 拒绝为定向证据。Debian 以原厂 ConditionVirtualization 跳过容器内的 ModemManager，物理业务仍在原外部边界内。

Enable、Location、MM 直接授权撤回的实际配对升级、完整 PPP 和其余 typed brokers 继续内部待办。Core/OSGi、历史权限及六组件/provider/launcher/安装升级移除 purge/中断恢复矩阵继续，父项 0/12，external deferred 仅原三项。原 7fd 保持 41 输入/16 回执。见 [检查点](actual-core-modem-reset-checkpoint.json)。

## 2026-11-20 继续实施 — 按包配对撤回 Main 的直接 WPA 授权

Core 3db2116cb4 和 Networking 3cb9bf2 将已安装 NM 扫描 JAR、Networking 授权规则与正式 dpkg 配对。包版只保留 NM、ModemManager 的原授权。历史两种确定的 Core/Networking polkit 规则可迁移，管理员规则保持。旧 WPA pkla 只在字节完全符合历史包授予时撤回。D-Bus XML 需与受保护的同路径备份以及旧安装器输出逐字配对，按原备份恢复并重新加载总线配置，支持已发布后重试加载。 [D-Bus 守护进程文档](https://dbus.freedesktop.org/doc/dbus-daemon.1.html)

在隔离 Debian 13 ARM64 从 Git 构建新版。实装旧 Core 后，其真实安装器生成已标记的 .save 与直接授权。实际系统总线旧版允许 kurad 获取测试用 WPA 服务名，三个 polkit 测试授权均通过。新版 Core 的已打包 preinst 在旧 provider 缺证明时拒绝，未向 dpkg 提交该次检查。实际先升级 Networking 后升级 Core，前一步单独撤掉 Networking 规则的 WPA 分支，Core 旧授权仍生效。后一步恢复原 XML 字节，退役确认的 pkla，系统总线拒绝 kurad 测试请求，root、NM 和 ModemManager 通过。D-Bus 进程、管理员文件、备份及凭据指纹保持。最终 Main 经正式包启动并返回真实网卡状态，夹具正常停放。

Linux 82 项定向、Networking Maven 2215 项通过。实际 .save 迁移通过，vendor .kurasave 两种精确输入、歧义拒绝及重新加载重试是 Linux 定向测试。真实射频仍按原硬件边界。完整历史权限矩阵、PPP/其余 typed brokers、完整 Core/OSGi 和六组件/provider/launcher/生命周期矩阵继续内部待办。12 父项 0/12，external deferred 仍仅原三项。原 7fd 保持 41 输入/16 回执。见 [检查点](actual-core-wifi-policy-cutover-checkpoint.json)。

## 2026-11-20 继续实施 — Wi-Fi 扫描由 NetworkManager 接管

Networking 56f9d84 在正式状态刷新路径使用既有 NetworkManager Wireless.RequestScan。调用前核定 D-Bus 唯一所有者、设备路径、接口名和 Wi-Fi 类型，仅接收同一所有者与设备的 LastScan 信号。未确认扫描留在当前实例和设备的等待记录中，后续刷新只读取同作用域的完成观察，已确认拒绝可释放本次等待。NetworkManager 1.52 的 supplicant 实现保持 AllowRoam=false。 [NetworkManager Wireless API](https://networkmanager.dev/docs/api/latest/gdbus-org.freedesktop.NetworkManager.Device.Wireless.html) [NetworkManager 1.52 源码](https://raw.githubusercontent.com/NetworkManager/NetworkManager/nm-1-52/src/core/supplicant/nm-supplicant-interface.c)

JDK 21 从 Git 构建，实际安装 Core d652342e41 和 Networking 6a85b70 的 Debian 13 ARM64 包。合法 Main 的真实 NetworkManager 1.52.1 返回本机 veth 状态，扫描入口拒绝真实非无线设备。另在只用于验收的独立 D-Bus 上，由实际安装的 NM bundle 发送两次结构化 RequestScan。其他设备路径的信号未完成等待，目标路径更新后正常返回。第二次无完成信号，后续刷新没有再次派发。私有总线与真实系统总线的 ID 分离，真实 NetworkManager invocation 保持。本机无无线设备，这一部分为真实 IPC 契约验证。实际射频路径仍在原硬件边界。夹具正常停放，Main 禁用。

Networking Maven 2215 项通过。旧直接 WPA 规则撤权及历史 XML/polkit 迁移、PPP 及其余 typed brokers、完整 Core/OSGi 与六组件/provider/launcher/生命周期矩阵继续内部待办。12 父项闭环 0/12，external deferred 仍仅原三项。原 7fd 保持 41 输入和 16 回执。见 [检查点](actual-nm-typed-wifi-scan-checkpoint.json)。

## 2026-11-20 继续实施 — DHCP 多链路、就地升版、移除与 purge

Core d652342e41 校验 dnsmasq 多接口的实际通配 UDP 67 socket。多接口的 socket 跨同一网络命名空间的接口范围，已有外部 IPv4 DHCP 监听时在意图前拒绝。broker 在原沙箱内复核配置文件身份、精确 systemd invocation 和 socket cgroup。运行时凭据目录的指纹由隔离现场的只读观察获得。 [dnsmasq 维护者对多接口 socket 的说明](https://lists.thekelleys.org.uk/pipermail/dnsmasq-discuss/2023q4/017349.html)

JDK 21 从 Git 构建，实装 Debian 13 ARM64。第二个 Root 从 Core 324aec118f，经 dfef47f5d3 到 d652342e41，两次真实版本单调上升且 Main 在升版前运行。两次包替换后旧 dnsmasq PID、进程启动时间、invocation、UDP socket inode、租约、DHCP 原回执和原 AES 凭据一致。中间版本的 worker 读不到其他服务的私有凭据目录，比较返回拒绝，零新增意图。最终版本业务 Main 只读比较通过，随后实际加第二条链路发租约，原租约按原条目保留，同 Root 两笔写入均有结果和确认。

独立隔离现场通过正式 dpkg 移除和 purge Networking，再移除和 purge Core。移除 Networking 时 Main 与 DHCP 原本均在运行，退出后无原 socket。四步沿原 Root 保存 DHCP 配置、租约、回执和 AES 凭据的字节与元数据。Core remove 的保留快照原字节与数字 UID/GID 权限一致，Core purge 按既有合同删除运行时恢复副本。原系统 dnsmasq 状态保持。两处夹具均正常停放。

第一处夹具遇到同一分钟构建导致 Debian 版本倒序，真实包替换和随后严格升版单独记账。旧监听门禁在增加第二链路后留下 UNKNOWN，原意图和现场保留，无重发、换编号或跨 Root 查询。Linux 定向 65 项、Networking Maven 2211 项通过。历史 DHCP 纳管与重装、竞争中断后的原编号核定、完整 Core/OSGi、其余 brokers 和完整六组件矩阵继续内部待办。冻结 12 父项 0/12，external deferred 仍仅原三项，原 7fd 41 输入/16 回执保持。见 [检查点](actual-core-dhcp-multilink-lifecycle-checkpoint.json)。

## 2026-11-20 继续实施 — DHCP 外部监听冲突与专用进程归属

Core 8e60accd21 用 Linux socket diagnostics 核对 IPv4 与能接收 IPv4 的双栈 UDP 67 监听。用正式 unit 的 systemd cgroup 判断 Kura 专用实例的 socket 归属，结合接口、主设备、地址及广播范围拒绝服务冲突。发布前重复核对，启动或重配后确认所需监听确实由专用 cgroup 持有。保留原包、已加载 Main、Root 配对和 UNKNOWN 围栏。 [Linux inet_diag 源码](https://raw.githubusercontent.com/torvalds/linux/master/net/ipv4/inet_diag.c)

从 Git 以 JDK 21 构建并在独立 Debian 13 ARM64 实装 Core 和 Networking 47a7dfb。独立运营方 dnsmasq 已绑定 kdlegacy0。实际 OSGi/NM 配置请求在同接口返回 DHCP_PORT_IN_USE，专用配置与元数据原样，专用实例未启动，未生成写入意图。另一条链路 kdfree0 成功启动专用实例，随后更改该链路租期只重启专用实例。两个实例同一 UID、不同 cgroup。运营方 PID、invocation、配置和 unit 指纹保持，Main 无重启，独立两笔 APPLIED 和确认保留。测试容器正常停放，原两处 DHCP 夹具未启动。

Linux 63 项定向、Networking Maven 2211 项通过。历史 DHCP 文件和存量租约合法纳管、实际就地升级与 remove/purge、竞争中断后原编号核定仍为内部待办。原 7fd 保持 41 输入/16 回执。完整 Core/OSGi、其余 typed brokers、六组件/provider/launcher/安装升级移除 purge/中断矩阵及冻结 12 父项范围保持，最终闭环 0/12。external deferred 仍仅原三项。见 [检查点](actual-core-dhcp-listener-authority-checkpoint.json)。

## 2026-11-20 继续实施 — 类型化 DHCP 与实际 NM 发租约

Core 324aec118f、Networking 47a7dfb 从 Git 构建，安装实际 Debian 13 ARM64 包。新 Core 为应答器配对正式 Networking 的两个 JAR 和已加载 Main。同 Root 记录意图、结果及结果确认。专用 dnsmasq 从 systemd 凭据目录读取结构化模型生成的固定配置，仅绑定所需接口，保留系统 dnsmasq 的配置和状态。

在独立容器由 OSGi ConfigurationService 更新正式 NM 配置，默认防火墙丢弃首个客户端请求。仅对隔离 veth 的 UDP 68 到 67 临时放行，实际 DHCP 请求、应答与 ACK 成功，租约 198.51.100.157 落盘，临时规则撤回。随后同入口禁用 DHCP，专用进程退出，Main PID 和 invocation 未变化，原系统 dnsmasq 状态与所查配置指纹精确，保留两笔原 Root 结果和确认。第一份 Core 的专用服务无法读取私有父目录，原 Root 保留单笔 UNKNOWN。修复通过 systemd 凭据目录交付配置，第二个 Root 使用独立链路和地址池，未查询或重发第一笔。两处新夹具均正常停放。

Linux 定向 60 项、Networking Maven 2211 项通过。另做 Python 全量 discovery，两版本在同一个缺少所需夹具的镜像中失败及报错的测试名称完全一致，此运行不作完整验收。DHCP 历史接管、并存服务、多链路及未决写核定，完整 Core/OSGi、其余 brokers、六组件/provider/launcher 和安装升级移除 purge/中断恢复继续内部待办。原 7fd 41 输入/16 回执、全部冻结 12 父项和三项 external deferred 保持。见 [检查点](actual-core-typed-dhcp-checkpoint.json)。

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
