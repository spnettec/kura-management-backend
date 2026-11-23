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

## 当前部署状态

| 组件 | 部署方式 | 状态 |
|---|---|---|
| 管理中心 (yofc-e2e-console) | Docker 容器 | ✅ 运行 |
| PostgreSQL (yofc-e2e-pg) | Docker 容器 | ✅ 运行 |
| 网关 1 (yofc-gw-1) | OrbStack machine | ✅ core ii + agent ii + agent active |
| 网关 2 (yofc-gw-2) | OrbStack machine | ✅ 同上 |
| 网关 3 (yofc-gw-3) | OrbStack machine | ✅ 同上 |
| 网关注册 | console API | ✅ 3/3 注册成功 |
| Identity Bundle | console API → 网关 | ✅ 已部署 |
| Kura Peer 配对 | runtime_setup | ✅ 3/3 APPLIED |
| Transport Endpoint | runtime_setup | ⏳ 需要完整 enrollment 流程 |
| 网关在线连接 | gRPC mTLS | ⏳ 需要 enrollment 完整流程 |

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
