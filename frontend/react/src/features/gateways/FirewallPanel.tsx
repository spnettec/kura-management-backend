import { useEffect,useRef,useState } from "react";
import { Alert,Button,Checkbox,Space } from "antd";
import { api,ApiError,type Gateway,type Session,type Task } from "../../api/client";
import { errorText,useI18n } from "../../i18n/I18n";
import { translatedCode } from "../../i18n/core";
import { FirewallEditor } from "./FirewallEditor";
import { candidateValid,emptyCandidate,operations,validReceipt,validView,type Candidate,type Pending,type Receipt,type View } from "./firewallModel";

export function FirewallPanel({gateway,user}:{gateway:Gateway;user:Session}) {
  const {t}=useI18n();
  const storage=`yofc.firewall:${user.id}:${gateway.id}:${gateway.bindingGeneration}`;
  const [seconds,setSeconds]=useState(180);
  const [view,setView]=useState<View>();const [draft,setDraft]=useState<Candidate>();const [plan,setPlan]=useState<Receipt>();
  const [consumed,setConsumed]=useState(false);
  const [busy,setBusy]=useState(false),[approved,setApproved]=useState(false),[initialize,setInitialize]=useState(false);
  const [pending,setPending]=useState(()=>!!sessionStorage.getItem(storage));const [error,setError]=useState<unknown>();
  const [taskId,setTaskId]=useState<string>(),[clock,setClock]=useState(Date.now());
  const abort=useRef(new AbortController());
  useEffect(()=>{abort.current=new AbortController();const current=abort.current;const timer=setInterval(()=>setClock(Date.now()),1000);return()=>{current.abort();clearInterval(timer);};},[]);
  const read=<T,>(path:string,method="GET",body?:unknown)=>api<T>(path,method,body,user.csrf,abort.current.signal);
  const allowed=user.role==="admin"&&gateway.enabled&&gateway.capabilities.includes("firewall.remote.v1");
  const base=`/gateways/${encodeURIComponent(gateway.id)}`;
  function retain(value:Pending) {const encoded=JSON.stringify(value);sessionStorage.setItem(storage,encoded);if(sessionStorage.getItem(storage)!==encoded)throw new ApiError("INTERNAL",0);setPending(true);}
  function accept(task:Task,operation:Pending["operation"]):boolean {
    if(task.gatewayId!==gateway.id||task.operation!==operation)throw new ApiError("CONFLICT",0);
    if(["QUEUED","RUNNING","UNKNOWN"].includes(task.state)){setPending(true);return false;}
    const value=task.result;
    if(task.state==="SUCCEEDED") {
      if(!validReceipt(value)||!(operation===31?["WAIT_CONFIRM","COMMITTED"].includes(value.state):value.state===(operation===32?"COMMITTED":"ROLLED_BACK")))throw new ApiError("CONFLICT",0);
      if(operation===31&&!candidateValid(value.candidate))throw new ApiError("CONFLICT",0);
      if(operation===31&&value.state==="WAIT_CONFIRM"&&(!value.confirmationToken||!/^[0-9a-f]{32}$/.test(value.confirmationToken)))throw new ApiError("CONFLICT",0);
      if(operation===32&&value.newManagementConnectionVerified!==true)throw new ApiError("CONFLICT",0);
      if(operation!==31&&plan&&plan.id!==value.id)throw new ApiError("CONFLICT",0);
      setClock(Date.now());setPlan(value);setView(old=>old?{...old,transaction:value}:old);
      if(operation===31){setDraft(structuredClone(value.candidate!));setConsumed(true);sessionStorage.setItem(`${storage}:prepare`,task.id);}
    } else if(validReceipt(value)) {
      if(value.state!=="ROLLED_BACK"||![31,32].includes(operation))throw new ApiError("CONFLICT",0);
      setClock(Date.now());setPlan(value);setView(old=>old?{...old,transaction:value}:old);
    }
    sessionStorage.removeItem(storage);setPending(false);setApproved(false);
    if(task.state!=="SUCCEEDED")throw new ApiError(task.error??task.state,0);
    return true;
  }
  async function readTask(id:string,operation:Pending["operation"],wait:boolean) {
    setTaskId(id);const deadline=Date.now()+35000;
    do {
      const task=await read<Task>(`/tasks/${encodeURIComponent(id)}`);
      if(accept(task,operation))return;
      if(!wait||task.state==="UNKNOWN")return;
      await new Promise(resolve=>setTimeout(resolve,500));
    }while(Date.now()<deadline&&!abort.current.signal.aborted);
    if(!abort.current.signal.aborted)throw new ApiError("TASK_PENDING",0);
  }
  async function load() {
    if(!allowed||busy)return;setBusy(true);setError(undefined);setApproved(false);
    try {
      const saved=sessionStorage.getItem(storage);
      if(saved) {
        const original=JSON.parse(saved) as Pending;
        if(!Object.hasOwn(operations,original.operation)||typeof original.key!=="string"||!original.key)throw new ApiError("CONFLICT",0);
        if(!original.id) {
          try {const found=await read<{id:string}>(`${base}/task-intents/${encodeURIComponent(original.key)}`);original.id=found.id;retain(original);}
          catch(e){if(!(e instanceof ApiError&&e.status===404))throw e;}
        }
        if(!original.id)return; // NOT_FOUND never authorizes a replacement write.
        await readTask(original.id,original.operation,false);
        if(sessionStorage.getItem(storage))return;
      }
      if(!gateway.online)return;
      const {id}=await read<{id:string}>(`${base}/tasks`,"POST",{operation:"firewall.view",idempotencyKey:crypto.randomUUID()});
      const deadline=Date.now()+35000;let next:View|undefined;
      do {
        const task=await read<Task>(`/tasks/${encodeURIComponent(id)}`);
        if(task.gatewayId!==gateway.id||task.operation!==30)throw new ApiError("CONFLICT",0);
        if(task.state==="SUCCEEDED"){if(!validView(task.result))throw new ApiError("CONFLICT",0);next=task.result;break;}
        if(!["QUEUED","RUNNING"].includes(task.state))throw new ApiError(task.error??task.state,0);
        await new Promise(resolve=>setTimeout(resolve,500));
      }while(Date.now()<deadline&&!abort.current.signal.aborted);
      if(!next)throw new ApiError("TASK_PENDING",0);
      setClock(Date.now());setConsumed(false);setView(next);setDraft(next.candidate?structuredClone(next.candidate):undefined);setInitialize(false);setPlan(undefined);
      const prepared=sessionStorage.getItem(`${storage}:prepare`);
      if(prepared&&next.transaction?.state==="WAIT_CONFIRM") {
        const task=await read<Task>(`/tasks/${encodeURIComponent(prepared)}`);
        if(task.gatewayId===gateway.id&&task.operation===31&&task.state==="SUCCEEDED"&&validReceipt(task.result)&&task.result.id===next.transaction.id&&task.result.confirmationToken) {
          if(!candidateValid(task.result.candidate))throw new ApiError("CONFLICT",0);
          setDraft(structuredClone(task.result.candidate));setPlan({...task.result,state:next.transaction.state,deadlineUnixMs:next.transaction.deadlineUnixMs});
        }
      }
    }catch(e){if(!abort.current.signal.aborted)setError(e);}finally{if(!abort.current.signal.aborted)setBusy(false);}
  }
  const transaction=plan??view?.transaction;
  const active=!!transaction&&! ["COMMITTED","ROLLED_BACK"].includes(transaction.state);
  const waiting=plan?.state==="WAIT_CONFIRM"&&clock<plan.deadlineUnixMs&&!!plan.confirmationToken&&view?.confirmationReady===true;
  const editing=!consumed&&allowed&&gateway.online&&view?.writeEnabled&&!busy&&!pending&&!active;
  const preparedReady=!!view&&!!draft&&candidateValid(draft)&&Number.isInteger(seconds)&&seconds>=30&&seconds<=300&&(!view.initialized?initialize:view.projectable);
  async function act(operation:Pending["operation"]) {
    if(!allowed||!gateway.online||busy||pending)return;
    if(operation===31&&(!editing||!preparedReady||!approved))return;
    if(operation!==31&&(!plan?.confirmationToken||plan.state!=="WAIT_CONFIRM"))return;
    if(operation===32&&(!approved||!waiting||!view?.writeEnabled))return;
    setBusy(true);setError(undefined);if(operation===31)setConsumed(true);
    const original:Pending={operation,key:crypto.randomUUID()};
    try {
      retain(original); // Only value-free correlation is persisted before POST.
      const body=operation===31?{firewallChange:{expectedRevision:view!.revision,sourceDigest:view!.sourceDigest,confirmSeconds:seconds,candidate:structuredClone(draft!)}}:{firewallAction:{transactionId:plan!.id,confirmationToken:plan!.confirmationToken}};
      const {id}=await read<{id:string}>(`${base}/tasks`,"POST",{operation:operations[operation],idempotencyKey:original.key,...body});
      original.id=id;retain(original);if(operation===31)sessionStorage.setItem(`${storage}:prepare`,id);
      await readTask(id,operation,true);
    }catch(e){if(!abort.current.signal.aborted){setError(e);if(!original.id&&e instanceof ApiError&&e.status>=400&&e.status<500&&e.status!==408){sessionStorage.removeItem(storage);setPending(false);setApproved(false);}}}
    finally{if(!abort.current.signal.aborted)setBusy(false);}
  }
  return <section aria-label={t("firewall.title")}><h3>{t("firewall.title")}</h3><p className="muted">{t("firewall.scope")}</p>
    {error!=null&&<Alert type="error" showIcon title={errorText(error,t)}/>}
    {pending&&<Alert type="warning" title={t("firewall.unknown")}/>}
    {taskId&&<p>{t("boot.task")}: <code>{taskId}</code></p>}
    <Button aria-label={t("firewall.read")} aria-busy={busy} disabled={!allowed||busy} loading={busy} onClick={()=>void load()}>{t("firewall.read")}</Button>
    {view&&<><p>{t("gateway.revision")}: {view.revision}</p>{!view.writeEnabled&&<p>{t("firewall.disabled")}</p>}
      {view.initialized&&!view.projectable&&<Alert type="warning" title={t("firewall.unprojectable")}/>}
      {!view.initialized&&!active&&<Checkbox disabled={!editing} checked={initialize} onChange={e=>{setInitialize(e.target.checked);setApproved(false);setDraft(e.target.checked?emptyCandidate():undefined);}}>{t("firewall.initialize")}</Checkbox>}
    </>}
    {transaction&&<><p>{t("firewall.transaction")}: <code>{transaction.id}</code> · {translatedCode(t,"state",transaction.state)}</p>
      {transaction.state==="WAIT_CONFIRM"&&!view?.confirmationReady&&<p>{t("firewall.reconnectRequired")}</p>}
      {transaction.state==="WAIT_CONFIRM"&&<p>{t("firewall.remaining")}: {Math.max(0,Math.ceil((transaction.deadlineUnixMs-clock)/1000))}</p>}
      {transaction.state==="COMMITTED"&&<Alert type="success" title={t("firewall.committed")}/>}
      {transaction.state==="ROLLED_BACK"&&<Alert type="info" title={t("firewall.rolledBack")}/>}
    </>}
    {!active&&draft&&<label style={{display:"block",marginBlock:12}}>{t("firewall.confirmSeconds")} <input type="number" min={30} max={300} disabled={!editing} value={seconds} onChange={e=>{setSeconds(Number(e.target.value));setApproved(false);}}/></label>}
    {draft&&<FirewallEditor value={draft} pending={active} disabled={!editing} onChange={v=>{setDraft(v);setApproved(false);}}/>}
    {draft&&!candidateValid(draft)&&<p role="alert">{t("firewall.invalid")}</p>}
    {(editing||waiting)&&<Checkbox disabled={busy||pending} checked={approved} onChange={e=>setApproved(e.target.checked)}>{t(waiting?"firewall.approveConfirm":"firewall.approveDraft")}</Checkbox>}
    <Space wrap><Button disabled={!editing||!preparedReady||!approved} onClick={()=>void act(31)}>{t("firewall.prepare")}</Button>
      <Button disabled={!allowed||!gateway.online||busy||pending||!waiting||!approved||!view?.writeEnabled} onClick={()=>void act(32)}>{t("firewall.confirm")}</Button>
      <Button disabled={!allowed||!gateway.online||busy||pending||!plan?.confirmationToken||plan.state!=="WAIT_CONFIRM"} onClick={()=>void act(33)}>{t("firewall.rollback")}</Button></Space>
  </section>;
}
