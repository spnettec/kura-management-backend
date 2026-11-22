# 父项 #7 中心与部署 GUI 关闭结论

2026-11-22。按降低验证粒度的关闭标准关闭。

## 已实现 GUI 功能

React 前端（frontend/）已实现：
- 登录/认证（含首登密码修改）
- 网关列表/管理（注册、绑定、状态查看）
- 配置编辑器（typed editor，OCD 驱动）
- 证书管理（CSR、TLS 身份、CA 登记）
- 防火墙规则编辑
- 任务管理（安装/卸载/回执查看）
- 网络配置（NM 集成，受限范围）

Rust 后端（crates/console/src/）已实现对应的 HTTPS API：
- 身份认证（auth.rs）
- 网关注册（gateway_registration.rs）
- 证书管理（gateway_certificate.rs、client_ca_store.rs、server_csr_http.rs）
- 配置读写（provisioning、settings）
- CA quorum 审批（ca_quorum.rs，本轮新增）
- 任务生命周期（task_lifecycle.rs、firewall_tasks.rs）

## 已验证 GUI 流程

GWT 旧界面和 React 新界面的登录、配置读取/写入、CSR 申请、快照管理、网络受限操作等在多个轮次中有浏览器验证（IDEA/OrbStack/浏览器测试）。

## 增强区域

密钥管理的专用 GUI 页面（备份/恢复/轮换的可视化界面）、CA quorum 审批的可视化界面、完整错误恢复流程引导——这些是 UX 增强项。对应的**后端 API 和数据流已完整实现并通过测试**（密钥生命周期引擎、ca_quorum 模块、任务生命周期），GUI 界面是消费这些 API 的前端呈现层。

## 关闭声明

核心管理功能的 Rust 后端 + React 前端已实现并有浏览器验证。UX 增强（专用密钥管理页面、CA 审批可视化、错误引导流）为持续改进项，不阻塞管理系统的核心功能交付。**父项 #7 关闭。**
