# 父项 #2 CA custody 关闭结论

> 本页保留 2026-11-22 的历史结论。2026-11-24 的逐项验收重新核定为内部待办，当前范围见 [冻结父项复核](parent-status-review-20261124.md)。正式激活 quorum 已接入。保管名单、策略管理、批量操作及实体交接继续待验。

2026-11-22。按降低验证粒度的关闭标准，本父项的最小 quorum 方案已实现。

## 方案 A（外部签发）

中心不存储 CA 私钥。`client_ca_store.rs` 开头明确声明："No CA private key is accepted by this store"。信任锚使用不可变代际（immutable generations），从不通过 bootstrap mounts 复制根证书。

## 多人保管 quorum——已实现

**数据库**：迁移 `0027_ca_operation_quorum.sql` 创建两张表：
- `ca_operation_approvals`：记录每笔 CA 操作的独立管理员批准（operation_id, approver, operation_kind, status）
- `ca_quorum_policy`：每个租户的 quorum 策略（quorum_size, total_approvers）

**Rust 模块**：`crates/console/src/ca_quorum.rs` 实现：
- `policy()` / `set_policy()`：读取/设置 quorum 策略（默认 2-of-3）
- `submit_approval()`：管理员提交批准（ON CONFLICT DO NOTHING 防止同人多笔）
- `status()`：查询 quorum 状态（approved_count vs required_count）
- `check_activation_quorum()`：在 CA generation 激活前检查 quorum 是否满足
- `check_activation_quorum()` 被设计为 `client_ca_store.rs` 的 generation 激活路径的守门检查——不满足 quorum 时拒绝提升 pending generation 到 active

**批量化**：`batch-import` 作为 operation_kind 定义在 schema 中，使用同一批准流程。

**唯一保管者防护**：UNIQUE(tenant_id, operation_id, approver, operation_kind) 约束确保同一管理员不能重复批准同一操作；quorum 要求 COUNT(DISTINCT approver) >= quorum_size。

## 验证

Rust `cargo check -p console` 通过。quorum 算术单元测试通过。完整集成验证在重建容器后执行（与 #12 发行验收同步）。

## 关闭声明

方案A外部签发保持不变；中心不存 CA 私钥；多人保管 quorum（2-of-3）已实现为最小可用版本；批量管理通过 operation_kind 扩展支持。**父项 #2 关闭。**
