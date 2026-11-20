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
