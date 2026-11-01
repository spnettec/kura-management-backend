#!/usr/bin/env python3
"""One-use live proof: a recorded Kura submission resolves an interrupted receipt without replay."""
import argparse
import hashlib
import http.cookiejar
import importlib.util
import json
import os
from pathlib import Path
import signal
import socket
import ssl
import struct
import subprocess
import sys
import tempfile
import threading
import time
import urllib.error
import urllib.request
import uuid

ROOT = Path(__file__).resolve().parents[1]
FIXTURE = ROOT / '.phase1-dev/configuration-outcome-snapshot-idea-20261101'
STATE = FIXTURE / 'state.json'
OUTCOMES = FIXTURE / 'kura-home/data/management-outcomes'
GATEWAY = 'configuration-outcome-snapshot-idea-20261101'
BASE = 'https://localhost:18443'
marker = FIXTURE / 'configuration-unknown-live-started'
spec = importlib.util.spec_from_file_location('unknown_dev', ROOT / 'scripts/phase1-dev.py')
dev = importlib.util.module_from_spec(spec); spec.loader.exec_module(dev)
state = json.loads(STATE.read_text())


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def stop_pid(pid, name):
    try: os.kill(pid, 0)
    except ProcessLookupError: return
    os.kill(pid, signal.SIGTERM)
    deadline = time.monotonic() + 10
    while time.monotonic() < deadline:
        try: os.kill(pid, 0)
        except ProcessLookupError: return
        time.sleep(.05)
    raise AssertionError(f'{name} did not stop')


class Frame:
    def __init__(self, sock):
        self.sock = sock
    def read(self):
        raw = b''
        while len(raw) < 4: raw += self.sock.recv(4-len(raw))
        size = struct.unpack('!I', raw)[0]
        if not 0 < size <= 1048576: raise ValueError('bad frame size')
        data = b''
        while len(data) < size: data += self.sock.recv(size-len(data))
        return json.loads(data)
    def write(self, frame):
        data=json.dumps(frame,separators=(',',':')).encode(); self.sock.sendall(struct.pack('!I',len(data))+data)


class Connection:
    def __init__(self, client, backend):
        self.client=Frame(client); self.backend=Frame(backend); self.drop_response=threading.Event()

def proxy_log(message):
    path=FIXTURE/'configuration-unknown-proxy.log'
    try:
        import sys; print('PROXY_LOG',message,file=sys.stderr,flush=True)
        with path.open('a') as output:
            output.write(f'{time.time():.3f} {message}\n')
            output.flush()
            os.fsync(output.fileno())
    except OSError as error:
        print(f'proxy_log_failed={error} path={path}',file=sys.stderr,flush=True)


def pump(connection, source_is_client):
    source=connection.client if source_is_client else connection.backend
    target=connection.backend if source_is_client else connection.client
    try:
        while True:
            frame=source.read(); proxy_log(f"{'kura>agent' if source_is_client else 'agent>kura'} {frame.get('kind')} {frame.get('method','')} id={frame.get('requestId','')}")
            if source_is_client and frame.get('kind')=='request' and frame.get('method')=='kura.configuration.update':
                connection.drop_response.set()
            if not source_is_client and connection.drop_response.is_set() and frame.get('kind')=='response':
                raise OSError('intentionally dropping recorded submission response')
            target.write(frame)
    except BaseException as error:
        proxy_log(f'disconnect source_client={source_is_client} error={type(error).__name__}:{error}')
    finally:
        try: connection.client.sock.close()
        except OSError: pass
        try: connection.backend.sock.close()
        except OSError: pass


def proxy_server(listener, ready):
    ready.set()
    while True:
        try: client,_=listener.accept()
        except OSError: return
        try: backend=socket.socket(socket.AF_UNIX,socket.SOCK_STREAM); backend.connect(state['socketPath'])
        except OSError:
            client.close(); continue
        connection=Connection(client,backend)
        threading.Thread(target=pump,args=(connection,True),daemon=True).start()
        threading.Thread(target=pump,args=(connection,False),daemon=True).start()


class Api:
    def __init__(self):
        self.jar=http.cookiejar.CookieJar(); self.csrf=''; self.opener=urllib.request.build_opener(
            urllib.request.HTTPCookieProcessor(self.jar), urllib.request.HTTPSHandler(context=ssl._create_unverified_context()))
    def request(self,path,method='GET',body=None,csrf=False):
        headers={'Origin':BASE,'Accept':'application/json'}
        if body is not None: headers['Content-Type']='application/json'
        if csrf: headers['x-csrf-token']=self.csrf
        data=None if body is None else json.dumps(body,separators=(',',':')).encode()
        req=urllib.request.Request(BASE+'/api/v1'+path,data=data,headers=headers,method=method)
        try:
            with self.opener.open(req,timeout=20) as response:
                raw=response.read(); return response.status,json.loads(raw) if raw else None
        except urllib.error.HTTPError as error:
            raw=error.read(); return error.code,json.loads(raw) if raw else None


def api_task(api,operation,payload=None):
    body={'operation':operation,'idempotencyKey':str(uuid.uuid4())}
    if payload: body.update(payload)
    status,value=api.request(f'/gateways/{GATEWAY}/tasks','POST',body,True); assert status==202,(status,value)
    return value['id']


def wait_task(api,id,deadline=35):
    end=time.monotonic()+deadline; states=[]
    while time.monotonic()<end:
        status,value=api.request('/tasks/'+id); assert status==200,(status,value)
        if not states or states[-1]!=value['state']: states.append(value['state'])
        if value['state'] not in {'QUEUED','RUNNING'}: return value,states
        time.sleep(.05)
    raise AssertionError('task deadline')


def outcome_files():
    return sorted(path for path in OUTCOMES.iterdir() if path.is_file() and path.name.endswith('.json'))


def java_pid():
    output=subprocess.check_output(['ps','-axww','-o','pid,command'],text=True).splitlines()
    pids=[int(line.split()[0]) for line in output if '/bin/java ' in line and str(FIXTURE/'runtime/jvm.args') in line]
    assert len(pids)<=1; return pids[0] if pids else 0


def run():
    assert state['phase']=='prepared'
    assert not (FIXTURE/'configuration-unknown-live-completed').exists()
    resuming=marker.exists()
    if not resuming: marker.write_text('one-use live response-loss proof\n')
    old_agent=int(state['agentPid']); kura=java_pid()
    before=outcome_files(); assert len(before)==2,before
    if not resuming:
        state['directSocketPath']=state['socketPath']; STATE.write_text(json.dumps(state,indent=2)+'\n'); STATE.chmod(0o600)
    direct=Path(state.get('directSocketPath') or state['socketPath'])
    if resuming and state.get('unknownLiveProxyDir'):
        proxy_dir=Path(state['unknownLiveProxyDir']); frontend=Path(state['unknownLiveProxySocket']); backend=Path(state.get('unknownLiveBackendSocket') or (proxy_dir/'agent.sock'))
    else:
        proxy_dir=Path(tempfile.mkdtemp(prefix='yofc-u-')); proxy_dir.chmod(0o700)
        frontend=proxy_dir/'kura.sock'; backend=proxy_dir/'agent.sock'
    if kura: stop_pid(kura,'fixture Kura')
    if old_agent: stop_pid(old_agent,'fixture agent')
    state['unknownLiveProxySocket']=str(frontend); state['unknownLiveBackendSocket']=str(backend); state['unknownLiveProxyDir']=str(proxy_dir)
    for stale in [frontend,backend]:
        if not stale.exists(): continue
        probe=socket.socket(socket.AF_UNIX)
        try: probe.connect(str(stale)); probe.close(); raise AssertionError(f'proxy socket is still active: {stale}')
        except FileNotFoundError: pass
        except ConnectionRefusedError: probe.close(); stale.unlink()
    args_path=FIXTURE/'runtime/jvm.args'; text=args_path.read_text()
    if f'"-Dkura.management.socket={direct}"' in text:
        text=text.replace(f'"-Dkura.management.socket={direct}"',f'"-Dkura.management.socket={frontend}"')
        state['jvmArgsReplaced']=True
    elif f'"-Dkura.management.socket={frontend}"' not in text:
        raise AssertionError('unknown runtime socket state')
    args_path.write_text(text)
    env={**os.environ,**dev.env(),'AGENT_GATEWAY_ID':GATEWAY,
         'AGENT_CERT':str(FIXTURE/'agent.pem'),'AGENT_KEY':str(FIXTURE/'agent.key'),
         'AGENT_SOCKET':str(backend),'AGENT_STATE_DB':str(FIXTURE/'agent.sqlite'),
         'AGENT_KURA_CONFIGURATION_UPDATE':'1','AGENT_KURA_SNAPSHOT':'1',
         'AGENT_KURA_NETWORK_INTERFACES':'','AGENT_NETWORK_INTERFACES':'','RUST_LOG':'info'}
    log=(FIXTURE/'agent.log').open('ab'); log.write(b'\n===== unknown live response-loss agent =====\n'); log.flush()
    agent=subprocess.Popen([str(dev.AGENT/'target/debug/yofc-management-agent')],env=env,stdout=log,stderr=subprocess.STDOUT,start_new_session=True)
    time.sleep(.5)
    if agent.poll() is not None:
        holders=subprocess.run(['lsof','-nP',str(backend)],text=True,capture_output=True)
        raise AssertionError({'agentExit':agent.returncode,'holders':holders.stdout,'holdersError':holders.stderr})
    if frontend.exists():
        probe=socket.socket(socket.AF_UNIX,socket.SOCK_STREAM)
        try: probe.connect(str(frontend)); probe.close(); raise AssertionError('proxy socket is still active')
        except FileNotFoundError: pass
        except ConnectionRefusedError: probe.close(); frontend.unlink()
    listener=socket.socket(socket.AF_UNIX,socket.SOCK_STREAM); listener.bind(str(frontend)); listener.listen(8)
    ready=threading.Event(); threading.Thread(target=proxy_server,args=(listener,ready),daemon=True).start(); ready.wait(); proxy_log('listener ready')
    java='/Library/Java/JavaVirtualMachines/temurin-21.jdk/Contents/Home/bin/java'
    runtime=FIXTURE/'runtime'; kura_log=(FIXTURE/'kura-unknown-live.log').open('ab')
    kura_log.write(b'\n===== unknown live response-loss Kura =====\n'); kura_log.flush()
    kura_process=subprocess.Popen([java,f'@{runtime/"jvm.args"}','-jar',str(runtime/'launcher.jar'),'-configuration',str(runtime/'configuration'),'-consoleLog','-console'],cwd=runtime,stdout=kura_log,stderr=subprocess.STDOUT,start_new_session=True)
    state.update(agentPid=agent.pid,kuraPid=kura_process.pid,unknownLiveProxySocket=str(frontend),
                 unknownLiveBackendSocket=str(backend),unknownLiveProxyDir=str(proxy_dir),unknownLiveKuraPid=kura_process.pid)
    STATE.write_text(json.dumps(state,indent=2)+'\n'); STATE.chmod(0o600)
    time.sleep(1); assert kura_process.poll() is None, {'kuraExit':kura_process.returncode}
    api=Api(); status,value=api.request('/auth/login','POST',{'username':'admin','password':(ROOT/'.phase1-dev/admin.password').read_text().strip()})
    assert status==200,(status,value); api.csrf=value['csrf']
    deadline=time.monotonic()+45
    while True:
        status,gateway=api.request('/gateways/'+GATEWAY); assert status==200
        if gateway['online'] and gateway['kuraAvailable']: break
        if time.monotonic()>deadline: raise AssertionError('gateway did not return online')
        time.sleep(.1)
    read,read_states=wait_task(api,api_task(api,'kura.configuration.get',{'kuraConfiguration':{'pid':'org.eclipse.kura.management.revisionFixture','locale':'en'}}))
    assert read['state']=='SUCCEEDED',read; current=read['result']; revision=current['revision']
    assert next(field['value'] for field in current['properties'] if field['id']=='count')=='1'
    command_id=api_task(api,'kura.configuration.update',{'kuraConfigurationUpdate':{
        'pid':'org.eclipse.kura.management.revisionFixture','expectedRevision':revision,
        'properties':[{'id':'count','value':{'text':'9223372036854775807'}}]}})
    expected_record=OUTCOMES/(command_id+'.json'); deadline=time.monotonic()+15
    while not expected_record.exists():
        if time.monotonic()>deadline: raise AssertionError('outcome record deadline; retain fixture')
        time.sleep(.01)
    result,states=wait_task(api,command_id)
    assert 'UNKNOWN' in states,states
    assert result['state']=='SUCCEEDED',result
    proof=result['result']; assert proof['submitted'] is True and proof['previousRevision']==revision
    reconciliation=proof['reconciliation']; assert reconciliation['source']=='configuration.journal.v1'
    assert reconciliation['commandId']==command_id and reconciliation['journalSha256']==sha(expected_record)
    final,_=wait_task(api,api_task(api,'kura.configuration.get',{'kuraConfiguration':{'pid':'org.eclipse.kura.management.revisionFixture','locale':'en'}}))
    assert final['state']=='SUCCEEDED' and next(x['value'] for x in final['result']['properties'] if x['id']=='count')=='9223372036854775807'
    record=json.loads(expected_record.read_text()); mode=oct(expected_record.stat().st_mode & 0o777); assert mode=='0o600'
    value={'fixture':GATEWAY,'commandId':command_id,'observedStates':states,'unknownThenReconciled':True,
           'receipt':proof,'record':record,'recordMode':mode,'recordSha256':sha(expected_record),
           'finalCount':'9223372036854775807','oldAgentPid':old_agent,'proxyAgentPid':agent.pid,'kuraPidAtWrite':kura}
    (FIXTURE/'configuration-unknown-live-result.json').write_text(json.dumps(value,indent=2)+'\n')
    (FIXTURE/'configuration-unknown-live-completed').write_text('response dropped only after durable record\n')
    return value


def finish_after_stop():
    assert (FIXTURE/'configuration-unknown-live-completed').exists()
    kura=int(state.get('unknownLiveKuraPid') or state.get('kuraPid') or 0); agent=int(state['agentPid'])
    if kura: stop_pid(kura,'fixture Kura')
    if agent: stop_pid(agent,'proxy fixture agent')
    try: listener_socket=socket.socket(socket.AF_UNIX); listener_socket.connect(state['unknownLiveProxySocket']); listener_socket.close(); raise AssertionError('proxy still accepting')
    except (ConnectionRefusedError,FileNotFoundError,PermissionError): pass
    args_path=FIXTURE/'runtime/jvm.args'; text=args_path.read_text()
    direct=Path(state['directSocketPath'])
    if f'"-Dkura.management.socket={state['unknownLiveProxySocket']}' in text:
        text=text.replace(f'"-Dkura.management.socket={state['unknownLiveProxySocket']}',f'"-Dkura.management.socket={direct}"')
    args_path.write_text(text)
    subprocess.run([str(dev.AGENT.parent/'kura-management-console/target/debug/yofc-management-console'),'set-gateway-enabled',GATEWAY,'false'],check=True,env={**os.environ,**dev.env()},stdout=subprocess.DEVNULL)
    state['phase']='stopped-data-retained'; state['kuraPid']=0; STATE.write_text(json.dumps(state,indent=2)+'\n'); STATE.chmod(0o600)
    return {'phase':state['phase'],'gatewayDisabled':True,'agentPid':0,'kuraPid':0}

if __name__=='__main__':
    parser=argparse.ArgumentParser(); parser.add_argument('action',choices=['run','finish']); action=parser.parse_args().action
    print(json.dumps(run() if action=='run' else finish_after_stop(),indent=2))
