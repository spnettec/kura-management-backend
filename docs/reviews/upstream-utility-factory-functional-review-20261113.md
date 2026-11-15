# G023：十六进制工具与 factory PID 功能整合（2026-11-13）

已按功能吸收 Kura f0b2120 与 management-ui 1c51246，原工作树共10路径（Core3/UI7），未提交或推送。同一R013 shadow对应7路径同步，2086源码+4fixture；R013的182路径未整体应用。上游当前源码权威519存在+89删除，接受游标停在这两项已验证提交。

TypeUtil新增小写hex方法；将上游bnd导出改动适配到YOFC手写MANIFEST，util.base1.3.0。包内Core class major65，Web major55；8处手写消费者范围均接受1.3，Maven3/JDK21/Java11及各fork边界保留。

GWT用SCR描述发现factory；只选择真实配置PID，避免无绑定service.pid别名把CM创建指向错误目标。模糊多PID不自行认领。保留YOFC本地化及G008唯一factory/rootFactory检查。默认PID与注解美元占位符语义按[OSGi DTO文档](https://docs.osgi.org/javadoc/osgi.cmpn/8.0.0/org/osgi/service/component/runtime/dto/ComponentDescriptionDTO.html#configurationPid)和[DS规范112.4.3](https://docs.osgi.org/specification/osgi.cmpn/8.0.0/service.component.html)核对；不增加通用运行时别名替换。

私有候选7模块774测试通过、GWT编译通过；同一R013 shadow新1802输入、193JUnit报告、2493测试、9JAR均通过，不再使用旧Java1799/2462不变结论。164份实际JUnit classpath指向已验证的私有util1.3 JAR；共享API522及共享Maven缓存未安装候选制品。两套测试有重叠，不能相加为独立用例总数。

v18缺失DTO导入失败、v19-v2普通JVM缺少OSGi元数据查询边界的5错误、v3旧G008 DTO夹具未填实际配置PID的1错误，均保留日志/源码/报告。修复测试边界并补真实SCR字段，不削弱生产校验；增加本地化PID、重复factory、无绑定别名拒绝回归，v4全部通过。

非fork仍分析关联：源OSGI-INF范围12个相关DS描述、无解析错误；YOFC ABPLC的service.pid不同于配置PID，但其DS默认PID和metatype factoryPid均为etherip.driver.ABPlcDriver，新规范化目标一致，未改下游应用源码。这是源码关系审查，不代表实际驱动/提供者运行验收。完整11fork/7nonfork、327Maven+1Cargo及装配/源迁移关系继续保留。

最新只读tip检查10一致、1新：Kura7eca88b比本次接受游标又多2项SSL/Crypto破坏性API提交。已单列内部兼容cohort待审，不直接合入，不只改导入导出范围；原7API2/R017不因此闭环。仅3externaldeferred维持原范围。

原18仓除本次10路径及审查文档外保持；分支/HEAD/index/staged/refs/remotes保持。正常console/PG/Orb状态、Parallels停止、57停止任务容器与V41历史中断证据保留。无正常服务重启、部署、清盘、旧CLOSED重放或UNKNOWN重发。证据见target/upstream-logical-sync-20261112/upstream-utility-factory-v19/current-state.json、final-binding.json及关联V48绑定。
