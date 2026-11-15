# 原 GWT 的首次身份交付与回执核对

2026-11-05 后续增量。Java bridge、服务端 RPC 和中英文 GWT 已接线，宿主契约和实际 Java provider→agent 验证通过；完整页面、IDEA 和 Linux 系统验收仍 pending。整体管理体系 active，尚未正式发行。

## 入口与权限

沿用按正式 agent 包安装显示菜单的规则，后端同时检查安装条件。未安装 agent 时原 Kura 独立使用；密钥/CSR、证书暂存和申请恢复不查询 agent 服务。只有最终交付和回执查询使用可选的 `RemoteManagementService`。

API 新增 getEnrollmentStatus 与 bindIdentity，使用默认 UNSUPPORTED 实现兼容旧 provider。实际 ManagementBridge 显式委托适配器，分别只调用 remote.enrollment-status、remote.bind；没有将通用 LocalAgentService.request 暴露给浏览器。

所有新 RPC 继续使用 ADMIN 权限、XSRF 和安装闸门。操作者从服务器会话 `Attributes.AUTORIZED_USER` 取得，并校验 UTF-8 长度/控制字符；浏览器没有用户名声明、文件路径或私钥参数。agent 仍独立验证本机连接角色/UID 和固定暂存目录授权，普通远程开关授权不能替代首次接收授权。

提交参数为预期远程 revision、选中申请 UUID、certificateStageSha256。RPC 先重新验证该申请的当前私钥/CSR/公开证书及精确 stage 摘要，再调用 bridge 一次。新的 staged() 读取不会补造密钥、暂存材料或申请。agent 的独立复制/校验/持久提交规则见 [live-agent-enrollment-v1.md](live-agent-enrollment-v1.md)。

## 回执与页面行为

只投影四个布尔状态 bound/legacyIdentity/recoveryRequired/canEnroll，以及完整的申请 UUID、stage 摘要、客户端证书指纹三项回执字段。类型错误、不完整回执、矛盾状态均拒绝，路径或私钥等额外字段不进入公开模型。

RPC 仅在返回身份已接收、无恢复故障，且申请/stage/客户端证书摘要全部匹配本次暂存材料时确认成功。其他或不确定结果不能作为成功；不会自动再发提交。

GWT 增加读取绑定回执、明确确认和首次交付按钮。提交中冻结申请及材料选择；成功后读取远程状态，但不自动开启远程连接。已有身份不能用首次交付替换，也不自动扩大普通开关权限。

选中暂存申请时显示固定中心地址、证书指纹和到期时间，并清空其他申请留下的公开证书输入。修改地址或输入新的证书形成未暂存草稿时，首次交付被禁用，须先校验暂存或重新读取申请；避免把输入框的新内容误当成已经核定的旧材料。

任何提交失败后，必须明确读取 agent 状态和绑定回执，再重新确认，才可能重新启用提交；切换页签或自动刷新不能解除该核对要求。RECOVERY_REQUIRED 持续禁用相应写操作，页面不删标记、不调用通用关闭恢复来清除身份故障。

首次本机接收保持 agent 运行、远程 OFF。离线材料/本机回执不证明中心登记、服务器可达、在线撤销或 mTLS 会话已经验证。原 GWT 账号、中心账号和 agent TLS 身份仍独立。

## 当前证据与剩余工作

- API 构建、bridge 44 项、Web2 80 项测试通过；包含真实 provider 的注册 UDS 请求、精确参数/公开投影、超时不重试、服务器会话操作者、未暂存/摘要不匹配/错误回执和安装/XSRF/ADMIN 合同。
- 完整 GWT login/denali 各 6 排列编译成功；制品包含新的 RPC 序列化模型和调用。此检查不是浏览器运行验收。增量构建保留了旧缓存脚本，当前模块入口由最终编译输出选择；尚未做正式发行的干净隔离打包。
- 全新 Java 生成的材料通过实际 ManagementBridge 服务 provider 交给新 agent，一次接收成功，第二次明确请求被拒绝；agent 重启后相同 revision/回执仍可读，远程 OFF，普通开关权限仍为 false，源材料逐字节摘要不变。两次宿主 agent 进程均停止，私有材料保留。这是实际 Java 类和 Rust 进程联调，不是完整 Kura/OSGi、真实 Web 会话或浏览器验收。
- 新完整 282 项 IDEA 运行集 `.phase1-dev/gwt-live-enrollment-idea-20261105-61fe4870` 及运行配置已准备。Mac 再次实测仍锁定，未启动；18392/18777/18778 保持关闭。上轮未启动的运行集同样保留，不能把它们标为已验。

下一步：解锁后执行新完整 IDEA Kura 和原 GWT 登录/兼容验证，再按顺序进入现有 Orb、Parallels。需要实际验证中英提交/回执、草稿挡提交、超时后核对、agent 离线和未安装兼容，以及真实 Web 操作者在持久回执中的对应关系。当前按环境顺序等待的这些验收均为 pending；既有 Parallels 缺 Java 的外部边界仍单独保留为 deferred。

仍须完成真实中心签发/登记/认证和绑定后开关、身份故障运维恢复与归档、轮换/转移和正式发行。原成功 INFO 审计可能被 WARN 过滤的历史缺口未被追溯修复；本轮原生 provider 的 agent 回执持久证据不能替代真实 GWT 成功审计验收。

原全部未交付业务仍保留：完整组件适用面、Java XML/加密/factory/核心与真实 Kura 业务恢复、防火墙适配/NAT/远程/早启、restart/reboot、一致备份恢复和发行。NM/Netplan 继续按用户核定现状，不修改 renderer。原 Orb console26/agent20 包、三服务和旧 phase2 agent 未改；Ubuntu/Windows VM 均 stopped。

源码修改前副本、18 仓状态、单元/构建日志、原生联调私有夹具指针和制品摘要保存在 `target/gwt-live-enrollment-20261105/`；证据见 `docs/evidence/gwt-agent-enrollment-20261105.json`。不得重放本轮 seed/accept/prepare 或旧一次性入口。

## 2026-11-05 本轮系统验证完成并按用户要求暂停

本轮最新核定见 [系统验证与暂停](reviews/gwt-agent-enrollment-system-20261105.md) 及 `evidence/gwt-agent-enrollment-system-20261105.json`。GWT空列表生产序列化与显式CA环境信任旁路已修复。真实IDEA完整Kura及Orb中英GWT交付/离线CSR/持久admin回执、Parallels最终agent严格沙箱通过；Parallels无Java，不代表实际Kura业务恢复。新夹具全部停止保留、VM均stopped，原Orb三服务原26/20及phase2-agent继续active。中心认证/远程开关与正式发行和原业务仍pending。等待明确继续，不重放一次性入口。

## 2026-11-05 恢复后的中心登记与通道验证

用户已恢复继续，整体active。最新实现、精确验证与运行状态见 [中心登记与mTLS闭环](reviews/gateway-registration-channel-20261105.md) 及 `evidence/gateway-registration-channel-20261105.json`。中心新增管理员双语公开叶证书登记；修复Java/SunEC最小PKCS8与ring兼容；原GWT开启→真实中心/Kura只读任务→关闭/重启保持OFF通过。36项既有身份/源材料不改。Parallels仅native严格沙箱/mTLS，无Java。新夹具全停/VM均stopped，原Orb26/20服务保持active。托管CA、身份运维和完整原业务/发行仍pending，不把此次通道验证计作Kura业务恢复。
