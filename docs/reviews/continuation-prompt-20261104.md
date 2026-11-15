# 下一会话交接提示词

以下正文可直接复制到新会话。本轮审查已结束，原实施目标应保持暂停，直到用户在新会话明确恢复。

```text
恢复 YOFC Kura 管理体系任务。先完成 2026-11-04 Review 的修复与范围核定，再继续原完整方案；不要把已有受限原型当作整体交付完成。

工作空间 /Users/heyoulin/iot-kura-develop/git。
先读取根目录及相关子仓库 AGENTS.md，使用 obsidian-workspace-knowledge：
python3 ~/.local/share/workspace-knowledge/kb.py context --dir=/Users/heyoulin/iot-kura-develop/git
知识库必须 scope=ws，父工作空间 vault=f9b864007dcd8a44。全文读取 Tasks.md、最新 Review 交接、原交接及关联决策。知识操作使用官方 Obsidian CLI；不要直接写 vault 文件，不写全局库，不改人工 Home。

首先全文读取：
kura-management-console/docs/reviews/management-redesign-review-20261104.md
kura-management-console/docs/management-redesign-handoff-20261102.md（含直到 11-04 的全部补记）
kura-management-console/docs/management-redesign-plan.md
原本会话交接：
Experience/2026-11-01-Kura管理体系本地保存点与配置适用性-47efca29.md
Experience/2026-11-01-Kura管理体系快照保存回滚接线-de6eed61.md
Experience/2026-11-01-Kura管理体系受限远程配置更新与暂停交接-6b48b7d2.md
以及最新 Review 笔记（从 Tasks 中取精确路径）。历史结论必须按日期/版本/环境解释。

2026-11-04 审查版本：
kura dd073cb077，kura-networking 81534f10be，二者工作树 clean。
console 5f5b1ab6af / heyoulin-kura / ahead28，agent 0b99202903 / heyoulin-kura / ahead17，二者有大量未提交及未跟踪的新实现。
旧保存点仍为 console eaa8b68、agent c8e41e4、kura e2c616cb44、networking c362c2e、container 000835d、management-ui 573a2c4。
本次 Review 没有改业务实现、提交或推送，只增加审查文档和 target 内的隔离探针。先核对当前状态，不假定这些 HEAD 之后没有用户改动。
保留全部未提交改动，不 reset/clean，不覆盖用户修改，不擅自提交/推送或覆盖两套旧远端。此前用户已要求保存过本地 checkpoint，但本次新的 Review 没有再次提交。docker/tls 有未跟踪的生成身份材料，后续源码保存点不得混入私钥/凭据。

优先修复 Review 中有证据的高风险项：
F01 grpc.rs:92 的握手 UPDATE 把同 bindingGeneration 的 QUEUED/RUNNING 标为 EXPIRED；普通断线重连会丢任务/不确定写入，旧代次队列反而没处理。完整覆盖同代次重连、代次变化、丢回执、UNKNOWN；别只换一个比较符后结束。
F02 ConfigurationConstraints.java:80/159 把 STRING options 当作长度比较：safe→NumberFormatException，option "4" 错误接受 "abcd"。
F03 recovery.rs:399 补偿不先停服务，且清理备份清单外文件会删除漂移候选和无关快照；必须先独占，未知/漂移进入人工恢复，不能无归属删除。
F04 agent setup.py:902/958 的维护/转移/purge guard 漏 firewall/recovery；WAIT_CONFIRM/COMMITTING 可通过 guard 并被 purge 删除。
F05 firewall.rs:450/456 的确认/回退持久化顺序有窗口：提前 ROLLED_BACK 导致失败不重试，确认文件已更新但旧 WAIT_CONFIRM 日志会在重启时反向回退。按持久决定恢复，执行和回读后才终态。
F06 RECOVERY_REQUIRED 允许新 prepare 覆盖原日志，应一直挡写直到明确人工核定。
F07 recovery 原子写不保留 uid/gid/mode，root agent UMask0077 产生 root0600 候选，低权限 Kura 读不到；两个新模块的日志/manifest/receipt 缺文件与目录 fsync，不能称 durable。
F08 firewall source_v4/source_v6 只做字符包含判断并拼入 nft 脚本，可注入全局 flush。用真实 IP/CIDR 解析与规范化渲染，补真实外来 writer 识别。
F10 console preinst 无条件读取不存在的 console.env；历史安装预写 env，不是新手册描述的干净首装。区分无旧状态首装与保留状态维护的 fail-closed。

其余已确认问题也要逐项修：
F09 firewall prepare/apply/status 都不返 token，现有脚本读私有 journal 绕过 API，不能形成正常 UDS confirm/rollback。
F11 console --instance activate 使用全局 UNIT，误启动默认实例。
F12 docker/tls-gen.sh:24 有多余右括号/字符串，sh -n exit2；client.ext 路径也错，应核对 tls/client.ext。
F13 中心 apply_result 在 seal 后把 data 遮蔽成密文，随后读 reconciliation.source，网络核定会误审计为配置核定。
F14 init-instance mkdir root:root 后立刻要求 yofc-console 组却未设置，新目录初始化失败；旧测试预先设属组掩盖了问题。

证据/文档问题：
agent docs/evidence/firewall-nftables-20261103.json:6 非法 JSON。
两条旧证据引用未定位，详见 target/review-20261104/evidence-reference-audit.json；不得伪造通过。
agent packaging 宿主默认 umask 为37/38（foreign key权限 fixture），umask077为37/38（公钥0644硬编码断言），要修可移植的测试准备，不削弱私钥权限验证。
Docker context 是仓库根，但 ignore 在 docker/.dockerignore，核对正确排除位置及私有资料边界。
OpenAPI 没包含新 PASSWORD_CHANGE_REQUIRED 错误枚举。
操作手册截图来自最新临时 UI，适用版本却标旧 console26；需要和真正发行 payload 对齐。

可复用验证和探针：
target/review-20261104/ConstraintsReview.java：编译当前 ConfigurationConstraints 的两个失败例。
target/review-20261104/handshake-query.json：原 SQL 对内存表的条件复现，不是真实 gRPC/PostgreSQL验收。
target/review-20261104/rust-probes：path 引用真实 firewall.rs/recovery.rs，六个 probe 用临时目录/fake nft/systemctl 确认缺陷，未触碰系统规则。
target/review-20261104/worktree-before-report.json：记录前 HEAD/dirty/SHA。
target/review-20261104-browser.json：22 条纯 mock/模型/双语浏览器通过。
当前通过：core12、bridge33、agent Rust76（真实nft1 ignored）、console15+protocol8（PG2/mTLS3 ignored）、React typecheck/build；不能把 ignored 或 fake probe 算作系统验收。
工具链真实入口 /opt/homebrew/opt/rustup/bin/cargo，Node /usr/local/bin/node，Python /opt/homebrew/bin/python3；默认 PATH 下的旧 ~/.cargo/bin/cargo 是无效链接，默认 python3 可能是无 tomllib 的3.9。

现有环境（核对后复用，不照抄旧 PID）：
OrbStack yofc-management-network-20261101，已装 console26/agent20；默认中心、@center2、agent active，18443/28443 health ready。
agent binary SHA 5e9f1d0c7d987947964ab01e687df3adf8dfb4750771d438a61c8baacc1457e6。
两个新子系统的真实事务文件在 Review 结束时均不存在。
effective ProtectSystem/NoNewPrivileges 都为no，agent cap_net_admin；不能当完整沙箱验收。源unit strict没有给Netplan/真实快照目录写路径，非OrbStack验证仍开放。
Docker compose 已down、卷 docker_pg-data/docker_console-data 保留，镜像 yofc-management-console:20261104 保留；yofc-agent-01/02/03 VM及agent service仍active。不要重新新建VM或重导身份。
已有 Parallels 26.04 只读证据保留，当前写能力/发行矩阵不能借用。

原完整方案仍有实现缺口：
防火墙 NAT/转发、Kura API/PID、gRPC/HTTP/React、早启加载与流量验证。
离线恢复 Java XML/加密工具、真实 Kura 恢复路径/factory 删除与保留、字段修正、恢复ID和业务核验、远程制品传输及React。opaque payload + sleep unit不是完整快照恢复。
独立 service.restart/system.reboot 持久任务闭环、完整旧网络字段映射/发行组合、持续多次key轮换、生产迁移与一致备份。
这些先记 pending，不能合并进“缺环境 deferred”。amd64、新26.04各能力、实际完整沙箱、NM/cloud-init/物理有线、跨物理中心网络分区、磁盘满/逐点崩溃/硬断电等继续deferred。

保持原架构边界：Rust/React，独立agent不进JVM；console/agent认证独立于Kura；另一中心普通注册不能抢占；网络/配置写默认关闭且审核PID显式开放；新增界面持续中英文。不要恢复删除的GPIO/Command/Bluetooth等功能。
代码发现优先codebase-memory，按list_projects真实root_path选子仓库；如果旧MCP关闭用官方CLI，不索引父目录，不用子代理。
验证按单元/契约与浏览器→IDEA完整隔离Kura及原GWT登录→已有OrbStack。修复后若需新写入，用全新命名隔离fixture；不得重放旧prepare/start/token或删除state消除UNKNOWN。
不连接192.168.2.125，不清理运行target/runtime和个人~/.kura-dev，不输出凭据，不碰旧远端。
本轮结束时更新知识库/台账并精确读回，给下一步交接。未修和未验均如实保留，不能把整体goal标complete。
```
