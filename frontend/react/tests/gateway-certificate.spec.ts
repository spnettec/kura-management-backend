import { test, expect, type Page } from "@playwright/test";
import { readFileSync, mkdirSync } from "node:fs";
import { resolve } from "node:path";
import { createHash, X509Certificate } from "node:crypto";
import { translator, type Locale } from "../src/i18n/core";
const leaf=readFileSync(resolve(import.meta.dirname,"../../../crates/console/tests/fixtures/tls-live/server-two.pem"),"utf8");
const fingerprint=new X509Certificate(leaf).fingerprint256.replaceAll(":","").toLowerCase();
const pemHash=createHash("sha256").update(leaf).digest("hex");
const previous="a".repeat(64), id="rotation-review", token="public-contract-token-not-a-real-secret";
const storage=JSON.stringify(["yofc.gateway-certificate-operation.v1","review","rotation-admin",id]);
type Mode="normal"|"lost-applied"|"missing"|"mismatch-id"|"mismatch-hash"|"mismatch-previous"|"mismatch-mode"|"mismatch-generation"|"missing-generation"|"rejected"|"lost-bundle"|"lost-download";
async function fixture(page:Page,language:Locale,mode:Mode="normal") {
 const t=translator(language), writes:any[]=[], originals:string[]=[], currentReads:string[]=[], downloads:string[]=[];
 let receipt:any;
 await page.addInitScript(language=>{
  localStorage.setItem("yofc.console.locale",language);
  class ContractEventSource {
   onopen:((event:Event)=>void)|null=null;
   constructor(){setTimeout(()=>this.onopen?.(new Event("open")),0);}
   addEventListener(){} close(){}
  }
  Object.defineProperty(window,"EventSource",{value:ContractEventSource});
  const originalFetch=window.fetch.bind(window);
  (window as any).__rotationDispatches=[];
  window.fetch=(input,init)=>{
   const address=String(input);
   const kind=address.endsWith("/certificate")&&init?.method==="POST"?"rotation":address.includes("/identity-bundle?")?"bundle":null;
   if(kind){const key=JSON.stringify(["yofc.gateway-certificate-operation.v1","review","rotation-admin","rotation-review"]);(window as any).__rotationDispatches.push({kind,saved:JSON.parse(localStorage.getItem(key)??"null")});}
   return originalFetch(input,init);
  };
 },language);
 await page.route("**/api/v1/session",r=>r.fulfill({json:{id:"rotation-admin",username:"rotation-admin",tenantId:"review",role:"admin",csrf:"rotation-csrf",mustChangePassword:false}}));
 const gateway={id,name:"Rotation review",online:false,enabled:true,kuraAvailable:false,lastSeen:null,agentVersion:null,capabilities:[],revision:1,uptimeSeconds:0,bindingGeneration:1,certificateSha256:previous};
 await page.route(/\/api\/v1\/gateways(?:\?.*)?$/,r=>r.fulfill({json:{items:[gateway]}}));
 await page.route(/\/api\/v1\/tasks(?:\?.*)?$/,r=>r.fulfill({json:{items:[]}}));
 await page.route("**/api/v1/events*",r=>r.fulfill({contentType:"text/event-stream",body:": contract\n\n"}));
 await page.route("**/api/v1/gateways/rotation-review**",async route=>{
  const req=route.request(),path=new URL(req.url()).pathname;
  if(path.endsWith("/certificate")&&req.method()==="POST") {
   const body=req.postDataJSON();writes.push(body);
   if(!body.operationId&&process.env.YOFC_ROTATION_REVIEW_BASELINE!=="1")expect(body.operationId).toBeTruthy();
   const generationAware=body.previousBindingGeneration!==undefined;
   receipt={id,operationId:body.operationId??"legacy-no-id",state:mode==="rejected"?"REJECTED":"APPLIED",previousCertificateSha256:previous,certificateSha256:fingerprint,certificatePemSha256:pemHash,bundleRequested:!!body.clientKeyPem,revision:mode==="rejected"?null:2,rejectionCode:mode==="rejected"?"INVALID_ARGUMENT":null,identityBundleTokenRecoverable:false,receiptVersion:generationAware?2:1,previousBindingGeneration:generationAware?1:null,bindingGeneration:generationAware?2:null};
   if(["lost-applied","missing","mismatch-id","mismatch-hash","mismatch-previous","mismatch-mode","mismatch-generation","missing-generation","lost-bundle"].includes(mode)) return route.abort("connectionreset");
   return route.fulfill({json:{...receipt,extraInternalMetadata:{value:token,key:"PRIVATE KEY"},...(body.clientKeyPem?{identityBundleToken:token}:{})}});
  }
  if(path.includes("/certificate-operations/")) {
   originals.push(path);let value=mode==="missing"?{id,operationId:path.split("/").at(-1),state:"UNKNOWN"}:{...receipt};
   if(mode==="mismatch-id")value.operationId="00000000-0000-4000-8000-000000000099";
   if(mode==="mismatch-hash")value.certificatePemSha256="0".repeat(64);
   if(mode==="mismatch-previous")value.previousCertificateSha256="0".repeat(64);
   if(mode==="mismatch-mode")value.bundleRequested=!value.bundleRequested;
   if(mode==="mismatch-generation")value.bindingGeneration=3;
   if(mode==="missing-generation"){value.receiptVersion=1;value.previousBindingGeneration=null;value.bindingGeneration=null;}
   return route.fulfill({json:value});
  }
  if(path.endsWith("/identity-bundle")) {
   downloads.push(req.url());
   if(mode==="lost-download")return route.abort("connectionreset");
   return route.fulfill({contentType:"application/octet-stream",body:"public-browser-contract-not-a-private-bundle"});
  }
  if(path.endsWith("/rotation-review")) {currentReads.push(path);return route.fulfill({json:{...gateway,certificateSha256:fingerprint,revision:2,bindingGeneration:2}});}
  return route.fulfill({status:404,json:{error:{code:"NOT_FOUND"}}});
 });
 await page.goto("/gateways");await page.getByRole("button",{name:/Rotation review/}).click();await page.getByRole("button",{name:t("rotation.open"),exact:true}).click();
 const panel=page.getByRole("dialog",{name:t("rotation.open"),exact:true});
 return {page,panel,t,writes,originals,currentReads,downloads};
}
async function submit(f:Awaited<ReturnType<typeof fixture>>,bundle=false,expectDispatch=true) {
 if(bundle) {await f.panel.getByRole("radio",{name:f.t("rotation.modeBundle"),exact:true}).check();await f.panel.getByRole("textbox",{name:f.t("rotation.newKey"),exact:true}).fill("-----BEGIN PRIVATE KEY-----\nBROWSER-CONTRACT-ONLY\n-----END PRIVATE KEY-----");}
 else await f.panel.getByRole("radio",{name:f.t("rotation.modePem"),exact:true}).check();
 await f.panel.getByRole("textbox",{name:f.t("rotation.newCertificate"),exact:true}).fill(leaf);
 await f.panel.getByRole("button",{name:f.t("settings.review"),exact:true}).click();
 await f.panel.getByRole("checkbox",{name:f.t("rotation.confirm"),exact:true}).check();
 await f.panel.getByRole("button",{name:f.t("rotation.submit"),exact:true}).click();
 if(expectDispatch){
  await expect.poll(()=>f.writes.length).toBe(1);
  if(process.env.YOFC_ROTATION_REVIEW_BASELINE!=="1"){
   const snapshots=await f.page.evaluate(()=>(window as any).__rotationDispatches);
   const saved=snapshots.find((x:any)=>x.kind==="rotation").saved;
   expect(saved.operationId).toBe(f.writes[0].operationId);expect(saved.certificatePemSha256).toBe(pemHash);
   expect(saved.previousCertificateSha256).toBe(previous);expect(saved.bundleRequested).toBe(!!f.writes[0].clientKeyPem);
   expect(saved.previousBindingGeneration).toBe(bundle?undefined:1);expect(f.writes[0].previousBindingGeneration).toBe(bundle?undefined:1);
   expect(JSON.stringify(saved)).not.toContain("PRIVATE KEY");expect(JSON.stringify(saved)).not.toContain(leaf);expect(JSON.stringify(saved)).not.toContain(token);
  }
 }
}
for(const language of ["en-US","zh-CN"] as const) {
 test(`public-only rotation persists original identity before one POST in ${language}`,async({page})=>{
  const f=await fixture(page,language);await expect(f.panel.getByRole("radio",{name:f.t("rotation.modePem"),exact:true})).toBeChecked();
  await submit(f);await expect(f.panel.getByText(f.t("rotation.doneCenterOnly"),{exact:true})).toBeVisible();
  expect(f.writes).toHaveLength(1);expect(f.writes[0].clientKeyPem).toBeUndefined();expect(f.currentReads).toEqual([]);
  const saved=await page.evaluate(key=>JSON.parse(localStorage.getItem(key)!),storage);expect(saved.operationId).toMatch(/^[0-9a-f-]{36}$/);
 });
 test(`lost rotation response recovers only original receipt after reload in ${language}`,async({page})=>{
  const f=await fixture(page,language,"lost-applied");await submit(f);await expect(f.panel.getByText(f.t("rotation.uncertain"),{exact:true})).toBeVisible();
  const original=f.writes[0].operationId;await page.reload();await page.getByRole("button",{name:/Rotation review/}).click();await page.getByRole("button",{name:f.t("rotation.open"),exact:true}).click();
  await expect(f.panel.getByText(f.t("rotation.doneCenterOnly"),{exact:true})).toBeVisible();expect(f.writes).toHaveLength(1);
  expect(f.originals.every(p=>p.endsWith(original))).toBe(true);expect(f.currentReads).toEqual([]);
  const evidence=process.env.YOFC_ROTATION_BROWSER_EVIDENCE;if(evidence){mkdirSync(evidence,{recursive:true});await f.panel.screenshot({animations:"disabled",path:resolve(evidence,`original-receipt-${language}.png`)});}
 });
 test(`current fingerprint does not settle a missing original receipt in ${language}`,async({page})=>{
  const f=await fixture(page,language,"missing");await submit(f);await f.panel.getByRole("button",{name:f.t("registration.reconcile"),exact:true}).click();
  await expect(f.panel.getByText(f.t("rotation.uncertain"),{exact:true})).toBeVisible();
  expect(f.currentReads).toEqual([]);await expect(f.panel.getByText(f.t("rotation.doneCenterOnly"),{exact:true})).toHaveCount(0);
  await page.reload();await page.getByRole("button",{name:/Rotation review/}).click();await page.getByRole("button",{name:f.t("rotation.open"),exact:true}).click();
  await expect(f.panel.getByText(f.t("rotation.uncertain"),{exact:true})).toBeVisible();await expect(f.panel.getByRole("button",{name:f.t("rotation.submit"),exact:true})).toBeDisabled();expect(f.writes).toHaveLength(1);
 });
 for(const mode of ["mismatch-id","mismatch-hash","mismatch-previous","mismatch-mode","mismatch-generation","missing-generation"] as const) test(`rotation ${mode} cannot settle the operation in ${language}`,async({page})=>{
  const f=await fixture(page,language,mode);await submit(f);await f.panel.getByRole("button",{name:f.t("registration.reconcile"),exact:true}).click();
  await expect(f.panel.getByText(f.t("rotation.uncertain"),{exact:true})).toBeVisible();await expect(f.panel.getByRole("button",{name:f.t("rotation.submit"),exact:true})).toBeDisabled();
  expect(f.writes).toHaveLength(1);expect(f.currentReads).toEqual([]);
  await expect(f.panel.getByRole("button",{name:f.t("rotation.downloadReceipt"),exact:true})).toHaveCount(0);
 });
 test(`a public receipt download strips unrelated POST fields in ${language}`,async({page})=>{
  const f=await fixture(page,language);await submit(f);
  const pending=page.waitForEvent("download");await f.panel.getByRole("button",{name:f.t("rotation.downloadReceipt"),exact:true}).click();
  const download=await pending;const path=await download.path();expect(path).toBeTruthy();
  const bytes=readFileSync(path!,"utf8"), value=JSON.parse(bytes);
  expect(value).toEqual({schema:"yofc.console.gateway-certificate-rotation.v2",receiptVersion:2,id,operationId:f.writes[0].operationId,state:"APPLIED",previousCertificateSha256:previous,certificateSha256:fingerprint,certificatePemSha256:pemHash,previousBindingGeneration:1,bindingGeneration:2,revision:2,bundleRequested:false,identityBundleTokenRecoverable:false});
  expect(bytes).not.toContain(token);expect(bytes).not.toContain("identityBundleToken\"");expect(bytes).not.toContain("PRIVATE KEY");
  expect(f.downloads).toEqual([]);expect(f.currentReads).toEqual([]);expect(f.writes).toHaveLength(1);
 });
 test(`durable rejection permits a separately reviewed operation in ${language}`,async({page})=>{
  const f=await fixture(page,language,"rejected");await submit(f);await expect(f.panel.getByText(f.t("rotation.rejected"),{exact:true})).toBeVisible();
  await f.panel.getByRole("button",{name:f.t("rotation.again"),exact:true}).click();await expect(f.panel.getByRole("textbox",{name:f.t("rotation.newCertificate"),exact:true})).toBeEnabled();
  expect(await page.evaluate(key=>localStorage.getItem(key),storage)).toBeNull();expect(f.currentReads).toHaveLength(1);expect(f.writes).toHaveLength(1);
 });
 test(`a recovered private-bundle rotation never reissues the original token in ${language}`,async({page})=>{
  const f=await fixture(page,language,"lost-bundle");await submit(f,true);await f.panel.getByRole("button",{name:f.t("registration.reconcile"),exact:true}).click();
  await expect(f.panel.getByText(f.t("rotation.bundleUnavailable"),{exact:true})).toBeVisible();await expect(f.panel.getByRole("button",{name:f.t("rotation.bundleDownload"),exact:true})).toHaveCount(0);
  await expect(f.panel.getByRole("button",{name:f.t("rotation.again"),exact:true})).toBeDisabled();expect(f.writes).toHaveLength(1);expect(f.downloads).toEqual([]);
  await expect(f.panel.getByText(f.t("rotation.legacyReceipt"),{exact:true})).toBeVisible();
  await expect(f.panel.getByRole("button",{name:f.t("rotation.downloadReceipt"),exact:true})).toHaveCount(0);
 });
 test(`an uncertain one-time bundle download is never requested twice in ${language}`,async({page})=>{
  const f=await fixture(page,language,"lost-download");await submit(f,true);const button=f.panel.getByRole("button",{name:f.t("rotation.bundleDownload"),exact:true});await button.click();
  await expect(button).toBeDisabled();await expect(f.panel.getByText(f.t("rotation.bundleUnknown"),{exact:true})).toBeVisible();expect(f.downloads).toHaveLength(1);
  const snapshots=await page.evaluate(()=>(window as any).__rotationDispatches);expect(snapshots.find((x:any)=>x.kind==="bundle").saved.bundleDownloadStarted).toBe(true);
  await page.reload();await page.getByRole("button",{name:/Rotation review/}).click();await page.getByRole("button",{name:f.t("rotation.open"),exact:true}).click();await expect(f.panel.getByText(f.t("rotation.bundleUnavailable"),{exact:true})).toBeVisible();expect(f.downloads).toHaveLength(1);expect(f.writes).toHaveLength(1);
 });
 test(`a stale fingerprint review cannot approve edited certificate bytes in ${language}`,async({page})=>{
  const f=await fixture(page,language);
  await page.evaluate(()=>{const original=crypto.subtle.digest.bind(crypto.subtle);crypto.subtle.digest=((...args:any[])=>original(args[0],args[1]).then(value=>new Promise(resolve=>{(window as any).releaseReview=()=>resolve(value);}))) as typeof crypto.subtle.digest;});
  const field=f.panel.getByRole("textbox",{name:f.t("rotation.newCertificate"),exact:true});await field.fill(leaf);
  await f.panel.getByRole("button",{name:f.t("settings.review"),exact:true}).click();await page.waitForFunction(()=>typeof (window as any).releaseReview==="function");
  await field.fill(leaf+"\n");await page.evaluate(()=>(window as any).releaseReview());
  await expect(f.panel.getByRole("checkbox",{name:f.t("rotation.confirm"),exact:true})).toBeDisabled();expect(f.writes).toEqual([]);
 });
 test(`storage failure prevents any rotation dispatch in ${language}`,async({page})=>{
  const f=await fixture(page,language);await page.evaluate(()=>{const original=Storage.prototype.setItem;Storage.prototype.setItem=function(key,value){if(key.includes("gateway-certificate-operation"))throw new DOMException("blocked","QuotaExceededError");original.call(this,key,value);};});
  await submit(f,false,false);await expect(f.panel.getByText(f.t("rotation.storageFailed"),{exact:true})).toBeVisible();expect(f.writes).toEqual([]);
 });
}
