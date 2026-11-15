# 5e0d8a41 — Kura 防火墙字段映射与实际流量

2026-11-07，接续 2e050db1。完整目标 active，本轮 progress。
本轮实现 v3 结构化语义及原 Kura API 双向数据转换，并完成真实完整 Kura 与两 VM 内核事务验证。
provider/PID/原页面写入、中心远程流程、正式 ownership 和早启仍 pending。
合同见 [firewall-kura-candidate-v3.md](../firewall-kura-candidate-v3.md)，
精确源码/制品/状态见 [机读证据](../evidence/firewall-kura-semantics-20261107.json)。

## 实现

新增 agent `src/firewall/model/kura.rs`：独立 IPv4/IPv6 策略、端口/源端口范围、源 CIDR/MAC、
允许/排除接口及旧前缀通配符、端口转发三类规则、手工/自动 NAT 与原 ruleType/空协议。
NAT all/ICMP/常用协议名及数值协议号均为有界类型；不接收 shell 或任意 nft 片段。
原 Kura 目标主机/协议转发范围、空协议的出口接口级 masquerade 语义明确保留。
冲突接口、错误类型、坏行和未知字段先拒绝，不采取旧实现逐行吞错的部分成功行为。

新增 bridge `KuraFirewallCandidate` 的 encode/decode，使用真实 Core API 类型；不依赖 agent
运行，不改 CM 或执行 UDS。导出包 1.1.0 / uses 与实际 net 2.7/firewall 3.0 最低导入同步，
仅补测试 Guava。原 API、GWT、React 与控制中心协议源码未改，未授予 Kura UID 新防火墙权限。
原管理员 UDS 接受显式 schema3；v1/v2 形态保留，版本混合不降级。

仍使用共同系统变更锁、默认关闭闸门、确认/回滚、持久日志和严格回读。
新 v3 用实际 IP header version 条件保持家族边界；nft 数值协议回读和精确依赖规范化解决真实
内核表达差异，不放宽任意规则漂移。未在宿主启用防火墙或创建系统归属标记，没有新 deb 安装。
默认账号/生产首登后台闸门、双语、独立身份、NM、UNKNOWN/RR 与现场保留合同不变。

## 验证与失败现场

前轮 manifest SHA `c66a041bfc29907e53ea7117df02c0af5b526c6c57b2d8c6970ff39af9d32c17`：
331 源码、14 制品、155 证据及 review 全部实时匹配，18 仓 HEAD/branch/完整 dirty 初始一致。
本轮未提交/推送/reset/clean/覆盖远端；仅刷新 agent 与 networking 真实子仓图谱。

宿主首批全量 agent **212 passed / 10 ignored**；实际内核差异修复后，宿主与 Linux 受影响
防火墙/UDS 合同均 **24 passed / 1 ignored**，fmt、严格 clippy、两平台 release 通过。
Java bridge 全部 **63 测试通过**并成功构建；前两次 Java 编译/测试暴露 checked exception 与
测试 Guava 缺失，已正确处理异常并仅补测试依赖。未重放旧 ignored 一次性 nft 测试。

新完整 IDEA `.phase1-dev/firewall-semantics-idea-20261107-0913c8dc` 从桌面 UI 一次启动，
PID **19449**；284 inventory / 实际283 bundles，**288 静态 pins**。
真实 API 生成 2 input、2 portForward、4 NAT，实际产品 bundle 双向转换通过。
原 GWT admin/admin 中文登录成功，无正式 agent 安装时远程菜单隐藏；首登 bypass 仅测试 profile。
该 JVM 已精确停止。后续仅 Rust renderer/比较器修复，Java 及 native 运行集制品未改，未重启旧入口。

Orb 首次 `/opt/yofc-fw-kura-orb-20261107-50f1f897`：apply 回读发现 nft 省略协议依赖及
一项 family 条件，返回 CONFLICT，已自动 ROLLED_BACK，confirmed revision0。两个新 unit 停止；
raw expected/kernel 成为源码回归输入，**623 文件 pins**，原 ELF 和全部数据保留。
随后增加显式 IP version 条件，且只移除同协议 payload 真正隐含的冗余 L4 判断。

Orb 第二次 `/opt/yofc-fw-kura-orb-20261107-302a76d0`：前 23 项流量通过，测试重复使用旧
UDP 五元组，established/related 使断流预期不成立。确认原日志为 WAIT_CONFIRM 后，仅一次
独立正常 daemon 重开，将该未确认 v2 补偿至已确认 v3 revision1 / ROLLED_BACK，原 request/token
保持不变，没有重发 prepare/apply。三新 unit 停止，**635 文件 pins**。新测试改为未建立的新连接。

最终 Orb → Parallels 每处新完整 Kura+生产 ELF、独立三节点 namespace、实际 root UDS：

| 实际检查 | 结果 |
|---|---|
| 端口范围首/中/末、源端口、源地址、MAC、允许/排除接口前缀 | 正例及反例通过 |
| IPv4/IPv6 对照端口与 ICMP 开关 | 不跨地址族开放，必要 IPv6 控制流量保留 |
| v4 UDP DNAT、v6 TCP DNAT及可选 masquerade | 对端载荷/源地址符合配置 |
| 原目标主机/协议 forwarding 范围、v4 手工与 v6 自动 NAT | 实际对端源地址符合旧语义 |
| 空协议出口接口级 masquerade | 真实流量验证，显式回滚恢复确认规则 |
| 未确认 v2 新连接拒绝，生产 agent 重开补偿回 v3 | 原 revision1 与语义规则恢复，无原请求重发 |
| 外部哨兵表、原 GWT 登录后的确认规则 | 新 prepare 拒绝；只读登录不改变规则 |
| 每处25项包检查；宿主 nft 与 IPv4/IPv6 forwarding | 全通过，宿主前后 SHA 相同 |

三处完整 Kura 生成的候选字节完全相同。Linux 动作由测试管理员 UDS 发布，尚未走新的 Java
provider/PID/GWT 写接口；loopback TCP 代理只用于原登录，不能证明新的远程管理通道已连通。

最终 Linux ARM64 ELF SHA `a53cbe8e9a42a76d535e1915050b7e032db19fef5d7be1e3fcec6e41ec6da821`；
macOS SHA `8dbc1787d28f2bc194b3d9f8095b58b67a0850759ab2ad1dae301060e725b644`。
首次两个平台候选均另存保留；旧 bridge target JAR 在新 package 前精确另存，旧制品未丢失。

## 最终状态与待办

最终 Orb `/opt/yofc-fw-kura-orb-20261107-ae273860`，Kura739160 / agent739278 / reopen739477；
三个新 unit 与 echo/代理全停，**638 文件 pins**。三套 Orb 夹具共 **8 新 unit**全 inactive/PID0；
namespace、规则、state、key、请求、日志保留。原四服务仍 active，原 PID244516/244911/286620/268、
原 console26/agent20 包保持不变。

Parallels `/opt/yofc-fw-kura-parallels-20261107-d55c79c5`，Kura2579 / agent2701 / reopen2971。
Kura 实际 CapEff/CapBnd=0，两 agent=1000；三者均 NNP1、既有系统文件只写打开 EROFS30，未截断写入。
三个新 unit 与 echo/代理全停，**822 文件 pins**含私有 JRE184；Ubuntu 正常关机后临时 namespace
自然结束，全部磁盘数据保留，Windows 始终 stopped。
Orb 仍受原 LXC 全局覆盖：agent 自身 NNP1，Kura NNP0、只读挂载未生效，未改全局强行通过。

没有新中心/PG/Vite/防休眠进程，没有修改个人 runtime 或连接192.168.2.125。
下一步仍需新 Firewall provider、原 API/PID 确认值/事务读写与原页面接线、Kura UID 显式默认关闭授权、
NM NAT、威胁/其他写入方正式 ownership adoption/retirement/安装 profile、中心加密持久任务与
UNKNOWN 原命令只读核定、双语远程流程/新管理连接证明、confirmed-only 实际早启/正式部署。

其他原内部 pending 保留：空/部分 CM、不一致 baseline 策略、完整生产组件/外部 PID 恢复、0/±1/
元数据/GWT 编辑、其余运维/Core 安装升级/bootstrap/启动策略交接退役、一致 DB/制品/key 备份恢复
轮换迁移及正式发行。真实 NM/cloud-init/物理硬件、amd64、磁盘满/硬断电为外部 deferred，
两历史 gap 独立；Java 环境可用。禁止重放任何已闭环或本轮保留的一次性入口。
