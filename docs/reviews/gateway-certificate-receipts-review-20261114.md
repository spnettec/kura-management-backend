# G033 / V61：网关证书轮换的原操作回执

网关登记证书轮换原先可能用“当前证书指纹”推断一次旧操作是否成功。当前候选实现为每次轮换保留独立 UUID 登记和公开持久回执；原响应丢失后，页面只查询原操作，后续轮换不改变旧回执。公开证书模式默认启用，私钥留在网关；兼容的私钥 bundle 模式只在首次 POST 返回一次性下载令牌，回执 GET 不恢复令牌、不重新导出 bundle。

页面在发送前保存网关/用户/租户范围内的公开操作身份与摘要，严格比对原回执；缺少或不匹配时保留 UNKNOWN，不重发、不换 UUID、不从当前状态推断。新操作只能在原操作结果明确后再次确认。证书指纹复核绑定确切 PEM 字节，丢弃迟到复核和跨页面旧回调。中心成功只代表登记已更换，不代表网关已经装入或启用了新证书。

后端先提交短事务意图，再以网关行锁完成凭据、回执、审计和版本更新。缺回执、登记归属不一致、未完成意图，以及 QUEUED/RUNNING/UNKNOWN 写任务均阻止后续凭据写入；读任务可用。申请后再次出现写任务时，旧客户端也收到 HTTP 409，不能误报成功。CLI 轮换使用相同保护。复核中新发现登记被改关联到另一个网关时的保护缺口：隔离测试先复现失败，再修复双侧归属检查与最后 APPLIED 回执的登记关联；保留不一致历史，不自动修复。

本轮验证：中心单元测试串行 114 项、协议 24 项通过；5 项新隔离 HTTPS/PostgreSQL 测试通过，覆盖原响应体不读取、后续轮换、缺回执、错误密钥、租户/权限/CSRF、排队和不确定写任务、一次性 bundle、迟到写入及登记归属不一致。响应体丢弃和状态/时序注入不等于真实断网或掉电。默认测试仍有 45 项及 mTLS 3 项 ignored，未冒充通过。并行运行的 maintenance freeze 锁忙偶发失败仍是内部 pending；串行通过不代表并行缺口消失。

React 浏览器接口契约 24 项通过，最终文案另有 2 项视觉用例通过；旧组件对照显示其使用当前状态推断原结果的问题。浏览器使用模拟 API，真实 HTTPS/PG 独立验证，未声称二者集成端到端验收。最终 ARM64 Linux release、React 静态资源及配对 core/installer 候选 DEB 已生成；归档读取验证 core 9、installer 18 项关键文件、root 所有权与原生包目录配对。最终包位于 `/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/gateway-certificate-receipts-v1/package-v2/`，未安装部署，正式跨平台与完整组件发行仍 pending。早期 V1 磁盘不足、浏览器早期上下文关闭和旧版本构建均保留，不能当作最终通过。

源码检查点 `/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/gateway-certificate-receipts-v1/source-checkpoint-gateway-certificate-receipts-v61.json`，SHA256 `60eb3b6302497db8d4f14f177521a84a3db3388e1c478f1da99abe8203c86dcb`：2207 源码 + 4 夹具，较 V51 差量 135，123 existing + 205 new 含 3 fixture-only，共 325 条生产/测试候选 UNAPPLIED。11 个输入变化（7 个已有、4 个新文件），agent 与 GWT 源码保持 V60。最后 FULL CODE seal 仍 V51；本组件不是全量代码/发行封存。检查入口为本目录的 validation-result.json、source-review-v61.json 和 final-binding.json。

现场：18 仓源码、工作树、index、HEAD、refs、remote 与共享 Maven API JAR 继续保留。新的 G033 PG 已停止并保留数据及故障历史；无数据库行修复/回填。Ubuntu 旧 VM 当前 stopped，guest 只读观察报 PRL_ERR_IO_STOPPED；先前 boot/5 PID 仅为历史，不能继续宣称当前运行或当下回执缺失。本轮未启动/停止旧 VM，不重发旧 startup.stop UNKNOWN，也不自动启动以重建现场。该状态变化需后续只读核对，不能据此结算旧操作。

卸载 Eclipse 后另做了实际 IDEA 环境检查：javac 21.0.13 编译 537 个 Java 源文件；基础 JUnit 7 项与真实 Equinox 8 项通过；新组装的 278-bundle 隔离 Kura 从 IDEA 启动，HTTP 登录页及 GWT 脚本 200。测试 JVM 已停止，原运行目录和个人数据未替换；不是 18 仓全量重打包，未发现 Eclipse 安装依赖缺陷。

目标保持 active。完整剩余台账 `/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/remaining-implementation-v52.md` 全文保留：正式 GUI/平台交付与当前目录/完整组件版本配对；中心地址/DB/CSR 原回执及续期；已绑定 agent 身份轮换/续期/通道（未实现）；半凭据、首 TLS UNKNOWN、外部 CA 多管理员 quorum/bulk；跨版本 DB/非空加密任务/备份恢复；sandbox/polkit/protocol0 在线升级；owner/ordinary/recovery、boot/netns/handleRestore/CM-runtime-NAT、init-retire/child-settlement/history/readoption 全矩阵；14 groups/7 API2/R017/全部 325 候选 guarded apply 与联合代际；G021 LDAP、G022 Go/vet/POM/可选生成/.NET。新内部项包括并行 maintenance 锁忙和已停止旧 VM 的现场核对；不转成外部 deferred。外部仍仅原 NM/cloud-init/物理、amd64、disk-full/hard-power 三组。

不提交、不推送，不重放已闭环的 setup/TLS activation、GWT 网络保存等一次性验收。下一步在原操作回执保护上实施独立的已绑定身份续期；首次绑定入口继续拒绝覆盖已有身份，私钥不交给中心。
