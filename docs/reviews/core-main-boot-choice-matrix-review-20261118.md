# Core 实际 Main 启用与屏蔽状态矩阵

2026-11-18，Core代码`97dbbc06e95db769813def751df5c1e318478a0b`及clock安装修复`44b428192320a87196ffd129207a627a8161734e`。同一原7fd ARM64现场实际enabled、enabled-runtime、masked、masked-runtime的正常产品移除/保留重装通过；Main均未启动，原UID/密钥/历史数据保持。修复runtime启用链接残留，以及凭据准备/clock安装对Main屏蔽的错误拒绝。完整矩阵及12父项继续开放，未把本结果当完整Core/OSGi或合法Main业务验收。

## 新发现及实现

实际PID1/systemd255.4-1ubuntu8.18命令探针显示，普通disable只清/etc的enable链接，enabled-runtime在/run仍保留；追加disable --runtime后两类enable链接均清，永久和runtime的/dev/null mask均保持。97改prerm按顺序同步disable两层，任一步失败在删除vendor unit前传播；补齐logrotate/cron和临时目录清理失败传播。真实shipped preRemove回调在6个受控unit profile条件通过，未初始化Core或网关journal；static/linked只作命令行为观察，完整Core对应路径仍待验。

原KeyStore.prepare一律拒绝/etc、/run的Main覆盖项，连Root显式/dev/null mask也拒绝。97只识别这两个固定路径下单链接、Root所有、父目录为物理Root目录且组/其他用户不可写、目标精确/dev/null、读前后稳定的屏蔽链接；必须已有合法canonical凭据及原回执。缺钥/缺回执不引导新钥，普通unit/其他目标/父链接或可写/错属主/多链接继续拒绝，mask增删/替换使stable失败。原regular-file reader、generation/UNKNOWN/Root配对未放宽；屏蔽项不作为key source，不读取目标。

实际97 masked-runtime重装继续暴露clock_setup.properties对Main LoadState=masked直接UNSAFE_UNIT。44只在Main固定Root/dev/null mask且明确inactive/MainPID0/ControlPID0时接受停止状态；clock service/socket自身masked、Root代码替换、非零/缺失PID仍拒绝。clock安装器仍不读取数据key，原code/effective-unit配对不变，Main从未被强制unmask或启动来取得产品通过。

最终选择135项宿主Python回归，134通过、1项不可变Root归档renameat2在macOS显式skip；不把skip算通过，不重放此前Linux归档/密钥/900秒验收。7个实际Linux Root mask/key条件通过（只在重定位的单元测试凭据目录，无网关journal/操作/PKI），6个实际shipped preRemove/systemd条件通过，不与重叠选择累加。第一次native测试因PID1继承umask000、夹具父目录可写而正确拒绝，原失败及目录保留；新独立夹具显式umask077，生产检查未改。

## 原夹具启动与数据观察边界

18仓初始同前检查点精确干净，冻结12父项SHA不变，官方父知识库最新记录/Tasks/Session全文与原readback一致。原7fd从健康stopped/disabled38a续接，未重初始化。启动后原30文件完整指纹首查失败，逐项只读核对确认全部仅st_dev变化，inode/uid/gid/mode/nlink/size/mtime/ctime/SHA九字段逐字精确；新根为overlay 0:347、原同CID/同image、无host binds。保留原失败及旧观察，记录本次重新挂载实例并建立独立当前boot基线，没有回填旧记录，也未改产品Root/namespace配对。该现场无GWT sender binding、key业务/startups0、clock namespace不存在。扩展packages/log只读库存，新增覆盖既有dpa.properties，总31，未修改任何数据。

新真实Git/JDK21/Maven3.10.1/private缓存依次重建support/Core/tools/distrib，不用旧target、不写共享项目缓存。97包`6.0.0~git202611180206.97dbbc0-1`，SHA `d9a326022f615cedc9d847c2a16af69b864655ec941dd5208b0385f39d213f4f`；44包`6.0.0~git202611180229.44b4281-1`，203549636字节，SHA `e3f8d8fa08aca1a715c697a5a378595c6c6985fa4aea5f9514a9ff80f502c407`。Root载荷/父目录、控制脚本、精确嵌入reader、clock manifest/Main依赖与Git精确；Java/clock业务未改，旧JVM及原VM+1ms不重跑。manifest文件SHA与protocol actor cohort分别记录，不混用。

## 实际产品路径

97普通disabled升级唯一一次exit0，原31输入保持，新增1正常package-history，总32。以下4个不同状态各自新正常本地维护目的，不重发此前已完成1fab/4f52，也不发送任何key/clock/Core启动业务：

| 真实状态/源码包 | 单次remove、单次retained reinstall | 保留输入/正常新增回执 | 完成的原remove目的 |
| --- | --- | --- | --- |
| enabled-runtime / 97 | 0、0；移除时/etc和/run wants均无残留，重装仅恢复runtime enable | 32→33；外部24完整、Core8逻辑字段 | `68bd1736-1eca-42eb-bb91-62cc45309e74` |
| enabled / 97 | 0、0；重装恢复持久enable，不使用--now | 33→34；外部25完整、Core8逻辑字段 | `16c8ecb7-ecb8-4392-8feb-1f9c46d88bb2` |
| masked-runtime / 完整44 | 0、0；原/run mask inode/属主/模式/链接/mtime/ctime精确 | 36→37；外部28完整、Core8逻辑字段 | `3793c1ea-b543-4976-a560-ed51950efe43` |
| masked / 完整44 | 0、0；原/etc mask对应字段精确 | 37→38；外部29完整、Core8逻辑字段 | `7b160323-4f5c-4618-a23e-e7a35add6d0f` |

两种enabled结果针对97；44后续仅新增严格masked分支、未改loaded路径，当前表未冒充所有代码版本/模式组合完整验收。每个remove的Root journal HMAC/Main选择/原NSS绑定原样用于其reinstall，leave由产品以同ID/firstAction=remove产生正常回执。每次Core逻辑字段uid/gid/mode/nlink/size/mtime/SHA保持；拷回inode/ctime差异保留，外部同boot完整dev/inode/uid/gid/mode/nlink/size/mtime/ctime/SHA精确，NOATIME自查不声称link/所有数据atime一致。packages/dpa.properties、两历史.data标记和Root0/0/0600 canary保留；.data未用作Root回调。

## 97 masked-runtime原失败及修复的明确范围

97的这项首次remove exit0，但reinstall在clock安装前UNSAFE_UNIT、exit1/half-configured。原目的 `2041f9fd-4a61-4acf-b0ae-afd84e8ab8cd`、原mask、retained及全部34份原文件（外部与retained）完整指纹再次精确，产品HMAC/原账号验证通过；canonical只是未配置factory，未假称已恢复原数据。未重发原安装、换ID或清副本。

只在此自有夹具先按旧manifest核全部clock/actor/Root/key配对，再发布新Git完整44包中7个clock代码/unit/JAR及manifest（8文件）到canonical；原code归档保留。独立clock cohort从`c7d41362…858cd1f`到`324441c5…1acc88`，key-domain manifest原字节及原key/clock维护目的保持；不复制/初始化clock真实数据。随后唯一一次dpkg --configure沿原2041目的exit0，正常产品恢复旧data/历史、同ID完成维护，库存35。该有界代码修复不计通用产品中断恢复或未修改97的masked通过。

之后完整44普通masked-runtime升级一次exit0、同mask保持，全部35输入保留、正常库存36；再运行上表的未修改完整44 masked-runtime/masked正常产品验收。临时双组件版本状态已由完整44发行包统一，最终Key generation完整cohort及canonical/actor/effective-clock配对实际核对通过。当前actor protocol cohort `324441c5b0606b8f551c801bb07d31b588bf76c03a9009270d7d97d2051acc88`，manifest文件SHA `fb0da9f0e9258cbfd2958677d4d8018ee8c00e822e1cf54016434541209a89b3`。

Main全程inactive/PID0/NRestarts0，最终ExecMainStartTimestampMonotonic=0；actor未启动，clock namespace/operations及key业务/startups均0。原clock账号目的2a388683…/UID995/GID988及kura1001/1001、kurad997/991、recovery996/990保持。四状态验证完成后单独以夹具意图恢复其原disabled停放选择，不把此操作伪装成产品重装保留结果；清除了自建mask/enable链接，原7fd完整安装44、38输入/14正常回执、正常停止exit0。额外命令探针d923也停止exit0，未初始化网关Root；其重定位单元测试凭据和失败目录均留在独立夹具，不属于原运行数据。

## 继续完整目标

02:39UTC原3/个人25/C9及原进程计数/D29原Main3135、E原身份20/Core6/local7/material5/switch9/transport7及native29323/Core29609/key4714/recovery4640/NM4668/原ON保持。两VM实时stopped，本轮未启动VM或重放原+1ms。E实际Main仍11/13caps、NNP0，未部署候选降低其权限。文档前18仓同分支干净，仅预期Core提交，冻结12父项全文SHA保持。

static/linked实际Core/旧态转换、purge、通用中断/半配置恢复、全部launcher/provider/资源/平台及六组件发行矩阵继续内部pending，不能用此四profile缩减目标。完整Core/OSGi与当前合法Main业务、RTC/Chrony/其余typed brokers、API/R017/GUI/网络/密钥/CA/数据库/上游及正式发行和12父项仍开放。仅原3 external deferred，G034未封签/协议未正式发布、最终0/12父项闭环，无可靠加权百分比。现有积分继续/goal active，无push/JDK27/活动target/runtime清理。旧UNKNOWN和丢失CSR不重发换ID跨Root查，闭环密钥/归档/轮换/恢复/900秒不重放；本轮已完成各profile目的也不凑证据重做。GUI仍次要，未改clientside JS。

回执：父工作空间 `.management-close-20261116/source-cohort-20261117/core-main-boot-choice-matrix-20261118/`。见[检查点](../checkpoints/20261116/core-main-boot-choice-matrix-checkpoint.json)。Core合同`docs/management-main-boot-choices-20261118.md`。
