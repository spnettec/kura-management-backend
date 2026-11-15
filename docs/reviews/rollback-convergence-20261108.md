# 2026-11-08 62d9e9b8 回滚视图收敛与密码存储

从 0d9a5379 续接。入口核对父清单和 18 仓工作树；247 个可能变化的旧指纹已保存原字节。

配置 Core 增加私有 RollbackConvergence：预检时分开冻结 CM 存储值、逻辑快照值和公开视图期望。成功 CM 回滚只清理目标范围 UNKNOWN，并保留待观察收敛门禁；SCC 仍公布旧值时 readRevision 返回 CONFLICT，收敛后才发令牌。原快照公开字段必须匹配；未公开的元数据默认值可缺省，若公开则必须匹配；已移除额外字段不能被旧视图重新带回。运行时 identity 不参与业务字段收敛。

快照 PASSWORD 是 XML 适配器解密后的逻辑值，回滚前必须严格 AES 加密后写入 CM；标量、数组和 Vector 保持容器形状并分离副本。加密失败、无效密码单元在 owner/CM 派发前拒绝。元数据标识的历史 STRING 密码只接受可验证密文或空值；默认密码不得利用旧加密失败回退明文。生成新快照使用独立逻辑值，避免 CM 密文在后续快照读取中变成密码本身。

验证：公开 API Password 修正后的旧 Core 负向 8 项中 7 失败，修复后 12 项新增测试通过；完整配置模块 189 项全过并 install。同包 deprecated Password 曾被测试通配导入误选，早期密码负向证据按夹具类型错误保留，不作产品结论；正确类型的重跑才是前后对照。中间 positive-first/full-install-first 报告保留。

实际 IDEA 全量隔离 Kura：PID 23362 / HTTP 18613，283 inventory 项逐一验 SHA，生产 Core 与最终 189 测试产物一致。新唯一命令 convergence-once，实际 ManagedService.updated 回调因回滚被阻塞，公开 SCC 仍为旧 true；Core.rollback 18 ms 返回，5 次 CM 变更尝试。旧视图 readRevision=CONFLICT，PID 不在 UNKNOWN；释放原回调后公开 false、取得新 token，未重发任何 writer。新快照落盘，所有旧快照字节保留。该实例已停止，全部现场保留。stop helper 第一次仅因 native.json 状态标签不是 running 而拒绝，未发信号；核对后修正标签再正常停止。

范围：本轮不是完整 Kura 恢复验收。真实 Console 密码端到端回读/逻辑快照再导入、全部 provider/default/identity/empty/add/remove、持久化 UNKNOWN 与重启恢复、全操作 deadline 等内部 pending 保留；NM/cloud-init/物理硬件、amd64、磁盘满/断电等外部 deferred 不变。未重放旧验收、未提交推送、未触碰原 Orb 服务与个人运行现场。目标 active。

证据：target/rollback-convergence-20261108；机器清单 docs/evidence/rollback-convergence-20261108.json。

下一步：用全新目标独立验证真实 Console 密码逻辑值、CM AES 存储及新快照再次只读解码，不重放 convergence-once。然后继续完整 14 项台账，保留全局服务器时限与生命周期/reentry 边界。
