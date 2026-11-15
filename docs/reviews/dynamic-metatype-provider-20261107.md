# 动态 MetaTypeProvider 生命周期及配置校验核定

核定：`239a42f7`，2026-11-07。接续 `bde06c22`；完整管理体系目标继续 **active**。本轮完成选定的软件适用面增量，尚未交付完整组件适用面、恢复运维或正式发行。

## 行为及实现

Core 现在跟踪显式 `MetaTypeProvider`，按 `metatype.pid` / `metatype.factory.pid` 发现普通及工厂元数据；保留无 XML 时的旧式 ManagedService/ManagedServiceFactory provider 兼容。显式服务优先，使用 service ranking / service ID 确定同 bundle 的来源；跨 bundle 同 PID 有歧义时关闭元数据写入，不任意选择。XML 通过真实资源解析，避免依赖 MetaTypeService 的事件缓存顺序。参考 [OSGi Meta Type Service 8.1](https://docs.osgi.org/specification/osgi.cmpn/8.1.0/service.metatype.html)。

服务修改、撤销及 bundle 撤销立即使旧来源失效，异步合并同 bundle 的重建任务。每个新来源有独立 generation，纳入 revision token；旧 token 不能穿过来源变更。来源锁覆盖已经准入的校验及写入，撤销等待该次写入结束；来源失效后的请求拒绝。Core 锁与来源锁顺序固定，读取 provider 定义和本地化不在反向锁顺序下调用任意 provider 回调。停止 Core 时不再种植默认配置。

所有已接入的 Core 普通更新、批量更新、工厂创建及内部 CM 写入路径都保留 provider 的原生 `AttributeDefinition.validate`，包括 STRING 和 PASSWORD。先合并实际值、默认值及本次 patch，再校验类型、基数、必填及选项和 provider 约束。非法值在烧录 revision admission 和 CM 更新前拒绝，错误不回显 provider 的私有文本；已有 legacy/self/external owner 校验保持原边界。

Provider 默认值按基数符号生成标量、数组或集合，保留空值和空格；无效类型、选项或自定义校验失败的默认值不种植。PASSWORD 默认值不出现在对外 AD 元数据中，写入 CM 前加密。刷新元数据仅补缺失默认值，不覆盖已有值、不产生无差异 CM 更新；`RECOVERY_REQUIRED` 时只登记读取元数据，不新增 CM 或补种默认值。

密码属性分类会记住已观察到的元数据；撤销 provider、撤销全部定义或切换类型时，原有任意命名的 PASSWORD 集合仍按密码读取和脱敏，不能因元数据消失变成普通字符串。定义撤销不删除 CM、业务实例、备份或现场。显式 provider 撤销后，有合法 XML 的 PID 回到 XML；没有其他来源的 PID 关闭写入。重新注册不重复写入已有 CM。原 GWT 使用 provider 的本地化名称及标签，结构、约束和密码默认值仍以当前来源为准。

实现集中在核心配置仓库的 `ProviderDefinition`、`MetadataDefinition`、`ComponentMetaTypeBundleTracker`、`ConfigurationServiceImpl`、`ConfigurationRevision`、`ComponentConfigurationImpl` 和 `ComponentUtil`，未新增公共 API 接口，未改变 Rust agent、中心或 React 产品代码。配置默认关闭、审核 PID 显式开放；本轮只在独立夹具开放 `dynamic.business`。

## 最终验证

最终冻结源码的测试：Core/API/util/crypto/XML/inventory/离线工具依赖 reactor **464**（其中 Core configuration **106**）、bridge **55**、GWT **87**，共 **606**，零失败、错误或跳过。另有 6 项未改动的基础测试夹具在前一次完整 reactor 已通过，未重复计入 606。新增测试包括真实嵌入式 Equinox 注册表的注册/修改 PID/工厂角色/排序/撤销回退、旧来源失效、来源撤销与已准入写入并发、默认选项、密码默认值、Core admission 未消耗、CM 刷新保值、恢复闸门和撤销后的密码分类。嵌入式单元框架不代替完整 Kura 验证。

最终制品依次经过 IDEA UI 完整隔离 Kura → OrbStack → 现有 Parallels Ubuntu 验证。每处都是新 home / CM / fixture，283 项 inventory、实际 282 个 bundle，无新 agent 或中心；复用已有静态运行包及经过校验的 Parallels 私有 JRE，仅复制到新的运行目录。

隔离夹具沿用专用 home 中的 `kura.need.password.change=false`，用于直接进入配置业务验证。生产默认账号、首次改密后台闸门及产品源码未因此改变；本轮不是首登改密流程的新增验收。

每处实际注册 provider 的两个工厂，真实 CM / DS 核对默认值、两项密码集合（含空密码和首尾空格），直接调用实际注册 bridge 的 frame 合同进行版本化更新；浏览器用 admin/admin 登录原 GWT，中文显示“动态管理业务”，仅保存一次 script。随后在同一个 JVM 修改 provider、撤销并回退 XML、撤销全部元数据的另一 PID，再重新注册。

每处 DS 事件均为 `1|allowed-v1|before-form` → `2|allowed-v1|before-form` → `2|allowed-v1|gwt-verified` → `3|allowed-v2|gwt-verified`。两次自定义 STRING 非法更新在写入前拒绝，且不消耗 token；两次旧来源 token 更新拒绝。各次元数据修改/撤销/重注册都不额外修改 CM change count 或 DS 事件；密码持续保型及脱敏，无明文回显，撤销全部定义后的 PID 不可写。初始 bridge 名称为英文，GWT 为中文。每处的 ready / GWT / lifecycle 三份 create-only proof 全部通过。

这是普通配置、真实 DS 和元数据生命周期验证；bridge 采用直接 frame 调用，**没有冒称 agent / UDS / mTLS / 中心远程任务或受控恢复链路**。`fullRecoveryVerified=false`、`kuraRecoveryVerified=false`、`normalBootVerified=false`，没有新增恢复准入、health/completion 或释放恢复闸门。恢复闸门的默认种植边界只有本轮单元证据。

## 环境及现场

| 环境 | 新现场 | 原 PID | 收尾 |
|---|---|---:|---|
| IDEA 最终 | `.phase1-dev/provider-idea-v2-20261107-aebe49fe` | 70453 | JVM 已停止，289 静态文件 pins |
| OrbStack | `/opt/yofc-provider-orb-20261107-8d613271` | 707196 | `yofc-provider-8d613271.service` inactive / MainPID 0，290 pins |
| Parallels | `/opt/yofc-provider-parallels-20261107-4c130f1b` | 2595 | `yofc-provider-4c130f1b.service` inactive / MainPID 0，474 pins，含私有 JRE 184 文件 |

第一套 IDEA `.phase1-dev/provider-idea-20261107-434f6bd5`（PID 69698）也已通过生命周期并停止。随后源码复核补充默认选项约束，故重新构建并新建最终 v2 夹具；第一套证据属于补充边界前版本，不能充当最终制品验收。没有重放任何旧夹具入口或旧 GWT 保存。初期编译/测试的失败日志保留，最终测试已通过。

OrbStack IPv4 `192.168.139.109` 从宿主错误地路由到物理网关，浏览器访问超时；实际 VM 和 JVM 正常。通过已有 IPv6 地址 `fd07:b51a:cc66:0:e84a:f8ff:fe9d:9c62` 完成原 GWT 验证，未改宿主/VM 网络路由、未重启 VM/JVM、未重发写。该环境观察另存 `orb-routing.json`，不充当 NM/Netplan 冲突或防火墙验收。

原 Orb 四服务仍为原包及原 PID **244516 / 244911 / 286620 / 268**，保持 active；本轮没有启动新 PostgreSQL、console、agent 或防休眠进程。Parallels Ubuntu 正常关闭，Windows 保持 stopped。18 仓 HEAD/branch 不变，所有未提交工作保留；无提交、推送、reset、clean、远端覆盖，不连接 `192.168.2.125`，不删除旧 state、密钥、备份、quarantine、卷或运行目录。

## 后续边界

动态 provider 的本轮注册/修改/撤销/回退/校验范围已闭环，不再列为未实现。完整组件适用面仍有内部 pending：历史 0/±1 标量兼容、其他类型和元数据边界（含图标），GWT 新增/删除/空元素等完整编辑语义；新元数据范围的受控恢复实际业务、生产组件、部分/空 CM 和外部 PID 恢复；其他未决恢复运维、Core 安装升级/bootstrap/启动策略交接与旧入口退役；NM/firewall/NAT/forward/远程流程/早启；一致 DB/制品/密钥备份恢复、轮换、迁移及正式发行。

外部 deferred 仍仅真实 NM/cloud-init/物理网络硬件、amd64、磁盘满/硬断电；两条历史 evidence gap 单列。Parallels Java 已可用，不把未完成软件工作记作环境缺失。保留 NM、Kura 无 agent 可独立工作、Java 独立生成 key/CSR、agent 常驻及可选远程通道、admin/admin 与后台首登改密闸门、中英、独立归属、UNKNOWN 不自动重发和 RECOVERY_REQUIRED 挡写保现场等核定均不变。

机读证据：`docs/evidence/dynamic-metatype-provider-20261107.json`。原始证据：`target/metatype-provider-20261107/`，包括前像、测试日志、四个新运行描述符、真实 CM/DS/typed/bridge 回执、密码撤销视图、pins、路由观察及 GWT 截图。知识库交接与 Tasks / Session Summaries / Knowledge Index 使用工作空间官方 Obsidian CLI 写入并完整读回核对。
