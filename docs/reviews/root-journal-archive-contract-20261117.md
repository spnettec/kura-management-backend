# Root 日志归档实施合同 — 2026-11-17

基线 Core c3699e2cb2、GWT a1e7132a5e。完整12父项不缩项；本合同不是完成或发布证明。

归档使用不可变、绑定Root命名空间和原字节摘要的签名意图/回执，保留原文件及其原路径。
只有正面完成的记录退出活动容量计数。所有原UUID、requestTag、结果与未受理围栏仍参加查询、
禁止重发和启动校验；不删除原件、不搬密钥、不提高活动上限。归档不意味着应用或全部provider就绪。

Root 原生 operations / startups / handoffs / sender-admissions / package-history 均须配对：预览不写入；
明确归档先保存原意图，再发布回执；回复丢失只读原编号。意图存在但回执缺失不能换ID重发；
只允许明确的同一元数据归档续接，重新核对原条目摘要和正面结果。任何归档步骤不调用密码、
StopUnit、StartUnit或其他key writer。其他UNKNOWN不被改判或关闭。

operations/startups须SUCCEEDED；共同准入须永久NOT_ADMITTED，或严格匹配原生成功/授权的TERMINAL或历史物理INTAKE；后者没有正面原件则拒绝，归档引用后不能改写原字节；
授权须正面未用撤销且相关预留已经永久关闭，或原固定发布与启动都成功且匹配授权HMAC、
不可变发布、原密钥代/cohort。旧Core PID不再是历史归档的前提。现用代仍读原发布/启动文件。
归档中或损坏的状态在初始化、包门禁、普通启动和运行期读取中按相同合同处理。

随后由所属UID分别处置GWT三类和独立恢复私有历史；Root不获得读取其私有目录的权限。
私有完成/未发送回执不能自行替代Root归档证明。界面提供预览、确认、只读查询和明确的原归档续接。
这些消费者及全部安装/升级/移除组合没有通过前，归档整体保持内部pending。
旧安装器对generations/handoffs的原有受限准入及旧版本prerm配对必须审查，不能仅放宽目录白名单。

## 本轮Root层实现

Core本地提交 `fb47e88ab56b8c2a4bc7a385ae6af97a568e194b`。
Root以单链的不可变 `archives/<UUID>.intent.json` 和 `.receipt.json` 保存HMAC证明；
每条关联原目录、原UUID和原字节SHA，绑定当前Root命名空间。仅回执存在且原件仍匹配才释放活动名额。
原文件不移动，原先按路径查询、requestTag、旧编号占用、现用代启动读取和未受理围栏继续有效。
归档元数据不回收原件占用的磁盘；构建中间产物继续按用户授权清理。

包维护快照兼容旧schema1；新schema2保存至多32个活动启动ID，加当时固定的归档链头及其摘要。
后来的归档和启动不能扩大这次维护的授权范围；Root旧历史不会因活动窗口32/128被截断。
归档ID也不能占用授权的固定ID、原请求或新密钥操作；未物化的签发预留在元数据归档开始前保护。
查询不完成原归档。意图已有而回执缺失时，重复普通归档仅返回UNKNOWN，须明确按原编号续接。
任何已有key/启动INTAKE缺原件都不被关闭或重发。

当前359项配置化/相关检查全部通过，无skip，含30项新增归档检查；
新增三UID实际UDS元数据11连接与原179连接兼容检查通过。Root和客户端零能力/NNP1，
Root仍不能读取Core/恢复两方私有目录，归档没有调用cipher或key writer，实际密钥操作为0。
Core/systemd/historical-padding仍明确为模型；不据此宣布实际安装Main/GWT人类会话/全部provider通过。
初始归档检查有一项测试把换journal.key后的旧namespace请求误期望为归档校验错误；实际正确在namespace门禁被拒，
修正为使用当前namespace后验证旧签名拒绝。新增包门禁测试初次未声明模型systemd目录，产品正确拒绝；
补齐模型管理器后已验证当前维护快照写入与当前格式升级读取。

GWT三类和独立恢复私有历史、GUI预览/确认/原续接尚未接入，归档整体仍为内部pending。
旧b36/c369安装器的initialize对generations/handoffs/archives有原先受限准入，且旧prerm执行旧helper；
新Root独立读取器已增加当前格式校验，但旧版本prerm兼容及真实安装/升级/移除矩阵仍需另行推进。
不能以当前读取器通过代替旧已安装版本的完整升级证明，也不能通过隐藏状态或盲加白名单绕过。

## 后续私有未发送分支 — cd82 / 3b8

Core `cd82fd011805ef71eae07e60314d2ba97e084ac2`、GWT `3b8b5fe168244d6a149f52837b6086889a033128` 已实现此前缺少的永久Root围栏：使用独立 `yofc.kura.sender-private-no-dispatch.v1` purpose及HMAC域，保存在Root原sender-admissions表，0400不可变。只作为永久元数据索引，不消耗/授予普通writer名额，也不冒充原完整请求HMAC。任何原生表、任何状态的原准入、固定授权及未物化签发预留冲突均拒绝；普通sender-attempt和read/close接口不能借用此证明。

GWT纯本地未发送回执的 `archive.json` 先保存原围栏UUID，再调用Root元数据写入；既有意图一律只读。Root缺失继续占活动名额，只有精确绑定原回执字节、data身份和Root命名空间的正面证明才退出64计数。原回执/路径/字节/旧编号保留，handoff未来pub/start编号也永久禁止写入。独立包读取器按同一合同核对，不能只凭私有标记放行。Root不读取私有目录，源UID自己创建私有意图。

Root基线359项回归在私有包读取器加入前通过；当前49项（26新Root/私有分支＋23既有GWT读取器）与90项JVM通过。打包JAR经真实Java/Root三UID26连接核验，64原回执归档1份后可新增第65份；中文原件SHA匹配，原4份Root准入字节/元数据不变，Root不能读Core/恢复私有目录。全部参与进程空能力/NNP1，密钥/cipher/Start/Stop写入0。Core/systemd和Main角色仍为模型，没有实际安装Main/人类GWT通过结论。初次模型未声明agent-cgroup替身、初次独立JVM runner选错JUnit launcher版本，均已修正夹具后通过，不计为产品修复或正式验收。

普通Root NOT_ADMITTED原件对应的私有回执不能走“无Root原件”围栏，仍需另一条Root归档证明路径。其他三类owning日志/共同发送方/独立恢复私有历史、公开GUI和全部安装升级移除矩阵均保持内部pending。旧prerm对新格式的配对问题未解决；新包不部署到原3现场。当前实现和产物见 [私有未发送归档检查点](../checkpoints/20261116/private-no-dispatch-archive-checkpoint.json)。

## 全部 GWT 私有历史候选 — 35cf / 37f2

Core `35cf6e953d8c5ec64bee353c74c90e6bda298cf8`、GWT `37f2fb04232fbc97d82c215d4bed5d47f7a515f6` 已替代上节“其他 GWT 尚未接入”的缺项：三类 owning、共同发送方、已有普通 Root NOT_ADMITTED 的私有未发送回执都使用所属 UID 的 `gateway-key-archives/<KIND>-<UUID>.json`。新只读 `journal-archive-reference-read` 提供脱敏关联和原归档回执；不得返回 requestTag、备份内容、口令或 bearer，不以缺失作为完成证明。已有 Root 归档直接采用原编号和请求，避免重复元数据 ID；已有私有意图普通调用只读，只有明确的同一原归档元数据续接可补原回执，不产生迟到首次发送。

私有标记绑定全部原文件 SHA、Root/data 身份和精确原生关联；只有匹配的 Root 回执才释放64/64/32/256/64活动容量。原路径、字节、结果、全部旧编号及完成授权的准备占用保留；普通未准入回执与无原件围栏的 purpose 不能混用。负准入的 owning 文件必须先保留准确 closed 回执，避免归档后再追加文件；真实缺原件的 INTAKE 不能被归档。已有明确原生成功的历史物理 INTAKE 一旦被归档意图引用，普通读只返回逻辑终态，不改写原件破坏 SHA。

大 helper 拆出 manifest 固定的 `kura_key_journal.py`；新 preinst 内嵌同一精确源码，在 unpack 前读取旧状态，无旧模块/pyc/工作目录回退。旧资产 profile 只读兼容不能借用未固定的新模块。当前受保护源码均低于原131072字节上限。归档元数据有独立有界 JSON 解析器（65536字节、128条引用、深度8、字段名128）；原密钥 RPC 的更小约束保持原样。

403项 Linux、103项 JVM 通过，无skip；生产JAR三UID新历史50连接、原未发送围栏兼容26连接通过。实际 Unix socket/内核UID/PID/能力/锁/HMAC/Java与Root元数据路径已核验，Root两私有目录读取被拒；native/cipher/启动写入为0。Main/systemd及预置历史加密结果明确为模型，不是已安装Main/人类GWT/旧数据解密的新增验收。新包46项配套、11脚本和提取reader通过，新preinst的两个源码嵌入已核对；旧已安装prerm兼容及真实完整升级矩阵仍未完成。当前产物与下一步见 [GWT全私有历史检查点](../checkpoints/20261116/gwt-private-history-archive-checkpoint.json)。独立恢复私有历史、公开GUI、完整发行和全部12父项继续内部pending。
