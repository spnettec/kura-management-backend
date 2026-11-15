# R013 V36 选中规则结果确认与原参数重建接口

2026-11-13，同一shadow2052源码+4fixture reactors；84existing+66new−3fixture=147生产/测试路径UNAPPLIED。
以target/nm-role-authority-20261112/native-outcome-v36/{verification-summary-v36.json,source-checkpoint-v36-current.json,candidate-source-pins-v36-current.json,final-binding.json}为准。

## 行为与证明

原Transfer的forward结果确认改用完整原始参数和有界语义，不要求内部transportHeader与nft的xt/readback JSON同形。
family/table/chain/注释等metadata仍精确；原始userdata中的原操作tag也逐字核对。unknown表达式/字段/副作用仍拒绝。
只有source已有完整raw时启用该分支，current必须保留全部规则（含外来）的raw及同代generation见证；未选规则JSON、顺序、handle、raw fingerprint保持精确。
计数器值可变化，但计数器存在数量和它之前的条件集合必须相符，不能把条件前计数移到条件后。
测试拒绝缺参数、外来漂移、JSON/raw回执伪造、metadata增加、重排、删header、改counter位置/数量、未知raw和NAT随机化标志。

新实际V36v1内核写入全部144条受管规则收到ACK，4条typed TCP/UDP header接受，但完整确认仍UNKNOWN而停止，inverse从未执行。
原因是nft_nat_init会把地址/端口寄存器已经隐含的MAP_IPS/PROTO_SPECIFIED位写回NAT_FLAGS（实际3），旧matcher只允许0。
保留真正红测及实际两族完整参数；修正仅接受寄存器已隐含的两位，额外random/range等选项仍拒绝。
依据为完整Git blob校验的[Linux v6.1 nft_nat.c](https://github.com/torvalds/linux/blob/v6.1/net/netfilter/nft_nat.c)，primary/保留全文及hash。

修正后对V36v1已经保存的ACK与完整同代raw快照只读核对，144条均与原计划对应、2外来不变。
v1-retained-forward-read-only-review.json明确liveKernelInspected=false、dispatchCount=0、journalChanged=false。
原scope/result仍failed，原operation未重发/换id绕过，未执行inverse；不是当前现场恢复、不是完整Transfer循环通过。

另建不同用途的原参数creation-context ABI场景：只对新namespace的144条原规则以原表达式/userdata重建，含tcp/udp/icmp/icmp6/conntrack/MASQUERADE/DNAT，
实际内核ACK且前后对象/规则及表达式userdata一致，2外来raw精确保留。它不使用旧operation，不重试或恢复V36v1，不创建owner/Transfer receipt。
kernel-context-v36-v1已CLOSED，禁止重放；完整Transfer forward→inverse循环仍pending。两场景均合成model注册，没有实际JVM/drain/export/fence/Coreowner证明。

最终host574pass/28ignored、严格Clippy过；Linux普通/测试/Clippy及native52pass/10ignored，普通ELF无任何旧/新测试控制标记，未启动正常agent。
V35 Java1794输入逐字不变，继承2455pass及已归档9包，未重复Java验收或install共享API。full-host-v10因新内核缺陷停止，exit−15和部分log保留，不当通过。
中间编译、红测、counter/NAT诊断均保留；旧V15/V18/V24/V26/V28/V30/V31/V32/V33/V34/V35成功一次性场景没有重放。

## 保存现场与接续

18原仓Git及未跟踪源码（本轮审查/交接文档除外）、原管理1986+20pins、上游508present+89absence、credentials141原值+5overlay、ca9/f07/API522、V32/V33/V34/V35/V15 bindings精确。
39任务容器停止，正常console/PG/Orb/Parallels保持。不是整个target内容hash声明；不提交、推送、reset、clean。
11fork新tip检查全UNCHANGED，20功能组/73dispositions及7nonfork、327Maven+1Cargo消费者关系保持；没有整仓直接merge。

接续same Baseline/Store/template/Coordinator完整factory、私有初始化committed receipt与原digest0显式绑定、imported原链布局、startup/Coreowner、完整实际Transfer循环。
源码接续审查见next-factory-audit-v36.json：除prepare/current_kernel_policy循环外，verify_delegated_backend还直接比较pair source与native封装source，并要求初始化前的firewall digest；须显式消费初始化回执而非改原baseline或跳过其他校验。
CM/runtime/ruleType/role/profile/automaticNAT映射、普通/modem/DHCP/DNS/Wi-Fi/VLAN/multiWAN、joint generations、retirement/history/re-adoption、GUI/current paired DS/live和147路径整批应用仍内部pending。
原14组+7API2+R017（含runtime configuration1.2/build1.8配套，非API2原因）、A外部CA/custody及仅3externaldeferred完整保留。prep/nativePlan仍false，目标active。
全文剩余台账见remaining-transfer-implementation-v36.md。周用量83%，普通可用，credits/reset未动。
