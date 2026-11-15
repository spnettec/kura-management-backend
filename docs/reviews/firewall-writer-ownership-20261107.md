# 2e050db1 — Kura 旧防火墙写入隔离

2026-11-07，接续 c5564ec4。完整目标 active，本轮 progress。
已交付 agent 归属下旧 Java、NM NAT writer 和安装/恢复脚本的拒写基础，
不将其计为完整防火墙接口适配、所有权迁移、早启或正式发行。
合同见 [firewall-writer-ownership-v1.md](../firewall-writer-ownership-v1.md)，
精确状态与 SHA 见 [机读证据](../evidence/firewall-writer-ownership-20261107.json)。

## 改动

新增本机 `FirewallOwnership`：默认 legacy；稳定系统标记和私有 home 标记均可抑制旧写入，
悬空链接不回退，Java 无法读取归属时拒绝。agent 离线也不回退；该判断不调用 agent，
不建立 Kura 启动依赖。provider 属性只抑制 Java，四个脚本依赖标记；未实现正式 adoption。

低层 iptables、IPv4/IPv6 高层规则和 forwarding、旧配置服务和威胁附加入口均在副作用前拒绝。
NM 无 NAT 的普通配置保持无操作；真实 NAT 请求明确拒绝，未映射为 agent 事务。
两种 firewall 启动入口、网络安装/卸载中的旧服务操作、备份、清链、重放及 forwarding 恢复
均新增归属检查，无标记时原逻辑顺序保留。iptables 导出包 1.3.0 与两个消费者的最低导入
同步，防止新消费者错误加载旧库。

未修改 Rust agent、Core API、原 GWT、React 或协议源码；未创建宿主系统归属标记，
未迁移网络或安装新 deb。admin/admin、生产首登后台闸门、双语、默认关闭及 UNKNOWN /
RECOVERY_REQUIRED 合同、NM 系统后端和无 agent 独立 Kura 合同继续保留。

## 验证

前轮 manifest SHA `324c1689aeb44b6a3cf2cb4b66045464a1f94d14113b7a1d526efc13cf7ea33a`，
312 源码、11 制品、124 证据共 447 pins 及 review 实时匹配，18 仓 HEAD/branch/完整 dirty
与前轮完全一致。没有 reset/clean/提交/推送/远端覆盖；图谱仅刷新 networking 子仓。

最终 Java **72 测试通过**，涵盖既有 iptables/防火墙/NM 回归和新增归属合同；三份产品 bundle
构建通过。四个脚本 bash 语法检查通过，宿主、Orb、Parallels 各 **2 个脚本合同方法及全部
子场景通过**。脚本测试使用重定位路径/原函数体，未执行实际安装卸载或旧一次性入口。
首轮新测试对 getConfiguration 异常层级的预期错误已纠正，最终沿真实 KuraException 合同验证。

最终 IDEA 配置 `Kura Firewall ownership v3 20261107` 从桌面 UI 一次启动：
`.phase1-dev/firewall-owner3-idea-20261107-75da0e71`，PID **8080**，已精确停止。
287 inventory、实际 286 bundles；3 个产品 bundle 在 start level 100 解析，不激活 NM DS。
原生实际 PrivilegedExecutorService 可用，**36 项旧入口被拒绝、0 次执行器调用**；
原 GWT admin/admin 中文登录成功，无正式 agent 安装时远程菜单隐藏。
首登 bypass 仅属隔离验证夹具，生产闸门未改。

随后 OrbStack → 现有 Parallels，各启动新的独立生产 agent、完整 Kura 和测试 TCP 代理。
私有网络命名空间只有 loopback，无宿主路由/veth 改动。agent 在默认关闭之外，仅对本夹具
显式授权本地防火墙写入，确认 revision 1 后添加独立外部哨兵表；Java 36 项拒写和原 GWT
登录之后，整个 namespace nft JSON 完全一致。两处宿主 nft/IPv4/IPv6 forwarding SHA
各自前后相同。旧 agent ELF SHA `5d51aa2f0017a25923c956fda9c48d6ff6d0dae138cb1a0eebc7dc5ae22e8136`
未经修改，没有重新执行旧 NAT 一次性流量验收。

| 环境 | 本轮实际沙箱与状态 |
|---|---|
| Orb | agent/Kura CapEff/CapBnd=1000；agent 自身 NNP=1，Kura NNP=0；只读挂载未生效 |
| Parallels | agent/Kura CapEff/CapBnd=1000、NNP=1；实际既有系统文件 O_WRONLY 打开均 EROFS 30，无截断/写入 |

Orb 原有 `/run/systemd/system/service.d/zzz-lxc-service.conf` 显式关闭 systemd NNP/ProtectSystem；
已采集生效值，不改变全局环境强行通过，也不以 Parallels 结果替代 Orb。
首次 Orb 夹具已经完成 Java/内核检查，随后因测试错误要求 Kura NNP=1 而停止；
纠正为记录 Orb 已知限制、严格要求 Parallels 后，另建新夹具验证，原夹具不重放。

## 保留现场与最终状态

- IDEA v1 `1ecc0492` / PID7734、v2 `60a7a039` / PID7838：测试装载方式触发 Equinox
  resolve 自动启动和 Mac 不存在的 NM D-Bus，均停止保留。定位后用独立最小非 Kura 框架
  验证 BundleStartLevel，再在新 v3 全运行集修正观察器装载方式；未改生产代码规避错误。
- 最终 native profile 静态 **293 pins**；源 profile 两份未引用额外 JAR 原位保留，新 profile
  只复制已核定 inventory，不删除旧文件。
- Orb 首次 `/opt/yofc-fw-owner-orb-20261107-6db5698c`，最终
  `/opt/yofc-fw-owner-orb-20261107-3ed2f222`；两夹具共 4 个新 unit inactive/MainPID=0，
  TCP 代理停止，namespace/rules/数据库/key/日志完整保留，分别 **641/645 文件 pins**。
- Parallels `/opt/yofc-fw-owner-parallels-20261107-16803407`：agent2486 / Kura2544 及代理全停，
  **830 文件 pins** 含私有 JRE184；Ubuntu 正常 stopped 后临时内核 namespace 自然结束，
  磁盘现场保留。Windows 始终 stopped。
- 原 Orb 四服务仍 active，PID **244516/244911/286620/268**，原 console26/agent20 包不变。
  没有新中心、PG、Vite 或防休眠服务；全部旧夹具、state、密钥、备份、quarantine、卷保留。

Equinox 自动启动行为核对了 [官方容器源码](https://raw.githubusercontent.com/eclipse-equinox/equinox/master/bundles/org.eclipse.osgi/container/src/org/eclipse/osgi/container/ModuleContainer.java)
及当前实际 framework 字节码。上述失败均是验证装载/环境断言纠正，不冒称失败夹具已通过。

## 仍待交付

防火墙内部 pending：Kura Firewall API/PID/页面全语义映射（包括原端口范围、源端口、MAC、
排除接口、ICMP/全协议等）、NM NAT 到 agent 的真实适配、威胁/其他管理者正式 ownership
adoption/retirement 与安装 profile、中心加密任务/UNKNOWN 原命令只读核定、双语远程流程、
受影响管理路径的新连接证明、只加载 confirmed 的实际早启及正式部署。
当前只交付旧写入隔离，不能将 NM/旧 provider 整体列为闭环。

其他内部 pending 沿用最终台账：空/部分原 CM 与不一致 baseline 的后续策略、生产组件/
外部 PID 恢复、0/±1/完整元数据/GWT 编辑、其他运维及 Core 安装升级/bootstrap/启动策略
交接退役、一致 DB/制品/key 备份恢复与轮换迁移、正式发行。真实 NM/cloud-init/物理硬件、
amd64、磁盘满/硬断电仍外部 deferred；两历史 gap 分列。Java 环境已具备。
已闭环的恢复、取消、restart/reboot、NAT 流量及本轮一次性入口不得重放。
