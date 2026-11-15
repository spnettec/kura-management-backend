# 中心公开证书登记（2026-11-05）

用户恢复实施后，补齐运维签发证书在中心的管理员登记入口。中心尚不托管 CA，也不替网关生成私钥。Kura 生成私钥/CSR、运维签发公开叶证书、中心登记、Kura 交付身份、最后本机明确开通远程通道分别执行。首次交付和登记均不会自动开启 agent 通道。

`POST /api/v1/gateways` 接受 `id`、`name`、`clientCertificatePem`；拒绝额外字段。使用现有管理员会话、Origin/CSRF、首次改密后台闸门。租户及操作者只从服务端会话取得。ID 限 ASCII 字母数字/下划线/横线、最多128字节，名称非空、无控制字符、最多256 UTF-8字节。只接受一张规范公开 PEM/X.509 DER，最多32KiB；不接受链、私钥、垃圾或多张证书。

登记只保存 DER SHA256，不保存证书正文，不接受 generation 或 enabled 输入；generation 固定1。按现有模型中心记录 enabled=true，仅代表中心允许该证书连接，不代表 agent 远程决定 enabled。认证时仍须通过中心配置的客户端 CA、TLS 证书有效期/用途及精确证书指纹/网关ID/generation核对。登记本身只检查格式，不声称已验证发行链、在线撤销、服务器可达或设备归属。

数据库只 INSERT，不 UPDATE/UPSERT。ID 和指纹全局唯一冲突返回409；不修改已有停用记录、代次或归属。新增记录、真实会话UUID审计和公开gateway事件同一事务提交，任一失败整体回滚。原register-gateway CLI复用相同校验/事务，操作者为local-admin。另一中心普通登记不向agent发送任何身份/转移/开关请求。

公开网关读取增加certificateSha256用于核定登记结果；不包含证书或私钥。React管理员双语对话框展示公开指纹、明确确认；改动材料使确认失效。发送后失败/未知或回执矛盾，冻结材料及提交，须显式读取该ID。ID/名称/指纹/generation都匹配才核定；404只解除冻结，仍需重新确认，不自动重发；已有不同记录持续冲突。新会话/用户切换不共享表单。成功可显式登记另一台网关。

宿主验证已完成：Rust中心17/协议8（显式环境测试另计）、实际PostgreSQL HTTP合同1、严格Clippy/fmt、React构建/28项适用浏览器及真实HTTPS登记1。真实HTTPS场景在服务器201提交后丢弃浏览器响应，只发一次POST，显式读取找回相同指纹，中英回执通过；数据库审计核定为registration-admin的实际会话UUID。默认admin/admin首次改密闸门保留，通过HTTP合同核对。

完整IDEA隔离282项Kura原admin/admin中文登录、无agent隐藏远程菜单、快照页通过并已停止。本轮未改Java/GWT源码或重放密钥生成/首次绑定。Orb中心编译及现有已绑定身份远程闭环进行中；Parallels对应新范围尚未执行。以target/resume-center-auth-20261105下最新机器记录为准，不把待执行记成通过。

## 2026-11-05 恢复后的中心登记与通道验证

用户已恢复继续，整体active。最新实现、精确验证与运行状态见 [中心登记与mTLS闭环](reviews/gateway-registration-channel-20261105.md) 及 `evidence/gateway-registration-channel-20261105.json`。中心新增管理员双语公开叶证书登记；修复Java/SunEC最小PKCS8与ring兼容；原GWT开启→真实中心/Kura只读任务→关闭/重启保持OFF通过。36项既有身份/源材料不改。Parallels仅native严格沙箱/mTLS，无Java。新夹具全停/VM均stopped，原Orb26/20服务保持active。托管CA、身份运维和完整原业务/发行仍pending，不把此次通道验证计作Kura业务恢复。
