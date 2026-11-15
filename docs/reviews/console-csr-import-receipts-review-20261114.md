# G031 / V59：证书上传原回执与界面恢复

2026-11-14，完整改造目标 ACTIVE。本批解决 CSR 证书上传的校验拒绝响应丢失后无法核定的问题，未重放旧首次设置、TLS 生效、GWT 三次保存或原 UNKNOWN 操作。

## 行为与审查

新页面在每次上传前保存独立 UUID 和公开证书 PEM 的 SHA-256，再发出 PUT。服务端按 CSR 保存私有上传注册清单、原接受记录和校验拒绝回执；成功签回的原业务回执同时绑定上传 UUID、接受记录摘要及证书摘要。GET 原上传接口只读取已存在的证据，不校验、导入或生成材料。校验拒绝有持久回执时，用户可在同一 CSR 上选择更正后的证书，旧上传仍保留；成功回执可恢复成功显示，不重新上传。

没有原回执、注册清单缺失、已登记记录缺失、签回回执缺失、接收记录摘要不符时均保持 UNKNOWN。当前证书文件或当前 SIGNED 状态不能代替原上传回执。新编号、旧导入入口、CSR 停用和 TLS 生效不能绕过未决上传。同一原编号到达后只观察原记录，材料改变会拒绝。失败持久化后本进程不把可见文件当作成功 ACK。旧 Signed 序列化字节保留；旧 API 导入格式继续兼容，旧浏览器 IMPORT 没有独立编号时不补造关联，即使当前已有证书也不自动解除 UNKNOWN。

已配置管理员会话与首次设置配对会话分别提供相同的原上传读取接口。原认证、角色、Origin/CSRF 和维护写入门禁保留。新的审计 target 含 CSR/上传 UUID，实际数据库中两个上传各一条 REQUESTED；GET 不添加或重发上传。私钥、CA 私钥和证书正文不写入浏览器跟踪记录；服务端接受记录仅含公开摘要和提交人。默认部署仍使用界面操作，方案 A 的生产 CA 私钥继续位于中心和 VM 挂载之外，未宣称外部多人审批已经实现。

代码审查通过当前子仓图谱定位并读取 entries/outcome/validate_signed/import_operation/import_with_receipt/decorate/unresolved、CSR 导入与状态、HTTP/首次设置路由及 React 恢复逻辑。六个关键路径元数据与当前文件匹配；这是最佳努力的图谱覆盖信号，不代替完整源码或运行验证。

## 验证与产物

- 默认 Rust **114 项通过**；主测试 40 项 ignored，另 3 个既有 mTLS 集成 ignored，共 43 项默认未执行。新增 10 个回执单元和一个配对会话 HTTP 契约覆盖原拒绝、原签回、原编号只读、缺失/伪造回执、部分恢复、旧格式及失败持久化。
- CSR 浏览器契约 **26 项通过**，覆盖中英文及 setup/settings，含丢失拒绝/成功响应、编号或摘要不匹配、旧 IMPORT 未决、持久化失败和序列化请求限额。第一条红例等待条件不足，保留原失败；修正等待后的第二条红例明确证明旧页面没有上传 UUID。首次整组一条断言错误地要求输入框必须存在且禁用；新缺回执行为隐藏它。改为检查无可用上传输入和零重发，产品门禁未放宽。
- 独立 PostgreSQL 中只执行本批新增的 **1 个真实 HTTPS 测试，通过**。平台为 Darwin ARM64 原生测试程序，真实 TLS 主机名/公开 CA 校验，验证未登录/非管理员/CSRF/跨 Origin 拒绝。上传响应正文故意不读取，随后仅 GET 原编号核定拒绝及成功。之后将成功回执移到保留位置作缺失注入，GET 保持 UNKNOWN，TLS generation 未变、无生效操作。此为原响应正文未使用与回执丢失测试，不冒称真实断电或网络丢包。旧忽略测试没有批量重放。
- 类型检查和生产前端通过。当前完整 Dockerfile 多阶段构建通过，普通 Linux ARM64 release 为非 test、opt_level=3。宿主与 Linux 生产前端 3 文件字节相同。Linux 二进制 SHA-256 `b4c6824664e96687dc11a9d4ffa19b80897aae94002f8c31240536bba2ee8716`，镜像 `yofc-console-csr-receipts:20261114-g031`，ID `sha256:c7800e039207de21f67be13c1041cf98e93821a873a714bf2d09d37edb8b17e6`。
- 配套候选核心和安装工具均为 `0.1.0~v59-csrreceipts20261114-1`，核心 SHA `20be67077e5e126d4126e43d4301710183f42830e0d000c84ce24fbccda49e77`，工具 SHA `aa6b917d47e260cdf365d9fe522a0855cdf1702444aeb5b0d46e261dd3ffaff0`。自包含构建源 194 文件，manifest SHA `cae137d6a05238606415bdc4d2e7f8fc302b656a00063bff3d49beb9a1a9067a`。直接解读两个 DEB，9 个核心和 18 个工具负载摘要、权限和 root owner 精确；工具目录批准同一核心 SHA/源码/二进制，最大 GLIBC 2.34。

没有安装新核心或工具、启动正常中心、初始化数据库产品实例或替换保护中的 agent。真实 HTTPS 新接口验证与浏览器公开 API 契约分别成立，尚不是新流程的真实浏览器端到端/Linux 完整平台验收。正式 sandbox、完整 GUI 交付和整体改造仍内部 pending。31 个既有 packaging 输入未变，保留 V58 的 109 项 Linux Python 证据；不把已变 CSR 的 26 项契约说成整组旧 47 项浏览器输入未变。

## 当前权威与保全

证据根 `target/nm-role-authority-20261112/registered-runtime-v52/console-csr-import-receipts-v1/`。`source-checkpoint-csr-import-receipts-v59.json` SHA `d356b5dded3e928e61f9ab9cf8ff78cb5870cb7ecdc69c28a3e5a554379001a9`，**2202 source + 4 fixture**；本批改 13 增 2，共 15 输入，距最后完整 CODE V51 为 126。候选 121 existing + 200 new = 321 记录，含 3 fixture-only，即 **318 production/test 路径整体 UNAPPLIED**。这是路径数量，不是任务数量；组件封存不代表整体 CODE seal 或正式发行。

新 HTTP 现场 `/var/folders/py/fqrj3gh53qj8kv52ytt428vc0000gn/T/.tmphtEmTT` 和独立 schema `tls_review_41700f9cbe8d459eb0537bd9490d6ddd` 保留。CSR `fb93d088-9177-4288-bb99-de564b6fb20d`，拒绝上传 `0d1b49ec-ec9b-40fe-84a0-fe6614ce88c7`，签回上传 `4fd0f851-1c20-4c0b-9a51-0b6b745277f0` 最终在缺失注入后保持 UNKNOWN，没有再次 PUT。成功回执只在保留路径，不搬回 canonical 路径。3 个新 PG/制品提取/打包容器均 exit0/noOOM/restart=no，数据库卷保留；HTTPS/gRPC 临时监听均关闭。已在本批前运行的 BuildKit 服务按原状态保留，未停用或重配置。用户清理的旧中心/PG/60 容器不重建。

18 原仓及 G025 的 40 个已集成功能路径、1986 active/20 management/580 upstream pin、89 absent 和共享 API522 按本批 preservation 核对。原注册 Parallels 仍 V52，boot `4db3daf0-dca3-4bc5-bc9e-01cdd088e45c`、五 PID 2799/3635/81713/82137/59158、agent `affa64e2` 不变；原 startup.stop UNKNOWN、G029 GUI reboot UNKNOWN 和本批注入的上传 UNKNOWN 分开保留，不重发或补造回执。不提交、推送、reset、clean、局部 apply 或安装共享 API。

下一步继续完整台账中的中心地址/DB/CSR 暂存与续期异常核定、agent 独立身份/CSR/续期/通道能力及正式交付。多人外部 CA custody quorum/bulk、DB 非空加密任务/备份恢复、正式 sandbox/polkit/协议0、owner/ordinary/recovery/原 forward-confirm-inverse-observation-stopped-expiry、合法 boot/netns/handle/全 CM-runtime-network-NAT 矩阵、初始化先退役及子结算历史、14组/7API2/R017/318 路径 guarded apply/正式 API与 joint generations、G021 LDAP、G022 Go31 vet/3非JavaPOM/regen/.NET 均保留。只有原 NM/cloud-init/physical、amd64、disk-full/hard-power 三组 external deferred。
