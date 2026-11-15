# G028 / V56 空系统软件前置检查及界面安装链审查（2026-11-14）

已把本机入口改为运行方式、软件安装、显式启动设置服务、领取凭据、打开设置页的顺序。核心包未核实前，启动、服务状态和凭据读取保持禁用。固定 dpkg-query 只读核对安装状态、架构和版本；验证 root 保护的必需文件、程序及源码摘要，不执行未知旧程序作能力探测，不连接 DB 或生成安装身份。缺失、部分安装和未核实状态分别展示。dpkg 对被依赖/替换引用的软件可能保留 unknown ok not-installed 占位，现按字段与全部必需文件缺失共同识别，不能把旧业务 UNKNOWN 当作未投递。

实际精简 Ubuntu 测试发现系统文档排除策略。新包回执保留完整 deb 摘要和三份文档摘要，分别列出 9 个必需运行文件与 3 个可由系统省略的文档；程序、两个 unit、来源记录/清单、包标记及全部 React 资源继续严格逐项核对。旧受理目录、原文件 pins、原命令回执不追改。

第一候选 f167f1f0-c53a-452c-aa55-6bae9564fb1f 的 dpkg 实际成功，postinst COMPLETE，但缺三份文档而保持 UNKNOWN。保留原失败/冻结/回执；开发者仅从原 sealed deb 创建缺失的三个原始文档，未执行 dpkg install、未重启 worker、未修改原目录或台账。随后由界面在同一 UUID 执行原成功回执核定，APPLIED 并解除冻结。该夹具记为人工文档修复后的原请求恢复，不算干净安装验收。此流程不适用于缺少原命令成功回执的 UNKNOWN。

最终独立空系统上，核心包 V56-3、安装工具 V56-4 完成真实 Cockpit/PAM 和 sudo 授权。新原请求 6be94ded-9607-4c7c-b5cd-8fbc84a27313 唯一上传受理/安装一次，实际命令 success、9运行文件和原postinst回执通过，文档省略策略保持，未人工补文件或追加核定。安装后默认中心 inactive/PID0，状态目录只有包标记；没有隐式停服或启动。界面明确点击 start 一次后 PID1589/NRestarts0、HTTPS READY；通过实际管理员通道下载凭据，令牌不入DOM/浏览器存储。下载观察器路径不一致时只从原GUID保存文件，未重复下载。发布地址实际浏览器叶证书与主机可信指纹匹配，管理会话不跨到中心主机名；只读打开 /setup，live200/ready503，未配对、未初始化，无业务API POST。首次业务初始化/TLS/GWT三保存/V36v1均未重放。

截图复核发现服务启动后旧停服快照和安装完成文案可能误述当前状态，已修正：start请求使停服快照失效，重新安装前必须读取主机状态；安装回执描述历史结果，当前运行情况通过服务状态核对。损坏的浏览器原请求存储也不能开放启动。V56-4工具单独更新时 PID与全部私有文件精确不变，刷新只恢复原APPLIED并只读核对服务，没有重装或重启。

验证：88项Linux root回归通过；最终19项软件安装/前置状态与28项主机/Docker契约共47项浏览器测试通过，0失败/skip。实际ARM64核心包使用G027普通非test release ELF 20bb95157abf7a93dd3b3fa04db7c60e0f9bc73fe894c3f0dc1e19eaeb2f3fa9，当前135个Rust/React运行输入精确，包装、UI与测试改动不要求重编同一运行输入。核心与可独立更新的工具分别绑定来源清单，工具V56-4批准的仍是原实际V56-3核心包，无自引用摘要。失败、旧候选和两套独立夹具全部保留。

边界：该验收使用独立Docker内systemd，现有 owned OrbStack VM始终stopped；测试OS由监督者以APT安装工具依赖，镜像policy-rc.d=101保持，Cockpit socket在新测试容器内显式启动。操作系统初始软件入口的图形化、完整业务配置/正式发行仍是内部pending，不能宣布空主机全GUI bootstrap已完成。privileged测试容器不代表正式sandbox/polkit。两套OS夹具在验收终态后停止，Docker记录停机exit130、无OOM、PID0；此事实保留，不计为正式主机关机验收。其他10个单元/打包容器退出0；所有新任务终态，浏览器与builder关闭，数据/包/镜像/缓存保留。

源码V56：本批10个已有路径，2196source+4fixture；相对最后完整CODE V51为120输入增量。121existing+194new=315候选，含3fixture-only，故312 production/test路径整体UNAPPLIED。未向原仓应用管理源码，G025原仓40路径保留；未提交推送。新检查点 source-checkpoint-empty-host-v56.json SHA31a0bd288df5c28fc873f0786273e595ea907b3223b33a69df1674500886556b，工作组件 target/nm-role-authority-20261112/registered-runtime-v52/console-empty-host-v1/。它是软件交付组件，不是全量CODE seal或改造完成。

原注册现场仍V52：boot/五PID/agent affa64e2、767863a9唯一startup.stop UNKNOWN/a87d94d5 intent前后精确，无stop receipt/publication/start。原a79b传输UNKNOWN和历史保留。18原仓与共享API522不动，既有seal全保留。

继续：操作系统软件入口与正式交付、中心地址/DB/CSR原回执恢复与续期、agent独立身份/CSR/续期/通道能力、多管理员外部CA custody/quorum；凭据部分恢复/首设TLS UNKNOWN只读核定、跨版本DB/非空加密任务/备份恢复、正式sandbox/polkit/协议0、完整owner/ordinary/recovery/原forward-confirm-inverse-observation-stopped-expiry、合法boot/netns/handle restore/全CM-runtime-NAT矩阵、初始化退役及原inverse/子结算历史/readoption、14组/7API2/R017/312路径guarded apply/正式API/校验和joint generations、G021真实LDAP、G022 Go31vet/3非JavaPOM/可选regen/.NET按原完整台账继续。仅NM-cloud-init-physical、amd64、disk-full-hard-power三组外部deferred。目标ACTIVE，周用量28%，credits与2reset未使用。
