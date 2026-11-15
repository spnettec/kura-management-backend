# 上游 Profinet GSDML 功能组 v11

2026-11-13。按功能整合 fd7baa848dab5a4845e8c34ae703f8f5e7bbdb9a，
不整仓 merge。证据位于 `target/upstream-logical-sync-20261112/profinet-gsdml-functional-v11/`。
87 路径（81 existing / 6 new）已 source-only 应用；当前 authority 为
`../applied-source-pins-after-gsdml-v11.json`，481 pins。其余完整工作未关闭。

## 来源与关联

85 个上游完整文件逐个验证 Git blob SHA；以完整反向 hunk 重建父版本 e385。
与当前 fork 对照：12 个相同，67 个仅命名空间差异，另外 6 个为两个 Maven 3
POM、两个配置解析入口、两个测试的 Jackson 3 异常处理。Maven 3、.B、构建/
覆盖率/生成配置均保留，只迁移 DOM 所需逻辑及相应测试。

两个驱动仍是独立模块、相同包的各自实现，不在本阶段合并模块。公开 getter/
constructor 签名没有减少，各仅新增 parse(Path)/parse(InputStream)。当前
kura/yofc-iot/kura-yofc-runtime 未检索到直接引用这些 GSDML 类型；不是动态调用
穷尽证明。原 7 nonfork 的 327 Maven+1 Cargo/source/assembly/runtime 关联台账
继续有效，未直接 merge 非 fork。

## 实际 fork 适配

新解析器使用 JDK DOM，禁用 DOCTYPE、实体、外部 DTD/schema、XInclude；保留
属性/子元素映射、顺序、list/null、数值和布尔行为。

直接照搬上游会产生真实兼容性回退：当前 Jackson 3 默认容忍未建模的命名
属性，上游以 Jackson 2 默认值把 5 类 DTO 改成严格拒绝。独立 before/after
探针先捕获两个驱动各 2 个未知字段反例。现在保留 ProfileBody、SystemDefined-
SubmoduleList、Input、Output、DataItem 的 vendor extension 容忍；标量文本
和非法值仍拒绝。对应上游测试按已验证的 fork 合同适配，原始上游文件和失败
对比均保留。未以删除测试掩盖差异。

每个模块仅删除不再直接使用的 jackson-core、XML、annotations 三个依赖，
POM 其它字节保留。全局 Jackson/其它消费者依赖未删除。最终包不含 Jackson
imports；GSDML 类依赖 java.base/java.xml 和既有 generated readwrite 类型。
一次仅给 GSDML 目录运行 jdeps 时缺少自身 generated enum 的诊断保留，补齐
自身 classes 路径后验证；不声称整个包只有 JDK 依赖。

## 验证与保存

- 原始 fork 两模块 baseline verify 通过，类保留供独立进程对照。
- 最终 `gsdml-jackson3-v3.json`：92 tests / 0 failures / 0 errors / 0 skips；RAT、
  原有 JaCoCo gate 和两个 Karaf feature 静态验证通过。原 GSDML coverage
  exclusion 保留，不能把 gate 通过解释成该包已测到某覆盖率。
- 五份实际 vendor 文件和 30 份边界文档，两个驱动共 70 组，141828 行结构和值
  记录：68 组完全相同，2 组只有明确的 DOCTYPE 从接受变为拒绝。
- 公开 API 对照无移除；新 JAR class major 65，两个 manifest 无 Jackson import。
- 各驱动新增实际 loopback HTTP 请求计数测试：DTD/普通与参数实体先拒绝，
  schema/XInclude 元数据不取外部资源，请求数为零；UTF-16 BOM/声明和中文保留。
  没有访问真实设备或重放已闭环管理场景。
- `applied-to-working-tree.json`：create-only journal/原像，18 仓范围外
  HEAD/branch/index/staged/refs/remotes/tracked/untracked 源码与 retained metadata
  相同；console/PG ID/health/start 不变。旧 394 pins、1986 management inputs /
  20 pins、R013 v32 binding 精确；v10 的 535 份绑定证据也未改变。

未 shared install、deploy、restart、commit、push、reset、clean。不是完整
97 模块/全部 Java --fail-at-end 或真实 Linux/硬件/发布验收；UNKNOWN 不重发。

## 续接

剩 4 个未应用组：event-pump、CtrlX HTTP/TLS、按真实使用清理依赖、commons-lang3。
统一完整 reactor 和受影响 YOFC/OSGi gate 后才关闭新 7 组 cohort；global
accepted 仍 e591，已知 advertised 671 不跨过 pending。然后返回同一 R013 v32
shadow 完成 factory/private init/imported layout/owner 整链。原 14 管理组 +
7 API2 + R017、A 外部 CA/custody、仅 3 external deferred 全部保留。
