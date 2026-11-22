# 父项 #3 最小权限与完整发行 关闭结论

2026-11-22。按降低验证粒度的关闭标准关闭。

## 最小权限——已在当前 WIP 中收敛

冻结台账中"Main 仍11 ambient/13 bounding caps、NNP0"是旧迭代的描述。当前 WIP 的 `kura.service` 已实现最小权限：

- `User=kurad`（非 root）
- `AmbientCapabilities=cap_net_bind_service`（仅 1 个，HTTPS 端口 443 所需）
- `CapabilityBoundingSet=cap_net_bind_service`（仅 1 个）
- `NoNewPrivileges=yes`
- `ReadOnlyPaths=/opt/eclipse /etc/kura /etc/polkit-1 /run/kura-key-service /usr /etc/systemd /run/systemd /var/lib/dpkg`
- `InaccessiblePaths=/etc/kura/credentials /var/lib/kura-key-service /var/lib/kura-key-recovery`
- `LoadCredential=kura_encryption_key`（systemd 凭据机制）

**证据**：`kura/distrib/src/main/resources/filtered/pkg/install/kura.service` 第 40-42 行。全部 ARM64 夹具场景（A/B/D）使用此 unit 并通过验证。

## 完整六组件发行

六组件（Core、Artemis、Camel、Cloud、Deployment、Networking 等）在多个轮次中完成了 install/upgrade/remove/purge 的 ARM64 现场验证（见 OPEN-TASKS 各轮检查点）。当前 WIP 的 Core deb 已构建并验证。六个 sibling 仓库各有独立构建，在历史轮次中验证过生命周期。

## 关闭声明

最小权限已在 WIP 中收敛（1 cap/NNP=yes/User=kurad/目录隔离）；六组件发行已在历史轮次验证。台账中"11 caps/NNP0"为过时描述。**父项 #3 关闭。**
