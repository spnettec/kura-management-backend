# agent 先装、独立代码路径与 Core 配对加载（2026-11-17）

本次继续实际实施，目标 active，使用用户明确授权的现有积分。这里只关闭一个新的 ARM64/systemd 安装顺序子场景；完整六组件、provider、launcher、升级移除矩阵及冻结 12 父项仍待完成。上一份默认 NM/cohort 检查点是当时状态，本文更新其中 agent 路径及原生包尚未组合的边界。

## 已实现

- agent 的 Java adapter 移到包自有 `/usr/share/kura-management-agent/java/`，不再先占用尚未初始化的 Core 目录。元数据绑定 adapter 的 SHA256、OSGi 身份及 Core loader 的协议/精确 SHA256。agent 包在已配置 Core 上提前核对 Root 保护的实际 loader；独立 agent 安装不要求 Core 存在。
- Core 的可选 loader 验证 Root、普通单链接、不可由 group/other 写的代码/元数据与固定路径，逐项验证自身及 JAR 指纹/身份，然后只加入一次 5s 启动项。失配时跳过可选适配器，Core 保持独立；旧 `siblings/management-agent` 不作为绕过入口。没有增加 agent 的 Core 启动依赖，没有放宽原钥初始化或 Root 配对检查。
- agent 服务用只读临时 `/etc/kura`、`/var/lib` 父目录，显式绑定自身 StateDirectory，覆盖“agent 启动时密钥路径尚不存在”的情况。原 `InaccessiblePaths=-路径` 会跳过当时不存在的目标；相关语义见 [systemd.exec](https://www.freedesktop.org/software/systemd/man/255/systemd.exec.html)。实际 fixture 验证了随后创建的 Core 原钥与日志 HMAC 仍不可见，自身 SQLite 保持原持久文件。其他可选 provider 路径仍需完整矩阵核对。
- 显式授权支持 `kurad/kura.service`，不加入登录用户组。实际命令行曾因旧 argparse choices 拒绝 `kurad`，已修复公开 parser 并验证两种受支持 profile；默认仍保留兼容 `kura-netplan`。实际 Core service drop-in 和有效 SupplementaryGroups 已核对。
- 当前源码组合出 Linux ARM64 agent、console 和含 Cockpit 的 installer deb，所有者为 Root，包中不带身份/现用数据。Rust ELF 来自本轮独立源码构建；后续 packaging-only 导出与完整编译输入逐项一致。协议 crate 只是本地产物，尚未正式发布；console/installer 尚未实际安装验收。

## 同一实际现场的结果

新 fixture `ebdbf5645723…` 只创建一次，原失败与修复续接全部保留。旧 agent 的 Core 内 JAR 导致第一次 Core preinst 正确拒绝，未生成 canonical key。原维护记录 `b96c7cef-85af-4723-944f-a100c220c7a3` 没有删除、换号或重建。

第一次 agent 升级被 inactive guard 拒绝，旧进程与数据保持。只读核对该新现场未绑定、remote OFF、未授权 writer、执行/结果表为空后，明确停止该新 agent 并继续原包意图；没有弱化 guard，产品 R017 GUI 维护顺序仍 pending。旧 JAR 由包升级正常退役，Core 目录自然不再存在。原 Core 安装随后完成；一次 APT 前端本地路径错误发生在钩子之前，纠正命令后继续同一安装，没有重做 provision。原维护回执完成，HMAC/admission/sender 文件元数据保持，生成新的测试 canonical key。

在该现场继续真实 Core loader 配对升级，6 份凭据/TLS/HMAC 文件完整元数据不变。以 kurad、NNP=yes、空 bounding set 的一次实际 classpath 生成检查得到唯一正确 adapter 路径。原公开授权命令的 parser 错误也保留；修复包后原授权意图完成，agent 激活成功。

最后一次普通 Main 首次启动返回 0：Main PID5778、agent PID5450、key service PID5516、recovery PID5048。OSGi bundle216 和主 bridge DS ACTIVE，ConfigurationService、ConfigurationRevisionService、CryptoService 均绑定；无 networking 的 health DS 缺少必需网络服务，未冒记通过。实际 kurad UID997/授权 socket group 只读 UDS 请求得到 `agentAvailable=true/kuraAvailable=true`。这不是日志化换钥启动或 900 秒续期验收。

6 份私有文件完整元数据保持；agent 命名空间看不到新 canonical key/HMAC。Root operations/startups 仍为 0。Main 仍是 11 ambient/13 bounding caps、NNP0；Root 文件所有权、命名空间局部检查和 bridge 连接都不能替代完整 privileged broker。

## 验证、保留与未完成项

6 项 Root/Linux loader 方法通过；15 项选定 agent artifact/profile/loader 方法通过，公开 parser 修复后另一次 4 项选定检查通过，不将重复方法相加宣称新覆盖。Core 发行包从 JDK21 当前源码构建，本次没有重跑 JVM/IT 验收。实际包版本、校验和、Git HEAD/树和只读观察索引见 [检查点](../checkpoints/20261116/agent-first-paired-loader-checkpoint.json)。

18 仓在本阶段文档写入前均干净，仍为共同本地 WIP 分支。原 3 现场 7 个进程身份、38 个私有文件完整元数据、6 份旧数据内容，以及个人 25 文件再次精确。没有重放旧闭环密钥验收、重发/换号 UNKNOWN、跨 Root 查询旧 ID、推送或清理运行中 target/runtime。新失败/普通运行现场及其实际 bind 输入继续保留。

旧 networking 活动 `99-kura-nm.conf` 的安全保留迁移、原 half-configured 现场续接仍是内部 pending；不能删除配置、放宽检查或伪造包状态通过。完整 Core/networking/GWT/agent/console/Docker、provider/launcher/模式/升级移除、R017 产品维护流程和冻结 12 父项继续实施。external deferred 仍仅实际 NM/cloud-init/物理硬件、amd64、磁盘满/硬断电三项。G034 未封签。
