import { Alert, Button, Checkbox, Space } from "antd";
import { useEffect, useRef, useState } from "react";
import { ApiError, api, type Gateway, type Session, type Task } from "../../api/client";
import { errorText, useI18n } from "../../i18n/I18n";

type Transaction = {
  id: string; state: string; action: string; unit: string; issued: boolean;
  requestAccepted: boolean; expiresAtUnixMs: number; beforeBootId: string; afterBootId?: string;
  before: { pid: number; invocationId: string }; after?: { pid: number; invocationId: string };
  applicationVerificationConfigured: boolean; applicationVerified?: boolean;
  processRestartVerified?: boolean; hostRebootVerified?: boolean;
  scope: string; kuraRecoveryVerified: false; normalBootVerified: false;
  profileId?: string; approvedSnapshotSha256?: string; componentPids?: string[]; checkIds?: string[];
};
type Plan = Transaction & { confirmToken: string; grantSha256: string; unitDefinitionSha256: string };
type View = { configurationState: string; allowRestart: boolean; allowReboot: boolean; transaction: Transaction | null };
type Pending = { operation: 24 | 25 | 26 | 27 | 28; key: string; id?: string };
const operations = {24:"service.restart.prepare",25:"service.restart",26:"system.reboot.prepare",27:"system.reboot",28:"system.control.cancel"} as const;

export function SystemControlPanel({ gateway, user }: { gateway: Gateway; user: Session }) {
  const { t } = useI18n();
  const storage = `yofc.system-control:${user.id}:${gateway.id}`;
  const [view, setView] = useState<View>();
  const [plan, setPlan] = useState<Plan>();
  const [busy, setBusy] = useState(false);
  const [approved, setApproved] = useState(false);
  const [error, setError] = useState<unknown>();
  const [taskId, setTaskId] = useState<string>();
  const [uncertain, setUncertain] = useState(() => !!sessionStorage.getItem(storage));
  const [clock, setClock] = useState(Date.now());
  const controller = useRef(new AbortController());
  useEffect(() => { controller.current=new AbortController();const current=controller.current;const timer=setInterval(()=>setClock(Date.now()),1000);return ()=>{clearInterval(timer);current.abort();}; },[]);
  const read = <T,>(path:string,method="GET",body?:unknown) => api<T>(path,method,body,user.csrf,controller.current.signal);
  const base = `/gateways/${encodeURIComponent(gateway.id)}`;
  const allowed = user.role === "admin" && gateway.capabilities.includes("system.control.v1");
  const admitted = allowed && gateway.online;
  const permitted = (operation:number) => ![24,25,26,27].includes(operation) || gateway.capabilities.includes(operation<26 ? "service.restart.v1" : "system.reboot.v1");
  function accept(task:Task,operation:Pending["operation"]) {
    if(task.gatewayId!==gateway.id || task.operation!==operation) throw new ApiError("CONFLICT",0);
    if(["QUEUED","RUNNING","UNKNOWN"].includes(task.state)) {setUncertain(true);return false;}
    const value=task.result as unknown as Plan;
    if(["SUCCEEDED","FAILED"].includes(task.state) && value?.id) {
      if(value.kuraRecoveryVerified!==false || value.normalBootVerified!==false || !["administrator-selected-checks","system-process-only"].includes(value.scope)) throw new ApiError("CONFLICT",0);
      if([24,26].includes(operation)) {
        if(task.state!=="SUCCEEDED"||value.state!=="PREPARED"||value.issued!==false||!value.confirmToken) throw new ApiError("CONFLICT",0);
        setPlan(value);sessionStorage.setItem(`${storage}:prepare`,task.id);
      } else {
        if(value.state!==(operation===28?"CANCELLED":"COMPLETED")) throw new ApiError("CONFLICT",0);
        setPlan(undefined);
      }
      setView(old=>({...old,configurationState:old?.configurationState??"UNREAD",allowRestart:old?.allowRestart??false,allowReboot:old?.allowReboot??false,transaction:value}));
    } else if(task.state==="SUCCEEDED") throw new ApiError("CONFLICT",0);
    sessionStorage.removeItem(storage);setUncertain(false);setApproved(false);
    if(task.state!=="SUCCEEDED") throw new ApiError(task.error??task.state,0);
    return true;
  }
  async function readTask(id:string,operation:Pending["operation"],wait:boolean) {
    setTaskId(id);const deadline=Date.now()+300000;
    do {
      const task=await read<Task>(`/tasks/${encodeURIComponent(id)}`);
      if(accept(task,operation)) return;
      if(!wait) return;
      await new Promise(resolve=>setTimeout(resolve,750));
    } while(Date.now()<deadline&&!controller.current.signal.aborted);
    if(!controller.current.signal.aborted) throw new ApiError("TASK_PENDING",0);
  }
  async function load() {
    if(!allowed||busy) return;
    setBusy(true);setError(undefined);setApproved(false);
    try {
      const retained=sessionStorage.getItem(storage);
      if(retained) {
        const p=JSON.parse(retained) as Pending;
        if(!Object.hasOwn(operations,p.operation)||!p.key) throw new ApiError("CONFLICT",0);
        if(!p.id) {
          try {
            const original=await read<{id:string}>(`${base}/task-intents/${encodeURIComponent(p.key)}`);
            p.id=original.id;sessionStorage.setItem(storage,JSON.stringify(p));
          } catch(e) {if(!(e instanceof ApiError&&e.status===404)) throw e;}
        }
        if(p.id) await readTask(p.id,p.operation,false);
      }
      if(!gateway.online) return;
      const {id}=await read<{id:string}>(`${base}/tasks`,"POST",{operation:"service.status",idempotencyKey:crypto.randomUUID()});
      const deadline=Date.now()+35000;let next:View|undefined;
      do {
        const task=await read<Task>(`/tasks/${encodeURIComponent(id)}`);
        if(task.state==="SUCCEEDED") {next=task.result as unknown as View;break;}
        if(!["QUEUED","RUNNING"].includes(task.state)) throw new ApiError(task.error??task.state,0);
        await new Promise(resolve=>setTimeout(resolve,500));
      } while(Date.now()<deadline&&!controller.current.signal.aborted);
      if(!next||typeof next.allowRestart!=="boolean"||typeof next.allowReboot!=="boolean") throw new ApiError("TASK_PENDING",0);
      setView(next);setPlan(undefined);
      const originalId=sessionStorage.getItem(`${storage}:prepare`);
      if(next.transaction?.state==="PREPARED"&&originalId&&!sessionStorage.getItem(storage)) {
        const original=await read<Task>(`/tasks/${encodeURIComponent(originalId)}`);
        const value=original.result as unknown as Plan;
        if(value?.id===next.transaction.id&&[24,26].includes(original.operation)) accept(original,original.operation as 24|26);
      }
    } catch(e) {if(!controller.current.signal.aborted)setError(e);} finally {if(!controller.current.signal.aborted)setBusy(false);}
  }
  async function act(operation:Pending["operation"]) {
    if(!admitted||busy||uncertain||!permitted(operation)) return;
    if([25,27].includes(operation)&&(!approved||!plan||Date.now()>=plan.expiresAtUnixMs)) return;
    if([25,27,28].includes(operation)&&(!plan||view?.transaction?.state!=="PREPARED"||view.transaction.id!==plan.id))return;
    if([24,26].includes(operation)&&(!settled||view?.configurationState!=="READY"||!(operation===24?view.allowRestart:view.allowReboot)))return;
    const pending:Pending={operation,key:crypto.randomUUID()};
    sessionStorage.setItem(storage,JSON.stringify(pending));setUncertain(true);setBusy(true);setError(undefined);
    try {
      const action=[25,27,28].includes(operation)?{transactionId:plan!.id,confirmationToken:plan!.confirmToken}:undefined;
      const {id}=await read<{id:string}>(`${base}/tasks`,"POST",{operation:operations[operation],idempotencyKey:pending.key,...(action?{systemControlAction:action}:{})});
      pending.id=id;sessionStorage.setItem(storage,JSON.stringify(pending));
      if([24,26].includes(operation))sessionStorage.setItem(`${storage}:prepare`,id);
      await readTask(id,operation,true);
    } catch(e) {
      if(!controller.current.signal.aborted) {
        setError(e);
        if(!pending.id&&e instanceof ApiError&&e.status>=400&&e.status<500&&e.status!==408) {sessionStorage.removeItem(storage);setUncertain(false);}
      }
    } finally {if(!controller.current.signal.aborted)setBusy(false);}
  }
  const transaction=view?.transaction;
  const settled=!!view&&(!transaction||["COMPLETED","CANCELLED"].includes(transaction.state));
  const prepared=plan&&transaction?.state==="PREPARED"&&transaction.id===plan.id;
  const reboot=plan?.action==="system.reboot";
  const yes=(value:unknown)=>value===true?t("system.observed"):t("system.unproven");
  return <section aria-label={t("system.region")}>
    <h3>{t("system.title")}</h3><p className="muted">{t("system.scope")}</p>
    {error!=null&&<Alert type="error" showIcon title={errorText(error,t)}/>}
    {uncertain&&<Alert type="warning" title={t("system.unknown")}/>}
    {taskId&&<p>{t("boot.task")}: <code>{taskId}</code></p>}
    <Button disabled={!allowed||busy} loading={busy} onClick={()=>void load()}>{t("system.status")}</Button>
    {view&&<>
      <p>{t("system.grantState")}: <code>{view.configurationState}</code></p>
      <p>{t("boot.latest")}: <code>{transaction?.state??t("boot.none")}</code> {transaction?.unit}</p>
      <Space wrap>
        <Button disabled={!admitted||busy||uncertain||!settled||view.configurationState!=="READY"||!view.allowRestart||!permitted(24)} onClick={()=>void act(24)}>{t("system.prepareRestart")}</Button>
        <Button danger disabled={!admitted||busy||uncertain||!settled||view.configurationState!=="READY"||!view.allowReboot||!permitted(26)} onClick={()=>void act(26)}>{t("system.prepareReboot")}</Button>
      </Space>
    </>}
    {prepared&&<>
      <p>{t("system.target")}: {plan.unit} (PID {plan.before.pid})</p>
      <p>{t("system.beforeBoot")}: <code>{plan.beforeBootId}</code></p>
      <p>{t("boot.grant")}: <code>{plan.grantSha256}</code></p>
      <p>{t("handoff.checks")}: {plan.checkIds?.join(", ")||t("system.processOnly")}</p>
      {plan.applicationVerificationConfigured&&<><p>{t("handoff.profile")}: <code>{plan.profileId}</code></p><p>{t("handoff.snapshot")}: <code>{plan.approvedSnapshotSha256}</code></p><p>{t("handoff.components")}: {plan.componentPids?.join(", ")}</p></>}
      <p>{t("boot.expires")}: {new Date(plan.expiresAtUnixMs).toLocaleString()}</p>
      <Checkbox disabled={busy||uncertain} checked={approved} onChange={e=>setApproved(e.target.checked)}>{t(reboot?"system.approveReboot":"system.approveRestart")}</Checkbox>
      <Space wrap><Button danger={reboot} disabled={!admitted||busy||uncertain||!approved||clock>=plan.expiresAtUnixMs||!permitted(reboot?27:25)} onClick={()=>void act(reboot?27:25)}>{t(reboot?"system.executeReboot":"system.executeRestart")}</Button>
        <Button disabled={!admitted||busy||uncertain} onClick={()=>void act(28)}>{t("boot.cancel")}</Button></Space>
    </>}
    {transaction?.issued&&<>
      <p>{t("system.accepted")}: {yes(transaction.requestAccepted)}</p><p>{t("system.process")}: {yes(transaction.processRestartVerified)} {transaction.after?.pid}</p>
      <p>{t("system.host")}: {yes(transaction.hostRebootVerified)} {transaction.afterBootId}</p><p>{t("system.application")}: {yes(transaction.applicationVerified)}</p>
      <p className="muted">{transaction.applicationVerificationConfigured?t("system.selectedOnly"):t("system.processOnly")}</p>
    </>}
  </section>;
}
