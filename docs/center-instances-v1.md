# 多中心 console 实例与 systemd 模板 v1

日期：2026-11-02。`kura-management-console.service` 继续表示默认实例；
`kura-management-console@.service` 表示显式命名实例。instance 只允许
`[a-z0-9][a-z0-9-]{0,31}`，例如 `center2`。

## 布局

- 配置：`/etc/kura-management-console-<instance>/console.env`，root mode 0600。
- 配置目录：root:yofc-console mode 0750。
- 状态：`/var/lib/kura-management-console-<instance>`，yofc-console mode 0700。
- 日志：`/var/log/kura-management-console-<instance>`，yofc-console mode 0700。
- systemd：`kura-management-console@<instance>.service`。
- 二进制和 React payload 可共享；数据库、task key、TLS/CA/client trust、origin 和端口必须独立。

## 显式初始化

```sh
sudo /usr/bin/python3 /usr/lib/kura-management-console/setup.py --instance center2 init-instance
sudoedit /etc/kura-management-console-center2/console.env
sudo /usr/bin/python3 /usr/lib/kura-management-console/setup.py --instance center2 init-key /var/lib/kura-management-console-center2/task-data.key
sudo /usr/bin/python3 /usr/lib/kura-management-console/setup.py --instance center2 migrate
sudo /usr/bin/python3 /usr/lib/kura-management-console/setup.py --instance center2 activate
```

`init-instance` 只创建目录和 package marker；不创建配置，不迁移数据库，不启动服务。

## 手工实例纳入 systemd 前的显式交接

不删除或重写既有 state。先停止手工进程，再核验/修正权限：

```sh
sudo chown root:yofc-console /etc/kura-management-console-center2
sudo chmod 0750 /etc/kura-management-console-center2
sudo chown root:root /etc/kura-management-console-center2/console.env
sudo chmod 0600 /etc/kura-management-console-center2/console.env
sudo chown root:yofc-console /var/lib/kura-management-console-center2/tls/server.key
sudo chmod 0640 /var/lib/kura-management-console-center2/tls/server.key
sudo /usr/bin/python3 /usr/lib/kura-management-console/setup.py --instance center2 init-instance
sudo /usr/bin/python3 /usr/lib/kura-management-console/setup.py --instance center2 check
sudo systemctl start kura-management-console@center2.service
```

state、task key、PostgreSQL 数据和 CA/client identity 都保留。失败时不要删除数据来绕过检查。

## 包维护

`preinst`/`prerm` 现在检查默认实例和全部 `kura-management-console@*.service` 实例；
有 `QUEUED/RUNNING/UNKNOWN` 任务时拒绝维护。普通 remove/purge 仍不删除外部 PostgreSQL、
task key 或实例 state。2026-11-02 OrbStack 已用 candidate15d 实装并把手工 center2 切换到
`kura-management-console@center2.service`；这只证明该 VM 的受控迁移，不代表生产发行或跨架构验收。

2026-11-05 修订：初始化会给新目录设置正确的服务属组；activate 使用所选实例。维护枚举同时检查 loaded units 和保留的配置/state 目录，覆盖未加载实例。OrbStack 已验证三种未决状态均阻止所选与未加载实例的维护；Parallels 已用当前候选包完成干净安装、严格沙箱命名实例、remove/reinstall/purge，TLS/task key/数据库均保留。详见本轮质量审查记录。
