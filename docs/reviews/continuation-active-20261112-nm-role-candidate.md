# 当前续接入口已迁移（2026-11-16）

请先读 [干净源码检查点](../checkpoints/20261116/README.md)、[未验收台账](../checkpoints/20261116/OPEN-TASKS.md) 和 [447路径归并判断](../checkpoints/20261116/CANDIDATE-REVIEW.md)。后续不依赖旧target、构建副本或大型证据恢复链，不要按下文旧路径重建闭环验收。

按用户最后确认，不能合入主线的代码只保留本地。18仓本地分支为 `wip/management-close-20261116`；447路径已存Git，其中仅2个独立CI/JDK最低版本文件合入 `spnettec/kura` 的 `heyoulin-kura`，提交 `dc788aa2361d2c14971c0b45533848e83f17aadd`。其余445路径仍本地，未推WIP分支，完整目标未完成。只清理可重建中间产物，保留密钥、加密数据、运行现场及原UNKNOWN；收尾状态见新检查点。JDK最低21，暂不追加27验证。

以下为历史记录；旧源数目、PID、证据依赖、提交/清理限制和“下一步”均须以新检查点及用户最新授权核对，不能据此重放旧动作。

---

# 当前续接：中心历史密文库真实备份恢复及迁移已作有限验证（2026-11-16）

目标active，G034未封签，最后接受G033/V61、完整封签V51、apply0、未提交推送。源/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/source-checkpoint-identity-renewal-v62-center-db-upgrade-wip.json，SHA e70765c8c8d60325441165afd2a98dd0eea6d151c9771e3417c60fbccf659a82；候选SHA 430f417b142b8d627c50265527ee5bcef7cae7f86317b270ea70989448dca4d5。2303源+4夹具、V51后251、439候选未应用、130快照。核验/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/center-encrypted-db-upgrade-v1/verification-center-db-upgrade-wip.json。

本轮只新增3个分阶段集成测试及cfg(test)注册，生产行为不变。独立PG16合成schema9密文库，103条任务、六种状态、有/无结果，经真实pg_dump及两次pg_restore、当前9→26迁移、原加密钥备份恢复、双代slot、重加密和旧dump再次恢复验证：103输入/51结果均可解密，17条UNKNOWN、原任务历史/audit保持；迁移前后原密文/绑定逐字一致，rekey才改变密文。缺钥/错钥/错口令拒绝，原钥/备份完整metadata精确。仅证明该合成合同；客户旧备份和完整GUI数据库备份恢复仍内部pending，不能标成整项完成。首次PG临时Unix-ready误判导致createdb明确失败，查明库不存在后接续，原脚本保留，没有seed/验收重放。

新PG f38774434de770e550500ab2c7a3be590cd5445542afa4b41b433e181f0ad447 已正常停止，数据卷、私有dump/加密备份保留。原三网关7PID/ticks/权限、38私有metadata与6数据内容精确；Core12/GWT6等六包不变，无任何key/startup/remote writer。已闭环ID不重发，767863a9/a87d94d5仍UNKNOWN。18原仓/API522/上游已审查游标保持。

**下一步：** 完整密钥生命周期、弱钥轮换/双代恢复/重加密、多人CA custody、完整broker/provider/launcher/模式/资源/平台/purge及原API/网络/DB/协议/发行/上游台账。Main仍11/13caps、NNP0。仅原三类external deferred。周57%、credits/reset0。以下旧scope/源计数/待验说法仅其历史阶段，所有未完目标继续保留。

Synthetic schema9 encrypted-task dump/restore/current migration/rekey is now accepted; do not repeat its three staged tests. No production behavior change was needed. The new PostgreSQL container is stopped, volume/dump/key backups retained privately. Preserve all three gateway fixtures and seven service processes. Continue the complete key lifecycle and original privilege/provider/launcher/mode/platform/resource/purge ledger: weak-key rotation and multi-person CA custody remain internal pending, plus original API2/R017/consumers, network/DB/protocol0/release and upstream logical groups. The independent GWT6 descriptor and LoginUi no-change fallthrough are now fixed; all303 established server/shared/policy entries are byte-exact again. Do not replay their closed writer acceptance. Current new fixture 863ea5ecfeb0558eabe4c110a5973efd26fbad58e076c48080b2e9bba5a40f0a runs Core12/GWT6 Core3792/key4229 with EMPTY key/startup operation journals; that namespace cannot settle older outcomes. Preserve a04 Core12/GWT5 Core5174/key4735/recovery4893 and its terminal f4115360-1598-4e2d-98ff-29978639936c, 57d19ee1-a8e8-4fc6-8dba-36cd4cc6bcd9 and 10296cdc-4a25-46e9-b4b4-8993fb31bac6 originals, query only. Old 5f6b Core10430/key10576 and all data are unchanged. Keep seven intentionally running jobs and all fixtures; browser and host relay closed. Then continue the complete internal ledger: privileged broker/provider/launcher/mode/platform/resource/purge matrix, weak-key rotation and CA custody with multiple-person control, original API2/R017 and consumers, network ordinary/recovery/NM/CM/NAT/boot/restore branches and child settlement, center DB/address/CSR/half-credentials/first TLS UNKNOWN/capacity/archive, customer-backup and full GUI database recovery (synthetic nonempty schema9 PG dump/restore/current migration/rekey now scoped accepted), protocol0 online upgrade, formal GUI/platform release, G021/G022, two PLC4X upstream logical groups and 439 guarded candidate paths. Only the existing three external deferred boundaries remain. Preserve all original trees, current fixture Core10430/key10576, UNKNOWN outcomes and dirty candidates; no source apply, commit or push.

---
# 当前续接：GWT6只读RPC隔离及登录修复已验证，原303写边界逐字恢复（2026-11-15）

目标active，G034未封签；最后接受G033/V61、完整封签V51，apply0、未提交推送。源/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/source-checkpoint-identity-renewal-v62-key-rpc-isolation-wip.json，SHA 42d4a6b07969aff71ec2cae8ba520301c1793256868338db1cba4e669a5b8431；候选SHA a1889f299a2a9cc707addd374380f82a800756ab0f6be64db8dd736c0bc0fef0。2302源+4夹具、V51后250、438候选未应用、129快照；本轮19变化/8新增。核验/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/gateway-key-rpc-isolation-v1/verification-key-rpc-isolation-wip.json。

公开恢复资料迁到独立gatewayRecovery RPC，管理员/XSRF、固定只读请求、公开JSON字符串，无新增key DTO。旧5生产文件/2测试恢复，clean仅新私有UI构建目录。Java40（旧31+新9）和完整GWT通过；与已闭环GWT3比，原303个server/shared/policy条目全部逐字不变，仅新增只读类/policy。旧写验收不重放。LoginUi不需改密跳转后return修复，后续真实中英登录零密码规则/更新请求；首次强制改密仍成功。初脚本networkidle超时属测试等待问题，只读续测未再改密。匿名descriptor未披露证书；中英资料下载及旧key status通过，无key writer。

Core12冻结包不重建；新GWT6 7d3dfdc0ae3c5301e8d9cedb8dc81714b7a29f3a72222861449b79a003c598db在新夹具863ea5ecfeb0558eabe4c110a5973efd26fbad58e076c48080b2e9bba5a40f0a验证，Core3792/key4229运行、recovery0（正常现钥/无pending），新7私有metadata不变。fresh自动provision1与管理key writer0分开，Root key/startup日志都空，不得由新空日志结算旧操作。私有合成口令勿打印，不纳入绑定。

原a04 Core12/GWT5 Core5174/key4735/recovery4893、原10私有metadata及f411/57d19ee1/10296cdc成功记录保持，不升级重启；更旧5f6b Core10430/key10576、21私有metadata/6数据/4Root记录保持。三现场7服务保留，浏览器/转发已关，入口/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/gateway-key-rpc-isolation-v1/owned-runtime-live-handoff.json。Source/UI图谱已按真实子仓刷新，best-effort使用。

**下一步：** 完整key生命周期/弱钥轮换/多人CA保管、最小权限/全部provider/launcher/模式/资源/平台/发行、原API/网络/DB/协议/上游逻辑组及438未应用。只读RPC隔离和LoginUi分支已闭环；下文GWT5“不再303一致”与待审查这两项仅历史。Main仍11/13 caps、NNP0，不能宣布完整broker或整套GUI/发行完成。

Continue the complete key lifecycle and original privilege/provider/launcher/mode/platform/resource/purge ledger: weak-key rotation and multi-person CA custody remain internal pending, plus original API2/R017/consumers, network/DB/protocol0/release and upstream logical groups. The independent GWT6 descriptor and LoginUi no-change fallthrough are now fixed; all303 established server/shared/policy entries are byte-exact again. Do not replay their closed writer acceptance. Current new fixture 863ea5ecfeb0558eabe4c110a5973efd26fbad58e076c48080b2e9bba5a40f0a runs Core12/GWT6 Core3792/key4229 with EMPTY key/startup operation journals; that namespace cannot settle older outcomes. Preserve a04 Core12/GWT5 Core5174/key4735/recovery4893 and its terminal f4115360-1598-4e2d-98ff-29978639936c, 57d19ee1-a8e8-4fc6-8dba-36cd4cc6bcd9 and 10296cdc-4a25-46e9-b4b4-8993fb31bac6 originals, query only. Old 5f6b Core10430/key10576 and all data are unchanged. Keep seven intentionally running jobs and all fixtures; browser and host relay closed. Then continue the complete internal ledger: privileged broker/provider/launcher/mode/platform/resource/purge matrix, weak-key rotation and CA custody with multiple-person control, original API2/R017 and consumers, network ordinary/recovery/NM/CM/NAT/boot/restore branches and child settlement, center DB/address/CSR/half-credentials/first TLS UNKNOWN/capacity/archive, nonempty old encrypted-task DB migration/backup, protocol0 online upgrade, formal GUI/platform release, G021/G022, two PLC4X upstream logical groups and 438 guarded candidate paths. Only the existing three external deferred boundaries remain. Preserve all original trees, current fixture Core10430/key10576, UNKNOWN outcomes and dirty candidates; no source apply, commit or push.

完整台账、三类external deferred、旧UNKNOWN均保留；周57%、credits/reset0。以下旧源数目/包/PID及上述已取代结论仅历史。

---

# 当前续接：Core12/GWT5已运行，升级后显式启动与公开恢复资料已验证（2026-11-15）

目标active，G034未封签；最后接受G033/V61、完整封签V51，apply0、未提交推送。源/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/source-checkpoint-identity-renewal-v62-key-cold-maintenance-wip.json，SHA a33d2bd79cf4f22139a018b6d23dcdc272ebf9cb9c3471f4a8d2dbb344baf7da；候选SHA ef3c95b132ae8957bbfcc797906402160863ce5a0d59138e50b5d971d63b66aa。2294源+4夹具、V51后241、429候选未应用、120快照；本轮21变化/3新增。核验/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/gateway-key-cold-maintenance-v1/verification-key-cold-maintenance-wip.json。

Root package-history不可变链在维护锁内绑定停止Main、已成功原启动集合、原钥和当前manifest；只post-unpack创建，兼容旧prerm。界面勾选后才能创建本次新启动UUID，旧原成功+最新未消费维护记录为授权，UNKNOWN不重发/换号绕过。schema2新启动10296cdc-4a25-46e9-b4b4-8993fb31bac6只1次，Root原成功/job5225；引用57d19ee1、原恢复f411及维护be8c0909。丢响应、重启恢复服务后只读接回成功；旧两操作未重发，三者均已闭环，只查询。

Core12 27fbc3bff99e685fbe5edadfac6a3a226a15711d8c928c1ef5a0d0678c2895c7及GWT5已构建安装，当前Java Main5174、key4735、recovery4893，HTTPS200及真实已登录GWT中英通过；现场/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/gateway-key-cold-maintenance-v1/owned-runtime-live-handoff.json。管理员/XSRF只读显示9443地址、公开证书与DER指纹，界面下载证书/说明并与实际网关匹配，无agent/无key writer。说明包含浏览器先核对保存TLS指纹，再上传备份口令，不能信任恢复页自报指纹。仅新夹具首次改默认admin密码一次，私有合成口令勿打印，未纳入绑定。89 host（88+skip1）/Linux89/Java34通过，Java11/GWT完整编译通过；217 OSGi+2 cipher JAR不变。新10个原私有metadata、旧21私有metadata/6数据/4Root记录及旧10430/10576/ticks保持。浏览器/转发已关，5服务与marker保留。

**下一步先审查/隔离只读descriptor的RPC边界：** GWT5扩展了既有GwtGatewayKeyResult公开字段，旧303条server/shared/policy逐字一致已为false；不能套用旧writer现场为GWT5全写验收，也不能重放闭环操作。当前只接受新只读资料/当前单元契约及限定启动。完整GUI审查还需核实旧LoginUi成功跳转后落入改密调用的分支，本轮无第二次密码写。然后继续完整key生命周期、弱钥/多人CA保管与最小权限/provider/launcher/资源/平台/发行及完整原台账。Main仍11/13 caps、NNP0，尚非完整privileged broker。

First review/isolate the ordinary GWT read-only recovery descriptor from the established key-writer DTO/RPC serialization policy: GWT5 adds optional public certificate fields and is not byte-exact to the prior 303 server/shared/policy entries; only new descriptor reads/downloads, unit contracts and scoped startup are accepted. Do not replay any closed key writer or startup. Keep f4115360-1598-4e2d-98ff-29978639936c, 57d19ee1-a8e8-4fc6-8dba-36cd4cc6bcd9 and 10296cdc-4a25-46e9-b4b4-8993fb31bac6 terminal, query only. Core12/GWT5 is actually running in a04, with Core5174/key4735/recovery4893; old 5f6b Core10430/key10576 untouched. Continue key lifecycle/weak-key and multi-person CA custody, complete least privilege/provider/launcher/resource/platform/package matrix, and remaining original source/apply ledger. Audit the existing LoginUi no-change redirect fallthrough during GUI review. Then continue the complete internal ledger: privileged broker/provider/launcher/mode/platform/resource/purge matrix, weak-key rotation and CA custody with multiple-person control, original API2/R017 and consumers, network ordinary/recovery/NM/CM/NAT/boot/restore branches and child settlement, center DB/address/CSR/half-credentials/first TLS UNKNOWN/capacity/archive, nonempty old encrypted-task DB migration/backup, protocol0 online upgrade, formal GUI/platform release, G021/G022, two PLC4X upstream logical groups and 429 guarded candidate paths. Only the existing three external deferred boundaries remain. Preserve all original trees, current fixture Core10430/key10576, UNKNOWN outcomes and dirty candidates; no source apply, commit or push.

三类external deferred、内部pending、429未应用与完整原目标保留；旧UNKNOWN不重发、不按新Main结算；周56%、credits/reset0。以下旧Core11 Main0及“入口/维护启动尚缺”的描述仅历史。

---

# 当前续接：Core11恢复续接和一次性启动交接已验证，继续GWT说明与升级后显式启动（2026-11-15）

目标active；G034未封签，最后接受G033/V61、完整封签V51，apply0、未提交推送。源/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/source-checkpoint-identity-renewal-v62-key-cold-handoff-wip.json，SHA 2d80953fb6266c4af45af8a030fb282c1ab497423d30d3b849bd08a630d8eeec；候选SHA a8c1938bddbd3e1263d2b70c76c409ee4041d1db5c7e1d850cca2cf90bacb104。2291源+4夹具、V51后238、426候选未应用、117选择快照；本轮10变化/2新增。核验/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/gateway-key-cold-handoff-v1/verification-key-cold-handoff-wip.json。

现钥存在后的恢复页续接、旧f411受保护原回执迁移、只读原编号查询已接入。Root固定StartUnit与独立持久日志，真实kurad peer禁用写，恢复账号仅固定kura.service；禁止任意unit/verb/argv，UNKNOWN不重发，新UUID不能绕过未决。页面接回服务端pending，启动回执和进程就绪分开。

新容器界面start57d19ee1-a8e8-4fc6-8dba-36cd4cc6bcd9仅1次，绑定f4115360-1598-4e2d-98ff-29978639936c，Root原SUCCEEDED/job4315；丢响应后只读原回执。Core10 Main3060进入Java且GWT HTTPS200；随后Core11升级停止Main，**当前Core11 Main0，不能将历史Core10当当前启动**。新key/recovery cap0/NNP1保留，PID/ticks见/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/gateway-key-cold-handoff-v1/owned-runtime-live-handoff.json。恢复服务即时重启和无localStorage新浏览器接回原回执通过。原restore、新start均已闭环，只读不再发。

Core11 7b879cc4cb81dfecaa102134ef99cdd78c87b005ca2368f8074541ab60690a6c已构建安装；217 OSGi+2 cipher JAR不变。修复私有目录umask022、HTML/JS配套、TLS TIME_WAIT立即重启及初次inspect限流。76契约75通过+skip1，Linux15新契约、隔离前端模板及真实浏览器中英390px通过。中途Core8失败及Linux v2旧树误计数已保留并由正确版本/源核验替代。新10文件完整metadata、旧21私有metadata/6数据/4Root记录及旧Core10430/key10576/ticks保持；本轮restore0、start1。浏览器与转发已关，现场/旧钥/marker保留。

**下一步：** 普通GWT恢复地址与TLS身份核验说明，及升级后有明确终态前驱的显式新启动确认。当前Core11 Main0，不能重放原57d19ee1，也不能从现进程结算UNKNOWN。全privileged broker、发行/全模式/平台及完整原目标仍pending。

First finish the ordinary GWT recovery address and TLS identity-verification guidance. Then implement an explicit, receipt-bound post-maintenance startup confirmation: Core11 upgrade deliberately stopped Main; the successful original Core10 StartUnit job is not current process health and cannot be replayed. A new operator-confirmed startup may be designed only after a proven terminal predecessor; UNKNOWN must remain non-retryable. Keep restore f4115360-1598-4e2d-98ff-29978639936c and start 57d19ee1-a8e8-4fc6-8dba-36cd4cc6bcd9 closed, query originals only. Current new container a04bfb08590b93fa3351447ded42c24b576c676699216f3636ff3ff658190c07 has Core11 Main0 and key/recovery processes in owned-runtime-live-handoff.json; old 5f6b45125fa35903f4c725aeadfdaa22025c4f2b62e33d156309fe09baf629bd Core10430/key10576 preserved. Do not replay either older key/GWT/cold prototype acceptance. Then continue the complete internal ledger: privileged broker/provider/launcher/mode/platform/resource/purge matrix, weak-key rotation and CA custody with multiple-person control, original API2/R017 and consumers, network ordinary/recovery/NM/CM/NAT/boot/restore branches and child settlement, center DB/address/CSR/half-credentials/first TLS UNKNOWN/capacity/archive, nonempty old encrypted-task DB migration/backup, protocol0 online upgrade, formal GUI/platform release, G021/G022, two PLC4X upstream logical groups and 426 guarded candidate paths. Only the existing three external deferred boundaries remain. Preserve all original trees, current fixture Core10430/key10576, UNKNOWN outcomes and dirty candidates; no source apply, commit or push.

完整原目标和台账保留，三类external deferred不变，周55%、credits/reset0。以下旧源数目、Core7状态和“启动交接尚未实现”仅历史。

---

# 当前续接：Core7已验证独立账号/TLS/打包冷恢复，继续恢复后续接和启动交接（2026-11-15）

目标active，G034未封签；最后接受G033/V61、完整封签V51，apply0、未提交推送。源/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/source-checkpoint-identity-renewal-v62-key-cold-install-wip.json，SHA 2af8b42ac9acd51ccf0f6f05eac9f045e4fa9df7468f5c17b432820bacc57af1；候选SHA 7186a85c32f94316a5c6ee487772cc9d9e1d8b78f5a5a13647da5f699a304a8e。2289源+4夹具、V51后236、424候选未应用、115快照；本轮13变化/3新增。核验/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/gateway-key-cold-install-v1/verification-key-cold-install-wip.json。

Core7 edf4dfb2已构建安装于新Ubuntu24 ARM64容器，217个OSGi与2个cipher JAR逐字不变。独立恢复UID995与Core UID997分离；socket仅control组，恢复peer禁止backup-create及备份回执，只能缺钥恢复/状态/inspect/原restore回执。独立TLS原子建立并原样保留，旧2单位与新3单位清单受控兼容，维护前阻断未决冷恢复。host18+skip1/Linux16/生命周期20检查通过。原Root代码共享只读组，私有数据不开放给恢复账号。

新容器a04bfb08590b93fa3351447ded42c24b576c676699216f3636ff3ff658190c07：缺钥新启动自动出现HTTPS恢复页，真实新恢复UUIDf4115360-1598-4e2d-98ff-29978639936c只投递1次；丢响应后刷新只查原回执，Root原成功确认。原install key保留同目录inode，原receipt/TLS所有metadata保持，Main尚未进入Java。新Main0/key253/recovery172，两服务cap0/NNP1，私有读打开均拒绝。新writer已闭环，不重放；原8fee及GWT/生命周期也不重放。浏览器/转发已关，新服务保留；入口/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/gateway-key-cold-install-v1/owned-runtime-live-handoff.json，合成口令仅私有packaged-ui-fixture-secrets.json勿打印。

原容器Core10430/key10576保留，21个private metadata、6个数据和4条Root记录保持。六包仅Core换7，当前Core7未做Java/GWT启动；旧Core6启动证明仅历史。仍未接受全部模式/平台/资源/purge/完整最小权限。

**下一步首先修复恢复后的续接：** 原钥存在后当前unit/serve守卫会跳过恢复服务重启，须在首次投递前落盘pending marker并提供只读续接，绝不从现钥结算UNKNOWN或重发恢复。然后绑定f411原回执接一次性Core启动交接与正常GWT恢复地址说明。

First preserve recovery original-result access after key restoration and a recovery-worker restart/upgrade: the actual unit condition and serve guard currently skip/refuse a present key. Add a durable public pending marker before first dispatch, an appropriate conditional/read-only resume path, and maintenance validation, without inferring any UNKNOWN outcome from the present key. Then add a fixed, durable, single-dispatch Core startup handoff from the UI, bound to the proven packaged restore f4115360-1598-4e2d-98ff-29978639936c. Keep Core start request acceptance and original job/process/health evidence distinct; never retry an ambiguous dispatch or accept arbitrary unit/verb/argv. Add the normal GWT recovery address and TLS verification guidance. Retain new live container a04bfb08590b93fa3351447ded42c24b576c676699216f3636ff3ff658190c07 with Core0/key253/recovery172 and preserved old container 5f6b45125fa35903f4c725aeadfdaa22025c4f2b62e33d156309fe09baf629bd Core10430/key10576. Do not replay f4115360-1598-4e2d-98ff-29978639936c, prior prototype restore 8fee5e2b-343e-4d71-b09d-7388af348ed9, either GWT writer or any old native key/data/lifecycle acceptance. Only original reads are needed. Then continue the complete internal ledger: privileged broker/provider/launcher/mode/platform/resource/purge matrix, weak-key rotation and CA custody with multiple-person control, original API2/R017 and consumers, network ordinary/recovery/NM/CM/NAT/boot/restore branches and child settlement, center DB/address/CSR/half-credentials/first TLS UNKNOWN/capacity/archive, nonempty old encrypted-task DB migration/backup, protocol0 online upgrade, formal GUI/platform release, G021/G022, two PLC4X upstream logical groups and 424 guarded candidate paths. Only the existing three external deferred boundaries remain. Preserve all original trees, current fixture Core10430/key10576, UNKNOWN outcomes and dirty candidates; no source apply, commit or push.

完整原目标/台账保留，三类external deferred不变；周55%、credits/reset0。以下旧数目/包版本/PID及“生产cold unit尚无”的描述仅历史。

---

# 当前续接：G034/V62 独立冷恢复页面已做新缺钥夹具核验，继续发行接入（2026-11-15）

目标active，G034未封签，最后接受G033/V61、完整封签V51，apply0、未提交推送。用户确认以备份文件＋解密口令授权，严格匹配原指纹。源/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/source-checkpoint-identity-renewal-v62-key-cold-ui-wip.json，SHA cb687c8bf8945f83578700cc3b4b0ca77c4e5d57054071f133b0bfb7f179f7a3；候选SHA 80e99904e10059ae7c875ee0c23c8382dbbbfc8ae3c31a7978ccbf0e69e23800。2286源+4夹具、V51后233、421候选未应用、112快照；核验/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/gateway-key-cold-recovery-v1/verification-key-cold-ui-wip.json。

新增独立HTTPS恢复页/适配器及契约测试5路径，21+1契约通过。新Linux合成夹具从创建时即缺钥；真实UI恢复UUID8fee5e2b-343e-4d71-b09d-7388af348ed9只投递1次，Root原回执确认；成功响应丢失后保持UNKNOWN，新钥已存在也不结算，刷新只查原编号、显式确认后清理intent。该新writer已闭环，禁止重放。中英文/窄屏/手工编号查询通过。Root桥及前端均cap0/NNP1，但前端UID997只是factory夹具，正式入口源码已强制独立非登录账号。

主容器Core10430/key10576不变，原4个Root操作及21个private文件metadata、6个数据文件内容保持。旧GWT两writer、旧key/data和生命周期不重放。新测试服务/浏览器/转发均已停止，夹具保留；入口/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/gateway-key-cold-recovery-v1/owned-runtime-live-handoff.json。合成口令仅私有native-ui-fixture-secrets.json，勿打印。当前安装Core6/GWT4不含新cold UI，包与全部当前生产输入匹配=false；原Core日志屏蔽和GWT守卫接受范围保持。

下一步：Integrate the tested cold-recovery UI into the distribution: create a distinct nonlogin kura-key-recovery identity, grant only status/inspect/restore/original-restore reads in the Root peer policy (never backup-create), restrict the socket group, mask its journal from Core, install/preserve separate recovery TLS credentials, wire boot failure and upgrade/remove maintenance fences, add a durable non-repeating post-restore startup handoff, and expose the recovery address through the normal UI. The production entrypoint already rejects Root, kurad, foreign and login-shell identities; do not weaken this to reuse kurad. Current Core6/GWT4 do not contain the new cold UI; source-only changes must not be reported as packaged or boot accepted. Reuse the proven 5 component implementations and original synthetic cold restore 8fee5e2b-343e-4d71-b09d-7388af348ed9 only for reads; never repeat that writer or either old GWT writer/native key/data acceptance. Then continue the complete internal ledger: privileged broker/provider/launcher/mode/platform/resource/purge matrix, weak-key rotation and CA custody with multiple-person control, original API2/R017 and consumers, network ordinary/recovery/NM/CM/NAT/boot/restore branches and child settlement, center DB/address/CSR/half-credentials/first TLS UNKNOWN/capacity/archive, nonempty old encrypted-task DB migration/backup, protocol0 online upgrade, formal GUI/platform release, G021/G022, two PLC4X upstream logical groups and 421 guarded candidate paths. Only the existing three external deferred boundaries remain. Preserve all original trees, current fixture Core10430/key10576, UNKNOWN outcomes and dirty candidates; no source apply, commit or push.

独立账号、Root peer仅恢复权限（禁止backup-create）、TLS生命周期、开机失败入口及恢复后启动交接尚未完成；完整原目标/台账保留，三类external deferred不变。周54%、credits/reset0。以下历史中的旧文件数量/包版本/PID及“冷恢复未有源码”仅作历史。

---

# 当前续接：G034/V62 密钥日志屏蔽与GWT状态守卫已核验（2026-11-15）

目标active，G034未封签；最后接受G033/V61、完整封签V51，apply0，未提交推送。源/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/source-checkpoint-identity-renewal-v62-key-journal-boundary-wip.json，SHA 46e8b09b9d07486be19fec16870c8b393814e850486734b2a71f136abdc8cd84；候选SHA e7da437100717d881cfcc7c994f1261fc3ed456930777694e8bd44b9dc975bce。2281源+4夹具、V51后228、416候选未应用、107快照。核验/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/key-journal-core-boundary-v1/verification-key-journal-boundary-wip.json。

本轮Core6 b4d2d9b9屏蔽私有日志目录，Core读写打开均被拒，独立Root服务写权限与typed socket原结果读取保持。GWT4 58e69692状态失败即禁用新写，旧响应不能解锁，粘贴UUID可查询；31项测试、实际浏览器只读验证通过。303个后端/共享/策略项与已接受GUI原写入版本相同。原4个Root记录、2个GUI结果及6个数据文件保持，新增密钥写入0。正常升级的cp -a使11个GWT缓存inode/ctime改变，内容/权限/所有者/mtime保持，Root10文件全metadata保持；未重跑APT。

现场5f6b45125fa35903f4c725aeadfdaa22025c4f2b62e33d156309fe09baf629bd：Core10430/ticks3504160、key10576/ticks3506794；HTTPS200、ERROR0、无重启，network=none、无发布端口。浏览器和转发已关闭；入口/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/key-journal-core-boundary-v1/owned-runtime-live-handoff.json。原备份fd78f9ad-d41d-4651-98bd-7a664f167255、恢复5a7dfd2d-51bf-4431-bb67-7995643b0c4b仍各仅1次，不重发。user/kura_custom.properties覆盖基础同名配置，kura.snapshots.encrypt=false保留。周53%、credits/reset0。

下一步：Continue cold recovery GUI when Core cannot start, retaining the existing Root key service/receipt and downloaded encrypted backup; do not repeat either accepted GUI writer or old native lifecycle/legacy-key/data acceptance. Then continue the complete internal ledger: privileged broker/provider/launcher/mode/platform/resource/purge matrix, weak-key rotation and CA custody with multiple-person control, original API2/R017 and consumers, network ordinary/recovery/NM/CM/NAT/boot/restore branches and child settlement, center DB/address/CSR/half-credentials/first TLS UNKNOWN/capacity/archive, nonempty old encrypted-task DB migration/backup, protocol0 online upgrade, formal GUI/platform release, G021/G022, two PLC4X upstream logical groups and 416 guarded candidate paths. Only the existing three external deferred boundaries remain. Preserve all original trees, current fixture Core10430/key10576, UNKNOWN outcomes and dirty candidates; no source apply, commit or push.

完整原目标和未验收台账保留；三类external deferred不变。完整最小权限、全部提供者/平台/模式和冷恢复GUI仍未接受。以下旧PID/包版本及本轮三项待修描述属于历史。

---

# 当前续接：G034/V62 GWT在线密钥备份恢复已验证，继续Root日志边界与界面守卫（2026-11-15）

目标active，G034未封签；最后接受G033/V61、完整封签V51，apply0、未提交推送。源/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/source-checkpoint-identity-renewal-v62-gateway-key-gwt-wip.json，SHA 6492da9f5061daff268eaf334c489788362c7e23d96d78fd5ff79dae126ffd13；候选SHA 75828418d83ba5f96be93d77a8c476dd4075ac724c5799c765c3df0e9f1131c3；2281源+4夹具、V51后228、416候选未应用、107快照。核验/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/gateway-key-gwt-v1/verification-gateway-key-gwt-wip.json，SHA 2149e2b20f56a8877764021582117b26f1e49de1b0f120134c837959bdf0d795，2342绑定；旧1517及15收尾绑定保持。以下历史逐字保留。

GWT安全页已提供ADMIN/XSRF的主密钥状态、加密备份下载、上传核验、匹配恢复及原UUID/本地历史恢复，不依赖agent。31项Java测试、Java11及login/denali各6排列编译通过。修复root版本目录符号链接预检、口令粘贴和侧栏溢出。真实GUI备份 fd78f9ad-d41d-4651-98bd-7a664f167255 和同钥恢复 5a7dfd2d-51bf-4431-bb67-7995643b0c4b **各只发1次**；备份成功后丢浏览器响应，刷新只查原编号，下载同一加密文件，无重发。错误口令拒绝/明确指纹与确认通过；恢复keyChanged=false。原8个private文件metadata与6个数据文件内容保持。旧密钥/旧密文及生命周期闭环均未重放。

写链路在GWT revision2验证；当前revision3 ca41217d只清理选中文件/旧指纹，303个server/shared/RPC-policy项完全相同，最终包只读恢复原回执/历史与清理已核验。Core52169474及其217 OSGi Java不变，旧Docker未重建。普通在线GUI不等于冷恢复或全部安全/平台验收。

**下一步优先修复：** 已用相同生产UID/能力且零写入确认Core能写打开/var/lib/kura-key-service的日志密钥/原回执；需给Core增加该目录只读挂载，保留独立Root服务的写能力。另修复服务不可用时新写入守卫、手工粘贴原UUID不启用按钮。均internal pending；完整broker/提供者/最小权限仍未完成，原Core11 ambient/13 bounding/NNP0保持。

**保留现场：** Docker 5f6b45125fa35903f4c725aeadfdaa22025c4f2b62e33d156309fe09baf629bd，Core8456/ticks3211347、key service5569，network=none、无发布端口；本轮浏览器/回环转发已关闭。入口/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/gateway-key-gwt-v1/owned-runtime-live-handoff.json。合成测试口令仅在本阶段gui-fixture-secrets.json（0600），不要打印。不要重建环境、重发两个GUI原操作或重放旧key/data/升级/卸载。原VM只读stopped，旧全局UNKNOWN/startup.stop不重发、不结算；周52%、credits/reset0。

下一步：First protect /var/lib/kura-key-service from the Core ambient DAC capabilities using a read-only Core mount, preserving the independent Root key service writer. The non-writing probe proved write-open of journal.key and an original record; all original bytes and metadata remain exact. Then fix GWT writes enabled when key status is unavailable and manual original-ID paste; use a new isolated build cohort and only original-ID reads. Never replay the accepted GUI backup fd78f9ad-d41d-4651-98bd-7a664f167255 or restore 5a7dfd2d-51bf-4431-bb67-7995643b0c4b, each dispatched once, or any closed lifecycle/legacy-key/data scene. Retain live container 5f6b45125fa35903f4c725aeadfdaa22025c4f2b62e33d156309fe09baf629bd Core8456/ticks3211347 and key service5569. Continue cold recovery GUI, full broker/providers/modes/platform/resource/purge matrix, weak-key rotation/CA quorum, original API2/R017, network, center DB/CSR/TLS UNKNOWN/capacity/archive, DB migration, protocol0, formal GUI/platform release, G021/G022 and two PLC4X groups.416 guarded candidates remain unapplied; no old UNKNOWN redispatch, commit or push.

完整原目标/台账保留，外部deferred仍仅三类。以下旧“Core未启动”“GWT未实现”及旧PID/包版本仅作历史，不作当前状态。

---

# 2026-11-15 当前续接：G034/V62 网关密钥服务实际包生命周期已验证，接GWT界面

目标active；源/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/source-checkpoint-identity-renewal-v62-gateway-key-lifecycle-wip.json，SHA 41496ed30ced85f2cff715396d647df1b1b5b5ab46d314dd0576129d59ce71d1；候选SHA 1e39585394d30611e7324eafdce2d5b8927b31afe5faaddfbc8523fcf12dd6d6；2267源+4夹具，V51后210，398候选未应用，89快照。核验/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/gateway-key-lifecycle-v1/verification-gateway-key-lifecycle-wip.json，SHA 06bd26dd3973478fe61d0a162b7e4ae70f490d78b3d2128e93e2962ae2b3de66，1517绑定，旧1380及8收尾绑定保持。G034未封签，最后接受G033/V61、完整封签V51，apply0、未提交推送。以下历史逐字保留。

Core已打包独立AES key service：真实root:kurad0660 socket，root cap0/NNP1；固定typed操作，无用户路径/shell。包操作持admission lock验证无正在执行/UNKNOWN，写持久maintenance fence后停止key socket/service，所有代码和保留数据恢复验证后再开放。root私有日志/HMAC密钥不补造，原UUID只读恢复。原Core11 ambient/13 bounding/NNP0保留，完整提权隔离待办。

新Ubuntu24.04 ARM64真实DEB/systemd10项通过，实际备份/缺钥按回执恢复各发1次，隔离的新原钥保留，恢复root0400单链接。持锁升级明确拒绝且原PID/private文件保持；正常revision3→4升级及revision5可信卸载修复安装、实际remove/retained reinstall均保留密钥、日志密钥、两原回执字节/inode/owner/mode/mtime/ctime及TLS/custom/snapshot内容。postrm原执行daemon可写.data脚本的问题已修复：bin中dpkg追踪root同源helper在prerm运行，实际daemon写入历史canary未被执行。两次正常升级APT夹具路径错误均在dpkg前，仅修正输入后继续未完成步骤，没有重放忙碌拒绝或UNKNOWN。

Core52169474，exec20261115 revision5，269载荷；旧217 OSGi Java SHA不变，2私有cipher JAR不进OSGi。当前生产文件和两嵌入preinst helper逐字对应源。六包仅Core换新，旧Docker未重建。主机46通过/2 Linux专属未执行，Linux48通过；测试夹具路径/umask失败均保留并纠正，无生产权限放宽。自定义覆盖/快照开关、旧真实加密store解密闭环保持，未重放旧场景。当前包未启动Core/GWT，源中旧boot布尔只作历史。

**保留运行现场供新GUI：** Docker 5f6b45125fa35903f4c725aeadfdaa22025c4f2b62e33d156309fe09baf629bd（yofc-g034-key-service-lifecycle-20261115），Ubuntu24 ARM64，key service PID5569 cap0/NNP1，Core MainPID0、agent未装，network=none、无发布端口。owned-runtime-live-handoff.json含身份与当前job。不要重建、重播已闭环10项/升级/卸载/重装；新增GWT实现后在此做首次Core/GUI启动。原Orb/Parallels VM均只读确认stopped；18原仓/API522保持。全局UNKNOWN/startup.stop不重发不结算。周50%、credits/reset0。

下一步：Implement admin/XSRF GWT key status, encrypted download/upload/inspect/restore and durable original-UUID recovery, independent of agent, using the existing live fresh Ubuntu container 5f6b45125fa35903f4c725aeadfdaa22025c4f2b62e33d156309fe09baf629bd and current package52169474. Do not rerun completed native key-service backup/restore, busy rejection, upgrade or remove/reinstall cases, or any old key/data scene. Core has not been started in this fixture; install/build only the new GWT GUI and affected package inputs, then perform its first GUI boot. Keep new source/cohort work isolated and old artifacts immutable. Continue cold recovery GUI, full privileged broker/providers/modes, weak-key rotation/CA quorum, original API2/R017, network, center DB/CSR/TLS UNKNOWN/capacity/archive, DB migration, protocol0, formal GUI/platform release, G021/G022 and two PLC4X groups.398 candidates unapplied; no old UNKNOWN redispatch, commit or push.

完整原目标/内部待办不缩小，外部deferred仍三类；冷恢复GUI、所有模式/平台/资源/purge和完整Root/provider矩阵均未完成。以下历史保留。

---

# 2026-11-15 当前续接：G034/V62 网关密钥后端基础已核验，界面与安装待接

本节只更新当前指针，以下历史逐字保留。目标active；源/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/source-checkpoint-identity-renewal-v62-gateway-key-backend-wip.json，SHA 1a4ad3e90c284c39a529018afaa9b56e8c111b032934df60fea7fbcb76ef19a9；候选SHA af64f865662083f01c956bc1527f87d18e8fdd8ab1c8966883c43178098ddabb；2260源+4夹具，V51后203，391候选未应用，82快照。核验/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/gateway-key-management-v1/verification-gateway-key-backend-wip.json，SHA 4b28bd08b49a57c727414c6ed135d7b0c31c4d7ae8b4ba1257d7f54bd36b4e81，1380绑定，旧1320一致。最后接受G033/V61、完整封签V51，G034未封签，apply0、未提交推送。

新增独立Core密钥服务与Java21备份加密模块：固定typed操作，无用户路径/shell，口令仅管道，Argon2id19MiB/t2/p1+AES256GCM。journal仅HMAC请求标签/加密结果。UNKNOWN只查原UUID，INTENT不从当前key倒推。已有同钥恢复不变，异钥拒绝；缺钥需要原root回执匹配，renameat2 NOREPLACE恢复root0400单链接。与agent/TLS/CA/中心任务钥匙分开，自定义配置覆盖和快照开关保持。

主机累计26通过/2 Linux专属未执行，Linux累计28通过；初始夹具写自身0400失败仅纠正2项夹具并补测，另新增2项缓存原回执绑定补测。新Alpine3.24 ARM64 Docker真实socket11项通过：root cap0/NNP1，真实kurad122允许、其他UID拒绝，备份只发一次丢响应后原UUID取回同一加密文件，错口令/用户路径拒绝，key/receipt全部metadata不变。fixture socket0666用于验证UID拒绝，非生产socket权限或Ubuntu systemd验收；最后严格缓存guard由主机/Linux补测覆盖，没有复启旧socket场景。

新容器均终态、私钥/日志/代码卷和层保留，旧场景未重放，18原仓/API522保持。旧六包仍d9f93709 Core，不包含新后端；GWT、安装生命周期、journal初始化/fence/正式root socket/systemd和冷恢复GUI未完成。原Core11 ambient与NNP0保留，完整提权隔离未完成；原UNKNOWN及startup.stop未重发或结算；周49%、credits/reset0。

下一步：Integrate the independent Core key service into package lifecycle with private journal initialization, maintenance admission fencing before code removal, root-owned socket/service and no-new-privileges/empty caps; then implement admin/XSRF GWT download/upload/inspect/restore and durable original-UUID recovery. Current native socket proof uses Alpine3.24 ARM64 Docker and fixture socket activation; Ubuntu systemd and GUI remain internal pending. Preserve existing same-key/no-overwrite contract, custom-properties precedence and all closed key/data scenes. Continue full privileged broker/providers/modes, weak-key rotation/CA quorum, original API2/R017, network, center DB/CSR/TLS UNKNOWN/capacity/archive, DB migration, protocol0, formal GUI/platform release, G021/G022 and two PLC4X groups. 391 candidates unapplied; no old UNKNOWN redispatch, commit or push.

完整原目标和所有internal pending保留，external deferred仍三类。以下历史保留。

---

# 2026-11-15 当前续接：G034/V62 systemd主密钥目录文件系统边界已闭环，目标active

本节更新当前指针，下方历史原文逐字保留。源/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/source-checkpoint-identity-renewal-v62-credential-fs-reviewed-wip.json，SHA 8c302895b644f8a04cd73de665aeb78bd3de5b009ed57df351c52519733d698a；候选SHA 066e197af6062defb6d3160a67c7e69f0eb9caab2e6bda8a8bf7567ba1d150c9；2254源+4夹具，V51后197，385候选未应用，76快照。核验/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/credential-filesystem-boundary-v1/verification-credential-fs-wip.json，SHA 87d21b24872e61e64e6bac31491fe4fee64db149ae64d2ee309ab3e2b5d39750，1320绑定，旧1226一致。最后接受G033/V61、完整封签V51，G034未封签，apply0、未提交推送。

真实新夹具确认原11 ambient能力可写打开root0400密钥/root0600回执；基线不写字节，所有SHA/inode/owner/mode/mtime/ctime保持。仅改unit ReadOnlyPaths=/opt/eclipse /etc/kura，父目录保持网络/防火墙标记可读，外部root更新可见。新策略11项写/新建/改名/替换/删除/链接/路径别名检查全部拒绝；runtime凭据仍有效。root原子替换无私钥测试标记，同一服务进程读到新值；实际网络/防火墙标记未播种。

初始credfs测试版本被APT按降级在dpkg前拒绝，现场未变；改当前exec序列revision2并先检查版本gt，未绕过降级保护。Core d9f93709、753源pin、261载荷仅unit变化，217 Java与安装脚本/LoadCredential/用户/caps等不变。真实APT升级、GWT/deployment安装、首次Core HTTPS200/ERROR0通过，JVM实际凭据匹配canonical，/etc/kura挂载ro。key/receipt字节、inode、owner、mode、mtime、ctime及TLS/custom/snapshot内容保持，custom snapshot=false不变。新夹具无store.save，未重放旧密文验收。此前真实旧store.save解密闭环保持。

专项CLOSED_SCOPED_CREDENTIAL_FS_8c302895_d9f93709；本轮Core及容器已停止，卷/层保留；18原仓/历史pin/API522一致，原VM仍停止。UNKNOWN和startup.stop不重发、不由停机倒推结算；周48%、普通额度可用、credits/reset0。6包仅Core更新；旧Docker未重建。

该项仅当前systemd文件系统边界；所有启动模式、root攻击隔离、完整特权提供者矩阵仍待办，原11 ambient和NoNewPrivileges0保留。下一步：Implement the typed privileged broker and complete consumers/modes while retaining FileMove/SystemUpdate, network/clock/log/Triton/YOFC behavior and standalone Kura. Keep original 11 ambient capabilities/NoNewPrivileges0 until delegation and resource/ordinary/recovery providers pass; the new /etc/kura read-only namespace is scoped filesystem protection, not full root compromise isolation or all launcher modes. Continue GUI key backup/import/restore, weak-key rotation/CA custody, original API2/R017, network/center DB/CSR/TLS UNKNOWN/capacity/archive, DB migration, protocol0, formal GUI/platform release, G021/G022 and two PLC4X logical groups;385 candidates remain unapplied. No closed-scene replay, UNKNOWN redispatch, original source apply, commit or push.

优先从独立于agent的受控密钥管理入口推进typed root broker及GUI，完整原台账/上游游标及三类external deferred保持。以下历史保留。

---

# 2026-11-15 当前续接：G034/V62 命令字面参数与降权环境专项已闭环，目标active

本节替代下方历史段落的当前指针，原文逐字保留。当前源/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/source-checkpoint-identity-renewal-v62-exec-boundary-reviewed-wip.json，SHA 4c14949d32ba4a87e1119b992456edd7705b662c6402664ecd2becec40077c67；候选SHA eeaeafd25b085960580d2abe75302137b99ed96e0c91272fe22146be6217e31a；2254源+4夹具，V51后197，385候选未应用，76选定快照。核验/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/unprivileged-exec-boundary-v1/verification-exec-boundary-wip.json，SHA 838d94a435f9f34e6fd1b5752e59e6ea6ea2fd65d5997bf353322c3b49d51703，1226绑定、旧1125一致。最后接受G033/V61、完整封签V51，G034未封签，apply0、未提交推送。

本轮修复ExecutorUtil默认argv/环境被原样拼成shell的问题：普通参数字面传递，显式shell仍支持管道，固定su/env/timeout及选项终止，请求环境仅在换用户后应用；无效参数在启动前拒绝、异步拒绝回调一次，减少传输参数日志。公共API及公开签名不变。宿主46回归、实际Ubuntu24.04 ARM64 kurad997→kura1001的10项命令/环境/stdin/异步/停止/超时/拒绝边界通过。stdin测试缓冲顺序修正后只续未通过的5项，首批5项没有重跑。

Core包ac7c83d6，753源pin，261载荷仅Core bundle变化；其内仅ExecutorUtil.class及构建Manifest不同，另216 Java、安装脚本与unit不变。新包实际APT、GWT/deployment安装、首次Core HTTPS200/ERROR0通过；原11 ambient、Bounding13、NoNewPrivileges0、custom snapshot=false保持。未播种或重放旧密钥/密文验收；旧store.save实际解密闭环保留；不是全提供者/模式/权限/GUI发行验收。6包仅Core替换，旧Docker仍旧Core未重建。

专项CLOSED_SCOPED_EXEC_BOUNDARY_ac7c83d6_f73c6cb7；新容器已停止、PID0，卷和层保留。18原仓/历史pin/API522一致，原VM只读仍停止。UNKNOWN与startup.stop不重发、不倒推结算。周48%，普通额度可用，credits/reset0。

下一步：Constrain ordinary daemon DAC writes to canonical /etc/kura/credentials and its receipt; implement a typed privileged broker while preserving FileMove/SystemUpdate, network/clock/log/Triton/YOFC consumers and standalone Kura behavior. Original 11 ambient capabilities and NoNewPrivileges0 remain until all consumers/modes pass. Continue GUI key backup/import/restore, legacy weak-key rotation and CA custody, original API2/R017, network/center DB/CSR/TLS UNKNOWN/capacity/archive, DB migration, protocol0, formal GUI/platform package lifecycle, G021/G022 and two PLC4X logical groups; 385 candidates remain unapplied. Preserve closed scenes and UNKNOWN without redispatch; no original source apply, commit or push.

消费图谱清单尚不等于完整解析/调用覆盖；全部原台账与三类external deferred保持，internal pending不移为deferred。以下历史原文保留。

---

# 2026-11-15 当前续接：G034/V62 root代码、su子进程、升级停机专项已闭环，目标active

本节覆盖下方历史段落中的“当前”指针，原历史记录逐字保留。当前源：/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/source-checkpoint-identity-renewal-v62-root-code-v4-reviewed-wip.json，SHA f1c8a41e13a8ef199f6fd505743bc09b81f633ae6a2dfe7ea2fca68ce5c156f6；候选SHA 292dd2932638cde7c7435b75d6b75399d6af8995a88b0e59068433bf4df280ec；2250源+4夹具，V51后193，381候选未应用，72选定快照。核验：/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/root-runtime-ownership-v1/verification-root-code-wip.json，SHA 62f69de39c56ec56abfa16b046f2672d249e9ebcc7f7113e37fc9c433d68bb56，1125绑定、旧928逐一匹配。最后接受G033/V61，完整代码封签V51，G034未封签，apply0、未提交推送。

实际完成静态代码root归属、/opt/eclipse只读与5私有数据根、恢复后仅归一代码权限、安装子进程失败传播、已有TLS身份保持、su子进程服务cgroup内退出和其他调用者原会话保持。新增21+32+9类回归分别在宿主及Linux通过。修复真实升级直接kill引起自动重启与目录替换NAMESPACE错误：key先保全、同步systemctl stop并核对完整停止后才复制/清理。Core9d48c92c真实v3→v4 APT成功、无替换期间重启、原key字节/inode/root0400及TLS/custom/snapshot内容保留；v4首次启动HTTPS200、ERROR0；安装的v4 prerm实际停止并7秒无重启，既有字节/inode/权限保持。计数观察器2→0误判已归档，接续同一进程，未为观察失败重启。

原11 ambient及NoNewPrivileges0保留，Bounding13仅允许现有SUID su换uid/gid。完整最小权限/所有提供者、FileMove/SystemUpdate/root broker及普通恢复/资源失败仍internal pending。用户确认的custom覆盖base已源码与现场核对；user/kura_custom.properties中snapshot=false保留，仅控制快照整体加密。本新夹具无store.save，不宣称重复完成旧密文解密；旧真实解密闭环原证据保持，未重放。6包仅Core更新；旧Docker仍旧Core，未重建为本包。

专项CLOSED_SCOPED_ROOT_CODE_SESSION_UPGRADE_f1c8a41e_9d48c92c；本轮容器已停止、PID0，层和卷保留；原Orb/Ubuntu/Windows VM均只读仍停止。18原仓、历史pin及API522一致。UNKNOWN与startup.stop不重发、不倒推结算；周48%，普通额度可用，credits/reset0。

下一步：Continue all privileged consumers and modes before reducing the original 11 ambient capabilities or enabling NoNewPrivileges: FileMove/SystemUpdate/root broker, deployment and provider writers, actual recovery/ordinary/network/boot paths and resource failures. Implement GUI key backup/import/restore and legacy weak-key rotation/CA custody. Preserve all original API2/R017, network, center DB/CSR/TLS UNKNOWN/capacity/archive, DB migration, protocol0 online upgrade, formal GUI/platform release, G021/G022 and two PLC4X logical groups; 381 candidates remain unapplied. Do not replay closed key/data/JNI/modern17/cold-polkit/Docker/PAM/this root-code scoped cases, redispatch UNKNOWN, apply original sources, commit or push.

完整原台账、381候选、上游审查游标与逻辑组范围保留，internal pending与三类external deferred分开。以下为历史记录。

---

<!-- G034 V62 ATOMIC SU PAM WIP CURRENT 2026-11-15 -->
## 当前 WIP：su PAM 成对规则原子生命周期已修复并完成专项验证

最新源 2d6400198879d534a53dcd048c7d87938f4b3ebf5deb578f03103b8315e62b47（2247源+4夹具，V51后190差异，378候选未应用，69选定快照），候选 c9cfa127e22a76891f8f73e6c77b491effaac82e62bcf1e7b4a6f9d15ff7578d；核验 6078057051c151412fcc9b89c47caa4283586d52d0e1b5d9dbf3e2300d3d0334，当前928条绑定，旧874条全部一致。证据 `/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/core-su-pam-policy-v1`。仍为G034/V62未封签WIP，最后接受G033/V61、完整封签V51，apply0、无提交推送。

实际权限消费者审查确认：Core UnprivilegedExecutor以su切换到kura；现有PAM目标限制与调用者授权必须成对。旧安装每次在rootok后插入两行，重复升级会累积；旧卸载实际执行两次独立sed删除（最初读片段漏第二行的判断已撤回，原误读记录标superseded）。单次正常安装+完整两次删除可以移除一对，不宣称该正常终态会授权root。缺陷是第一次删除后的中间/失败状态留下无目标限制的kurad授权，以及多次安装后完整卸载仅移除最新一对，旧对仍残留。真实Ubuntu24.04 ARM64 PAM/su已分别复现：重复安装后旧完整卸载仍可kurad→kura；两次旧写入之间可kurad→root。

仅影子修改3现有路径manage_users、kura_install、prerm，新增独立kura_su_pam.py及测试2文件。新helper只维护已知两行及明确管理块，检查rootok/类型/所有者/模式/链接/大小，未知块或非标准anchor保留拒绝；内容SHA命名root私有备份，源身份复核后一次os.replace+fsync发布。重复安装不增行、不换inode，卸载清理所有已知历史对及中断孤立授权，保留其他PAM配置；备份冲突、并发、复制失败/源被编辑时拒绝覆盖。安装入口在standalone文件/密钥/用户/unit变更前PAM只读预检，manage_users在用户变更前检查/撤权；支持实际复制到.data的脚本路径。真正DEB prerm在remove保存数据后、拆除集成前调用helper撤权，失败直接返回；upgrade路径不撤权。

24项主机及24项实际Linux单测通过；真实libpam1.5.3/util-linux2.39的14项认证决策通过，覆盖基线、旧两种缺陷、修复后kurad→kura允许、kurad→root拒绝、nobody→kura拒绝、root原通路保持、卸载撤权；原PAM文件恢复逐字一致，备份600。新Core包dbec2f03958eb85e7c464dc8e1438f46716c386491bfe3ebac7a361e17432669由746条Core源pin构建；260常规载荷，仅2安装脚本+新helper共3路径变化，217 Java JAR、嵌入preinst、密钥helper、原service/custom配置不变，嵌入prerm与当前源逐字一致。

两台独立Ubuntu夹具执行真实新DEB解包出的入口：实际polkit状态预检、异常PAM在standalone变更前拒绝、install和.data两个manager路径就绪，真正嵌入prerm remove成功撤权并保存factory snapshot；异常管理块使manage -u/prerm remove失败，保留原策略、账户及拆除标记。此为解包入口专项，不冒充dpkg完整安装/运行/卸载验收，不播种旧密钥/密文。6包集合仅Core换新包；已闭环Docker镜像仍绑定此前f41d380c Core，本轮未重建Docker、不宣称全发行已配齐。现代systemd policy writer与原service逐字不变，旧17/密钥数据/JNI/polkit冷安装/Docker专项不重跑。

专项闭环 CLOSED_SCOPED_SU_PAM_LIFECYCLE_2d640019_dbec2f03。4个自有新容器已确认terminal、PID0，层保留；原APT缓存只读复用，未挂载旧key/data，原VM只读仍停止。18原仓所有head/branch/index/staged/ref/remote/dirty源码、历史pin、共享API522不变（仍仅4个审查前缀允许写）；原UNKNOWN及startup.stop仍UNKNOWN、不重发、不倒推结算。周额度46%，普通可用，credits/reset0，目标active。

下一步优先实际root helper与代码所有权/可写消费者审查：当前create_users仍递归chown运行目录给kurad，含bin/install；原unit仍11个宽ambient能力含DAC override/read_search，SUID su仍依赖当前NoNewPrivileges行为。不能因本PAM专项通过就宣称最小权限完成或直接开启NoNewPrivileges。完整提供者/UID/安装失败/非标准PAM与历史模式、root脚本/不可变代码归属、GUI密钥备份导入恢复、弱密钥轮换、CA多人控制、原7组API2/R017、网络各路径、中心/旧DB/CSR/协议/正式GUI发行、14组/378候选及G021/G022/PLC4X两逻辑组均保留internal pending；external deferred仍仅NM/cloud-init/物理、amd64、磁盘满/硬断电3类。

<!-- prior review retained byte-for-byte below -->

<!-- G034 V62 DOCKER PRIVATE KEY WIP CURRENT 2026-11-15 -->
## 当前 WIP：Docker 独立私钥、旧密文迁移与同卷重建专项已通过

当前源码 aff8831c2e20c98cfb4b1abe332baa2c6be13781451b17f27a3d5b0e9c1660e1（2245源文件+4夹具，V51后188项差异，376项生产/测试候选尚未应用，67个选定快照）；候选 8c233eb49f1ee8e3708f9f13d09d97f7c1484de50559e8dfb29b29d49cf63f35。核验 ab9af96c183b0b72d7b3c605ae6dbed6a0f7214c09a667417b620e9611f5afb5，继承755条不可变绑定并逐项核对，当前绑定874条。仍为G034/V62未封签WIP，最后接受G033/V61、完整代码封签V51；管理源码apply=0，未提交推送。新证据目录 `/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/docker-private-key-v1`。

按用户纠正再次核实SystemServiceImpl：先加载kura.properties，再以putAll覆盖kura_custom.properties同名项。保留原配置和Java快照逻辑，当前custom中kura.snapshots.encrypt=false；实际新容器产生明文XML快照。此开关不关闭store.save中的独立AES口令加密。

关联消费者审查发现Docker镜像固定公开加密密钥、构建时生成HTTPS私钥、只持久化部分user目录、启动eval/跟踪及错误home路径。仅在影子候选修复4现有路径，新增bootstrap和测试2文件：首次启动为每卷生成32字节私有主密钥和独立HTTPS私钥；完整user/data/packages与凭据同卷保存，保留逻辑路径；凭据优先、支持可核验旧来源，已识别旧密文必须用选定密钥通过GCM认证；来源不明、密钥冲突或原TLS缺失时保留并拒绝，不自动换密钥。去除镜像默认密钥和构建期TLS私钥，JVM通过私有凭据加载、不把密钥留在参数或旧环境变量中；exec作为PID1，实例锁防同卷并发写。

真实OverlayFS测试暴露目录rename的EXDEV及版本目录symlink父目录fsync问题，已补复制、全量校验、fsync并保留备份，归档失败保留原目录。23项主机测试及23项实际Linux ARM64测试均通过。重新使用现有179项目Java队列的私有缓存，仅打包deployment/wires；当前Core f41d380c与GWT包逐字复用。4DEB输入构建新Alpine ARM64镜像 `sha256:75c84dad9429b12b1c1faee647e833130efd7e29b50df506912a10217592285c`，原latest标签不变，DEB只读构建挂载减少COPY缓存。原Core及217Java JAR、原6包管理发行集合不变；没有重跑旧agent720/旧IDEA/已闭环场景。

5个独立实际迁移用例通过：历史Core默认、旧Docker默认、自定义密钥保留；错误密钥、缺失原TLS拒绝。旧store.save及加密快照由实际打包CryptoServiceImpl生成并解密，原文件字节/inode均保留；仅为合成旧数据专项，不冒充完整活体旧网关升级。两个新实例主密钥及证书各自独立；使用原数据卷重建后，主密钥字节/inode/0400模式、receipt和HTTPS证书保持，实际Core解密全部运行时口令，原TLS私钥签名可用，正常重定向后的登录页面200、应用ERROR=0。Core正常更改密钥库口令会改PKCS12文件字节，已通过实际口令解密及证书/签名确认不是TLS密钥丢失，不声称所有运行时文件逐字不变。

首批网络none夹具未给容器主机名地址，出现UnknownHost与启动延迟；同卷重建夹具改为可解析的localhost，未改全局DNS、生产配置或放宽沙箱。失败镜像/容器/卷均保留。闭环 `CLOSED_SCOPED_DOCKER_KEY_MIGRATION_aff8831c_75c84dad`，后续不重放。本轮所有自有容器已按原ID/PID1 ticks核对后停止，镜像/层/卷保留；原OrbStack VM、Ubuntu/Windows Parallels只读确认仍停止，未启停原场景。18原仓HEAD/分支/index/暂存/ref/remote/dirty代码、旧pins和共享API522均不变（只允许4个审查前缀更新）；原UNKNOWN操作及startup.stop仍UNKNOWN，绝不重发或倒推结算。周额度46%已用、普通额度可用，credits/reset使用0，目标active。

仍待实际实施：GUI密钥备份恢复、旧容器层私钥/自定义来源的受控导入、弱密钥轮换、完整失败/权限/提供者/安装模式矩阵；Core/容器root、CommandExecutor/su和能力集/NoNewPrivileges仍需按实际消费者审查，不能因本专项通过而直接收紧破坏su。CA托管/多人控制、原7组API2/R017、网络普通/恢复/owner各路径、中心地址DB/CSR旧恢复/协议升级、数据库非空旧加密任务、正式GUI/发行生命周期、14组/376候选守卫应用、G021/G022及两组PLC4X逻辑上游等完整台账全部保留为internal pending。外部deferred仍仅NM/cloud-init/物理、amd64、磁盘满/硬断电3类。旧0.105生成器已按核定Ubuntu矩阵退休，不另增旧OS工作；旧key/data、JNI、现代17授权、polkit冷安装均保持闭环。

<!-- prior review retained byte-for-byte below -->

<!-- G034 V62 POLKIT RELEASE CONTRACT WIP CURRENT 2026-11-15 -->
## 当前 WIP：前置依赖与 polkit 自动就绪已落地；标准 APT 冷安装及首次启动通过

核对正式方案557/678行，发行矩阵为 Ubuntu24.04/26.04 × amd64/arm64；0.105旧平台不在约定范围，不新增旧OS支持工作。已删除旧pkla通配授权生成分支，保留历史文件的卸载/现代迁移处理；安装脚本要求已配置的polkitd>=0.106，并先验证版本格式再比较（含epoch处理）。standalone入口在文件合并、密钥准备、服务/用户变更前预检，用户创建也先检查。缺失、未配置、旧/畸形版本均拒绝。新Core控制字段为Pre-Depends: python3, polkitd (>=0.106)，明确systemd依赖，删除已移除Bluetooth的强制包依赖；原NM等依赖未在本轮扩改。

实际安装器在账户及策略写入后自动ReloadConfig并要求原polkit服务就绪，不重启系统D-Bus、不削弱沙箱。独立新Ubuntu24.04/systemd255 ARM64容器初始无Core/无polkitd/无Core key；使用已有签名Ubuntu索引和缓存、标准可写APT目录、网络none，一次正常APT install直接完成新Core及依赖。真实日志显示polkitd配置早于Core解包；dpkg audit干净，polkit按原NoNewPrivileges/MemoryDenyWriteExecute沙箱运行，没有手工先装polkit或手工重载总线。Bluez守护进程未被本轮强制依赖拉入。首次Core以kurad997运行，应用ERROR=0，原unit无drop-in、TasksMax614，custom文件快照加密false保持。Core自身NoNewPrivileges仍0/原11项ambient能力，完整最小权限未验收。

9项Linux真实dpkg预检测试通过，覆盖缺失/未配置/0.105/epoch/畸形版本及普通安装入口变更前拒绝。初始8过1失败暴露dpkg compare对畸形版本只警告，已补validate-version；初始候选revision2及失败保留、未安装，最终revision3由修复后的输入构建。两脚本语法通过。新包 kura_6.0.0~g034.v62.policy20261115-3_arm64.deb SHA256 f41d380cc176d95f6b0432219e6f889daa8c15df551156f416c62c0c8c5c8f9c；259常规载荷仍仅2安装脚本变化，217Java JAR、密钥helper、嵌入preinst、原unit、自定义配置不变，控制元数据按上文变化。现代writer函数及实际策略SHA3545f728与上轮17决策场景逐字相同，沿用闭环，不重放17查询或旧key/data seed/升级/remove/reinstall。

当前薄源 source-checkpoint-identity-renewal-v62-polkit-contract-wip.json SHA256 2d84af7c40d978f3818cf94df75c3b201de00ecf696803b80601e4457af00b98：2239源码+4夹具、较V51差量182、370候选UNAPPLIED、61选择映射。新增control模板和预检测试，构建744个Core源码pin逐字匹配；新构建集使用APFS写时复制，保留初始失败薄源及最终单文件修正，不改原18仓源码或共享API522。六包清单位于native-polkit-release-contract-v1，余5包摘要复用，核验索引 verification-polkit-release-wip.json。

本轮PID3671/ticks896154及容器已停止留层留卷，原VM和旧容器未启动。旧密钥/旧数据scene仍CLOSED_SCOPED_NATIVE_CORE_KEY_DATA_258aa82c；不把当前新包扩大为全密钥生命周期或全正式平台验收。kura_custom.properties覆盖和加密语义保持，UNKNOWN不重发，不提交推送、不局部apply。G034未封印，G033/V61和FULL CODE V51保持；原3类external deferred保持，未新增“旧polkit平台deferred”。

下一实际实施：先追踪CommandExecutor/su和受限系统文件写入的真实消费者，收敛运行身份/能力/NoNewPrivileges；继续界面密钥备份恢复、受控弱key轮换、原七API2/provider/恢复/网络/正式发行、两组PLC4X及完整台账。仅当前Core包冷安装/原生启动子集完成，其他内部pending不外推。周45%，普通额度可用，额外/reset credits消耗0。下方历史逐字保留，旧0.105“待支持”表述以本段矩阵核定取代。

<!-- G034 V62 MODERN SYSTEMD POLICY WIP CURRENT 2026-11-15 -->
## 当前 WIP：现代 systemd 授权已修复并验包；旧版 polkit 和完整最小权限仍待实施

本轮修改 shadow 中2个既有安装脚本并新增1个测试文件，未改原18仓源码，不提交推送。运行期真实调用只需 DNS named/bind9 的 start/stop/restart 与 chrony/chronyd 的 start/stop；root 安装脚本自行处理 enable/disable。现代规则按 unit+verb 收紧，对 kurad 的其他 systemd 写操作明确拒绝，移除会通过 imply 绕过服务名单的 manage-unit-files 宽授权及已删除 Bluetooth 的服务/DBus授权创建。其他用户、NM/Modem/power 规则的原有语义不扩大。旧0.105分支尚保留历史通配授权，仍是内部pending，不声称全平台已修复。

现代安装不再只在不存在时写规则：原规则按内容SHA生成 .kurasave 原字节备份，再原子替换；遗留 systemd1.pkla 通配授权同样备份后移出活动路径，备份root/0600且不会被polkit加载。拒绝符号链接、非普通文件、备份冲突与未知profile；重复配置保留规则inode。kura_install.sh 传播用户/权限脚本失败。11项宿主文件迁移/JavaScript授权测试和两脚本语法检查通过。

实际 Maven3.10/JDK21 用独立缓存构建新 Core kura_6.0.0~g034.v62.policy20261115-1_arm64.deb，SHA256 9b4d326aea0b62646d6bc36ac22d96eb3c85d964bbb7236e959e1c2d271f5d6f。259项常规载荷中仅2个安装脚本变化，217个Java JAR、密钥helper、嵌入preinst、原unit和自定义配置原字节不变；其余5个包复用并校验摘要。源码2237+4、较V51差量180、368候选UNAPPLIED、59选择映射；当前薄源 source-checkpoint-identity-renewal-v62-systemd-policy-wip.json SHA256 d62a94293d7aad24796fea828ae328a2d5a78bc4a2dfc4fd507f893582f90363。仅新增3份选择快照，不复制全源码证据；构建集采用APFS写时复制。

新独立ARM64 Ubuntu24.04/systemd255容器中预置旧规则和旧pkla，再由实际新DEB正常配置。两份旧规则原字节备份root/0600、legacy活动文件移除、实际.data维护脚本与归档逐字相同。真实polkit沙箱保持，原Core unit无drop-in、TasksMax614。实际kurad996 JVM经17项原生pkcheck决策：必要DNS/clock允许，无关服务/已删Bluetooth/异常verb/缺失参数/任意unit-files/reload-daemon/set-environment拒绝，真实nobody负控拒绝，NM原规则保持。查询没有实际调其他服务或改网络。应用ERROR=0。初始只读探针错读已移除的install目录，保留失败后仅改读.data完成核对；未重装/重跑配置。

本轮现场仅用于现代规则文件迁移和授权决策，不是全包升级、实际DNS/chrony启停、旧polkit、所有profile、完整能力/UID/sandbox/资源或全平台验收。原Core仍11项ambient能力及NoNewPrivileges=0，全最小权限继续内部pending。PID3928/ticks728968及本轮容器已停止留层/留卷，其他旧容器与原VM未启动。旧密钥/旧数据验收保持 CLOSED_SCOPED_NATIVE_CORE_KEY_DATA_258aa82c；未重播seed/升级/remove/reinstall，不向当前新包扩称全密钥生命周期已过。kura_custom.properties 的覆盖与快照开关继续保持，UNKNOWN不重发。

核验目录 native-systemd-policy-v1，索引 verification-systemd-policy-wip.json。G034仍未封印，G033/V61及FULL CODE V51保持。下一步处理0.105服务授权及完整运行身份/能力/沙箱，再推进界面密钥备份恢复、受控弱key轮换、原七API2/provider/恢复/网络/正式发行、两组PLC4X和完整台账。仅原3类external deferred，内部pending不外推。周45%，普通额度可用，额外/reset credits消耗0。下方历史逐字保留，以本段更新当前状态。

<!-- G034 V62 NATIVE READINESS DIAGNOSIS CURRENT 2026-11-15 -->
## 当前 WIP：自定义覆盖已核实；原生 JNI 通过，polkit 越权原因已定位

用户本轮要求再次核实覆盖关系。实际源码 SystemServiceImpl 236–237 行在基础配置后执行 kuraDefaults.putAll(kuraCustomProps)；ConfigurationServiceImpl.writeSnapshot 从合并结果读取 kura.snapshots.encrypt。当前新原生现场 user/kura_custom.properties 第23行为 false，基础文件无此键；保留自定义覆盖及快照开关，不修改 Java 加密逻辑。上一轮原生密钥/旧数据生命周期仍闭环，本轮未重播 seed、升级、remove/reinstall 或旧验收。

本轮仅复用同一 Core revision3 DEB，未修改源码或重新构建。真实 JNI 错误是旧 Docker /tmp tmpfs 的 noexec 阻止 mmap；实际 aarch64 fragment 中的同一 SO 配合真实 libudev/libcap 在可执行缓存下加载通过。新独立 ARM64 Ubuntu/systemd255 容器使用 exec /tmp、容器内 shared /run 和4096 PID预算，原包 unit 无 drop-in，原生默认 TasksMax614。安装全部离线依赖后，kurad996 的 Kura、USB DS、实际 LinuxUdevNative 线程运行，应用 ERROR 为0；未验收物理 USB/hotplug、noexec 缓存方案或全平台。

polkit 初始失败发生于 Core 安装前：D-Bus 在 polkitd 账户创建前读取策略。仅重新加载本轮容器总线配置后，原包 polkitd 的 NoNewPrivileges/MemoryDenyWriteExecute 沙箱保持并正常运行。另确认真实发行授权缺陷：kura.rules 无条件授予 kurad manage-unit-files，已安装 systemd policy 的 imply 连带授予任意 manage-units/reload-daemon，绕过表面服务名单。仅在本轮容器临时移除此一授权，真实 Kura PID 的只读 pkcheck 查询显示 named restart 仍允许，cron restart、无关 enable 和 reload-daemon 均需认证；逐字恢复原规则后宽授权复现。未实际重启其他服务、启停单元或写网络，未把实验规则写回源码/发行包。下一实际实施是核清真实服务调用并收紧现代/旧版授权；完整 UID/polkit/能力/沙箱/资源矩阵仍为内部 pending。

源码继续沿用 source-checkpoint-identity-renewal-v62-native-key-wip.json SHA256 258aa82c9d79b4677157fca86726b67147d42d2b6a12b0196e16a8844e4ff03a：2236源码+4夹具、较V51差量179、367候选UNAPPLIED、58选择映射，原6包不变。新核验目录 native-runtime-readiness-v1；本轮容器、Core PID3991/ticks577895及两个JNI探针全部终态并留层/留卷，旧密钥现场未启动。原18仓/共享API522/UNKNOWN 保持，不提交推送，不局部 apply。G034未封印；G033/V61和最后 FULL CODE V51 保持。界面密钥备份恢复、弱key受控轮换、原七项API2/全部provider与恢复模式、正式发行、两组PLC4X及完整台账继续；仅原3类 external deferred。周额度44%，普通额度可用，额外/reset credits消耗0。下方历史逐字保留，旧下一步由本段更新。

<!-- G034 V62 NATIVE KEY AND CUSTOM OVERRIDE WIP CURRENT 2026-11-15 -->
## 当前 WIP：原生systemd密钥生命周期通过；YOFC自定义配置覆盖关系已核实

用户本轮澄清以配置决定快照加密，并指出kura_custom.properties覆盖基础配置。已核对SystemServiceImpl：先加载kura.properties，再加载user/kura_custom.properties，236–237行putAll覆盖同名属性。实际新装的user/kura_custom.properties含kura.snapshots.encrypt=false，基础文件未声明此项；原生配置DS ACTIVE且snapshot_0仍XML明文。该用户文件本来就随user/完整保留，实际升级、卸载留存、重装后SHA均相同。先前只查基础文件而提出的额外属性迁移不适用当前YOFC配置；已完整撤回该临时parser/控制脚本/6项测试，未构建或安装它们。最终保留的生产改动只有密钥凭据ACL校验修复，不改Java开关、默认值或自定义覆盖机制。store.save中的密码沿用独立AES保存语义。

真实systemd255作为新ARM64隔离容器PID1，全124项签名Ubuntu依赖按要求配置，dpkg审计通过；实际core DEB revision1→2→3升级、服务启动、remove、保留数据reinstall及再次启动全部完成。两处原生问题已在shadow修复：systemd按POSIX ACL仅授权服务UID时文件呈root/0440，启动守卫需识别合法ACL；管理员捕获运行中凭据也需校验实际服务UID。仍拒绝普通组可读、额外读者、错误UID及异常ACL，canonical root/0400规则保持。宿主与Linux最终输入各32项通过；revision3归档中的单一helper和嵌入preinst原字节一致，217个Java JAR不变，其余5包摘要复用核验。

kurad UID997通过真实LoadCredential运行，无用户密钥环境变量；真实Crypto、Configuration及System DS ACTIVE。仅消费实际CryptoService的专用测试bundle单次写入加密store值和快照流，未提供替代provider；fixture consumer已持久停止，后续两次启动未再次加载。实际加密store在升级和重装后均逐值解密通过；原32字节key、收据、key inode及9项用户/数据文件原字节一致，user自定义false与明文snapshot_0保留。真实preinst备份和remove留存中的key均root/0400，私有备份保留。新装store按需生成，最初等待其在启动时出现的探针假设已纠正，未重放seed。旧16字节迁移/加密流验收沿用已闭环结果。

夹具边界如实保留：Docker本轮/run tmpfs原为private，凭据挂载未传递；只在该容器内改为shared后正常，未改宿主挂载。systemd按总512进程派生服务TasksMax76，曾导致线程失败；仅该夹具运行时调至384，总512、内存1536MiB保持。实际unit仍有原11项ambient能力且NoNewPrivileges=0，未验收最小权限/完整沙箱。LinuxUdevNative加载错误及初始polkit失败的完整权限复核仍属内部pending，不宣称全部原生运行健康、JNI/网络/恢复模式或完整平台发行通过。APT首次本地路径内部错误在Core脚本前退出，核实未写key后只续未完成的unpack/依赖配置；所有失败和诊断留存。

当前薄源 `/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/source-checkpoint-identity-renewal-v62-native-key-wip.json` SHA256 `258aa82c9d79b4677157fca86726b67147d42d2b6a12b0196e16a8844e4ff03a`：2236源码+4夹具、较V51差量179、367候选UNAPPLIED、58选择映射；较上一当前2b0913c0仅2个helper/测试文件变化。六包清单与验证索引位于 `/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/systemd-key-lifecycle-v1/live-credential-acl-fix`，索引名verification-native-key-wip.json。原18仓、共享API522、旧UNKNOWN和既有闭环保持；新建容器全部停止留层/留卷，不提交推送或局部apply。G034/V62仍未封印，最后已验收组件G033/V61和FULL CODE seal V51保持。

下一步处理实际包JNI/原生权限/完整API与恢复模式，再推进界面密钥备份恢复、受控旧弱key轮换及下文全部内部台账；旧弱key只保留迁移未轮换，原个人/生产数据未在本测试解密。仅原3类external deferred，内部pending不外推；两组PLC4X新增功能适配及其余全台账保持。周额度44%，普通额度可用，额外/reset credits消耗0。旧operation 767863a9-fe6b-4135-9783-ddcb131030af及startup.stop UNKNOWN不重发、不补结算，原VM不启动。下方历史逐字保留，旧“下一步”以本段为准。

<!-- G034 V62 CORE KEY CONTINUITY WIP CURRENT 2026-11-15 -->
## 当前 WIP：私有密钥自动配置、旧密钥迁移与旧加密数据连续性已在隔离现场验证

按用户最新要求优先完成密钥和数据连续性。仅在 shadow 改14份构建/安装/启动/测试输入：新装自动生成32字节随机密钥，保存在 /etc/kura/credentials（目录0700、密钥root/0400），systemd LoadCredential 提供给服务；正式unit删除固定开发密钥。三种启动入口与unit使用0077，未提供环境变量也可正常启动。升级先核对并保存原密钥，再允许停止/清理旧程序；不重生成覆盖旧密钥，不盲目重加密旧数据。旧标准unit环境密钥、根私有凭据和实际运行Java凭据按明确合同核对；不支持的自定义来源、来源冲突、异常权限或丢失旧密钥均在破坏性步骤前拒绝。复杂drop-in/EnvironmentFile等未自动迁移，不能宣称覆盖全部历史部署。

修复原安装/用户管理脚本递归处理整个 /opt/eclipse 导致私有备份归属被改的问题，改为只处理当前物理Kura运行目录。实际最终core DEB重建成功，259普通文件较上版变化8项，217个Java JAR原字节不变；其他5个配套包复用并核验摘要。构建只写任务隔离缓存，共享API522和原18仓功能源码保持。宿主密钥生命周期24项、Linux24项、原启动守卫/参数21项通过；最终增量打包也成功，未重放旧绑定/CSR/续期或旧整套测试。

实际Core CryptoService验证：16字节旧密钥迁移前后，原加密值与AES快照流可解密；不同密钥被拒绝；留存备份恢复后仍可解密且密文不变。真实Kura生成的store.save是Java Properties容器，逐值解密通过，密钥/文件SHA前后精确一致；最初将整个Properties文件误当单个密文的探针错误已修正并保留失败记录，不能当作数据损坏。OrbStack被外部中断后，先只读核验原32字节密钥未变，原JVM已退出且没有重启；仅恢复本轮容器的sleep入口以续做未完成核验，没有重放已经通过的迁移/留存恢复。

最终v4实际DEB又在新的物理运行目录保留原10个用户/数据文件；执行实际账号助手和安装器权限代码，私有备份密钥仍root/0400、备份unit仍root。仅启动一次新权限版本，调用者umask0000仍得到0600 PID文件；不传用户密钥环境变量。原应用store逐值解密再次通过，旧物理目录、原密文、原密钥与备份均保留。核验完只向同一已核对PID发一次SIGTERM，权威终态已确认；本轮容器停止留层/留卷。范围是隔离root/Core现场与真实加密实现，未解密原个人/生产数据，未宣称完整快照/CM恢复或原生systemd正式生命周期通过。

当前薄源 `/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/source-checkpoint-identity-renewal-v62-core-key-wip.json` SHA256 `2b0913c002187a7e812ae3146ed5ab1723e80fbc3a246f0f9e8e3e0fe3e236e3`：2236源码+4夹具、较V51差量179、367候选UNAPPLIED，58份选择映射并沿用V61基线。当前六包清单及核验目录 `/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/core-encryption-key-v1`，验证索引 `verification-core-key-wip.json`。固定开发密钥的新装源缺陷已在候选修复；既有弱密钥仅保留迁移，没有轮换重加密，安全收尾仍待做。G034/V62继续WIP未封印，G033/V61及V51最后完整代码封印保持，管理候选未回写原仓，不提交推送。

下一步推进原生systemd LoadCredential/UID/polkit/沙箱和正式安装升级卸载的凭据连续性，再接界面备份恢复/轮换、完整API/provider/恢复/网络模式、包解析与下文全部内部台账（含两组PLC4X新增功能适配）。内部pending不转外部deferred，仅原3组external deferred保留。原旧operation 767863a9-fe6b-4135-9783-ddcb131030af及startup.stop UNKNOWN不重发、不补结算；原VM不启动、闭环验收不重放、现场不清理。以下历史逐字保留，旧“下一步”以本段为准。

<!-- G034 V62 REAL NM PROVIDER WIP CURRENT 2026-11-15 -->
## 当前 WIP：修复 networking 安装参数，真实 NM 与非 root Kura 装配通过

实际 networking 安装器把 install,true 传给只接受首个 networking 布尔参数的子脚本，导致退出0却跳过防火墙模板初始化。已用原DEB实际脚本复现，shadow只移除错误的install参数，重打networking DEB成功；34普通文件仅该脚本变化，9个Java JAR原字节不变。直接执行新DEB中的调用行，真实模板正确移动/定制，net.interfaces为lo,eth0；core及其他配套包复用。此前core启动入口修复保留。

新ARM64 Linux命名空间使用真实NM1.54.3、D-Bus及polkit，实际core/networking/GWT/agent四包311普通文件逐项核对。执行原包账号/核心定制及修复后的network profile步骤，kurad UID990以原service中的4项ambient能力子集运行；NM、bridge、GWT及配置/status DS ACTIVE，NM的service.pid和kura.service.pid均为org.eclipse.kura.net.admin.NetworkConfigurationService，health组件由缺依赖变为SATISFIED，HTTPS登录页200，最终Kura应用日志ERROR为0。查询7类当前管理API有5类真实注册；另2类恢复服务只在离线RecoveryBootstrap的受控恢复路径注册（completion还需明确opt-in），不人为补provider；此项不等同原台账七项API2迁移或完整恢复矩阵。未启动agent/UDS或启用远程。

范围仍有限：容器无systemd，DNS provider构造仍报bind9/named unavailable，ModemManager position DS缺可选provider；不宣称所有bundle生命周期、完整网络模式、权限/UID/polkit/sandbox及正式安装升级完成。非root进程带dac_override等原unit能力，不能推成最小权限已过。原包解到usr-merge根会替换/lib别名，已保留失败目录，通过Docker档案API恢复OS别名并在私有目录完整解包后合并，属于夹具修正；第二运行代漏新profile的正常bin chmod而退出13，未启动业务，保留后第三代通过。所有代的数据、旧缓存和同一私有随机加密credential保留，未换密钥重试。新容器和全部子进程已停止留卷。

另发现正式core unit仍含固定开发加密密钥，记为内部发行阻断：新装需自动生成私有密钥，升级必须先保留/核验既有加密快照和store的密钥连续性，不可盲目重生成；本轮只评审记录，原unit未改。下一实际实施优先这条凭据发行/连续性，再继续真实systemd、完整API/provider/GUI/权限/生命周期及下文完整台账；仅原3类external deferred，内部pending不外推。

当前薄源 `/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/source-checkpoint-identity-renewal-v62-network-profile-wip.json` SHA256 `c0f89c42a8cfe82beb407d675b281ec5b568318873ade35670757ec4608be100`：2225源码+4夹具、较V51差量166、354候选UNAPPLIED；新增1份安装脚本薄快照，沿用45份映射及V61只读基线。六包清单/当前核验 `/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/packaged-network-provider-v1`。18原仓、共享API522、旧UNKNOWN及闭环现场保持，不局部apply、不提交推送；G034仍未封印，G033/V61及V51最后完整代码封印保持。历史全文如下。

<!-- G034 V62 PACKAGED STARTUP WIP CURRENT 2026-11-15 -->
## 当前 WIP：修复实际发行包前台启动，独立及 agent 不可达启动通过

实际旧 core DEB 的 start_kura.sh 错将 ${DIR} 转义为字面路径，并遗漏 sibling 根目录；全新隔离现场复现空配置/JVM退出13。仅在 shadow 修正该调用的展开、引用及第四参数，新增一个已有源码候选。重新构建 ARM64 core DEB 成功；与旧包逐文件比较，257普通文件仅此启动脚本变化，全部Java JAR原字节相同，其他5个配套DEB不重建、不重跑已闭环业务验收。

两个实际DEB解包运行现场已通过：core+GWT无agent时，API/GWT bundle及Console DS ACTIVE，HTTPS登录页200；加实际agent包但不启动agent/UDS时，适配器自动发现一次、bundle和bridge DS ACTIVE，RemoteManagementService真实注册，HTTPS登录页仍200。两者Kura应用日志无ERROR。正常安装别名、bin执行位及本地HTTPS证书在夹具中明确建模，未执行包维护脚本；网络隔离需本机hosts解析。前两次解包夹具分别在别名顺序/缺安装chmod处停止，均保留，不算产品缺陷；读取Gogo需TTYPE=xterm。登录页断言已按YOFC实际compiled login module修正，没有重放HTTP请求。离线CRL/NTP告警不改全局配置。

本轮不含networking DEB，network.status.health DS缺真实网络provider的UNSATISFIED REFERENCE保持公开；不以独立启动代替全部7类API-provider、网络模式、Java/Rust注册、菜单浏览器、非root/UID/polkit/systemd沙箱、安装升级生命周期或正式平台交付验收。六个新容器均已停止且保留卷；原18仓现场、旧UNKNOWN和已闭环写操作保持原样，不提交推送。

当前薄源 `/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/source-checkpoint-identity-renewal-v62-launcher-wip.json`，SHA256 `c84302d9e1fa2d93d0f608a2bab5b066f00c241e532f9fffe168d674ba71b596`：2224源码+4夹具、较V51差量165、353生产/测试候选UNAPPLIED；新存1份启动脚本，沿用既有快照。当前包清单 `/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/packaged-kura-boot-v1/management-distribution-package-set.json`；当前运行核验目录 `/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/packaged-kura-boot-v1`。仍是G034/V62 WIP，未封印、未回写原功能源码；G033/V61组件验收及V51最后完整代码封印保持。完整后续台账及仅3类外部deferred沿用下文，内部pending不转deferred。

<!-- G034 V62 ACTUAL KURA DISTRIBUTION WIP CURRENT 2026-11-15 -->
## 当前 WIP：修复 agent 缺 Java 适配器的发行问题，六个配套 DEB 已构建

发现旧agent DEB没有任何Java JAR，而正常core/networking发行配置也没有交付联调所用的management.bridge；此前手工组装的运行集不代表正式安装可用。现仅修改shadow agent打包器及其检查：正式deb必须提供摘要匹配、正确BSN/版本/DS实现的真实Java适配JAR，随可选agent包安装到siblings/management-agent/5s，并记录公开摘要；不向core/GWT增加sibling实现依赖，不自动修改网络、启用远程或重启Kura。9项源码导出/构件身份检查通过。真实新agent DEB含正确JAR、root归属/0644及原Rust二进制；Rust/Java业务源码未变，复用既有720通过/29忽略的本机默认回归，不重跑已结束session68768。

已按当前输入完成14个Java仓库、179个Maven项目的编译/package/install（仅写新隔离缓存，测试未执行，不能说179组测试通过）。联合源5021项旧基线/当前覆盖输入逐项核对，当前固定覆盖1813项，较G025联合源26项变化。首次构建暴露隔离导出遗漏被旧空Git索引忽略的已追踪资源，补齐58个源文件后重新验证全部5021输入；随后OT3001范围依赖缺新缓存仓库ID对应的已有元数据，将原缓存XML原字节复制到新ID并只续跑尾部6项目成功，未修改依赖版本或产品源码。14仓完整编译已完成，不把这些夹具错误算作Eclipse卸载缺陷。新缓存按需只读取旧24GiB缓存，未复制整份，当前约1GiB；共享API522保持不变。

当前core ARM64、networking及GWT DEB均已构建；结合本轮含适配器的agent DEB和上一轮同输入的中心/图形安装器DEB，形成6包配对清单 `/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/kura-distribution-cohort-v1/management-distribution-package-set.json`。core第一次打包因测试PATH漏/sbin而找不到macOS自带md5，修正本轮环境并保留部分档案后成功，未改产品脚本。打包检查首命令误选CLT Python3.9，使用现有Homebrew Python后9项通过；真实构件输入记录安装路径覆盖源路径的问题也仅修正夹具，未发生Docker或业务写入。

从实际core/networking/GWT/agent四包提取并逐字节核对311个普通文件，含229个JAR；保留必要安装软链接的建模说明，未执行维护脚本。core原gen_config_ini.sh在未提取agent时不含适配器，提取agent包后自动发现且仅发现一次，启动级别5并自启，无需手工放JAR。第一次检查误把合法Java properties转义@5\:start当作普通文本，现直接读取并正确解码已生成结果，没有重放提取操作。实际5个关键Java bundle均在包内，GWT的Gson stream导入修复保留。这只证明包内容和启动清单，尚未启动这套实际包的Kura、注册Java/Rust、执行升级/权限/全部API-provider运行矩阵，不记为正式安装或完整GUI交付通过。

当前薄源 `/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/source-checkpoint-identity-renewal-v62-distribution-wip.json` SHA256 `ebb5221b61cc7e526f43d2f1da04a05d79576ddca72e1706cec18dd2ac8ccd98`：2223源码+4夹具、较V51差量164、352生产/测试候选UNAPPLIED；仅较05bfa8dc更新2个已有打包/检查文件，沿用43份变化快照映射并只另存2份输入。agent图谱已更新6561节点/43745边、0解析失败；Cargo.lock及3类显式夹具仍不作图谱全覆盖声明。源码未写回原18仓，不提交推送，不全面清理。最新验证索引将为 `/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/kura-distribution-cohort-v1/verification-distribution-wip.json`；最后已验收组件仍G033/V61、FULL CODE seal V51，G034整体WIP，完整目标ACTIVE。周额度41%，普通额度允许，额外credits消耗0。

下一步先用实际DEB文件启动新的隔离Kura：核验不装agent可独立启动、安装agent但服务不可达时也不阻塞Kura、GWT菜单及Java适配自动加载；接service.pid/7类API-provider、实际Java/Rust注册及权限/正式安装升级。保持本轮包和179项目编译结果，不无故重建；不重放旧CSR/绑定/续期场景或原UNKNOWN。全部原remaining台账逐项保留：旧首次绑定原历史、legacy/半凭据/容量恢复、UserAdmin中途重启异常、GUI全平台/CA quorum-bulk/DB恢复/网络-retire-history/352 guarded apply/G021-G022/maintenance锁忙及2组新PLC4X适配。内部pending与仅3组外部deferred分开，旧VM不启动，旧startup.stop UNKNOWN不补结算。

<!-- G034 V62 ACTUAL KURA DISTRIBUTION HISTORICAL TAIL BELOW: retained verbatim -->

<!-- G034 V62 DEFAULT AGENT SUITE TERMINAL 2026-11-15 -->
## 最新终态：agent 本机默认整套回归720通过、0失败、29条件忽略

原session68768已经完成并关闭，cargo15673/test15677及父进程均退出；耗时2354.91秒。声明749项中720通过、29忽略，不称749项全部执行或通过。29项逐项清单与原因已保存，其中包含明确CLOSED/只读保留的旧场景，禁止重放；其余按具体当前源码/新夹具/平台边界核对，不能整批转为外部deferred，也不把IDEA基础启动专项或本轮续期验收替代这些不同的运行场景。未重复启动本轮测试。

当前Linux中心镜像、配套中心/图形安装器DEB、8项产物配对、6份协议实现输入一致性和非root中英文只读设置页检查均已通过；所有本轮拥有的构建、浏览器、容器及测试任务均终态，数据/源码/历史现场保留。源码仍05bfa8dc（2223+4、较V51差量164、352候选UNAPPLIED），其创建时的pending元数据作为历史保留，当前验证索引以linux-delivery-cohort-v1/verification-delivery-wip.json为准。

下一步直接推进完整当前Kura core/networking/GWT发行DEB与service.pid/7类API-provider实际运行及后续权限/legacy/半凭据/容量矩阵，不再收取已完成的session68768或重建已通过的同版中心/安装器。所有原remaining台账及2组新PLC4X功能适配保持；G034整体仍WIP、G033/V61为最后已验收组件、V51为最后FULL CODE seal，完整目标ACTIVE，不提交推送，不重放旧闭环/UNKNOWN，不全面清理。

<!-- G034 V62 DEFAULT AGENT SUITE HISTORICAL TAIL BELOW -->

<!-- G034 V62 CURRENT LINUX DELIVERY WIP 2026-11-15 -->
## 当前 WIP：Linux 中心、配套图形安装器及实际非 root 设置页已验证

在上一段真实续期/原UUID恢复验收之上，当前源05bfa8dc已完成新的ARM64 Linux中心正常Docker构建（113.49秒），199份独立中心源码较V61变化11个输入，当前React产物与此前已验收的前端逐字节相同。已构建中心DEB和配套图形安装器DEB，核对9个中心固定文件、18个安装工具固定文件、root归属、维护脚本及安装目录；安装器实际catalog绑定同一V62版本、源码清单、二进制及DEB摘要。未安装到原宿主或旧网关，不把构建与静态归档核对算作正式安装/升级生命周期验收。

已将中心DEB、图形安装器DEB、当前agent DEB及API/crypto/configuration/bridge/GWT五个Java产物组成8项明确配对清单 `/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/linux-delivery-cohort-v1/matched-artifact-cohort-v62.json`。GWT使用补齐Gson stream的3a232544产物，未误用较早9848c4f9候选；agent导出协议crate与中心有6份协议实现输入逐字节相同。只有两份原位可变的Java target产物另存只读副本，其余复用已存在的固定产物，不重复复制整套运行树。完整Kura core/networking/GWT发行DEB、所有API/provider运行矩阵仍pending，不能因这8项配对就宣布完整Linux发行组合已通过。

当前Linux正常镜像在全新数据卷启动后，实际主进程UID999（非root），健康页与React入口200；本机installation-status确认未配置，实际TLS指纹与本机私有安装凭据一致。真实Chrome已通过中英文设置表单、配对后的刷新恢复及页面检查；仅有认证pair请求，0 CSR/DB初始化/业务写请求。首次检查误用“中文”选项文本，在认证成功后超时；真实选项为简体中文/zh-CN，修正测试定位后通过，前次失败及两次已知认证均保留，不是初始化或续期重发。此项只补当前Linux发布产物的启动与只读UI证据，没有重放已闭环的首次设置、CSR签发或续期。

本轮三个新容器均已终止并保留镜像/容器/数据卷/日志：产物提取及配对打包退出0；未初始化的设置页由SIGINT停止，退出1，日志为已有明确分支“setup interrupted; preserve original records”，OOMKilled=false，不是内存耗尽，也不据此结算任何原UNKNOWN。当前agent 749项全套本机回归仍在原session68768、cargo15673/test15677运行；采样时进程占用约11.1MiB、峰值47.3MiB，正在处理有限时子进程/等待。只代表该采样瞬间，不作整机或完整性能结论。续接先恢复原进程及读取原result文件，不能因耗时长另起同批测试；整套尚未记通过。

源码仍 `/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/source-checkpoint-identity-renewal-v62-live-reviewed-wip.json` SHA256 `05bfa8dc6d8420eafe4b50dbafbc6c7de52014734c0f640a48de5910539e3d62`：2223源码+4夹具、较V51差量164、352生产/测试候选UNAPPLIED；本轮没有新增功能源码改动。原18仓、4985原输入、G02540及既有source pins/API522继续保留；不提交推送，不清理旧现场，原VM仍不启动，旧startup.stop UNKNOWN不重发/补结算。最后已验收组件G033/V61、最后FULL CODE seal V51，G034整体仍WIP。最新工作验证索引将为 `/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/linux-delivery-cohort-v1/verification-delivery-wip.json`。

下一步先收取原agent全回归终态，再完成当前完整Kura core/networking/GWT发行组合、service.pid及7类API/provider真实运行矩阵；继续处理旧首次绑定原历史、legacy/半凭据、权限/容量恢复和UserAdmin中途重启异常。完整下方remaining台账、352 guarded apply、GUI全平台/正式安装升级、CA quorum/bulk、DB恢复、网络/retire/history、G021/G022及maintenance锁忙均保留；新PLC4X 3提交/2组逻辑适配也仍内部pending，已过的当前Linux中心构建和只读设置页不再重复列为待执行。内部pending与仅3组外部deferred分开，完整目标ACTIVE。

<!-- G034 V62 CURRENT LINUX DELIVERY HISTORICAL TAIL BELOW: retained verbatim -->

<!-- G034 V62 ACTUAL INTEGRATION WIP CURRENT 2026-11-15 -->
## 当前 WIP：真实界面续期及原编号恢复已通过，完整交付继续

本段替代下方历史前缀中“实际 Java/Rust/GWT/HTTPS/PG/mTLS 尚未验证”的范围描述。新建隔离现场已实际完成：中心首次设置、中心本机生成服务端 CSR、外部测试 CA 签署后从界面上传、PG 初始化及正常 HTTPS 独立 CA 校验；ARM64 Linux Kura 的 GWT 生成网关 CSR、上传公开材料、中心登记、首次绑定 OFF、明确开启并建立真实 Rust mTLS。第二份 CSR 和公开证书在中心以原操作回执推进代际1→2，下载13字段公开回执，再由 GWT→Java→Rust 执行续期。

续期原 UUID b16963b9-ac62-4a97-b8d4-4953fec4403a 仅一次写请求，发送前已保存12字段公开引用；真实服务端200返回被测试拦截器丢弃后，页面显示 UNKNOWN 并互锁。刷新只读取同一原 UUID，恢复 APPLIED/代际2/提交时OFF；确认后仍OFF，另一次明确开启建立新证书 mTLS，30秒稳定，中心公开接口确认代际2、指纹、online及KuraAvailable。之后当前通道ON时，从服务器界面历史和真实 agent UDS 仍读回原APPLIED/OFF-at-commit，0续期/通道写请求，原GWT日志文件散列不变。没有以当前状态反推旧结果、自动重发或换号；这些是本轮新场景，原V52 UNKNOWN未触碰。

修复两项实际缺陷（仅shadow候选）：agent源码导出遗漏4份非Rust编译资源，现复制并校验全部常规源码文件、拒绝资源符号链接，6项针对性检查通过，完整独立ARM64 Linux release及DEB构建通过；GWT实际OSGi加载器缺少com.google.gson.stream导入，复现NoClassDefFoundError后补齐2.11版本约束，实际bundle加载器严格JSON解析成功。当前API/web2/web2.test合计353测试及12个GWT生产排列通过；37消费者/302产物的Equinox包解析通过。后者只证明包解析和目标解析器执行，不代表所有业务服务生命周期及7类API运行矩阵已通过。

当前源检查点 `/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/source-checkpoint-identity-renewal-v62-live-reviewed-wip.json` SHA256 `05bfa8dc6d8420eafe4b50dbafbc6c7de52014734c0f640a48de5910539e3d62`：2223源码+4夹具，较V51差量164，352生产/测试路径UNAPPLIED（134existing+221new共355，排除3个fixture-only）。43个变化文件使用增量只读快照，其余复用V61。前一5dae检查点保留，新检查点只校正验收范围元数据，源码字节未再变化。当前agent 749项本机回归正在原session68768串行执行，cargo15673/test15677；先读取本轮agent-full-tests-result.json或恢复该进程，不重复开跑，不提前记整套通过。当前结果索引将位于 `/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/live-integration-v1/verification-integrated-wip.json`。

验收范围是新隔离ARM64 Linux容器网关、GWT回环HTTP、原生macOS中心界面HTTPS/PG及真实Rust mTLS。浏览器bootstrap允许自签，正常中心证书另做严格公共CA校验，尚非操作系统/浏览器全信任矩阵；网关实际DEB内容已提取验证，尚非安装卸载/systemd/polkit/sandbox/不同UID正式验收。CA私钥仅在外部测试夹具，不上传中心或网关；同一宿主用户测试不证明生产密钥分权或quorum。中途旧框架缓存重启记录有Persisted identity view unavailable/read-only及网络REST角色注册异常，最终新框架代际未再出现，但其生命周期/权限问题仍内部pending，不能声称279个bundle全健康。

本轮自建中心、网关子进程及PG已停止，数据卷、容器、日志和各代凭据保留；网关容器退出137且OOMKilled=false，子进程先以-15/143停止，PG退出0。IDEA仍开着；Eclipse卸载专项已实际编译537 Java、7单元+8真实Equinox测试及278bundle启动通过，无需恢复Eclipse。原Ubuntu仍stopped，未启动原VM，不由停机推断旧startup.stop UNKNOWN，未重放旧闭环，不提交推送、不全面清理。周额度39%，普通额度允许，额外credits消耗0。

本轮官方上游分支头只读检查：10个fork未变，PLC4X从ea42d3db新增3提交到ac8ddb2a，10个路径，仅plc4net及共享SLMP mspec。按随机写/顺序保持和多块写模型分为2组，已读补丁，加入G022非Java/可选生成的内部pending；未应用、未推进已验收游标，也未宣称Java生成或.NET验收。7个非fork继续按既有来源/消费者关系审查跳过直接合并。首次错误使用不存在的upstream别名产生的失败记录保留，最终核对使用此前已确认的官方HTTPS URL，无fetch/ref/remote修改。

下一步先收取原agent回归终态，然后推进当前完整Linux中心/agent/GWT/图形安装器组合、所有API消费者和权限/legacy/半凭据/容量恢复矩阵，处理上述UserAdmin生命周期问题及2组新PLC4X逻辑适配。完整历史台账逐项保留：GUI全平台、中心地址/DB/CSR/原操作恢复、旧首次绑定/首次TLS UNKNOWN、CA多管理员/quorum/bulk、DB跨版本/非空加密任务及备份恢复、sandbox/polkit/protocol0在线升级、owner/ordinary/recovery/fullnetwork/boot/netns/handleRestore/CM-runtime-NAT、init-retire/子任务结算/history/readoption、14groups/7API2/R017/全部352 guarded apply、G021 LDAP、G022 Go vet/3个非Java POM/可选生成/.NET及并行maintenance锁忙。内部pending与原三组外部deferred（NM/cloud-init/物理、amd64、磁盘满/硬断电）分开。G034尚无整体组件绑定，最后已验收组件G033/V61、最后FULL CODE seal V51；完整目标保持ACTIVE。

<!-- G034 V62 ACTUAL INTEGRATION HISTORICAL TAIL BELOW: retained verbatim; superseded current claims above -->

<!-- G034 V62 GWT WIP CURRENT 2026-11-15 -->
## 当前 WIP：续期原操作日志及 GWT 上传、确认、恢复已实现

GWT 服务端新增独立原操作日志，先持久化原 UUID 与公开引用摘要、提交标记，再至多调用一次 agent 写接口。已知预检拒绝有独立原回执；提交后异常保持 UNKNOWN，只查原 UUID，不用当前指纹或通道状态结算，不自动重发、换号或补写旧结果。缺损记录阻止新的续期，历史只读恢复不修改旧日志；受控容量归档仍待实施。操作者来自实际管理员会话，保留权限及 XSRF。公开中心 JSON 严格允许字段并校验实际网关、端点、旧/新证书摘要及代际，拒绝重复键、越界数字、私钥和半份结果；它是未签名的人工核对数据，不是密码学授权证明。

界面支持公开回执文件上传、材料复核、明确确认、发送前保存公开操作编号及引用、原编号查询和服务器历史恢复。UNKNOWN 及不匹配回执保持锁定；浏览器存储不可用时不发送写请求，仍可只读查询服务器原记录；成功后仍 OFF，另行明确开启。修复了切换 CSR 后上传按钮未重新计算可用性的实际缺陷。新界面与原首次绑定、通道开关的互锁分开处理，读取和草稿校验不误锁旧入口。

当前 API、web2、web2.test Maven package 通过，分别 25、43、285 项测试，合计353项，0失败/错误/跳过；新增19项协议、日志与Servlet检查，含实际构建 JAR 内 GWT RPC policy 的 DTO/历史列表序列化验证。login 与 denali 各6个生产排列编译通过。中英文当前编译界面26项浏览器检查通过，覆盖文件上限/私钥拦截、发送前存储、原结果归属、响应中断、UNKNOWN、历史恢复及存储故障；RPC 对端为模拟，未验收真实 Java/Rust agent、浏览器/HTTPS/PG/mTLS 联合流程。早期浏览器失败及修复保留：一项为按钮刷新缺陷，一项为模拟对端在发送响应头前重置连接触发 Chrome 网络重发；最终响应体中断用例只恢复原编号，不重发应用写请求。函数注入的日志边界不等于真实掉电/磁盘满验收。

当前源码 `/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/source-checkpoint-identity-renewal-v62-gwt-wip.json` SHA256 `2eea447756f2fd95e19ed508fab0565c1a2bb828d02e699b12319bdaa0cffea7`：2223源码+4夹具、较V51差量162、349生产/测试候选UNAPPLIED（131 existing+221 new，共352，其中3个existing为fixture-only）。本段14个GWT变化输入，9新文件；累积40个变化文件复用前阶段只读快照。host GWT JAR只作未安装候选，共享 Maven API522不变，未更改原18仓功能源码、提交或推送。最后已验证组件仍G033/V61，最后FULL CODE seal仍V51；G034整体、完整联合代际、所有API消费者与正式Linux发行组合未完成。

下一步：新隔离现场进行实际 Java/Rust UDS、GWT浏览器、中心HTTPS/PG及续期后新身份mTLS验证，再做当前完整API消费者和发行组合。继续保留旧绑定原历史、legacy/半凭据、权限/跨版本provider、容量恢复、GUI全平台、CA quorum/bulk、DB恢复、网络/retire/history矩阵、14groups/7API2/R017/全部349 guarded apply、G021/G022及完整原台账；内部pending与原三组外部deferred分开。原Ubuntu当前仍stopped，旧startup.stop UNKNOWN不从停机推断，未启动旧VM或重放旧一次性场景。IDEA实际编译537 Java、7单元+8真实Equinox测试及278 bundle启动已通过，无Eclipse安装路径依赖；IDEA仍打开。周额度38%，普通额度允许，未使用额外credits；不做全面清理。

<!-- G034 V62 GWT WIP HISTORICAL TAIL BELOW -->

<!-- G034 V62 JAVA WIP CURRENT 2026-11-15 -->
## 当前 WIP：Java 可选续期 API / bridge 已接通，GWT 仍待实施

当前 API 增加默认可选 getIdentityRenewalStatus(operationId) / renewIdentity(...)，旧provider默认明确UNSUPPORTED；null只读新操作资格，非null只读原UUID结果。networking bridge固定一次请求、仅公开标量、严格投影及结果归属校验；超时/断线或不匹配写回执为UNKNOWN，不自动查询当前状态、重发或更换UUID。成功回执必须OFF、旧/新代际恰加1且摘要/原请求/操作者匹配；准入拒绝不伪造已提交OFF状态。旧首次绑定与开关接口保留。

API/core crypto/configuration/bridge候选Maven package通过，当前bridge全量148项测试通过，含新增9项投影/单次写/默认provider合同，以及2项真实Java UDS注册连接检查（正常原UUID读回、收到请求后断线UNKNOWN）。UDS对端为合成Java peer，并非实际Rust agent或新身份mTLS全流程；没有把mock与真实连接合并作完整端到端验收。2个host Java bundle只作未安装候选，共享API522保持。原18仓Git/源码、运行目录及旧场景不改、不提交推送；本段只追加审查与台账前缀。

源码 `/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/source-checkpoint-identity-renewal-v62-java-wip.json` SHA256 `ad6efd3d2b15a581429d8053607fdf5e4e89191a7b5df3a4f3017787ee0b4076`：2214源码+4夹具，较V51差量151，338生产/测试候选UNAPPLIED（129existing+212new含3fixture-only）。较上个中心WIP有6个Java输入变化，其中2新文件；其余复用既有只读快照。台账数量由实时清单计算，API和adapter原先不在未应用候选中，不能只按新文件数推算。最后已验证组件仍G033/V61，FULL CODE seal仍V51，无G034组件完成或正式发行声明。

下一步优先补GWT服务端原操作日志：浏览器保存原UUID后，服务器的已知预检拒绝也须有独立可读原回执；写入已投递标记后UNKNOWN只按该UUID查询agent，禁止从当前指纹/通道状态结算，也不能靠刷新或新UUID解锁。原回执文件严格允许公开字段、拒绝重复JSON键/私钥和越界值；核对实际绑定、CSR暂存摘要、旧/新指纹及代际，操作者来自真实管理员会话。回执是未签名的人工核对数据，不能声称中心签名授权。然后接GWT上传/确认/恢复界面、实际新隔离Java/Rust UDS/浏览器/HTTPS/PG/mTLS和当前完整发行组合。所有API消费者/跨版本provider及完整联合代际仍待验证。

完整remaining台账继续保留，包括GUI全平台交付、中心/DB/CSR续期、CA quorum/bulk、DB非空加密任务/恢复、sandbox/polkit/protocol0、owner/网络/retire/history矩阵、14groups/7API2/R017/338 guarded apply、G021/G022等。内部pending与原三组外部deferred分开。原Ubuntu仍停机，旧启动stop UNKNOWN不从停机推断；本轮不启动或清理原现场。

<!-- G034 V62 JAVA WIP HISTORICAL TAIL BELOW -->

<!-- G034 V62 CENTER WIP CURRENT 2026-11-15 -->
## G034 / V62 当前 WIP：续期后端与中心公开回执

在已通过77项agent选定回归的后端之上，中心已增加显式代际续期合同：新网页提交已核对的旧代际，数据库在同一事务提交新指纹、代际加1、连接重置和原操作APPLIED回执。旧回执和省略代际的旧轮换工具保持v1、不改代际、不补填证据；旧工具兼容回归曾复现失败，修复后通过。中心默认公开证书模式已提供“下载公开证书更新回执”按钮，JSON只取公开允许字段，缺代际/不匹配/UNKNOWN不能生成可用于网页续期的v2回执；私密包下载仍保持旧合同，不把旧CLI指令套用于新代际流程。

本次中心8项真实HTTPS/PG测试通过，含实际V25→V26非空旧原回执迁移、代际冲突/预约后代际变化/耗尽、连续轮换后旧回执不变、旧工具兼容。中心115+协议24项串行回归通过，48+3 ignored明确保留；先前并行maintenance锁忙仍内部pending。中英文浏览器30项通过，含公开文件下载与断线原UUID恢复；API为mock，未合并为实际浏览器+HTTPS/PG或新身份mTLS验收。一次浏览器检查因loading图标改变可访问名称失败，已固定按钮aria-label并在当前版本30项全通过。当前生产前端构建单独保存，未部署新中心/agent/Linux组合包。

当前源码 `/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/source-checkpoint-identity-renewal-v62-center-wip.json` SHA256 `c932f6bb2fdce2bc8f4d65693665407ab3f624e6755f058853028bd47892a8f0`：2212源码+4夹具、较V51差量145、334生产/测试候选UNAPPLIED；127existing+210new含3fixture-only。20个变化输入（agent9+中心11），仅变化文件另存，其余复用V61既有只读快照。最后已验证完整组件仍G033/V61，最后FULL CODE seal V51；该WIP不是组件完成或正式交付。

下一步优先Java公共API默认可选方法及networking bridge的单次请求/严格结果投影，再接GWT服务端真实操作者和源授权、公开中心回执文件上传校验、明确续期确认、发送前保存UUID和原操作只读恢复。公开回执是操作者核对数据，未签名，不构成密码学中心授权证明；中心身份/地址映射及权限矩阵仍须在服务端结合实际绑定检查。旧首次绑定历史读取、legacy/半凭据、UNKNOWN显式恢复、容量归档与新现场实际Java/UDS/浏览器/mTLS、当前完整发行组合均未完成。完整remaining台账、14groups/7API2/R017/334 guarded apply等不缩减，内部pending与原三组外部deferred继续分开。原Ubuntu VM实时仍stopped，旧PIDs只作历史，不重启、不从停机结算旧startup.stop UNKNOWN；Windows 11为用户已有running，未触碰。

<!-- G034 V62 CENTER WIP HISTORICAL TAIL BELOW -->

<!-- G034 V62 WIP CURRENT 2026-11-15 -->
## G034 / V62 WIP 当前实施进展

已绑定agent身份续期后端已有候选：独立原操作日志、旧凭据链、关闭状态原子切换、过期历史身份续期、明确拒绝回执和UNKNOWN恢复保护；身份22+通道13+本地协议42项通过。Java/GWT入口、中心代际回执与公开文件流程、实际端到端/mTLS和当前完整发行仍未完成；不能将此WIP视为已交付续期。当前只编译验证Mac工作副本，未安装新agent。

源码 `/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/source-checkpoint-identity-renewal-v62-wip.json` SHA256 `8ef267aa86552043667062d26202040ff1f69cd282df7bcb3c4580c477440679`，2211+4，较V51差量144，333候选UNAPPLIED。9输入变化，4新文件；变化文件另存，其余复用V61只读快照。最后已验证组件G033/V61，最后FULL CODE seal V51；原V52运行仅历史、Ubuntu VM仍stopped。本轮不重启原VM，不重发旧UNKNOWN。审查 `/Users/heyoulin/iot-kura-develop/git/kura-management-console/docs/reviews/agent-identity-renewal-wip-20261115.md`。完整目标active，内部pending和原三组外部deferred不混合；下方历史台账全文保留。

<!-- G034 V62 WIP HISTORICAL TAIL BELOW -->

<!-- G033 V61 CURRENT 2026-11-15 -->
## G033 / V61 当前实施进展

网关登记证书轮换新增原操作公开持久回执、默认公开证书模式及浏览器原 UUID 恢复查询；不再用当前指纹推断旧结果。缺回执、登记归属不一致、未完成意图和不确定写任务拦截后续凭据写入；迟到写任务对旧客户端也返回 409。新归属不一致缺口已先复现再修复。串行中心 114 + 协议 24 + 新隔离 HTTPS/PG 5 项通过；模拟 API 的 React 契约 24 + 最终文案视觉 2 项通过。ARM64 release 和配对候选 DEB 在 `/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/gateway-certificate-receipts-v1/package-v2/`，未部署；不是完整平台或实际 agent 续期验收。

源码 `/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/gateway-certificate-receipts-v1/source-checkpoint-gateway-certificate-receipts-v61.json` SHA256 `60eb3b6302497db8d4f14f177521a84a3db3388e1c478f1da99abe8203c86dcb`，2207+4，较 V51 差量135，325候选 UNAPPLIED；最后 FULL CODE seal V51。agent/GWT 保持 V60；已绑定身份续期仍待实施。审查 `/Users/heyoulin/iot-kura-develop/git/kura-management-console/docs/reviews/gateway-certificate-receipts-review-20261114.md`。并行 maintenance 锁忙仍内部 pending。旧 Ubuntu VM 现 stopped，旧 boot/5 PID 仅历史，本轮未重启或重发旧 UNKNOWN；不再标注为当前运行。

IDEA 卸载 Eclipse 后编译537源文件、JUnit7+Equinox8测试及隔离 Kura HTTP启动通过。18仓和个人配置保留、不提交推送。完整目标 active，内部 pending 与外部 deferred 分开；外部仍仅原三组。旧台账全文保留如下。

<!-- G033 V61 HISTORICAL TAIL BELOW -->

<!-- G032 V60 CURRENT 2026-11-14 -->
## G032 / V60 当前实施进展

原 GWT 公开 CSR 下载与三个公开证书文件选择入口已完成候选实现；文件仅填写本地草稿，仍需明确暂存/绑定/开启通道。完整 GWT 正常发布编译通过，中英文真实编译 widget/Chrome 文件契约 38 项通过；不是实际 OSGi/agent 绑定验收。新 UI JAR 尚未安装。中心 Rust/React 保持 G031，无需为本次 UI 变更重建其 Linux 产物。

当前源码 `/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-csr-files-ui-v1/source-checkpoint-agent-csr-files-v60.json`，SHA256 `6b91be7275c6c5a13dd404deec11fdaf976fb2a753062449b5dcf0d2368f41f4`；2203+4，较 V51 差量 130，320 候选 UNAPPLIED；最后 FULL CODE seal V51，原运行 V52。组件 `/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-csr-files-ui-v1/final-binding.json`；审查 `/Users/heyoulin/iot-kura-develop/git/kura-management-console/docs/reviews/agent-csr-files-ui-review-20261114.md`。目标 active，整体验收/正式交付/证书续期仍 OPEN。原 18 仓、VM boot/5 PID、startup.stop UNKNOWN 保留；未提交推送，无 UNKNOWN 重发，无旧 CLOSED 场景重放。完整旧台账在下方保留，不减少内部 pending；外部 deferred 仍仅原三组。

<!-- G032 V60 HISTORICAL TAIL BELOW -->

## 2026-11-14 G031 / V59：证书上传原回执和页面恢复完成，完整改造继续

新证书上传在发送前保存独立UUID与公开PEM摘要，服务端持久保存接受/拒绝及精确绑定的签回业务回执。刷新或查询只GET原上传；匹配的拒绝回执允许用户更正证书，成功回执恢复成功显示，不重发。缺失/伪造回执、注册历史缺失、旧IMPORT没有上传编号时保留UNKNOWN，当前文件或SIGNED不能补造原结果；新编号、旧导入、停用和TLS生效不能绕过。私钥和证书正文不进入本地跟踪，旧Signed序列化字节及旧API导入格式保留。

114项Rust默认测试通过（43项默认ignored另行保留），26项中英文setup/settings CSR浏览器契约通过；1个新建独立PG的真实HTTPS接口测试通过，未登录/非管理员/CSRF/跨Origin拒绝。上传响应正文未读取后，只查询原编号核定拒绝和签回；缺失注入后原上传保持UNKNOWN，无TLS生效、无首次初始化。真实接口在Darwin ARM64原生测试程序验证，浏览器为公开API契约，两段不冒称真实浏览器端到端/Linux全平台验收。旧红例等待问题、旧DOM断言错误和修正保留。宿主类型检查/生产前端及完整多阶段Linux ARM64普通release构建通过，两个前端3文件字节相同。

候选核心和工具均V59-1，核心20be6707、工具aa6b917d、普通Linux二进制b4c68246；194构建源manifest cae137d6，9核心+18工具归档pin/root owner精确，工具批准同一核心源码及摘要。未安装或启动候选中心；31个host packaging输入未变，109项Linux Python历史证据保留，不将已变CSR契约说成旧47浏览器输入全未变。3个新PG/提取/打包容器均exit0/noOOM，数据库卷、HTTP现场和故意UNKNOWN保留，监听已关闭。此前已运行的BuildKit按原状态保留。

当前source-checkpoint-csr-import-receipts-v59.json SHA d356b5dded3e928e61f9ab9cf8ff78cb5870cb7ecdc69c28a3e5a554379001a9：**2202source+4fixture、距V51为126输入增量、318 production/test路径整体UNAPPLIED**。本批13改+2增共15输入；注册现场仍V52，最后完整CODE seal仍V51。审查docs/reviews/console-csr-import-receipts-review-20261114.md，组件console-csr-import-receipts-v1。原18仓/G02540路径/API522与原现场保全，不提交推送、局部apply、UNKNOWN重发或CLOSED重放。

下一步中心地址/DB/CSR暂存和续期异常核定、agent独立身份/CSR/续期/通道能力及正式交付继续；下方完整owner/网络/退役、多人外部CA custody quorum/bulk、DB/备份、sandbox/polkit/协议0、14组/7API2/R017/318路径guarded apply/正式API与joint generations、G021/G022等不缩减。仅原3组external deferred，目标ACTIVE。

## 2026-11-14 G030 / V58：图形启用、公开证书和认证后管理员入口已验证

本机菜单改为普通用户 GTK 窗口；明确授权后仅启用 Cockpit socket，公开证书可界面核对和保存，私钥不导出。109 项 Linux root 测试通过，最终工具 V58-4 通过实际 GDebi/polkit 安装，18 个 root 负载 pin 精确；旧14/15文件及 headless 兼容。唯一启用请求 d0dcf1e5 已有成功命令回执，v2/v3/v4 工具升级和会话刷新后原 registry/请求/证书字节精确保留，没有再次启用。核心安装/启动/配对/初始化均0。

默认 Epiphany 到 TLS gate；另外以真实宿主 Chrome 私有 profile，经浏览器可见证书详情核对主机公开 DER pin 71c8c584 后进行本 profile 单主机信任，PAM 登录和管理员密码授权实际成功，页面显示“管理员模式”。一次只读查询正确显示核心尚未安装及核定 V56-3。没有 TLS-ignore/全局 CA 导入。首次管理员认证曾因缺 dbus-user-session 在应用命令前明确失败；最终包补齐依赖并提示重新登录，新用户管理器真实创建 bus 后通道恢复。原失败记录保留，约650 MB且无OOM，不能归因为内存不足。监督者仅刷新本轮测试桌面登录；旧 session46 closing/X99 残留不算正式OS生命周期通过。最初保存对话框不可写问题已修复，实际公共PEM保存成功，无PRIVATE KEY。

当前 source-checkpoint-host-trust-v58.json SHA 65c7a96c658a912efd87c19a93bbfcf18f528864ed1345a77913d2389f2e18c9：**2200source+4fixture、距V51为124输入增量、316 production/test路径整体UNAPPLIED**。本批5改+3增，仅安装工具/测试/说明；145个明确列出的更宽Rust/React/migrations/OpenAPI输入精确，保留旧135口径历史及47项未变浏览器契约。注册现场仍V52、最后完整CODE seal仍V51。9个新Docker对象已停止保留，8个退出0、桌面130/noOOM，私有Chrome正常关闭；原owned OrbStack仍stopped。G029 GUI reboot UNKNOWN及原startup.stop UNKNOWN分开保留，不重发。用户清理过的旧中心/PG/60容器不重建。

审查 docs/reviews/console-host-bootstrap-review-20261114.md；当前组件 console-host-trust-v1。**正式原生OS全GUI交付/sandbox和完整改造仍内部pending**，目标ACTIVE，不缩减下方完整台账。下一步继续正式交付、中心地址/DB/CSR异常核定/续期和agent身份/CSR/续期/通道能力；多人外部CA custody quorum、DB/备份、owner/网络/退役、14组/7API2/R017/316路径guarded apply/正式API与joint generations、G021/G022等保留。只有原3组external deferred。18仓不提交推送，不重放CLOSED现场。

## 2026-11-14 G029 / V57：系统图形安装和菜单启动已验证，完整入口仍 pending

默认工具包新增受 root provenance 保护的中英文应用菜单入口，精简为 Cockpit 网页必要组件和 xdg-utils，并补齐 Installed-Size。完整 Linux root **90 项通过**；实际 GDebi/polkit 管理员一次授权安装 V57-2、GUI 成功提示、dpkg 唯一安装和 15 个负载摘要/权限相符。菜单真实启动普通用户默认 Epiphany 至固定 HTTPS URI，不打开终端；未安装核心、未配对或初始化。早期未认证的两个请求经系统明确拒绝后结束，旧误判与更正保留。

**仍不能宣布空系统全 GUI 部署通过。** 测试容器 stock policy-rc.d=101 保留；一次原生 GUI reboot 授权期间超时，保持独立 UNKNOWN，不重发、不据 PID/状态推断未派发。监督者仅为独立菜单验证准备 Cockpit socket；浏览器停在主机 TLS 信任门，公开 peer pin 与 root 保护的主机证书相符，但没有关闭安全检查或提交浏览器凭据。GUI 主机证书信任、认证后入口和完整原生系统部署为内部 pending。5 个本批 Docker 对象已停止、数据保留，桌面退出130/noOOM、restart=no，不算正式 OS shutdown 或原 reboot 核定。

当前 source-checkpoint-os-entry-v57.json SHA 806b289099d85e35f2252cc1e98c149df183a25b505e7370afcb3cd209fc4a8f：**2197source+4fixture、距 V51 为121输入增量、313 production/test 路径整体 UNAPPLIED**，本批仅改4输入+增1桌面文件。注册现场仍V52，最后完整CODE seal仍V51，G025原仓40路径保留。审查 docs/reviews/console-os-entry-review-20261114.md；完整 owner/网络/退役/API/DB/agent/多人CA/发行等下方台账不缩减。仅原3组 external deferred。18仓不提交推送，原 startup.stop UNKNOWN 保留，旧 CLOSED 验收不重放，目标 ACTIVE。

## 2026-11-14 G028 / V56：空系统界面安装至设置页通过，OS入口及正式交付继续

本机页先核对软件再允许启动，修复dpkg占位误判、精简系统省略文档导致的校验不兼容，以及启动后旧停服提示。88项Linux root和47项浏览器契约通过；独立空系统真实PAM/sudo完成唯一软件安装、显式start一次、凭据下载、发布叶证书核对和只读打开设置页，无配对/初始化/TLS重放。旧候选缺文档仅补齐原文件后用同一成功命令回执核定，不重装；失败与人工修复边界保留。测试OS初始工具包仍由监督者APT安装，不算OS入口全GUI或正式sandbox，完整业务配置与正式交付继续。

当前source-checkpoint-empty-host-v56.json：2196source+4fixture、距V51为120输入增量、312production/test路径整体UNAPPLIED；原注册现场仍V52，G025原仓40路径保留。新工具V56-4独立更新不停止已有服务，批准实际核心V56-3。审查docs/reviews/console-empty-host-review-20261114.md；新任务终态，夹具/失败/数据保留，18仓不提交推送，UNKNOWN不重发。下方完整内部台账及3组外部deferred保留，目标ACTIVE。

## 2026-11-14 G027 / V55：真实界面安装及升级链路通过，整体改造继续

已完成批准 DEB 的界面选择、摘要校验、上传、全实例显式冻结/停服、唯一安装、成功回执及刷新恢复；默认工具包依赖 Cockpit。83 项 Linux root 和 14 项浏览器契约通过，真实 Cockpit/PAM 在隔离 ARM64 测试机安装 V55-2，三中心保持停止，原 53 个私有文件精确，历史 UNKNOWN 保留。只新增本批独立验收，未重放已闭环首次设置/TLS/GWT/V36v1。

当前权威为 console-installer-delivery-v1/source-checkpoint-native-delivery-v55.json：2196source+4fixture，120 输入增量，312 production/test 路径仍整体 UNAPPLIED；原注册现场仍 V52，最后完整 CODE seal 仍 V51。G025 原仓 40 路径保留，无提交推送。审查详见 docs/reviews/console-native-delivery-review-20261114.md；目标 ACTIVE，空主机完整 GUI bootstrap、正式交付及下方完整内部台账继续，仍仅三组外部 deferred。

## 2026-11-14 G026 / V54：中心 CSR 界面与外签上传已实现，正式交付和完整改造继续 ACTIVE

同一 R013 shadow 本批修改13已有源路径、新增10路径，共23；当前 **2191source+4fixture**，相对最后完整 CODE V51 为115输入增量。候选 **121 existing+189 new=310记录，含3fixture-only，所以307 production/test路径整体UNAPPLIED**。这是路径计数，不是剩余任务数量。当前源码检查点 `target/nm-role-authority-20261112/registered-runtime-v52/console-installer-bootstrap-v1/source-checkpoint-server-csr-v54.json` SHA `8c1c1c718990002f89fc4f9b07d29500d3583cf882d58bef6993452f1f4f12fb`；限定 CSR 组件 `final-binding.json` SHA `26730f1433c0d378319a2624091070fe34f67005f0c95b29937662672e8ba754`。V54为源码及独立应用/界面检查点，注册现场仍V52、最后完整CODE seal仍V51。

首次设置配对后及管理员证书设置页，默认点击生成P-256服务器私钥和CSR、下载公开CSR、外部CA签发后上传服务器证书；服务器私钥不进入HTTP响应/DOM/浏览器存储，CA私钥不上传、不放进中心VM挂载。保留已有服务器证书/私钥导入选项。签回校验密钥匹配、DNS/IPv4/IPv6、有效期、serverAuth和非CA约束。明确验证拒绝发生在导入intent之前，可更正同一CSR的证书；缺响应/缺回执则保留原编号UNKNOWN，只查询，404不清本地编号。新编号不能绕过未结CSR。只对已确认且未使用的CSR提供保留历史的停用，已安装/已暂存/UNKNOWN不能停用。初装旧NONE输入的序列化字节与HMAC不变；初装消费仅由原APPLIED回执及精确证书/密钥pin只读推导，缺初装回执不冒称已消费。

签回和暂存不改变当前TLS，仍走既有联合TLS显式确认；缺CSR暂存回执时，旧上传入口及activation也被阻断。服务生成专用OpenSSL配置，明确named-curve/PKCS8，修复本轮实测LibreSSL空配置和EC编码兼容失败；早期失败日志保留。

**103单元通过/39独立集成测试默认ignored，另在最终V54源码上1个新建私有PG+真实HTTPS CSR集成通过（20项未登录/非管理员/CSRF/跨Origin拒绝）；13中英文浏览器契约通过，typecheck/生产前端及Darwin ARM64普通非test debug构建通过。** 浏览器契约使用公开API夹具，不冒称当前Linux包或真实生产CA审批验收。首次HTTP夹具的故意UNKNOWN仍原字节保留；最终源码变更后使用不同schema/应用目录的新夹具验证，没有重试旧UNKNOWN。仅本批私有PG已正常停止，数据库、两个HTTP夹具和失败记录保留。当前普通Darwin可执行SHA `7dee2515932110213d3eb1827809d0a7d16d03dfe2b081a514917cacb7de8dbb`。V54 Linux/容器镜像、正式DEB、完整sandbox/polkit和端到端交付未认证，不以旧V52产物充当前源码通过。

18原仓HEAD/branch/index/staged/refs/remotes及原工作树继续按本批preservation核对；G025的40原仓功能改动、580上游pin/89absent和9个shadow回补均保留。G025/G024/G023/V52Docker/完整V51旧seal共26924个绑定文件只读核验精确。共享API522未改/安装。原767863a9仍唯一startup.stop UNKNOWN/intent a87d94d5，原boot4db3、2799/3635/81713/82137/59158及agentaffa64e2精确，无stop receipt/publication/start/complete/activate/cancel/re-export；owned OrbStack仍stopped。用户已确认清理的旧中心/PG/60容器不重建，旧首次设置/TLS/GWT3保存/V36v1不重放。无提交、推送、reset、clean。

**下一实际实施仍优先installer/bootstrap界面与正式交付**，再按完整台账推进中心地址/DB和证书异常核定、agent独立身份/CSR续期/通道能力、凭据部分恢复、多人外部CA custody quorum/bulk、跨版本DB/非空加密任务/备份恢复、正式sandbox/polkit/协议0升级、完整owner/ordinary/recovery/原forward-confirm-inverse-observation-stopped-expiry、合法boot/netns/handle恢复/全CM-runtime-network-NAT矩阵、初始化先退役再原inverse/子结算退役历史、14组/7API2/R017/307整体guarded apply/正式API与协议校验/joint generations、G021 LDAP、G022 Go31vet/3非JavaPOM/regen/.NET。上述均内部pending；只有原NM-cloud-init-physical、amd64、disk-full-hard-power三组external deferred。未因CSR组件通过缩减总目标。周额度26%已用、ordinary allowed，credits54476.099936及2reset未使用，目标保持ACTIVE。

## 2026-11-14 G025 / V53：SSL 与 Crypto 按功能协同完成源码集成，整体改造继续

源码组件：`target/upstream-logical-sync-20261112/ssl-crypto-joint-cohort-v21/final-binding.json`，SHA `cb5e1924a67d4d545b9fca32775a346890f1f024d68eff447d6a9ca7270c4a54`。本组件只封存G025源码协调结果，完整改造仍ACTIVE。

本批对上游 core 6e1b1380/7eca88b2、deployment 0c32ff7b、triton 9fc62c85、wires 836e974e 做完整接口、实现及消费者适配，向 11 个原仓写入 39 个已有文件和 1 个新测试，共 40 路径；没有 merge/commit/push。旧 String 加密及文件式 SSL 接口移除，Crypto 包 2.0、SSL 包 3.0、core.ssl 包 2.0 配对。char[]、流、现有密文/密钥格式、YOFC 空服务证书库回退及异常原因保留。指定托管别名现在限制实际使用的凭据，同时遵守证书库的别名大小写规则，不回退到其他凭据。

隔离普通源码版本完成 406 项单元测试；与全部 R013 管理功能组合后完成 3,108 项，均 0失败/0错误/0skip。每个版本有 13 个真实双向 TLS 场景，覆盖 Socket、SSLEngine、TLS1.2/1.3、大小写别名及缺失别名拒绝。Keystore 的真实增删、缓存、CRL、密钥及错误回归保留。没有修改 CA 托管边界或生成现场新密钥。

两个版本分别完成 179 项 Maven reactor 的生产及测试编译、GWT 生产策略和 JAR；GWT 自有 class 为 Java11/major55，核心 major65。13 个发行元数据/DEB 模块因隔离 Git 为 unborn 无法取得 HEAD，发行包不算通过；后续编译明确排除这 13 项。普通 test 生命周期不能给现有 GWT 策略测试提供生产 JAR，File construction mock 也干扰目录 classpath；按 package 生命周期运行后全部通过，没有降低断言或屏蔽测试。失败记录保留。

真实 Equinox 仅 init、不启动业务 bundle：普通版本 36 个、联合版本 37 个实际生产消费者均解析；包括 NM、Triton、恢复工具、GWT 和 YOFC 应用。两种服务实现与实际 API 类身份一致，旧导入范围负例被拒绝，无 mock provider。首版探针启动 framework 后触发依赖激活的失败记录保留，接受版本固定独立 storage、instance、user/install area 且业务启动数为 0。这只证明包绑定及实现接口一致，不是实际服务生命周期、系统网络或整机验收。

完整旧 2181source+4fixture 和旧工作候选归档于 accepted-shadow-before-g025。仅将已验证的 9 个 API/元数据源改动同步到 SAME 工作 shadow；新 source-checkpoint-after-ssl-crypto-v53.json 仍 2181+4，相对最后完整 CODE V51 为 105 输入增量。当前 121 existing+179 new=300 候选记录，含 3 fixture-only，故 **297 production/test 路径整体 UNAPPLIED**。减少的一处是已由本批原仓集成覆盖的 core.crypto MANIFEST，不是一个业务任务完成。旧 V51/V52/G023/G024 seal 全部原字节保留；注册运行现场仍 V52，新 V53 仅为源码协调检查点，未宣布全量 CODE seal 或发行。

本次只读核对的 11 fork tip 均无新增，5 个待处理提交已按功能集成，完整 reviewed 游标可推进到本次已公布 tip；7 个 nonfork 仍根据来源和供应/消费者关系处理，不直接 merge。当前上游 source pins 为 580：旧 546 中 540 原字节保留、6 声明的版本文件由本批替代，新增 34；89 退役路径不复活。1986 原 management 输入与 20 先前管理 pin 的过渡单列，不强行声称旧 SHA 仍当前。共享 API522未安装或改动，18仓 HEAD/branch/index/staged/refs/remotes 不变。Git 续接 authority 使用本组件的 preservation-after-docs.json。

原 767863a9 唯一 startup.stop UNKNOWN 与 a87d94d5 intent 不变；仍无 stop receipt/publication/start/complete/activate/cancel/re-export。原 boot、五 PID 和 agent affa64e2 精确，OrbStack 同一 owned machine 保持 stopped。已闭环首次设置/TLS/GWT 三保存/V36v1 不重放。无提交、推送、reset、clean。周用量24%，ordinary allowed，credits与2次reset未使用。

下一实际实施回到 installer/bootstrap 界面及正式交付、中心 CSR 下载/外签上传与地址/DB、agent 独立身份/CSR/续期/通道能力、凭据部分恢复/UNKNOWN只读核定、多管理员外部 CA custody quorum。下方完整台账及 remaining-implementation-v52.md 保留：跨版本 DB/非空加密任务/备份恢复、正式 sandbox/polkit/协议0升级、完整 owner/ordinary/recovery/原 forward-confirm-inverse-observation-stopped-expiry、合法 boot/netns/handle restore/全网络 CM-runtime-NAT 矩阵、初始化先退役再原 inverse/子结算退役历史、14组/7API2/R017/297整体 guarded apply/正式 API/协议校验与 joint generations、G021 LDAP、G022 Go31 vet/3非JavaPOM/regen/.NET 都是内部 pending。只有 NM-cloud-init-physical、amd64、disk-full-hard-power 三组外部 deferred，目标保持 ACTIVE。

## 2026-11-14 最新续接：G024 部署文件传输限额及并发修复已按功能写入原仓；完整改造 ACTIVE

本块补充已通过的 Docker 界面部署/维护/升级/恢复/冷启动结果。管理体系仍是2181source+4fixture、298候选整体UNAPPLIED，source297bbace/组件ac11ff33/组合09da819f及最后完整CODE V51不变。

本批只对原 `kura-deployment` 按功能适配上游a859df58：**17已有+10新增，共27路径**，115输入精确。默认/无效25MiB，负数unlimited、零拒绝；远程下载、DEPLOY-V2、REST上传同时检查声明长度和实际流，REST包含multipart封装；超限/失败清理临时文件，本地已有包在安装失败时保留。并发名额覆盖接收至异步安装，正确URL完成/结束/排队失败才释放，其他事件和旧请求不释放新名额。HTTPS先配置factory再握手、URI、虚拟线程和当前SSL/Crypto配对保留，不增加独立management-agent启动依赖。

针对fork Jersey3.1.9缺file时nullable EntityPart注入NPE，HTTP入口改用标准List<EntityPart>，没有引入生产Jersey类型；测试接入真实RestExceptionMapper后分块超限正确413，缺文件400。另修复实际DeploymentAgent队列明确RejectedExecutionException时预留URL不释放的问题；只对明确拒绝释放，其他不确定异常不作未排队推断。既有回归保留，新增重定向后限额、超限不进入安装器、清理/并发等场景。

**Maven3.10/JDK21/private cache离线package：core121+agent47+REST54=222，0失败/0skip，新增51。** 3个JAR的49生产class、3个DS描述符及52依赖JAR精确。第一218版、队列219版、最终222版、失败/rejects/原105及最终115source snapshot均保留。27是最终不同路径数，不是26+3+2，也不是剩余任务数；所有原应用和补充回执保留。未构建正式deb或部署/重启原运行时，完整provider/发行矩阵继续内部pending。

当前功能组件 `target/upstream-logical-sync-20261112/ssl-crypto-functional-review-v20/final-binding.json` SHA `d9390303b7a61b189e11738f3650cc61adff50b350cbc62c2ad69bdf0f751db8`；当前逐文件上游pin为 `applied-source-pins-after-transfer-limits-v20-reviewed.json`（**546，旧519仍精确+本批27**）。旧G023 functional seal7d04f7ee不变。11fork tip在本批收尾再次只读确认无新增，7nonfork按来源/消费者关联分析跳过直接merge。

仍有**5个SSL/Crypto跨仓库破坏性提交内部pending**：core6e1b1380/7eca88b2、deployment0c32ff7b、triton9fc62c85、wires836e974e。不只抬导入/导出范围。a859功能已合入，其parent0c32未合入，deployment完整review游标仍7c31，coref0/UI1c/PLCea42不动，逐commit功能disposition另记，不吞ancestor。

**续接原仓Git基线必须使用本批 `ssl-crypto-functional-review-v20/preservation-after-docs.json`**，它保留17其他仓完整状态、deployment仅显式27路径，18index/refs/remotes/HEAD/branch不变；不能把旧V51/上批18仓指纹当作本批27路径未发生。1986旧原输入/20management pins/旧519/新增27/89absent、credentials141+5/API522精确。R013 shadow没有deployment源输入重叠，298候选未局部apply，共享API未安装，无提交/推送/reset/clean。

下一完整审查上述SSL/Crypto消费者/provider/OSGi/API协同后再决定移除旧API，同时继续installer bootstrap、中心CSR外签上传/地址/DB、agent身份/CSR续期/通道能力、多管理员外部CA quorum、bulk/备份恢复/正式sandbox/polkit。完整owner/ordinary/recovery/原forward-confirm-inverse-observation-stopped-expiry、合法boot/netns/handle restore/全网络CM-runtime-NAT矩阵、初始化先退役再原inverse/子结算退役历史、14组/7API2/R017/298整体guarded apply/正式API/协议校验和joint generations、G021LDAP、G022Go31vet/3非JavaPOM/regen/.NET均保留，下方完整台账不缩减。

原767863a9唯一startup.stop UNKNOWN/intent a87d94d5、原5PID/boot/agent仍精确，无stop receipt/publication/start/complete/activate/cancel/re-export。原现场不动；owned OrbStack仍stopped、旧Docker/PG/fixtures不重建，CA外部保管边界不变。CLOSED/GWT3保存/V36v1/旧首次TLS不重放。仅原NM/cloud-init/physical、amd64、disk-full/hard-power三组external deferred，其余internal pending。周23%已用/ordinary allowed，credits54476.099936与2reset未用，目标ACTIVE。

## 2026-11-14 最新续接：已配置 Docker 中心界面维护、升级、恢复及冷启动通过；完整改造 ACTIVE

本块替代上一批“Docker 已配置实例维护/升级尚未验证”的当前进度，保留所有历史未知结果与完整台账。

- 同一 R013 shadow **2181 source+4 fixture**；本批改14父输入、增2source，相对V51为97输入增量。**298 production/test路径整体未应用**（122existing+179new共301记录含3fixture-only），不是298项任务。完整CODE seal仍V51、G023游标不变。精确snapshot只用于续接/回退参考，不能冒称整体封存或发行。
- 当前source `source-checkpoint-console-docker-maintenance-final.json` SHA `297bbace579f466b5fcffceac89f98c02a409cde6401c3a452876059ec191eb3`；组合 `registered-console-docker-maintenance-v52.json` SHA `09da819f7d8de1d7cb3259a26c886491ad8aac7c1c593dad3673142aa2e5dbab`；组件 `console-docker-maintenance-v1/final-binding.json` SHA `ac11ff339b2a789a80af6f228efe7147b917fc4953276bb96ca1b7f7cdbfb800`（151证据pin）。根目录 `kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/`。
- 真实 Cockpit PAM/管理员提权、可信浏览器peer、固定root命令 `superuser=require`：已完成已配置Docker原63da部署的freeze、stop、镜像选择/升级和same-image resume。所有变更先持久化intent、成功才收receipt；独立systemd worker不依附浏览器。原volume、安装身份、证书、task key保留，旧容器停留且restart=no。
- Rust维护协议2 `serve-maintenance 原UUID` 仅接受既有已配置安装和原freeze；加载DB/私有状态时独占、开始监听仍保持业务冻结；实际TLS及DB ready后首次complete，再恢复restart。最终immutable启动命令可在同一完成UUID下冷启动，其他未结freeze仍拒绝，不重做首次设置。
- 真实故障保留：旧镜像PID1忽略SIGTERM，Docker停服最终exit137/noOOM；未重发旧stop。首协议2镜像root shell等待子server，旧PID1假设导致已成功start后核定失败，原CCC保持UNKNOWN。只读核定唯一server2450/UID999/普通hash1bb/原fence/DB-TLS后，实际界面严格同ID `verify-start`：已有成功start及全部变更回执，无重复create/start/stop，首次complete。`priorStartReview.previousPhase=UNKNOWN/newStartIssued=false`及旧失败保留；不适用于原767863a9。
- 后续entrypoint改直接exec、默认SIGINT；pidfd仅指向原核定server并检查starttime，发一次SIGINT，无强制升级。M2 `73a357b0-547b-4786-925b-8628b1a9fc7c` 升级及M3 `f203b659-a5ef-42ad-a0ab-43a0b69a824b` 同镜像恢复均正常exit0。M1/M2/M3皆COMPLETE。最终image b0de8c4f/OCI838b41a3，普通opt3非test debug0 binary1bb7b4fc；最终容器1a8c3c01是非root UID999直接PID1、cap0/NoNewPrivs1。
- 原Docker fixture此前未配对，本批新pair1/setup operation1/login1，初始化 `2f14e5dd-97f4-424f-9207-8ca5d8009065`，不是旧一次性验收重放。实际restricted PG role：1安装/1管理员/1COMMITTED审计/0tasks/0gateways。外部CA私钥位于所有VM mount之外，仅leaf私钥/证书及public CA上传；不是多人CA quorum认证。
- 同owned machine计划冷停启后，20私有文件、7操作、3代际、selected registry逐字节一致，原f203完成UUID不变，无新pair/init/export，原leaf e2cae79b/session200/DB ready。原17文件升级前后精确。41旧native文件、2旧DB初始化、8旧native维护记录不变，旧a79b传输UNKNOWN_RETAINED不重发。
- **88 Rust通过/38ignored、67 Linux-root Python、28浏览器契约通过**；真实UI/进程/DB/TLS证明独立。154包上下文当前，116compiler/web+2运行包装输入精确；初版全recipe实际编译，最终全recipe复用同普通binary，不能声称第二次编译。Tools15 SHA9f43661f，14payload/11critical，Core6仍12payload/binary3e934cb9，无重叠。图谱2654nodes/10605edges和10路径metadata match仅best-effort。
- 本轮owned Chrome已关闭，同 `yofc-console-v52` 已正常停止，私有profile/失败/旧包/3旧Docker代际/volume/PG均留；14全局新对象12exited0+2never-started，无OOM。最后关机前已验证current ready，不冒称关机后individual Docker退出验收。未改全局provider/引擎，不重建用户清理的旧中心/PG/60容器。完整sandbox仍内部pending。

**下一实际实施**：在这个稳定阶段刷新G023 fork/nonfork与关联依赖、按逻辑功能划分上游批次，非fork分析关联后跳过，不直接bulk merge；core7eca88b2 SSL/Crypto破坏性cohort内部pending。随后installer bootstrap/正式平台、跨版本DB及非空加密任务升级/备份恢复、凭据部分恢复续期/首次TLS UNKNOWN核定、中心CSR下载外签上传/地址/DB、agent独立身份CSR续期通道能力、多管理员/外部CA custody quorum、bulk/正式sandbox/polkit继续。协议0在线升级未认证。

完整owner/ordinary/recovery/原forward-confirm-inverse-observation-stopped-expiry、合法boot/netns/handle恢复/全CM-runtime-NAT与网络矩阵、初始化先退役再原inverse/子记录结算退役历史、14组/7API2/R017/**298整体guarded apply**/正式API/协议crate及joint generations、G021真实LDAP、G022 Go31vet/3非JavaPOM/可选regen/.NET和G023全部保留，不因本批通过而缩减范围。保留本文件下方完整历史台账。

18原仓工作树/未跟踪/index/refs/remotes及1986/20/519/89、credentials141+5/API522精确。原767863a9仍唯一startup.stop UNKNOWN，intent a87d94d52839e316845b7c831b265daf928f0bfd5fb0a883fb49b721cba5f63c不变，**无stop receipt/publication/start/complete/activate/cancel/re-export**；原boot4db3、2799/3635/81713/82137/59158、agentaffa64e2精确，原现场未停/重启/发布。CLOSED/GWT三次保存/V36v1/旧首次TLS不重放。仅原3组external deferred（NM/cloud-init/physical、amd64、disk-full/hard-power），其余internal pending。不提交/推送/reset/clean/局部apply/共享API安装。周23%已用/ordinary allowed，credits54476.099936及2reset未用，目标ACTIVE。

## 2026-11-14 最新续接：Docker 实际界面部署与正向主机 GUI 停服通过；完整改造仍 ACTIVE

本块替代上一批“Docker 创建/启动 GUI、正向主机 GUI stop 尚未验证”的当前进度，历史未知结果及完整台账保留。

- 同一 R013 shadow **2179 source+4 fixture**，本批改8父输入、增3source；相对V51为95输入增量。**296 production/test 候选全未应用**（122existing+177new共299记录含3fixture-only），不是296项任务。原仓未局部apply/提交/推送。完整CODE seal仍V51；精确snapshot用于续接/回退参考，不能冒称封存或整体可发布。
- 当前source `source-checkpoint-console-docker-ui-final.json` SHA `f68e215e0aaa30705150a48b43949c80b2d2505a3edbb65750edc75a3453e58d`；组合 `registered-console-docker-ui-v52.json` SHA `1c4b917a17d2c76fd768f108a21aebff990413515e5debef7e476e80e0e5fb4c`；组件 `console-docker-ui-v1/final-binding.json` SHA `0365b5612937d96fc8ef28fb2f6bfeed331a8ff47e520a3566873e320fc45e68`。路径根为 `kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/`。
- 真实 Cockpit PAM/管理员提权和可信浏览器peer、固定 `superuser=require`：已完成离线镜像上传/导入、镜像选择、专用volume、实例/HTTPS与gRPC端口、容器创建/start、实际TLS就绪核对、私有凭据下载、按钮打开不同域名设置页。新Docker实例只到未配对设置门：本批0pair/0初始化，不重放旧一次性验收。
- 原镜像load `2e48fda6-bde5-4f8f-9336-913bac2839ed` 有真实成功回执，Docker29/containerd使用manifest ID导致旧config-ID后验失败，曾保留UNKNOWN。补齐分别核定archive/config/OCI manifest，并由真实界面在同一原ID只读核定成功，**0新增Docker写、load共1次**。旧UNKNOWN与核定证据保留；无load成功回执时，不因镜像存在而核定，也不重发/换ID。
- 新部署 `63da2cc4-eb21-4fec-b340-5c78f66f09d5`：`yofc-console-docker-d-v52` / `yofc-console-data-docker-d-v52`，48443/48444，volume/create/start各1成功回执，512MiB/1CPU/只读根/降权console；实际发布peer与主机只读指纹匹配，READY通过。下载0600私有保存，浏览器实际访问 `https://docker.yofc-console-v52.orb.local:48443/setup`，未上传凭据。6bootstrap私有文件保留。刷新保留原完成ID，不重发。
- 新的独立正向主机GUI维护 `59b07646-8d67-4b85-bf8f-5f31ccabdd50`：实际begin/stop/complete各一次，3成功systemctl stop回执+停止/PID0/空Job，校验后解除freeze且不autostart，刷新保留COMPLETE。原7条维护记录字节精确、现8条。**旧a79b传输仍UNKNOWN_RETAINED，原parent取消回执不变、旧stop未重发**；新正向验收不改写原传输历史。
- 独立Tools9已实际安装，version `0.1.0~v52-readiness20261114-9`，deb SHA `469cb8a43cbffd3efe4b1b7dff70b22146b9d67755513f64af1633f3e11e88b8`。Root10关键文件；13tools+12core实际payload/dpkg owner精确无重叠，Core仍6。41旧私有文件、2原安装/管理员/COMMITTED初始化、0tasks/gateways不变，fresh paired仍0operation/generation。
- **57 Linux-root Python +24浏览器契约通过**；真实PAM/Docker/TLS/GUI证据独立。152完整包上下文当前，115普通运行输入精确，binary3e934cb9/image1c087e91不变，本批未重建Rust/image或重放旧全栈验收。Docker29本地引擎在已有owned Linux machine中运行，未改全局引擎/provider。
- 本轮独立浏览器已关闭，profile/下载/失败/旧包均留；8全局新构建/测试容器终态、无OOM（包括保留首版测试失败）。3native已由新GUI维护停止，再正常停止同一 `yofc-console-v52`（2CPU/1536MiB/12GB）；新Docker容器/卷/PG保留。其最后在关机前验证running/READY/unpaired，不冒称Docker GUI停服或关机后容器exit验收。不恢复用户已清理的旧中心/PG/60容器。

**下一实际实施**：Docker已配置实例的freeze/stop/complete与真正镜像升级GUI、工具bootstrap；保留既有Docker原卷/安装身份/请求与回执。临时身份/凭据部分恢复续期、首次/TLS UNKNOWN核定、中心CSR/外签上传/地址/DB、agent本地身份/CSR/续期/远程开关能力、多管理员/外部CA custody quorum、bulk/备份恢复、正式sandbox/polkit继续。协议0在线升级仍不认证。完整owner/ordinary/recovery/原forward-confirm-inverse-observation-stopped-expiry、合法boot/netns/handle恢复/全CM-runtime-network-NAT矩阵、初始化先退役再原inverse/子记录结算退役历史、14组/7API2/R017/**296整体guarded apply**/正式API/协议crate和joint generations、G021真实LDAP、G022 Go31vet/3非JavaPOM/可选regen/.NET、G023逻辑同步（core7eca88b2 SSL/Crypto破坏性cohort）全部保留，不能因本批Docker入口通过而缩减。

18原仓Git/工作树/未跟踪/index/refs/remotes、1986/20/519/89、credentials141+5/API522精确。原保护767863a9仍唯一startup.stop UNKNOWN：a87d94d52839e316845b7c831b265daf928f0bfd5fb0a883fb49b721cba5f63c intent精确，**无stop receipt/publication/start/complete/activate/cancel/re-export**；原boot4db3、2799/3635/81713/82137/59158、agentaffa64e2精确，不停/重启/发布原现场。CLOSED/GWT三次保存/V36v1/既有首次TLS不重放。方案A外部CA私钥custody不变，未挂/传CA私钥；仅原3组external deferred（NM/cloud-init/physical、amd64、disk-full/hard-power），其余internal pending。周21%已用/ordinary allowed，credits54476.099936与2reset未用，目标ACTIVE。

## 最新续接：独立主机工具与真实界面取消已验证（2026-11-14，V52 ACTIVE/OPEN）

按用户确认的重建现场继续，未恢复旧中心/PG/60容器。完整改造仍OPEN。同一R013 shadow **2176 source+4 fixture，293 production/test候选全UNAPPLIED**（122existing+174new含3fixture-only），相对V51有92输入增量；本批改9父输入、新增1测试源。文件数量不是剩余任务数量，最后完整CODE seal仍V51，G023游标不动。

当前source-checkpoint-console-maintenance-host-ui-final.json SHA `13a2d0cbc68d7c89a9778ddb77342a0cc7ffff79ae44290dd602a74d6698ad2b`；组合registered-console-maintenance-host-ui-v52.json SHA `4ce006052dfb1d27dde5ea468f93d24706544c2151c046585d9b7491878f8f5d`；组件console-maintenance-host-ui-v1/final-binding.json SHA `bd03e65d6e0f86bbc7f95231c5c4ec8f048b06f39ec9d9610bc52c7726d3bc3c`（80证据pin）。当前完整候选与2176+4精确snapshot均在console-maintenance-host-ui-v1。上一批组件/源、失败和私有现场均保留。

实际Cockpit PAM登录/管理员提权在浏览器TLS peer与主机私有通道证书精确匹配后通过。旧安装页面真实冻结3中心，原编号 **a79b3e1f-ba0f-4ea1-b8f7-e9bbd840e4ab** 重载后可只读找回。随后停服点击的观察超时，工具报告kernel reset，owned浏览器进程随后缺失；本地停服意图文件保留。权威服务端仍FROZEN、无native stop intent/receipt、3原PID精确。延迟的只读orb exec均轮询原handle到完成；没有为观察失败重启服务/VM。主机当时memory_pressure显示48%free，不据此归因OOM。**该GUI停服传输结果仍UNKNOWN，未重发，也未认可GUI正向停服验收。**

由此补齐了实际界面恢复缺口：修改动作前持久保存公开UUID/请求步骤，重载/未知不重新发送stop或complete；缺服务端记录改为UNOBSERVED，不把缺记录当未送达证明；原完成编号/结果保持可见。浏览器该记录仅UUID/step/outcome，不存凭据、密码、私钥或Cookie；中英文维护结果使用普通语句。

新增独立的负向取消：在同一host/exclusive writer锁内先退役原维护UUID，再移除冻结，后到begin/stop被原编号拒绝。未观测请求仅写tombstone，不移除别的操作fence。COMPLETE配CANCELLED_BEFORE_ACCEPTANCE或CANCELLED_BEFORE_STOP_INTENT保持旧协议1读取兼容；回执明确transportNonDeliveryCertified=false。已存在native stop意图/不确定回执、offline mask/接管、参与者改变、协议不符或服务不再运行时拒绝取消。取消不stop/start/restart；负向回执已落但pointer清除失败时有独立“完成已确认取消”。**仅主机维护退役，不是首次初始化/远程写入UNKNOWN重置或重试，更不适用于原Parallels startup UNKNOWN。**

主机工具拆为architecture-all **kura-management-console-installer**，有独立root文件/源码provenance；不含中心程序、unit、state或任何maintainer script。Replaces仅接收旧monolithic helper文件，后续中心包依赖匹配工具包。root入口验证8固定关键文件及其独立source-manifest。实际apt先只安装tools6，中心保持运行/冻结，原程序/PID/旧core prerm/原journal/fence及41私有文件精确；Core5未升级，无stop/start。这个路径为冻结期间修复管理工具提供实际交付依据，完整bootstrap GUI/正式发行仍pending。

更新后的真实中文主机页面读取旧a79b冻结，禁用stop，通过实际superuser=require仅执行一次新的maintenance-cancel。原编号回执成为COMPLETE/CANCELLED_BEFORE_STOP_INTENT、native stop intent/receipt仍空、lateActionsFenced=true，传输是否送达仍不认证；3PID/41私有文件精确。实际页面重载后原取消编号/结果仍可查。新一轮helper调用只有status/cancel/status，没有旧stop。新功能验证使用独立Chrome观察server，保留私有CDP handle，验证回执后才关闭；旧失败profile/截图/意图均保留。

仅在原编号已退役后，为**本批新包布局**进行必要升级回归：Core5→拆分Core6使用独立新包维护记录，不重发旧GUI stop。标准apt实际通过；12中心+11工具payload/owner精确、无重叠，3新stop成功回执/目标摘要属于这次新升级，旧a79b取消回执逐字节不变。现7完成维护记录保留。两原DB各仍1原安装/管理员/初始化COMMITTED、0tasks/0gateways，原4870795f与085ff3b5初始化不重发；全部41私有文件精确，fresh paired仍0operation/generation。本批0初始化/配对/CA上传/网关命令/原保护现场动作。

Core6 SHA `1b488ad5996b9338280c477f6832e4d9309b11ba39fe5a8a75d37aa483fc1279`；Tools6 SHA `fe7df354197df0e6b69b792ae6db1a245fde960044b3b3680fa5646bbef9a8d8`，版本0.1.0~v52-readiness20261114-6，maxGLIBC2.34。普通ARM64程序仍3e934cb9c25bf2eb2c1ca7a76d6f5b9db0ead6178475920a177f61d331287823，runtime image仍1c087e91f7c38a60a88e884100b1b373fa863b14a880b981dc6811c50ad4daec。115编译/Web runtime输入含78Rust/迁移/构建输入与已建制品一致；仅主机工具/打包/文档/测试变化，未重建Rust/image或重放父批Rust验收。143完整包上下文输入当前。46Linux-root Python+19host browser契约通过，新增10不同检查（6Python+4browser）；真实PAM/peer/取消/包证据独立。Node观察导入/证书缓存/变量或frame错误和未派发的歧义Close、wrong-pin服务端BrokenPipe诊断均保留，不能冒称应用成功。

本批3Docker对象exited0/noOOM，owned浏览器均缺失且profile保留；3native unit已由新的明确包升级停止，随后machine停止。所有PG/state/volume/下载/profile/失败/cache保留。仍只用owned yofc-console-v52/01M7BKVJ6SNTMPY3124ANRHNXD（2CPU/1536MiB/12GB），仅旧/proof新增inbox，未挂CA私钥。provider覆盖精确未改，完整sandbox仍内部pending。

下一优先Docker镜像/volume/端口创建、start/upgrade/维护GUI及bootstrap交付；**正向主机GUI stop仍未认可，不重放a79b**；协议0在线升级仍不认证。凭据部分恢复/续期、首次/TLS UNKNOWN核定、中心CSR/外签/上传/地址/DB维护、独立agent身份/CSR/续期/通道/能力、多管理员/CA quorum/bulk/备份/正式sandbox/polkit继续。完整owner/ordinary/recovery/原forward-confirm-inverse-observation-stopped-expiry、boot/netns/handles后合法restore、全family/CM/runtime/NAT true/false/多WAN/modem/DHCP/DNS/WiFi/VLAN/packet/fragment/unsupported、初始化退役先于原inverse、子记录结算/退役/历史/重接管、14组/7API2/R017、**293整体guarded apply**、正式API配对/协议crate校验和/joint generations、G021真实LDAP、G022 Go31vet/3非JavaPOM/可选regen/.NET、G023逻辑上游同步全部保留。G023仍10原+7shadow/UI-util1.3、11fork7nonfork/327Maven1Cargo，Coref0b2120/UI1c51246/PLCea42游标不动，7eca88b2 SSL/Crypto breaking cohort内部pending。方案A外部CA/custody不变。

18原仓Git/未跟踪/index/refs/remotes、1986/20/519/89、credentials141+5/API522精确。原保护操作767863a9-fe6b-4135-9783-ddcb131030af仍UNKNOWN；stop intent a87d94d52839e316845b7c831b265daf928f0bfd5fb0a883fb49b721cba5f63c精确，无stop receipt/publication/start。原boot4db3、bus2799/NM3635/agent81713/Kura82137/proxy59158及agent affa64e2保持。原保护现场从未取消/stop/restart/发布/start/activate。仅原3组external deferred，其余internal pending。旧CLOSED/V36v1/GWT三次保存/闭环首次与TLS不重放；不提交推送/reset/clean/局部apply/共享API安装。周20%已用/ordinary allowed，credits54476.099936和2reset未用。目标ACTIVE，完整改造OPEN。

---

以下保留历史；仅在上述明确范围替代“主机工具只能随中心一起更新、维护步骤仅内存保存、取消未实现”。不得把真实负向取消或新包升级当作旧GUI正向stop通过，更不能据此处置原Parallels UNKNOWN。

## 最新续接：维护并发冻结及协议1升级已验证（2026-11-14，V52 ACTIVE/OPEN）

已按用户确认清理/重建后的现场推进，未恢复旧中心/PG/60容器。完整改造仍OPEN。同一R013 shadow现**2175 source+4 fixture，292 production/test候选全部UNAPPLIED**（122existing+173new含3fixture-only），相对V51有91输入增量。本批改25父输入、增4输入；另4份原先相同的测试文件现在产生差异，因此候选数从284增8。文件数量不是剩余任务数量。最后完整CODE seal仍V51，G023游标不变。

当前source-checkpoint-console-maintenance-barrier-final.json SHA `87bf82311b23fd4afa5333c33aa6e669a3e8461e81169ea177a38411a70d7058`；组合registered-console-maintenance-barrier-v52.json SHA `f551ebc7ea0e9678de90b826765872f3049f6373b4ec7d6499bfee39eee1f7e1`；组件console-maintenance-barrier-v1/final-binding.json SHA `c42c2138d37dad9d8a835f65ad452b5007ff5df937da9bc29c1c634c266b9455`（99证据pin）。候选及2175+4精确snapshot位于console-maintenance-barrier-v1。上一批readiness组件/源及全部失败保留。

HTTP、gRPC连接/帧/派发/定时清理、独立初始化job、启动迁移/密钥恢复和本地修改命令共同持有写入租约，直至持久提交。初始化job在accepted落盘前取得独立租约；已消费gRPC回执只等临时Busy，不越过持久冻结。主机协调器在排他租约内检查全部保留实例和任务，再发布root持久fence；Busy/QUEUED/RUNNING/UNKNOWN/首次或TLS缺回执在停服前拒绝。已装native程序及实际进程摘要必须匹配维护协议声明。root控制位于/var/lib/yofc-console-maintenance，不混入实例目录枚举；启动/建实例与维护互斥。通用CLI名称/参数个数/基本类型和材料在DB迁移/恢复/admin创建之前检查；业务授权仍在事务内，旧transfer发行commit先于token输出的恢复仍内部pending，不能重发。

停服前先持久原UUID意图；实际派发要求systemctl成功return0、inactive/failed、MainPID0、Job空才落回执，原本已停的实例明确commandIssued=false。缺回执即UNKNOWN，即使后来进程消失也不重stop/换ID。完成回执绑定安装程序/source-manifest摘要，再解除fence；不自动start/enable。未完成接管/unmask保留冻结供明确核定。界面新增冻结/原操作查看/停服/验证解冻；未知结果仅查原ID。首次页面显示维护状态，首次启动15次轮询耗尽有明确结束提示。新增界面本批是adapter契约，**实际Cockpit新维护动作端到端仍pending**。

受控owned测试机仍yofc-console-v52/01M7BKVJ6SNTMPY3124ANRHNXD，2CPU/1536MiB/12GB，仅原/proof挂载加新inbox，不挂外部签发私钥。3中心0任务/0网关后做受控离线准备，41私有文件精确。协议0在线升级不能认证：dpkg先运行旧prerm，新preinst不能补救旧窗口；只接管已全部停止的旧实例，mask默认/template并复核，无新stop，未知保留fence/mask，通用GUI交付旧helper仍pending。

实际PostgreSQL表锁阻塞真实chosen-admin登录写入，维护Busy在fence/stop前拒绝，3PID保持；释放后仅该原登录提交一次。随后真实冻结拒绝HTTP、本地migrate/建key及创建新实例，公开只读状态仍可查；完成3真实停服回执与同原ID解冻。协议1标准apt live升级-3→-4通过并保持服务停止。最终-5补显式commandIssued/returncode和目标摘要回执，受影响回归实际3次成功stop/3份回执后完成，不自动启动。5份完成维护原记录保留；20安装payload/root ownership当前；全部41私有文件精确，旧2DB各仍1原安装/管理员/COMMITTED初始化、0任务/网关，原4870795f与085ff3b5初始化不重发；fresh paired实例0初始化记录/generation。没有初始化POST/CA上传/原现场动作。

完整普通ARM64 opt3/non-test SHA `3e934cb9c25bf2eb2c1ca7a76d6f5b9db0ead6178475920a177f61d331287823`；78编译输入/143完整上下文精确，最终image `sha256:1c087e91f7c38a60a88e884100b1b373fa863b14a880b981dc6811c50ad4daec`。仅host-helper元数据变化后完整缓存构建仍同不可变image，未重启容器贴源码标签。标准候选deb0.1.0~v52-readiness20261114-5 SHA `d3868baf6123c484a1066e650062d7bad50c0bd306fe3739c418c3899a33b9f2` 实际apt安装，maxGLIBC2.34。新Docker fixture UID999/CapEff0/只读rootfs，0初始化/配对；持久冻结拒绝API/serve，跨显式SIGINT后仍在，之后已知新启动被拒绝；同UUID由unprivileged one-off完成后仍不启动，6原私有文件精确，仅增完成回执。实际exit1/1保留，非强凑exit0；Docker选择/维护GUI仍pending。

85Rust、40Linux-root Python、15host浏览器契约通过；38隔离PG+3stack MTLS保持ignored，非验收重放。新增12不同自动检查（5Rust+6Python+1browser），真实PG/系统/容器证据另列。root Python首版装配失败、wrong-pin服务端BrokenPipe诊断和driver转义warning保留，不因观察失败重启服务。本批12Docker对象terminal/never-started，无OOM，builder停/缓存留；3native unit和machine已停，PG/state/volume/profile/下载/失败保留。provider沙箱覆盖精确未改，完整sandbox仍内部pending。

下一批优先实际Cockpit维护动作、旧helper/离线交付、Docker选择/维护GUI；凭据部分恢复/续期、首次/TLS UNKNOWN核定、中心CSR/下载/外签/上传/地址/DB维护、独立agent身份/CSR/续期/通道/能力、多管理员/CA quorum/bulk/备份及正式sandbox/polkit继续。完整owner/ordinary/recovery/原forward-confirm-inverse-observation-stopped-expiry、boot/netns/handles后合法restore、全family/CM/runtime/NAT true/false/多WAN/modem/DHCP/DNS/WiFi/VLAN/packet/fragment/unsupported、初始化先退役再原inverse、子记录结算/退役/历史/重接管、14组/7API2/R017、**292整体guarded apply**、正式API配对/协议crate校验和/joint generations、G021真实LDAP present/absent、G022 Go31vet/3非JavaPOM/可选regen/.NET、G023逻辑上游同步全部保留。G023仍10原+7shadow/UI-util1.3、11fork7nonfork/327Maven1Cargo，Coref0b2120/UI1c51246/PLCea42游标不动，7eca88b2 SSL/Crypto breaking cohort内部pending。方案A外部CA/custody不变。

18原仓完整Git/未跟踪/index/refs/remotes、1986/20/519/89、credentials141+5和共享API522精确；原boot4db3、bus2799/NM3635/agent81713/Kura82137/proxy59158及agent affa64e2保持；原stop intent a87d94d52839e316845b7c831b265daf928f0bfd5fb0a883fb49b721cba5f63c精确，无stop receipt/publication/start，原UNKNOWN仅只读观察。仅原3组external deferred，其余全internal pending。旧CLOSED/V36v1/GWT3次保存/闭环首次与TLS不重放。不提交推送/reset/clean/局部apply/共享API安装。周19%已用、ordinary allowed、credits54476.099936及2reset未用；目标ACTIVE，完整改造OPEN。

---

以下保留历史。仅在上述明确实现/验证范围替代上一块“维护冻结和CLI前置检查未实现”；实际维护GUI、旧版在线升级、完整平台/源码整体接纳仍未完成。

## 最新续接：HTTPS就绪、双栈和完整镜像已验证（2026-11-14，V52 ACTIVE/OPEN）

本批完成了实际主机HTTPS就绪核对、首次启动只读轮询、新安装双栈/端口默认，以及**完整多阶段镜像源码构建**。未代表完整平台/改造完成；下一重点是维护并发冻结/排他和生命周期协调。同一R013 shadow现2171 source+4 fixture，相对V51有83个输入路径增量，**284 production/test候选全部UNAPPLIED**（118 existing +169 new含3 fixture-only）。本批修改上一阶段19输入、引入4输入/候选，不将83解释为83项新增需求或已完成任务。

source-checkpoint-console-platform-readiness-final.json SHA `77fc76d7a6c7a8daca66deea8ad35522a1e89204f34d934616c951720d61f6d1`；组合registered-console-platform-readiness-v52.json SHA `d87cbd0e90837f35a9c449d27cf4b9c31576500c0e9ce5d18f3f3fc94eb22432`；组件console-platform-readiness-v1/final-binding.json SHA `8c3b77b4f28d2fbbe48ecd3d723f8d67ec72b798a2c251498615cbf0595677f6`。完整候选console-platform-readiness-v1/working-candidate-readiness-v52.json，working-source-snapshot保留2171+4逐文件精确输入。不是完整CODE seal，最后完整CODE仍V51，G023游标不变。上一阶段c5e9/dc48/0a17及全部失败证据保留。

安装器区分进程active和HTTPS ready：新增固定只读installation-status命令，在任何DB启动前分支处理；只读已选不可变TLS及原bootstrap，不连接DB、不生成身份/密钥、不取得服务租约、不发布指针、不读取待审批材料作为现用材料、不导出token/password/privatekey。root helper先验证声明能力、程序/source-manifest摘要及active进程程序摘要，再以本机可信叶证书精确SHA核对TLS peer，之后只GET固定/health/live；受限成功响应才显示主机HTTPS就绪。该判断不替代浏览器核对发布地址/域名/证书。旧制品无能力声明时在应用命令前拒绝，不探测未知命令。

界面改为普通就绪提示，不把PID当可操作结果；启动后最多15次附加只读status，无自动重发start或转换restart，目标变化取消旧观察，未核实HTTPS不打开本机中心页面。原先读取状态覆盖端口的问题保留修复。新自动首次监听优先IPv6通配双栈；仅IPv6 unavailable回退IPv4，端口占用不回退。显式IPv4保持原地址族。unit不再强制HTTP地址覆盖已保存设置；新实例界面只创建内部CONSOLE_SETUP_PORT文件，用户不必手填环境变量。向导根据实际HTTPS地址族/端口选择代理通道下一端口（65535取65534），保留用户修改。Cargo.lock仅将已有锁定socket2加为直接依赖，没有依赖版本漂移。

完整Dockerfile实际执行node npm ci/build、Rust普通release和Ubuntu24.04运行阶段；Rust固定1.95-bookworm，2编译job。独立builder实际限2CPU/3GiB/noOOM，未改默认builder或全局Docker设置。普通ARM64 opt3/non-test程序SHA `bb94bb3e20f33a2e4fa2b47ef56522fd85337f1ac02920974b9c8b49200c4879`；76个Rust/迁移/构建输入、140个完整源码构建上下文输入逐一当前。最终镜像 `sha256:fa292ef0a10a0df0141202e0542c9cbd537b472df3d8574d1613828537f8426d`，后续前端/host-helper修复通过完整构建和相同不可变imageID接纳，没有重启运行容器重贴源码标签。真实运行UID999/CapEff0，IPv4/IPv6 localhost HTTPS均核对证书与hostname；无初始化/DB动作，5个私有文件在SIGINT停止后用独立只读volume观察逐一精确。

候选标准deb0.1.0~v52-readiness20261114-2_arm64 SHA `8982a11c5ea12d9d83745c803e891ac470ff6634191331820b3fbba792f035cb`，实际apt安装；19个安装payload/root archive ownership与当前制品一致，最大GLIBC2.34。复用上一批owned OrbStack Ubuntu24.04 ARM64 machine yofc-console-v52 / 01M7BKVJ6SNTMPY3124ANRHNXD（2CPU/1536MiB/12GB），未新增machine/重型VM，也未恢复用户清理的旧Docker中心/PG/60容器。只在原共享证据的全新inbox交付包/脚本，原挂载范围不扩大、不挂签发私钥目录。

该测试机已enable的两份APPLIED中心正常启动，随后在验证只读维护guard后升级，最后从实际主机界面对受影响的普通监听做已配置数据启动回归。原默认4870795f-125e-4d9e-828b-c984dbdb0fb1和实例085ff3b5-0123-48e7-8247-7a9850101a02各1条原安装/COMMITTED审计、1管理员、0任务，全部34个私有文件精确；原初始化不重发、不换ID。原明确IPv4地址继续保持，未宣称修复provider旧IPv4域名路由。

新实例ready-c-v52在38443实际IPv4/IPv6 HTTPS通过，实际fresh.yofc-console-v52.orb.local无需浏览器DNS映射或宿主DNS修改即可从主机页面打开；先核对peer SHA与主机下载的原凭据，再只做一次解锁，实际表单默认[::]:38444且保留手改。只POST /api/setup/pair，**本轮0初始化POST、0DB凭据/服务器私钥/CA上传**；原6个bootstrap文件精确，只增setup/session.json，无operation/generation/taskkey。方案A外部CA/custody不变。

实际首版就绪检查把/proc/PID/exe错误送入控制台路径的符号链接保护，保持UNVERIFIED，服务未因观察失败重启，界面没有重发start。失败及15次只读观察保留；revision2仅对固定数字PID的内核链接独立读取，不放宽配置/状态路径保护，并增加真实Linux kernel link回归。新实例仅在已知无配对/初始化、通过维护guard且标准包升级已停止服务后由用户动作再次start，原身份未变。首版source不冒称最终source。

Cookie辅助库URL过滤误报保留；Chrome原生Network.getCookies对精确setup API匹配仅yofc_setup，排除主机会话。未捕获原POST Cookie原始报头，不把它当wire证明，也未因此改应用或重启服务。最初关闭浏览器块被该观察断言中断，stop元数据误写browsersClosed，原错误记录与显式更正并存；实际stop前已独立验证全部服务结算且0新初始化，随后浏览器关闭。额外原现场只读-c取证的参数拆分失败保留，改用已保存只读脚本后验证五进程/boot/agent SHA，不做服务动作。

当前30项不同相关Rust通过（1项live PG ignored未重放）、34项Linux root Python通过、14项主机浏览器契约+3项设置默认契约通过。相对父组件新增21项不同检查：5Rust+8Python+5host browser+3defaults；模拟桥只证明契约，实际主机/域名/peer另有证据。8个Docker对象均terminal或从未启动、noOOM；runtime显式SIGINT后exit1、builder显式stop后exit1按实际保留，构建/测试命令本身成功，未为凑exit0重跑。三owned native unit与machine停止，PG/私有状态/volume/下载/profile/cache/失败证据全部保留。下次启动machine会自动启动enable的已配置/未初始化实例，先观察原状态，绝不重发两笔原初始化或生成新身份。

**仍internal pending**：维护冻结/排他与跨实例/CLI/HTTP/gRPC完整生命周期协调、全CLI参数在DB前校验、Docker选择完整GUI、OS/Docker Desktop交付、临时身份/凭据半成品恢复/续期、首次/TLS UNKNOWN核定UI、中心CSR生成下载/外部签发上传、地址/DB维护、独立agent本地身份/CSR/续期/通道/能力、多管理员/CA quorum、bulk/备份与正式sandbox/polkit。provider沙箱覆盖SHA精确未改，实际NoNewPrivileges/ProtectSystem等仍被削弱；功能链不认证完整沙箱。完整镜像和新安装双栈只在上述ARM64/宿主内核范围闭环，未新增hardware/amd64/断电认可。

完整owner/ordinary/recovery/原forward-confirm-inverse-observation-stopped-expiry、boot/netns/handles后合法restore、全family/CM/runtime/NAT true/false/多WAN/modem/DHCP/DNS/WiFi/VLAN/packet/fragment/unsupported矩阵、初始化退役先于原native inverse、子记录结算/退役/历史/重接管、14组/7API2/R017、284整体guarded apply、正式API配对/协议crate校验和/joint generations、G021真实LDAP present/absent、G022 Go31 vet/3非Java POM/可选regen/.NET、G023逻辑上游同步全部保留。G023仍10原+7shadow/UI-util1.3、11fork7nonfork/327Maven1Cargo，Coref0b2120/UI1c51246/PLCea42游标不动，新Core7eca88b2 SSL/Crypto breaking cohort内部pending；不批量merge/nonfork合并。

18原仓完整Git/未跟踪源/index/refs/remotes、1986/20/519/89、credentials141+5及共享API522精确。原boot4db3、bus2799/NM3635/agent81713/Kura82137/proxy59158活跃，agent程序affa64e2指纹精确；原stop intent a87d94d52839e316845b7c831b265daf928f0bfd5fb0a883fb49b721cba5f63c，无stop receipt/publication/start。原767863a9-fe6b-4135-9783-ddcb131030af UNKNOWN保持，不stop/restart/发布/启动/激活/换ID/重导出，预算release8b07未部署。仅原3组external deferred（NM/cloud-init/物理、amd64、disk-full/hard-power），其余全internal pending。旧CLOSED/V36v1/GWT三次保存/已闭环首次与TLS验收不重放，不提交推送/reset/clean/局部apply/共享API安装。周用量18%已用、ordinary allowed，extra credits54476.099936不变、2 reset未用。目标ACTIVE，完整改造OPEN。

---

以下保留历史；上一批“可靠就绪/双栈/完整多阶段仍pending”仅在上述实现和实际验证范围被替代，其余完整台账继续有效。

## 最新续接：真实标准包与双中心主机界面验收（2026-11-14，V52 ACTIVE/OPEN）

本批已完成真实Ubuntu24.04 ARM64标准deb/systemd默认与命名实例、真实Cockpit PAM登录/提权/普通账号拒绝、凭据下载和浏览器首次配置。它是**部署链路的实际增量，不是完整改造完成或正式发行**。同一R013 shadow仍2167 source+4 fixture、相对V51的79个输入路径增量、280 production/test候选全部UNAPPLIED（118 existing +165 new含3 fixture-only）。本批修改上一阶段7个既有输入，没有增加候选路径；75个Rust编译输入逐一不变，复用普通ARM64 opt3/non-test release `91e56aef1f1adcf0bc5c06236af5c83535752e1390fd75436ab9dceeb1d76805`，没有为计数重跑已闭环Rust/应用验收。

当前source-checkpoint-console-systemd-host-final.json SHA `c5e995d82f2443c7eca9883c49d21d886a3b8a025c227b4f1a5bc5385f484edb`；组合registered-console-systemd-host-v52.json SHA `dc48d84302d69ef3707a229197a32a23c3fd8dea8533e0ed0ac34cfcfab71939`；组件console-systemd-host-ui-v1/final-binding.json SHA `0a1709b2d6eef124fd4521bed4f96382ff64427462e3b7d2cbad77568b4bbbe2`。完整候选为console-systemd-host-ui-v1/working-candidate-systemd-host-v52.json；该组件working-source-snapshot保留全部2167+4逐文件精确输入。不是完整CODE seal；最后完整CODE仍V51，G023游标不变。旧平台349e/776a/4d81记录和各失败证据保留，不覆盖。

用户已确认旧Docker环境清理/重建，旧中心/PG/60容器没有恢复。只在现有OrbStack共享内核中创建一台全新轻量隔离Linux machine `yofc-console-v52` / `01M7BKVJ6SNTMPY3124ANRHNXD`，限2CPU/1536MiB/12GB，只共享本组件/proof，无全宿主/SSH agent/签发目录挂载；不是新增完整Parallels重型VM。全局OrbStack设置和旧Parallels现场不变。

候选deb实际通过apt装入标准/usr、/etc、/var/lib路径；最终版本0.1.0~v52-platform20261114-5_arm64，SHA `9a45511336b4986b99a276eb78e460bd2eac5c54b47194d61109e602be64740b`，19个实际安装payload与归档和root ownership一致。真实Cockpit以yofc-test-admin登录，先捕获Limited access下的已知access-denied，再从顶部实际开启管理权限；通过页面启动服务、下载已有安装凭据、填写发布地址。真实非sudo viewer登录后，命令执行前明确拒绝，下载保持disabled。模拟桥另列为契约证据，不替代实际认证。

默认中心原初始化 `4870795f-125e-4d9e-828b-c984dbdb0fb1`，原PID5599，同进程切换到上传证书并用自选管理员登录，原回执APPLIED。包维护只在该操作已结算和实例未配对时执行；已配置默认中心通过界面重新启动为PID7875，原操作不重发，17个私有文件与升级前逐一精确。命名实例host-b-v52原初始化 `085ff3b5-0123-48e7-8247-7a9850101a02`，PID7596，APPLIED。每库1条安装绑定、1位自选管理员、1条console.initial-setup COMMITTED审计、0 gateway tasks；任务密钥/身份/CA/数据库及角色独立，交叉原回执查询404。PostgreSQL默认PUBLIC CONNECT允许跨库连接，交叉数据表SELECT权限为false；没有修改夹具权限来强行通过错误的CONNECT拒绝假设。

页面先将实际bootstrap TLS peer的SHA与可信下载凭据核对，再上传解锁；TLS交接后用新连接及独立curl CA/hostname校验正式叶证书。CA私钥只留在组件旁的console-systemd-host-signing-v1，未挂载或上传中心；方案A外部签发/custody未改变，多人审批未实现。主机与两个中心采用不同hostname，令牌不进DOM/URL/浏览器存储；DB/管理员密码不进浏览器存储。首次配置只上传服务器私钥与公共CA，不托管CA私钥。

实际联调修复：access-denied明确解释命令未进入并提示启用主机管理权限；发布地址留空由用户明确填写且不能与主机管理同hostname；读取状态不覆盖正在输入的首次端口。原端口覆盖导致实例撞18443，失败/自动重启日志保留，只停止本批已知未配对/无操作记录实例。恢复入口仅在inactive/failed且无Job、无旧启动设置、状态恰好为已知bootstrap文件/允许空目录、原凭据验证通过时创建首次端口配置；任何session/操作/generation/未知项/符号链接均拒绝，原身份逐文件保留。revision4缩进错误使已有目录跳过端口检查，真实恢复发现后由revision5修正并加回归。所有失败发生在该实例配对/初始化前，原下载凭据保留；没有替换身份或重发任何已受理操作。

新增7项独立自动检查（3 Python+4 browser），当前26项Linux root Python和9项浏览器契约全过。首次root测试缺少完整源目录的夹具错误、错误CONNECT权限假设、浏览器观察超时/CDP旧连接证书缓存、rev4失败均保留。观察超时时独立查明尚未配对或初始化，保留服务后续接；旧连接证书缓存通过新连接观察确认，不重启服务或初始化。

**本批边界仍internal pending**：OrbStack provider的zzz-lxc-service.conf削弱NoNewPrivileges/ProtectSystem等实际沙箱，覆盖文件保持原样，不能认证完整沙箱。该环境独立域名IPv6通、IPv4机器/域名路由超时，但localhost转发通；仅测试浏览器将两个中心域名映射到现有localhost转发，未改宿主DNS，实际指纹与CA/hostname仍校验。通用IPv6/双栈尚待实现验证。Type=simple active/PID不代表监听ready，立即导航曾收到connection refused，仅重试只读导航；可靠就绪提示/轮询待实施。完整多阶段镜像、Docker选择GUI、旧包/镜像兼容升级与维护并发冻结/排他没有闭环。

两笔初始化APPLIED后关闭3个实际浏览器context，通过已验证程序的只读维护检查，只停止本批两个中心unit，再停止该新machine；全部PG、身份/配置、下载、profile、失败证据保留。3个新包构建容器均exited0/noOOM。此新夹具已CLOSED，不重放初始化；后续维护只读查原记录。原agent81713/Kura82137 active、Job空/NRestarts0、stop intent a87d94d52839e316845b7c831b265daf928f0bfd5fb0a883fb49b721cba5f63c，无stop receipt/publication/start；原767863a9-fe6b-4135-9783-ddcb131030af UNKNOWN未触碰，不stop/restart/发布/启动/激活/换ID/重导出，预算release8b07未部署。

下一步先收敛可靠就绪与IPv6、全多阶段/Docker选择GUI和完整维护排他，再按完整继承台账推进。临时身份/凭据半成品恢复续期、首次/TLS UNKNOWN核定UI、中心CSR生成/下载/外部签发上传、地址/DB凭据维护、独立agent本地身份/CSR/续期/通道/能力UI、多管理员/CA quorum、bulk/备份和正式Linux sandbox/polkit仍内部pending。不能因本批实际Linuxhost链通过，缩减完整平台范围。

完整owner/ordinary/recovery/原forward-confirm-inverse-observation-stopped-expiry、boot/netns/handles后合法restore、全family/CM/runtime/NAT true/false/多WAN/modem/DHCP/DNS/WiFi/VLAN/packet/fragment/unsupported矩阵、初始化退役先于原native inverse、子记录结算/退役/历史/重接管，原14组/7API2/R017、280整体guarded apply、正式API配对/协议crate校验和/joint generations、G021真实LDAP present/absent、G022 Go31 vet/3非Java POM/可选regen/.NET、G023逻辑上游同步全部保留。G023仍10原+7shadow/UI-util1.3、11fork7nonfork/327Maven1Cargo；Coref0b2120/UI1c51246/PLCea42游标不动，新Core7eca88b2 SSL/Crypto breaking cohort内部pending，不批量merge，不对nonfork合并。

18原仓完整工作树/未跟踪源/index/refs/remotes、1986/20/519/89、credentials141+5及共享API522仍精确。仅原3组external deferred（NM/cloud-init/物理、amd64、disk-full/hard-power），其余均internal pending。旧CLOSED/V36v1/GWT三次保存/已闭环首次与TLS验收不重放；不提交推送/reset/clean/局部apply/共享API安装。周用量18%已用、ordinary allowed，额外credits余额54476.099936不变、2枚reset未用。目标ACTIVE，完整改造OPEN。

---

以下保留前阶段历史；“真实systemd/Cockpit尚未验证”仅在上述标准包Linuxhost范围被替代，其他完整台账继续有效。

## 最新续接：默认启动入口与主机安装页面候选（2026-11-14，V52 仍 ACTIVE/OPEN）

同一 R013 shadow 当前 **2167 source +4 fixture、相对V51有79个输入路径增量、280 production/test候选全部UNAPPLIED**（118 existing +165 new含3 fixture-only）。本批在原259候选上增加21个生产/测试路径；新纳入24输入含3个未修改的原始辅助输入，不将79解释为79个新增生产改动。source-checkpoint-console-first-platform-final.json SHA `349e780d4fbf23da2bc6d3b08b8e1ab9caf4069481e7aaacc0135c89b17bcb29`；完整候选 console-first-setup-platform-v1/working-candidate-platform-v52.json；2167+4输入另外保留在该组件working-source-snapshot逐文件核对的只读交接副本。该副本不是完整CODE seal；V51仍最后完整CODE封存，G023游标不变。当前组合registered-console-first-platform-v52.json SHA `776a3f32104a39e5acd5a11c2690d29eebe816453ed0dd19b909f115ed3b5f17`，组件绑定console-first-setup-platform-v1/final-binding.json SHA `4d81cd6145f501a724a71ebc7327a910e9073aadc83de0766f15f11fff88b0a4`。

systemd默认/实例unit已去掉mandatory console.env门槛，使用可选高级环境文件、私有StateDirectory和只查负载/权限的launch-check。Docker默认只有独立状态卷，不要求DB/key/origin/TLS环境值或证书挂载，也不配置弱口令PG；已有部署的3项分离只读TLS输入保留于显式compose-legacy.yml。入口降权后直接serve，不再提前key-init/migrate。空白环境值视为未配置、非空不裁剪，非UTF8失败时不输出原值。安装等待live200/ready503；正常live200、ready继续查DB。正式发行/全多阶段和环境兼容尚未闭环。

新增双语Cockpit主机安装页：选择默认/命名实例及首次端口、显式启动/观察服务、读取可信指纹并下载已有私有安装凭据；页面通过已认证主机通道申请管理员权限，无低权限/公开HTTP令牌后备。新实例目录只在不存在时创建，端口只在无旧设置/身份时私有create-only；active服务不重启，有Job/变化中的服务拒绝start。目标变化清除页面内凭据，令牌不写DOM/URL/浏览器存储/日志。已配置安装拒绝导出。方案A外部CA/custody不变，未新增CA私钥托管或多人审批。

审查发现旧程序可能先进入DB启动路径再报告未知命令，**未调用旧程序探测**。包/镜像现在生成root保护的local-command-provenance；native helper先验证程序/source-manifest摘要和声明能力，Docker固定本机socket并先校验root所有权/权限、实际程序摘要与能力，再以console身份执行固定只读命令。旧制品无记录即在调用应用前拒绝，没有未知命令fallback。尚须统一全CLI在DB前校验参数；当前修复只证明安装器这条路径不以未知命令探测。维护前只读检查未决首次/TLS原回执、数据库安装绑定和任务，再请求stop并保留停后检查；并发冻结/排他和完整升级事务仍内部pending。

新增24项自动检查：5Rust、14Python、5主机页面适配契约；Python连同9项相关既有检查共23项Linux root环境全过，首次配置既有11Rust受影响回归通过，端口UI后3项受影响契约通过。契约桥为明确测试适配器，**未认证真实Cockpit登录/提权**。新增实际Ubuntu容器使用普通ARM64 opt3/non-test release，最终SHA `91e56aef1f1adcf0bc5c06236af5c83535752e1390fd75436ab9dceeb1d76805`；UID999/CapEff0，缺DB/key/TLS变量仍可提供设置页，live200/ready503，其他UID读凭据被拒。最终源码/静态provenance镜像接纳均复用原5文件/私钥/凭据，未重生成或发任何初始化/迁移；实际浏览器只读页面0写请求，实际Docker适配器先查provenance后返回同一凭据。镜像构建仅实际验证运行阶段、正常release和既有前端，不冒称全多阶段验收。

候选ARM64 deb `console-first-setup-platform-v1/deb-v2/package/kura-management-console_0.1.0~v52-platform20261114-2_arm64.deb` SHA `8a5c5edd0c09bd61c029f214184a0582f5cee00312e05ddb4341e393f5c1ff86` 已产出，实际ELF最大GLIBC2.34，包声明Ubuntu24.04 floor；19个payload文件的root ownership、程序/源码摘要、主机UI/助手/unit与当前源码核对。**未安装deb、未认证真实systemd/Cockpit/完整GUI部署**。编译初始fs/tower引用问题已修复，失败日志保留；旧包无provenance及维护竞争均按内部兼容/恢复待办保留。10个本批新构建/测试容器全部terminal，无OOM，状态卷/数据/失败证据保留。没有恢复用户清理的旧中心/PG/60容器，也没有新VM。

下一步先只读审查真实Linux旧包/单位/保留状态，选择不覆盖旧夹具的真实systemd/Cockpit验收位置，再完成默认/实例启动、主机认证/提权/凭据下载与完整GUI链。不能用模拟桥、改路径的自定义负载或窄检查冒称标准安装完成。继续多阶段镜像、旧包/镜像升级/维护并发、OS/Docker Desktop交付适配；临时身份/凭据半成品恢复与续期、首次/TLS UNKNOWN核定UI、中心CSR下载/外部签发上传、地址/DB凭据维护、独立agent本地身份/CSR/续期/通道UI、多管理员/bulk/备份及正式沙箱均内部pending。

完整原owner/ordinary/recovery/原forward-confirm-inverse-observation-stopped-expiry、boot/netns/handle后合法restore、全CM/runtime/NAT双态/多WAN/modem/DHCP/DNS/WiFi/VLAN/packet/fragment/unsupported矩阵，初始化退役先于原native inverse、子记录收敛/退役/历史/重接管，原14组/7API2/R017、280整体guarded apply、正式API配对/协议crate校验和/joint generations、G021真实LDAP present/absent、G022 Go31 vet/3非Java POM/可选regen/.NET、G023逻辑上游同步均未缩减。G023仍10原+7shadow/UI-util1.3/11fork7nonfork/327Maven1Cargo，Coref0b2120/UI1c51246/PLCea42游标不动，新Core7eca88b2 SSL/Crypto breaking cohort内部pending，不批量merge/merge非fork。

18原仓完整工作树/未跟踪源/index/refs/remotes、1986/20/519/89、credentials141+5及共享API522继续精确。原Linux只读仍agent81713/Kura82137 active，Job空/NRestarts0；原stop intent a87d94d52839e316845b7c831b265daf928f0bfd5fb0a883fb49b721cba5f63c，无stop receipt/publication/start。原767863a9-fe6b-4135-9783-ddcb131030af UNKNOWN保护，不stop/restart/发布/启动/激活、不换ID/重导出；预算release8b07未部署。仅原3组external deferred（NM/cloud-init/物理、amd64、disk-full/hard-power），其余全部internal pending。旧CLOSED/V36v1/GWT三次保存/两次已闭环初始化不重放；未提交推送、reset/clean、局部apply或共享API安装。周额度16%已用、ordinary allowed，无额外credits/reset credit消耗。目标ACTIVE，完整改造OPEN。

---

以下保留前阶段历史；旧计数和“入口仍全未改写”只在上述实现范围被替代，完整其余台账继续有效。

## 最新续接：中心首次配置页面应用内闭环（2026-11-14，V52 仍 ACTIVE/OPEN）

同一 R013 shadow 当前 **2143 source + 4 fixture、相对 V51 55 路径增量、259 production/test 候选全部 UNAPPLIED**。259 = 原247 + 本批新增12个应用路径，另3个 fixture-only 差异不应用；完整候选 console-first-setup-ui-v1/working-candidate-initial-setup-v52.json（108 existing +154 new，含3 fixture-only）。最新源码 source-checkpoint-console-initial-setup-final.json，SHA `70f26463e034fe159759ce49cd1231b22ef59fb8156283d8e7678a647639b943`；组合 registered-console-initial-setup-v52.json，SHA `fc210b8569a5fcddf7c1a94db939b540f9ecdaa4fae20b9066908675a81f5ac9`；本批应用范围绑定 console-first-setup-ui-v1/final-binding.json。V51 仍最后完整 CODE seal，G023 游标不变，当前源码未整体封存/应用，不可用旧247/223/218清单局部apply。

新 `serve` 在无持久配置/无完整旧环境配置时先提供 HTTPS 首次设置页，不连接数据库或创建默认管理员。安装凭据私有0600、临时身份只是服务器终端证书；原生 ring 生成私钥，OpenSSL仅签临时证书。界面上传安装凭据并核对可信安装器指纹，填写地址/数据库/组织管理员，选择服务器证书/私钥和 public client CA，经复核提交。只接受独立空schema；持久保存受摘要保护的不可覆盖 generation、数据库安装绑定、active指针和原回执，同一进程接管监听器进入正常HTTPS/gRPC，无需手工重启。方案A外部CA/custody不变；没有CA私钥托管/多人审批实现。

界面先保存公开原ID再发一次请求；服务端先持久accepted、再独立后台执行，只有实际活着的任务句柄可报PENDING。失去句柄/缺回执为UNKNOWN，原记录阻挡换ID绕过；刷新只查询原ID。已配置启动在迁移前验证数据库安装绑定和有效管理员。无active但已有首次accepted/intent/receipt时，共用连接入口拒绝legacy fallback，完整旧环境变量也不能令自动migrate绕过原UNKNOWN。安装/Origin/HTTP2 authority/CSRF、私有路径/材料、TLS用途/主机名以及state独占租约均受校验。

本批18种新检查通过：11 Rust、独立已有业务表库拒绝、错误目标库在DDL前拒绝、2真实中英文首次配置、配置持久性、实际第二进程拒绝、普通二进制legacy migrate故障journal保护。中英文原ID分别0cadf1b7-8040-481a-b501-53ee9e05d996（PID1565）、27523ffd-f911-4d0d-8638-f701b2ce8d15（PID1794），每套同进程切换、唯一自定义管理员/安装审计/绑定、没有默认admin，且没有传DB/key/origin/TLS环境变量；两次初始化已闭环，不重放。最终 macOS ARM64 普通non-test debug SHA f79a03e64bdbbac5c00bdea5511959cb6365a351a5d98634d506b20be188c2b9，以PID3207在已配置安装仅做新源码只读接纳：16文件及账号/安装/crypto/恰一条COMMITTED初始化审计摘要不变，无再次初始化/密钥生成。

失败证据完整保留，包括OpenSSL/LibreSSL测试兼容、SQLx夹具构造、驱动在动作前拒绝旧场景、严格Python验证器对旧TEST ONLY证书缺AKI的拒绝，以及最后观测脚本用户/字段错误（均在中心启动前）。没有放宽全局TLS验证；实际curl/OpenSSL验证CA和主机名通过，严格测试证书互操作边界仍未关闭。所有本批新中心已停，PG c6d7c688cfd31b3b2bea7b0d8befd10ec080ac23f7df92e21af1810279e25c15 exited，数据/volume保留；旧TLS中心闭环和已清理旧中心/60容器不重建。

**下一步优先接通标准发行首次启动**：systemd的console.env条件/mandatory EnvironmentFile/旧setup.py check，Docker先key-init/migrate/prepare_tls及mandatory TLS/key环境值尚未改写。需要同一shadow整合安装器/systemd/Docker，可信平台UI交付安装凭据，统一空环境覆盖语义、安装等待的liveness/readiness、state权限；不能让操作者为首次配置手动设置变量、运行脚本或从终端读取秘密。当前仅应用内闭环，不冒称完整GUI部署。半成品临时身份/凭据恢复、临时证书续期、首次UNKNOWN核定界面、中心CSR下载/外部签发上传、已配置地址/DB凭据维护、独立agent本地身份/CSR/续期/通道能力界面、组织多管理员/备份/bulk和正式Linux包/沙箱均继续内部pending。

完整原owner/ordinary/recovery/原forward-confirm-inverse-observation-stopped-expiry、boot/netns/handle后合法restore、CM/runtime与NAT双态/多WAN/modem/DHCP/DNS/WiFi/VLAN/packet/fragment/unsupported矩阵，初始化必须先退役才能原native inverse、子记录收敛/退役/历史/重接管，原14组/7API2/R017、259整体guarded apply、正式API配对、协议crate校验和/跨组件joint generations、G021真实LDAP present/absent、G022 Go31 vet/3非Java POM/可选regen/.NET和G023逻辑上游同步均保留。G023仍10原+7shadow/UI-util1.3/11fork7nonfork/327Maven1Cargo，Core f0b2120/UI1c51246/PLCea42游标不动，新Core7eca88b2 SSL/Crypto breaking cohort为内部pending，不批量merge、不merge非fork。

18原仓完整工作树/index/refs/remotes/未跟踪源、1986/20/519/89和共享API522继续精确。原Linux只读仍agent81713/Kura82137 active、Job空/NRestarts0；原stop intent a87d94d52839e316845b7c831b265daf928f0bfd5fb0a883fb49b721cba5f63c，无stop receipt/publication/start。原767863a9-fe6b-4135-9783-ddcb131030af仍UNKNOWN，不stop/restart/publish/start/activate，不换ID/重导出；预算release8b07仍未部署。仅原3组external deferred（真实NM/cloud-init/物理、amd64、disk-full/hard-power），其余全为internal pending。未提交推送、reset/clean、局部apply或共享API安装；V36v1/旧CLOSED/GWT三次保存/两次新初始化均不重放。周额度16%已用，ordinary allowed，无额外credits/reset credit消耗。目标保持active。

---

以下保留前阶段历史。旧计数、下一步及“首次配置向导尚未实施”的表述仅在上述应用内有限范围被替代；完整其余台账继续有效。

## 最新续接：中心 TLS/信任配置界面生效（2026-11-14，V52 仍 ACTIVE/OPEN）

同一 R013 shadow 当前 **2134 source + 4 fixture、相对 V51 43 路径增量、247 production/test 候选全部 UNAPPLIED**。247 = 原223 + 本批新增24个应用路径，另3个 fixture-only 差异不应用；完整逐路径候选见 console-tls-live-ui-v1/working-candidate-console-tls-v52.json（105 existing +145 new，含3 fixture-only）。最新精确源码 source-checkpoint-console-tls-ui-final.json；组合 registered-console-tls-ui-v52.json，SHA `3e6ddc4d0397c9af6a8f065efd44568e719d9cff237d96003ec8da711493678a`。V51 仍最后完整 CODE seal，G023 游标/上游边界不变，不能用旧223/218候选清单局部应用当前源码。

中心设置页现在可选择文件、核对指纹、暂存服务器证书/私钥与 public client CA，再联合生效 HTTPS 和 gRPC/mTLS；该维护操作不要求环境变量、脚本或手工重启。启动仅采用联合 active 指针，不把 pending 当批准。实际握手 peer identity 与 epoch 随连接传入 gRPC，生效关闭旧握手/流并取消旧流工作。独占 state 租约、材料完整摘要/私钥匹配/有效期/服务器 origin 主机名校验均在原意图前执行；bootstrap/旧代保留。方案A外部CA与私钥 custody 不变，没有 CA 私钥上传、托管或多人审批的新增实现。

UI 先保存原 operationId 后发一次 POST，原回执/执行前拒绝/UNKNOWN 分开；重复同ID只观察，改请求拒绝。缺回执、404、丢响应不会重发；当前配置可观察不补造回执。服务端阻挡后续暂存/生效，另一浏览器或管理员也不能换ID绕过 UNKNOWN；另一浏览器恢复原ID并查询。联合指针发布不确定会停止网关通道而保留认证查询。TLS UNKNOWN 的明确人工核定/恢复界面仍内部pending，不能直接删记录放行。

本批新增19项不同检查通过：9 Rust、2独立真实PG/HTTPS/Tonic、7双语浏览器契约、1真实新中心浏览器闭环；最后仅修正生效观察后的暂存提示刷新，保留 live-accepted-frontend 后构建当前前端并通过2项受影响双语契约，没有重放真实生效。普通 macOS ARM64 non-test debug SHA `b44c44ff495afd5f2a8b7348bd7be0b09ebd0d2ff0bf0b1c8726c13bc174c3fd`。真实独立中心唯一生效ID `a93db1f3-430d-496f-aa0a-09f65b629e8f`，原回执/联合指针/REQUESTED审计1条以及实际新服务器证书SHA均核对；该闭环不重放。初始编译、测试socket blocking和浏览器“用户名/账号”定位错误均保留；后者确认无暂存/意图/证书审计后才修正进行首次动作。不是正式Linux安装/沙箱/发行验收。

新测试中心与开发Vite已停，专属PG容器 e67b5467362f1c97307c5bf8d5ee941289bb54bfe437c5b0155a17343d3c54bc exited；新数据库/volume/本地失败与成功证据全部保留。没有恢复用户已清理的旧中心/PG/60容器，没有增VM。18原仓完整工作树/index/refs/remotes/未跟踪源、1986/20/519/89及共享API522指纹继续精确。原网关只读核对仍 agent81713/Kura82137 active、Job空/NRestarts0，原stop intent a87d94...c仍在、无stop receipt/publication/start。没有操作保护UNKNOWN现场；stop-budget Linux release8b07...仍未部署。

完整改造未完成：首次安装/中心首次配置向导、独立agent本地配置与证书续期、bulk、维护备份/恢复、正式Linux组合继续内部pending；完整 owner/ordinary/recovery/原forward-confirm-inverse-observation-stopped-expiry、真实合法restore、全部网络/CM/runtime矩阵、初始化/子记录退役与历史、14组/7API2/R017、正式API配对、协议crate校验和、跨组件joint generations、G021/G022/G023新SSL/Crypto逻辑合并均保留。仅原3组external deferred。下一步按上述部署/证书GUI继续，原UNKNOWN不动，不重放旧CLOSED、V36v1或GWT三次保存；不提交推送、reset/clean、局部apply或共享API安装。周额度当前15%已用，ordinary allowed，未用reset credit/额外credits。

---

以下为前阶段历史，旧计数、下一步与“TLS仍全未实施”的表述在上述有限范围内被本增量替代；其他完整待办继续有效。

# 最新续接：V52 ACTIVE / OPEN，与V51/G023

当前最新代码与证据（2026-11-14，覆盖下文旧计数/待构建说法）：source-checkpoint-startup-stop-budget-final.json为2115 source+4 fixture、相对V51有13个路径增量、223候选全部UNAPPLIED。当前组合registered-linux-startup-stop-budget-v52.json，SHA61ebf995c1feb02f52e50805ea6709463a673a0ebee793999bf167bf66da1ce4；V51仍最后完整CODE seal，G023游标未变。live仍为普通debug ELF affa64e2e164e5eabb6d02938d8d8f1f1e753771854e04aab1c469000f1fdc5f，绑定source-checkpoint-startup-launch-v3-final.json；不能把本次新源码/release当作已部署。

新增停止预算修复：coordinator现把原60秒deadline一直传入stop.arm；完整profile/unit/stop-policy重检结束并验证execution身份后，在publish_new(stop intent)之前检查剩余预算。已过期时返回稳定TIMEOUT，停止意图、停止回执、业务journal/参数/profile均不改变；不放宽pin/hash/actor/epoch/marker/cgroup条件，也不重解释已有UNKNOWN。新增“入口已过期”和“重检中耗尽”两回归，在等价旧语义（传入但忽略deadline）均明确红；修复后2绿。正常一次stop/历史schema3/旧UNKNOWN不可重发/有效policy与原控制器、execution lease共15项受影响回归全绿。source-checkpoint-startup-stop-budget.json只记录构建前阶段，以-final为最新。

正常ARM64 Linux release已从当前2115+4精确源码构建，SHA8b07fa7ea2d96832c27e6392dcdec2483eb70832c8c7efde122ba75649c54035，实际Cargo opt_level=3、非test/非诊断，ELF64 e_machine183。独立只读release probe为另一SHA，绝不能安装为daemon。两次build均terminal0；正常release与配套probe共用Docker构建容器415e6f683d3114b79220df62e9b999ee38251b994c31730c3c3627dc5d439d9c限制2CPU/2GiB/network none，无OOM、未增VM。正常release仅产出，未替换保护UNKNOWN的agent。

同一真实VM、原profile295文件与原startup proposal的同一digest9cf65d8bf25df119efe300f9f6907f31b9e091cfcbe5c94fe85333b22d6b835a：旧debug逻辑单次20487ms，新release逻辑单次1754ms；release child CPU约1.75s。两个独立CLI只读实际profile，不启动daemon/替代服务/owner，原业务记录SHA均未变，实际agent/Kura仍81713/82137。仅两次单样本、未清cache；不冒称冷启动性能或整轮通过。Profile/controlled_startup/durable三份validator依赖与live源码相同。该证据支持重复校验耗尽60秒预算的原因路径；原停止动作仍UNKNOWN，不能从耗时推断从未发送。

当前原场景最终只读API：parent FENCED、native APPLIED、初始化COMMITTED、startup STOP_OUTCOME_UNKNOWN；两进程active/Job空/NRestarts0，stop intent a87d94d52839e316845b7c831b265daf928f0bfd5fb0a883fb49b721cba5f63c，stop receipt/publication/start均不存在。未再stop，未publish/start/complete/activate，未重导出/fence/native/init。既有file recovery要求已停unit、原stop receipt、原publication intent；本场景不具备，不能调用它或造回执。合法原记录恢复审查继续内部pending，不能把新增代码/正常release当作重发旧stop的理由。

本次预算代码、15项单元回归与单样本只读性能比较已按各自有限范围闭环，未变更时不重跑。完整owner/ordinary/recovery/原forward-confirm-inverse-observation-stopped-expiry、初始化和子记录退役历史、全矩阵、14组/7API2/R017、223整体guarded apply、GUI部署/证书下载上传/TLS/bulk/正式协议crate/joint generations/G021/G022及新SSL/Crypto上游cohort仍保留；仅原3组external deferred。下一步保留UNKNOWN及同一两进程，继续合法恢复与独立完整台账，尤其产品部署/证书GUI；不手工stop、不通过新ID或新导出绕过旧UNKNOWN，不提交推送、reset/clean、局部apply、共享API安装。旧GWT三次保存、旧CLOSED及V36v1限制不变。

---

以下保留预算修复前的阶段历史，当前以前述最新代码/组合证据为准。

当前权威（2026-11-14，V52 ACTIVE/OPEN）：同一R013 shadow现为2115 source+4 fixture，相对V51有11个路径增量，223候选路径全部UNAPPLIED；source-checkpoint-startup-v3-status-reservations.json是当前精确源码清单，live ELF另绑定前一source-checkpoint-startup-launch-v3-final.json。V51仍是最后完整CODE封存，G023游标未变。真实Linux当前普通agent PID81713（ELF affa64e2e164e5eabb6d02938d8d8f1f1e753771854e04aab1c469000f1fdc5f）、Kura UID994/PID82137，bus/NM/proxy仍2799/3635/59158。旧PID、2112/220、“未export/全停”等仅为历史。

本轮原操作767863a9-fe6b-4135-9783-ddcb131030af已得到一次加密导出PREPARED、paired fence FENCED、真实Java drain后的native APPLIED、realStore初始化COMMITTED回执（revision1）。第一次startup.prepare/verify分别CONFLICT且各自无proposal/关联回执，已独立核实已知未执行后继续同一原操作：v3 proposal建立、实际旧JVM与unit/profile关联OLD_JVM_ASSOCIATED。随后且仅随后一次startup.stop返回OUTCOME_UNKNOWN；已有stop intent SHAa87d94d52839e316845b7c831b265daf928f0bfd5fb0a883fb49b721cba5f63c、没有stop receipt。只读核对Kura82137与普通agent81713仍active、旧PID存在、Job为空、NRestarts0。不能把这种观测倒推为“从未发送”；UNKNOWN永不重发停止，不通过新ID、手工stop或重导出绕过。publish/start/complete/activate辅助脚本只已编写，未执行；owner/ordinary/recovery及后续全整轮均未完成。

当前组合权威registered-linux-startup-stop-unknown-v52.json（SHA1b4d18a137fe104f78c9f57a048131bbd1f04aba36b4e78d689f5ee9ca9845a7）。本地source-checkpoint-startup-v3-status-reservations.json为最新2115+4/11增量/223未apply；live普通ELF仍来自source-checkpoint-startup-launch-v3-final.json，不能混用。新v3状态requiredReservedPids遗漏两个Firewall PID已修正，新断言在旧实现明确红；当前生成合同/历史schema4字节保留/原查询不重做三项受影响测试全绿。仅shadow更新，未升级正在保留UNKNOWN现场的daemon。

停止步骤的候选原因仍须审查：同一VM内，stop intent在客户端原dispatch后67.420879秒才生成，而coordinator入口预算60秒；arm后还有使用原deadline的注册检查。该时间关系支持“完整295文件多次校验耗尽预算”假设，但尚不是已证明根因，也不得据此改写UNKNOWN或造stop receipt。下一步仅原记录只读检查、真实profile校验成本/正常debug与release差异、意图发布前预算门禁和合法原记录恢复审查；独立剩余台账可以继续。此项是内部pending，不新增external deferred。原初始化必须先退役才能native inverse，不能现在绕过子记录/UNKNOWN。

启动前置已实际完成：真实NetworkStatus RecoveryHealthCheck新增并通过30项相关Java测试，语义仅“状态可观测”，不认证连通性/业务健康。启动参数v3把JVM owner属性放在-jar/主类前、保留应用参数，v1/v2历史字节不变；同一生产实现累计50项相关Rust测试通过、1项ignored，最后一项只修正过期测试版本预期。正常Linux构建terminal0、没有诊断注入。已封存实际ordinary profile profile-cd71ebaf-8286-41bf-b2b4-b6021d5789e7，grant SHA7ee5c6e5f14ded5dd4648d0ece7123f5b044b9de80e6902ecf318f4bc607ed38，295个真实文件精确。

已完成观测包退役、原observer autostart=false、load目录空；实际VM flags读取一致。安装辅助脚本的SIGTERM143、Parallels255未进入、umask导致父目录0700及后续可读性断言范围过宽等失败全部原样保留；独立读回证明修正后291运行文件可由Kura UID读取、295文件字节和grant不变。Equinox曾以旧Manifest元数据加载新磁盘JAR，导致健康服务0；真实SDK update/refresh只涉及bundle281 bridge与279 firewall-agent，同一JVM/agent继续，真实provider版本1.0.0.202611131926、实际状态检查通过。初始INTERNAL生命周期根因尚未单独证明，不因当前来源恢复而关闭。

首次导出前普通ELF无tracer读回完整source，network revision16d9a33c29019719c1c6618d584c0ace06e74ab1acf3c7af608c231bd06c2490、sourceDigest7fa33800ccb68f3a8523f605163c7df78c8aa20c3dcf14d385001426722ae1dc、115条原规则；真实Crypto custom-key能力与Core逻辑读通过。global nft/iptables/ip6tables为空、原两份Netplan YAML及旧5快照精确。它们是导出前证据，后续native动作仅在原专属netns，不能继续宣称native动作0。该前置封存在registered-linux-controlled-startup-v52.json；当前实际阶段必须结合linux-validation/first-current-{export,paired-fence,native-apply,initialization-apply,startup-prepare}-result.json及current-startup-conflict-original-state.json。

下一步：保留原UNKNOWN stop、同一原操作及两进程，不重发stop，不发布/启动/激活。先完成上述预算/实际运行成本和合法原记录恢复审查，并继续独立内部pending。完整台账与223整体guarded apply。初始化必须先独立退役才能做原native inverse。原14组/7API2/R017、合法restore、全CM/runtime/网络矩阵、子记录退役/历史、GUI部署/证书下载上传/TLS/bulk/正式协议crate/joint generations、G021/G022、新SSL/Crypto上游cohort都仍内部pending。仅原3组external deferred，方案A外部CA/custody不变。不提交推送、reset/clean、局部apply或共享API安装；UNKNOWN不重发，V36v1不查询/反向/重发，已闭环GWT三次保存和旧验收不回放。

---

以下保留各阶段历史；与上述当前权威冲突的计数、PID及动作状态仅适用于原阶段。

先全文读 registered-runtime-review-20261114.md，以及 target/nm-role-authority-20261112/registered-runtime-v52/current-state.json、progress-checkpoint.json、source-checkpoint-after-ipv4-family-fix.json、remaining-implementation-v52.md及当前证据。V51仍是最后完整代码封存：owned-task-ui-flow-v51/final-binding.json及phase2-source-manifest.json、working-candidate-phase2-v51.json。同一shadow2112+4、220UNAPPLIED，相对V51有4个路径增量。G023上游封存与游标不变。以下旧计数、旧PID及“全停/源码未变”仅作对应阶段历史，当前以最新Linux段及current-state为准。

V52已在完整真实宿主运行集启动普通agent/Core/bridge/配置版本服务并注册，实际读取Clock版本，Container配套消费者resolved，0框架解析错误；原GWT实际登录已成功。首次缺codec与旧Container制品已定位并用精确companion/独立private缓存重打当前Container修正，78测试通过。private configuration实际导出1.9.0；历史1.8说法仅作历史。没有伪服务/seeded CM/owner，业务任务0，私有进程全部停止且数据保留。Status截图有加载遮罩，仅证明登录/首屏；不能宣称全页面交互、IDEA GUI启动或Linux NM/native整轮通过。原失败与v5 supervisor失败报告原样保留，composite绑定同一v5进程的真实Core与v6浏览器成功。

下一步继续V52实际Linux注册整轮：先审查旧自动启动/未决现场，创建新的受控当前cohort，source→一次export→paired fences→Java drain→native/realStore→startup/Core owner→ordinary boot/recovery→原forward/confirm/inverse/observation/stopped/expiry，再218整体guarded apply/部署收尾。完整V51/V50/V49/V48台账、原14组/7API2/R017/restore/矩阵/退役/GUI部署证书TLS/bulk/protocol crate/G021/G022/上游新SSL-Crypto兼容cohort继续；只有原3组external deferred，方案A外部CA/custody不变。

保留18仓、同一shadow、旧seal/失败/现场。不提交推送、reset/clean、局部apply、共享API安装。UNKNOWN不重发/换ID/重导出，V36v1不做live query/inverse/reissue，旧CLOSED不回放，本轮已完成的真实宿主登录不重跑。用户确认的Docker/Orb清理使旧运行健康仅作历史。目标active；实际周额度100%才必要收尾暂停，不动重置credits。


2026-11-14 V52 Linux续接：当前已真实注册普通Linux agent/Core（Kura UID994），修复shadow NM DS缺kura.service.pid并真实读取network/v4/v6三配置版本。最后完整CODE封存仍V51；当前2112+4仅1 XML增量，218未apply，新source-checkpoint-after-nm-pid-fix.json。请全文读registered-runtime-review-20261114.md最新Linux段及registered-linux-composite-v52.json/live-linux-checkpoint.json，保留当前VM及PID2799/3635/4679/50463；不能按历史“全停/源码未变”处理。preparation只读INTERNAL、初始CM角色空，继续真实配置和整轮；未export/fence/drain/native-transfer/realStore init。三setup脚本有部分失败/续接，禁止整脚本重跑；不重放旧验收、不commit/push/apply、UNKNOWN不重发、V36v1不查询。完整台账与仅原3组external deferred不变。

2026-11-14 最新续接：同一JVM50463/普通agent58322保留，实际GWT已确认LAN+DHCP WAN+NAT Only共3次保存，均不能重做。双栈元数据重复ID已修复，15测试及真实SDK/Core/普通agent准备来源通过；旧INTERNAL正常重启后消失但生命周期根因仍pending。NAT true实际内核规则与双族115条raw匹配，preparation因误将IPv4 NAT矩阵要求到IPv6而CONFLICT。该Rust判断已在shadow修正，3项新回归及6项相关回归均过；正常Linux构建/真实来源读回继续，不修改CM来绕过。请使用当前checkpoint与linux-validation证据，不把旧composite当作当前完整结论。2112+4、4路径增量、220未apply；完整原台账、3组external deferred及UNKNOWN/V36v1限制保持。

再续：当前固定正常ELF已构建terminal0并运行，实际agent PID73027/SHA93d9cd5d6c797065bde1b13d8deca441e410ab84e5084a29bcd6de3e76b8dd0d、JVM仍50463；NAT true来源已通过，revision及writer模型保持原值，115条规则精确。最新registered-linux-configured-source-v52.json绑定本次证据。激活辅助脚本的立即exe断言失败原样保留，后续独立只读核验确认同一PID实际ELF、Core和来源；不重跑激活脚本。下一步先补真实startup合同并退役/保留一次性观测，再首次导出和全整轮；private-agent-etc为空、原观测@5:start仍在，未配置前不能冒称已具备重启接管条件。无export/fence/drain/native-transfer/realStore init，完整台账未删减。
