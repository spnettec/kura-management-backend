# 本机运行与远程通道开关基础

2026-11-05 实施增量，整体功能未交付。遵循用户最新核定：保留 NM/Netplan 的现有系统组合，实际隔离测试出现冲突再决定取舍；不预先移除 NM 或修改现网 renderer。Kura 可独立运行，菜单按 agent 安装显示，私钥/CSR 由 Kura 后端生成。

## 已实现代码

agent 可在完全没有远程身份环境变量时启动本机 UDS、状态及事务恢复；部分身份变量缺失仍拒绝，不把损坏身份悄悄当作首次注册。无身份时不生成 TLS 私钥，不连接中心。

`remote.status` 返回 enabled、bound、opaque revision、connectionState、activeRequests、recoveryRequired 和已配置的公开中心坐标。默认关闭。首次创建状态时可用安装配置 `AGENT_REMOTE_ENABLED=1` 显式保存启用决定；存在持久决定时它优先于环境。旧部署的正式安装迁移现有显式命令，见本文件末节；不得直接以新二进制覆盖既有运行包来隐式迁移。

`remote.set-enabled` 只接受 expectedRevision 和布尔 enabled。root/admin可调用；Kura通过已登记连接且另获 `AGENT_KURA_REMOTE_MANAGEMENT=1` 明确委托才可修改。该方法不接收地址、证书、文件路径或任意systemd操作。默认Kura只读状态；不能借此换中心或身份。配置变更先持久化再切流，旧会话revision不能在关闭后重新启用时恢复任务准入。

实际远程请求持准入permit直至执行/持久回执工作结束；期间关闭返回CONFLICT，不取消已准入副作用。通道关闭不停止进程，不清除既有任务/UNKNOWN或网络/firewall/recovery日志。待确认系统事务保持原恢复机制。

写开关意图保存在 `remote-management-pending.json`；中断/落盘不确定立即停止新远程准入，保留意图，重启后 status=RECOVERY_REQUIRED，新的set不能覆盖。包维护和身份变更守卫同样检查该意图及 `remote-management-resolution.json`；显式本机核定入口见下节，不能用删除日志作为恢复步骤。

## 已验证范围

- agent宿主95 passed，真实nft1 ignored未重复执行；严格Clippy/all-targets与fmt通过。
- Python packaging43通过，含新远程开关未决意图挡维护与保留测试。
- 当前宿主真实二进制/公开UDS，无任何身份或中心：UNBOUND、启用拒绝、保持关闭、额外字段拒绝、本机agent.status、正常进程重启和持久revision共7项通过。新夹具停止保留，未操作网络。

这些不是中心断流/真实远程写在途的系统验收。后续 GWT 和 Linux 增量见下节；无身份正式包初始化及原启用部署显式迁移、Kura生成身份的受限交付、证书/中心完整认证产品流程仍pending。不能将功能未写归为环境deferred。

详细记录 `target/remote-control-20261105/`。原Orb三服务active原包26/20，Parallels/Windows仍stopped，未提交推送或重放旧夹具。

## GWT 状态/开关适配增量（2026-11-05）

核心 API 新增可选 `RemoteManagementService`，只投影公开标量状态及 expectedRevision/boolean 开关。桥接包经固定 UDS 方法调用，不暴露路径、证书或任意系统操作。GWT 按 ADMIN/XSRF/安装条件检查，仅按请求查找服务；密钥生成不查 agent 服务。注册 Kura 默认只能读状态，显式委托后才可切换。agent 对每个连接角色返回 canControl。

GWT 中英页面增加状态读取、确认复选框及开关。未授权、未绑定、恢复闸门或在途请求期间禁止切换；失败清除已读 revision，需显式重读，不自动重试未知写。此增量完成适配代码，尚不等于绑定后真实中心启停闭环。

验证：bridge40、GWT72、agent宿主/Linux各95通过，真实nft1 ignored未重跑；GWT六种编译排列和严格Clippy通过。当前宿主真实UDS7项通过。IDEA最新完整282项运行集正常登录，无agent时菜单隐藏。Orb以真实既有包安装标记判断已安装，独立当前agent+完整Kura实际通过 GWT→Java→UDS 读取 UNBOUND、canControl=false；停止隔离agent后菜单保留、状态读取失败且原快照页面可用。没有生成/交付身份，也没有连接中心或修改网络。

系统验证发现桥接手写manifest缺少 `org.eclipse.kura` 导入，导致 `KuraException` 类加载失败；已补与API导出匹配的[1.8,2)范围，重新构建/IDEA/Orb实际状态读取通过。失败运行集、日志、缓存均保留。

Parallels实际零capability、ProtectSystem=strict/ProtectHome/PrivateTmp沙箱下：无身份本机状态、启用拒绝、关闭幂等、状态查询、服务重启revision保留及既有Netplan/agent配置哈希不变共7项通过。首次开发夹具位于/var/tmp被PrivateTmp隔离而启动失败，保留日志；新建/opt夹具后通过，未改生产unit。缺Java，GWT/Java未执行，不能代替业务恢复或正式包验收。

证据 `target/gwt-remote-control-20261105/`，新主机/Orb/Parallels夹具均停止保留。开关绑定后系统正向验收、包初始化/升级迁移、身份交付及完整注册产品仍pending。NM后端实际冲突适配尚未验收，当前不变更系统renderer。

## 无身份安装和显式旧部署迁移增量（2026-11-05）

正式包的postinst已接通干净本机初始化：私有agent.env只有本机状态/UDS及关闭的远程/写授权，不生成TLS身份；干净首装请求启动agent。重复配置保留原设置及决定，不擅自启动已停服务。缺少env但残留身份/数据库等现场拒绝当成干净首装。现有部署维护继续要求显式停服及原事务守卫。

新增 `init-local` 和 `migrate-remote-control --enabled|--disabled --operator ...`。旧已绑定配置缺少远程决定时启动检查拒绝，迁移必须保存明确bool、UUID revision和操作者，原env/证书/任务不改；已有决定不覆盖。旧绑定代次迁移和远程决定迁移分步完成。首次向本机未绑定安装导入身份保留本机选择与关闭决定，并留存local-enrollment-before.env；中断保留marker/备份并持续挡启动/重试。导入不启用远程管理，不生成密钥，也不代表GWT身份交付已接通。

宿主/Orb/Parallels当轮Python各50测试通过。Orb与Parallels分别在新前缀根和独立命名unit下执行实际setup/lifecycle代码，无TLS文件、状态、服务重启和重配置不隐式启动共6项通过。Orb另从最终deb提取ELF/setup和内嵌postinst实际重复该验证。**隔离范围纠正**：当轮Orb夹具请求了严格unit配置，但未检查其有效属性；最新实测Orb报告namespace隔离关闭，不能用请求属性证明其已生效。完整实际沙箱证据以Parallels为准，见下节。这里将enable --now映射到隔离unit的start以避免系统自启污染；**不是在原系统上执行dpkg首装/升级验收**。

新arm64候选 `0.1.0~local20261105-3` 已以SHA核定协议crate独立stage和Linux native build产出，原包及候选1/2保留。日志、源码基线、包和元数据 `target/unbound-packaging-20261105/`。尚未完成真实旧系统包升级迁移/中心连通与绑定后开关闭环，仍属pending；原Orb三服务未换包。Java/GWT/Rust业务源码本轮未改，沿用上一增量的完整IDEA/浏览器验证，无需重放已闭环入口。Parallels最终正常stopped，所有本轮新unit停服保留。

## 远程未决开关的本机核定入口（2026-11-05）

新增两个本机UDS方法，仅root或配置的admin UID可调用；已登记Kura连接即使拥有普通开关委托、甚至同UID为root，也不能调用。未向中心或GWT暴露核定权限。

- `remote.recovery-status` 空参数：只读返回observation，含expectedRevision及当前决定/未决意图/未决核定文件SHA256；没有的文件摘要为null。另有canResolve、activeRequests、recoveryRequired，不返回文件内容、私钥或路径。
- `remote.resolve-disabled`：参数为刚核对的observation。只允许空闲RECOVERY_REQUIRED状态，摘要/revision不匹配拒绝。它创建新的关闭决定，**不重放原unknown开关、不接受enabled=true、不删除旧意图、不清理其他事务**。使用新revision之后仍须单独读取状态并明确开启，才能重新连中心。

每次核定建独立 `remote-management-recoveries/<UUID>/`，在任何决定变更前持久化原决定/意图/先前核定及操作者选择；随后写独立resolution闸门、持久化关闭决定，原pending按rename归档并同步双侧目录，最后将resolution闸门移入同一归档。中断/错误保守挡新准入；重启若仍有任一闸门保持RECOVERY_REQUIRED，若最终闸门已归档则磁盘决定必为已持久关闭。原文件逐字节可追溯。存在任一闸门但缺少决定文件时拒绝当新装初始化。最多64个归档，达到上限拒绝，不自动删除旧记录。

I/O不确定的核定回复UNKNOWN，只能重新检查，不自动重发。日常运维CLI/UI封装尚未接入，当前是受限本机API。普通GWT开关和后台权限不放宽。

验证：Rust宿主/Orb各101通过，真实nft1 ignored未重跑；严格Clippy通过，Python host/Orb/Parallels各51。单测覆盖五个持久化边界故障、旧摘要拒绝、原件/其他事务保留、缺决定不初始化、symlink拒绝和Kura角色隔离。宿主真实UDS+IDEA完整282项Kura验证核定/重启关闭、桥接继续可用、原admin/admin登录/快照正常。Orb对pending-only和resolution-only新场景分别8项通过；**Orb有效ProtectSystem/ProtectHome/PrivateTmp/NoNewPrivileges属性为no**，其namespaces隔离未通过，只核实进程CapEff=0/NoNewPrivs=1及功能/文件哈希。Parallels真实resolution-only严格沙箱8项通过，有效属性均yes/strict，CapEff=0。

日志及新ELF在 `target/remote-resolution-20261105/`；历史候选包local20261105-3不包含本节新API/新guard。未换原包/未发布。全部新夹具停服保留、Parallels正常stopped，失败的测试注册合同/Orb隔离断言现场均保留。身份交付、完整认证、绑定后中心开关及原业务台账继续pending，不以本机核定代替业务恢复。

## 2026-11-05 恢复后的中心登记与通道验证

用户已恢复继续，整体active。最新实现、精确验证与运行状态见 [中心登记与mTLS闭环](reviews/gateway-registration-channel-20261105.md) 及 `evidence/gateway-registration-channel-20261105.json`。中心新增管理员双语公开叶证书登记；修复Java/SunEC最小PKCS8与ring兼容；原GWT开启→真实中心/Kura只读任务→关闭/重启保持OFF通过。36项既有身份/源材料不改。Parallels仅native严格沙箱/mTLS，无Java。新夹具全停/VM均stopped，原Orb26/20服务保持active。托管CA、身份运维和完整原业务/发行仍pending，不把此次通道验证计作Kura业务恢复。
