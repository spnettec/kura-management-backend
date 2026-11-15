import { expect, test, type Page } from "@playwright/test";
const pem = "-----BEGIN CERTIFICATE-----\nAQID\n-----END CERTIFICATE-----";
// SHA256 of DER bytes 01 02 03. Server X.509 parsing is covered by the HTTP contract.
const fingerprint = "039058c6f2c0cb492c533b0a4d14ef77cc0f78abccced5287d84a1a2011cfb81";
async function open(page: Page, mode = "success", role = "admin") {
  let writes = 0; let reads = 0;
  await page.route("**/api/v1/session", route => route.fulfill({json:{id:"admin-id",username:"admin",tenantId:"test",role,csrf:"test"}}));
  await page.route("**/api/v1/gateways?*", route => route.fulfill({json:{items:[]}}));
  await page.route(/\/api\/v1\/tasks(?:\?.*)?$/, route => route.fulfill({json:{items:[]}}));
  await page.route("**/api/v1/events", () => {});
  await page.route("**/api/v1/gateways", async route => {
    writes++; expect(route.request().postDataJSON()).toEqual({id:"new-gateway",name:"网关 Gateway",clientCertificatePem:pem});
    expect(route.request().headers()["x-csrf-token"]).toBe("test");
    if (mode !== "success") return route.abort("failed");
    return route.fulfill({status:201,json:{id:"new-gateway",name:"网关 Gateway",certificateSha256:fingerprint,bindingGeneration:1}});
  });
  await page.route("**/api/v1/gateways/new-gateway", route => {
    reads++;
    if (mode === "absent") return route.fulfill({status:404,json:{error:{code:"NOT_FOUND"}}});
    return route.fulfill({json:{id:"new-gateway",name:"网关 Gateway",certificateSha256:mode === "mismatch" ? "a".repeat(64) : fingerprint,bindingGeneration:1}});
  });
  await page.goto("/gateways");
  return {counts:() => ({writes,reads})};
}
async function prepare(page: Page) {
  await page.getByRole("button",{name:"登记网关",exact:true}).click();
  const dialog=page.getByRole("dialog");
  await dialog.getByLabel("网关 ID",{exact:true}).fill("new-gateway");
  await dialog.getByLabel("显示名称",{exact:true}).fill("网关 Gateway");
  await dialog.getByLabel("客户端叶证书 PEM",{exact:true}).fill(pem);
  await expect(dialog.getByRole("button",{name:"确认登记",exact:true})).toBeDisabled();
  await dialog.getByRole("button",{name:"核对证书指纹"}).click();
  await expect(dialog.getByText(fingerprint,{exact:true})).toBeVisible();
  await dialog.getByRole("checkbox").check();
  return dialog;
}
test("registration requires reviewed material and explicit bilingual confirmation",async({page})=>{
  const f=await open(page);const d=await prepare(page);
  await d.getByLabel("显示名称",{exact:true}).fill("Changed");
  await expect(d.getByRole("button",{name:"确认登记",exact:true})).toBeDisabled();
  await d.getByLabel("显示名称",{exact:true}).fill("网关 Gateway");
  await d.getByRole("button",{name:"核对证书指纹"}).click();await d.getByRole("checkbox").check();
  await d.getByRole("combobox",{name:"语言 / Language"}).selectOption("en-US");
  await expect(d.getByRole("button",{name:"Confirm registration",exact:true})).toBeEnabled();
  await d.getByRole("button",{name:"Confirm registration",exact:true}).click();
  await expect(d.getByText("Registered. This operation does not change gateway ownership or enable remote management.")).toBeVisible();
  expect(f.counts()).toEqual({writes:1,reads:0});
  await d.getByRole("button",{name:"Register another gateway"}).click();await expect(d.getByLabel("Gateway ID",{exact:true})).toHaveValue("");
});
test("unconsumed registration reply requires explicit read and never resends",async({page})=>{
  const f=await open(page,"lost");const d=await prepare(page);await d.getByRole("button",{name:"确认登记",exact:true}).click();
  await expect(d.getByText(/登记结果尚未核定/)).toBeVisible();await expect(d.getByRole("button",{name:"确认登记",exact:true})).toBeDisabled();
  await page.waitForTimeout(200);expect(f.counts()).toEqual({writes:1,reads:0});
  await d.getByRole("button",{name:"读取登记记录"}).click();await expect(d.getByText("已登记。此操作不会改变网关归属或开启远程管理。")).toBeVisible();expect(f.counts()).toEqual({writes:1,reads:1});
});
test("mismatched existing certificate keeps uncertain write blocked",async({page})=>{
  const f=await open(page,"mismatch");const d=await prepare(page);await d.getByRole("button",{name:"确认登记",exact:true}).click();
  await d.getByRole("button",{name:"读取登记记录"}).click();await expect(d.getByRole("button",{name:"确认登记",exact:true})).toBeDisabled();await expect(d.getByLabel("网关 ID",{exact:true})).toBeDisabled();expect(f.counts().writes).toBe(1);
});
test("explicit not-found requires fresh confirmation before manual registration",async({page})=>{
  const f=await open(page,"absent");const d=await prepare(page);await d.getByRole("button",{name:"确认登记",exact:true}).click();await d.getByRole("button",{name:"读取登记记录"}).click();
  await expect(d.getByText(/当前租户未找到该记录/)).toBeVisible();await expect(d.getByRole("checkbox")).not.toBeChecked();await expect(d.getByRole("button",{name:"确认登记",exact:true})).toBeDisabled();expect(f.counts()).toEqual({writes:1,reads:1});
});
test("nonadmin has no registration controls",async({page})=>{await open(page,"success","operator");await expect(page.getByRole("button",{name:"登记网关",exact:true})).toHaveCount(0);});
