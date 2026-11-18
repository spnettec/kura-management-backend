# Core 配对 static 保留转换与真实 Main 启停

2026-11-18，Core 实现 `40268c05c37d88963b16790c47e65fdb9d1b3a94`。续接起点为 Core `d49e65824f`、GWT `18c1b5eed4` 和 console `f97ab3d`。启动实施前 18 仓分支、HEAD、树指纹和清洁状态均匹配暂停回执。官方 Obsidian CLI 从父工作空间读取暂停交接、Tasks 和 Session Summaries，三个全文 SHA 与暂停读回一致。

## 实现与构建

static 准入同时校验现有 schema 2 包记录的 Main 摘要、固定 vendor 路径、Root 文件和 PID1 当前加载的 fragment。外链、替代 fragment、drop-in、过期加载和无包定义的 static 在新围栏前拒绝。移除保存时重新校验并生成 schema 2 的原 Root HMAC 回执。重装沿用原围栏、原 UID/GID 及保存的 unit 摘要，明确校验新包的有效 Main。在安装型单元上把原 static 选择呈现为 disabled，无 enable 或 start。schema 1 回执继续按原合同核验，其 static 仍需审查证据才可转换。其他已支持选择继续走原路径。

50 项定向回归通过。另有 69 项相邻回归，68 通过，Linux renameat2 用例在 macOS 跳过。新 Git 检出、JDK 21、Maven 3.10.1 和新隔离缓存顺序构建 support、Core/tools、distrib。ARM64 标准包版本 `6.0.0~git202611180332.40268c0-1`，203553172 字节，SHA256 `3188ad6f415945c65fcc2461536f3c7e54b50767a22bdcd8d4e502ca955bfe6e`。两个包各核对 39 个 key/generation 文件、3 个 key 单元、Main 摘要、嵌入的 lifecycle/clock guard 及控制脚本。

## 真实包矩阵

独立 Ubuntu ARM64 fixture 从相同 Git 配方构建 static 输入包，唯一源码差异是模板省去 `[Install]`。包生成器在正常构建中为这份 unit 生成精确清单。两次夹具构建因独立检出未先生成 offline-recovery 载荷失败，现场保留。补全该模块的源码构建后，实际首装、remove、用标准包重装均 exit 0。23 份旧输入连续，Main 从 static 转为 disabled，未启动。原 remove 围栏 `856ef3a4-ae63-49cc-b171-782c46aee03f` 由重装完成并保存在历史中。

同一测试现场单次显式 purge exit 0，6 个配对单元及包自带的 `preserved-state.conf` 移除，15 份外部输入精确，原 static 移除回执保留。初次只读采集误把该已声明 conffile 列为应保留，错误回执留存，后续核对未再次分发 purge。purge 后沿原维护 `4f7f0d9a-c8a3-4a71-a8d7-c9faff8336c0` 单次重装完成。重启后 15 份外部输入只变化 st_dev，原九字段精确。随后一次 Main 启动，ClockService 激活，正常 stop 的 systemd Result 为 success、退出值 143、耗时 0.562 秒。夹具已正常停止，当前包保留且 Main 选择为 enabled。

## 原 7fd 现场

沿原 39 输入和 15 回执对标准包单次升级，31 个外部输入完整指纹精确，8 个 Core 数据的逻辑字段精确。新增一个正常升级回执 `bb951367-b2e7-44e6-9777-9894ae55cfbd`，至 40 输入及 16 回执。原账号、密钥和历史 .data 继续使用。

首次启动的 LoadCredential 预检因容器私有 `/run` 传播而读不到加载目录。公开 canary 在相同隔离配置下复现 ENOENT，修正 fixture 的 `/run` 传播后可以读到。原 Core 在修正后启动，实际 Root writer lease 与同 PID、ticks、启动 id、已装 cohort 和原密钥摘要配对，检查到 FD 9 及内核持有锁。Equinox、ClockService、ConfigurationService、HTTPS keystore 及 HttpService 激活。该次容器本机名解析报错，Main 普通 stop 达到 90 秒超时，systemd 记录 SIGKILL。40 份原输入仍完整精确，并新增一个配置分派的运行锁，总 41。

独立 fixture 的本机名解析及正常停机结果出来后，在同一个 7fd 上按原数据续验。重启时 41 份输入仅变 st_dev，独立记录新 boot 的基线。仅调整该容器内 `/run` 传播和本机名的 loopback 解析。无外部网络、宿主挂载或 SYS_TIME。再次核完整代码和账户配对后，Main 以 disabled 选择运行，ClockService 和配置服务启动，当前启动日志本机名错误为 0。普通 stop 返回 0，systemd Result success、秒数 0.504，Main inactive、PID0、NRestarts0。采集器最初固定要求退出 143，保留该断言失败，随后用已存在的实际结果和只读验证完成，未再发送 stop。

最终 41 份输入在本 boot 的全指纹精确、16 回执，`7fd` 容器正常停止 exit 0。下一轮使用 `7fd-shutdown-recovery-final-observation.json`。原 key/clock 业务和日志化 startup 均为 0，clock namespace 未创建。capability 观察受本容器限制，实际 E 的 Main 仍为 11 ambient、13 bounding、NNP0。

## 保留与未结项

原三个网关、个人 25 文件、C 和 D 现场均完成前后复核，E 原运行、权威记录和通道检查保留，两台 Parallels VM 保持停止。UNKNOWN、旧备份/归档/轮换/恢复/续期和已完成的维护目的均沿原根与历史保留。无推送。

已接受的是一个声明配对的 static 转换、独立包的 purge 和重装、原 7fd 的 Core 启停及有限 OSGi 服务观察。历史 schema 1 static/实际 linked 审核迁移，完整 Core/OSGi 业务、当前 Main 时钟写入、RTC/Chrony 与其余 typed brokers、六组件/provider/launcher/升级移除及中断恢复矩阵继续内部 pending。12 个父项完整保留，最终闭环 0/12。仅实际 NM/cloud-init/物理交接、amd64、磁盘满/硬断电仍为 external deferred。

私有回执和完整指纹位于父工作空间 `.management-close-20261116/resume-20261118.FqDevXKS/`。机器摘要见 [当前检查点](../checkpoints/20261116/core-paired-static-retained-checkpoint.json)。
