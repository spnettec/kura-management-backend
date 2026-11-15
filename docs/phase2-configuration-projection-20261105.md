# 配置适用性续接：基准合同与本地化展示

日期：2026-11-05。恢复整体目标，未提交或推送。六个关键仓库 HEAD 与质量重构交接一致；18 仓状态已记录，原 dirty 保留。

## 本次实施

Java bridge 的字段集合改为基准 AD、本地化 AD、当前属性三者并集。本地化缺项不隐藏基准字段；本地化独有字段没有权威合同，UNKNOWN/只读。类型、cardinality、required、约束和脱敏以基准 AD 为准，任一基准/本地化 PASSWORD 均脱敏。合同发生变化时字段只读；一致时使用本地化选项标签，选项值和界限仍来自基准。默认值、敏感选项不导出。

新增可选 readOnlyReason：NO_METATYPE、LOCALIZATION_MISMATCH、SECRET_OPTIONS、UNSUPPORTED_CONTRACT。OpenAPI、生成类型、React 模型与中英字段适用性列同步；理由与 editable=true 矛盾或未知理由被前端拒绝。字段支持仍不意味着已获远程写授权，审核 PID 与默认关闭保持。

## 验证范围

- bridge 宿主 37/37，含4项新增本地化/脱敏/合同用例；OrbStack Java21 同一受影响套件37/37。
- React 类型/生产构建通过；受影响配置浏览器7/7，含新增只读原因合同和中英实际展示。
- 新隔离完整282项运行集通过 IDEA UI 启动，原 GWT admin 登录成功；HTTPS→当前中心/agent→UDS→真实Kura，当前已列出的25个PID中英50次读取全部成功，含CONFIGURABLE与SELF_CONFIGURING。所有配置/快照/防火墙写关闭，无业务写入。
- 扩展默认浏览器在纯静态preview上25通过/6失败：6项依赖既有开发中心登录/网关；未启动旧夹具，不将其当当前通过。首轮受影响浏览器因未启动preview而5连接拒绝，启动独立preview后7通过；首轮Java1项Mockito嵌套stubbing错误已修正。失败记录保留。
- Parallels现有VM启动只读核查，没有Java运行时，本次Java验证未执行；不冒充实际Java或完整系统沙箱验收。未安装软件或替换原服务，随后正常关机。

25个已实例化PID不等于所有可能factory实例或完整组件业务适用面；删除/reset/factory管理及其他未交付业务继续pending。本增量不改变任何恢复业务核定。

## 状态与新增需求

新夹具 `.phase1-dev/configuration-applicability-20261105-4e5550de` 停服保留，网关disabled，独立PG停止；全部旧夹具和个人runtime保留。Orb默认中心/center2/agent保持active原包26/20。HEAD未变，未reset/clean或提交推送。

用户新增原GWT配置agent证书、console认证与是否启用远程管理的需求，已纳入实施；当前仍pending，详见 `local-agent-provisioning-v1.md`。整体goal继续active，不沿用上轮暂停。
