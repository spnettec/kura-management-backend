# 阶段 2 网络事务原型实施证据

日期：2026-11-01（Asia/Shanghai）。这是首次网络事务验收快照；后续单一来源接管与 revision 2 的新增证据见 `phase2-takeover-validation-20261101.md`，以下数字和环境描述保留当时语义。整体改造保持进行中；阶段 2 的受限接口事务原型可运行，
不能作为旧 Kura 网络 provider 已替换或整个方案验收完成的结论。
架构方案、原始 consolidation 和阶段 1 证据保留，未重复迁移、提交、推送或改远端。

## 实际代码

- console：共享 Protobuf 加入五种网络操作及类型化输入；PostgreSQL 增量 migration 0003
  扩展操作和持久输入；任务幂等比较内容，管理员网络写权限和拒绝审计；gRPC 派发类型化请求。
- agent：`src/network.rs` 实现来源/权限/字段校验、真实 staging generate、运行层候选、
  持久确认意图、原子文件与目录 fsync、进程锁、期限恢复、重启恢复及运行值回读。
  观测读取原子快照，不占写锁；UDS 管理方法只授予 root/本机 admin，不授予 Kura 登记连接。
  gRPC 结果缓存增补完整请求指纹，prepare 后主动重连，新 epoch 才能远程确认。
- React：网络来源与接口、DHCP/静态地址/路由/DNS/MTU 表单、独立事务状态、倒计时和确认/回滚。
  当前事务置于表单之前并定位显示，避免关键确认按钮在长表单下方；任务成功与网络提交分开显示。
- packaging：保留默认只观测 profile，新增显式网络 profile 示例；它尚未通过发布级 sandbox 验收。

详细字段、顺序、权限、边界和未完成项见 `phase2-network-contract.md`。

## 验证与可复跑证据

遵循单元/契约 → 浏览器 → IDEA 完整 Kura/原控制台 → OrbStack 的首次验证顺序，随后仅对新增
代码和修正做必要复跑。以下均为本次 Rust/React 的新增证据，不引用旧 Java/Vue 的 17 项冒烟作验收。

| 检查 | 结果 | 证据 |
| --- | --- | --- |
| 中心/协议单元 | 4 通过，mTLS 的 3 个需 fixture 测试本轮未重跑 | console cargo test 输出；阶段 1 仍保留原 mTLS 实测 |
| agent macOS/Linux 单元 | 各 13 通过，包含 9 个网络状态/恢复/并发观测与首次接管拒绝测试 | `target/phase2-agent-macos.log`、`phase2-agent-linux.log` |
| Rust Clippy | console/agent all-targets、`-D warnings` 通过 | 构建输出及 agent 日志 |
| React 类型检查/生产构建 | 通过 | npm build；942 kB 左右 chunk 提示仍存在 |
| 实际 HTTPS/gRPC 新契约 | 16 项通过，宿主仅观测 | `target/phase2-contract.json` |
| 浏览器观测模式 | 1 流程通过，0 unexpected/flaky | `target/phase2-browser.json` |
| IDEA 完整隔离 Kura | 启动及原控制台 admin 登录；新 agent 查询得到 24 个 PID；网络观测同时可用 | `target/phase2-kura-live.json`，本次 UI 实见 `/admin/console` |
| Ubuntu 实际网络 | 16 项通过 | `target/phase2-network.json` |
| 首次接口配置/补偿 | 23 项通过，静态地址、MTU、UP/down、agent 重启、外部漂移及拒绝无授权接管 | `target/phase2-initial-link.json` |
| 首次远程确认 | 真正 mTLS 新流确认并持久化 revision 1 | `target/phase2-network-commit.json`，任务 `6f7375bf-95be-4b3b-b81e-674a89457454` |
| 浏览器实际写入/回滚 | 1 流程通过，0 unexpected/flaky，回读恢复 192.0.2.2/24 | `target/phase2-network-browser.json`、`phase2-network-candidate.png` |

网络 VM：新建 `yofc-management-network-20261101`，Ubuntu 24.04.5 arm64，Netplan/systemd-networkd，
专用 dummy 接口 `yofc-test0`。证明候选静态地址与基本路由真正生效、30 秒未确认回滚、
主动回滚清除候选路由、agent 进程重启回滚、VM 重启后只加载已确认 revision 1。
原 `/etc/netplan/10-lxc.yaml` 和默认管理路由前后相同；Kura 不运行时网络事务仍可处理。
首次确认后后续脚本可以从 revision 1 续跑，16 项不重复计入首次空接口初始化；首次提交另有持久任务证据。

浏览器截图已人工查看。初次观测测试的非唯一文本 locator 已修正；Linux 观测测试 fixture
曾误用真实 ip 子进程，在并行 close/reopen 锁测试中产生 fork 窗口，改为统一 fake 后两平台通过。
真实 VM 仍使用系统程序，不将 fake 单元结果写成 Netplan 验收。

新增首次接口检查实际发现 Netplan 在移除该接口配置后清除了地址但保留候选 MTU；旧实现正确
报 RECOVERY_REQUIRED，未冒称回退通过。现已为首次空接口增加目标范围固定的 MTU/行政状态
补偿，真实 VM 显式回滚及 agent 重启均回读恢复；外部修改 MTU 时拒绝覆盖，操作员纠正后可
通过同一回滚入口恢复。预先存在手工 IPv4、路由或全局 IPv6 会在写候选前拒绝，原日志向后可读。

可复跑命令（登录同一账号会替换旧会话，请串行跑 API 与浏览器）：

```bash
python3 scripts/phase1-dev.py up
python3 scripts/phase2-contract-smoke.py
cd frontend/react
npx playwright test --grep 'network sources'
cd ../..
python3 scripts/phase2-orbstack-install.py yofc-management-network-20261101
python3 scripts/phase2-initial-link-smoke.py yofc-management-network-20261101
python3 scripts/phase2-network-smoke.py yofc-management-network-20261101
cd frontend/react
npx playwright test tests/network-write.spec.ts
```

Linux 安装脚本只更新命名 fixture；不是 deb 安装器。网络 smoke 明确重启该 VM，且只允许其固定名称。
脚本维护渐进 JSON 证据；遇到实际 RECOVERY_REQUIRED 即失败，不把文件/进程状态替代运行恢复。

## 留下的环境与下一步

Mac `https://localhost:18443` / gRPC 18444、私有 PG 与原阶段 1 fixture 沿用。
IDEA 隔离 Kura 在验证后已停，原 `target/runtime`、个人 `~/.kura-dev` 未清理，核心无跟踪改动。
原 Ubuntu fixture 保留运行，仅复用其 VM 本地 Rust 构建缓存；其混合来源 eth0 未接管。
新 fixture 的服务是 `yofc-management-phase2-agent.service` 和 `yofc-management-phase2-link.service`，
程序/配置/状态分别在 `/opt`、`/etc`、`/var/lib` 的 `yofc-management-phase2` 目录。
只接管固定测试接口，最后确认值为 192.0.2.2/24，普通测试候选已回滚。
首次接口脚本临时创建并授权 `yofc-first0`，成功后撤回其授权、删除该 dummy；失败则留下现场以便诊断。

下一步仍是阶段 2：明确已有文件的接管计划与恢复；完成 DHCP/resolver/IPv6/冲突等运行矩阵；
在 networking 做 provider、Kura 镜像和三个旧入口的事务协调，再按方案推进 nftables/系统控制/
离线恢复/发布。一次性注册、自动证书轮换、六项桥接、细粒度目标授权、完整审计与容量等阶段 1
剩余项仍然开放。

**deferred**：真实物理有线接口、硬断电/存储故障、非 LXC 的 systemd 沙箱、Linux 真正 Kura
provider 集成、amd64/26.04 与正式包生命周期。已有环境可做但尚未实现/验收的项目保留 pending，
不借 deferred 表示完成。未连接 192.168.2.125。

代码与结果指纹摘要见 `docs/evidence/phase2-20261101.json`。历史 phase1 指纹是带日期的旧快照，
本轮扩展协议和服务后不再将其当作当前源码指纹。
