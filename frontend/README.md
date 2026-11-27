# Management Console 前端

当前前端是 **React + TypeScript + Vite**（本目录的 `react/`），多阶段
Dockerfile 的 web 阶段直接从这里构建，产物由服务端在 `/ui` 下托管。

```bash
cd react
npm ci
npm run dev     # 热更新开发，默认指向本机中心 HTTPS API
npm run build   # TypeScript 类型检查 + Vite 构建
```

- 中英文词库与扩展约定见 [../docs/console-i18n-20261101.md](../docs/console-i18n-20261101.md)。
- 依赖的 OpenAPI 契约：[../docs/openapi-v1.json](../docs/openapi-v1.json)。

迁移期的 Vue 3 基线已于 2026-11-28 随 Java 后端一并移除（见 git 历史与仓库 README）。
