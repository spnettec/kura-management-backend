# 未执行普通交接取消的实际系统验证

核定 `d91353d6`，2026-11-07。接续 f2ea1b07，完整管理体系目标保持 **active**。本轮闭环该批内部 pending 的新 IDEA → OrbStack → 现有 Parallels 系统验证，并生成含 op29 的独立 review 制品。前轮一次性 restart/reboot 不重放；没有复制旧成功事务、completion 或 state 作为新父事务。

代码合同见 agent `docs/ordinary-handoff-cancellation-v1.md`；本轮源自新运行集的正向应用、独立补偿、快照集恢复、普通路径、实时健康与明确完成，均实际建立新的 approved parent，之后才准备取消计划。所有取消结果仍为 `ORDINARY_CANCELLED / cancelled=true / issued=false / ordinaryUnitHandedOff=false / scope=administrator-selected-checks`，`kuraRecoveryVerified=false / normalBootVerified=false`。这证明未执行计划被放弃，保留先前批准范围；不报告新业务恢复或全部组件恢复。

## 真实系统结果

| 环境 | 实际父进程与范围 | 取消及现场核对 | 当前状态 |
|---|---|---|---|
| IDEA/macOS | 新目录 `handoff-cancel-idea-20261107-8924490c`，284 inventory；真实 CM/DS/密码/快照及 Java 写释放；normal PID53819 | 自然过期后实际 recover 仍 PREPARED；错误 token 不改 journal；两份不同本地 UDS 计划各取消一次；取消前后 unit adapter 调用日志、PID、快照、业务事件精确不变；首份取消 journal 原字节归档；原 GWT admin/admin 刷新后 Count9/密码脱敏 | 新 JVM 与测试 server 已停，全部数据保留 |
| OrbStack | `/opt/yofc-handoff-cancel-orb-20261107-171c3eed`，284 inventory，真实完整 Kura 新父 PID702406 | 两组远程21/29绑定、真实 mTLS；自然过期后仅重开本轮 agent702763→703029，原 prepared journal、四个受管 unit 的 PID/Invocation/时间/重启计数、快照和 DS 事件不变；两次取消均未 execute；原 GWT9；中英通过 | 新五个 unit 已停，旧四服务原 PID/包保持 active |
| Parallels | 现有 Ubuntu26.04 ARM64，`/opt/yofc-handoff-cancel-prl-20261107-45ec6e0e`；私有 Java184文件+284 inventory，共483个固定 profile 输入；真实新父 PID4568 | 两组远程21/29绑定；自然过期重开本轮 agent5166→5947，不停父 Kura；实际 CapEff0/NNP1/EROFS30，重开后再次验证；四 unit、快照、业务事件不变；首份历史归档及原 GWT9/中英通过 | 新五个 unit 已停，VM正常 shutdown；Windows仍 stopped |

宿主的 native-systemctl 是明确的 IDEA 协调夹具，只等待 UI 原生启动或停止精确自有 JVM，不自行启动 Java；不把它算作 systemd。OrbStack 仍有原全局 LXC override，因此其请求的沙箱属性不冒称实际隔离；本轮实际严格沙箱由 Parallels 的进程状态和 namespace EROFS 证明。私有 Java 制品只复用已核对的位元输入，不安装系统 Java、不复用旧事务，也不追认旧历史验证的 scope。

Orb 原 prepare `21c97e1c-d80b-45a7-8464-fd858edf3e2d`，原 cancel `49304001-af69-4086-9524-d8fa3df2b331`；后续独立 prepare `04686530-52ed-4274-87f9-eff782cb4a47` / cancel `46bdffab-1cae-49e3-88b4-4ea4bdd98144`。Parallels 原 prepare `a8f3d7b1-efb8-493e-83d4-c87e66452513` / cancel `5dd0f789-0da5-4ee9-b7dc-2042c677bd2a`；后续独立 prepare `d4b9ff8b-33b8-4eed-a9ff-d2bfce42167f` / cancel `db741126-ada2-499e-a8fa-13ef1452d131`。每个环境均是两个不同 plan、各一次取消，**execute 总数0**。第二次 prepare 前验证原 parent/token 并精确归档首份取消 journal；旧 prepare/cancel 的原任务结果保持一致。

两处 native 原生 command_decisions 各4条，private 决定/receipt SHA、完整原请求 hash 与 SQLite 原行逐一匹配。中心新私有 PG 总计15个任务：status20=7，prepare21=4，cancel29=4，均 SUCCEEDED；8个写任务的结果全部加密，4个取消输入均加密。默认 admin 的 must_change_password 仍 true，默认账号未改。没有本轮新的 op29 UNKNOWN 故障注入；实际取消均直接成功，不套用 ops21/22 的历史进程丢失回执结论。UNKNOWN/RR 的拒绝、持续挡写和纯原命令查询合同保持 f2ea1b07 的实现与测试证据。

## 制品、检查和证据

- protocol crate SHA `7a35891343e24c76153f9f2dfef02580f8ea4a4bb1759062bead33c9182de010`；独立 staging 使用摘要路径 vendor，避免旧 target 缓存误用 wire。Linux实际构建源45个记录全部读回匹配（Cargo.toml原始/派生各自检查），manifest SHA `ba24cd7ce1ff208bbc7313d2918bc78024639979e8167404768580c3c92bb1dc`。
- review `0.1.0~review20261107.handoffcancel1`，ELF SHA `bd85f7b98c5e9ae124c368904d2099f6b54d6af11bf477c03f70eb0761f1a939`，deb SHA `9bf88fff3765cec4813238a2bd624e8f8cef8454d081a021e552d29d1ef7c1dc`。只复制当前 ELF到新夹具，review deb未安装到原系统；旧 systemcontrol3 仅ops1–28，不代表本批版本。原 console26/agent20包未升级。
- 宿主常规 agent192/9ignored通过；随后只修改 ignored 原生夹具分支，最终优化构建的完整原生用例1通过、严格 Clippy/fmt通过。Linux当前完整源192/10ignored、严格Clippy、release/test构建通过。本轮新增代码仅为 ignored 夹具的自然过期/外部UDS取消核验和显式 live 浏览器合同，生产取消逻辑沿用 f2ea1b07。
- 两处实际绑定远程决定的 agent/Core 三份纯 guard 合同各5通过，含原 prepare token 漂移的复制现场拒绝；宿主实际 UDS 取消现场的无变更接受合同1通过。Core/agent既有全部包装测试不冒称本轮重跑。
- 本轮实际浏览器：Orb原 ID 续接最终1通过，Parallels全新流程1通过；均双语、两计划/两取消/无execute/历史结果及原GWT9。新 live 测试与配置显式严格 TS 检查通过，两仓 diff-check、agent fmt/Clippy通过。f2ea1b07 的 console24/protocol11、HTTP/PG3及mock回归是沿用，不记作本轮重跑；本轮确实重新编译当前 console并在新PG/mTLS闭环。
- 机读 [evidence](../evidence/handoff-cancel-system-20261107.json)；日志、私有数据库/CA/key、源码旧副本、模板输入、截图和 native/VM 状态在 `target/handoff-cancel-system-20261107/`。截图 `cancel-live-zh.png`、`cancel-live-en.png` 及 `parallels/` 同名，原 GWT截图 `gwt-after-cancel.png`。其余关键证据：`native/native-cancel-check.json`、两处 `native-post-cancel-proof.json`、`center-final-db-proof.json`、`linux-source-readback.json`、`final-state.json`。

## 保留的中间失败与安全续接

宿主 debug 的软件 SHA256 校验开销过高，采样确认在校验输入；停止前证实没有 journal、UDS、request 或 unit 动作，完整保留初始化目录，换独立 optimized state，未重跑 seed。第一次 guard 环境变量误传 fixture.json文件（合同要求目录），保留失败日志后用目录输入验证实际现场。

Orb首个 browser prepare 已获202并记录原ID后，Node API未加载本轮CA而中断；Chrome仅叶证书SPKI信任和Node进程CA都需显式配置，未启全局TLS绕过。后续一次只读加载被5秒断言过早终止，另一次原ID恢复早于登录清空storage而被清掉；修正为35秒产品查询期限对应的40秒断言，并在已登录的网关界面挂载后只恢复原task ID。所有续接都沿用 `21c97e1c-d80b-45a7-8464-fd858edf3e2d`，没有第二个替代 prepare；最终中心/native/SQLite核对确认每个环境仅两份不同计划、每份各一次取消。live入口 create-only 标记和已发取消检查阻止误重跑。所有旧日志、失败和证据保留。

## 收尾及后续台账

新宿主中心54382、PG16652、IDEA父53819及 keepawake 均已停止；宿主独立夹具端口全部关闭。Orb本轮新五 unit停止，Parallels新五 unit停止后正常关VM；两VM stopped。两新测试 gateway已在本轮私有中心 disabled，所有task、DB、密钥、receipt、backup、quarantine和卷保留。原Orb四服务244516/244911/286620/268与console26/agent20包继续active，未触及运行中的target/runtime、个人~/.kura-dev或192.168.2.125。18仓HEAD/branch和旧dirty保留，无reset/clean/提交推送。

本核定只覆盖 f2ea1b07 的系统验证内部 pending。其余内部 pending 保留：其他未决恢复运维，Core完整安装/升级/bootstrap/启动策略交接和非原生入口退役，全组件/metatype/provider/部分或空CM/外部PID的Java离线恢复及完整业务，NM/firewall/NAT/forward/远程流程/早启，一致DB/制品/密钥备份恢复、多轮key轮换/旧系统迁移和正式发行。外部 deferred 单列为真实 NM/cloud-init/物理网络与硬件、amd64、磁盘满/硬断电等；两个旧 evidence gap仍独立保留。本轮实际所选 Parallels Java/Kura环境已解决，不再把它列为本批 blocker，也不追认历史无Java场景。继续保留NM、Kura无agent独立运行、agent常驻与独立远程开关、Java本地密钥/CSR、首登后台闸门、中英、默认关闭配置/firewall及独立归属。

完整目标继续 active。下一从未交付业务台账继续选择实际业务，不重放本轮 seed/恢复/prepare/cancel 或前轮 restart/reboot 一次性入口。
