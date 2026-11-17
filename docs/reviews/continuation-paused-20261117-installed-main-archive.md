# 额度边界暂停交接：实际 Main 密钥备份及归档

2026-11-17。用户要求周额度耗尽时暂停，少量额外credits仅用于必要收尾。
接口当前显示周用量100%，ordinaryUsageAllowed仍true；按该额度边界只保存交接，
不开始下一项实施，不使用可用的额度重置。恢复时先实时确认周额度，再继续。

工作空间 `/Users/heyoulin/iot-kura-develop/git`，18仓本地分支
`wip/management-close-20261116`。Git源码是权威；不需要还原中间产物或旧证据链。

## 恢复入口

1. 先读工作空间AGENTS.md、共享知识库规则/技能，使用父工作空间与官方Obsidian CLI读取最新交接、Tasks.md和Session Summaries.md。不要另建子仓库vault或混入全局。
2. 全文读[当前台账](../checkpoints/20261116/OPEN-TASKS.md)、[README](../checkpoints/20261116/README.md)、[新检查点](../checkpoints/20261116/installed-main-key-archive-checkpoint.json)及[本轮审查](installed-main-key-archive-review-20261117.md)。
3. 实时核对18仓工作树/分支/HEAD/树指纹、实际保留进程/密钥及数据。按需从源码重建必要缓存，不重建旧环境或已闭环的一次性场景。

## 当前已接受的有限范围

Core源码`de5f4e6901`，生产包来自`e016f60c03`（之后仅追加测试/说明）；
GWT源码`f135710e0038aee6224f45b6a81b29bd59f7b345`。
修复旧Crypto API消费者混包、隐藏页签初始化、GWT插件/安装器Root所有权、并发只读历史BUSY。
181项JVM、7项包输入及7项安装器检查通过，双GWT模块12变体构建通过。

实际Ubuntu24.04 ARM64、PID1/systemd、Main、Root UDS、HTTPS和真实ADMIN/XSRF登录
已参与一个新测试环境的受限联合链路：一次加密备份下载，LEGACY及ADMISSION各一次
元数据归档，独立恢复HTTPS API只读解密核验。原生阶段Main/6份凭据元数据保持，
Root原操作字节不变；恢复0、换钥0、日志化启动0。Main仍11 ambient/13 bounding、NNP0。
此结果不代表完整broker、所有provider/模式、六组件发行或人工体验验收。

新场景已完全结算并删除，以下ID只保留历史，**不得重建、重发或在其他Root中查询**：

- backup-create `cd2ee5cd-1c6c-4e6f-9e82-c545231c50d8` / SUCCEEDED。
- LEGACY元数据归档 `eb3a5a16-256f-4626-a8b4-8ee7af425b9c`。
- ADMISSION元数据归档 `47ecb515-0524-4b6f-82b5-1eaed22ac737`。

新测试容器、测试口令和696字节备份已删除。Root归档为2条、4个intent/receipt文件。
恢复页健康态按合同禁用恢复材料输入，本次只调用只读inspect API，没有重放冷恢复验收。

## 优先继续

先分析并修复其余11个sibling发行包的daemon所有代码/提权脚本映射；GWT已修。
涉及artemis、camel、cloud、container、deployment、networking、opcua、position、
triton、wires、yofc-runtime。分别识别可执行代码和真实运行数据，保留NM后端与既有数据，
不批量改所有`.data`权限、不放松Root配对。Networking布局不同，需单独处理。

随后继续完整六组件、全部provider/launcher/模式和安装升级移除矩阵、完整密钥生命周期，
以及[冻结12父项](../checkpoints/20261116/OPEN-TASKS.md)的全部剩余工作。
冻结全文SHA256 `4e1aaca54eb94c3fc36266506c87a0781498aa797f0c18a01036f74a21584cc7`。
只有实际NM/cloud-init/物理硬件、amd64、磁盘满/硬断电为external deferred，其余内部pending。
G034未封签，最后接受G033/V61、最后完整封签V51。界面使用modern-web-guidance渐进改造，仍为次要。

## 现场及清理

原3现场/7进程/38私有文件元数据/6旧数据SHA保持；原UNKNOWN
`767863a9-fe6b-4135-9783-ddcb131030af`及startup.stop
`a87d94d52839e316845b7c831b265daf928f0bfd5fb0a883fb49b721cba5f63c`
只留原现场，绝不重发、换ID或手动结算。旧900秒续期等已闭环ID同样不重放。

当前保留Core包`kura_6.0.0~git202611170024.e016f60-1_arm64.deb`和
GWT包`kura-management-ui_3.0.0~git202611170052.f135710-1_all.deb`、当前stage及
b7/c369 Core和a693 GWT旧升级输入。确认无用的重复包、zip/SFX、测试秘密等已删除，
无备份；个人现用`.kura-dev`和运行中target/runtime未操作。
本轮没有推送。恢复后继续用户授权范围；未满足主线门禁的源码留在本地WIP。
