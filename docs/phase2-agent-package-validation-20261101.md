# 独立 agent 包、身份和 Kura UID 授权验证

日期：2026-11-01。整体改造继续进行，阶段 2–5 未完成。本增量接续
`phase2-netplan-package-validation-20261101.md`，没有重复迁移仓库或清理旧运行集。
React 中英文要求继续适用，详见 `console-i18n-20261101.md`。

## 实际代码

独立 agent 仓库的 `packaging/build.py` 增加两步输入：先校验版本化 protocol crate，
生成包含 vendor 和源文件 SHA 的独立源码目录；再校验 Linux ELF、架构、glibc 和 binary SHA，
构建 `kura-management-agent` deb。开发 Cargo sibling patch 保留，staged source 不依赖 sibling。
实际 protocol crate 0.1.0 来自本地 `cargo package --locked --allow-dirty --no-verify`，未发布 registry；
随后 staged agent 已实际编译及运行测试，不能将 `--no-verify` 当作协议发布验收。

`setup.py` 提供 root-only 的 import-identity / authorize-kura / check / activate。
先捕获有界输入字节，在私有临时目录校验证书链、client 用途和 keypair，再写 root0600 身份。
配置和状态目录 root0700。新安装不注册、不启服、不接管接口。
注册由中心管理员 CLI 显式完成，独立于 Kura Web 账号。
相同身份停止后重复导入幂等；另一 console、不同身份或中断导入均拒绝覆盖，没有 force 抢占。

默认 `AGENT_KURA_UID=4294967295`，不是 nobody 占位授权。
authorize-kura 只接受现有非 root 的 `kura-netplan`，记录实际 UID，安装本 unit 的
`SupplementaryGroups=kura-management` drop-in，不用 usermod。
Kura 与 agent 的网络写接口 allowlist 均为空。runtime0750、socket0660。
打包 unit 仍为空 capability bounding set；这是观察模式候选，不是网络 writer 发行授权。

维护脚本各自嵌入完整 Python 生命周期逻辑，postrm 不依赖已删除的 payload。
preinst 使用 Python，因此 deb 声明 `Pre-Depends: python3`。
升级和普通卸载先检查 writer/journal、停止服务；再激活须显式执行。
普通卸载保留身份、快照、配置、状态和日志；purge 仅清 agent 专属目录及内容未改的授权文件。
非空 writer、未终结事务、已有 confirmed 网络或外部修改授权均要求明确交接。
system group 和系统网络不删除。完整 writer 交接流程仍 pending。

说明与操作入口在 agent `packaging/README.md`。验证辅助代码为 console
`scripts/phase2-agent-package-vm.py` 和 `scripts/phase2-agent-package-smoke.py`，
按固定非生产 VM/网关、单向状态及不覆盖回执执行，不作为生产安装工具。

## 开发验证与 IDEA 门槛

| 实际范围 | 本轮结果 | 证据，相对 console |
| --- | --- | --- |
| 身份、授权、生命周期、归档与锁回归 | macOS 和 Linux 各 15 项通过 | `target/agent-package-unit-lock-fixed[-linux]-20261101.log` |
| 独立 staged source 的 Rust 测试 | 40 项通过 | `target/agent-package-rust-linux-tests-20261101.log` |
| 同一 staged source 的 Linux 编译 | 成功，测试用 debug ELF | `target/agent-package-rust-linux-build-20261101.log` |
| React 默认浏览器集合 | 11 项通过，0 skipped/unexpected/flaky | `target/agent-package-react-browser-20261101.json` |
| IDEA 真实 Run 完整隔离 Kura | PID15938；原控制台登录和状态页可见；IDEA Stop 后独立 ps 确认退出 | `target/agent-package-idea-console-20261101.png` |

单元/浏览器和 IDEA 门槛完成后才进入本次实际包安装。后续仅修改 packaging 的启动锁和测试，
没有改 Rust/React/Java 生产源码。11 个 Rust 输入文件及 protocol SHA 再核对与 staging manifest 一致。
没有把历史构建/冒烟当作完整新架构验收，没有声明 cargo fmt 已通过。
宿主默认 Python3.9 缺 tomllib，使用 Homebrew Python3.15 跑打包测试；源码 staging 在 Linux Python3.12。
初次 cargo 路径及 ResourceWarning 失败的旧日志保留，后者已修正。

## 包制品与失败修正

| 用途 | 版本/修订 | SHA256 |
| --- | --- | --- |
| protocol crate | 0.1.0，本地制品 | `1412d0c81462c29538a2d5133bd9b544b5bac85cb11f70d6e40fde5097a80c7b` |
| Linux arm64 binary | staged source debug ELF | `45959c228cabffc258fc6b133df68428db1a1e60b0f6097eecf5ed350931cf83` |
| agent 首装候选 | 0.1.0~package20261101-2 | `c94723afae217e673e2cb0a7cb1a5a94ae1372dfc4b9517746a51857336385f5` |
| agent 启动锁修复候选 | 0.1.0~package20261101-4 | `82c13c294c1ffc0c88bbd5aa4618e5ba9b56df96ae5bad519b9f1feb05ed8a93` |
| agent 升级/重装/最终候选 | 0.1.0~package20261101-5 | `b67c055f7bf71edf8c3d437379334ca9c5d828b674e6018b3bdc6a1cbd20ffdb` |
| 搭配 core | 6.0.0~netplan20261101-3 | `b72b7009fd71005d577b387df351a6058e4c1cffe5b6aa9b85afb237e857e3e9` |
| 搭配 networking | 3.0.0~netplan3 | `d03423942fe0cbc83ce30b4313e50b9be888fb2f2b63d97365256742c17c1a4c` |

最终 agent deb 位于 `target/kura-management-agent_0.1.0~package20261101-5_arm64.deb`，
对应 receipt、静态解包记录与 source manifest 均在 `target/agent-package-*`。
19 个 payload entry 全部 root:root，仅 `/usr`，四份维护脚本可执行且自包含，未打包身份/state。
ELF 为 AArch64，glibc 最大需求 2.39，ldd 只有 libgcc_s、libm、libc 和 loader。
debug binary 的性能/优化和正式发行验收仍未完成。

首装候选2在身份成功导入后发现 activate 死锁：父进程持 setup lock 等待 systemctl，
ExecStartPre 的 check 等同一把锁。已停止该次 start-pre，保留身份及失败日志，修复为
检查后释放锁再启动；ExecStartPre 再检查。同时拒绝 ActiveState=activating/deactivating
时修改身份，即使 MainPID=0。新增两项回归，15项在两种环境通过。
候选4原地升级并成功激活，候选5随后验证升级/重装。不得声称最终候选重做全部首装步骤。

首次 HTTP 检查使用 Python3.15，现有开发 server cert 缺 Authority Key Identifier，被其严格校验拒绝。
后续使用此前开发验收的系统 Python3.9，并继续使用可信 CA 和主机名验证，没有关闭 TLS 校验。
另一次检查发生在死锁 agent 尚未在线时而失败；保留为失败，不计通过。
静态审计最初对 tar 根条目 `.` 的路径归一化有误，随后工具 `file` 缺失，改用已有 readelf 完成最终审计。
这些工具失败不改变实际包哈希或身份。

## 实际 OrbStack 业务与生命周期

VM 为既有 `yofc-management-network-20261101`，Ubuntu24.04 arm64。
新 fixture `/opt/yofc-agent-package-20261101`；独立 gateway `package-gateway-20261101`，
单独客户端证书、结果数据库和默认 socket。未重复登记/覆盖 network-gateway，未运行旧 install fixture。

| 真实执行 | 结果/边界 |
| --- | --- |
| apt 模拟与首装三包 | 不新增 NM/MM/iptables；agent/Kura 均未自动启动；身份不存在 |
| 显式导入、重复导入、第二中心覆盖尝试 | 导入/幂等成立；第二中心拒绝，身份与状态未变；首次激活死锁按前节修复 |
| Kura 未运行时，经 HTTPS/mTLS 查询 | 11 检查通过；agent.status/network.inspect 成功；配置/快照为 KURA_UNAVAILABLE |
| OS 授权、完整低权限 Kura 启动 | 实际 UID997、Kura CapEff0；真实 Java UDS bridge 注册 |
| 实际配置/快照/网络只读业务 | 12 检查通过，含原外部网络 PID 可见 |
| 运行中重启 packaged agent | 6组检查通过：PID变化、mTLS lastSeen推进、Kura不重启自动重新注册、三项读任务成功 |
| OS 权限 | agent所有16线程 no_new_privs=1/CapEff0；runtime0750/socket0660；nobody连接返回EACCES |
| 停止 Kura | Result=success；agent仍在线；停止后11检查再次通过 |
| 升级4→5、普通remove、重装 | 身份、Kura数据、配置和状态逐文件SHA保持；服务不自动启动 |
| 最终5重装后显式启动 | 真实业务12检查再次通过，复用原身份和 UID 授权 |
| 停服、归档和purge | 专属路径及自己的授权删除；root0600归档留在VM；无网络写入 |

这些只读查询经真实中心/PostgreSQL/mTLS gRPC/Java UDS/Kura OSGi，不使用 test-only owner。
本 core 包未带 GWT，开发运行集登录结果不替代新包 GWT/REST/初始改密验收。
没有模拟为已验证的包业务，没有新增网络写入授权。

新 fixture 最终 `phase=purged`，12份阶段回执；失败激活和 root0600
`retained-before-purge.tar.gz` 仍在VM。测试 gateway 已在中心显式 disabled，审计保留。
宿主 `.phase1-dev/agent-package-20261101` 仍为本地私有开发身份目录，不入图谱/证据归档。
现有 `yofc-management-phase2-agent.service` 始终 active，其 env、系统 Netplan/networkd/NM 配置及
两个旧 fixture state SHA 始终不变；旧 profile仍restored、旧包fixture仍purged，禁止删state重放。
Parallels Ubuntu/Windows 均保持stopped，没有增开VM，也没有连接192.168.2.125。

## 图谱、证据与剩余项

官方 CLI/permanent daemon 已刷新真实 agent/console 子仓库：671/2681、4129/8783 nodes/edges。
13关键路径中12个metadata_match，unit为not_tracked，已核对源码及真实deb。
console仍有既有SQL/Mosquitto解析缺口；未索引父目录。原聊天MCP连接仍Transport closed，
CLI可用不能描述为聊天连接已恢复。文档补记发生在本轮索引后，不等于文档已自动刷新。

机器可核对清单：`docs/evidence/agent-package-20261101.json`。此前证据不覆盖，失败记录保留。
代码保持未提交状态；没有 reset/clean、推送或远端整理。

完整 systemd ProtectSystem/ProtectHome/PrivateTmp 在 OrbStack全局LXC覆盖下 **deferred**；
process no_new_privs/零cap的实际检查不能替代这些项目。未弱化生产unit，也未套用旧Parallels结果。
当前真实包矩阵仅Ubuntu24.04 arm64；26.04与amd64的新包验收、物理有线/硬断电均deferred。
阶段2完整旧字段/NM/cloud-init交接、可写agent发行权限与排空/交接、Kura断连持久化策略、
完整业务桥接、登记/转交/轮换产品流程、nftables、系统控制、离线恢复、发布矩阵仍pending/deferred。
