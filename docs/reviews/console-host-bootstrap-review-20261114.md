# G030 / V58：本机图形启用、公开证书与管理员入口审查

2026-11-14，完整改造目标 ACTIVE。本批补齐 G029 未通过的本机页面启用、公开证书保存和认证后入口。实际证据由 Ubuntu 24.04 ARM64 图形容器及宿主 Chrome 两段组成；正式原生 OS 全 GUI 交付、sandbox 和完整改造仍为内部 pending。

## 实现与验证

同一 R013 shadow 修改 5 个已有输入，新增 `packaging/host_ui.py`、其测试及固定 polkit policy，共 8 路径。菜单现在打开普通用户 GTK 窗口，提供读取状态、显式启用安装页面、授权读取公开证书、保存公开证书和打开浏览器。默认依赖补齐 pkexec、GTK/PyGObject 和 dbus-user-session；显式 headless 不引入 GUI。18 个 root 保护负载 pin 和协议版本覆盖新入口，旧 14/15 文件工具包继续兼容。

授权仅为固定 helper 的 `auth_admin`，不缓存授权，不引入自定义宽松规则。helper 只允许只读检查或固定 `systemctl --no-ask-password start cockpit.socket`。不启用/重启中心，不生成中心身份，不读取或导出 CA 私钥。用户 UUID 在申请授权前落盘；root intent/registry 在执行前落盘。缺成功命令回执保持 UNKNOWN，同一编号或新编号均不能绕过。服务当前 active 不能代替原回执。

完整 Linux root 测试 **109 项通过**。审查当前图谱中的 HostUi、UserRequests、GUI、CLI、打包及负载验证；测试覆盖丢失回执、持久化失败、注册记录缺失、伪造回执、用户状态损坏、授权拒绝、TLS peer/公开证书核对和旧包兼容。Rust/React 运行代码未变：本批明确绑定并核对 145 个更宽的 Rust/React/migrations/OpenAPI 输入；旧组件 135 项口径保留为历史。47 项浏览器契约按 G029 最终源码逐文件确认输入未变，未重复运行旧现场。

最终工具包 `0.1.0~v58-hostui20261114-4`，SHA-256 `f5bf1d03fdfb624e957620f53ae6962cc2ef35b3db13916e21c625e82091e471`。真实 GDebi/polkit 安装 v1 后，针对本批发现的问题依次升级 v2/v3/v4；各版本一次，安装日志及 GUI 结果保留。最终 18 文件摘要和 root 权限精确匹配。批准目录仍独立引用 G028 实际核心 V56-3；本批核心安装、启动、配对和初始化均为 0。

## 实际界面证据与保留的失败

容器保留 stock policy-rc.d=101，安装后 socket inactive。通过菜单窗口实际点击启用并完成一次管理员授权，原请求 `d0dcf1e5-0691-4962-94ef-ee6f1eb52781` 获得命令 returnCode=0 成功回执。root registry/原操作和用户请求在后续三个工具升级、会话刷新及查询后字节不变，始终仅一个原启用操作。本批没有监督者 CLI 启动 Cockpit socket，也没有原生系统 reboot 请求。

最初保存对话框落在不可写的 `/`，实际保存失败。改为个人 Downloads/Home 默认目录，并在模态确认后再次核对原请求与证书，O_EXCL/NOFOLLOW 创建 0600 文件；实际 v3 窗口保存成功。公开证书 DER SHA-256 为 `71c8c584283067a00401862ace1dd6944f5d51eddf889048153fad811bc482b9`，PEM 文件字节 SHA-256 为 `b6f0fbba31898a1a54fa8af4e35073d3ace3c045c2d34d0a28cfa9cf4da5ac6a`；两种摘要口径区分。文件只含 canonical 公开证书，无 PRIVATE KEY。

默认 Epiphany 真实打开固定 HTTPS URI，停在 TLS gate；不把它计为认证成功。另用真实安装的宿主 Chrome、独立私有 profile 和本地映射端口 29502，通过浏览器自身可见的证书详情提取公开证书并精确核对上述 DER 摘要，再由实际页面进行本 profile 的单主机信任。没有 TLS-ignore/关闭证书检查标志、全局 CA 导入或浏览器安全覆盖 API。CDP 证书查询返回空数组的早期诊断不算证书不匹配，原记录保留。

首次 Chrome PAM 登录成功，但管理员认证通道断开。日志明确证明是认证阶段失败、尚无应用命令。实测缺少 dbus-user-session，sd_bus_default_user 返回 ENOENT/空指针；Cockpit 出现 EINVAL，sudo 认证会话失败并关闭。内存约 650 MB，当时及最终均无 OOM，不能归因于内存不足。最终工具包通过 GDebi 自动安装该组件，并在窗口显示需要重新登录的提示。

安装后旧用户管理器未自动获得 bus，监督者仅刷新本轮测试容器的 UID1001 桌面 PAM 登录及用户管理器；没有直接启动用户 bus 或 Cockpit。新用户管理器实际创建 bus，底层查询返回有效连接。容器旧 session46 仍有 closing/X99 残留，刷新不是正式 OS logout/shutdown 验收，原始日志保留。真实浏览器新 PAM 登录后显示密码授权对话框，唯一提交成功，页面显示“管理员模式”，sudo 日志记录实际 privileged bridge。一次只读“读取核定版本及主机状态”正确显示核心尚未安装、核定版本 V56-3、核心摘要及启动/凭据操作禁用。等待脚本曾误用“管理权限”而超时，随后只读发现实际翻译为“管理员模式”；没有再次提交认证。

## 保全与续接

证据根 `target/nm-role-authority-20261112/registered-runtime-v52/console-host-trust-v1/`。源码检查点 `source-checkpoint-host-trust-v58.json` SHA-256 `65c7a96c658a912efd87c19a93bbfcf18f528864ed1345a77913d2389f2e18c9`：**2200 source + 4 fixture**，距最后完整 CODE V51 为 124 输入增量；候选 121 existing + 198 new = 319 条，含 3 fixture-only，**316 production/test 路径仍整体 UNAPPLIED**。路径数不是剩余任务数。仅封存当前组件证据和可回退源码快照，不宣布全量 CODE seal、正式发行或整个改造完成。

主要证据：`v4-installed-critical-files.json`、`v4-native-before-session-refresh-snapshot.json`、`actual-ui-public-certificate-download-v3.json`、`actual-browser-certificate-pin.json`、`original-browser-authorization-failure-classification.json`、`v4-new-session-readonly-bus-and-peer-observation.json`、`v4-authenticated-admin-app-view.json`、`v4-actual-readonly-catalog-result.json`、对应截图、`actual-final-host-readonly-observation.json` 和 `linux-root-v4.log`。最终组件 binding 另绑定源码、打包、原失败、日志与这些证据。

9 个新 Docker 对象已停止并保留全部层、日志、包和 profile：8 个构建/测试退出 0，桌面监督者关闭退出 130/noOOM/restart=no；私有 Chrome 正常关闭，其他浏览器不受影响。原 owned OrbStack VM 仍 stopped。用户确认清理/重建过的旧中心、PG 和 60 容器按当前现场处理，不恢复、不重放。G029 的原 GUI reboot UNKNOWN 和原注册现场 startup.stop UNKNOWN 各自保留，均无重发。

18 仓及 G025 原仓 40 路径按本批 preservation 核对；原 Parallels boot `4db3daf0-dca3-4bc5-bc9e-01cdd088e45c`、五 PID 2799/3635/81713/82137/59158、agent `affa64e2` 精确。原 `767863a9` 的 stop intent `a87d94d5` 未变，仍无 stop receipt/publication/start。未提交、推送、reset、clean 或安装共享 API。

下一步继续正式 GUI 交付和平台边界，推进中心地址/DB/CSR 异常原回执核定/续期、agent 独立身份/CSR/续期/通道能力。完整 owner/ordinary/recovery/网络/退役、DB/非空加密任务/备份恢复、外部 CA 多人 custody quorum/bulk、正式 sandbox/polkit/协议0、14组/7API2/R017/316 路径 guarded apply/正式API与协议校验/joint generations、G021 LDAP、G022 Go31 vet/3 非JavaPOM/regen/.NET 台账保留。只有原 NM/cloud-init/physical、amd64、disk-full/hard-power 三组 external deferred。
