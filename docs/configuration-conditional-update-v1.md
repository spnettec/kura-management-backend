# Kura 配置条件更新基础契约

2026-11-01 后续：受限远程更新已接通，当前授权、typed 协议和 UNKNOWN 规则见
`configuration-remote-update-v1.md`。下文“尚未接线/不登记”保留基础增量时的历史状态；
核心 API、直接 ConfigAdmin 的并发边界以及未完成的通用元数据约束继续有效。

日期：2026-11-01。此增量交付核心 API 和 Java 桥接处理基础，不表示远程配置写入闭环完成。
Protobuf/agent、中心写授权与任务分发、React 编辑器尚未接线；Java 仍只登记三个读取方法。
新增界面需按 `console-i18n-20261101.md` 同时提供中英文。

## 核心 API

`org.eclipse.kura.configuration.ConfigurationRevisionService` 与原 ConfigurationService 由同一 DS 实例提供。
`readRevision(pid)` 返回当前配置及 opaque revision；`updateRevision(pid, expectedRevision, properties)`
在同一个 provider monitor 内核对 revision，再走原配置更新与快照保存路径。
原 ConfigurationService 的接口签名保持，新增独立接口；配置包导出 1.4.0，核心和 bridge 要求最低 1.4。
container provider 的配置包范围改为 `[1.2,2.0)`，兼容这次附加 API；需同步更新制品，旧 jar 的上限 1.4 会拒绝解析。

- revision 为随机实例密钥的 HMAC-SHA256，64 个小写十六进制字符；包含 PID、完整属性（含秘密）、
  AD 类型/cardinality/required/default/min/max/options 值、CM change count 和条件写准入计数。
  属性顺序不改变 token；重启失效。它不是身份凭据、可迁移内容 hash 或秘密值的裸 hash。
- 同一旧 revision 的并发写入只有一个可准入；原 ConfigurationService 写入导致的 CM ABA 也使旧 token 失效。
  **直接 ConfigAdmin 写者不参与此 monitor**。其 change count 仅在准入时核对，不承诺对这些写者提供原子 CAS。
- 仅 set patch，1–64 个字段。遗漏字段与密码保持；显式 Password 表示设置密码。null、删除字段、
  reset 和 factory 创建均不隐含。系统字段、未知字段、类型/cardinality 不符在调用 writer 前拒绝。
- 返回表示普通配置提交与快照策略完成，不证明异步组件健康。SelfConfiguringComponent 仍报告旧视图时，
  暂不发新 token；只有提交值可被观测到时才恢复条件写。
- `Rejected` 的 INVALID_ARGUMENT / NOT_FOUND / CONFLICT / UNSUPPORTED 只代表写入前拒绝。
  writer 或快照阶段其他异常可能发生在实际写入之后，调用方不得自动重放。
  这时 PID 被内存 uncertain guard 挡住，不因值看起来正确就发新 token；明确成功的旧入口更新、rollback
  或 provider 重启后解除。此保护不是持久化业务结果核定，未来 agent 仍须保留 UNKNOWN 和跨进程执行日志。
- Netplan 等外部 owner 的 PID 不允许通过本 API 绕过权威事务。

canonical 编码严格标注 Java 类型，不调用未知对象 toString；限制 512 属性、数组/Vector 256 项、
单文本 16 KiB、总输入 256 KiB，防御复制可变密码与容器，拒绝循环/嵌套容器和非有限浮点数。
Read.toString 和新增审计均不输出配置值。

本基础额外检查类型/cardinality，其他校验继续依赖原核心路径；SelfConfiguringComponent 与字符串等的
完整元数据约束、组件适用性仍要完善，不能据此开放通用远程编辑器。

## Java 桥接处理

`kura.configuration.get` 可附加 `revision` 与 `writable`。缺少新服务、外部 owner 或等待生效时保持只读投影。
这里 writable 表示核心条件更新能力；仍须同时满足未来 agent 能力、注册方法和中心权限，不能据此单独授权。
桥接继续使用 OS 身份，与 Kura Web 用户、密码和角色无关。

桥接内部新增 `kura.configuration.update` 处理，但当前 **不登记/advertise** 此方法，旧 agent 白名单保持。
测试和未来新版 agent 的本机参数形状为：

```json
{
  "pid": "example.component",
  "expectedRevision": "64 lowercase hex characters",
  "expiresAtUnixMs": 1790000000000,
  "properties": {"count": "9223372036854775807", "enabled": true}
}
```

仅接受四个顶层键，整体 16 KiB。数值必须 JSON 字符串，bool 必须 JSON boolean，数组元素遵循 AD 类型；
正 cardinality 转类型数组，负 cardinality 转 Vector。拒绝 null、未知字段、嵌套、错误类型、溢出和 NaN/Infinity。
先读 revision/元数据，转换后再次核对 deadline，核心最后再次条件准入。
成功只返回 PID、submitted=true、previousRevision，不回显属性。
明确 Rejected 使用固定码，过期为 TIMEOUT，其他异常为 OUTCOME_UNKNOWN，隐藏内部异常细节。
UDS 发送后超时/断开、任务去重、敏感参数加密和 UNKNOWN 阻挡须在后续 typed agent/中心接线时落实，
不能沿用当前只读 call_with_params 的失败映射。

真实验证、失败记录与剩余项见 `phase2-configuration-revision-validation-20261101.md`。
