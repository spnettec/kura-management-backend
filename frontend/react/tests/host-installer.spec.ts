import { test, expect, Page } from '@playwright/test';
import { readFile, chmod } from 'node:fs/promises';

// Docker adapter contracts inject only the bridge. Native engine/PAM/HTTPS proof
// is a separate owned-machine acceptance, never inferred from these fixtures.
async function dockerBridge(page:Page,mode='normal'){
  await page.addInitScript(({mode})=>Object.assign(window,{__dockerMode:mode,__calls:[],__files:[],__opened:[],__dockerRecord:null,__dockerAvailable:false}),{mode});
  const body=`(()=>{const w=window;const image={key:'unit-arm64',title:'Contract image',archiveSize:4,available:false};
    w.open=(...args)=>{w.__opened.push(args);return null;};
    w.cockpit={file:(path,options)=>({replace:async(data,tag)=>{w.__files.push({path,options,tag,bytes:Array.from(data)});},close:()=>{}}),spawn:async(args,options)=>{
      w.__calls.push({args,options});const a=args[2],id=args[args.indexOf('--operation')+1];
      if(a==='containers')return JSON.stringify(w.__dockerRecord?.kind==='DEPLOY'&&w.__dockerRecord.phase==='APPLIED'?[{id:'d'.repeat(64),name:'owned-unit',state:'running'}]:[]);
      if(a==='docker-overview')return JSON.stringify({engine:{available:true},images:[{...image,available:w.__dockerAvailable||w.__dockerMode!=='upload'}],deployments:w.__dockerRecord?.kind==='DEPLOY'?[{operationId:w.__dockerRecord.operationId,containerId:'d'.repeat(64)}]:[],operations:w.__dockerMode==='foreign'?[{operationId:'c8a1f911-b885-412f-b23f-c3d987f8550f',kind:'DEPLOY',phase:'UNKNOWN'}]:[]});
      if(a==='docker-upload-prepare')w.__dockerRecord={operationId:id,kind:'IMPORT',phase:'UPLOADING',uploadReady:false,uploadPath:'/var/lib/yofc-console-installations/uploads/'+id+'.tar.gz'};
      if(a==='docker-deploy'){
        const saved=JSON.parse(localStorage.getItem('yofc.console.docker-deployment.v1'));if(saved.operationId!==id||saved.step!=='DEPLOY_REQUESTED')throw new Error('intent missing');
        w.__dockerRecord={operationId:id,kind:'DEPLOY',phase:'PENDING'};if(w.__dockerMode==='lost')throw new Error('lost reply');
      }
      if(a==='docker-import'){w.__dockerRecord={operationId:id,kind:'IMPORT',phase:'UNKNOWN'};}
      if(a==='docker-operation'){
        if(!w.__dockerRecord)return JSON.stringify({operationId:id,phase:'UNOBSERVED',transportNonDeliveryCertified:false});
        if(w.__dockerRecord.phase==='UPLOADING'&&w.__files.length)w.__dockerRecord.uploadReady=true;
      }
      if(a==='docker-status')return JSON.stringify({state:'active',ready:false,readiness:'NOT_READY'});
      return JSON.stringify(w.__dockerRecord);
    }};})();`;
  await page.route('**/base1/cockpit.js',route=>route.fulfill({contentType:'text/javascript',body}));
}

async function dockerMaintenanceBridge(page:Page,lostStop=false){
  await page.addInitScript(({lostStop})=>Object.assign(window,{__calls:[],__lostStop:lostStop,__maintenance:null}),{lostStop});
  const body=`(()=>{const w=window,dep='8cfa6f31-2b47-43a4-9b4a-c7da0baab75c',cid='d'.repeat(64);
    const image={key:'next',title:'Next image',available:true,maintenanceProtocolVersion:2,imageId:'sha256:new'};
    function record(id,phase){return {operationId:id,deploymentId:dep,kind:'MAINTENANCE',phase,oldGeneration:{image:{maintenanceProtocolVersion:1,imageId:'sha256:old'}}};}
    w.cockpit={spawn:async(args,options)=>{
      w.__calls.push({args,options});const action=args[2],id=args[args.indexOf('--operation')+1];
      if(action==='containers')return JSON.stringify([{id:cid,name:'selected center',state:'running'}]);
      if(action==='docker-overview')return JSON.stringify({engine:{available:true},images:[image],deployments:[{operationId:dep,containerId:cid}],operations:[]});
      if(action==='docker-maintenance-begin')w.__maintenance=record(id,'FROZEN');
      if(action==='docker-maintenance-stop'){
        const saved=JSON.parse(localStorage.getItem('yofc.console.docker-maintenance.v1'));if(saved.operationId!==id||saved.step!=='STOP_REQUESTED')throw new Error('intent missing');
        if(w.__lostStop)throw new Error('lost reply');w.__maintenance=record(id,'STOPPED');
      }
      if(action==='docker-maintenance-upgrade'){
        const saved=JSON.parse(localStorage.getItem('yofc.console.docker-maintenance.v1'));if(saved.operationId!==id||saved.step!=='UPGRADE_REQUESTED')throw new Error('intent missing');w.__maintenance=record(id,'PENDING');
      }
      if(action==='docker-operation'){
        if(!w.__maintenance)w.__maintenance=record(id,'FROZEN');
        else if(w.__maintenance.phase==='PENDING')w.__maintenance=record(id,'COMPLETE');
      }
      return JSON.stringify(w.__maintenance);
    }};
  })();`;
  await page.route('**/base1/cockpit.js',route=>route.fulfill({contentType:'text/javascript',body}));
}
test('Docker maintenance contract: lost stop preserves original request after reload without a resend',async({page})=>{
  await dockerMaintenanceBridge(page,true);await page.goto('/');await page.locator('#mode').selectOption('docker');await page.locator('#dockerRefresh').click();
  await page.locator('#dockerFreeze').click();await page.locator('#dockerStop').click();await expect(page.locator('#dockerStop')).toBeDisabled();
  const before=await page.evaluate(()=>JSON.parse(localStorage.getItem('yofc.console.docker-maintenance.v1')!));expect(before.step).toBe('STOP_REQUESTED');
  expect(await page.evaluate(()=>(window as any).__calls.filter((row:any)=>row.args[2]==='docker-maintenance-stop').length)).toBe(1);
  await page.reload();await page.locator('#mode').selectOption('docker');await page.locator('#dockerRefresh').click();await page.locator('#dockerMaintenanceObserve').click();
  await expect(page.locator('#dockerStop')).toBeDisabled();await expect(page.locator('#dockerFreeze')).toBeDisabled();await expect(page.locator('#dockerMaintenanceResult')).toContainText(before.operationId);
  expect(await page.evaluate(()=>(window as any).__calls.some((row:any)=>row.args[2]==='docker-maintenance-stop'))).toBe(false);
});
test('Docker maintenance contract: stopped legacy generation upgrades once under the original ID',async({page})=>{
  await dockerMaintenanceBridge(page);await page.goto('/');await page.locator('#mode').selectOption('docker');await page.locator('#dockerRefresh').click();
  await page.locator('#dockerFreeze').click();await page.locator('#dockerStop').click();await expect(page.locator('#dockerResume')).toBeDisabled();
  await page.locator('#dockerUpgrade').click();await expect(page.locator('#dockerMaintenanceResult')).toContainText('completed',{timeout:5000});
  const calls=await page.evaluate(()=>(window as any).__calls.filter((row:any)=>row.args[2].startsWith('docker-maintenance-')));
  expect(calls.map((row:any)=>row.args[2])).toEqual(['docker-maintenance-begin','docker-maintenance-stop','docker-maintenance-upgrade']);
  expect(new Set(calls.map((row:any)=>row.args[row.args.indexOf('--operation')+1])).size).toBe(1);
  expect(calls.every((row:any)=>row.options.superuser==='require')).toBe(true);
  const saved=await page.evaluate(()=>JSON.parse(localStorage.getItem('yofc.console.docker-maintenance.v1')!));expect(Object.keys(saved).sort()).toEqual(['deploymentId','operationId','step']);expect(saved.step).toBe('COMPLETE');
});
test('Docker maintenance contract: corrupt public intent blocks all deployment mutations',async({page})=>{
  await dockerMaintenanceBridge(page);await page.addInitScript(()=>localStorage.setItem('yofc.console.docker-maintenance.v1','broken'));
  await page.goto('/');await page.locator('#mode').selectOption('docker');await page.locator('#dockerRefresh').click();
  for(const id of ['dockerFreeze','dockerStop','dockerUpgrade','dockerResume','dockerDeploy','dockerUpload'])await expect(page.locator('#'+id)).toBeDisabled();
  expect(await page.evaluate(()=>(window as any).__calls.every((row:any)=>['docker-overview','containers'].includes(row.args[2])))).toBe(true);
});
test('Docker maintenance contract: acknowledged start can be explicitly verified without another start',async({page})=>{
  await dockerMaintenanceBridge(page);await page.goto('/');await page.locator('#mode').selectOption('docker');await page.locator('#dockerRefresh').click();
  await page.locator('#dockerFreeze').click();await page.locator('#dockerStop').click();
  await page.evaluate(()=>{
    const w=window as any,original=w.cockpit.spawn;
    w.cockpit.spawn=async(args:string[],options:unknown)=>{
      if(args[2]==='docker-maintenance-upgrade'){
        w.__calls.push({args,options});w.__maintenance={...w.__maintenance,phase:'UNKNOWN',newGeneration:{containerId:'b'.repeat(64)},stopReceipt:{successfulReturn:true},intents:{'start-generation':{}},receipts:{'start-generation':{commandReturnedSuccess:true}}};return JSON.stringify(w.__maintenance);
      }
      if(args[2]==='docker-maintenance-verify-start'){
        const saved=JSON.parse(localStorage.getItem('yofc.console.docker-maintenance.v1')!);if(saved.step!=='VERIFY_START_REQUESTED')throw new Error('intent missing');
        w.__calls.push({args,options});w.__maintenance.phase='COMPLETE';return JSON.stringify(w.__maintenance);
      }
      return original(args,options);
    };
  });
  await page.locator('#dockerUpgrade').click();await expect(page.locator('#dockerVerifyStarted')).toBeEnabled();await page.locator('#dockerVerifyStarted').click();
  await expect(page.locator('#dockerMaintenanceResult')).toContainText('completed');
  const calls=await page.evaluate(()=>(window as any).__calls.filter((row:any)=>['docker-maintenance-upgrade','docker-maintenance-verify-start'].includes(row.args[2])));
  expect(calls.map((row:any)=>row.args[2])).toEqual(['docker-maintenance-upgrade','docker-maintenance-verify-start']);
  expect(calls[0].args[calls[0].args.indexOf('--operation')+1]).toBe(calls[1].args.at(-1));
});

test('Docker deployment contract: lost create reply persists original ID through reload and never resends',async({page})=>{
  await dockerBridge(page,'lost');await page.goto('/');await page.locator('#mode').selectOption('docker');await page.locator('#dockerRefresh').click();
  await page.locator('#dockerName').fill('owned-unit');await page.locator('#dockerDeploy').click();
  await expect(page.locator('#dockerDeploy')).toBeDisabled();
  const saved=await page.evaluate(()=>JSON.parse(localStorage.getItem('yofc.console.docker-deployment.v1')!));
  expect(saved.step).toBe('DEPLOY_REQUESTED');expect(Object.keys(saved).sort()).toEqual(['kind','operationId','step']);
  expect(await page.evaluate(()=>(window as any).__calls.filter((c:any)=>c.args[2]==='docker-deploy').length)).toBe(1);
  await page.reload();await page.locator('#mode').selectOption('docker');await page.locator('#dockerRefresh').click();await page.locator('#dockerObserve').click();
  await expect(page.locator('#dockerDeploy')).toBeDisabled();await expect(page.locator('#dockerResult')).toContainText(saved.operationId);
  expect(await page.evaluate(()=>(window as any).__calls.some((c:any)=>c.args[2]==='docker-deploy'))).toBe(false);
});
test('Docker deployment contract: approved binary upload and uncertain import are not repeated',async({page})=>{
  await dockerBridge(page,'upload');await page.goto('/');await page.locator('#mode').selectOption('docker');await page.locator('#dockerRefresh').click();
  await page.locator('#dockerArchive').setInputFiles({name:'unit.tar.gz',mimeType:'application/gzip',buffer:Buffer.from([1,2,3,4])});
  await page.locator('#dockerUpload').click();await expect(page.locator('#dockerImport')).toBeEnabled();
  const files=await page.evaluate(()=>(window as any).__files);expect(files).toHaveLength(1);expect(files[0].options).toEqual({binary:true,superuser:'require'});expect(files[0].tag).toBe('-');
  await page.locator('#dockerImport').click();await expect(page.locator('#dockerImport')).toBeDisabled();await page.locator('#dockerObserve').click();
  expect(await page.evaluate(()=>(window as any).__calls.filter((c:any)=>c.args[2]==='docker-import').length)).toBe(1);
  expect(await page.evaluate(()=>(window as any).__calls.every((c:any)=>c.options.superuser==='require'))).toBe(true);
  expect(await page.evaluate(()=>localStorage.getItem('yofc.console.docker-deployment.v1'))).not.toContain('uploadPath');
});
test('Docker deployment contract: pending jobs only poll original ID and readiness cannot be inferred from active',async({page})=>{
  await page.clock.install();await dockerBridge(page);await page.goto('/');await page.locator('#mode').selectOption('docker');await page.locator('#dockerRefresh').click();
  await page.locator('#dockerName').fill('owned-unit');await page.locator('#dockerDeploy').click();await page.clock.runFor(20000);
  await expect(page.locator('#message')).toContainText('read-only observation has ended');
  expect(await page.evaluate(()=>(window as any).__calls.filter((c:any)=>c.args[2]==='docker-deploy').length)).toBe(1);
  await page.evaluate(()=>{(window as any).__dockerRecord.phase='APPLIED';});await page.locator('#dockerObserve').click();await page.locator('#dockerStatus').click();
  await page.locator('#url').fill('https://console.example.com:48443');await page.locator('#open').click();
  expect(await page.evaluate(()=>(window as any).__opened.length)).toBe(0);
});
test('Docker deployment contract: another profile unknown fences new create without resetting its ID',async({page})=>{
  await dockerBridge(page,'foreign');await page.goto('/');await page.locator('#mode').selectOption('docker');await page.locator('#dockerRefresh').click();
  await expect(page.locator('#dockerDeploy')).toBeDisabled();await expect(page.locator('#dockerResult')).toContainText('c8a1f911-b885-412f-b23f-c3d987f8550f');
  expect(await page.evaluate(()=>(window as any).__calls.some((c:any)=>c.args[2]==='docker-deploy'))).toBe(false);
});
test('Docker deployment contract: explicit post-load verification retains original ID and cannot repeat import',async({page})=>{
  await dockerBridge(page,'upload');await page.goto('/');await page.locator('#mode').selectOption('docker');await page.locator('#dockerRefresh').click();
  await page.locator('#dockerArchive').setInputFiles({name:'unit.tar.gz',mimeType:'application/gzip',buffer:Buffer.from([1,2,3,4])});
  await page.locator('#dockerUpload').click();await page.locator('#dockerImport').click();
  await expect(page.locator('#dockerVerifyImport')).toBeDisabled();
  await page.evaluate(()=>{
    const w=window as any;w.__dockerRecord.receipts={load:{commandReturnedSuccess:true}};const original=w.cockpit.spawn;
    w.cockpit.spawn=async(args:string[],options:unknown)=>{
      if(args[2]!=='docker-verify-import')return original(args,options);
      w.__calls.push({args,options});w.__dockerRecord.phase='APPLIED';return JSON.stringify(w.__dockerRecord);
    };
  });
  await page.locator('#dockerObserve').click();await page.locator('#dockerVerifyImport').click();
  await expect(page.locator('#dockerResult')).toContainText('has completed');
  const calls=await page.evaluate(()=>(window as any).__calls.filter((row:any)=>['docker-import','docker-verify-import'].includes(row.args[2])));
  expect(calls.map((row:any)=>row.args[2])).toEqual(['docker-import','docker-verify-import']);
  expect(calls[0].args.at(-1)).toBe(calls[1].args.at(-1));
});

// UI adapter contracts only: no Cockpit login/elevation or first setup is certified.
const credential={schemaVersion:1,installationId:'08271197-47a6-4b3e-a123-853925071244',token:'c'.repeat(64),certificateFingerprint:'a'.repeat(64)};
async function bridge(page:Page,denied=false,accessDenied=false) {
  await page.addInitScript(({credential,denied,accessDenied}) => {
    Object.assign(window,{__calls:[],__opened:[],__statuses:[],__defaultStatus:{state:'active',ready:true,readiness:'READY'},__credential:credential,__denied:denied,__accessDenied:accessDenied});
    window.open=((...args:unknown[])=>{(window as any).__opened.push(args);return null;}) as typeof window.open;
  },{credential,denied,accessDenied});
  const body = "window.cockpit={spawn:async(args,options)=>{window.__calls.push({args,options});if(window.__accessDenied)throw Object.assign(new Error('denied'),{problem:'access-denied'});if(window.__denied)throw new Error('denied');if(args[2]==='native-overview')return JSON.stringify({packages:[],units:[],allStopped:true,maintenance:null,current:null,installedCore:{state:'VERIFIED',version:'0.1.0~contract',architecture:'arm64'}});const port=args.includes('--port')?Number(args[args.indexOf('--port')+1]):18443;return JSON.stringify(args[2]==='credential'?window.__credential:args[2]==='containers'?[]:{unit:'kura-management-console.service',pid:'123',httpAddress:'[::]:'+port,httpPort:port,...(window.__statuses.shift()||window.__defaultStatus)});}};";
  await page.route('**/base1/cockpit.js',route=>route.fulfill({contentType:'text/javascript',body}));
}
async function inspectCore(page:Page){
  await page.locator('#nativeRefresh').click();await expect(page.locator('#start')).toBeEnabled();
}
for (const language of ['zh','en']) test.describe(language,() => {
  test.use({locale:language==='zh'?'zh-CN':'en-US'});
  test('host installer contract: known privilege refusal explains unentered command and never retries',async({page})=>{
    await bridge(page,false,true);await page.goto('/');await page.locator('#nativeRefresh').click();
    await expect(page.locator('#message')).toContainText(language==='zh'?'命令尚未执行':'The command was not executed');
    await expect(page.locator('#download')).toBeDisabled();
    expect(await page.evaluate(()=>(window as any).__calls.length)).toBe(1);
  });
  test('host installer contract: authorized channel, private download, no browser storage',async({page},info) => {
    await bridge(page);await page.goto('/');await inspectCore(page);
    await expect(page.locator('#title')).toHaveText(language==='zh'?'YOFC 管理中心安装设置':'YOFC console installation');
    expect(await page.evaluate(() => (window as any).__calls.map((c:any)=>c.args[2]))).toEqual(['native-overview']);
    await page.locator('#read').click();
    await expect(page.locator('#fingerprint')).toHaveText(credential.certificateFingerprint);
    await expect(page.locator('#download')).toBeEnabled();
    expect(await page.locator('body').innerText()).not.toContain(credential.token);
    const event=page.waitForEvent('download');await page.locator('#download').click();const download=await event;
    const path=info.outputPath('contract-only-credential.json');await download.saveAs(path);await chmod(path,0o600);
    expect(JSON.parse(await readFile(path,'utf8'))).toEqual(credential);
    const calls=await page.evaluate(() => (window as any).__calls);
    expect(calls).toHaveLength(2);
    expect(calls[1].options).toEqual({superuser:'require',err:'message'});
    expect(calls.map((c:any)=>c.args[2])).toEqual(['native-overview','credential']);
    expect(calls[1].args).toEqual(['/usr/bin/python3','/usr/lib/kura-management-console/installer.py','credential']);
    expect(await page.evaluate(() => [localStorage.length,sessionStorage.length])).toEqual([0,0]);
    await page.screenshot({path:info.outputPath('installer-contract-'+language+'.png'),fullPage:true});
  });
  test('host installer contract: first start polls read-only until HTTPS is ready',async({page})=>{
    await bridge(page);await page.goto('/');await inspectCore(page);await page.evaluate(()=>{(window as any).__statuses.push({state:'active',ready:false,readiness:'NOT_READY'},{state:'active',ready:true,readiness:'READY'});});
    await page.locator('#start').click();
    await expect(page.locator('#service')).toContainText(language==='zh'?'不会重复启动':'will not be started again');
    await expect(page.locator('#service')).toContainText(language==='zh'?'HTTPS 服务已就绪':'Host HTTPS is ready',{timeout:4000});
    const calls=await page.evaluate(()=>(window as any).__calls);
    expect(calls.map((c:any)=>c.args[2])).toEqual(['native-overview','start','status']);
    expect(calls.every((c:any)=>c.options.superuser==='require')).toBe(true);
  });
});
test('host installer contract: published URL is explicit and cannot reuse the host manager cookie hostname',async({page})=>{
  await bridge(page);await page.goto('/');await inspectCore(page);await expect(page.locator('#url')).toHaveValue('');
  await page.locator('#url').fill('https://127.0.0.1:18443');await page.locator('#open').click();
  await expect(page.locator('#message')).toContainText('different hostname');
  expect(await page.evaluate(()=>(window as any).__calls.map((c:any)=>c.args[2]))).toEqual(['native-overview']);
});
test('host installer contract: denied host privilege never falls back to a public token source',async({page}) => {
  await bridge(page,true);await page.goto('/');await page.locator('#nativeRefresh').click();
  await expect(page.locator('#nativeRefresh')).toBeEnabled();
  await expect(page.locator('#download')).toBeDisabled();await expect(page.locator('#identity')).toHaveText('—');
  expect(await page.evaluate(() => (window as any).__calls.length)).toBe(1);
});
test('host installer contract: target change retires the credential; invalid instance is rejected before dispatch',async({page}) => {
  await bridge(page);await page.goto('/');await inspectCore(page);await page.locator('#read').click();await expect(page.locator('#download')).toBeEnabled();
  await page.locator('#instance').fill('../other');await expect(page.locator('#download')).toBeDisabled();
  await page.locator('#read').click();await expect(page.locator('#identity')).toHaveText('—');
  expect(await page.evaluate(() => (window as any).__calls.length)).toBe(2);
});
test('host installer contract: unverified start is not automatically repeated or converted to restart',async({page}) => {
  await bridge(page);await page.goto('/');await inspectCore(page);await page.evaluate(()=>{(window as any).__denied=true;});await page.locator('#start').click();await expect(page.locator('#start')).toBeEnabled();
  const calls=await page.evaluate(() => (window as any).__calls);
  expect(calls).toHaveLength(2);expect(calls[1].args[2]).toBe('start');expect(JSON.stringify(calls)).not.toContain('restart');
});
test('host installer contract: reading status preserves the chosen first-start port',async({page})=>{
  await bridge(page);await page.goto('/');await inspectCore(page);await page.locator('#instance').fill('center2');
  await page.locator('#port').fill('28443');await page.locator('#status').click();
  await expect(page.locator('#status')).toBeEnabled();await expect(page.locator('#port')).toHaveValue('28443');
  await page.locator('#start').click();await expect(page.locator('#start')).toBeEnabled();
  const calls=await page.evaluate(()=>(window as any).__calls);
  expect(calls).toHaveLength(3);expect(calls[2].args).toEqual(['/usr/bin/python3','/usr/lib/kura-management-console/installer.py','start','--instance','center2','--port','28443']);
});
test('host installer contract: active without verified HTTPS cannot open setup',async({page})=>{
  await bridge(page);await page.goto('/');await inspectCore(page);await page.evaluate(()=>{(window as any).__statuses.push({state:'active',ready:false,readiness:'NOT_READY'});});
  await page.locator('#status').click();await expect(page.locator('#status')).toBeEnabled();
  await page.locator('#url').fill('https://console.example.com:18443');await page.locator('#open').click();
  await expect(page.locator('#message')).toContainText('verify that host HTTPS is ready');
  expect(await page.evaluate(()=>(window as any).__opened)).toHaveLength(0);
  await page.locator('#status').click();await expect(page.locator('#service')).toContainText('Host HTTPS is ready');
  await page.locator('#open').click();expect(await page.evaluate(()=>(window as any).__opened[0][0])).toBe('https://console.example.com:18443/setup');
});
test('host installer contract: target change cancels the original readiness polling',async({page})=>{
  await page.clock.install();await bridge(page);await page.goto('/');await inspectCore(page);
  await page.evaluate(()=>{(window as any).__defaultStatus={state:'active',ready:false,readiness:'NOT_READY'};});
  await page.locator('#start').click();await expect(page.locator('#start')).toBeEnabled();
  await page.locator('#instance').fill('center2');await page.clock.runFor(2000);
  expect(await page.evaluate(()=>(window as any).__calls.map((c:any)=>c.args[2]))).toEqual(['native-overview','start']);
  await expect(page.locator('#service')).toHaveText('');
});
test('host installer contract: readiness polling is bounded and never repeats start',async({page})=>{
  await page.clock.install();await bridge(page);await page.goto('/');await inspectCore(page);
  await page.evaluate(()=>{(window as any).__defaultStatus={state:'active',ready:false,readiness:'NOT_READY'};});
  await page.locator('#start').click();await expect(page.locator('#start')).toBeEnabled();await page.clock.runFor(20000);
  const calls=await page.evaluate(()=>(window as any).__calls.map((c:any)=>c.args[2]));
  expect(calls).toHaveLength(17);expect(calls.filter((c:string)=>c==='start')).toHaveLength(1);
  expect(calls.slice(2).every((c:string)=>c==='status')).toBe(true);
  await expect(page.locator('#service')).toContainText('read-only check has ended');
});
test('host maintenance contract: unknown stop retains original identity and permits only observation',async({page})=>{
  await bridge(page);await page.goto('/');
  await page.evaluate(()=>{
    (window as any).cockpit.spawn=async(args:string[],options:unknown)=>{
      (window as any).__calls.push({args,options});
      const command=args[2],id=args[args.indexOf('--operation')+1];
      if(command==='maintenance-begin')return JSON.stringify({operationId:id,phase:'FROZEN'});
      if(command==='maintenance-stop')throw new Error('observation expired after stop intent');
      return JSON.stringify({operation:{operationId:id,phase:'STOPPING'}});
    };
  });
  await page.locator('#maintenanceBegin').click();await expect(page.locator('#maintenanceStop')).toBeEnabled();
  await page.locator('#maintenanceStop').click();await expect(page.locator('#maintenanceStop')).toBeDisabled();
  await expect(page.locator('#maintenanceBegin')).toBeDisabled();await expect(page.locator('#maintenanceComplete')).toBeDisabled();
  await page.locator('#maintenanceObserve').click();await expect(page.locator('#maintenanceResult')).toContainText('unverified');
  const calls=await page.evaluate(()=>(window as any).__calls);
  expect(calls.map((c:any)=>c.args[2])).toEqual(['maintenance-begin','maintenance-stop','maintenance-status']);
  expect(new Set(calls.map((c:any)=>c.args[c.args.indexOf('--operation')+1])).size).toBe(1);
  expect(calls.every((c:any)=>c.options.superuser==='require')).toBe(true);
  await expect(page.locator('#maintenanceStop')).toBeDisabled();
});
test('host maintenance contract: reload cannot turn an uncertain stop into another stop; cancellation retains its original ID',async({page})=>{
  await bridge(page);
  await page.route('**/base1/cockpit.js',route=>route.fulfill({contentType:'text/javascript',body:`
    window.cockpit={spawn:async(args,options)=>{
      window.__calls.push({args,options});const command=args[2],id=args[args.indexOf('--operation')+1];
      if(command==='maintenance-stop')throw new Error('reply lost');
      if(command==='maintenance-cancel')return JSON.stringify({operationId:id,phase:'COMPLETE',outcome:'CANCELLED_BEFORE_STOP_INTENT'});
      const record={operationId:id,phase:'FROZEN',stopIntents:{}};
      return JSON.stringify(command==='maintenance-status'?{active:record,activeFencePresent:true}:record);
    }};` }));
  await page.goto('/');await page.locator('#maintenanceBegin').click();await expect(page.locator('#maintenanceStop')).toBeEnabled();
  await page.locator('#maintenanceStop').click();await expect(page.locator('#maintenanceStop')).toBeDisabled();
  const before=await page.evaluate(()=>JSON.parse(localStorage.getItem('yofc.console.host-maintenance.v1')!));
  expect(Object.keys(before).sort()).toEqual(['operationId','step']);expect(before.step).toBe('STOP_REQUESTED');
  await page.reload();await expect(page.locator('#maintenanceBegin')).toBeDisabled();await expect(page.locator('#maintenanceStop')).toBeDisabled();
  await page.locator('#maintenanceObserve').click();await expect(page.locator('#maintenanceCancel')).toBeEnabled();
  await expect(page.locator('#maintenanceStop')).toBeDisabled();await expect(page.locator('#maintenanceResult')).toContainText(before.operationId);
  await page.locator('#maintenanceCancel').click();await expect(page.locator('#maintenanceResult')).toContainText('cancelled');
  await expect(page.locator('#maintenanceResult')).toContainText(before.operationId);await expect(page.locator('#maintenanceBegin')).toBeEnabled();
  const calls=await page.evaluate(()=>(window as any).__calls);
  expect(calls.map((c:any)=>c.args[2])).toEqual(['maintenance-status','maintenance-cancel']);
  expect(calls.every((c:any)=>c.args[c.args.indexOf('--operation')+1]===before.operationId)).toBe(true);
});
test('host maintenance contract: absent server record after lost begin reply remains unobserved until original ID is retired',async({page})=>{
  await bridge(page);
  await page.route('**/base1/cockpit.js',route=>route.fulfill({contentType:'text/javascript',body:`
    window.cockpit={spawn:async(args,options)=>{
      window.__calls.push({args,options});const command=args[2],id=args[args.indexOf('--operation')+1];
      if(command==='maintenance-begin')throw new Error('delivery unverified');
      if(command==='maintenance-status')return JSON.stringify({active:null,activeFencePresent:false,operation:{operationId:id,phase:'UNOBSERVED'}});
      return JSON.stringify({operationId:id,phase:'COMPLETE',outcome:'CANCELLED_BEFORE_ACCEPTANCE'});
    }};` }));
  await page.goto('/');await page.locator('#maintenanceBegin').click();await expect(page.locator('#maintenanceBegin')).toBeDisabled();
  await page.locator('#maintenanceObserve').click();await expect(page.locator('#maintenanceBegin')).toBeDisabled();
  await expect(page.locator('#maintenanceStop')).toBeDisabled();await expect(page.locator('#maintenanceCancel')).toBeEnabled();
  await page.locator('#maintenanceCancel').click();await expect(page.locator('#maintenanceResult')).toContainText('rejects delayed actions');
  const calls=await page.evaluate(()=>(window as any).__calls);
  expect(calls.map((c:any)=>c.args[2])).toEqual(['maintenance-begin','maintenance-status','maintenance-cancel']);
  expect(new Set(calls.map((c:any)=>c.args[c.args.indexOf('--operation')+1])).size).toBe(1);
});
test('host maintenance contract: completion keeps its original receipt visible and never starts a service',async({page})=>{
  await bridge(page);
  await page.route('**/base1/cockpit.js',route=>route.fulfill({contentType:'text/javascript',body:`
    window.cockpit={spawn:async(args,options)=>{
      window.__calls.push({args,options});const command=args[2],id=args[args.indexOf('--operation')+1];
      return JSON.stringify({operationId:id,phase:command==='maintenance-begin'?'FROZEN':command==='maintenance-stop'?'STOPPED':'COMPLETE'});
    }};` }));
  await page.goto('/');await page.locator('#maintenanceBegin').click();await expect(page.locator('#maintenanceStop')).toBeEnabled();
  await page.locator('#maintenanceStop').click();await expect(page.locator('#maintenanceComplete')).toBeEnabled();
  const original=await page.locator('#maintenanceResult').innerText();
  await page.locator('#maintenanceComplete').click();await expect(page.locator('#maintenanceResult')).toContainText('Services remain stopped');
  const id=original.match(/[a-f0-9-]{36}/)![0];await expect(page.locator('#maintenanceResult')).toContainText(id);
  const calls=await page.evaluate(()=>(window as any).__calls.map((c:any)=>c.args[2]));
  expect(calls).toEqual(['maintenance-begin','maintenance-stop','maintenance-complete']);
});
test('host maintenance contract: inability to persist original identity prevents dispatch',async({page})=>{
  await bridge(page);await page.addInitScript(()=>{Storage.prototype.setItem=()=>{throw new Error('storage unavailable');};});
  await page.goto('/');await page.locator('#maintenanceBegin').click();await expect(page.locator('#maintenanceObserve')).toBeEnabled();
  expect(await page.evaluate(()=>(window as any).__calls)).toEqual([]);
});
