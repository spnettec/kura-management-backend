# 受限远程配置更新契约

日期：2026-11-01。接续 `configuration-conditional-update-v1.md`；该文档的核心 API 语义继续有效，
其中“远程尚未接线”已由本增量替代。此功能仅面向明确审核的组件，不是通用配置编辑器。

## 授权与方法

console 登录会话、租户和 admin 权限独立于 Kura Web 账号。所有写 POST 要求同源与 CSRF。
agent 以预登记的证书和 mTLS 主动连中心；普通注册不代表可以抢占另一个中心的所有权。
agent 与 Java 通过 UDS 和操作系统身份验证，不传递 Kura Web 密码。

远程操作为 `kura.configuration.update`，Protobuf Operation=12，ReadCommand 新字段 9。
`ReadCommand` 是历史消息名；此操作始终分类为写，绝不走只读失败映射或重新租约执行。
仅在 `AGENT_KURA_CONFIGURATION_UPDATE=1` 时 agent 宣告能力；默认 0，其他值启动失败。
Java 另需 `-Dkura.management.configuration.update.pids=<审核后的逗号分隔PID>`；默认空。
只有非空 allowlist 才登记写方法。两侧开关互不替代，旧读取方法仍必需，写方法注册可选。

get 中 `writable` 表示核心条件更新可用；`updateEnabled` 表示 PID 已显式开放，属性 `editable`
表示该字段符合本轮受限规则。React 还检查 admin、online、agent capability 和有效 revision。
仅声明过类型的非系统字段可编辑；带 min/max/options 的字段暂不开放。
必填 string/password、数组和 Vector 不能设为空。完整元数据约束与 SelfConfiguringComponent
适配仍待实施；不得通过扩大 allowlist 绕过审核。

## HTTP 输入

`POST /api/v1/gateways/{id}/tasks`，每个逻辑动作生成一个 idempotencyKey：

```json
{
  "operation": "kura.configuration.update",
  "idempotencyKey": "one-logical-submission",
  "kuraConfigurationUpdate": {
    "pid": "reviewed.component",
    "expectedRevision": "64 lowercase hexadecimal characters",
    "properties": [
      {"id": "count", "value": {"text": "9223372036854775807"}},
      {"id": "enabled", "value": {"boolean": false}}
    ]
  }
}
```

属性 value 为 typed oneof：text、boolean、textArray（`{"values":["a"]}`）、
booleanArray（`{"values":[true]}`）。数值以字符串发送，避免浏览器 Long 精度损失。
密码只有显式勾选并输入新值才提交；遗漏表示保持，不用空值或遮罩代替密码。
禁止重复字段、null、隐含 reset/delete/factory。1–64 字段、数组最多256项、单文本16KiB；
最终 Java UDS params 也不得超过16KiB。agent 添加可信期限，Java 转换后再检查期限，核心最终检查 revision。

成功结果仅含 `pid`、`submitted:true`、`previousRevision`，表示配置提交与原快照策略完成；
不证明异步组件健康。输入及结果沿用中心密文存储、agent 密文执行日志；回执不回显属性值。

## 重放与结果不确定

中心在准入时检查租户、enabled、近期在线、能力与权限。相同 key 返回原任务；不同内容同 key 拒绝。
操作12不重新租约；agent 串行锁覆盖持久 admission、UDS 调用和持久 finish。
已知未交付的拒绝可确定失败；进入发送队列之后的断流/超时/不合法成功响应均为 UNKNOWN。
agent 重启后仍以持久的 RUNNING/UNKNOWN 阻挡新配置写入，不能靠重启 Java 清除。
中心同样阻挡未决写入。查询 `/api/v1/tasks/{id}` 只取原回执，不能用新任务或重新读取当前值冒充结果核定。
配置 UNKNOWN 的持久业务结果核定尚未实现；不能套用网络事务的特定业务证明。

React 使用中英文确认界面，只提交用户选择字段；提交后清空草稿、禁用旧视图修改，要求重新读取。
UNKNOWN 或 HTTP 准入结果不明时不自动重试，显示任务查询/阻挡状态。切换语言保留未提交草稿。

实际验证和失败边界见 `phase2-configuration-update-validation-20261101.md`。
