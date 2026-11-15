# GWT 条件删除与原生资产类型化更新

2026-11-07，64235d8f，续接031c0ed0。完整目标 active，未暂停、未完成。合同 `../configuration-plan-local-v1.md`；证据 `../evidence/gwt-conditional-delete-20261107.json`；原始记录 `target/gwt-conditional-delete-20261107/`。入场18仓完整 HEAD/branch/porcelain 与594 source、3280 artifacts、3566 evidence全部匹配。

## 条件删除与旧写入口

通用 GWT、restricted keystore/SSL 与 driver/asset 新增显式条件删除 RPC，保留原权限、XSRF 和真实服务类型授权。客户端在确认/异步取令牌之前固定已显示 PID 与 revision；不在删除前临时读取新 revision 并绑定旧画面。共享 helper 只派发一次 Core typed deletion plan，由原 monitor 校验实例版本、工厂身份与准入，再按原策略删除和写最终 snapshot。typed conflict/unsupported/invalid 仍为写前拒绝；派发后的其他失败为 UNKNOWN，不补偿、不重放。

旧 PID-only 删除 RPC 和 raw updateProperties 签名保留，但明确 OPERATION_NOT_SUPPORTED，不能再直接调用旧 ConfigurationService 删除或 ConfigurationAdmin.update。旧缓存客户端不会获得 legacy 回退。图谱与源码补查在该 web2 src 范围未见 raw updateProperties 的实际生产客户端调用；这不是跨所有外部客户端的不存在证明，也不表示 cloud stack 等其他生命周期入口已全部条件化。

实际原页面先验证 stale 删除冲突：Count1 显示后另一个真实 Core 写改为2；旧页面删除一次得到中文冲突，原 CM/DS 及最新 snapshot 路径与字节完全不变。重新加载真实Count2后进行新确认删除，CM与新snapshot移除目标。此处是已知写前冲突后的新显示意图，不是 UNKNOWN 重发。

英文受控测试在实际 Core 删除计划调用后抛异常，页面正确显示 UNKNOWN，目标已从 CM 与新snapshot消失。首次发现原 ServicesUi 仍可点击删除，失败截图保留；随后加入当前编辑器 UNKNOWN 挡写状态，Apply/Apply&watch/Reset/Delete 一起禁用，setDirty 不能重新启用写入，直接 apply/delete 也有状态检查。restricted 删除回调同时保留对应编辑器和外层按钮封锁。新v2原英文页面已见全部禁用，真实 DS/服务消失、deactivate一次，无再次请求。后续客户端改动在 driver/asset 重读路径，通用封锁逻辑保持。

8项共享删除/旧入口合同覆盖精确 PID+revision 到一次 Core plan、缺失/错误 baseline、无效 PID、Core各类拒绝、provider不可用、派发后 UNKNOWN 无重发及旧 CM bypass 拒绝。旧实现先得到2项行为失败、0error，红XML保留。新增1项真实客户端错误策略合同核定何时封锁当前编辑器，401/403保留原登录处理。

## 原生资产实际发现后修复

原资产 Apply 使用两个 RPC 先删后建。本轮改用已有类型化条件 update，保留实例；原生 WireAsset.updated(Map) 本来就支持 modified，并委托 BaseAsset.updated，不需要删除重建。图谱对 updated 的重载只返回 WireAdmin 回调，另按确切文件范围读取 Map 重载核对，未据单个图谱结果误判没有 modified。

随后原创建面板真实返回 OPERATION_NOT_SUPPORTED。直接原因是 Wires 的 metatype 文件虽名为 WireAsset，其 OCD/Designate/Object四处却登记成 org.eclipse.kura.asset，而同包 DS 名称是 org.eclipse.kura.wire.WireAsset。新增声明一致性回归在旧XML上失败后，修正四处身份。基础 Asset 与 WireAsset 是两个独立工厂；Core asset provider 增加自身三字段元数据，保留基础工厂创建能力，Wires拥有自己的九字段声明。未改名或迁移旧 CM/快照。

修复身份后实际新建 WireAsset 成功，但第一次通道 Apply 被写前冲突保护拒绝：页面仍拿本地新建模板，没有成对的 Core revision。driver/asset 创建与更新成功后现进行有界只读重读，最多十次200ms间隔等待缺失/未settled视图；只接收精确唯一 PID、合法 revision 与同次实际服务值的完整 DTO，再替换模型/重画表单。不会给旧提交值附一个新token。读取失败不再派发写，不把已经明确成功的写重新称为 UNKNOWN；保留未清理表单并要求重读，认证失败仍进入原登录处理。新增3项生产客户端选择策略合同覆盖实际值与版本成对、拷贝隔离、缺失/未settled/重复PID不能授权。

原页面新驱动创建→第一次修改、资产创建→新增动态 temperature 通道→同页第二次修改 request.timeout=12 均实际成功。每次确认只提交一次，成功后按钮清除dirty；实际 CM从1→2→3，service.pid/component.id/service.id保持，真实原生资产通过测试Driver读到 Integer41。测试Driver只提供本轮确定性业务输入，不代替物理设备验收。

核对 CM 又发现原生 BaseAsset.getConfiguration 把 DS context 的注册属性当作配置返回，Core保留自配置业务视图时因此将 component.id/name、DS satisfying-condition 与静态 UI 注册标记写进CM和快照。修复落在原生 provider：返回独立业务属性视图，排除明确的注册属性，保留稳定 CM/工厂/Kura身份、业务字段和通道专属同名键；内部 DS map 不修改。旧坏现场和旧快照原样保留。新增实际方法回归先失败后通过；最终两类原生资产的更新后CM及磁盘snapshot均没有这些注册字段，真实通道Integer/Boolean类型保留。

基础 Asset 页面还实际显示 %requestTimeout/%requestTimeoutDesc。BaseAssetOCD 绑定本 bundle 的真实en/zh资源，提供非OSGi英语回退，基础组件名称修正为 Asset，WireAsset继续其独立名称。原生资源合同与原英文/中文控制台均验证 Request timeout/请求超时和对应说明，显示实际12；没有用客户端替换字符串伪装服务端元数据。旧上游测试对基础资产误称Wire Asset的断言随明确名称合同更新，字段数量、业务与原断言其余部分保留。

## 测试与失败分层

最终 GWT168、Core asset35、Wires component84通过。Core configuration130与Wire provider21本轮未改，沿用未重跑。生产GWT login/denali各6 permutations及服务器JAR编译、打包安装通过；新增客户端策略直接编译真实生产client源。最后一次全reactor在生产模块成功后因新测试误导入嵌套枚举而于testCompile失败；纠正后单独测试模块168及安装成功，未把测试import错误算产品红，也未为未变化生产源再次编译。两份错误import日志保留。

Wire工厂身份与原生注册属性有各自有效红XML。资源合同首轮修复后仍失败，一项因测试用file URL不是ComponentUtil要求的bundle-root路径，一项为旧基础名称断言；改为bundle形状URL读取实际资源字节，并按新基础名称合同调整断言，最终35通过。原生缺标签截图是独立实际红证据，不拿这个夹具问题充当新的产品缺陷。

原生进程均通过IDEA原Run启动完整283 inventory/282 OSGi bundles，原admin/admin登录，未装agent；首次密码闸门绕过仅在私有测试profile明确记录，生产安全默认未改。每套运行集均核对全部JAR哈希，使用新UUID：

- 首套PID82463：中文stale拒绝/当前版本正常删除、英文真实删除后UNKNOWN；发现写按钮未封锁，停止保留。
- v2 PID82994：最终UNKNOWN封锁通过；原WireAsset创建被错误metadata身份拒绝，CM不存在，停止保留。
- v3 PID83558：修复metadata后WireAsset创建、空通道读取通过；第一次编辑缺baseline被拒绝、CM仍1，停止保留。
- v4 PID84835：原页面两次同实例保存与41业务通过；CM/snapshot出现注册属性的红现场，停止保留。
- v5 PID85774：原新驱动创建/修改、WireAsset原创建及两次同页保存、基础Asset原页面更新、41业务、干净CM/snapshot通过；保留基础标签缺口截图，停止保留。
- 最终v6 PID86707：同版两类native factory条件helper创建/更新、同实例/干净CM/snapshot与41、原界面en/zh基础字段显示通过；停止保留。

v5额外探针对基础Asset更新选用了包含non-metatype字段的完整读取接口。该接口当前参数没有type，转换会在派发前失败；实际ILLEGAL_ARGUMENT、CM仍1。随后先检查原实例，用原页面的成对类型化视图完成新的明确修改，最终CM2；没有重发UNKNOWN。v6与Linux探针改用实际driver/asset页面对应的filtered paired读取。**完整non-metatype视图的标量/容器/密码类型转换仍是产品内部pending；没有假称其编辑能力完成，也没有raw CM回退。**

辅助归档遇到一次空字符串XML无子元素，以及观察者写state-latest时的一次JSON读取竞争；读取修正后仍采集同一个原进程/原回执，未启动第二次业务请求或重开进程。所有派发与结果以独立admitted文件和原receipt核对。

## OrbStack 与现有 Parallels

最终v6全部283项产品/探针制品原样复制至新Orb PID753249 `/opt/yofc-gwt-conditional-delete-orb-4100cacb` 与现有Parallels PID2822 `/opt/yofc-gwt-conditional-delete-parallels-ee37139e`。每处16个唯一命令：真实创建/显示revision/另一写入、旧入口拒绝与stale删除拒绝、重新读取当前版本后正常删除；两类原生资产带真实GUI固定的通道类型值创建和条件修改、同一CM/DS/service及41业务；独立目标Core删除后受控UNKNOWN、无请求重放。拒绝前后原snapshot路径/hash与CM/DS完全一致；两类资产snapshot保留Integer12、unit.id Integer5与enabled Booleantrue，注册属性缺席；正常/UNKNOWN删除的新snapshot无目标，实际服务/DS消失且deactivate各一次。

Linux是共享GWT helper→真实Core/CM/DS/磁盘及native Asset服务验证，没有浏览器RPC、agent/中心传输或产品网络/系统控制任务。仅编排本轮新fixture unit。所有283 inventory hash一致；Parallels另核184私有JRE文件，实际strict/CapEff0/NoNewPrivs1，外部写EROFS30；Orb沿用既有全局覆盖的ProtectSystem=no/NNP=no，不算严格沙箱。nft hash及IPv4/IPv6 forwarding各自入出场一致。新unit全部MainPID0/inactive，Ubuntu随后正常停止、Windows保持停止；原Orb四服务原PID244516/244911/286620/268、console26/agent20原包保持active，未重启。

## 适用面与继续台账

最终冻结运行集离线声明清单为72项factory Designate：70生产/2探针；70生产PID唯一，12延迟DS，0同bundle未匹配DS，XML解析错误0。旧69项中的一个错误归属修正，同时为基础Asset补独立声明。这只证明静态声明匹配，不是实时所有SCR或动态provider/全业务健康证明。延迟原生业务仍仅先前H2所选证明，本轮Base/WireAsset为immediate，不能替代其余延迟provider验证。

下一内部pending包括完整non-metatype视图类型转换、其余driver/asset/restricted删除与依赖/dirty-form/并发域约束、CSV/import/clone/上传/专用通道/所有native metatype与实际端口、cloud stack等其他生命周期入口；创建/删除持久请求回执、丢回复核定与进程重开、真正部分工厂派发及snapshot失败完整恢复，均未闭环。受控删除已实际发生的UNKNOWN不等于持久核定或部分失败恢复。任意provider的硬RPC期限仍未建立。

生产/外部PID、空/部分/不一致CM恢复，NM/NAT其他writer正式归属与安装profile，加密中心防火墙/中英远程与受影响新管理链路，confirmed-only实际早启/Core完整安装升级交接，一致DB/制品/key备份恢复轮换迁移/正式发行，继续保留原完整台账。fullFirewall/fullKuraRecovery/fullComponentCoverage均false，正式新deb未产出安装。

external deferred单列：真实NM/cloud-init/物理有线与其他硬件、amd64、disk-full/hard-power；Java已可用。初始历史CONFLICT确切锁持有者未证实、旧3份Surefire XML原文缺失和两旧evidence gap分别保留，不伪称补齐。旧manifest已固定证据字节保持，旧可变源码/JAR/报告先归档，新增失败与成功现场全部保留；不提交推送/reset/clean，不连192.168.2.125，不清理旧运行集/密钥/状态/卷，不重放旧一次性验收，UNKNOWN不重发。知识库和最终18仓/制品/运行状态核对见manifest。
