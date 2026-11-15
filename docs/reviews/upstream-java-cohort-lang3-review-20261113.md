# PLC4X Java 7 功能组收口与 commons-lang3 v15

2026-11-13。源码级功能批次已验证，最新评审源 tip 671e086ce85ae9c0722ceb031cb7377a66185d99。
这不是整套管理体系或保留运行集的部署验收。18 仓 HEAD/branch/index/staged 不改变，未 commit/push。
证据在 `target/upstream-logical-sync-20261112/lang3-cohort-v15/`。

## 来源与变更

最后整合 5d1835231faa6d0896630a520f09c6037a0680b3：仅 root POM 的 commons-lang3
3.21.0→3.22.0。上游完整 POM 用 Git blob 校验，第三方 JAR/POM 下载并校验实际
SHA1，JAR 基线 major52。保留 Maven3/Java21/.B/Jackson3 和前六组必要的 fork
行为适配。只有一条版本属性变化，root 前 pin 与实际原像保留，create-only journal
已应用一次，不可重放。当前 authority 是 `../applied-source-pins-after-lang3-v15.json`
508 present pins，加 `../source-retirements-after-ctrlx-v13.json` 89 absence paths。

源码检索 37 个文件含 lang3 引用。全 JAR javap 比较旧4665/新4680 public/protected
成员描述符，无类退休/可见性修饰改变；两个原始差异并非两处公共 API 被删除：
非公开 CheckIntervalData 的 count int→long，JavaUnicodeEscaper 移除的 override
仍由父类同描述符方法提供。用旧 JAR 编译的子类在新 JAR 上运行，6 supplementary
protected 调用和6 public 转义结果相同。探针初版遗漏构造器、随后直接调用旧实现
不支持的 BMP protected 路径，诊断与源码保留；修正探针范围，无产品改动。
不将此有限检查称为完整 ABI/行为保证。

## 完整源码验证和共享产物

从实际原仓复制11928个源码输入，不带旧 target。仅版本属性不同，验证后当前
原仓与全部候选输入逐项相同。97 项目排除 website、串行、--fail-at-end verify
成功：11943 tests，11839 passed /104 skipped /0 failures /0 errors。默认 profile
不启用全硬件测试；104 项逐项原因保存在完整报告，包括既有禁用、PCAP 权限、
socat 缺失、手工设备等，不算通过、不新增外部 deferred 组。RAT/现有阻断门槛
通过。protocols/s7 45%/50% 与1 missed class 是原 haltOnFailure=false 的警告；
旧3.21 baseline 复现并 exit0，未降阈值。

同一已验证输入完成97项目共享缓存 install，586个旧2.B缓存文件/186MB原像保留。
-DskipTests 只跳过 UT，Failsafe3.6 仍运行 IT，16份 XML 被后续安装覆盖；这不是
重放管理一次性现场。保留两个完整日志、初始 XML 的计数/hash、当前报告快照和
明确不可恢复的旧 XML 字节边界，不伪造旧文件。初始完整日志独立重算11943/104
与初始 XML 汇总精确一致。源输入不变，原仓 target 没有构建或清理。

## 实际 YOFC 消费和运行集差异

YOFC335个当前源码输入在新共享缓存上完成6模块 package，现有 Plc4xDriverTest
14/0/0/0（文件模拟读写/映射，无现场设备）。新应用中37个嵌入PLC JAR逐项匹配
本次共享 install。没有 install 该应用或 shadow Kura API。共享 Kura API 的
52205637...614ed 不变。

使用保留运行集实际 osgi.bundles 的278条路径（280 inventory 含 launcher重复
和 framework bootstrap），隔离框架只替换新应用。必须区分以下结果：

| 消费包 | configuration provider | 结果 |
| --- | --- | --- |
| 原运行集内旧应用 `[1.2,2)` | 原运行集1.2 | resolve成功 |
| 本批前重编译应用 `[1.8,2)` | 原运行集1.2 | mandatory range不符 |
| 本批新应用 `[1.8,2)` | 原运行集1.2 | 同样range不符 |
| 本批新应用 `[1.8,2)` | 实际共享构建API1.8，其余277提供者原样 | resolve及真实类加载调用通过 |

所以是保留运行集与既有构建来源版本未配套，非本批 PLC 改动新增。初次误归因
API2已明确纠正；历史文件名 api2 仅诊断命名，不能据此认定 provider2。
matching-build 实际 OSGi provider 仍为 lang3 3.20、Jackson3.2.0、Engine3.0.1，
没有用源码所编译3.22/3.2.4/3.2伪装消费环境。真实调用通过 JSON、YAML读/写、
XML、audit、驱动类加载和15种 ServiceLoader 发现。SLF4J 无绑定的警告保留。

类加载使 API/应用 lazy bundle 变为 ACTIVE，但二者无 BundleActivator；事件
观察确认无 Activator 代码执行、无产品服务注册，DS/Kura core未启动，未连设备。
最初“不能有ACTIVE”断言过宽，保留红例，改查实际服务/Activator边界；无产品
源码或覆盖率策略改变。早期 storage重叠/重复launcher仅私有探针设置问题，均保留。

这是 matching-build 消费验收，不是原运行集直接部署通过。R017部署/发行收口
必须核对 API-provider 与消费包配套；原7API2刻意迁移仍是独立 internal pending。
运行集280文件、framework/inventory/config、实际API522均保持精确，不重组装
正常runtime，不触碰个人数据。

## 收口边界与后续

本批 codec/audit/GSDML/event/CtrlX/conditional-deps/lang3 七组源码与匹配构建消费
可关闭；源 tip可记671，不能称部署/管理全完成。既有13组+本批7组=20源码功能组。
7 nonfork继续跳过直接merge，327Maven+1Cargo/source/assembly/runtime关联保留，
补充本次YOFC版本配套证据，不扩大其依赖版本。上游每功能阶段实时轻量核对，
R013一致检查点及发行冻结前完整复审。

当前508/89、11928PLC/335YOFC、1986management/20pins、R013v32/v13/v14 binding
与正常console/PG ID/health/start均精确；Orb gw-001仍运行，两Parallels仍停止。
未 reset/clean/deploy/restart/commit/push，UNKNOWN不重发，不重放管理已闭环现场。

回同一R013v32 shadow继续factory/private init/imported layout/owner整链，137路径
仍UNAPPLIED；原14管理组、7API2、R017、A外部CA/custody和仅3 external deferred
保持。目标active。当前周78%普通额度可用，credits/reset未动。
