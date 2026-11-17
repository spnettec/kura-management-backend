# 原生中心真实 GUI 引导与独立工具升级（2026-11-17）

用户授权现有积分继续，目标 active。本阶段完成新 ARM64/systemd 主机从已安装工具包的 Cockpit/PAM 入口，到唯一中心包安装、显式启动、凭据核对、外签 CSR、专用空库初始化及管理员登录的局部实际流程。完整 OS 图形包安装/GTK 启动器、人工证书信任体验、六组件及全部 12 父项仍待完成。

## 发现与修复

默认工具包虽然依赖 Cockpit/pkexec，未显式依赖 stock Cockpit bridge 使用的 sudo。不安装推荐依赖的空主机已正常 PAM 登录，切换管理员权限时却明确返回找不到 sudo；当时没有中心程序、中心状态或已受理产品写操作。新增默认依赖 sudo，显式 headless 包不携带该 GUI 依赖，不添加免密码规则或额外授权。9 项 Linux Root 打包/源码绑定检查通过。同一主机更新工具及依赖后，实际密码认证进入管理员模式，核定目录读取成功。相关机制见 [Cockpit 权限说明](https://docs.cockpit-project.org/cockpit-guide/main/guide/privileges.html)。

同主机名的中心 URL 被现有会话隔离检查正确拒绝，保留这个限制。把不同主机名要求加入中英文持续帮助，并用 `aria-describedby` 关联地址输入框。采用 modern-web-guidance 的 forms 指南：`keep critical format instructions`，以及输入与帮助文字的程序化关联；没有扩大全站样式改造。实际浏览器中英文文本、关联及截图通过。

当前工具包为 `0.1.0~cohort20261117.z004.b2e8afc-1`。其独立更新在中心运行时完成，7 份私有文件完整元数据、Main PID3135/ticks16649159、NNP1 和空 capability 集全部精确，无 stop/start/restart 或迁移。初次打包的隔离 Python 导入方式及已存在输出目录均在构建前拒绝，按脚本约定与新输出子目录纠正；没有绕过输出保留门禁。

## 实际新现场

主机 `5a4b2b32d7a5…` 只创建一次，来自当前声明的 Ubuntu24.04 systemd 基础镜像，不含旧 Kura/中心数据。测试准备 OS/PAM/Cockpit 前提，中心安装、启动、CSR 和初始化均通过实际 GUI；OS 的工具包安装及 stock Cockpit socket 准备不作为 GTK/GDebi 产品流程通过。浏览器为独立 Chrome157 会话，只为这台主机做固定端口 TCP 转发，TLS 端到端；未更改全局代理、DNS 或系统信任。测试浏览器在独立核对实际 TLS peer 后接受本次测试证书，不代表人工浏览器信任体验验收。

- 软件安装原编号 `47f7062c-d1cc-48fa-8c68-c38c6c070ae9`：一次实际上传、一次确认安装。原 dpkg 成功回执、匹配 postinst 验证及 9 个 Root/精确模式/摘要的必需载荷分别核对，原维护 COMPLETE、冻结解除。中心保持 inactive/PID0/NRestarts0，随后才从界面显式启动。
- 安装身份 `29efc3d9-7093-4696-9fee-ffe03ef680b2`：可信 Cockpit 提供的 bootstrap 指纹与实际 TLS peer 精确匹配，凭据通过浏览器下载到私有目录，令牌未显示/记录到 DOM 或公共材料。首次设置页使用与主机管理页不同的主机名，验证后上传凭据解锁。
- 服务器 CSR `5a1cad6e-48a7-4ef4-a7d3-9970cfd116e7`：中心生成并保留私钥，只下载公开 CSR。外部测试 CA 签发；CSR 签名、叶证书公钥/SAN/用途及 CA 签名独立核对，CA 私钥始终位于中心之外。证书原上传 `4b54bdca-d1b0-4664-9174-7b5cef0cab4f` 的回执被 GUI 核定，未重新上传或更换 CSR。
- 外部 PostgreSQL `40eca01d6c35…`/独立新卷：实际版本16.15，应用角色 `console_native` 为专用空库 owner，无 superuser/createdb/createrole。初始化前从中心主机核对可达且 0 应用表；组织、数据库及证书材料全部由 GUI 填写。
- 初始化原编号 `8d038ae0-07b6-4b79-9428-be2e0827074e`：仅发一次。TLS 切换后 GUI 保留未核定结果，没有重发、换 ID、重置或另建安装。核对实际 peer 为签回证书后，在同一物理中心的发布地址登录；通过实际 GUI 的只读 GET 得到原编号 `APPLIED`、原 origin 与 `observedConfiguration=true`。结论来自原权威回执，未用监听、PID 或表已存在推断结算。

最终发布地址 `https://192.168.215.3:18443`（本次 Docker 私网），gRPC 监听 `0.0.0.0:18444`。实际中文网关总览登录成功，实时连接正常，网关/任务 0。数据库有 14 个应用表，应用账号仍无上述服务器管理权限。Main 全程 PID/ticks 不变、NRestarts0、UID999、cap0、NNP1；现有 29 个私有文件仅作散列/元数据记录，原 7 份 bootstrap 文件完整元数据仍精确。该主机不含 Core Root key journal，也未安装 Kura/agent；这不等于 gateway mTLS 或完整六组件/provider 准入。

本阶段中心程序和 React 来自本轮 `6d521a3` 源码 release 构建，后续 Git 差异确认 Rust、Cargo、React 和协议编译输入未变。独立工具的 packaging/Cockpit 增量分别重新导出完整源码并重新打包；不要求恢复旧中间产物。正式当前六组件 joint generations、协议 crate 正式发布、跨版本/客户非空数据库仍是内部 pending。

## 保留与继续

原 3 现场的 7 个进程身份、38 私有完整元数据、6 份旧数据与个人 25 文件再次精确；18 仓在本阶段文档写入前均为共同干净 WIP 分支，无推送。新主机、数据库卷、实际 bind 输入及私有凭据/测试 CA 材料继续保护保存，均未放入 Git。旧 UNKNOWN 未重发/换号或跨 Root 查询；旧闭环备份、归档、轮换、恢复和 900 秒验收未重放。

上一阶段 networking 属性 inode/ctime 偏差仍保留，不被本次结果覆盖。Core Main 11/13 caps、NNP0、完整 privileged broker/provider 重加密、networking purge/运行、Core/networking/GWT/agent/console/Docker 全模式与安装升级移除、R017 agent 维护顺序、客户数据库、多人 CA custody/quorum、网络/API/上游及全部冻结 12 父项继续内部 pending。external deferred 仍仅实际 NM/cloud-init/物理硬件、amd64、磁盘满/硬断电三项；G034 未封签。

具体 Git、包摘要及脱敏观察索引见 [检查点](../checkpoints/20261116/native-console-empty-host-checkpoint.json)。这些新已完成操作保留原回执，不为了后续恢复上下文而重放。
