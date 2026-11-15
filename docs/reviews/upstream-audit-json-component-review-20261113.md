# 上游审计日志功能组 v10：兼容投影与秘密值脱敏

2026-11-13，续接 JSON codec v9，按逻辑而非整仓 merge。
上游 e385cd8d614c5dd59cd3b379d2df87f74023a830，证据与脚本位于
`target/upstream-logical-sync-20261112/audit-json-functional-v10/`。

已 source-only 应用 7 路径（5 existing / 2 new）。新 authority 为
`../applied-source-pins-after-audit-json-v10.json`，共 394 pins。
不是全量 reactor、发行集或 OSGi 现场验收，global accepted tip 仍 e591，
已知 advertised 671 不跨过余下工作。R013 v32 candidate 不应用。

## 改动及原因

图谱 AuditLog.write 有 91 caller，包含重载与启发式误连；继续按显式
AuditLogEventType 调用检索，140 个 callsite 中唯一三参数生产调用是
BaseTransportInstance 的实际 transport configuration。不是宣称动态调用穷尽。
实际 TLS/PSK 配置使用 public fields；直接上游 JDK Json 替换会丢失它们。

因此保留 Jackson 3.2.4 的 bean/annotation 投影，共享不可变 mapper；受约束
stream 限制循环/嵌套，然后由新 JDK Json 写出普通树。保留长整数、小数精度、
ISO 时间、二进制、别名、忽略规则和非秘密 JsonValue。仍使用 Jackson 的依赖
不能随上游依赖清理删除。stream 中间表示有额外复制成本，实际调用在配置审计处。

秘密值在读取前遮蔽：支持字段、继承 getter/JSON 别名、数组位置和 scalar
projection。Secret 扩展到 METHOD，实际 PSK getPskKeyBytes 明确标记，避免
只遮蔽十六进制字段却泄漏派生 Base64。字段对应 connection-string 参数的
规则不变，未标记的 PSK identity/普通配置保持可诊断。该标记不声称自动修复
任意手写日志/toString 或恶意自定义 serializer。序列化失败写合法 JSON
"<serialization-failed>"，不调用对象 toString、不输出异常消息中的值。

## 证据

- 原实现与新契约先出现 3 failures / 1 error；v2 valueToTree 对循环 Map 仍
  StackOverflow，保留失败记录，改用实际 stream 深度边界。
- 实际 TLS/PSK 配置探针先证明密码、PSK hex/Base64 泄漏；v3 字段脱敏仍漏
  派生 byte[]，方法标记后已消除。仅使用测试哨兵，探针不构造网络传输。
- 最终 `audit-final-v6.json`：5 模块 verify，406 tests / 0 failures / 0 errors /
  0 skips；RAT 通过，其余 4 模块 JaCoCo gate 通过。impl 原 POM 已有 skip
  保留，不声称该 gate 通过。TLS 自身单元测试含本地握手，不代表物理 PLC。
- v5 新增时间断言误预期 numeric epoch，实际保留 ISO，修正测试后 v6 通过；
  失败证据不覆盖。helper javac -Xlint:deprecation 无警告。
- `actual-transport-probe/final-v6-result.json`：实际 source 配置类、public 字段
  丢失反例、各秘密形式无输出、identity/timeout 保留且配置对象不被修改。
- 包装 class major 65，OSGi imports 保留 SPI config / 新 JSON [2,3) 和
  tools.jackson [3.2,4)；不是实际 resolver 验收。
- `applied-to-working-tree.json`：create-only journal、原像、全 18 仓 scope 外
  HEAD/branch/refs/remotes/index/staged/tracked/untracked 源码及 retained metadata
  前后相同；console/PG ID/health/start 不变。387 旧 upstream pins、1986 management
  inputs / 20 pins、R013 v32 binding 精确。未声称完整 target 字节哈希。

## 后续

余下 5 个未应用功能组：双 Profinet GSDML DOM、event-pump 多格式、CtrlX
HTTP/TLS、按真实使用情况清理依赖、commons-lang3。最后统一完整 97 模块串行
构建及全部 Java --fail-at-end 测试、受影响 YOFC/OSGi 消费验证；不因本组件通过
而关闭全 cohort。7 nonfork 不直接 merge，327 Maven + 1 Cargo/source/assembly/
runtime 关系继续保留。

再返回同一 R013 v32 shadow，完成 factory/private initialization/imported layout/
owner 整链，保留原 14 管理组 + 7 API2 + R017。仅 3 external deferred 不变。
A 外部 CA 与 custody 决策不变。UNKNOWN 不重发、不 re-export/new-ID 绕过；
不重放已闭环现场，不 commit/push/reset/clean，不替换 shared Kura API。
