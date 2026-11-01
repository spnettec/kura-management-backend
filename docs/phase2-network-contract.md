# 阶段 2：受限有线接口事务原型

2026-11-01。本文件记录当前实际实现；总方案仍为 `management-redesign-plan.md`。
本轮尚未替换 Kura 的旧网络 provider，不能据此宣称阶段 2 全部完成。

2026-11-01 后续状态：已新增旧 PID 的 Netplan 确认值/状态适配和 Kura 最小网络委托，
见 `phase2-netplan-adapter-validation-20261101.md`。下文“实际 provider/权限未实现”是本文件
早期增量状态；原字段写入、UI/REST 事务确认、正式 profile 和 Linux 实际 Kura 仍待交付。

## 已实施的边界

中心添加 `network.inspect/prepare/confirm/rollback/transaction` 五种操作，Protobuf
使用 `NetworkChange/WiredConfig/StaticRoute/NetworkAction` 类型，不接受 shell、路径或任意参数。
HTTP/OpenAPI 使用同一字段；嵌套未知字段拒绝。prepare/confirm/rollback 目前限租户 admin，
operator 可观测，viewer 沿用不能创建任务的既有规则。网关仍按租户和登记证书绑定。
幂等比较包含输入内容；agent 结果缓存也比较完整 Protobuf 请求指纹。

本次增补 `network.takeover.plan/prepare` 两种操作及 `NetworkTakeover` 类型，PG migration
0004 扩展允许操作。plan 只接收接口名；prepare 携带 revision、来源摘要和已审阅计划摘要。
plan 可由 operator/admin 读取，prepare 只授 admin。HTTP 旧回执中省略的可选输入与新存储的
null 视为同一空值，其他内容差异仍拒绝同幂等键重用。

任务回执和网络事务分别处理。`SUCCEEDED` 只说明请求处理完毕，结果中的 `WAIT_CONFIRM`
不能解释成网络已提交。React 提供来源、接口、字段、倒计时、刷新事务、确认和回滚；
任务页面将网络请求标为“请求已处理”。

agent 默认观测，`AGENT_NETWORK_INTERFACES` 为空不写系统网络。显式接口名单只是允许写入的
必要条件，不是接管已有来源的命令。默认必须是存在的物理有线 Ethernet 接口；VM fixture
通过独立的 `AGENT_NETWORK_ALLOW_VIRTUAL_FIXTURE=1` 开关使用 dummy，生产不启用此开关。
普通 prepare 对目标接口存在其他 YAML、match/set-name 或复杂拓扑时，保守返回 CONFLICT；
单一既有 networkd YAML 可通过下述独立审阅流程转交写入权。
没有删除旧 YAML、停用 NM、改网卡名或直接写 resolver。

读取 `/lib/netplan`、`/etc/netplan`、`/run/netplan` 的来源并计算摘要；文件须为安全所有者、
普通文件且非组/其他用户可写，拒绝符号链接。最多 64 个 YAML，来源总计 128 KiB；受管生成内容
上限 64 KiB。候选先在私有 staging 根目录保留全部来源，调用实际 Netplan generate 校验。
其他接口文件不修改。既有自有配置中的 IPv6 属性、IPv6 地址/路由及其他接口保留。

字段覆盖 IPv4 DHCP/静态地址、基本路由/metric、DNS/搜索域、MTU 和 optional。
`optional` 表示不阻塞 wait-online，不能当成接口启停或自动连接；后两者的旧语义映射尚未交付。
地址/路由和 MTU 有运行值回读；DHCP、resolver、IPv6 全矩阵另需验收，不能由 YAML 生成代替。

首次配置某接口时，若它尚未出现在已确认的自有配置中，已有运行 IPv4、IPv4 路由或非
link/host scope 的 IPv6 地址均会在写候选前被拒绝，等待明确接管流程。首次空接口的运行
快照记录原 MTU 与行政 UP 状态。Netplan 删除其候选后可能保留 MTU，因此 agent 仅对该
接口用固定 `ip link set` 参数恢复这两项，并检查地址已清除；不接受远端程序/路径或任意命令。
若当前 MTU 既不等于候选也不等于原值，则保留外部修改并报告 RECOVERY_REQUIRED。
旧日志缺少新增字段时仍可读取，不伪造原 UP 状态。这一补偿不等于已有手工配置接管或完整 DHCP 恢复。

## 明确转交已有来源

当前仅支持目标接口恰好由一个 `/etc/netplan/*.yaml` 定义、networkd 已运行的来源。
plan 返回原来源/内容摘要、权限/组、服务状态、原接口字段、保留的其他接口、归一后摘要和
绑定完整计划的 planDigest。React 展示来源/模式、原接口字段和其他接口，必须勾选已核对来源才能发出 prepare。
转交时地址配置保持原值，后续变更仍走普通 network.prepare。

拒绝 NM/cloud-init 生成来源或竞争服务、同名 `/lib`/`/run` 来源覆盖、match/set-name、复杂拓扑
及无明确映射的字段；不自动停服务、不执行 NM/cloud-init 退出。服务快照检查 networkd、NM、
cloud-init-local/network 和 cloud-init.service；普通变更与接管共用后端归属检查。
非空 `/etc/systemd/network`、`/etc` 或 `/run` 的 NM connections 均拒绝。

`/run/systemd/network` 必须与当前安全 Netplan 来源在私有 staging 中实际生成的文件内容一致，
不能只凭 `10-netplan-*` 文件名判定归属；未知文件、内容漂移或 runtime drop-in 均拒绝。
只读检查发行版 `/usr/lib`（或 `/lib`）和 `/usr/local/lib` networkd 文件的类型、权限、内容摘要，
保守拒绝排序在 `11` 之前的 vendor 文件；检查 `networkctl` 报告的活动 NetworkFile 与 drop-in。
服务与 vendor 快照进入计划摘要及持久 journal，确认/回滚重查；漂移不覆盖外部内容，需明确恢复。
这仍是受限的写入者归属模型，未阻止任意 root 程序并发修改，也未实现一般化 NM/cloud-init 退出。

staging 使用 Ubuntu 打包的 `/usr/libexec/netplan/` 原生生成器和 `--root-dir`。
现有安装提供 `configure` 时优先使用（已在 26.04 fixture 核对）；否则使用 `generate`
（已在 24.04 fixture 核对）。
实际 fixture 发现 Python `netplan generate` 包装器即使指定 root-dir，仍会触发全局 udev/systemd
reload，并让宿主 generator 覆盖真实 backend、掩盖漂移，因此不回退到该包装器。
真正应用仍调用固定 `/usr/sbin/netplan apply`；其他发行版/版本的原生路径须另验。

待确认时仅向 `/run/netplan/<原文件名>` 写入移除目标接口的归一覆盖，再写入自有 runtime
候选；原 `/etc` 文件字节和权限不变。journal 保存原字节、归一内容、权限/组、服务快照和候选。
未确认重启会失去这些运行覆盖，加载原来源并回滚。确认要求重新建立 mTLS 的 epoch，持久
确认意图后先保存自有 `/etc/netplan/90-yofc-managed.yaml`，再归一原来源，保留其他接口与原权限/组。
两份持久定义短暂重叠时接口值完全相同；真实 staging generate 对此窗口也执行校验。
已接受确认后的重启补完提交。来源/权限/服务漂移不覆盖，进入显式恢复处理。

静态 IPv6 地址、策略字段和路由原样复制，并在普通 IPv4 编辑中继续保留；不提供 IPv6 编辑。
实际 Ubuntu fixture 已覆盖带全局 IPv6 的转交、回滚及重启，未覆盖 DHCP/resolver 全部行为。
详细证据及限制见 `phase2-takeover-validation-20261101.md`。

## 持久事务与启动

固定自有文件名 `90-yofc-managed.yaml`。未确认候选仅在 `/run/netplan/`，持久确认值在
`/etc/netplan/`。候选不写入持久启动来源，从存储策略避免把未确认文件留给重启后的 generator。
真实 VM 已证明本布局在本机 Netplan 版本的覆盖行为；这不构成物理断电/存储控制器保证。

私有状态目录的原子 JSON 日志保存原确认值、全部来源字节/摘要、请求指纹、阶段、确认意图、
期限和连接 epoch。文件写入、rename 和父目录按顺序 fsync；无跨文件系统原子提交假设。
系统写入由进程内变更锁与跨进程 flock 单一所有者协调；只读观测/事务查询读取原子快照，
不等待 Netplan 写锁。UDS Kura 登记角色不能调用这些管理方法；root/配置的本机 admin 可调用。
双向 UDS 的持续收帧与有限队列发帧分离，本机最多四个并发请求；处理慢请求或发送反向调用时
保留同一读帧 future，避免取消半帧；超时/断线不表示已接受系统事务取消。

`PREPARED -> APPLYING -> WAIT_CONFIRM -> COMMITTING -> COMMITTED`；失败、超时和未确认启动
进入 `ROLLING_BACK -> ROLLED_BACK/RECOVERY_REQUIRED`。COMMITTING 是确认决定已持久化的内部阶段，
重启补完提交而不是撤回已接受的确认；外部文件漂移时不覆盖并报告 RECOVERY_REQUIRED。
WAIT_CONFIRM 的后端内容必须严格匹配当前来源；恢复中仅额外允许 journal 能归属的旧/候选
生成字节，处理运行候选已消失但 networkd 生成文件尚存的启动窗口。旧 journal 没有写入者快照时
仍做当前归属检查，不补造历史服务/vendor 状态。新增验收见 `phase2-writers-validation-20261101.md`。
终态幂等回执保留，最多 8192 个，达到上限拒绝新事务，不自动删除以免旧命令重放产生新副作用。
历史维护和按事务 ID 查询旧回执尚需完善，目前查询入口面向当前事务。

确认窗口 30–300 秒，默认 UI 为 120 秒，中心 30 秒命令 TTL 与之分离。apply 返回后 agent
主动重连；远程确认须来自与 prepare 不同的新认证 epoch，并携带对应随机确认 token。
获权本机工具可明确本地确认。旧流在线、心跳或 ping 不视为确认。
确认前核对候选/来源、实际地址/MTU/路由；之后先持久确认意图，再替换持久文件和确认状态，
移除运行覆盖并应用。超时恢复每秒检查，不依赖中心或 Kura；应用子进程有限时并终止其进程组。
回滚不仅检查文件，还回读地址、MTU 及候选新增路由；不匹配返回 RECOVERY_REQUIRED。

## 当前不可作为发布完成证据的部分

- NM/cloud-init 退出、多来源/任意系统写入者协调和一般化接管；单一明确 networkd 来源已可转交，
  受限 Netplan 生成 backend 归属检查已补齐。
- 首次空接口的静态地址/MTU/行政启停及重启恢复已有 fixture 证据；已有手工运行值的明确接管
  与完整回滚仍待实施。DHCP 租约变化、DNS/resolver、IPv6 和
  地址/路由冲突的全部运行组合，物理链路与硬断电。
- 实际 Kura provider、NetworkStatusService、旧 PID/字段和 REST、镜像与来源防循环。
  核心三入口归属协调与 Java 双向调用基础已实施，见 `phase2-provider-coordination-20261101.md`；
  synthetic PID 的通过不等同于网络集成通过。当前 Java bridge 入站仍仅列配置和快照，
  Kura 登记角色尚未获得网络方法权限。
- 自主回滚事件上传、完整阶段审计、历史维护、容量/慢消费者和真正系统级协调器的其他子系统。
- 正式网络 systemd profile、非容器 Ubuntu 沙箱、deb 生命周期。新增 packaging 示例尚未按发布标准验收。

继续实施以上范围后再推进 nftables、系统控制、离线恢复与发布；不要恢复旧 Java/NM 双写。
