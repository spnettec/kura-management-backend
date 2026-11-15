# 2026-11-08 d350b27d 实际密码存储与 Console CM 差异

从 62d9e9b8 续接；全部父指纹、18 仓工作树及原 Orb 四服务 PID/版本与两台已停 Parallels 状态实时匹配。只增加隔离夹具与证据，无生产代码改动、无构建或测试重跑；继承最终配置模块 189 项通过/install 的精确产物，三套 283 inventory 均核对。

## 实际验证

全新 IDEA Kura PID23897 / HTTP18616，唯一命令 password-verified-once。真实 org.eclipse.kura.core.data.transport.mqtt.MqttDataTransport 元数据有一个 PASSWORD 字段。保存检查点后创建较新快照，调用真正 Core.rollback(checkpoint)，15ms 返回、4 次 CM 变更尝试。实际 CryptoService.decryptAes 能把回滚后的 CM 密文解回原公开 Password 逻辑值；回滚新生成快照的 getSnapshot 连续两次只读解码也与原逻辑值一致，paired readRevision 公布同一逻辑值并发新令牌。旧快照字节全部保留，观察及解码未重发 writer。未输出密码值；此项证明当前默认密码值的实际存储/解码链，不能替代任意非空、数组/集合或全部 provider 验收。

Console 的真实元数据没有 PASSWORD；auth.methodPassword 是 Boolean 开关。ConsoleOptions 由 SCC 生成公开默认配置，初始 CM getProperties 为 null。回滚把七个字段持久化：SslManagerService.target、app.root、auth.methodCertificate、auth.methodPassword、kura.service.pid、service.pid、session.max.inactivity.interval，均从 absent 变为相应 String/Boolean/Integer。所有原 Console 公开逻辑属性保持一致、readRevision 成功发令牌。这解释此前 Console update 的正常默认持久化行为，替代 0d/62d 中尚未证实的“Console 密码差异”猜测；不需要为该猜测修改生产。

## 夹具错误保留

1. PID23742 /18614 错误假设 Console 必有 PASSWORD 元数据，预检立即拒绝，未调用 snapshot/rollback。
2. PID23826 /18615 改用真实 MQTT PASSWORD 后，未处理 Console 空 CM Dictionary，快照和回滚前 NPE；该隔离 fixture 的 getConfiguration 可能分配 dormant CM 条目。属于夹具错误，非生产失败，命令不重放。
3. PID23897 /18616 在新夹具中处理 null Dictionary 后通过。原失败收据、源码、所有现场完整保留，三个实例均已停止。

## 范围与下一步

此轮不是完整恢复验收，也不证明非空或密码容器/全部组件恢复、重启后 UNKNOWN、网络 owner/NM、全服务器时限。内部 14 项 pending 保留；外部 real NM/cloud-init/physical、amd64、磁盘满/硬断电 deferred 不变。没有提交推送，没有改原 Orb 或个人运行目录，没有重放已闭环验收或 UNKNOWN。

下一步在全新程序化 SCC + ManagedService 上验证带非空/空字符串的 PASSWORD 标量、数组、Vector，真实 Crypto/CM/快照/公开视图链与字段增删；随后按完整14项台账推进，不把该子证据提升为全系统恢复。

证据：target/rollback-password-native-20261108；机器清单 docs/evidence/rollback-password-native-20261108.json。目标 active。
