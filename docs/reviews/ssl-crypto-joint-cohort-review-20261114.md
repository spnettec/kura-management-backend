## 2026-11-14 G025 / V53：SSL 与 Crypto 按功能协同完成源码集成，整体改造继续

源码组件：`target/upstream-logical-sync-20261112/ssl-crypto-joint-cohort-v21/final-binding.json`，SHA `cb5e1924a67d4d545b9fca32775a346890f1f024d68eff447d6a9ca7270c4a54`。本组件只封存G025源码协调结果，完整改造仍ACTIVE。

本批对上游 core 6e1b1380/7eca88b2、deployment 0c32ff7b、triton 9fc62c85、wires 836e974e 做完整接口、实现及消费者适配，向 11 个原仓写入 39 个已有文件和 1 个新测试，共 40 路径；没有 merge/commit/push。旧 String 加密及文件式 SSL 接口移除，Crypto 包 2.0、SSL 包 3.0、core.ssl 包 2.0 配对。char[]、流、现有密文/密钥格式、YOFC 空服务证书库回退及异常原因保留。指定托管别名现在限制实际使用的凭据，同时遵守证书库的别名大小写规则，不回退到其他凭据。

隔离普通源码版本完成 406 项单元测试；与全部 R013 管理功能组合后完成 3,108 项，均 0失败/0错误/0skip。每个版本有 13 个真实双向 TLS 场景，覆盖 Socket、SSLEngine、TLS1.2/1.3、大小写别名及缺失别名拒绝。Keystore 的真实增删、缓存、CRL、密钥及错误回归保留。没有修改 CA 托管边界或生成现场新密钥。

两个版本分别完成 179 项 Maven reactor 的生产及测试编译、GWT 生产策略和 JAR；GWT 自有 class 为 Java11/major55，核心 major65。13 个发行元数据/DEB 模块因隔离 Git 为 unborn 无法取得 HEAD，发行包不算通过；后续编译明确排除这 13 项。普通 test 生命周期不能给现有 GWT 策略测试提供生产 JAR，File construction mock 也干扰目录 classpath；按 package 生命周期运行后全部通过，没有降低断言或屏蔽测试。失败记录保留。

真实 Equinox 仅 init、不启动业务 bundle：普通版本 36 个、联合版本 37 个实际生产消费者均解析；包括 NM、Triton、恢复工具、GWT 和 YOFC 应用。两种服务实现与实际 API 类身份一致，旧导入范围负例被拒绝，无 mock provider。首版探针启动 framework 后触发依赖激活的失败记录保留，接受版本固定独立 storage、instance、user/install area 且业务启动数为 0。这只证明包绑定及实现接口一致，不是实际服务生命周期、系统网络或整机验收。

完整旧 2181source+4fixture 和旧工作候选归档于 accepted-shadow-before-g025。仅将已验证的 9 个 API/元数据源改动同步到 SAME 工作 shadow；新 source-checkpoint-after-ssl-crypto-v53.json 仍 2181+4，相对最后完整 CODE V51 为 105 输入增量。当前 121 existing+179 new=300 候选记录，含 3 fixture-only，故 **297 production/test 路径整体 UNAPPLIED**。减少的一处是已由本批原仓集成覆盖的 core.crypto MANIFEST，不是一个业务任务完成。旧 V51/V52/G023/G024 seal 全部原字节保留；注册运行现场仍 V52，新 V53 仅为源码协调检查点，未宣布全量 CODE seal 或发行。

本次只读核对的 11 fork tip 均无新增，5 个待处理提交已按功能集成，完整 reviewed 游标可推进到本次已公布 tip；7 个 nonfork 仍根据来源和供应/消费者关系处理，不直接 merge。当前上游 source pins 为 580：旧 546 中 540 原字节保留、6 声明的版本文件由本批替代，新增 34；89 退役路径不复活。1986 原 management 输入与 20 先前管理 pin 的过渡单列，不强行声称旧 SHA 仍当前。共享 API522未安装或改动，18仓 HEAD/branch/index/staged/refs/remotes 不变。Git 续接 authority 使用本组件的 preservation-after-docs.json。

原 767863a9 唯一 startup.stop UNKNOWN 与 a87d94d5 intent 不变；仍无 stop receipt/publication/start/complete/activate/cancel/re-export。原 boot、五 PID 和 agent affa64e2 精确，OrbStack 同一 owned machine 保持 stopped。已闭环首次设置/TLS/GWT 三保存/V36v1 不重放。无提交、推送、reset、clean。周用量24%，ordinary allowed，credits与2次reset未使用。

下一实际实施回到 installer/bootstrap 界面及正式交付、中心 CSR 下载/外签上传与地址/DB、agent 独立身份/CSR/续期/通道能力、凭据部分恢复/UNKNOWN只读核定、多管理员外部 CA custody quorum。下方完整台账及 remaining-implementation-v52.md 保留：跨版本 DB/非空加密任务/备份恢复、正式 sandbox/polkit/协议0升级、完整 owner/ordinary/recovery/原 forward-confirm-inverse-observation-stopped-expiry、合法 boot/netns/handle restore/全网络 CM-runtime-NAT 矩阵、初始化先退役再原 inverse/子结算退役历史、14组/7API2/R017/297整体 guarded apply/正式 API/协议校验与 joint generations、G021 LDAP、G022 Go31 vet/3非JavaPOM/regen/.NET 都是内部 pending。只有 NM-cloud-init-physical、amd64、disk-full-hard-power 三组外部 deferred，目标保持 ACTIVE。

