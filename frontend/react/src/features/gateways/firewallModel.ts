import type { components } from "../../api/schema";
export type Candidate=components["schemas"]["FirewallCandidate"];
export type Family="ipv4"|"ipv6";
export type Category="input"|"portForward"|"nat";
export type Receipt={id:string;state:string;deadlineUnixMs:number;revision:number;requestId:string;confirmationToken?:string;candidate?:Candidate;newManagementConnectionVerified?:boolean};
export type View={schemaVersion:1;revision:number;sourceDigest:string;initialized:boolean;projectable:boolean;candidate:Candidate|null;writeEnabled:boolean;confirmationReady:boolean;transaction:Receipt|null};
export type Pending={operation:31|32|33;key:string;id?:string};
export const operations={31:"firewall.prepare",32:"firewall.confirm",33:"firewall.rollback"} as const;
export const emptyCandidate=():Candidate=>({schemaVersion:4,ipv4:{inputPolicy:"accept",forwardPolicy:"drop",outputPolicy:"accept",allowIcmp:false},ipv6:{inputPolicy:"accept",forwardPolicy:"drop",outputPolicy:"accept",allowIcmp:false},input:[],portForward:[],nat:[]});
const object=(v:unknown):v is Record<string,unknown>=>!!v&&typeof v==="object"&&!Array.isArray(v);
const ports=(v:unknown)=>object(v)&&Number.isInteger(v.first)&&Number.isInteger(v.last)&&Number(v.first)>0&&Number(v.first)<=Number(v.last)&&Number(v.last)<=65535;
const optional=(v:unknown,check:(v:unknown)=>boolean)=>v==null||check(v);
const text=(v:unknown)=>typeof v==="string"&&v.length<=128&&!/[\s"'`;\\]/.test(v);
const iface=(v:unknown)=>typeof v==="string"&&v.length<=15&&/^(?:\+|[a-zA-Z0-9_.:-]+\+?)$/.test(v);
export function candidateValid(value:unknown):value is Candidate {
  if(!object(value)||![3,4].includes(Number(value.schemaVersion))||!Number.isInteger(value.schemaVersion)||!["ipv4","ipv6"].every(f=>{
    const p=value[f];if(!object(p))return false;
    const keys=value.schemaVersion===4?["inputPolicy","forwardPolicy","outputPolicy","allowIcmp"]:["inputPolicy","forwardPolicy","allowIcmp"];
    return Object.keys(p).length===keys.length&&keys.every(k=>Object.hasOwn(p,k))&&["accept","drop"].includes(String(p.inputPolicy))&&["accept","drop"].includes(String(p.forwardPolicy))&&typeof p.allowIcmp==="boolean"&&(value.schemaVersion!==4||["accept","drop"].includes(String(p.outputPolicy)));
  })||!["input","portForward","nat"].every(k=>Array.isArray(value[k])))return false;
  const groups=[value.input,value.portForward,value.nat] as unknown[][];
  if(groups.reduce((sum,v)=>sum+v.length,0)>256)return false;
  for(const [category,rows] of [["input",groups[0]],["portForward",groups[1]],["nat",groups[2]]] as const)for(const r of rows) {
    if(!object(r)||!["ipv4","ipv6"].includes(String(r.family))||!optional(r.source,text))return false;
    if(category==="input") {
      if(!["tcp","udp"].includes(String(r.protocol))||!ports(r.ports)||!optional(r.sourcePorts,ports)||!optional(r.inputInterface,iface)||!optional(r.excludedInterface,iface))return false;
    } else if(!iface(r.inputInterface)||!iface(r.outputInterface)||typeof r.masquerade!=="boolean")return false;
    if(category==="portForward"&&(!["tcp","udp"].includes(String(r.protocol))||!text(r.address)||!r.address||![r.inPort,r.outPort].every(v=>Number.isInteger(v)&&Number(v)>0&&Number(v)<=65535)||!optional(r.sourcePorts,ports)))return false;
    if(category!=="nat"&&!optional(r.sourceMac,v=>typeof v==="string"&&/^([0-9a-f]{2}:){5}[0-9a-f]{2}$/i.test(v)))return false;
    if(category==="nat"&&(typeof r.automatic!=="boolean"||!optional(r.destination,text)||!optional(r.ruleType,v=>["GENERIC","IP_FORWARDING","PORT_FORWARDING"].includes(String(v)))||!optional(r.protocol,v=>["all","tcp","udp","icmp","icmpv6","gre","esp","ah","sctp"].includes(String(v))||(typeof v==="number"&&Number.isInteger(v)&&v>=0&&v<=255))))return false;
  }
  return new TextEncoder().encode(JSON.stringify(value)).length<=16384;
}
export function validView(v:unknown):v is View {
  return object(v)&&v.schemaVersion===1&&Number.isSafeInteger(v.revision)&&Number(v.revision)>=0&&typeof v.sourceDigest==="string"&&/^[a-f0-9]{64}$/.test(v.sourceDigest)&&typeof v.confirmationReady==="boolean"&&typeof v.writeEnabled==="boolean"&&typeof v.initialized==="boolean"&&typeof v.projectable==="boolean"&&(v.projectable?candidateValid(v.candidate):v.candidate===null)&&(v.transaction===null||validReceipt(v.transaction));
}
export function validReceipt(v:unknown):v is Receipt {
  return object(v)&&typeof v.id==="string"&&/^fw-[0-9a-f-]{36}$/.test(v.id)&&typeof v.state==="string"&&["PREPARED","APPLYING","WAIT_CONFIRM","CONFIRMING","COMMITTED","ROLLING_BACK","ROLLED_BACK","RECOVERY_REQUIRED"].includes(v.state)&&Number.isSafeInteger(v.deadlineUnixMs)&&Number.isSafeInteger(v.revision);
}
export function newRule(category:Category,family:Family):Record<string,unknown> {
  if(category==="input")return {family,protocol:"tcp",ports:{first:0,last:0}};
  if(category==="portForward")return {family,protocol:"tcp",inputInterface:"",outputInterface:"",address:"",inPort:0,outPort:0,masquerade:false};
  return {family,inputInterface:"",outputInterface:"",protocol:null,masquerade:false,automatic:false,ruleType:null};
}
