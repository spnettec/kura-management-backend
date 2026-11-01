# 中心任务输入与结果存储 v1

2026-11-01。适用 Rust console 的 `mgmt_tasks.input/result`。业务标识、租户、网关、操作码、
幂等键、状态、错误码和时间仍是可查询元数据；HTTP/Protobuf 结构不变。

## 密钥与格式

- `CONSOLE_DATA_KEY_FILE` 指向中心服务用户拥有的独立 32 字节随机密钥文件。
  文件必须为 0600、单硬链接、普通文件；直接父目录必须私有且归当前 UID 所有。
  拒绝相对路径、符号链接和不安全权限。密钥内容不保存在 PostgreSQL、环境变量或日志中。
- 使用 AES-256-GCM，每次加密产生新的 12 字节 nonce。JSON 信封只含 `$yofcTask:1`、
  `nonce` 和 `ciphertext`。AEAD 绑定格式版本、任务 UUID、tenant、gateway、operation 和
  input/result 字段，拒绝跨任务、跨租户、跨操作和输入/结果之间的密文调换。
- 最大明文为 1 MiB；既有 HTTP/协议限制继续在入口生效。解密只接纳严格版本的信封，
  没有运行时明文回退。空 SQL result 保持为空；JSON null 可作为加密后的业务结果。

## 初始化与升级

先停止旧中心，保留数据库备份，然后以中心服务用户创建私有状态目录。在首次部署中显式执行：

```sh
yofc-management-console task-key-init /var/lib/kura-management-console/task-data.key
```

设置 `CONSOLE_DATA_KEY_FILE` 和 `DATABASE_URL` 后执行 `migrate`，再启动 `serve`。
`task-key-init` 不访问数据库，也不会覆盖已有文件。其他 CLI 命令也会核对原数据密钥。
生产目录可由 unit 的 StateDirectory 或安装流程预建；不要以 root 生成后直接交给另一个 UID 读取。

首次有密钥的启动在一个数据库事务内排他锁定任务表，每页最多 100 行读取并加密旧 input/result，
不改变任务状态、有效期、执行租约、幂等键、事件或审计。成功后安装拒绝明文写入的数据库约束，
移除旧明文 input 默认值，并保存用同一密钥认证的 key-check。锁等待上限 10 秒。
中途失败会回滚整个数据转换。发现无 key-check 的加密数据时停止，要求恢复原存储元数据。

后续启动只认证 key-check，不重新加密历史记录。旧二进制不能继续向升级后的表写入明文。
版本回退需要兼容的二进制或协调恢复升级前备份，不能把旧进程直接接回新表。
开发脚本只在数据库尚未绑定密钥时创建首次 key；已绑定但缺 key 时停止。

## 故障与保留范围

缺少或错误密钥时服务不会启动监听；不生成替代密钥。恢复时必须使用与数据库一致的原 key，
分别保护数据库备份和密钥备份。`task-key-rotate` 可在单事务中将数据库绑定到新 key。

`task-key-backup` 使用独立的 0600 passphrase 文件和 Argon2id（19 MiB/t=2/p=1）派生保护，
AES-256-GCM AAD 绑定备份格式、版本、salt 和 nonce；输出 0600 新文件，不覆盖。
`task-key-restore` 认证备份后恢复 32 字节 key，要求私有目录、0600、单硬链接且拒绝覆盖现有 key。
passphrase 文件也必须为当前 UID 的 0600 单链接普通文件，至少 16 字节；错误 passphrase 不生成 key。

运行中密文认证失败返回原有 `INTERNAL` 类型，HTTP 为 503，gRPC 为 unavailable；
不返回或记录明文。分发先在事务内解密验证，再提交 RUNNING 和发送执行请求。
任务查询先通过原认证/租户检查，再解密结果；不会将信封直接发给浏览器。

这不是数据库整库加密，也不清除升级前 WAL、旧数据页、备份或旧 agent results 历史中的明文。
本地升级备份仍保留在私有开发目录，不纳入公开证据清单。中心 key 的发行、备份、保留与轮换流程
仍需随正式中心安装包实施；agent 的密钥及卸载约定保持独立。
