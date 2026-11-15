# 本机证书校验与暂存 v1

> 后续 agent 后端接收增量见 [live-agent-enrollment-v1.md](live-agent-enrollment-v1.md)。本文件保留公开证书暂存的既有证据范围；GWT最终身份提交已接线，最新见 [gwt-agent-enrollment-v1.md](gwt-agent-enrollment-v1.md)；完整页面和系统验证仍pending，不能将后端UDS测试称为完整页面绑定。

2026-11-05 增量，整体管理体系和首次绑定仍未完成。Kura Java 后端独立生成私钥/CSR；本增量接通申请恢复、公开证书校验与暂存，不启动/调用agent，不联系中心，不启用远程连接。

## 入口与权限

原GWT远程管理页持续中英，仍按正式agent安装标记显示。已有申请列表可恢复CSR和公钥指纹，不因页面重载重新生成密钥。不完整/损坏申请显示需要本机核查，不能从读取入口补造私钥。用户明确开始新申请才选新requestId。

本机RPC `listIdentityRequests` 与 `stageCertificates` 均在服务端检查ADMIN、XSRF及安装条件；暂存沿用固定Audit注解，不记录材料/私钥。现有RPC成功审计为INFO，本轮夹具AuditLogger=WARN将其过滤，**未取得持久成功审计证据**；后续身份提交须补可靠持久审计，不改动原全局日志策略来伪造本轮证据。密钥/证书操作不查找agent服务，agent进程离线只影响状态和最终身份交付。未安装时原控制台仍可独立使用。

暂存参数只有申请UUID、纯HTTPS中心地址、客户端证书链、客户端签发CA、中心信任CA。三类证书材料均为公开PEM；没有私钥上传或下载字段。签发CA用于离线PKIX客户端链校验，中心CA是未来验证中心TLS的独立信任材料，二者可不同。尚未验证远端中心身份或注册状态，不能把暂存成功称为认证/绑定成功。

## 校验及现场

- 保留CSR生成时的单一CN主体，网关ID大小写精确匹配；证书公钥必须匹配本申请SPKI。重新验证CSR签名/主体/SPKI，再用后端私钥作不导出的签名自检，确保当前私钥也未损坏/替换。
- 客户端叶证书不可为CA，必须允许clientAuth；KU存在时须允许digitalSignature。有效期、关键扩展及PKIX链验证，拒绝错误签发CA、不匹配主体/密钥、过期和重复/夹杂非证书内容。当前v1接受RSA≥2048、EC≥256的公开证书密钥，CSR固定P-256。
- CA须具备CA约束；KU存在时须允许keyCertSign。每份PEM输入/规范化输出最多64KiB、最多8张证书，不接受私钥PEM、尾随垃圾或混合数据。客户端链按叶证书在前提交；信任锚按DER摘要排序，PEM统一64列LF。
- 地址只接受无userinfo/query/fragment、无业务路径的HTTPS主机/有效端口。校验不发网络请求；PKIX撤销检查关闭，**此离线步骤不证明在线撤销或中心登记状态**。
- 每个Kura暂存区最多8个申请，目录0700、文件0600、属主与Kura数据目录一致，拒绝链接/非普通文件/额外文件。读/写共用文件锁。先校验全部材料再建暂存目录；写文件及目录fsync，最后写complete。相同材料规范化后可重复读取/提交而不改写；不同材料、部分写入、篡改及到期材料保留并拒绝覆盖，没有自动重试或清理。

## 后端交付格式（下一增量的输入，尚未交付agent）

`${kura.data}/management-enrollment/<requestId>/` 保留本次Java生成的private.pk8、public.der、request.pem、gateway.id、complete。私钥仅后端访问，不进入ConfigAdmin/配置快照、RPC、中心、网页或任务结果。

正式一致备份/恢复须另行纳入这个私有暂存目录及未决标记；ConfigAdmin快照不能替代它的备份，也不能作为恢复agent归属的依据。该整体备份工作仍在原pending台账中。

新增私有子目录 `certificates/`，只含client-chain.pem、issuer-ca.pem、console-ca.pem、manifest.json、complete。manifest精确9项：schema=`yofc.kura.remote.certificates.v1`、requestId、gatewayId、endpoint、publicKeySha256、clientCertificateSha256、issuerCaSha256、consoleCaSha256、notAfter。

publicKeySha256为SPKI DER摘要，clientCertificateSha256为叶证书DER摘要；issuerCaSha256/consoleCaSha256为规范化PEM信任锚**集合字节**摘要。certificateStageSha256为manifest原始ASCII JSON字节摘要；certificates/complete内容为该摘要加LF。notAfter为叶证书UTC到期时间。公开模型返回摘要/CSR/地址/到期状态，不返回私钥或后端路径。

后续常驻agent首次绑定必须另有明确本机授权、独立校验上述材料及私钥，按预期申请/摘要持久提交并保持远程关闭；不得接受任意路径、以普通注册抢占既有归属、重用此入口实现转移或恢复。当前没有调用旧停服导入命令来伪装常驻交付。

## 验证范围

宿主Java77通过，完整GWT六排列构建通过。真实签发的根/中间链、独立中心CA、错误用途/主体（含大小写）/密钥/有效期/未知关键扩展、CSR/私钥损坏、原件保留及权限顺序均覆盖。初次测试缺Gson依赖和BC构造调用错误已修正，失败日志保留；手写OSGi导入增加实际引用的ASN1/x509/X500包。

IDEA当前完整282运行集实际admin/admin原GWT登录、无agent隐藏菜单/快照可用。Orb原包安装标记下的新隔离完整Kura：实际OSGi/GWT中英申请恢复、不完整状态、暂存指纹/地址/到期显示和真实浏览器暂存RPC通过；同一材料提交前后全部申请文件摘要一致。没有启动新agent或联系中心，原Orb三服务仍原包运行。

Orb JRE实际运行12项受影响Java测试通过；没有javac，使用宿主编译的Java21测试类在VM JRE执行，未安装软件。Parallels上次实测缺Java，本轮Java/GWT未执行，保留deferred且VM维持stopped。此环境边界不把未实现的身份交付/注册产品归为deferred。

日志/私有夹具 `target/certificate-staging-20261105/`；新IDEA及Orb运行集均停止保留。首次绑定/私钥受限交付、证书轮换/转移产品、真实中心启停以及原未交付业务仍pending，未发布新发行包。

## 2026-11-05 本轮系统验证完成并按用户要求暂停

本轮最新核定见 [系统验证与暂停](reviews/gwt-agent-enrollment-system-20261105.md) 及 `evidence/gwt-agent-enrollment-system-20261105.json`。GWT空列表生产序列化与显式CA环境信任旁路已修复。真实IDEA完整Kura及Orb中英GWT交付/离线CSR/持久admin回执、Parallels最终agent严格沙箱通过；Parallels无Java，不代表实际Kura业务恢复。新夹具全部停止保留、VM均stopped，原Orb三服务原26/20及phase2-agent继续active。中心认证/远程开关与正式发行和原业务仍pending。等待明确继续，不重放一次性入口。
