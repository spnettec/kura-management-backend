# 2026-11-18 能力宣告与原生状态任务

目标 active，现有积分继续，无推送。修正默认空接口范围仍宣告 Netplan 写能力的问题，并在同一 E 保留原身份/授权/地址配置完成 source 升级、恢复连接和一次真实状态任务。完整12父项仍0/12最终闭环；此数字不能解释为实现0%，也没有可靠加权整体百分比。

## 代码与验证

实际中心前次 Hello 15项中含 network.prepare/confirm/rollback/takeover.prepare，而 E 原环境 `AGENT_NETWORK_INTERFACES=` 为空。执行门禁仍拒绝写入，清单却可能令中心显示不可用操作。

agent `5f95a7ec7973f3e590e78b1f281ab81b7acbae70` 抽取 Hello 清单并复用后端固定 allowlist：无后端不宣告 network.*；有后端/空范围仅保留 inspect、transaction、takeover.plan；非空范围保留四个既有写协议名。`24552f60c815bbf990feeceaff53ad2cd2214688` 修正测试使用未引入的 tempfile；没有添加依赖或改变执行授权。仅此 Netplan 宣告范围，不代表所有 provider/readiness 已审计。

当前 Git 干净自包含导出：transport 18通过、5个显式 mTLS/系统故障夹具 ignored，network35通过/0 ignored，含4个新能力清单场景；忽略不算通过。Python/Root 源码与已验 `3e28a44` 逐字节无 diff，既有266 selected/264通过/2历史导出夹具排除的适用证据保留，本轮未重跑、不累计。release/四控制脚本构建通过。协议本次从重新核验的声明 Git1c5842a检出重新打包，与当前协议/workspace/lock一致，SHA61163caf…e70c2，未正式发布；bridge只读当前Root配对E，无旧ELF/crate输入。

| arm64 `0.1.0~native202611171836.24552f6-1` | SHA-256 |
| --- | --- |
| deb | `239eac991d5444f64ab3053c7f1babe8a6a7d4ce15af79b07358b1226a41f0fc` |
| ELF | `03b87f800e9f42091531048bdf913b54ee490739926961a9b0e5e4198227baa9` |
| source manifest | `44a7249847e39c38731af0dc4dd65c057f572a66c18a124aad813ff55be2ead9` |

## 原现场维护与恢复

同 E CIDa60111a6…94414，原 init24560864…、peer25abd835…、material13d50bed…、switch49e58164…、transport2031e3d3…、CSR7c3361b3…、candidate42e96468…全部沿用。原材料18443、有效gRPC18444、叶证书3681eb86…、网关native-e-a60111a6/generation1不变，CSR/stage/bind/登记总各1。

先核实际已知 ON revision c693589f…、Root namespace/source、无活动请求；新的维护 OFF 目的 **ecddaf54-d27d-4ab4-ba89-68e48a8ea328** 单次GWT确认、HTTP200//OK，推进到 OFF revision **790a3e0b-afc5-4f29-be41-0149ea3a32f8**。这是已知成功后的独立维护动作，没有重发旧 UNKNOWN 或原首次开启。

source **8a93b64a-0c41-4816-b092-29eb683f783e** 一次 prepare/install APPLIED、不自动启动。72停止态资产逐项精确，含已有开关9及连接配置7证明；没有重授权/重配置。env SHA仍d8f51c99…77434，原身份20/材料32/Core6/local7/material5精确。native identity代码指纹推进c52a97a3…e555；publisher/relay Python未变，现行配对仍完整检查。

显式启动后 native **23882/ticks20724103**，UID/GID997、groups996/997、五capability集合0、NNP1、NRestarts0；Core **24168/ticks20724118**，UID996/GID989，11 effective/13 bounding、NNP0内部待办保留。key4714/ticks18777292、recovery4640/ticks18759746、NM4668/ticks18777273不变。两材料socket enabled/listening/NConnections0，NAccepted仍4/6。

新 source/route/OFF 原 revision 核验后，恢复目的 **a2cb7bad-74f5-4d26-981f-89542714677c** 另行确认，单次GWT开启HTTP200//OK，当前 ON revision **bc29b319-608b-4920-bef3-9a161b0f5a11**、actor kura-uid:996。本机CONNECTED、中心同证书/generation1，新连接epoch **7787b274-956a-4d2e-a722-361b046c2295**、原boot38663672…；中心实际能力清单15→11，恰去四个未授权Netplan写能力，读取及结果核定能力保留。原首次ON/OFF/本次ON写前均独立O_EXCL/fsync留存和单次guard，无CSRF/request body输出，无写入重试。

## 单次真实任务

中心原数据库 console_empty_native/PG40eca01d…，原中心D5a4b2b32…未重启。UI Query status只点一次，写前留存原 idempotency key，固定网关/operation字段和单次guard，HTTP202返回QUEUED：

- 目的 `37487b53-e59d-4026-bdaa-a69cce30cc0e`
- 原 idempotency key `4f46051a-eeff-4aef-ae9d-f8debfd82a51`
- 原 task ID `7e30af94-6bcd-40cb-a0e4-af475360cab6`

之后仅GET原ID，无新key或任务重发。中心数据库 operation1、bindingGeneration1、dispatchEpoch7787b274…、stateSUCCEEDED、error空，1网关/1任务；GET原任务返回 agentAvailable=true、kuraAvailable=true、uptimeSeconds158、version0.1.0。UI显示Agent status/7e30af94/Completed，页面错误0；原登记计数仍1。

以真实UID/GID997、五cap0、NNP1、SQLite mode=ro/query_only读取同一原记录元数据：现行 `executions` 表1条SUCCEEDED，plan114字节/result153字节均加密、requestHash78ab76b2…b97b，原操作/幂等键匹配；未导出加密载荷或钥。旧results表及其余5表均0，Root operations/startups仍0。网关 WAL **12,392字节** 是合法执行记录，保留原样，没有截断、checkpoint、清数据或补写结果凑验收。后续 source 升级必须先交付保留真实业务状态的显式维护路径；既有空业务准入不放宽。

初次只读观察器有三项错误：把浏览器Fetch Response.status当方法；对PostgreSQL jsonb调用octet_length；误期望旧results表而非现行journal.claim/finish拥有的executions记录。原失败文件保留，依据实际拥有方修正只读查询后正向核对；任务始终只发送一次，不修改产品日志或历史结果。GWT重启后另一次异步登录页分支过早判断的只读等待超时保留，登录加载完成后继续，没有重复Mode写入。

## 保留与下一步

18:46:21 UTC原身份20/材料32/Core6/local7/material5/native selected5精确，Root原grant9/transport7证明及其READY保持，native业务计数采用同现场UID997只读executions元数据，未用忽略WAL的immutable查询冒充当前计数。18:46:34 UTC原3现场、个人25、C9及原进程/计数、D29及原Main3135/ticks16649159再次精确；文档前18仓干净、冻结完整12父项SHA4e1aaca5…84cc7不变。无推送、不加JDK27、不清active target/runtime。

本次只证明agent.status真实任务，不外推配置/快照/网络/全部provider读写。继续实际configuration等受限读取、保留业务/续期/TLS/UNKNOWN source及升级移除恢复、typed brokers/Core最小权限、跨刷新Mode原目的/UNKNOWN及地址GUI、完整六组件/provider/launcher/R017和全部12父项。G034仍未封存，协议未正式发布。仅原3 external deferred：NM/cloud-init/物理硬件、amd64、磁盘满/硬断电。旧UNKNOWN不重发/换ID/跨Root查询，闭环备份/归档/轮换/恢复/900秒续期不重放。此轮无HTML/CSS/client JS改动。

证据：工作空间 `.management-close-20261116/source-cohort-20261117/native-capability-readonly-20261118/`，同名JSON检查点见 `docs/checkpoints/20261116/native-capability-status-checkpoint.json`。
