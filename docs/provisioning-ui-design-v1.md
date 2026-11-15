# 管理中心/网关代理 UI 化配置设计规划 v1

> 后续审查与用户新要求：见 [UI 与组织 CA 托管评估](provisioning-ui-security-review.md)。
> 本文 P1/P2/P3 的历史“已交付”不代表完整界面化；独立 agent 的导入、CSR、
> 远程/能力开关仍有必需 CLI/env 步骤，需继续实施。CA 归属正在按单人保管风险重评，
> 未自动启用 CA 托管签发。热生效、批量登记等工程工作保留内部 pending。

2026-11-09。目标：**部署与配置全部走 Web 控制台界面，去掉必需的命令行步骤**；证书可由脚本预生成后经界面上传。

## 1. 现状盘点（已有能力，不重复造）

| 能力 | 载体 | 状态 |
| --- | --- | --- |
| 网关叶证书登记（管理员粘贴 PEM + 指纹核对 + 双语界面 + 审计/事件事务化） | React 控制台 RegisterGateway + `POST /api/v1/gateways` | ✅ 已交付（gateway-registration-v1，40feee51 台账） |
| 装有 Kura 的网关身份交付（网关内 Java 生成私钥/CSR → 中心 GWT 界面暂存 endpoint/client-chain/issuer-CA/console-CA → agent 首次绑定；私钥不出网关） | 原 GWT Settings→RemoteManagementTabUi + RemoteIdentityStore.stageCertificates | ✅ 已交付并真机验证（b5bdfd26/40feee51 台账） |
| 独立 agent 身份导入（无 Kura 场景） | 网关侧 CLI `setup.py import-identity` + 中心 CLI `register-gateway` | ⚠️ 可用但需命令行，**本轮 UI 化对象** |
| 中心自身 TLS（server 证书/client CA）与任务密钥生命周期 | env + CLI（task-key-backup/rotate 等） | ⚠️ 仅命令行，**本轮 UI 化对象** |

## 2. 设计原则

1. **私钥归属分层**：装 Kura 的网关沿用既有 GWT 流程（私钥由 Kura Java 生成、永不出网关）；**独立 agent 新增「中心托管」模式**——中心生成密钥对，私钥只在身份包下载中出现一次，下载即标记已交付，界面不再可见；后续再补 CSR 模式（网关自生成，中心只签发/登记）。
2. **证书先脚本生成**：`tls-gen.sh` 预生成 CA/server/gw-001..003，UI 只做上传、登记、核对指纹、下发；UI 不做签发逻辑（P2 起中心托管生成除外）。
3. **所有写操作沿用既有闸门**：admin 角色 + Origin/CSRF + 首改密码后台 + 审计/事件同一事务；回执未知不自动重试。
4. **CLI 保留为后备**，UI 为一等入口；两者共用同一校验规则与审计。

## 3. 中心控制台 UI 设计（React 新增「设置」域）

### 3.1 设置页（新增 `features/settings/`，admin 专属，侧栏新入口）

| 区块 | 内容 | 后端 |
| --- | --- | --- |
| 服务器证书 | 当前指纹/NotAfter 展示；上传 server.pem+key（PEM 校验、指纹显示、确认对话框）；提示「保存后需重启容器生效」 | `GET/PUT /api/v1/settings/tls`（写入数据目录新文件 + 待生效标记） |
| 客户端 CA（网关信任锚） | 当前指纹；上传/替换；替换前列出受影响已登记网关数 | `GET/PUT /api/v1/settings/client-ca` |
| 任务数据密钥 | 状态展示；备份（下载 passphrase 加密包）/恢复（上传）/轮换（双确认） | 包装既有 CLI 逻辑为内部端点（复用 task-key 生命周期，规则不变） |
| 远程管理开关 | 各网关远程通道 enabled 状态总览入口 | 只读跳转，不新增写面 |

安全要点：上传走 multipart（有界 64KB/文件）、PEM 结构+指纹双校验、替换需二次确认输入新指纹尾 4 位、审计记录操作者/新旧指纹；**私钥上传后立即落数据目录 0600 并从响应丢弃**。

### 3.2 网关注册向导（增强既有 RegisterGateway）

在既有「粘贴证书 PEM 登记」之上，新增第二种模式：

**中心托管身份包**（面向独立 agent）：

1. 输入网关 ID/名称 → 选择「由中心生成密钥对」
2. 中心生成 keypair、自签/CA 签发 client 证书、登记（复用 register 规则与审计）
3. 返回一次性**身份包下载**：`<id>-bundle.tar.gz`（ca.pem / agent.pem / agent.key / agent.env 预填 endpoint+gateway-id / manifest.json 含全部指纹）
4. 界面标注「私钥仅此一次可下载；关闭页面后需吊销重发」，登记表记录 `bundleIssuedAt`，吊销网关即拒绝该包导入

### 3.3 网关详情页增强

- 证书区：显示当前 client 证书指纹/NotAfter；**轮换**按钮（生成新包或上传新 PEM，走既有 rotate 语义：同代次、中心在线窗口登记，agent 侧 `rotate-client-certificate` 消费）
- 远程通道开关：启停（既有 `set-gateway-enabled` 语义已有 UI 之外的 CLI，此处 UI 化）

## 4. 网关侧简化（agent，下一 deb 迭代）

1. `setup.py import-bundle /tmp/<id>-bundle.tar.gz`：一条命令完成身份导入（解包→校验 manifest 指纹链→写入 `/etc` 0600→渲染 agent.env），替代手工四文件 import-identity；bundle 不匹配 manifest 即拒绝并保留现场。
2. env 缺省值由 bundle 的 `agent.env` 片段提供（endpoint/gateway-id/generation），其余开关仍手工加。
3. 不新增网络能力：bundle 导入是本地文件操作，无出网请求。

## 5. 中心后端要点

- 新端点全部 admin-only + CSRF + 审计同事务；`/settings/*` 写入先写数据目录临时文件→校验→原子替换→记录待生效标记
- 身份包生成：密钥用 ring 生成（P-256 或 RSA-2048 与 CA 一致）、签发走 CA（tls-gen 的 ca.key 需导入中心数据目录或 UI 上传时一并托管——见 §6 取舍）
- TLS 生效方式：Phase 1 = 界面提示 `docker restart console`；Phase 2 = rustls resigning resolver + 管理接口热替换（无需重启）

## 6. CA 托管取舍（需拍板）

- 方案 A（推荐 P1）：**CA 私钥不进中心**。脚本预生成全部证书，UI 只上传/登记/下发身份包（包内私钥由上传的 gw key 提供）。中心零签发职责，风险最小。
- 方案 B（P2）：CA 导入中心托管，UI「生成网关身份」一键签发。便利性最好，但 CA 私钥进入中心数据目录（0600+审计），泄露面扩大；需增加 CA 口令保护与操作二次审批。
- 两方案 UI 结构相同，仅「生成」按钮是否出现。

## 7. 实施分期

| 期 | 内容 | 交付判据 | 状态 |
| --- | --- | --- | --- |
| P1 | 设置页 TLS/CA 上传（重启生效）+ 身份包下载式注册向导 + agent `import-bundle` + 任务密钥备份/恢复 UI | 全流程浏览器完成：上传证书→注册→下载包→网关一条命令导入→在线，无任何 CLI | ✅ 2026-11-10 交付并真实 VM 在线闭环 |
| P2 | 客户端证书轮换 UI（中心登记新指纹+一次性轮换包 + agent `rotate-bundle`）+ 任务密钥轮换 UI（在线重加密） | 轮换全流程界面化 | ✅ 2026-11-10 交付（API 全链+UI 实测）。**TLS 热生效 deferred**：tonic `ServerTlsConfig` 静态，agent gRPC 通道热替换需连接级 reload，工程代价大于秒级重启窗口收益；HTTP 面单独热替换意义有限。**CA 托管生成（方案 B）待用户拍板**（P1 已按方案 A 交付，引入 CA 私钥进中心需重新取舍） |
| P3 | CSR 模式对接独立 agent：`kura-agent-setup create-certificate-request`（本地生成密钥+CSR，密钥不出网关）→ 外部 CA 签发 → 中心「仅登记证书」（已有能力）→ `activate-pending-certificate` 绑定。批量注册/导出 **deferred**：单台全浏览器流程已 ≤2 分钟，批量收益低，且与「不做无确认批量导入」约束纠缠 | 私钥不出网关的可选路径可用 | ✅ CSR 模式 2026-11-11 交付（agent 67 测试含 3 条 CSR）；批量 deferred |

## 8. 明确不做

- 不在浏览器生成/展示任何 CA 私钥
- 不做自动批量导入（无确认的大规模下发）
- 不替代既有 Kura GWT 身份交付流程（装 Kura 的网关继续走原流程，两套并存）
- 未知回执不自动重试；UNKNOWN 只按原命令核定
