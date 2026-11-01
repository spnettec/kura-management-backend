# 中心任务数据加密增量

日期：2026-11-01。接续 `phase2-task-journal-validation-20261101.md`。
本次实现中心 PostgreSQL 任务 input/result 加密；阶段 2–5、配置写入、快照写入仍未完成。
格式、初始化与恢复边界见 `task-payload-storage-v1.md`。

## 实际改动

新增 `task_crypto.rs` 和迁移 `0007_task_payload_encryption.sql`。独立 32 字节 key 文件要求
私有目录、当前 UID、0600、单硬链接、普通文件，使用 O_NOFOLLOW 打开。`task-key-init` 显式创建且
拒绝覆盖；其他命令先加载 `CONSOLE_DATA_KEY_FILE`，再连接数据库，不自动补造丢失 key。

input/result 使用随机 nonce 的 AES-256-GCM 信封，AAD 绑定版本、任务 UUID、租户、网关、操作码和字段。
首次启动排他锁定任务表，每页 100 行，在单一事务中完成历史转换、明文写入约束与数据库 key-check。
任务状态、有效期、租约、事件、审计不被迁移重写；失败整体回滚。后续启动认证 key-check，不改历史密文。
HTTP 创建与幂等比较、gRPC 分发、回执写入及任务查询均已接入。分发先解密，再提交执行租约。
结果查询认证失败仅返回原有 INTERNAL 类型，不把密文信封或明文诊断返回浏览器。

开发入口、示例环境与 README 同步 key 约定。故障脚本改用停下本夹具 agent 后的真实 HTTP admission，
再仅注入该任务的 UNKNOWN 元数据；不直接插入明文 payload，不发送执行请求。

## 单元、数据库与真实故障

| 验证 | 结果 | 本仓库证据 |
| --- | --- | --- |
| console 单元 / protocol | 7 / 6 通过，数据库 2 项及 mTLS 3 项默认 ignored | `target/task-storage-unit-final-20261101.log` |
| 显式 PostgreSQL | 2 项通过，各自独立 schema 后清理 | `target/task-storage-postgres-first-20261101.log` |
| 显式 mTLS | 3 项通过 | `target/task-storage-mtls-20261101.log` |
| Rust fmt / 构建 | 通过 | `target/task-storage-fmt-20261101.log`、`task-storage-build-20261101.log` |
| Python 入口语法 | 3 个修改/新增脚本通过 | py_compile 本轮执行 |
| 默认浏览器 | 16 通过，0 skipped/unexpected/flaky | `target/task-storage-browser-20261101.json` |
| 真实协议故障 | 22 项通过 | `target/task-storage-live-contract-20261101.json` |

5 个新单元验证随机密文、字段/任务/租户/网关/操作/错误 key 绑定、篡改和明文拒绝、文件权限/链接/长度、
显式创建不覆盖。新 PostgreSQL 契约先放入 101 条历史任务和一条故障数据，确认跨页转换失败整体回滚；
修复仅测试数据后验证两个并发启动只转换一次，102 条元数据不变，旧明文写入被拒绝，错误 key 不改变记录。
另一项为既有 UNKNOWN/epoch/迟到终态契约，已适配加密存储后重新执行。

22 项 live 契约使用新宿主 fixture `task-storage-20261101`、独立身份与 SQLite/socket，经过真实
HTTPS/PostgreSQL/mTLS/UDS。受控 test-only 桥接执行一次；新检查证明数据库不含测试 PID/私有值明文、
授权查询还原原结果。临时将该测试任务 result 换成 input 密文，得到 HTTP503/INTERNAL，再精确恢复原密文。
这仍是协议/存储故障证据，不是 Java 配置写入或 Linux 未决系统写入业务对账。
fixture 最终 stopped-data-retained，gateway disabled，不删 state 重放。

## 开发库实际升级

停止本轮独立中心，私有目录 `.phase1-dev/task-storage-upgrade-20261101` 保留升级前数据库 dump、
任务元数据和计数；未作为导出制品。原 610 条任务全部转换：610 个加密 input、509 个加密 result、
101 个 SQL 空 result。全部任务元数据逐值匹配，未更改业务状态。
错误 key 和缺 key 的 migrate 均被拒绝，缺失文件未生成。
证据 `target/task-storage-upgrade-20261101.json`。

完成新故障夹具后实际重启中心，614 条当时任务的全部密文与原 key 不变，开发 agent 保持原进程。
证据 `target/task-storage-restart-20261101.json`。默认浏览器执行期间发生该重启，16 项全部通过；
不将其扩展为可写事务或高可用验收。

## IDEA 与 Linux

单元/契约和默认浏览器之后，从 IDEA Run `Kura management phase1 isolated` 启动完整隔离 Kura，
PID26069。CUA 刷新旧页面后实际重新登录原控制台，见 admin 和设备状态页。
真实 HTTP31 与 React live 双语配置读取/密码脱敏1项通过；截图
`target/task-storage-idea-react-20261101.png` 已查看。IDEA Stop 后 ps 确认退出，
按当前 `kura-runtime/inventory.json` 核对281项SHA全部匹配。
汇总 `target/task-storage-idea-gate-20261101.json`。

首轮 HTTP/live 浏览器并行使用同一 admin，新登录替代旧会话，使 HTTP 检查失败；
生产单会话策略未改，随后独立 HTTP31 通过。首轮日志与重试JSON分别保留为
`task-storage-idea-http-20261101.log`、`task-storage-idea-http-retry-20261101.json`。
最初核对误用了历史 pre-bridge inventory，两条已更新jar SHA不同；改用当前运行集清单后281项匹配，
没有替换或清理运行集。本轮未改 Java/React 生产源码，不将旧 Java 测试或前端构建算作新执行。

IDEA 门槛之后，既有 OrbStack Ubuntu24.04 arm64 的 `network-gateway` 通过真实 mTLS 连接新中心，
状态与网络观测、密文保存、JSON还原、幂等和租户隔离16项通过。
证据 `target/task-storage-linux-agent-20261101.json`。旧服务PID8391保持active，Kura缺席，
无网关写入、未重启旧agent。包夹具状态复核 `target/task-storage-linux-baseline-20261101.json`
确认旧baseline不变、新包夹具仍purged。Parallels Ubuntu26.04/Windows均stopped，未新开VM。

在既有编译 VM `kura-package-validation-20261029` 内复制独立 console 源码，23 项输入 SHA
与宿主当前源码逐项匹配，不依赖 sibling。Linux console 单元7项、protocol6项通过；
数据库2项和mTLS3项仍为默认ignored，未算作Linux实库执行。独立 `cargo build --locked --workspace`
通过，Linux debug 中心制品已导出校验；本段没有部署 Linux 中心或访问 Linux PostgreSQL。
日志为 `target/task-storage-linux-unit-20261101.log`、`target/task-storage-linux-build-20261101.log`，
源码清单为 `target/task-storage-linux-console-source-20261101/source-manifest.json`。

本次只刷新真实 console 子仓库：4498节点/9902边，8个关键路径均为metadata_match；
7个Rust/Python路径无记录解析缺口，0007迁移第4行partial，已全文读取核对。
当前SQL/配置共3 partial/3 unusable，不能用indexed推导完整解析。聊天原MCP仍Transport closed，
官方CLI可用，没有刷新父工作空间或删除历史缓存。图谱与覆盖输出为
`target/task-storage-graph-console-20261101.txt`、`target/task-storage-coverage-console-20261101.json`。
当前源码、制品及选择的证据SHA见 `docs/evidence/task-storage-20261101.json`；
此前 task-journal 清单保留历史快照，不覆盖为新源码版本。

## 保留边界与后续

- 加密范围是中心任务 input/result，业务标识及状态等元数据仍可查询。旧 WAL、旧数据页、备份和旧 agent
  results 历史未清除；不能声明整库加密、历史介质擦除或全系统敏感存储验收。正式中心包的 key备份/恢复/
  轮换/保留和细粒度敏感结果权限仍需继续实施。
- 网络 UNKNOWN 业务结果证明已在 2026-11-01/02 补齐；配置 update、snapshot save/rollback 也已完成真实IDEA验证。
- 新agent包与真实Linux Kura组合、旧revision5升级、26.04新版回归仍pending；完整OrbStack文件系统沙箱、
  amd64/物理有线/硬断电仍deferred。本轮没有重做包生命周期或更改网络provider。
- 本轮 shell 的 cargo 不在 PATH，原 `~/.cargo/bin` 链接目标缺失；使用已经安装的
  `~/.rustup/toolchains/1.95.0-aarch64-apple-darwin/bin` 完成构建，未修改全局链接或重装工具链。
  两份127失败日志保留。代码未提交、推送或reset/clean，未触碰个人 ~/.kura-dev。


## 2026-11-02 任务数据 key 轮换

新增 `task-key-rotate CURRENT_KEY_FILE NEW_KEY_FILE`、`TaskCipher::rotate_storage`
和迁移 `0010_task_key_rotation.sql`。轮换在单事务中排他锁定任务表，逐页用旧 key 解密、
新 key 重加密并更新 AAD 密文；最后才把 `mgmt_task_crypto` 绑定升到 version 2。新 key
必须已存在且与当前 key 不同，两个方向不同 key 拒绝可证明；失败整体回滚。

显式 PostgreSQL 契约在既有 102 条任务基础上轮换：102 条全部转换，任务元数据历史一致，
新旧 key 双向验证、错误方向拒绝、旧 key 失效、新 key 重启校验 0 条重写。
开发中心实际停服轮换：原 key 文件保留未改，新 key 0600；version 升为 2，
审计 `task.key.rotate/singleton/SUCCEEDED`，中心用新 key 重启后 `migrated_tasks=0`。
旧 key 再次轮换被版本保护拒绝，无写入。轮换后原 dev-gateway `agent.status` 真实任务
`f060cc15-5921-40a2-9417-d1db18a45111` 成功。本方案只轮换 PostgreSQL payload key；
旧介质/WAL/备份/agent历史和文件系统恢复仍不在本轮范围。


## 2026-11-02 任务 key 加密备份与恢复

新增 `task-key-backup` / `task-key-restore`。备份由独立 0600 passphrase 文件经 Argon2id 派生保护，
AES-256-GCM AAD 绑定格式、版本、salt 和 nonce；备份 0600、不覆盖。恢复要求私有目录、0600、
单硬链接、不覆盖现有 key，并认证完整备份；错误 passphrase 不生成文件。
单元覆盖精确恢复、错误 passphrase、尾字节篡改、弱 passphrase、公开 passphrase 和拒绝覆盖。

开发中心 version 2 key 已真实备份到
`.phase1-dev/key-recovery-20261102/task-data-key-2.backup`；恢复到独立文件后用其执行 `migrate`，
成功认证 version 2 数据库绑定。错误 passphrase 恢复失败且未生成 key。
该流程保护 key 文件恢复；仍不覆盖数据库旧 WAL/备份、agent历史或整机恢复。
