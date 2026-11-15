import { ApiError } from "./client";

export async function setupApi<T>(path: string, method = "GET", body?: unknown, csrf?: string): Promise<T> {
  const r = await fetch(`/api/setup${path}`, { method, credentials: "same-origin",
    headers: { ...(body ? { "Content-Type": "application/json" } : {}), ...(csrf ? { "X-Setup-CSRF": csrf } : {}) },
    ...(body ? { body: JSON.stringify(body) } : {}) });
  if (!r.ok) { const e = await r.json().catch(() => ({})); throw new ApiError(e.error?.code ?? "INTERNAL", r.status); }
  return r.json();
}
