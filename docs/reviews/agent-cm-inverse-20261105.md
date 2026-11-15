# Agent 显式 CM 逆向事务与真实业务验证（2026-11-05）

用户已恢复继续。本增量接通本地管理员的独立逆向事务，不把CM回执扩大为整机恢复完成。合同见 `kura-management-agent/docs/cm-compensation-v1.md`。此前 `cm-compensation-20261105.md` 证明的是手工显式Java逆向；本轮证明由agent准备、发布、启动、读取回执并持久核对的路径。

## 实现与边界

可选 `AGENT_RECOVERY_COMPENSATION_STARTUP_CONFIG` 默认缺省关闭，必须已有正向startup/Java validator配置，逆向使用另一条allowlist unit、另一请求路径及固定profile指纹。原请求永久保留。local UDS管理员可用 `recovery.compensation.prepare` 提交原事务ID/token；仅接受已证明的CM_APPLIED且有新补偿sidecars的AWAITING_COMPLETION/RECOVERY_REQUIRED，不给历史无补偿字段回执或未知部分应用自动授权。调用者不能指定任意来源、unit、key或命令。

prepare精确复核源请求/回执/CM backup/intent/applied/plan/账本、原Java/key及新profile授权、完整当前快照集合，持久create-only原journal锚，产生新恢复ID/token及COMPENSATION_PREPARED；不停止unit、不发布请求、不修改CM。execute需新ID/token，再次复核后停止正向unit，确认逆向unit停止，在发布新最高编号AES旧CM副本及独立请求前持久化逆向意图。Java保持SCR/业务启动门禁，公开CM API逆向恢复；agent只在完整匹配新回执/候选/来源与独占unit后进入AWAITING_COMPENSATION_COMPLETION。

该状态保持fullRecoveryVerified=false/businessHealth=UNVERIFIED，不发布终态成功、不清上传/请求或解除普通写。重启agent只核对证据与unit，不重启Kura或重发请求；逆向意图中断或证据/unit不一致停止两条受控unit并持续RECOVERY_REQUIRED。原rollback保留保守停服/留现场语义，有inverse时同时停两条unit，不绕过独立准备/执行授权。两条新RPC沿用600秒响应预算，OUTCOME_UNKNOWN只查询。

## 验证结果

宿主单元130通过、3显式ignored；之后新增的受环境控制native入口单独显式1项通过，最终测试集合另含该默认ignored入口。严格Clippy通过。Orb冻结同版源，130通过、4ignored及严格Clippy通过，实际使用独立release二进制；未替换已安装agent。新增四组合同覆盖独立ID/token/禁用及旧回执拒绝、来源/快照/原journal锚漂移修改前拒绝、意图崩溃停止双unit且不重发、非终态重启与挡写。原Maven424证据按未变Java版本继续保留，本轮无Java生产源码变更，不重跑本轮未受影响的Java测试范围。

IDEA成功夹具 `.phase1-dev/agent-cm-inverse-idea-v3-20261105-230c592c`：284项库存/实际283 bundles，seed → agent正向请求 → IDEA原生apply → agent独立补偿请求 → IDEA原生compensate通过。真实DS正向2、逆向keep/remove9、旧属性/stale与密码解密/普通snapshot写阻挡通过；原GWT admin/admin中文登录Count2/9通过。只读Java审计26配置187属性含密文原字节完全相等，原八项来源文件SHA不变。native unit adapter仅协调原生IDEA进程，不能称真实systemd。最终新JVM已停，journal保留AWAITING_COMPENSATION_COMPLETION。

失败与调整现场保留：首套b32532d1因协调过早在agent请求发布前启动apply，立即停止JVM/测试；不能作为通过。v2 ca6701d4调试构建反复计算377MiB/288文件指纹，原生启动窗口超时进入RECOVERY_REQUIRED，没有启动apply或重发请求。切换release测试的尝试在编译阶段停止，未对v2重新执行事务；v3先完成优化构建，再依据实际请求/等待标记启动。最初错误的--exact筛选运行0项，保留日志，不计测试通过。

Orb成功夹具 `/opt/yofc-agent-cm-inverse-20261105-5ddf8f05`：真实UDS/systemd/完整Kura正向与agent逆向事务通过；真实DS keep/remove9、密码及挡写，原GWT admin/admin中文Count9通过。Java审计同26配置187属性含密文精确恢复。正向待完成与逆向待完成两种状态均实测重启agent，Kura PID不变且新upload被挡。初版驱动包含重复execute负例，发现后在该负例执行前停止驱动；当时逆向请求已在途，随后只查询原journal/回执确认AWAITING_COMPENSATION_COMPLETION，未重发execute。负例未执行，不拿它计入系统通过。补充脚本只核验现有结果及新的agent重启。

最后停止逆向Kura后再次重启agent：进入RECOVERY_REQUIRED并挡upload，正/逆向proof及18项制品字节不变，无Kura重启或自动重发。所有新增agent/正向/逆向unit MainPID0、inactive/dead；测试unit显式SuccessExitStatus=143，正式包unit处理仍pending。原Orbconsole26/agent20三服务与phase2-agent active原包未改；284库存SHA及实际agent二进制SHA核对。两Parallels VM实时stopped，当前缺Java/Kura继续deferred，未启动或套用旧沙箱结果。Orb请求的隔离属性不等于Parallels完整有效沙箱。

## 未交付

完整snapshot文件集一致补偿、最高AES制品之后的普通启动一致性、部分CM应用/无CM_APPLIED来源的显式核定、空原集合bootstrap准入、桥接和真实业务健康合同及明确完成后释放仍pending。勿删请求/修改journal/普通启动冒充完成。本轮只新增local管理员流程，远程中心界面与运维封装、独立UID/key grant和正式发行仍未交付。外部PID当前Rust权威与完整组件/factory/metatype面仍pending。

完整原目标不缩减：防火墙适配/NAT/远程/早启、restart/reboot闭环、一致备份恢复、身份运维与正式发行。外部deferred单列Parallels Java/Kura、真实NM/cloud-init、物理有线/硬件、amd64、磁盘满和硬断电。保留NM/Netplan现状、Kura独立/agent常驻、Kura生成key/CSR、双语及默认关闭/UNKNOWN不重发等核定。

日志和准确现场见 `target/agent-cm-inverse-20261105/`，结构化证据 `../evidence/agent-cm-inverse-20261105.json`。全18仓HEAD/分支及完整未提交状态复核保留；无提交推送reset/clean/远端覆盖，不连接192.168.2.125，不清任何旧夹具/key/state/备份/quarantine/卷或个人运行集。代码图谱MCP断开后使用官方CLI重建真实agent子仓，未索引父目录。
