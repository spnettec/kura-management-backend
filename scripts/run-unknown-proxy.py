#!/usr/bin/env python3
import importlib.util, os, socket, threading, time
from pathlib import Path
script=Path(__file__).resolve().parents[1]/'scripts/phase2-configuration-outcome-unknown-idea.py'
spec=importlib.util.spec_from_file_location('unknown_proxy_runtime',script); m=importlib.util.module_from_spec(spec); spec.loader.exec_module(m)
frontend=Path(m.state['unknownLiveProxySocket'])
if frontend.exists():
    probe=socket.socket(socket.AF_UNIX)
    try: probe.connect(str(frontend)); raise SystemExit('proxy socket already active')
    except ConnectionRefusedError: probe.close(); frontend.unlink()
listener=socket.socket(socket.AF_UNIX,socket.SOCK_STREAM); listener.bind(str(frontend)); listener.listen(8)
ready=threading.Event(); threading.Thread(target=m.proxy_server,args=(listener,ready),daemon=True).start(); ready.wait(); m.proxy_log('persistent listener ready')
print('PID',os.getpid(),'PPID',os.getppid(),'SID',os.getsid(0),'FD',listener.fileno(),flush=True)
print('proxy-ready',flush=True)
signal_ignore=['SIGHUP','SIGTERM']
import signal
for name in signal_ignore: signal.signal(getattr(signal,name), signal.SIG_IGN)
import faulthandler
faulthandler.dump_traceback_later(10, repeat=True)
while True:
    time.sleep(1)
    print('ALIVE',flush=True)
