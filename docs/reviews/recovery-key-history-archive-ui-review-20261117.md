# 独立恢复页归档 GUI/API 审查 — 2026-11-17

当前 Core `141e1be06853418a8554c2f1eee73e378d848017`，归档功能提交为 `6d41ab7308217a3b8eccd067d148e8ea59065470`，后续仅追加一行原生焦点CSS；GWT保持 `25fd6b7328dc38e2df44f488b1d093dbccaf3758`。这是本地候选，未部署、未推送，不关闭完整管理改造。

恢复页提供 RESTORE/START 和两类 ADMISSION；交接页提供 HANDOFF 和发布/原续接/启动/续期四类 ADMISSION。每页20条物理历史，已归档原件仍可列出；不在选择、分页或预览时写入归档。交接列表先校验原授权，只返回同一申请的记录；冷恢复写操作仍须原备份＋口令、备份摘要和原钥指纹匹配。

HTTP预览/查询返回脱敏状态与许可，不直接暴露Root证明/私有路径/口令/备份/bearer。写操作现在额外要求reviewSha256，在同一私有锁下重新读取原字节、Root证明和本地意图；变化则拒绝。Root原件缺失或HTTP丢回复不允许迟到首次发送；只有明确原Root元数据意图可显式续接同一编号。查询不代行续接，不改判或关闭密钥UNKNOWN。

archive.js独立加载并纳入包清单，原recovery.js逐字节不变，保留每资源65536字节上限。原生form约束、明确submit和可见label；安全取消默认焦点的原生dialog负责模态/Escape/返回焦点，close读取后清returnValue；aria-disabled按钮保留焦点但处理器阻止激活。材料仅留内存，提交/取消/页面隐藏及时清空；生命周期代次阻断迟到回调。新旧精确包profile同时保留，原3资源profile不会绕过UNKNOWN、完成回执或完整generation-reader验证。

127项去重当前Linux检查无失败/错误/跳过：私有API17、交接权限9、普通UID安装器3、生命周期23、预解包12、维护15、启动27、包配对21。18组新浏览器场景使用源码可重建的Ubuntu24.04测试镜像、实际生产页面/HTTPS/私有文件/Root归档HMAC，覆盖九类选择、原授权、重载、失效摘要、Root意图缺失/中断、原元数据续接、HTTP丢回复、生命周期、分页及390px双语布局。Root UID/Main/systemd、历史结果和备份inspect分别有模型范围；交接fixture在全新临时树构造自己的授权/历史，不能称为全程零密钥置备或另一轮实际恢复验收。

127项功能检查和18组浏览器场景基于6d41；141e与其仅差新区域的一行原生focus-visible覆盖。另用当前样式在实际Chromium逐项核对button/input/select键盘焦点，均为outline:auto及3px offset，无Root请求。功能代码未变，不将其说成另一轮完整验收。

新ARM64包203,492,422字节，SHA256 `8919b248b513233e3b939ae32750eaa2502242e2c35e9a93e1b0f45d0f761157`。42个唯一manifest文件/单元；另连同实际启动脚本、未纳入该清单的其余Python脚本、Main单元及manifest，共49个提取文件与stage匹配；12个Python脚本及4个静态资源与Git源码一致。提取的独立Lifecycle/cohort reader验证通过，控制脚本3份内嵌reader均匹配当前源码，337成员无pyc。这不等于所有安装文件/六组件/平台均已完成配对验收。

从源码复现见 `kura/distrib/src/test/recovery-archive-browser.md`，不依赖旧特殊镜像或证据文件。确认无构建进程及直接Docker文件挂载引用后，删除旧be66候选、提交前错标签临时包、zip/SFX及截图1,424,858,770字节，无备份；保留当前deb、b7/c369升级输入、stage和当前GWT JAR。临时容器和JDK卷已自清，原6容器/17卷及3现场不变。源码已本地保存，未推送。

接下来仍须当前完整cohort与实际Main/Root UDS/GWT/独立HTTPS联合核验、全部provider/原子重加密、合法原记录恢复、最小权限与完整包/模式/平台矩阵。所有12父项保留，只有原定3项external deferred。
