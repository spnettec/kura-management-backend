import { test, expect } from "@playwright/test";
for(const invalid of ["none","wrong-command","full-recovery-claim"] as const) test(`system original command proof label ${invalid}`,async({page})=>{
  const id="88888888-8888-4888-8888-888888888888";let posts=0;
  page.on("request",r=>{if(r.method()==="POST")posts++;});
  await page.addInitScript(()=>{class Source extends EventTarget{close(){}}Object.defineProperty(window,"EventSource",{value:Source});});
  await page.route("**/api/v1/session",r=>r.fulfill({json:{id:"system-proof",username:"admin",tenantId:"test",role:"admin",csrf:"fixture"}}));
  await page.route("**/api/v1/gateways?*",r=>r.fulfill({json:{items:[]}}));
  await page.route(/\/api\/v1\/tasks(?:\?.*)?$/,r=>r.fulfill({json:{items:[{id,gatewayId:"fixture",operation:25,state:"SUCCEEDED",error:null,createdAt:"2026-11-06T15:00:00Z",result:{id:"system-11111111-1111-4111-8111-111111111111",state:"COMPLETED",scope:"system-process-only",kuraRecoveryVerified:invalid==="full-recovery-claim",normalBootVerified:false,processRestartVerified:true,hostRebootVerified:false,applicationVerified:false,reconciliation:{source:"system.control.journal.v1",commandId:invalid==="wrong-command"?"other":id,operation:25}}}]}}));
  await page.goto("/commands");await page.getByRole("button",{name:/重启 Kura 服务 88888888/}).click();const dialog=page.getByRole("dialog");
  const label=dialog.getByText("已按持久证据核定原重启任务；进程、主机和业务结果分别列出。",{exact:true});
  if(invalid==="none") {
    await expect(label).toBeVisible();await dialog.getByRole("combobox",{name:"语言 / Language"}).selectOption("en-US");
    await expect(dialog.getByText("The original restart task was reconciled from durable evidence; process, host and application outcomes are listed separately.",{exact:true})).toBeVisible();
    await expect(dialog.locator("pre")).toContainText('"applicationVerified": false');
  } else await expect(label).toHaveCount(0);
  expect(posts).toBe(0);
});
