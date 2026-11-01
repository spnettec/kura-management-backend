# 阶段 2 受限网络写入者归属与恢复证据

2026-11-01 后续更正：本文“完整 VM 尚未创建”和“非 LXC 沙箱 deferred”已被限定范围的新证据替代。
用户已安装 Parallels Ubuntu 26.04 ARM64（2 vCPU / 4 GiB），实际只读沙箱、服务及 VM 正常重启通过；
多网关采用 Parallels + OrbStack。原 24 项单元、39 项写入者结果仍属于原二进制快照，
不追改历史计数。新兼容构建、范围及未完成边界见 `phase2-parallels-validation-20261101.md`。

日期：2026-11-01，Asia/Shanghai。整体改造继续进行。本轮补齐受限 Netplan/networkd
后端归属检查，未替换 Kura 旧网络 provider，未完成一般化 NM/cloud-init 交接或正式发行。
此前目录整合和未提交改动均保留，没有 reset/clean、提交、推送或更改远端。

## 实际代码

agent 新增 `src/network/writers.rs`，普通变更和明确来源转交共用服务、vendor 文件与
runtime backend 的检查。将安全来源放在私有 staging 中实际生成，按内容识别
`/run/systemd/network` 的归属；伪装生成文件名、未知 backend、drop-in 和 NM profiles
均拒绝。只读记录发行版文件的内容/所有者/权限，检查活动 NetworkFile/drop-in。
快照进入计划摘要和持久 journal，确认与恢复时重查；服务/vendor 漂移保留现场并拒绝覆盖。

WAIT_CONFIRM 必须严格匹配当前生成结果；恢复时只额外允许 journal 能归属的旧/候选
字节组合。旧 journal 缺少新增快照仍做当前归属检查，不伪造历史状态。首次接口回滚后
已恢复未受管且生成文件确实消失时，允许继续补偿；候选仍在却无活动归属继续拒绝。
`src/transport.rs` 仅传递有界的领域诊断，内部错误细节不进入任务结果。

新增 7 个单元测试，agent 总计 24 个。新增 writers 契约/运行脚本，已接管接口回归增加
独立 evidence prefix；浏览器截图也接受 prefix，避免覆盖历史验收文件。测试安装器先检查
授权范围，再停服/替换二进制，不放宽允许接口。

## 实际失败与修正

第一次真实 Linux writers 验收发现：Python `netplan generate --root-dir` 包装器会全局
reload udev/systemd，宿主 generator 随后重写运行 backend，使外部漂移在校验前消失。
直接运行已安装的 `/usr/libexec/netplan/generate --root-dir` 可只生成 staging，保留真实
运行文件。生产代码已改为原生生成器，不回退 Python 包装器；真正应用仍使用 Netplan apply。
该失败尝试不计通过，注入内容已恢复，未确认事务经显式回滚结束。

随后首次空接口回归发现：MTU 外部漂移使第一次补偿进入 RECOVERY_REQUIRED；修正 MTU 后
再次回滚，归属检查误要求 journal 候选仍有活动 NetworkFile。已按磁盘实际文件和旧来源
允许该未受管状态，补充单元边界，原失败现场通过授权本机回滚恢复，未改 journal。
恢复模式仍要求操作者显式重试；重启不会自动清除 RECOVERY_REQUIRED。

失败记录及恢复见 `target/writers-failed-attempts.json`、
`target/writers-recovery-from-failed-run.json`。

## 验证结果

| 检查 | 结果 | 证据 |
| --- | --- | --- |
| agent macOS/Linux arm64 | 各 24 单元通过，Clippy all-targets `-D warnings` 与 build 通过；macOS fmt check 通过 | `target/writers-agent-checks.json` 为实际工具输出的结构化摘要；旧 raw log 不代表最终版本 |
| HTTPS/gRPC 领域错误契约 | 7 项通过 | `target/writers-contract.json`；宿主仍仅观测，不修改网络 |
| 浏览器实际网络观测 | 1 通过，0 unexpected/flaky | `target/writers-browser.json`、`writers-network-observe.png` |
| IDEA 完整隔离 Kura | admin 原控制台登录；新 Rust/gRPC/UDS 查询 24 个配置 PID；验后停止 | `target/writers-kura-live.json` 最后 recoveryFixRun PID 79854 已退出，`writers-kura-recovery-login.png` |
| Ubuntu 写入者归属与恢复 | 最后恢复修正后 39 项通过 | `target/writers-linux.json`；包括外部漂移拒绝、现场保留及候选来源消失后的恢复 |
| 普通网络回归 | 最后恢复修正后 17 项通过 | `target/writers-network-regression.json`；包括超时、agent 重启和真实测试机正常重启 |
| 首次空接口补偿 | 修正后 24 项通过 | `target/writers-initial-link-regression.json`；包括 MTU 纠正后的显式恢复及初始 down 状态重启补偿 |
| 已接管接口普通修改/回滚 | 修正后 7 项通过 | `target/writers-adopted-link-regression.json`；IPv4/MTU 实际变更，IPv6/原持久来源保留 |

真实运行 fixture 为 `yofc-management-network-20261101`，Ubuntu 24.04.5 arm64，
`systemd-detect-virt` 为 lxc。实际 Netplan/networkd 管理专用 dummy；未安装 Kura，
仍可由独立 agent 接收任务、回滚和恢复。NetworkManager 服务策略 fixture 执行
`/usr/bin/true`，没有运行真正 NM，也未应用外来 profile，不能记为 NM 交接验收。

浏览器旧脚本此前覆盖了 `target/phase2-network-observe.png`；原 phase2 指纹记录的
15 项中现有 14 项保持原摘要。旧指纹不改写，截图差异明确保留；新截图写 writers 前缀。
原 takeover 指纹仍是历史版本，不能作为本轮源代码指纹。

## 环境与续接

宿主中心 HTTPS 18443、gRPC 18444，私有 PG 及凭据仍位于 `.phase1-dev`。
测试机确认 revision 2，保留 yofc-test0 与 yofc-adopt0、未授权 preserve0、原 eth0 来源
和默认管理路由。同一 VM 的修改脚本及同账号 API/浏览器登录必须串行；不重复一次性转交。
没有清理原 `target/runtime`、个人 `~/.kura-dev`，未连接 192.168.2.125。

下一轮建议另备 Ubuntu Server 24.04 LTS arm64 完整 VM，可用 Parallels；建议 4 vCPU、8 GiB、
60 GiB，一张 Shared 管理网卡、一张 Host-only 测试网卡、干净快照，中心继续运行在宿主。
该环境用于非 LXC systemd 沙箱、真实启动及后续 Linux Kura/agent/provider 联调；当前未创建、
未验收，不影响现有 OrbStack 开发。虚拟断电测试仍不能替代物理硬断电/存储故障证据。

**pending**：DHCP/resolver/IPv6 与冲突矩阵、真正 NM/cloud-init 退出和多来源协调、
enable/autoconnect、Kura provider/NetworkStatusService、旧 PID/REST/配置/snapshot 三入口事务
协调、镜像防循环、自主事件与历史维护。任意 root 写入者的并发协调尚未交付。
随后继续 nftables、系统控制、离线恢复、发行包生命周期。

用户已确认 console/agent 的注册认证独立于 Kura；一 agent 绑定一逻辑中心，其他中心普通
登记不能自动抢占，转交必须原中心授权或网关本机授权，并持久隔离旧中心控制权。
已写入总方案与 phase0 契约，正式 enrollment/审批/转交与自动轮换仍是 pending。

**deferred**：物理有线网口/拔插、硬断电/存储故障、非 LXC 系统沙箱、Linux 真实 Kura
集成、amd64/26.04 和正式包验收。环境缺失与代码未实现分别记录，不将其记为通过。
代码图谱服务已恢复，原聊天 MCP 通道仍需重新连接；现用官方 CLI，详见
`codebase-graph-repair-20261101.md`。
