# 最终一轮结果

- 运行目录：`/Users/heyoulin/iot-kura-develop/git/.management-close-20261116/core-rtc-hotplug-20261120/clean-redeploy-20261126/runs/r4`
- 步骤：12 步，12 步 OK，0 步失败；transcript 末尾 有 `ALL STEPS PASSED`
- 管理中心终验：4/4 台 online=True 且 kuraAvailable=True
- 日志扫描：0 行命中错误类模式，40 行命中警告类模式，逐条判断

## 产物

- kura-core：`kura_6.0.0+api2~git20261126165524.56f59db-1_arm64.deb`  SHA-256 `8514725f9297a7be69b2d93d7a35a23a0649097752c57a5dc1e8bf196375c517`  源码 56f59db2da
- kura-management-agent：`kura-management-agent_0.1.0~cohort20261126164221.3a713f5-1_arm64.deb`  SHA-256 `773c53d9089f7e998730ba3f271a2af9aa2a98c16d285ed43e3c2b814d650a93`  源码 3a713f549c
- 管理中心镜像：`sha256:9f4ee47496eeabd6f759e5accfed1e12f560303542c3bcc84711ccf70ae1ecff`  源码 871f60f58e

## 步骤（steps.tsv 原文）

| 时间 | 结果 | 步骤 | 说明 |
|---|---|---|---|
| 01:33:20 | OK | wipe | 上一轮环境已清空 |
| 01:33:20 | OK | ca | 证书体系已生成 |
| 01:33:34 | OK | console | 初始化完成，管理员可登录 |
| 01:41:03 | OK | install-yofc-gw-1 |  active active  |
| 01:41:17 | OK | enroll-yofc-gw-1 | FINAL: CONNECTED | bound: True | enabled: True receipt actor: kura-uid:999 |
| 01:44:52 | OK | install-yofc-gw-2 |  active active  |
| 01:45:06 | OK | enroll-yofc-gw-2 | FINAL: CONNECTED | bound: True | enabled: True receipt actor: kura-uid:999 |
| 01:47:24 | OK | install-yofc-gw-3 |  active active  |
| 01:47:38 | OK | enroll-yofc-gw-3 | FINAL: CONNECTED | bound: True | enabled: True receipt actor: kura-uid:999 |
| 01:50:02 | OK | install-yofc-gw-4 |  active active  |
| 01:51:42 | OK | enroll-yofc-gw-4 | FINAL: CONNECTED | bound: True | enabled: True receipt actor: kura-uid:994 |
| 01:51:44 | OK | verify | 4/4 online + kuraAvailable |

## 管理中心终验（verify.txt 原文）

```
yofc-gw-1    online=True  kuraAvailable=True  agent=0.1.0
yofc-gw-2    online=True  kuraAvailable=True  agent=0.1.0
yofc-gw-3    online=True  kuraAvailable=True  agent=0.1.0
yofc-gw-4    online=True  kuraAvailable=True  agent=0.1.0
total=4 online=4 kura_available=4
```

## 日志扫描命中（错误类，逐条判断）

（没有命中）

## 截图

- `01-console-setup-wizard.png`
- `02-console-login.png`
- `04-install-yofc-gw-4-console.png`
- `05-enroll-yofc-gw-4-console.png`
- `gateways-overview.png`
