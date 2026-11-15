# G032 / V60：原 GWT 公开 CSR 下载与证书文件草稿

2026-11-14。目标保持 active；本组件完成代码与隔离浏览器验证，完整改造未完成。

原远程管理页面已有 Kura 独立生成私钥/CSR 以及公开证书校验、暂存后端，但用户需要复制粘贴。候选页面现增加“下载公开 CSR”和客户端证书链、签发 CA、中心信任 CA 三个文件选择入口。选择文件只在浏览器填写草稿，原有“校验并暂存”、首次绑定确认与远程开关保持独立。后端和 agent 无改动；已有身份续期不在本组件完成范围。

新增 PublicPemFiles 使用本地 FileReader/Blob；每文件最多 65536 字节，只接收公开 CERTIFICATE PEM 块，拒绝私钥、混合块、非 ASCII、空或超大文件。此处仅作格式检查，证书链、网关身份、公钥匹配、用途、有效期仍由现有后端校验。选错文件保留原草稿，文件读取期间锁定控件，过期回调不能跨 CSR 请求填入。CSR 下载维持后端原字节，文件名绑定公开请求 UUID，Blob URL 用后释放。

验证：

- Maven 3.10 / JDK 21 私有缓存完整构建成功，GWT 2.12.2；login/denali 各 6 个正常发布 permutation，Web Java class major 55。
- 真实编译 GWT widget + Chrome 原生文件操作，中英文各 19 项、共 38 项通过。旧 widget 对照缺少下载及三处选择入口，预期失败成立。涵盖 64 KiB 边界、私钥/混合/二进制/错误块、重复选同名文件、异步读取失败、故障注入迟到回调、请求切换、无上传和无 prepare/stage/bind/channel RPC。
- 新隔离 Java 夹具直接调用未修改的 RemoteIdentityStore 生成两组独立 CSR，不依赖 agent。中英文四个实际下载文件与原 CSR 字节一致，OpenSSL 均验证签名。公开文件选择使用测试叶证书；未执行真实密码学暂存或绑定。
- 私钥留在新夹具 private/，文件 0600、目录 0700；未创建签发 CA。公开静态夹具无真实私钥。未启动中心/agent、未新建 VM/Docker 对象，Chrome 及静态服务已退出。
- 新产物 `/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-csr-files-ui-v1/production-build/kura-management-ui/bundles/org.eclipse.kura.web2/target/org.eclipse.kura.web2-3.0.0-SNAPSHOT.jar`，SHA256 `e30b5f5cd139150a6850c963b1355c86ccab0b1d06f82f30518baa790de85877`。尚未安装；未声称真实 OSGi 页面、agent 绑定或完整平台交付通过。

源码检查点 `/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-csr-files-ui-v1/source-checkpoint-agent-csr-files-v60.json`，SHA256 `6b91be7275c6c5a13dd404deec11fdaf976fb2a753062449b5dcf0d2368f41f4`：2203 源码 + 4 夹具条目，较 V51 差量 130；122 existing + 201 new 含 3 fixture-only，共 320 条生产/测试候选 UNAPPLIED。本次仅 3 个已有源码变更及 1 新 helper。console Rust/React 无变化，G031 的 Linux release / 候选 DEB 为继承组件，不重建、不安装。最后 FULL CODE seal 仍 V51；原运行现场 V52。

证据入口 `/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/agent-csr-files-ui-v1/validation-result.json`、`harness/browser-v2/report.json`、`source-review-v60.json`、`final-binding.json`、`final-verification.json`。早期 driver 路径/只读克隆权限/候选清单假设和 browser v1 选择器问题均保留在本组件，已纠正；不属于已通过的产品失败。原始现场只作只读观察：boot、5 PID、agent hash 与 startup.stop UNKNOWN intent 一致；没有 stop receipt/publication/start，不重发。

完整剩余工作继续以 `/Users/heyoulin/iot-kura-develop/git/kura-management-console/target/nm-role-authority-20261112/registered-runtime-v52/remaining-implementation-v52.md` 为准：正式 GUI 全平台交付、中心地址/DB/CSR 原回执和续期、agent 已绑定身份轮换/续期/通道、半凭据和首 TLS UNKNOWN、外部 CA 多管理员 quorum/bulk、跨版本 DB/非空加密任务与备份恢复、sandbox/polkit/protocol0 在线升级、owner/ordinary/recovery 全闭环、boot/netns/handleRestore/CM-runtime-NAT 网络矩阵、init-retire/child-settlement/history/readoption、14 groups/7 API2/R017/全部 320 guarded apply 与联合代际，以及 G021 LDAP、G022 Go/vet/POM/可选生成/.NET。内部 pending 不改为外部 deferred；外部仅保留原 NM/cloud-init/物理、amd64、disk-full/hard-power 三组。

18 仓保留，不提交、不推送；不重放初次 setup/TLS activation、三次 GWT 网络保存等 CLOSED 验收；所有旧 UNKNOWN 保留原始身份、不重发、不回填。下一阶段优先收敛已绑定身份续期及正式交付的实际缺口，沿完整台账实施。
