# 2026-11-18 同一原身份的首次 native mTLS 连接

目标 active，现有积分继续，无推送。本次完成显式 Root gRPC 地址配置、同 E 源码升级和原 GWT 一次开启后的真实 mTLS/心跳。全部冻结 12 父项仍 0/12 最终验收闭环；此数字不能替代实现完成度或折算可信整体百分比。当前剩余范围仍为数周级，完整平台/故障矩阵可能更久，未承诺完成日期。

## 修复与边界

代码发现确认中心 HTTPS API `18443` 与 gRPC `18444` 分开监听。原登记界面标签为 HTTPS endpoint，前次输入了 API 端口；此前从未开启通道。保留原登记材料、回执、stage、私钥及网关，增加显式 Root 连接配置；不重新 CSR/签发/暂存/绑定/登记，不交换中心监听端口或改写 `.data` 绕过。

agent 代码 `96438c7`、修复 `efd0bb4`/`3e28a44`：Root 配置 15 字段/原安装-peer-result-角色/网关/原回执 SHA/最低 generation；同 HTTPS host、严格有效端口、无 credentials/path/query/fragment。Rust 解析及恢复/绑定/续期运行配置使用选定的 gRPC 地址。缺配置保留既有行为，坏配置拒绝；Root 当前代码配对继续完整检查。详见 agent `docs/native-transport-endpoint-v1.md`。

配置动作要求原 namespace、Core/native/两 socket 停止排空、远程 OFF、原完整首次身份和业务 0；全局原意图先持久化，intent/result/公开配置/current/ACK 哈希一致。中断仅按原号和同一未变的代码/保留态补全，不重做外部写。丢 current、额外资产、错回执/网关/角色、代码 epoch 变化拒绝。首个配置不可用新 ID 替换，完整更新/图形工作流仍 pending。

## 当前构建与测试

证据目录：工作空间 `.management-close-20261116/source-cohort-20261117/native-mtls-first-channel-20261118/`，原失败与实际回执均保留。

| 范围 | 实际结果 |
| --- | --- |
| Python Root/打包回归 | 266 selected，264 通过/2 既有导出夹具排除，含 16 新方法 |
| 干净 Git source Rust identity | 15/15，0 ignored |
| enrollment/renewal 回归 | 30 通过、4 个显式 Root/cross-UID 夹具 ignored；未算通过 |
| 制品 | release ELF 构建，四生成控制脚本编译通过 |

首次 Rust 编译错把 u16 当 Option，修正后地址测试暴露 URI 库将 65536 解释为无显式 port；改为检查 authority 原端口文本后全部相关测试通过。Python 追加全局意图与身份不一致须在公开发布前拒绝。最早 Root staging 的本地意图文件名与已有 source-export 回执冲突，O_EXCL 在任何 Docker dispatch 前拒绝；换专属文件名，不覆盖原证据。两端只读 UI 等待曾使用猜测的按钮/文案超时，读取实际标签后成功，未重发任何写请求。

协议从 Git `1c5842a47291921bfdea734eb1786baf5b419a57` 本次重新打包，与当前协议/workspace/lock 无 diff，crate SHA `61163caf87448c050693b4af36badc05dc01763166dfea6bc41cff6f940e70c2`，未正式发布。bridge 从当前 Root 配对 E 的已装 JAR 以 O_NOATIME 读取验证，SHA `3e0a432ac2d54b8acb9d471ace3e912b219d80876af936eeb6e846a2ad4a5eb9`；未使用旧 ELF/crate/证据中间产物。

| arm64 agent `0.1.0~native202611171817.3e28a44-1` | SHA-256 |
| --- | --- |
| deb | `bb3230eb083139d5ea0546531960444a31c85de4e0926a5ef3fd396b15033c13` |
| ELF | `f53f7994da6a8ed5e8fd88eafd8f1236c24ed35faed24a22f228af5b0821bdaa` |
| source manifest | `0cfe4cdf9d7b3e41c0c09dce84d157ef578feb44f568e23343ea4d26d6472135` |

## 同一实际现场

E CID `a60111a62e828e5b50535252a98e3eb3b219fc95f65a1565cdd1fd8dd0944414`；原 init `24560864-f974-46aa-940a-0a97ad926118`、peer `25abd835-cf55-4871-929d-87abee746024`、material `13d50bed-dab2-412c-90ed-73d634258fb9`、CSR `7c3361b3-3073-4394-be1c-e18d359525df`、candidate `42e96468-bcf9-4940-9b81-0bc4fe70d13f`、开关授权 `49e58164-08a1-4511-8b61-ab0e585c2467` 全部沿用。

source **`a12f9c21-c02c-4c7a-a63e-99cbfc4a6c8f`** 各一次 prepare/install，APPLIED、不自动启动。65 个当前停止态资产逐项精确，不预设 WAL/SHM 固定数量；原权限及原结果不重写，仅代码指纹沿 source 链推进。原 native identity/current relay 代码绑定更新到 `2822c7e6…f716c0e`、`d5d60810…37b7a`、`7155bdc4…64ffe`，publisher `5f0736e0…f8b658b59` 不变。

独立 Root 地址配置 **`2031e3d3-a90b-4ad8-8445-f5a5138ecb0f`** 只发送一次，APPLIED/READY。实际连接 `https://192.168.215.3:18444`；原材料地址 `https://192.168.215.3:18443`、原回执 SHA `cdbe13d4…ddfe4e`、stage SHA `adf0acc7…76ca8` 及所有身份字节保持。配置后原 65 资产精确；原开关授权 9 证明与新配置 7 证明在启动后逐项精确。

显式启动后 native **20323/ticks20606914**，UID/GID997、groups996/997，五 capability 集 0、NNP1、NRestarts0。Core **20608/ticks20606930**，UID996/GID989，11 effective/13 bounding capabilities、NNP0 的义务仍开。key4714/ticks18777292、recovery4640/ticks18759746、NM4668/ticks18777273 保持。agent/kura=true，原身份20/盘点材料32/Core6/local7/material5 精确，原 env 和 plan-key 等私有保持。业务7表/Root业务/启动计数 0，两 socket enabled/listening、NConnections0，NAccepted仍4/6。

先以同一 UID997、cap 全0、NNP1 完成单独 TLS1.3 双向认证探针（当时未发 Hello/未开启通道）。然后重新登录同一 GWT，先核 Root namespace/source/配置与原 OFF revision `148047bd-0da7-4b65-86a8-e75893f701ee`；沿原目的 **`8808df94-c590-42cd-88f7-b340b117f6e1`** 增存 source/route revalidation，不换目的或原 revision。写前 O_EXCL/fsync 留存、单次 route guard、明确 checkbox 后只发一次 setRemoteEnabled，HTTP200/`//OK`，无重复或阻断，XSRF/request body 未记录。

远程决策现为 **ON**，revision **`c693589f-03e5-4dc1-be19-56bd87363ea8`**、actor `kura-uid:996`。真实 agent `CONNECTED`，无 recovery/active request；原证书 `3681eb8603b4f432078929e673bdc5911c3366459612185964885e1265e79f1a`、同网关 `native-e-a60111a6`、generation1。D 数据库从无 epoch/boot/lastSeen 到 epoch **`4b8e3040-33d7-47b6-b8a4-62f160fceaa9`**、boot `38663672-c6a8-476d-a08f-437f4ca99f72`，18:25:27 UTC 心跳 sequence16、online/kuraAvailable=true。仍 1 网关/0 任务。本机显示 Connected to management center，中心显示 Online/Connected；截图和结构化读回已存。CSR/stage/bind/中心登记总计仍各1，页面错误0。

Root 原授权/地址配置回执中的 `remoteEnabled=false/unitStarted=false` 只描述对应动作没有启动或开启通道；当前真实状态由上述独立 remote 决策和 live status 表达，不能混用。

## 保留与下一步

18:25:51 UTC 原3现场、个人25文件、C9及原进程/计数、D29及原Main3135/ticks16649159 再次只读精确。文档前18仓干净、共同本地分支不变；冻结12父项区块 SHA `4e1aaca54eb94c3fc36266506c87a0781498aa797f0c18a01036f74a21584cc7` 不变。未清理 active runtime/个人配置/数据，旧 C 保留 CSR 不触碰，UNKNOWN 不重发/换号/跨 Root 查询；闭环备份/归档/轮换/恢复/900秒不重放。

本次未实现新的 HTML/CSS/client JS，没有把现有页面实测写成新 Modern Web Guidance 实现成果。完整跨刷新开关原目的/UNKNOWN UI 工作流、连接地址发现与图形入口仍内部 pending；此次单次正向回执不代表全部故障恢复矩阵。

继续受限真实只读任务、能力宣告与实际授权一致性、保留业务/续期/TLS/UNKNOWN 的 source/安装升级；typed brokers/Core 最小权限、完整六组件/provider/launcher/升级移除恢复/R017 和全部12父项。只有既有3 external deferred：NM/cloud-init/物理硬件、amd64、磁盘满/硬断电。JDK最低21，未追加27；G034仍未封存，不改写旧通过范围。
