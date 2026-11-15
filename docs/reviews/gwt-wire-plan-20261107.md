# 原 GWT Wires 条件生命周期计划实施与实际验证

2026-11-07，6ce49a2e，续接 01a4bbe2。整体目标 **active，未完成**。证据 `../evidence/gwt-wire-plan-20261107.json`；合同 `../configuration-plan-local-v1.md`；新日志根 `target/gwt-wire-plan-20261107/`。18 仓完整工作树/HEAD/branch、578 源码/1276 制品/2853 旧证据入场核对通过。旧报告/待更新源码先存 `before/`，保留原现场、不提交推送。

## 实施结果

- WireGraph API 增加成对编辑视图和条件保存，默认实现拒绝 unsupported，未新增强制激活依赖。provider 从 Core 已持久化图配置推导成员，批量读取值与 revision 并复核图版本，不把新 token 配到旧 DS 拓扑。
- 图 DTO 保留图 PID、全部旧成员、被移除成员和附加 driver 的完整 baseline。GWT 只走 `updateIfUnchanged`，以实际服务 PID 集校验 scope；新增工厂参数使用类型转换，既不继承另一工厂的旧密码，也不把 UI datamap 当组件属性。
- provider 在副作用前完成身份、版本、拓扑、端口范围、重复节点/边与创建/删除/替换计划。现有图外资产仅允许实际 WireAsset 采用，不能借新资产删除无关工厂。Core 一次 typed plan 接收图、业务组件与 driver 变化；WireAdmin 清理在之后执行，tracker 保持开启。
- 准入前拒绝不写；准入后错误为 UNKNOWN，provider 阻止获取新编辑 baseline，不重发写。该额外不确定标记在内存中，尚非持久结果核定。成功保存后仅有界重读 pending 视图，再清 dirty；同页第二次保存以新 baseline 进行。

## 测试与红绿证据

Wire provider **21** 项（原 6 + 新 15），GWT **132** 项（原 120 + 新合同 11 + 生产 RPC policy 1），完整 login/denali 六排列编译及安装通过；Core **130** 沿用上一轮已固定结果，未冒称重跑。API 实际安装与原 Wire 六项兼容验证通过，diff 检查通过。图谱对真实 core/wires/ui 子仓重新索引，精确路径 freshness 匹配；仍为 best effort，不作为全覆盖保证。

有效产品红证据：`gwt-factory-scope-red-valid`（换工厂携带旧密码）、`gwt-new-instance-properties-red`（新配置混入 UI 属性）、`wire-asset-ownership-red`（资产路径可删除无关现有工厂）。修复后相应回归通过。首次合同编译错误、immutable fixture 误用及 `gwt-factory-scope-red` 的夹具失败单独保留，不能当产品红证据。

## 原 IDEA / 原 GWT 实际证明

新 v2 完整隔离运行集：283 inventory 文件、282 实际 OSGi bundles；IDEA Run `Kura Wire Conditional v2 20261107` 启动 PID 73869。实际原 admin/admin 登录，未安装 agent；专用业务工厂提供真实配置服务、DS 生命周期和 Wire emitter/receiver。

1. 中文旧页面在专用外部 Core writer 更新 primary 后保存 secondary，被 CONFLICT 拒绝；前后真实 CM/DS/图/WireAdmin 状态相同。显式 Reset 读取新视图，之后连续同页保存 secondary=4、5，磁盘快照与 DS 值一致。
2. 新唯一生命周期命令逐一执行：旧 baseline 删除拒绝、节点替换、同 PID 工厂替换、删除最终图。替换后真实 WireAdmin 连接有效并投递 `proof-replace-node` / `proof-replace-factory`；空图磁盘配置与 WireAdmin 空集合核对。整个 Core 并未变空，不能据此声称实际空 Core 快照验证。
3. 原中文页面从空图新建 `wire.primary`，name/description/Integer Count 正确，实际 CM/DS/快照无 componentId、configurationRevision 等 UI 元数据泄漏。
4. 通过原页面退出/语言选择/重新登录切英文，旧 baseline 保存出现英文 CONFLICT，实际状态与外部更新后完全相同。
5. **受控故障注入**：仅对该拥有的 JVM 装载单独 attach 测试 agent，验证固定 fixture.root；将真实 WireGraph provider 的 WireAdmin.getWires 一次性抛错。Core 属性/图已各提交一次后，原英文页面显示 UNKNOWN；后续只读观察完全相同，未重发；独立新页面读取被 CONFLICT 屏障阻止。该证明涉及属性/图更新，未实测在非幂等工厂创建/删除或 snapshot 中途失败。agent JAR 单独固定，未改 283 项基础运行集，也未放宽产品认证。

`v2/native-system/result.json` 和 `native-unknown-write-proof.json` 固定上述范围。raw 回执的 dsPresent/dsEvents 仅统计 fixture Business 组件，不能解释为真实图服务 DS 回调计数。外部 writer 是专用 bundle 调真实 Core API，本轮没有中心→agent 传输验收。成功运行 PID 已 SIGTERM 并确认消失，UNKNOWN 现场与文件保留，不重新启动旧入口。

## Linux 实际证明与夹具修正

相同产品制品在 OrbStack 与现有一台 Parallels Ubuntu 上分别启动新隔离单元；每处独立只执行 stale-delete / replace-node / replace-factory / delete-graph 四个唯一命令，核对 CM/业务 DS、Integer/工厂磁盘值、WireAdmin 真实 String 消息及空图。各 283 inventory hash 匹配；仅测试 probe 在 v3 增加有界等待非 null OSGi 服务，产品 JAR 与 native v2 完全相同。无浏览器、无 agent、无中心传输，也无网络控制写。

Orb 实测 ProtectSystem=no、NoNewPrivileges=no，现有全局覆盖仍在，不以请求属性充作严格沙箱证据。Parallels 实测 ProtectSystem=strict / CapEff=0 / NoNewPrivs=1，root nsenter 外部写测试 EROFS=30；私有 JRE 184 文件固定。两处 nft 内容 hash 与 IPv4/IPv6 forwarding 均与入场一致。新单元全部停止；Ubuntu 正常停止，Windows 始终停止。

失败现场：首 native 未启动编译缺 import；native v1 PID73693 因 probe Import-Package 缺 org.eclipse.kura 未进 bootstrap；Orb v2 PID750065 因获取到 null 服务在 bootstrap 前退出。均没有业务计划准入，日志/运行集保留。该 transient null 的更深原因未证明；v3 仅等待服务非 null，不重发动作。另一次在宿主误运行 Linux 观察脚本只因共享路径不存在退出，未触及 VM/命令；后在正确 VM 内首次执行。失败入口一律不重启。

## 内部 pending

- 全部 Wire/native factory/metatype/实际端口约束与组件面；asset/channel/CSV/import/clone/dirty-form 专属路径；全局其他新工厂转换仍需审查。
- 实际非幂等 factory 部分派发、snapshot/外部 provider 失败，丢回执业务结果的持久核定及进程重开后的 UNKNOWN 行为；实际整个 Core 空集合快照/恢复。
- 正式最低 API 版本、包版本/安装升级兼容；旧 API 链接不由新默认方法保证。本轮未产出或安装新正式 deb。
- 生产/外部 PID Java 恢复、空/部分/不一致 CM；NM 自动 NAT/其他 writer 归属/采用/退役；加密中心防火墙与中英远程编辑及受影响管理路径证明；confirmed-only 实际早启与 Core 正式交接；一致 DB/制品/密钥备份恢复、轮换、迁移与发行。

`GwtServerUtil.providesService` 其他调用仍有旧 PID 拼 LDAP 的实现，本轮 Wire 授权避免使用它，未声明全局审计闭环。完整 firewall、全业务恢复和完整组件覆盖仍 false。

## 外部 deferred 与证据限制

真实 NM/cloud-init/物理链路与硬件、amd64、disk-full/hard-power 按原台账外部分列。历史初始 CONFLICT 的确切锁持有者仍不可追溯；本轮不补造历史因果。旧三份被覆盖 Surefire XML 原文缺失及两项历史 gap 保留原声明。2853 旧 evidence 字节保持；旧源码/制品有变化者必须能在新 before 副本中核对旧 hash。所有旧请求、快照、密钥、卷、状态和未提交修改保留，不 reset/clean、不接 192.168.2.125。
