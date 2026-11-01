# 2026-10-31 管理中心本地项目合并记录

## 结果

- 原 `kura-management-backend/` 改名为 `kura-management-console/`，保留其 `.git`、分支和主线历史。
- 原独立 `kura-management-console` 前端撤出工作空间，37 个受 Git 管理文件复制到新项目 `frontend/`；复制时逐文件 SHA-256 核对。
- 新 `frontend/` 没有嵌套 `.git`；前端源码由中心仓库统一管理，npm 包名改为 `kura-management-console-web`。
- Maven artifact、应用名改为 `kura-management-console`。Java package 的 `backend` 暂留作为过渡实现命名。
- 新增统一构建、静态资源打包和明确的 SPA 页面路由；API 仍执行原认证。
- 更新工作空间 AGENTS、agent 说明、旧网络方案入口和本地联调脚本中的活动路径。
- 历史测试 JSON、旧会话与旧开发计划保留原名，避免篡改当时证据。

当前仍是 Java/Spring Boot + Vue/MQTT 基线。Rust、React、新通信协议及系统管理功能的后续实施以
[完整改造方案](management-redesign-plan.md) 为准。

## 历史和备份

| 内容 | 位置/标识 |
| --- | --- |
| 原 backend HEAD | `1ca46851af05ff3150aec49a3a0b3853062e6406` |
| 原前端 HEAD | `cf8d9634c35d4c368b30a6cc15f335f62241ce90` |
| 原前端归档引用 | 新中心仓库 `refs/archive/frontend-before-consolidation`，包含其可达历史 |
| 工作空间外备份 | `/Users/heyoulin/iot-kura-develop/repository-backups/20261031-233155-management-consolidation/` |
| 后端/前端 Git bundle | 上述目录 `backend.bundle`、`frontend.bundle`；创建后均通过 `git bundle verify` |
| 原前端完整目录 | 上述目录 `original-frontend/`，含原 `.git` 与未纳入新仓库的本地构建/依赖目录 |
| 源清单与哈希 | 上述目录 `manifest.json`，记录两侧 HEAD、origin 和 37 个前端源文件哈希 |
| 改写前网络方案 | 上述目录 `rust-netplan-wired-plan-before-consolidation.md` |

在新中心仓库可用以下命令查看旧前端历史，不需要恢复同名项目：

```bash
git log refs/archive/frontend-before-consolidation
git show refs/archive/frontend-before-consolidation:package.json
```

归档引用不属于普通 `git push` 默认推送范围；远端整理时需显式保存为归档分支或提供 bundle。
备份不放在多仓库工作空间内部，避免仓库盘点、构建或代码索引把旧项目重新当作活动项目。

## 远端状态

两个原项目目前都各有 GitHub origin；因此本次只执行本地合并，未删除远端仓库、未强推、未提交或推送。
新中心本地 `origin` 仍指向原 backend 远端：`spnettec/kura-management-backend`。
现有 `spnettec/kura-management-console` 远端仍是原前端历史，不能直接用新中心主线覆盖。

后续远端整理建议按顺序进行：保留/改名原前端远端为历史仓库，释放 console 名称；
将原 backend 远端改名为 console；更新 origin；用正常推送发布主线、迁移变更与旧前端归档分支。
仓库设置、议题、权限、CI 和克隆 URL 一并核对，不通过强推或删库来解决命名冲突。

## 本次验证

| 检查 | 结果与范围 |
| --- | --- |
| 目录和历史 | 活动同级 Git 仓库共 18 个；旧 backend 路径不存在；中心 HEAD 保持原 backend，旧前端 ref 匹配 |
| 前端 | `npm ci`、`npm run typecheck`、`npm run build` 通过 |
| 统一构建 | `scripts/build-with-frontend.sh -Dmaven.repo.local=/Users/heyoulin/.m2/repository` 通过，产物为 `target/kura-management-console-0.1.0-SNAPSHOT.jar` |
| 产物检查 | jar 内含 `BOOT-INF/classes/static/index.html` 及构建 JS/CSS |
| 运行冒烟 | 独立本机 `127.0.0.1:18081`、内存 H2、不可达测试 broker；17/17 HTTP 检查通过，进程已停止 |
| 页面/静态资源 | `/`、登录、网关/命令列表与详情可直接访问，JS 资源可读取 |
| API 与登录 | 未登录 401；错误密码 401；正确登录、授权网关查询和退出通过，退出后旧 token 拒绝 |
| 路由隔离 | 已认证未知 API 保持 404；缺失静态资源 404，不错误返回 SPA HTML |
| 测试边界 | 原 Java 中心没有测试源码，Maven 报告 `No tests to run`；本次不宣称单元测试套件已通过 |
| 旧依赖问题 | `npm audit`：15 项（3 moderate、12 high）；Vite 报大 chunk 提示，已列入完整方案的迁移基线 |

HTTP health 可能早于开发用户初始化完成；冒烟等待用户初始化后才验证登录，没有为测试修改生产就绪行为。
HTTP 冒烟不等于浏览器交互、IDEA 完整 Kura 启动、Rust/React、新协议或 Linux/网关验收。
这些新架构验收按完整方案的阶段顺序实施，本次未运行 VM 或修改系统网络/防火墙。

本机详细输出位于 `target/consolidation-smoke.json`、`target/consolidation-smoke.log` 和
`target/frontend-audit-20261031.json`；`target` 是可重建且不提交的输出，关键结论记录在本文件。
