# YOFC Kura 管理体系 E2E 部署脚本

3 台 OrbStack 虚拟机 + 1 台 Parallels 虚拟机作为 IoT 网关，管理中心运行在 Docker。
每台网关只装 `kura-core` 与 `kura-management-agent` 两个 deb。操作步骤、截图与排错见
[`../manual/e2e-deployment-manual-20261127.html`](../manual/e2e-deployment-manual-20261127.html)。

## 用法

```bash
cd kura-management-console/docs/e2e-scripts-20261126
export CORE_DEB=<arm64 kura-core deb> AGENT_DEB=<arm64 kura-management-agent deb> E2E_ROOT=<本轮的工作目录>
bash 00-run-all.sh            # 01 清空 → 02 证书 → 03 管理中心 → 每台网关 04 安装 + 05 登记 → 06 终验
bash 07-screenshots.sh        # 登录管理中心后截网关总览（需要 Node 22+ 与 Google Chrome）
python3 -B check-run.py "$E2E_ROOT" --manifest <release-manifest.json>   # 核对并留档：RESULT.md、scan.json
```

`00-run-all.sh` 任一步失败即停止并以非 0 退出；每步结果追加到 `$E2E_ROOT/steps.tsv`。

## 文件

| 文件 | 作用 |
|------|------|
| `e2e-common.sh` | 配置（均可用环境变量覆盖）与公共函数 |
| `00-run-all.sh` … `07-screenshots.sh` | 按编号顺序的各步 |
| `console-shots.mjs` | 无头 Chrome 经 DevTools 协议登录后截图，只依赖 Node 内置模块 |
| `check-run.py` | 核对最终一轮：步骤全 OK、4/4 在线、扫描日志里的错误与警告行 |
| `e2e-api.py`、`e2e-console-setup.py` | 管理中心 REST（严格校验服务端证书）与首次配置 |
| `pki/` | 在 OpenSSL 容器里生成与签发证书 |
| `vm/` | 在网关内以 root 执行的步骤脚本 |
| `build/` | 三个交付物的构建驱动与缺陷对照实验；按“工作目录”布局书写，见其中说明 |

`E2E_ROOT` 下的 `pki/` 含 CA 私钥，`share/` 是虚拟机取文件的 httpd 根目录；两者都是每轮的中间产物。
