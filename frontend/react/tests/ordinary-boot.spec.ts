import { expect, test, type Page } from "@playwright/test";
const id="boot-11111111-1111-4111-8111-111111111111";
const token="22222222-2222-4222-8222-222222222222";
async function open(page: Page, role="admin", capable=true, unknown=false) {
  const writes: any[]=[]; const tasks: Record<string,any>={};let transaction:any={state:"UNCONFIGURED"};let policy:any=null;
  await page.route("**/api/v1/session",r=>r.fulfill({json:{id:"boot-user",username:role,tenantId:"test",role,csrf:"test"}}));
  await page.route("**/api/v1/gateways?*",r=>r.fulfill({json:{items:[{id:"boot-gateway",name:"Boot fixture",online:true,enabled:true,kuraAvailable:true,revision:0,capabilities:capable?["ordinary.boot.v1"]:[]}]}}));
  await page.route("**/api/v1/events",()=>{});
  await page.route(/\/api\/v1\/tasks(?:\?.*)?$/,r=>r.fulfill({json:{items:[]}}));
  await page.route("**/api/v1/tasks/*",r=>r.fulfill({json:tasks[r.request().url().split("/").at(-1)!]}));
  await page.route("**/api/v1/gateways/boot-gateway/tasks",async r=>{
    const v=r.request().postDataJSON();writes.push(v);const task="task-"+writes.length;let result:any;
    if(v.operation==="ordinary.boot.status")result={transaction,confirmedPolicy:policy};
    if(v.operation==="ordinary.boot.prepare") {transaction={id,state:"BOOT_POLICY_PREPARED",targetUnit:"ordinary.service",retiredUnits:["old.service"]};result={plan:transaction,confirmToken:token,targetBefore:{unit:"ordinary.service",enablement:"disabled",pid:100},retiredBefore:[{unit:"old.service",enablement:"enabled"}],proposedActions:["disable old.service","enable ordinary.service"],ordinaryGrantSha256:"a".repeat(64),expiresAt:Date.now()+300000};}
    if(v.operation==="ordinary.boot.execute") {expect(v.ordinaryBootAction).toEqual({transactionId:id,confirmationToken:token});transaction={...transaction,state:"BOOT_POLICY_COMMITTED"};policy=transaction;result=transaction;}
    if(v.operation==="ordinary.boot.cancel") {transaction={...transaction,state:"CANCELLED"};result=transaction;}
    const operation=({"ordinary.boot.status":15,"ordinary.boot.prepare":16,"ordinary.boot.execute":17,"ordinary.boot.cancel":18} as any)[v.operation];
    tasks[task]={id:task,gatewayId:"boot-gateway",operation,state:unknown&&operation===17?"UNKNOWN":"SUCCEEDED",error:unknown&&operation===17?"OUTCOME_UNKNOWN":null,result};
    return r.fulfill({status:202,json:{id:task,state:"QUEUED"}});
  });
  await page.goto("/gateways");await page.getByRole("button",{name:"Boot fixture boot-gateway",exact:true}).click();
  return {writes,tasks,panel:page.getByRole("region",{name:/Kura (启动策略|boot policy)/})};
}
test("boot review requires confirmation, survives reload by original task and separates cancelled follow-up from confirmed policy",async({page})=>{
  const f=await open(page);await f.panel.getByRole("button",{name:"读取策略状态"}).click();await f.panel.getByRole("button",{name:"准备并审阅本机策略"}).click();
  await expect(f.panel.getByText("disable old.service")).toBeVisible();await expect(f.panel.getByRole("button",{name:"确认启动策略",exact:true})).toBeDisabled();
  await page.reload();await page.getByRole("button",{name:"Boot fixture boot-gateway",exact:true}).click();await f.panel.getByRole("button",{name:"读取策略状态"}).click();
  await expect(f.panel.getByText("enable ordinary.service")).toBeVisible();expect(f.writes.filter(v=>v.operation==="ordinary.boot.prepare")).toHaveLength(1);
  await page.getByRole("dialog",{name:"Boot fixture",exact:true}).getByRole("combobox",{name:"语言 / Language"}).selectOption("en-US");
  await expect(f.panel).toContainText("without starting, stopping or restarting Kura");await f.panel.getByRole("checkbox").check();await f.panel.getByRole("button",{name:"Confirm boot policy",exact:true}).click();
  await expect(f.panel).toContainText("Confirmed policy: BOOT_POLICY_COMMITTED");await f.panel.getByRole("button",{name:"Prepare and review local policy"}).click();await f.panel.getByRole("button",{name:"Cancel unissued plan"}).click();
  await expect(f.panel).toContainText("Latest transaction: CANCELLED");await expect(f.panel).toContainText("Confirmed policy: BOOT_POLICY_COMMITTED");
});
test("unknown boot execution never repeats or creates replacement prepare",async({page})=>{
  const f=await open(page,"admin",true,true);await f.panel.getByRole("button",{name:"读取策略状态"}).click();await f.panel.getByRole("button",{name:"准备并审阅本机策略"}).click();await f.panel.getByRole("checkbox").check();await f.panel.getByRole("button",{name:"确认启动策略",exact:true}).click();
  await expect(f.panel).toContainText("不会自动重发");await expect(f.panel.getByRole("button",{name:"确认启动策略",exact:true})).toBeDisabled();await expect(f.panel.getByRole("button",{name:"准备并审阅本机策略"})).toBeDisabled();expect(f.writes.filter(v=>v.operation==="ordinary.boot.execute")).toHaveLength(1);
});
for(const [role,capable] of [["operator",true],["admin",false]] as const)test(`boot policy hidden for ${role} capability=${capable}`,async({page})=>{const f=await open(page,role,capable);await expect(f.panel).toHaveCount(0);expect(f.writes).toHaveLength(0);});

test("an uncertain original boot task is only read until its durable result arrives", async ({ page }) => {
  const f=await open(page,"admin",true,true);
  await f.panel.getByRole("button",{name:"读取策略状态"}).click();
  await f.panel.getByRole("button",{name:"准备并审阅本机策略"}).click();
  await f.panel.getByRole("checkbox").check();
  await f.panel.getByRole("button",{name:"确认启动策略",exact:true}).click();
  await expect(f.panel).toContainText("不会自动重发");
  await expect(f.panel.getByRole("button",{name:"准备并审阅本机策略"})).toBeDisabled();
  const original=Object.values(f.tasks).find((v:any)=>v.operation===17)!;
  original.state="SUCCEEDED";original.error=null;
  await expect(f.panel).toContainText("已确认策略: BOOT_POLICY_COMMITTED");
  expect(f.writes.filter(v=>v.operation==="ordinary.boot.execute")).toHaveLength(1);
  expect(f.writes.filter(v=>v.operation==="ordinary.boot.prepare")).toHaveLength(1);
});
