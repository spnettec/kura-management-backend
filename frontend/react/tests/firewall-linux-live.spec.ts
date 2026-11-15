import { test,expect } from "@playwright/test";
import { readFileSync,openSync,closeSync,writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { execFileSync } from "node:child_process";
test("real Linux remote firewall: reviewed apply/new authenticated connection/confirm, expiry and lost browser acknowledgement rollback",async({page})=>{
 test.setTimeout(150000);page.setDefaultTimeout(15000);
 const path=process.env.YOFC_FIREWALL_LINUX_STACK;if(!path)throw new Error("Explicit fresh Linux stack required");
 const out=resolve(path,"..");const stack=JSON.parse(readFileSync(path,"utf8"));const linux=JSON.parse(readFileSync(resolve(out,"linux.json"),"utf8"));
 closeSync(openSync(resolve(out,"browser-workflow-v2.once"),"wx",0o600));
 const posts:any[]=[];const flows:any[]=[];const progress:any={state:"starting",posts,flows};const save=()=>writeFileSync(resolve(out,"browser-progress-v2.json"),JSON.stringify(progress,null,2),{mode:0o600});
 page.on("request",r=>{if(r.method()==="POST"&&r.url().endsWith("/gateways/fw-linux-gateway/tasks")){const b=r.postDataJSON();posts.push({operation:b.operation,key:b.idempotencyKey,at:Date.now()});save();}});
 const epoch=()=>execFileSync("/opt/homebrew/opt/postgresql@17/bin/psql",[stack.databaseUrl,"-At","-c","select connection_epoch from mgmt_gateways where id='fw-linux-gateway'"],{encoding:"utf8"}).trim();
 function probe(label:string,port:number,allowed:boolean){
  const script="import socket,json,sys\ns=socket.socket();s.settimeout(1)\ntry:\n s.connect(('10.249.17.1',int(sys.argv[1])));v=s.recv(128);print(json.dumps({'allowed':bool(v),'reply':v.decode().strip()}))\nexcept OSError:print(json.dumps({'allowed':False}))\nfinally:s.close()";
  const value=JSON.parse(execFileSync("orb",["run","-m","yofc-management-network-20261101","-u","root","ip","netns","exec",linux.namespaces.w,"python3","-c",script,String(port)],{encoding:"utf8"}));flows.push({label,port,expected:allowed,observed:value});save();expect(value.allowed).toBe(allowed);
 }
 function kernel(label:string){const value=execFileSync("orb",["run","-m","yofc-management-network-20261101","-u","root","ip","netns","exec",linux.namespaces.g,"nft","-s","-j","list","ruleset"],{encoding:"utf8"});writeFileSync(resolve(out,`kernel-${label}.json`),value,{mode:0o600});}
 await page.goto("/login");await page.getByLabel("账号",{exact:true}).fill(stack.reviewUser);await page.getByLabel("密码",{exact:true}).fill(readFileSync(stack.passwordFile,"utf8").trim());await page.getByRole("button",{name:/登\s*录/}).click();
 await expect(page.getByRole("heading",{name:"网关总览",exact:true})).toBeVisible();
 const select=async()=>{await page.getByRole("button",{name:"Remote firewall Linux fixture fw-linux-gateway",exact:true}).click();};await select();
 const panel=page.getByRole("region",{name:"防火墙事务",exact:true});const read=async()=>{await panel.getByRole("button",{name:/^读取确认值与原任务/}).click();await expect(panel.getByRole("button",{name:/^读取确认值与原任务/})).toHaveAttribute("aria-busy","false");};
 await read();await panel.getByRole("checkbox",{name:/^显式建立已审阅/}).check();
 for(const family of ["IPv4","IPv6"])await panel.getByRole("group",{name:family,exact:true}).getByRole("combobox",{name:"入站默认策略",exact:true}).selectOption("drop");
 await panel.getByRole("button",{name:"添加规则",exact:true}).click();
 const ports=panel.getByRole("group",{name:"目标端口范围",exact:true});
 const editPort=async(port:number)=>{await ports.getByLabel("起始端口",{exact:true}).fill(String(port));await ports.getByLabel("结束端口",{exact:true}).fill(String(port));};await editPort(19090);
 await panel.getByLabel("入口接口",{exact:true}).fill("mgmt0");await panel.getByLabel("源地址或网段",{exact:true}).fill("10.249.17.2/32");
 const prepare=async(seconds:number)=>{await panel.getByLabel("确认窗口（秒，30–300）",{exact:true}).fill(String(seconds));await panel.getByRole("checkbox",{name:/^已核对本草稿/}).check();await panel.getByRole("button",{name:"临时应用已审阅规则",exact:true}).click();await expect(panel).toContainText("剩余确认秒数",{timeout:25000});};
 probe("baseline-19090",19090,true);probe("baseline-19091",19091,true);kernel("baseline");const originalEpoch=epoch();progress.originalEpoch=originalEpoch;
 await page.screenshot({path:resolve(out,"reviewed-draft-zh.png"),animations:"disabled"});await prepare(180);progress.state="applied-awaiting-new-connection";save();kernel("first-applied");probe("applied-allowed",19090,true);probe("applied-blocked",19091,false);
 await expect.poll(epoch,{timeout:15000}).not.toBe(originalEpoch);progress.newEpoch=epoch();await read();
 await expect(panel.getByRole("checkbox",{name:/^已重新读取并核对新管理连接/})).toBeVisible();await panel.getByRole("checkbox",{name:/^已重新读取并核对新管理连接/}).check();await panel.getByRole("button",{name:"确认保留候选",exact:true}).click();
 await expect(panel).toContainText("规则已确认",{timeout:20000});await expect(panel).not.toContainText("剩余确认秒数");kernel("confirmed");progress.state="confirmed";save();
 await read();await editPort(19091);await prepare(30);kernel("timeout-applied");probe("timeout-temporary-allowed",19091,true);probe("timeout-temporary-blocked",19090,false);progress.state="waiting-expiry";save();
 await expect.poll(()=>{const value=execFileSync("orb",["run","-m","yofc-management-network-20261101","-u","root","cat",linux.fixture+"/state/firewall/transaction.json"],{encoding:"utf8"});return JSON.parse(value).state;},{timeout:45000,intervals:[1000]}).toBe("ROLLED_BACK");
 await read();await expect(panel).toContainText("候选已回退");await expect(panel).not.toContainText("剩余确认秒数");probe("expiry-restored-allowed",19090,true);probe("expiry-restored-blocked",19091,false);kernel("expired-restored");
 await editPort(19091);let drop=true;let lost:any;
 await page.route("**/api/v1/gateways/fw-linux-gateway/tasks",async route=>{const body=route.request().postDataJSON();if(drop&&body.operation==="firewall.prepare"){drop=false;const actual=await route.fetch();lost={status:actual.status(),receipt:await actual.json(),key:body.idempotencyKey};progress.lostBrowserAcknowledgement=lost;save();await route.abort("failed");}else await route.continue();});
 await panel.getByLabel("确认窗口（秒，30–300）",{exact:true}).fill("180");await panel.getByRole("checkbox",{name:/^已核对本草稿/}).check();await panel.getByRole("button",{name:"临时应用已审阅规则",exact:true}).click();await expect(panel).toContainText("不会重发",{timeout:10000});
 await page.reload();await select();await read();await expect(ports.getByLabel("起始端口",{exact:true})).toHaveValue("19091");probe("lost-ack-applied",19091,true);
 await panel.getByRole("button",{name:"显式回退候选",exact:true}).click();await expect(panel).toContainText("候选已回退",{timeout:20000});await expect(panel).not.toContainText("剩余确认秒数");probe("explicit-restored-allowed",19090,true);probe("explicit-restored-blocked",19091,false);kernel("explicit-restored");
 expect(posts.filter(x=>x.operation==="firewall.prepare")).toHaveLength(3);expect(posts.filter(x=>x.operation==="firewall.confirm")).toHaveLength(1);expect(posts.filter(x=>x.operation==="firewall.rollback")).toHaveLength(1);expect(lost.status).toBe(202);
 progress.tasks=await page.evaluate(async()=>await(await fetch("/api/v1/tasks")).json());progress.state="passed";progress.noOriginalReplay=true;progress.actualProductionMtls=true;progress.lossScope="browser acknowledgement only; agent stream loss not established";save();
 await page.screenshot({path:resolve(out,"rolled-back-zh.png"),animations:"disabled"});
});

test("fresh lost browser acknowledgement queries its original Linux task and explicitly rolls back without replaying closed confirm/expiry scenes",async({page})=>{
 test.setTimeout(60000);page.setDefaultTimeout(15000);
 const path=process.env.YOFC_FIREWALL_LINUX_STACK;if(!path)throw new Error("Explicit Linux fixture required");const out=resolve(path,"..");const stack=JSON.parse(readFileSync(path,"utf8"));const linux=JSON.parse(readFileSync(resolve(out,"linux.json"),"utf8"));closeSync(openSync(resolve(out,"browser-loss-v3.once"),"wx",0o600));
 const trace:any={state:"starting",posts:[]};const save=()=>writeFileSync(resolve(out,"browser-loss-v3-progress.json"),JSON.stringify(trace,null,2),{mode:0o600});
 page.on("request",r=>{if(r.method()==="POST"&&r.url().endsWith("/gateways/fw-linux-gateway/tasks")){const b=r.postDataJSON();trace.posts.push({operation:b.operation,key:b.idempotencyKey});save();}});
 const kernel=()=>JSON.parse(execFileSync("orb",["run","-m","yofc-management-network-20261101","-u","root","ip","netns","exec",linux.namespaces.g,"nft","-s","-j","list","ruleset"],{encoding:"utf8"}));
 await page.goto("/login");await page.getByLabel("账号",{exact:true}).fill(stack.reviewUser);await page.getByLabel("密码",{exact:true}).fill(readFileSync(stack.passwordFile,"utf8").trim());await page.getByRole("button",{name:/登\s*录/}).click();await expect(page.getByRole("heading",{name:"网关总览",exact:true})).toBeVisible();
 const failed=JSON.parse(readFileSync(resolve(out,"browser-progress-v2.json"),"utf8")).posts.filter((p:any)=>p.operation==="firewall.prepare").at(-1);
 const previousIntent=await page.evaluate(async key=>{const response=await fetch(`/api/v1/gateways/fw-linux-gateway/task-intents/${key}`);return {status:response.status,body:await response.json()};},failed.key);expect(previousIntent.status).toBe(404);trace.previouslyFailedTlsIntent={key:failed.key,...previousIntent,noResend:true};save();
 const select=()=>page.getByRole("button",{name:"Remote firewall Linux fixture fw-linux-gateway",exact:true}).click();await select();const panel=page.getByRole("region",{name:"防火墙事务",exact:true});const read=async()=>{await panel.getByRole("button",{name:/^读取确认值与原任务/}).click();await expect(panel.getByRole("button",{name:/^读取确认值与原任务/})).toHaveAttribute("aria-busy","false");};await read();
 const ports=panel.getByRole("group",{name:"目标端口范围",exact:true});await expect(ports.getByLabel("起始端口",{exact:true})).toHaveValue("19090");trace.kernelBefore=kernel();await ports.getByLabel("起始端口",{exact:true}).fill("19091");await ports.getByLabel("结束端口",{exact:true}).fill("19091");
 let drop=true;await page.route("**/api/v1/gateways/fw-linux-gateway/tasks",async route=>{const b=route.request().postDataJSON();if(drop&&b.operation==="firewall.prepare"){drop=false;const response=await route.fetch({maxRetries:0});trace.lost={key:b.idempotencyKey,status:response.status(),receipt:await response.json(),sourceCandidate:b.firewallChange.candidate};save();await route.abort("failed");}else await route.continue();});
 await panel.getByLabel("确认窗口（秒，30–300）",{exact:true}).fill("180");await panel.getByRole("checkbox",{name:/^已核对本草稿/}).check();await panel.getByRole("button",{name:"临时应用已审阅规则",exact:true}).click();await expect(panel).toContainText("不会重发",{timeout:10000});
 await page.reload();await select();await read();await expect(ports.getByLabel("起始端口",{exact:true})).toHaveValue("19091");await expect(panel.getByRole("button",{name:"显式回退候选",exact:true})).toBeEnabled();trace.state="original-task-restored";trace.kernelApplied=kernel();save();
 await page.screenshot({path:resolve(out,"lost-ack-restored-zh.png"),animations:"disabled"});await panel.getByRole("button",{name:"显式回退候选",exact:true}).click();await expect(panel).toContainText("候选已回退",{timeout:15000});await expect(panel).not.toContainText("剩余确认秒数");
 trace.kernelAfter=kernel();const clean=(x:any):any=>Array.isArray(x)?x.filter(v=>!v.metainfo).map(clean):x&&typeof x==="object"?Object.fromEntries(Object.entries(x).filter(([k])=>!['handle','index'].includes(k)).map(([k,v])=>[k,clean(v)])):x;expect(clean(trace.kernelAfter)).toEqual(clean(trace.kernelBefore));expect(clean(trace.kernelApplied)).not.toEqual(clean(trace.kernelBefore));
 expect(trace.posts.filter((x:any)=>x.operation==="firewall.prepare")).toHaveLength(1);expect(trace.posts.filter((x:any)=>x.operation==="firewall.rollback")).toHaveLength(1);expect(trace.posts.filter((x:any)=>x.operation==="firewall.confirm")).toHaveLength(0);expect(trace.lost.status).toBe(202);
 trace.originalTask=await page.evaluate(async id=>await(await fetch(`/api/v1/tasks/${id}`)).json(),trace.lost.receipt.id);expect(trace.originalTask.state).toBe("SUCCEEDED");expect(trace.originalTask.result.candidate).toEqual(trace.lost.sourceCandidate);trace.state="passed";trace.noResend=true;trace.lossScope="browser acknowledgement only, production mTLS and nft; agent-stream-loss not asserted";save();await page.screenshot({path:resolve(out,"explicit-rollback-zh.png"),animations:"disabled"});
});
