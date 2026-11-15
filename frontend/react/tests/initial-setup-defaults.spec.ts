import { test, expect } from '@playwright/test';

// UI defaults only. Public API fixtures do not certify authentication,
// initialization, actual IPv6 availability, or a production database.
for (const [listen,expected] of [
  ['[::]:38443','[::]:38444'],
  ['0.0.0.0:28443','0.0.0.0:28444'],
  ['[::]:65535','[::]:65534'],
]) test('initial setup uses the actual listener family and adjacent port: '+listen,async({page})=>{
  const writes:string[]=[];
  await page.route('**/api/setup/**',async route=>{
    if(route.request().method()!=='GET'){
      writes.push(route.request().method());return route.abort();
    }
    await route.fulfill({json:{configured:false,authenticated:true,csrf:'contract-only',
      installationId:'08271197-47a6-4b3e-a123-853925071244',certificateFingerprint:'a'.repeat(64),
      blockingOperationId:null,listenAddress:listen}});
  });
  await page.goto('/setup');
  await expect(page.getByLabel('Agent channel listen address',{exact:true})).toHaveValue(expected);
  await page.getByLabel('Agent channel listen address',{exact:true}).fill('127.0.0.1:39444');
  await expect(page.getByLabel('Agent channel listen address',{exact:true})).toHaveValue('127.0.0.1:39444');
  expect(writes).toEqual([]);
});
