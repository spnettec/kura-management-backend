# 受保护普通启动与独立原快照格式授权（2026-11-06）

目标 active，整体管理体系仍未完成。本轮承接已交付的 CM 逆向及原快照文件集补偿，新增保持写入阻挡的原启动路径核验。合同见 agent `docs/guarded-normal-startup-v1.md`；证据见 `../evidence/guarded-normal-startup-20261106.json`，日志位于 `target/normal-boot-verification-20261106/`。

新增本地管理员 UDS `recovery.normal.prepare/execute`，输入只有 ID/token，执行须新 ID/token。第三份固定 normal startup grant/unit/request 独立于正/逆向，默认关闭。完整源 journal/CM 及 sidecars、原备份/manifest、活动原文件和元数据、保留候选及文件集回执均在 prepare、执行前后和重读核对。意图先落盘再发布一次性请求，不自动重发，失败停止三条固定 unit 并保留 RECOVERY_REQUIRED。

Java bootstrap 的 normal-read-only 模式在 SCR/业务阻挡期间，认证 inverse CM_APPLIED 来源及精确加密 CM-applied，使用公开 CM API 核对当前值，保留加密观察和启动级别账本。bootstrap 本身不重应用 CM；核心受控准入返回无替代列表，实际走原普通 snapshot/CM 加载。原路径正常的 CM seeding/merge 仍会执行，但普通管理写始终被挡。受保护模式禁止空快照删除、改名 .bad、加密转换与解析回退，并绑定实际读取目录和规范最高快照；不带请求的原 Kura 行为保持。新回执 NORMAL_BOOT_ADMITTED 不等于业务健康或恢复完成，agent 只到 AWAITING_NORMAL_BOOT_COMPLETION，三个完成/健康字段仍 false/UNVERIFIED。

真实原 snapshot_0.xml 含 3 个 legacy 密码字段。严格 authenticated 上传授权正确拒绝直接将它用于 normal 核验，未发布请求、未启动。为此新增独立可选 NORMAL_VALIDATOR_CONFIG，显式授权原快照的格式/key policy；原上传授权仍 authenticated，无自动降级。正常 CM 来源仍须通过同一启动请求 crypto 的认证和精确公开 API 核对，策略及工具/key fingerprint 持续固定。

宿主 Rust 140 passed/5 ignored、严格 Clippy；Linux 同版 140 passed/6 ignored、严格 Clippy。多一项 Linux ignored 是此前实际挂载原语入口，本轮不重放。新增五组合同覆盖独立授权/token/非终态、执行前来源/目录/启动配置漂移、发布中断与错来源停三 unit、不重放、重读观察漂移和独立旧格式授权 fingerprint。Maven 受影响 reactor 427 passed，含 core configuration 90、offline tool 28；新增保护路径/目录/写闸门及来源认证合同通过。无 UI 产品源码变化，不重放无关前端/发行验收。

IDEA 全新 v2 `.phase1-dev/normal-boot-v2-idea-20261106-01cc754e`，HTTP18405/HTTPS18803/认证18804，库存284/实际283 bundles。UI 启动 seed、agent 正向请求后 UI apply、逆向请求后 UI compensate，原 GWT 中文 Count2→9、真实 DS/密码/普通 snapshot 写拒绝通过。新文件集恢复原1文件并保留2候选后，严格策略在 normal prepare 拒绝 legacy 原文件，无 normal plan/request。新独立 baseline grant 与新 normal-only continuation 入口只接续该已核定文件集，未重放正/逆向/文件集入口；随后 UI normal 启动及原 GWT admin/admin/Count9、真实两个工厂实例与密码通过。原 snapshot0 没有 revisionFixture 记录，此值来自实际 CM/原路径，未按文件存在推断。新显式 normal continuation 测试1 passed，重读及挡写通过。Java 独立只读审计 normal 放行前26配置187属性含密文与最初 old CM 精确一致；不冒充放行业务后全 CM 审计。

本轮首份 native a322fb71 的驱动提前结束，测试进程退出时保留 STARTING、CM_APPLIED 和实际已通过的正向 DS 证据；只停止该新 JVM，未重发/删除/手改 journal。该首份仍为中断现场，不计为完整恢复通过。v2 完整链入口因严格 legacy 拒绝而 FAILED 亦如实保留；之后新 normal-only 入口单独1通过。两份夹具均保留，全部新 native JVM 已停。native adapter 仅协调 IDEA 进程，不称 systemd。

Orb 全新 `/opt/yofc-normal-boot-20261106-a72f7377`，HTTP18406/HTTPS18805/认证18806；实际 UDS/systemd/完整 Kura 正向5.363秒、逆向12.944秒、文件集恢复和 normal15.779秒通过。原 GWT 中文 admin/admin/Count9、两工厂/密码、原1活动文件字节/元数据、2保留候选及放行前26/187精确 CM 审计通过。真实 agent 重启保持 AWAITING_NORMAL_BOOT_COMPLETION，normal Kura PID不变，前两 unit PID0，无启动重放，新 upload 仍被挡。真实 grant 中 original authenticated 与独立 baseline legacy 分开。VM只有 JRE，审计用宿主同版 Java21 编译 class，未安装/假称有 javac。

验证后按资源边界停止新 agent/normal/inverse/forward：均 inactive/dead、MainPID0、Result success。最后 journal 仍 AWAITING_NORMAL_BOOT_COMPLETION；这是主动停服后的保留状态，不声称在停 Kura 后又通过 active-unit reopen（后者按合同会转 RR）。库存284 SHA 与 Linux agent release SHA一致。原 Orb 四服务 active，console26/agent20原包未改。Parallels Ubuntu/Windows 实时均 stopped；完整 Java/Kura 环境上轮确认缺失，本轮未开机、不重放已闭环挂载原语，相关验证继续 deferred。

所有18仓 HEAD/分支不变，完整 dirty/untracked 及 source SHA 记录保留；未提交/推送/reset/clean/覆盖远端，未接192.168.2.125。无旧夹具/key/卷/备份/quarantine删除；不动个人~/.kura-dev或其他运行集。两个真实子仓图谱重索引成功，受影响路径metadata_match；不是父工作空间索引或全解析完成声明。

| 原目标业务 | 本轮后核定 |
| --- | --- |
| Java 恢复/补偿 | 精确CM逆向、原快照文件集及保持闸门的普通加载已实现并获选定真实业务证据；正常无闸门启动、桥接/全业务健康和明确完成释放仍 pending |
| 原/部分CM与组件面 | 部分CM无CM_APPLIED来源、空原集合bootstrap、全组件/factory/metatype及外部PID当前权威仍 pending |
| 防火墙 | Kura适配、NAT/forward、原UI/远程流程及早启仍 pending，默认关闭 |
| restart/reboot | 远程任务与重连/主机重启后的完成闭环仍 pending |
| 一致备份与发行 | 整库/制品一致恢复、多次身份/key轮换及授权、旧系统迁移、正式发行/运维界面仍 pending |

外部 deferred 单列：Parallels完整Java/Kura、真实NM/cloud-init、物理有线/硬件、amd64、磁盘满/硬断电等。保留NM、Kura独立、agent常驻、Kura后端生成CSR、双语及首次设密后台闸门的产品核定不变。
