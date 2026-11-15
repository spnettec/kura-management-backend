# 周额度100%触发暂停 — ca9f292d

2026-11-08，用户要求周额度耗尽即暂停，仅允许少量额外credits做必要收尾。get_usage_limits现返回 usedPercent=100（周10080分钟，reset1794408709 = Asia/Shanghai 2026-11-11 22:51:49）。ordinaryUsageAllowed=true不改变本次用户暂停条件；不使用reset券，不继续功能开发/新验收。此记录落盘后调用update_goal(paused)，最终状态以工具返回为准。整体未完成，无自动唤醒安排。

## 当前现场与验证

最后完整封存f07d2c1a，manifest docs/evidence/remote-firewall-tasks-20261108.json，SHA518ef2389c817ad6e0666a8e4e4b52f10d72bd2e907e8cdd4282b100da5850e4。ca9仍为未完整封存实施批次，事实依据为target/nm-automatic-nat-20261108/progress-checkpoint.json、docs/reviews/nm-automatic-nat-20261108.md及docs/nm-firewall-coordination-v1.md。

已实施：Java NM/NAT前置归属检查及晚失败不发成功事件；DNS优先级32位签名修正，完整NM1502测试/package通过。agent固定接口私有配置、只读NM来源观察、来源绑定typed有线候选预览；真实NM1.54.3正向/Orb无NM拒绝仅属此前冻结read pair。新共享Guard的NM持久journal/intent/terminal receipt门禁：未决或损坏挡写，匹配终态回执方可准入；没有真实NM事务writer。六项新门禁合同、最终native268pass/13ignored/Linux268pass/14ignored/严格Clippy通过。Linux v4与109当前输入精确配对，release及SHA见linux-release-v4.json；未部署或向真实NM/nft写入。初次权限fixture失败已保留并只修测试父目录。

当前checkpoint14source/8artifact pins；exact before链、NM完整target克隆、四个Linux历史pair均保留。journal-fence-final-state.json确认18仓HEAD/branch与旧full dirty条目、原Orb四PID244516/244911/286620/268及Home不变，两Parallels stopped。全部新build/test handles终态，无新运行service。原现场、UNKNOWN、key/state/namespace/派生证据都保留；没有提交、推送、reset、clean或UNKNOWN重发。

## 恢复顺序

1. 先查询周额度，确认实际恢复；本暂停不授权用额外credits继续开发。读取本文件、根AGENTS与相关子AGENTS。按obsidian-workspace-knowledge技能/RULES，经官方CLI核对父工作空间scope=ws/vault=f9b864007dcd8a44，全文读最新Experience/2026-11-08-ca9f292d-NM自动NAT事务协调-实施中.md、Tasks.md与Session Summaries.md。不要重建子仓库库或覆盖Home。
2. 读上述ca9审查/合同/checkpoint及f07完整manifest/完整父台账，实时核对18仓、当前pin/配对输入及真实运行状态。已有成功一次性入口不重放；初始读锁/终态倒计时功能已闭环，旧exact holder只是证据限制。
3. 当前下一实际动作是新的只读NM daemon introspection，确认持久profile VersionId/CAS实际来源和Update2版本检查支持。官方Update2 version-id>=1.44；不能把device appliedVersion、hash或0当作持久profile CAS。现preview最低1.40不是写入最低版本合同。可复用既有Parallels VM的新只读探针；不要启动旧业务fixture或重跑旧read/写验收。代码发现先用agent/networking真实root图谱；已刷新agent索引，仍需按实际源行核对。
4. 再实施完整同权威NM/firewall writer：typed普通字段覆盖、自动NAT角色/保护、共享Guard下重校验与独立owner绑定、durable逐动作pre-intents、NM checkpoint/profile CAS、nft应用、全新管理连接后的显式confirm/rollback、丢结果原编号只读核定/恢复。当前文件reader不是已完成该状态机，也不是live业务成功凭证；其他writer接管退出/正式profile安装、生命周期/备份迁移/密钥轮换/发行及父14大组仍内部pending。真实NM/cloud-init/物理硬件、amd64、磁盘满/硬断电三组外部deferred独立保留。

建议恢复指令：恢复目标，先确认周额度已恢复，再全文读取 /Users/heyoulin/iot-kura-develop/git/kura-management-console/docs/reviews/continuation-paused-20261108-ca9f292d.md 并按其中顺序实际续接。
