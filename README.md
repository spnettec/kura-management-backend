# YOFC Kura Management Console

多网关管理中心：**Rust（Axum + Tonic）服务端 + React + TypeScript 前端**。
中心通过 gRPC 双向流、Protobuf 和 mTLS 连接独立 Rust agent（`kura-management-agent`，
独立仓库、独立进程，不进入 Kura JVM）。

架构要点：

- 浏览器 ↔ 中心：HTTPS REST API + SSE（端口 18443）。
- 中心 ↔ agent：agent 主动建立的 gRPC 双向流（端口 18444），mTLS 双向认证。
- 持久化：PostgreSQL（compose 里独立容器 `yofc-e2e-pg` 同网络）。
- 仅网关 agent 执行 Netplan/nftables 与离线恢复；中心不直接写网关文件。
- 首次配置：从数据卷读安装凭据 `installation-credential.json`，
  `/api/setup/pair` → `/api/setup/operations`，完成后 setup 路由下线。

完整改造方案与验收边界见
[docs/management-redesign-plan.md](docs/management-redesign-plan.md)；
迁移交接与验证台账见 [docs/management-redesign-handoff-20261101.md](docs/management-redesign-handoff-20261101.md)。

## 目录结构

```text
kura-management-console/
├── crates/              # Rust 工作区：console（Axum+Tonic 服务端）、management-protocol（Protobuf 契约）
├── frontend/react/      # React + TypeScript + Vite 前端（中英文，词库约定见 docs/console-i18n-20261101.md）
├── migrations/          # PostgreSQL 迁移（sqlx）
├── docker/              # compose.yml、Dockerfile（node 构建 React、rust 构建服务端、ubuntu:24.04 运行）、entrypoint
├── packaging/           # 中心实例与桌面/驾驶舱部署工具（agent_control、docker_control 等）
├── scripts/             # 历史 phase1/phase2 验证脚本与 sibling 包检查工具
└── docs/                # 设计、交接、检查点证据；E2E 脚本与操作手册见下
```

## 构建与运行

```bash
# 服务端（Rust 1.95，见 rust-toolchain.toml）
cargo build --release --workspace --locked

# 前端（类型检查 + Vite 构建，产物 frontend/react/dist）
cd frontend/react && npm ci && npm run build

# 容器（多阶段：node 构建前端 → rust 构建服务端 → ubuntu:24.04 运行）
cd docker && docker compose build --no-cache && docker compose up -d
```

开发期前端热更新：`cd frontend/react && npm run dev`（默认指向本机中心的 HTTPS API）。

## 测试与端到端验证

- 服务端单元测试：`cargo test --workspace`。
- 前端：`npm run build` 内含 TypeScript 类型检查。
- 全链部署（3 台 OrbStack + 1 台 Parallels 网关 + Docker 中心）：
  [docs/e2e-scripts-20261126/](docs/e2e-scripts-20261126/)（`00-run-all.sh` 一键执行），
  操作手册与 PPT 见
  [docs/manual/e2e-deployment-manual-20261127.html](docs/manual/e2e-deployment-manual-20261127.html)。

## 历史说明

迁移前的 Java（Spring Boot）/Vue 基线已于 2026-11-28 移除；当时保留它的原因、
过渡期验证与证据见 `docs/checkpoints/` 与知识库台账。如需查阅旧实现，请从 git
历史（2026-11-28 之前的提交）读取。
