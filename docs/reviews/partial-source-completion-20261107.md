# 部分失败来源的所选业务完成与独立启动（2026-11-07，ff72eb79）

完整目标继续 **active**，上一轮 b9eb0ad3 为 progress。本轮关闭该轮“部分失败来源、非空原始 CM 备份之后，所选业务的文件集/正常加载/健康/明确完成与独立启动”验收缺口。空原始 CM、其他不一致或部分原始 CM 的策略、完整生产组件适用面和正式发行仍待交付。

## 改动与验证范围

产品 Rust/Java/GWT/React/协议和生产 ELF 未改。新增两个部分来源跨阶段合同，扩展显式忽略的真实 IDEA 测试，并更新五份恢复合同。新 Java 业务观察器和注入器仅在本轮独立夹具中；未安装或覆盖原系统包。

宿主和 Linux 各7个部分来源合同通过（5个原合同回归+2个新增），最终 fmt/严格 all-targets Clippy 通过。显式真实 IDEA 测试另1项通过。上一轮宿主198项全量不冒称本轮重跑。Linux 在全新独立源码树编译合同，108输入与当前工作树匹配；旧构建树保留。图谱仅刷新真实 agent 子仓库，best-effort coverage 不等于完整解析保证。

新增合同验证：候选文件移入保留目录后，原失败 receipt/source_partial 不变，不能隐式正常启动或释放写入；保留 checkpoint 漂移后留住归档、进入 RR、不重发原请求。普通路径的独立 grant 仍默认关闭。

实际完整运行顺序为新 IDEA → OrbStack → 现有 Parallels。每处新 home/CM/state/密钥目录和独立请求；每处 inventory284、实际 bundles283。只复制冻结静态代码、工具和私有JRE，不复制旧运行状态，不执行旧一次性入口。

## 真实链路

1. 正常新 Kura seed9，原 GWT admin/admin 登录后才生成本轮原快照。原 GWT 的专用开发首登例外仅在新测试 home，产品默认账号和后台强制改密闸门未改。
2. 新 apply JVM 在真实 Equinox CM 第一项 updateIfDifferent 写入业务2、API回读2后，于第二项抛出受控 IOException。SCR/业务启动级别不开放；agent停原unit、RR持续挡写、原 cmApplication=null，无伪造 CM_APPLIED 或 applied 侧车。
3. 新独立 inverse 恢复原 CM。真实 AES 审计为原备份9/9 → 故障现场新旧混合2/9 → 补偿applied9/9；完整原27记录匹配，DS实际秘密解密、foreign77保留、普通写继续挡住。
4. 独立 snapshot-set 逐项恢复原活动文件集及元数据，原正向/逆向候选原子移至私有保留目录，不删现场、不启动Kura。
5. 独立 guarded normal 从最高原快照走普通加载路径，CM与真实业务仍9/秘密正确，snapshot/update/rollback仍挡住，原GWT再登录读9。原始27记录精确保留，Console的1项新增自配置记录仅因独立additionalSelfPids显式授权才可进入健康报告。
6. 实际 Java UDS whole-CM及两个所选组件注册/真实业务检查通过。健康报告源27、managed28、property203、框架级别5、normalLoad/writeGuard/CM匹配；没有忽略未列出的配置。健康成功仍挡写。
7. 独立 completion grant 及新 ID/nonce 批准后，Java重新检查真实业务，持久发布一次写释放回执，agent同步核定 COMPLETED_APPROVED_CHECKS，scope=administrator-selected-checks。原 source_partial、失败receipt及原cmApplication=null持续保留，绝不把原请求改成应用成功。
8. 仅在明确释放之后，各处原 GWT 单次保存9→10；实际 CM、DS、秘密解密及真实新快照同时证明。随后无恢复请求的独立 ordinary 进程从10启动并实际写11/新快照，原账号登录读11/秘密脱敏通过。

Native五个完整JVM均由IDEA UI启动；直接生产Rust引擎与实际本机UDS由显式测试服务协调，未启动生产agent daemon，生产unit命令窗口始终30秒。ordinary启动前测试服务已退出，验证Kura不依赖agent。两VM用同一当前生产ELF、实际main UDS/systemd/30秒合同；各写步骤只执行一次，多次agent重开保持原JVM/原journal，不重发。完成后的合法GWT/ordinary变更不会被当成未完成恢复漂移。

| 环境 | seed / inverse / normal / ordinary PID | 固定 profile | 停服后现场文件 pins |
|---|---|---:|---:|
| IDEA | 92147 / 92356 / 92661 / 93000 | 315 | 755 |
| OrbStack | 725911 / 726354 / 726889 / 727544 | 318 | 740 |
| Parallels | 2647 / 3589 / 4870 / 6250 | 502 | 925 |

Native失败apply PID92263已停；两VM失败apply由agent实际确认停止，没有额外声称采集其瞬时PID。Native源实际AES混合dump25记录，原备份/恢复applied各27；VM同样逐项审计。记录数不代替值、类型、秘密和业务验证。

Linux ELF保持SHA256 `539244e2c73a36a6ccc8c8fd200311d87bf4ed61f3aae4fc4263498bcad80689`，macOS release保持 `89a55d60e17a2b53bd5fd1cc511eb5c8869edd24bf0974772e6ad5c24a18cad2`。它们是上一轮冻结静态制品；本轮产品代码未变且在新实际链路中重新核验，未伪称新正式包。

Parallels含私有Java184文件，末次agent重开后再次实测CapEff0/NoNewPrivs1/EROFS30。Orb全局LXC override仍readOnlyEnforced=false，不将requested属性当实际只读隔离，也不改全局设置强行制造通过。

## 最终现场与边界

所有本轮新IDEA JVM/测试服务、Orb及Parallels各六单元已停，fixture/state/密钥/备份/请求/归档/quarantine/卷均保留。三处journal终态保持COMPLETED_APPROVED_CHECKS，selectedPartialRecoveryVerified=true；广义fullRecoveryVerified/kuraRecoveryVerified/normalBootVerified仍false。不是全网关恢复、自动生产unit/profile交接、开机策略或全部生产业务验收。

Ubuntu正常关机、Windows保持stopped。原Orb四服务原PID244516/244911/286620/268及console26/agent20包保持active。18仓HEAD/branch/全部旧dirty保留，无reset/clean/提交推送，无新中心/PG/Vite/防休眠进程，未连接192.168.2.125，未动个人运行目录。证据根 `target/partial-source-completion-20261107/`，机读核定 `docs/evidence/partial-source-completion-20261107.json`；本目录所有已admitted入口一次性，禁止重放。

## 继续交付

内部pending：空/部分原CM及其他不一致baseline的正常启动/健康/明确完成策略；完整生产组件与外部PID恢复；0/±1标量、其他类型/元数据/图标及完整GWT编辑；其他未决运维/Core完整安装升级/bootstrap/启动策略交接与旧入口退役；NM/firewall/NAT/forward/远程/早启；一致DB/制品/key备份恢复、轮换、迁移和正式发行。本轮非空、原始baseline一致的部分失败来源所选链路已闭环，不再笼统列为未实施。

外部deferred：真实NM/cloud-init/物理网络硬件、amd64实际目标、磁盘满/硬断电。本轮受控CM IOException不是这些故障验收。两历史evidence gap单列；Parallels Java不再缺环境。已闭环的取消、restart/reboot及旧恢复入口不重做。下一轮先核对最终SHA和18仓完整status，再继续尚未交付业务。
