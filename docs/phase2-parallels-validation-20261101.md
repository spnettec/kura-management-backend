# Parallels Ubuntu 26.04 与 OrbStack 联合验证

日期：2026-11-01。这是独立 agent 的受限环境验收和兼容修正，不是完整阶段 2 或正式发行。

## 实际环境

用户已安装 `Ubuntu 26.04 ARM64`，UUID `8061d57e-0120-42ec-a9a9-5799b436c4eb`，
`aarch64`、实际 `systemd-detect-virt=parallels`。保持 **2 vCPU / 4096 MiB / 64 GiB**，
多网关使用这一台 Parallels 配合既有 OrbStack，不再扩建多台重型 VM。

安装前正常保存快照 `yofc-before-agent-20261101`，UUID
`ce60c087-5614-477b-92d1-8c003317d150`。运行中增加网卡被拒绝，未改变硬件；
随后正常关机添加一张 Host-only 网卡并开机，保留原 Shared 管理网卡。
当前 `eth0=10.211.55.4`、`eth1=10.37.129.3`，没有接管或重写任何网卡配置。

该镜像原本同时运行真实 NetworkManager 与 networkd，`eth0` 有 metric 100/1024 两条默认路由；
networkd 的运行来源为 `zzzz-dracut-default.network`，vendor Netplan 选用 NetworkManager，
安装器 YAML 还有旧 MAC。**不是纯 networkd 的可写基线**，不能禁用竞争服务后声称通用接管已完成。
两个原 `/etc/netplan/*.yaml` 的字节与权限均已留证并保留。

## 独立只读 agent

`scripts/phase2-parallels-fixture.py` 固定上述 VM，安装独立的
`kura-management-agent.service`、配置和数据路径，网关 ID 为 `parallels-2604-gateway`。
接口授权为空；不安装 Kura、不启用网络写权限。证书注册仍为开发用静态预登记，
重跑必须核对中心中原租户、证书指纹及 enabled，不覆盖别的绑定。

宿主中心 gRPC 仅监听 `127.0.0.1:18444`。开发 TCP relay 只绑定 Parallels Shared 宿主地址
`10.211.55.2:18444`，转发原始 TLS，不终结 mTLS；guest hosts 使用原有证书 SAN
`host.orb.internal`。不得扩为公网或生产注册方案。

初始真实沙箱读取失败：unit 未允许 `AF_NETLINK`，导致运行地址和路由为空。
已在只读 unit 增加该地址族；进程有效和 bounding capabilities 仍为零，
`NoNewPrivs=1`，ProtectSystem strict / ProtectHome / PrivateTmp 生效，UDS 0660。

## Netplan 26.04 修正

实际测试发现 `/usr/libexec/netplan/generate --root-dir …` 在 26.04 因 Open vSwitch generator
状态断言触发 SIGABRT。该软件包将原生命令行入口拆到 `configure`；直接调用
`/usr/libexec/netplan/configure --root-dir …` 成功生成私有暂存目录中的 networkd 后端，
原管理来源、运行后端、NM profile 与默认路由未改变，没有执行 apply。

agent 的 `src/network/writers.rs` 因此优先选择存在的原生 `configure`，24.04 继续使用
原生 `generate`，新增入口选择回归。仍不退回 Python `netplan generate`，避免其全局 reload。
26.04 原 YAML 0644 权限警告保留为真实诊断，不擅自改动原文件。

## 实际检查

先完成单元/契约与浏览器，再由 IDEA 启动完整隔离 Kura、登录原控制台，然后安装新 Linux agent。

| 范围 | 结果与证据 |
| --- | --- |
| 新 agent macOS / Linux arm64 | 各 25 单元，all-target Clippy `-D warnings`、build；macOS fmt。`target/parallels-agent-checks.json` 是实际执行结果摘要，并非原始构建日志 |
| 宿主 HTTP/gRPC 错误契约 | 7 项，`target/parallels-host-contract.json` |
| React 中英文 | 3 契约、9 浏览器，见 `console-i18n-20261101.md`，模拟 UI 写入不计 Linux 通过 |
| IDEA 完整隔离 Kura | 原 admin UI 登录，经 Rust/gRPC/UDS 读取 24 配置 PID；PID 86046 验后经 IDEA 停止。`target/i18n-kura-live.json`、`i18n-kura-login.png` |
| Parallels 新构建只读沙箱 | 17 项，mTLS、服务自启、实际进程约束、eth0/eth1 实际地址/MTU、无 Kura 独立管理、未授权写 FORBIDDEN、UDS、源文件和管理路由不变。`target/parallels-smoke.json` |
| OrbStack 新构建写入者回归 | 39 项，实际原生 generate 与后端归属/漂移/人工恢复/重启补偿；revision 2。`target/parallels-compat-writers.json` |
| 双网关隔离 | 11 项，停止 Parallels agent 时 OrbStack 仍可管理，离线任务保持网关绑定，重连收到自身任务，其他网关事务不变。`target/parallels-hybrid.json` |
| 26.04 私有暂存生成 | native configure 成功，现场不变；`target/parallels-generate.json`，不计网络写入/接管通过 |

两个 VM 已安装相同新构建 SHA256
`ae7231b77e04a4851f3dfe70bd43adecf53f5d91a9dc5762debdd4c577e7ac41`。
macOS agent 也已重启为新构建。当前源码和结果指纹见 `docs/evidence/parallels-i18n-20261101.json`。

验后 Parallels 已正常关闭释放内存，快照/配置/证书/数据保留，宿主中心和 OrbStack 继续保留。
ACPI 单独没有关闭 guest；核查只有 greeter 电源键抑制且无待执行系统任务后，经 Guest Tools
执行正常 `systemctl poweroff`，最终 `status=stopped`，没有强制停止。下次启动原 VM 后启用的
agent 会正常自启。证据 `target/parallels-final-poweroff.json`。

原构建 `4572d2e…` 的真实服务重启及正常 VM 重启共两项结果保留为
`target/parallels-lifecycle.json`，不是新构建复跑的重启证据。早期启动时 NM 默认路由尚未恢复，
首次就绪断言曾失败；随后现场验证恢复原双路由，就绪改为等待原完整路由集合。
NM runtime profile UUID 在重启后改变，因此仅在同一 boot 中比较，持久 Netplan 仍对原基线比较。

AF_NETLINK、旧 native generate、路由就绪和 UI 测试的失败证据均保留；Guest Tools 偶发 255
仅允许只读 probe 限次重试，不盲重试安装/重启/控制操作。旧 writers 证据保留原 24/39 计数，
不把历史清单冒充新源验证。

## 未完成范围

**pending**：实际 NM/cloud-init/dracut 的受控交接，DHCP/resolver/完整 IPv6 矩阵、
Kura network provider 与 PID/REST/snapshot 三入口协调、正式注册审批/转交/自动轮换、
nftables、系统控制、离线恢复及发行包。注册认证独立于 Kura，其他中心普通注册不能抢绑定。

**deferred**：物理有线接口及拔插、硬断电与存储故障、Linux 上真实 Kura/provider 系统联调。
完整 Parallels 的只读沙箱与正常重启已有证据，不再笼统标为“非 LXC VM 未准备”。
