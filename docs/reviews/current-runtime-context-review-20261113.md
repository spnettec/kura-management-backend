# R013 V46 当前运行上下文候选审查

同一隔离工作空间，2081源码输入+4fixture reactors；88已有+95新−3fixture=180生产/测试路径未整体应用。本轮相对V45改7个路径，增加runtime.rs和runtime_cases.rs。原18仓、原始初始化、owner租约、startup receipt、baseline0和正常运行现场保留，无提交/推送/API安装/普通agent部署。

V45的current_runtime_challenge复用了原启动activation证明，要求实际unit与原启动ACK的unit完全相同。V46新增独立、只读、不可从JSON反序列化的当前上下文；原completion/current/activation严格合同保持不变。新路径必须有SAME COMMITTED原owner、完整原startup终态、六项选中文件及其文件身份、原配置/stop-policy定义，并在固定原unit及原cgroup范围采集当前OS事实。

Linux正常入口固定读取/proc，不接受调用方提供的proc路径或ready字段。采样boot UUID、PID启动ticks、UID/GID四元组、cmdline摘要、cgroup及可执行文件路径/device/inode/size/ctime身份；限制读取字节，拒绝不完整/僵尸/错误PID、自身或外来cgroup、空命令、缺失exe等。两次采样unit/process，并复核原startup锚点；Core反向读取后再复核整套上下文。字节/文件或进程/boot变化不能继续使用旧challenge。

实际Core见证仍来自注册通道的三固定PID、普通profile、framework/JVM association、Owner与网络写入drain。新PID或invocation还必须使用不同于原startup完成见证的observer，不能把旧association换个PID重放。原启动回执只作为历史锚点，不改写、不复制为新的启动ACK、不创建第二份owner journal。新的当前上下文加入V45普通事务的runtime哈希，不能替代实际native raw/generation或成为存储授权的写入许可。

四组新增测试使用显式synthetic proc和startup-child临时文件；没有真实COMMITTED根owner租约、注册JVM或系统服务，不能算新运行验收。覆盖只读保存原历史、PID/boot/credential变化、新invocation与旧observer重放、保持旧activation拒绝、错误proc身份/自cgroup、缺失/替换选中材料和服务定义。早期v2测试导入编译失败、v3一项测试误替换保留旧profile而非实际selected profile，原日志/源码保留；v4改为真实selected文件并确认inode改变。

最终宿主全量、严格Clippy及Linux普通/测试ELF和精选契约以verification-summary-v46.json及197关联输入清单为准，其中119个.rs；Linux精选而非全量。Java1799输入与9生产JAR字节不变，精确复用2462测试。正常ELF排除synthetic-proc等测试入口标记，普通agent未运行，没有新物理内核场景。

剩余内部实施：外来/NM generation变化后的合法current-source observation续接，保留逻辑策略/版本及Kura-owned raw、handle、声明和回执身份；不能把观察伪装成ACK或凭名称重新接纳。owned身份变化/boot restore需另行合法恢复证明；Kura-off confirmed authority、导入事务自动到期/原查询调度、UI/task/protocol/终态倒计时、实际NAT/家族/多WAN/设备矩阵、新完整注册生命周期/初始化退役/再接纳、180整体guardedapply及原14组+7API2+R017/API1.2-build1.8等完整台账仍pending。

上游周期检查发现5943c897 service.pid修复。G021逐项核对27生产类：核心21、非fork kura-cloud6；现有手写DS的PID/String/原DSnamespace与目标一致，27个现有JAR的XML字节及Service-Component声明也一致，保留现有实现和YOFC额外接口，无源码变动或git merge/fetch/ref变更。上游新增JUnit4测试不盲拷；其无配置/有CM配置时PID查找及单值断言纳入新联合运行/Provider配对内部待办，不称执行通过。审查游标推进到5943c897，旧20组/73处置、508present+89absence及11fork/7nonfork327Maven+1Cargo联合来源保留，附加G021现有等价审查。详见target/upstream-logical-sync-20261112/new-servicepid-5943c897/functional-review.json。

仅3 external deferred不变：NM/cloud-init/物理硬件，amd64，disk-full/hard-power。所有CLOSED场景不重放，V36v1不查询/补偿/重发；UNKNOWN不重发/换ID/重导出。历史V41 ENOSPC与V43容量guard/cache回收保留，不当作计划验收。本阶段不重启正常服务、不清盘，最终54任务容器停止。

V46截止复核另发现PLC4X ea42d3d新增3提交：Go依赖/C# SLMP功能不直接并入Java fork，父POM exec3.7.0/build-helper3.8.0/xml1.4.0需先验证Maven3/JDK21/IDEA/代码生成及yofc-iot消费者影响。尚未合入、未推进PLC reviewed cursor，记录upstream-plc-pending.json；这是内部待审，非外部deferred。下一步先处理此功能审查，再继续current-source续接。

最终宿主 641 pass / 28 ignored；Linux 115 selected pass / 11 ignored，strict Clippy 与普通/测试 ELF 完成。普通 ELF SHA256 `fd04aa26dd617f9739b41bd969316484339cf7636d91bb539e700503da789776`。周额度 95%、ordinaryAllowed=True；额度与保管现场见本组件证据。
