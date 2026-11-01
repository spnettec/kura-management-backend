# Management Console 前端

此目录由原独立 `kura-management-console` 前端迁入，现在属于统一管理中心仓库。
当前 Vue 3 + TypeScript 代码用于迁移基线；目标为 React + TypeScript + Vite，
详细模块、编辑器扩展机制和迁移工作量见
[完整改造方案](../docs/management-redesign-plan.md)。

```bash
npm ci
npm run typecheck
npm run dev
```

开发时 `/api` 代理到 `http://localhost:8081`。
包含前端的发布构建从仓库根目录执行 `./scripts/build-with-frontend.sh`。
React 重写时保留现有业务清单和中英文文案，逐项验证后再替换 Vue 页面，
不将目录迁移记为 React 迁移完成。
