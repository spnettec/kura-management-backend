import { useEffect,useRef,useState } from "react";
import { Alert,Button,Checkbox,Input,Select,Space } from "antd";
import { api,ApiError,type Gateway,type Session,type Task } from "../../api/client";
import { errorText,useI18n } from "../../i18n/I18n";
import { translatedCode } from "../../i18n/core";
import { changesValid,natValid,operations,validReceipt,validView,type NmView,type Pending,type NmReceipt } from "./nmModel";

// Joint NM/firewall transactions. The workflow mirrors FirewallPanel: read the
// observed source, edit a separate typed proposal, apply it under the agent's
// checkpoint, then confirm only after a new management connection.
export function NmPanel({gateway,user}:{gateway:Gateway;user:Session}) {
  const {t}=useI18n();
  const storage=`yofc.nm:${user.id}:${gateway.id}:${gateway.bindingGeneration}`;
  const [seconds,setSeconds]=useState(120);
  const [view,setView]=useState<NmView>();const [changes,setChanges]=useState<string>();
  const [natMode,setNatMode]=useState<"keep"|"disable"|"replace">("keep");
  const [natWan,setNatWan]=useState("");const [natLan,setNatLan]=useState<string[]>([]);
  const [plan,setPlan]=useState<NmReceipt>();
  const [consumed,setConsumed]=useState(false);
  const [busy,setBusy]=useState(false),[approved,setApproved]=useState(false);
  const [pending,setPending]=useState(()=>!!sessionStorage.getItem(storage));const [error,setError]=useState<unknown>();
  const [taskId,setTaskId]=useState<string>(),[clock,setClock]=useState(Date.now());
  const abort=useRef(new AbortController());
  useEffect(()=>{abort.current=new AbortController();const current=abort.current;const timer=setInterval(()=>setClock(Date.now()),1000);return()=>{current.abort();clearInterval(timer);};},[]);
  const read=<T,>(path:string,method="GET",body?:unknown)=>api<T>(path,method,body,user.csrf,abort.current.signal);
  const allowed=user.role==="admin"&&gateway.enabled&&gateway.capabilities.includes("nm.remote.v1");
  const base=`/gateways/${encodeURIComponent(gateway.id)}`;
  function retain(value:Pending) {const encoded=JSON.stringify(value);sessionStorage.setItem(storage,encoded);if(sessionStorage.getItem(storage)!==encoded)throw new ApiError("INTERNAL",0);setPending(true);}
  function accept(task:Task,operation:Pending["operation"]):boolean {
    if(task.gatewayId!==gateway.id||task.operation!==operation)throw new ApiError("CONFLICT",0);
    if(["QUEUED","RUNNING","UNKNOWN"].includes(task.state)){setPending(true);return false;}
    const value=task.result;
    if(task.state==="SUCCEEDED") {
      if(!validReceipt(value)||!(operation===35?["WAIT_CONFIRM","COMMITTED"].includes(value.state):value.state===(operation===36?"COMMITTED":"ROLLED_BACK")))throw new ApiError("CONFLICT",0);
      if(operation===35&&!/^[0-9a-f]{32}$/.test(String(value.confirmationToken)))throw new ApiError("CONFLICT",0);
      if(operation===36&&value.newManagementConnectionVerified!==true)throw new ApiError("CONFLICT",0);
      if(operation!==35&&plan&&plan.id!==value.id)throw new ApiError("CONFLICT",0);
      setClock(Date.now());setPlan(value);setView(old=>old?{...old,transaction:value}:old);
      if(operation===35) {
        const prepareKey=`${storage}:prepare`;
        sessionStorage.setItem(prepareKey,task.id);
        if(sessionStorage.getItem(prepareKey)!==task.id)throw new ApiError("INTERNAL",0);
        setConsumed(true);
      }
    } else if(validReceipt(value)) {
      if(value.state!=="ROLLED_BACK"||![35,36].includes(operation))throw new ApiError("CONFLICT",0);
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
      if(task.id!==id)throw new ApiError("CONFLICT",0);
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
      const {id}=await read<{id:string}>(`${base}/tasks`,"POST",{operation:"nm.view",idempotencyKey:crypto.randomUUID()});
      const deadline=Date.now()+35000;let next:NmView|undefined;
      do {
        const task=await read<Task>(`/tasks/${encodeURIComponent(id)}`);
        if(task.id!==id||task.gatewayId!==gateway.id||task.operation!==34)throw new ApiError("CONFLICT",0);
        if(task.state==="SUCCEEDED"){if(!validView(task.result))throw new ApiError("CONFLICT",0);next=task.result;break;}
        if(!["QUEUED","RUNNING"].includes(task.state))throw new ApiError(task.error??task.state,0);
        await new Promise(resolve=>setTimeout(resolve,500));
      }while(Date.now()<deadline&&!abort.current.signal.aborted);
      if(!next)throw new ApiError("TASK_PENDING",0);
      setClock(Date.now());setConsumed(false);setView(next);setPlan(undefined);setNatMode("keep");
      const prepared=sessionStorage.getItem(`${storage}:prepare`);
      if(prepared&&next.transaction?.state==="WAIT_CONFIRM") {
        const task=await read<Task>(`/tasks/${encodeURIComponent(prepared)}`);
        if(task.id===prepared&&task.gatewayId===gateway.id&&task.operation===35&&task.state==="SUCCEEDED"&&validReceipt(task.result)&&task.result.id===next.transaction.id&&task.result.confirmationToken) {
          setPlan({...task.result,state:next.transaction.state,deadlineUnixMs:next.transaction.deadlineUnixMs??task.result.deadlineUnixMs});
        }
      }
    }catch(e){if(!abort.current.signal.aborted)setError(e);}finally{if(!abort.current.signal.aborted)setBusy(false);}
  }
  const transaction=plan??view?.transaction;
  const active=!!transaction&&!["COMMITTED","ROLLED_BACK","CANCELLED","RECOVERY_REQUIRED"].includes(transaction.state);
  const waiting=view?.writeEnabled===true&&plan?.state==="WAIT_CONFIRM"&&clock<plan.deadlineUnixMs&&!!plan.confirmationToken&&view?.confirmationReady===true;
  const editing=!consumed&&allowed&&gateway.online&&view?.writeEnabled&&!busy&&!pending&&!active;
  function draftReady():boolean {
    if(!view||!changes||!seconds||seconds<30||seconds>300)return false;
    let parsed:unknown;try{parsed=JSON.parse(changes);}catch{return false;}
    if(!changesValid(parsed))return false;
    if(natMode==="replace"&&!natValid({wan:natWan,lan:natLan}))return false;
    return true;
  }
  const preparedReady=editing&&draftReady();
  async function act(operation:Pending["operation"]) {
    if(!allowed||!gateway.online||busy||pending||view?.writeEnabled!==true)return;
    if(operation===35&&(!editing||!preparedReady||!approved))return;
    if(operation!==35&&(!plan?.confirmationToken||plan.state!=="WAIT_CONFIRM"))return;
    if(operation===36&&(!approved||!waiting||!view?.writeEnabled))return;
    setBusy(true);setError(undefined);if(operation===35)setConsumed(true);
    const original:Pending={operation,key:crypto.randomUUID()};
    try {
      retain(original); // Only value-free correlation is persisted before POST.
      const body=operation===35?{nmChange:{sourceDigest:view!.sourceDigest,confirmSeconds:seconds,changes:JSON.parse(changes!),...(natMode==="disable"?{nat:{schemaVersion:1,action:"disable"}}:natMode==="replace"?{nat:{wan:natWan,lan:natLan}}:{})}}:{nmAction:{operationId:plan!.id,confirmationToken:plan!.confirmationToken}};
      const {id}=await read<{id:string}>(`${base}/tasks`,"POST",{operation:operations[operation],idempotencyKey:original.key,...body});
      original.id=id;retain(original);
      if(operation===35)sessionStorage.setItem(`${storage}:prepare`,id);
      await readTask(id,operation,true);
    }catch(e){if(!abort.current.signal.aborted){setError(e);if(!original.id&&e instanceof ApiError&&e.status>=400&&e.status<500&&e.status!==408){sessionStorage.removeItem(storage);setPending(false);setApproved(false);}}}
    finally{if(!abort.current.signal.aborted)setBusy(false);}
  }
  return <section aria-label={t("nm.title")}><h3>{t("nm.title")}</h3><p className="muted">{t("nm.scope")}</p>
    {error!=null&&<Alert type="error" showIcon title={errorText(error,t)}/>}
    {pending&&<Alert type="warning" title={t("nm.unknown")}/>}
    {taskId&&<p>{t("boot.task")}: <code>{taskId}</code></p>}
    <Button aria-label={t("nm.read")} aria-busy={busy} disabled={!allowed||busy} loading={busy} onClick={()=>void load()}>{t("nm.read")}</Button>
    {view&&<><p>{t("nm.sourceDigest")}: <code>{view.sourceDigest.slice(0,16)}…</code></p>{!view.writeEnabled&&<p>{t("nm.disabled")}</p>}</>}
    {transaction&&<><p>{t("nm.transaction")}: <code>{transaction.id}</code> · {translatedCode(t,"state",transaction.state)}</p>
      {transaction.state==="WAIT_CONFIRM"&&!view?.confirmationReady&&<p>{t("nm.reconnectRequired")}</p>}
      {transaction.state==="WAIT_CONFIRM"&&<p>{t("nm.remaining")}: {Math.max(0,Math.ceil(((transaction.deadlineUnixMs??0)-clock)/1000))}</p>}
      {transaction.state==="COMMITTED"&&<Alert type="success" title={t("nm.committed")}/>}
      {transaction.state==="ROLLED_BACK"&&<Alert type="info" title={t("nm.rolledBack")}/>}
      {transaction.state==="RECOVERY_REQUIRED"&&<Alert type="error" title={t("nm.recovery")}/>}
    </>}
    {editing&&<label style={{display:"block",marginBlock:12}}>{t("nm.confirmSeconds")} <input type="number" min={30} max={300} value={seconds} onChange={e=>{setSeconds(Number(e.target.value));setApproved(false);}}/></label>}
    {editing&&<label style={{display:"block",marginBlock:12}}>{t("nm.changes")}
      <Input.TextArea rows={8} value={changes} onChange={e=>{setChanges(e.target.value);setApproved(false);}} placeholder='[{"interface":"eth0","ipv4":{...}}]'/>
    </label>}
    {editing&&<fieldset style={{marginBlock:12}}><legend>{t("nm.nat")}</legend>
      <label>{t("nm.natMode")} <select aria-label={t("nm.natMode")} value={natMode} onChange={e=>{setNatMode(e.target.value as "keep"|"disable"|"replace");setApproved(false);}}>
        <option value="keep">{t("nm.natKeep")}</option><option value="disable">{t("nm.natDisable")}</option><option value="replace">{t("nm.natSet")}</option>
      </select></label>
      {natMode==="replace"&&<><label style={{display:"block",marginBlock:8}}>{t("nm.natWan")} <Input aria-label={t("nm.natWan")} value={natWan} maxLength={15} onChange={e=>{setNatWan(e.target.value);setApproved(false);}} placeholder="eth0"/></label>
        <label style={{display:"block",marginBlock:8}}>{t("nm.natLan")} <Select mode="tags" aria-label={t("nm.natLan")} value={natLan} onChange={value=>{setNatLan(value);setApproved(false);}} tokenSeparators={[","," "]} style={{minWidth:240}}/></label></>}
    </fieldset>}
    {editing&&!draftReady()&&changes&&<p role="alert">{t("nm.invalid")}</p>}
    {(editing||waiting)&&<Checkbox disabled={busy||pending} checked={approved} onChange={e=>setApproved(e.target.checked)}>{t(waiting?"nm.approveConfirm":"nm.approveDraft")}</Checkbox>}
    <Space wrap><Button disabled={!editing||!preparedReady||!approved} onClick={()=>void act(35)}>{t("nm.prepare")}</Button>
      <Button disabled={!allowed||!gateway.online||busy||pending||!waiting||!approved||!view?.writeEnabled} onClick={()=>void act(36)}>{t("nm.confirm")}</Button>
      <Button disabled={!allowed||!gateway.online||busy||pending||view?.writeEnabled!==true||!plan?.confirmationToken||plan.state!=="WAIT_CONFIRM"} onClick={()=>void act(37)}>{t("nm.rollback")}</Button></Space>
  </section>;
}
