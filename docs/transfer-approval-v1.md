# 独立归属转移审批 v1

日期：2026-11-02。接续 `management-redesign-handoff-20261102.md` 的 signed transfer E2E。
本契约把“能签发 token”变成中心内的权威流程：签名方、请求、批准、签发和撤销都有独立记录。
普通另一中心登记仍不得抢占网关归属；本流程只服务显式跨中心交接。

## 数据与角色

`mgmt_transfer_signers` 保存租户内 Ed25519 SPKI 的 SHA256 与 `ACTIVE/REVOKED` 状态；
`key_id` 不能复用为另一个 key、租户或重新启用已撤销值。
`mgmt_transfer_approvals` 保存精确的旧/新 generation、gateway、endpoint 和新 CA/client hash，
并强制同一网关最多一个 `PENDING/APPROVED` 请求。
请求人和批准人必须是同一租户、enabled 的 admin，且批准人不能等于请求人。
`mgmt_transfer_issuances` 以 `approval_id` 唯一约束签发；签发事务把 approval 标为 `ISSUED`。
因此 token 交付失败也不会再次签发同一 approval，必须重新请求和批准。

## 命令序列

1. 生成 key：`generate-transfer-key PRIVATE_KEY_DER PUBLIC_KEY_PEM`。
2. 登记签名方：`register-transfer-signer KEY_ID PUBLIC_KEY_PEM TENANT ACTOR_UUID`。
3. 请求：`request-gateway-transfer-approval GATEWAY_ID EXPECTED_GENERATION NEW_ENDPOINT \
   NEW_GATEWAY_ID NEW_CA_PEM NEW_CERT_PEM REQUESTER_UUID`。
4. 另一 admin 批准：`approve-gateway-transfer-approval APPROVAL_ID APPROVER_UUID`。
5. 签发：`issue-gateway-transfer APPROVAL_ID OPERATOR_UUID GATEWAY_ID EXPECTED_GENERATION \
   NEW_ENDPOINT NEW_GATEWAY_ID NEW_CA_PEM NEW_CERT_PEM EXPIRES_IN_SECONDS SIGNER_KEY_ID \
   PRIVATE_KEY_DER PUBLIC_KEY_PEM TOKEN_FILE`。
   该命令要求 operator、签名方、gateway generation、approval 声明完全一致。
6. 拒绝或撤销 open approval：`reject-gateway-transfer-approval` /
   `revoke-gateway-transfer-approval APPROVAL_ID ACTOR_UUID`。
7. 撤销签名方：`revoke-transfer-signer KEY_ID ACTOR_UUID`；只阻止后续签发，
   不伪造已发出 token 的过期时间。

## HTTP 与双语界面

管理员可通过同源 HTTPS/CSRF 接口查询和登记 `ACTIVE/REVOKED` 签名方；同一 key、租户和公钥可幂等重放，
revoked key 或不同公钥不能复用 key ID。`POST /api/v1/gateways/{id}/transfer-approval` 使用当前
enabled gateway 的 generation 记录精确请求；同一网关只允许一个 `PENDING/APPROVED` 请求。
批准要求另一个 enabled tenant admin；viewer 只能被拒绝，operator 不能操作。
React admin 网关抽屉提供中英文签名方状态、请求确认、批准/拒绝/撤销界面；它不显示、保存或签发 token。

## 边界

- token 仍是不超过 900 秒的一次性材料；agent 精确 claim 校验继续生效。
- agent 在改身份前把 transferId、原始 token SHA256、签名方 SHA256 和精确 claims 落盘到
  `transfer-token-uses/`；中断后同 token 不重试，必须重新申请/批准。
- 已用临时 PostgreSQL authority 和 agent staged Setup root 完成 approval → token → agent ledger/generation E2E；installed package/grpc heartbeat E2E、生产 systemd center2、物理主机和断网转移仍 pending。
- 未执行环境继续 deferred；不把本地单元/迁移检查记作生产发行验收。

## 已签发 token 的撤销（2026-11-02 补充）

签发事务不回滚、approval 保持 `ISSUED`。已签发但未被消费的 token 可由启用租户 admin 用
`revoke-transfer-issuance TRANSFER_ID ACTOR` 撤销：`mgmt_transfer_issuances.revoked_at/revoked_by`
（迁移 `0014_transfer_revocation.sql`）只允许一次，重复撤销或撤销他人租户的 issuance 都拒绝；
已消费的 token 不受影响，撤销也不重新打开 approval。

消费前检查：agent 端 `transfer-identity` 在签名校验通过后、消耗授权或修改任何身份字节之前，
用当前身份经新 gRPC `CheckTransferRevocation`（mTLS 网关证书识别，一次性 CLI
`yofc-management-agent check-transfer-revocation --transfer-id UUID`）向原中心查询该 issuance。
只有精确属于本网关、已签发且未撤销的 token 才回答 `ACTIVE`；撤销为 `REVOKED`，未知 id 为
`NOT_FOUND`；任何非 `ACTIVE`、网络失败或超时都拒绝转移且不改变任何状态。每次查询写
`transfer.revocation.check` 审计。

## 撤销的 HTTP 与双语界面（2026-11-02 补充）

`GET /api/v1/transfer/issuances` 供 admin 查看本租户已签发授权（含签发人、签名方、目标与
`ACTIVE/REVOKED` 状态；GET 只读、无 CSRF 要求）。`POST /api/v1/transfer/issuances/{id}/revoke`
要求 admin + 同源 + CSRF：未登录 401、viewer 403、缺 CSRF/Origin 403、未知或他租户 id 404、
重复撤销 409；成功写 `transfer.revocation` 审计。浏览器与 API 都不显示、保存或签发 token 字节。
React 转移面板新增"已签发转移授权"中英文表格：仅 ACTIVE 行可撤销，撤销后按钮消失并显示
撤销人与撤销说明（撤销只阻止未消费 token，不重开审批）。本地 mock 契约、临时真实实例 HTTP
契约与 live 浏览器撤销/双语截图均通过；installed 正式包携带该面板待下一 candidate。
