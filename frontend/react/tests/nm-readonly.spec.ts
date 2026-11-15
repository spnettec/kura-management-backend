import { test,expect,type Page } from "@playwright/test";
import { randomUUID } from "node:crypto";
import { resolve } from "node:path";
import { mkdirSync } from "node:fs";
const token="a".repeat(32),operationId="11111111-1111-4111-8111-111111111111";
const evidence=process.env.YOFC_READONLY_EVIDENCE_DIR||resolve(import.meta.dirname,"../../../target/nm-readonly-ui");
mkdirSync(evidence,{recursive:true});
const base={schemaVersion:1,backend:"NetworkManager",writeEnabled:false,sourceDigest:"a".repeat(64),
 candidateDigest:null,confirmationReady:false,transaction:null};
async function fixture(page:Page,view:any,prepared=false) {
  const writes:Record<string,any>[]=[];
  const tasks:Record<string,any>={},intents:Record<string,string>={};
  let transaction:any=view.transaction;
  await page.route("**/api/v1/session",r=>r.fulfill({json:{id:"nat-user",username:"fixture",tenantId:"test",role:"admin",csrf:"owned-nat-test"}}));
  await page.route("**/api/v1/gateways?*",r=>r.fulfill({json:{items:[{id:"nat-gateway",name:"NAT fixture",online:true,enabled:true,kuraAvailable:false,revision:0,bindingGeneration:1,capabilities:["nm.remote.v1"]}]}}));
  await page.route("**/api/v1/transfer/signers",r=>r.fulfill({json:{items:[]}}));
  await page.route("**/api/v1/transfer/approvals*",r=>r.fulfill({json:{items:[]}}));
  await page.route("**/api/v1/events",()=>{});
  await page.route(/\/api\/v1\/tasks(?:\?.*)?$/,r=>r.fulfill({json:{items:[]}}));
  await page.route("**/api/v1/tasks/*",r=>{
    const task=tasks[r.request().url().split("/").at(-1)!];
    return r.fulfill({json:task});
  });
  await page.route("**/api/v1/gateways/nat-gateway/task-intents/*",r=>{
    const id=intents[r.request().url().split("/").at(-1)!];
    return r.fulfill(id?{json:{id}}:{status:404,json:{error:{code:"NOT_FOUND"}}});
  });
  const codes:Record<string,number>={"nm.view":34,"nm.prepare":35,"nm.confirm":36,"nm.rollback":37};
  await page.route("**/api/v1/gateways/nat-gateway/tasks",async r=>{
    const body=r.request().postDataJSON();writes.push(body);
    const id=randomUUID(),operation=codes[body.operation];intents[body.idempotencyKey]=id;
    let result:any;
    if(operation===34)result=view;
    if(operation===35){transaction={id:operationId,state:"WAIT_CONFIRM",deadlineUnixMs:Date.now()+120000,
      confirmationToken:token,newManagementConnectionRequired:true};result={...transaction};}
    if(operation===36||operation===37){
      expect(body.nmAction).toEqual({operationId,confirmationToken:token});
      transaction={...transaction,state:operation===36?"COMMITTED":"ROLLED_BACK"};
      result={id:operationId,state:transaction.state,deadlineUnixMs:transaction.deadlineUnixMs,
        newManagementConnectionVerified:operation===36};
    }
    tasks[id]={id,gatewayId:"nat-gateway",operation,state:"SUCCEEDED",error:null,result};

    return r.fulfill({status:202,json:{id}});
  });
  const select=()=>page.getByRole("button",{name:"NAT fixture nat-gateway",exact:true}).click();
  if(prepared)await page.addInitScript(({token,operationId})=>{
    sessionStorage.setItem("yofc.nm:nat-user:nat-gateway:1:prepare","saved-prepare-id");
  },{token,operationId});
  if(prepared)tasks["saved-prepare-id"]={id:"saved-prepare-id",gatewayId:"nat-gateway",operation:35,state:"SUCCEEDED",
    result:{id:operationId,state:"WAIT_CONFIRM",deadlineUnixMs:Date.now()+120000,confirmationToken:token,newManagementConnectionRequired:true}};
  await page.goto("/gateways");await select();
  const panel=page.getByRole("region",{name:/^(NM 网络事务|NM network transactions)$/});
  const read=()=>panel.getByRole("button",{name:/^(读取 NM 来源与原任务|Read the NM source and original task)/}).click();
  return {writes,panel,read,select};
}

for(const language of ["zh-CN","en-US"] as const) {
 test(`read-only NM view displays status without editor in ${language}`,async({page})=>{
  const f=await fixture(page,base);
  if(language==="en-US")await page.getByRole("dialog",{name:"NAT fixture",exact:true}).getByRole("combobox",{name:"语言 / Language"}).selectOption(language);
  await f.read();await expect(f.panel).toContainText(language==="en-US"?"Source digest":"来源摘要");
  await expect(f.panel).toContainText(language==="en-US"?"not authorized":"未授权");
  await expect(f.panel.getByRole("textbox")).toHaveCount(0);
  for(const name of language==="en-US"?["Apply reviewed proposal temporarily","Confirm proposal","Roll back proposal"]:["临时应用已审阅提案","确认保留提案","显式回退提案"])
   await expect(f.panel.getByRole("button",{name,exact:true})).toBeDisabled();
  expect(f.writes.map(v=>v.operation)).toEqual(["nm.view"]);
  await page.screenshot({path:resolve(evidence,`readonly-${language}.png`),animations:"disabled"});
 });
}
test("read-only WAIT_CONFIRM cannot use a previously saved prepare token",async({page})=>{
 const f=await fixture(page,{...base,candidateDigest:"b".repeat(64),transaction:{id:operationId,state:"WAIT_CONFIRM",deadlineUnixMs:Date.now()+120000}},true);
 await f.read();await expect(f.panel).toContainText("剩余确认秒数");
 await expect(f.panel.getByRole("button",{name:"显式回退提案",exact:true})).toBeDisabled();
 await expect(f.panel.getByRole("checkbox")).toHaveCount(0);
 expect(f.writes.map(v=>v.operation)).toEqual(["nm.view"]);
});
test("PREPARED view shows original status without inventing a deadline",async({page})=>{
 const f=await fixture(page,{...base,candidateDigest:"b".repeat(64),transaction:{id:operationId,state:"PREPARED",deadlineUnixMs:null}});
 await f.read();await expect(f.panel).toContainText(operationId);
 await expect(f.panel).not.toContainText("剩余确认秒数");expect(f.writes.map(v=>v.operation)).toEqual(["nm.view"]);
});
test("malformed readonly confirmation authority is rejected",async({page})=>{
 const f=await fixture(page,{...base,confirmationReady:true});await f.read();
 await expect(f.panel).not.toContainText("aaaaaaaaaaaaaaaa");
 expect(f.writes.map(v=>v.operation)).toEqual(["nm.view"]);
});
