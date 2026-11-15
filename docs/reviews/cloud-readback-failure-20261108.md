# 1274e471：已确认写入的双语读回失败与视图恢复

日期：2026-11-08；前序冻结：8c0383b0（cloud-camel-rebaseline-20261108.json）。本轮为 progress，目标保持 active，整体改造未完成。

## 实施与结果

- Raw MQTT 订阅工厂中文名称和描述由“原生云发布”改为“原生云订阅”；资源已打包，原 GWT 英文/中文 PubSub 选择框实际显示和 PID 映射已核实。仅打开选择框并取消，没有创建 PubSub。
- 原始英文故障路径真实完成一次 Core 写入和快照，随后五次版本读回失败，显示“Changes were saved, but the current configuration could not be read. Reload this view before making further changes.”；中文路径显示“更改已保存，但无法读取当前配置。请重新加载此视图后再进行修改。”。按钮和输入均禁写，Core uncertainty 集为空，没有重发写入或标记 UNKNOWN。
- 发现并修复：已保存但读回失败仍保留 dirty，导致导航误报未保存；服务侧栏重选复用旧配置，可能显示旧值和旧版本。EntryClassUi 现在按选择重新读取指定 PID 的配置，配合选择代次检查；ServicesUi 读回失败清除 dirty 并保持 baseline unavailable。初始或重新读取缺少有效版本的视图仍可显示，但字段及所有写操作禁用。
- ConfigurationBaseline.selectDisplay 只选择唯一匹配 PID 并复制显示数据；select 仍要求有效版本，显示数据不能据此获得写权限。新增测试覆盖只读实际值、无版本、复制隔离、重复及非目标 PID。

## 隔离原生验证

IDEA 完整 Kura、283 项 runtime inventory（282 个实际观察 bundle 加一个探针），无 agent。原故障实例 PID3297 与修复实例 PID4428 均已停止，原目录、日志、命令 admission/receipt、快照观察原字节和两份 scene-before-stop 均保留。探针只覆盖本实例 Clock emulator 的 ConfigurationRevisionService 版本读取；高 ranking 服务被原 GWT 选中，实际写入始终转给原 Core，快照成功返回后才拦截读取。emulator 不执行系统时钟同步。此证据不等同真实 Linux Clock、网络、防火墙或三成员云栈读回。

| 实例/步骤 | 实际写入/确认累计 | 被拦版本读取累计 | CM change count | 实际 enabled |
|---|---:|---:|---:|---|
| 原故障英文保存 | 1/1 | 5 | 2 | true |
| 原故障整页读取后独立新保存 | 2/2 | 5 | 3 | false |
| 原故障中文保存 | 3/3 | 10 | 4 | true |
| 修复后故障保存 | 1/1 | 5 | 2 | true |
| 修复后导航并显式重读，读取仍不可用 | 1/1 | 6 | 2 | true |
| 只解除版本读取门控并重选同一服务 | 1/1 | 6 | 2 | true |
| 修复后独立新保存 | 2/2 | 6 | 3 | false |

修复实例全过程 serviceId154 保持不变，Core 配对值与 CM 值一致，uncertainty 为空。已保存页面直接离开没有未保存弹窗；缺版本重读显示实际 true 但禁写；解除只读门控后无需整页刷新，重选同一服务显示实际 true 且输入可编辑，之后 false 是独立新用户意图而非重试。另一个只读统计命令 blocked-reselection-stats-01 仍为5：关闭警告动画期间的第一次重选没有触发读取，未计为成功；随后导航并重选的明确读取才使计数变为6。

## 构建、证据与边界

JDK21 / Maven3.10，显式本机 Maven 缓存。Raw 资源包 install 成功（资源改动，跳过 Java 测试）；最终 UI reactor install 成功，GWT login/denali 各六个排列，207 tests/0 failures/0 errors/0 skipped。第一轮206报告及 JAR另存 intermediate-first-build；旧8c所钉 UI/Raw可变原字节在 before 目录。Core150、helper12、Camel8等前序测试保留而未重跑。图谱按 UI/cloud 子仓检索与更新，精确改动路径覆盖为 best effort、metadata_match；源文件及实际包/浏览器另行验证。

证据根：target/cloud-readback-failure-20261108；清单：docs/evidence/cloud-readback-failure-20261108.json；机器判断：native-result.json。所有自动读取次数均指本次真实保存的 ConfigurationSave 五次版本尝试；第六次是显式服务读取，不能混记为自动重试。没有新 Linux 夹具，也没有重放已冻结的一次性验收。18仓和原 Orb四服务及 package26/20 在封口实时核对，Parallels两 VM保持 stopped。

## 完整台账续接

本轮只关闭选定英文/中文 acknowledged-readback failure、通用服务恢复和 Raw 中文工厂重名子项。前序14项 pending逐项继承并缩小已证明范围，原26项 disposition 链仍保留；外部 deferred 原样独立继承。

仍需推进：英文/新 Linux 云/Camel及生命周期、restricted与三成员云栈读回；完整 dirty/并发/选择/成员变化与 provider/RPC 时序；全部双语/provider体验及类型/密码/空字段等配对值；所有专用业务计划/工厂健康/审计与写者所有权；durable UNKNOWN与 lost-reply业务裁决、重启恢复；完整空 Core/外部 PID恢复；NM所有权及安装、远程防火墙与受影响管理链、早期启动、备份/恢复/密钥/迁移/发行。历史初始 CONFLICT 的精确持有者仍未证实，既有证据缺口不补造、不升级为通过。

不提交推送，不 reset/clean，不清理个人运行集，不连接192.168.2.125。下一项优先 restricted/三成员云栈的真实配对保存读回及独立新保存，再按完整台账推进。
