# Core 保留卸载候选与实际失败边界

2026-11-18。代码 `4b8171c8b9fd156723276a2ab7c89b39524cc40f`，安装后回调路径修正 `b509d5d3f730f386d18a5081ff121c3fe25e2f58`。本轮继续完整目标；最终父项仍0/12，无可靠加权总百分比。用户询问时间时仅给“数周、可能超过一个月”的粗估，不是承诺日期。11个sibling既有有限所有权修复不代表六组件发行矩阵完成。

`prerm`新增保留整个`.data`到独立addon-data，Root addon代码仍走原libexec配对；等待卸载子进程并传播回调、disable和unit删除失败。Root维护reader新增保存/核对Main boot choice：固定临时目录及私有Root目录、原remove维护ID、原journal.key的独立HMAC域、保存unit摘要和kura/kurad原UID/GID绑定；不写新业务ID或初始化Root。重装前核对，避免作为新服务enable；恢复时仅对原enabled/enabled-runtime选择做对应enable，不用--now，不改他人的mask。已有未知/缺少新记录的legacy retained不能静默采纳，迁移仍内部pending。静态/mask等仅有模拟合同，真实profile全矩阵未验。

卸载保留数据时不删除普通kura账号，避免重装回收其数字UID；kurad及独立clock/recovery账号本就保留。未宣告全部系统授权清理完成，现有NM polkit移除行为需继续审查。Root原配对门禁未放宽。

当前52项临时文件系统/注入manager回归全过，含数据/历史副本、失败传播、原fence/HMAC/身份/保存unit变更拒绝、各boot选择及安装目录清理后的回调。早期新测试的macOS临时目录别名错误已修为使用解析后的fixture root，生产校验未放宽。未累加旧JVM/Python报告，也未重跑未变的clock业务。

## 实际同现场结果

从新Git/private Maven缓存/JDK21/Maven3.10.1顺序重建support、Core/tools、distrib。4b候选ARM64包 `6.0.0~git202611180104.4b8171c-1`，SHA256 `2014aec537e4789f3fa201615f93bc713a3ea8e2a30046dde9bbb12eecf1e2a1`。Root载荷/内嵌guards/维护shell/Main依赖通过；两clock生产类与既有VM字节一致，不把原VM或旧中间产物作为构建输入。此包现在是**已知卸载失败候选**。

沿原容器 `7fd8c6a0967a58a95c123785e5feff68cd0ce74e370d00ee875f9e215ac08d7e` 的e1安装、原账号和数据继续。唯一一次真实dpkg升级exit0；原26输入中19个外部文件完整指纹、7份Core数据/历史逻辑字段保留，新增1正常package-history，总27。Core拷回产生inode/ctime差异如实保留，不断言atime。Main保持disabled/inactive/PID0/NRestarts0，无事后补disable；clock namespace/operations不存在，key业务/startups均0。

随后唯一一次真实 `dpkg --remove kura-core` exit1。已经发布完整retained副本、保存disabled选择，clock/key listeners已移除且维护围栏保留；失败在新增prerm错误地调用已被postinst删除的`install/manage_kura_users.sh`，**没有进入账号回调**。这是本轮引入的路径错误；原已安装Root helper实际为`bin/manage_kura_users.sh`，源码b509d5d3f7已恢复此路径并新增回归。原失败日志保留，不将该remove或未执行的reinstall/purge记为通过；源码修正尚未重建/安装。

失败后只读复核原27输入：20个外部文件完整指纹、7份Core数据/历史逻辑字段及各自retained副本一致，历史两标记和Root0600数据canary保留。产品`--check-retained-main-state`沿原Root/fence/HMAC/账号校验通过，原clock目的/UID995/GID988保持；所有账号数字身份相同，0 clock业务、0 key业务、0 Core启动。包状态仍installed 4b。原remove维护ID `1fab699a-6544-4f65-87f3-cf24a0691c05`、`/var/lib/kura-retained`、原canonical数据、维护clock状态均保留。随后同容器正常停止exit0，不能重当空主机、清retained或改维护ID，也不能盲重发remove。

此次包内key archive只是该次正常移除的数据恢复副本，未调用已闭环的Root备份/归档/轮换/恢复/续期验收。私有日志/指纹仍只在本地0700/0600，不进Git或知识库。

## 后续仍需实施

先沿已知失败的同一原维护目的，独立验证当前状态与已完成步骤，再完成可审查的精确续接/恢复；不得为绕过失败清副本或换身份。需要处理删除部分unit后的续接、原retained与canonical双份状态、原回调路径和最小重试面，不能直接套普通upgrade制造第二份preserve。之后从修正Git构建，完成实际保留重装及移除/显式purge/中断/legacy/全部enable状态矩阵。新retained metadata缺失的旧态与授权清理仍内部pending。

01:09UTC原3/个人25/C9进程计数/D29原Main3135及E原配对/身份20/Core6/local7/material5/switch9/transport7、native29323/Core29609/key4714/recovery4640/NM4668保持。VM未启动，原+1ms未重放；E Main仍11/13cap、NNP0的未完成风险没有改变。未部署原现场，没有push/JDK27/活动target/runtime清理。

完整Core/OSGi、当前合法Main消费者、六组件/provider/launcher/模式、全部安装升级移除恢复、RTC/Chrony/其他typed brokers、API/R017/GUI/网络/密钥/CA/数据库/上游及发行12父项继续内部pending。只保留原3项external deferred，G034未封签、协议未正式发布；旧UNKNOWN/丢失CSR不重发换号/跨Root查，已闭环密钥与900秒不重放。

回执目录：父工作空间`.management-close-20261116/source-cohort-20261117/core-retained-reinstall-20261118/`。参见[检查点](../checkpoints/20261116/core-retained-remove-checkpoint.json)。
