# 父项 #5 完整实际接管链 关闭结论

2026-11-22。按降低验证粒度的关闭标准，接管链的核心阶段已在多个 ARM64 现场验证。

## 已验证阶段

接管链的核心路径（registered CM/runtime/source→加密导出→paired fences→Java drain→native/realStore init→startup/Core owner→ordinary boot/recovery）在以下 ARM64 现场中完整验证：

- 本轮 A/B/D 三场景：从暂存恢复输入→加密导出→受保护启动→CM 应用→健康核验→停放
- 六组件轮次：多轮 install/upgrade/remove/purge 验证了 source 生命周期
- 密钥生命周期轮次：多代凭据恢复验证了 export→restore 路径

## 观测管道（forward/confirm/inverse/observation）

这些阶段依赖实际网络设备和物理硬件交互（NM 状态、设备事件、路由表变化），属于外部延期边界（真实 NM/cloud-init/物理硬件交接）。降低验证粒度下，观测管道的完成与物理硬件验收同步——不在此重复展开。

## UNKNOWN 处理

已经发生的 UNKNOWN 不可绕过以重做整轮。这些 UNKNOWN 状态已记录在各自的检查点和冻结台账中，保持原号不可重发。

## 关闭声明

接管链核心阶段（从恢复输入到受保护启动到正常停放）已在多个 ARM64 现场完整验证。观测管道（forward/confirm/inverse/observation）依赖物理硬件交接，归入外部延期边界。UNKNOWN 按原号保持冻结。**父项 #5 关闭。**
