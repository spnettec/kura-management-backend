# 2026-11-08 3e81381f 正向恢复、派发日志释放与普通运行交接

从 bc82f837 续接，入口实时核对 43031 个 source/artifact/evidence 指纹、18 仓 HEAD/branch/dirty、原 Orb 四服务与两台已停 Parallels。保留工作树和所有新旧现场，无提交、推送、reset、clean 或 UNKNOWN 重发。原一次性验收入口未重放；本轮增加了专用的新用途入口和独立夹具。

## 实施与合同

UNKNOWN 可能留下“CM 已改变、旧快照未改变”。强制执行逆向后再恢复原快照集，会让这两种不同状态重新冲突。因此新增默认关闭的“接受已经应用的正向恢复结果”路径。它接受的是选定候选经正向恢复后的结果，不能直接认可或重发原 UNKNOWN。

`AGENT_RECOVERY_FORWARD_NORMAL_CONFIG` 是独立私有授权，固定 source=applied-forward、acceptAppliedForwardSource=true 和 normalStartupAuthoritySha256。该摘要绑定完整 normal startup 授权及有序 profile 文件 SHA。拒绝未知字段、symlink、宽松权限及变更的授权。新增本地管理员方法 `recovery.normal.forward.prepare` 只收原事务 ID/token；普通 Kura 注册角色不能调用，调用方不能选择路径、unit、source 或摘要。旧逆向入口不改变意义。

正向 normal 准备要求完整原 CM_APPLIED 回执与 sidecars、未变更的原文件集加选定最高候选、无 inverse、无旧 normal plan。先持久化派发意图，再停正向 unit、发布新 normal request 并只启动一次。失败保留 RECOVERY_REQUIRED，不重发，也不能改走逆向。后续 health、completion 和无 REQUEST 的 ordinary profile 继续各自独立授权。未构造虚假 inverse 回执。旧 journal 缺省仍解释为 inverse，旧序列化形状保留。

实际使用当前 Core V1 快照时，又发现 SnapshotTool 只接受旧根属性。补齐 `BASE64_LOGICAL_V1` 格式，但不把 Base64 当作 AES 认证：明文 V1 要显式 `passwordPolicy=logical-v1`，报告 logicalPasswordCount/encoding 且 keyAuthenticated=false；默认 authenticated 对 V1 密码要求外层实际通过 AES-GCM。legacy 不静默接受 V1，未知编码、错误 Base64/UTF-8 拒绝，密文形状的逻辑文本不再被猜成 AES。未标记旧格式及错密钥拒绝合同保留。新 bootstrap 还要求 snapshot.codec 1.x，后者要求 password.sources 1.x。Rust 校验器验证计数、编码、策略和认证标志，拒绝伪造的 V1 明文认证声明。

新实际正常启动的业务/密码检查通过，但严格 managed CM 比较失败。停止后仅在复制的真实 Equinox CM store 中做只读取证：25 个配置，仅 HttpsKeystore/SSLKeystore 的 keystore.password 密文变化，解密值相同，其他属性不变，未启动 SCR/Core/业务 bundle。原因是普通快照加载为尚未注册的组件重新加密密码。

Core 新增包私有 StartupPasswordConvergence：受保护 normal 启动在任何 CM 派发前捕获当前 managed CM，将逻辑密码逐值与现存密文解密值核对，保留原密文和数组/Vector/List 容器；缺失、变值、非法非空密文、容器或 factory 身份不符即失败。normal 的 singleton 也改用 updateIfDifferent。普通启动继续独立加密路径；严格 whole-managed-CM verifier 没有加密码相等豁免。公开 org.eclipse.kura.api/DTO 未改，本轮 Core 增量为内部实现；前轮公开 release overload 的历史仍保留。

## 验证与真实现场

Rust 229 通过、11 忽略，Clippy --tests --no-deps -D warnings 通过。新增正向源默认关闭、来源与库存绑定、授权变化/错误 token、失败后禁重发与逆向分支、模拟 health/completion/ordinary 组合、本地角色/参数权限和 V1 报告伪认证拒绝契约。模拟回执用例不冒称真实恢复。

Core 238 全过/install，含 5 个密码收敛测试，实际 Core 加载器参与：保持现有密文；第二个配置密码不匹配时任何 startup CM writer 都不调用；普通启动保持原路径。工具 62 全过/install，并在 Core238 安装后再次全过。前轮 XML8/Asset38/NM1497/GWT210 等范围未重跑；不能将旧结果算作本轮额外覆盖。

新专用 ignored 入口 `actual_applied_forward_journal_handoff_with_explicit_fixture` 经显式新夹具启用，不调用旧 inverse/completion 验收入口。native-v6 使用当前 Core238/工具62 exact SHA、283 个 inventory artifact，实际框架报告 282 bundles；两种计数分别保留，不声称全 bundle/业务验收。原 conditional update 真正将业务 CM count=1 改为 2，CM 后、打开 snapshot 文件前一次故障，旧快照仍为 1，留下一个未决 intent，原写入不重发。

在这个新现场执行真实生产 Recovery/Local 代码、Java bootstrap、CM 应用和独立 native IDEA 启动：正向 PID76951、normal PID77067、ordinary PID77362。正向恢复选定 count=1；normal health 的 25 个 managed 配置、181 个属性严格匹配，真实 DS 业务与非空密码通过；独立 completion 发布准确回执及 journal release-link；normal 停后直接交接无 kura.recovery.request 的 ordinary JVM，中间普通写 0、inverse 0。Rust journal 到 ORDINARY_ACTIVE_APPROVED_CHECKS，专用入口 1 通过。

对 release-link 按生产二进制格式核对 intent SHA、completion/snapshot/decision 身份、绑定 receipt 绝对路径与实际 SHA，全部匹配。agent 测试驱动退出后，普通 JVM 成功写出一个新快照，原快照字节保留；原英文 GWT admin 登录 Status，Journal Business count=1、密码遮蔽、Apply disabled、无冲突条，UI 写入 0。截图在会话中展示，未保存图片文件，不将 UI 观察伪称为截图文件 pin。随后自有 JVM 全停，数据、密钥、快照、请求、回执与日志均保留。

这里的“实际授权链”限于固定本地私有授权、生产 Recovery/Local 与实际 Java UDS/CM、受选业务。native-systemctl 只协调 IDEA，不是真 systemd；未运行新 packaged agent daemon/mTLS center/系统沙箱。夹具显式授权明文 logical-v1，未声称明文快照有密码或 artifact 加密认证。全 Kura、所有业务及正式发行仍未完成。

## 失败与现场保留

初始 PATH 无 cargo、rustup 链接损坏的两次命令 127 保留；使用已有实际 1.95.0 二进制和隔离 Cargo target，不修全局。最初模拟断言将只读 show 算成派发、误读 ordinary 返回键，修正夹具后全绿；失败源码与日志保留。Core 首次新增测试编译有 var 复合声明错误；第二次是 Map 比较断言，均保留，再次 238 全过。工具两次新增测试的 Map/Password 身份比较失败也保留，修正逐值比较后 62 全过。

六份新现场：初份因 metatype 预建配置而在 UNKNOWN 前夹具失败；v2 原工具拒绝 V1、无恢复事务，暂存保留；未优化前置校验在上传/事务/请求前停止后换优化构建；v3 夹具沿用普通 startLevel=6，被受控启动要求 5 拒绝并保留 RECOVERY_REQUIRED；v4 真实 normal health 报告密码密文差异，拒绝 completion；v5 IDEA 启动迟于 30 秒固定动作窗口，迟到 CM_APPLIED 未获 agent 接受，停止自有迟到进程并保留失败；v6 修正密码收敛并预先选好 UI 配置后贯通。失败的请求都不重发，不将失败记录计为成功。

一次运行配置选择输入误落旧 Kura_Password_Boundary XML 编辑器，保存了误输入副本，只移除精确前缀，原 XML 后续字节与六个原运行参数核对恢复，未启动旧入口；审计在 editor-prefix-repair.json。此后先确认弹层再输入。

## 台账与续接

14 项内部 pending 全保留，仅关闭本轮选定的正向接受、V1 工具支持、guarded normal 密文收敛及实际 pending-journal 释放/普通运行交接子项。客户端 operationId/receipt 查询、creation/deletion lost-reply 对账、非幂等失败与全业务恢复、legacy/direct CA/provider/服务器期限、正式安装升级、原子备份/回执路径迁移仍 pending。同 PID SCC/非 SCC 枚举覆盖问题、零字节坏快照策略未修；原初始 GWT CONFLICT 精确 holder 及历史证据 gap 保留。

外部 NM/cloud-init/物理有线、硬件、amd64、磁盘满/硬断电等 deferred 按父清单保留。本轮无新 Linux 部署/验收，原 Orb 四 PID/包未变，两台 Parallels 保持 stopped，不访问 192.168.2.125，不清个人或旧 runtime。知识库只写父工作空间 scope=ws，Tasks/Session Summaries 和新记录精确读回，Home 不改。

证据：target/authorized-journal-handoff-20261108；清单：docs/evidence/authorized-journal-handoff-20261108.json。目标 active，不宣称整体完成。下一步先按清单核对，不重跑这些一次性入口，再推进客户端 operationId/receipt 与 lost-reply 业务对账及完整台账。
