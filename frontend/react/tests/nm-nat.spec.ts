import { test,expect,type Page } from "@playwright/test";
import { randomUUID } from "node:crypto";
import { mkdirSync } from "node:fs";
import { resolve } from "node:path";

const token="a".repeat(32),operationId="11111111-1111-4111-8111-111111111111";
const evidence=process.env.YOFC_NAT_EVIDENCE_DIR||resolve(import.meta.dirname,"../../../target/nm-nat-ui");
mkdirSync(evidence,{recursive:true});
const changes=[{interface:"eth0",ethernet:{mtu:1500,acceptAllMacAddresses:-1}}];

async function fixture(page:Page,lostPrepare=false,mismatchedPrepare=false) {
  const writes:Record<string,any>[]=[];
  const tasks:Record<string,any>={},intents:Record<string,string>={};
  let transaction:any=null;
  await page.route("**/api/v1/session",r=>r.fulfill({json:{id:"nat-user",username:"fixture",tenantId:"test",role:"admin",csrf:"owned-nat-test"}}));
  await page.route("**/api/v1/gateways?*",r=>r.fulfill({json:{items:[{id:"nat-gateway",name:"NAT fixture",online:true,enabled:true,kuraAvailable:false,revision:0,bindingGeneration:1,capabilities:["nm.remote.v1"]}]}}));
  await page.route("**/api/v1/transfer/signers",r=>r.fulfill({json:{items:[]}}));
  await page.route("**/api/v1/transfer/approvals*",r=>r.fulfill({json:{items:[]}}));
  await page.route("**/api/v1/events",()=>{});
  await page.route(/\/api\/v1\/tasks(?:\?.*)?$/,r=>r.fulfill({json:{items:[]}}));
  await page.route("**/api/v1/tasks/*",r=>{
    const task=tasks[r.request().url().split("/").at(-1)!];
    return r.fulfill({json:mismatchedPrepare&&task?.operation===35?{...task,id:randomUUID()}:task});
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
    if(operation===34)result={schemaVersion:1,backend:"NetworkManager",writeEnabled:true,
      sourceDigest:"a".repeat(64),candidateDigest:transaction?"b".repeat(64):null,
      confirmationReady:!!transaction,transaction:transaction?{id:operationId,state:transaction.state,
        deadlineUnixMs:transaction.deadlineUnixMs}:null};
    if(operation===35){transaction={id:operationId,state:"WAIT_CONFIRM",deadlineUnixMs:Date.now()+120000,
      confirmationToken:token,newManagementConnectionRequired:true};result={...transaction};}
    if(operation===36||operation===37){
      expect(body.nmAction).toEqual({operationId,confirmationToken:token});
      transaction={...transaction,state:operation===36?"COMMITTED":"ROLLED_BACK"};
      result={id:operationId,state:transaction.state,deadlineUnixMs:transaction.deadlineUnixMs,
        newManagementConnectionVerified:operation===36};
    }
    tasks[id]={id,gatewayId:"nat-gateway",operation,state:"SUCCEEDED",error:null,result};
    if(operation===35&&lostPrepare)return r.abort("connectionreset");
    return r.fulfill({status:202,json:{id}});
  });
  const select=()=>page.getByRole("button",{name:"NAT fixture nat-gateway",exact:true}).click();
  await page.goto("/gateways");await select();
  const panel=page.getByRole("region",{name:/^(NM 网络事务|NM network transactions)$/});
  const read=()=>panel.getByRole("button",{name:/^(读取 NM 来源与原任务|Read the NM source and original task)/}).click();
  return {writes,panel,read,select,repairTaskIdentity(){mismatchedPrepare=false;}};
}

for(const mode of ["keep","disable","replace"] as const) {
  test(`NM automatic NAT ${mode} is an explicit GUI intention`,async({page})=>{
    const f=await fixture(page);await f.read();
    if(mode==="replace")await page.getByRole("dialog",{name:"NAT fixture",exact:true})
      .getByRole("combobox",{name:"语言 / Language"}).selectOption("en-US");
    const english=mode==="replace";
    const draft=f.panel.getByLabel(english?"Typed proposal (changes JSON array, ≤16 interfaces)":"类型化提案（changes JSON 数组，≤16 接口）",{exact:true});
    await draft.fill(JSON.stringify(changes));
    const chooser=f.panel.getByRole("combobox",{name:english?"Automatic NAT action":"自动 NAT 操作",exact:true});
    await expect(chooser).toHaveValue("keep");await chooser.selectOption(mode);
    if(mode==="replace") {
      await f.panel.getByRole("textbox",{name:"WAN interface",exact:true}).fill("eth0");
      const lan=f.panel.getByRole("combobox",{name:"LAN interfaces",exact:true});
      await lan.fill("eth1");await lan.press("Enter");
    }
    await f.panel.getByRole("checkbox",{name:english?/^I reviewed this proposal/:/^已核对本提案/}).check();
    await page.screenshot({path:resolve(evidence,`nat-${mode}-${english?"en":"zh"}.png`),animations:"disabled"});
    await f.panel.getByRole("button",{name:english?"Apply reviewed proposal temporarily":"临时应用已审阅提案",exact:true}).click();
    await expect(f.panel).toContainText(english?"Seconds left to confirm":"剩余确认秒数");
    const submitted=f.writes.filter(v=>v.operation==="nm.prepare");expect(submitted).toHaveLength(1);
    if(mode==="keep")expect(Object.hasOwn(submitted[0].nmChange,"nat")).toBe(false);
    if(mode==="disable")expect(submitted[0].nmChange.nat).toEqual({schemaVersion:1,action:"disable"});
    if(mode==="replace")expect(submitted[0].nmChange.nat).toEqual({wan:"eth0",lan:["eth1"]});
    expect(await page.evaluate(()=>JSON.stringify(sessionStorage))).not.toContain(token);
  });
}

test("lost NM prepare acknowledgement retains the original lookup after view refresh",async({page})=>{
  const f=await fixture(page,true);await f.read();
  await f.panel.getByLabel("类型化提案（changes JSON 数组，≤16 接口）",{exact:true}).fill(JSON.stringify(changes));
  await f.panel.getByRole("combobox",{name:"自动 NAT 操作",exact:true}).selectOption("disable");
  await f.panel.getByRole("checkbox",{name:/^已核对本提案/}).check();
  await f.panel.getByRole("button",{name:"临时应用已审阅提案",exact:true}).click();
  await expect(f.panel).toContainText("不会重发");
  expect(await page.evaluate(()=>JSON.stringify(sessionStorage))).not.toContain(token);
  await page.reload();await f.select();await f.read();
  await expect(f.panel.getByRole("button",{name:"显式回退提案",exact:true})).toBeEnabled();
  await f.read(); // A second unrelated view must retain the same original lookup.
  await expect(f.panel.getByRole("button",{name:"显式回退提案",exact:true})).toBeEnabled();
  await f.panel.getByRole("button",{name:"显式回退提案",exact:true}).click();
  await expect(f.panel).toContainText("提案已回退");
  expect(f.writes.filter(v=>v.operation==="nm.prepare")).toHaveLength(1);
  expect(f.writes.filter(v=>v.operation==="nm.rollback")).toHaveLength(1);
});

test("a receipt from a different task ID never replaces the original NM lookup",async({page})=>{
  const f=await fixture(page,false,true);await f.read();
  await f.panel.getByLabel("类型化提案（changes JSON 数组，≤16 接口）",{exact:true}).fill(JSON.stringify(changes));
  await f.panel.getByRole("checkbox",{name:/^已核对本提案/}).check();
  await f.panel.getByRole("button",{name:"临时应用已审阅提案",exact:true}).click();
  await expect(f.panel).toContainText("不会重发");
  await expect(f.panel.getByRole("button",{name:"显式回退提案",exact:true})).toBeDisabled();
  expect(f.writes.filter(v=>v.operation==="nm.prepare")).toHaveLength(1);
  f.repairTaskIdentity();await f.read();
  await expect(f.panel.getByRole("button",{name:"显式回退提案",exact:true})).toBeEnabled();
  expect(f.writes.filter(v=>v.operation==="nm.prepare")).toHaveLength(1);
  expect(await page.evaluate(()=>JSON.stringify(sessionStorage))).not.toContain(token);
});
