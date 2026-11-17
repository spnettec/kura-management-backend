# 2026-11-18 本机 Kura 开关授权与主机入口

目标保持 active，使用用户已授权的现有积分继续；无推送。此次完成独立 native 开关权限的代码、受限原号恢复、主机入口夹具及 E 固定 Root 控制器正向验收。完整 12 父项仍 0/12 最终闭环；已有子项不能折算成可信整体完成百分比。

## 代码与入口

- agent `a7e5d3c3f9343ee300fec515477b034afa6fe21c`：Root 原意图/前后环境/结果/公开证明/ACK/current、原号补全、稳定原 peer 范围和代码 epoch；启动 Rust 验证恰 13 字段 Root 证明与环境标志一致。只授予本机 Kura 控制开关，不开启通道。原 Root pairing 和 source 检查保持。
- console `a88ad2f03069044732298c24b04accf10724acf5`：独立 `AgentControl`，Root 保护的 9 个 agent 文件/source pins 在执行前核对；先发现本机 namespace，再核对浏览器保存的 initialization ID，最后查询/变更原请求。固定参数、清洁环境、拒绝任意实例/容器/路径/命令，不启停服务。
- 主机页双语原生 form/required 确认、48px label/按钮、动态 status、纯 `textContent`。公开 localStorage 只有版本、安装 ID、原授权 ID、步骤；Web Lock 内发送前保存/读回，跨 tab 只读采用已存在原记录，身份/source 在确认后变化要求重新确认，缺锁或坏存储不写 Root。未决不换号或重发。
- 主机图形入口/PAM 仅夹具验证。E 依赖预演需 110 新包和 1 个 Perl 升级，均未安装；图形实机验收及更轻的网关交付依赖仍内部 pending。正向授权由现有固定 Root 控制器完成，不能写成图形部署已完成。

## 构建与回归

证据目录：工作空间 `.management-close-20261116/source-cohort-20261117/native-remote-switch-20261118/`；仅引用当前实际记录，不重建历史链。

| 范围 | 结果 |
| --- | --- |
| agent Python | 250 selected，248 通过/2 历史导出夹具排除；含 23 新方法 |
| 当前 Git 干净 source Rust | 身份检查 10/10，0 ignored，780 filtered |
| console packaging 全集 | 129/129，含 Root 所有权/链接/源码配对/namespace/固定 CLI 边界 |
| 主机页/原安装流程浏览器 | 最终 60/60，含 13 新场景；双语 390px 图已读 |

前次测试互有覆盖，不求和。首轮 58 浏览器场景有 13 项 runner 默认语言不匹配、5 项异步断言过早；修正后 59/60 暴露真实存储异常未锁按钮的问题。统一存储错误/锁住提交后 60/60；原失败文件均保留。先前 Rust 格式器组件/运行库失败及 Root 授权 source epoch 初次失败也按原范围保留。

协议重新从声明 Git `1c5842a47291921bfdea734eb1786baf5b419a57` 打包，与当前协议、workspace manifest/lock 无 diff，SHA `61163caf87448c050693b4af36badc05dc01763166dfea6bc41cff6f940e70c2`；未正式发布。无旧 crate/ELF 输入，bridge 从当前 Root 配对 E 以 O_NOATIME 读取验证。agent 四生成控制脚本编译通过。

| 新制品 | SHA-256 / source |
| --- | --- |
| agent `0.1.0~native202611171716.a7e5d3c-1` arm64 | `7175a667bec36df3ac558930431eda057d686855e05abf02c68b18d35bc184cb` |
| ELF | `db55b68bbe7c8748caeb7fad95a849ff9b97ac33f06e3b7f6f4d8cc56b5c92ba` |
| agent source manifest | `fb2a9b26f6a2a8c7e283764bc067f89598769a8d0ded588cb4c02df9b110476a` |
| installer `0.1.0~host202611171715.a88ad2f-1` all，未安装 | `529f4699957d7c3bfb7b79e3b2ef7ec01ecc109ce26b7911ae3cda50f15541dd` |
| installer source manifest | `8ef1257aa0b0def4a737f8aa1304f84a4e84ea74247680d0f04ee1717d04ab21`；20 critical files、无 maintainer scripts、不启停中心 |

## 实际 E

同 CID `a60111a6…94414`，初始化 `24560864-f974-46aa-940a-0a97ad926118`、peer `25abd835-cf55-4871-929d-87abee746024`、material `13d50bed-dab2-412c-90ed-73d634258fb9`、CSR `7c3361b3-3073-4394-be1c-e18d359525df`、candidate `42e96468-bcf9-4940-9b81-0bc4fe70d13f` 全部沿用。

新 source `a8226dd5-fe5e-4631-a9ea-76bb89ee3130` 仅一次准备和 dpkg，`APPLIED`，不自动启动。56 停机资产指纹精确，原身份 20/Core 私有 6/local 7/material 5、回执及权限范围保持，旧授权仍 `APPLIED/READY`。新代码绑定：native identity `bbba1b9f…50fb36`、publisher `5f0736e0…f8b658b59`、relay pair `3a9cfb24…01809`、relay grant `3650d8cd…5917c2`。显式恢复后先验证非 Root/cap0/NNP1，未授予或开启通道。

随后独立开关授权 **`49e58164-08a1-4511-8b61-ab0e585c2467`** 由固定安装 Root 控制器只发送一次，`APPLIED/READY`。环境从 SHA `9698352f00e3e14d0cd6b475c40f8da0801d290ac70496b98ec18472b04c12e1` 到 `d8f51c992a47e33f1ff3bd32e0c30129fdcaf0831647103a6739aa746da77434`，恰改本机开关权限一行，通道/写能力/角色/接口不改。58 个原停机对象中其余 57 个指纹精确，9 个授权证明资产在启动后精确；既有证明不重写。

观察器的固定 56 数量假设失败后，实际差异是已有协议支持的空 `results.sqlite-wal`、32KiB SHM（997:997/0600）及启动产生的目录元数据，原 41 个普通文件精确，未改产品准入。后续只读核验另有换行转义、目录误传文件指纹、独立 snapshot 缺材料 context 的失败；最终使用原 enrolled context 通过。所有失败回执保留，授权未重发、数据未修补。

最终 Core **16836/ticks20298805**，UID996/GID989，11 effective/13 bounding capabilities、NNP=0 的义务仍开；native **16554/ticks20298790**，UID/GID997、groups996/997，五 capability 集全 0、NNP=1、NRestarts=0。key4714/recovery4640/NM4668 原 PID/ticks 保持。agent/kura=true，原身份/材料 32/Core6/local7/material5/native selected6 精确；7 业务表及 Root 业务/启动计数全 0。两 socket enabled/listening/NConnections0。

现用密码重新登录原 GWT，仅读取状态；本机开关确认可用、未勾选，开启按钮仍等待明确确认。原 CSR/stage/bind/中心登记各总 1 次、页面错误 0。remote 原 revision `148047bd-0da7-4b65-86a8-e75893f701ee` **仍 OFF**；不把 grant 当成已建立 mTLS/心跳。

## 尚待完成

实际 GWT 开启通道前仍须检查其原 revision、发送前留存与 UNKNOWN 处理；原号不重发或换号。接着验收真实 mTLS/心跳/受限只读任务、保留业务/TLS/UNKNOWN 的 source/安装升级矩阵。Cockpit/PAM、typed brokers/Core 最小权限、完整六组件/provider/launcher/移除恢复、R017 与其他 12 父项均内部 pending。

只有既有 3 external deferred：NM/cloud-init/物理硬件，amd64，磁盘满/硬断电。原 3 现场、个人现用配置、C/D、旧 UNKNOWN 及已闭环备份/归档/轮换/恢复/900 秒续期保持，不重放、不跨 Root 查询。

Modern Web Guidance 本次重新 search/retrieve 并全文读取 forms/accessibility/security；使用 required 确认、发送后锁定（“prevent double-posts”）、状态播报（“Announce dynamic errors via live regions”）和文本 DOM（“Prefer `textContent` or `innerText` over `innerHTML`”）。实际主机页认证链仍单列，夹具不代表实机通过。

17:37:03 UTC 原3现场、个人25文件、C9及进程/计数、D29与原Main3135再次只读精确；见 `phase21-preservation-after-grant.json`。18仓及冻结父项另有读前/后检查点。
