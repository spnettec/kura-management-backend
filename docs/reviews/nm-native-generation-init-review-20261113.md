# R013 V38 generation 与原 journal 初始化回执

2026-11-13，同一shadow2057源码+4fixture reactors，84existing+71new−3fixture=152生产/测试路径UNAPPLIED，goalactive。
权威native-generation-init-v38/{source-checkpoint-v38-current.json,candidate-source-pins-v38-current.json,verification-summary-v38.json,final-binding.json}。

## 实现与证据

两个真红测证明：只要规则/receipt相同，旧结果确认会接受错误forward/inverse generation。现已绑定一次forward提交和第二次inverse提交，u32回绕跳过0。
完整Linux v6.1 nf_tables_api.c已保存并校验Git blob7a09421f19e15f50008344ef109c272f35071988，commit仅推进一次base_seq且跳过0；不把NFgen16位header当32位NFTA_GEN_ID。
缺generation、额外提交、未提交、inverse步数错误、MAX→1→2均有新契约。旧批次和receipt身份回归保持，不重发任何旧operation。

初始化作为同一个ownership journal的可选阶段：INTENT先于目录/策略发布，随后create-only生成firewall/imported-confirmed.json，完整文件核对后记录COMMITTED回执。
document携原operation/plan、不可变source digest0/firewall digest0、roles/writer model、完整原Transfer及已ACK native快照；不以平铺candidate文本冒充原表链，不造新owner journal或空计划。
回执还绑定真实inode/device/sha和原paired markers。缺失/替换/不安全文件、重复apply、旧source/model/generation、actor/token错误不能发布或解锁startup。
丢发布回复只查原已有文件；INTENT缺文件不重建，COMMITTED已丢/已替换文件也不重建。已提交original查询是历史回执，不重复live source检查。
原native inverse在初始化存在时明确要求先单独retire初始化，不能留下指向已回退kernel的确认策略。

admin UDS initialization.apply/original已接原token/id；生产controller仍要求认证原model、实际registered drain、CM/profile/CAS和backend pair一致。
backend消费初始化COMMITTED回执，将原pair与native封装两层binding同初始化后revision1/source/layout对应；原baseline/plan/ciphertext原字节不变。
Core source witness和完整kernel reader识别imported回执；孤立或未提交文件没有权威。现有私有engine明确拒绝imported文件，避免误报revision0或覆盖原布局。
普通imported更新及实际firewall.kura.view仍待新布局适配，不能把这些helper通过等同整个Core/GUI合同通过。

4项原Store/真实临时文件系统契约通过：提交、丢回复查询、缺失/替换不重发、错误actor/model/generation。测试用明确fake kernel/Java运输，不代表实际registered JVM。
1项backend契约用原状态机提交receipt，证明revision1下原digest0和baseline不改；同时证明生产controller缺真实drain仍拒绝初始化。无seed owner/grant、无新物理namespace。
发现全Transfer身份摘要按每规则重复序列化；改为每次batch构造/结果检查计算一次，不跨请求缓存、不弱化任何检查。
同4项契约从165.73s降到23.08s，前后完整日志/输入保留；这是该组测试耗时，不是生产性能基准。

最终host589pass/28ignored，严格Clippy；Linux normal/test/Clippy/native57pass10ignored、initialization4pass、backend1pass；普通ELF未含已列测试控制标记，未启动正常agent。
Java1794输入不变，继承原2455pass/9包，不重复Java或旧一次性物理验收；所有中间红测/编译诊断原样保留。

## 接续与现场

先读next-imported-policy-audit-v38.json。下一步为imported policy/backend的实际Core view与普通prepare/apply/confirm/rollback、版本化current-policy lineage。
保留初始receipt/原策略锚点，普通更新通过既有firewall事务定义继承，不改旧确认文件来制造通过，不引入平行owner journal。
必须保留原ip/ip6表链/hook/type/prio/order/RETURN/basepolicy、类型/反向状态和opaque foreign；禁止直接重新编译为平铺私有inet表。
INTENT遇JVM丢失/新注册的原查询仍需要恢复合同，当前会保留UNKNOWN，未宣称所有process-loss前缀闭环。
随后接GUI初始化操作、真实CM/runtime/family-aware NAT与multiWAN/modem/Wi-Fi/VLAN、actual registered export/drain、startup/Coreowner/正常boot/recovery及全Transfer循环。
startup-child settlement/retirement/history/readoption、普通/DHCP/DNS、current paired DS/live、152路径整批应用仍内部pending。
原14组+7API2+R017（含runtime configuration1.2/build1.8配套，非API2原因）、A外部CA/custody、仅3externaldeferred完整保留。

18原仓（本轮审查交接文档除外）/1986+20管理pins/508present+89absence上游/credentials141+5/ca9/f07/API522及V32/V33/V34/V35/V36/V37/V15 bindings精确。
41任务容器停止，正常console/PG/Orb/Parallels保持；不是全target内容hash声明。11fork tip全UNCHANGED，20功能组/73dispositions、7nonfork/327Maven+1Cargo保留。
无commit/push/reset/clean，不重放旧闭环；UNKNOWN不重发/换id绕过/重导出。周85%普通可用，credits/reset未动。
