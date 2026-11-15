# 快照文件集补偿与真实挂载沙箱验证（2026-11-06）

目标 active，整体管理体系尚未完成。上一轮 agent 独立 CM 逆向事务已通过真实验证；本轮继续补齐其后活动快照文件集，未重放任何历史入口。合同见 `kura-management-agent/docs/snapshot-set-compensation-v1.md`。

## 本轮实现

新增本地管理员 UDS `recovery.snapshots.prepare/execute`，沿用默认关闭的恢复/独立逆向授权与600秒响应预算。prepare需要原inverse ID/token，只接受已证明CM逆向应用的事务，核对正/逆向回执和全部sidecars、两代原journal锚、完整原快照备份manifest/字节、整个活动目录文件与元数据；生成新set ID/token及新源锚，不停止unit、不移动候选。缺失、漂移或未知项不能被猜测为旧值。

execute需新ID/token，再次核对、停止两条固定Kura unit并复核后持久化意图，将两份新增候选原子移入受保护保留区，每次移动同步源/目标目录后持久化进度。使用Linux renameat2/RENAME_NOREPLACE与macOS renamex_np/RENAME_EXCL，不覆盖目的地，无copy/unlink降级。原文件集合按字节/owner/group/mode保持；旧备份、请求、计划、回执、key及账本保留。原始文件始终由本事务保留，故验证后不重新覆盖它们；第三方删除/修改或未知文件会拒绝，不能借补偿清现场。

保留区位于快照授权目录内 `.recovery-retained/<set-id>`，有私有目录owner/权限及不可覆盖的ownership marker。仅精确匹配且验证归属的保留根从agent活动清单排除，其他未知目录不忽略。Kura原非递归snapshot文件名枚举不将其当快照。此布局避免把候选移到状态目录时跨systemd独立RW bind mount；仅比较底层st_dev不足以保证rename可用。

完整文件/备份/应用证据复核通过后，创建SNAPSHOT_SET_RESTORED回执并到AWAITING_SNAPSHOT_COMPLETION。普通配置写与upload仍阻挡，`kuraRecoveryVerified=false`、`normalBootVerified=false`。agent重读只验证现场与两条已停unit，不启动Kura。任一中断（包括已原子移动、尚未写进度的窗口）或后续漂移停止双unit并保持RECOVERY_REQUIRED，无自动补全、回退、重发或清理。

## 已验证

宿主135通过、4显式ignored，严格Clippy通过；新增五项合同覆盖完整原目录（含.bad及普通文件）/元数据、真实不覆盖移动、备份/活动文件/应用证据/源锚漂移在停服前拒绝、移动与journal之间崩溃留现场、重读故障不重启/不解锁。Linux冻结同版源码135通过、5ignored与严格Clippy通过。多一项ignored为必须指定全新实际沙箱的Linux入口。Java生产源码未变，原424项Maven证据沿用其原范围，不冒充本轮新Java覆盖。

IDEA全新 `.phase1-dev/snapshot-set-idea-20261106-bf630f68`，HTTP18402/HTTPS18797/认证18798，284库存/实际283 bundles。原生seed、agent正向请求/IDEA apply、agent逆向请求/IDEA compensate、原GWT admin/admin中文Count2→9通过。真实DS及密码/普通快照写闸门通过；Java只读审计26配置187属性含密文字节精确恢复。随后通过agent真实文件集代码停止新JVM，恢复原1份活动文件（snapshot_0.xml）及元数据、保留新2候选，八项来源字节按保留后位置核对未变。显式native测试1通过，journal保留AWAITING_SNAPSHOT_COMPLETION，全部新JVM已停。native unit adapter仅协调IDEA进程，不称systemd。

Orb全新 `/opt/yofc-snapshot-set-20261106-58f4df68`，HTTP18403/HTTPS18799/认证18800，实际UDS/systemd/完整Kura正向5.400秒、逆向12.977秒，原GWT中文Count9及26配置187属性审计通过。独立新set准备/执行恢复原1份活动文件、保留新2候选，原元数据与来源字节一致；重启真实agent保持AWAITING_SNAPSHOT_COMPLETION，两条Kura unit始终MainPID0，无重启/重发，新upload被挡。最终新agent/正向/逆向unit全停、Result success。原Orbconsole26/agent20三服务及phase2-agent active，原包未改；284项库存及实际agent release二进制SHA匹配。运行日志/数据/保留目录均保留。

Parallels仅复用现有Ubuntu26.04 ARM64 UUID8061d57e-0120-42ec-a9a9-5799b436c4eb。新 `/opt/yofc-snapshot-retention-prl-20261106-18c7a8b2` 在实际ProtectSystem=strict、ProtectHome/PrivateTmp/NoNewPrivileges和两条独立ReadWritePaths下运行同版Linux优化测试二进制。内核mountinfo证明状态/快照挂载ID不同；跨挂载原子移动被拒绝且源保留，快照挂载内部移动成功并保字节/元数据。显式实际合同1通过，新unit停止MainPID0/Result success。VM正常关机，Ubuntu与Windows均stopped。VM实时仍无Java，因此此处只证明文件保留原语及有效沙箱，不是完整agent/Java/Kura业务验证；该范围继续deferred。旧夹具/状态/包/unit未覆盖。

所有本轮外部事务只调用一次execute，超时只查询；测试中的重复拒绝合同只用全新内存/临时假unit场景。未删除任何旧夹具、卷、key、备份、quarantine；本轮只对新夹具中本事务创建的候选执行保留移动。无提交/推送/reset/clean/远端覆盖或192.168.2.125连接，不清个人~/.kura-dev及其他target/runtime。

## 后续未交付与环境边界

正常启动路径及完成后释放仍pending。本轮明确未删除请求、未普通重启；恢复原快照文件与CM逆向回执不能直接证明正常启动一致性。原snapshot_0.xml未含revisionFixture记录，须以当前CM与正常启动的真实行为核验，不能按快照中有Count9推断。后续应在新夹具中建立保持闸门的启动核定，再完成桥接/业务健康与明确完成。部分CM无CM_APPLIED来源、空原集合bootstrap、全组件/factory/metatype与外部PID当前权威仍未交付。

完整原目标继续：防火墙适配/NAT/远程/早启、restart/reboot闭环、一致备份恢复、身份运维/独立UID-key授权、正式发行与运维界面。保留NM/Kura独立/agent常驻/Kura生成CSR及双语/默认关闭/UNKNOWN不重发要求。Parallels完整Java/Kura、真实NM/cloud-init、物理有线/硬件、amd64、磁盘满/硬断电等外部deferred单列。

证据 `../evidence/snapshot-set-compensation-20261106.json`，详细日志 `target/snapshot-set-compensation-20261106/`。agent真实子仓图谱已更新，新增模块与相关核心读取路径coverage均metadata_match，未索引父工作空间。全18仓HEAD/分支及未提交状态按本轮前后清单保留。目标保持active。
