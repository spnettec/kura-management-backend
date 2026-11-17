import {test,expect,type Page} from '@playwright/test';
import {mkdirSync} from 'node:fs';
import {resolve} from 'node:path';
// Browser/bridge contracts only. No native controller, PAM or service is invoked.
const storage='yofc.kura.local-switch-grant.v1';
const scope='d31eb6d9-7213-49ac-884e-e02ce42de42b';
const original='8b4f452e-04fe-46b8-8b0c-6fabee8898b2';
type Model={request:string|null,pending:boolean,active:boolean,initialization?:string,source?:string};
async function bridge(page:Page,mode='normal',model:Model={request:null,pending:false,active:false}) {
  const calls:any[]=[],errors:string[]=[];
  page.on('pageerror',error=>errors.push(error.message));
  if(mode==='pending')Object.assign(model,{request:original,pending:true});
  if(mode==='active')model.active=true;
  await page.exposeFunction('__agentRpc',async(args:string[],options:any)=>{
    calls.push({args,options});const command=args[2];
    const namespace=model.initialization||scope;
    if(command==='agent-overview')return {schemaVersion:1,profile:'native-nonroot-v1',initializationId:namespace,
      sourceManifestSha256:model.source||'a'.repeat(64),nativeUid:997,controlGid:996,
      permission:model.request?model.pending?'PENDING':'GRANTED':'NONE',requestId:model.request,
      readiness:model.pending?'REFUSED':'READY',allStopped:!model.active,
      services:['kura.service','kura-management-agent-v2.service','kura-management-agent-enrollment.socket','kura-management-enrollment-reader.socket'].map(unit=>
        ({unit,state:model.active?'active':'inactive',pid:unit.endsWith('.socket')?null:model.active?42:0,connections:unit.endsWith('.socket')?0:null,jobPending:false}))};
    const request=args[args.indexOf('--operation')+1],initial=args[args.indexOf('--initialization')+1];
    expect(initial).toBe(namespace);expect(options.superuser).toBe('require');
    const saved=await page.evaluate(key=>JSON.parse(localStorage.getItem(key)!),storage);
    expect(saved.requestId).toBe(request);expect(saved.initializationId).toBe(scope);
    expect(Object.keys(saved).sort()).toEqual(['initializationId','requestId','schemaVersion','step']);
    if(command==='agent-authorize'){
      expect(saved.step).toBe('REQUESTED');expect(model.request).toBeNull();model.request=request;model.pending=mode==='lost';
      if(mode==='lost')throw new Error('original reply lost');
    }
    if(command==='agent-complete'){expect(saved.step).toBe('COMPLETE_REQUESTED');expect(request).toBe(model.request);model.pending=false;}
    return {schemaVersion:1,requestId:request,state:model.pending?'UNKNOWN':'APPLIED',
      ...(model.pending?{}:{currentReadiness:'READY',canControl:true}),unitStarted:false,remoteEnabled:false,replayed:false};
  });
  await page.route('**/base1/cockpit.js',route=>route.fulfill({contentType:'text/javascript',body:
    'window.cockpit={spawn:async(args,options)=>JSON.stringify(await window.__agentRpc(args,options))};'}));
  return {calls,errors,model};
}
const writes=(calls:any[])=>calls.filter(row=>['agent-authorize','agent-complete'].includes(row.args[2]));
async function idle(page:Page){await expect(page.locator('#agentRefresh')).toBeEnabled();}
async function inspect(page:Page){await page.locator('#agentRefresh').click();await expect(page.locator('#agentIdentity')).toContainText(scope);await idle(page);}
test.use({locale:'en-US'});

for(const language of ['zh','en'] as const)test.describe(language,()=>{
  test.use({locale:language==='zh'?'zh-CN':'en-US'});
  test('explicit native confirmation persists the public context before one grant and survives reload',async({page})=>{
    const {calls,errors}=await bridge(page);await page.goto('/');await inspect(page);
    await page.locator('#agentGrant').click();await expect(page.locator('#agentApprove')).toBeFocused();expect(writes(calls)).toHaveLength(0);
    await page.locator('#agentApprove').check();await page.locator('#agentGrant').click();
    await idle(page);
    await expect(page.locator('#agentGrant')).toBeDisabled();expect(writes(calls)).toHaveLength(1);
    const saved=await page.evaluate(key=>JSON.parse(localStorage.getItem(key)!),storage);
    expect(saved.step).toBe('APPLIED');expect(saved.initializationId).toBe(scope);
    await page.reload();await inspect(page);await page.locator('#agentObserve').click();
    await idle(page);
    expect(writes(calls)).toHaveLength(1);expect(errors).toEqual([]);
    expect(calls.every(row=>row.args[2].startsWith('agent-'))).toBe(true);
    await page.setViewportSize({width:390,height:844});
    expect(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth)).toBe(true);
    const screenshots=process.env.YOFC_AGENT_SCREENSHOTS;
    if(screenshots){mkdirSync(screenshots,{recursive:true});await page.locator('#agentControl').screenshot({path:resolve(screenshots,'local-permission-'+language+'.png')});}
  });
  test('lost grant reply keeps the original ID and allows explicit original completion only',async({page})=>{
    const {calls,errors}=await bridge(page,'lost');await page.goto('/');await inspect(page);
    await page.locator('#agentApprove').check();await page.locator('#agentGrant').click();
    await idle(page);
    const saved=await page.evaluate(key=>JSON.parse(localStorage.getItem(key)!),storage);
    expect(saved.step).toBe('REQUESTED');await page.reload();await inspect(page);
    await expect(page.locator('#agentGrant')).toBeDisabled();await page.locator('#agentObserve').click();
    await expect(page.locator('#agentComplete')).toBeEnabled();await page.locator('#agentComplete').click();
    await idle(page);
    await expect(page.locator('#agentComplete')).toBeDisabled();
    expect(writes(calls).map(row=>row.args[2])).toEqual(['agent-authorize','agent-complete']);
    expect(new Set(writes(calls).map(row=>row.args[row.args.indexOf('--operation')+1])).size).toBe(1);
    expect(errors).toEqual([]);
  });
});
test('another Root context is read without querying or replacing the saved original request',async({page})=>{
  const {calls}=await bridge(page);
  await page.addInitScript(({storage,original})=>localStorage.setItem(storage,JSON.stringify({schemaVersion:1,
    initializationId:'e6886dcf-a32b-4d92-8f25-11921f6de1ac',requestId:original,step:'REQUESTED'})),{storage,original});
  await page.goto('/');await inspect(page);await expect(page.locator('#agentObserve')).toBeDisabled();
  await page.locator('#agentObserve').dispatchEvent('click');
  await idle(page);await expect(page.locator('#agentResult')).toContainText('another installation');
  expect(calls.every(row=>row.args[2]==='agent-overview')).toBe(true);
  expect(await page.evaluate(key=>JSON.parse(localStorage.getItem(key)!).requestId,storage)).toBe(original);
});
test('running services do not offer a grant or perform stop and start through this page',async({page})=>{
  const {calls}=await bridge(page,'active');await page.goto('/');await inspect(page);
  await expect(page.locator('#agentGrant')).toBeDisabled();await expect(page.locator('#agentApprove')).toBeDisabled();
  expect(writes(calls)).toHaveLength(0);expect(calls.every(row=>row.args[2]==='agent-overview')).toBe(true);
});
test('storage failure blocks dispatch before the Root authorization boundary',async({page})=>{
  const {calls}=await bridge(page);await page.goto('/');await inspect(page);
  await page.evaluate(key=>{const set=Storage.prototype.setItem;Storage.prototype.setItem=function(k,v){if(k===key)throw new Error('storage unavailable');set.call(this,k,v);};},storage);
  await page.locator('#agentApprove').check();await page.locator('#agentGrant').click();
  await idle(page);
  await expect(page.locator('#agentGrant')).toBeDisabled();expect(writes(calls)).toHaveLength(0);
});
test('corrupt public journal never enables another grant after local inspection',async({page})=>{
  const {calls}=await bridge(page);await page.addInitScript(key=>localStorage.setItem(key,'corrupt'),storage);
  await page.goto('/');await inspect(page);await expect(page.locator('#agentGrant')).toBeDisabled();
  expect(writes(calls)).toHaveLength(0);
});
test('current Root pending intent can be discovered without an old client ID or a new grant',async({page})=>{
  const {calls}=await bridge(page,'pending');await page.goto('/');await inspect(page);
  await expect(page.locator('#agentComplete')).toBeDisabled();await page.locator('#agentObserve').click();
  await expect(page.locator('#agentComplete')).toBeEnabled();await page.locator('#agentComplete').click();
  await idle(page);
  await expect(page.locator('#agentComplete')).toBeDisabled();
  expect(writes(calls).map(row=>row.args[2])).toEqual(['agent-complete']);
  expect(writes(calls)[0].args).toContain(original);
});
test('two tabs serialize public intent and dispatch only one Root grant',async({page,context})=>{
  const model:Model={request:null,pending:false,active:false};const first=await bridge(page,'normal',model);
  const other=await context.newPage();const second=await bridge(other,'normal',model);
  await page.goto('/');await other.goto('/');await inspect(page);await inspect(other);
  await page.locator('#agentApprove').check();await other.locator('#agentApprove').check();
  await Promise.all([page.locator('#agentGrant').click(),other.locator('#agentGrant').click()]);
  await idle(page);await idle(other);
  await expect(page.locator('#agentGrant')).toBeDisabled();await expect(other.locator('#agentGrant')).toBeDisabled();
  expect(writes([...first.calls,...second.calls])).toHaveLength(1);
  expect([...first.errors,...second.errors]).toEqual([]);
});
test('without browser lock support the operation stays undispatched and reading remains available',async({page})=>{
  const {calls}=await bridge(page);await page.addInitScript(()=>Object.defineProperty(navigator,'locks',{value:undefined}));
  await page.goto('/');await inspect(page);await page.locator('#agentApprove').check();await page.locator('#agentGrant').click();
  await idle(page);
  await expect(page.locator('#agentResult')).toContainText('Web Locks');expect(writes(calls)).toHaveLength(0);
  await expect(page.locator('#agentRefresh')).toBeEnabled();
});
for(const changed of ['identity','source'])test(`changed ${changed} after review requires a new explicit confirmation before an intent`,async({page})=>{
  const model:Model={request:null,pending:false,active:false};const {calls}=await bridge(page,'normal',model);
  await page.goto('/');await inspect(page);await page.locator('#agentApprove').check();
  if(changed==='identity')model.initialization='331be3fc-a826-4b46-b2f0-bc3caf496aa2';else model.source='b'.repeat(64);
  await page.locator('#agentGrant').click();await idle(page);
  await expect(page.locator('#agentApprove')).not.toBeChecked();
  await expect(page.locator('#agentResult')).toContainText('changed after confirmation');
  expect(writes(calls)).toHaveLength(0);expect(await page.evaluate(key=>localStorage.getItem(key),storage)).toBeNull();
});
