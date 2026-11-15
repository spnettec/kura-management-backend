# 管理体系交接后全量审查 — 2026-11-04

结论：**不通过“整体目标已完成”的核定。** 后续会话交付了多项有效增量，但当前源码仍有任务状态、配置校验、恢复补偿、包维护和部署入口缺陷。防火墙和离线恢复目前是受限本机原型；完整方案中的远程产品流程和若干核心实现仍未交付。这些实现缺口不能归为“缺环境 deferred”。

本次只审查、复现和记录，没有修改业务实现、提交或推送。既有成功记录按原日期、版本和环境保留，不因发现其他缺陷而抹去。

## 审查起点与实际版本

起点是本会话的 `Experience/2026-11-01-Kura管理体系本地保存点与配置适用性-47efca29.md`、`Experience/2026-11-01-Kura管理体系快照保存回滚接线-de6eed61.md`，并全文读取原暂停交接 `6b48b7d2` 的全部续接、关联架构决策、Tasks 和中心包交接。仓库交接核对到 `management-redesign-handoff-20261102.md` 的 2026-11-04 补记。

| 仓库 | 保存点 | 本次实际审查终点 |
| --- | --- | --- |
| kura | `e2c616cb44` | `dd073cb077`，工作树 clean |
| kura-networking | `c362c2e` | `81534f10be`，工作树 clean |
| kura-management-console | `eaa8b68` | `5f5b1ab6af` 加全部现有未提交及未跟踪源码，ahead 28 |
| kura-management-agent | `c8e41e4` | `0b99202903` 加全部现有未提交及未跟踪源码，ahead 17 |
| kura-container / kura-management-ui | `000835d` / `573a2c4` | 保存点后无新增业务差异 |

四个真实子仓库的 codebase-memory 已刷新；查过路径覆盖，SQL、shell 和证据 JSON 的解析缺口直接核对原文。没有索引父工作空间，没有使用子代理。

## 需要修复的发现

### F01 · P1 · 同代次重连误终结正常任务

位置：`crates/console/src/grpc.rs:92`。

握手时的 UPDATE 使用 `binding_generation=$2`，会把**当前代次**的 QUEUED/RUNNING 全部改成 EXPIRED/BINDING_GENERATION_CHANGED，旧代次队列反而留着。普通断线重连即可触发，已经交付但丢回执的写任务也会失去 UNKNOWN/原回执核定路径；`apply_result` 只接收 RUNNING/UNKNOWN。

以当前原 SQL 对一次性内存表复现：generation 4 的 queued/running 都变 EXPIRED，generation 3 的 queued 不变。该复现验证 SQL 条件，没有伪称重跑真实 gRPC。需要分别覆盖同代次重连、真实代次改变、已交付写任务和 UNKNOWN，不应仅机械改一个比较符后结束。

### F02 · P1 · 字符串 options 被当作长度约束

位置：`kura/kura/org.eclipse.kura.api/src/main/java/org/eclipse/kura/configuration/ConfigurationConstraints.java:80`、`:159`。

选项比较复用了长度比较函数。合法 STRING option `safe` 在校验值 `safe` 时抛 NumberFormatException；option 为字符串 `"4"` 时，非法值 `"abcd"` 却得到 SATISFIED。已编译当前源文件并通过独立 Java 探针复现。普通字符串枚举是本轮元数据约束交付的一部分，必须按值匹配；现有测试只覆盖整数 options 与字符串长度。

### F03 · P1 · 恢复补偿未停住服务，并会删除无法归属的文件

位置：agent `src/recovery.rs:399`、`:419`。

STARTING/VERIFYING 中断后，补偿先改文件，直到最后才查询 unit 是否 active，从不先 stop/确认独占。即使前面因目标哈希漂移而保留候选，后续“备份清单外文件”循环仍会把它删掉，也会删除事务之外新增的快照。

独立临时目录和 fake systemctl 复现：unit 始终 active；漂移的 snapshot_2 与无关 snapshot_77 同时被删除。应先建立独占，漂移/未知文件进入人工恢复，不能无归属清除。备份恢复后的重启错误也不能直接忽略。

### F04 · P1 · agent 包维护漏掉防火墙与离线恢复

位置：agent `packaging/setup.py:902`、`:958`。

guard_writer 只查网络事务和远程 executions。新增 firewall/recovery 的本机事务没有写入该 executions 表，也未被单独检查。已用两个一次性 Setup root 复现：firewall WAIT_CONFIRM 和 recovery COMMITTING 均通过 guard，随后 purge 删除整个日志目录；确认防火墙状态也没有交接门槛。

升级、卸载、purge、身份/证书转移必须检查这些新子系统；有效规则要先交接，恢复日志和备份不能在未决状态下丢失。

### F05 · P1 · 防火墙状态落盘顺序可把失败回退写成终态

位置：agent `src/firewall.rs:450`、`:456`。

rollback_inner 在执行 nft 和回读前先写 ROLLED_BACK。nft 失败或此处崩溃后，recover/expire 都不会再补偿，候选规则可能一直有效。confirm 又先写 confirmed.json、后写 COMMITTED；若在两者间中断，启动恢复会撤回已经持久化的确认决定，却保留新的 confirmed revision。

后一窗口已用当前模块的 fake nft 复现：revision=1、confirmedManagedRules=1，但 appliedManagedRules=0，transaction=ROLLED_BACK。应像已有网络事务一样记录可恢复的决定和进行中阶段，执行/回读完成后才落终态。

### F06 · P1 · RECOVERY_REQUIRED 可以被下一次准备覆盖

位置：agent `src/recovery.rs:27`、`:187`。

RECOVERY_REQUIRED 被列为允许下一次 prepare 的终态。独立探针确认，新 VALIDATED 会直接覆盖尚需人工处理的 transaction.json；此前恢复信息失去当前索引。应持续挡写，直到有明确的人工核定/恢复流程。

### F07 · P1 · 恢复文件没有保留属主/权限，持久化助手也缺 fsync

位置：agent `src/recovery.rs:264`、`:347`、`:429`、`:490`；防火墙持久化另见 `src/firewall.rs:284`。

备份清单记录 mode，却在恢复时只重新写字节；没有应用 mode/uid/gid。探针中原应为 0640 的文件恢复成 0644。在真实 root agent 的 UMask=0077 下，新候选会成为 root:0600，低权限 Kura 无法读取。已验证的 sleep probe 不会读取快照，不能覆盖这个边界。

两个新模块的 journal/atomic helper 都是 write+rename，没有文件与目录 fsync；backup manifest/receipt 也未形成可恢复的持久化顺序。原子改名不能支持文档中的 durable 声明。硬断电环境仍 deferred，但缺少必要持久化实现本身必须先补齐。

### F08 · P1 · 防火墙源前缀可注入 nft 脚本

位置：agent `src/firewall.rs:118`、`:132`。

source_v4 只检查含点且不含冒号，source_v6 只检查含冒号，随后直接插入完整 nft 文本。换行和闭合括号可退出自有表，再加入 `flush ruleset` 和重新声明的表；nft -c 只是检查完整脚本语法，不是字段授权。

当前源码探针确认恶意 source_v4 被接受并写入含全局 flush 的 candidate_text；仅使用 fake nft，未执行任何实际 flush。必须用真正的 IP/CIDR 解析并规范化输出，不能靠字符包含判断。另需补真实 UFW/iptables-nft/未知表的写入者识别，当前只匹配少数表名子串。

### F09 · P2 · firewall UDS 流程拿不到确认令牌

位置：agent `src/firewall.rs:408`、`:429`。

prepare/apply/status 的返回值都不包含 token，confirm/rollback 却要求它。现有单测/运行脚本直接读取私有 transaction.json 绕过了协议缺口。探针已确认全部正常响应均无令牌。应只向获授权的事务发起方交付令牌，并测试完全通过 UDS 的闭环。

### F10 · P1 · 文档描述的干净首装无法经过 preinst

位置：console `packaging/lifecycle.py:8`、`packaging/setup.py:146`。

preinst 无条件调用 guard_all_instances，后者立即读取尚未生成的 console.env。一次性空 Setup root 复现 FileNotFoundError。历史成功安装预先写过 env，证明的是已预配置安装；新手册则写成先 dpkg 再初始化，不能支持其干净首装声明。

要区分确无旧状态的首装与必须保留 fail-closed 的已有状态维护。另需检查未加载到 systemd list-units 中、但保留了配置/未决任务的命名实例，不能只枚举当前 loaded unit。

### F11 · P2 · 命名实例 activate 启动默认实例

位置：console `packaging/setup.py:225`。

虽然构造器保存了 self.unit，activate 仍对全局 UNIT 执行 enable/start。Spy 复现：`--instance review-center` 的目标是 @review-center，实际调用却是默认 service。既有运行证据用手工 systemctl 接管，未覆盖这个命令。应改为所选实例并加调用目标回归。

### F12 · P2 · 当前 Docker TLS 生成入口有语法和路径错误

位置：console `docker/tls-gen.sh:24`。

`sh -n` 返回 2：多余的字符串和右括号使脚本无法解析；该行还引用 client.ext，而写出的文件是 tls/client.ext。快速开始和新增网关证书命令均受影响。现有镜像/旧证书存在，不证明当前生成脚本可用。应在全新临时目录验证完整 CA/server/client 生成，不碰现有 PKI。

### F13 · P2 · 网络业务核定被记录成配置核定审计

位置：console `crates/console/src/grpc.rs:340`、`:387`。

data 在 seal 后被同名密文信封遮蔽，后面再查 data.reconciliation.source 永远取不到明文来源。因此 proved=true 的网络核定会落到 task.reconcile.configuration。应在加密前固定审计类别，测试真实存储的审计行，而不只测试 proof 形状。agent SQLite 的 source 修复不能代替中心此处修复。

### F14 · P2 · init-instance 无法创建全新的实例目录

位置：console `packaging/setup.py:170`。

mkdir 新目录产生 root:root，紧接着要求 group 等于 yofc-console，却没有设置该 group。已在既有 OrbStack 中仅用临时 Setup root 调用安装 payload 复现：创建 gid0，要求 gid983，立即失败。原测试预先建目录并设属组，掩盖了初始化入口本身的缺口。

## 台账逐项核定

“保留已有证据”指原版本和原范围可追溯，并不声称本次重新执行对应系统写入。

| 交接后任务 | 审查核定 | 关键边界/发现 |
| --- | --- | --- |
| 本地保存点、历史和远端保留 | 保留已有结果 | 六仓保存点存在；本轮 HEAD 未动，未推送；新 dirty 尚未保存为提交 |
| 元数据 min/max/options | 需修复 | 数值/长度有回归通过；字符串 options 被 F02 否定 |
| SelfConfiguringComponent 适用性 | 受限实现保留 | 当前视图合并、默认关闭/审核 PID、directConfigAdminCas=false；不能扩大为通用编辑器 |
| snapshot save / rollback / snapshot 0 修复 | 保留受限闭环 | 真实 IDEA/Linux 历史正常路径和新单测支持；不等于离线恢复，未覆盖所有并发/断电窗口 |
| 配置 UNKNOWN 原提交记录核定 | 保留受限闭环 | IDEA 真回执损失记录、无重发、0600/outcome 证据存在；Linux 断流核定仍非已验 |
| resolution source 修复及双语展示 | 部分成立 | agent source 修复与浏览器通过；中心网络审计另有 F13 |
| task key 单事务轮换 | 保留已验证范围 | 锁表/事务、旧 key 校验有实现；当前仅允许 version1→2，不能称持续多次轮换已完成 |
| task key Argon2id/AES-GCM 备份恢复 | 保留已验证范围 | 篡改/错误口令/覆盖防护单测通过；整库+制品一致备份与整机恢复仍开放 |
| 正式中心包 source/build/安装 | 部分成立 | 24.04 arm64 预配置 fixture 有证据；干净首装 F10，完整 hardening 未验 |
| 中心 QUEUED/RUNNING/UNKNOWN 升级 guard | 部分成立 | 默认实例受控任务实测保留；命名实例覆盖不完整，见 F10 |
| 包 payload 中英文浏览器 | 保留 focused smoke | 原 1 条真实 smoke 的截图/JSON保留；未代替全部配置读取与写入集合 |
| 中心 remove/reinstall/purge/key 恢复 | 保留历史受限闭环 | 当时 24.04 arm64 fixture 有文件/数据库对照；不是当前所有新子系统的生命周期验收 |
| agent 新协议打包与普通登记不抢占 | 基础成立，任务隔离有回归 | 身份导入/代次检查与拒绝换中心保留；F01 阻止完成核定 |
| gateway-local rebind / signed transfer | 保留受限闭环 | 旧代次/身份精确绑定、停服、备份、签名检查；新 firewall/recovery guard 漏接，见 F04 |
| 双独立逻辑中心迁移 | 保留原范围 | 独立 DB/CA/process，非独立物理主机；当前双实例 health ready |
| 双人审批/签名方权威/HTTP/React | 保留受限实现与证据 | 同租户 admin、不同请求/批准人、一次性签发；token 不走浏览器；当前安装包与最新 UI 范围须分开 |
| agent token durable replay ledger | 保留已有闭环 | token ID 和签名/claim 绑定；不重放已消费材料；转移中断操作步骤仍需明确 |
| center2 systemd 模板和实例接管 | 部分成立 | 既有手工迁移和服务运行成立；全新 init/activate 分别有 F14/F11 |
| signer rotation（源码/staged/installed） | 保留原范围 | 已核对单次授权/签名方绑定/旧料备份；不等同于 CA 信任锚轮换 |
| token 撤销 registry/gRPC/HTTP/React | 保留受限闭环 | fail-closed 查询与旧 token 不重开审批；最新 HTTP/React 的 installed candidate 仍未交付 |
| 同中心 client 叶证书轮换 | 保留原 Linux 成功范围 | 不能泛化为 CA 轮换；macOS packaging 存在 umask/openssl 相关失败，见下节 |
| grant/handover network writer | 仅 setup/state 级成立 | 原 11 步无真实接口写入；完整 writer 生命周期与统一系统变更锁未覆盖新模块 |
| nftables 防火墙 | 原型，不能验收完成 | F05/F08/F09；缺 NAT/转发、Kura API/PID、远程协议/UI、早启加载和真实流量验证 |
| 离线恢复 | 原型，不能验收完成 | F03/F04/F06/F07；opaque/sleep probe 不能证明 Kura XML/加密/factory/真实恢复成功 |
| 安装卸载操作手册及 11 张截图 | 文档存在，版本适用性需修正 | 抽查中英截图可读；源自临时当前 UI，手册标题却标 console26，不能保证该旧包拥有截图中的最新撤销面板；首装/实例步骤受发现影响 |
| Docker/compose/首登强改/3 VM | 部分成立 | 源码首登闸门与 mock 通过，现有镜像/保留卷/3 agent VM 存在；TLS 生成脚本 F12；本轮未重启 compose，未重建接入流程 |
| 正式发行及完整阶段 2–5 | 未完成 | 仍有实现缺口和测试缺口；与缺环境 deferred 分开记录 |

## 本次验证与证据质量

| 检查 | 当前结果 |
| --- | --- |
| core constraints/revision/admission | 12 passed |
| management bridge 全部当前单测 | 33 passed |
| agent Rust 当前套件 | 76 passed，真实 nft 1 ignored |
| console / protocol Rust 当前套件 | 15+8 passed；PostgreSQL 2、mTLS 3 ignored，未冒充通过 |
| React typecheck + production build | 通过；约 1026 kB 主 chunk 提示仍在 |
| 选定纯 mock/模型/双语浏览器集合 | 22 passed，报告 `target/review-20261104-browser.json`；不包含依赖旧开发中心的 live 集合 |
| console packaging | 3 passed，2 个 root 专用用例 skipped |
| agent packaging，宿主默认 umask | 37 passed / 1 failed：foreign 证书用例的临时私钥未固定 0600，先被权限检查拒绝，未到期望的 trust-chain 分支 |
| agent packaging，umask 077 对照 | 37 passed / 1 failed：另一用例硬编码期待公钥 0644，实际 0600；说明套件依赖外部 umask，未修改实现/断言来“跑绿” |
| 独立 Java 当前源码探针 | F02 两个错误均复现 |
| 独立 Rust 当前模块探针 | 六个探针确认补偿删文件/不停服、确认窗口不一致、令牌缺失、RECOVERY_REQUIRED 被覆盖、权限丢失、nft 文本注入；均为临时目录/fake 子进程，不是 Linux 系统验收 |
| 两个一次性 agent Setup root | F04 的未决 guard/purge 缺口复现，无真实 state 删除 |
| 安装 payload 的临时实例 root | F14 在现有 OrbStack 实际 Python/用户组下复现，没有创建真实 systemd 实例 |
| Docker shell 静态检查 | tls-gen `sh -n` exit2；entrypoint 语法可检查，未执行部署 |

审查探针位于 `target/review-20261104/`，其中 `rust-probes` 通过 path 引用真实源文件，没有复制修改业务实现。`worktree-before-report.json` 保存本次记录前的 HEAD、dirty 清单和 SHA，`evidence-reference-audit.json` 保存 33 个后续证据清单的路径核对。

证据清单发现：

- agent `docs/evidence/firewall-nftables-20261103.json:6` 是非法 JSON，嵌入 nftables JSON 的引号未转义，标准解析失败；不能声称其机器可读核对通过。
- 两个引用未定位：`target/package/console-deb-20261102-binding-generation/agent-tests.log`、`target/transfer-ui-browser.json`。保留为证据缺口，不据此否定其他仍在的独立证据。
- 旧源码哈希与后续格式化/增量不一致是历史版本差异，不能把旧测试汇总成当前全量验收。
- Docker 构建 context 为仓库根 `..`，但 ignore 文件放在 `docker/.dockerignore`；根 `.dockerignore` 和 `docker/Dockerfile.dockerignore` 均不存在。应核对实际 context 排除；当前 `docker/tls/` 仍有未跟踪的生成身份材料，后续源码保存点不可混入这些私有数据。
- 首登改密新错误 PASSWORD_CHANGE_REQUIRED 未见于 authored OpenAPI 错误枚举；API/类型文档应同步，不能只靠 UI 翻译测试证明契约齐全。

## 只读环境核对与未完成范围

2026-11-04 本次观察：OrbStack 现有管理 VM 安装 console `0.1.0~package20261102-26`、agent `0.1.0~package20261103-20`。默认中心、@center2 和 agent 都 active；18443/28443 health ready。agent 二进制 SHA 为 `5e9f1d0c7d987947964ab01e687df3adf8dfb4750771d438a61c8baacc1457e6`。firewall/recovery 事务文件均不存在，未对真实业务执行恢复/防火墙操作。

三个服务 effective ProtectSystem/NoNewPrivileges 均为 no，agent 有 cap_net_admin，ReadWritePaths 为空。现有 OrbStack 覆盖不能证明发行 unit 的沙箱成立；原 unit ProtectSystem=strict 又未为 Netplan/实际快照目录声明写路径，需要在非 OrbStack 环境继续验证和修正。

Docker 无运行容器，镜像 `yofc-management-console:20261104` 和 `docker_pg-data` / `docker_console-data` 卷仍在；yofc-agent-01/02/03 三个 service 均 active。没有重启这些服务，没有恢复 compose，没有连接 192.168.2.125，没有清理运行 target/runtime、个人 ~/.kura-dev 或旧夹具。

以下是**实现未交付**，不是单纯缺环境：防火墙 NAT/转发/Kura 兼容入口/中心及 React 面/早启加载；离线恢复 Java 格式与加密工具、真实 Kura 恢复启动/factory 删除与保留、字段修正、恢复 ID 和业务核验、远程制品传输/React；固定 service.restart/system.reboot 的独立任务闭环；完整网络字段映射与全量发行组合；持续多次 key 轮换、生产迁移和一致备份等。

缺环境部分继续 deferred：amd64、最新 26.04 各对应包/写能力回归、完整生效 systemd 沙箱、真实 NM/cloud-init/物理有线、独立物理中心及网络分区、磁盘满/逐点进程与主机故障/硬断电。已有 Parallels 只读证据不能填这些格。

下一会话先修 F01–F08/F10，再处理其余入口与证据；单元/契约和浏览器通过后，才用新名字的隔离 fixture 进入 IDEA 完整 Kura及原 GWT 登录，再到已有 OrbStack。不得重放旧一次性入口或通过删除 state 消除 UNKNOWN。用户明确要求停止本轮，审查完成后暂停原目标；不能标整体 complete。
