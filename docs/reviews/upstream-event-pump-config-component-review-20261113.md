# 上游 event-pump 配置功能组 v12

2026-11-13。按逻辑整合 db1485d06a902b68a7cb11bb227ac51c8c00662f，不整仓 merge。
证据位于 `target/upstream-logical-sync-20261112/event-pump-config-functional-v12/`。
最终 source-only 共 13 路径：7 modified / 4 new / 2 retired。当前 authority 为
`../applied-source-pins-after-event-config-v12-v4.json`，491 pins。

## 来源和行为

10 个仍存在的上游完整文件已逐个 Git blob 校验，两个删除文件保留完整 patch；
完整反向 hunk 重建父版本，再与实际 fork 对照。Maven 3、.B、Jackson 3 和
既有 watchdog/毫秒触发等功能保留。配置类改用显式普通树绑定，JSON 用新 SPI
codec、YAML 用 SnakeYamlEngine 3.2、XML 用 JDK DOM；配置 DTO 的 public
constructor/getter/setter 签名完全一致。旧 TagMapDeserializer 及其耦合测试
退休，替代的配置/格式/兼容测试覆盖短式、扩展式、混合 tags 和 transform。
原源文件、已通过基线及字节码均留存；不声称内部 serializer 类还存在。

旧版本生成的三种格式、wrapped XML 列表及 derived simpleTagMap 继续读取；
新文件省略 unset/derived 值，短式 tag 保持地址。新裸数值/布尔转文本和明确
的 whole-number/结构错误定位采用上游改善，DOCTYPE 明确拒绝。

直接采用上游仍会回退实际 fork 行为，已用独立旧/新进程验证后适配：

- Jackson 3 原本容忍未知命名属性；保留 root/connection/batch/tag/trigger 的
  同样容忍，不改成上游按 Jackson 2 假定的未知字段失败。
- 原输入流支持 UTF-8 BOM、UTF-16/32 及相应检测。保留 4 字节 lookahead 的
  流式检测，拒绝坏编码，不以整文件缓冲或替换字符掩盖错误。
- 原 XML 支持中文 tag 名。用 JDK DOM 的名称验证取代 ASCII 限制，仍拒绝
  非法名称/未绑定前缀；失败时不覆盖已有输出文件。
- 原空 XML list entry 为默认 bean；保留它，修复上游 List.of(null) 的 NPE。
  实际 v3 应用后又确认 formatted XML 的空白 root/wrapper/bean 回归，单独
  3 路径修复后应用。只在 XML 结构上下文处理空白，JSON 错误 list 字符串
  继续失败。没有重跑任何应用 journal。
- XML 使用 JDK 默认 factory，禁用 DTD/实体/外部 DTD/schema/XInclude，单次
  factory 显式限制深度 512；不设置全局环境或 JVM property。名称和 factory
  属性依据 [JDK DOM API](https://docs.oracle.com/en/java/javase/21/docs/api/java.xml/org/w3c/dom/Document.html)
  与 [JAXP 属性](https://docs.oracle.com/en/java/javase/21/docs/api/java.xml/module-summary.html)，并以本机 JDK 21 实测。

## 验证与应用

原始两模块 baseline verify 通过。直接上游 primary 自带测试通过，但独立
兼容样本只有 7/21 相同、中文 XML writer 失败；失败内容保留。适配 v3 后
388 通过且 20/21 相同；随后三个实际格式化空白 XML 反例单独留证，v4 修复。

最终 `event-whitespace-v4.json`：SPI-utils 124 + event-pump 265，共 389 tests /
0 failures / 0 errors / 0 skips；RAT 和既有 JaCoCo gate 通过，不改覆盖政策。
旧 serializer 的 7-test XML report 仍留在目录，但不属于当前 job，计数已按
实际 Running suites 排除。旧 class/test-class 不在当前输出，包中无旧 serializer。

`verification-summary-v4.json`：24 组真实旧读取器/新读取器 getter 对照，23 组
完全相同，仅 DOCTYPE 从接受变为拒绝。新 writer 的 6 个空/完整 JSON/YAML/XML
文件同时被新旧 reader 读取，完整对象一致。10 种编码组合分别覆盖 File/Stream，
Long.MAX_VALUE、中文、结构校验、坏编码和深度限制通过。配置 DTO API 相同；
配置包无 Jackson imports，仍通过 SPI 值类型使用 Jackson 的消费者不受删除影响。

初始 13 路径应用与后续 3 路径修复各有独立 create-only journal/原像和完整
18 仓范围外比对；见 `applied-to-working-tree.json`、
`followup-v4-applied-to-working-tree.json`、`aggregate-source-manifest-v4.json`。
根 POM 先前 pin 通过 previousPin 保留，新 Engine property/managed dependency
为明确 successor；旧 authority 不覆盖。其余 pins、management 1986 inputs /
20 pins、R013 v32 binding 精确。console/PG ID/health/start 不变。

仅下载第三方 SnakeYamlEngine 3.2 到正常 Maven cache；没有 install 本项目或
shadow Kura API，没有 deploy/restart/commit/push/reset/clean。v3 包在 v4 构建前
归档保留。不是完整 reactor、真实设备、Linux 系统/发布或 GUI 验收。

## 关联和续接

`consumer-dependency-delta.json` 补充既有 327 Maven + 1 Cargo/source/assembly/
runtime 关联台账：event-pump 单独消费 SPI JSON/Engine；其配置 DTO API 保留。
7 nonfork 不直接 merge；根 POM 中其它仍在用的 Jackson/版本/依赖未删除。

剩 3 个未应用功能组：CtrlX HTTP/TLS、按实际使用清理依赖、commons-lang3。
完整 97 模块串行构建/全部 Java --fail-at-end/受影响 YOFC 和 OSGi gate 后才
关闭新 7 组 cohort；global accepted 仍 e591，不越过 pending 到 advertised 671。
再返回同一 R013 v32 shadow 完成 factory/private init/imported layout/owner。
原 14 管理组、7 API2、R017、A 外部 CA/custody 和仅 3 external deferred 全部保留。
UNKNOWN 不重发、不 re-export/new-ID 绕过，不重放已闭环现场。目标 active。
