# Core 历史 static 与 linked 迁移

2026-11-18。从 Core `40268c05c3`、console `ee18054` 续接。18 仓分支、HEAD、树和工作树符合原检查点。原 7fd 的容器状态与停放记录完全一致，沿用 41 输入、16 回执的最后读回。原 3、个人配置、C、D、E 继续保留。

## 迁移合同

历史 schema 1 static 回执在维护前检查原 HMAC、移除围栏、UID/GID 及已保存 Main 摘要。Root 将原发行包的 `libexec/key-backup/manifest.json` 原字节放在 `/var/lib/kura-retained/systemd/source-package-manifest.json`，文件保持 Root 私有。安装器要求包历史完整且有唯一因果头，原 manifest 内容摘要与该头匹配，原定义中的 Main 摘要与保存单元匹配。随后标准重装验证新包的有效 Main，保留 static 的停放语义。若原清单本身没有 Main 摘要，继续走已有审查门禁。

对明确选中的 linked 或 linked-runtime，管理员从已核验的新 Core deb 提取控制文件，在原安装上运行 `bash /path/to/extracted/preinst --migrate-linked-main`，随后走正常包操作。该入口沿用 preinst 的普通启动与 agent 维护锁，以及原 Root key/sender/recovery 检查。仅接受 PID1 当前加载、无作业的 inactive Main，链接及原文件存在于 Root 保护路径且与已装包的完整 generation cohort 配对。复制 vendor 单元、原子保存链接、reload 和重新校验后才释放临时链接。原源文件继续保留。负向单元测试验证 reload 失败留原链接证据并可沿同一个状态补完；通用掉电和中断矩阵继续待验。普通安装和移除仍拒绝未经明确迁移的 linked 状态。

实现提交 `54942b0fba`、`3f9b5e334e`、`77d53eda4a`。首次构建触发原 128 KiB lifecycle 源码上限，构建日志保留。将 linked 逻辑独立嵌入已核验 preinst，保留原 key reader 上限。74 项定向测试，73 通过，1 项 Linux renameat2 在 macOS 跳过。JDK21/Maven3.10.1 从 Git 构建，验证 ARM64 包中的 39 个 key/generation 文件、3 个 key 单元、Main 摘要、控制脚本和 clock guard。

## 真实 ARM64 结果

历史夹具由 e99 源码对模板省去 `[Install]` 构建，实际 `dpkg` 安装和移除生成 schema 1 回执。第一次尝试标准包重装，因缺原清单 exit 1，24 份原输入、retained、副本及围栏保持。提供原发行清单后，标准包沿移除 `8d6c85f8-c391-4dae-b065-871480ed1683` 重装 exit 0，24 份原输入连续，Main disabled，维护回执总数 2。

同一新架构夹具的永久和 runtime linked 均完成实际 systemd 迁移，25 个保护输入和辅助单元观察精确。首个永久链接样本的目标位于 `/var/tmp` 可写祖先下，产品正确拒绝并保留证据。夹具另提供受保护路径下的精确字节后通过。另一新夹具安装原 `40268c05c3` 发布包，从实际 linked 布局调用新包提取的控制入口，原包版本、23 个输入、旧链接源文件和辅助 socket 状态保持。接着实际 `dpkg` 升级 exit 0，23 个输入连续，回执 1→2，Main 保持 disabled。两个新夹具均最终正常停止 exit 0。

## 已安装 Core 的 OSGi 业务

在同一 schema 1 迁移夹具，临时 sibling 探针通过实际 Main 写租约及真实 CryptoService，完成 65,329 字节内存 AES 流往返。随后真实 ConfigurationRevisionService 对两个现有 PID 返回存储视图，并取得 ClockService 的逻辑视图。216 个 bundle 可见，配置选取和加密测试限于本夹具。25 个迁移后保护输入最终完整指纹精确，Root key、clock 业务与日志化启动记录均 0。

原环境的线程创建曾撞到隔离夹具的 PID 限额，第一次探针实际未分发。后续 AES 完成，初版采集器误要求存储 revision 无前缀，保留失败输出。约束同一夹具的 CPU affinity，按实际 `stored:` 合同用单独只读探针补完，AES 已过步骤未重复执行。停机后验证 Main disabled/inactive，容器 exit 0。

## 续接

已验证的是有可信原清单的 schema 1 static、配对 linked/linked-runtime、一次从原发布包的 linked 后升级，以及两个安装态 OSGi 业务切片。其余历史输入、完整 Core/OSGi 业务、RTC/Chrony、其余 typed brokers、六组件/provider/launcher、通用中断恢复和完整 12 父项仍内部 pending。最终闭环 0/12，唯一 external deferred 仍是实际 NM/cloud-init/物理硬件、amd64、磁盘满/硬断电。原 UNKNOWN 和旧维护目的保留，无推送。

私有回执位于父工作空间 `.management-close-20261116/legacy-main-migration-20261118/`。见 [机器检查点](../checkpoints/20261116/core-historical-main-migration-checkpoint.json)。
