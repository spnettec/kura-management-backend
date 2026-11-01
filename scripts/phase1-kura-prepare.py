#!/usr/bin/env python3
"""Copy an existing complete developer runtime, then prepare isolated IDEA data and bridge."""
import getpass
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import shutil
import sys

ROOT=Path(__file__).resolve().parents[1]
KURA=ROOT.parent/'kura'
DEV=ROOT/'.phase1-dev'
DEST=DEV/'kura-runtime'
BRIDGE=ROOT.parent/'kura-networking/bundles/org.eclipse.kura.management.bridge/target/org.eclipse.kura.management.bridge-1.0.0-SNAPSHOT.jar'
SOURCE=KURA/'kura-dev-runtime/target/runtime'

if DEST.exists():
    raise SystemExit(f'{DEST} already exists; retain it and use the existing IDEA configuration')
DEST.mkdir(parents=True)
for name in ['launcher.jar','framework.jar','inventory.json']:
    shutil.copy2(SOURCE/name,DEST/name)
shutil.copytree(SOURCE/'plugins',DEST/'plugins')
(DEST/'configuration').mkdir()
shutil.copy2(SOURCE/'configuration/config.ini',DEST/'configuration/config.ini')
sys.path.insert(0,str(KURA/'kura-dev-runtime/tools'))
spec=importlib.util.spec_from_file_location('kura_development_runtime',KURA/'kura-dev-runtime/tools/runtime.py')
module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
module.RUNTIME=DEST
os.environ.update(KURA_DEV_HOME=str(DEV/'kura-home'),KURA_HTTP_PORT='18080',KURA_HTTPS_PORT='18445',KURA_CLIENT_AUTH_PORT='18446')
module.prepare('macos')
shutil.copy2(BRIDGE,DEST/'plugins'/BRIDGE.name)
config=DEST/'configuration/config.ini'
text=config.read_text().replace('osgi.bundles=',f'osgi.bundles=reference:file:plugins/{BRIDGE.name}@5:start,',1)
text+='\norg.osgi.framework.system.packages.extra=jdk.net\n'
config.write_text(text)
with (DEST/'jvm.args').open('a') as out:
    out.write(f'"-Dkura.management.socket={DEV / "run/control.sock"}"\n')
    out.write(f'"-Dkura.management.agent.user={getpass.getuser()}"\n')
from xml.etree import ElementTree as E
component=E.Element('component',name='ProjectRunConfigurationManager')
c=E.SubElement(component,'configuration',default='false',name='Kura management phase1 isolated',type='JarApplication',factoryName='JAR Application')
for key,value in [('JAR_PATH',str(DEST/'launcher.jar')),('VM_PARAMETERS',f'@"{DEST / "jvm.args"}"'),('PROGRAM_PARAMETERS',f'-configuration "{DEST / "configuration"}" -consoleLog -console'),('WORKING_DIRECTORY',str(DEST)),('ALTERNATIVE_JRE_PATH','21'),('ALTERNATIVE_JRE_PATH_ENABLED','true')]:
    E.SubElement(c,'option',name=key,value=value)
E.SubElement(c,'method',v='2')
# Local IDEA run configuration under the existing .idea exclusion; no personal configuration overwritten.
destination=KURA/'.idea/runConfigurations/Kura_management_phase1_isolated.xml'
destination.parent.mkdir(exist_ok=True)
E.ElementTree(component).write(destination,encoding='unicode')
evidence={'runtime':str(DEST),'dataHome':str(DEV/'kura-home'),'sourceRuntime':str(SOURCE),'bridgeSha256':hashlib.sha256(BRIDGE.read_bytes()).hexdigest(),'ideaConfiguration':str(destination),'ports':[18080,18445,18446]}
(DEV/'kura-prepare.json').write_text(json.dumps(evidence,indent=2)+'\n')
print(json.dumps(evidence,indent=2))
