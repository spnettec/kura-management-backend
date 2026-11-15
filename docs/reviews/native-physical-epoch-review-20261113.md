# V47：原始物理 epoch 凭据（2026-11-13）

## 实施结果

本轮补上合法物理 source 续接的前置条件，没有把“同名/同句柄/同字节”当成跨启动实例的归属证明。SAME shadow 2083源码+4fixture，较V46改9个已有文件并新增2个Rust文件；88existing+97new−3fixture=182路径仍未整体应用。普通source观察续接、boot restore及真实registered完整生命周期仍未完成。

`native/epoch.rs` 从固定Linux proc路径采样规范非nil boot UUID和当前线程network namespace的device/inode，并重复采样检查稳定性；普通程序不接收调用方proc路径、环境变量或JSON授权替身。序列化epoch仅作证据，命令不能据此提供新鲜授权。

首次native effect在观察原规则前采样epoch，在INTENT前后及readback后再次检查。原ownership journal的native step使用schema3保存首次采样；实际effect必须有完整generation fence，仍只调用一次固定native sender。namespace/boot变化时保留原intent，不将相同kernel字节补成完成，不重发或换ID。

初始化文档schema2逐字继承首次native intent的epoch，初始化观察或发布前epoch变化会拒绝；旧文档、baseline0、原owner/startup/lease均不回填或改写。旧schema1/2已完成凭据仍可按历史查询；缺少epoch的未决native intent始终UNKNOWN，不能借当前匹配ruleset补成APPLIED。旧无epoch记录也不能得到新的初始化/逆向授权。

当前physical source由原初始化携带epoch，确认/逆向沿同一历史继承。普通coordinator在新鲜Core/OS/NM上下文中单独核对实际epoch，再检查完整raw/native generation，并将epoch纳入runtime摘要；normal `require_native`的完整相等约束保持。**这不是foreign/NM generation观察续接实现**，也不替代变化owned身份的合法恢复。

## 验证与保留

最终199个关联输入（121个Rust文件）；12项epoch契约和严格Clippy通过。最终宿主全量653通过、0失败、28忽略，inventory681。Linux普通/测试ELF、严格Clippy及128项选定契约通过、11项忽略，未全量执行Linux测试。新增实际Linux epoch用例只读取隔离构建容器的proc身份；其余为合成kernel/临时文件系统存储契约，没有真实owner lease或新的nft/network写场景。普通ELF SHA为 `0447e15ed10afe2874ba9f277e126265d55f2868f980ebe957164311e1b6bef6`。

早期host `epoch-full-v4` 和Linux `linux-epoch-v47` 在审查发现legacy intent分支后主动停止，日志/源码/容器保留；分别为SIGINT和容器停止退出137，不是通过或OOM。最终修复后采用新source pin及新的验证任务，没有重启旧任务。V46及以下所有CLOSED一次性现场仍禁止重放。

1799个Java输入及9个留存JAR再次逐字核对，精确复用2462测试，未重新执行Java验收。原18仓按G022新权威核对，1986+20、509存在+89删除保留、凭据141+5、共享API522均保持；正常console/PG的ID/StartedAt/healthy、Orb运行、Parallels停止和57个停止任务容器保留。V41历史运行中断仍单独保留。无管理源码应用、正常服务重启、共享API安装、清盘、提交或推送。

V46封存检查点的`files`清单为2081项，但继承的sourceInputs/sourceCount/currentComponent/predecessor字段仍是旧V44/V43值。V47从实际files清单重算2083项，明确指向V46前驱；旧封存件保持原字节。后续以V47 `current-state.json`、source/candidate清单及final binding读取当前状态，旧progress的兼容历史字段不作为最新计数。

## 上游与后续

G022的5路径功能合入与509项新原源码权威保持。最新只读远端检查：11fork中9个游标一致，新增Kura `f0b2120`（TypeUtil十六进制API/包导出1.3）与GWT UI `1c51246`（无service.pid的factory发现/配置PID准入），尚未改源或推进游标。GWT逻辑合入须保留YOFC本地化、唯一factory与rootFactory校验及Java11目标，并验证列表PID与真实CM创建目标一致。7nonfork跳过直接merge，327Maven+1Cargo及源迁移关联继续用于影响审查；见新tip的pending-functional-review。

恢复后先修复并验证已冻结的G023独立候选，在新原源码权威与同一shadow间做有证据的协调，再继续SAME transaction/current/kura-history内的明确物理观察事件。观察保持逻辑revision、已证明owned raw/handles/顺序/声明和receipt来源；同epoch只是必要条件，不能伪造ACK或靠名称认领。未决、孤立或缺失记录必须阻断新ID；区分无kernel effect但推进physical head的观察与不推进head的取消/NACK，保留首次发布及“已尝试后缺失不重建”的约束。

owned身份恢复、Kura-off authority、自动到期/原调度、UI/task/protocol/终态倒计时、真实family/NAT/multiWAN矩阵、新完整registered迁移/boot/recovery/退役再接管、182路径整体guarded apply及原14组/7API2/R017/API1.2-1.8运行配对/GUI部署等完整台账仍内部pending。A外部CA/custody保持；仅3externaldeferred（NM/cloud-init/物理硬件、amd64、disk-full/hard-power）。UNKNOWN不重发、不换ID、不重导出。

宿主full-v7的651通过/1失败/28忽略保留：新增真实proc读取遮蔽了Mac合成controller原有model-drift断言。修复只在cfg(test)增加显式线程内epoch注入及RAII恢复；普通路径、原断言和读写顺序不变。精确回归v8与最终full-v11/Clippy-v10/Linux-accepted均通过。Linux-final旧127项结果为修复前源码，不作最终权威。

G023已在独立目录准备9路径候选，尚未应用；package-v1因GwtServerUtil缺少ComponentDescriptionDTO导入而失败，三个编译错误，web2测试未执行，不能记为验收。2783输入/失败日志/已产出JUnit报告冻结在upstream-utility-factory-v18，游标未推进；尚需审查SCR configurationPid默认值/$及CM目标一致性。

周用量100%，按用户预授权只做必要收尾并暂停；ordinaryUsageAllowed=true不视为周额度剩余。保留三次reset credits，不自动使用。接口预计恢复时间2026-11-18 23:08:50 Asia/Shanghai，恢复时仍需实时确认；不设置自动续跑。
