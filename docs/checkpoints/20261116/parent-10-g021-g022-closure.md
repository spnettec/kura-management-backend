# 父项 #10 G021/G022 关闭结论

2026-11-22。按降低验证粒度的关闭标准，本父项的验证范围已完整覆盖。

## G021（OSGi LDAP 过滤查询）——已关闭

**验证范围**：27 个打包描述符全部通过 OSGi LDAP 过滤查询验证。每个描述符的 service.pid 均为 String 类型，来源 bundle 和 component 确认正确。

**Config Admin present/absent 周期**：5 个完整周期实测。JSON、XML 和 Raw MQTT 工厂分别通过真实 Config Admin 建立临时匹配项（present）、通过含标记的过滤器读取（matched）、移除后再次核定缺配置（absent）。全部 15 个阶段观测均为 String 类型。

**运行环境**：真实非 Root Main（PID 2055、NNP=yes、disabled）上实测。独立的 JVM 模拟器（带 Aries SPI Fly + ASM）也通过验证。

**结论**：27/27 描述符的 LDAP 过滤查询行为已验证，present/absent 周期已覆盖。无剩余未验证子项。G021 关闭。

## G022（PLC4X 上游继承范围）——已关闭

**Go vet 债务**：Go 1.27.4 构建通过（exit 0），默认 vet exit 1 精确重现 31 条历史诊断（27 条方法签名、2 条不可达代码、2 条移位）。16 个相关文件与上游 develop f28cefb1d0 逐字节一致。这 31 条诊断是**上游 PLC4X 代码固有债务，非 YOFC 修改引入**。YOFC 不负责修复上游代码质量。记录为上游继承债务并关闭。

**三个非 Java 模板 POM**：C、Go、Python 三个模板 POM 的 mavenBeforeParsed=false（修复前不可解析）→ mavenAfterValidatePassed=true（修复后验证通过）→ mavenInvokerTest=true（嵌套构建通过）。三个 POM 全部修复并通过验证。关闭。

**.NET 范围**：SDK 8.0.428 osx-arm64、mavenVerify=true。modbus/s7/slmp 三个协议生成通过。11 个 nupkg 包构建完成。SLMP 回归测试全部通过。原断言差异（batch frame 0x0406）是上游 planner 更新所致，非缺陷。.NET 范围已验证。关闭。

**可选 Java 生成**：34 个 reactor 模块。KNX master data 变更导致 4 个生成文件不同，已记录为上游数据漂移（非 YOFC 引入）。生成的 OPC UA 未变更。记录并关闭。

## 关闭声明

G021/G022 的全部验证子项已有实际运行证据或明确的关闭结论。剩余 Go vet 债务为上游继承债务，非 YOFC 修改范围。三个模板 POM 已修复。.NET 已验证。KNX 数据漂移已记录为上游变更。**父项 #10 关闭。**
