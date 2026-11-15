import { resolve } from "node:path";
import { mkdirSync } from "node:fs";
const evidence=resolve(import.meta.dirname,"../../../target/remote-firewall-tasks-20261108/browser-approved");mkdirSync(evidence,{recursive:true});
import { test,expect,type Page } from "@playwright/test";
const transactionId="fw-11111111-1111-4111-8111-111111111111",token="a".repeat(32);
const initial={schemaVersion:3,ipv4:{inputPolicy:"accept",forwardPolicy:"drop",allowIcmp:true},ipv6:{inputPolicy:"accept",forwardPolicy:"drop",allowIcmp:false},input:[{family:"ipv4",protocol:"tcp",ports:{first:18888,last:18889},excludedInterface:"wan+",source:"192.0.2.0/24",sourceMac:"02:00:00:00:00:01",sourcePorts:{first:20000,last:20002}}],portForward:[{family:"ipv6",protocol:"udp",inputInterface:"wan0",outputInterface:"lan0",address:"2001:db8::2",inPort:10001,outPort:10002,masquerade:true,sourcePorts:{first:21000,last:21001}}],nat:[{family:"ipv4",inputInterface:"lan0",outputInterface:"wan0",protocol:null,ruleType:null,automatic:true,masquerade:true}]};
async function fixture(page:Page,options:{role?:string;capable?:boolean;lostPrepare?:boolean;unknownConfirm?:boolean;connectionReady?:boolean}={}) {
  const writes:any[]=[];const tasks:Record<string,any>={};const intents:Record<string,string>={};let transaction:any=null;let candidate:any=structuredClone(initial);let submitted:any;let revision=0;
  await page.route("**/api/v1/session",r=>r.fulfill({json:{id:"fw-user",username:"fixture",tenantId:"test",role:options.role??"admin",csrf:"fixture"}}));
  await page.route("**/api/v1/gateways?*",r=>r.fulfill({json:{items:[{id:"fw-gateway",name:"Firewall fixture",online:true,enabled:true,kuraAvailable:false,revision,bindingGeneration:1,capabilities:options.capable===false?[]:["firewall.remote.v1"]}]}}));
  await page.route("**/api/v1/transfer/signers",r=>r.fulfill({json:{items:[]}}));await page.route("**/api/v1/transfer/approvals*",r=>r.fulfill({json:{items:[]}}));await page.route("**/api/v1/events",()=>{});
  await page.route(/\/api\/v1\/tasks(?:\?.*)?$/,r=>r.fulfill({json:{items:[]}}));
  await page.route("**/api/v1/tasks/*",r=>r.fulfill({json:tasks[r.request().url().split("/").at(-1)!]}));
  await page.route("**/api/v1/gateways/fw-gateway/task-intents/*",r=>{const id=intents[r.request().url().split("/").at(-1)!];return r.fulfill(id?{json:{id}}:{status:404,json:{error:{code:"NOT_FOUND"}}});});
  const codes:Record<string,number>={"firewall.view":30,"firewall.prepare":31,"firewall.confirm":32,"firewall.rollback":33};
  await page.route("**/api/v1/gateways/fw-gateway/tasks",async r=>{
    const body=r.request().postDataJSON();writes.push(body);const operation=codes[body.operation];const id=`firewall-task-${writes.length}`;intents[body.idempotencyKey]=id;let result:any;
    if(operation===30)result={schemaVersion:1,revision,sourceDigest:(revision?"b":"a").repeat(64),initialized:true,projectable:true,candidate:structuredClone(candidate),writeEnabled:true,confirmationReady:!!transaction&&options.connectionReady!==false,transaction:transaction?{id:transaction.id,state:transaction.state,revision:transaction.revision,deadlineUnixMs:transaction.deadlineUnixMs,requestId:transaction.requestId}:null};
    if(operation===31){submitted=structuredClone(body.firewallChange.candidate);transaction={id:transactionId,state:"WAIT_CONFIRM",deadlineUnixMs:Date.now()+body.firewallChange.confirmSeconds*1000,revision,requestId:id,newManagementConnectionRequired:true,newManagementConnectionVerified:false,confirmationToken:token,candidate:structuredClone(submitted)};result={...transaction};}
    if(operation===32||operation===33){expect(body.firewallAction).toEqual({transactionId,confirmationToken:token});transaction={...transaction,state:operation===32?"COMMITTED":"ROLLED_BACK",newManagementConnectionVerified:operation===32};if(operation===32&&!options.unknownConfirm){candidate=submitted;revision++;}result={...transaction,candidate:undefined,confirmationToken:undefined};}
    tasks[id]={id,gatewayId:"fw-gateway",operation,state:options.unknownConfirm&&operation===32?"UNKNOWN":"SUCCEEDED",error:options.unknownConfirm&&operation===32?"OUTCOME_UNKNOWN":null,result:options.unknownConfirm&&operation===32?{}:result};
    if(options.lostPrepare&&operation===31)return r.abort("connectionreset");
    return r.fulfill({status:202,json:{id}});
  });
  const select=()=>page.getByRole("button",{name:"Firewall fixture fw-gateway",exact:true}).click();await page.goto("/gateways");await select();
  const panel=page.getByRole("region",{name:/^(防火墙事务|Firewall transactions)$/});
  const read=()=>panel.getByRole("button",{name:/^(读取确认值与原任务|Read confirmed values and original task)/}).click();
  return {writes,tasks,panel,select,read,complete(){const task=Object.values(tasks).find(v=>v.operation===32)!;task.state="SUCCEEDED";task.error=null;transaction={...transaction,state:"COMMITTED",newManagementConnectionVerified:true};task.result={...transaction,candidate:undefined,confirmationToken:undefined};candidate=submitted;revision++;},bump(){revision++;}};
}
async function prepare(f:Awaited<ReturnType<typeof fixture>>) {await f.panel.getByRole("checkbox",{name:/^已核对本草稿/}).check();await f.panel.getByRole("button",{name:"临时应用已审阅规则",exact:true}).click();}
test("frozen structured draft preserves every other category and needs a reread/new connection before confirm",async({page})=>{
  const f=await fixture(page);await f.read();
  const ports=f.panel.getByRole("group",{name:"目标端口范围",exact:true});await ports.getByLabel("起始端口",{exact:true}).fill("18900");await ports.getByLabel("结束端口",{exact:true}).fill("18901");
  await expect(ports.getByLabel("起始端口",{exact:true})).toHaveValue("18900");
  const window=f.panel.getByLabel("确认窗口（秒，30–300）",{exact:true});await window.fill("29");await f.panel.getByRole("checkbox",{name:/^已核对本草稿/}).check();await expect(f.panel.getByRole("button",{name:"临时应用已审阅规则",exact:true})).toBeDisabled();await window.fill("120");
  await page.screenshot({path:resolve(evidence,"draft-zh.png"),animations:"disabled"});
  await prepare(f);const input=f.writes.find(v=>v.operation==="firewall.prepare").firewallChange;
  expect(input.confirmSeconds).toBe(120);expect(input.expectedRevision).toBe(0);expect(input.sourceDigest).toBe("a".repeat(64));expect(input.candidate.portForward).toEqual(initial.portForward);expect(input.candidate.nat).toEqual(initial.nat);expect(input.candidate.input[0].sourcePorts).toEqual(initial.input[0].sourcePorts);
  await expect(f.panel.getByRole("button",{name:"确认保留候选",exact:true})).toBeDisabled();
  await f.read();await f.panel.getByRole("checkbox",{name:/^已重新读取并核对新管理连接/}).check();await f.panel.getByRole("button",{name:"确认保留候选",exact:true}).click();
  await expect(f.panel).toContainText("规则已确认");await expect(f.panel).not.toContainText("剩余确认秒数");await expect(f.panel.getByRole("button",{name:"临时应用已审阅规则",exact:true})).toBeDisabled();
  expect(f.writes.filter(v=>v.operation==="firewall.prepare")).toHaveLength(1);expect(f.writes.filter(v=>v.operation==="firewall.confirm")).toHaveLength(1);
  expect(await page.evaluate(()=>JSON.stringify(sessionStorage))).not.toContain(token);
});
test("lost prepare acknowledgement restores only its original task without caching candidate or token",async({page})=>{
  const f=await fixture(page,{lostPrepare:true});await f.read();const edited=f.panel.getByRole("group",{name:"目标端口范围",exact:true});await edited.getByLabel("起始端口",{exact:true}).fill("18887");await prepare(f);await expect(f.panel).toContainText("不会重发");
  const stored=await page.evaluate(()=>JSON.stringify(sessionStorage));expect(stored).not.toContain(token);expect(stored).not.toContain("192.0.2.0");expect(stored).not.toContain("candidate");
  await page.reload();await f.select();await f.read();await expect(f.panel.getByRole("checkbox",{name:/^已重新读取并核对新管理连接/})).toBeVisible();
  await expect(f.panel.getByRole("group",{name:"目标端口范围",exact:true}).getByLabel("起始端口",{exact:true})).toHaveValue("18887");
  await f.panel.getByRole("button",{name:"显式回退候选",exact:true}).click();await expect(f.panel).toContainText("候选已回退");await expect(f.panel).not.toContainText("剩余确认秒数");expect(f.writes.filter(v=>v.operation==="firewall.prepare")).toHaveLength(1);
});
test("UNKNOWN survives reload and no new write is issued until the exact original outcome is proved",async({page})=>{
  const f=await fixture(page,{unknownConfirm:true});await f.read();await prepare(f);await f.read();await f.panel.getByRole("checkbox",{name:/^已重新读取并核对新管理连接/}).check();await f.panel.getByRole("button",{name:"确认保留候选",exact:true}).click();await expect(f.panel).toContainText("不会重发");
  await page.reload();await f.select();await f.read();await expect(f.panel).toContainText("不会重发");await expect(f.panel.getByRole("button",{name:"临时应用已审阅规则",exact:true})).toBeDisabled();
  f.complete();await f.read();await expect(f.panel).toContainText("规则已确认");expect(f.writes.filter(v=>v.operation==="firewall.confirm")).toHaveLength(1);expect(f.writes.filter(v=>v.operation==="firewall.prepare")).toHaveLength(1);
});
test("new connection readiness remains an authoritative gate and labels switch to English",async({page})=>{
  const f=await fixture(page,{connectionReady:false});await f.read();await prepare(f);await f.read();await expect(f.panel.getByRole("button",{name:"确认保留候选",exact:true})).toBeDisabled();
  await page.getByRole("dialog",{name:"Firewall fixture",exact:true}).getByRole("combobox",{name:"语言 / Language"}).selectOption("en-US");await expect(f.panel).toContainText("Seconds left to confirm");await page.screenshot({path:resolve(evidence,"waiting-en.png"),animations:"disabled"});await f.panel.getByRole("button",{name:"Roll back candidate",exact:true}).click();await expect(f.panel).toContainText("The candidate was rolled back");await expect(f.panel).not.toContainText("Seconds left to confirm");
});
for(const [role,capable] of [["operator",true],["admin",false]] as const)test(`firewall panel hidden for ${role}, capability=${capable}`,async({page})=>{
  const f=await fixture(page,{role,capable});await expect(f.panel).toHaveCount(0);expect(f.writes).toHaveLength(0);
});
