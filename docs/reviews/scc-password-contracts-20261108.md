# 2026-11-08 4a8845f3 SCC 密码来源合同与安装边界

从9ee0ebed继续，动前核对全部指纹、18仓HEAD/branch/dirty、原Orb四服务与两台已停Parallels；保留动前源文件及相关模块目标产物副本。没有提交推送或清理现场。

## 修复与来源合同

审计发现不能把所有SelfConfiguringComponent默认视为逻辑密码：NM在解密后端写入参数之前缓存原存储属性，getter将缓存String转Password；BaseAsset也保存原CM属性，动态driver PASSWORD定义此前没有统一转型。原9ee广义SCC假设在这里收窄并纠正。

ComponentConfigurationImpl使用可缺省的Boolean来源：未声明沿旧存储解码；显式false表示存储值，true表示逻辑值。confirmed外部所有者缺省仍逻辑，显式来源优先。Core公开读取存储型CCI时创建属性副本再解密并标逻辑，不修改provider的稳定或不可变Map；合并当前配置保留显式false。snapshot使用同一来源判定。

NM getter明确声明存储值；不改真实NM后端，也不切换网络。BaseAsset根据动态PASSWORD AD将原String/String[]/Vector转为Password/Password[]/Vector并声明存储值，原缓存保持原样。WireAsset继承此getter。逻辑fixture显式声明true。

Core和XML分别提供password.sources与snapshot.codec的1.0.0能力，并要求对方1.x；Asset/NM要求sources1.x，uses约束绑定Core包类空间。保留原EE要求。未改org.eclipse.kura.api模块；Core导出DTO增加来源查询方法，属于公开API增量，必须协调分发。

## 范围审计

18真实子仓库图谱、literal补充与DS声明交叉核对，找到10个生产SCC声明和1个测试fixture：BaseAsset、WireAsset、Console、两个AgentFirewall、FloodingProtection、NM、Netplan和两个传统Firewall。ConfirmedFirewall是两个agent实现的基类，不应把它与具体DS混计。Console/Firewall/Flooding无PASSWORD配置；Netplan旧prototype confirmed路径保留，无网络方案回迁。

图谱是best-effort，Core/Asset/NM本轮修改后metadata_changed，结构发现结合实际源代码与DS哈希核对，不以ready/no_recorded_issue声称完整解析或第三方动态服务全覆盖。

## 验证

Core214、XML8、Asset38、NM1497项全部通过并install。新Core来源测试先在旧210产物复现4项中3失败；修复后通过。Asset标量/数组/Vector真实getter测试3项；NM测试使用真实cache/getter及NetworkCommon、mock DHCP/DNS/NAT构造，无activate/update/系统网络调用。

首次NM构建因本地缓存旧linux.net缺既有FirewallOwnership编译失败，日志保留；从当前未修改linux.net源码install -DskipTests更新前置artifact后NM1497全过。linux.net没有执行测试，不把前置安装算系统验收。

真实Equinox使用实际生产osgi.bundles列表，只init/install/resolve，不启动业务bundle。5场景：旧baseline与全新Core/XML/Asset可解析；新Core+旧XML、旧Core+新XML、新Asset+旧Core被能力缺失阻断。NM只核对sources requirement provider数量（新Core1、旧Core0），没有宣称完整NM解析。前两次resolver夹具失败分别为configuration/storage冲突、重复launcher路径，均发生在业务启动前，全部日志/存储保留，最终v3结果是依据。

IDEA新隔离完整Kura PID30627/HTTP18627，283 inventory全部匹配最终Core214/XML8/Asset38。仅提交新sources-once命令：未标记legacy、显式stored、显式logical、实际BaseAsset getter四来源都通过真实Core paired读取和新V1快照读回，PASSWORD标量/数组/Vector相等，provider原cipher值保持；同一cipher字节按显式来源解释。BaseAsset使用只读注入cache，无driver启动。没有业务CM update/rollback；revision读取可能分配dormant CM条目，不声称零CM状态变化。验证后只停30627，现场保留。

## 尚未完成

新的来源合同与选定fresh解析边界已闭环；旧provider+新Core没有统一resolver硬门禁，但保留旧存储缺省。热更新/refresh、降级、全发行包部署恢复、第三方SCC、旧无标记材料歧义没有因此完成。真实NM/cloud-init/有线、硬件、amd64、磁盘满/断电仍外部deferred。完整14项内部pending保留，durable UNKNOWN/restart/reconcile、全服务器时限等继续；UNKNOWN与历史一次性命令均未重发。

证据target/scc-password-contracts-20261108；清单docs/evidence/scc-password-contracts-20261108.json。目标active。
