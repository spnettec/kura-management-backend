# 原 GWT 网络事务入口 v1

2026-11-01。适用 `kura-management-ui` 的原本机控制台，与 React 机群中心分开。
中心与 agent 的注册/mTLS 仍独立于 Kura；本入口沿用原 Kura 用户、`NETWORK_ADMIN` 权限和写操作 XSRF。

在显式 `-Dkura.networking.provider=netplan` 模式下，网络页通过公共
`NetworkTransactionService` 调用独立 agent，不调用旧网络写入器。
普通模式继续原有路径。正式 provider 的发行 profile 尚待交付。

## 编辑和无损保留

每次加载接口时，服务端从同一个 network view 捕获 revision、sourceDigest、canonical config 和显示基线。
浏览器持有随机 edit ID；服务端基线按 HTTP 会话隔离，最多 64 个、有效期 10 分钟。
提交不重新读取 revision 来套用旧表单；失效或其他会话的 edit ID 返回 CONFLICT。
revision 保留整数精度，浏览器只接收其字符串显示值。

本版可编辑 DHCP4/手动、首个 IPv4 地址及掩码、单一默认网关、IPv4 DNS 和 MTU。
未编辑字段保持 canonical 原值；编辑首地址保留其余地址，编辑网关保留原默认路由 metric 和其他路由，
编辑 DNS 保留 IPv6 DNS 及其相对顺序。多个默认网关不支持通过该单值字段修改。
searchDomains、optional、dhcpMetric 等不因旧表单缺少控件而丢失。

旧 WAN/LAN/禁用状态、优先级、DHCP renew、IPv6 MTU、promiscuous 等控件禁用；
IPv6/DHCP server/NAT/802.1x 等未支持页面隐藏。未获委托或无法表示的配置只读。
硬件页只显示观测值，不回填写请求。企业属性保留原基线，不从隐藏页面重建默认值。
伪造未支持字段、企业属性或来源元数据仍由服务端拒绝，不静默忽略。
这不是完整旧网络字段或 IPv6 写入支持。

## 事务交互

点击“应用”确认后，只发送一次 prepare，请求确认窗口为 120 秒。
页面展示事务状态和剩余秒数；不会把 prepare 成功当作最终确认。
只有创建事务并收到回执的同一 HTTP 会话有控制权。确认令牌最多保留 16 个，
仅保存在服务端会话，不进入 RPC DTO、URL、浏览器属性、快照或审计文本；终态清除。
其他会话可以观测事务，不能从查询结果取得控制权。

用户须勾选已通过受影响管理路径重新连接，才能明确确认；也可明确回退。
这项勾选是操作者声明，不是系统已自动证明连通性的声明。
忙碌或待确认时禁止再次应用。终态后重新读取确认态配置；待确认期间保留编辑内容。
状态每两秒观测；卸载页面后取消定时器。

请求失败不自动重试写入。明确的 INVALID_ARGUMENT/FORBIDDEN/UNSUPPORTED/CONFLICT 拒绝允许修正表单；
超时、断开或协议异常按结果不确定处理，只进行观测。丢失 prepare 回执时不从 view 恢复令牌，
只能观测并等待 agent 回退。旧的终态观测不能被当作新 prepare 的结果。
原状态页的内部只读调用使用临时基线；事务查询与控制仍要求实际 HTTP 会话。

## 中英文和验证边界

新增提示、按钮、状态及倒计时提供英文和简体中文。原 GWT 继续使用既有 locale cookie 和编译版本；
React 的即时语言切换、选择持久化及错误本地化见 `console-i18n-20261101.md`。

服务端、GWT 完整构建、实际 IDEA 登录及浏览器验证的证据见
`phase2-network-gwt-validation-20261101.md`。UI 测试使用明确的 test-only OSGi network owner，
不会执行 Netplan、UDS 或主机网络命令；模拟 UI 事务不作为 Linux 实际 Kura/agent 写入验收。
