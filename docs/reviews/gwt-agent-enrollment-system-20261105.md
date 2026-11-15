# GWT 本机身份交付系统验证与暂停（2026-11-05）

本轮完成后按用户要求暂停，等待明确继续指令。整体管理体系目标未完成。桌面已解锁并完成实际 IDEA 验证；历史 blocked 和“新运行集仅准备”已被本记录覆盖。

## 本轮修复

1. 实际原 GWT 首次空申请列表出现 500：JDK ImmutableCollections$ListN 不在生产序列化策略内。仅在 RPC 边界返回 ArrayList，保留领域只读集合；新增实际生产 .gwt.rpc 策略下空/非空返回编码回归。
2. 实测 OpenSSL verify 会使用环境 SSL_CERT_DIR 补充显式发行 CA，使错误显式 CA 被接受。统一 verify 参数加入 -no-CAfile、-no-CApath、-no-CAstore，仅使用显式信任输入。独立测试目录复现并回归，未修改系统信任库。

## 验证及边界

- Web2 81；宿主/Orb Rust 各113，nft各1 ignored；严格Clippy/fmt通过。已有bridge44证据复用。完整GWT login/denali六排列为前一增量编译，本轮服务器改动重新打包测试。
- 原生 IDEA 启动完整282项隔离Kura，admin/admin原登录、中英兼容、无agent菜单隐藏及快照页通过；所有新进程已停止。
- Orb真实原GWT：空列表修复、Kura Java独立生成私钥/CSR、公开证书暂存、草稿阻止提交、agent离线时UNKNOWN不自动重发、显式回执核定后新确认、首次持久交付、真实Web操作者admin审计、重启保持OFF、中英回执、agent离线仍可新建CSR通过。私钥未进入快照/浏览器，源材料未改。
- 首次浏览器交付使用112版本原生agent（3e49f72c…）；CA修复后的113版本（64fe8022…）加载同一身份、保持revision/OFF并再次实际中文只读核验，未重放首次绑定。
- Orb请求的namespace隔离属性未生效，不计为严格沙箱；实际命名unit的内核CapEff=0/NoNewPrivs=1。最初直接root启动也不计沙箱。瞬态unit退出被GC后的重建仅为测试控制器修复。
- 现有Parallels最终113版本真实严格沙箱：ProtectSystem=strict/ProtectHome/PrivateTmp/NoNewPrivileges及零能力实际生效；mount namespace内源目录只读、仅state可写。错误显式CA在正确ambient CA存在时仍被拒绝，拒绝候选保留；正确首次交付，提交持久后客户端不消费响应、显式查询找回回执；重启保持OFF及revision，普通Kura切换授权未扩大。此为native contract，operator为native-sandbox-contract-operator；无Java，不是实际Kura/Web会话。
- 浏览器未知场景是派发前agent离线，不声称已验证浏览器提交成功后的响应丢失。旧公开暂存成功INFO日志过滤审计缺口保留；本次真实绑定持久admin审计已证实。

## 收尾与续接

新IDEA、Orb Kura/agent和Parallels unit均停止，所有材料/state/key/失败候选保留；Ubuntu与Windows均stopped。原Orb console/center2/agent及phase2-agent继续active，原包console26/agent20。未安装软件、未构建新deb/Docker、未提交推送、未改NM/Netplan。

产品pending：中心签发/注册认证与绑定后远程开关、身份故障运维/归档和轮换、完整组件适用面、Java XML/加密/factory/核心离线恢复及真实Kura业务核验、防火墙适配/NAT/远程/早启、restart/reboot任务、一致备份恢复及正式发行。opaque/unit验证不能计作业务恢复。

外部deferred：Parallels完整Java/Kura、真实NM/cloud-init/物理有线/硬件、amd64、磁盘满/硬断电。pending与deferred分别保留。等待用户明确继续后按最终台账选择下一未交付业务，禁止重放旧一次性入口。

机器证据：`../evidence/gwt-agent-enrollment-system-20261105.json`。原始日志/截图/前后18仓完整状态：`target/gwt-enrollment-system-20261105/`；所有18仓HEAD/branch未变。最终运行状态以其中final-runtime-state.json为准。

知识库最新全文交接：`Experience/2026-11-05-Kura本机身份交付系统验证与暂停-b5bdfd26.md`；Tasks/Session Summaries及旧4e5550de均已通过官方Obsidian CLI追加核定。
