import { test, expect, type Page } from '@playwright/test';
import { createHash } from 'node:crypto';
import { mkdirSync } from 'node:fs';
import { resolve } from 'node:path';
// UI contracts only. Real root permissions/dpkg/Cockpit/PAM require separate evidence.
const data = Buffer.from('unit-deb');
const release = {key:'unit-release',version:'0.1.0~native-contract-1',architecture:'arm64',archiveSize:data.length,archiveSha256:createHash('sha256').update(data).digest('hex')};
const original = '244c78db-8ae1-431a-80a6-ed961a197a4e';
const storage = 'yofc.console.native-package.v1';
type Mode = 'success'|'lost-install'|'missing'|'receipt'|'held'|'pending'|'foreign'|'lost-upload'|'active'|'fresh'|'damaged';
async function bridge(page:Page,mode:Mode='success') {
  const calls:any[] = [], files:any[] = [];
  let started = false;
  let record:any = mode==='foreign'?{operationId:original,phase:'UNKNOWN',package:release}:null;
  await page.exposeFunction('__nativeRpc', async (args:string[],options:unknown) => {
    calls.push({args,options});
    const command=args[2],operation=args.includes('--operation')?args[args.indexOf('--operation')+1]:null;
    if(command==='native-overview')return {packages:[release],units:[{unit:'kura-management-console.service'}],allStopped:mode!=='active'&&!started,maintenance:null,current:mode==='foreign'?record:null,
      installedCore:{state:mode==='damaged'?'UNVERIFIED':mode==='fresh'&&record?.phase!=='APPLIED'?'ABSENT':'VERIFIED',version:release.version,architecture:release.architecture}};
    if(command==='start') { started = true; return {state:'active',ready:true,readiness:'READY',httpPort:18443}; }
    if(command==='native-operation')return mode==='missing'&&record?.phase==='UNKNOWN'?{operationId:operation,phase:'UNOBSERVED',transportNonDeliveryCertified:false}:record||{operationId:operation,phase:'UNOBSERVED',transportNonDeliveryCertified:false};
    if(command.startsWith('native-')) {
      const saved=await page.evaluate(key=>JSON.parse(localStorage.getItem(key)!),storage);
      expect(saved.operationId).toBe(operation);
      const expected:Record<string,string>={'native-upload-prepare':'UPLOAD_REQUESTED','native-install':'INSTALL_REQUESTED','native-retire':'RETIRE_REQUESTED','native-verify':'VERIFY_REQUESTED','native-finalize':'FINALIZE_REQUESTED'};
      expect(saved.step).toBe(expected[command]);
    }
    if(command==='native-upload-prepare') {
      record={operationId:operation,phase:'UPLOADING',package:release,uploadReady:false,uploadPath:'/var/lib/yofc-console-native-packages/uploads/'+operation+'.deb'};
      if(mode==='lost-upload')throw new Error('prepare reply lost');
    }
    if(command==='native-install') {
      record={operationId:operation,package:release,phase:['lost-install','missing','receipt'].includes(mode)?'UNKNOWN':mode==='pending'?'PENDING':'APPLIED',canVerify:mode==='receipt',finalizationRequired:mode==='held'};
      if(['lost-install','missing','receipt'].includes(mode))throw new Error('original response lost');
    }
    if(command==='native-retire')record={operationId:operation,phase:'RETIRED',lateActionsFenced:true};
    if(command==='native-verify'||command==='native-finalize')record={...record,phase:'APPLIED',finalizationRequired:false};
    return record||{};
  });
  await page.exposeFunction('__nativeFile', (path:string,options:unknown,length:number) => {files.push({path,options,length});record.uploadReady=true;});
  await page.exposeFunction('__nativeCallsSnapshot',()=>calls);
  await page.exposeFunction('__nativeFilesSnapshot',()=>files);
  await page.route('**/base1/cockpit.js',route=>route.fulfill({contentType:'text/javascript',body:`
    window.__nativeCalls=${JSON.stringify(calls)};window.__files=${JSON.stringify(files)};
    window.cockpit={
      file:(path,options)=>({replace:async data=>{await window.__nativeFile(path,options,data.length);window.__files=await window.__nativeFilesSnapshot();},close:()=>{}}),
      spawn:async(args,options)=>{
        try{return JSON.stringify(await window.__nativeRpc(args,options));}
        finally{window.__nativeCalls=await window.__nativeCallsSnapshot();}
      }
    };
  `}));
}
async function upload(page:Page) {
  await page.locator('#nativeRefresh').click();
  await page.locator('#nativeFile').setInputFiles({name:'approved.deb',mimeType:'application/vnd.debian.binary-package',buffer:data});
  await page.locator('#nativeUpload').click();
  await expect(page.locator('#nativeApprove')).toBeEnabled();
  await expect(page.locator('#message')).toHaveText('');
}
for(const language of ['zh','en'] as const)test.describe(language,()=>{
  test.use({locale:language==='zh'?'zh-CN':'en-US'});
  test('fresh host reviews software before setup, then enables only an explicit first start',async({page})=>{
    await bridge(page,'fresh');await page.goto('/');
    for(const id of ['start','status','read'])await expect(page.locator('#'+id)).toBeDisabled();
    expect(await page.evaluate(()=>(window as any).__nativeCalls)).toEqual([]);
    await upload(page);
    await expect(page.locator('#nativeInstalledState')).toContainText(language==='zh'?'尚未安装':'not installed');
    await expect(page.locator('#start')).toBeDisabled();
    await page.locator('#nativeApprove').check();await page.locator('#nativeInstall').click();
    await expect(page.locator('#start')).toBeEnabled();
    await expect(page.locator('#nativeInstalledState')).toContainText(language==='zh'?'来源摘要核验通过':'source digests are verified');
    expect(await page.evaluate(()=>(window as any).__nativeCalls.some((row:any)=>['start','credential'].includes(row.args[2])))).toBe(false);
    await page.reload();await expect(page.locator('#start')).toBeEnabled();
    expect(await page.evaluate(()=>(window as any).__nativeCalls.filter((row:any)=>row.args[2]==='native-install').length)).toBe(1);
    await page.locator('#start').click();
    await expect(page.locator('#nativeHostState')).toContainText(language==='zh'?'停服信息已过期':'snapshot expired');
    await expect(page.locator('#nativeApprove')).toBeDisabled();
    expect(await page.evaluate(()=>(window as any).__nativeCalls.filter((row:any)=>row.args[2]==='start').length)).toBe(1);
  });
  test('unverified installed software keeps startup and credential export blocked',async({page})=>{
    await bridge(page,'damaged');await page.goto('/');await page.locator('#nativeRefresh').click();
    await expect(page.locator('#nativeInstalledState')).toContainText(language==='zh'?'尚未核实':'unverified');
    for(const id of ['start','status','read'])await expect(page.locator('#'+id)).toBeDisabled();
    expect(await page.evaluate(()=>(window as any).__nativeCalls.map((row:any)=>row.args[2]))).toEqual(['native-overview']);
  });
  test('native package review, authenticated upload and installation do not start a center',async({page})=>{
    await bridge(page);await page.goto('/');
    expect(await page.evaluate(()=>(window as any).__nativeCalls)).toEqual([]);
    await upload(page);
    await expect(page.locator('#nativeInstall')).toBeDisabled();
    await page.locator('#nativeApprove').check();await page.locator('#nativeInstall').click();
    await expect(page.locator('#nativeResult')).toContainText(language==='zh'?'原安装成功回执':'original install receipt');
    await expect(page.locator('#nativeRefresh')).toBeEnabled();
    const calls=await page.evaluate(()=>(window as any).__nativeCalls);
    expect(calls.map((row:any)=>row.args[2])).toEqual(['native-overview','native-upload-prepare','native-operation','native-install','native-overview']);
    expect(calls.every((row:any)=>row.options.superuser==='require')).toBe(true);
    const saved=await page.evaluate(key=>JSON.parse(localStorage.getItem(key)!),storage);
    expect(Object.keys(saved).sort()).toEqual(['operationId','packageKey','step']);expect(saved.step).toBe('APPLIED');
    expect(JSON.stringify(saved)).not.toContain('uploadPath');
    expect(await page.evaluate(()=>(window as any).__files[0].options)).toEqual({binary:true,superuser:'require'});
    await page.reload();await expect(page.locator('#nativeResult')).toContainText(saved.operationId);
    expect(await page.evaluate(()=>(window as any).__nativeCalls.filter((row:any)=>row.args[2]==='native-install').length)).toBe(1);
    const evidence=process.env.YOFC_NATIVE_SCREENSHOTS;if(evidence){mkdirSync(evidence,{recursive:true});await page.locator('#nativePackages').screenshot({path:resolve(evidence,'native-installed-'+language+'.png')});}
  });
  for(const mode of ['lost-install','missing'] as const)test(`native ${mode} retains its original id and disables another installation after reload`,async({page})=>{
    await bridge(page,mode);await page.goto('/');await upload(page);
    await page.locator('#nativeApprove').check();await page.locator('#nativeInstall').click();
    await expect(page.locator('#nativeInstall')).toBeDisabled();
    const saved=await page.evaluate(key=>JSON.parse(localStorage.getItem(key)!),storage);
    await page.reload();await expect(page.locator('#nativeResult')).toContainText(saved.operationId);
    await expect(page.locator('#nativeInstall')).toBeDisabled();await expect(page.locator('#nativeUpload')).toBeDisabled();
    await page.locator('#nativeObserve').click();
    expect(await page.evaluate(()=>(window as any).__nativeCalls.filter((row:any)=>row.args[2]==='native-install').length)).toBe(1);
    expect(await page.evaluate(key=>JSON.parse(localStorage.getItem(key)!).operationId,storage)).toBe(saved.operationId);
  });
});
test('native package hash mismatch prevents file upload and request acceptance',async({page})=>{
  await bridge(page);await page.goto('/');await page.locator('#nativeRefresh').click();
  await page.locator('#nativeFile').setInputFiles({name:'wrong.deb',mimeType:'application/octet-stream',buffer:Buffer.from('wrongxxx')});
  await page.locator('#nativeUpload').click();
  expect(await page.evaluate(()=>(window as any).__files.length)).toBe(0);
  expect(await page.evaluate(()=>(window as any).__nativeCalls.map((row:any)=>row.args[2]))).toEqual(['native-overview']);
});
test('native package durable browser storage failure prevents any prepare/upload dispatch',async({page})=>{
  await bridge(page);await page.goto('/');await page.locator('#nativeRefresh').click();
  await page.locator('#nativeFile').setInputFiles({name:'approved.deb',mimeType:'application/octet-stream',buffer:data});
  await page.evaluate(()=>{const original=Storage.prototype.setItem;Storage.prototype.setItem=function(key,value){if(key==='yofc.console.native-package.v1')throw new Error('unit-only storage failure');return original.call(this,key,value);};});
  await page.locator('#nativeUpload').click();
  expect(await page.evaluate(()=>(window as any).__nativeCalls.map((row:any)=>row.args[2]))).toEqual(['native-overview']);
  expect(await page.evaluate(()=>(window as any).__files.length)).toBe(0);
});
test('corrupt saved native metadata cannot enable startup after a valid software inspection',async({page})=>{
  await bridge(page);await page.addInitScript(()=>localStorage.setItem('yofc.console.native-package.v1','corrupt'));
  await page.goto('/');await page.locator('#nativeRefresh').click();
  await expect(page.locator('#nativeInstalledState')).toContainText('source digests are verified');
  await expect(page.locator('#start')).toBeDisabled();await expect(page.locator('#nativeUpload')).toBeDisabled();
  expect(await page.evaluate(()=>(window as any).__nativeCalls.map((row:any)=>row.args[2]))).toEqual(['native-overview']);
});
test('actual success receipt permits explicit original verification without reinstalling',async({page})=>{
  await bridge(page,'receipt');await page.goto('/');await upload(page);await page.locator('#nativeApprove').check();await page.locator('#nativeInstall').click();
  await page.locator('#nativeObserve').click();
  await expect(page.locator('#nativeVerify')).toBeEnabled();await page.locator('#nativeVerify').click();
  await expect(page.locator('#nativeFinish')).toBeEnabled();
  expect(await page.evaluate(()=>(window as any).__nativeCalls.filter((row:any)=>['native-install','native-verify'].includes(row.args[2])).map((row:any)=>row.args[2]))).toEqual(['native-install','native-verify']);
});
test('applied package with a retained fence only offers explicit finalization, without start or reinstall',async({page})=>{
  await bridge(page,'held');await page.goto('/');await upload(page);await page.locator('#nativeApprove').check();await page.locator('#nativeInstall').click();
  await expect(page.locator('#nativeFinish')).toBeDisabled();await expect(page.locator('#nativeFinalize')).toBeEnabled();
  await page.locator('#nativeFinalize').click();await expect(page.locator('#nativeFinish')).toBeEnabled();
  expect(await page.evaluate(()=>(window as any).__nativeCalls.filter((row:any)=>row.args[2]==='native-install').length)).toBe(1);
  expect(await page.evaluate(()=>(window as any).__nativeCalls.some((row:any)=>row.args[2]==='start'))).toBe(false);
});
test('another profile original UNKNOWN blocks a new upload without replacing its identity',async({page})=>{
  await bridge(page,'foreign');await page.goto('/');await page.locator('#nativeRefresh').click();
  await expect(page.locator('#nativeResult')).toContainText(original);
  await expect(page.locator('#nativeUpload')).toBeDisabled();await expect(page.locator('#nativeInstall')).toBeDisabled();await expect(page.locator('#nativeRetire')).toBeDisabled();
  expect(await page.evaluate(()=>(window as any).__nativeCalls.map((row:any)=>row.args[2]))).toEqual(['native-overview']);
});
test('unobserved upload may only be explicitly retired on its original id, fencing delayed actions',async({page})=>{
  await bridge(page,'lost-upload');await page.goto('/');await page.locator('#nativeRefresh').click();
  await page.locator('#nativeFile').setInputFiles({name:'approved.deb',mimeType:'application/octet-stream',buffer:data});await page.locator('#nativeUpload').click();
  await expect(page.locator('#nativeUpload')).toBeDisabled();await page.locator('#nativeObserve').click();
  await expect(page.locator('#nativeRetire')).toBeEnabled();await page.locator('#nativeRetire').click();
  await expect(page.locator('#nativeFinish')).toBeEnabled();
  const calls=await page.evaluate(()=>(window as any).__nativeCalls.filter((row:any)=>['native-upload-prepare','native-retire'].includes(row.args[2])));
  expect(calls.map((row:any)=>row.args[2])).toEqual(['native-upload-prepare','native-retire']);expect(calls[0].args[calls[0].args.indexOf('--operation')+1]).toBe(calls[1].args.at(-1));
});
test('active centers cannot be implicitly stopped by native package installation',async({page})=>{
  await bridge(page,'active');await page.goto('/');await page.locator('#nativeRefresh').click();
  await page.locator('#nativeFile').setInputFiles({name:'approved.deb',mimeType:'application/octet-stream',buffer:data});await page.locator('#nativeUpload').click();
  await expect(page.locator('#nativeApprove')).toBeDisabled();await expect(page.locator('#nativeInstall')).toBeDisabled();
  expect(await page.evaluate(()=>(window as any).__nativeCalls.some((row:any)=>['start','maintenance-stop','native-install'].includes(row.args[2])))).toBe(false);
});
test('native pending jobs poll only the same original, with a finite observation budget',async({page})=>{
  await page.clock.install();await bridge(page,'pending');await page.goto('/');await upload(page);
  await page.locator('#nativeApprove').check();await page.locator('#nativeInstall').click();
  for(let step=0;step<18;step++){await page.clock.runFor(1000);await expect(page.locator('#nativeObserve')).toBeEnabled();}
  await expect(page.locator('#nativeResult')).toContainText('This read-only observation ended');
  expect(await page.evaluate(()=>(window as any).__nativeCalls.filter((row:any)=>row.args[2]==='native-install').length)).toBe(1);
});
