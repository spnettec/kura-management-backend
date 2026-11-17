/* Authenticated host channel. Credentials stay in memory; browser storage holds
   only public maintenance UUID/step metadata, persisted before dispatch. */
(() => {
  const $ = id => document.getElementById(id);
  const words = {
    zh: {
      dockerVerifyStarted:'核验已启动版本并完成维护',
      dockerMaintenanceTitle:'Docker 中心维护与升级',dockerMaintenanceHelp:'先导入所需的新镜像，再冻结所选中心、核对任务并停服。新版本会沿用原数据卷，在写入仍冻结时启动并接受核验，最后恢复运行。历史容器和原记录保留，结果不明时仅核对原任务。',dockerFreeze:'冻结所选中心写入',dockerStop:'停止已冻结的中心',dockerResume:'完成维护并启动',dockerUpgrade:'升级为所选镜像并启动',dockerMaintenanceObserve:'核对原容器维护',dockerMaintenanceIdle:'尚未发起容器维护。',dockerMaintenanceFrozen:'所选中心已冻结，原服务仍在运行。',dockerMaintenanceStopped:'原中心已确认停止，数据卷保留。请选择完成维护或升级。',dockerMaintenanceComplete:'维护已完成，当前中心已核验并恢复运行。',dockerMaintenancePending:'主机正在执行原维护操作。仅核对状态，不重复发送。',dockerMaintenanceUnknown:'维护结果尚未核实，请保留原编号和数据卷。不会重复停服、启动或解冻。',dockerMaintenanceRefused:'维护在变更前被拒绝，请核对安装是否有未完成的任务。',dockerMaintenanceImportFirst:'当前版本需要先导入支持维护启动的新镜像。',
      dockerVerifyImport:'核定已成功载入的原镜像',
      dockerTitle:'Docker 安装',dockerHelp:'选择随安装包核定的镜像。离线镜像可在此上传、导入；创建时自动分配专用数据卷，填写实例名称及两个发布端口即可。页面断开后，原任务仍由主机执行。',dockerRefresh:'读取镜像和安装列表',dockerImageLabel:'中心镜像',dockerArchiveLabel:'离线镜像文件（tar.gz）',dockerUpload:'上传所选镜像',dockerImport:'导入已核实的上传文件',dockerNameLabel:'新实例名称',dockerHttpLabel:'中心 HTTPS 发布端口',dockerGrpcLabel:'网关连接发布端口',dockerDeploy:'创建并启动中心',dockerObserve:'核对原部署任务',dockerStatus:'核对所选容器就绪状态',dockerAvailable:'Docker 后端可用',dockerUnavailable:'当前主机的 Docker 后端不可用，请先安装并启动 Docker Engine。',dockerImagePresent:'已导入',dockerImageMissing:'尚未导入',dockerIdle:'尚未发起部署任务。',dockerUploading:'正在上传镜像。请保留当前页面；断开后核对原任务。',dockerUploadReady:'上传文件已核实，可导入镜像。',dockerUploadPending:'上传尚未核实。请保留原任务和文件，不重复上传。',dockerPending:'主机正在执行原任务，仅核对状态。',dockerApplied:'原任务已完成。',dockerUncertain:'原任务结果尚未核实，已阻止重复导入或创建。请保留原编号和现场。',dockerRefused:'主机在写入 Docker 前拒绝了任务。原记录已保留，请核对名称、镜像和端口。',dockerPollEnded:'本轮只读核对已结束，请再次核对原部署任务；不会重复发送。',dockerId:'部署任务',dockerBadArchive:'请选择与此版本匹配的离线镜像文件。',
      maintenanceCancel:'取消维护，保持服务运行',maintenanceFinalizeCancel:'完成已确认的取消',maintenanceFrozen:'新写入已冻结，服务保持运行。',maintenanceStopped:'已确认停服，可验证安装并解除冻结。',maintenanceCompleteDone:'维护已完成，服务保持停止。',maintenanceCancelled:'维护已取消，原编号不再接受后到操作，服务状态保持不变。',maintenanceCancelPending:'取消已确认，尚需解除保留的冻结。',maintenancePendingNotice:'请求结果尚未确认，请保留原编号；不会重复发送停服。若服务端尚无停服意图，可取消本次维护。',
      maintenanceTitle:'主机维护',maintenanceHelp:'维护范围包含此主机上所有本机安装的中心。先冻结新写入，再核对任务并停服；升级完成后解除冻结。结果不明确时仅核对原操作，不重复停服。旧版本须在全部中心已停止后由管理员完成离线接管。',
      maintenanceObserve:'核对原维护操作',maintenanceBegin:'冻结所有中心写入',maintenanceStop:'停止已冻结的中心',maintenanceComplete:'验证安装并解除冻结',maintenanceNone:'没有已接受的维护操作。',maintenanceUnknown:'维护结果尚未核实，请保留原操作，仅核对状态。',maintenanceActive:'维护操作',maintenance:'主机正处于维护冻结中，请在下方核对原操作。',pollStopped:'本轮只读核对已结束，服务仍未就绪。可点击“查看服务状态”再次核对；不会重复启动。',
      privilegeRequired:'此操作需要主机管理员权限。请使用有授权的主机账号，并在主机页面顶部开启管理权限后重试。命令尚未执行。',
      separateHost:'中心请使用与主机管理页不同的主机名，以隔离登录会话。',
      portLabel:'首次设置监听端口',
      ready:'主机 HTTPS 服务已就绪。打开后仍需核对发布地址和证书。',waiting:'服务尚未就绪，正在只读核对；不会重复启动。',inactive:'服务尚未启动。',unverified:'无法核实当前服务就绪状态。请查看原服务和原记录。',notReady:'请先查看服务状态，确认主机 HTTPS 已就绪后再打开。',
      title:'YOFC 管理中心安装设置',intro:'使用当前主机的管理员权限启动设置服务并领取安装凭据。数据库、组织账号和证书在中心设置页面填写。',
      targetTitle:'1. 选择安装',modeLabel:'运行方式',nativeOption:'本机安装包',dockerOption:'本机 Docker 容器',instanceLabel:'实例名称',instanceHelp:'默认安装留空；多个中心分别使用自己的实例名称、数据库和端口。',refresh:'读取容器列表',containerLabel:'选择管理中心容器',status:'查看服务状态',start:'启动设置服务',
      credentialTitle:'4. 领取安装凭据',credentialHelp:'此操作需要主机管理员授权，只读取已有凭据；不会生成新证书、重置安装或重发初始化。已配置的安装会拒绝下载。',read:'读取安装信息',identityLabel:'安装 ID',fingerprintLabel:'临时服务器证书 SHA-256',download:'下载安装凭据',privateNotice:'凭据文件含安装令牌，请保存在个人受保护目录中，不转发、不放入共享目录。',
      openTitle:'5. 打开中心设置页面',urlLabel:'中心 HTTPS 地址',trustNotice:'中心地址须使用与当前主机管理页不同的主机名，以隔离登录会话。先核对浏览器安全详情中的证书 SHA-256 与上方一致，再上传凭据文件解锁。中心页面自身显示的指纹不能代替此处主机管理通道的可信信息。',open:'打开设置页面',
      needHost:'请从已登录的主机管理页面打开此工具。',failure:'操作未能核实。请先查看原服务状态和原记录；不要重置安装或重发初始化。',busy:'正在通过主机管理通道核对…',readDone:'安装信息已读取，可下载凭据。',downloadDone:'凭据已交给浏览器下载，请妥善保管。',badTarget:'请选择正确的安装目标。',badUrl:'请输入不含账号或密码的 HTTPS 地址。',containerHelp:'容器须已由部署平台启动，并带有 YOFC 管理中心标签。'
    },
    en: {
      dockerVerifyStarted:'Verify started generation and complete maintenance',
      dockerMaintenanceTitle:'Docker center maintenance and upgrade',dockerMaintenanceHelp:'Import the target image first, freeze the selected center, check tasks and stop it. The next generation reuses the original volume and starts with writes still frozen. It is verified before service resumes. Old containers and records are retained; uncertain operations are observed only.',dockerFreeze:'Freeze selected center writes',dockerStop:'Stop frozen center',dockerResume:'Complete maintenance and start',dockerUpgrade:'Upgrade to selected image and start',dockerMaintenanceObserve:'Observe original container maintenance',dockerMaintenanceIdle:'No container maintenance was requested.',dockerMaintenanceFrozen:'The selected center is frozen. Its original service is still running.',dockerMaintenanceStopped:'The original center is confirmed stopped. Its volume is retained. Complete maintenance or choose an upgrade.',dockerMaintenanceComplete:'Maintenance completed. The selected center was verified and is running.',dockerMaintenancePending:'The host is executing the original maintenance. Observe only; requests are not repeated.',dockerMaintenanceUnknown:'Maintenance outcome is unverified. Retain the original ID and volume. Stop, start and unfreeze will not be repeated.',dockerMaintenanceRefused:'Maintenance was refused before a change. Check for unsettled installation tasks.',dockerMaintenanceImportFirst:'Import an image with maintenance startup support before freezing this version.',
      dockerVerifyImport:'Verify originally loaded image',
      dockerTitle:'Docker installation',dockerHelp:'Select an image approved by the installed package. Upload and import an offline archive here, then enter an instance name and two published ports. A dedicated data volume is allocated automatically. Accepted jobs continue on the host when this page disconnects.',dockerRefresh:'Read images and installations',dockerImageLabel:'Console image',dockerArchiveLabel:'Offline image archive (tar.gz)',dockerUpload:'Upload selected image',dockerImport:'Import verified upload',dockerNameLabel:'New instance name',dockerHttpLabel:'Published console HTTPS port',dockerGrpcLabel:'Published gateway port',dockerDeploy:'Create and start console',dockerObserve:'Observe original deployment',dockerStatus:'Read selected container readiness',dockerAvailable:'Docker backend is available',dockerUnavailable:'Docker is unavailable on this host. Install and start Docker Engine first.',dockerImagePresent:'Imported',dockerImageMissing:'Not imported',dockerIdle:'No deployment job has been requested.',dockerUploading:'Uploading the image. Keep this page open; observe the original job after a disconnect.',dockerUploadReady:'The uploaded archive is verified and ready to import.',dockerUploadPending:'The upload is unverified. Retain the original job and file; do not upload again.',dockerPending:'The host is executing the original job. Observe only.',dockerApplied:'The original job has completed.',dockerUncertain:'The original outcome is unverified. Duplicate import and creation are blocked. Retain its ID and objects.',dockerRefused:'The host refused the job before a Docker write. Its record is retained. Check the name, image and ports.',dockerPollEnded:'This read-only observation has ended. Observe the original deployment again; no request will be resent.',dockerId:'Deployment job',dockerBadArchive:'Select the offline image archive for this approved version.',
      maintenanceCancel:'Cancel maintenance and keep services running',maintenanceFinalizeCancel:'Finish verified cancellation',maintenanceFrozen:'New writes are frozen. Services remain running.',maintenanceStopped:'Services are confirmed stopped. Verify the installation to release the freeze.',maintenanceCompleteDone:'Maintenance is complete. Services remain stopped.',maintenanceCancelled:'Maintenance was cancelled. The original ID rejects delayed actions; service states are unchanged.',maintenanceCancelPending:'Cancellation is verified. The retained freeze still needs to be released.',maintenancePendingNotice:'The request outcome is unverified. Retain its original ID; no stop will be resent. If no native stop intent exists, this maintenance can be cancelled.',
      maintenanceTitle:'Host maintenance',maintenanceHelp:'Includes every natively installed center on this host. Freeze new writes before checking tasks and stopping services. Verify the installed package before releasing the freeze. If an outcome is uncertain, observe the original operation without repeating the stop. Older packages require administrator offline adoption after all centers are stopped.',
      maintenanceObserve:'Observe original maintenance',maintenanceBegin:'Freeze writes on all centers',maintenanceStop:'Stop frozen centers',maintenanceComplete:'Verify installation and release freeze',maintenanceNone:'No maintenance operation was accepted.',maintenanceUnknown:'Maintenance outcome is unverified. Retain the original operation and observe only.',maintenanceActive:'Maintenance operation',maintenance:'Host maintenance is frozen. Observe the original operation below.',pollStopped:'This read-only check has ended; the service is not ready. Read service status again to observe it. No start will be repeated.',
      privilegeRequired:'This action requires host administrative access. Use an authorized host account and enable administrative access at the top of the host page, then try again. The command was not executed.',
      separateHost:'Use a different hostname for the console and host management page to keep login sessions separate.',
      portLabel:'Initial setup listener port',
      ready:'Host HTTPS is ready. Verify the published address and certificate after opening it.',waiting:'The service is not ready yet. Checking read-only; it will not be started again.',inactive:'The service is not started.',unverified:'Service readiness could not be verified. Observe the original service and records.',notReady:'Read service status and verify that host HTTPS is ready before opening the page.',
      title:'YOFC console installation',intro:'Use host administrator authorization to start setup and obtain the installation credential. Configure the database, organization account and certificates on the console setup page.',
      targetTitle:'1. Select installation',modeLabel:'Deployment',nativeOption:'Installed host package',dockerOption:'Docker container on this host',instanceLabel:'Instance name',instanceHelp:'Leave empty for the default installation. Each center needs its own instance, database and ports.',refresh:'Read container list',containerLabel:'Console container',status:'Read service status',start:'Start setup service',
      credentialTitle:'4. Obtain installation credential',credentialHelp:'Host administrator authorization is required. This only reads an existing credential; it never creates a certificate, resets installation or resends initialization. Configured installations refuse download.',read:'Read installation information',identityLabel:'Installation ID',fingerprintLabel:'Temporary server certificate SHA-256',download:'Download installation credential',privateNotice:'This file contains an installation token. Store it in a protected personal directory. Do not forward it or put it in a shared directory.',
      openTitle:'5. Open console setup',urlLabel:'Console HTTPS address',trustNotice:'Use a different hostname from this host management page to keep login sessions separate. Match the SHA-256 in browser security details with the host-provided fingerprint above before uploading the credential. The console page itself cannot establish this trust.',open:'Open setup page',
      needHost:'Open this tool from an authenticated host management session.',failure:'The operation could not be verified. Observe the original service and records before taking further action. Do not reset installation or resend initialization.',busy:'Checking through the host management channel…',readDone:'Installation information read. The credential is ready to download.',downloadDone:'The credential was handed to the browser download. Keep it private.',badTarget:'Select a valid installation target.',badUrl:'Enter an HTTPS address without a username or password.',containerHelp:'Start the container in the deployment platform first. It must have the YOFC console label.'
    }
  };
  let language = navigator.language.startsWith('zh') ? 'zh' : 'en';
  let credential = null, credentialTarget = null, busy = false;
  let observation = 0, poll = null, readyTarget = null, lastStatus = null;
  const maintenanceKey='yofc.console.host-maintenance.v1';
  let maintenanceId=null, maintenancePhase=null, maintenanceUncertain=false, maintenanceSaved=null;
  let maintenanceFence=false, maintenanceNativeStop=false, maintenanceOutcome=null, maintenanceStorageBroken=false;
  try {
    const raw=localStorage.getItem(maintenanceKey);
    if(raw){
      const value=JSON.parse(raw);
      if(!/^[a-f0-9-]{36}$/.test(value.operationId) || !['BEGIN_REQUESTED','FROZEN','STOP_REQUESTED','STOPPED','COMPLETE_REQUESTED','CANCEL_REQUESTED','COMPLETE'].includes(value.step)) throw new Error('invalid');
      maintenanceSaved=value;maintenanceId=value.operationId;maintenanceUncertain=true;
    }
  } catch { maintenanceStorageBroken=true;maintenanceUncertain=true; }
  const dockerKey='yofc.console.docker-deployment.v1';
  let dockerSaved=null,dockerRecord=null,dockerBroken=false,dockerOverview=null,dockerPoll=null,dockerGeneration=0;
  const dmKey='yofc.console.docker-maintenance.v1';
  let dmSaved=null,dmRecord=null,dmBroken=false,dmPoll=null,dmGeneration=0;
  try{
    const raw=localStorage.getItem(dmKey);
    if(raw){const value=JSON.parse(raw);if(!/^[a-f0-9-]{36}$/.test(value.operationId)||!/^[a-f0-9-]{36}$/.test(value.deploymentId)||!['BEGIN_REQUESTED','FROZEN','STOP_REQUESTED','STOPPED','RESUME_REQUESTED','UPGRADE_REQUESTED','VERIFY_START_REQUESTED','COMPLETE','REFUSED'].includes(value.step))throw new Error('invalid');dmSaved=value;}
  }catch{dmBroken=true;}
  try {
    const raw=localStorage.getItem(dockerKey);
    if(raw){const value=JSON.parse(raw);if(!/^[a-f0-9-]{36}$/.test(value.operationId)||!['IMPORT','DEPLOY'].includes(value.kind)||!['UPLOAD_REQUESTED','UPLOADED','IMPORT_REQUESTED','VERIFY_REQUESTED','DEPLOY_REQUESTED','APPLIED','REFUSED'].includes(value.step))throw new Error('invalid');dockerSaved=value;}
  }catch{dockerBroken=true;}
  const controls=['mode','instance','port','container','refresh','status','start','read','download','open','maintenanceObserve','maintenanceBegin','maintenanceStop','maintenanceComplete','maintenanceCancel',
    'dockerRefresh','dockerImage','dockerArchive','dockerUpload','dockerImport','dockerVerifyImport','dockerName','dockerHttp','dockerGrpc','dockerDeploy','dockerObserve','dockerStatus',
    'dockerFreeze','dockerStop','dockerResume','dockerUpgrade','dockerMaintenanceObserve','dockerVerifyStarted',
    'nativeRefresh','nativePackage','nativeFile','nativeUpload','nativeApprove','nativeInstall','nativeObserve','nativeRetire','nativeVerify','nativeFinalize','nativeFinish'];
  const nativeUi=window.yofcNativePackages({invoke,action:fn=>action(async()=>{await fn();$('message').textContent='';}),getLanguage:()=>language,getBusy:()=>busy,
    onInstalled:()=>{clear();retireObservation();$('service').textContent='';}});
  const text = key => words[language][key];
  function render() {
    document.documentElement.lang = language === 'zh' ? 'zh-CN' : 'en';
    for (const [key, value] of Object.entries(words[language])) if ($(key)) $(key).textContent = value;
    $('language').textContent = language === 'zh' ? 'English' : '中文';
    if (lastStatus) showStatus(lastStatus);
    renderMaintenance();
    renderDocker();
    nativeUi.render();
  }
  function target() {
    if ($('mode').value === 'docker') {
      const id = $('container').value;
      if (!/^[a-f0-9]{64}$/.test(id)) throw new Error('target');
      return ['--container', id];
    }
    const instance = $('instance').value;
    if (instance && !/^[a-z0-9][a-z0-9-]{0,31}$/.test(instance)) throw new Error('target');
    return instance ? ['--instance', instance] : [];
  }
  function clear() {
    credential = null; credentialTarget = null;
    $('identity').textContent = '—'; $('fingerprint').textContent = '—'; $('download').disabled = true;
  }
  function retireObservation() {
    observation++; clearTimeout(poll); poll=null; readyTarget=null; lastStatus=null;
  }
  function showStatus(result) {
    lastStatus=result;
    const ready=result.state==='active' && result.ready===true && result.readiness==='READY';
    readyTarget=ready ? JSON.stringify(target()) : null;
    $('service').textContent=text(result.readiness==='MAINTENANCE' ? 'maintenance' : ready ? 'ready' : result.state==='inactive' ? 'inactive' : result.readiness==='UNVERIFIED' || result.state==='failed' ? 'unverified' : 'waiting');
  }
  function scheduleObservation(args, generation, remaining=15) {
    if (generation!==observation) return;
    if (!remaining) { $('service').textContent=text('pollStopped'); return; }
    poll=setTimeout(() => {
      if (generation!==observation) return;
      if (busy) { scheduleObservation(args,generation,remaining-1); return; }
      void action(async () => {
        const result=await invoke('status',args);
        if (generation!==observation) return;
        showStatus(result); $('message').textContent='';
        if (!readyTarget && ['active','activating'].includes(result.state)) scheduleObservation(args,generation,remaining-1);
      });
    },1000);
  }
  async function invoke(action, args) {
    if (!window.cockpit) throw new Error('host');
    const output = await window.cockpit.spawn(['/usr/bin/python3', '/usr/lib/kura-management-console/installer.py', action, ...args], {superuser:'require', err:'message'});
    return JSON.parse(output);
  }
  async function action(fn) {
    if (busy) return;
    busy = true;
    for (const id of controls) $(id).disabled = true;
    $('message').textContent = text('busy');
    try { await fn(); }
    catch (error) {
      retireObservation();
      if (error?.problem === 'access-denied') { clear(); $('message').textContent = text('privilegeRequired'); }
      else $('message').textContent = text(error?.message === 'target' ? 'badTarget' : !window.cockpit ? 'needHost' : 'failure');
    }
    finally {
      busy = false;
      for (const id of controls) $(id).disabled = false;
      $('download').disabled = !credential;
      maintenanceButtons();
      dockerButtons();
      nativeUi.buttons();
    }
  }
  function changed() {
    clear(); retireObservation(); $('service').textContent = ''; $('message').textContent = '';
    const docker = $('mode').value === 'docker';
    $('instanceRow').hidden = docker; $('nativeActions').hidden = docker; $('containerRow').hidden = !docker;
    $('maintenanceActions').hidden=docker;
    $('dockerActions').hidden=!docker;
    $('nativePackages').hidden=docker;
    $('nativeLaunch').hidden=docker;
    nativeUi.changed();
    dockerGeneration++;clearTimeout(dockerPoll);dockerPoll=null;
    dmGeneration++;clearTimeout(dmPoll);dmPoll=null;
    dockerButtons();
  }
  $('language').onclick = () => {language = language === 'zh' ? 'en' : 'zh'; render();};
  $('mode').onchange = changed; $('instance').oninput = changed; $('container').onchange = changed;
  $('refresh').onclick = () => action(async () => {
    clear(); retireObservation(); const rows = await invoke('containers', []); $('container').replaceChildren();
    for (const row of rows) {
      if (!/^[a-f0-9]{64}$/.test(row.id)) continue;
      const option = document.createElement('option'); option.value = row.id;
      option.textContent = `${row.name} (${row.state}, ${row.id.slice(0,12)})`; $('container').append(option);
    }
    $('message').textContent = text('containerHelp');
  });
  for (const name of ['status','start']) $(name).onclick = () => action(async () => {
    if ($('mode').value !== 'native') throw new Error('target');
    retireObservation(); const generation=observation;
    const args = target();
    const observeArgs=[...args];
    if (name === 'start') {
      const port = Number($('port').value);
      if (!Number.isInteger(port) || port < 1024 || port > 65535) throw new Error('target');
      args.push('--port', String(port));
      nativeUi.serviceMayChange();
    }
    const result = await invoke(name, args);
    const actualPort = result.httpPort || result.httpAddress?.match(/:(\d+)$/)?.[1];
    if (name === 'start' && actualPort) $('port').value = actualPort;
    showStatus(result);
    $('message').textContent = '';
    if (!readyTarget && ['active','activating'].includes(result.state)) scheduleObservation(observeArgs,generation);
  });
  $('read').onclick = () => action(async () => {
    clear(); const args = target(), result = await invoke('credential', args);
    if (result.schemaVersion !== 1 || !/^[0-9a-f-]{36}$/i.test(result.installationId) || !/^[0-9a-f]{64}$/i.test(result.token) || !/^[0-9a-f]{64}$/i.test(result.certificateFingerprint)) throw new Error('invalid');
    credential = result; credentialTarget = JSON.stringify(args);
    $('identity').textContent = result.installationId; $('fingerprint').textContent = result.certificateFingerprint;
    $('message').textContent = text('readDone');
  });
  $('download').onclick = () => {
    if (!credential || credentialTarget !== JSON.stringify(target())) return;
    const url = URL.createObjectURL(new Blob([JSON.stringify(credential)], {type:'application/json'}));
    const anchor = document.createElement('a'); anchor.href = url;
    anchor.download = `yofc-installation-${credential.installationId}.json`; anchor.click();
    setTimeout(() => URL.revokeObjectURL(url), 1000); $('message').textContent = text('downloadDone');
  };
  function maintenanceButtons() {
    const terminal=maintenancePhase==='COMPLETE' && !maintenanceFence;
    $('maintenanceBegin').disabled=busy || maintenanceStorageBroken || maintenanceUncertain || !!maintenanceId && !terminal;
    $('maintenanceStop').disabled=busy || maintenanceUncertain || maintenancePhase!=='FROZEN';
    $('maintenanceComplete').disabled=busy || maintenanceUncertain || maintenancePhase!=='STOPPED';
    const finalize=maintenancePhase==='COMPLETE' && maintenanceFence && maintenanceOutcome?.startsWith('CANCELLED_');
    const cancellable=['FROZEN','NOT_ACCEPTED','UNOBSERVED'].includes(maintenancePhase) || !maintenancePhase && maintenanceSaved?.step==='BEGIN_REQUESTED';
    $('maintenanceCancel').textContent=text(finalize?'maintenanceFinalizeCancel':'maintenanceCancel');
    $('maintenanceCancel').disabled=busy || maintenanceStorageBroken || !maintenanceId || maintenanceNativeStop || !finalize && (!cancellable || maintenanceSaved?.step==='CANCEL_REQUESTED');
  }
  function rememberMaintenance(id,step,outcome=null) {
    const value={operationId:id,step,...(outcome?{outcome}: {})};
    localStorage.setItem(maintenanceKey,JSON.stringify(value));
    maintenanceSaved=value;
  }
  function renderMaintenance() {
    const cancelled=maintenanceOutcome?.startsWith('CANCELLED_');
    const key=maintenancePhase==='COMPLETE' ? cancelled ? maintenanceFence ? 'maintenanceCancelPending' : 'maintenanceCancelled' : 'maintenanceCompleteDone' :
      maintenanceUncertain ? 'maintenancePendingNotice' : maintenancePhase==='FROZEN' ? 'maintenanceFrozen' : maintenancePhase==='STOPPED' ? 'maintenanceStopped' : maintenanceId ? 'maintenanceUnknown' : 'maintenanceNone';
    $('maintenanceResult').textContent=text(key)+(maintenanceId?` ${text('maintenanceActive')}: ${maintenanceId}`:'');
  }
  function showMaintenance(result,command) {
    const record=result.active || result.operation || (result.operationId ? result : null);
    if(!record){
      maintenanceUncertain=!!maintenanceId;renderMaintenance();return;
    }
    if(!/^[a-f0-9-]{36}$/.test(record.operationId))throw new Error('invalid');
    if(maintenanceSaved?.step.endsWith('_REQUESTED') && maintenanceSaved.operationId!==record.operationId){
      maintenanceUncertain=true;renderMaintenance();return; // Retain the unresolved original request.
    }
    maintenanceId=record.operationId;maintenancePhase=record.phase;maintenanceOutcome=record.outcome||null;
    maintenanceFence=typeof result.activeFencePresent==='boolean' ? result.activeFencePresent : !['COMPLETE','NOT_ACCEPTED','UNOBSERVED'].includes(record.phase);
    maintenanceNativeStop=Object.keys(record.stopIntents||{}).length>0 && record.phase!=='COMPLETE';
    const saved=maintenanceSaved?.operationId===maintenanceId ? maintenanceSaved.step : null;
    if(record.phase==='COMPLETE' && !maintenanceFence){
      rememberMaintenance(maintenanceId,'COMPLETE',maintenanceOutcome);maintenanceUncertain=false;
    } else if(record.phase==='STOPPED' && saved!=='COMPLETE_REQUESTED'){
      rememberMaintenance(maintenanceId,'STOPPED');maintenanceUncertain=false;
    } else if(record.phase==='FROZEN' && (command==='maintenance-begin' || saved==='BEGIN_REQUESTED' || saved==='FROZEN')){
      rememberMaintenance(maintenanceId,'FROZEN');maintenanceUncertain=false;
    } else {maintenanceUncertain=true;}
    // A missing receipt does not prove a queued request was never delivered.
    renderMaintenance();
  }
  for (const [button,selectedCommand] of [['maintenanceObserve','maintenance-status'],['maintenanceBegin','maintenance-begin'],['maintenanceStop','maintenance-stop'],['maintenanceComplete','maintenance-complete'],['maintenanceCancel','maintenance-cancel']]) {
    $(button).onclick=()=>action(async()=>{
      retireObservation();
      if ($('mode').value!=='native') throw new Error('target');
      const command=selectedCommand==='maintenance-cancel' && maintenancePhase==='COMPLETE' && maintenanceFence ? 'maintenance-cancel-finalize' : selectedCommand;
      if (command==='maintenance-begin') {
        maintenanceId=crypto.randomUUID();maintenancePhase=null;maintenanceOutcome=null;
        maintenanceFence=false;maintenanceNativeStop=false;
      }
      const args=maintenanceId ? ['--operation',maintenanceId] : [];
      if(command!=='maintenance-status'){
        const step=command==='maintenance-begin'?'BEGIN_REQUESTED':command==='maintenance-stop'?'STOP_REQUESTED':command==='maintenance-complete'?'COMPLETE_REQUESTED':'CANCEL_REQUESTED';
        rememberMaintenance(maintenanceId,step);maintenanceUncertain=true; // Before any dispatch.
      }
      try { showMaintenance(await invoke(command,args),command);$('message').textContent=''; }
      catch(error){maintenanceUncertain=true;renderMaintenance();throw error;}
    });
  }
  $('url').placeholder = 'https://console.example.com:18443';
  $('open').onclick = () => {
    try {
      const url = new URL($('url').value);
      if (url.protocol !== 'https:' || url.username || url.password || !['/','/setup'].includes(url.pathname) || url.search || url.hash) throw new Error('url');
      if (url.hostname === window.location.hostname) { $('message').textContent = text('separateHost'); return; }
      if (readyTarget!==JSON.stringify(target())) { $('message').textContent=text('notReady'); return; }
      url.pathname = '/setup'; window.open(url.href, '_blank', 'noopener,noreferrer');
    } catch { $('message').textContent = text('badUrl'); }
  };
  function rememberDocker(operationId,kind,step){
    const value={operationId,kind,step};localStorage.setItem(dockerKey,JSON.stringify(value));dockerSaved=value;
  }
  function dockerButtons(){
    const terminal=dockerRecord && ['APPLIED','REFUSED'].includes(dockerRecord.phase);
    const unresolved=!!dockerSaved&&!terminal;
    const available=dockerOverview?.engine.available===true;
    const image=dockerOverview?.images.find(row=>row.key===$('dockerImage').value);
    const maintenanceActive=dockerOverview?.operations.some(row=>row.kind==='MAINTENANCE')||!!dmSaved&&!['COMPLETE','REFUSED'].includes(dmRecord?.phase);
    const blocked=busy||dockerBroken||dmBroken||!available||maintenanceActive;
    $('dockerUpload').disabled=blocked||unresolved||!image;
    $('dockerDeploy').disabled=blocked||unresolved||!image?.available;
    $('dockerImport').disabled=blocked||dockerSaved?.step!=='UPLOADED'||dockerRecord?.phase!=='UPLOADING'||dockerRecord.uploadReady!==true;
    $('dockerVerifyImport').disabled=blocked||dockerRecord?.kind!=='IMPORT'||dockerRecord?.phase!=='UNKNOWN'||dockerRecord?.receipts?.load?.commandReturnedSuccess!==true;
    $('dockerObserve').disabled=busy||!dockerSaved;
    $('dockerStatus').disabled=busy||!dockerOverview?.deployments.some(row=>row.containerId===$('container').value);
    dmButtons();
  }
  function renderDocker(){
    if(dockerOverview)$('dockerEngine').textContent=text(dockerOverview.engine.available?'dockerAvailable':'dockerUnavailable');
    let key=dockerBroken?'dockerUncertain':!dockerSaved?'dockerIdle':!dockerRecord?'dockerUncertain':
      dockerRecord.phase==='UPLOADING'?dockerRecord.uploadReady && dockerSaved.step==='UPLOADED'?'dockerUploadReady':'dockerUploadPending':
      ['APPLIED','REFUSED'].includes(dockerRecord.phase)?dockerRecord.phase==='APPLIED'?'dockerApplied':'dockerRefused':
      ['ACCEPTED','PENDING'].includes(dockerRecord.phase)?'dockerPending':'dockerUncertain';
    $('dockerResult').textContent=text(key)+(dockerSaved?` ${text('dockerId')}: ${dockerSaved.operationId}`:'');
    renderDm();
  }
  function showDockerRecord(record){
    if(!dockerSaved||record.operationId!==dockerSaved.operationId)throw new Error('invalid');
    dockerRecord=record;
    if(['APPLIED','REFUSED'].includes(record.phase))rememberDocker(record.operationId,dockerSaved.kind,record.phase);
    else if(record.phase==='UPLOADING'&&record.uploadReady&&dockerSaved.step==='UPLOAD_REQUESTED')rememberDocker(record.operationId,'IMPORT','UPLOADED');
    renderDocker();
  }
  async function refreshDocker(){
    const previous=$('dockerImage').value;
    dockerOverview=await invoke('docker-overview',[]);$('dockerImage').replaceChildren();
    for(const row of dockerOverview.images){
      const option=document.createElement('option');option.value=row.key;
      option.textContent=`${row.title} (${text(row.available?'dockerImagePresent':'dockerImageMissing')})`;$('dockerImage').append(option);
    }
    if(dockerOverview.images.some(row=>row.key===previous))$('dockerImage').value=previous;
    // An unresolved host record also fences a second browser profile.
    const deploymentJobs=dockerOverview.operations.filter(row=>['IMPORT','DEPLOY'].includes(row.kind));
    if((!dockerSaved||['APPLIED','REFUSED'].includes(dockerSaved.step))&&deploymentJobs.length){
      const original=deploymentJobs[0];
      rememberDocker(original.operationId,original.kind,original.kind==='IMPORT'?'IMPORT_REQUESTED':'DEPLOY_REQUESTED');
      showDockerRecord(original);
    }
    const maintenance=dockerOverview.operations.find(row=>row.kind==='MAINTENANCE');
    if(maintenance&&(!dmSaved||['COMPLETE','REFUSED'].includes(dmSaved.step))){
      rememberDm(maintenance.operationId,maintenance.deploymentId,'BEGIN_REQUESTED');showDm(maintenance);
    }
    const current=$('container').value;const rows=await invoke('containers',[]);$('container').replaceChildren();
    for(const row of rows){if(!/^[a-f0-9]{64}$/.test(row.id))continue;const option=document.createElement('option');option.value=row.id;option.textContent=`${row.name} (${row.state}, ${row.id.slice(0,12)})`;$('container').append(option);}
    if(rows.some(row=>row.id===current))$('container').value=current;
    if(current!==$('container').value){clear();retireObservation();}
    renderDocker();
  }
  function pollDocker(generation,remaining=15){
    if(generation!==dockerGeneration)return;
    if(!remaining){$('message').textContent=text('dockerPollEnded');return;}
    dockerPoll=setTimeout(()=>{
      if(generation!==dockerGeneration)return;
      if(busy){pollDocker(generation,remaining-1);return;}
      void action(async()=>{
        const record=await invoke('docker-operation',['--operation',dockerSaved.operationId]);
        if(generation!==dockerGeneration)return;
        showDockerRecord(record);$('message').textContent='';
        if(record.phase==='APPLIED')await refreshDocker();
        else if(['ACCEPTED','PENDING'].includes(record.phase))pollDocker(generation,remaining-1);
      });
    },1000);
  }
  $('dockerImage').onchange=()=>{dockerButtons();};
  $('dockerRefresh').onclick=()=>action(async()=>{await refreshDocker();if(dockerSaved)showDockerRecord(await invoke('docker-operation',['--operation',dockerSaved.operationId]));$('message').textContent='';});
  $('dockerUpload').onclick=()=>action(async()=>{
    const image=dockerOverview?.images.find(row=>row.key===$('dockerImage').value),file=$('dockerArchive').files[0];
    if(!image||!file||file.size!==image.archiveSize||file.size>256*1024*1024){$('message').textContent=text('dockerBadArchive');return;}
    const id=crypto.randomUUID();rememberDocker(id,'IMPORT','UPLOAD_REQUESTED');dockerRecord=null;renderDocker();
    const prepared=await invoke('docker-upload-prepare',['--operation',id,'--image',image.key]);showDockerRecord(prepared);
    if(prepared.phase!=='UPLOADING'||prepared.uploadPath!==`/var/lib/yofc-console-installations/uploads/${id}.tar.gz`)throw new Error('invalid');
    $('message').textContent=text('dockerUploading');
    const channel=window.cockpit.file(prepared.uploadPath,{binary:true,superuser:'require'});
    try{await channel.replace(new Uint8Array(await file.arrayBuffer()),'-');}finally{channel.close();}
    showDockerRecord(await invoke('docker-operation',['--operation',id]));$('message').textContent='';
  });
  $('dockerImport').onclick=()=>action(async()=>{
    const id=dockerSaved.operationId;rememberDocker(id,'IMPORT','IMPORT_REQUESTED');
    showDockerRecord(await invoke('docker-import',['--operation',id]));$('message').textContent='';pollDocker(++dockerGeneration);
  });
  $('dockerVerifyImport').onclick=()=>action(async()=>{
    const id=dockerSaved.operationId;rememberDocker(id,'IMPORT','VERIFY_REQUESTED');
    showDockerRecord(await invoke('docker-verify-import',['--operation',id]));
    if(dockerRecord.phase==='APPLIED')await refreshDocker();$('message').textContent='';
  });
  $('dockerDeploy').onclick=()=>action(async()=>{
    const name=$('dockerName').value,http=Number($('dockerHttp').value),grpc=Number($('dockerGrpc').value),image=$('dockerImage').value;
    if(!/^[a-z0-9][a-z0-9-]{0,31}$/.test(name)||![http,grpc].every(value=>Number.isInteger(value)&&value>=1024&&value<=65535)||http===grpc)throw new Error('target');
    const id=crypto.randomUUID();rememberDocker(id,'DEPLOY','DEPLOY_REQUESTED');dockerRecord=null;clear();retireObservation();renderDocker();
    showDockerRecord(await invoke('docker-deploy',['--operation',id,'--image',image,'--name',name,'--http-port',String(http),'--grpc-port',String(grpc)]));
    $('message').textContent='';pollDocker(++dockerGeneration);
  });
  $('dockerObserve').onclick=()=>action(async()=>{
    const record=await invoke('docker-operation',['--operation',dockerSaved.operationId]);showDockerRecord(record);
    if(record.phase==='APPLIED')await refreshDocker();
    else if(['ACCEPTED','PENDING'].includes(record.phase))pollDocker(++dockerGeneration);
    $('message').textContent='';
  });
  $('dockerStatus').onclick=()=>action(async()=>{
    clear();retireObservation();const row=dockerOverview?.deployments.find(value=>value.containerId===$('container').value);
    if(!row)throw new Error('target');showStatus(await invoke('docker-status',['--operation',row.operationId]));$('message').textContent='';
  });
  function rememberDm(operationId,deploymentId,step){
    const value={operationId,deploymentId,step};localStorage.setItem(dmKey,JSON.stringify(value));dmSaved=value;
  }
  function dmButtons(){
    const selected=dockerOverview?.deployments.find(row=>row.containerId===$('container').value);
    const supported=dockerOverview?.images.some(row=>row.available&&row.maintenanceProtocolVersion===2);
    const terminal=dmRecord&&['COMPLETE','REFUSED'].includes(dmRecord.phase);
    const blocked=busy||dmBroken||dockerOverview?.engine.available!==true;
    $('dockerFreeze').disabled=blocked||!selected||!supported||!!dmSaved&&!terminal||dockerOverview?.operations.some(row=>row.kind!=='MAINTENANCE');
    $('dockerStop').disabled=blocked||dmRecord?.phase!=='FROZEN'||dmSaved?.step!=='FROZEN';
    $('dockerResume').disabled=blocked||dmRecord?.phase!=='STOPPED'||dmSaved?.step!=='STOPPED'||dmRecord?.oldGeneration?.image?.maintenanceProtocolVersion!==2;
    const image=dockerOverview?.images.find(row=>row.key===$('dockerImage').value);
    $('dockerUpgrade').disabled=blocked||dmRecord?.phase!=='STOPPED'||dmSaved?.step!=='STOPPED'||!image?.available||image.maintenanceProtocolVersion!==2||image.imageId===dmRecord?.oldGeneration?.image?.imageId;
    $('dockerMaintenanceObserve').disabled=busy||!dmSaved;
    const intents=Object.keys(dmRecord?.intents||{}),receipts=dmRecord?.receipts||{};
    $('dockerVerifyStarted').disabled=blocked||dmRecord?.phase!=='UNKNOWN'||!dmRecord?.newGeneration||!dmRecord?.stopReceipt||
      receipts['start-generation']?.commandReturnedSuccess!==true||intents.includes('complete')||intents.length!==Object.keys(receipts).length||
      !intents.every(key=>Object.hasOwn(receipts,key))||dmSaved?.step==='VERIFY_START_REQUESTED';
  }
  function renderDm(){
    const supported=dockerOverview?.images.some(row=>row.available&&row.maintenanceProtocolVersion===2);
    const key=dmBroken?'dockerMaintenanceUnknown':!dmSaved?dockerOverview&&!supported?'dockerMaintenanceImportFirst':'dockerMaintenanceIdle':dmRecord?.phase==='COMPLETE'?'dockerMaintenanceComplete':
      dmRecord?.phase==='REFUSED'?'dockerMaintenanceRefused':dmRecord?.phase==='FROZEN'&&dmSaved.step==='FROZEN'?'dockerMaintenanceFrozen':
      dmRecord?.phase==='STOPPED'&&dmSaved.step==='STOPPED'?'dockerMaintenanceStopped':
      ['PENDING','ACCEPTED'].includes(dmRecord?.phase)?'dockerMaintenancePending':'dockerMaintenanceUnknown';
    $('dockerMaintenanceResult').textContent=text(key)+(dmSaved?` ${text('maintenanceActive')}: ${dmSaved.operationId}`:'');
  }
  function showDm(record){
    if(!dmSaved||record.operationId!==dmSaved.operationId||record.deploymentId&&record.deploymentId!==dmSaved.deploymentId)throw new Error('invalid');
    dmRecord=record;
    if(['COMPLETE','REFUSED'].includes(record.phase))rememberDm(dmSaved.operationId,dmSaved.deploymentId,record.phase);
    else if(record.phase==='FROZEN'&&['BEGIN_REQUESTED','FROZEN'].includes(dmSaved.step))rememberDm(dmSaved.operationId,dmSaved.deploymentId,'FROZEN');
    else if(record.phase==='STOPPED'&&['BEGIN_REQUESTED','FROZEN','STOP_REQUESTED','STOPPED'].includes(dmSaved.step))rememberDm(dmSaved.operationId,dmSaved.deploymentId,'STOPPED');
    renderDm();
  }
  function pollDm(generation,remaining=15){
    if(generation!==dmGeneration)return;
    if(!remaining){$('message').textContent=text('dockerPollEnded');return;}
    dmPoll=setTimeout(()=>{
      if(generation!==dmGeneration)return;
      if(busy){pollDm(generation,remaining-1);return;}
      void action(async()=>{
        const record=await invoke('docker-operation',['--operation',dmSaved.operationId]);if(generation!==dmGeneration)return;
        showDm(record);$('message').textContent='';
        if(['PENDING','ACCEPTED'].includes(record.phase))pollDm(generation,remaining-1);
        else if(record.phase==='COMPLETE')await refreshDocker();
      });
    },1000);
  }
  for(const [button,stage] of [['dockerFreeze','begin'],['dockerStop','stop'],['dockerResume','resume'],['dockerUpgrade','upgrade'],['dockerVerifyStarted','verify-start'],['dockerMaintenanceObserve','observe']]){
    $(button).onclick=()=>action(async()=>{
      if(stage==='begin'){
        const selected=dockerOverview?.deployments.find(row=>row.containerId===$('container').value);if(!selected)throw new Error('target');
        rememberDm(crypto.randomUUID(),selected.operationId,'BEGIN_REQUESTED');dmRecord=null;
      }
      const id=dmSaved.operationId,args=['--operation',id];
      if(stage==='begin')args.push('--deployment',dmSaved.deploymentId);
      if(stage==='upgrade')args.push('--image',$('dockerImage').value);
      if(stage!=='observe')rememberDm(id,dmSaved.deploymentId,stage==='begin'?'BEGIN_REQUESTED':stage==='stop'?'STOP_REQUESTED':stage==='resume'?'RESUME_REQUESTED':stage==='verify-start'?'VERIFY_START_REQUESTED':'UPGRADE_REQUESTED');
      renderDm();
      const record=await invoke(stage==='observe'?'docker-operation':'docker-maintenance-'+stage,args);showDm(record);$('message').textContent='';
      if(['PENDING','ACCEPTED'].includes(record.phase))pollDm(++dmGeneration);
      else if(record.phase==='COMPLETE')await refreshDocker();
    });
  }
  render(); changed();maintenanceButtons();dockerButtons();
  if (!window.cockpit) $('message').textContent = text('needHost');
})();
