# Java 离线快照校验增量（2026-11-05）

整体管理体系目标继续 active。本轮只交付原方案 §11.3 的短时 Java 工具前置能力，不是完整离线恢复。没有修改 Rust 恢复成功判定，没有发布/恢复任何业务快照，也没有删除 CM 缓存或工厂实例。

## 实际变化

核心 CryptoServiceImpl 增加显式密钥的非 OSGi AES 上下文，只接受明确 raw key 或明确默认密钥选择，无环境查找、生成、轮换和错误密钥回退。真实 XML mapper/密码适配器增加可注入 CryptoService，原默认构造仍走原 OSGi 服务获取及释放；手写 XML bundle manifest 补 crypto import。新增 tools/kura-offline-recovery 独立短生命周期 JAR，复用这些真实实现，不启动框架、业务或脚本。

工具仅 validate：完整 AES-GCM/明文模式及原密钥必须显式选择；有界内存解密，无明文临时文件；严格结构/重复/类型/数组/数值校验后用 Kura 实际 mapper 转换。拒绝 DTD/实体/未知内容、叶子符号链接及宽权限密钥。stdout 只有 SHA/计数/认证证据，错误固定 JSON，fullRecoveryVerified 永远 false。默认每个 Password 值必须认证；历史明文/Base64-only 只在明确 legacy 模式接受并单独计数。真实持久快照省略 false 属性及附带未用 OCD namespace 的格式已兼容；不接受含 OCD 内容的导出文档。

详细使用、格式、资源上限和限制见核心仓库 kura/tools/kura-offline-recovery/README.md。工具尚未进入 agent 事务、安装包或正式发行。

## 验证证据

受影响 reactor 9 组共405项测试全部通过（含新工具18项）；没有 skip。独立 fork JVM 正常一行 JSON、损坏输入只有固定错误、不泄露输入内容；实际 mapper 生成各标量/数组/CDATA、直接和包装 AES 密码、legacy、默认/显式/克隆密钥、错误 key、整包篡改、源/key不变、XXE/DTD/重复/非法类型和 symlink 等已覆盖。未改变核心默认运行密码行为。首次遗漏 flag 方法、运行依赖与 stream writer 可选属性不兼容均在首次实际系统验证前修复，初始日志保留。

IDEA 原生启动新的完整282项运行集（PID75463），原 admin/admin 中文登录与原快照页面通过。只在这个新夹具用原 GWT 把 revision fixture count 1→2，确认保存得到 snapshot_1793844222450.xml；原值/初始化快照保留，不涉及用户业务。新真实快照25组件/6工厂/206属性/4个密码：1个实际 AES 认证、3个历史兼容，严格模式拒绝，显式legacy通过且分开报告。初始 snapshot_0 为11组件/3工厂/69属性/3个legacy，密钥认证false。原GWT新快照列表及数值保存页面有截图。

OrbStack 复用原VM，在新的 /var/tmp/yofc-offline-java-20261105-9dc1895e 运行同一282项清单和当前3个核心JAR；原 admin/admin 中文登录及原快照页通过。独立Linux JVM5项合同：自身初始化快照、IDEA实际保存快照、真实CryptoService生成整包AES、错误密钥拒绝、严格拒绝legacy，全通过，所有输入SHA不变。整包AES是测试夹具在agent外用真实Java实现生成，不能说是Kura整包加密模式原生产写出验收。

这证明工具和默认运行兼容，不证明恢复后的业务健康。没有把unit active、opaque备份或正常配置保存当成离线恢复。

## 运行状态及保留范围

宿主新的 .phase1-dev/offline-java-idea-20261105-207020ce 已停止，PID75463不存在；所有数据、生成快照和IDEA配置保留。Orb新Kura PID330621已正常停止，目录保留，未启动新的agent/中心/PG。原 kura-management-console、kura-management-console@center2、kura-management-agent 和既有 phase2-agent 仍 active，原包26/20未升级。Parallels Ubuntu/Windows均stopped；缺Java/Kura，本轮该范围deferred，没有为无关验证启动VM。旧夹具/身份/密钥/备份/quarantine/卷、个人~/.kura-dev均未清理。

18仓HEAD/branch没有改变，全部原未提交改动保留，无提交推送/reset/clean。状态、基线备份、完整构建/失败日志和截图：target/java-business-recovery-20261105。机器台账：../evidence/java-offline-validator-20261105.json。

## 下一个未交付步骤

继续实际实施 Java工具→Rust预检与事务→Kura受控启动的完整恢复合同：最大snapshot编号/溢出、明确密钥停服后可用、完整替换及缺失工厂删除、当前网络/防火墙外部归属、禁止静默旧快照回退、准确restore ID/hash/逐PID应用回执、真实桥接及所选业务核验、异常后持续挡写及现场。以上是pending，不是环境deferred。当前工具既不输出可投产恢复计划，也没有修改运行快照。

原完整目标其余pending继续保留：完整组件适用面、防火墙适配/NAT/远程/早启、restart/reboot任务、一致备份恢复、身份运维/轮换/转移、正式发行。外部deferred：Parallels Java/Kura、真实NM/cloud-init/物理有线/硬件、amd64、磁盘满/硬断电。NM和Netplan保持用户已核定现状。不要重做本次校验或旧一次性登记/交付。

项目知识库最新全文：`Experience/2026-11-05-Kura短时Java离线快照校验与完整运行兼容续接-c8cf8a09.md`；Tasks/Session Summaries及上一交接已通过官方CLI追加。共享ensure脚本首轮未带--dir而命中默认FIS5，随后显式Kura路由校验通过；没有向FIS5或全局写入。
