# 受限远程配置更新实施与验证

日期：2026-11-01。阶段2配置增量。typed Protobuf → Rust中心/agent → UDS → Java →
真实核心配置与快照已接通，React 提供中英文编辑与任务查询。仅对审核PID显式开启，
完整管理体系和阶段2–5尚未完成。当前契约见 `configuration-remote-update-v1.md`。

## 代码与边界

Operation12和新typed oneof保留旧wire编号。中心要求admin/租户/CSRF/同源、启用和近期在线网关、
对应能力；敏感输入/结果沿用独立文件密钥加密，幂等不再执行，未决写阻挡新任务。
agent仅在显式opt-in时宣告能力，单写锁跨越持久准入、UDS和持久完成；发送后不确定返回UNKNOWN，
持久RUNNING/UNKNOWN跨agent重启继续挡写。结果仅白名单投影，不回显属性，不重新租约配置写。

Java默认不登记update，只有显式审核PID列表才开放。get附updateEnabled/editable；
仅声明类型且无min/max/options的字段可编辑，必填空值拒绝。核心opaque revision及其并发边界
见前次基础契约。遗漏密码保持，显式新密码才设置；Long使用字符串，数组/Vector有界。
React明确选择字段并确认；提交后禁用旧视图；UNKNOWN和HTTP准入不明不自动POST重试。
新增界面双语，切换语言保留未提交草稿。最后补齐能力标签Operation12的翻译映射。

成功只证明提交/原快照策略完成，不代表组件健康。完整metatype验证、SelfConfiguringComponent适配、
删除/reset/factory、配置UNKNOWN持久业务核定尚未交付，不能开放通用编辑器。

## 本地门槛

证据均位于本仓库target，清单见 `evidence/configuration-update-20261101.json`。

| 范围 | 结果 | 证据文件前缀 |
| --- | --- | --- |
| Rust中心/协议 | 中心8、协议7通过；默认排除的2项DB与3项mTLS另行显式通过 | configuration-update-console-reviewed、configuration-update-explicit-database-mtls |
| 宿主agent | 60通过，无ignored；构建通过 | configuration-update-agent-reviewed、configuration-update-agent-build |
| Java bridge | 26通过并install | configuration-update-bridge-first |
| React类型/生产构建 | 通过，约1005kB chunk提示保留 | configuration-update-react-reviewed-final |
| 最终默认浏览器 | 以最终JSON统计为准，完整23项通过 | configuration-update-browser-serial-final |
| 真实传输/故障契约 | 22通过，测试Python桥接，真实Java写入0 | configuration-update-transport-reviewed |

传输契约用独立身份和测试桥接，验证typed参数、权限拒绝、加密、幂等、UDS交付后断开UNKNOWN、
未决挡写和agent重连不重放；不能代替下一节真实Java验证。协议夹具已停服/禁用身份，数据保留。
默认浏览器中的模拟任务明确标注stubbed；它们不是系统写入证据。

## IDEA完整Kura与真实浏览器

在单元/契约/默认浏览器之后，用IDEA Run `Kura configuration update isolated`启动独立运行集，
`.phase1-dev/configuration-update-idea-20261101`，端口18380/18745/18746。
只开放测试PID `org.eclipse.kura.management.revisionFixture`；独立agent、证书、socket、SQLite和Kura home。
inventory282项SHA匹配，OSGi安装281 bundles。只读观察器只在test-classes单独打包，不负责写入。

| 检查 | 结果 | 证据 |
| --- | --- | --- |
| 原GWT实际登录 | 首次与重启后各4项通过 | configuration-update-idea-gwt / idea-restart-gwt |
| 真实React双语写入 | live浏览器1项通过；中文改Long并保持密码，英文显式设置新密码 | configuration-update-idea-live-recovered |
| 真实CM/DS/快照 | initial/keep/set/rejected/restart各9项；CM两次各增1，密码对象匹配、快照无明文、0600 | configuration-update-idea-receipts目录 |
| HTTPS拒绝 | 15项通过，旧revision、非法类型、未审核PID拒绝，CM count不变 | configuration-update-idea-rejected |
| 正常重启 | 11项HTTPS通过，恢复值/密码，新revision，旧token拒绝 | configuration-update-idea-restart |
| 停止保留 | IDEA Stop后JVM退出，独立agent停止、gateway禁用，原宿主agent保留 | configuration-update-idea-finish |

写JVM40357、重启41238，独立agent恢复后40778。恢复前首次启动因OpenSSL产出的key为0644被agent
正确拒绝，浏览器因此没有配置面板；确认网关任务为0、stage仍initial、SQLite尚未创建后，
仅收紧key为0600并启动该agent。首次失败日志和一次性marker保留，prepare未重放。
准备脚本已修复私钥模式，并提供严格前置条件的recover-agent，成功浏览器使用一次性恢复marker。

## IDEA之后的Ubuntu24.04 arm64

构建复用既有 `kura-package-validation-20261029`；55项源文件SHA匹配，额外派生协议专用workspace
成员清单，VM本地独立Cargo target。首次缺workspace根导致manifest继承失败，未生成二进制；
保留tests-first日志，补齐根清单后60项通过、ELF构建成功。
这是本地源码映射验证，不是新发布crate或正式agent deb验收。

联调复用 `yofc-management-network-20261101`，独立
`/opt/yofc-configuration-update-20261101`、账号yofc-config-update（UID993）、两个新unit和新身份。
完整282制品运行集逐项匹配，Kura使用none profile、768MiB且非root；agent网络权限列表为空。
两进程CapEff=0；agent运行时NoNewPrivs=1。现有OrbStack全局service覆盖使Kura NoNewPrivs=0，
原始unit声明不等于实际生效，完整systemd文件系统/进程沙箱继续deferred，未改全局覆盖。

| 检查 | 结果 | 证据 |
| --- | --- | --- |
| 真实远程写入 | HTTPS13项通过，Long/保持密码/显式密码两写 | configuration-update-linux-write |
| 拒绝无CM写 | HTTPS15项通过 | configuration-update-linux-rejected |
| 正常服务重启 | HTTPS11项通过，值/密码/快照恢复、旧token拒绝 | configuration-update-linux-restart |
| 只读观察器 | 5阶段各9项，真实生产provider/CM/DS/快照 | configuration-update-linux-receipts目录 |
| 夹具最终停止 | 两unit停止、身份禁用、数据保留；旧agent8391不变、8项基线SHA不变、282制品匹配 | configuration-update-linux-finish / linux-final-audit |

首次start已启动服务并生成initial证明，随后status额外断言Kura NoNewPrivs失败；保留失败日志。
修正观察器只对agent强制NNP、明确记录Kura的沙箱限制，再调用只读status继续，未重放start。
运行时写JVM20094、重启20383、独立agent20092；网络系统写入为0。

默认浏览器最后一次标签补齐后的回归曾22通过/1失败，失败页明确显示typed429“请求较多”；
此前API登录及本套登录累计触发限流。保留该日志，在无其他登录并经过窗口后独立串行复跑，
不降低生产限流。更早未取得响应证据的登录失败仍只记录为未确定原因，不能倒推为同一根因。

## 停止与续接

本轮按用户明确要求收尾后暂停goal，不把阶段2–5标为完成。新IDEA/Linux夹具均停服保留，不删除
state、快照、日志或一次性marker，不重放prepare/start/write/restart。原target/runtime和~/.kura-dev未清理，
不连接192.168.2.125，未新增VM/启动额外Parallels，未提交/推送/改旧远端。

下一轮先读 `management-redesign-handoff-20261101.md` 和workspace知识库最新交接。
优先完成配置适用性/元数据约束，再推进snapshot save/rollback和配置UNKNOWN业务核定，保持双语。
正式中心包、agent新协议发行包及Linux真实Kura包组合、旧revision5升级、新版26.04回归仍pending。
key轮换、完整网络发行、nftables、系统控制、离线恢复仍未完成；amd64、真实有线、磁盘满、
逐点崩溃与硬断电等无环境验证继续deferred。旧revision证据清单是历史快照，不能替代本轮SHA。
