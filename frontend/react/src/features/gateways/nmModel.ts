// Client-side bounds for the joint NM/firewall workflow. The agent coordinator
// remains the authority; these checks only keep obviously invalid input from
// creating center tasks.
export type NmView = {
  schemaVersion: 1;
  backend: string;
  writeEnabled: boolean;
  sourceDigest: string;
  candidateDigest: string | null;
  confirmationReady: boolean;
  transaction: NmTransaction | null;
};
export type NmTransaction = {id: string; state: string; deadlineUnixMs: number | null};
export type NmReceipt = {
  id: string;
  state: string;
  deadlineUnixMs: number;
  requestId?: string;
  newManagementConnectionRequired?: boolean;
  newManagementConnectionVerified?: boolean;
  confirmationToken?: string;
  candidateDigest?: string;
  operationId?: string;
};
export type Pending = { operation: 35 | 36 | 37; key: string; id?: string };
export const operations = { 35: "nm.prepare", 36: "nm.confirm", 37: "nm.rollback" } as const;

const object = (v: unknown): v is Record<string, unknown> => !!v && typeof v === "object" && !Array.isArray(v);
const digest = (v: unknown) => typeof v === "string" && /^[a-f0-9]{64}$/.test(v);
const uuid = (v: unknown) => typeof v === "string" && /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/.test(v);
const hex32 = (v: unknown) => typeof v === "string" && /^[0-9a-f]{32}$/.test(v);
const iface = (v: unknown) => typeof v === "string" && v.length >= 1 && v.length <= 15 && /^[a-zA-Z0-9_.-]+$/.test(v);

const ip = (v: unknown, v6: boolean) =>
  typeof v === "string" &&
  (v6 ? /^[0-9a-fA-F:]+$/.test(v) && v.includes(":") : /^[0-9.]+$/.test(v) && !v.includes(":"));

const family = (v: unknown, v6: boolean): boolean => {
  if (!object(v)) return false;
  if (!["auto", "manual", "disabled", "link-local"].includes(String(v.method))) return false;
  if (!Array.isArray(v.addresses) || v.addresses.length > 16) return false;
  const manual = v.method === "manual";
  if (manual && v.addresses.length === 0) return false;
  if (["disabled", "link-local"].includes(String(v.method)) && (v.addresses.length > 0 || (Array.isArray(v.dns) && v.dns.length > 0) || v.gateway != null)) return false;
  const seen = new Set<string>();
  for (const a of v.addresses) {
    if (!object(a) || !ip(a.address, v6) || !Number.isInteger(Number(a.prefix))) return false;
    const prefix = Number(a.prefix);
    if (prefix < 0 || prefix > (v6 ? 128 : 32)) return false;
    if (seen.has(String(a.address))) return false;
    seen.add(String(a.address));
  }
  if (!Array.isArray(v.dns) || v.dns.length > 8) return false;
  for (const d of v.dns) if (!ip(d, v6)) return false;
  if (v.gateway != null && (!manual || !ip(v.gateway, v6))) return false;
  if (v.wanPriority != null && (!Number.isInteger(Number(v.wanPriority)) || Number(v.wanPriority) < -1 || Number(v.wanPriority) >= 2147483647)) return false;
  return true;
};

export const changesValid = (v: unknown): v is unknown[] => {
  if (!Array.isArray(v) || v.length === 0 || v.length > 16) return false;
  const seen = new Set<string>();
  for (const c of v) {
    if (!object(c) || !iface(c.interface) || seen.has(String(c.interface))) return false;
    seen.add(String(c.interface));
    const any = c.ipv4 != null || c.ipv6 != null || c.ethernet != null;
    if (!any) return false;
    if (c.ipv4 != null && !family(c.ipv4, false)) return false;
    if (c.ipv6 != null && !family(c.ipv6, true)) return false;
    if (c.ethernet != null) {
      const e = c.ethernet;
      if (!object(e) || !Number.isInteger(Number(e.mtu)) || Number(e.mtu) < 0 || Number(e.mtu) > 65535 || ![-1, 0, 1].includes(Number(e.acceptAllMacAddresses))) return false;
    }
  }
  return JSON.stringify(v).length <= 65536;
};
export const natValid = (v: unknown): boolean => {
  if (!object(v) || !iface(v.wan) || !Array.isArray(v.lan) || v.lan.length === 0 || v.lan.length > 16) return false;
  return new Set(v.lan).size === v.lan.length && v.lan.every((l: unknown) => iface(l) && l !== v.wan);
};
const states = ["PREPARED", "CHECKPOINT_INTENT", "CHECKPOINT_CREATED", "PROFILE_INTENT", "PROFILE_APPLIED", "FIREWALL_INTENT", "WAIT_CONFIRM", "CONFIRM_DECIDED", "ROLLBACK_DECIDED", "ROLLBACK_RUNNING", "RECOVERY_REQUIRED", "COMMITTED", "ROLLED_BACK", "CANCELLED"];
const viewTransaction = (v: unknown): v is NmTransaction =>
  object(v) && Object.keys(v).every(k=>["id","state","deadlineUnixMs"].includes(k)) &&
  uuid(v.id) && typeof v.state === "string" && states.includes(v.state) &&
  ((Number.isSafeInteger(v.deadlineUnixMs) && (v.deadlineUnixMs as number)>0) ||
    (v.deadlineUnixMs === null && v.state !== "WAIT_CONFIRM"));
export const validView = (v: unknown): v is NmView =>
  object(v) && Object.keys(v).every(k=>["schemaVersion","backend","writeEnabled","sourceDigest","candidateDigest","confirmationReady","transaction"].includes(k)) &&
  v.schemaVersion === 1 && v.backend === "NetworkManager" && typeof v.writeEnabled === "boolean" &&
  digest(v.sourceDigest) && (v.candidateDigest === null || digest(v.candidateDigest)) &&
  typeof v.confirmationReady === "boolean" && (v.transaction === null || viewTransaction(v.transaction)) &&
  (!v.confirmationReady || (v.writeEnabled && object(v.transaction) && v.transaction.state === "WAIT_CONFIRM"));
export function validReceipt(v: unknown): v is NmReceipt {
  return object(v) && uuid(v.id) && typeof v.state === "string" && states.includes(v.state) &&
    Number.isSafeInteger(v.deadlineUnixMs) && (v.deadlineUnixMs as number) > 0 &&
    (v.newManagementConnectionRequired === undefined || v.newManagementConnectionRequired === true);
}
