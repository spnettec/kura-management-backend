# 构建与证据脚本

三个脚本按“工作目录”的布局书写（路径相对脚本自身的位置），不是放在本目录就能运行。
工作目录需要与下面两个历史目录同级，并照这样摆放：

```
<父目录>/
  configuration-capability-20261120/linux-agent-cargo/          cargo 缓存（挂进容器作 CARGO_HOME）
  core-networking-agent-peer-20261120/rust/protocol-target/package/yofc-management-protocol-0.1.0.crate
  <工作目录>/
    rebuild-all.sh          清空中间产物 → Core deb → agent deb → 管理中心镜像 → release-manifest.json
    source-revisions.txt    每行 "<仓库> <提交> tree=<树>"：kura、kura-management-agent、kura-management-console
    agent/build-agent.py    agent deb：stage → test → build → package
    agent/collect-evidence.py   ETXTBSY 缺陷的修复前/后对照实验，日志留档在 agent/evidence/
```

`rebuild-all.sh` 调用 `kura/build-all.sh`（Core 的正式构建入口）、`build-agent.py` 和
`kura-management-console/docker` 下的 `docker compose build --no-cache`；三者都成功才输出 `REBUILD OK`。
