# 配置元数据约束与组件适用性

日期：2026-11-01。接在本地保存点之后，尚未提交。保存点只覆盖此前已完成的受限远程更新，
不包含本增量。IDEA 完整 Kura、原控制台登录和 OrbStack 尚未对本增量复验，不能记为通过。

## 行为

配置包导出 1.5.0。`ConfigurationConstraints` 对已审核 PID 执行可解释的 min/max/options：
数值和字符为闭区间，字符串与密码按 UTF-16 码元长度。无法解释的约束、超过 64 项选项、
密码选项列表保持关闭，不静默忽略。默认仍关闭，审核 PID 才开放。

`SelfConfiguringComponent` 的条件更新在 provider monitor 内把当前自有视图与补丁合并后再提交，
避免未勾选字段被元数据默认值覆盖。观察不到提交值时仍不发新 token。
`directConfigAdminCas` 固定为 false；直接 ConfigurationAdmin 写入不参与该 monitor，不承诺原子 CAS。
这仍不是通用配置编辑器。删除、reset、factory 仍未实现。后续配置 UNKNOWN 核定见 `configuration-outcome-reconciliation-v1.md`。
同日后续的显式快照保存/回滚见 `snapshot-actions-v1.md`；本文件不证明该增量。

## 已核验证据

核心 `ConfigurationConstraintsTest`、`ConfigurationRevisionTest`、`ConfigurationRevisionAdmissionTest`
共 12 项通过。bridge `ConfigurationPatchTest`、`ConfigurationViewTest`、`ManagementBridgeTest`
共 16 项通过。React 类型检查、生产构建和默认浏览器 `configuration-update.spec.ts` 6 项通过。
宿主开发中心原先未监听 18443，已用既有 `.phase1-dev` 重新启动；证书和用户未重建，
配置更新夹具未重放。约 1008 kB chunk 提示保留。
