# 2026-11-08 40141a77 Core 客户端回执基础

从 3e81381f 续接。入口实时核对 47689 个既有指纹、18 仓 HEAD/branch/工作树，以及原 Orb 四 PID/包和两台已停 Parallels；全部匹配。保留新旧现场，不提交推送，不重放旧一次性入口，不重发 UNKNOWN。本轮没有新 native IDEA 或 Linux 运行场景。

## 实施

公开 configuration API 升为 1.8.0：新增带客户端 op-UUID 的普通/删除闭包 plan overload，旧 provider 默认 UNSUPPORTED；新增 Java 11 可用的值无关 Receipt DTO 与独立查询/未登记 ID 封存服务。Core 显式要求 API1.8，并发布 receipts1.x capability。完整发行/升级混装仍未验收，不能把这个最低导入边界当成全安装闭环。

Core 在预检前持久预留 request，在实际 writer 前强制原 intent 与精确 SHA 绑定的 dispatch。原 intent UUID 与客户端 ID 相同。任何已预留 ID 都拒绝复用，包括相同请求、成功、拒绝和取消。不是幂等重放接口，不持久化属性值、密码、revision token 或属性值摘要。只保存创建/更新/删除/作用域元数据及这些无值记录的绑定。输入先深拷贝并限制规模，预检中的同步重入 legacy writer 不得抢用该 intent。

只有尚未尝试 intent 的失败才可持久 REJECTED；writer/快照不确定后仍 UNKNOWN，既有全局未决 intent 门禁保留。CORE_COMMITTED 仅说明原 Core writer 与快照完成，不说明工厂激活或业务健康。后来受控恢复的准确回执对应 RECOVERED，不能冒充原请求成功。部分/损坏/不安全记录 UNKNOWN；释放链接优先于名义 complete，绑定发布丢失或变化不能转成功。

独立 DS 仅依赖 SystemService，无 CM/Core 配置激活/密码/agent/业务引用；查询不拿 writer 锁、不读快照、不调 provider、不重发。未来远程入口必须自行鉴权和 XSRF，本轮没有添加远程端点。取消只允许用 CREATE_NEW 对完全未预留 ID 写取消墓碑，阻止迟到原请求，不取消已预留/已派发/UNKNOWN，也不松 journal 门禁。发现“request 丢失但有孤立 dispatch/decision”的边界后，查询补为 UNKNOWN、封存拒绝；没有削弱私有目录权限检查。

合同见 docs/lifecycle-client-receipts-v1.md。旧 journal v1 和 release 记录保留，旧匿名 API 不自动变成可查询客户端事务。

## 验证与失败证据

API24、Core253、工具62 全过/install。Core 新增15项有效测试覆盖同/异参数重复 ID 与重启、值无关落盘、预检拒绝、快照失败门禁、取消竞争/迟到请求、writer 进行中独立查询、同步预检重入、完整/部分/不安全/超限记录、精确恢复发布、旧 provider 无降级及 Java11 编译。真实独立 JVM 查询原完成和未完成 ID，未完成 journal 仍拒绝普通启动；没有重放 writer。

新增独立 targeted Equinox/SCR 用例1通过：实际打包的 Core/API，夹具不安装 CM/Crypto provider 或旧场景 bundle，ConfigurationService 不可用，生产 Receipt DS 仍可注册、查询未完成 intent 为 UNKNOWN、对另一缺失 ID 封存；未读写快照。该 intent 是无 CM writer 的协议夹具，不能记作真实创建/删除故障或实际业务恢复。未重跑旧配置场景参数集，也未追加旧测试恢复任务的 Linux 验收。

失败保留：首轮 Core 编译误用 getDataDirectory，改为实际 getKuraDataDirectory；扩展恢复用例一次 CompletionEvidence 构造参数缺失；一次孤立记录用例错误创建了宽权限根目录，被生产权限检查拒绝，修正夹具为0700，未放宽实现。首轮 runtime 构建缺少本地6.0测试夹具，按当前 API 构建/install 后离线执行；第二轮断言 Crypto 不在场失败，原因是旧夹具/依赖触发 lazy provider，保留诊断，最终新用途夹具明确不安装 Crypto/旧场景 bundle 后通过。日志、失败报告及失败 runtime 都保留。另新增两项矛盾证据回归先复现 CANCELLED/RECOVERED 误判，再修为核验全部在场决策文件；取消后的孤立决策/预留后的孤立 release、有效恢复下损坏 complete 均 UNKNOWN，失败源码和2负例报告保留。

新源码完成子仓库图谱刷新与精确路径 coverage 检查；结果是 best effort/no recorded issue，不是全解析覆盖证明。原 GWT210、Rust229/11ignored、NM1497、XML8、Asset38 等历史结果保留，未计入本轮新覆盖。

## 台账与续接

只关闭客户端回执的 Core 协议基础子项；GWT/client 尚未接入，creation/deletion lost-reply 与真实业务对账没有关闭。完整14项内部 pending 不减少；外部 NM/cloud-init/物理有线、amd64、磁盘满/硬断电三组 deferred 单列保留。原初始 GWT CONFLICT 精确 holder gap、同 PID SCC/non-SCC 枚举覆盖、零字节失败快照策略未修。

下一步优先接 GWT：在原请求前保留客户端 ID 与 PID/域等无值元数据，创建和删除 RPC 携带 ID；响应丢失或重载只查询原 ID，不自动重发；Core commit 与 readiness/业务结果分开，在明确对账前保留写门禁。覆盖创建后/删除后断线、NOT_FOUND 与迟到请求竞争、服务重启、中文/英文实际页面，并继续完整父台账。不要把旧 UNKNOWN 改用新 ID 再发来“补回执”。

最终清单 docs/evidence/lifecycle-client-receipts-20261108.json；证据 target/lifecycle-client-receipts-20261108。父工作空间知识库 scope=ws 记录并精确读回，Home 不改。目标继续 active，未宣称整体完成。
