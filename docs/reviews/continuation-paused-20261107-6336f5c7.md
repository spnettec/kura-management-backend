/goal 恢复 YOFC Kura 管理体系改造，继续实际实施。

工作空间：/Users/heyoulin/iot-kura-develop/git。上一会话按用户要求于 2026-11-07
完成本轮收尾后暂停，整体目标未完成；现在明确继续。

先读取根目录和相关子仓库 AGENTS.md，使用 obsidian-workspace-knowledge：
python3 ~/.local/share/workspace-knowledge/kb.py context --dir=/Users/heyoulin/iot-kura-develop/git

通过官方 Obsidian CLI 全文读取最新交接：
Experience/2026-11-07-Kura防火墙GWT与真实Core快照系统验证暂停-6336f5c7.md，
以及 Tasks.md、Session Summaries.md 最新核定。使用父工作空间库，不另建子仓库库。

再全文读 kura-management-console/docs/reviews/firewall-gwt-system-20261107.md、
kura-management-console/docs/evidence/firewall-gwt-system-20261107.json、
kura-management-console/docs/reviews/continuation-prompt-20261105.md 及其中列出的原方案、
历史交接与证据；早期 9d21a7f4 交接保留为历史，不覆盖最新核定。

先实时核对 18 仓 HEAD/branch/全部未提交改动、新 manifest 指纹和运行状态。
保留工作树，不 reset/clean、提交推送或覆盖远端，不用子代理。代码发现优先
codebase-memory，按真实子仓库选项目，索引过期则更新；MCP 不可用时用官方 CLI，
绝不索引父工作空间。

本会话已完成最终双语 GWT 夹具 5 prepare/3 confirm/2 rollback、0 系统写；修复真实
Core IPv6 转发目标导出成对象标识，受影响合同 229 项通过；新 IDEA v4、Orb v2、
现有 Parallels 的所选真实 GWT/CM/snapshot 与流量检查通过。Orb 最终 revision3、
4 unique prepare/2 confirm/2 rollback，真实 prepare/confirm 丢回执后只查询原事务、
没有重发。Parallels revision2、1 prepare/1 confirm，Kura Cap0/agent Cap1000、
NNP1/EROFS30、private JRE184。两 VM 真实旧 Core snapshot 不回退防火墙、旧文件保留，
新 snapshot 仅确认值；这些所选检查不再笼统列为未实施，也不重放任何一次性入口。

首先处理仍未闭环的初始 GWT CONFLICT（native-v3 与 Parallels 写前复现，根因未定），
以及 COMMITTED/ROLLED_BACK 后确认倒计时仍递减的问题。先从保留证据和只读路径定位，
不要为调查重发旧写或重开失败恢复入口。三份旧可变 Surefire XML 被重跑更新，原文
未找到；旧 manifest/日志及151 evidence pins 保留。把这个保留限制与两历史 gap 分开，
不宣称旧45 artifact pins仍全部匹配。

再按最终台账推进完整组件/元数据/GWT编辑、生产及外部PID Java恢复与空/部分/
不一致CM策略、NM自动NAT与其他writer正式归属接管/退役/安装profile、中心加密
防火墙任务/双语远程与受影响新管理链路、confirmed-only实际早启、其余Core安装
升级/bootstrap交接、一致DB/制品/key备份恢复轮换迁移和正式发行。完整firewall与
广义Kura恢复未完成。真实NM/cloud-init/物理硬件、amd64、磁盘满/硬断电为external
deferred，内部pending与两历史gap另列；Java已可用。已闭环F01–F19、所选恢复/
补偿/取消/restart/reboot/NAT等一次性验收不重做。

保留NM；Kura无agent仍独立使用；agent始终运行，远程开关仅管到中心的通道；
Kura后端生成私钥/CSR，原GWT仅安装agent后显示远程菜单。保留admin/admin和首次
仅新密码+确认、成功直接进控制台及后台闸门；console中英文、配置默认关闭与PID
显式审核、firewall默认关闭、身份与归属独立、普通注册不得抢占；UNKNOWN不重发，
RECOVERY_REQUIRED持续挡写并保留现场。

最终新增Mac/VM进程与unit/helper全停；Parallels Ubuntu正常stopped、Windows仍stopped；
原Orb四服务原PID244516/244911/286620/268、console26/agent20原包active，不重启原服务。
全部旧新fixture/state/key/backup/quarantine/namespace/卷保留，不连接192.168.2.125，
不清理运行中的target/runtime或个人~/.kura-dev。新变化按单元/契约与浏览器→IDEA
UI完整隔离Kura及原登录→Orb→现有Parallels验证，缺失环境如实保留deferred。
每轮收尾更新知识库、台账、证据和准确运行/停止状态。
