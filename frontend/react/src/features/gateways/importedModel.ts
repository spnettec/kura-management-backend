import {candidateValid,type Candidate} from "./firewallModel";
export const importedOperations={view:38,review:39,prepare:40,apply:41,confirm:42,rollback:43,cancel:44,outcome:45,sourceReview:46,sourceObserve:47,sourceOutcome:48} as const;
export type ImportedKind=keyof typeof importedOperations;
export type Correlation={kind:ImportedKind;key:string;id?:string;originalTaskId?:string;operationId?:string};
export type ImportedResult={schemaVersion:1;operation:number;originalTaskId:string|null;originalCommandSha256:string|null;
  writeEnabled:boolean;confirmationReady:boolean;newManagementConnectionVerified:boolean;receipt:Record<string,unknown>;decisionEvidence:Record<string,unknown>|null};
export type ImportedView={revision:number;sourceDigest:string;sourceKind:"nft.imported.confirmed.v1";initialized:true;
  projectable:boolean;candidate:Candidate|null;journalSettled:boolean;transaction:Record<string,unknown>|null;
  preservedSource:{families:Record<string,{runtimePortForwardDestinationPrefixes:number[]}>}};
export const object=(v:unknown):v is Record<string,unknown>=>!!v&&typeof v==="object"&&!Array.isArray(v);
export const uuid=(v:unknown):v is string=>typeof v==="string"&&/^[0-9a-f]{8}-[0-9a-f]{4}-[1-8][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/.test(v);
export const digest=(v:unknown):v is string=>typeof v==="string"&&/^[0-9a-f]{64}$/.test(v);
export const importedTerminal=new Set(["COMMITTED","ROLLED_BACK","REJECTED","OBSERVED"]);
const states=new Set(["PREPARED","APPLYING","WAIT_CONFIRM","CONFIRMING","COMMITTED","ROLLING_BACK","ROLLED_BACK","REJECTING","REJECTED","RECOVERY_REQUIRED","OBSERVING","OBSERVED"]);
function secret(v:unknown):boolean {
  return Array.isArray(v)?v.some(secret):object(v)&&Object.entries(v).some(([key,value])=>["ownerKey","confirmationToken"].includes(key)||secret(value));
}
export function validImportedResult(v:unknown,operation:number):v is ImportedResult {
  if(!object(v)||v.schemaVersion!==1||v.operation!==operation||!object(v.receipt)||secret(v)
    ||typeof v.writeEnabled!=="boolean"||typeof v.confirmationReady!=="boolean"||typeof v.newManagementConnectionVerified!=="boolean"
    ||!(v.originalTaskId===null||uuid(v.originalTaskId))||!(v.originalCommandSha256===null||digest(v.originalCommandSha256))
    ||!(v.decisionEvidence===null||object(v.decisionEvidence)))return false;
  const r=v.receipt;
  if(r.state!==undefined&&(!states.has(String(r.state))||!uuid(r.id)))return false;
  return !v.confirmationReady||v.newManagementConnectionVerified&&v.writeEnabled&&r.state==="WAIT_CONFIRM"&&r.deadlineExpired===false&&Number.isSafeInteger(r.deadlineUnixMs);
}
export function validImportedView(v:unknown):v is ImportedView {
  return object(v)&&v.sourceKind==="nft.imported.confirmed.v1"&&v.initialized===true&&digest(v.sourceDigest)
    &&Number.isSafeInteger(v.revision)&&Number(v.revision)>=0&&typeof v.projectable==="boolean"
    &&(v.projectable?candidateValid(v.candidate):v.candidate===null)&&typeof v.journalSettled==="boolean"
    &&(v.transaction===null||object(v.transaction))&&object(v.preservedSource)&&object(v.preservedSource.families);
}
export function confirmationRemaining(result:ImportedResult|undefined,now:number,online:boolean,unresolved:boolean):number|undefined {
  if(!result||!online||unresolved||!result.writeEnabled||!result.newManagementConnectionVerified||!result.confirmationReady||result.receipt.state!=="WAIT_CONFIRM"
    ||result.receipt.deadlineExpired!==false||!Number.isSafeInteger(result.receipt.deadlineUnixMs))return;
  const seconds=Math.ceil((Number(result.receipt.deadlineUnixMs)-now)/1000);
  return seconds>0?seconds:undefined;
}
export function prefixInput(candidate:Candidate,prefixes:Record<string,number[]>):boolean {
  return (["ipv4","ipv6"] as const).every(family=>{
    const rows=candidate.portForward.filter(row=>row.family===family),values=prefixes[family]??[];
    return rows.length===values.length&&values.every(value=>Number.isInteger(value)&&value>=0&&value<=(family==="ipv4"?32:128));
  });
}
