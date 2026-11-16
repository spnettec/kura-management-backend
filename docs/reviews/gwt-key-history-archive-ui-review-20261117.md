# GWT 密钥历史归档界面与 API 审查 — 2026-11-17

当前本地 GWT `25fd6b7328dc38e2df44f488b1d093dbccaf3758`，Core 保持 `be66a9f95445ef6b848076f8675422ed01500cc0`。这是密钥生命周期的有限候选，不关闭完整管理改造或正式发行。

页面从所属物理历史读取每页20条记录，保留已归档原件。所有新RPC先经过ADMIN、XSRF和HTTPS，再验证类型、原UUID、游标和确认摘要。操作人取自认证会话；客户端不选择私有路径、Root动作或新元数据UUID。NOT_DISPATCHED仍须无owning意图，独立永久围栏与普通发送方准入不互相冒充。

预览不建立归档意图。确认时重读原字节、Root证明和本地意图；摘要变化立即拒绝。第一次元数据发送先保存原编号；失去回复或缺Root原件时只查该编号，不自动重试、不换编号。Root明确允许且本地原意图匹配时，独立按钮才可显式继续同一元数据。读操作不会代行续接，密钥请求UNKNOWN不能借归档关闭或重发。

原生dialog由浏览器管理模态、Escape及返回焦点，取消为安全默认；close后清空returnValue。原生控件有可见关联label，aria-disabled保留焦点且点击/键盘处理器阻止激活，终态使用单一status，忙碌使用progress。异步令牌、预览和写入回调都核对当前页面代次、附着和可见性；隐藏页不派发迟到写入。

180项相关JVM检查无失败/错误/跳过。19组真实编译组件、HTTPS、GWT RPC浏览器检查涵盖五类历史/两种未发送回执、分页、重载、离线、摘要失效、HTTP丢回复、原意图续接、权限拒绝、延迟回调、键盘和中文390px布局。测试Root、systemd与人类会话明确为模型。没有调用原密钥writer；测试夹具不打入生产包。

生产JAR34,133,957字节，SHA256 `3204cb5f68eb129de81a07993f63f7f7756067ac32b24fea7f0b9aeb87abb78b`；60个相关类与当前编译输出完全一致、major55，login/denali各6个变体。JDK21/Maven3.10.1验证；未验证JDK27，未部署。

可从源码复现：按browser/README.md运行run_archive_browser.py；JVM选择GatewayKey*Test、GwtGatewayKey*Test、GatewayRecovery*Test和GwtGatewayRecovery*Test，显式Maven缓存并跳过该轮重复GWT编译。生产包用正常package重新生成全部GWT变体。本轮浏览器/GWT中间目录已清理，重跑无需旧报告或凭据。

保留：独立HTTPS恢复端归档按钮、实际Root UDS/安装Main/真实管理员会话联合链路、完整cohort及provider/权限/发行矩阵仍为内部pending。原3现场、7进程、38份私有元数据与6份旧数据精确不变；未重放一次性安装密钥验收、未推送。仅3项external边界和完整12父项不变。
