# 普通交接未执行计划取消与宿主合同验证

> 后续核定 `d91353d6` 已完成本批新 IDEA/Orb/Parallels 实际系统验证，见 [实际系统核定](handoff-cancel-system-20261107.md) /对应evidence；下文 f2ea1b07 的当轮宿主范围和 pending 声明保留为历史，不重放旧入口。

核定 `f2ea1b07`，2026-11-07，完整管理体系目标保持 **active**。接续4230b9b8，前轮真实 restart/reboot 已闭环，不重放。本轮选择最终台账中的未决交接运维，实际补齐普通 handoff 未执行计划的显式取消。代码与宿主单元/契约及浏览器层已交付，**本批新 IDEA → Orb → 现有 Parallels 系统验证仍是内部 pending**。没有新系统请求、发行包或正式发行；旧systemcontrol3只含ops1–28，不代表本批代码。

证据根 `target/pending-plan-operations-20261107/`；机读 `docs/evidence/pending-plan-operations-20261107.json`；合同 agent `docs/ordinary-handoff-cancellation-v1.md`。当前进展不替代完整Core安装升级、全部组件适用面、NM/firewall、一致备份或正式发行范围。

## 实际缺口与实现

原普通交接只有status/prepare/execute。prepare后没有取消入口；过期也进入ordinary_preflight失败，后台会记RR并停止四个相关unit，即使尚未issued。新合同把过期与系统动作区分：仍有效的既有事实、但只因确认期限过去的未执行计划保持ORDINARY_PREPARED；执行仍被期限拒绝，由管理员明确取消。其他漂移/不确定事实不被解除。

- 协议major1追加op29 `recovery.ordinary.cancel`，复用严格field13 action；旧wire编号不动，无caller unit/path/command。现有ordinary remote授权继续默认0；新agent另外广告 `recovery.ordinary.cancel.v1`，旧agent仅有recovery.ordinary.v1不获得取消能力。中心要求两个capability、admin/CSRF/租户归属/首次密码闸门/新鲜在线连接；dispatch独立复核。新取消绑定当前epoch、固定TTL30秒，换连接前未派发队列过期，已派发UNKNOWN只查原任务。
- 本机root/admin UDS同名方法只收transactionId/confirmToken；已注册Kura不获得管理员角色，即使UID相同。普通Kura不需要agent，agent常驻、独立远程开关、Kura后端密钥/CSR、保留NM和默认admin/admin及首登设置均不变。
- 仅ORDINARY_PREPARED、issued=false和原token/不可变档案匹配时可取消。状态PREPARED → CANCELLING_ORDINARY → ORDINARY_CANCELLED；意图先落盘，私有create-only命令决定与取消receipt随后提交，最终journal精确链接SHA。Plan新增字段缺省省略，旧receipt字节含义不变；prepare+execute或prepare+cancel仍至多两个决定。
- 取消不调用systemctl、Java或业务provider，不启停服务、不写快照、不回滚原completion。过期可以取消；issued、已交接、RR和其他未决写仍挡住。取消中断只在已存在、完整匹配的回执/决定可证明时完成marker；提交前补文件和目录sync。缺失/损坏进入RR并保留，不重复取消，也不停止健康parent来补偿一次未执行的动作。后续RR恢复同样不发unit命令。本机决定变化但响应不确定返回OUTCOME_UNKNOWN。
- 原approved completion保留。后续新prepare先完整验证并create-only归档取消journal，再从原parent准备新计划；旧prepare/cancel命令可由历史独立核定。原wire SHA包括原ID/operation/key/TTL/token；只读query不systemctl、不业务检查、不repair/replay。prepare决定与原token、expiry、检查等完整审阅值匹配，不能修改token后仍解除维护闸门。SQLite resolution及中心reconcile审计各一次。
- agent与Core pre/post两脚本使用精确相同的纯取消证据guard，检查private source/grant/receipt、链接命令及原prepare身份；损坏或字段漂移继续挡维护。Core仍无需调用运行中的agent。旧guard拒绝新terminal状态，不能删journal强制降级。
- 双语面板按新能力显示明确取消按钮；不以执行勾选授权取消，不允许已执行/RR取消。浏览器仅存意图key和原taskID，丢失响应/刷新从原intent续接；404/UNKNOWN持续挡写，不生成替代取消或交接。任务详情与中心拒绝issued/full-recovery等越界成功结果。migration0019及OpenAPI/生成TS已更新。

取消结果限定 `cancelled=true / issued=false / ordinaryUnitHandedOff=false`，`kuraRecoveryVerified=false / normalBootVerified=false`，不报告新的业务恢复。原生state为ORDINARY_CANCELLED，传输任务在有证明时SUCCEEDED，两者含义分开。

## 本轮证据与准确范围

- 最终宿主agent **192 passed /9 ignored**，严格Clippy通过；含原普通交接13项、4个新增取消场景及其他受影响模块。新增覆盖过期无unit变更、错误token、已issued/自身RR/其他RR拒绝、取消中断有无receipt、后续RR不停止parent、原命令一次核定及新prepare后的历史查询。
- console最终常规 **24 passed /13 ignored**（实际PG需显式运行），mTLS fixture另3 ignored；protocol **11 passed**，扩展原wire/strict payload测试至29。两仓fmt、Clippy和React生成/typecheck/build通过，保留既有bundle体积提示；三仓diff check通过。
- 本轮显式新私有PG的实际HTTP/PG **3个测试通过**：七操作角色/租户/首登/CSRF/typed payload/新双capability/在线epoch/TTL/输入加密/原intent/UNKNOWN准入；op29已过期原命令从UNKNOWN按原wire证明、拒错误原wire/身份/越界flag、加密终态且中心reconcile audit一次；旧连接op29未派发过期、已派发仅核定。均是实际HTTP和PostgreSQL，native系统动作不是实机。
- Python包装 **70 tests，68 passed /2 skipped**；两个skip是显式native导出合同。单独选择Rust真实生成的测试证据后合同 **5 passed**，包含agent/Core三份精确guard、实际native JSON接受且字节不变、原prepare token漂移拒绝。Core脚本 **17 passed**。测试原生unit为明确mock，不冒充Linuxsystemd或Kura业务。
- 浏览器受影响10项通过（新取消4、原handoff6）；补强越界回执完成等待后单项通过；清理新mock未覆盖的transfer读端点和Ant Design loading图标DOM断言后，新4项最终通过。过期取消/中英、HTTP响应丢失后原intent、UNKNOWN只读/拒成功越界、旧agent不显示入口。最终截图在browser-v3，未覆盖旧截图输出。截图的loading图标节点一直存在，改用按钮loading语义类断言；该中间失败不是业务重试。
- 调试编译失败（测试访问私有字段、前端numeric enum未加入29）已修正；保留失败日志。原测试57只是test_packaging.py单文件，不当成完整包装套件；最终为全test_*.py的70项。源码旧副本185条在before/，不reset/clean。

代码图谱按agent/console/Core真实root更新，相关源码metadata读回一致；新增SQL0019两行parse_unusable已全文直接核对，并由实际PG迁移验证。其余图谱覆盖是best effort，不以ready代表整个仓库完整解析。

## 收尾状态及下一步

本轮只启动新私有PG16650和Vite19701，均已停止。fixture `.phase1-dev/handoff-cancel-host-20261107-c48a12e4`、socket `/tmp/yofc-handoff-cancel-pg-1eec8858`、3个PG schema/key位置、Rust mock导出、日志/截图全部保留。没有启动新console、agent、IDEA JVM或VM；没有发布真实prepare/execute/cancel，也未启用系统授权或安装包。原Orb四服务原PID244516/244911/286620/268及26/20包active保持；Ubuntu与Windows stopped。18仓HEAD/branch及旧dirty路径保留，无提交推送。

下一步：冻结/复核本批源码，生成新protocol crate和Linux候选（不能复用systemcontrol3）。从新运行集完成IDEA完整Kura及原GWT，创建全新获批parent/ordinary计划，实际验证取消不变更parent JVM/业务及新prepare历史证明，再按Orb→现有Parallels严格sandbox验证。复用上一轮私有Java制品可作输入，但不能复制旧事务当新completion，或重放旧seed/恢复/系统动作。所有这些是pending，不是external deferred。

其余pending保持：其他未决恢复运维、Core完整安装升级/启动策略交接与退役、全组件/metatype/provider/空CM/外部PID及Java离线恢复完整业务、NM/firewall/NAT/forward/远程流程/早启、一致DB/制品/密钥恢复、多轮key/旧系统迁移/正式发行。外部deferred仍为真实NM/cloud-init/物理网络/硬件、amd64、磁盘满/硬断电等；两个旧evidence gap单列。完整目标继续active。
