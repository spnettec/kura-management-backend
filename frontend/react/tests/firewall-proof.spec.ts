import { test,expect } from "@playwright/test";
for(const variant of ["valid","wrong-hash","contradictory-confirm"] as const)test(`firewall proof badge ${variant} never exposes the prepare token`,async({page})=>{
  const id="11111111-1111-4111-8111-111111111111",transaction=`fw-${id}`,token="private-fixture-token-must-not-render";
  const operation=variant==="contradictory-confirm"?32:31;
  const result={id:transaction,state:variant==="contradictory-confirm"?"ROLLED_BACK":"WAIT_CONFIRM",newManagementConnectionVerified:false,confirmationToken:token,
    reconciliation:{source:"firewall.remote.journal.v1",commandId:id,operation,transactionId:transaction,requestSha256:variant==="wrong-hash"?"wrong":"a".repeat(64),journalSha256:"b".repeat(64)}};
  await page.route("**/api/v1/session",r=>r.fulfill({json:{id:"proof-user",username:"fixture",tenantId:"test",role:"admin",csrf:"fixture"}}));
  await page.route("**/api/v1/gateways?*",r=>r.fulfill({json:{items:[]}}));await page.route("**/api/v1/events",()=>{});
  await page.route(/\/api\/v1\/tasks(?:\?.*)?$/,r=>r.fulfill({json:{items:[{id,gatewayId:"proof-gateway",operation,state:"SUCCEEDED",error:null,result,createdAt:new Date().toISOString()}]}}));
  await page.goto("/commands");await page.getByRole("button",{name:operation===31?/临时应用防火墙候选/:/确认防火墙候选/}).click();
  await expect(page.getByText("已按持久防火墙日志核定原命令；待确认不表示最终提交。",{exact:true})).toHaveCount(variant==="valid"?1:0);
  await expect(page.locator("pre.result")).not.toContainText(token);
});
