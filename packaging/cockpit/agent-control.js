/* Public local journal only. Root pairing and durable grant belong to the agent. */
(() => {
  window.yofcAgentControl = ({invoke,action,getLanguage,getBusy}) => {
    const $=id=>document.getElementById(id),key='yofc.kura.local-switch-grant.v1';
    const uuid=/^(?!00000000-0000-0000-0000-000000000000$)[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12}$/;
    const words={
      zh:{agentTitle:'网关本机权限',agentHelp:'此项管理当前主机已安装的 Kura 与独立 agent。授权后，本机 Kura 管理员可控制中心通道；通道仍保持关闭，须随后在 Kura 远程管理页明确开启。',
        agentRefresh:'读取本机身份与服务',agentGrant:'授权本机 Kura 控制开关',agentObserve:'核对原授权',agentComplete:'补全原授权记录',
        agentLegend:'确认本机权限',agentApproveLabel:'我同意让本机 Kura 管理员控制中心通道。授权会保留原身份、密钥和数据。',
        agentStopHelp:'授权前，请在主机“服务”页停止下表中的 Kura、独立 agent 和两个材料 socket，并等待连接退出。授权完成后，在“服务”页重新启动它们；远程通道保持关闭。',
        idle:'先读取当前主机的身份。',ready:'本机身份已核实，所需服务已停止，可明确授权。',running:'本机身份已核实。请先在主机“服务”页停服并等待材料连接退出，然后重新读取。',
        granted:'原授权已发布。按主机服务状态完成启动后，可在 Kura 远程管理页控制通道。',pending:'原授权结果尚未闭环。保留原编号，仅核对状态或补全原记录；不会另建授权。',
        refused:'当前安装未通过完整核验。请保留原记录，完成原维护或恢复。',unknown:'本次结果尚未核实。原编号已保留，不会重发授权或改用新编号。',
        foreign:'保存的请求属于另一安装身份。本页已阻止查询和补全该请求，请回到原主机。',changed:'本机身份或源码在确认后发生变化。请重新核对并明确确认。',conflict:'本机已有另一个原授权编号。保存的请求已保留，请核对原记录。',
        storage:'本地公开请求记录无法可靠保存。已停止提交；可继续读取本机身份。',unsupported:'当前浏览器不能串行保护本地请求记录。请使用支持 Web Locks 的浏览器继续；读取仍可用。',
        unavailable:'无法核实本机原生 agent 或服务。请确认已安装支持本机权限管理的配对版本。',identity:'本机安装 ID',request:'原授权 ID',source:'当前源码指纹',services:'服务状态',connections:'连接',pid:'进程'},
      en:{agentTitle:'Local gateway permission',agentHelp:'This manages Kura and the separate agent installed on this host. The grant lets local Kura administrators control the center channel. The channel stays off until explicitly enabled on the Kura remote management page.',
        agentRefresh:'Read local identity and services',agentGrant:'Authorize local Kura switch control',agentObserve:'Observe original grant',agentComplete:'Complete original grant records',
        agentLegend:'Confirm local permission',agentApproveLabel:'I authorize local Kura administrators to control the center channel. The original identity, keys and data are retained.',
        agentStopHelp:'Before granting, use the host Services page to stop Kura, the separate agent and both material sockets listed below, and wait for connections to drain. After the grant, start them on the Services page. The remote channel stays off.',
        idle:'Read the current host identity first.',ready:'The local identity is verified and the required services are stopped. An explicit grant is available.',running:'The local identity is verified. Stop the required services and drain material connections on the host Services page, then read again.',
        granted:'The original grant is published. Complete startup using the host service states, then control the channel on the Kura remote management page.',pending:'The original grant is unsettled. Keep its ID; observe or complete its original records. Another grant will not be created.',
        refused:'The installation did not pass full verification. Retain its records and complete the original maintenance or recovery.',unknown:'The outcome is unverified. Its original ID is retained; the grant will not be resent or replaced with a new ID.',
        foreign:'The saved request belongs to another installation. Query and completion are blocked here. Return to its original host.',changed:'The local identity or source changed after confirmation. Review it and explicitly confirm again.',conflict:'This host has a different original grant ID. The saved request is retained; inspect its original records.',
        storage:'The public local request journal cannot be saved reliably. Submission is blocked; local identity can still be read.',unsupported:'This browser cannot serialize the local request journal. Use a browser with Web Locks to proceed. Reading remains available.',
        unavailable:'The local native agent or services could not be verified. Install a paired version that supports local permission management.',identity:'Local installation ID',request:'Original grant ID',source:'Current source fingerprint',services:'Service states',connections:'connections',pid:'process'}
    };
    let saved=null,broken=false,unsupported=false,current=null,notice='idle';
    const text=id=>words[getLanguage()][id];
    function decode(raw) {
      const v=JSON.parse(raw);
      if(!v || Object.keys(v).sort().join(',')!=='initializationId,requestId,schemaVersion,step' || v.schemaVersion!==1 || !uuid.test(v.initializationId) || !uuid.test(v.requestId) ||
          !['REQUESTED','COMPLETE_REQUESTED','OBSERVED','APPLIED'].includes(v.step))throw new Error('storage');
      return v;
    }
    try{const raw=localStorage.getItem(key);if(raw)saved=decode(raw);}catch{broken=true;notice='storage';}
    function persist(value) {
      try{
        const raw=JSON.stringify(value);localStorage.setItem(key,raw);
        if(localStorage.getItem(key)!==raw)throw new Error('storage');
        saved=decode(raw);
      }catch{broken=true;throw new Error('storage');}
    }
    function local() {return current && (!saved || current.initializationId===saved.initializationId);}
    function matching() {return local() && (!saved || !current.requestId || current.requestId===saved.requestId);}
    function buttons() {
      const idle=!getBusy(),same=matching(),canGrant=idle && !broken && !unsupported && !saved && same && current.permission==='NONE' && current.readiness==='READY' && current.allStopped===true;
      $('agentRefresh').disabled=!idle;
      $('agentApprove').disabled=!canGrant;$('agentGrant').disabled=!canGrant;
      $('agentObserve').disabled=!idle || broken || !same || !(saved || current?.requestId);
      $('agentComplete').disabled=!idle || broken || unsupported || !same || !saved || current.permission!=='PENDING' || current.requestId!==saved.requestId || current.allStopped!==true;
    }
    function render() {
      for(const id of ['agentTitle','agentHelp','agentRefresh','agentGrant','agentObserve','agentComplete','agentLegend','agentApproveLabel','agentStopHelp'])$(id).textContent=text(id);
      $('agentResult').textContent=text(broken?'storage':current && !local()?'foreign':current && !matching()?'conflict':unsupported?'unsupported':notice);
      const details=[];
      if(current)details.push(text('identity')+': '+current.initializationId,text('source')+': '+current.sourceManifestSha256);
      if(saved)details.push(text('request')+': '+saved.requestId);
      else if(current?.requestId)details.push(text('request')+': '+current.requestId);
      $('agentIdentity').textContent=details.join('\n');
      $('agentServices').textContent=current ? text('services')+'\n'+current.services.map(row=>
        `${row.unit}: ${row.state}${row.pid===null?'':` · ${text('pid')} ${row.pid}`}${row.connections===null?'':` · ${text('connections')} ${row.connections}`}`).join('\n') : '';
      buttons();
    }
    async function refresh() {
      const previous=current;current=null;
      const value=await invoke('agent-overview',[]);
      if(value.schemaVersion!==1 || !uuid.test(value.initializationId) || !Array.isArray(value.services) || typeof value.allStopped!=='boolean' ||
          !['NONE','PENDING','GRANTED'].includes(value.permission) || !['READY','REFUSED'].includes(value.readiness))throw new Error('unavailable');
      current=value;
      if(previous && (previous.initializationId!==value.initializationId || previous.sourceManifestSha256!==value.sourceManifestSha256))$('agentApprove').checked=false;
      notice=value.permission==='PENDING'?'pending':value.readiness!=='READY'?'refused':value.permission==='GRANTED'?'granted':saved?'unknown':value.allStopped?'ready':'running';
    }
    async function run(fn) {
      await action(async()=>{
        try{await fn();}
        catch(error){notice=['storage','unsupported','changed'].includes(error?.message)?error.message:saved?'unknown':'unavailable';if(notice==='storage')broken=true;if(notice==='unsupported')unsupported=true;}
        finally{render();}
      });
    }
    async function exclusive(fn) {
      if(!navigator.locks?.request)throw new Error('unsupported');
      await navigator.locks.request(key,async()=>{
        let latest;
        try{const raw=localStorage.getItem(key);latest=raw?decode(raw):null;}catch{throw new Error('storage');}
        if(JSON.stringify(latest)!==JSON.stringify(saved)){
          if(latest && (!saved || latest.initializationId===saved.initializationId && latest.requestId===saved.requestId)){
            saved=latest;await refresh();return;
          }
          throw new Error('storage');
        }
        await fn();
      });
    }
    function accept(value) {
      if(value.schemaVersion!==1 || value.requestId!==saved.requestId || !['UNKNOWN','APPLIED'].includes(value.state) || value.replayed!==false)throw new Error('unavailable');
      if(value.state==='APPLIED')persist({...saved,step:'APPLIED'});
      notice=value.state==='APPLIED' && value.currentReadiness==='READY'?'granted':'pending';
    }
    const args=()=>['--operation',saved.requestId,'--initialization',saved.initializationId];
    $('agentRefresh').addEventListener('click',()=>run(refresh));
    $('agentGrantForm').addEventListener('submit',event=>{
      event.preventDefault();if(!$('agentGrantForm').reportValidity())return;
      if(getBusy() || $('agentGrant').disabled)return;
      const reviewed={initializationId:current.initializationId,sourceManifestSha256:current.sourceManifestSha256};
      run(()=>exclusive(async()=>{
        await refresh();
        if(current.initializationId!==reviewed.initializationId || current.sourceManifestSha256!==reviewed.sourceManifestSha256)throw new Error('changed');
        if(broken || saved || !local() || current.permission!=='NONE' || current.readiness!=='READY' || !current.allStopped)throw new Error('unavailable');
        persist({schemaVersion:1,initializationId:current.initializationId,requestId:crypto.randomUUID(),step:'REQUESTED'});
        accept(await invoke('agent-authorize',args()));
        await refresh();
      }));
    });
    $('agentObserve').addEventListener('click',()=>run(async()=>{
      await refresh();if(broken || !matching())return;
      if(!saved){if(!current.requestId)return;persist({schemaVersion:1,initializationId:current.initializationId,requestId:current.requestId,step:'OBSERVED'});}
      accept(await invoke('agent-status',args()));
    }));
    $('agentComplete').addEventListener('click',()=>run(()=>exclusive(async()=>{
      await refresh();if(broken || !saved || !matching() || current.permission!=='PENDING' || current.requestId!==saved.requestId || !current.allStopped)return;
      persist({...saved,step:'COMPLETE_REQUESTED'});
      accept(await invoke('agent-complete',args()));await refresh();
    })));
    render();return {render,buttons};
  };
})();
