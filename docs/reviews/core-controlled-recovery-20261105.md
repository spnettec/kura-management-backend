# 核心受控离线恢复增量（2026-11-05）

整体目标 active，未完成正式离线业务恢复。代码已实现启动期完整替换 Kura 管理的 Configuration Admin 配置，并用真实 DS 业务组件验证；尚未接通 agent 的完整恢复事务。

## 本轮实现

- `ConfigurationRestore` 通过公开 CM API 进行精确替换、缺失工厂实例删除、旧属性清除与单例/工厂类型切换；保留无 Kura 管理标记的其他记录。歧义标记、占用冲突和不可忠实备份的类型在修改前拒绝。
- `ControlledRecovery` 每 JVM 仅接纳一次确切请求/快照 hash。核心加载接纳配置，不合并旧 CM、不回退旧快照；受控 JVM 的配置、工厂、snapshot、rollback 公共写入口持续挡写。没有隐式成功或解锁接口。
- 工具 JAR 保留独立 validate 模式，新增可选 OSGi bootstrap，普通未指定请求时 no-op。显式恢复 profile 在 level1 挡住 SCR/业务，先初始化日志和 CM，持久保存 bundle 级别与加密 CM API 备份后应用配置；回执仅 CM_APPLIED，fullRecoveryVerified=false/businessHealth=UNVERIFIED。
- 标准 launcher 负责最终升到5，bootstrap 不竞争修改框架目标级别。失败请求停框架、保留证据，不清理/重放。当前有 external.pids 的 profile 明确拒绝，不能把旧网络/防火墙快照当现行权威。
- 支持当前共享 mapper 可忠实表示的标量/包装数组。primitive arrays、Collection、自定义对象的完整适用面仍 pending。原部署 key 必须显式选择，部署级 key 授权/一致性接线尚未交付。

## 验证与实际边界

受影响 reactor 416 项测试通过，0失败/错误/跳过。含真实 Equinox Configuration Admin 持久化重启合同、精确删除/替换/备份前拒绝、forked JVM 写闸门、默认 no-op 和恢复失败阻断。

IDEA 原生运行新隔离完整 Kura，284项清单（实际283个bundle），先种植真实 DS 业务实例 keep/remove=9 及单例旧属性，再正常停止。用新一次性目标恢复后，业务事件仅 `business.keep:2:true`；真实 CryptoService 解密验证，remove 未激活且未出现9，单例旧属性清除，核心读取2，普通 snapshot 写被阻断。26项旧 CM 配置的 AES-GCM 备份经真实 mapper/crypto 审计，包含 keep/remove=9 与旧属性。原 GWT admin/admin 中文登录、Count=2、目标快照编号通过。

OrbStack 同一现有 VM、独立新目录完整 Kura 按相同步骤实际通过，原 GWT 登录/Count=2/快照编号通过。未升级原包或复用旧一次性请求。云连接处于 DISCONNECTED，以上只覆盖所列本地业务断言，不宣称所有网关业务健康。

首次 IDEA 试验曾在 CM_APPLIED 后暴露日志初始化/launcher级别竞争问题，业务观察器与端口检查失败；该现场完整保留，未判通过、未重放。修正后使用全新 v2 目录和全新请求完成 IDEA/Orb 验证。

## 运行状态与证据

新 IDEA seed/apply、失败首套 IDEA、Orb seed/apply JVM 全部停止；请求、快照、密钥、CM备份、级别账本、日志和旧夹具均保留。原 Orb agent20/console26 三服务和 phase2-agent active；Ubuntu/Windows VM stopped。Parallels 无 Java/Kura，当前 Java/core 验证 deferred，没有以旧 unit/sandbox 结果替代。

证据 `../evidence/core-controlled-recovery-20261105.json`；完整日志、前后18仓状态、源码/制品SHA、场景状态和截图在 `target/core-offline-recovery-20261105/`。18仓 HEAD/分支与本轮开始一致；没有提交、推送、reset/clean、远端覆盖或个人 runtime 清理。

## 下一必要工作（pending）

接通 agent 的 allowlisted 受控启动请求发布、CM备份补偿、精确回执核定、桥接/业务健康以及明确完成和普通写释放；随后接入外部网络/防火墙当前 Rust revision，补齐完整组件/工厂/属性适用面与离线历史上传流程。不能用删请求、普通重启或 CM_APPLIED 代替正式完成。

完整原目标继续保留：防火墙适配/NAT/远程/早启、restart/reboot闭环、一致备份恢复、身份轮换运维和正式发行。真实 NM/cloud-init、物理有线/硬件、amd64、磁盘满/硬断电等外部 deferred 单列，不把尚未实现的产品功能改写为环境缺失。
