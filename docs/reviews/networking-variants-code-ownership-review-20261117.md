# Networking 兼容变体与实际普通启动检查点

2026-11-17。目标 active，继续使用用户明确授权的现有积分。继 [主 sibling 检查点](../checkpoints/20261116/sibling-code-ownership-checkpoint.json) 后，新增 [兼容变体检查点](../checkpoints/20261116/networking-variants-code-ownership-checkpoint.json)。完整 12 父项、原 3 项 external deferred 和未封签边界保持。

## 源码和包

`firewall-only` 原有 4 处 daemon JAR 映射改为 Root/0644；Root 安装代码进入 `libexec/addons/firewall`，复用主 networking 的相同受限注册表更新器。preinst/prerm/postinst 的代码升级不再修改快照或清空内核规则。安装、移除沿用原启动脚本的 agent ownership 标记保护；daemon-reload、legacy kernel reset 和启动规则恢复失败传播，不能作为成功。

首次安装对基线和最大数值快照先完成全部输入检查，再由应用身份原子替换所需单个文件，保留内容其余字节、owner/group/mode；已有防火墙配置不重写，历史其他快照不修改，链接或无效/加密输入被拒。此为明文 bootstrap 的逐文件修改，不是多文件崩溃恢复、密钥重加密或旧文件永久保留协议。用户配置由应用身份写入，移除保留用户配置和运行数据。无 agent 标记时的既有全内核 reset 仍需完整 ownership/inverse 审查，不能用本次 guard/model 检查关闭该父项。

9 个独立源码/Linux 原生 test methods 通过；内核及 marker 结果明确有模型，未代替真实 agent ownership 事务。实际包 4 个 Root JAR、2 份 libexec 源码及额外 Root boot writer 已核对，3 条 Crypto/配置 API 声明匹配。网络仓本地提交 `43fa0265ce6c855ad8546434c63075ad76ded11c`，未推送。

Netplan 兼容适配仍是显式非默认入口，没有切换或删除 NM。从当前源码编译五个适配 bundle 并出包，5 个 JAR Root/0644、6 条源 API 声明匹配；其 payload 没有可变注册表或提权安装脚本。没有安装该适配包，也没有把它视为完整 Core-netplan/agent/launcher 配对通过。

## 实际 systemd 场景

独立 fixture `bfcae177c554ab8dbb181879ca1c75b6d9651b5bea7410b46e685795d2125eb7` 使用当前显式 SHA 校验的 Core、GWT、firewall-only 和其余十个主 sibling，全部 13 个声明包与依赖配置成功。Ubuntu24.04 ARM64 / PID1 systemd / OpenJDK21；安装期间 Main 启动 0，随后按既定 fixture 隔离为仅 loopback。

只进行一次普通 Main 启动，实际 PID5277 / UID997，HTTPS根返回302到 `/admin/console`。全部59个实际 sibling JAR为Root所有且group/other不可写；Root原配对和读入门禁未降低。6份凭据/TLS/HMAC原字节和完整记录元数据保持，canonical与Main加载材料一致。operations/archives/startups为空，没有备份、归档、恢复、换钥、900秒续期或日志化启动请求。该场景的TLS为隔离自签证书，不等于一般信任链或人类ADMIN/XSRF联合验收。

实际Root服务PID4893/UID0、恢复PID4295/UID995，二者全部cap为0、NNP1；Main仍11 ambient/13 bounding、NNP0，完整broker继续pending。OSGi诊断仅发送只读 `ss`/`diag 200`，没有管理配置写入。业务bundle处于ACTIVE不代表所有DS/provider配置可用：仅lo导致缺少主网络接口及默认MQTT clientId，完整provider配置/业务/重加密仍待验。

额外Jackson JAXB bundle仍缺`javax.xml.bind [2.3,3)`；已对照当前POM及 `kura/docs/migration/jackson-yaml-boundary.md`，这是明确保留的历史额外扩展边界。没有补公共API、扩大导出或声称全部发行JAR已解析。它与应用provider业务缺项分别记录。

原普通 networking fixture `8d5c8e8f493c818c9fcc1800df51af6a765482ede93fc68f988d76ea073daee2` 仍保留、half-configured。该原失败及原日志没有由本次不同的firewall-only profile替代；容器udev、历史旧prerm首次升级以及完整六组件/provider/launcher/模式/升级移除purge继续内部pending。

## 后续

继续完整组件配对、所有模式与版本升级及凭据连续性，补完整实际provider/readiness和完整broker。兼容模式的普通启动、单包源码检查和模型不能关闭全部12父项。原3现场/个人现用配置/密钥/数据/运行中target/runtime继续保留；原UNKNOWN、startup.stop和全部闭环密钥验收不重发、不换号、不重放或跨Root查询。仅实际NM/cloud-init/物理硬件、amd64、磁盘满/硬断电为external deferred。
