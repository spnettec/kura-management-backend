# 组合网关运行、首改密码和身份目录（2026-11-17）

用户已授权使用现有积分继续，目标 active。当前只关闭下述局部安装/读取与首改缺陷；完整六组件、provider/launcher/模式/升级移除及全部 12 父项继续内部 pending。

## 同一 agent-first 新现场

保留原新夹具 `ebdbf5645723…`，不重建 Core/agent。安装当前 networking `5d27fea` 与 GWT `04a2de4` 前，只读确认 6 份原 Core 私有材料精确、执行库各表 0、Root operations/startups 0、agent 无法读取 Core 原钥/HMAC。为新增组件明确停下这台新现场的 Main；agent 始终为 PID5450/ticks16411356。接入现有 Docker bridge 以验证新组件组合，未新增 VM，未动原三现场。

实际 APT 安装两个包成功，Core/agent/networking/GWT 均 configured。11 个 sibling JAR 为 Root 普通非组/其他可写文件；Root libexec 安装器及注册表没有放回 `.data`。首次 networking 安装以 kurad 追加两项预期界面标志，原 799 字节前缀摘要精确，新增 55 字节准确；未产生全局 `99-kura-nm.conf`。resolver 内容、inode、属主/模式保持，但此轮接入测试网络和安装后的 mtime/ctime 改变；不把它记成全部系统元数据精确，也不单独归因于 networking 钩子。

普通 unit 启动成功。实际 OSGi：NM224、GWT227、bridge216 ACTIVE；network status health 从旧 UNSATISFIED REFERENCE 变成 SATISFIED（惰性服务），不是业务健康检查已执行通过。后续两次 GWT 源码修复均在同一新现场明确维护停启 Main；最终 PID20499/ticks17029346、NRestarts0、UID997。Main 仍 11 ambient/13 bounding caps、NNP0；完整 broker 未关闭。Root key/recovery/NM 与 agent 进程保留，9 份私有文件完整元数据精确；Root operations/startups 和 agent 执行表仍为 0，Core 原钥/HMAC 在 agent 命名空间仍不可见。

## 首次改密与事件门禁

实际 admin/admin 登录后，旧页面仍显示旧密码栏。`0d6dba8` 将刚刚认证成功的凭据保留在本次对话内存中，强制改密只显示新密码与确认；不把原凭据填回隐藏输入框。服务端仍检查原凭据、强度、XSRF 与待改密会话锁；登录后的普通改密仍要求旧密码。提交/取消清除表单与临时引用。

同时发现事件路由漏过待改密锁，并且实际 `/admin/sse` 未纳入“不延长空闲期”的事件集合。补齐事件锁及实际路径，保留正常事件不刷新空闲期。新增 4 项完整路由策略测试；原实现中“锁住的事件”和“实际 SSE 不续会话”两项先失败，修复后 4/4。包括 RPC/身份兼容的首批 14 项通过；后续目录修复的最终选择共 20 项通过，包含这 14 项，不相加成 34。

采用 modern-web-guidance 的 forms/autofill-sign-up-form：新/确认字段使用 `autocomplete="new-password"`；按字段完成编辑/提交校验，以原生 input 事件清除旧错误，避免逐键报错；错误关联 aria-invalid 并把焦点移至失败字段。GWT2.12 没有该事件的内置 adapter，使用小型 DomEvent adapter，不引入依赖。新旧两种表单保持显式标签，强制模式旧栏不可见且 disabled；显示中的表单去除错误的 aria-hidden。实际中英文编译各模块 6 种，共 12 变体，Java11 目标/JDK21 构建通过。

浏览器实际两字段、空值/不匹配零写入、错误焦点及编辑后清除状态通过。一次有意的管理 API 拒绝探测按 `HttpServletContextHelper` 使会话失效，随后 XSRF 明确 403，此时密码写入为 0；后续探测已无会话，不能当成每条路由独立的带锁验收。重新认证后唯一一次密码写入返回 200，成功回调进入实际控制台；抓响应体遇到立即导航竞争，未保留原响应体。原服务端审计明确记录密码更新/解除强制更改和新会话，随后使用现有新密码再次登录成功。没有再次发送密码更新，也不以单独 PID/文件出现判断结果。

## 安装版身份目录修复与保留的 CSR 意图

当前原启动器把 `kura.data=/opt/eclipse/kura/data` 放在 framework/kura.properties，由 Core SystemService 管理；并非 JVM 同名 -D。旧 GWT 直接读 JVM 属性，导致身份历史加载的 INTERNAL_ERROR 弹窗。`46a2d92` 改用正式 `SystemService.getKuraDataDirectory()`；若旧 JVM override 与服务值冲突则拒绝，未迁移、替换或猜测旧目录。缺失/相对服务路径也不回退到工作目录。新增实际 Java 生成 CSR、同目录保留、冲突保留、缺失/相对路径拒绝 4 项，并保证读取空续期历史不造目录；最终 20 项相关检查全通过。测试准备纠正了方法可见性、缺失 Core 数据父目录和错误的“只读列表会造目录”假设，没有放宽生产私有目录检查。

当前包 `3.0.0~git202611170539.46a2d92-1_all`，包 SHA256 `7d67ddaa5b9855c2ce165398198c794b6df7771d17e9c7a99618ff12f0da311d`，GWT JAR SHA256 `d32e344ed62533774a2f2ec358794b665fdaf6ad2fbbfb9476f806c59981c0cd`。实际包内 9 个 Root 载荷与本轮源码构建 JAR 精确匹配，安装/普通启动返回 0。修复后实际 GWT：安装检测、身份列表、绑定及远程状态均成功；无 INTERNAL_ERROR 弹窗。常驻未绑定、开关和首次接收未授权的状态正确显示，写按钮保持禁用；续期状态的 RPC 仍为不可用，不能算续期通过。

创建新网关 CSR 的自动化意图 `combination-c-ebdbf564` 已保留。自动化在等待响应时发生未处理超时，Node 会话退出，原专用浏览器/转发随之结束。只读重建浏览器后，在未修复版本中观察到上述异步错误弹窗遮挡生成按钮；这是原点击可能未触发的解释，不是原提交已结算的凭据。原 `prepareIdentity` RPC 回执和客户端 UUID 均未恢复；原现场没有申请目录/受理审计，修复后只读列表仍为空。不能用这些缺失推断正式结算。没有再次生成 CSR、换 ID、签发、绑定或开启通道；该网关标识保留，原意图继续内部待核定。GWT 首次申请在发出前持久保存原编号/材料并在会话丢失后恢复，也需要补齐。不要用另一网关标识/Root 重做来掩盖本次中断。

恢复浏览器仅使用已更改密码登录、读取当前原现场；独立固定 TCP 转发仅连这台新网关 localhost:443，TLS 指纹核对后才接受测试证书。转发不替换 API 返回，不更改全局代理/DNS/信任，不使用个人 Chrome profile。完整人工信任/平台体验未验。

## 台账、文档与保留

`provisioning-ui-design-v1.md` 已按方案 A 重写为当前合同，去除旧稿的中心默认 CA 私钥托管、P1/P2/P3 整体交付和工程 deferred 表述。agent `01ff367` 的后备导入示例补上审查未决工作后的明确停机，并覆盖旧 CSR 的 inactive 要求。CLI 示例正确不等于 R017 本机授权/维护/独立 agent GUI 已完成；不改成默认停 agent，不放宽 guard。

原三现场的 7 个进程、38 私有完整元数据、6 旧数据和个人 25 文件再次精确。新原生中心的 29 私有文件及 PID3135/ticks16649159/cap0/NNP1/NRestarts0 精确；其数据库/新卷、原完成的 GUI 回执及材料继续保存。18 仓保持共同 WIP 分支，只本地提交，无推送；本阶段文档写入前仅 console 当前设计文档待提交。原 NM 属性 inode/ctime 偏差仍按上一阶段保留。

六组件/provider/launcher/模式/安装升级移除、完整 broker/Main caps、provider 重加密、R017 本机授权/维护与独立 agent GUI、CA custody/quorum、客户库/协议/上游及全部冻结 12 父项继续内部 pending，G034 未封签。external deferred 仍仅实际 NM/cloud-init/物理硬件、amd64、磁盘满/硬断电。旧 UNKNOWN 不重发、不换 ID、不在其他 Root 查询；已闭环备份、归档、轮换、恢复及 900 秒验收未重放。

具体源码、包、进程与脱敏观察索引见 [检查点](../checkpoints/20261116/combined-gateway-runtime-checkpoint.json)。
