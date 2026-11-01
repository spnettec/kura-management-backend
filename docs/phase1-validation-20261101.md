# Rust / React 首个独立只读闭环：实施与证据

日期：2026-11-01（Asia/Shanghai）。对应总方案阶段 0/1 的受控原型；整个管理体系改造仍在进行。
2026-10-31 的 Java/Vue 整合及 17 项旧冒烟未作为下表新架构验收依据。

## 交付代码

- 中心：`crates/console`，Axum HTTPS + Tonic mTLS + PostgreSQL；持久会话、租户角色、任务租约/幂等/过期、SSE、审计及准入限流。
- 协议：`crates/management-protocol`、`proto/management/v1/control.proto`、`docs/openapi-v1.json`；远程 Protobuf 和本机有界 UDS 帧。React HTTP 类型由 OpenAPI 生成。
- 前端：`frontend/react`，登录、网关、详情、只读任务和结果；同源 Cookie/CSRF/SSE；会话切换清空数据、丢弃旧响应并合并事件刷新。旧 Vue 仍保留。
- agent：独立仓库 `../kura-management-agent/src`，本机服务先启动；5 秒心跳、1–30 秒退避、SQLite WAL/FULL 结果去重；私钥/状态权限和 Linux 进程 `no_new_privs`。
- Kura：`../kura-networking/bundles/org.eclipse.kura.management.bridge`，Java 21 / DS / UDS，直接调用 OSGi `ConfigurationService`。当前仅配置 PID 与快照 ID，不读写秘密、不依赖 Web 账号。
- 运维入口：`create-user`、`register-gateway`、`set-gateway-enabled`、`rotate-gateway-certificate`。服务启动不自动生成生产默认账号。

工具链、状态/错误/权限和容量约束见 [phase0-contract.md](phase0-contract.md)。

## 已验证

| 检查 | 实际结果 | 原始证据 |
| --- | --- | --- |
| Rust 中心/协议单元及契约 | 4 passed | `target/phase1-tests.log` |
| Rust agent 单元，macOS + Linux | 各 4 passed；含并发冲突、SQLite 重开、失联注册清理和权限拒绝 | `/tmp/yofc-agent-test.log`；VM cache `agent-tests.log` |
| Clippy | 两个 Rust 仓库 all-targets / `-D warnings` 通过 | `target/phase1-clippy.log`；`/tmp/yofc-agent-clippy.log` |
| Java 桥接单元与 bundle | 2 passed；Bundle Plugin 6 构建成功 | `/tmp/yofc-bridge-test.log` |
| 真实 PostgreSQL + HTTPS/SSE | 37 项通过：租户/角色、CSRF/Origin、统一拒绝、持久任务及 Kura 缺席 | `target/phase1-smoke.json` |
| 真实 mTLS | 3 测试通过：未登记证书、伪造 ID、未知协议、无客户端证书、错误主机名及过期证书均拒绝 | `target/phase1-mtls.log` |
| 重启与身份撤销 | 15 项通过：中心/agent 重启、会话/任务/结果保留、禁用和轮换撤销旧流 | `target/phase1-resilience.json` |
| 排队/过期 | 110 请求、12 并发 worker：100 接受、10 个 typed 429；幂等重试通过；100 项全部 TTL 过期且有审计 | `target/phase1-queue.json` |
| 登录限速 | typed 429；轮换用户名、伪造 X-Forwarded-For 无法绕过来源 IP 限制 | `target/phase1-login-limit.json` |
| Chrome 浏览器 | 2 流程通过、0 unexpected / flaky；登录、状态、任务、刷新、退出、viewer 与其他租户 | `target/phase1-browser.json`；`target/phase1-{login,gateways}.png` |
| IDEA 完整 Kura + 原控制台 | 隔离运行完整 runtime；原控制台 admin 登录成功；中心经 gRPC→UDS→DS 返回 24 个配置 PID 和快照 `[0]` | `target/phase1-kura-live.json`；`target/phase1-kura-console.png` |
| Kura 停止 | 仅停止本轮隔离 JVM 后，agent 心跳、UDS 与只读管理继续正常；Kura 方法返回 KURA_UNAVAILABLE | `target/phase1-smoke.json`；`target/phase1-resilience.json` |
| OrbStack 实际 Ubuntu 24.04.5 arm64 | 14 项通过：Linux 编译、独立 systemd、UDS、mTLS、服务重启及 VM 重启自启动/回连、真实任务、空 capability set、全部线程 NoNewPrivs=1 | `target/phase1-orbstack.json`；`target/phase1-orbstack-journal.log` |

证据摘要和代码 SHA-256 归档在 [evidence/phase1-20261101.json](evidence/phase1-20261101.json)。
IDEA 首次发现 manifest 错误导入了 configuration 1.4；实际 API 为 1.2，已修正为 `[1.2,2)`，
并在隔离 JVM 停止后用 Equinox `-clean` 刷新其缓存。未删除或清理原运行集、快照或个人数据。

## 复跑入口和当前运行状态

```sh
cd /Users/heyoulin/iot-kura-develop/git/kura-management-console
cargo test --locked --workspace
cargo clippy --locked --workspace --all-targets -- -D warnings
cd frontend/react
npm ci
npm run generate
npm run build
cd ../..
cargo build --locked --workspace
# 在 agent 仓库也执行 cargo build --locked，然后启动隔离 PostgreSQL、中心和 agent：
python3 scripts/phase1-dev.py up
python3 scripts/phase1-smoke.py
cargo test --locked -p yofc-management-console --test mtls -- --ignored
python3 scripts/phase1-resilience.py
python3 scripts/phase1-queue-smoke.py
```

浏览器测试在 `frontend/react` 执行 `npm run test:browser`。涉及登录的脚本应串行；
每次登录会撤销同用户旧会话，累计测试会触发 10 次/60 秒限制。`phase1-login-limit.py`
专门验证这一限制；之后可停止/重启本轮隔离中心恢复开发预览。

本机 HTTPS `https://localhost:18443`，gRPC `18444`。开发随机密码只在
`.phase1-dev/admin.password`，CA 为 `.phase1-dev/ca.pem`；密钥不纳入 Git。
测试证书有效期 7 天；不是生产注册/签发系统。`phase1-dev.py down` 仅停止本轮进程并保留数据。
数据库仅使用隔离 Unix socket，未启动 Homebrew PostgreSQL 服务。

IDEA 配置在核心仓库忽略目录 `.idea/runConfigurations/Kura_management_phase1_isolated.xml`。
隔离 runtime/data 为本仓库 `.phase1-dev/kura-runtime` / `kura-home`，端口 18080/18445/18446；
验收后隔离 Kura 已停止。原 `kura-dev-runtime/target/runtime` 和 `~/.kura-dev` 均保留。

实际 VM 为 `kura-package-validation-20261029`；历史 `ubuntu` VM 已不存在。
新 agent fixture 为 `yofc-management-phase1-agent.service`，程序/配置/状态分别在
`/opt/yofc-management-phase1`、`/etc/yofc-management-phase1`、`/var/lib/yofc-management-phase1`。
VM 与 fixture 当前保留运行，便于接续；这是 systemd fixture，不能记成 `.deb` 安装生命周期验收。
VM 内构建使用 `/home/heyoulin/.cache/yofc-management-phase1/{cargo,rustup,target-agent}`，不执行 Mach-O 产物。
复跑 `phase1-orbstack-smoke.py` 必须显式传 VM 名，会重启该 VM。

## 仍未交付或 deferred

- 阶段 2–5 的 Netplan/nftables 写事务、系统控制、离线恢复及发行包仍未实施；本轮未改变 Linux 网络/防火墙。
- 当前 VM 没有 Kura 安装；Linux 真实 Kura 桥接验收 deferred，不能用 macOS 或缺席测试替代。
- OrbStack `/run/systemd/system/service.d/zzz-lxc-service.conf` 覆盖 NoNewPrivileges/ProtectSystem/ProtectHome/PrivateTmp 等。进程自设 no_new_privs 已验证；其余文件系统沙箱须在非容器 Ubuntu 验收。
- 100 网关吞吐、重连风暴/慢消费者、硬件有线拔插/硬断电、Ubuntu 26.04 与 amd64 矩阵均未据此验收。
- 一次性注册引导/自动轮换、细粒度网关授权、完整六项 Kura 桥接与敏感字段、完整拒绝审计、中心历史配额和正式协议制品仍需补齐。
- agent 本地 `[patch.crates-io]` 协议映射仅供工作空间开发；发布必须改为可校验的版本制品。
- React production audit 为 0；完整开发依赖 audit 有 3 moderate（OpenAPI 生成工具链），无 high/critical。仍有约 908 KB JS chunk 提示。
- 阶段 2 前置观测：VM 有 Netplan 1.1.2 / systemd 255.4，networkd active；`10-lxc.yaml` 和 `90-NM-…yaml` 同时描述 eth0，后者 renderer 为 NetworkManager。不可把它当纯 networkd 基线自动接管。

## 接续顺序

先核对此报告和 evidence，不重复迁移/已过检查。阶段 2 从显式识别 Netplan 多来源/外部写入者、
有线字段模型、候选/确认配置与持久事务状态机开始；在专用可回退环境证明断线回退和启动恢复后，
再接 Kura provider/快照镜像和 React 编辑。任何上线网络写权限须有实际证据，不能直接扩展阶段 1 空 capability set。

全部改动仍未提交、未推送。既有整合改动、旧 Java/Vue/MQTT、前端归档和两个旧 GitHub 仓库保留；
没有 reset/clean，也未触碰已移除的 192.168.2.125。
