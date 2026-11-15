# R013 V37 原状态 source/factory 与准备入口

2026-11-13，同一shadow2055源码+4fixture reactors，84existing+69new−3fixture=150生产/测试路径UNAPPLIED。目标active。
当前权威native-source-factory-v37/{source-checkpoint-v37-current.json,candidate-source-pins-v37-current.json,verification-summary-v37.json,final-binding.json}。

## 实际改变

认证原writer model+完整同代raw现在可以生成绑定原operation的非空Transfer，按原表/链/handle/顺序改写全部已选规则，原basepolicy及未选对象不平铺/改写。
counter数量和条件位置有界核对，原source用既有Snapshot语义规范化，只排除流量值/无权威metainfo及跨链展示顺序；所有raw参数、链内顺序、handles、generation仍绑定。
因此流量计数变化不会让export前后原baseline永远不相等；真实原规则/参数/顺序/generation变化仍拒绝。

sourceDigest与随机operation ID分离，实际prepare绑定传入id，export后重查与fence重建同一id的baseline。
新增可选sourceModelSha256持久绑定原writer模型/实例；缺字段旧Transfer逐字序列化、原receipt与完整batch hash保持V36实际原计划身份。
native apply/rollback在读drain之前检查原模型摘要，模型能力消失/实例变化不进入新INTENT/dispatch。

未初始化的原状态分支不再读取不存在的私有confirmed政策，prepare也不再要求先通过私有static-rule对照。
已有私有confirmed回执则明确走原私有合同，即使桥接层广告legacy model；不以能力探测替换现有权威。
原分支广告model读取失败不fallback；firewall journal未settled、CM/roles/profile/raw/fence/model漂移均拒绝。

原CM角色的完整自动NAT矩阵须与两族writer model所有masquerade=true项精确相等，不能借agent未拥有的NAT记录作原策略证明，也不自动补齐缺失pair。
agent-owned NAT另查原提交回执，原分支若与既有agent-owned NAT重叠则拒绝并保留现场；IPv4 forwarding仍须系统owner已启用。
model中的false项和实际原链完整保留，不能把false当true或抹除。

## 已验证范围

8项新增契约：丰富实际Java renderer留存数据的144规则生成；原source/operation/计数条件；3接口独立stored/applied UUID；启用/禁用LAN NAT；稳定预览→一次export→original查询→fence。
丢最终export回包仍只export一次；缺完整raw/model不fallback；原双WAN模型缺lan0→eth0时在export/fence前拒绝；export后writer实例漂移保留原ciphertext且不重导出。
已有私有策略仍可准备且不调用legacy模型探测；缺真实注册JVM drain时native apply仍拒绝。没有用seeded drain/owner或私有confirmed文件给原路径制造通过。

正向controller fixture是明确合成的coherent roles（eth0禁用LAN NAT、lan0启用LAN NAT、wan0 upstream）配真实Java留存model/raw，3个NM profile均唯一。
它不是实际CM/runtime联调，也不是多WAN正向验收。原留存Java两WAN角色与writer矩阵不一致，拒绝结果保留；不能宣称完整角色/运行态映射已闭环。

最终host582pass/28ignored、严格Clippy过；Linux normal/test/Clippy及native55pass/10ignored，普通ELF无旧测试控制，未启动正常agent。
Java1794输入不变，继承V35 2455pass/9包并由V36绑定保护，没有重放Java、旧物理或任何已闭环一次性场景。
中间编译/fixture UUID重复、原NAT矩阵缺失、测试修改counter时误增JSON字段等失败均保留，不改写绿色、不降低生产断言。

## 现场与剩余

18原仓Git/未跟踪源码（本轮审查交接文档除外）、原管理1986+20pins、上游508present+89absence、credentials141+5overlay、ca9/f07/API522、V32/V33/V34/V35/V36/V15 bindings精确。
40任务容器停止，正常console/PG/Orb/Parallels不变。不是全target内容hash声明，无提交/推送/reset/clean，150路径仍未应用。
11fork本阶段tip全UNCHANGED，20功能组/73dispositions与7nonfork、327Maven+1Cargo关系保持。

先处理next-initialization-audit-v37.json记录的ACK后generation约束：当前有dispatch前CAS，但初次结果确认没有要求精确post-batch generation；须新红测和修正后才能新物理生命周期或应用源码。
随后接同一Store原operation/plan/source的私有初始化committed receipt，原digest0不可改；verify_delegated_backend须识别pair与native封装两层绑定并消费明确转换回执。
imported原ip/ip6表链/hook/type/prio/order/RETURN/basepolicy/opaque foreign、普通与joint NAT更新、startup/Coreowner、完整实际Transfer forward/inverse循环仍pending。
CM/runtime/ruleType/全角色与profile/NAT真假的实际联调、modem/DHCP/DNS/Wi-Fi/VLAN/multiWAN、startup child settlement、retirement/history/re-adoption、GUI/current paired DS/live及150路径整批应用仍内部pending。
原14组+7API2+R017（含runtime configuration1.2/build1.8配套，非API2原因）、A外部CA/custody及仅3externaldeferred完整保留。
周84%普通可用，credits/reset未动；UNKNOWN不重发/不换id/不重导出。全文接续remaining-transfer-implementation-v37.md。
