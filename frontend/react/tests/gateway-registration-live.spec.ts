import { test, expect } from "@playwright/test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { createHash, randomUUID } from "node:crypto";
const report = JSON.parse(readFileSync(process.env.KURA_REGISTRATION_FIXTURE!,"utf8"));
const pem = readFileSync(report.clientCertificateFile || report.pki + "/agent.pem","utf8");
const der=Buffer.from(pem.replace(/-----[^-]+-----/g,"").replace(/\s/g,""),"base64");
const fingerprint=createHash("sha256").update(der).digest("hex");
test("real HTTPS registration commit survives a discarded browser response and explicit reconciliation",async({page})=>{
  const id=report.gatewayId || "registration-"+randomUUID();let writes=0;let committed=false;
  await page.goto("/login");await page.getByLabel("账号",{exact:true}).fill(report.testUser);await page.getByLabel("密码",{exact:true}).fill(readFileSync(report.passwordFile,"utf8"));await page.getByRole("button",{name:/登\s*录/}).click();
  await expect(page.getByRole("heading",{name:"网关总览",exact:true})).toBeVisible();
  await page.getByRole("button",{name:"登记网关",exact:true}).click();const d=page.getByRole("dialog");
  await d.getByLabel("网关 ID",{exact:true}).fill(id);await d.getByLabel("显示名称",{exact:true}).fill(report.displayName || "HTTPS真实登记");await d.getByLabel("客户端叶证书 PEM",{exact:true}).fill(pem);await d.getByRole("button",{name:"核对证书指纹"}).click();await expect(d.getByText(fingerprint,{exact:true})).toBeVisible();await d.getByRole("checkbox").check();
  await page.route("**/api/v1/gateways",async route=>{
    if(route.request().method()!=="POST") return route.continue();
    writes++;const response=await route.fetch();expect(response.status()).toBe(201);const receipt=await response.json();expect(receipt).toMatchObject({id,certificateSha256:fingerprint,bindingGeneration:1});committed=true;await route.abort("failed");
  });
  await d.getByRole("button",{name:"确认登记",exact:true}).click();await expect(d.getByText(/登记结果尚未核定/)).toBeVisible();expect(committed).toBe(true);expect(writes).toBe(1);
  await page.screenshot({path:resolve(report.evidence,(report.evidencePrefix || "host")+"-live-registration-unknown-zh.png"),fullPage:true});
  await d.getByRole("button",{name:"读取登记记录"}).click();await expect(d.getByText("已登记。此操作不会改变网关归属或开启远程管理。")).toBeVisible();expect(writes).toBe(1);
  await page.screenshot({path:resolve(report.evidence,(report.evidencePrefix || "host")+"-live-registration-reconciled-zh.png"),fullPage:true});
  await d.getByRole("combobox",{name:"语言 / Language"}).selectOption("en-US");await expect(d.getByText("Registered. This operation does not change gateway ownership or enable remote management.")).toBeVisible();await page.screenshot({path:resolve(report.evidence,(report.evidencePrefix || "host")+"-live-registration-reconciled-en.png"),fullPage:true});
  console.log(JSON.stringify({id,fingerprint,writes,committed}));
});
