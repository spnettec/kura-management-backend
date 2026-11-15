# CM API补偿与精确恢复意图合同（2026-11-05，选定真实业务逆向验证通过）

后续覆盖：agent独立逆向事务的实现与实际验证见 `agent-cm-inverse-20261105.md`；本文件保留手工Java分支的原验收范围。

用户已恢复执行。IDEA原生完整Kura/原GWT及Orb新版本实际正向、逆向与故障挡写验证完成；整体产品目标仍未完成。下文区分已验证分支与agent事务尚未交付范围。

## 实现

`ConfigurationRestore` 新增 capture/expected/compensate。expected保留真正CM String/数组表示中的密码密文字节，不能通过重新解密/加密快照推算；capture保存受管理记录。compensate只经CM公开API逆向替换，恢复旧属性与已删工厂、删除新实例、保留未管理记录；原先空受管理集合也可恢复。当前记录必须完全匹配原始或精确目标值；只有删除/创建可解释的缺失才接受。未知受管理PID、第三方字段漂移、重复身份、无关单例占用、在位保留记录消失均在备份/修改前拒绝。逆向修改前再次持久备份当前状态；失败不伪造补偿成功。

bootstrap在首次CM修改前生成 `.cm-intended.aes` 和旧CM备份，并由callback fsync create-only `.cm-plan.properties`，绑定restore ID、源快照hash、旧备份hash、精确目标hash；成功后保存`.cm-applied.aes`，CM_APPLIED增加applied.sha256。备份生成受32MiB制品界限保护，缓冲区清零。4096条受管理记录界限与离线格式保持一致。

实验性Java逆向启动请求可添加完整三项`compensation.source.receipt`、`compensation.source.receipt.sha256`、`compensation.source.plan.sha256`；新最高编号snapshot必须是原CM备份AES原字节、新恢复ID/请求/回执。SCR/业务仍挡住时，严格核对源决定/plan/备份/目标意图及AES认证后调用compensate，生成新的前后证据；普通写仍保持挡住。此分支尚未接入agent rollback的发布/核定流程。

agent可读新applied.sha256，核对实际/预期CM副本及五字段plan，记录compensation证据指纹；重启时漂移继续RECOVERY_REQUIRED。旧回执无该字段可按原范围读取，不自动获得补偿授权。没有自动重发、清journal或解锁写入口。

## 已验证与未验证

受影响Maven reactor424项全过，0失败/错误/跳过；core configuration89，工具26。新增4项核心逆向合同、2项真实Equinox CM重启持久化合同、2项源证据/AES/漂移拒绝合同。Rust126通过、3显式ignored，严格Clippy通过；新增真实证据合同逐项篡改applied/intent/plan都挡住并保留候选/回执。以上没有DS业务启动，不能代表完整Kura恢复。

IDEA原生 v2 `cm-inverse-idea-v2-20261105-98f5b45e` 已完成 seed → apply → compensate；284项清单SHA核对，实际283 bundles。正向真实DS Count=2、工厂删除、旧属性清除和密码解密通过；逆向 keep/remove 均9、旧属性/stale恢复，普通快照写持续挡住。两次均通过原GWT admin/admin中文登录，Count与快照显示一致。只读Java审计原CM备份与逆向应用副本：26条受管理配置、187个属性完全相等（含密码密文原字节，仅排除CM框架身份字段）。来源请求/回执/plan/backup/intended/applied与原目标SHA未变。三个新JVM均已停止，现场保留。

首套 `cm-inverse-idea-20261105-8f79a6c2` 的seed完成后，窗口共享导致桌面输入不生效；人工操作误重跑该套seed，已立即停止并保留为unexpected-seed-restart，不能作为通过证据。用户关闭共享后控制恢复；新v2测试探针增加level1重复seed拦截后才运行。没有对旧请求重放，也没有把首次误运行抹去。

随后在现有Orb冻结源码独立构建，Rust126通过/3ignored与严格Clippy通过。新夹具 `/opt/yofc-cm-inverse-20261105-53012dfb` 实际UDS execute用时72.894秒，返回AWAITING_COMPLETION并保存新补偿证据。首版测试脚本错误要求DS同值updated只出现一次，故停服保留；修正为允许所有事件均为预期值后只继续新逆向请求，未重发正向请求。逆向准备脚本另一次缺少os导入在发布请求/启动前退出，确认候选SHA与无请求/unit后继续该候选，没有重放CM操作。

Orb新显式Java逆向请求/独立unit完成真实DS keep/remove=9、旧值恢复及挡写；原GWT admin/admin中文登录Count=9与snapshot_2/1/0通过。只读审计同样26配置/187属性完全相等（含密文）。来源所有证据SHA不变。错误来源plan指纹的另一个新请求返回RECOVERY_REQUIRED，在CM修改与业务激活前拒绝，无新CM备份/意图制品，正向/逆向业务事件均未变化。

最后仅重启新proof agent读取原pending journal，不调用execute/rollback、不重启Kura：精确补偿证据保持，已停止Kura触发RECOVERY_REQUIRED，新upload持续拒绝。当前版本“待完成Kura仍运行时agent正常重启保留AWAITING_COMPLETION”的正例未重测，不能沿用旧版本扩大结论。所有新IDEA JVM、Orb Kura/agent/proof/reject units均停止/MainPID0；原Orb26/20三服务及phase2-agent仍active。新夹具unit显式SuccessExitStatus=143，不改变旧failed现场，也不代表发行unit已修复。两Parallels VM实时stopped；缺Java/Kura仍deferred。

## 下一步与边界

本轮选定真实业务逆向补偿已验收；**整体恢复未完成**。Orb逆向是显式Java bootstrap请求，尚非agent rollback闭环；CM_APPLIED仍fullRecoveryVerified=false/businessHealth=UNVERIFIED，普通写保持挡住。用户已解锁并恢复任务，历史桌面锁定不再构成阻塞；目标控制器仍返回blocked，已请求用户在目标面板恢复自动执行（工具不能自行resume），不是用户要求暂停。

helper允许空原集合，但bootstrap当前非空snapshot准入仍拒绝空旧备份，该启动边界pending。完整snapshot文件集补偿、agent CM逆向事务/来源授权/崩溃核定、桥接/业务健康及明确完成后释放写、外部PID当前Rust权威和完整组件适用面均pending。不可删除请求后普通重启；实验性最高AES快照的后续普通启动/快照文件一致性尚未交付。正式unit退出143处理、独立UID原key授权和发行打包仍未完成。

完整原目标继续保留：防火墙适配/NAT/远程/早启、restart/reboot闭环、一致备份恢复、身份轮换运维和正式发行；真实NM/cloud-init、物理有线/硬件、amd64、磁盘满/硬断电等外部deferred单列。旧夹具/state/keys/备份/quarantine/卷均保留，无提交推送reset/clean/远端覆盖或192.168.2.125连接。

证据与日志：`target/cm-compensation-20261105/`；结构化合同状态`../evidence/cm-compensation-20261105.json`。核心子仓图谱已刷新并核对新增源码覆盖，未索引父目录；全工作空间18HEAD/分支与上轮一致。


历史desktop-gate-turn1–3记录只保留当时锁屏事实，已由本轮用户解锁/关闭共享后的实际验证覆盖。详情见idea-v2.json、orb.json与两份full-cm-inverse-audit.json；首套失败与脚本失败证据均保留。
