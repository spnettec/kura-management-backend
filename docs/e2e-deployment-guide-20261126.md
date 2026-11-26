# YOFC Kura 管理体系——全链部署操作手册（2026-11-26 版）

2026-11-26 全新重部署实录。本手册描述从零开始部署完整 YOFC Kura 管理体系：管理中心（Docker 容器，端口 18443/18444）+ IoT 网关（OrbStack VM 和 Parallels VM）。本版全部步骤于 2026-11-26 在清理后的环境中实际重跑验证：3 台 OrbStack 网关全链 CONNECTED 并在控制台 online=true + kura_available=true。

## 架构概览

```
┌────────────────────────────────────┐
│ 管理中心 (Docker, compose)          │
│   yofc-management-console:latest   │
│   + PostgreSQL 16 (yofc-e2e-pg)    │
│   端口: 18443(HTTPS) 18444(gRPC)   │
└───────────┬────────────────────────┘
            │ mTLS / gRPC（agent 主动连接）
    ┌───────┼────────┬───────────┐
┌───┴───┐ ┌───┴───┐ ┌───┴───┐ ┌─┴────────┐
│gw-1   │ │gw-2   │ │gw-3   │ │Parallels │
│OrbStack│ │OrbStack│ │OrbStack│ │Ubuntu VM │
└───────┘ └───────┘ └───────┘ └──────────┘
```

网关→中心地址：OrbStack VM 用 `https://198.18.0.1:18444`（OrbStack 宿主网关 IP），Parallels VM 用 `https://10.211.55.2:18444`（Parallels 共享网络宿主 IP）。两个地址都必须出现在中心服务端证书的 SAN 里。

## 前置条件

- macOS + OrbStack（Docker + Linux VM）+ Parallels Desktop（prlctl）
- JDK 21、Maven 3.10.2（构建 deb）
- 已构建的 deb 包：
  - `kura-management-agent_0.1.0~cohort202611240042.393051e-1_arm64.deb`（归档：`.management-close-20261116/core-rtc-hotplug-20261120/snapshot-import-conversion/agent-packages-rebuild/release-393051e/`）
  - `kura_6.0.0~git20261123053849.9e91f64-1_arm64.deb`（`kura/kura/distrib/kura-core/target/`）
- 操作脚本库 `/tmp/deploy/share/`（gw-bootstrap.sh、gw-keygen.py、gw-stage-final.py、gw-enroll-final.py）
- CA 工具 `/tmp/deploy/ca/`（gen-ca.sh、gen-server.sh、sign-leaf.sh）

## 第一阶段：中心证书（每轮重新生成）

```bash
mkdir -p /tmp/deploy/ca-r3 && cd /tmp/deploy/ca-r3
sed 's|cd /work|cd /tmp/deploy/ca-r3|' /tmp/deploy/ca/gen-ca.sh | bash   # client-ca + server-ca
# server 证书 SAN 必须覆盖网关可达的全部地址：
cat > server.ext <<EOT
basicConstraints=critical,CA:FALSE
keyUsage=critical,digitalSignature,keyEncipherment
extendedKeyUsage=serverAuth
subjectKeyIdentifier=hash
authorityKeyIdentifier=keyid,issuer
subjectAltName=DNS:localhost,DNS:host.orb.internal,DNS:heyoulin-MacProM3.local,IP:127.0.0.1,IP:198.18.0.1,IP:192.168.128.1,IP:192.168.1.55,IP:10.211.55.2,IP:10.37.129.2
EOT
openssl genpkey -algorithm EC -pkeyopt ec_paramgen_curve:P-256 -out server.key
openssl req -new -key server.key -subj "/CN=yofc-e2e-console" -out server.csr
openssl x509 -req -in server.csr -CA server-ca.pem -CAkey server-ca.key \
  -CAcreateserial -out server.pem -days 3650 -extfile server.ext -sha256
```

网关 leaf 证书由 client-ca 在宿主侧签发（`sign-leaf.sh`：basicConstraints=critical,CA:FALSE；keyUsage=digitalSignature；extendedKeyUsage=clientAuth）。

## 第二阶段：管理中心（compose + setup pair→operations）

```bash
cd kura-management-console/docker
docker compose up -d --build          # console: 18443/18444，卷 console-data
docker run -d --name yofc-e2e-pg --network docker_default --platform linux/arm64 \
  -e POSTGRES_USER=console -e POSTGRES_PASSWORD=YofcE2ePg2026 -e POSTGRES_DB=console \
  postgres:16-alpine
```

pair 凭据在 console 数据卷内，首次启动后读取：

```bash
docker run --rm -v docker_console-data:/state:ro alpine \
  cat /state/setup/installation-credential.json > /tmp/deploy/credential-r3.json
```

setup（pair → operations，写入数据库、admin 账号、server 证书 + client CA 锚）：

```bash
python3 -B /tmp/deploy/console-setup-r3.py    # 脚本内 CA 路径与凭据路径按本轮调整
```

注意：operations 成功后 setup 路由即下线，脚本末尾的轮询会收到 404——这不是失败。以 `GET /api/setup/status` 的 `configured:true` 和 admin 登录成功为准：

```bash
curl -sk https://localhost:18443/api/setup/status
```

## 第三阶段：OrbStack 网关（每台约 5 分钟）

### 3.1 创建并安装

```bash
orbctl create --arch arm64 --memory 2G --cpus 2 debian:trixie yofc-gw-$N
orb run -m yofc-gw-$N -u root bash /tmp/deploy/share/gw-bootstrap.sh   # 依赖：python3/openjdk-21/gpsd/dnsmasq 等
orb run -m yofc-gw-$N -u root bash -c "dpkg -i <agent.deb>; dpkg -i <core.deb>; apt-get -f install -y"
```

agent 先于 core 安装。core 首遍 configure 报依赖错由 `apt-get -f install -y` 收敛。

### 3.2 初始化与双授权（固定顺序，不可颠倒）

```bash
python3 /usr/lib/kura-management-agent/runtime_setup.py initialize --request-id <uuid>
systemctl stop kura.service kura-management-agent-v2.service
python3 /usr/lib/kura-management-agent/runtime_setup.py authorize-kura-peer --request-id <uuid>          # → READY, kuraUid 写入
python3 /usr/lib/kura-management-agent/runtime_setup.py authorize-enrollment-reader --request-id <uuid>   # 漏掉则 bind FORBIDDEN
systemctl start kura-management-agent-v2.service kura.service
```

### 3.3 keygen → 签发 → 注册 → stage → enroll

```bash
# 网关侧（GW_ID 必须传）
GW_ID=yofc-gw-$N python3 /tmp/deploy/share/gw-keygen.py > gw.csr
# 宿主侧：sign-leaf.sh 签发 leaf.pem
# 宿主侧：注册网关（cert sha 需与 leaf 一致）
POST /api/v1/gateways {"id":"yofc-gw-$N","name":"...","clientCertificatePem":"<leaf.pem>"}
# 网关侧：放材料并 stage（单次；endpoint 用本机可达的中心地址）
python3 /tmp/deploy/share/gw-stage-final.py <rid> yofc-gw-$N https://198.18.0.1:18444
# 网关侧：enroll（stop kura → kurad bind（kura-uid:999 回执）→ enable → start → 轮询 CONNECTED）
python3 /tmp/deploy/share/gw-enroll-final.py <rid> <stage-sha>
```

**注册这一步不能省**：未注册的网关 bind 后 transport 会被中心拒 FORBIDDEN。leaf 证书 sha256 必须与注册时一致（可用 `GET /api/v1/gateways/{id}` 核对 certificateSha256）。

### 3.4 验收

```bash
GET /api/v1/gateways → online=true, kuraAvailable=true
# 网关侧回执：/var/lib/kura-management-agent-runtime/enrollment/identity.json actor=kura-uid:999
```

## 第四阶段：Parallels 网关

与 OrbStack 相同的链，差异：

- 文件投递走 `python3 -m http.server 18999`（`/tmp/deploy/vmshare/`）+ VM 内 curl `http://10.211.55.2:18999/...`（Parallels VM 看不到宿主 /tmp，共享目录只覆盖用户目录）。
- endpoint 用 `https://10.211.55.2:18444`。
- `prlctl exec` 按空格重切参数：复合命令必须先 `curl -o` 单文件再 `bash` 执行，不能内联。
- 快照还原：`prlctl snapshot-switch <vm> --id <snapshot>`。对运行中的 VM 切换可能延迟生效——切换后必须核实状态再操作。

### 已知阻塞（2026-11-26 实测，已源码级排查）

Ubuntu 26.04 + agent 393051e 的本地 enrollment bind 稳定返回 `INVALID_ARGUMENT`（干净快照还原 5+ 次、全新注册、正确证书链、单次 stage、python3 -I 均复现；Debian 13 的 OrbStack 网关同流程 3/3 通过）。已构建带行号诊断的 agent（393051e+诊断补丁，debug/bind-diag-20261126 分支）确认：**拒绝发生在 `BindInput` 反序列化之前的本地分发校验**（非证书/材料/签名问题——全部 openssl 校验逐项手工复测通过）。同时 agent 状态机保留 authority 绑定旧二进制摘要，使任何重装/换版路径（purge 重装、retained-successor、Scheme A identity-bundle、configure-native-transport-endpoint）均被 "requires its original migration / differs from original Root proof" 类门禁拒绝。

当日处置：Parallels VM 已还原干净基线 ce60c087 并停放（`YOFC Review ARM64 20261124`，内含 diag20261126134045 诊断版 agent）。控制台保留 yofc-vm-1/yofc-parallels-1/yofc-parallels-1-leaf 轮换记录（enabled+offline）。恢复路径：Debian 13 arm64 重装该 VM（Parallels 无 CLI 自动装 Debian 模板，需 ISO 半自动安装），或修复 agent 对 Ubuntu OpenSSL/systemd 的校验分歧。诊断分支 debug/bind-diag-20261126 与构建驱动 build-agent-diag.py 已保留。

## 第五阶段：环境清理（每轮收尾）

1. 测试 OrbStack VM：`orbctl delete <name>`（个人数据卷先甄别保留）。
2. Docker：测试容器 `docker rm -f`（个人 mariadb/redis 容器甄别保留；注意 zsh 不做词分割，KEEP 列表循环要用数组）；测试卷 `docker volume rm`；`docker builder prune -af`；测试镜像 `docker rmi`。
3. workspace：`*/target` 构建产物删除（源码可重建）；worktree `git worktree remove --force` + 已合并分支 `git branch -d`。
4. 保留：个人配置/密钥/数据、证据归档（`.management-close-*/` 的 HANDOFF/回执/HMAC）、`/tmp/deploy/` 脚本与 CA。

## 故障排除

| 症状 | 原因 | 处置 |
|---|---|---|
| bind 后 transport FORBIDDEN | 网关未在中心注册，或漏 authorize-enrollment-reader | 补注册（cert sha 与 leaf 一致）；按顺序双授权 |
| bind INVALID_ARGUMENT（Ubuntu 26.04） | agent 本地校验环境分歧（见第四阶段已知阻塞） | Debian 化或等 agent 修复 |
| Generation-reader package is not paired | 后继包跨 lineage（构建分支≠基线组件分支） | 后继包必须从基线所在分支顶端构建 |
| Core trigger 拒绝 + 嵌入过期构件 | sibling distrib 引用 monorepo 产物时共享缓存供给了旧 SNAPSHOT | 从配对 cohort 恢复构件装入本轮构建仓库后重建 |
| configure-native-transport-endpoint 被拒 | initialization-id 不匹配 / retained enrollment / socket 未停 | 用 admin/initialization.json 的当前 operationId；先停双 socket；retained 态走独立维护流程 |
| prlctl exec 只执行第一个词 | Parallels 按空格重切参数 | curl 单文件脚本进 VM 再 bash |
| zsh 下 KEEP 白名单失效 | zsh 不分词未引用变量 | 用数组或显式比较 |
