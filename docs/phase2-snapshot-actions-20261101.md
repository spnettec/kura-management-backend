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
