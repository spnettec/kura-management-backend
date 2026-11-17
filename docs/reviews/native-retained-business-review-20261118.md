# 2026-11-18 原生业务保留检查与 source 维护候选

目标 active，使用现有积分继续；不推送。完整12父项最终闭环仍0/12，不能解释为实现0%，没有可靠加权总体百分比。此次完成只读加密库存与独立维护事务代码、夹具回归、当前Git发行包和同E真实只读检查；**没有进行真实业务 source 安装升级**。

## 代码与权限边界

agent3874716/c0253d2增加八表只读库存及独立检查CLI，fea2521提供仅cfg(test)显式一次性夹具生产器，41688a7移除检查密钥路径对原数据库的SQLite打开，eab6821接入Root维护事务。普通daemon完整代码/unit/source/peer/material检查保持。独立检查角色只限固定原Root路径、专用非Root身份、原Native/reader组、五capability集合0及NNP1；调用的ELF由Root完整候选包配对认证并经sealed memfd执行，不授予daemon/socket/backend能力。

原初始化先于业务解密和调用者旧ID路径查询。原库、密钥、marker及WAL/SHM不认领、不重建、不改所有权、不checkpoint、不剪枝、不recover、不重发。只复制加密DB/WAL到新建/run Root0711/native0700夹具；密钥留原路径。检查精确schema/integrity、plan/result envelope的AAD/tuple/hash、imported原始记录和既有连接证明，输出计数/状态/摘要，不输出载荷、旧ID或密钥。RUNNING/UNKNOWN只计为未决。

新prepare-retained-business-material-kura-source-update仍要求原local/material权限、完整首次身份、remote OFF、Core/native停止、材料socket停止且排空、原writer锁可取得。普通空业务入口继续拒绝非空表/WAL；未知provider文件、续期/多候选/pending/legacy另行处理。DB/WAL各8GiB、SHM16MiB、普通表百万/imported1024的界限不代表完整历史迁移能力。

所有公开source prepare/status/complete现在要求--initialization-id及原request-id；发布中仅原pending允许previous/incoming配对区间，原初始化、roles、continuity、历史链和完整安装门禁仍验证。另一个ID即使同包SHA也拒绝。新retained-business-source-status/complete-retained-business-source-update只续接原事务，不重装。原库输入/加密库存及所有保留指纹绑定intent/result；准入和发布重检，丢失证明或状态变化拒绝，不自动启动。

## 验证范围与保留失败

Rust journal选择40过、role15过、native-state1过、source-inspector1过；选择重叠不相加。真实UID997一次性夹具持有32,992字节WAL及SUCCEEDED/RUNNING/UNKNOWN各1；正确角色认证通过，Root/错误UID/非零cap拒绝。原目录及5文件字节和含atime元数据精确，未复制密钥。此前夹具因原DB atime变化失败的证据保留；41688a7通过不在原库打开SQLite修正，没有恢复时间戳或改历史结果。

Python当前完整284选择，282通过、2个历史导出合同夹具跳过；18个新方法含事务/候选archive/CLI初始化边界。事务方法仅替换检查器image/execute，不能算真实安装证据；候选archive配对及真实UID流程另外验证。首轮发现测试镜像缺openssl，下一轮修正Source.json调用的代码错误，再补齐/run夹具；另一次宿主共享读取发生截断，而主机文件完整，改为带SHA的完整stdin archive后验证。全量首轮缺NET_ADMIN和Core sibling guard输入，按实际拥有方补齐当前Core文件及仅新隔离容器NET_ADMIN后全量通过；中间一次tar输入组装失败也保留。全部失败不算通过，不修改产品数据、历史回执或既有跳过项。

协议旧比较路径crates/protocol不是实际拥有方。新证据核验声明1c5842a、当前Git及workspace/lock的**crates/management-protocol**，对象一致；协议本次再从声明Git重新package，SHA61163caf…e70c2，未正式发布。原错误范围回执不重写。bridge3e0a432a…a5eb9只读当前配对E输入，无旧ELF/crate中间产物。

当前Git eab6821915e453b3a7251481d7ab60767f086943自包含导出、release、deb及四生成控制脚本通过。包0.1.0~native202611171954.eab6821-1：

| 内容 | SHA256 |
| --- | --- |
| arm64 deb | 0dc111ebb9c95bffd6a4709f9c8560f720257124a970b2ae26be56e64243f5b0 |
| ELF | c03bbb72d05017029c9429d856c590835a268f568a127bd771b22e740c5d3691 |
| source manifest | 0a5eb4728259465f37126118a16f4bb327a0ebf37df3bbc5d1541f02db9d157c |

## 同E真实只读观察

19:56:13 UTC，原E a60111a6…94414，先核原初始化24560864…及已装source44a72498…2ead9。新公开代码与候选包仅暂存Root0700目录/opt/kura-retained-business-inspection-f6e198ad-6752-4d90-88f0-c6074b8787f1，文件Root0600；完整archive/四maintainer/source/unit/ELF配对后，通过实际Root Python复制与非Root sealed memfd流程检查原业务。

原库69,632字节SHA c5dce99266b22331eae21225d2714e1bde729240379350d2c53c39831f5cc547；executions1/SUCCEEDED1，其他6业务表0，generation1、未决0，原加密载荷认证通过。schema摘要70be30f7…c3b8e、logical acc02cf4…1a4ab。检查前后原目录/文件字节及含atime元数据完全相同，密钥未复制到scratch或host，无source intent/安装/服务/任务/remote写入。

**此时在本次复制前已没有WAL/SHM**，此前18:46的12,392字节WAL记录保留，不改历史、不归因其消失原因；本次未调用checkpoint/truncate。此观察仅证明当前同Root真实记录检查，不冒充非空WAL安装升级。32,992字节非空WAL证据仍是独立一次性夹具。

19:58:03 UTC native23882/ticks20724103及Core24168/ticks20724118仍active/NRestarts0；E仍运行24552f6，未安装eab6821。原身份/授权/transport和已知ON bc29b319…没有本轮更改；不重发原任务7e30af94…或任何旧Mode操作。原3现场、个人25、C9/原进程计数、D29/原Main3135再次精确。文档前18仓同分支且干净，仅agent推进；冻结12父项SHA4e1aaca5…84cc7不变。

## 继续实施

先按当前环境和原目的交付实际同E source维护：新的已知ON后维护OFF目的须写前留存并单次GUI确认，不能用旧UNKNOWN/旧Mode ID；停止并排空后重新取基线，保留真实业务、Root权限与完整证明，原source准入/安装各一次。UNKNOWN只查原ID和原Root，发布续接不重装，不自动启动。已暂存候选未分配source request，不能把诊断purpose当安装ID。

随后继续配置等受限读取、续期/TLS/legacy/UNKNOWN/provider保留与升级移除恢复、typed brokers/Core11/13caps/NNP0、Mode刷新/地址及安装GUI、完整六组件/provider/launcher/R017和全部12父项。原11 sibling只是已验有界修复，不外推所有变体/平台。G034未封存，协议未正式发布。仅NM/cloud-init/物理硬件、amd64、磁盘满/硬断电三项external deferred；其余内部pending。原3现场/C丢失CSR ID/个人配置/活动target/runtime保留，闭环备份/归档/轮换/恢复/900秒不重放，无push/JDK27。本次没有HTML/CSS/client JS修改。

证据：父工作空间.management-close-20261116/source-cohort-20261117/native-retained-business-20261118/；检查点见../checkpoints/20261116/native-retained-business-checkpoint.json。
