import { expect, test, type Page } from "@playwright/test";
const id="handoff-11111111-1111-4111-8111-111111111111";
const token="22222222-2222-4222-8222-222222222222";
async function open(page: Page, options: {role?:string;capable?:boolean;unknown?:boolean;lost?:boolean;cancelCap?:boolean;expired?:boolean;lostCancel?:boolean;overclaim?:boolean}={}) {
  const writes: any[]=[];const tasks: Record<string,any>={};let state="COMPLETED_APPROVED_CHECKS";let lostIntent:{key:string;id:string}|undefined;let lookupReady=false;
  const role=options.role??"admin";
  const plan={transactionId:id,confirmToken:token,state:"ORDINARY_PREPARED",unit:"ordinary.service",profileId:"profile-33333333-3333-4333-8333-333333333333",profileDigest:"a".repeat(64),grantSha256:"b".repeat(64),sourceJournalSha256:"c".repeat(64),snapshotSha256:"d".repeat(64),componentPids:["business.pid"],checkIds:["business.ready"],expiresAtUnixMs:Date.now()+(options.expired?-60000:300000),scope:"administrator-selected-checks",normalBootVerified:false,kuraRecoveryVerified:false};
  await page.route("**/api/v1/session",r=>r.fulfill({json:{id:"handoff-user",username:role,tenantId:"test",role,csrf:"test"}}));
  await page.route("**/api/v1/gateways?*",r=>r.fulfill({json:{items:[{id:"handoff-gateway",name:"Handoff fixture",online:true,enabled:true,kuraAvailable:true,revision:0,capabilities:options.capable===false?[]:["recovery.ordinary.v1",...(options.cancelCap===false?[]:["recovery.ordinary.cancel.v1"])]}]}}));
  await page.route("**/api/v1/events",()=>{});
  for(const route of ["signers","approvals","issuances"])await page.route(`**/api/v1/transfer/${route}`,r=>r.fulfill({json:{items:[]}}));
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
    let result:any;const operation=({"recovery.ordinary.status":20,"recovery.ordinary.prepare":21,"recovery.ordinary.execute":22,"recovery.ordinary.cancel":29} as any)[v.operation];
    if(operation===20)result={state,canPrepare:["COMPLETED_APPROVED_CHECKS","ORDINARY_CANCELLED"].includes(state),completionId:"completion-approved",ordinary:state==="COMPLETED_APPROVED_CHECKS"?null:{id,unit:"ordinary.service"},scope:plan.scope,normalBootVerified:false,kuraRecoveryVerified:false};
    if(operation===21){expect(v.ordinaryHandoffAction).toBeUndefined();state="ORDINARY_PREPARED";result=plan;}
    if(operation===22){expect(v.ordinaryHandoffAction).toEqual({transactionId:id,confirmationToken:token});state="ORDINARY_ACTIVE_APPROVED_CHECKS";result={id,state,ordinaryUnitHandedOff:true,scope:plan.scope,normalBootVerified:false,kuraRecoveryVerified:false};}
    if(operation===29){expect(v.ordinaryHandoffAction).toEqual({transactionId:id,confirmationToken:token});state="ORDINARY_CANCELLED";result={id,state,cancelled:true,issued:false,ordinaryUnitHandedOff:options.overclaim??false,scope:plan.scope,normalBootVerified:false,kuraRecoveryVerified:false};}
    if(options.lostCancel&&operation===29){lostIntent={key:v.idempotencyKey,id:"lost-cancel-original"};tasks[lostIntent.id]={id:lostIntent.id,gatewayId:"handoff-gateway",operation:29,state:"SUCCEEDED",result};return r.fulfill({status:503,json:{error:{code:"INTERNAL"}}});}
    tasks[task]={id:task,gatewayId:"handoff-gateway",operation,state:options.unknown&&[22,29].includes(operation)?"UNKNOWN":"SUCCEEDED",error:options.unknown&&[22,29].includes(operation)?"OUTCOME_UNKNOWN":null,result};
    return r.fulfill({status:202,json:{id:task,state:"QUEUED"}});
  });
  await page.goto("/gateways");await page.getByRole("button",{name:"Handoff fixture handoff-gateway",exact:true}).click();
  return {writes,tasks,allowLookup:()=>{lookupReady=true;},panel:page.getByRole("region",{name:/Kura (普通配置交接|ordinary profile handoff)/})};
}
test("expired unissued handoff can be cancelled explicitly in both languages without starting Kura",async({page})=>{
 const f=await open(page,{expired:true});await f.panel.getByRole("button",{name:"读取交接状态与原任务"}).click();await f.panel.getByRole("button",{name:"准备并审阅普通配置交接"}).click();
 await expect(f.panel.getByRole("button",{name:"确认普通配置交接",exact:true})).toBeDisabled();await expect(f.panel.getByRole("button",{name:"取消尚未执行的交接计划"})).toBeEnabled();expect(await page.evaluate(()=>Object.values(sessionStorage).join(" "))).not.toContain(token);
 await page.getByRole("dialog").getByRole("combobox",{name:"语言 / Language"}).selectOption("en-US");await f.panel.getByRole("button",{name:"Cancel unissued handoff plan"}).click();await expect(f.panel).toContainText("ORDINARY_CANCELLED");await expect(f.panel.getByRole("button",{name:"Prepare and review ordinary handoff"})).toBeEnabled();
 await expect(f.panel.getByRole("button",{name:/Read handoff state and original task|读取交接状态与原任务/})).not.toHaveClass(/ant-btn-loading/);
 if(process.env.KURA_CANCEL_SCREENSHOT_ROOT)await page.screenshot({path:process.env.KURA_CANCEL_SCREENSHOT_ROOT+"/cancel-unissued-en.png",fullPage:true,animations:"disabled"});
 expect(f.writes.filter(v=>v.operation==="recovery.ordinary.cancel")).toHaveLength(1);expect(f.writes.filter(v=>v.operation==="recovery.ordinary.execute")).toHaveLength(0);
});
test("lost cancellation response reads its original intent after reload without replacement commands",async({page})=>{
 const f=await open(page,{lostCancel:true});await f.panel.getByRole("button",{name:"读取交接状态与原任务"}).click();await f.panel.getByRole("button",{name:"准备并审阅普通配置交接"}).click();await f.panel.getByRole("button",{name:"取消尚未执行的交接计划"}).click();await expect(f.panel).toContainText("不重发");
 await page.reload();await page.getByRole("button",{name:"Handoff fixture handoff-gateway",exact:true}).click();await f.panel.getByRole("button",{name:"读取交接状态与原任务"}).click();await expect(f.panel.getByRole("button",{name:"准备并审阅普通配置交接"})).toBeDisabled();
 f.allowLookup();await f.panel.getByRole("button",{name:"读取交接状态与原任务"}).click();await expect(f.panel).toContainText("ORDINARY_CANCELLED");await expect(f.panel.getByRole("button",{name:"准备并审阅普通配置交接"})).toBeEnabled();expect(f.writes.filter(v=>v.operation==="recovery.ordinary.cancel")).toHaveLength(1);expect(f.writes.filter(v=>v.operation==="recovery.ordinary.prepare")).toHaveLength(1);
 await expect(f.panel.getByRole("button",{name:/Read handoff state and original task|读取交接状态与原任务/})).not.toHaveClass(/ant-btn-loading/);
 if(process.env.KURA_CANCEL_SCREENSHOT_ROOT)await page.screenshot({path:process.env.KURA_CANCEL_SCREENSHOT_ROOT+"/cancel-original-intent-zh.png",fullPage:true,animations:"disabled"});
});
test("UNKNOWN cancellation only reads the original task and rejects success overclaims",async({page})=>{
 const f=await open(page,{unknown:true,overclaim:true});await f.panel.getByRole("button",{name:"读取交接状态与原任务"}).click();await f.panel.getByRole("button",{name:"准备并审阅普通配置交接"}).click();await f.panel.getByRole("button",{name:"取消尚未执行的交接计划"}).click();await expect(f.panel).toContainText("不重发");
 const original=Object.values(f.tasks).find((t:any)=>t.operation===29)!;original.state="SUCCEEDED";original.error=null;
 await expect(f.panel.getByRole("button",{name:"读取交接状态与原任务"})).toBeEnabled();await expect(f.panel).toContainText("请求与当前状态或已有任务冲突");
 await expect(f.panel.getByRole("button",{name:"准备并审阅普通配置交接"})).toBeDisabled();await expect(f.panel).toContainText("不重发");expect(f.writes.filter(v=>v.operation==="recovery.ordinary.cancel")).toHaveLength(1);
});
test("older agents do not advertise or show cancellation",async({page})=>{
 const f=await open(page,{cancelCap:false});await f.panel.getByRole("button",{name:"读取交接状态与原任务"}).click();await f.panel.getByRole("button",{name:"准备并审阅普通配置交接"}).click();await expect(f.panel.getByRole("button",{name:"取消尚未执行的交接计划"})).toHaveCount(0);expect(f.writes.filter(v=>v.operation==="recovery.ordinary.cancel")).toHaveLength(0);
});
