# 1f93dd7c：restricted 与三成员栈实际保存读回

2026-11-08；前序1274e471。此轮为真实路径验证进展，不新增生产代码、不重跑已钉构建/单元测试；目标仍active，整体未完成。

## 场景与事实

新 IDEA 完整隔离 Kura，PID4941，HTTP18596，283项runtime inventory（282实际bundle+探针），启动业务写入0；无agent。原生英文GWT，admin/admin既有夹具登录，没有改密码。只在新私有实例创建stack.ssl和stack.readback；原现场和原Orb服务不变。

1. Security → SSL Configuration 原 restricted API创建独立stack.ssl。吊销检查一直false，hostname verification一直true，SSL protocol TLSv1.2保持；修改的是不生效的吊销模式PREFER_OCSP→PREFER_CRL→CRL_ONLY。两次原GWT同页保存，原Core真实写入/确认各2次，CM count1→2→3，配对实际属性与CM相符，serviceId277不变，uncertainty为空。首个保存补入空cipherSuites；两个changeme目标保持不变，此SSL实例没有业务消费者，不代表TLS握手或Keystore端到端验收。
2. 原生Raw工厂创建stack.readback，三个成员为root/DataService/MqttDataTransport，探针捕获初始配对版本。原GWT DataService同页将connect.retry-interval 60→61→62；两次保存后取得完整三成员视图，仅DataService版本变化，CM count1→2→3，root及transport版本不变。没有整页刷新、重新选择连接或重发写入。
3. 同一栈切换MqttDataTransport，keep-alive 30→31，一次独立保存成功；仅该成员版本与CM count1→2变化，root及DataService版本不变。client-id与lwt.payload由缺省补为空字符串；其余值的整体类型/密码语义仍按完整台账验证，不能据此称全部规范化通过。connect.auto-on-startup始终false，没有点击Connect，没有MQTT投递验收。
4. 使用保存前已捕获的整栈版本调用条件删除，得到typed CONFLICT；完整探针before/after相同，全部CM/服务注册/八份实际快照原字节一致，没有真正删除、清理或补偿。此为保存后旧整栈baseline负向，不是重放旧验收入口。

探针只给restricted目标stack.ssl的条件更新计数，无读门控、无失败注入。云成员写入数由每个明确UI确认、CM change count、快照、逐次原生readConfigurationForEditing版本与原控制台成功读回共同证明；不能把SSL专用计数器称为云全局RPC计数。探针所有Core调用转给原对象；没有在客户端伪造配置/版本。

## 证据

证据根：target/cloud-stack-readback-20261108；清单：docs/evidence/cloud-stack-readback-20261108.json；native-result.json；*.result.json admissions/receipts；restricted-save1/2、cloud-data-save1/2、cloud-transport-save AX与PNG；snapshots-before/after-stale.json。scene-before-stop保留日志、状态观察及各快照观察原字节。

PID4941已停止且现场保留；原Orb4服务package26/20、Parallels两VM stopped和18仓在封口实时核对。生产源/构建产物与1274e471钉定内容保持，GWT207、Core150、helper12、Camel8及login/denali各6排列仅保留前序证据，未在本轮重跑。图谱使用UI/Core真实子仓，六个相关路径coverage metadata_match，仍属best effort，非完整性证明。

## 台账

关闭选定英文restricted同页两次保存、Raw三成员栈DataService同页两次保存和传输成员保存的实际读回子项。仅选定provider及字段，不泛化全部云/SSL/Keystore/Linux/竞态。完整14项pending逐项继承，外部deferred独立原样保留；原26项disposition链保留于前序。

本轮未实测其他标签页dirty保持、旧选择异步回调、成员变更竞态、失败后restricted恢复、所有provider/密码/多值/空字段、生产Linux系统路径。Raw根成员当前metatype没有可编辑属性，未将其“名称修改”记为通过。

源码发现RestrictedComponentServiceUi.updateSelection缺少异步选择代次保护，回调和delete可用性还需按当前selected/factory/dirty/availability统一核对；这只是代码风险发现，未记为已复现或修复。下一步优先构造受控延迟读取/选择竞态并完成实现和实际验证，随后推进完整台账。CloudConnectionConfigurationsUi已有generation与dirty保护但本轮未穷举竞态。历史初始GWT CONFLICT精确持有者与既有证据缺口仍未证实。

不提交推送、不reset/clean、不清理个人runtime、不连接192.168.2.125；UNKNOWN不重发。无全防火墙、全Kura恢复、发行完成声明。
