import { expect, test, type Page } from "@playwright/test";
import { resolve } from "node:path";
import { mkdirSync } from "node:fs";
const evidence=resolve(import.meta.dirname,"../../../target/restart-reboot-lifecycle-20261106/browser");
const id="system-11111111-1111-4111-8111-111111111111",token="22222222-2222-4222-8222-222222222222";
async function open(page:Page,options:{role?:string;capable?:boolean;reboot?:boolean;lostPrepare?:boolean;unknown?:boolean;recovery?:boolean}={}) {
  const {role="admin",capable=true,reboot=true,lostPrepare=false,unknown=false,recovery=false}=options;
  const writes:any[]=[];const tasks:Record<string,any>={};const intents:Record<string,string>={};let transaction:any=null;
  await page.route("**/api/v1/session",r=>r.fulfill({json:{id:"system-user",username:role,tenantId:"test",role,csrf:"test"}}));
  await page.route("**/api/v1/gateways?*",r=>r.fulfill({json:{items:[{id:"system-gateway",name:"System fixture",online:true,enabled:true,kuraAvailable:false,agentVersion:"system-control-fixture",revision:0,capabilities:capable?["system.control.v1","service.restart.v1",...(reboot?["system.reboot.v1"]:[])]:[]}]}}));
  await page.route("**/api/v1/transfer/signers",r=>r.fulfill({json:{items:[]}}));
  await page.route("**/api/v1/transfer/approvals*",r=>r.fulfill({json:{items:[]}}));
  await page.route("**/api/v1/events",()=>{});
  await page.route(/\/api\/v1\/tasks(?:\?.*)?$/,r=>r.fulfill({json:{items:[]}}));
  await page.route("**/api/v1/tasks/*",r=>r.fulfill({json:tasks[r.request().url().split("/").at(-1)!]}));
  await page.route("**/api/v1/gateways/system-gateway/task-intents/*",r=>{
    const id=intents[r.request().url().split("/").at(-1)!];return r.fulfill(id?{json:{id}}:{status:404,json:{error:{code:"NOT_FOUND"}}});
  });
  const codes:any={"service.status":23,"service.restart.prepare":24,"service.restart":25,"system.reboot.prepare":26,"system.reboot":27,"system.control.cancel":28};
  await page.route("**/api/v1/gateways/system-gateway/tasks",async r=>{
    const body=r.request().postDataJSON();writes.push(body);const op=codes[body.operation];const task=`system-task-${writes.length}`;intents[body.idempotencyKey]=task;let result:any;
    if(op===23) result={configured:true,configurationState:recovery?"RECOVERY_REQUIRED":"READY",allowRestart:!recovery,allowReboot:reboot&&!recovery,transaction};
    if(op===24||op===26) {
      transaction={id,state:"PREPARED",action:op===24?"service.restart":"system.reboot",unit:"kura.service",issued:false,requestAccepted:false,before:{pid:123,invocationId:"a".repeat(32)},beforeBootId:"33333333-3333-4333-8333-333333333333",expiresAtUnixMs:Date.now()+300000,applicationVerificationConfigured:false,scope:"system-process-only",kuraRecoveryVerified:false,normalBootVerified:false};
      result={...transaction,confirmToken:token,grantSha256:"b".repeat(64),unitDefinitionSha256:"c".repeat(64)};
    }
    if([25,27,28].includes(op)) {
      expect(body.systemControlAction).toEqual({transactionId:id,confirmationToken:token});
      transaction={...transaction,state:op===28?"CANCELLED":unknown?"AWAITING_BOOT":"COMPLETED",issued:op!==28,requestAccepted:op!==28,processRestartVerified:!unknown&&op!==28,hostRebootVerified:!unknown&&op===27,applicationVerified:false,after:{pid:234}};
      result=transaction;
    }
    tasks[task]={id:task,gatewayId:"system-gateway",operation:op,state:unknown&&[25,27].includes(op)?"UNKNOWN":"SUCCEEDED",error:unknown&&[25,27].includes(op)?"OUTCOME_UNKNOWN":null,result};
    if(lostPrepare&&op===24)return r.abort("connectionreset");
    return r.fulfill({status:202,json:{id:task}});
  });
  const select=async()=>page.getByRole("button",{name:"System fixture system-gateway",exact:true}).click();
  await page.goto("/gateways");await select();
  return {writes,tasks,select,panel:page.getByRole("region",{name:/Kura (重启与主机重启|restart and host reboot)/}),complete(){const task=Object.values(tasks).find(v=>[25,27].includes(v.operation))!;transaction={...transaction,state:"COMPLETED",processRestartVerified:true,hostRebootVerified:task.operation===27};task.state="SUCCEEDED";task.error=null;task.result=transaction;}};
}
test("fixed Kura restart requires review, works while Kura is unavailable and separates process from business",async({page})=>{
  const f=await open(page);await f.panel.getByRole("button",{name:"读取服务及原任务状态"}).click();await f.panel.getByRole("button",{name:"准备重启 Kura"}).click();
  await expect(f.panel).toContainText("kura.service (PID 123)");await expect(f.panel.getByRole("button",{name:"确认重启 Kura",exact:true})).toBeDisabled();
  await page.getByRole("dialog",{name:"System fixture",exact:true}).getByRole("combobox",{name:"语言 / Language"}).selectOption("en-US");
  await expect(f.panel).toContainText("System process verification only");
  mkdirSync(evidence,{recursive:true});await page.screenshot({path:resolve(evidence,"restart-review-en.png"),animations:"disabled"});
  await f.panel.getByRole("checkbox").check();await f.panel.getByRole("button",{name:"Confirm Kura restart",exact:true}).click();
  await expect(f.panel).toContainText("Kura process transition: Verified");await expect(f.panel).toContainText("Selected application checks: Unproven");
  expect(f.writes.filter(v=>v.operation==="service.restart")).toHaveLength(1);
});
test("lost prepare acknowledgement restores the exact task by intent without storing a token or repeating POST",async({page})=>{
  const f=await open(page,{lostPrepare:true});await f.panel.getByRole("button",{name:"读取服务及原任务状态"}).click();await f.panel.getByRole("button",{name:"准备重启 Kura"}).click();
  await expect(f.panel).toContainText("不会自动重发");
  expect(await page.evaluate(()=>JSON.stringify(sessionStorage))).not.toContain(token);
  await page.reload();await f.select();await f.panel.getByRole("button",{name:"读取服务及原任务状态"}).click();
  await expect(f.panel).toContainText("kura.service (PID 123)");await f.panel.getByRole("checkbox").check();await f.panel.getByRole("button",{name:"确认重启 Kura",exact:true}).click();
  await expect(f.panel).toContainText("Kura 进程切换: 已证实");
  expect(f.writes.filter(v=>v.operation==="service.restart.prepare")).toHaveLength(1);expect(f.writes.filter(v=>v.operation==="service.restart")).toHaveLength(1);
});
test("host reboot keeps UNKNOWN across reload and only reads the original task until observed",async({page})=>{
  const f=await open(page,{unknown:true});await f.panel.getByRole("button",{name:"读取服务及原任务状态"}).click();await f.panel.getByRole("button",{name:"准备重启主机"}).click();
  await expect(f.panel.getByRole("button",{name:"确认重启主机",exact:true})).toBeDisabled();await f.panel.getByRole("checkbox").check();await f.panel.getByRole("button",{name:"确认重启主机",exact:true}).click();await expect(f.panel).toContainText("不会自动重发");
  await page.reload();await f.select();await f.panel.getByRole("button",{name:"读取服务及原任务状态"}).click();
  await expect(f.panel).toContainText("AWAITING_BOOT");await expect(f.panel.getByRole("button",{name:"准备重启主机"})).toBeDisabled();await expect(f.panel.getByRole("button",{name:"准备重启 Kura"})).toBeDisabled();
  f.complete();await f.panel.getByRole("button",{name:"读取服务及原任务状态"}).click();await expect(f.panel).toContainText("主机启动换代: 已证实");
  expect(f.writes.filter(v=>v.operation==="system.reboot.prepare")).toHaveLength(1);expect(f.writes.filter(v=>v.operation==="system.reboot")).toHaveLength(1);
  expect(await page.evaluate(()=>JSON.stringify(sessionStorage))).not.toContain(token);
});
test("cancel remains available only for an unissued reviewed plan and local grants independently gate reboot",async({page})=>{
  const f=await open(page,{reboot:false});await f.panel.getByRole("button",{name:"读取服务及原任务状态"}).click();await expect(f.panel.getByRole("button",{name:"准备重启主机"})).toBeDisabled();
  await f.panel.getByRole("button",{name:"准备重启 Kura"}).click();await f.panel.getByRole("button",{name:"取消未执行计划"}).click();await expect(f.panel).toContainText("CANCELLED");expect(f.writes.filter(v=>v.operation==="system.control.cancel")).toHaveLength(1);
  mkdirSync(evidence,{recursive:true});await page.screenshot({path:resolve(evidence,"restart-cancelled-zh.png"),animations:"disabled"});
});
test("RECOVERY_REQUIRED remains write gated",async({page})=>{
  const f=await open(page,{recovery:true});await f.panel.getByRole("button",{name:"读取服务及原任务状态"}).click();await expect(f.panel).toContainText("RECOVERY_REQUIRED");
  await expect(f.panel.getByRole("button",{name:"准备重启 Kura"})).toBeDisabled();await expect(f.panel.getByRole("button",{name:"准备重启主机"})).toBeDisabled();expect(f.writes).toHaveLength(1);
});
for(const [role,capable] of [["operator",true],["admin",false]] as const)test(`system control hidden for ${role} capability=${capable}`,async({page})=>{
  const f=await open(page,{role,capable});await expect(f.panel).toHaveCount(0);expect(f.writes).toHaveLength(0);
});
