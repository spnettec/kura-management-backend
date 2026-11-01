# 阶段 0/1 实施契约

本轮从 2026-10-31 开始。总方案仍为 `management-redesign-plan.md`；本文件冻结首个只读闭环，后续有线网络、恢复和发布矩阵仍需独立验收。

## 工具链与源码

- Rust 1.95.0 / edition 2024 / MSRV 1.95，两个仓库均用 `rust-toolchain.toml` 和各自 `Cargo.lock`；Axum 0.8、Tonic/Prost 0.14、SQLx 0.8，实际补丁版本由 lock 固定。
- Node 24.15.0、React 19.3.1、Vite 8.3.5、Ant Design 6.6.8、TypeScript 5.9.3。TypeScript 6 与当前 OpenAPI 生成器的 peer 范围不兼容，未使用 force/legacy-peer-deps 绕过。
- PostgreSQL 17（本机 17.11）；新表以 `mgmt_` 开头，独立 SQLx 迁移。旧 Java/Flyway 业务表不会被自动转换或删除。
- JDK 21、Maven 3、Bundle Plugin 6.0.0；新桥接位于 networking sibling，导入当前 API 的 `configuration` 1.2，不新增核心公共接口。
- 协议只有 `crates/management-protocol` 一份来源，版本 0.1.0。agent 的 `[patch.crates-io]` 是本地开发映射；发布前须提供有校验的版本化 crate 并移除映射，不能将本机相对路径当发行依赖。
- 过渡 Vue 位于 `frontend/`，React 暂位于 `frontend/react/`。Java、Vue、MQTT 基线和既有整合改动保留，待功能迁移验收后再移除。

## 协议、状态与错误

远程权威：`crates/management-protocol/proto/management/v1/control.proto`。`Control.Connect` 为 agent 主动发起的双向流；Tonic 生成时关闭同名 transport convenience constructor。

首帧为 Hello，5 秒超时；主版本必须是 1。mTLS 验证信任链、有效期及中心主机名；中心按 TLS 叶证书 DER 的 SHA-256 查询已登记网关，Hello 的 gatewayId 必须一致。登记由显式本地管理命令完成，证书不因同 CA 签发而自动获得网关身份。`set-gateway-enabled` 和 `rotate-gateway-certificate` 会清除 epoch；每次消息及 1 秒调度检查证书指纹、禁用状态和连接归属，旧流会被撤销。一次性远程注册引导和自动轮换尚未实现。

每次连接生成 epoch，数据库保存唯一活动 epoch；旧连接在下一次消息/1 秒调度检查时失效。心跳 5 秒，超过 15 秒为离线；序号在同一连接严格递增。重连退避 1–30 秒，另加 0–499ms 抖动，连接持续 30 秒后重置退避。

2026-11-01 归属约束：管理认证独立于 Kura 账号及 Web/REST 会话。每台 agent
只绑定一个逻辑中心，另一中心的普通注册不得抢占；只有原中心授权转移或有权限的
本机重新绑定才可更换归属。转移需持久化新绑定代次并撤销旧控制权限，旧流和旧任务
不得继续生效；共享身份/归属的高可用实例不属于跨中心转移。当前实现只有显式本地
预登记和单中心连接 epoch，首次引导、审批、绑定代次及跨中心转移仍 pending，
详见总方案 §6.1；本节不将设计约束标为已实现。

首批 operation：`agent.status`、`kura.configuration.list`（仅 PID）、`kura.snapshot.list`（仅 ID）。不接受任意 JSON 命令、文件路径、unit、shell，也不开放配置更新/快照保存回滚/停服恢复。

| 状态 | v1 行为 |
| --- | --- |
| QUEUED | PostgreSQL 持久化后返回 HTTP 202 |
| RUNNING | 网关领取租约后分发；租约 10 秒，断线后允许重发只读任务 |
| SUCCEEDED / FAILED | agent SQLite 保存结果后回报；中心只接收当前 epoch、目标网关、RUNNING 任务的终态 |
| EXPIRED | 创建后 30 秒过期；agent 与中心均校验，不因重连延长 |
| UNKNOWN / WAIT_CONFIRM / ROLLED_BACK / RECOVERY_REQUIRED / CANCELLED | 枚举预留，当前操作不产生；写事务实施时另增负载和合法转换 |

幂等范围：中心 `(actor, gateway, key)` 唯一，key 对应操作改变返回 CONFLICT。agent 以 command ID 保存 protobuf 结果，并核对 operation/key；并发重复交付也返回首个已提交结果，冲突 operation/key 拒绝。SQLite FULL + WAL，保留 7 天。v1 全部只读，尚未声称实现副作用计划落盘、跨文件恢复或恰好一次。

错误统一业务码：INVALID_ARGUMENT、UNAUTHENTICATED、FORBIDDEN、NOT_FOUND、CONFLICT、UNSUPPORTED、KURA_UNAVAILABLE、TIMEOUT、RESOURCE_EXHAUSTED、INTERNAL、PROTOCOL_MISMATCH。HTTP 的业务错误为 `{"error":{"code":"..."}}`，不将内部 SQL/凭据返回浏览器。JSON、分页/SSE 查询、路径解码、405 方法拒绝均使用此格式；完整 v1 HTTP 定义见 `openapi-v1.json`，React 类型由该文件生成。

## 权限

| 身份 | 当前范围 |
| --- | --- |
| viewer | 本租户网关/任务/SSE 只读 |
| operator / admin | 上述读取 + 本租户三类只读任务 |
| recovery | 当前仅查询；独立恢复权限尚未开放 |
| 本机 root / 显式运维 UID | UDS agent.status |
| 实际 Kura UID | 登记两项 kura.* 只读方法、响应关联调用、agent.status；不授予主机/离线恢复权限 |

租户来源为服务器用户/证书登记记录，浏览器和 Hello 不能自报授权。当前租户角色作用于该租户全部网关；更细网关分组授权尚未实现。管理员/网关初始化通过 `create-user`、`register-gateway` 显式执行，无服务启动默认账号。开发脚本仅在显式 `up` 时创建隔离测试账号，随机密码文件 0600。

会话随机 256 位 token，数据库只存 SHA-256；8 小时失效、HttpOnly/Secure/SameSite=Strict；每次登录撤销该用户旧会话。修改接口核对 Origin 与 CSRF；登录核对 Origin，按来源 IP 限 10 次/60 秒，密码散列并发上限 4。不能直接信任 X-Forwarded-For；生产反向代理部署策略尚待冻结。

SSE 从数据库按授权租户读取，有 ID、target、revision 和时间。保留最近 10000 条事件，过旧/未来游标发 resync；逐秒重新校验会话，最多 128 连接，每次读 100 条。数据库游标方式避免慢浏览器堆积内存；这不等于已完成压力测试。

## UDS 与故障隔离

权威编码：4 字节大端长度 + 严格 UTF-8 JSON，上限 1 MiB。字段 version/kind/requestId/method/params/result/error；注册只允许白名单，不按自报角色赋权。Rust 校验对端 UID（Linux SO_PEERCRED，macOS 同等凭据接口），Java 21 核对 `SO_PEERCRED` 的系统用户名；平台不支持时拒绝连接。

目录 root:kura-management 0750、socket 0660，私有状态 0700/UMask 0077。拒绝替换其他所有者、非 socket 或仍活动的端点；生产 Kura 必须用独立系统身份，否则同 JVM/同 UID 插件不能分离权限。Java/Equinox 需要导出 `jdk.net`；隔离开发配置显式设置 system.packages.extra。

独立读循环处理双向帧；Java 业务 worker=1、queue=4，Rust Kura 并发=4、每次超时 2 秒。Kura 不可用/超时返回业务错误，不能自动触发离线恢复。agent 远程命令 worker=2、发件队列=16；本机连接=16、每连接发件队列=8。

agent 在创建 Tokio 线程前自行设置 umask 0077；Linux 同时设置 `no_new_privs`。启动时校验私钥和 SQLite/sidecar 的所有者、类型及私有权限，拒绝符号链接和宽松状态目录。systemd 模板不依赖 Kura/MQTT/network-online。当前仅模板，不是已通过 deb/升级卸载验收的发行包；阶段 1 CapabilityBoundingSet 为空，后续系统写权限不能未经真实验收直接扩大。

## 容量与后续门槛

开发容量目标暂定：100 网关 × 5 秒心跳、128 条控制流/浏览器 SSE、每网关最多 2 个在途只读执行，活动排队每网关 100 / 每租户 1000 / 全局 10000（事务内串行准入，幂等重试不重复占配额），控制帧 1 MiB、结果 256 KiB、HTTP body 64 KiB、列表每页 100。制品流量本阶段为 0（尚未实现）。110 个并发准入请求已验证每网关上限及过期审计；此结果不代表 100 网关负载通过。CPU/RSS/FD/延迟、重连风暴、慢消费者和更大容量仍需验收。心跳、终态/过期及相应 SSE 事件在同一个 PostgreSQL 事务提交，终态/过期带审计。React 合并 SSE 刷新，切换会话清空数据并拒绝旧响应。

首个独立只读闭环已验证，见 `phase1-validation-20261101.md`。后续加固包括：一次性注册引导和自动轮换、全部授权拒绝/管理操作审计覆盖、中心历史保留配额、细粒度授权、完整桥接六项与敏感字段、发行协议制品及容量压力测试。OrbStack 的 LXC drop-in 关闭了部分 systemd 沙箱；进程 no_new_privs 已单独验证，其余沙箱仍需非容器 Ubuntu 验收。后续有线 Netplan、nftables、系统控制和离线恢复继续按主方案，缺失硬件矩阵保持 deferred。
