# 阶段 2 单一网络来源接管实施证据

日期：2026-11-01，Asia/Shanghai。整体改造继续进行。本轮已实现并实际验证单一明确
Netplan/networkd 来源的写入权转交；Kura 旧网络 provider 尚未替换，阶段 2 全部范围和正式发行均未完成。
此前 consolidation、阶段 1 和首次阶段 2 结果保留为各自时间点快照，不重做目录迁移，不提交/推送或修改远端。

## 本轮代码与事务约束

- 共享协议增加 `NetworkTakeover`、`network.takeover.plan/prepare`（操作 9/10），中心增加
  migration 0004、类型化 HTTP/OpenAPI 与 gRPC 分发、管理员写入授权和内容幂等比较。
  旧回执的缺省可选输入与 null 兼容，其他输入变化仍冲突。
- agent `src/network/takeover.rs` 实现计划、来源摘要、原字节/权限/组、服务快照、归一内容和
  原接口配置备份。接口名单仍是必要条件；仅允许单一明确 `/etc/netplan` 来源，不自动接管 NM/cloud-init。
- prepare 只将目标接口从原来源的 `/run` 同名覆盖中移除，再放入自有 runtime 候选；原 `/etc`
  文件不变。未确认重启使用原持久来源，回滚移除覆盖。确认要求新的 mTLS epoch。
- 确认意图持久后先写自有持久候选，再归一原来源并保留其他接口、模式/组，最后移除运行覆盖。
  中间两份定义保存相同接口值，实际 staging generate 也校验此窗口。确认后重启补完提交；
  来源、权限或服务漂移拒绝覆盖。四个新增测试覆盖这些恢复边界，详细契约见 `phase2-network-contract.md`。
- React 展示来源、原权限、原接口字段和其他接口；必须勾选已核对才允许应用接管候选。
  接管不改变原地址值，后续普通 IPv4 修改继续保留 IPv6 字段/地址/路由。
- fixture 脚本允许从 revision 2 续接、保留两个明确授权接口；installer 保留已有 fixture 授权，
  不回写成单接口。新增 `--evidence-prefix` 将本次回归结果与旧验收 JSON 分开保存。

## 已执行验证

| 检查 | 结果 | 证据 |
| --- | --- | --- |
| 中心/协议单元、Clippy | 4 通过，3 个专用 mTLS fixture 测试本轮默认运行中 ignored；Clippy all-targets 通过 | 本轮构建输出；实际新 epoch 另见 Linux 接管 |
| agent macOS/Linux 单元、Clippy | 两平台各 17 通过（增加 4 个 takeover 测试）；all-targets `-D warnings` 通过 | Linux 持久日志 `target/takeover-agent-linux.log`；macOS 本轮工具输出 |
| React 类型/生产构建 | 通过；既有大 chunk 提示仍在 | 本轮 npm build 输出 |
| 实际 HTTPS/gRPC 契约 | 12 项通过 | `target/takeover-contract.json`；宿主无授权来源，不执行系统写入 |
| UI 审阅契约 | 1 通过，0 unexpected/flaky | `target/takeover-ui-browser.json`；任务结果 stub，不充作 Linux 验收 |
| 浏览器真实来源转交与回滚 | 1 通过，0 unexpected/flaky | `target/takeover-live-browser.json`、`takeover-live-review.png`、`takeover-live-candidate.png`；没有路由 stub |
| IDEA 完整隔离 Kura | 原控制台 admin 登录成功；Rust 通道查询 24 个 PID；验后进程停止已核实 | `target/takeover-kura-live.json`，隔离启动 PID 75016 已退出 |
| Ubuntu 真实来源转交 | 18 项通过 | `target/takeover-runtime.json`，真实 Netplan/networkd、两次 VM 重启 |
| 普通网络回归 | 17 项通过，baseline revision 2 | `target/takeover-network-regression.json`；超时、agent/VM 重启后保留接管接口的全局地址、MTU、路由和归一来源 |
| 首次空接口补偿回归 | 24 项通过 | `target/takeover-initial-link-regression.json`；临时接口验后删除且恢复两个原授权接口 |
| 已接管接口普通修改与回滚 | 7 项通过 | `target/takeover-adopted-link-regression.json`；IPv4/MTU 实际变更、IPv6 保留、持久来源不变、完整运行值回退 |
| fixture installer 再安装与最终回读 | 通过；两个授权接口、revision 2、无候选/临时接口；Linux build/install 二进制摘要相同 | `target/takeover-final-state.json` |

Ubuntu fixture 为 `yofc-management-network-20261101`，24.04.5 arm64。专用 dummy `yofc-adopt0`
原配置含 IPv4 192.0.2.90/24、全局 IPv6 2001:db8::90/64、MTU 1400、静态路由、DNS/搜索域，
原文件为 `/etc/netplan/40-yofc-existing-fixture.yaml`、0640，并包含未授权的 `yofc-preserve0`。
18 项实际检查证明：过期 plan 在写前拒绝；待确认时原持久字节不变；显式回滚恢复原所有权；
未确认 VM 重启恢复原来源和 revision 1；新 mTLS 流确认至 revision 2；源文件归一后仍保留另一
接口和 0640；确认后再次重启保留全局 IPv4/IPv6。原 eth0 来源及默认管理路由未改，Kura 不在 VM 运行。

截图已查看，真实候选页显示 WAIT_CONFIRM、倒计时和确认/立即回滚按钮。审阅截图的全页 capture
在抽屉内按可视窗口记录，完整字段及勾选行为另由 locator 断言；没有把截图当作内核回读。

后续回归曾有两次测试同时修改 fixture；该两次结果作废，恢复原授权后按顺序重跑。
另一回归首次比较了 dummy 重启后按新 MAC 自动生成的链路本地 IPv6，产生预期外差异；
已改为比较受配置管理的全局地址/前缀、MTU、路由和源文件，未改生产代码或伪造运行结果。
首次空接口脚本原先使用的手工路由与新增 preserve 接口的直连网段相同，内核返回 File exists；
已将该测试路由改到独立的文档测试网段，串行复跑 24 项通过。

## 复跑与现场

所有 API/浏览器登录及修改同一 VM 的脚本均须串行；同账号登录会替换旧会话，且有限速。
`phase2-takeover-smoke.py` 的完整验收是一次性所有权转交，现已 revision 2，不能重新初始化
或再次运行来冒充首次接管。保留原证据和现场。

可复跑普通回归（从 console 仓库执行；network smoke 会正常重启专用 VM）：

```bash
python3 scripts/phase2-network-smoke.py yofc-management-network-20261101 --evidence-prefix takeover-network-regression
python3 scripts/phase2-initial-link-smoke.py yofc-management-network-20261101 --evidence-prefix takeover-initial-link-regression
python3 scripts/phase2-adopted-link-smoke.py yofc-management-network-20261101
```

宿主中心 HTTPS 18443、gRPC 18444 和私有 PG 沿用原 fixture。专用 VM 服务、证书和状态在原
`yofc-management-phase2` 路径。确认状态为 revision 2，yofc-test0 192.0.2.2/24，yofc-adopt0
192.0.2.90/24 和 2001:db8::90/64。原 `target/runtime` 与个人 `~/.kura-dev` 未清理。

## 尚未交付与验证边界

**pending**：完整 NM/cloud-init 退出和多来源接管、全部 `/run/systemd/network`/发行版写入者
归属、DHCP/resolver/冲突运行矩阵、enable/autoconnect 映射、Kura provider/NetworkStatusService、
旧 PID/REST/通用配置/snapshot 三入口事务协调、历史维护和自主回滚事件。转交保留 DNS YAML
不等于 resolver 行为已验收；本轮未实现停止/恢复 NM 等服务。

阶段 1 注册/自动证书轮换、六项业务桥接/敏感字段、细粒度目标授权、完整审计/容量和正式协议
制品仍开放。随后继续 nftables、系统控制、离线恢复、包生命周期及发布。

**deferred**：物理有线硬件、硬断电/存储故障、非 LXC systemd 沙箱、Linux 真实 Kura provider
集成、amd64/26.04 与正式包生命周期。正常 VM 重启不能代替硬断电。
未连接 192.168.2.125，旧 Java/Vue 和两个旧远端保留。

新指纹见 `docs/evidence/phase2-takeover-20261101.json`；历史 phase1/phase2 指纹未覆盖成本轮代码。
历史 phase2 的 15 项结果文件摘要另已复核，全部保持原值；核心 kura 工作区仍干净。
