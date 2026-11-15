import {useEffect,useRef,useState} from "react";
import {Alert,Button,Checkbox,Space} from "antd";
import {api,ApiError,type Gateway,type Session,type Task} from "../../api/client";
import {errorText,useI18n} from "../../i18n/I18n";
import {translatedCode} from "../../i18n/core";
import {FirewallEditor} from "./FirewallEditor";
import {candidateValid,type Candidate} from "./firewallModel";
import {confirmationRemaining,digest,importedOperations,importedTerminal,object,prefixInput,uuid,validImportedResult,validImportedView,
  type Correlation,type ImportedKind,type ImportedResult,type ImportedView} from "./importedModel";

export function ImportedFirewallPanel({gateway,user}:{gateway:Gateway;user:Session}) {
  const {t}=useI18n();
  const storage=`yofc.imported:${user.id}:${gateway.id}:${gateway.bindingGeneration}`;
  const base=`/gateways/${encodeURIComponent(gateway.id)}`;
  const [view,setView]=useState<ImportedView>(),[draft,setDraft]=useState<Candidate>();
  const [prefixes,setPrefixes]=useState<Record<string,number[]>>({}),[seconds,setSeconds]=useState(120);
  const [review,setReview]=useState<{request:Record<string,unknown>;result:ImportedResult}>();
  const [observationReview,setObservationReview]=useState<{request:Record<string,unknown>;result:ImportedResult}>();
  const [result,setResult]=useState<ImportedResult>(),[fresh,setFresh]=useState(false),[approved,setApproved]=useState(false);
  const [busy,setBusy]=useState(false),[error,setError]=useState<unknown>(),[clock,setClock]=useState(Date.now());
  const [pending,setPending]=useState(()=>!!sessionStorage.getItem(`${storage}:write`));
  const [prepare,setPrepare]=useState(()=>sessionStorage.getItem(`${storage}:prepare`)??undefined);
  const [observation,setObservation]=useState(()=>sessionStorage.getItem(`${storage}:observation`)??undefined);
  const [lastTask,setLastTask]=useState<string>();
  const abort=useRef(new AbortController());
  useEffect(()=>{abort.current=new AbortController();return()=>abort.current.abort();},[]);
  const remaining=confirmationRemaining(result,clock,gateway.online&&gateway.capabilities.includes("firewall.imported.write.v1"),pending);
  useEffect(()=>{if(remaining===undefined)return;const timer=setInterval(()=>setClock(Date.now()),1000);return()=>clearInterval(timer);},[remaining!==undefined]);
  const allowed=user.role==="admin"&&gateway.enabled&&gateway.capabilities.includes("firewall.imported.original.v1");
  const writable=allowed&&gateway.online&&gateway.capabilities.includes("firewall.imported.write.v1")&&result?.writeEnabled===true;
  const transaction=result?.receipt.state?result.receipt:view?.transaction;
  const phase=String(transaction?.state??"");
  const active=!!phase&&!importedTerminal.has(phase);
  const editing=writable&&fresh&&view?.journalSettled===true&&!active&&!pending&&!busy;
  const read=<T,>(path:string,method="GET",body?:unknown)=>api<T>(path,method,body,user.csrf,abort.current.signal);
  function retained(slot:string,value:Correlation) {
    const text=JSON.stringify(value);sessionStorage.setItem(`${storage}:${slot}`,text);
    if(sessionStorage.getItem(`${storage}:${slot}`)!==text)throw new ApiError("INTERNAL",0);
    if(slot==="write")setPending(true);
  }
  function remembered(slot:string):Correlation|undefined {
    const value=sessionStorage.getItem(`${storage}:${slot}`);if(!value)return;
    const parsed=JSON.parse(value) as Correlation;
    if(!Object.hasOwn(importedOperations,parsed.kind)||!uuid(parsed.key)||[parsed.id,parsed.originalTaskId,parsed.operationId].some(id=>id!==undefined&&!uuid(id)))throw new ApiError("CONFLICT",0);
    return parsed;
  }
  function rememberOriginal(kind:ImportedKind,id:string) {
    const slot=kind==="prepare"?"prepare":kind==="sourceObserve"?"observation":undefined;
    if(slot){sessionStorage.setItem(`${storage}:${slot}`,id);if(slot==="prepare")setPrepare(id);else setObservation(id);}
  }
  function accept(task:Task,original:Correlation):ImportedResult|undefined {
    const kind=original.kind;
    if(task.gatewayId!==gateway.id||task.operation!==importedOperations[kind])throw new ApiError("CONFLICT",0);
    if(task.state==="UNKNOWN"){setResult(undefined);setFresh(false);setApproved(false);return;}
    if(["QUEUED","RUNNING"].includes(task.state))return;
    if(task.state!=="SUCCEEDED")throw new ApiError(task.error??task.state,0);
    if(!validImportedResult(task.result,task.operation))throw new ApiError("CONFLICT",0);
    const value=task.result;
    if(original.originalTaskId&&value.originalTaskId!==original.originalTaskId)throw new ApiError("CONFLICT",0);
    if(["prepare","sourceObserve"].includes(kind)&&value.originalTaskId!==task.id)throw new ApiError("CONFLICT",0);
    if(original.operationId) {
      const id=["review","sourceReview"].includes(kind)&&object(value.receipt.nativePlan)?value.receipt.nativePlan.operationId:value.receipt.id;
      if(id!==original.operationId)throw new ApiError("CONFLICT",0);
    }
    if(kind==="view") {
      if(!validImportedView(value.receipt))throw new ApiError("CONFLICT",0);
      const nextView=value.receipt;
      setView(nextView);setFresh(true);setReview(undefined);setObservationReview(undefined);
      setDraft(nextView.candidate?structuredClone(nextView.candidate):undefined);
      setPrefixes(Object.fromEntries(["ipv4","ipv6"].map(family=>[family,[...(nextView.preservedSource.families[family]?.runtimePortForwardDestinationPrefixes??[])]])));
    }
    if(!["view","review","sourceReview"].includes(kind))setFresh(false);
    if(kind==="prepare")setReview(undefined);
    if(kind==="sourceObserve"||kind==="sourceOutcome"&&value.receipt.state==="OBSERVED")setObservationReview(undefined);
    setResult(value);setClock(Date.now());setApproved(false);return value;
  }
  async function poll(original:Correlation,slot:string):Promise<ImportedResult|undefined> {
    if(!original.id) {
      try {const found=await read<{id:string}>(`${base}/task-intents/${encodeURIComponent(original.key)}`);
        if(!uuid(found.id))throw new ApiError("CONFLICT",0);original.id=found.id;retained(slot,original);rememberOriginal(original.kind,found.id);
      } catch(e){if(e instanceof ApiError&&e.status===404){
        if(slot==="read"){sessionStorage.setItem(`${storage}:lastRead`,JSON.stringify(original));sessionStorage.removeItem(`${storage}:read`);setResult(undefined);setFresh(false);}
        return;
      }throw e;}
    }
    setLastTask(original.id);const until=Date.now()+35000;
    do {
      const task:Task=await read<Task>(`/tasks/${encodeURIComponent(original.id!)}`);
      if(task.id!==original.id||task.gatewayId!==gateway.id||task.operation!==importedOperations[original.kind])throw new ApiError("CONFLICT",0);
      if(["FAILED","EXPIRED"].includes(task.state)) {sessionStorage.removeItem(`${storage}:${slot}`);setResult(undefined);setFresh(false);setApproved(false);if(slot==="write")setPending(false);}
      const value=accept(task,original);
      if(value) {sessionStorage.removeItem(`${storage}:${slot}`);if(slot==="write")setPending(false);return value;}
      if(["FAILED","EXPIRED"].includes(task.state))break;
      if(task.state==="UNKNOWN") {
        // Retain the uncertain query for audit. A later explicit click may make
        // another read of the SAME original write; no original task is resent.
        if(slot==="read"){sessionStorage.setItem(`${storage}:lastRead`,JSON.stringify(original));sessionStorage.removeItem(`${storage}:read`);}
        setError(new ApiError("OUTCOME_UNKNOWN",0));return;
      }
      await new Promise(resolve=>setTimeout(resolve,500));
    }while(Date.now()<until&&!abort.current.signal.aborted);
    if(!abort.current.signal.aborted)throw new ApiError("TASK_PENDING",0);
  }
  async function run(kind:ImportedKind,fields:Record<string,unknown>={}):Promise<ImportedResult|undefined> {
    const writing=["prepare","apply","confirm","rollback","cancel","sourceObserve"].includes(kind),slot=writing?"write":"read";
    const saved=remembered(slot);if(saved){await poll(saved,slot);return;} // Never reconstruct a lost write body or attach a new draft to an old review.
    const proposed=fields.review??fields.proposal??fields.observation;
    const original:Correlation={kind,key:crypto.randomUUID(),...(uuid(fields.originalTaskId)?{originalTaskId:fields.originalTaskId}:{}),
      ...(object(proposed)&&uuid(proposed.operationId)?{operationId:proposed.operationId}:{})};retained(slot,original);
    try {
      const {id}=await read<{id:string}>(`${base}/tasks`,"POST",{operation:`firewall.imported.${kind==="sourceReview"?"source.review":kind==="sourceObserve"?"source.observe":kind==="sourceOutcome"?"source.outcome":kind}`,
        idempotencyKey:original.key,importedFirewall:{kind,...fields}});
      if(!uuid(id))throw new ApiError("CONFLICT",0);original.id=id;retained(slot,original);rememberOriginal(kind,id);
      return await poll(original,slot);
    } catch(e) {
      if(!original.id&&e instanceof ApiError&&e.status>=400&&e.status<500&&e.status!==408) {
        sessionStorage.removeItem(`${storage}:${slot}`);if(writing)setPending(false);
      }
      throw e;
    }
  }
  async function action(work:()=>Promise<unknown>) {
    if(busy||!allowed||!gateway.online)return;setBusy(true);setError(undefined);setApproved(false);
    try{await work();}catch(e){if(!abort.current.signal.aborted)setError(e);}
    finally{if(!abort.current.signal.aborted)setBusy(false);}
  }
  async function load() {
    const original=remembered("write");if(original){await poll(original,"write");return;}
    await run("view");
  }
  async function query(source=false) {
    setResult(undefined);setFresh(false);setApproved(false);
    const unresolved=remembered("write");
    if(source&&unresolved&&unresolved.kind!=="sourceObserve")return;
    const id=unresolved?.id??(source?observation:prepare);
    if(!id) {if(unresolved)await poll(unresolved,"write");return;}
    const value=await run(source||unresolved?.kind==="sourceObserve"?"sourceOutcome":"outcome",{originalTaskId:id});
    if(value&&unresolved?.id===value.originalTaskId) {
      // The center resolves only a hash-bound original with durable evidence.
      const original=await read<Task>(`/tasks/${encodeURIComponent(unresolved.id)}`);
      if(original.id===unresolved.id&&original.gatewayId===gateway.id&&original.operation===importedOperations[unresolved.kind]
        &&["SUCCEEDED","FAILED","EXPIRED"].includes(original.state)) {sessionStorage.removeItem(`${storage}:write`);setPending(false);}
    }
  }
  async function reviewDraft() {
    if(!view||!draft||!candidateValid(draft)||!prefixInput(draft,prefixes))return;
    const request={operationId:crypto.randomUUID(),expectedRevision:view.revision,sourceDigest:view.sourceDigest,candidate:draft,destinationPrefixes:prefixes};
    const value=await run("review",{review:request});
    if(value?.operation===39&&digest(value.receipt.cohortDigest))setReview({request,result:value});
  }
  async function reviewObservation() {
    if(!view)return;const request={operationId:crypto.randomUUID(),expectedRevision:view.revision,sourceDigest:view.sourceDigest};
    const value=await run("sourceReview",{review:request});
    if(value?.operation===46&&digest(value.receipt.cohortDigest)&&object(value.receipt.nativePlan)&&digest(value.receipt.nativePlan.observedNativeSha256))setObservationReview({request,result:value});
  }
  const queryReady=!!prepare||!!observation||pending;
  const invalidate=()=>{setReview(undefined);setApproved(false);};
  return <section aria-label={t("imported.title")}><h3>{t("imported.title")}</h3><p className="muted">{t("imported.scope")}</p>
    {error!=null&&<Alert type="error" showIcon title={errorText(error,t)}/>}
    {pending&&<Alert type="warning" title={t("imported.unknown")}/>}
    <Space wrap><Button disabled={!allowed||!gateway.online||busy} onClick={()=>void action(load)}>{t("imported.read")}</Button>
      <Button disabled={!allowed||!gateway.online||busy||!queryReady} onClick={()=>void action(()=>query(!prepare&&!!observation))}>{t("imported.query")}</Button></Space>
    {lastTask&&<p>{t("boot.task")}: <code>{lastTask}</code></p>}
    {view&&<><p>{t("imported.revision")}: {view.revision} · {t("imported.version")}: {view.candidate?.schemaVersion??"—"}</p>
      <p>{t("imported.source")}: <code style={{overflowWrap:"anywhere"}}>{view.sourceDigest}</code></p>
      {!writable&&<Alert type="info" title={t("imported.readonly")}/>}</>}
    {phase&&<p>{t("nm.transaction")}: <code>{String(transaction?.id??"")}</code> · {translatedCode(t,"state",phase)}</p>}
    {phase==="RECOVERY_REQUIRED"&&<Alert type="error" title={t("nm.recovery")}/>}
    {remaining!==undefined&&<p role="timer">{t("nm.remaining")}: {remaining}</p>}
    {phase==="WAIT_CONFIRM"&&remaining===undefined&&<p>{t("imported.queryRequired")}</p>}
    {draft&&<FirewallEditor value={draft} disabled={!editing} pending={active} onChange={value=>{
      setPrefixes(Object.fromEntries((["ipv4","ipv6"] as const).map(family=>{
        const count=value.portForward.filter(row=>row.family===family).length;
        return [family,count===draft.portForward.filter(row=>row.family===family).length?prefixes[family]??[]:Array(count).fill(NaN)];
      })));setDraft(value);invalidate();
    }}/>}
    {draft&&<fieldset disabled={!editing}><legend>{t("imported.prefixes")}</legend>{(["ipv4","ipv6"] as const).flatMap(family=>
      draft.portForward.filter(row=>row.family===family).map((row,index)=><label key={`${family}:${index}`} style={{display:"block",marginBlock:8}}>{family} {row.address}:{row.inPort} / <input type="number" aria-label={`${family} ${t("imported.prefix")} ${index+1}`} min={0} max={family==="ipv4"?32:128} value={Number.isInteger(prefixes[family]?.[index])?prefixes[family][index]:""} onChange={event=>{
        const values=[...(prefixes[family]??[])];values[index]=event.target.value===""?NaN:Number(event.target.value);setPrefixes({...prefixes,[family]:values});invalidate();
      }}/></label>))}</fieldset>}
    {editing&&<label>{t("firewall.confirmSeconds")} <input type="number" min={30} max={300} value={seconds} onChange={event=>{setSeconds(Number(event.target.value));setApproved(false);}}/></label>}
    {review&&<p>{t("imported.cohort")}: <code>{String(review.result.receipt.cohortDigest)}</code></p>}
    {observationReview&&<p>{t("imported.observed")}: <code>{String((observationReview.result.receipt.nativePlan as Record<string,unknown>).observedNativeSha256)}</code></p>}
    {(review||phase==="PREPARED"||remaining!==undefined||observationReview)&&<Checkbox disabled={busy||pending||!writable} checked={approved} onChange={event=>setApproved(event.target.checked)}>{t("imported.approve")}</Checkbox>}
    <Space wrap><Button disabled={!editing||!draft||!candidateValid(draft)||!prefixInput(draft,prefixes)} onClick={()=>void action(reviewDraft)}>{t("imported.review")}</Button>
      <Button disabled={!editing||!review||!approved||!Number.isInteger(seconds)||seconds<30||seconds>300} onClick={()=>void action(()=>run("prepare",{proposal:{...review!.request,cohortDigest:review!.result.receipt.cohortDigest,confirmSeconds:seconds}}))}>{t("imported.prepare")}</Button>
      <Button disabled={!writable||busy||pending||phase!=="PREPARED"||!prepare||!approved} onClick={()=>void action(()=>run("apply",{originalTaskId:prepare}))}>{t("imported.apply")}</Button>
      <Button disabled={!writable||busy||pending||remaining===undefined||!prepare||!approved} onClick={()=>void action(()=>run("confirm",{originalTaskId:prepare}))}>{t("imported.confirm")}</Button>
      <Button disabled={!writable||busy||pending||phase!=="WAIT_CONFIRM"||!prepare} onClick={()=>void action(()=>run("rollback",{originalTaskId:prepare}))}>{t("imported.rollback")}</Button>
      <Button disabled={!allowed||!gateway.online||busy||pending||phase!=="PREPARED"||!prepare} onClick={()=>void action(()=>run("cancel",{originalTaskId:prepare}))}>{t("imported.cancel")}</Button>
    </Space>
    <p>{t("imported.observationScope")}</p><Space wrap>
      <Button disabled={!writable||busy||pending||active||!view||!fresh} onClick={()=>void action(reviewObservation)}>{t("imported.sourceReview")}</Button>
      <Button disabled={!writable||busy||pending||active||!fresh||!observationReview||!approved} onClick={()=>void action(()=>run("sourceObserve",{observation:{...observationReview!.request,
        cohortDigest:observationReview!.result.receipt.cohortDigest,observedNativeSha256:(observationReview!.result.receipt.nativePlan as Record<string,unknown>).observedNativeSha256}}))}>{t("imported.observe")}</Button>
      <Button disabled={!allowed||!gateway.online||busy||!observation||pending} onClick={()=>void action(()=>query(true))}>{t("imported.sourceOutcome")}</Button>
    </Space>
  </section>;
}
