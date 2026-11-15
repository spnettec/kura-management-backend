import type { components } from "./schema";
export type Gateway = components["schemas"]["Gateway"];
export type Session = components["schemas"]["Session"];
export type Task = components["schemas"]["Task"];
export type TransferSigner = components["schemas"]["TransferSigner"];
export type TransferApproval = components["schemas"]["TransferApproval"];
export type TransferIssuance = components["schemas"]["TransferIssuance"];
export class ApiError extends Error {
  constructor(
    public code: string,
    public status: number,
  ) {
    super(code);
  }
}
export async function api<T>(
  path: string,
  method = "GET",
  body?: unknown,
  csrf?: string,
  signal?: AbortSignal,
): Promise<T> {
  const response = await fetch(`/api/v1${path}`, {
    method,
    credentials: "same-origin",
    signal,
    headers: {
      ...(body ? { "Content-Type": "application/json" } : {}),
      ...(csrf ? { "X-CSRF-Token": csrf } : {}),
    },
    ...(body ? { body: JSON.stringify(body) } : {}),
  });
  if (!response.ok) {
    const error = await response.json().catch(() => ({}));
    throw new ApiError(error.error?.code ?? "INTERNAL", response.status);
  }
  return response.status === 204 ? (undefined as T) : response.json();
}

// Binary download for staged artifacts (identity bundle, task key backup).
// JSON bodies only; the response is never interpreted as JSON.
export async function apiDownload(
  path: string,
  method: "GET" | "POST" = "GET",
  body?: unknown,
  csrf?: string,
): Promise<Blob> {
  const response = await fetch(`/api/v1${path}`, {
    method,
    credentials: "same-origin",
    headers: {
      ...(body ? { "Content-Type": "application/json" } : {}),
      ...(csrf ? { "X-CSRF-Token": csrf } : {}),
    },
    ...(body ? { body: JSON.stringify(body) } : {}),
  });
  if (!response.ok) {
    const error = await response.json().catch(() => ({}));
    throw new ApiError(error.error?.code ?? "INTERNAL", response.status);
  }
  return response.blob();
}

export function saveBlob(blob: Blob, filename: string): void {
  const url = URL.createObjectURL(blob);
  const anchor = document.createElement("a");
  anchor.href = url;
  anchor.download = filename;
  anchor.click();
  URL.revokeObjectURL(url);
}

// SHA-256 fingerprint of a PEM certificate body, matching the console.
export async function certificateFingerprint(pem: string): Promise<string | null> {
  const match = pem.trim().match(/^-----BEGIN CERTIFICATE-----([A-Za-z0-9+/=\s]+)-----END CERTIFICATE-----$/);
  if (!match) return null;
  const encoded = match[1].replace(/\s/g, "");
  const raw = atob(encoded);
  if (!raw.length || btoa(raw) !== encoded) return null;
  const hash = await crypto.subtle.digest("SHA-256", Uint8Array.from(raw, (c) => c.charCodeAt(0)));
  return Array.from(new Uint8Array(hash), (b) => b.toString(16).padStart(2, "0")).join("");
}
