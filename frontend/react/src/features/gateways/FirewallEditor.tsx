import { Button, Tabs } from "antd";
import type { Candidate,Category,Family } from "./firewallModel";
import { newRule } from "./firewallModel";
import { useI18n } from "../../i18n/I18n";
import type { MessageKey } from "../../i18n/messages";
type Row=Record<string,unknown>;
export function FirewallEditor({value,disabled,pending=false,onChange}:{value:Candidate;disabled:boolean;pending?:boolean;onChange:(next:Candidate)=>void}) {
  const {t}=useI18n();const label=(field:string)=>t(`firewall.${field}` as MessageKey);
  const fieldStyle={display:"flex",flexDirection:"column",gap:4,minWidth:0} as const;
  function rows(category:Category,family:Family) {
    const list=value[category] as unknown as Row[];
    const update=(index:number,patch:Row)=>{const next=structuredClone(value);(next[category] as unknown as Row[])[index]={...list[index],...patch};onChange(next);};
    const select=(row:Row,index:number,key:string,options:string[],nullable=false)=><label style={fieldStyle}>{label(key)}<select disabled={disabled} value={row[key]==null?"":String(row[key])} onChange={e=>update(index,{[key]:e.target.value||null})}>{nullable&&<option value="">{label("unspecified")}</option>}{options.map(v=><option key={v} value={v}>{v}</option>)}</select></label>;
    const text=(row:Row,index:number,key:string,required=false)=><label style={fieldStyle}>{label(key)}<input disabled={disabled} maxLength={key.includes("Interface")?15:128} value={String(row[key]??"")} onChange={e=>update(index,{[key]:e.target.value||(required?"":null)})}/></label>;
    const number=(row:Row,index:number,key:string)=><label style={fieldStyle}>{label(key)}<input disabled={disabled} type="number" min={1} max={65535} value={Number(row[key]??0)} onChange={e=>update(index,{[key]:Number(e.target.value)})}/></label>;
    const range=(row:Row,index:number,key:"ports"|"sourcePorts")=><fieldset style={{minWidth:0,marginInline:0}}><legend>{label(key)}</legend>{["first","last"].map(end=><label key={end} style={fieldStyle}>{label(end)}<input disabled={disabled} type="number" min={1} max={65535} value={(row[key] as Row|null)?.[end] as number??""} onChange={e=>{
      const next={...(row[key] as Row??{first:null,last:null}),[end]:e.target.value===""?null:Number(e.target.value)};
      update(index,{[key]:next.first==null&&next.last==null?null:next});
    }}/></label>)}</fieldset>;
    return <><Button disabled={disabled||value.input.length+value.portForward.length+value.nat.length>=256} onClick={()=>{const next=structuredClone(value);(next[category] as unknown as Row[]).push(newRule(category,family));onChange(next);}}>{label("add")}</Button>
      {list.map((row,index)=>row.family!==family?null:<fieldset key={index} disabled={disabled||row.automatic===true} style={{marginTop:12,minWidth:0,marginInline:0}}><legend>{label(category)} {index+1}{row.automatic===true?` · ${label("automatic")}`:""}</legend>
        {row.automatic===true?<p>{String(row.inputInterface)} → {String(row.outputInterface)} · {label("automaticKept")}</p>:<>
        <div style={{display:"grid",gridTemplateColumns:"repeat(auto-fit,minmax(160px,1fr))",gap:12}}>
          {category!=="nat"&&select(row,index,"protocol",["tcp","udp"])}
          {category==="input"?<>{range(row,index,"ports")}{text(row,index,"inputInterface")}{text(row,index,"excludedInterface")}</>:<>{text(row,index,"inputInterface",true)}{text(row,index,"outputInterface",true)}</>}
          {category==="portForward"&&<>{text(row,index,"address",true)}{number(row,index,"inPort")}{number(row,index,"outPort")}</>}
          {text(row,index,"source")}
          {category!=="nat"?<>{text(row,index,"sourceMac")}{range(row,index,"sourcePorts")}</>:<>
            {text(row,index,"destination")}
            <label style={fieldStyle}>{label("protocol")}<select value={typeof row.protocol==="number"?"number":String(row.protocol??"")} onChange={e=>update(index,{protocol:e.target.value==="number"?0:e.target.value||null})}><option value="">{label("unspecified")}</option>{["all","tcp","udp","icmp","icmpv6","gre","esp","ah","sctp"].map(v=><option key={v}>{v}</option>)}<option value="number">{label("protocolNumber")}</option></select></label>
            {typeof row.protocol==="number"&&<label style={fieldStyle}>{label("protocolNumber")}<input type="number" min={0} max={255} value={row.protocol} onChange={e=>update(index,{protocol:Number(e.target.value)})}/></label>}
            {select(row,index,"ruleType",["GENERIC","IP_FORWARDING","PORT_FORWARDING"],true)}
          </>}
          {category!=="input"&&<label><input type="checkbox" checked={row.masquerade===true} onChange={e=>update(index,{masquerade:e.target.checked})}/>{label("masquerade")}</label>}
        </div><Button disabled={disabled} danger onClick={()=>{const next=structuredClone(value);(next[category] as unknown as Row[]).splice(index,1);onChange(next);}}>{label("remove")}</Button>
        </>}
      </fieldset>)}
    </>;
  }
  return <fieldset disabled={disabled} style={{minWidth:0,marginInline:0,padding:12}}><legend>{label(pending?"candidate":"draft")}</legend>
    {(["ipv4","ipv6"] as Family[]).map(family=><fieldset key={family} style={{minWidth:0,marginInline:0}}><legend>{family==="ipv4"?"IPv4":"IPv6"}</legend><div style={{display:"flex",gap:12,flexWrap:"wrap"}}>{(value.schemaVersion===4?["inputPolicy","forwardPolicy","outputPolicy"] as const:["inputPolicy","forwardPolicy"] as const).map(key=><label key={key} style={fieldStyle}>{label(key)}<select value={(value[family] as {inputPolicy:string;forwardPolicy:string;outputPolicy?:string})[key]} onChange={e=>onChange({...value,[family]:{...value[family],[key]:e.target.value}})}><option value="accept">{label("accept")}</option><option value="drop">{label("drop")}</option></select></label>)}<label><input type="checkbox" checked={value[family].allowIcmp} onChange={e=>onChange({...value,[family]:{...value[family],allowIcmp:e.target.checked}})}/>{label("allowIcmp")}</label></div></fieldset>)}
    <Tabs items={(Object.keys({input:0,portForward:0,nat:0}) as Category[]).flatMap(category=>(["ipv4","ipv6"] as Family[]).map(family=>({key:`${category}-${family}`,label:`${family==="ipv4"?"IPv4":"IPv6"} ${label(category)}`,children:rows(category,family)})))}/>
  </fieldset>;
}
