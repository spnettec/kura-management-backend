# R013 V43：同域当前历史读取与恢复审查（2026-11-13）

已实现 SAME firewall 目录内的 typed imported history/current-source 读取和校验、残留证据检查、其他系统写入的未决事务准入，以及仅补全既有终态标记的恢复。**普通 imported 事务的生产写入方、真实 Core/NM lease/drain/current cohort、GUI 仍未接入，不能计为普通更新或完整迁移验收。**

同一 shadow 2074 源码＋4构建夹具；88既有＋88新增−3夹具＝173生产/测试候选路径 UNAPPLIED。较 V42 改动11路径，仅新增 initialization/journal.rs 与 tests/journal_cases.rs。原18仓管理源码无部分应用，不提交推送。

记录共用 firewall/transaction.json、confirmed.json 与既有 kura-history/{hash}.json；计划、forward/inverse ACK 放在同域 {hash}.artifacts 内。没有第二 owner journal。typed backend 为 nft.imported.v1，严格区分原私有格式。UID/request 只能导出固定摘要路径；证据文件绑定 hash、device/inode，firewall/history/artifact 目录也绑定实际身份与700私有权限，文件600/no-follow/nlink1。历史遍历、目录数量及记录体积有界。

当前 revision、policy、显式目的前缀和原 ip/ip6 布局，必须从原初始化 receipt 经实际已完成历史逐步推导。UpdatePlan 不开放 Deserialize；从可信父来源重新构造生产计划后比较完整 claim，再校验原 Transfer 的完整 forward/inverse ACK。调用方 revision、只改 pointer 的 hash/revision、重算摘要的伪造计划、新 inode 的相同字节或缺失 ACK 均不能成为来源。rollback 保留逻辑版本，但形成新的实际 generation/handles/source receipt。初始化文件、receipt、原 owner journal 与 baseline0 保持不变。

补齐了丢失可变文件时的恢复缺口：即使 current pointer 和 active journal 都缺失，也检查现存历史与 artifact 目录；当前 epoch 的物理结果不在 head 链/原 active 内，或只有孤立计划目录，保持 OUTCOME_UNKNOWN，不回退 revision1。有效旧 pointer 也不能隐藏后续孤立计划/历史。其他接管 epoch 的 settled history 只作审计，不被采纳为当前来源；dispatch 前无效应取消可合法留在 head 链外。

APPLYING/RECOVERY_REQUIRED 等未决事务只能原查询；过期也不自动重发或换ID。恢复只在原 CONFIRMING/ROLLING_BACK 已有完整 immutable settled history 和 acknowledged current pointer 且完全一致时，补全同一个 active final marker。缺失 pointer/history 不重建，不 apply/inverse/创建新计划；实际后端调用计数为0。恢复使用系统 EX observation lock，未放宽真实 writer admission；普通 agent 启动遇到错误仍保持可用并记录待处理。其他系统 writer 会校验 imported 当前证据；firewall 原事务控制器、ownership adjudicator 与 configuration 保留各自现有准入规则。

Core 只读视图与 NM binding 使用实际 journalSettled、transaction 与派生当前 revision/policy/prefix/source；明确标记 confirmedTransactionAcknowledgement，writeEnabled/ownerGrant=false。旧 private 已归档结果从真实 outcome 请求入口独立返回 historical receipt；currentRevision 明确 unavailable，不解析当前 imported 文档/活动记录，也不调用后端；仍校验原 UID/owner/input。原初始化 committed 查询不重新 inspect/drain/export。

验证：最终190 Rust输入的宿主全量 625 pass/28ignored，初始化30项通过，宿主与 Linux 严格 Clippy 通过。Linux normal/test 构建、native63pass/11ignored、初始化30、Origin2、renderer2、UID授权1、旧private history1通过。最终 normal ELF 为 e9b50910ad0d8077df3b7f306c99142ba95bb2b7ae2ae1c1b728be83385e4602，未启动正常 agent、无 fixture 入口，network=none/CAP_DROP=ALL。Java1799输入与V41完全相同，精确复用2462tests/9生产JAR；新增 Java probe 用真实最终 Rust revision2/PF2451/prefix32 响应及归档JAR解码，LocalAgentService为测试替身，不是已注册JVM/实际owner验收。

保留初版 fixture 私有目录权限错误、错误使用 writer admission、strict Clippy cmp_owned 以及新增 inventory 初版借用编译失败的原始源码/log。审查还发现旧 helper-only 测试漏掉 outcome→projection→private confirmed guard：补请求入口测试先真实失败，再拆分不可变 receipt 与当前投影；初始红测完整保留。修复没有降低生产权限、匹配或凭据要求。较早全量617pass/28ignored为中间输入，仅最终 625 记录代表当前190输入全等。

preservation 核对18仓 Git/源码、1986管理输入＋20 overlay、508present/89absent、凭据141＋5、共享API522和旧绑定；51任务容器全停。正常console/PG仍healthy且StartedAt与V41恢复后相同，Orb gw-001running、Parallels两VMstopped。本阶段未重启正常服务或创建新内核验收场景。最终 Linux 构建被12GiB预先容量门槛挡住，未创建容器且没有ENOSPC；仅回收本任务未绑定的 Rust incremental 缓存，原已验证 ELF 校验不变，证据见 capacity-guard-cache-reclaim-result.json。最终隔离构建禁用 incremental 缓存，未降低容量门槛。V41 ENOSPC/自动恢复历史仍保留；V42及其他 CLOSED 场景均未重放。

11fork上游tip再次全UNCHANGED；20功能组/73dispositions/PLC671和7nonfork/327Maven＋1Cargo关联保持。阶段性同步继续按逻辑、前置依赖与消费者影响审查，不直接整批merge。原14组＋7API2＋R017、configurationAPI1.2/1.8配对、UI部署/TLSreload/bulk/protocol发布校验、ordinary/modem/DHCP/DNS/Wi-Fi/VLAN/multiWAN与联合generation全部保留。A外部CA/custody方向不改。仅3externaldeferred：NM/cloud-init/物理；amd64；disk-full/hard-power。

下步接入同一事务的 create-only artifact/prepare/apply/confirm/rollback/original-query 生产路径，再接实际 Core/NM owner lease、drain 与随当前 firewall revision 合法推进的 cohort；不能复用会再次 apply 的 private commit，也不能通过删除 guard 或重写 baseline0 解决。不重放旧一次性场景；之后新的完整 registered 迁移/boot/recovery/退役再接管、UI/task/protocol及173整体guardedapply仍为internalpending。

权威证据见 target/nm-role-authority-20261112/imported-journal-v43/final-binding.json、verification-summary-v43.json、source-checkpoint-v43-current.json、candidate-source-pins-v43-current.json、next-original-transaction-audit-v43.json 和 remaining-transfer-implementation-v43.md。周91%ordinaryallowed，credits/reset未用，goalactive；仅实际耗尽后必要收尾并按用户授权暂停。
