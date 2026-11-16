# Root 日志归档实施合同 — 2026-11-17

基线 Core c3699e2cb2、GWT a1e7132a5e。完整12父项不缩项；本合同不是完成或发布证明。

归档使用不可变、绑定Root命名空间和原字节摘要的签名意图/回执，保留原文件及其原路径。
只有正面完成的记录退出活动容量计数。所有原UUID、requestTag、结果与未受理围栏仍参加查询、
禁止重发和启动校验；不删除原件、不搬密钥、不提高活动上限。归档不意味着应用或全部provider就绪。

Root 原生 operations / startups / handoffs / sender-admissions / package-history 均须配对：预览不写入；
明确归档先保存原意图，再发布回执；回复丢失只读原编号。意图存在但回执缺失不能换ID重发；
只允许明确的同一元数据归档续接，重新核对原条目摘要和正面结果。任何归档步骤不调用密码、
StopUnit、StartUnit或其他key writer。其他UNKNOWN不被改判或关闭。

operations/startups须SUCCEEDED；共同准入须TERMINAL或永久NOT_ADMITTED并核对原件；
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
