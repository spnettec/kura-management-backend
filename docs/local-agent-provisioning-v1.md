# Kura 本机控制台配置远程管理

> 最新Java/GWT接线见 [gwt-agent-enrollment-v1.md](gwt-agent-enrollment-v1.md)：服务端从认证会话取操作者，固定UUID/摘要/revision一次提交并核对回执，中英UI对未暂存草稿及不确定结果挡提交。宿主契约/真实Java provider联调通过，完整页面与系统验证因桌面锁定仍pending，未发行新包。

> 2026-11-05 最新增量见 [常驻首次接收后端合同](live-agent-enrollment-v1.md)：agent后端已有显式目录授权、独立校验和默认OFF的持久接收；Java bridge/GWT提交、完整Kura/Linux验收和真实中心认证仍pending。旧停服导入不作为常驻接收产品入口。

最新增量：申请恢复、Kura后端私钥/CSR一致性自检、独立签发CA/中心CA的公开证书校验与不可覆盖暂存已接通GWT中英及实际浏览器RPC，详见 `certificate-staging-v1.md`。这不是向常驻agent交付身份或完成中心注册；原停服导入没有被接入页面来替代该设计。

2026-11-05 用户在恢复实际实施期间提出：agent 安装后的证书、console 认证等配置能否放到 Kura 原管理控制台，并提供是否开启远程管理的设置。纳入当前管理体系实施范围。本文件为待实现合同与实施顺序，不表示页面或本机配置接口已经交付。

## 用户入口

原本机 GWT 控制台新增“远程管理 / Remote management”设置，持续中英文。显示 agent 可用性、远程管理开关、逻辑中心归属、中心地址、网关标识、证书指纹和有效期、连接状态及失败原因。修改操作使用现有服务端权限、会话和 CSRF，不以隐藏按钮代替授权。

“关闭远程管理”关闭 agent 到 console 的通道，不能等同于停止 agent 进程或删除身份。已确认网络、防火墙及本机恢复独立继续运行。未决有副作用动作按原状态处理，UNKNOWN 不重发；不能为了关闭连接删除任务或 RECOVERY_REQUIRED 现场。默认远程管理关闭；已有启用部署的升级迁移必须显式保留其状态，不能悄悄切断现有中心。

## 身份与材料

Kura 登录与 agent/console mTLS 身份独立。Kura 提供本机配置入口；agent 持有和验证身份、维护归属。普通地址编辑或另一中心登记不能绕过 generation、原中心签名转移、撤销检查及未决事务守卫。

用户随后明确：agent 始终运行，私钥由 Kura 管理控制台生成，不采用 agent 生成私钥作为前置。实现由 Kura Java 后端使用 JCA 生成私钥和 CSR；生成及导出 CSR 不调用 agent，也不需要 console 在线。私钥只在后端受保护暂存区短暂存在，不返回网页、不进入 ConfigAdmin、快照、任务结果或日志；页面展示公开 CSR/指纹。仅应用身份和查询连接状态时调用可选 agent 适配服务，不能将其声明为 Kura 或 GWT 启动的 mandatory OSGi 依赖。agent 不可用只影响该功能，不影响其他控制台功能。交付成功后按受控流程清理本次临时副本，不删除任何既有身份、备份或现场。首次绑定、同中心叶证书更新、CA 信任锚轮换、跨中心转移分别处理，不能将一次证书导入当作自动换中心。

中心目前只登记客户端证书，尚无托管 CA 的自动签发产品流程；不得把“一次性注册包”当成已存在的发证能力。可以先兼容运维签发证书并在中心登记，公开注册材料与 Kura 本次生成的私钥匹配后交付给 agent，再实现完整中心注册产品流程。

## 实施顺序及边界

1. agent 支持没有远程身份的本机运行状态；持久开关控制远程通道，与本机 socket/恢复初始化分离。安装激活方式须与现有观察模式、用户/组授权及升级契约一同修改，不回退为匿名接口。
2. 增加窄范围本机配置 API 和明确安装授权。Kura 共享 JVM 身份不获得任意 root、文件路径、unit、离线恢复或主机 reboot 权限。状态和修改均使用有界数据、预期 revision 和持久审计。
3. Java 服务适配和 GWT 中英页面：先启停/状态闭环，再独立的 Kura 后端密钥/CSR 生成、公开证书链校验、身份交付、连接检查及证书维护。能力不支持时如实显示。
4. 单元/契约/浏览器 → IDEA 完整隔离 Kura及原登录 → 现有 OrbStack → 现有 Parallels。旧三服务保持原包，使用新命名隔离夹具；旧身份/state/一次性令牌不重放。

本新增项属于 pending；Java 离线恢复、防火墙/NAT/早启、系统任务、一致备份恢复和正式发行继续保留原台账，未被此需求取消。

## 最新职责核定

2026-11-05 用户纠正并已接受：agent 永远保持运行；Kura 控制台负责生成私钥。先前“agent 生成私钥/CSR”的提案由本节替代，未实施的提案不得继续作为需求。生成端不依赖 agent；应用端通过可选、受限适配交付到 agent，不能以 Required DS 引用或启动等待把整个 Kura/GWT 绑定到 agent。该新增流程仍 pending，不能把本文件当成交付证据。

## 未安装agent的兼容性（用户最新补充）

原有Kura可以完全不安装agent并照常运行。原GWT的远程管理菜单按本机正式包安装标记和程序检测；未安装/不完整安装时默认隐藏，已安装后才显示。检测不依赖进程在线、证书配置完成或中心连通；停服时显示未运行，而不是当作未安装。Kura不等待agent。生成密钥接口同时检查安装条件，不能仅靠隐藏菜单授权。独立生成表示不调用agent服务，不表示向没有agent的安装开放无用途功能。此规则已加入当前实现，仍需验证。

## 网络方向已核定

用户明确保留 NM 系统后端，Kura 可不装 agent 独立使用；agent 添加远程及故障管理。见 `management-network-nm-decision-20261105.md`。本机配置入口不以 Netplan provider 为依赖，不与 renderer 切换捆绑。

## 当前实施与验收边界（2026-11-05）

Kura后端独立私钥/CSR、安装检测、条件菜单及状态/开关适配代码已经实现；bridge40/GWT72、完整GWT编译、IDEA无agent原登录、Orb真实安装标记/当前agent的状态读取及进程离线降级通过。不能把此进度扩大为证书导入、身份交付、中心完整认证或正式发行完成。具体范围、失败修复和系统状态见 `agent-remote-control-v1.md`。Parallels仅完成Rust本机沙箱7项，无Java，未执行GWT。

## 2026-11-05 本轮系统验证完成并按用户要求暂停

本轮最新核定见 [系统验证与暂停](reviews/gwt-agent-enrollment-system-20261105.md) 及 `evidence/gwt-agent-enrollment-system-20261105.json`。GWT空列表生产序列化与显式CA环境信任旁路已修复。真实IDEA完整Kura及Orb中英GWT交付/离线CSR/持久admin回执、Parallels最终agent严格沙箱通过；Parallels无Java，不代表实际Kura业务恢复。新夹具全部停止保留、VM均stopped，原Orb三服务原26/20及phase2-agent继续active。中心认证/远程开关与正式发行和原业务仍pending。等待明确继续，不重放一次性入口。

## 2026-11-05 恢复后的中心登记与通道验证

用户已恢复继续，整体active。最新实现、精确验证与运行状态见 [中心登记与mTLS闭环](reviews/gateway-registration-channel-20261105.md) 及 `evidence/gateway-registration-channel-20261105.json`。中心新增管理员双语公开叶证书登记；修复Java/SunEC最小PKCS8与ring兼容；原GWT开启→真实中心/Kura只读任务→关闭/重启保持OFF通过。36项既有身份/源材料不改。Parallels仅native严格沙箱/mTLS，无Java。新夹具全停/VM均stopped，原Orb26/20服务保持active。托管CA、身份运维和完整原业务/发行仍pending，不把此次通道验证计作Kura业务恢复。
