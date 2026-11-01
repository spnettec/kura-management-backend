# Kura Management Console

多网关管理中心，统一包含服务端与浏览器前端。2026-10-31 将原
`kura-management-backend` 本地仓库改名为本项目，原独立前端并入 `frontend/`。

已确定的目标是 **Rust（Axum + Tonic）服务端 + React + TypeScript 前端**，
中心通过 gRPC 双向流、Protobuf 和 mTLS 连接独立 Rust agent。
完整范围、接口、故障恢复、验收和工作量见
[完整改造方案](docs/management-redesign-plan.md)。

Spring Boot + Vue 3/MQTT 保留为迁移前的可运行基线。2026-11-01 已有下文的 Rust/React
独立原型及真实验证，完整迁移仍进行中。Java 包名中的 `backend` 暂时保留。

## 当前目录

```text
kura-management-console/
├── src/                 # 过渡期 Spring Boot 服务端
├── pom.xml
├── frontend/            # 原 Vue 前端源码；后续在此迁移到 React + TypeScript
├── docker/              # 过渡期 MQTT 联调配置
├── scripts/
└── docs/
```

## 开发当前基线

服务端使用 JDK 17 以上、Maven 和本地 MQTT。默认 HTTP 端口为 `8081`。

```bash
docker compose -f docker/docker-compose.yml up -d
mvn -Dmaven.repo.local=/Users/heyoulin/.m2/repository spring-boot:run -Dspring-boot.run.profiles=dev
```

`dev` 使用内存 H2，初始化演示网关 `gw-001`；生产配置使用 PostgreSQL/Flyway。
当前初始化器会在任意 profile 预置演示账号和数据，broker 也带开发默认配置，
当前产物仅作为迁移联调基线。Rust 版本将改为明确的管理员初始化和网关注册，
并移除对中心 MQTT broker 的依赖。

另一个终端启动前端：

```bash
cd frontend
npm ci
npm run dev
```

Vite 将 `/api` 代理到 `http://localhost:8081`。前端已不再是同级独立项目。

## 构建包含前端的当前服务端产物

```bash
./scripts/build-with-frontend.sh -Dmaven.repo.local=/Users/heyoulin/.m2/repository
java -jar target/kura-management-console-0.1.0-SNAPSHOT.jar --spring.profiles.active=dev
```

脚本依次执行 `npm ci`、类型检查、前端构建及 `mvn -Pwith-frontend clean verify`，
将浏览器静态资源打入同一个 jar。页面路径支持刷新，`/api/**` 仍按原认证规则处理。
单独启用 Maven profile 不会自动执行 npm；发布时使用上述脚本。

## Rust / React 独立管理原型（已实施）

新中心在 `crates/console`，共享协议在 `crates/management-protocol`，新前端在
`frontend/react`，独立 Rust agent 在同级 `kura-management-agent`。
现已打通 HTTPS 登录/网关/任务/SSE、mTLS 心跳和重连、SQLite/PostgreSQL 持久化、
UDS 与真实 Kura 只读桥接。前端构建资源由中心同源托管，生产不需要 Node/Vite 常驻。
React console 支持简体中文和英文，登录页、顶栏与详情抽屉均可切换并记住选择；
首次访问匹配浏览器语言。见 [中英文实施与验证](docs/console-i18n-20261101.md)。
中心任务 input/result 现在使用独立文件密钥加密，启动需要 `CONSOLE_DATA_KEY_FILE`；
首次初始化、旧数据迁移和保留边界见 [任务存储契约](docs/task-payload-storage-v1.md)。
`scripts/phase1-dev.py up` 会初始化尚未绑定密钥的开发库，已经绑定时不会补造丢失密钥。
网络 UNKNOWN 已增加原任务与业务日志绑定的历史结果核定、一次审计及中英文说明；查询不重放网络动作。
准确范围和 Linux 回执损失注入见 [网络结果核定验证](docs/phase2-network-outcome-validation-20261101.md)。

见 [实施契约](docs/phase0-contract.md) 和
[2026-11-01 实施、复跑与验证报告](docs/phase1-validation-20261101.md)。
2026-11-01 已增加受限有线接口事务：类型化网络任务、候选/确认/回滚、React 操作面板和
真实 Ubuntu Netplan/networkd 验证。见 [网络契约](docs/phase2-network-contract.md) 与
[阶段 2 原型证据](docs/phase2-validation-20261101.md)。Kura 旧网络入口的适配、防火墙、
离线恢复与正式发行仍未完成。上文 Java/Vue 入口保留为迁移基线。
随后已实施单一明确 networkd 来源的审阅/转交和重启恢复，保留原权限、其他接口及 IPv6；
见 [来源接管证据](docs/phase2-takeover-validation-20261101.md)。NM/cloud-init 退出、完整写入者
清单和运行矩阵仍待实现，不能当作通用系统网络接管工具。

## 仓库迁移记录

参见 [2026-10-31 本地合并记录](docs/project-consolidation-20261031.md)。原 backend 的
Git 主线和旧前端的归档引用均已保留。远端仓库名称、删除和推送未在本次本地调整中执行。
