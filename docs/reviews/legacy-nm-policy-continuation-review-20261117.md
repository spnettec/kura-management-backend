# 旧 NM 策略保留与原失败安装续接（2026-11-17）

用户授权现有积分继续，目标 active。旧 networking 失败现场已通过同一安装意图续接到 Core＋12 sibling 全部 configured；这不是完整运行、purge、GUI 或正式发行验收。原始失败及本次暴露的数据偏差均保留。

## 代码与实际迁移

旧包把活动 `/etc/NetworkManager/conf.d/99-kura-nm.conf` 当作普通载荷。当前正常包已不携带此文件，直接省略会被 dpkg 删除。新增受限过渡构建器从指定 SHA256 的当前正常包生成一次 conffile 过渡包，核对 Root 代码、实际钩子/模板及无活动 NM 载荷；其他数据载荷保持，仅增加该配置模板及父目录，变更 control/conffiles/preinst。没有 diversion、移动现用配置、手改 dpkg 数据库或伪造配置版本。

过渡入口只允许已有包升级，要求显式 `--force-confold`，拒绝冲突选项、降级、链接/多链接、非 Root 或可写文件/目录。随后正常包省略活动文件，dpkg 按原生 conffile 语义保留它。依据见 [deb-conffiles](https://manpages.debian.org/trixie/dpkg-dev/deb-conffiles.5.en.html) 与 [Debian 配置文件策略](https://www.debian.org/doc/debian-policy/ch-files.html#configuration-files)。该文件仍是 obsolete conffile，通常 purge 会删除；完整保留/purge 产品策略继续 pending。

第一次实际构建的版本后缀含连字符，改变 Debian revision 的拆分，版本门禁在生成归档前拒绝；修复为 `~nmpolicyretain1` 并补带实际 revision 的回归。后续归档核对曾因 tar 目录类型位及新增根目录项误报，按权限位/明确目录项纠正，只读结果保留；没有借此改动实际包或网关。

## 实际发现并修复的生命周期缺陷

同一现场 `8d5c8e8f493c…` 的首次过渡包配置返回 0，活动 NM 文件完整元数据保持。但 postinst 将“旧 configured-version 为空”误认为首装：旧包从未配置成功时，dpkg 正会传空值，结果给新测试现场的 `kura_custom.properties` 追加了两个 UI 标志。验收立即停止正常包安装，未将此结果记为完整数据保留通过。

修复后 preinst 在 `/var/lib/kura-networking/install-mode` 保存受保护、持久化的实际安装模式，postinst 严格读取它；upgrade、未完成的首次配置及重新安装不再初始化现用属性。缺失、损坏、链接或不安全的准入记录会拒绝。该模块状态不是 Root 密钥事务或网络 ownership 授权，保留跨卸载状态，不读写快照或密钥。

只读核验追加后的前 831 字节与原 SHA256 精确一致，后 55 字节正好是两项已知 UI 标志。以实际 kurad UID 和既有 Core traversal group 只撤销该追加，恢复原字节、属主、组、模式、大小和 mtime；首次修复因缺少 traversal group 在打开文件前拒绝，原记录续接后完成。原 inode 已被原子替换，ctime 也已改变，二者不能恢复，不能声称原 19 文件完整元数据始终不变。其余 18 文件完整元数据保持，修复后的 19 文件作为后续安装新基线。没有运行密钥恢复或重放任何旧业务验收。

从 JDK21 当前源码构建含修复的正常包，在原现场继续一次安装，返回 0。当前包为 `3.0.0~git202611170419.5d27fea-1`。13 包全部 configured、64 个 sibling JAR 为 Root 所有且 group/other 不可写，12 个注册项顺序保留且无重复。新模式为 upgrade，修复后 19 文件完整元数据精确，原活动 NM 文件仍保持原完整元数据。Main 启动数 0，Root operations/startups 均 0，网络服务 PID/ticks 不变。resolver 仅 ctime 变化，原因未确定，其内容、inode、mtime、属主、模式保持；不把全部系统文件元数据宣称为不变。未执行 NM reload、renderer 切换或网络 apply。

## 验证与继续范围

24 项 sibling 源码/Linux 检查通过，含旧半配置且空版本的回归及缺失/危险准入记录拒绝；9 项真实 dpkg 迁移检查通过，覆盖未改/定制配置、新装、缺保留选项、冲突选项、降级与危险路径。当前正常包核对 9 个 Root/0644 JAR、26 份源码一致的代码/模板、8 条 API 声明。最新修复版过渡归档已组合，尚未将该最新版过渡包另作完整实际 Kura 迁移验收；原现场没有重建来掩盖首次偏差。JDK21 打包时明确跳过 JVM/IT，不增加 Java 测试通过数。

18 仓均在共同本地 WIP 分支，本阶段文档写入前均干净，无推送。原 3 现场的 7 个进程身份、38 私有完整元数据、6 旧数据及个人 25 文件再次精确。agent-first 普通运行现场继续保留；不重发/换号 UNKNOWN，不跨 Root 查询旧 ID，不重放已闭环备份/归档/轮换/恢复/900 秒验收，不清理运行中 target/runtime。

接下来继续完整 Core/networking/GWT/agent/console/Docker、provider/launcher/模式/安装升级移除矩阵；legacy 配置 purge 策略、原默认 networking 运行及 R017 产品维护 UI 继续内部 pending。Main 11/13 caps、NNP0、完整 broker、provider 重加密和全部冻结 12 父项没有关闭。仅实际 NM/cloud-init/物理硬件、amd64、磁盘满/硬断电三项 external deferred；G034 未封签。

包、Git 与原现场记录见 [检查点](../checkpoints/20261116/legacy-nm-policy-continuation-checkpoint.json)。
