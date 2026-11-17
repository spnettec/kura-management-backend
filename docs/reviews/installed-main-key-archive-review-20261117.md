# 实际 Main / GWT / Root 密钥备份与归档联合审查

2026-11-17。本次接受一个新建隔离环境中的受限实际链路。周用量显示100%后按用户要求只做必要交接并暂停目标；ordinaryUsageAllowed标志仍true，未开始下一项实施。
完整12父项继续内部 pending，仅3项 external deferred，G034未封签。
当前入口是[检查点](../checkpoints/20261116/installed-main-key-archive-checkpoint.json)及
[完整台账](../checkpoints/20261116/OPEN-TASKS.md)，不要求恢复中间产物或旧证据链。

## 修复及实际反例

| 问题 | 实际现象与修复 |
| --- | --- |
| Crypto API旧消费者混入 | 新API导出2.0，包中旧HTTP/REST/identity等仍导入1.x上界，Main进程在但HTTP/GWT未解析。重建消费者及依赖，重新配对离线工具；`prepare-key-service.py`在出包前按当前手写清单核对Crypto版本声明，拒绝旧产物。没有降低导入范围。 |
| 隐藏页签过早初始化 | 实装旧GWT首次打开密钥页后，归档刷新一直`aria-disabled=true`，没有读取归档。等Bootstrap完成选择后再初始化，忽略已选其他页签的延迟回调。旧包只读反例已复现。 |
| sibling代码所有权 | 更新GWT包后JAR为kurad所有，Root配对按合同拒绝普通Main重启；没有放松Root校验。GWT的两个JAR改为Root/0644，安装脚本移入Root拥有的`libexec/addons/management-ui`。注册表拒绝符号/硬链接及不安全目录，保留原顺序并改为Root/0644；postinst传播管道失败。 |
| 并发只读历史BUSY | 实际页签并发请求中`listOperations`返回GWT CONNECTION_FAILURE。纯历史/归档预览在打开FD之前对同JVM锁最多等1秒；写入仍立即拒绝争用，同线程重入和跨进程非阻塞内核锁保持。没有重发Root请求。 |

Core修复`9cee4264b7`，安装夹具`e016f60c03`，实际页签测试与说明`de5f4e6901`。
GWT页签`a2a5e644a3`、发行权限`6c79fa0b05`、并发读取`f135710e00`。
全部只存本地WIP，没有推送。Core的de5增量仅为测试/说明；生产包来自e016。

## 当前实际验证

- Maven 3.10.1 / Temurin21.0.13；181项相关JVM检查0失败/错误/跳过，含新增读取等待与第二写者拒绝。7项打包输入和7项Linux安装器检查通过；双GWT模块共12变体构建通过。
- 全新Ubuntu24.04 ARM64容器，真实PID1、声明依赖、Core及GWT Debian安装；只有lo，通过透明TCP转发访问实际TLS和生产页面。未替换Root、HTTP、ADMIN、XSRF或Main回复。浏览器由Playwright驱动，自签名测试TLS不代表一般信任链验收。
- 完成正常初始账户登录和界面强制改密。首次及再次选择密钥页签，8个实际历史/状态调用全部成功；Main和6份凭据/TLS/HMAC文件的原始元数据保持。
- 只创建一次新测试钥的加密备份，下载696字节；Root状态SUCCEEDED。原LEGACY与ADMISSION各进行一次元数据归档，2条归档对应4个intent/receipt文件。Root原密钥记录字节及元数据不变，原文件和编号保留。
- 实际独立恢复服务以UID995运行，经HTTPS API只读解密核验同一备份，`sameDeviceKey=true`、`keyReady=true`、`restoreAllowed=false`。证书指纹与安装证书匹配。健康Main期间页面按合同禁用恢复材料，此次没有点击冷恢复或修改该限制。
- 原生阶段Main PID15506/UID997保持，Root PID5538/UID0、恢复PID5519/UID995均实际存在；Root和恢复全能力集0、NNP1。Main仍11 ambient/13 bounding、NNP0，完整broker继续pending。
- key操作1、元数据归档2、恢复0、轮换0、日志化启动0。专用环境的原操作均正面结算，容器、临时账户口令、备份口令和备份文件随后删除；不得重建或在其他Root中查询原编号。

| 关闭的原编号 | 类型 |
| --- | --- |
| `cd2ee5cd-1c6c-4e6f-9e82-c545231c50d8` | backup-create / SUCCEEDED |
| `eb3a5a16-256f-4626-a8b4-8ee7af425b9c` | LEGACY元数据归档 |
| `47ecb515-0524-4b6f-82b5-1eaed22ac737` | ADMISSION元数据归档 |

## 制品及保留

Core包`kura_6.0.0~git202611170024.e016f60-1_arm64.deb`，203,499,552字节，
SHA256 `ad56fb864d90f9f95a99a5d8862c4d37377378b41fc160026a2ad5379012cb79`。
15个Crypto契约、离线reader配对及47个实际包内文件核对通过。
GWT包`kura-management-ui_3.0.0~git202611170052.f135710-1_all.deb`，34,095,166字节，
SHA256 `35b3fc08cf84c5021712eca3188128a31dab74df48f84bf5b744ff467d2f665c`；
包内JAR SHA256 `a2306b5a04a322a8dcff48f1a47120821dae1e53e8b9b73a1e73534a69035e3f`。

原3现场/7进程/38私有文件元数据/6份旧数据指纹逐项保持。原UNKNOWN
`767863a9-fe6b-4135-9783-ddcb131030af`及原startup.stop未重发、换号或手动结算。
旧一次性轮换/恢复/真实900秒续期验收没有重放。

删除重复包、zip/SFX及测试秘密等712,958,966字节文件，结束1个已完成容器，
其可写层逻辑大小1,294,621,968字节。宿主可用空间从412,210,839,552变为
412,937,846,784字节；不能把逻辑删除量相加当成物理释放量。未创建备份，未删卷。
保留当前包和stage、b7/c369 Core及a693 GWT旧升级输入、可复建的轻量测试镜像。

## 下一阶段边界

其余11个sibling发现同类daemon所有代码映射：artemis、camel、cloud、container、
deployment、networking、opcua、position、triton、wires、yofc-runtime。
需分别核对JAR/可执行安装代码、真实运行数据和各自生命周期，不能批量放宽Root检查。
Networking的脚本布局不同；不能把所有`.data`或数据文件盲改为代码权限。

本次只覆盖Core＋GWT及独立恢复API只读材料核验；完整六组件、全部provider数据盘点和
原子重加密、全部模式/升级/purge、broker、人工体验及其余完整台账继续pending。
仅实际NM/cloud-init/物理硬件、amd64、磁盘满/硬断电为external deferred。
冻结台账SHA256仍为`4e1aaca54eb94c3fc36266506c87a0781498aa797f0c18a01036f74a21584cc7`。

## Modern Web Guidance

检索并全文读取`tabs`及`accessibility`。本次保持现有导航，只修复页签显示与归档按钮
可用状态的同步，落实“Match ARIA implementations to actual behavior”；没有扩展为全站改造。
