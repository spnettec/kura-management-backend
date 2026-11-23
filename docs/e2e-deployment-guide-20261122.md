# YOFC Kura 管理体系——全链部署操作手册

2026-11-22。本文档描述从零开始部署完整 YOFC Kura 管理体系的完整步骤，包括中心管理平台（Docker 容器）和 3 台网关设备（OrbStack Linux machine）。

## 架构概览

```
┌───────────────────────────────┐
│   管理中心 (Docker 容器)        │
│   kura-management-console deb │
│   + PostgreSQL 16             │
│   + React Web UI              │
│   端口: 8443(HTTPS) 9443(gRPC) │
└──────────┬────────────────────┘
           │ mTLS / gRPC
           │ (agent 主动连接)
┌──────────┼────────────────────┐
│  ┌───────┴──────┐ ┌──────────┐ │
│  │  网关 1      │ │  网关 2  │ │ ...
│  │  kura-core   │ │  ...     │ │
│  │  + agent     │ │          │ │
│  │  (OrbStack VM)│ │          │ │
│  └──────────────┘ └──────────┘ │
└───────────────────────────────┘
```

## 前置条件

- macOS + OrbStack（含 Docker 和 Linux VM 支持）
- JDK 21（Maven 构建 kura-core deb）
- 已构建的 deb 包：
  - `kura-management-console` deb（管理控制台）
  - `kura-core` deb（网关运行时）
  - `kura-management-agent` deb（网关代理）

## 第一阶段：部署管理中心

### 1.1 创建 Docker 网络

```bash
docker network create yofc-e2e
```

### 1.2 启动 PostgreSQL

```bash
docker run -d --name yofc-e2e-pg --network yofc-e2e \
  -e POSTGRES_USER=console \
  -e POSTGRES_PASSWORD=console123 \
  -e POSTGRES_DB=console \
  --platform linux/arm64 \
  postgres:16-alpine
```

### 1.3 构建 Console 镜像

Console deb 需要 systemd 但容器没有。用手动提取方式：

```dockerfile
FROM debian:trixie-slim
RUN apt-get update && apt-get install -y \
    libssl3 ca-certificates openssl python3 binutils zstd \
    && rm -rf /var/lib/apt/lists/*
COPY console.deb /tmp/console.deb
RUN mkdir -p /tmp/extract && cd /tmp/extract && \
    ar x /tmp/console.deb && zstd -d data.tar.zst && tar xf data.tar && \
    mkdir -p /usr/share/kura-management-console && \
    cp -a usr/share/kura-management-console/* /usr/share/kura-management-console/ && \
    cp -a usr/bin/yofc-management-console /usr/bin/ && \
    cp -a usr/lib/kura-management-console/* /usr/lib/kura-management-console/ 2>/dev/null; \
    chmod +x /usr/bin/yofc-management-console && \
    rm -rf /tmp/extract /tmp/console.deb
COPY frontend-dist /usr/share/kura-management-console/react/
COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh
EXPOSE 8443 9443
ENTRYPOINT ["/entrypoint.sh"]
CMD ["serve"]
```

**关键文件：**
- `console.deb`：kura-management-console deb 包
- `frontend-dist/`：React 前端构建产物（frontend/dist/）
- `entrypoint.sh`：初始化脚本

**entrypoint.sh（完整）：**

```bash
#!/bin/sh
set -e
STATE=/var/lib/kura-management-console
MAINT=/var/lib/yofc-console-maintenance
mkdir -p "$STATE" "$STATE/.lifecycle" "$MAINT"
chmod 700 "$STATE" "$MAINT"
KEY=/etc/kura-management-console/data.key
if [ ! -s "$KEY" ] || [ "$(wc -c < "$KEY")" -ne 32 ]; then
  openssl rand -out "$KEY" 32
fi
chmod 600 "$KEY"
chmod 700 /etc/kura-management-console
chmod 700 /etc/kura-management-console/tls
chmod 600 /etc/kura-management-console/tls/*.key 2>/dev/null || true
touch "$MAINT/writers.lock"
chmod 600 "$MAINT/writers.lock"
chown root:root "$MAINT/writers.lock"
exec /usr/bin/yofc-management-console "$@"
```

**TLS 证书（Dockerfile 内生成）：**

```dockerfile
RUN mkdir -p /etc/kura-management-console/tls && \
    openssl req -x509 -newkey rsa:2048 -nodes -days 365 \
    -keyout /etc/kura-management-console/tls/console.key \
    -out /etc/kura-management-console/tls/console.crt \
    -subj "/CN=console.yofc-e2e.local" \
    -addext "subjectAltName=DNS:localhost,IP:127.0.0.1" \
    -addext "keyUsage=digitalSignature,keyEncipherment" \
    -addext "extendedKeyUsage=serverAuth" \
    -addext "basicConstraints=critical,CA:FALSE" && \
    openssl req -x509 -newkey rsa:2048 -nodes -days 365 \
    -keyout /etc/kura-management-console/tls/client-ca.key \
    -out /etc/kura-management-console/tls/client-ca.crt \
    -subj "/CN=yofc-e2e-client-ca" \
    -addext "basicConstraints=critical,CA:TRUE" \
    -addext "keyUsage=critical,keyCertSign,cRLSign"
```

**数据密钥：** 32 字节原始二进制（不是 hex 文本）

```dockerfile
RUN openssl rand -out /etc/kura-management-console/data.key 32 && \
    chmod 600 /etc/kura-management-console/data.key
```

### 1.4 Console 环境变量

创建 `console-docker.env`：

```
CONSOLE_ORIGIN=https://localhost:8443
CONSOLE_HTTP_ADDR=0.0.0.0:8443
CONSOLE_GRPC_ADDR=0.0.0.0:9443
CONSOLE_TLS_CERT=/etc/kura-management-console/tls/console.crt
CONSOLE_TLS_KEY=/etc/kura-management-console/tls/console.key
CONSOLE_CLIENT_CA=/etc/kura-management-console/tls/client-ca.crt
CONSOLE_WEB_DIR=/usr/share/kura-management-console/react
CONSOLE_STATE_DIR=/var/lib/kura-management-console
CONSOLE_DATA_KEY_FILE=/etc/kura-management-console/data.key
DATABASE_URL=postgres://console:console123@yofc-e2e-pg:5432/console
```

### 1.5 启动 Console

```bash
docker build --platform linux/arm64 -f Dockerfile.console -t yofc-console-e2e .
docker run -d --name yofc-e2e-console --network yofc-e2e \
  -p 8443:8443 -p 9443:9443 \
  --env-file console-docker.env \
  yofc-console-e2e
```

验证：`curl -sk https://localhost:8443/` 返回 HTTP 200。

## 第二阶段：部署网关（3 台 OrbStack machine）

### 2.1 创建 machine

```bash
for i in 1 2 3; do
  orb create debian:trixie yofc-gw-$i
done
```

### 2.2 安装系统依赖

```bash
for i in 1 2 3; do
  orb run -m yofc-gw-$i sudo apt-get update
  orb run -m yofc-gw-$i sudo apt-get install -y python3 libssl3 gpsd
done
```

### 2.3 安装 kura-management-agent

```bash
orb run -m yofc-gw-1 sudo dpkg -i kura-management-agent_..._arm64.deb
```

输出 `Native payload configured` 表示成功。**注意：agent 必须先于 kura-core 安装**（agent 的 preinst 会创建 kura 用户和 systemd units）。

### 2.4 安装 kura-core

```bash
orb run -m yofc-gw-1 sudo dpkg -i kura_6.0.0-..._arm64.deb
```

如果报依赖错误，运行 `apt-get install -f` 补齐。

**安装顺序要求：**
- **agent 先装 → kura-core 后装**（推荐，因为 agent 的初始化不依赖 kura 的凭据状态）
- 如果 agent 先装，kura-core 的 preinst 会发现 agent 已创建 key-service drop-in 目录但会接受（修复 e886e82f12）
- 如果 kura-core 先装，agent 后装也可以

### 2.5 初始化 agent

```bash
# request-id 必须是 UUID 格式
REQUEST_ID=$(python3 -c "import uuid;print(uuid.uuid4())")
sudo python3 /usr/lib/kura-management-agent/runtime_setup.py initialize --request-id $REQUEST_ID
```

输出 `{"initialized": true, "created": true}` 表示成功。

### 2.6 启动 agent

```bash
sudo systemctl enable --now kura-management-agent-v2.service
```

验证 `systemctl is-active` 返回 `active`。

## 第三阶段：网关注册到管理中心

### 3.1 生成网关身份证书

在每台网关上生成带完整扩展的自签证书：

```bash
openssl req -newkey rsa:2048 -nodes \
  -keyout /etc/kura-management-agent-identity/gateway.key \
  -x509 -days 365 -out /etc/kura-management-agent-identity/gateway.crt \
  -subj "/CN=yofc-gw-1/O=YOFC" \
  -addext "subjectKeyIdentifier=hash" \
  -addext "authorityKeyIdentifier=keyid,issuer" \
  -addext "keyUsage=digitalSignature,keyEncipherment" \
  -addext "extendedKeyUsage=serverAuth,clientAuth" \
  -addext "basicConstraints=critical,CA:FALSE"
```

**必须包含的扩展：**
- `basicConstraints=critical,CA:FALSE`（console 拒绝 CA 证书作为网关身份）
- `keyUsage` 和 `extendedKeyUsage`
- `subjectKeyIdentifier` 和 `authorityKeyIdentifier`

### 3.2 在 Console 注册网关（API 调用）

```bash
# 登录获取 session
curl -sk -c cookies.txt -X POST https://localhost:8443/api/v1/auth/login \
  -H "Content-Type: application/json" -H "Origin: https://localhost:8443" \
  -d '{"username":"admin","password":"admin"}'

# 首次登录修改密码
curl -sk -b cookies.txt -X POST https://localhost:8443/api/v1/auth/password \
  -H "Content-Type: application/json" -H "Origin: https://localhost:8443" \
  -H "X-CSRF-Token: <csrf-from-login>" \
  -d '{"currentPassword":"admin","newPassword":"<new-password>"}'

# 注册网关（带私钥获取 identity bundle token）
curl -sk -b cookies.txt -X POST https://localhost:8443/api/v1/gateways \
  -H "Content-Type: application/json" -H "Origin: https://localhost:8443" \
  -H "X-CSRF-Token: <csrf>" \
  -d '{
    "id": "yofc-gw-1",
    "name": "YOFC Gateway 1",
    "clientCertificatePem": "<gateway-cert-pem>",
    "clientKeyPem": "<gateway-key-pem>"
  }'
```

返回中的 `identityBundleToken` 用于下一步下载。

### 3.3 下载 Identity Bundle

```bash
curl -sk -b cookies.txt \
  "https://localhost:8443/api/v1/gateways/yofc-gw-1/identity-bundle?token=<token>&endpoint=https://<console-ip>:9443" \
  -o identity-bundle.tar
```

Bundle 是 tar 包，含 `agent.key`、`agent.pem`、`ca.pem`、`manifest.json`。

### 3.4 在网关上部署 Enrollment 材料

```bash
sudo mkdir -p /var/lib/kura-management-agent-runtime/enrollment
sudo tar xf identity-bundle.tar -C /var/lib/kura-management-agent-runtime/enrollment/
sudo chmod 700 /var/lib/kura-management-agent-runtime/enrollment
```

同时生成 `identity.json`（enrollment 回执）和 `private.pk8`（PKCS8 私钥）。

### 3.5 Kura Peer 配对

```bash
sudo systemctl stop kura-management-agent-v2.service kura.service
sudo python3 /usr/lib/kura-management-agent/runtime_setup.py authorize-kura-peer \
  --request-id <uuid>
sudo systemctl start kura-management-agent-v2.service
```

配对完成后 agent.env 中的 `AGENT_KURA_UID` 从 `4294967295` 变为 kurad 的实际 UID。

### 3.6 配置 Transport Endpoint

```bash
sudo systemctl stop kura-management-agent-v2.service
sudo python3 /usr/lib/kura-management-agent/runtime_setup.py configure-native-transport-endpoint \
  --request-id <uuid> \
  --initialization-id <initialization-uuid> \
  --endpoint https://<console-ip>:9443
sudo systemctl start kura-management-agent-v2.service
```

## 第四阶段：Web UI 测试

浏览器访问 `https://localhost:8443/`（需要忽略自签证书警告）。

### 截图清单

| 截图 | 页面 | 说明 |
|---|---|---|
| 01-login-page.png | 登录页 | React SPA 初始页面 |
| 02-console-home.png | 管理首页 | 需登录 |
| 03-gateway-list.png | 网关列表 | 需登录 |
| 04-settings-tls.png | TLS 设置 | 需登录 |

Chrome headless 截图命令：

```bash
"/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" \
  --headless=new --no-sandbox --disable-gpu --ignore-certificate-errors \
  --screenshot=<output.png> --window-size=1280,800 \
  "https://localhost:8443/#/gateways"
```

## 故障排除

| 问题 | 原因 | 解决 |
|---|---|---|
| console 报 "no retained key matches database" | PG 数据与 data.key 不匹配 | 重置 PG：`DROP SCHEMA public CASCADE; CREATE SCHEMA public;` |
| console 报 "No such file or directory" | writers.lock 缺失 | entrypoint 中 `touch /var/lib/yofc-console-maintenance/writers.lock` |
| console 报 "TLS state directory must be owned and private" | /etc/kura-management-console/tls 权限 | `chmod 700` 目录 + `chmod 600` key |
| kura-core preinst "Key-service package admission refused" | agent 先装创建了 drop-in | 需要修复版本 e886e82f12+ 的 preinst |
| agent initialize "ValueError" | request-id 不是 UUID | 用 `uuid.uuid4()` 生成 |
| agent initialize "Effective non-Root agent profile differs" | agent unit 有 LXC drop-in | 需要修复 54544d7+ 的 runtime_setup.py |
| authorize-kura-peer "Unreviewed effective Core unit" | kura.service 有 LXC drop-in + fields 比较含 DropInPaths 键 | 需要修复 ac88f23+ 的 runtime_setup.py |
| configure-transport "FileNotFoundError" | enrollment 材料不完整 | 需先完成 identity bundle 部署（3.4） |
| docker build "invalid reference format" | --pull=never 与非本地镜像冲突 | 去掉 --pull=never 或确保镜像在本地 |
| core preinst 读 `/etc/kura/clock-broker-account.json` 报 FileNotFoundError | 历史 fixture 的 `/var/lib/kura-clock-broker` 或 `/usr/libexec/kura-clock-broker` 残留 | 归档后删除（kura-clock 账号一并清理） |
| configure 卡 "Installer step brokers is unresolved" | `--configure-brokers` 中途失败（intent 已签发未回收），resume 按设计不重放 | 清除诱因后原步不可恢复；remove+purge+重装（新 fence） |
| 半配置 kura-core 无法 dpkg --remove | prerm 的 clock guard 在 unit 未发布时 quiesce 失败 | 恢复路径缺陷（已记录），需按合同手工补齐状态后 remove |
| enrollment INVALID_ARGUMENT（openssl 均成功） | Mac LibreSSL 生成的 EC 为显式参数编码（无 ASN1 OID 行） | 全部 EC 材料改用 Linux OpenSSL 生成 |
| enrollment INVALID_ARGUMENT（早期退出） | private.pk8 是传统 EC 格式而非 PKCS#8 | `openssl pkcs8 -topk8 -nocrypt` 重制 |
| "Invalid retained endpoint or certificate expiry" | manifest.notAfter 用了空格分隔 | 按 `-dateopt iso_8601` 输出把空格替换为 T |
| bind 后巡检 "Unreviewed retained identity receipt" | bind 由 root 发起（actor=local-uid:0） | 复位身份后以 kurad register 握手重新 bind |
| remote-switch "Explicitly stop..." | 还需 kura.service 与 enrollment/material sockets 停止 | 全部停止后执行（或改走 enrollment 前的正序） |
| console "joint TLS material changed" | 手工替换容器内 /etc 证书 | 走 server-csr 受支持轮换（5.2） |

## 当前部署状态

| 组件 | 部署方式 | 状态 |
|---|---|---|
| 管理中心 (yofc-e2e-console) | Docker 容器 | ✅ 运行 |
| PostgreSQL (yofc-e2e-pg) | Docker 容器 | ✅ 运行 |
| 网关 1 (yofc-gw-1) | OrbStack machine | ✅ core ii + agent ii + agent active |
| 网关 2 (yofc-gw-2) | OrbStack machine | ✅ 同上 |
| 网关 3 (yofc-gw-3) | OrbStack machine | ✅ 同上 |
| 网关注册 | console API | ✅ 3/3 + yofc-vm-1 注册成功 |
| Identity Bundle | console API → 网关 | ✅ 已部署 |
| Kura Peer 配对 | runtime_setup | ✅ 3/3 + VM APPLIED |
| Enrollment 完整流程 | Parallels VM（完整 systemd） | ✅ 2026-11-23 打通（见第五阶段） |
| 网关在线连接 | gRPC mTLS | ✅ yofc-vm-1 online=true；OrbStack 3 台待 LoadCredential 兼容（见缺陷任务） |

## 第五阶段：Enrollment 完整流程（Parallels VM，2026-11-23 已验证）

### 5.0 环境前提

- 完整 systemd VM（Parallels/物理机），kura.service + agent 双 active
- console 可达地址（如 `https://10.211.55.2:9443`），console 服务端证书 SAN 必须包含该地址
- VM 已清理历史 fixture（否则 clock 账号/key-service 准入拒绝；归档后删除，见故障排除）

### 5.1 网关密钥与证书（必须在 Linux 上生成）

```bash
# P-256 密钥（PKCS#8！openssl pkey 输出的是传统 EC 格式，agent 会拒绝）
openssl genpkey -algorithm EC -pkeyopt ec_paramgen_curve:P-256 -out gw.key
openssl pkcs8 -topk8 -nocrypt -in gw.key -outform DER -out gw.pk8
openssl pkey -in gw.key -pubout -outform DER -out gw.spki.der
openssl req -new -key gw.key -subj "/CN=yofc-vm-1" -out gw.csr
```

**网关 leaf 必须由 console 的 client CA 签发**（gRPC 服务端强制 `WebPkiClientVerifier`，
自签证书无法通过握手；console 不签发 CSR，由操作者用 client CA 私钥签）：

```bash
# client-ca 在 console 容器 /etc/kura-management-console/tls/client-ca.{crt,key}
printf 'basicConstraints=critical,CA:FALSE\nkeyUsage=critical,digitalSignature\nextendedKeyUsage=clientAuth\n' > leaf.ext
openssl x509 -req -in gw.csr -CA client-ca.crt -CAkey client-ca.key -CAcreateserial \
  -out gw-leaf.crt -days 3650 -extfile leaf.ext -sha256
```

在 console 注册该 leaf（或用 rotate 换掉已注册的自签证书）：

```
POST /api/v1/gateways            {id,name,clientCertificatePem,clientKeyPem}
POST /api/v1/gateways/{id}/certificate   {previousCertificateSha256,clientCertificatePem}
```

### 5.2 console 服务端证书轮换（如需新增 SAN，如 VM IP）

走受支持的 server-csr 流程（直接改容器内证书会触发 joint TLS 材料校验拒绝）：

```
POST /api/v1/settings/server-csr        {origin, requestId}   → csrPem
（用 Linux OpenSSL 生成 CA 并签 CSR，SAN 加 IP:10.211.55.2 等）
PUT  /api/v1/settings/server-csr/{id}/certificate  {serverPem}
POST /api/v1/settings/server-csr/{id}/stage
GET  /api/v1/settings/tls-activation    → generation + pendingRevision
POST /api/v1/settings/tls-activation    {operationId,expectedGeneration,pendingRevision}
```

**注意：签发 CA/证书必须用 Linux OpenSSL 生成。** macOS LibreSSL 的 EC 产物是
X9.62 显式参数编码（无 namedCurve OID），agent 的公钥强度检查要求 `ASN1 OID:`
行，会以 `INVALID_ARGUMENT` 拒绝。

### 5.3 Kura 侧 staging（本地控制器角色）

在 `/opt/eclipse/kura_*/data/` 下构建（全部 kurad:kurad，目录 0700 文件 0600）：

```
management-enrollment-context-v1      # 内容："yofc.kura.remote-request-context.v1\n<uuid4>\n"，0600
management-enrollment/.enrollment.lock    # 空文件，0600
management-enrollment/<requestId>/        # requestId = 小写 uuid4
    private.pk8        # PKCS#8 DER
    public.der         # SPKI DER（91 字节 P-256）
    request.pem        # CSR PEM，subject=CN=<gateway-id>
    gateway.id         # <gateway-id>（无换行）
    complete           # "kura.remote.identity.v1\n"
    certificates/
        client-chain.pem    # leaf（由 console client CA 签发）
        issuer-ca.pem       # = console client CA 证书
        console-ca.pem      # = 签发 console 服务端证书的 CA（CA:TRUE 自签锚点！不是服务端叶子）
        manifest.json       # 见下
        complete            # "<stageSha256>\n"
```

manifest.json（紧凑 JSON，键排序）：`{schema:"yofc.kura.remote.certificates.v1",
requestId, gatewayId, endpoint:"https://10.211.55.2:9443", publicKeySha256,
clientCertificateSha256(leaf DER 摘要), issuerCaSha256(文件字节摘要),
consoleCaSha256(文件字节摘要), notAfter}`。
`notAfter` 由 `openssl x509 -enddate -dateopt iso_8601` 输出**去前缀并把空格替换为 T**。

### 5.4 授权与绑定（bind 必须由 kurad 发起）

```bash
systemctl stop kura-management-agent-v2
python3 /usr/lib/kura-management-agent/runtime_setup.py authorize-enrollment-reader --request-id <uuid>
systemctl start kura-management-agent-v2
```

bind 走 agent control socket（UDS，4 字节大端长度 + 紧凑 JSON 帧）：
**第一帧必须是 `kind:"register"` + `method:"kura.register"` + `params:{methods:[...KURA_METHODS]}`**
（kurad ∈ kura-management 组），之后 `remote.bind`
params `{expectedRevision, requestId, expectedStageSha256, operator}`。
root 直连也可以 bind，但 receipt.actor 会是 `local-uid:0`，
后续 remote-switch/巡检要求 `kura-uid:<kurad-uid>`，所以**必须以 kurad 注册连接发起**。

### 5.5 启用通道

绑定后 `remote.set-enabled {expectedRevision, enabled:true}`（root 本地控制器），
agent 立即建立 mTLS gRPC（日志 `control connected epoch=...`），
console `GET /api/v1/gateways` 显示 `online:true`。

正式的 `authorize-kura-remote-switch`（改 agent.env）要求在 enrollment **之前**执行，
且需要 kura.service 停止 + material sockets 停止；绑定后执行会被
"unreviewed owned state" 拒绝。

## OrbStack LXC 环境限制

在 OrbStack Linux machine 上运行 kura.service 遇到以下固有限制：

1. **systemd LoadCredential 不可用**：OrbStack 的 LXC 内核无法正确注入 systemd
   LoadCredential 文件。kura.service 的 ExecStartPre `check-loaded-managed`
   无法找到凭据文件。在物理网关或完整 VM 上此功能正常。

2. **全局 service.d drop-in**：OrbStack 在 `/run/systemd/system/service.d/`
   放置 `zzz-lxc-service.conf`（禁用所有安全限制以适配容器环境）。该文件出现在
   每个 unit 的 `DropInPaths` 中，触发多处安全检查拒绝。已在以下位置修复：
   - kura preinst（e886e82f12）：接受 agent 创建的 key-service drop-in
   - kura_key_store.py（63b960b7dd）：跳过 zzz-lxc-service.conf
   - kura_clock_broker.py（dcca188962 + 9ec0042862）：跳过 generic drop-in
   - kura_reboot_broker.py（1b61790561 + 678ea4698f）：跳过 generic drop-in
   - agent runtime_setup.py（54544d7 + ac88f23）：4 处检查跳过 adapter

3. **Enrollment 流程**：网关的完整 enrollment 需要通过 kura.service 首次启动
   创建 enrollment context。在 OrbStack 上因 LoadCredential 限制无法完成。

**结论**：OrbStack machine 适合验证 console 管理能力和网关 agent 安装配置，
但 kura.service 的完整启动和 gRPC 在线连接需要在物理网关或 Parallels/VMware
等完整虚拟化环境上进行。

## 第六阶段：无凭据注入内核兼容（2026-11-23 已双基线验证）

kura_key_store.py 现内建探测（transient unit 实测 LoadCredential 注入）：
- 注入可用 → 规范模型不变（root 0400 + systemd 凭据注入）。
- 注入不可用 → `credential-fallback-setup`（kura_install.sh 用户创建后自动调用）把
  规范钥的**直接读取副本**发布到 `/var/lib/kura-credentials/`（kurad 0700/0400，
  不在任何 Main 访问过滤器路径上），launcher 与 check-loaded-managed 自动回退该目录。
  规范 /etc/kura/credentials 保持 root 0400 0700 不变。

OrbStack 三台（无注入基线）完整流程实测：
1. 兼容 core deb（6.0.0~git20261123031756.678ea46-1）purge+重装；purge 需先临时挪开
   `/run/systemd/system/service.d/zzz-lxc-service.conf` 并 daemon-reload（旧 prerm 的
   DropInPaths 检查不容适配器），装完恢复（OrbStack 会自动重建该文件）。
2. **必须使用含适配器修复的 agent deb**（0.1.0~cohort20261123034939.c3da8ad-1）：
   c3da8ad 修复 enrollment-reader 准入、firewall boot 与 fence reader 三处
   DropInPaths 检查在 LXC 上的拒绝。旧版 agent 在新 core 上会陷入维护死锁。
3. agent 全新 install→initialize→authorize-kura-peer（APPLIED, kuraUid 绑定新 core）。
4. grant→stage→**bind 须在 kura.service 停止窗口**（Java 桥接占用 bridge 时新注册
   被拒）→set-enabled→启动 kura→agent gRPC 连接。
5. relay 链路四 socket（agent-enrollment / enrollment-reader 两侧）必须在位。

实测结果：三台 kura.service active（NRestarts=0）、agent bound+CONNECTED、
console online=true ×3；Parallels VM 升级同 deb 后注入路径保持、online=true 不变。

### 已知残留（待后续修复核定）
- lifecycle `paired_main_unit` 与 agent `prepare-source-update`（联合维护流）的
  LXC 适配：purge/升级需挪适配器文件；联合包更新未在 LXC 上验证。
- bind 收据 actor：root 发起为 `local-uid:0`；正规 `kura-uid:*` 收据应由 kurad
  发起 register 握手（本批 OrbStack 因 kura 桥接占用以 root 完成，收据已记录）。

## 第七阶段：全量测试记录（2026-11-23，兼容落地后）

### 权威全量（build-all RUN_TESTS=1，macOS，JDK 21 / Maven 3.10.2）

```
BUILD_DOCKER=0 RUN_TESTS=1 ./build-all.sh
```

- 17 个子构建全部 BUILD SUCCESS（Stage 1 公共 → Stage 2 core 142 模块 →
  Stage 3 十二个 siblings + yofc-iot → Stage 4 YOFC 应用 → Stage 5 发行包与
  开发运行集 278 bundles）。
- surefire 总计 **11,180 tests / 0 failures / 0 errors / 16 skipped**。
- 产物：`kura_6.0.0~git20261123053849.9e91f64-1_arm64.deb`
  （heyoulin-kura 9e91f64cac，即含凭据回退兼容与 LXC 适配的提交树）。
- 已知瞬时抖动：`org.eclipse.kura.http.server.manager` 的 HttpServiceTest
  曾报 `Connection reset`（socket 读），单模块复测 15/15 全过——复现时按
  `mvn -pl :org.eclipse.kura.http.server.manager test` 单独复跑即可，无需整轮重跑。

### 补充套件（环境说明）

| 套件 | 环境 | 结果 | 环境限制 |
|---|---|---|---|
| agent packaging（21 文件） | macOS 宿主 | 325 过 | test_enrollment_reader 16 项需 Linux（socket.SO_PEERCRED） |
| agent packaging（21 文件） | 裸 Linux 容器 | 197 过 | 6 文件需 systemd/用户夹具（裸容器不可复现；历史权威基线为专用 Linux 宿主 90 项） |
| distrib（100 文件） | macOS 原始 unittest | 533 过 | 51 文件需正式 Linux harness（历史在 field fixture 运行）；环境性失败与本次改动无关（改动前后对照一致） |

### 残留验证项
- LXC 上的联合包更新流（prepare-source-update 需 core+networking 成套）未验证。
- OrbStack 三台 identity 收据 actor 已规范化为 kura-uid:995（kura 停止窗口内
  kurad register 重绑；decision 文件须保留 agent 属主写入）。

## 第八阶段：四机全新部署与 enrollment 终态（2026-11-23/24 已验证）

本轮在**全新 OrbStack ×3 + 容器 console + Parallels VM（还原点还原后）**上走通
完整链路：安装→init/authorize→keygen→签发→console 注册→stage→bind→enable→
**console 4/4 online=true 且 kura_available=true**。收据 actor 全部 `kura-uid:*`
（gw-1/2/3=999，yofc-vm-1=994）。

### 版本基线
- core：`kura_6.0.0~git20261123053849.9e91f64-1_arm64.deb`（heyoulin-kura 9e91f64cac）
- agent：`kura-management-agent_0.1.0~cohort202611240042.393051e-1_arm64.deb`
  （agent 393051e：握手拒绝 warn 日志 + 0750 测试对齐；binary sha c14ea7de…，
  package.json 记录 bridge jar 532c124e…/coreLoader 547a54ef… 与 core 9e91f64 配套）
- console：正式 Dockerfile 镜像（含 ca_quorum 修复 5a963d1），compose 路径 18443/18444

###本轮修复的三个 bind 失败根因（按发现顺序）
1. **客户端 EOF 空转**：agent 对握手拒绝（桥占用 CONFLICT、uid 不符、方法表不符）
   静默关连接且只记 debug 日志；客户端 `rf()` 无 EOF 检查时 `recv()` 返回空字节
   死循环，表现成 120s 超时。修复两处：agent 393051e 将拒绝日志升为 warn；
   部署脚本 `rf()` 加 `if not ch: raise RuntimeError(...)`。
2. **stage 从未发生**（新机进度误记）：机器重建后 `/tmp/enroll` 只有 keygen 三件套，
   stage 脚本必需的 `leaf.pem/issuer-ca.pem/console-ca.pem` 缺失即不会产生 slot。
   **机器重建后必须从 keygen→签发→注册→stage 全链重做**，不能复用旧机器材料。
3. **leaf 公钥不匹配**：`/tmp/deploy/materials` 的 leaf 由重建前旧机 CSR 签发
   （bind INVALID_ARGUMENT 的真实来源，console pinned 也会一起错）。已用
   `POST /api/v1/gateways/{id}/certificate`（previousCertificateSha256+新 leaf PEM，
   admin+X-CSRF-Token）原子轮换 pinned 指纹，再以新 CSR 重签 leaf。

### 操作要点（本轮实证）
- stage manifest 的 `endpoint` **必须带 `https://` 前缀**（如
  `https://heyoulin-MacProM3.local:18444`），bind 校验 `endpoint()` 会拒绝裸 host:port。
- enroll-final 顺序：relay 两 socket `enable --now` → 停 kura 释放 bridge 槽位 →
  kurad register+bind（kura-uid 收据）→ root `remote.set-enabled` → 启 kura →
  轮询 `remote.status` 至 `CONNECTED`（Java 启动到上线约 60–90s）。
- 重建机器的 enroll-final 卡死排查顺序：`journalctl -u kura-management-agent-v2`
  是否有 `local control peer rejected`（393051e 起）；kura.service 是否在运行
  （桥占用）；slot 与 stage sha 是否与参数一致；leaf 是否来自本机 CSR。
- prlctl exec 会把参数按空格重切、只执行第一个词：复合命令必须先以单文件脚本
  curl 进 VM 再 `prlctl exec <id> bash /tmp/x.sh` 执行。
- agent deb 重建管线（393051e 实跑）：
  1. `python3 packaging/build.py source --protocol <yofc-management-protocol-0.1.0.crate>
     --sha256 <crate sha> --destination <新目录>`（crate pin c1304333…）；
  2. `rust:1.95-bookworm` 容器 `cargo test`（807/0/0）+ `cargo build --release`；
  3. `yofc-agent-packager` 容器 `build.py deb --runtime-profile native-nonroot-v1
     --bridge-bundle … --bridge-sha256 532c124e… --core-loader-sha256 547a54ef…`。
- Parallels VM 挂起恢复后 agent 自动重连（本轮 yofc-vm-1 identity 在快照还原后的
  升级批次中保留，pinned a472c3e1 与机器 leaf 一致，未重做 enrollment）；
  该机 agent 版本为 c3da8ad-1（LoadCredential 注入路径不需要 5c8ba36 的 0750
  兼容），第 2 轮全新部署统一使用 393051e-1。

