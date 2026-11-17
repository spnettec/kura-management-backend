# 默认 NM 安装、Core 代码保留与源码 cohort（2026-11-17）

这是继续实施中的局部检查点，目标 active。用户已明确授权使用现有积分；不是正式发行、完整 broker、provider 重加密或 12 父项关闭。

## 本轮改动

- networking 默认安装不再停止/禁用 networkd、时间或 DNS 服务，不改 cloud-init、Netplan renderer、NM 全局配置、系统 DNS、防火墙或快照。旧模板/兼容实现保留为代码，不自动执行接管。`99-kura-nm.conf` 仅保存到 Root libexec；新的包自有 polkit 规则不授予全局 unit-file、PAM、Bluetooth 或会话权限。NM/Modem/WPA 仍属于现有 Kura provider 的权限面，不宣称已完成最小 broker。
- 两个应用能力标志以 kurad 写入；拒绝符号/硬链接，原子替换、保留文件权限与组，重复相同值不重写。Root service drop-in 只接受未被修改的专用文件。安装/移除只 reload systemd，不切换系统网络权威。旧防火墙 boot writer 开启失败传播，unit 使用固定执行路径；旧 broad kernel 行为的全面替代仍 pending，默认安装不启用它。
- 旧包拥有的活动 `99-kura-nm.conf` 会被 dpkg 当作 obsolete payload 删除。当前候选先在 preinst 拒绝此转换，保留现用配置；仍需完整、可恢复的配置保留迁移。不能用删除文件或伪造配置版本绕过。原 udev 失败现场仍 half-configured；新默认代码移除的是合同禁止的自动 apply，不是伪造 udev 成功。
- Core 升级原先只保存 sibling JAR 与 `.data`，会遗漏新的 Root `libexec/addons`。现在三个包钩子嵌入同一源码策略，单独验证 Root、不可由 group/other 写、普通单链接文件与受保护目录；拒绝链接/特殊文件。私有复制经校验及 fsync 后用 Linux no-replace rename 发布。恢复不覆盖同事务已解包的新 addon，也不向其中补回已删除旧脚本；`.data` 不混入代码权限归一化。
- agent 导出新增 packaging/service 输入和完整 vendored protocol 文件指纹，schema 2 在打包前重新核对源码集。中心打包也核对完整源码；符号链接、遗漏/重复和漂移被拒绝。agent 的显式本地授权补齐 `kurad/kura.service`，保留独立 Netplan profile；要求 vendor/effective User 匹配、单一 profile、原 UID 与受保护 drop-in，既不加入登录用户组，也不增加密钥读取权。后续检查不依赖健康/运行中的 Core unit。产品 GUI 顺序仍未验收。

## 实际验证及边界

22 项 sibling 源码/Linux 原生方法、9 项变体方法、8 项 Core Root 文件系统方法通过。12 项 agent 源码/Java adapter 方法、10 项 Linux 中心打包方法及 5 项授权回归通过；授权回归的账户/systemd 由声明的替身提供，不能当作实际 provider 准入。

以 JDK21 从当前 Git 运行 Core 源码及发行构建（本次明确跳过 Java/IT 测试）。新 networking 实际包核对 9 个 Root/0644 JAR、26 份源码一致的代码/模板、8 条 API 声明及 Root unit/polkit，且无活动 `/etc/NetworkManager` 载荷。新 Core 包的 preinst/postinst/prerm 均逐字包含策略源码。

在既有、本轮新建的原 networking 失败夹具上执行了一次真实 Core 升级，未重新 provision。56 份 sibling Root 代码字节全部保留；12 份外部私有文件完整元数据不变，7 份 user/.data 文件内容、属主、权限保持。Core 从未启动，Root operations/startups 均为 0。准备过程中首次命令因包目标目录不存在而在打开归档前以 2 返回；原准备记录补齐目录/校验后，一次实际升级钩子返回 0。该有限文件保留检查不代表全移除/purge/平台矩阵或密文重加密验收。

当前源码导出已完成独立 Linux ARM64 console、agent release 构建及 Node24.15 前端 typecheck/build；协议 crate 校验和见 JSON，仅为本地产物，未正式发布。agent 后续 packaging 授权改动未改变编译的 Rust 输入，实际原生包尚待重新导出/组合核对。前端已有 1245.69 kB chunk 提示，本轮未改客户端源码。

原 3 现场的 7 个进程身份、38 个私有元数据和 6 份旧数据内容，以及个人 25 文件再次精确。没有旧 Root ID 查询、UNKNOWN 重发/换号，或已闭环备份、归档、恢复、轮换、900 秒验收重放。18 仓当次读回全部干净、共同本地 WIP 分支，无推送。

## 继续顺序

先完成旧活动 NM 配置的安全保留迁移，并继续同一失败现场；核对 agent 首装/后装时 Java adapter 的实际位置（当前仍在 Core 目录，安装顺序不能只凭推断宣称通过）。再组合当前 agent/中心/安装器/Docker 与 Core/GWT/networking 的完整六组件、provider、launcher、升级/移除矩阵。Root 配对不放宽；Main 11/13 caps、NNP0 与全部冻结 12 父项继续内部 pending。external deferred 仍仅实际 NM/cloud-init/物理硬件、amd64、磁盘满/硬断电三项。

具体 Git、包、校验和与现场见 [检查点](../checkpoints/20261116/default-nm-core-code-cohort-checkpoint.json)。冻结台账原文哈希继续为 `4e1aaca54eb94c3fc36266506c87a0781498aa797f0c18a01036f74a21584cc7`；G034 未封签。
