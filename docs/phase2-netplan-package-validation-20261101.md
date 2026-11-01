# Netplan 独立发行入口与包生命周期验证

日期：2026-11-01。阶段 2 的发行入口增量，整体改造继续进行中。
前序开发运行集证据见 `phase2-network-profile-validation-20261101.md`；
本记录使用真实新 deb，不以开发运行集代替包验证。

## 实际代码

核心仓库新增 `kura/distrib/kura-netplan-core/`，networking 仓库新增 `distrib-netplan/`。
两个父 POM 的 `netplan-package` profile 均为显式选择；旧默认模块保留。
构建时必须同时指定目标 `-pl`，避免把旧发行模块也作为 Netplan 发行构建。

| 项目 | 实现 |
| --- | --- |
| 核心包 | `kura-core-netplan`，版本化 Provides `kura-core`，与旧 core/networking/firewall-only 冲突 |
| 适配包 | `kura-networking-netplan`，五个 bundle：bridge、netplan、公共网络配置模型及配置/状态 REST |
| 程序 | dpkg 管理、root 拥有的 `/opt/eclipse/kura`；核心 215 个 JAR bundle，包含 launcher/framework |
| 变量数据 | `/var/lib/kura-netplan`，独立用户 `kura-netplan`；user/data/packages/load 为 dpkg 显式链接 |
| 配置与秘密 | conffile `/etc/kura-netplan/runtime.json`；首次初始化独立 crypto key、随机 keystore 密码及随机 admin 密码 |
| 初始密码 | 仅写 root 0600 的 `/etc/kura-netplan/bootstrap-admin`，保留强制改密标志，不输出到验证报告 |
| JVM | foreground Python launcher，无 shell eval、无 Linux capability；8080/8443/8444，密钥走环境而非 argv |
| 系统依赖 | 新包不依赖 NM/ModemManager/iptables，不复制旧接管脚本、polkit 或 Linux clock writer |
| 加载守卫 | 每次启动检查 bundle 身份，拒绝 NM、linux.net、旧 firewall/threat writer 及重复身份 |

相同身份且字节相同的重复 JAR 在 stage 去重；身份相同但内容不同则拒绝。
核心原清单的重复 Gson 已由真实组装覆盖。适配包不反向移动 sibling 实现到核心。
默认 socket `/run/kura-management-agent/control.sock`、agent OS user root 仅为配置默认值；
本轮没有给现有 agent 授权新 Kura UID 或接口，没有变更 agent env。
这些本机 Kura 凭据与 console/agent 的独立登记认证无关。

维护脚本由 `tools/lifecycle.py` 单源生成，每份脚本自包含，postrm 不依赖已经移除的程序。
安装及升级不自动 start/enable，升级/卸载前仅停止自己的 `kura-netplan.service`。
普通 remove 保留数据、快照、密钥、配置和日志；重装不重新播种。
purge 只清理 `/var/lib/kura-netplan`、`/etc/kura-netplan`、`/var/log/kura-netplan` 及
`/etc/systemd/system/kura-netplan.service.d`，保留系统账号；不碰 agent 身份、系统网络、旧保留区或用户家目录。
旧安装、旧 unit/保留状态、危险 symlink、中断初始化及保留数据缺原 key 均拒绝自动迁移或重新生成。

## 构建与静态检查

在各自真实仓库执行，不使用 clean：

```sh
# kura/
mvn -B -f kura/distrib/pom.xml -Pnetplan-package,arch-aarch64,!arch-x86_64 \
  -pl kura-netplan-core package -Dmaven.repo.local=/Users/heyoulin/.m2/repository \
  -Dpackage.version=6.0.0~netplan20261101 -Dpackage.revision=3

# kura-networking/
mvn -B -Pnetplan-package -pl distrib-netplan package \
  -Dmaven.repo.local=/Users/heyoulin/.m2/repository -Dnetplan.package.version=3.0.0~netplan3
```

| 最终候选 | SHA256 |
| --- | --- |
| core `6.0.0~netplan20261101-3` arm64 | `b72b7009fd71005d577b387df351a6058e4c1cffe5b6aa9b85afb237e857e3e9` |
| networking `3.0.0~netplan3` all | `d03423942fe0cbc83ce30b4313e50b9be888fb2f2b63d97365256742c17c1a4c` |

两个真实 Maven/jdeb 构建通过。生命周期/打包回归 13 项通过，覆盖保留/清理范围、缺 key、
中断、symlink、重复 bundle 和 XML 前缀。archive inspector 直接解 deb，检查 control、
依赖、root ownership、四个数据链接、自包含脚本、unit、真实 bundle 身份和 seed 的 13 个配置。
最终 inspector 也检查 `SuccessExitStatus=143`。
`netplan-package-archive-revision3-20261101.json` 中 `linuxLifecycle=pending` 是该独立静态工具
不执行 Linux 测试的边界；实际生命周期结果另见 VM 报告，不能把两者混作同一个检查。

## 验证顺序与真实结果

先做本地 13 项和 React 默认浏览器 11 项（0 unexpected/skipped/flaky），再通过 IDEA 实际
Run 按钮启动完整隔离 Kura PID 12745，使用原控制台 admin 登录并查看 Status；IDEA Stop 后
独立确认进程退出。React 截图使用本增量独立的 `netplan-package-current` 前缀。
原 `target/runtime` 和个人 `~/.kura-dev` 未清理。
该 IDEA 运行集仅验证既有完整开发环境，不能作为下面新包组合的 GWT/REST 业务验收。

之后复用 OrbStack `yofc-management-network-20261101`（Ubuntu 24.04 arm64），未启动 Parallels。
VM helper 维护单向 state，不删除 state、不重放已 restored 的旧网络 fixture。

| 步骤 | 实际结果 |
| --- | --- |
| apt 模拟 | 没有新增 network-manager、modemmanager、iptables |
| 修正后的首装 | core revision1 + networking netplan2；初始 seed 可被当前 Kura XML mapper 读取 |
| 显式启动 | UID997，CapEff=0，密钥不在 argv，加载清单无旧网络 writer；HTTP root 404 |
| 升级 | core revision1 → revision2、networking netplan2 → netplan3；保留数据逐文件 SHA 不变 |
| 普通 remove | 数据、快照、key、配置逐文件 SHA 不变 |
| 重装 | core revision2 + networking netplan3，保留数据不变，不自动启服 |
| 停止修正 | 更新到最终 core revision3 + networking netplan3，数据不变；start/stop 后 inactive、Result=success |
| purge | 先在 VM root0600 归档本轮生成数据/密钥，再清理声明路径；最终两个包均 absent，unit not-found/inactive |
| 其他 owner | Netplan/networkd/NM 配置、旧 fixture state 和 agent env 与基线一致；现有 agent 仍 active |

生命周期按上述不同候选逐步执行，未声称最终 revision3 重新执行了全新首装的每个步骤。
最终候选实际经过升级、启动/停止及 purge；完整最终包的业务回归仍 pending。
HTTP 404 只证明监听器响应，核心不含 GWT UI，不能记成控制台或 API 业务通过。

OrbStack 全局 `/run/systemd/system/service.d/zzz-lxc-service.conf` 关闭了文件系统沙箱及
NoNewPrivileges；实际 NoNewPrivs=0、ProtectSystem=no。
低权限 UID/零 capability 已验证，**此包的完整 systemd 沙箱仍 deferred**。
生产 unit 未为测试弱化，不能借用先前 Parallels agent 的沙箱结果。

## 真实失败及修正

首轮 ElementTree 默认生成 `ns0` 前缀，而当前 mapper 用字面 `esf:configuration` 查询，
导致配置读取为空，运行时 seed0 变为加密空快照。
后续修复守卫发现 seed 已变化，拒绝原地修改；未把前缀替换当成恢复成功。
stage 和 lifecycle 现显式注册 `esf`/`ocd`，回归按当前 mapper 的实际要求检查。
首轮新建状态、配置及日志保存在 VM fixture 的 `failed-bootstrap/`，然后只清理该次新包拥有的路径，
以修正包重新首装。旧 restored fixture 未重放。首轮安装回执不等于首轮启动成功。

另一个实际问题是 Java 收到正常停止信号后退出143，systemd 默认记 failed。
unit 增加 `SuccessExitStatus=143`，最终 revision3 在 VM 的真实 start/stop 验证通过。
不扩大成功码范围，异常退出仍由 Restart=on-failure 处理。

最初 inventory 中 agent env 路径拼写有误；后续对照旧 restored fixture 的
`originalAgentEnvSha256` 校验真实路径后补入验证基线，实际没有修改 agent env。
失败日志和历史候选 SHA 全部保留，不覆盖为最终成功结果。

## 现场与证据

VM fixture `/opt/yofc-kura-netplan-package-20261101/state.json` 最终 phase 为 `purged`，
不可重放 inventory/install/start 等 action。失败归档和
`retained-before-purge.tar.gz` 均留在 VM，后者 root:0600，不将秘密复制到仓库。
旧 `/opt/yofc-kura-netplan-profile/state.json` 仍 restored，agent env 校验与其原 SHA 一致。
Parallels Ubuntu/Windows 均保持 stopped；未连接已移除网关。

关键本机证据位于 console 的 `target/`：

- `netplan-package-unit-final-20261101.log`、`netplan-package-react-browser-20261101.json`。
- `netplan-package-idea-console-20261101.png`。
- `netplan-package-core-final-build-20261101.log`、`netplan-package-network-fixed-upgrade-build-20261101.log`。
- `netplan-package-archive-revision3-20261101.json`。
- `netplan-package-vm-{inventory,install-verified,start-verified,upgrade,remove,reinstall,service-stop-final,purge,final}-20261101.json`。
- `netplan-package-vm-failure-archived-20261101.json` 及首轮失败 stdout/err，保持历史语义。

源码、产物和报告哈希汇总到新增 `docs/evidence/netplan-package-20261101.json`。
旧 evidence manifest 不改写；target 可重建，报告缺失时不能据 manifest 宣称重新通过。

## 尚未交付

本增量完成显式包入口及受控生命周期验证，未完成生产发行或阶段 2。
新包尚未配置真实 agent 的 Kura UID/socket 权限；GWT/REST 与此包组合、初始改密、
快照保存和实际 agent 委托的业务验收仍 pending。
既有核心契约在外部 owner 不可用时拒绝涉及确认网络镜像的快照保存，本轮未修改该 Java 契约。
断连下本地配置持久化策略须后续处理，不能隐藏 `AGENT_UNAVAILABLE` 或以启动成功替代。

下一步先补可独立安装的 agent 包、明确 UID/socket 授权和两包组合的可复核业务验证，
然后推进完整旧字段、NM/cloud-init 真实交接、登记/转交/轮换、nftables、系统控制、离线恢复和发行矩阵。
缺少环境的 Ubuntu26.04/amd64/物理有线、拔插和硬断电边界继续 deferred；
不把既有开发运行集或 test-only owner 的通过记录扩大到本发行组合。
