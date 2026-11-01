# 显式快照保存/回滚验证

日期：2026-11-01。契约见 `snapshot-actions-v1.md`。IDEA 完整 Kura、原控制台登录和 OrbStack
尚未对本增量复验，不能记为通过。旧配置更新夹具未重放。

开发中心已用新 debug 二进制重新启动。migration 0009 已应用到既有开发库，operation 约束为 1–14。
`phase1-dev.py up` 的 Python 3.15 就绪检查因 CA 缺少 AKI 失败，但进程日志已 ready，curl 对 `/health/ready` 返回 200。
证书和用户未重建。宿主 agent 未因本增量打开 `AGENT_KURA_SNAPSHOT`。

## 已通过

| 范围 | 结果 |
| --- | --- |
| Java bridge `SnapshotCommandTest` 及既有 patch/view/bridge 测试 | 4 项新测试及同次 surefire 选择通过；默认关闭、ID 冲突、指定 ID 回滚、部分成功/外部 PID 缺失分类 |
| 中心 workspace `cargo test --offline` | 8 通过，2 个显式 PostgreSQL 测试 ignored，3 个 mTLS 集成 ignored |
| 协议 | 8 通过，含十进制 ID、不能回滚到当前快照 |
| agent `cargo test --offline` | 62 通过；默认关闭、回执必须 `networkRewound:false`、未知配置写入阻挡后续快照写入 |
| React 类型检查与生产构建 | 通过；chunk 约 1012 kB，原有体积提示保留 |
| 默认浏览器 `snapshot-actions.spec.ts` | 2 通过；中文范围说明、英文确认后才提交，无写入能力时面板不出现 |

## 未执行

真实 IDEA 完整 Kura、原 GWT 登录、真实快照写入/回滚和 OrbStack 仍未跑。
不能把单元回执或浏览器桩当成系统网络未被倒回的运行证明。
配置 UNKNOWN 持久业务核定已另有 `configuration-outcome-reconciliation-v1.md` 增量；删除/reset/factory 仍未实现。


## 2026-11-01 IDEA 真实运行增量

新夹具 `configuration-outcome-snapshot-idea-20261101` 完成 React 双语真实流程：两次受限写入、
显式保存、过期 latest 冲突和回滚到初始快照。原 GWT 登录在本夹具此前已通过；本轮完整 Kura 重启后
281 个 bundle 在线，inventory 282 项 SHA 全部匹配。只读观察器 initial/keep/set 各 9 项通过；
快照文件保持 0600，两次 outcome 记录 0600 且不含属性值。回滚到 snapshot 0 后重新读取当前配置，
count 回到 `1`、enabled 为 false。最终回执 `506c9948-940e-423e-8b0e-c2c0e5076c2f`
为 SUCCEEDED 且 `networkRewound:false`。

本轮发现并修复两处真实缺陷：中心协议与 Java bridge 原先把 snapshot `0` 当作非法回滚目标；
旧 probe 只识别上一个夹具的 ownership marker，在新夹具启动即失败。失败证据保留为
`observer-error.pre-outcome-marker-fix.json`，旧 jar/bridge 以 `.pre-*-fix` 后缀保留。
修复后协议 8 项、agent 65 项、console workspace 9+8 通过；Java bridge 33 项通过；
React 默认浏览器 26 项和生产构建通过。chunk 约 1013 kB 提示保留。

由于测试辅助代码曾失败重试，夹具内还保留多次成功 save 和 9 次 CONFLICT、2 次 INVALID_ARGUMENT
任务记录；这些是证据的一部分，不删除也不算成一次理想流程。最终成功 save/rollback 如上。
当时 OrbStack 尚未复验本增量；下方同日晚间增量已复验。


## 2026-11-01 OrbStack 真实运行增量

复用 `yofc-management-network-20261101`，新一次性目录
`/opt/yofc-outcome-snapshot-20261101`。Linux aarch64 agent 以 1.95 release 重建，SHA
`7054c35beba2068bfec6a5c563dbf1ef4ca8b38eb96b9269b3cd662d3e45f72f`；282 项 inventory 全匹配；
8 项旧 fixture/netplan 基线不变。新 agent `NoNewPrivs:1/CapEff:0`，Kura UID 992 且 `CapEff:0`。
既有 OrbStack 全局 service 覆盖仍使 Kura `NoNewPrivs:0`，完整 systemd 沙箱继续 deferred。

真实流程：两次受限写入成功，initial/keep/set 观察器各 9 项通过；2 条 outcome 记录 0600 且无属性值。
显式 save 成功；expected latest `0` 的 stale 任务按终态 `CONFLICT` 拒绝；随后回滚 snapshot `0` 成功，
回执 `networkRewound:false`。回滚后重新读取当前配置，count 回到 `1`、enabled 为 false。
关键回执：save `744bde25-34c0-4d1d-bb70-9536247fcda8`，rollback
`efe96f8f-99d5-4449-9458-f48bbf254438`。完成后两个新 unit 停止、gateway 已禁用，
状态 `stopped-data-retained`；旧 network fixture agent 仍 active，数据未清理。
start 安全属性以 retroactive observation 记录：原 start.json 因当时 status 序列化 bug 未写出，
initial proof 与服务 PID/安全属性在失败前已经实际观察到。
