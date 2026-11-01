# Netplan 开发运行集与真实 Linux Kura 委托

2026-11-01。本增量提供可选择的开发运行集和真实 Java → UDS → Rust agent 网络事务证据。
正式 deb/profile、NM/cloud-init 交接和阶段 2 完整验收仍未完成；整体 goal 继续 active。
各仓库现有未提交改动保留，未 reset/clean、提交、推送或修改远端。
父工作空间交接为
`Experience/2026-11-01-Kura管理体系Rust-React-Netplan开发运行集与真实Linux委托交接-411e4f9c.md`，
经官方 Obsidian CLI 写入并全文读回；Tasks 首项更新，其余任务保留。

## 实现

核心仓库 `kura-dev-runtime/network-providers.json` 明确区分 `none` 和 `netplan`。
默认运行集仍为 278 bundles。显式 Netplan 变体为 280 bundles，增加本机 bridge 和 Netplan provider，
在框架启动前固定 `kura.networking.provider=netplan` 及旧网络 PID 的外部归属。
旧 NM、linux.net、防火墙、threat writer 与新变体冲突时拒绝组装。

`tools/network_profiles.py` 被 Maven 生成器和运行集工具共同使用；Netplan 输出到独立
`target/runtime-netplan`，使用独立数据目录。prepare 校验 profile 标记、归属预留、socket 和 agent OS 用户，
非法输入在替换运行集或初始化数据前拒绝。clean guard 同时保护两个运行集。
此选择本身不修改系统网络、不授予接口管理权，也不自动停用 NM/cloud-init。

真实 Linux 暴露出原 GWT 的旧 WAN 校验问题：无默认路由的静态接口仅改 MTU 也被拒绝。
`TabIp4Ui` 现在仅在 Netplan 模式允许空网关，非空仍校验 IPv4；旧 provider 规则不变。
同步修正清空网关的 ValueChange 校验，并增加无网关、空 DNS、仅改 MTU 的无损契约回归。
此次没有新增 UI 文案；已有 React 和 GWT 中英文保留。

## 验证顺序与结果

先执行运行集单元/契约和 React 浏览器，再 IDEA 完整隔离 Kura/原控制台登录，最后复用 OrbStack。

| 检查 | 实际结果 |
| --- | --- |
| 运行集 Python 测试 | 18 项通过，含 profile 隔离、冲突 writer、预留及非法输入 |
| React | 类型检查/生产构建成功；默认浏览器 11 项通过，0 skipped/unexpected/flaky |
| 原 GWT | 完整 login/denali 编译与 Maven install；63 项服务端测试通过，0 failures/errors/skipped，含本次新增 1 项 |
| Maven 组装 | 显式 Netplan 变体实际组装 280 bundles；原默认运行集三份配置/清单 SHA 保持一致 |
| IDEA | 初次 PID 6505，修正后 PID 7884；完整隔离 Kura、真实 bridge/provider 激活和原控制台 admin 登录 |
| 宿主中心链路 | HTTPS → mTLS gRPC → 真实 Java UDS，在线 12 项和停 Kura 后 11 项均通过 |
| Linux 中心链路 | 真实 Linux Kura 在线 12 项；恢复并停 Kura 后 11 项通过，agent 和 network.inspect 仍可用 |
| Linux 原 GWT | 中文明确回退、英文确认提交及恢复基线；真实低权限 Kura UID 999，不含 synthetic owner |

宿主运行集与数据目录分别为 `kura/kura-dev-runtime/target/runtime-netplan` 和
`.phase1-dev/netplan-profile-kura-home`，端口 19080/19445/19446。
两个 IDEA 实例均由 IDEA Stop 停止，独立进程核对已退出。
原 `target/runtime`、个人 `~/.kura-dev` 和旧一次性 GWT fixture 未改动或重放。

## 真实 Linux 事务

复用 `yofc-management-network-20261101`，Ubuntu 24.04 ARM64；Parallels 保持停止，未增开 VM。
从其 Ubuntu 源安装 OpenJDK 21 JRE。使用同一 280 bundles 的开发运行集，仍含平台 emulator 服务，
不是正式包、完整硬件平台或发行矩阵验收。浏览器经 OrbStack 自动端口转发访问 `127.0.0.1:19081`。

Kura 用户 `yofc-kura-profile` 为 UID 999 / GID 990；systemd unit 未 enable。
agent 临时只向该 UID 委托 `yofc-test0`，socket 为 root:GID 990、0660，父目录不可由 Kura 写入。
`eth0`、`yofc-adopt0` 和 `yofc-preserve0` 在原 GWT 中不可写。
下面所有网络写入均由原控制台经真实 Java bridge/provider 发起；辅助观察器只读。

| 步骤 | 实际 MTU | 确认 MTU / revision | 结果 |
| --- | ---: | --- | --- |
| 初始基线 | 1500 | 1500 / 3 | 静态 192.0.2.2/24，空 routes/DNS |
| 中文 prepare | 1390 | 1500 / 3 | WAIT_CONFIRM；填入再清空网关事件校验也通过 |
| 中文明确 rollback | 1500 | 1500 / 3 | ROLLED_BACK，所有网络基线值恢复 |
| 英文 prepare | 1390 | 1500 / 3 | WAIT_CONFIRM，确认按钮受勾选门槛约束 |
| 英文 confirm | 1390 | 1390 / 4 | COMMITTED，页面重新读取确认值为 1390 |
| 英文恢复 prepare/confirm | 1500 | 1500 / 5 | COMMITTED，最终 YAML 内容/哈希恢复 |

共 3 次准备、1 次明确回退和 2 次确认提交。独立 `ip -j`、配置 SHA 和确认 YAML 观察证明：
待确认时系统候选和确认值分离；最终受管 YAML、eth0 来源、默认路由、dummy 地址/MTU 全部恢复原值。
revision 从 3 正常递增到 5，不回写旧 revision。初始 Kura snapshot_0 SHA 未改变。
确认勾选属于此 dummy 接口的受控 UI 测试，不能记作真实受影响管理路径重连证明。

`scripts/phase2-network-profile-vm.py restore` 已停止 Linux Kura，恢复原 agent env（Kura UID 65534、
无网络委托）与 socket root:root，移出临时 group drop-in。保留 JRE、独立账号、JAR、数据和证据。
fixture state 为 restored，不能删除 state 后重跑 prepare/resume。更新 web2 时先停 JVM，旧 JAR/清单保留
在 `/opt/yofc-kura-netplan-profile/web2-before-<sha>/`。

## 证据与失败记录

本增量哈希清单：`evidence/network-profile-20261101.json`。旧 REST/GWT manifest 保留历史语义。

- `target/netplan-profile-unit-final.log`、`netplan-profile-react-browser.json`：18 项和 11 项。
- `target/netplan-profile-gateway-fix-build.log`：修正后的完整构建及 63 项。
- `target/netplan-profile-{maven,gateway-fix-assemble}.log`：实际 280 bundles。
- `target/netplan-profile-idea-*.png`、`netplan-profile-idea-fixed.png`：IDEA/provider 与原控制台。
- `target/netplan-profile-{host-live-fixed,host-stopped-fixed,vm-live-fixed,vm-stopped}.json`：中心链路。
- `target/netplan-profile-vm-{before-write,mtu-pending-rollback,mtu-rolled-back,mtu-pending-confirm,mtu-confirmed,baseline-pending,baseline-confirmed,restored}.json`：实际系统观察；新观察器不记录确认令牌。
- `target/netplan-profile-vm-*-{zh,en}.png`、`netplan-profile-vm-final-dom.txt`：实际双语事务页。
- `target/netplan-profile-vm-journal.log`、`netplan-profile-final-check.json`：真实 Linux 日志和独立收尾检查。

第一次 VM prepare 因辅助脚本误用 `bundles.json` 失败，实际为 `runtime.json`。
保留失败日志，限定该失败状态 resume 后成功，没有删数据或重初始化。该 state 现在已恢复，不可重放。
宿主直接访问 VM IP 超时，随后使用已验证的 localhost 转发；不将此记为代码失败或 IP 验收通过。
JVM 更新后旧浏览器会话失效，旧未保存表单阻止导航，关闭旧 tab 后重新登录成功。
最初真实接口 MTU 操作被 GWT 空网关校验挡住，没有发出 prepare；修正后才有上述三次真实网络写事务。
首轮收尾脚本把 inventory 总条目数误当成 plugin bundle 数而断言失败：实际为 280 个 plugin，
另有 launcher/framework 两项，282 项 SHA 均正确。修正统计口径后由
`scripts/phase2-network-profile-evidence.py` 重新核对，不把首次核对记作通过。

## 图谱与剩余范围

只刷新真实 core、management-ui、console 子仓库并核对关键路径覆盖；不索引父工作空间。
原聊天 MCP 通道仍报 Transport closed，官方 CLI/permanent daemon 可用，不能声称旧通道恢复。
解析缺口和 not_tracked 配置按覆盖报告保留，索引 ready 不代表完整解析。
本次 17 条关键路径为 metadata_match/no_recorded_issue；输出见
`target/netplan-profile-graph-*.json`、`netplan-profile-coverage-*.json`。

正式 provider/deb 选择和包生命周期、完整旧字段写入、真实 NM/cloud-init 交接、登记/转交/证书轮换、
nftables、系统控制与离线恢复仍 pending。另一 console 不自动抢占，注册认证独立于 Kura。
DHCP/resolver、发行矩阵、物理有线管理路径/拔插/断电等缺失环境的验证保持 deferred。
原 GWT 状态页观察到缺失 HwType 的日志，原因尚未核实；终态还显示原 deadline 剩余秒，
两项 UI 改进仍 pending，不影响本轮事务终态/禁用规则。React 大主包拆分仍 pending。
