# Core 保留重装与 NM 授权清理实际验证

2026-11-18，代码 `38a983787e9b8ce9d6fbb446f5e5322de8762da1`。修复Core移除后仍保留`kura-nm.rules`授予kurad网络权限的问题。正常产品脚本在同一原7fd现场实际升级、移除、保留重装均exit0；Main的disabled选择、原UID/GID、密钥及历史数据保持，实际polkit授权允许→拒绝→允许。本轮只接受这个Ubuntu24.04 ARM64/systemd/disabled profile的有限范围，完整矩阵和12父项继续开放。

## 原失败的同目的精确续接

先核18仓与前检查点完全一致、原7fd stopped/exit0、冻结12父项SHA不变。原包4b的失败维护 `1fab699a-6544-4f65-87f3-cf24a0691c05`、原canonical/retained、27输入、8份全部可变文件及其副本、原NSS、原Root journal HMAC、完整Root代码/当前unit/clock actor配对均独立核对；五个helper unit已经not-found/inactive，Main仍disabled/PID0，key/startup/clock业务0。原prerm SHA `d47d97a1…b2f6b26c`及已安装Root bin回调SHA `972c85a3…452339b`精确。

仅在这个自有隔离夹具归档原prerm，发布带上述固定原边界检查的局部续接：从尚未到达的账号回调开始，执行原runPreRemove/preRemove尾段；没有重做已完成的数据保留或key/clock unit移除。不是通用产品中断恢复验收。第一次只读检查因dpkg回调期间实际状态为half-configured而非installed拒绝，账号回调0；一次脚本发布又因dpkg将脚本mode改为0755而拒绝，仍无账号回调。原日志不改。

修正为回调环境/精确版本核对，保留全部原Root/fence/代码/数据检查；原文件指纹上下文移入独立Root0600文件。此前短暂存在于该自有夹具可读prerm中的仅为测试元数据指纹，无密钥载荷；后续已原子替换，最终prerm由正常产品包覆盖。宿主私有日志/指纹均0700/0600，没有带入Git/知识库。局部续接exit0，原27输入20外部完整指纹、7份Core数据/历史逻辑字段及副本保持，包为config-files，原账号/clock目的/原remove ID不变。

新真实Git/JDK21/private Maven缓存完整构建14f包 `6.0.0~git202611180116.14f777b-1` / SHA `cb14bb336e4caef0c0320969e04b49be9182f12c1437c3278c57169fe6ac4cc4`。正常产品单次保留重装exit0，原1fab维护由产品leave生成同ID/firstAction=remove的package-history，原围栏和clock维护正常完成、retained恢复后消费；原27输入保持，新增1正常回执，总28。没有改ID绕过失败。

## 代码修复与回归

实际移除观察到旧Root0644规则仍在，SHA `ff81d098ed539b00d0de3e83dfa81ea6a103aa5b59c1758c6aa6069cfa10609e`，仍授予kurad NM/ModemManager/wpa命名空间权限。新Root本地manage_network_polkit_policy固定路径和原规则精确字节，check/install/remove均检查所有父目录、Root所有权、非可写、普通单链接及稳定读；不跟随链接、不接受额外内容/超长/FIFO。未知本地规则保留并显式失败，原Root/key配对未放宽。

新规则完整fsync后以独占硬链接发布，再撤临时名，不能覆盖并发加入的site规则；既有相同规则不重写，已知代码0600可正规化为polkit可读0644。安装器在准入和账号创建前只读检查，卸载通过安装后的Root bin回调撤销已知规则。补齐原sudoers/legacy规则删除、组移除和原配置还原错误传播；dialout已无成员时不误当失败。未知site配置及所有历史治理场景仍需完整矩阵，不据此宣布全部授权清理闭环。

63项当前宿主合同/文件系统/manager注入/systemd policy回归通过；12项实际Linux Root权限/文件系统条件通过、无skip，包含精确原字节/幂等、未知内容、链接/硬链接、错误文件或父属主、可写文件或父目录、FIFO、超长、非法动作和已知代码模式。Linux测试只改policy目标到自有tmpfs，生产Root检查逐字不改；临时只读容器无宿主绑定/网络/Kura初始化。最初symlink测试的readlink自查改变atime已修为lstat测元数据、之后核链接文本，失败保留、不回填时间戳。Java/clock生产逻辑未改，旧JVM/crypto验收不重跑、不累加。

## 正常产品路径及实际授权

38a从新Git依次重建support/Core/tools/distrib、JDK21/Maven3.10.1、独立Maven缓存，不写个人共享项目产物或活动target。ARM64包 `6.0.0~git202611180130.38a9837-1`，203548340字节，SHA `e365e1ab089ff7a1cd4a5d9640f57b1ebcbfe7bad5cd26ac78fb3b39fa1c2caf`。Root所有权/父目录、7个clock载荷/guard/维护脚本、Main依赖、当前bin回调与Git精确；clock类字节仅与留存VM作对照，旧产物不作构建输入。

| 同一原7fd场景，未修改产品脚本 | 实际结果 | 数据/维护 |
| --- | --- | --- |
| 14f→38a正常升级，单次dpkg | exit0/installed；Main disabled | 原28：外部21完整、Core7逻辑字段保持；新增1package-history，总29 |
| 38a正常移除，单次dpkg | exit0/config-files；NM及systemd已知规则均移除 | 原29：外部22完整、Core7及retained副本保持；保留原账号及新正常remove目的4f52… |
| 同38a正常保留重装，单次dpkg | exit0/installed；Main仍disabled | 同目的 `4f52e089-e745-4f1b-83e6-ac2fe8059252`由产品leave正常完成；原29保持，新增1package-history，总30 |

新4f52为1fab已完成后的正常独立矩阵场景，不能混作原失败换号。每个目的只在同一Root内使用，未发送旧UNKNOWN或闭环密钥/900秒请求。

注册一个没有业务处理器、默认拒绝的自有fixture action，前缀匹配该NM规则；Subject PID1574/ticks22949568，UID997/GID991、无附加组、五cap0、NNP1。实际pkcheck始终绑定同PID/start-time/UID：移除前exit0、移除后exit1、重装后exit0；三次均首读达预期，polkit PID62/原ticks/NRestarts0保持，没有重启daemon促结果。只证明该规则在真实polkit中的授权撤销/恢复，未执行NM网络操作。fixture action随后按原字节删除，subject验证终止，私有上下文Root0600保留。

所有Main inactive/PID0/NRestarts0，最终ExecMainStartTimestampMonotonic=0；clock actor未启、namespace/operations不存在，key业务/startups0。clock账号原目的2a388683…、UID995/GID988，以及kura1001/1001、kurad997/991、recovery996/990不变。外部完整字段dev/inode/uid/gid/mode/nlink/size/mtime/ctime/SHA；Core逻辑字段uid/gid/mode/nlink/size/mtime/SHA，复制产生inode/ctime差异保留，NOATIME自查不外推atime。两个不同历史标记及Root0/0/0600 canary保持，.data未作为回调执行。包内本地key恢复副本仅属正常数据保留，不是重放Root备份验收。

当前原容器 `7fd8c6a0967a58a95c123785e5feff68cd0ce74e370d00ee875f9e215ac08d7e` 安装38a，健康、Main disabled，正常停止exit0，30份数据/回执和原身份保留。不能重当空主机或重初始化。原1fab及4f52已完成，后续只读对照现存回执，不重做这两条验收凑进度。实际purge、其余boot选择、通用中断、legacy/source/资源/provider矩阵仍内部pending。

## 原现场及完整范围

01:39UTC原3现场/个人25/C9及原进程计数/D29原Main3135、E原身份20/Core6/local7/material5/switch9/transport7及native29323/Core29609/key4714/recovery4640/NM4668/原ON保持。两台Parallels均实时stopped，VM未启动、原+1ms未重放。E Main仍11/13caps、NNP0，尚未被本轮变更降低权限。文档前18仓同分支干净，仅Core38a预期提交，冻结12父项SHA保持。

完整Core/OSGi及当前合法Main业务、六组件/provider/launcher/模式/平台、purge/中断/legacy/全部安装升级矩阵、RTC/Chrony/其余typed brokers、API/R017/GUI/网络/密钥/CA/数据库/上游和发行12父项继续内部pending。仅原3external deferred，G034未封签/协议未正式发布，最终0/12父项闭环、无可信加权总百分比。目标active/现有积分继续，无push/JDK27/活动runtime清理；旧UNKNOWN和丢失CSR不重发换号跨Root查，闭环密钥和900秒不重放。GUI仍次要，本轮没有client JS改动。

回执目录：父工作空间 `.management-close-20261116/source-cohort-20261117/core-retained-remove-recovery-20261118/`。见[检查点](../checkpoints/20261116/core-retained-reinstall-policy-checkpoint.json)，Core合同`docs/management-nm-polkit-policy-20261118.md`。
