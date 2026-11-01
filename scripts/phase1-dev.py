#!/usr/bin/env python3
"""Isolated macOS development stack. Never touches Kura runtime, personal state or system networking."""
import getpass
import datetime
import json
import os
from pathlib import Path
import secrets
import shlex
import signal
import ssl
import subprocess
import sys
import time
import urllib.request
import urllib.parse

ROOT = Path(__file__).resolve().parents[1]
DEV = ROOT / '.phase1-dev'
AGENT = ROOT.parent / 'kura-management-agent'
PG = Path(os.environ.get('PHASE1_PG_BIN', '/opt/homebrew/opt/postgresql@17/bin'))
HTTP_PORT = 18443
GRPC_PORT = 18444
PG_PORT = 15432

def run(args, **kw):
    return subprocess.run([str(a) for a in args], check=True, **kw)

def env():
    return dict(
        DATABASE_URL=f'postgresql://{getpass.getuser()}@localhost/management?host={urllib.parse.quote(str(DEV / "pgsock"), safe="")}&port={PG_PORT}',
        CONSOLE_DATA_KEY_FILE=str(DEV/'task-data.key'),
        CONSOLE_ORIGIN=f'https://localhost:{HTTP_PORT}', CONSOLE_HTTP_ADDR=f'127.0.0.1:{HTTP_PORT}',
        CONSOLE_GRPC_ADDR=f'127.0.0.1:{GRPC_PORT}', CONSOLE_TLS_CERT=str(DEV/'server.pem'),
        CONSOLE_TLS_KEY=str(DEV/'server.key'), CONSOLE_CLIENT_CA=str(DEV/'ca.pem'),
        CONSOLE_WEB_DIR=str(ROOT/'frontend/react/dist'),
        AGENT_CONSOLE_ENDPOINT=f'https://localhost:{GRPC_PORT}', AGENT_GATEWAY_ID='dev-gateway',
        AGENT_CA=str(DEV/'ca.pem'), AGENT_CERT=str(DEV/'agent.pem'), AGENT_KEY=str(DEV/'agent.key'),
        AGENT_SOCKET=str(DEV/'run/control.sock'), AGENT_STATE_DB=str(DEV/'agent-state/results.sqlite'),
        AGENT_KURA_UID=str(os.getuid()), AGENT_ADMIN_UID=str(os.getuid()),
        AGENT_KURA_NETWORK_INTERFACES='', RUST_LOG='info')

def certificates():
    if (DEV/'ca.pem').exists():
        if not (DEV/'expired.pem').exists():
            expired_certificate()
        return
    run(['openssl','req','-x509','-newkey','rsa:2048','-nodes','-days','7','-subj','/CN=YOFC phase1 local CA',
         '-addext','basicConstraints=critical,CA:TRUE','-addext','keyUsage=critical,keyCertSign,cRLSign',
         '-keyout',DEV/'ca.key','-out',DEV/'ca.pem'], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    for name,usage in [('server','serverAuth'),('agent','clientAuth'),('unregistered','clientAuth')]:
        ext=DEV/f'{name}.ext'
        ext.write_text(f'basicConstraints=critical,CA:FALSE\nkeyUsage=critical,digitalSignature,keyEncipherment\nextendedKeyUsage={usage}\nsubjectAltName=DNS:localhost,DNS:host.orb.internal,IP:127.0.0.1\n')
        run(['openssl','req','-newkey','rsa:2048','-nodes','-subj',f'/CN={name}', '-keyout',DEV/f'{name}.key','-out',DEV/f'{name}.csr'],stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL)
        run(['openssl','x509','-req','-in',DEV/f'{name}.csr','-CA',DEV/'ca.pem','-CAkey',DEV/'ca.key','-CAserial',DEV/'ca-serial','-CAcreateserial','-days','7','-extfile',ext,'-out',DEV/f'{name}.pem'],stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL)
    expired_certificate()
    for p in DEV.glob('*.key'):
        p.chmod(0o600)

def expired_certificate():
    """Use explicit dates; macOS LibreSSL x509 rejects zero/negative -days."""
    fixture=DEV/'expired-ca';fixture.mkdir(exist_ok=True,mode=0o700)
    (fixture/'newcerts').mkdir(exist_ok=True,mode=0o700)
    (fixture/'index').write_text('');(fixture/'serial').write_text(secrets.token_hex(12)+'\n')
    config=fixture/'openssl.cnf'
    config.write_text(f'''[ca]
default_ca=fixture
[fixture]
database={fixture/'index'}
serial={fixture/'serial'}
new_certs_dir={fixture/'newcerts'}
certificate={DEV/'ca.pem'}
private_key={DEV/'ca.key'}
default_md=sha256
policy=policy
x509_extensions=client
[policy]
commonName=supplied
[client]
basicConstraints=critical,CA:FALSE
keyUsage=critical,digitalSignature,keyEncipherment
extendedKeyUsage=clientAuth
''')
    now=datetime.datetime.utcnow()
    run(['openssl','req','-newkey','rsa:2048','-nodes','-subj','/CN=expired-fixture','-keyout',DEV/'expired.key','-out',DEV/'expired.csr'],stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL)
    run(['openssl','ca','-config',config,'-batch','-notext','-in',DEV/'expired.csr','-out',DEV/'expired.pem','-startdate',(now-datetime.timedelta(days=3)).strftime('%y%m%d%H%M%SZ'),'-enddate',(now-datetime.timedelta(days=2)).strftime('%y%m%d%H%M%SZ')],stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL)
    (DEV/'expired.key').chmod(0o600)

def alive(name):
    pidfile=DEV/f'{name}.pid'
    if not pidfile.exists():
        return False
    try:
        pid=int(pidfile.read_text())
        result=subprocess.run(['ps','-p',str(pid),'-o','command='],text=True,capture_output=True)
        expected=str(ROOT/'target/debug/yofc-management-console') if name=='console' else str(AGENT/'target/debug/yofc-management-agent')
        return result.returncode==0 and result.stdout.strip().startswith(expected)
    except (ValueError,ProcessLookupError):
        return False

def launch(name,binary):
    if alive(name):
        return
    with (DEV/f'{name}.log').open('ab') as log:
          child=subprocess.Popen([str(binary)],cwd=ROOT,env={**os.environ,**env()},stdout=log,stderr=log,start_new_session=True)
    (DEV/f'{name}.pid').write_text(str(child.pid))

def wait_ready():
    context=ssl.create_default_context(cafile=str(DEV/'ca.pem'))
    for _ in range(100):
        try:
            with urllib.request.urlopen(f'https://localhost:{HTTP_PORT}/health/ready',context=context,timeout=1) as r:
                if r.status==200:
                    return
        except (OSError,urllib.error.URLError):
            time.sleep(.2)
    raise RuntimeError(f'console did not become ready; see {DEV / "console.log"}')

def up():
    os.umask(0o077)
    DEV.mkdir(exist_ok=True,mode=0o700)
    (DEV/'pgsock').mkdir(exist_ok=True,mode=0o700)
    (DEV/'agent-state').mkdir(exist_ok=True,mode=0o700)
    certificates()
    if not (DEV/'pgdata/PG_VERSION').exists():
        run([PG/'initdb','-D',DEV/'pgdata','-A','trust','--no-locale','-E','UTF8'],stdout=subprocess.DEVNULL)
    status=subprocess.run([str(PG/'pg_ctl'),'-D',str(DEV/'pgdata'),'status'],capture_output=True)
    if status.returncode!=0:
        options=f'-k {shlex.quote(str(DEV / "pgsock"))} -p {PG_PORT} -c listen_addresses= -c max_connections=30'
        run([PG/'pg_ctl','-D',DEV/'pgdata','-l',DEV/'postgres.log','-o',options,'-w','start'],stdout=subprocess.DEVNULL)
    dbs=run([PG/'psql','-h',DEV/'pgsock','-p',PG_PORT,'-d','postgres','-Atc',"SELECT datname FROM pg_database WHERE datname='management'"],capture_output=True,text=True)
    if not dbs.stdout.strip():
        run([PG/'createdb','-h',DEV/'pgsock','-p',PG_PORT,'management'])
    binary=ROOT/'target/debug/yofc-management-console'
    if not (DEV/'task-data.key').exists():
        bound=run([PG/'psql','-h',DEV/'pgsock','-p',PG_PORT,'-d','management','-Atc',
                   "SELECT to_regclass('mgmt_task_crypto') IS NOT NULL"],capture_output=True,text=True)
        if bound.stdout.strip()=='t':
            rows=run([PG/'psql','-h',DEV/'pgsock','-p',PG_PORT,'-d','management','-Atc',
                      'SELECT count(*) FROM mgmt_task_crypto'],capture_output=True,text=True)
            if rows.stdout.strip()!='0':
                raise RuntimeError('original task-data.key is missing; restore it without regenerating')
        run([binary,'task-key-init',DEV/'task-data.key'])
    run([binary,'migrate'],env=env())
    for user,tenant,role in [('admin','dev','admin'),('viewer','dev','viewer'),('other','other','operator')]:
        pwfile=DEV/f'{user}.password'
        if not pwfile.exists():
            pwfile.write_text(secrets.token_urlsafe(24));pwfile.chmod(0o600)
            run([binary,'create-user',user,tenant,role,pwfile],env=env())
    exists=run([PG/'psql','-h',DEV/'pgsock','-p',PG_PORT,'-d','management','-Atc',"SELECT id FROM mgmt_gateways WHERE id='dev-gateway'"],capture_output=True,text=True)
    if not exists.stdout.strip():
        run([binary,'register-gateway','dev-gateway','dev','本机开发网关',DEV/'agent.pem'],env=env())
    launch('console',binary)
    wait_ready()
    launch('agent',AGENT/'target/debug/yofc-management-agent')
    print(json.dumps({'url':f'https://localhost:{HTTP_PORT}','username':'admin','passwordFile':str(DEV/'admin.password'),'caFile':str(DEV/'ca.pem'),'socket':str(DEV/'run/control.sock')},ensure_ascii=False))

def down(name=None):
    for item in [name] if name else ['agent','console']:
        if alive(item):
            os.kill(int((DEV/f'{item}.pid').read_text()),signal.SIGTERM)
            deadline=time.monotonic()+3
            while alive(item) and time.monotonic()<deadline:
                time.sleep(.05)
            if alive(item):
                raise RuntimeError(f'{item} did not stop; retained without SIGKILL')
    if not name and (DEV/'pgdata/PG_VERSION').exists():
        subprocess.run([str(PG/'pg_ctl'),'-D',str(DEV/'pgdata'),'-m','fast','-w','stop'],check=False,capture_output=True)
    print('Stopped requested phase1 development processes; data retained.')

if __name__=='__main__':
    command=sys.argv[1] if len(sys.argv)>1 else 'up'
    if command=='up':up()
    elif command=='down':down()
    elif command=='stop-agent':down('agent')
    elif command=='stop-console':down('console')
    elif command=='env':print(json.dumps(env()))
    else:raise SystemExit('usage: phase1-dev.py up|down|stop-agent|stop-console|env')
