# 显式快照保存与回滚

日期：2026-11-01。接在配置适用性增量之后。这不是离线恢复，也不是通用配置编辑器。

## 授权

默认关闭。agent 只有 `AGENT_KURA_SNAPSHOT=1` 才宣告 `kura.snapshot.write`；其他非 0/1 值拒绝启动。
Java 只有 `-Dkura.management.snapshot.enabled=true` 才登记 `kura.snapshot.save` 与 `kura.snapshot.rollback`。
中心要求 admin、CSRF、租户、网关在线且具备该能力。console/agent 认证仍独立于 Kura Web 账号。
另一个中心的普通注册不能凭此操作抢占归属。

操作 13 为保存，操作 14 为指定 ID 回滚。两者都是写，不重新租约。
它们与配置更新共用 agent 的 `configuration_write_lock`。任一操作 12/13/14 处于 RUNNING 或 UNKNOWN 时，其余新的配置或快照写入被拒绝。
查询只取原回执，不能凭当前快照列表相同判断原动作成功，也不能换 key 重发。

## 语义

保存和回滚都携带十进制 `expectedLatestId`；没有快照时为 `"0"`。不匹配返回 CONFLICT。
回滚必须给出更早的 `snapshotId`，调用 `ConfigurationService.rollback(long)`，不用无参 `rollback()`。
不能回滚到当前最新快照。成功回执必须包含 `networkRewound:false`。已确认系统网络由既有
`ExternalConfigurationHandler` 的 `SNAPSHOT_ROLLBACK` 路径保持，不在本操作中倒回。
快照里缺少的组件仍可能按 Kura 既有路径回到默认值。界面必须说明这一点，不能说成选择性 PID 回滚。
外部 owner PID 缺失时，核心在应用前拒绝；桥接把它标为 UNSUPPORTED，而不是成功。
部分成功或快照 ID 没有前进时为 OUTCOME_UNKNOWN。

删除、reset、factory 和配置 UNKNOWN 的持久业务核定仍未实现。
