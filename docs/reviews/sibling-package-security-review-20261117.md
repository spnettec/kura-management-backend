# Sibling 发行包代码权限与安装失败传播

2026-11-17，目标 active。用户在实时查询周用量 100% 后，明确选择使用现有积分继续实施；旧的额度边界暂停已被本次选择替代，没有兑换 reset 或推送。

入口：[当前检查点](../checkpoints/20261116/sibling-code-ownership-checkpoint.json)、[完整台账](../checkpoints/20261116/OPEN-TASKS.md)。完整 12 父项仍为内部 pending，仅原 3 项 external deferred，G034 未封签。

## 已核对的起点

18 仓工作树、分支、HEAD、树指纹与上次检查点逐项一致。Core `de5f4e6901`、GWT `f135710e00`、console `4fc9828`；全部在本地 `wip/management-close-20261116`。原 3 现场的 7 个 PID/ticks/身份/能力集、38 份私有文件完整元数据、6 份旧数据 SHA256 精确匹配，未调用任何写入或业务 RPC。

个人 `.kura-dev` 的现用 25 文件已另作只读当前元数据/摘要基线；没有改动其配置、密钥或数据，也没有操作运行中的 `target/runtime`。知识库通过官方 Obsidian CLI、父工作空间 selector `f9b864007dcd8a44` 读取；检查助手首次默认 FIS5 路由失败后已显式修正，未使用其他 scope 代替。

## 本次源码修复

- 11 个剩余主 sibling 包的 49 处 JAR 输入映射改为 Root/0644。10 包的安装、卸载代码从 `.data` 移至各自 Root 所有的 `libexec/addons/<name>`。GWT 补齐同类卸载遗漏，未改变其 Java/页面产物。
- 12 包使用一致的受限注册表更新器。只接受固定安装根和合法 sibling 名；检查 framework 及全部规范父目录的 Root 所有权和写权限；拒绝符号链接、硬链接、不安全锁及其他所有者。串行化更新，以同目录 Root 临时文件替换注册表，保持已有顺序；不对旧 daemon 所有 inode 提权 chown/chmod。postrm 内嵌相同代码，dpkg 删除载荷后仍可安全移除单行。脚本、tee 或日志接收方失败均返回非零。
- Networking 单独拆分 4 份 shell 代码、Python 模块和 named/iptables/snapshot 安装模板，均进入 Root 代码目录；Python 不再从 `.data` 加载模块。实际运行 `.data`、用户快照及既有备份保留。首次生成的可变输出使用应用身份；不批量修改 `.data` 权限。
- Networking 安装、卸载改为同步执行，失败可传回 dpkg；仅在安装成功后注册；缺少 udev 时在主机配置前拒绝，实际 apply 错误保留到日志。缺少可选 systemd unit 与执行失败分别处理。当前源码的升级分支不重新配置网络、不读写加密快照、不覆盖原系统备份；升级 prerm 不再进入网络撤销，也不从任意 Java PID 推断 Core 或删除 `/tmp/.kura`。安装代码和模板不再安装后自删。
- Cloud 的应用配置更新使用 `kurad` 身份，拒绝链接目标，成功后才注册，避免 Root 通过用户目录重定向写入系统文件或将失败安装登记为可用。

没有放宽 Root 配对、启动锁或 UNKNOWN 门禁，没有删除旧钥、扩大 agent 原钥读取权或增加 Main 权限。完整 broker、全部 provider/资源的权限收敛仍待完成。

## 当前验证

使用 Maven 3.10.1、Temurin 21.0.13，从当前源码完成 11 个 sibling 所需 reactor 及 Debian 包构建；Java 测试未在此构建阶段执行。GWT 只重打安装器包，其生产 JAR SHA256 仍为已接受 `f135710e00` 的 `a2306b5a04a322a8dcff48f1a47120821dae1e53e8b9b73a1e73534a69035e3f`。

实际 12 包逐一读取 Debian control/data 归档：64 个 JAR 均 Root/0644，56 份安装代码/模板与源码原字节及声明权限匹配；54 条 Crypto/配置 API Import-Package 声明与所属源码清单一致。这证明本次载荷与声明范围，不代表全部 provider 实际解析、启动或重加密通过。

[源码及 Linux 原生检查](../../scripts/test_sibling_package_security.py)共 19 个 test methods 通过，包含各包 subtests：新装/重复注册、旧顺序、并发、Core symlink、符号/硬链接和可写目录拒绝、原子替换失败、卸载后无载荷、升级 hook、Cloud 配置链接、Networking 加密快照升级及同步子失败。使用已提交 Dockerfile 从 Ubuntu 24.04 构建新测试镜像；这些检查使用真实内核权限/文件系统，日志接收与 systemctl 结果有明确模型，未启动 Main 或执行密钥业务。

另以 Git 中原 Artemis 安装脚本复现了“通过 symlink 写入并返回成功”，以原 Networking preinst 复现了“代码升级拒绝加密快照”的反例。仅在无密钥的新临时容器中执行，没有复用历史业务验收或现场。

复现源码检查：

```sh
python3 scripts/test_sibling_package_security.py
python3 scripts/test_sibling_package_security.py --container
```

[实际包检查器](../../scripts/inspect_sibling_packages.py)接收显式 `.deb` 路径与 `--report`；不安装或操作网关。新包的路径、SHA256 和源提交见检查点，不要求还原旧 target 或旧证据链。

## 仍需继续

新的实际 Ubuntu 24.04 ARM64 / PID1 systemd / JDK21 fixture `8d5c8e8f493c818c9fcc1800df51af6a765482ede93fc68f988d76ea073daee2` 已安装 Core，并完成 11 个 sibling 的实际配置；Networking 首次安装明确失败。实际 NetworkManager 已 active，Netplan 因缺少真实 udev 控制通道失败；原脚本曾隐藏 stderr。沿同一现场仅诊断一次原 Netplan 调用，随后安装带提前 udev 检查的新候选 `fd1f0b7`，在再次进入主机配置前明确拒绝。19 份原私有/用户文件的字节和完整元数据保持，Main 启动与显式密钥请求均为 0，Root operations/startups 均空。首次观察器曾错误预期诊断出现在 stdout；实际 tee/systemd-cat 路径的原文件及 journal 已只读核实，原观察失败与首次安装日志均保留，没有重发该安装尝试。

该 fixture 仍保留，Networking 仍 half-configured，完整安装矩阵没有通过；不能因观察超时重新 provision，也不能把容器 udev 缺失移入 external deferred。当前 12 个源码候选的载荷检查与原现场最初安装的版本分开记录：Cloud 后续仅调整失败前不注册的顺序，Networking 后续为提前检查及可见错误，实际已安装版本见该现场 dpkg 状态与原 inputs。

既有 Parallels Ubuntu 26.04 ARM64 只读检查显示 udev 可用、两个网络后端 active，同时已有 `/etc/kura` 和 agent 数据，且没有 JDK21。没有往其现有根目录安装新包或改网络后端；正常恢复到原 stopped 状态。后续真实主机矩阵仍须隔离这些既有配置与数据，不能盲目采用该环境。

历史 Networking 包的首次升级仍须单独处理：dpkg 在新 preinst 之前调用旧 prerm，本次新源码不能追溯修复历史已安装脚本。旧脚本所在 `.data` 的不可信代码、旧 host 修改/备份以及中断恢复不得通过强制忽略失败、Root 放宽配对或重新装环境来绕过。

完整六组件、firewall-only/netplan 兼容变体、全部 provider/launcher/模式、升级/移除/purge、凭据连续性及完整 12 父项继续内部 pending。实际 NM/cloud-init/物理硬件、amd64、磁盘满/硬断电仍是仅有的 external deferred。Main 既有 11 ambient/13 bounding、NNP0 没有因本次安装器修复而关闭。

原 UNKNOWN、startup.stop 与全部已闭环备份、归档、轮换、恢复及 900 秒续期编号均未重发、换号、手动结算或在其他 Root 查询。冻结台账原文 SHA256 保持 `4e1aaca54eb94c3fc36266506c87a0781498aa797f0c18a01036f74a21584cc7`。
