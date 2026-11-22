# Kura 密钥相关数据盘点（2026-11-16 增量）

先查18个真实子仓图谱，再用Git跟踪的生产Java源码补查直接加解密/密码仓库调用。图谱漏掉了当前NmTemplateCodec，刷新networking后已可定位；ready不等于完整覆盖。本页记录静态边界，**不关闭全部provider盘点或旧钥退役**。源码与精确验证范围见 [本轮检查点](managed-core-start-checkpoint.json)。

| 数据或消费者 | 本轮结论 | 剩余工作 |
| --- | --- | --- |
| `data/store.save` | 文件keystore密码及SQLite key/salt记录使用CryptoService；已加入私有暂存、同步、原子替换和同JVM串行更新，读失败不覆盖旧条目 | 停机事务中的完整重加密、原文件保留、实际provider重开 |
| CM密码与配置 | ComponentUtil、启动收敛、回滚、typed API及组件消费同一加解密服务 | 完整单例/工厂/非DS provider、类型及源格式；直接调用数不能代替此范围 |
| snapshots及导出 | 整份加密仍按最终配置，kura_custom.properties覆盖语义保留；logical-V1与历史密文须区分 | 历史文件、上传候选、外部旧备份逐项核对，不能只切现用钥后删旧钥 |
| 恢复backup/intended/applied制品 | 加密字节绑定原计划、摘要与回执 | 原事务合法恢复和保留政策，不能改写原制品、刷新摘要或换ID |
| NM原模板chunks/manifest/receipt | Java/Rust白名单遗漏KURA2，现支持严格语法，Java真实Crypto认证，agent保管原字节 | 实际注册链及正式runtime/grant/cohort准入；原UNKNOWN不重导出、不补造缺块 |
| H2/SQLite、PKCS12、PKCS11 | Kura包装钥、数据库数据钥、keystore密码/PIN是不同对象 | 证明解开包装后仍使用原服务秘密；包装重加密不等于数据库/keystore改密 |
| 中心任务库和agent其他加密状态 | 各自密钥/协议合同独立 | 客户库、GUI备份恢复和joint generations仍按原台账 |

直接引用生产Java文件（含定义）：core17、Artemis1、cloud2、container1、GWT2、networking2、OPC UA1、Triton1、YOFC8。其他仓本次无这些直接调用命中。PLC4X四个getKeyStorePassword同名结果属于其证书配置接口，不属于Kura CryptoService，不据此合并上游。

cloud、Artemis、container、OPC UA、Triton、YOFC MQTT/InfluxDB等已定位为密码消费者，尚未逐一完成真实provider验收。Camel/Wires即使无直接AES调用，也可经CM Password、脚本、动态provider消费秘密，间接路径继续盘点。

store.save检查覆盖旧密文字符串保留、两实例并发32次更新、发布失败/不支持原子移动、坏文件、符号/硬链接，以及“可写但不可读”的旧文件。macOS及隔离Linux/JDK21的6项均通过。序列化范围是同一JVM，不替代维护锁、跨文件事务、provider重加密或旧钥退役。

NM测试使用合法自定义原钥；默认弱钥继续拒绝新模板。代号形状不是认证，Java认证整个头和内容；agent不取得原钥、不解密、不重建模板。没有恢复或重发原网关UNKNOWN。

## 2026-11-21 文件数据库的实际进度

正式六组件 Main 已验证两条独立的文件保留路径。H2 工厂沿原凭据重开原库，凭据被拒绝后仍能读回升级前的记录。可选 SQLite provider 在原地更新、卸载及重新安装后仍能通过原工厂读回原记录。SQLite JDBC 回归另验证凭据记录保存失败后的原文件重开。两条路径均未更换网关包装钥，SQLCipher、旧钥退役与其他 provider 仍按原台账待验。[实装回执](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-21 SQLCipher 文件库

ARM64 六组件正式 Main 已验证同一 SQLite 工厂内的数据库凭据换钥，以及第二次 Main 启动后的业务行与加密页完整性。现场使用测试专用 JNI 和 Debian libsqlcipher1，原包装钥及先前存储项保留。发行驱动选择、跨代包装钥轮换、旧钥退役仍待验证。[驱动来源、实装与离线复核](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-21 PKCS12 业务签名钥

原六组件安装生成一次合成 RSA 业务钥。Core 升级前后，原 PKCS12 均能以原凭据打开。正式 Main 改变文件库口令并冷启动后，原 X.509 证书仍能验证新挑战签名。独立 JCA PKCS12 reader 拒绝旧口令。网关包装钥仍为原钥，文件库写入中断、其他认证模式和跨代包装重加密继续待验。[文件指纹、三个 Main 及安装包记录](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-21 PKCS12 文件发布中断

既有业务库在序列化错误和发布前写进程退出后保持可读。首次建库已发布且暂存硬链接尚在时，后续写进程只回收可核对的临时链接。正式 Main 遇到已有文件锁返回可观察的失败，原钥仍可签名。此次没有更换网关包装钥。库文件、密码记录和配置三者在随机改密过程中的共同恢复仍待验证。[文件发布和实际 Main](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-22 Artemis 的两种凭据入口

在发行包 `1bcdc5d` 中，Simple MQTT 的 `password` 声明为 Password，由组件调用 CryptoService 解密再配置 broker。XML provider 的 `users` 声明为 String，组件用 Java Properties 解析用户名、口令和角色。正式 Main 已通过这两条配置入口分别收发认证 MQTT。本轮 XML 更新显式跳过快照，恢复了原配置。持久化验收仍需覆盖历史 String 用户表、快照的实际保护、冷启动认证和换钥后原业务凭据的连续性。盘点时应按字段和配置保存模式识别这两条路径。[本轮检查点](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-22 Artemis XML 用户表在磁盘上的保护

原包配置关闭整份快照加密，0600 XML 中直接匹配合成口令。显式开启后，新快照使用 Core 的整份加密；正式 Main 整机重启后通过 getSnapshot 读回同一用户表，并以原测试口令在 MQTT 3.1.1 完成 QoS 1 收发。XML provider 的 users 仍是 String，本次使用原网关包装钥。启动自动转换旧明文快照会替换其 inode；转换中断、跨代重新包装和旧钥退役继续待验。[实装检查点](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-22 历史快照转换的原位续办

Core 已在第一份快照转换完成、最新一份仍为明文时通过进程中断验收。已有密文按当前钥认证并保留，最新编号成功续办并加载。新的 ARM64 deb 由正式 Main 读回混合历史中的最新业务设置，尚待验证的是跨代重新包装及旧钥退役。[实装检查点](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-22 历史快照跨代重加密的原位续办

正式 ARM64 Main 已验证固定安装历史上一份旧加密快照的原位重包。原密文单独保留，替换后的中断沿原序号只补签，活跃钥可独立读回完全相同的 XML。Main 正式启动后读回同两份历史配置。Legacy 内嵌密码混合旧代已通过 JDK 21 codec 测试，尚需逐 provider、CM 存储和已登记恢复制品的生命周期验证。[实装检查点](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-22 默认 Debian Main 的 Equinox CM 存储边界

正式 Main 通过 `-configuration /tmp/.kura/configuration` 和 `osgi.clean=true` 启动。Equinox CM 把运行值写成带 CRC 的 `data*.cfg.<代号>`，本场三份旧文件在正常停机后仍可读。切换发布代后原三份文件退出，Main 从未改动的原快照重建标量密码和密码顺序容器，使用活跃钥封装。仅经 ConfigurationAdmin 写入的旧运行覆盖和临时 PID 未回放。这个默认 profile 的持久配置来源是快照，CM backing store 是启动后重建的状态。受控恢复、显式保留的 CM 与自定义 framework 存储另按原配对检查。[实装检查点](actual-native-wires-retained-reinstall-checkpoint.json)

## 2026-11-22 历史快照类型化密码写入 CM

默认正式 Main 已按旧格式来源逐项解码标量与混合密码序列，在写入 Equinox CM 前重新加密。两份实际 ARM64 deb 的对照分别得到明文 4/5 和 0/5，修复版快照 API 读回 5/5 预期值。离线重加密的 JVM 回归覆盖 Base64 包裹的旧代 CM 密文，以及保留相似字面值。自定义持久 CM、受控恢复和逐 provider 仍按其配对环境验证。[实装检查点](actual-native-wires-retained-reinstall-checkpoint.json)
