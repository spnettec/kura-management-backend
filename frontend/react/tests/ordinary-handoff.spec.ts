import { expect, test, type Page } from "@playwright/test";
const id="handoff-11111111-1111-4111-8111-111111111111";
const token="22222222-2222-4222-8222-222222222222";
async function open(page: Page, options: {role?:string;capable?:boolean;unknown?:boolean;lost?:boolean}={}) {
  const writes: any[]=[];const tasks: Record<string,any>={};let state="COMPLETED_APPROVED_CHECKS";let lostIntent:{key:string;id:string}|undefined;let lookupReady=false;
  const role=options.role??"admin";
  const plan={transactionId:id,confirmToken:token,state:"ORDINARY_PREPARED",unit:"ordinary.service",profileId:"profile-33333333-3333-4333-8333-333333333333",profileDigest:"a".repeat(64),grantSha256:"b".repeat(64),sourceJournalSha256:"c".repeat(64),snapshotSha256:"d".repeat(64),componentPids:["business.pid"],checkIds:["business.ready"],expiresAtUnixMs:Date.now()+300000,scope:"administrator-selected-checks",normalBootVerified:false,kuraRecoveryVerified:false};
  await page.route("**/api/v1/session",r=>r.fulfill({json:{id:"handoff-user",username:role,tenantId:"test",role,csrf:"test"}}));
  await page.route("**/api/v1/gateways?*",r=>r.fulfill({json:{items:[{id:"handoff-gateway",name:"Handoff fixture",online:true,enabled:true,kuraAvailable:true,revision:0,capabilities:options.capable===false?[]:["recovery.ordinary.v1"]}]}}));
  await page.route("**/api/v1/events",()=>{});
  await page.route(/\/api\/v1\/tasks(?:\?.*)?$/,r=>r.fulfill({json:{items:[]}}));
  await page.route("**/api/v1/tasks/*",r=>r.fulfill({json:tasks[r.request().url().split("/").at(-1)!]}));
  await page.route("**/api/v1/gateways/handoff-gateway/task-intents/*",r=>{
    const key=r.request().url().split("/").at(-1);
    return lookupReady&&lostIntent?.key===key?r.fulfill({json:{id:lostIntent.id}}):r.fulfill({status:404,json:{error:{code:"NOT_FOUND"}}});
  });
  await page.route("**/api/v1/gateways/handoff-gateway/tasks",async r=>{
    const v=r.request().postDataJSON();writes.push(v);const task="task-"+writes.length;
    if(options.lost&&v.operation==="recovery.ordinary.prepare"){
      lostIntent={key:v.idempotencyKey,id:"lost-original"};state="ORDINARY_PREPARED";
      tasks[lostIntent.id]={id:lostIntent.id,gatewayId:"handoff-gateway",operation:21,state:"SUCCEEDED",result:plan};
      return r.fulfill({status:503,json:{error:{code:"INTERNAL"}}});
    }
    let result:any;const operation=({"recovery.ordinary.status":20,"recovery.ordinary.prepare":21,"recovery.ordinary.execute":22} as any)[v.operation];
    if(operation===20)result={state,canPrepare:state==="COMPLETED_APPROVED_CHECKS",completionId:"completion-approved",ordinary:state==="COMPLETED_APPROVED_CHECKS"?null:{id,unit:"ordinary.service"},scope:plan.scope,normalBootVerified:false,kuraRecoveryVerified:false};
    if(operation===21){expect(v.ordinaryHandoffAction).toBeUndefined();state="ORDINARY_PREPARED";result=plan;}
    if(operation===22){expect(v.ordinaryHandoffAction).toEqual({transactionId:id,confirmationToken:token});state="ORDINARY_ACTIVE_APPROVED_CHECKS";result={id,state,ordinaryUnitHandedOff:true,scope:plan.scope,normalBootVerified:false,kuraRecoveryVerified:false};}
    tasks[task]={id:task,gatewayId:"handoff-gateway",operation,state:options.unknown&&operation===22?"UNKNOWN":"SUCCEEDED",error:options.unknown&&operation===22?"OUTCOME_UNKNOWN":null,result};
    return r.fulfill({status:202,json:{id:task,state:"QUEUED"}});
  });
  await page.goto("/gateways");await page.getByRole("button",{name:"Handoff fixture handoff-gateway",exact:true}).click();
  return {writes,tasks,allowLookup:()=>{lookupReady=true;},panel:page.getByRole("region",{name:/Kura (普通配置交接|ordinary profile handoff)/})};
}
test("ordinary handoff reviews fixed checks, restores only the original prepare, and requires explicit bilingual approval",async({page})=>{
 const f=await open(page);await f.panel.getByRole("button",{name:"读取交接状态与原任务"}).click();await f.panel.getByRole("button",{name:"准备并审阅普通配置交接"}).click();
 await expect(f.panel).toContainText("business.ready");await expect(f.panel.getByRole("button",{name:"确认普通配置交接",exact:true})).toBeDisabled();
 expect(await page.evaluate(()=>Object.values(sessionStorage).join(" "))).not.toContain(token);
 await page.reload();await page.getByRole("button",{name:"Handoff fixture handoff-gateway",exact:true}).click();await f.panel.getByRole("button",{name:"读取交接状态与原任务"}).click();
 await expect(f.panel).toContainText("business.ready");expect(f.writes.filter(v=>v.operation==="recovery.ordinary.prepare")).toHaveLength(1);
 await page.getByRole("dialog",{name:"Handoff fixture",exact:true}).getByRole("combobox",{name:"语言 / Language"}).selectOption("en-US");await expect(f.panel).toContainText("starts ordinary Kura once");
 await f.panel.getByRole("checkbox").check();await f.panel.getByRole("button",{name:"Confirm ordinary handoff",exact:true}).click();await expect(f.panel).toContainText("ORDINARY_ACTIVE_APPROVED_CHECKS");expect(f.writes.filter(v=>v.operation==="recovery.ordinary.execute")).toHaveLength(1);
});
test("unknown handoff survives reload without trusting current unit state or sending a replacement",async({page})=>{
 const f=await open(page,{unknown:true});await f.panel.getByRole("button",{name:"读取交接状态与原任务"}).click();await f.panel.getByRole("button",{name:"准备并审阅普通配置交接"}).click();await f.panel.getByRole("checkbox").check();await f.panel.getByRole("button",{name:"确认普通配置交接",exact:true}).click();
 await expect(f.panel).toContainText("不重发");await page.reload();await page.getByRole("button",{name:"Handoff fixture handoff-gateway",exact:true}).click();await f.panel.getByRole("button",{name:"读取交接状态与原任务"}).click();
 await expect(f.panel).toContainText("不重发");await expect(f.panel.getByRole("button",{name:"准备并审阅普通配置交接"})).toBeDisabled();expect(f.writes.filter(v=>v.operation==="recovery.ordinary.execute")).toHaveLength(1);expect(f.writes.filter(v=>v.operation==="recovery.ordinary.prepare")).toHaveLength(1);
});
test("an original late receipt is read through UNKNOWN without replay",async({page})=>{
 const f=await open(page,{unknown:true});await f.panel.getByRole("button",{name:"读取交接状态与原任务"}).click();await f.panel.getByRole("button",{name:"准备并审阅普通配置交接"}).click();await f.panel.getByRole("checkbox").check();await f.panel.getByRole("button",{name:"确认普通配置交接",exact:true}).click();
 const original=Object.values(f.tasks).find((v:any)=>v.operation===22)!;await expect(f.panel).toContainText("不重发");original.state="SUCCEEDED";original.error=null;
 await expect(f.panel).toContainText("ORDINARY_ACTIVE_APPROVED_CHECKS");await expect(f.panel.getByText(/原请求仍待核对/)).toHaveCount(0);expect(f.writes.filter(v=>v.operation==="recovery.ordinary.execute")).toHaveLength(1);
});
test("lost submission acknowledgement remains gated across reload with only an intent key",async({page})=>{
 const f=await open(page,{lost:true});await f.panel.getByRole("button",{name:"读取交接状态与原任务"}).click();await f.panel.getByRole("button",{name:"准备并审阅普通配置交接"}).click();await expect(f.panel).toContainText("不重发");
 await page.reload();await page.getByRole("button",{name:"Handoff fixture handoff-gateway",exact:true}).click();await f.panel.getByRole("button",{name:"读取交接状态与原任务"}).click();await expect(f.panel.getByRole("button",{name:"准备并审阅普通配置交接"})).toBeDisabled();expect(f.writes.filter(v=>v.operation==="recovery.ordinary.prepare")).toHaveLength(1);
 f.allowLookup();await f.panel.getByRole("button",{name:"读取交接状态与原任务"}).click();await expect(f.panel).toContainText("business.ready");
 await expect(f.panel.getByText(/原请求仍待核对/)).toHaveCount(0);await expect(f.panel.getByRole("button",{name:"确认普通配置交接",exact:true})).toBeDisabled();
 expect(f.writes.filter(v=>v.operation==="recovery.ordinary.prepare")).toHaveLength(1);
});
for(const options of [{role:"operator"},{capable:false}])test(`ordinary handoff hidden without administrative authority ${JSON.stringify(options)}`,async({page})=>{const f=await open(page,options);await expect(f.panel).toHaveCount(0);expect(f.writes).toHaveLength(0);});
