# 配置条件更新基础实施与验证

日期：2026-11-01。接续网络 UNKNOWN 业务核定。交付核心条件更新 API、Java 桥接内部处理和真实
CM/DS/快照证明；**远程配置写入口尚未接通，阶段 2–5 仍进行中**。完整契约见
`configuration-conditional-update-v1.md`。已有中英文保持，新界面继续双语。

## 实际代码

核心新增独立 ConfigurationRevisionService；与旧入口同 DS 实例和 monitor，使用随机实例密钥 HMAC
绑定秘密、类型、元数据、CM change count 和准入计数。并发旧 token、CM ABA、跨实例 token 拒绝。
仅 set patch，遗漏密码保持，显式 Password 设置，系统/未知字段与错误类型在 writer 前拒绝。
SelfConfiguringComponent 未报告提交值时暂不发新 token；发生可能已写入的异常时启用 uncertain guard，
须明确成功的旧更新/rollback 或 provider 重启后解除。后者不是持久业务结果核定。

Java 新 ConfigurationPatch 严格转换元数据类型、Long 字符串、bool、数组/Vector，并核对 revision 与 deadline；
隐藏内部异常和输入值。get 可附 revision/writable，但 update 尚不登记，旧 agent 白名单未变。
配置包导出升至 1.4.0，core/bridge 最低 1.4；core CM 最低 1.5。
实际 IDEA 揭示 container provider 的旧上限 `[1.2,1.4)`，已修为 `[1.2,2.0)` 并构建整个 container 仓库。

直接 ConfigAdmin 写者不共享此 monitor，只在准入时检查 change count；不宣称对这些写者原子 CAS。
额外校验覆盖类型/cardinality，其他约束仍沿用旧路径；完整元数据校验、SelfConfiguringComponent 适用性、
删除/reset/factory 语义需继续实施。不得据此开放通用远程编辑器。

## 单元与浏览器门槛

| 范围 | 实际结果 | 本仓库 target 下证据 |
| --- | --- | --- |
| 最终 API/core 配置依赖构建 | test-util 6、API 23、util 136、core 111、configuration 73，均无失败/错误/跳过，install 通过 | `configuration-revision-core-metadata-final-20261101.log` |
| 最终 Java bridge | 24 项通过/install，含 6 项新 patch 测试 | `configuration-revision-bridge-final-20261101.log` |
| container 整仓库 | 53+78=131 项通过，install 通过 | `configuration-revision-container-20261101.log` |
| React 类型/生产构建 | 通过，既有大 chunk 提示保留 | `configuration-revision-react-build-20261101.log` |
| 最终默认浏览器集 | 17 项，0 skipped/unexpected/flaky | `configuration-revision-browser-final-20261101.json` |
| IDEA 真实 React 双语读取 | 1 项通过，无任务模拟 | `configuration-revision-idea-react-live-20261101.json` |

配置模块新增 revision 编码/类型/界限 3 项、准入并发/ABA/旧视图/失败后挡写 5 项，
网络 owner 不得绕过 1 项，实际核心 CM 转换/密码/快照策略路径 2 项。
后两项 CM 为 mock、snapshot marshal 为捕获，不替代下面真实 XML/OSGi 验证。
bridge 另验证 JSON 数值/密码类型、拒绝错误输入、stale/过期无写、转换后的期限检查、核心竞态拒绝及 UNKNOWN 脱敏。
原 Rust 中心/agent/Protobuf 与 React 生产代码此增量未改，未重跑其 Rust 测试，不沿用前次计数冒充新证据。

## IDEA 完整隔离应用

新增 test-only ConfigurationRevisionRuntimeProbe 和 SCR/metatype 夹具，仅从 test-classes 单独打包。
生产 JAR 不含探针，准备脚本会核对。新运行集为
`.phase1-dev/configuration-revision-final-20261101/runtime`，数据单独位于其同级 kura-home；
原 `.phase1-dev/kura-runtime`、普通 target/runtime 及个人 ~/.kura-dev 均未清理。

通过 IDEA Run `Kura configuration revision isolated`，写入 PID 34416、重启 PID 35319，实际安装 281 bundles。
26 项写入检查使用真实生产 provider、ConfigAdmin 与 SCR，两个相同 revision 的并发请求只有一个 CM 写入；
密码保持/显式更改、Long.MAX_VALUE、数组、非法输入无写、真实 snapshot 读取、磁盘无明文测试密码、0600、
直接 CM ABA 均通过。Stop 后明确刷新审核过的 API/core 元数据，再通过 IDEA Run 进行 12 项重启检查：
持久值与密码恢复、旧 token 拒绝且 CM change count 不变、新实例 token 改变。

元数据刷新前后仅 JAR 的 MANIFEST.MF 字节不同，全部 class bytes 相同，旧 JAR 保留；
逐项比较见 `target/configuration-revision-jar-compare-20261101.json`。
最终 metadata 制品的全套写入检查另外在下面 Linux 真实执行，不能把 macOS 首段 JAR SHA 当最终 SHA。

| 门槛 | 结果 | 证据 |
| --- | --- | --- |
| 写入/重启探针 | 26 / 12 项通过，fixtureOnly=true | `configuration-revision-idea-probe-write-20261101.json`、`configuration-revision-idea-probe-restart-20261101.json` |
| 真实读取 HTTP 契约 | 首段 31、最终重启后 31 通过 | `configuration-revision-idea-http-final-20261101.json`、`configuration-revision-idea-restart-http-20261101.json` |
| 新 revision 远程只读投影 | 8 项通过，密码脱敏、Long 精度及未完成 update 不准入 | `configuration-revision-idea-read-20261101.json` |
| 原控制台实际 admin 登录 | CUA 登录观察及最终自动化 4 项通过，重启后再次通过 | `configuration-revision-idea-console-verified-20261101.json`、`configuration-revision-idea-restart-console-20261101.json` 与同名 png |
| 最终停止保留 | IDEA Stop 后 PID 35319 已退出，状态 stopped-data-retained | `configuration-revision-idea-finish-20261101.json` |

## IDEA 之后的真实 Linux

复用 OrbStack `yofc-management-network-20261101`，Ubuntu 24.04 arm64；未创建 VM，未启动额外 Parallels。
新目录 `/opt/yofc-configuration-revision-20261101`、新账号 yofc-revision（UID 994）、新专用 Kura unit。
复制当前 282 项 inventory 的完整运行集，使用最终审核的 Java JAR；none profile、768 MiB heap、
NoNewPrivileges、CapabilityBoundingSet 为空，实际进程 CapEff=0。网络 writer 未授权，桥接指向独立的缺席 socket。
这是独立核心 API/DS/快照验证；没有新 agent 身份，没有开启远程写入，没有安装正式包。

| 步骤 | 结果 | 证据 |
| --- | --- | --- |
| 独立准备 | 当前 inventory 输入逐项匹配、独立数据与账号 | `configuration-revision-linux-prepare-reviewed-20261101.json` |
| 真实写入 | 26 项通过，Kura PID 19533、UID994、CapEff0 | `configuration-revision-linux-write-20261101.json` 及 receipts/write-result.json |
| 正常服务重启 | 12 项通过，新 PID，旧 token 无写、值与密码恢复 | `configuration-revision-linux-restart-20261101.json` 及 receipts/restart-result.json |
| 最终停止保留 | 新服务停止、旧 agent PID8391 在线、八项旧状态/Netplan 输入 SHA 不变 | `configuration-revision-linux-finish-20261101.json`、`configuration-revision-linux-final-audit-20261101.json` |

独立最终复核确认全部 282 项 inventory SHA 匹配。七份无凭据回执保存在
`target/configuration-revision-linux-receipts-20261101/`。新夹具为 stopped-data-retained；
同名 prepare/start 不得重放，不删除状态或快照。普通 JVM/systemd 重启不是断电或逐点崩溃验收。

## 保留的失败与边界

- 核心测试最初的 Tocd fixture、checked exception、Mockito 嵌套 stubbing 问题与 bridge 测试缺 Log4j API
  已修复；原失败日志保留。最终生产依赖未为此增加日志 provider，bridge 新 log4j-api 仅 test scope。
- 首次 IDEA 发现 container 上限不匹配；夹具首次/第二次未注册，最终精确源代码确认 Kura 按 PID.xml
  读取 metatype，修正测试打包文件名。两个失败根目录均 stopped-failed-data-retained，无重置重放。
  显式 Kura PID 属性同时保留，但它不是第二次失败的最终根因。
- reviewed 夹具准备碰到旧 HTTP 端口仍不可 bind，在没有 data/JVM args 的明确阶段停止；保留文件后用
  resume-ports 续接并换独立端口。最终夹具为新目录，不覆盖旧证据。
- HTTP 首轮在桥接注册前执行，live 断言失败，原日志保留；实际就绪后及最终重启后各完整 31 项通过。
- 新原控制台自动化最初假定英文、匹配到隐藏模板 admin、等待 networkidle 均失败。
  最终显式英文登录语言、按可见 body 的双语状态与 JVM 信息核对，不等待持续请求全部 idle；
  原控制台真实 CUA 登录本身已成功。所有旧日志/诊断截图保留，不将其错误当生产缺陷。
- Linux 首次准备把旧 outcome 目录的 final-state.json 写成 state.json，基线读取即拒绝，尚未创建新目录/账号。
  确认新目录不存在后修正路径，另存成功证据；没有擦除或重播任何旧夹具。

图谱只使用真实子仓库；聊天 MCP 仍 Transport closed，本地官方 CLI 可用，刷新与覆盖证据见
`target/configuration-revision-graph-*20261101.*`。kura 28664 节点/109502 边、networking 9342/43456、
container 1376/4479、console 4749/10635；分别检查 10/4/1/4 条关键路径，均无记录缺口。
15 条 Java/XML/脚本路径 freshness=metadata_match；4 个 MANIFEST.MF 为 not_tracked，
不能凭图谱证明其文本新鲜度，改以实际文件/JAR manifest 与制品 SHA 核对。
kura 仍有15 partial/1 unusable，console 3 partial/3 unusable，ready 不代表完整解析。
core updateRevision 的精确限定名双向 trace 保留；同名 getBytes 等仍有误匹配，
静态引用不替代真实 CM/并发探针。SHA 清单含37项源码/配置/文档、8项制品和66项所选证据，逐项读回匹配。
源码、制品和所选证据 SHA 清单为 `docs/evidence/configuration-revision-20261101.json`。

下一步必须真正接通 typed Protobuf 更新、agent 可选方法/能力与发送后 UNKNOWN、中心 admin/CSRF/租户权限、
加密/幂等/未决阻挡，以及 React 明确 keep/set 密码、数字字符串、revision 确认和中英文编辑界面。
当前仅内部 Java handler，不得宣称远程配置修改已完成。之后继续 snapshot save/rollback、持久业务结果核定、
完整网络发行、nftables、系统控制与离线恢复。26.04/amd64/完整 systemd 文件系统沙箱、物理有线、
磁盘满、真实断电等未验证项继续 deferred。所有改动仍未提交、未推送，旧远端未改。
