# 2026-11-08 821a895f SCC 元数据容器修复与真实密码恢复

从 d350b27d 续接，全部父指纹/18仓工作树/原 Orb 四服务与两台已停 Parallels 状态匹配。原有生产/测试/文档可能变化指纹均留原字节 before-copies。

## 生产修复与回归

真实 SCC PASSWORD Vector 在旧 Core 的 getScalarFromObject 中被当成类名 Vector 交给 Scalar.fromValue，整个 SCC 公开配置返回 null。进一步的真实 getComponentConfiguration 回归还证明旧路径对基本类型数组强转 Object[] 抛 ClassCastException；类型不匹配只记日志却仍返回 true，null 数组元素也可蒙混通过。

修复 isMatchingADType：集合与数组逐元素核对声明 Scalar，空集合使用 AD 声明类型，空数组核对 component type，基本类型数组用 Array.get 及 primitive->boxed Scalar 映射；拒绝错误标量/混合或嵌套容器/null 元素。该路径不再日志输出属性值。四个新实际 SCC 入口回归在旧产物上 3 failure+1 error，修复后配置模块193项全过/install。未改公共 API；不声称元数据 cardinality/AD.validate/全局时限已全面完成。

## 真实 Kura 证据

新 IDEA 全量隔离 Kura PID24919/HTTP18620，283 inventory全部验SHA，生产Core与193测试最终产物一致。新唯一命令 containers-final-once 在自有程序化 SCC+ManagedService 上，使用实际 CryptoService 加密 seed 到真实CM，由真实 updated 回调解码并发布 typed Password。六字段：非空/空密码标量、非空+空元素 Password[]、非空+空元素 Vector<Password>、空 Password[] 与空 Vector。公开 Vector 由 SCC 从 CM Collection 显式构造；实际 CM 返回 ArrayList，不能把 CM 具体类保留误认为 Vector 合同。

先真实 callback baseline并snapshot，再真实CM callback改变标量、删去原普通字段并新增obsolete字段，另存新快照；Core.rollback(checkpoint)22ms返回、5次CM变更、总callback3次（baseline/advance/rollback各一次）。全部 CM 密码 AES 解码等于原逻辑值，paired readRevision公开值一致并恢复普通字段、移除obsolete；新快照连续两次解码保持Password数组与Vector及逻辑值；空容器一致；旧快照字节保留，观察无写入重发。收据无密码值。其余fixture错误/生产负向现场见 fixture-classification.json，均保留且四实例已停。

## 分类与范围

PID24294元数据反射addAD参数错误发生注册与CM前；PID24388真实生产Vector缺陷发生seed后回滚前；PID24855修复后真正回滚已返回、CM解码/公开值已过，后续对具体Vector CM类型的夹具断言失败，不代表生产回滚失败也不能重发；PID24919在新目标上分别验证CM集合值与SCC公开Vector后通过。所有原收据和现场保留，无重放。

这不是完整Kura恢复：任意合法密码恰好可被AES解密时的重复解密/逻辑值语义、全部provider与默认值/identity/生命周期、creation/deletion、durable UNKNOWN/restart/reconcile、全服务器时限及其余14项内部pending保留。外部 NM/cloud-init/物理、amd64、磁盘满/断电 deferred不变。不提交推送、不改原Orb/个人运行目录、不清现场。

下一步先用严格逻辑密码（内容恰好是有效AES密文）回归明确 SCC/快照/公开view 的解密边界；若复现，修边界而非内容猜测，再按14项台账继续。不能重放本轮任何已派发命令。

证据 target/rollback-password-containers-20261108；机器清单 docs/evidence/rollback-password-containers-20261108.json。目标active。
