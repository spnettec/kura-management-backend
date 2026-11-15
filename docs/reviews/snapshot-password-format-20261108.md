# 2026-11-08 9ee0ebed 明确快照密码表示与来源边界

从101967da续接：全部父指纹/18仓/原Orb四服务与两台已停Parallels实时匹配。268个原指纹动前复制；XmlComponentConfigurations与CloudConfigurationHandler原本没有独立source pin，二者before记录由精确反向本轮编辑重建（此前已全文图谱读取），单独标注before-unpinned-source-reconstruction.json，不冒称动前文件复制。其余原证据不变。

## 实施结果

新快照根增加 passwordEncoding="BASE64_LOGICAL_V1"。XmlComponentConfigurations记录编码，XML provider的stream/DOM写入与DOM读取均传递该版本；标记模式中PASSWORD仅Base64 decode逻辑值，不再AES试探；未知显式版本在codec拒绝。无标记格式继续原适配器/公共getSnapshot兼容路径，并保留系统既有损坏快照回退策略；不声称未知版本完整启动策略已另行验证。

存储前区分来源：CM读值沿旧CM解码边界归一化一次，SCC/外部confirmed逻辑值不解；decoded DTO有显式logical来源，provider不在场仍保留；普通更新快照仅解已由本操作加密的补丁字段，保持SCC其余逻辑值。已知加密补丁解码失败不再被伪装为逻辑文本，文件生成前抛错。rollback已准备的逻辑快照显式保留来源；writeSnapshot内部副本保留根版本。明文快照转外层加密时，标记逻辑密码不再额外加密，原无标记规则保留。

getSnapshot和云快照响应对标记结果跳过二次解密；公开getComponentConfiguration按SCC/confirmed/decoded来源返回逻辑密码，普通CM读仍解一次。稳定属性Map SCC的独立负向回归先复现1失败，最终修复；此前fresh-getter夹具掩盖的公开路径已用最终stable-map实际夹具验证。启动从标记快照推CM前，对逻辑密码重新严格加密；singleton与persisted factory以及加密失败零CM交互都有真实Core入口单测。未改org.eclipse.kura.api；Core导出DTO增加表示元数据与adapter构造重载，必须与新XML provider协调安装，单独替换Core/旧读者降级兼容不在已证实范围。

## 验证

配置模块最终210项全过/install；XML provider8项全过/install，含stream/DOM标记、Base64逻辑值无AES试探、legacy路径、未知显式编码拒绝。保存了198/201/205/206/208等中间报告及public SCC负向8项中1失败，未将中间产物算最终验收。

最终IDEA全量Kura PID27908/HTTP18625，283 inventory与最终Core210/XML8字节匹配。新logical-final-once公布实际AES密文样式的逻辑Password，getter返回稳定map，公开读取、原XML解码和公开快照均保留原值，根版本V1。随后新目标final-containers-once证明6字段非空/空标量、Password[]、Vector、空容器在真实CM AES/paired/新快照重复解码及字段恢复删除均一致；25ms回滚、6次CM变更、3次callback，无writer重发，旧快照字节保留。CM ArrayList与SCC公开Vector分别验证。

最终启动验证PID27990/HTTP18626：从已停27908复制user材料与冻结快照，不复制CM存储（prepare时断言不存在CM目录），全新runtime用最终Core210/XML8启动。目标SCC未注册，实际CM恰有一个由启动路径生成的配置；CM密文经实际AES解回快照里的“本身为有效密文”的原逻辑密码，标量/数组/Vector均相等，公共快照也相等。final-bootstrap-read-once仅只读验证、无验证业务CM写入或rollback。启动源快照SHA与原冻结输入一致。

中间206产物也有独立正向实例PID27088/27178，保留为中间证据；最终结论使用27908/27990。四实例都停、现场保留。六个本轮新命令均通过；旧一次性验收和UNKNOWN未重发。没有提交推送、没有改原Orb/个人runtime。

## 保留范围和下一步

旧无标记Base64逻辑密码与CM密文可以同字节，V1没有自动消除旧材料歧义，旧SCC/CM导入和迁移/降级/混合版本安装仍是内部pending。SCC逻辑来源约定下的全部真实provider合同审计仍pending；CM历史非空明文/坏密文保留旧fallback，不宣称全部旧材料恢复正确。实际选定标量数组Vector及新标记启动并不是全Kura恢复，更不是持久UNKNOWN/restart/reconcile、任意factory/empty/add/remove/identity、全服务器时限、全部业务副作用证明。

完整14项内部pending继续，外部NM/cloud-init/物理硬件、amd64、磁盘满/断电deferred不变。下一优先核对旧SCC Password表示与全部真实provider（图谱按子仓库），建立明确legacy/source兼容合同与成对bundle安装边界；随后推进durable UNKNOWN与全服务器时限等完整台账，不重放本轮命令。新逻辑格式所选读取/回滚/启动子项已闭环，目标仍active。

证据target/snapshot-password-format-20261108；机器清单docs/evidence/snapshot-password-format-20261108.json。
