# 注册材料 FD、原授权回执与运行时身份审查 — 2026-11-17

本轮仅保存源码与隔离检查，**新 agent 包未构建、未安装、未启用**。agent `75ebf3c` 加入被动发布器，`9a25c91` 接入 Rust 消费端，`36fff96` 加入 Root 代码/unit/源码清单配对及原 UUID 授权意图，`8a7c03e` 阻止 purge 删除已完成的授权记录。全部为共同本地 WIP 分支，无推送。

## 已实施的有限范围

现有 UID0、仅 NET_ADMIN 的 agent 无法直接读取 Kura UID997 的 0700/0600 注册暂存。候选发布器以 Kura UID、零 capabilities、NNP1 运行，只按固定请求编号/暂存摘要打开十个固定文件并交付只读 FD。没有路径/命令输入，私钥不进入 JSON、浏览器、中心或日志。Rust 校验固定端点/Root peer、单响应 EOF、FD 数量/顺序/只读/CLOEXEC/UID/模式/链接/大小/稳定元数据，再保留原独立复制、manifest、PKCS8/CSR/证书校验及原共享写门禁。错误不回退其他路径。

包映射包含脚本和 socket/template unit；Root 元数据与完整 schema2 源码清单绑定，拒绝公共及实例 override、链接、错属主/权限、漂移配对。新本机授权必须 inactive、已有准确 Kura UID 授权、通道关闭；保留原 UUID、前后环境、grant、intent/result。七个持久化中断点及环境切换失败保留 UNKNOWN，并阻止换号重试。同编号再次调用只读取原结果。授权本身不启动服务/开启通道，显式 activate 才启动只读 socket 后启动 agent。升级/移除会停止 socket/已加载 worker，purge 遇到原授权/回执要求显式已验证归档。这些生命周期路径仍需真实 systemd 验收。

## 检查与实际边界

- 9 项 Python publisher Linux 检查通过。UID997/cap0/NNP1 的真实 Python worker 向受限 UID0/capNET_ADMIN/NNP1 传递只读 FD；公开合成源保持指纹与元数据。
- Rust 注册/续期 30 项通过，常规 run 中 2 项 Root interop 入口未执行；随后显式运行隔离 parent，包含其 child，通过。受限消费进程直接读源 EACCES，FD 路径完成真实独立证书校验、创建私有候选，未发布 identity pointer/intent。仅一次性测试钥/证书，源字节、inode、UID/GID、模式、链接、大小与 atime/mtime/ctime 精确。
- 扩展 Python run：116 项通过，2 项依赖显式历史原生导出夹具的检查未执行。后续 purge 补丁相关 14 项通过，含现有 12 项本机授权检查；这些与前一 run 重叠，不累加成覆盖量。
- 环境失败记录保留：初次 Rust mount 遮住 cargo、工具链组件检查试图联网、protocol 缺 workspace 根；纠正隔离挂载/指定已安装 1.95.0 后编译。分片测试同步写满测试 socket 的阻塞已修复。扩展测试初次容器缺 OpenSSL/Core 源码挂载，随后使用已有明确 image ID 和只读 Core 挂载通过。失败的测试镜像构建没有产物，也未修改任何原现场。

## 新反例：Root 同 UID 仍可绕过直接路径遮蔽

在既有新组合现场 C 只创建一个独占的公开临时探针 `/var/lib/yofc-reader-sandbox-public-probe-<UUID>`（Root/0600），然后进入 agent 的挂载视图，以 UID0、仅 NET_ADMIN、NNP1 读取此公开探针。直接路径不可见；经实际 UID0/cap0 key-service 的 `/proc/PID/root` 可读；经 PID1 及不同 UID 的 recovery service 均拒绝。探针已删除；`/var/lib` 目录时间戳因此改变。**未读取密钥、HMAC、凭据、日志内容，没有调用 Root 业务 RPC 或任何旧 ID。**

Linux 的 [proc_pid_root](https://man7.org/linux/man-pages/man5/proc_pid_root.5.html) 描述该入口会提供目标进程的挂载视图。仅设置 daemon 不可转储不能证明完整边界，[execve](https://man7.org/linux/man-pages/man2/execve.2.html) 可重置 dumpability；UID0 对特权 IPC 的身份也没有与 agent 分离。因此不能再把“直接路径不存在”外推成 agent 无法接触 Root 保管区。此前原生直接路径观察保留为历史有限证据，完整最小权限父项从未关闭。

此反例阻止部署当前 Root-actor 候选。后续从专用非 Root agent 身份与受限委托实施：Root 源码/unit/公开 peer 身份必须配对；Core key-service 继续保持原 Root 校验，不接受任意用户或 Root/非Root二选一的宽泛匹配。只读注册材料委托应成为窄接口，而非赋予 agent DAC 或 Root IPC 权限。非 Root 自有运行状态与 Root 管理/授权记录分离，旧数据迁移、凭据连续性和所有 Root 必需能力须逐项实施与验证，不盲改旧目录/文件属主。

## 保留与继续项

原 3 现场的 7 进程、38 私有完整元数据、6 份旧数据以及个人当前 25 文件再次精确。新 C 的 9 私有元数据、各运行进程身份、Root operations/startups=0 及 agent 执行表均保持；新中心 D 的 29 私有元数据与 PID3135/startTicks16649159 保持。全 18 仓已核对共同分支、HEAD/树及清洁工作树；本文档自身提交不作为代码树的循环证明。

`combination-c-ebdbf564` 的未确认浏览器 CSR 意图仍只保留，不生成、不换 ID/标识、不移到其他 Root 重做。新的 FD/授权单元夹具不结算该业务意图。所有旧 UNKNOWN 和已闭环的备份、归档、轮换、恢复及 900 秒续期不重发/重放。原 frozen 12 父项全文 SHA256 仍为 `4e1aaca54eb94c3fc36266506c87a0781498aa797f0c18a01036f74a21584cc7`。

完整非 Root/privileged broker、实际 socket/bind/sandbox/生命周期、R017 本机 GUI/维护/独立激活、完整六组件/provider/launcher/安装升级移除/purge、GWT 发出前持久化编号及全部 12 父项继续 internal pending。external deferred 仍只有 NM/cloud-init/物理硬件、amd64、磁盘满/硬断电三组。目标 active，用户授权使用现有积分继续。

证据与检查点：[当前检查点](../checkpoints/20261116/enrollment-custody-runtime-identity-checkpoint.json)，父工作空间 `.management-close-20261116/source-cohort-20261117/enrollment-custody-20261117/`。无正式发行或全盘验收结论。
