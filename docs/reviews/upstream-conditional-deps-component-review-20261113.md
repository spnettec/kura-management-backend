# 上游依赖清理功能组 v14

2026-11-13，整合 972af8504cf0bf0d57b1ef567f9e274ce5e2716e 的可用部分。
实际 source-only 应用 5 个已有文件；原像、create-only journal、验证和证据在
`target/upstream-logical-sync-20261112/conditional-deps-functional-v14/`。

保留 Jackson 3.2.4 JSON/databind、annotations 2.22 和 YAML，因为实际源码仍有
审计日志的兼容投影/脱敏以及 legacy scraper。不能照搬上游“全部不用 Jackson”
的结论。移除仅 root management 的 XML/Woodstox/stax2，以及 EIP 强制 optional
注解 import、S7 未使用的 test 注解依赖。修正 XML read/write buffer 教程，
发布说明使用 fork 的真实行为。静态源码扫描有 7 个实际 Jackson 用户文件，位于
审计和 scraper；此前 v12 泛指 SPI-values/driver 的理由不精确，本次明确替代其
解释，不改写已封存旧证据。SPI-values/EIP/S7 未发现这些源码 import。

实际 root active-Java dependency:tree 98 个项目（含 website）执行成功，XML/
Woodstox/stax2 依赖边为 0；该只读分析不是完整构建。最终构建仍应采用 97 个
项目、排除 website 的固定范围。保留 Maven 3、Java 21、.B 版本、实际 fork POM。
7 nonfork 不做直接 merge，327 Maven+1 Cargo 关联台账保留。

私有 EIP+S7 完整 verify：531 tests，527 passed / 4 skipped，无失败；RAT、
原有覆盖率门槛和 Karaf feature 静态验证均通过。4 项跳过来自测试进程继承旧
Docker Desktop socket，而真实 Docker 在 OrbStack。仅为补跑进程指定真实 socket，
使用既有 EipDockerIT 和实际 cpppo 容器，4 项全通过。因此当前独立用例 531 全过，
无未解决 skip。首次 focused v2 命令缺 JaCoCo 初始化，JVM 未启动；保留诊断，
v3 按插件实际参数初始化后成功。没有改全局 Docker 配置，没有改变源码或覆盖率
门槛来绕过失败。focused 只执行 IT，不把它称为重新完整 verify。

新 EIP manifest 没有 Jackson annotation import。没有新增镜像单元测试；使用
既有功能回归。测试容器由 Testcontainers 收尾，正常 console/PG ID、健康、启动
时间不变。没有替换正常服务或使用真实设备写入。

应用前后所有 18 仓范围外 HEAD/branch/refs/remotes/index/staged/tracked 及
untracked source/retained metadata 精确；源输入 381、此前 503 非重叠 pins、
1986 management inputs /20 pins、R013 v32 和 CtrlX v13 binding 精确。root POM
前 pin 通过 previousPin 和实际原像保留。当前 authority 为
`../applied-source-pins-after-conditional-deps-v14.json` 的 508 present pins，加
`../source-retirements-after-ctrlx-v13.json` 的 89 absence paths，必须同时验证。
不声称对所有大型 target 作完整内容 hash。

剩 commons-lang3 3.22 功能组，然后完整 97 模块串行构建/install、全部 Java
--fail-at-end 和受影响 YOFC/OSGi 消费验证。新 7 组 cohort 尚未关闭，global
accepted 仍 e591，不推进到 advertised 671。之后回同一 R013 v32 shadow 完成
factory/private init/imported layout/owner 整链，137 路径仍 UNAPPLIED。

未 install 本项目/shadow Kura API，未 deploy/restart/commit/push/reset/clean。
UNKNOWN 不重发，不重放已闭环现场。原 14 管理组、7 API2、R017、A 外部 CA/
custody 和仅 3 external deferred 保留，目标 active。
