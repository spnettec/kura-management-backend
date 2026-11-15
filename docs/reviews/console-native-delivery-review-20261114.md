# G027 / V55 本机安装包图形化交付审查（2026-11-14）

本批已实现并验证：从 Cockpit 主机管理界面选择批准版本、核对本地 deb 大小与 SHA-256、上传、显式冻结并停止全部本机中心、安装、核对原回执及刷新恢复。用户无需设置环境变量或运行部署脚本。配套工具包默认依赖 Cockpit；显式 headless 维修构建仍可不带入口。空主机从安装入口到完整业务初始化的总体验收仍是内部 pending，不能据此宣称正式发行已完成。

批准包目录由构建时的实际核心包回执生成，固定包名、架构、版本、文件摘要及权限，不接受浏览器提供命令或主机路径。上传在受理前校验，受理后复制到不同 inode 并再次校验，防止迟到上传改变已受理内容。发行目录不作为核心 source manifest 的输入，避免自引用摘要。源码打包补齐嵌套 TLS 测试夹具和测试配置，保留此前失败记录。

安装使用独立 root worker 和原维护编号，要求所有参与实例已停止。持久化安装意图后只执行一次固定 dpkg 安装；不会隐式停服、启动、回滚或重试。postinst 只能记录完成并保留安装冻结；必须有原 dpkg 成功回执、文件和原 postinst 回执匹配，才可解除冻结。APPLIED 标签本身不够。成功命令回执缺失时维持 UNKNOWN，不能依据包已存在或进程停止推断未投递；只允许有原成功回执的显式文件核定，不重新安装。受理前取消保留墓碑，拒绝迟到原命令。协议 0 在线升级仍拒绝，保留为内部 pending。

验证结果：

- 完整 Linux root 回归 83 项通过；最终仅两处 JS/测试提示修正，Python 输入保持相同。
- 中文、英文浏览器契约 14 项通过，0失败、0skip，覆盖原编号、存储失败、上传摘要拒绝、未知回执、有限观察和冻结解除。
- 当前 Rust/React 运行输入完成 ARM64 普通 release、多阶段镜像及实际 DEB 构建；最终 source-v3 与镜像的 135 个运行输入逐项一致。普通 ELF 为 20bb95157abf7a93dd3b3fa04db7c60e0f9bc73fe894c3f0dc1e19eaeb2f3fa9，并非复用旧 V52 二进制。
- 真实 Cockpit/PAM 管理员界面在保留的 yofc-console-v52 测试机安装 0.1.0~v55-native20261114-2。核对实际浏览器 TLS 对端后才使用原测试账号；无 DNS、密码或全局配置更改。
- 新维护 029851a6-2c6f-4ba3-a43e-7a7456a54ccf 的三个显式停服回执齐全；新安装 fcf5b19a-e6f8-4dfc-baa5-b55a4194ccd2 仅上传受理一次、安装一次，真实 worker 返回 success/0，独立核对 12 个已安装文件的摘要、root 属主、权限及 dpkg 归属。
- 全部三个本机中心保持 inactive/PID0/NRestarts0；原 53 个私有文件逐字节保持。冻结解除只发生在成功回执校验后。刷新只恢复原 APPLIED 和维护 COMPLETE，不重装、不重发初始化、TLS 或启动。
- 真实界面发现的“状态已读取但仍显示核对中”提示已修复后才执行唯一安装；最终 14 项浏览器回归通过。早期浏览器观察超时、测试错误和旧候选包保留。

本批仅改 shadow 中 13 个源路径：8 个已有输入、5 个新输入。V55 冻结 2196source+4fixture；相对最后完整 CODE V51 有 120 输入增量。当前 121existing+194new=315 候选记录，含 3fixture-only，故 312 个 production/test 路径整体仍 UNAPPLIED。管理源码未向原仓应用，G025 已集成的 40 个上游源路径保留。V55 是源码及交付组件检查点，不是全量 CODE seal、系统沙箱、真实 NM 或正式发行验收。

原 Parallels boot、五 PID、agent affa64e2、767863a9 唯一 startup.stop UNKNOWN 和 a87d94d5 intent 前后精确，无 stop receipt/publication/start。旧 a79b 主机传输 UNKNOWN 与其他历史记录保留，不将历史未知改写为未投递。本次新受理操作均有实际回执；已闭环首次设置、TLS、GWT 三保存和 V36v1 均未重放。18 仓保留工作树；不提交、推送、reset、clean 或安装共享 Maven API。测试机、构建器及浏览器在新工作终态后正常关闭，包、数据、镜像、容器和缓存保留。

继续顺序：空主机安装入口与完整 GUI bootstrap/正式交付、中心地址与 DB 及 CSR/续期的原回执核定、agent 独立身份/CSR/续期/通道能力；随后按完整台账推进凭据部分恢复、首设/TLS UNKNOWN 只读复核、多管理员外部 CA custody/quorum、跨版本 DB/非空加密任务/备份恢复、正式 sandbox/polkit/协议0、完整 owner/ordinary/recovery 与原 forward-confirm-inverse-observation-stopped-expiry、合法 boot/netns/handle restore 和全 CM/runtime/NAT/网络矩阵、初始化先退役再原 inverse/子结算退役历史/readoption、14组/7API2/R017/312路径 guarded apply/正式 API/校验与 joint generations、G021 真实 LDAP、G022 Go31 vet/3非JavaPOM/可选 regen/.NET。仅 NM-cloud-init-physical、amd64、disk-full-hard-power 三组属于外部 deferred。

证据入口：同目录目标组件 target/nm-role-authority-20261112/registered-runtime-v52/console-installer-delivery-v1/，以 source-checkpoint-native-delivery-v55.json、final-binding.json、publication-manifest.json、final-verification.json 和 preservation-after-docs.json 为准。源码检查点 SHA-256：206d7527bdf3a9f59b77d555fbdcdf2c3d54dfe327f36378700ffa3b66952894。完整目标保持 ACTIVE。
