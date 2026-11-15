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

# G034 / V62 WIP：已绑定 agent 身份续期后端

本阶段仍在实施；没有组件完成或完整交付声明。最后已验证组件是 G033/V61。当前变化仅在隔离工作副本的 agent，原18仓源码未应用，不提交推送，不重放旧一次性业务验收，不重发 UNKNOWN。

已加入独立续期日志、v2凭据回执及旧代际链，首次绑定仍拒绝覆盖已有身份。续期要求固定Kura目录授权、当前身份摘要/远程版本一致、通道关闭且无执行中的远程请求，并拒绝 QUEUED/RUNNING/UNKNOWN 写任务。新证书由固定授权目录复制并独立验证，网关ID和中心地址保持一致，代际只递增1；旧候选、私钥及公开回执保留。凭据指针和关闭决策提交后才发布原操作结果；配置watch在释放远程准入锁前更新，避免并发开启使用旧配置。成功后仍OFF，必须另行明确开启。

原操作结果独立查询，不依赖当前指针或指纹。连续续期后旧结果保持原值；缺结果、损坏历史或不完整意图保留UNKNOWN并在重启后阻止准入，不自动恢复写入。明确的准入拒绝有独立公开回执，保留原开关和已接受工作；拒绝回执丢失仍UNKNOWN，不回填。读取公开结果与修改凭据的权限分开，撤回源目录授权后管理员仍能读取原结果。

过期历史身份的装载现在按原接受时刻检查签名、用途、链及私钥/CSR匹配，避免把单纯过期当成身份损坏而阻止续期。新暂存材料仍按当前时间严格校验，过期替换被拒绝。实际测试使用已过期的证书，并明确构造已在过去有效接受的测试安装，不代表当前首次绑定可接受过期证书。现有中心WebPkiClientVerifier和agent实时mTLS配置未修改；本阶段没有验收真实续期后的mTLS连接。

已接入可选本地方法 remote.renewal-status / remote.renew-identity。注册Kura调用者须有通道控制授权，凭据修改另须有固定源目录授权；原首次绑定合同不变。提交后异常只返回UNKNOWN并查询原操作，不能隐式重试。中心确认字段目前由已认证本地操作者提供，只是公开确认数据，不是密码学签名证明；后续界面服务仍须校验上传回执的网关、中心、旧/新指纹及代际。当前中心尚未提供所需的完整代际回执，不能按完成流程实际部署。

当前编译与选定回归：身份22项、通道13项、本地协议42项，共77项通过。含10个持久化边界中断、连续两次续期、过期历史身份/过期新材料、明确拒绝与原结果缺失、配置watch、合成未完成写任务行和权限/严格参数。中断是函数内注入，合成任务行没有执行器投递；不是掉电、真实网络故障、新方法的实际Java/UDS端到端或浏览器验收。未运行整个agent 749项套件，未构建新Linux发行包。

源码 `/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-identity-renewal-v2/source-checkpoint-identity-renewal-v62-wip.json`，SHA256 `8ef267aa86552043667062d26202040ff1f69cd282df7bcb3c4580c477440679`：2211源码+4夹具，较V51差量144，127 existing+209 new含3 fixture-only，共333条生产/测试候选UNAPPLIED。9个变化输入，其中4个新文件。只复制并绑定9个变化文件，其余使用V61已有只读快照，避免重复复制全量工作空间；检查点明确标记WIP、未封存、未应用。当前输入和原仓保留核对均须按配套验证记录读取。

下一步必须继续：中心证书轮换的原回执增加明确的旧/新绑定代际及界面公开下载；Java公共API与bridge的可选一次请求接口和严格结果投影；原GWT的回执文件上传/校验、续期明确确认、发送前保存原UUID及只读恢复查询。服务器端校验回执归属和操作者授权，旧provider/agent不支持时保持明确不支持，不从当前状态推断原结果。然后以新隔离现场完成实际Java/UDS、浏览器、HTTPS/PG和新身份mTLS闭环，再构建当前agent/中心/GWT/installer完整发行组合。旧绑定首操作在后续续期后的独立历史读取、legacy/半凭据迁移、权限矩阵与恢复范围仍要审查。

完整台账 `/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/remaining-implementation-v52.md` 继续保留：GUI全平台交付、中心地址/DB/CSR续期、CA多管理员quorum/bulk、跨版本DB/非空加密任务/备份恢复、sandbox/polkit/protocol0、owner/ordinary/recovery、网络/retire/history全矩阵、14 groups/7 API2/R017/全部333 guarded apply与联合代际，以及G021 LDAP、G022 Go/vet/POM/生成/.NET。并行maintenance锁忙、原Ubuntu VM已停止后的只读核对继续内部pending；外部deferred仍仅原三组。原VM未被本轮启动或停止，旧startup.stop UNKNOWN不从停机状态结算。
