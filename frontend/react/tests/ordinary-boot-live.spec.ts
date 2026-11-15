import { expect, test } from "@playwright/test";
import { readFileSync, writeFileSync } from "node:fs";
import { execFileSync } from "node:child_process";
import { resolve } from "node:path";
const b=process.env.KURA_BOOT_EVIDENCE_ROOT ?? resolve(import.meta.dirname,"../../../target/center-ordinary-boot-20261106");
const host=JSON.parse(readFileSync(`${b}/host-stack.json`,"utf8"));
const orb=JSON.parse(readFileSync(`${b}/orb.json`,"utf8"));
test("actual center boot review, confirmation, cancelled follow-up and immutable replay",async({page})=>{
  test.setTimeout(180000);const requests:any[]=[];const accepted:Record<string,string>={};
  page.on("request",r=>{if(r.method()==="POST"&&r.url().endsWith(`/gateways/${orb.gateway}/tasks`))requests.push(r.postDataJSON());});
  page.on("response",async r=>{if(r.status()===202&&r.url().endsWith(`/gateways/${orb.gateway}/tasks`)){const input=r.request().postDataJSON();accepted[input.idempotencyKey]=(await r.json()).id;}});
  await page.goto("/login");await page.getByLabel("账号",{exact:true}).fill(host.testAdmin);await page.getByLabel("密码",{exact:true}).fill(readFileSync(`${host.fixture}/${host.testAdmin}.password`,"utf8"));await page.getByRole("button",{name:/登\s*录/}).click();
  const gateway=page.getByRole("button",{name:`本轮真实 Kura 启动策略 ${orb.gateway}`,exact:true});await expect(gateway).toBeVisible({timeout:30000});await gateway.click();
  const resume=process.env.KURA_BOOT_RESUME_PREPARE_TASK;
  if(resume) { const user=await (await page.request.get('/api/v1/session')).json(); await page.evaluate(({key,id})=>sessionStorage.setItem(key,id),{key:`yofc.boot.prepare:${user.id}:${orb.gateway}`,id:resume}); }
  const panel=page.getByRole("region",{name:/Kura (启动策略|boot policy)/});await expect(panel).toBeVisible();await panel.getByRole("button",{name:"读取策略状态"}).click();if(resume) { await expect(panel.getByRole("button",{name:"取消未执行计划"})).toBeVisible({timeout:15000}); await panel.getByRole("button",{name:"取消未执行计划"}).click(); await expect(panel).toContainText("CANCELLED",{timeout:15000}); } else { await expect(panel).toContainText("UNCONFIGURED"); }
  await panel.getByRole("button",{name:"准备并审阅本机策略"}).click();await expect(panel.getByText(`enable ${orb.ordinaryUnit}`,{exact:true})).toBeVisible({timeout:30000});await expect(panel.getByRole("button",{name:"确认启动策略",exact:true})).toBeDisabled();
  const task=await panel.locator("p").filter({hasText:"操作任务"}).locator("code").innerText();const original=await (await page.request.get(`/api/v1/tasks/${task}`)).json();expect(original.result.confirmToken).toBeTruthy();execFileSync("/opt/homebrew/bin/python3",[`${b}/check-token-role.py`,task]);
  await page.getByRole("dialog",{name:"本轮真实 Kura 启动策略",exact:true}).getByRole("combobox",{name:"语言 / Language"}).selectOption("en-US");await panel.getByRole("checkbox").check();await panel.getByRole("button",{name:"Confirm boot policy",exact:true}).click();await expect(panel).toContainText("Confirmed policy: BOOT_POLICY_COMMITTED",{timeout:30000});
  const execute=requests.find(r=>r.operation==="ordinary.boot.execute");expect(execute).toBeTruthy();
  await panel.getByRole("button",{name:"Prepare and review local policy"}).click();await expect(panel.getByRole("button",{name:"Cancel unissued plan"})).toBeVisible({timeout:30000});await panel.getByRole("button",{name:"Cancel unissued plan"}).click();await expect(panel).toContainText("Latest transaction: CANCELLED",{timeout:30000});await expect(panel).toContainText("Confirmed policy: BOOT_POLICY_COMMITTED");
  const session=await (await page.request.get('/api/v1/session')).json();const replay=await page.request.post(`/api/v1/gateways/${orb.gateway}/tasks`,{headers:{'X-CSRF-Token':session.csrf,Origin:host.http},data:execute});expect(replay.status()).toBe(202);expect((await replay.json()).id).toBe(accepted[execute.idempotencyKey]);
  await page.reload();await page.getByRole("button",{name:`本轮真实 Kura 启动策略 ${orb.gateway}`,exact:true}).click();await panel.getByRole("button",{name:"Read policy status"}).click();await expect(panel).toContainText("Latest transaction: CANCELLED",{timeout:30000});await expect(panel).toContainText("Confirmed policy: BOOT_POLICY_COMMITTED");
  await page.getByRole("dialog",{name:"本轮真实 Kura 启动策略",exact:true}).getByRole("combobox",{name:"语言 / Language"}).selectOption("zh-CN");
  await page.screenshot({path:`${b}/center-policy-completed.png`,fullPage:true});
  writeFileSync(`${b}/center-browser-live.json`,JSON.stringify({status:'passed',gateway:orb.gateway,actualHttpsGrpcAgent:true,executeTaskId:accepted[execute.idempotencyKey],prepareTaskId:task,executeTaskAcceptedOnce:true,browserExecuteRequests:requests.filter(r=>r.operation==='ordinary.boot.execute').length,originalIdempotencyReplay:true,confirmedPolicySurvivedFollowupCancellation:true,reloadReadOnly:true,scope:'native boot policy only; not Kura reboot/business recovery certification'},null,2));
});
