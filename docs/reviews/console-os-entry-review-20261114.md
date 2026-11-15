# G029 / V57：系统图形包安装与应用菜单启动审查

2026-11-14，完整改造目标 ACTIVE。本批实际完成一次标准系统图形安装和一次应用菜单启动；**浏览器停在主机证书信任页，完整认证入口与原生系统全 GUI 部署仍为内部 pending**。

同一 R013 shadow 修改 `packaging/build.py`、`packaging/installer.py`、`packaging/test_installer_package.py` 和平台说明，新增 `packaging/desktop/com.yofc.KuraConsoleSetup.desktop`，共 5 个源码输入。原管理源码未应用、未提交推送。

## 实现与验证

- 默认工具包依赖 `cockpit-ws`、`cockpit-bridge`、`cockpit-system`、`xdg-utils`，不依赖 Cockpit 元包。实际 GDebi 审阅由初版 178 个依赖降至 74 个；初版仅审阅，未执行安装。补齐 `Installed-Size`。
- 安装后提供中英文应用菜单入口。普通用户点击后由默认浏览器打开固定 `https://localhost:9090/yofc-console-setup`，不打开终端、不带令牌、不变更中心运行状态。桌面文件纳入 15 个 root 保护的工具负载 pin；缺失、篡改或伪造 desktop provenance 被拒绝。旧 14 文件工具包及显式 headless 变体仍兼容。
- 最终源码的完整 Linux root 测试 **90 项通过**，含默认/无桌面归档、依赖、受保护负载和篡改拒绝。React/Rust 运行输入没有变更；保留前批 47 项浏览器契约及普通 Linux binary 的 135 项运行输入证据，未重跑已闭环现场。
- 实际工具包 `0.1.0~v57-osentry20261114-2`，SHA-256 `4a164436a5463749b849766873c8cec3dee6fcc309090121265eff238ac43121`。其批准目录仍引用 G028 实际核心包 V56-3；本批不安装核心、不配对、不初始化。
- 独立 Ubuntu 24.04 ARM64 轻量桌面容器中，从文件管理器打开同一 DEB，实际 GDebi/polkit 选择管理员、输入密码并完成安装。成功一次性授权、GUI 完成提示、实际 dpkg 唯一安装记录及安装后 15 文件摘要/权限互相印证。应用菜单中真实出现 `YOFC Console Setup`；激活后实际非 root Epiphany 进程打开上述 URI，WebKit 沙箱保留。

## 保留的失败与边界

测试桌面最初缺 SVG loader 和 GTK2 无障碍组件。先前“窗口消失即授权前终止”的判断被实际两个等待认证窗口纠正，旧分类文件不覆盖。两个请求从未提交密码，随后由授权组件注销明确以认证失败结束，系统日志保存 `Not authorized`；在此确定拒绝之后才重新申请授权。最终管理包只发生一次安装。组件修复仅在新测试夹具内，未放宽 polkit/PAM 策略。

容器保留 stock `policy-rc.d=101`，安装后 Cockpit socket 为 enabled/inactive。一次实际系统 GUI reboot 请求在等待管理员认证期间显示 `Timeout was reached`，认证稍后成功，**原 reboot 结果保持 UNKNOWN，不根据 PID/状态推断未派发，不再重发**。为独立验证菜单启动，监督者随后单独启动夹具的 Cockpit socket；这项 CLI 准备不能证明整条无命令行部署通过。

默认浏览器到达 Cockpit TLS 信任门。实际 peer 公共证书 SHA-256 `38e95d549d9d2e9b95350b6f9da14b1a6c7db47e2889aafe9560b4f5f7b56432` 与该主机 root 保护的公开证书精确匹配，但没有在浏览器中绕过 TLS、关闭安全检查或提交登录凭据。GUI 主机证书信任、认证后设置入口和完整原生系统部署继续 **内部 pending**，不增加 external deferred 类别，不冒称本批首次设置链路已通过。

本批 5 个 Docker 对象均已停止并保留 writable layer、日志、桌面 profile、失败和包文件。4 个构建/测试对象退出 0；桌面容器由监督者关闭，退出 130、无 OOM、restart=no。此关闭不核定原 GUI reboot，也不算正式 OS shutdown 验收。用户确认已清理/重建的旧中心、PG 与旧 60 个验收容器不重建；新夹具独立记录。

## 源码权威与续接

证据根：`target/nm-role-authority-20261112/registered-runtime-v52/console-os-entry-v1/`。当前 `source-checkpoint-os-entry-v57.json` SHA-256 `806b289099d85e35f2252cc1e98c149df183a25b505e7370afcb3cd209fc4a8f`，**2197 source + 4 fixture**，距最后完整 CODE V51 为 121 输入增量。候选 121 existing + 195 new = 316 记录，含 3 fixture-only，即 **313 production/test 路径整体 UNAPPLIED**；路径数不是剩余任务数。精确快照用于续接/审查/回退，不代表整体封存或正式发行。G025 原仓 40 路径及当前 580 上游 pin、89 absent 保留。

重点证据：`actual-gui-evidence-v57.json`、`package-installed.png`、`native-admin-authorization-journal.txt`、`actual-dpkg-log.txt`、`actual-installed-critical-files-readback.txt`、`menu-security.png`、`after-menu-entry-processes.txt`、`native-os-reboot-unknown.json`、`owned-containers-final.json`、`linux-root-v2.log`。组件 `final-binding.json` 绑定本批证据；不是全量 CODE seal。

原 Parallels 注册现场仍为 V52：boot `4db3daf0-dca3-4bc5-bc9e-01cdd088e45c`，五 PID 2799/3635/81713/82137/59158、agent `affa64e2` 精确。原 `767863a9` 的唯一 startup.stop UNKNOWN 与 `a87d94d5` intent 保留，仍无 stop receipt/publication/start；本批夹具 reboot UNKNOWN 与之分开。18 仓工作树/HEAD/branch/index/staged/refs/remotes、共享 API522 只读保全，不提交推送/reset/clean，不重发 UNKNOWN，不重放 CLOSED 首次设置/TLS/GWT 三次保存/V36v1。

下一步继续 GUI 主机证书信任和完整原生系统部署，再推进中心地址/DB/CSR 异常原回执核定/续期、agent 独立身份/CSR/续期/通道能力。完整 owner/网络/退役、跨版本 DB/非空加密任务/备份、多管理员外部 CA custody quorum/bulk、正式 sandbox/polkit/协议0、14组/7API2/R017/整体 guarded apply/正式API与协议校验/joint generations、G021 LDAP、G022 Go31 vet/非JavaPOM/regen/.NET 台账不缩减。仅原 NM/cloud-init/physical、amd64、disk-full/hard-power 三组 external deferred。
