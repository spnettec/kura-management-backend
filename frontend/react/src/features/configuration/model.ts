import { ApiError, api, type Session, type Task } from "../../api/client";
import type { components } from "../../api/schema";

export type Configuration = components["schemas"]["KuraConfigurationView"];
export type Property = components["schemas"]["KuraConfigurationProperty"];
export type ConfigurationUpdate = components["schemas"]["KuraConfigurationUpdate"];
const types = new Set(["STRING", "BOOLEAN", "BYTE", "SHORT", "INTEGER", "LONG", "FLOAT", "DOUBLE", "CHAR", "PASSWORD", "UNKNOWN"]);
const object = (v: unknown): v is Record<string, unknown> => !!v && typeof v === "object" && !Array.isArray(v);
const scalar = (v: unknown) => v === null || typeof v === "string" || typeof v === "boolean";
const invalid = () => new ApiError("PROTOCOL_MISMATCH", 0);

export function configuration(value: unknown, pid: string): Configuration {
  if (!object(value) || value.pid !== pid || !["en", "zh"].includes(String(value.locale)) ||
      !Array.isArray(value.properties) || value.properties.length > 512 ||
      [value.name, value.description].some(v => v !== undefined && typeof v !== "string")) throw invalid();
  if ((value.revision !== undefined && (typeof value.revision !== "string" || !/^[0-9a-f]{64}$/.test(value.revision))) ||
      [value.writable, value.updateEnabled].some(v => v !== undefined && typeof v !== "boolean")) throw invalid();
  const keys = new Set<string>();
  for (const field of value.properties) {
    if (!object(field) || typeof field.id !== "string" || keys.has(field.id) || !types.has(String(field.type)) ||
        typeof field.present !== "boolean" || typeof field.redacted !== "boolean" ||
        (field.editable !== undefined && typeof field.editable !== "boolean") ||
        [field.name, field.description].some(v => v !== undefined && typeof v !== "string")) throw invalid();
    keys.add(field.id);
    if (field.redacted) {
      if (Object.hasOwn(field, "value") || !["SECRET", "UNSUPPORTED"].includes(String(field.redaction))) throw invalid();
    } else if (!Object.hasOwn(field, "value") || !(scalar(field.value) ||
        (Array.isArray(field.value) && field.value.length <= 256 && field.value.every(scalar)))) throw invalid();
  }
  return value as Configuration;
}

export function configurationPatch(view: Configuration, selected: Record<string, string>): ConfigurationUpdate {
  if (!view.writable || !view.updateEnabled || !view.revision || !/^[0-9a-f]{64}$/.test(view.revision)) throw invalid();
  const properties: ConfigurationUpdate["properties"] = [];
  for (const [id, text] of Object.entries(selected)) {
    const field = view.properties.find(p => p.id === id);
    if (!field?.editable || field.type === "UNKNOWN" || new TextEncoder().encode(text).length > 16384) throw invalid();
    const array = field.cardinality !== undefined && field.cardinality !== 0;
    const boolean = field.type === "BOOLEAN";
    if (array) {
      let values: unknown;
      try { values = JSON.parse(text); } catch { throw new ApiError("INVALID_ARGUMENT", 0); }
      if (!Array.isArray(values) || values.length > Math.min(256, Math.abs(field.cardinality!)) ||
          values.some(v => typeof v !== (boolean ? "boolean" : "string")) || (field.required && values.length === 0)) {
        throw new ApiError("INVALID_ARGUMENT", 0);
      }
      properties.push({ id, value: boolean ? { booleanArray: { values: values as boolean[] } } : { textArray: { values: values as string[] } } });
    } else if (boolean) {
      if (text !== "true" && text !== "false") throw new ApiError("INVALID_ARGUMENT", 0);
      properties.push({ id, value: { boolean: text === "true" } });
    } else {
      if (field.required && text === "") throw new ApiError("INVALID_ARGUMENT", 0);
      properties.push({ id, value: { text } });
    }
  }
  if (!properties.length || properties.length > 64) throw new ApiError("INVALID_ARGUMENT", 0);
  const patch = { pid: view.pid, expectedRevision: view.revision, properties };
  if (new TextEncoder().encode(JSON.stringify(patch)).length > 12000) throw new ApiError("RESOURCE_EXHAUSTED", 0);
  return patch;
}

export function configurationSubmission(task: Task, gateway: string, pid: string, revision: string): boolean {
  if (task.gatewayId !== gateway) throw invalid();
  if (task.state !== "SUCCEEDED") return false;
  const value: unknown = task.result;
  if (!object(value) || value.pid !== pid || value.submitted !== true || value.previousRevision !== revision) throw invalid();
  return true;
}

export function configurationPids(value: unknown): string[] {
  if (!Array.isArray(value) || value.length > 10000 || value.some(v => typeof v !== "string" || !v || v.length > 512)) throw invalid();
  return [...new Set(value as string[])].sort();
}

export async function readTask(gatewayId: string, user: Session, operation: "kura.configuration.list" | "kura.configuration.get",
  target: components["schemas"]["KuraConfigurationTarget"] | undefined, signal: AbortSignal): Promise<unknown> {
  const accepted = await api<{ id: string }>(`/gateways/${encodeURIComponent(gatewayId)}/tasks`, "POST",
    { operation, idempotencyKey: crypto.randomUUID(), ...(target ? { kuraConfiguration: target } : {}) }, user.csrf, signal);
  const deadline = Date.now() + 35000;
  while (Date.now() < deadline) {
    signal.throwIfAborted();
    const task = await api<Task>(`/tasks/${encodeURIComponent(accepted.id)}`, "GET", undefined, undefined, signal);
    if (task.id !== accepted.id || task.gatewayId !== gatewayId) throw invalid();
    if (!["QUEUED", "RUNNING"].includes(task.state)) {
      if (task.state !== "SUCCEEDED") throw new ApiError(task.error ?? task.state, 0);
      return task.result;
    }
    await new Promise(resolve => setTimeout(resolve, 500));
  }
  throw new ApiError("TASK_PENDING", 0);
}
