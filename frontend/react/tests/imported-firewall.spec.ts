import {test,expect,type Page} from "@playwright/test";
import {randomUUID} from "node:crypto";
import {mkdirSync} from "node:fs";
import {resolve} from "node:path";
const evidence=process.env.YOFC_IMPORTED_EVIDENCE_DIR||resolve(import.meta.dirname,"../../../target/imported-ui");
mkdirSync(evidence,{recursive:true});
const preparedId="11111111-1111-4111-8111-111111111111",operationId="22222222-2222-4222-8222-222222222222";
const storage="yofc.imported:imported-user:imported-gateway:1";
const candidate={schemaVersion:4,ipv4:{inputPolicy:"accept",forwardPolicy:"drop",outputPolicy:"accept",allowIcmp:false},
  ipv6:{inputPolicy:"accept",forwardPolicy:"drop",outputPolicy:"accept",allowIcmp:false},input:[],portForward:[],nat:[]};
const codes:Record<string,number>={view:38,review:39,prepare:40,apply:41,confirm:42,rollback:43,cancel:44,outcome:45,sourceReview:46,sourceObserve:47,sourceOutcome:48};
type Options={readonly?:boolean;lostApply?:boolean;phase?:string;expired?:boolean;saved?:boolean;missingIntent?:boolean;wrongOriginal?:boolean;lostQueryOnce?:boolean};
async function fixture(page:Page,options:Options={}) {
  const writes:any[]=[],tasks:Record<string,any>={},intents:Record<string,string>={};
  let phase=options.phase,proposalId=operationId,preparation=preparedId;
  let lastAction:string|undefined;
  let connectionVerified=false;
  const writable=!options.readonly;
  const receipt=()=>({id:proposalId,requestId:"remote-value-free-id",state:phase,deadlineUnixMs:Date.now()+(options.expired?-1000:120000),
    deadlineExpired:!!options.expired,ownerGrant:false,historical:["COMMITTED","ROLLED_BACK","REJECTED","OBSERVED"].includes(phase??"")});
  const wrap=(operation:number,value:any,target:string|null=null)=>({schemaVersion:1,operation,originalTaskId:target,
    originalCommandSha256:target?"a".repeat(64):null,writeEnabled:writable,
    newManagementConnectionVerified:connectionVerified,confirmationReady:writable&&connectionVerified&&value.state==="WAIT_CONFIRM"&&!options.expired,receipt:value,
    decisionEvidence:[45,48].includes(operation)?{recordSha256:"b".repeat(64),historical:["COMMITTED","ROLLED_BACK","REJECTED","OBSERVED"].includes(value.state),
      sourceObservation:value.state==="OBSERVED",forwardIntent:["WAIT_CONFIRM","COMMITTED","RECOVERY_REQUIRED"].includes(value.state),
      forwardAcknowledged:["WAIT_CONFIRM","COMMITTED"].includes(value.state),inverseIntent:false,inverseAcknowledged:false,
      negativeAcknowledged:value.state==="REJECTED",confirmedDecision:value.state==="COMMITTED"}:null});
  const task=(id:string,operation:number,result:any,state="SUCCEEDED")=>({id,gatewayId:"imported-gateway",operation,state,error:state==="UNKNOWN"?"OUTCOME_UNKNOWN":null,result,createdAt:new Date().toISOString()});
  if(options.saved)await page.addInitScript(({storage,preparedId})=>sessionStorage.setItem(`${storage}:prepare`,preparedId),{storage,preparedId});
  if(options.missingIntent)await page.addInitScript(({storage,key})=>sessionStorage.setItem(`${storage}:write`,JSON.stringify({kind:"prepare",key})),{storage,key:randomUUID()});
  tasks[preparedId]=task(preparedId,40,wrap(40,receipt(),preparedId));
  await page.route("**/api/v1/session",r=>r.fulfill({json:{id:"imported-user",username:"fixture",tenantId:"test",role:"admin",csrf:"imported-fixture"}}));
  await page.route("**/api/v1/gateways?*",r=>r.fulfill({json:{items:[{id:"imported-gateway",name:"Imported fixture",online:true,enabled:true,kuraAvailable:false,revision:0,bindingGeneration:1,
    capabilities:["firewall.imported.original.v1",...(writable?["firewall.imported.write.v1"]:[])]}]}}));
  await page.route("**/api/v1/transfer/signers",r=>r.fulfill({json:{items:[]}}));
  await page.route("**/api/v1/transfer/approvals*",r=>r.fulfill({json:{items:[]}}));
  await page.route("**/api/v1/events",()=>{});
  await page.route(/\/api\/v1\/tasks(?:\?.*)?$/,r=>r.fulfill({json:{items:[]}}));
  await page.route("**/api/v1/tasks/*",r=>r.fulfill({json:tasks[r.request().url().split("/").at(-1)!]}));
  await page.route("**/api/v1/gateways/imported-gateway/task-intents/*",r=>{
    const id=intents[r.request().url().split("/").at(-1)!];return r.fulfill(id?{json:{id}}:{status:404,json:{error:{code:"NOT_FOUND"}}});
  });
  await page.route("**/api/v1/gateways/imported-gateway/tasks",async r=>{
    const body=r.request().postDataJSON();writes.push(body);const request=body.importedFirewall,kind=request.kind,operation=codes[kind],id=randomUUID();
    expect(body.operation).toBe(`firewall.imported.${kind==="sourceReview"?"source.review":kind==="sourceObserve"?"source.observe":kind==="sourceOutcome"?"source.outcome":kind}`);
    expect(JSON.stringify(body)).not.toMatch(/ownerKey|confirmationToken|"uid"|"unit"/);
    intents[body.idempotencyKey]=id;let value:any,target:string|null=null;
    if(kind==="view")value={schemaVersion:1,revision:7,sourceDigest:"b".repeat(64),sourceKind:"nft.imported.confirmed.v1",initialized:true,projectable:true,
      candidate,journalSettled:!phase||["COMMITTED","ROLLED_BACK","REJECTED","OBSERVED"].includes(phase),transaction:phase?receipt():null,
      preservedSource:{families:{ipv4:{runtimePortForwardDestinationPrefixes:[]},ipv6:{runtimePortForwardDestinationPrefixes:[]}}}};
    if(kind==="review")value={revision:7,sourceDigest:"b".repeat(64),cohortDigest:"c".repeat(64),nativePlan:{operationId:request.review.operationId},ownerGrant:false};
    if(kind==="prepare"){proposalId=request.proposal.operationId;preparation=id;phase="PREPARED";target=id;value=receipt();}
    if(["apply","confirm","rollback","cancel"].includes(kind)) {
      expect(request.originalTaskId).toBe(preparation);target=preparation;
      phase=kind==="apply"?"WAIT_CONFIRM":kind==="confirm"?"COMMITTED":"ROLLED_BACK";value=receipt();lastAction=id;
    }
    if(kind==="outcome"||kind==="sourceOutcome") {
      connectionVerified=kind==="outcome"&&phase==="WAIT_CONFIRM";
      target=options.wrongOriginal?randomUUID():request.originalTaskId;value=receipt();
      if(lastAction&&tasks[lastAction]?.state==="UNKNOWN")tasks[lastAction]=task(lastAction,41,wrap(41,value,preparation));
    }
    if(kind==="sourceReview")value={revision:7,sourceDigest:"b".repeat(64),cohortDigest:"c".repeat(64),kind:"SOURCE_OBSERVATION",kernelEffect:false,ownerGrant:false,
      nativePlan:{operationId:request.review.operationId,observedNativeSha256:"d".repeat(64)}};
    if(kind==="sourceObserve"){proposalId=request.observation.operationId;phase="OBSERVED";target=id;value={...receipt(),kind:"SOURCE_OBSERVATION",kernelEffect:false};}
    const lostQuery=kind==="outcome"&&options.lostQueryOnce; if(lostQuery)options.lostQueryOnce=false;
    tasks[id]=task(id,operation,wrap(operation,value,target),kind==="apply"&&options.lostApply||lostQuery?"UNKNOWN":"SUCCEEDED");
    return r.fulfill({status:202,json:{id}});
  });
  await page.goto("/gateways");await page.getByRole("button",{name:"Imported fixture imported-gateway",exact:true}).click();
  const panel=page.getByRole("region",{name:/^(已接管防火墙|Imported firewall)$/});
  return {writes,panel,read:()=>panel.getByRole("button",{name:/^(读取确认值|Read confirmed values)/}).click(),
    query:()=>panel.getByRole("button",{name:/^(查询原操作结果|Query original operation)$/}).click()};
}
async function prepare(page:Page,f:Awaited<ReturnType<typeof fixture>>) {
  await f.read();await f.panel.getByRole("button",{name:/^(审阅规则提案|Review policy proposal)$/}).click();
  await f.panel.getByRole("checkbox",{name:/^(已核对规则|I reviewed the rules)/}).check();
  await f.panel.getByRole("button",{name:/^(准备已审阅提案|Prepare reviewed proposal)$/}).click();
  await expect(f.panel.locator("p").filter({hasText:/^(当前事务|Current transaction):/})).toContainText(/已准备|准备|Prepared/);
  await f.panel.getByRole("checkbox",{name:/^(已核对规则|I reviewed the rules)/}).check();
}
for(const language of ["zh-CN","en-US"] as const)test(`form review through explicit confirmation has no browser capabilities: ${language}`,async({page})=>{
  const f=await fixture(page);
  if(language==="en-US")await page.getByRole("dialog",{name:"Imported fixture",exact:true}).getByRole("combobox",{name:"语言 / Language"}).selectOption(language);
  await prepare(page,f);
  await f.panel.getByRole("button",{name:/^(临时应用原提案|Apply original proposal temporarily)$/}).click();
  await expect(f.panel.getByRole("timer")).toHaveCount(0);
  await expect(f.panel.getByRole("button",{name:/^(确认保留规则|Confirm policy)$/})).toBeDisabled();
  await f.query();await expect(f.panel.getByRole("timer")).toBeVisible();
  await page.screenshot({path:resolve(evidence,`waiting-${language}.png`),animations:"disabled"});
  await f.panel.getByRole("checkbox",{name:/^(已核对规则|I reviewed the rules)/}).check();
  await f.panel.getByRole("button",{name:/^(确认保留规则|Confirm policy)$/}).click();
  await expect(f.panel.getByRole("timer")).toHaveCount(0);
  expect(f.writes.map(v=>v.importedFirewall.kind)).toEqual(["view","review","prepare","apply","outcome","confirm"]);
  expect(await page.evaluate(()=>JSON.stringify(sessionStorage))).not.toMatch(/ownerKey|confirmationToken|candidate|cohortDigest/);
});
test("UNKNOWN apply is queried by its original task and is never dispatched again",async({page})=>{
  const f=await fixture(page,{lostApply:true});await prepare(page,f);
  await f.panel.getByRole("button",{name:"临时应用原提案",exact:true}).click();
  await expect(f.panel).toContainText("原任务尚未核定");await expect(f.panel.getByRole("timer")).toHaveCount(0);
  await page.reload();await page.getByRole("button",{name:"Imported fixture imported-gateway",exact:true}).click();
  await f.query();await expect(f.panel.getByRole("timer")).toBeVisible();
  expect(f.writes.filter(v=>v.importedFirewall.kind==="apply")).toHaveLength(1);
  expect(f.writes.at(-1).importedFirewall.kind).toBe("outcome");
});
test("lost admission and NOT_FOUND preserve correlation without replacement",async({page})=>{
  const f=await fixture(page,{missingIntent:true});await f.read();await f.read();await f.query();
  expect(f.writes).toEqual([]);await expect(f.panel).toContainText("原任务尚未核定");
  expect(await page.evaluate(storage=>sessionStorage.getItem(`${storage}:write`),storage)).toBeTruthy();
});
for(const phase of ["COMMITTED","ROLLED_BACK","REJECTED","RECOVERY_REQUIRED"])test(`terminal/recovery query stops countdown: ${phase}`,async({page})=>{
  const f=await fixture(page,{saved:true,phase});await f.query();
  await expect(f.panel.getByRole("timer")).toHaveCount(0);
  await expect(f.panel.getByRole("button",{name:"确认保留规则",exact:true})).toBeDisabled();
  expect(f.writes.map(v=>v.importedFirewall.kind)).toEqual(["outcome"]);
});
test("write-disabled and JVM-offline original query remains available",async({page})=>{
  const f=await fixture(page,{readonly:true,saved:true,phase:"WAIT_CONFIRM"});await f.query();
  await expect(f.panel.getByRole("timer")).toHaveCount(0);
  await expect(f.panel.getByRole("button",{name:"确认保留规则",exact:true})).toBeDisabled();
  expect(f.writes.map(v=>v.importedFirewall.kind)).toEqual(["outcome"]);
});
test("expired WAIT_CONFIRM has no actionable countdown",async({page})=>{
  const f=await fixture(page,{saved:true,phase:"WAIT_CONFIRM",expired:true});await f.query();
  await expect(f.panel.getByRole("timer")).toHaveCount(0);await expect(f.panel).toContainText("当前不能确认");
});
test("countdown ends locally when its deadline passes without a replacement action",async({page})=>{
  await page.clock.install();const f=await fixture(page,{saved:true,phase:"WAIT_CONFIRM"});await f.query();
  await expect(f.panel.getByRole("timer")).toBeVisible();await page.clock.fastForward(121000);
  await expect(f.panel.getByRole("timer")).toHaveCount(0);await expect(f.panel.getByRole("button",{name:"确认保留规则",exact:true})).toBeDisabled();
  expect(f.writes.map(v=>v.importedFirewall.kind)).toEqual(["outcome"]);
});
test("a result for another original ID is refused without enabling confirmation",async({page})=>{
  const f=await fixture(page,{saved:true,phase:"WAIT_CONFIRM",wrongOriginal:true});await f.query();
  await expect(f.panel.getByRole("timer")).toHaveCount(0);await expect(f.panel.getByRole("button",{name:"确认保留规则",exact:true})).toBeDisabled();
});
test("an uncertain query stops countdown and a later explicit lookup still names the same original",async({page})=>{
  const f=await fixture(page,{saved:true,phase:"WAIT_CONFIRM",lostQueryOnce:true});await f.query();
  await expect(f.panel.getByRole("timer")).toHaveCount(0);await f.query();await expect(f.panel.getByRole("timer")).toBeVisible();
  expect(f.writes.map(v=>v.importedFirewall.kind)).toEqual(["outcome","outcome"]);
  expect(f.writes.map(v=>v.importedFirewall.originalTaskId)).toEqual([preparedId,preparedId]);
});
test("source observation has its own review and outcome without confirmation",async({page})=>{
  const f=await fixture(page);await f.read();await f.panel.getByRole("button",{name:"审阅来源变化",exact:true}).click();
  await f.panel.getByRole("checkbox",{name:/^已核对规则/}).check();
  await f.panel.getByRole("button",{name:"记录已审阅来源",exact:true}).click();
  await expect(f.panel.getByRole("timer")).toHaveCount(0);
  await expect(f.panel.getByRole("button",{name:"记录已审阅来源",exact:true})).toBeDisabled();
  await f.panel.getByRole("button",{name:"查询原来源观察",exact:true}).click();
  expect(f.writes.map(v=>v.importedFirewall.kind)).toEqual(["view","sourceReview","sourceObserve","sourceOutcome"]);
});
