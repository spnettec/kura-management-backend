# 常驻 agent 首次身份接收：后端合同与当前边界

> 后续Java/GWT接线已实现，证据与未完成范围见 [gwt-agent-enrollment-v1.md](gwt-agent-enrollment-v1.md)。本文件下方关于尚未接线的段落保留为agent后端增量结束时的历史边界；新代码通过宿主契约和真实Java provider联调，尚未完成完整浏览器/IDEA/Linux验收和中心认证。

2026-11-05 增量。整体管理体系继续 active。保留 NM 与现有 Netplan 配置，不修改 renderer；真实冲突依据现场取舍。此增量实现 agent 后端首次接收，不代表 GWT 提交按钮、中心登记认证或正式发行已交付。

## 授权与材料

Kura Java 后端继续独立生成 P-256 私钥和 CSR。agent 不生成网关私钥，仅在显式授权后，从固定本机暂存目录读取、复制和转换现有 PKCS8。浏览器、中心、UDS 参数和任务结果都不承载私钥。

`AGENT_KURA_ENROLLMENT_DIR` 默认不存在，表示未授予首次接收能力；设置时必须为固定绝对目录，并配置明确的 `AGENT_KURA_UID`。这是独立于 `AGENT_KURA_REMOTE_MANAGEMENT` 的授权：允许首次接收不会自动允许远程开关。接收后可以撤销目录授权，已接收的身份从 agent 私有状态加载。

新本机方法：

- `remote.enrollment-status`：返回 bound、legacyIdentity、recoveryRequired、canEnroll，以及已有回执的申请 UUID、stage 摘要和客户端证书指纹。不会返回路径、私钥或任意文件内容。
- `remote.bind`：精确参数 expectedRevision、requestId、expectedStageSha256、operator；拒绝额外参数、任意路径及密钥材料。仅明确授权目录下的已注册 Kura 连接或本机 root/admin 可调用。普通 Kura UID 的未注册连接不能借此取得权限。

operator 是有界的操作者声明；回执另存通过本机连接验证的角色/UID，二者不混为已经证明的 Web 用户身份。后续 GWT 适配应从服务器认证上下文取得操作者，而不能信任网页填写的用户名。

私有目录及文件须满足固定属主、0700/0600、普通文件、单硬链接和非符号链接路径检查。申请/证书目录采用 `certificate-staging-v1.md` 的固定格式和摘要语义。请求只引用 UUID 和预期摘要；先复制到全新 agent 私有候选目录，再针对副本独立校验。每次尝试均保留候选，最多 16 个，达到上限要求明确核查归档，不自动删除失败材料。

## 独立校验

agent 使用固定 OpenSSL 3 可执行文件及有界子进程执行，不调用 shell、不获取网络材料。校验 P-256 PKCS8/SPKI、私钥派生公钥、CSR 签名/单 CN、客户端证书主体/公钥、用途和签名 KU、有效期、关键扩展、CA 约束及签发链；验证独立中心信任 CA。证书 PEM 必须规范、最多 8 张/64KiB，拒绝重复、垃圾和不支持的弱公钥。

接收后的证书验证仍为离线验证，不证明中心服务器可达、中心已登记该网关或在线撤销状态。启动时重新验证已接收材料及回执，缺失/损坏的派生文件不自动补造。证书过期或身份材料不一致时，本机进程仍可运行，但远程身份进入需要恢复的状态。

## 持久提交与运行

共享系统变更锁检查网络、防火墙、恢复未决事务，另检查执行库中的 RUNNING/UNKNOWN 写任务。已有环境身份或已接收身份均拒绝通过首次接收替换；此入口不是转移或恢复入口。

提交顺序为：候选完整材料 → 持久操作者/材料回执及摘要 → 根目录 identity-enrollment-pending.json → 只创建一次的 enrollment/identity.json → 新 revision 的远程 OFF 决定 → 将 pending 归档为候选 completed-intent.json，并同步目录。

回执包含申请/材料摘要、网关/中心地址、generation=1、已验证的连接角色/UID、operator 声明和时间。成功回执不依赖全局 INFO 日志是否被过滤。已有公开证书暂存 RPC 的 INFO 审计缺口仍保留，不能宣称其被此回执追溯修复。

身份和远程决定在同一个内存准入锁下更新，传输协程监听身份变化，不停 agent 服务。成功后远程仍关闭，另行明确授权的开关操作才可能建立中心连接。失败或不确定提交立即阻止身份/远程通道写操作；重启发现 pending、孤立回执或损坏身份同样挡住。普通 remote.resolve-disabled 不能清除身份提交故障。身份故障的运维核定/恢复入口仍 pending，不通过删除标记或重放 bind 处理。

打包检查允许无私钥环境路径的已接收身份正常启动，实际完整身份验证由 Rust 启动执行；旧 import-identity 拒绝覆盖任何已保留的 managed 候选/回执。未决身份提交挡住包维护，purge 拒绝删除候选/回执，须先有明确核定的完整归档。尚未提供正式归档/转移工具。

## 本轮证据及未完成事项

宿主 Rust 112 项通过，另有真实 nft 测试 1 项 ignored；Python 打包 54 项通过，严格 Clippy 和 fmt 通过。新增测试包含 7 个持久发布边界的故障注入、7 类签名有效但不适用的客户端证书、权限/额外文件/私钥损坏、孤立回执、角色授权，以及 UNKNOWN/未决网络事务准入。

两组全新宿主夹具均由实际 Java RemoteIdentityStore 生成私钥、CSR 和公开证书暂存，再由新 agent 通过已注册 Kura 角色的 UDS 连接完成一次接收；最后一组对应最终运行代码。已验证持久回执、默认关闭、重启加载、拒绝再次接收，原始暂存文件摘要完全不变；agent 未生成私钥、未连接中心，夹具均已停止并保留。这里的注册连接是协议测试夹具，不能称为完整 Kura/GWT 身份提交验收。

新 IDEA 完整 282 项运行集已准备，但 Mac 桌面锁定，未启动。IDEA/浏览器验证 pending，按用户要求的顺序，Orb/Parallels 新代码验收也尚未执行，不以旧通过结果替代、不记为 deferred。现有 Parallels 缺 Java 的既定外部环境边界继续保留。

下一步仍须接通可选 Java bridge、ADMIN/XSRF/安装门禁下的 GWT 提交/回执核对和中英状态，完成完整隔离 Kura 及原控制台验证后再进入 Orb、现有 Parallels；之后验证真实中心登记认证和绑定后开关。当前未发行新包，原候选包不包含此增量。

日志、全新私有夹具指针、修改前副本及状态：`target/live-enrollment-20261105/`。原 Orb 三服务仍 active，原 console26/agent20 包未变，旧 phase2 agent active 未动；Ubuntu/Windows VM 均 stopped。原业务 pending（完整组件适用面、Java 真实业务恢复、防火墙/NAT/远程/早启、restart/reboot、一致备份恢复、轮换/转移和正式发行）均保留，不能把本机身份接收计作 Kura 业务恢复。

## 2026-11-05 本轮系统验证完成并按用户要求暂停

本轮最新核定见 [系统验证与暂停](reviews/gwt-agent-enrollment-system-20261105.md) 及 `evidence/gwt-agent-enrollment-system-20261105.json`。GWT空列表生产序列化与显式CA环境信任旁路已修复。真实IDEA完整Kura及Orb中英GWT交付/离线CSR/持久admin回执、Parallels最终agent严格沙箱通过；Parallels无Java，不代表实际Kura业务恢复。新夹具全部停止保留、VM均stopped，原Orb三服务原26/20及phase2-agent继续active。中心认证/远程开关与正式发行和原业务仍pending。等待明确继续，不重放一次性入口。

## 2026-11-05 恢复后的中心登记与通道验证

用户已恢复继续，整体active。最新实现、精确验证与运行状态见 [中心登记与mTLS闭环](reviews/gateway-registration-channel-20261105.md) 及 `evidence/gateway-registration-channel-20261105.json`。中心新增管理员双语公开叶证书登记；修复Java/SunEC最小PKCS8与ring兼容；原GWT开启→真实中心/Kura只读任务→关闭/重启保持OFF通过。36项既有身份/源材料不改。Parallels仅native严格沙箱/mTLS，无Java。新夹具全停/VM均stopped，原Orb26/20服务保持active。托管CA、身份运维和完整原业务/发行仍pending，不把此次通道验证计作Kura业务恢复。
