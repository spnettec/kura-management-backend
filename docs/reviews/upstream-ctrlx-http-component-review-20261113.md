# 上游 CtrlX HTTP/TLS 功能组 v13

2026-11-13，按功能整合 94e114ee9abe67d668314c4ca2837871edcaa7b7。
证据位于 `target/upstream-logical-sync-20261112/ctrlx-http-functional-v13/`。
已 source-only 应用 100 路径：5 modified / 8 new / 87 retired。当前 present
源码 authority 为 `../applied-source-pins-after-ctrlx-v13.json`，504 pins；
absence authority 为 `../source-retirements-after-ctrlx-v13.json`，89 路径（此前
配置组 2 + 本组 87）。继续核对两者，不能只检查存在的文件。

## 完整来源和关联

原提交 97 文件有 18 份 patch 缺失。取得完整 parent/after CtrlX 子树并与
97 路径逐项匹配：92 旧文件、10 终态文件全部具备 Git blob 校验。74 完整
删除 patch 先由父树 blob 独立校验，其余通过 28 次完整源读取补齐；没有用
截断 diff 推断删除。初次提取遇到 null patch，保留已有 74 份完整结果后只
继续缺失项。实际 fork 的 modified 文件只有 POM 有差异，另外 SDK 内部有
Jackson 3 改造，所有实际原像均保留。

移除复制的 Data Layer/identity-manager SDK 与 OauthHelper，改 JDK HTTP/1.1 +
SPI Json。保留三个 GET 节点/浏览端点、token login、整段节点路径编码、browse
总量/深度上限和异常 future 完成。read/write/subscribe 仍未实现，metadata
仍报告 unsupported；不是新增可用的 CtrlX 读写功能或真实设备验收。

18 仓静态文本查询只有原 CtrlX 内 92 个文件匹配旧 SDK，外部源码/配置匹配为
零；不声称外部用户或动态类加载穷尽。既有 7 nonfork 的 327 Maven+1 Cargo /
source/assembly/runtime 关联台账保留，不直接 merge 非 fork。完整 YOFC/OSGi
消费仍在最终 cohort gate。

## 两项必要适配

直接采用上游的 NoHostnameCheckTrustManager 会把所有 extended overload 降为
两参数调用。对照本机 JDK X509TrustManagerImpl 确认，这会丢失连接级算法约束、
协商签名算法及 stapled OCSP 上下文；[JDK trust-manager 合同](https://docs.oracle.com/en/java/javase/21/docs/api/java.base/javax/net/ssl/X509ExtendedTrustManager.html)
也要求带连接参数的检查。独立契约先出现 3 个“应被策略拒绝却放行”反例。

现在只对 JDK HTTP 所用的 server SSLEngine 提供 endpoint-identification 为空
的上下文视图；原 engine 参数不被修改，session/constraints/协商状态继续传给
extended delegate。client/socket 检查保持完整原上下文，socket 不作主机名放宽。
基本型、不能接收连接上下文的 provider 在需该上下文时明确 fail closed，不
静默削弱策略；没有全局 TLS property。原上游测试中要求降级到两参数的断言
按正确合同适配，原始文件/红测保留。没有声称已测试真实 OCSP responder。

实际 loopback HTTPS 验证：匹配证书默认成功；主机名不匹配默认在 HTTP 前拒绝；
显式 ignore-common-name + 正确 pin 成功；同名不同公钥仍在 HTTP 前拒绝；
extended 策略 veto 仍先于凭据发送。另验证缺失凭据变为 checked connection
error 且未发送 HTTP。全部使用新生成测试密钥；没用真实设备、OEM/中心 CA 私钥。
factory 默认弱证书仍是原有显式选项，不声称持有其私钥做过现场验证。

另外，JDK HttpClient.close 会等待响应完成。新受控“设备不回复”反例证明它
可卡住连接关闭；改为立即 shutdownNow，并清理登录失败路径。当前 JDK 实测
关闭在 1 秒内返回、等待调用在 2 秒内失败、peer 未放行、请求未重发。该 API
是 best effort，不能把此结果当成所有实现的完成保证，依据 [JDK 生命周期 API](https://docs.oracle.com/en/java/javase/21/docs/api/java.net.http/java/net/http/HttpClient.html)。

## 验证和受保护应用

最终 `ctrlx-final-v6.json`：两模块共 168 tests，167 passed / 0 failures /
0 errors / 1 原有手工 BrowseTest skip。RAT、Karaf feature 静态验证、JaCoCo
通过；instruction 83.01% >= 80%。初次 adapted v3 只有 75% 而失败，未降低
门槛，补充有意义的连接上下文/实际 HTTPS/生命周期验证后通过。旧 SDK/Oauth
宽排除随退休移除，原 connection/discovery/factory 排除保留。

新 JAR major 65，不含退休类或 Jackson/Apache HTTP direct imports。旧增量
输出留下 4 个空目录条目，最初打包检查误把它们算作旧类，已保留诊断并按实际
文件核对；没有旧 class，没有为此清理现场。测试密钥、证书、日志继续留在
私有 target，并有 hash。

`applied-to-working-tree.json` 有 create-only journal、92 个实际原像及全部
18 仓范围外 HEAD/branch/refs/remotes/index/staged/tracked/untracked source /
retained metadata 比对；均精确。旧 491 pins 和此前两项 absence 精确；
management 1986 inputs / 20 pins、R013 v32 binding 不变。console/PG ID、
health、start 不变。未 install 本项目/shadow Kura API，未 deploy/restart、
commit/push/reset/clean。UNKNOWN 不重发，不重放已闭环管理现场。

## 续接

剩两组：按真实引用处置 Jackson/Woodstox 依赖、commons-lang3。fork 审计日志
仍用 Jackson 3，不能照搬“全 plc4j 已不用 Jackson”的上游结论。完整 97 模块
串行构建、全部 Java --fail-at-end 和受影响 YOFC/OSGi gate 后才关闭新 7 组
cohort；global accepted 仍 e591，不越过 pending 到 advertised 671。

再返回同一 R013 v32 shadow 完成 factory/private init/imported layout/owner
整链。137 管理 candidate 仍 UNAPPLIED。原 14 管理组、7 API2、R017、A 外部
CA/custody 和仅 3 external deferred 全部保留。目标 active。
