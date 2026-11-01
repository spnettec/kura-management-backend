# 原 Kura 网络事务 REST v1

2026-11-01。这些端点运行在 Kura 的原 REST 服务中。中心 HTTPS API、中心用户和 agent
mTLS 身份维持各自认证边界；注册认证独立于 Kura Web 账号。

## 路径与权限

基础路径为 `/services/networkConfiguration/v1/transactions`，仍受原 REST 服务的
认证方式、允许端口及 `network.configuration` 权限控制。浏览器调用还需遵循原服务的
会话和跨站请求防护。新增端点不会授予 Kura 进程新的系统写权限：独立 agent 的注册角色与
`AGENT_KURA_NETWORK_INTERFACES` 委托仍决定能否写指定接口。

| 方法 | 相对路径 | 含义 |
| --- | --- | --- |
| GET | `/view` | 确认配置、观测接口、revision、sourceDigest 和当前事务 |
| POST | 空路径 | 提交候选，等待明确确认 |
| GET | `/{transactionId}` | 查询 Kura actor 的事务，不需要确认令牌 |
| POST | `/{transactionId}/confirm` | 明确确认候选 |
| POST | `/{transactionId}/rollback` | 明确回退候选 |

原配置端点保持原行为。修改旧网络字段仍拒绝，不能通过通用配置写入绕过事务。
公共服务未注册或动态解绑时返回 503，不回退到原 NetworkManager writer。

## 请求与结果

POST 使用 `Content-Type: application/json`。prepare 的外层只能包含 `requestId` 和
`change`；`change` 使用共享网络契约的 `expectedRevision`、`sourceDigest`、
`confirmSeconds`、`config`。必须使用编辑时取得的 revision 和摘要；不能在提交时刷新
revision 后套用旧表单。`config` 使用 agent 的严格字段与组合校验。

```json
{
  "requestId": "example-request-1",
  "change": {
    "expectedRevision": 3,
    "sourceDigest": "<digest-from-edit-baseline>",
    "confirmSeconds": 60,
    "config": {
      "interface": "eth0",
      "dhcp4": true,
      "addresses": [],
      "routes": [],
      "nameservers": [],
      "searchDomains": [],
      "mtu": 1500,
      "optional": false,
      "dhcpMetric": 100
    }
  }
}
```

`requestId` 为 1–64 位 ASCII 字母、数字、`_ . : -`，相同 ID 仅可用于完全相同的输入。
`transactionId` 为 1–128 位 ASCII 字母、数字、`_ -`。
confirm/rollback 的 body 只能是 `{"confirmationToken":"<token>"}`；非空字符串至多 256 字符，
不能含控制字符。令牌不得出现在 URL、日志、审计或快照中。

prepare 返回 `state=WAIT_CONFIRM` 时为 **202**，表示候选正在等待确认。其他成功结果为
200。用户需要通过受影响路径重新连接并明确确认；HTTP 超时不代表取消，不应自动重发写操作。
查询事务以确定持久状态。事务查询和令牌的 actor 归属由 agent 校验。

Java 公共边界使用 detached Map/List、字符串、布尔、整数和 null，无 provider 内部类型。
REST 请求先保留 JSON 数字，再以 BigDecimal 转换，provider 只接受整数，避免 revision
经过 double 丢精度。原 REST 序列化器会省略值为 null 的 Map 字段；例如没有事务时
`transaction` 可缺省，客户端按无事务处理。

资源返回的成功响应和类型化错误均带 `Cache-Control: no-store`、`Pragma: no-cache`。
原认证过滤器及 JSON 解析器生成的响应继续沿用原服务行为。

## 错误

资源错误 body 为 `{"error":{"code":"<stable-code>","requestId":"<safe-id>"}}`，
不返回异常详情或请求令牌。

| code | HTTP |
| --- | ---: |
| INVALID_ARGUMENT | 400 |
| FORBIDDEN | 403 |
| NOT_FOUND | 404 |
| CONFLICT、RECOVERY_REQUIRED | 409 |
| UNSUPPORTED | 422 |
| RESOURCE_EXHAUSTED | 429 |
| PROTOCOL_MISMATCH | 502 |
| AGENT_UNAVAILABLE 或未公开的内部错误 | 503 |
| TIMEOUT | 504 |

## 当前验证边界

Java provider 15 项、REST 19 项通过。真实隔离 Kura 验证原身份、匿名拒绝、view、
无委托写入拒绝、严格输入、未知事务、令牌要求与无状态变更；HTTP 共 21 项通过。
202 和成功确认/回退在本增量属于单元/契约证据，未开放宿主接口委托执行实际候选。
Linux 实际 Java provider、真实非管理员角色、GWT 事务交互、旧字段无损映射仍待验证或实施。
