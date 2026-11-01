# 配置详情读取与中英文验证

日期：2026-11-01。接续独立 agent 包与 UID 授权增量。此处交付配置只读闭环，
完整六项业务桥接和阶段 2–5 继续进行。React 中英文约定见 `console-i18n-20261101.md`。

## 实际实现

共享 Protobuf 增加 `KURA_CONFIGURATION_GET = 11` 和 `KuraConfigurationTarget`。
请求使用明确的 `pid`、`locale`，后者只接受 `en` / `zh`；PID 不允许空白、控制字符，
最多 512 UTF-8 字节。中心校验 operation 与参数的搭配，将完整目标纳入任务幂等比较，
通过新增迁移 `0005_configuration_read.sql` 扩展持久化操作约束。
新增 `GET /api/v1/tasks/{id}`，按认证租户读取任务；跨租户或未知 UUID 返回 404，非法 UUID 返回 400。
OpenAPI 及生成的 TypeScript 类型同步更新，修正任务操作枚举此前缺少 9/10 的问题。

agent 经 mTLS 双向流接收请求，通过 UDS 调用 `kura.configuration.get`。
桥接仍直接调用 OSGi `ConfigurationService`，不读取或依赖 Kura Web 用户名、密码及角色。
旧桥接只注册两个 list 方法仍可使用；未注册 get 返回 `UNSUPPORTED`，Kura 缺席返回
`KURA_UNAVAILABLE`。已知业务错误保留固定码，未知 Java 错误不直接暴露给中心。

Java `ConfigurationView` 显式投影 PID、元数据和属性值，按 en/zh 读取 OCD/AD。
密码类型、Kura Password、字符数组及敏感名称均不输出 `value`；未知类型也不直接序列化。
不导出默认值或选项，避免绕过当前值脱敏。数值以字符串保留 Java long 精度。
结果限制为最多 512 属性、数组 256 项、单字符串 16 KiB、整体 200 KiB。
未设置值保留 `value: null`，与空字符串、空数组和已脱敏值分开。

React 新 `features/configuration` 面板支持列出 PID、搜索选择、读取属性和本地化错误。
按角色、在线状态及 get 能力控制入口；关闭/切换目标会取消轮询，直接读取单任务回执。
网关详情由最新列表对象派生，避免 SSE 更新后仍保留旧的在线/能力状态。
中英文切换保留选择和结果；已有错误即时翻译，组件元数据提示按新语言重新读取。
密码显示双语“已脱敏 / Redacted”，不把脱敏占位文字存作真实配置值。
本增量没有配置写入、快照保存或回滚入口。

## 开发检查和 IDEA 门槛

证据路径均相对本仓库；先单元/契约和浏览器，再 IDEA，最后 OrbStack。

| 范围 | 实际结果 | 证据 |
| --- | --- | --- |
| Java bridge 测试与 bundle package | 18 项通过 | `target/configuration-read-java-wirefix-20261101.log` |
| Rust console 单元 / protocol | 1 / 4 项通过 | `target/configuration-read-console-tests-final-20261101.log` |
| 独立 mTLS 契约 | 默认 ignored 的 3 项随后显式执行，全部通过 | `target/configuration-read-mtls-20261101.log` |
| macOS agent | 44 项通过 | `target/configuration-read-agent-tests-final-20261101.log` |
| React 类型检查、生产构建 | 通过，仍有大 chunk 提示 | `target/configuration-read-react-build-final-20261101.log` |
| 默认浏览器集合 | 15 通过，0 skipped/unexpected/flaky | `target/configuration-read-browser-wirefix-20261101.json` |
| 宿主 Kura 缺席 HTTP 契约 | 26 项通过 | `target/configuration-read-absent-20261101.json` |
| IDEA 真实 Kura HTTP 契约 | 31 项通过 | `target/configuration-read-idea-live-20261101.json` |
| IDEA 真实配置浏览器 | 1 项通过，无任务模拟 | `target/configuration-read-live-browser-wirefix-20261101.json` |

默认 15 项包含原 HTTPS/SSE/语言检查和明确使用模拟数据的 UI 契约，不能将它们全部描述为 Linux 实机业务。
另外的 live 浏览器通过真实 HTTPS/PostgreSQL/mTLS/UDS 读取 ClockService 和 HttpsKeystore，
确认密码没有可见值及切换英文后重新读取元数据。
截图 `target/configuration-read-live-browser-20261101.png`。

最终 bridge 修复后，IDEA 再次 Run `Kura management phase1 isolated`，PID 20092。
原控制台实际登录并显示状态页，截图 `target/configuration-read-idea-console-final-20261101.png`。
IDEA Stop 后独立 `ps -p 20092` 确认退出。没有清理原运行集或个人 `~/.kura-dev`。
停机后仅校正 `.phase1-dev/kura-runtime/inventory.json` 的 core.configuration/bridge 实际 SHA 和版本，
保留旧清单 `target/configuration-read-idea-inventory-before-20261101.json`，未再替换运行文件。

## 真实 Linux 闭环

编译使用既有 OrbStack `kura-package-validation-20261029`，Ubuntu 24.04 arm64。
`packaging/build.py source` 生成独立 vendor 源码，不依赖 console sibling source。
staging manifest 为 `target/configuration-read-linux-stage-20261101.log`。
本地 protocol 版本仍为 0.1.0，但本次 SHA 已变；没有发布 registry，也没有复用旧包验收结论。
独立 Cargo target 目录的 44 项测试和 debug ELF 编译通过，日志分别为
`configuration-read-linux-tests-isolated-20261101.log`、`configuration-read-linux-build-20261101.log`。

| 制品 | SHA256 |
| --- | --- |
| 新 protocol crate | `e30950f8731f4959ee24610d5cf87dd5fbba97550625766baa11013e9a5e0f59` |
| 最终 Java bridge | `59e7c347646d305308718b8a8c52de2a34ed96c4015261745fb86c1ab3ca06be` |
| 新 Linux arm64 debug agent | `ca33df2696c833594359f5a20f35f8132b37d08c03120ef19d715694a6fa16ed` |

联调复用 `yofc-management-network-20261101`，全新 fixture `/opt/yofc-config-read-20261101`，
gateway `config-read-gateway-20261101`，单独客户端证书、SQLite、socket 和 units。
新 Kura 账号 UID 995、CapEff 0，运行完整默认开发运行集及最终 bridge；网络 provider 为 none。
agent 的网络执行 allowlist 与 Kura 委托 allowlist 均为空。socket 为 root:987、0660，目录 0750。
此处是开发运行集与 debug binary，没有安装/升级本次新 agent deb 或正式 core/networking 包。

| 真实步骤 | 结果 | 证据 |
| --- | --- | --- |
| 先启动 agent，Kura 缺席 | 26 项通过，agent.status 成功、get 为 KURA_UNAVAILABLE | `target/configuration-read-linux-absent-20261101.json` |
| 启动低权限 Kura | 实际 UDS 注册；31 项通过，含密码脱敏、未知 PID、权限/租户/幂等参数 | `target/configuration-read-linux-live-20261101.json` |
| React 真实 Linux 配置读取 | 1 项通过，中英文切换及重新读取，无任务模拟 | `target/configuration-read-linux-live-browser-20261101.json` |
| 重启独立 agent | agent PID 改变，Kura PID 不变，自动重新注册 | `target/configuration-read-linux-restart-agent-20261101.json` |
| 重连后读取及停止 Kura | 15 项通过；en/zh 均读取并脱敏；停 Kura 后 get 失败、agent 仍可管理 | `target/configuration-read-linux-reconnect-20261101.json` |
| 最终停服保留 | 两个新 unit 停止，数据与身份保留；新测试网关 disabled，审计保留 | `target/configuration-read-linux-finish-20261101.json` |

fixture 最终 `stopped-data-retained`，不能重放 prepare 或删除 state。
6 份无凭据回执复制在 `target/configuration-read-linux-receipts-20261101/`。
旧 `yofc-management-phase2-agent.service` 始终 active；其 env、三个旧 fixture state、两份系统 Netplan
输入的 SHA 均保持不变。没有系统网络写入，没有启动 Parallels Ubuntu/Windows，也未连接已移除网关。
新开发身份保留在本地 `.phase1-dev/configuration-read-20261101`，不纳入证据归档或图谱。

## 失败和修复

- 真实浏览器暴露 Gson 默认省略 null：未设置属性没有 `value` 导致前端 PROTOCOL_MISMATCH。
  `FrameCodec` 已使用 `serializeNulls()`；新增真实编解码回归确认 unset 为 null、密码仍完全没有 value。
  `configuration-read-live-browser-second-20261101.*` 保留失败。
- 首轮浏览器换语言后仍按中文 region 定位、选择 Ant Select 隐藏虚拟 option 的测试已修正。
  一次 live HTTP 选择未注册为 configurable 的 PID，真实返回 NOT_FOUND；改为列表内 ClockService。
- HTTP 和浏览器并行登录曾触发真实限流；随后独立执行通过，未修改生产限流。
- 首次 Linux 复用旧 Cargo target，读取了同版本号 protocol 的旧生成代码，报新类型不存在。
  保留 `configuration-read-linux-tests-20261101.log`，换用本次独立 target 后 44 项通过。
- 新 Linux fixture 首次复制默认运行集时发现 none 变体没有可选 profile.json；修正脚本并严格限定续接，
  保留 prepare 失败。随后外层目录受 umask 影响而为 0700，Kura 账号无法遍历；显式改为 0755，
  仅续接未创建服务的保留状态。两次错误在 `configuration-read-linux-{prepare,resume}-20261101.err`，
  成功续接为 `configuration-read-linux-resume-runtime-20261101.json`，没有重建身份或抹除 state。

浏览器 error-context 可能含开发输入，不转抄进知识库或可提交证据清单。
机器清单 `docs/evidence/configuration-read-20261101.json` 记录源码、制品及所选验证证据 SHA。
聊天 codebase-memory MCP 仍 Transport closed；本地官方 CLI 可用，两者状态分别记录。

官方 CLI 已刷新真实 console、agent、networking 子仓库，分别为 4277/9266、678/2727、
9315/43488 nodes/edges。console 首次刷新因文档同时变化而中止，旧图谱保留；文件稳定后重试成功，
两份日志均保留。18 个关键路径全部 metadata_match；17 个无已记录缺口，新增 SQL migration
为 parse_unusable，已直接全文读取并以实际数据库任务验证。既有 SQL/Mosquitto 解析缺口仍保留，
没有索引父工作空间，ready/indexed 不代表全仓库完整解析。

配置 update、快照 save/rollback、执行前持久化、UNKNOWN 协调、敏感写入数据存储仍待实现。
完整网络旧字段/NM/cloud-init 交接、可写发行权限、nftables、系统控制、离线恢复和发行矩阵仍开放。
本轮未复测新版本 deb 生命周期、Ubuntu 26.04/amd64、完整 systemd 文件系统沙箱、物理有线与硬断电；
这些保持 deferred，不能复用历史版本结果宣称通过。代码全部保留未提交，没有 reset/clean、推送或改远端。
