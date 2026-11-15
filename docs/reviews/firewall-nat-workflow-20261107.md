# c5564ec4 — 防火墙 v2 本地事务与真实 NAT 流量

2026-11-07，接续 ff72eb79。整体目标 active，本轮为 progress。
本轮交付 agent 本地防火墙的地址族明确模型、转发和基础 NAT；Kura 原接口适配、
中心远程业务和早期加载继续 pending，不把本轮计为完整防火墙产品或正式发行。
合同见 [firewall-transactions-v2.md](../firewall-transactions-v2.md)，
机器证据见 [firewall-nat-workflow-20261107.json](../evidence/firewall-nat-workflow-20261107.json)。

## 实现与保留约束

- 新 `src/firewall/model.rs` 接受严格版本 2、明确 IPv4/IPv6 的 input、forward、DNAT、
  SNAT 和 masquerade。总计最多 256 条，拒绝未知字段、非法接口、地址族混用和端口注入。
  NAT 不隐式放行转发，不修改路由或 sysctl。对应地址族的系统转发未开启时，
  prepare/apply/commit 均拒绝。默认写闸门仍关闭。
- 仅原管理员 UDS `firewall.prepare` 增加独立 candidate 形态；不开放 Kura UID 新权限，
  不增加 gRPC operation、HTTP 入口或 React 页面。v1 语义和旧日志可继续读取，
  v1/v2 能双向回滚至上一已确认版本，revision 与共享系统变更锁沿用原实现。
- nft 只在自有 `inet yofc_kura` 表内单批替换。完整比对链身份/type/hook/prio/policy、
  有序规则表达式和附加属性，未知对象拒绝。内核省略的同地址族冗余依赖作等价规范化，
  冲突的地址族条件仍可检测。回读不符先另存原始内核观察，再回滚。
- 新测试保留了真实 nft 1.0.9 的 candidate/kernel 捕获，避免只有 fake renderer 自证。
  既有 admin/admin、首登强制改密后台闸门、双语、身份归属、NM 与 Kura 独立使用合同不变。
  未启用任何生产防火墙、网络接管或自动迁移。

## 本轮验证

初始 18 仓 HEAD、branch、完整 dirty 与 ff72eb79 完全一致；前轮 308 源码、9 制品、
188 证据共 505 pins 实时匹配。没有 reset/clean/提交/推送/远端覆盖，也没有使用子代理。
仅对真实 agent 子仓刷新图谱，不索引父工作空间。

宿主首批完整 agent 回归 **204 passed / 10 ignored**；fmt、严格 clippy、发布构建通过。
新增实际 nft 规范化修复后，按受影响范围重跑 **16 passed / 1 ignored** 的防火墙/UDS 合同、
严格检查和发布构建；Linux 同批 **16 passed / 1 ignored**。原 ignored 实际 nft 一次性测试
未重放，本轮使用全新生产 daemon 与网络命名空间完成真实业务检查。

原 GWT/Java/React/protocol 制品没有修改，无对应旧一次性写入验收重放。
新的完整隔离 Kura 运行集（283 inventory）从 IDEA UI 一次启动，PID **4488**，
HTTP 18482 返回 200。浏览器 admin/admin 登录成功，无 agent 安装时远程管理菜单不显示。
该兼容夹具沿用显式 test-only 首登 bypass；生产首登后台闸门源码未改。
完成后精确停止这一 JVM，profile、快照和日志保留：
`.phase1-dev/firewall-v2-idea-20261107-d55bda68`。
该检查证明原 Kura 独立启动/登录，不冒称 Mac 实际执行了 nft。

随后在 OrbStack，再在现有 Parallels Ubuntu 26.04 内，各用全新的三节点网络命名空间，
真实生产 agent、实际 root UDS、systemd CAP_NET_ADMIN/NoNewPrivileges：

| 范围 | 结果 |
|---|---|
| 默认写关闭、缺少对应系统转发开关 | 拒绝，未产生候选事务 |
| IPv4/IPv6 TCP/UDP 入站规则与同端口另一地址族 | 允许/拒绝符合明确地址族 |
| 双地址族 TCP/UDP 普通转发与 DNAT 端口映射 | 服务端收到实际载荷和原源地址 |
| IPv4/IPv6 SNAT 与 masquerade | 对端观察到固定或出口接口地址 |
| 确认、显式回滚、关闭候选的新流量、进程重开补偿 | revision 1 保留，最终 ROLLED_BACK |
| 自有表外的防火墙管理者 | 新候选拒绝 |
| 两处各 26 项流量检查 | 全部通过 |
| 两 VM 宿主 nft 与系统转发开关 | 各自前后 SHA 完全一致 |

Parallels 三个新 agent PID **2541/2593/2874**，均实际 CapEff/CapBnd
`0000000000001000`、NoNewPrivs=1；通过进程根目录以 O_WRONLY 打开既有文件、未截断/写入，
均返回 EROFS 30，证明实际只读挂载。Orb 本轮请求了相同沙箱，未额外核定实际只读挂载，
不沿用 Parallels 的结果代替 Orb 的已知全局 override 边界。

最终成功夹具：

- Orb `/opt/yofc-fw-v2-orb-20261107-a7a0955d`，3 个新 unit 均 stopped，
  namespace/rules/state/key/SQLite/日志保留。
- Parallels `/opt/yofc-fw-v2-parallels-20261107-f431bdd1`，3 个新 unit 均 stopped，
  全部磁盘现场保留；正常关机后临时内核 namespace 自然结束，不声称仍活动。

## 失败与纠正

Orb 首次 `/opt/yofc-fw-v2-orb-20261107-71adf99d`：实际 nft 省略冗余协议依赖，
新比较器错误地判定漂移，事务自动 ROLLED_BACK。保留原日志、candidate、独立诊断命名空间、
实际 kernel JSON，并增加相应合同后才重新构建。

第二次 `/opt/yofc-fw-v2-orb-20261107-b7e3e86d`：真实流量已通，测试却把 IPv6 masquerade
固定预期为出口接口第一地址。改为检查实际出口接口地址集合；已知 WAIT_CONFIRM 候选通过
独立正常 agent 重开补偿为 ROLLED_BACK，未重发原请求，随后停止该新恢复 unit。
两次失败夹具均保留；后续验证另建全新夹具，未在失败夹具重放原事务。

## 制品与最终状态

最终 Linux ARM64 ELF SHA256：
`5d51aa2f0017a25923c956fda9c48d6ff6d0dae138cb1a0eebc7dc5ae22e8136`；
macOS Mach-O 可执行文件 SHA256：
`210450445db9edbd7a07a2f3d25508faba53f5b4f1620e4cba8f869a06d8e9d3`。
原制品全部保留；未制作或安装新 deb，不将源代码增量冒称已发行。

本轮新 Mac JVM、agent、echo、构建/测试进程均结束；Orb 三个夹具合计 **8 个新 unit**
（含一次独立补偿）均 inactive/MainPID=0。Parallels Ubuntu 已正常 stopped，Windows 仍 stopped。
原 Orb 四个服务继续 active，原 PID **244516/244911/286620/268**，原 console
`0.1.0~package20261102-26`、agent `0.1.0~package20261103-20` 包未替换。
无新中心、PG、Vite 或防休眠服务；所有旧夹具、state、密钥、备份、quarantine、卷保留。

## 下一步与边界

防火墙内部 pending：原 Kura Firewall API/PID/页面的完整语义适配；NM/旧 Java firewall writer、
恢复脚本、威胁模块与其他防火墙管理者的明确所有权交接；中心加密持久任务、UNKNOWN 只读核定、
双语远程流程及受影响管理路径的新连接确认；仅加载 confirmed 的实际早启和正式部署。
当前本地 token 不能证明中心新管理路径已经连通。

其他内部 pending：空/部分原 CM 和不一致 baseline 的后续策略、全生产组件/外部 PID 恢复、
0/±1 标量及完整元数据/GWT 编辑、其余运维/Core 完整安装升级/bootstrap/策略交接退役、NM、
一致 DB/制品/key 备份恢复与轮换迁移、正式发行。
外部 deferred 仍为真实 NM/cloud-init/物理网络硬件、amd64、磁盘满/硬断电；
两个历史证据 gap 独立保留。Java 环境已具备，不再笼统 deferred。
已闭环的取消/restart/reboot 和旧恢复验收不得重放。

本轮精确源码、制品、证据、知识库官方回读与 18 仓完整状态，以对应 JSON manifest 为准。
整体目标继续 active；下一轮优先完成上述接口/所有权/中心/早启业务，不重复本轮一次性流量入口。
