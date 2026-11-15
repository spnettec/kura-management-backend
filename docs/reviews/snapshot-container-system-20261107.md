# 容器快照的实际 VM 恢复与补偿验证

核定 `01258353`，2026-11-07，接续 [ff350488](snapshot-container-fidelity-20261107.md)。完整管理体系目标保持 **active**。本轮关闭 ff350488 新类型变更的 OrbStack→现有 Parallels 系统验证内部 pending；上轮449 Java、53 bridge、mock中英浏览器1及 IDEA 完整 Kura 的结果按原证据沿用，没有重跑其一次性入口。本轮未修改产品源码，没有重新迁移、重构或打包安装。

新夹具只复用固定 bundle、Java工具、探针字节码及已核对的 agent ELF，分别从新 Kura home/CM/agent state 种植，未复制旧成功事务、receipt、completion 或 state。新 agent 的 remote=0、配置写入/firewall=0、未绑定中心；本轮验证本地管理员 UDS 的真实 agent→Java 校验→受控完整 Kura 应用/补偿。没有新的中心、gateway、mTLS或远程任务证据；旧远程闭环不冒充本轮结果。

## 实际结果

| 环境 | 全新现场与进程历史 | 核对 | 收尾 |
|---|---|---|---|
| OrbStack | `/opt/yofc-containers-orb-20261107-47dd82d8`；seed703804→apply704122→inverse705941，末次agent706081 | 284 inventory/现场283 bundles，306 profile pins；9→2→9的8种基本类型数组、4空容器、集合、密码集合均由真实 DS 与 ConfigurationService核对；正向/逆向 agent重开不重启 Kura，journal/原证据/业务事件核对，上传持续拒绝；原GWT admin/admin、Count1及 `type=password`通过 | 新4 unit全停，所有数据保留；旧4服务原PID/包运行 |
| Parallels | 现有Ubuntu26.04 ARM64，`/opt/yofc-containers-parallels-20261107-867c998f`；seed2681→apply3103→inverse3499，末次agent3629 | 284 inventory/现场283 bundles，私有Java184文件，共490 profile pins；同样实际9→2→9、agent重开和写闸门核对；原GWT登录/Count1/密码输入类型通过 | 新4 unit全停，正常关VM；Windows仍stopped |

每环境各一次新 upload/prepare/execute，以及一次独立 compensation.prepare/execute。Orb正向 `rec-5f73e291-77d9-423c-a705-9118edcbc3d3`、逆向 `rec-52e66517-c34e-4f79-bed1-0b5c4bd56939`；Parallels正向 `rec-39e1f40b-1790-4466-b938-f5d8e2cf842b`、逆向 `rec-57ae1956-650b-4a3f-96d0-4de9a6766547`。恢复输入均为26配置/7工厂/225属性，6个 Password值全部通过认证、legacy0。分别 SHA：

- Orb `e0a0a5638987548d218ad655420f6322278b88dec434af7a7d8b35ba67a07d11`。
- Parallels `d795351efd072c4a222bf00bdc24241830300f26f5f6364c259795246a86f270`。

字段值9/2/9属于独立容器测试组件；旧标量组件Count1原样保留，GWT读取没有提交配置。集合允许 CM/SCR 的合法具体类规范化，核对元素类型、顺序、重复值及空集合，基本类型数组仍核对精确类型。密码实际经过 Kura AES、CM字符串转换和 typed API解密；没有以opaque制品或unit代替 DS 回调。

正向最终 `AWAITING_COMPLETION`，独立补偿最终 `AWAITING_COMPENSATION_COMPLETION`。两阶段普通 snapshot写入实际拒绝，agent重开后 upload仍拒绝，Kura PID、原source artifact/request/receipt/plan及备份/意图/实际CM证据保持。最后journal继续非终态，不执行normal/health/completion/写释放，不追认为完整网关恢复：`kuraRecoveryVerified=false / normalBootVerified=false / fullRecoveryVerified=false`。

## 沙箱与输入

Parallels初始agent以及补偿后末次重开的agent3629均实际检查 `CapEff=0`、`NoNewPrivs=1`，进入其mount namespace向专用只读探针写入返回EROFS30。`ProtectSystem=strict/ProtectHome=yes/PrivateTmp=yes`及受限RW目录实际保留；agent与其standalone Java校验可读私有JRE/工具，不安装系统Java。此严格沙箱结论针对agent进程，不扩大为Kura JVM或整机完整隔离。

Orb既有全局LXC override仍将只读/其他属性覆盖；其专用探针实际可写，明确 `readOnlyEnforced=false`。不以unit请求值冒称严格沙箱，严格边界由本轮Parallels证实。

产品源码277条和5个核心/工具JAR执行前复核与ff350488完全一致。agent复用已验证ELF SHA `bd85f7b98c5e9ae124c368904d2099f6b54d6af11bf477c03f70eb0761f1a939`，没有安装review deb。Parallels私有JRE沿用已验证tar SHA `b2ee6b00515631b684f124276dd61423667db10a7b6c12039765d2552aa567a3`、184文件。独立profile和现场捕获分别357/539文件pin；Parallels关机前全unit PID0且journal hash与最后运行核验一致。

机读 [evidence](../evidence/snapshot-container-system-20261107.json)；日志、含确认token的私有事务记录、生命周期脚本、输入/现场hash与浏览器观察在 `target/container-system-20261107/`。公开review/evidence只保留事务ID等必要元数据，不复制确认token。浏览器观察由CUA直接完成，记录原GWT登录、Count1与密码input类型，不宣称新增负基数密码form通过。

## 中间失败和原事务续核

首个Orb agent因本轮夹具把startup timeout写成180、超过既有1–120合同而在准入失败，未出现journal、request、apply/inverse启动或forward准入标记。保留原两个grant、失败journal与原descriptor，按120修正后启动该新agent，未重跑seed，也未扩展产品限制。

Orb第一次正向实际已执行且DS通过，观测脚本新增fail-fast代码却覆盖了等待的目标状态变量，使其一直等待。停止的是该只读观测进程；原业务JVM和原事务保留。修正脚本后用原ID纯观察续接，没有第二个upload/prepare/execute。另做一次新的agent重开，先持久保存journal/业务事件/PID前像，再核对原字节不变；不声称丢失的第一份内存前像已经归档。之后独立补偿正常完成。Parallels从修正后的脚本执行。

Parallels首次只读collect返回guest-tools `Invalid argument`；未据此判断VM或业务进程终止。先实际确认VM运行、collect进程不存在且产物尚无，再只重读原现场；后续490 profile pins、严格沙箱及539现场pins成功。未重发任何业务请求。全部失败材料保留。

## 最终状态与未交付业务

本轮两处各4新unit均停止；Parallels正常shutdown，Ubuntu/Windows都stopped；临时防休眠60944停止，无本轮宿主中心、PG、Vite或JVM。上轮native四JVM仍停。原Orb四服务244516/244911/286620/268与console26/agent20包持续原样active。18仓HEAD/branch/旧dirty保留，无reset/clean/提交推送，未触及192.168.2.125、活动target/runtime或个人~/.kura-dev。所有夹具、快照、state、私有key、备份、quarantine与卷保留；本轮seed/恢复/补偿全部禁止重放。

本轮只替代ff350488“新类型VM验证pending”的状态。下一实际实施任意合法metatype XML文件名/动态provider、原GWT负基数Password集合空form、Collection远程revision/投影/编辑适用面；保持默认关闭与审核PID，不自动放开组件。继续原总台账其他内部pending：完整组件/真实生产组件业务、部分或空CM/外部PID离线恢复，其他未决恢复运维，Core安装升级/bootstrap/启动策略交接与旧入口退役，NM/firewall/NAT/forward/远程流程/早启，一致DB/制品/key备份恢复、多轮key轮换/旧系统迁移及正式发行。已闭环restart/reboot和交接取消不重跑。

外部deferred仍独立为真实NM/cloud-init/物理网络硬件、amd64、磁盘满/硬断电；两个历史evidence gap原样保留。已具备的Parallels Java/完整Kura环境不再记为缺失，也不把完整软件业务适用面推给外部deferred。完整目标继续active。
