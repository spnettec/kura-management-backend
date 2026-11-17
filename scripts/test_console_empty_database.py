#!/usr/bin/env python3
# Copyright (c) 2026 YOFC IOT and others
# SPDX-License-Identifier: EPL-2.0
"""Create one owned external PostgreSQL fixture for a new native setup GUI.

Never connects to an existing database or initializes the console. Application
database details must be entered in the real setup page after this preparation.
Keep the original container, volume, access material and failures; never rerun
this creator over a retained case.
"""
import argparse
import json
import os
from pathlib import Path
import secrets
import subprocess
import time
import uuid


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--owned-host', type=Path, required=True)
    parser.add_argument('--results', type=Path, required=True)
    parser.add_argument('--postgres-image', required=True)
    parser.add_argument('--disposable-fixture', action='store_true', required=True)
    args = parser.parse_args()
    output = args.results.resolve()
    output.mkdir(mode=0o700, parents=True, exist_ok=False)

    def save(name, value):
        fd = os.open(output / name, os.O_CREAT | os.O_EXCL | os.O_WRONLY, 0o600)
        with os.fdopen(fd, 'w') as stream:
            json.dump(value, stream, indent=2)
            stream.write('\n')

    def command(arguments):
        return subprocess.run(arguments, capture_output=True, text=True, check=True, timeout=30).stdout

    owner = json.loads(args.owned_host.read_text())
    host = json.loads(command(['docker', 'inspect', owner['containerId']]))[0]
    assert host['Config']['Labels'].get('yofc.purpose') == 'native-console-empty-host'
    assert host['State']['Running']
    address = host['NetworkSettings']['Networks']['bridge']['IPAddress']
    assert address and not address.startswith('192.168.2.')
    image = json.loads(command(['docker', 'image', 'inspect', args.postgres_image]))[0]
    assert image['Id'] == args.postgres_image and image['Architecture'] == 'arm64'
    suffix = uuid.uuid4().hex[:12]
    name = 'yofc-console-empty-pg-' + suffix
    volume = 'yofc-console-empty-pg-data-' + suffix
    access = dict(host=None, port=5432, database='console_empty_native', username='console_native',
                  password=secrets.token_urlsafe(32), publicConsoleOrigin='https://' + address + ':18443')
    save('creation-intent.json', dict(ownedHost=owner['containerId'], postgresImage=image['Id'],
                                     containerName=name, volumeName=volume,
                                     publicConsoleOrigin=access['publicConsoleOrigin'],
                                     existingDatabaseAdoption=False, consoleInitialization=False))
    envfile = output / 'postgres-admin.env'
    fd = os.open(envfile, os.O_CREAT | os.O_EXCL | os.O_WRONLY, 0o600)
    with os.fdopen(fd, 'w') as stream:
        stream.write('POSTGRES_USER=fixture_admin\nPOSTGRES_DB=postgres\nPOSTGRES_PASSWORD=' +
                     secrets.token_urlsafe(32) + '\n')
    command(['docker', 'volume', 'create', '--label', 'yofc.purpose=native-console-empty-database', volume])
    cid = command(['docker', 'run', '--detach', '--pull=never', '--name', name,
                   '--label', 'yofc.purpose=native-console-empty-database', '--network', 'bridge',
                   '--memory', '512m', '--cpus', '1', '--env-file', str(envfile),
                   '--mount', 'type=volume,src=' + volume + ',dst=/var/lib/postgresql/data',
                   image['Id']]).strip()
    save('owned-fixture.json', dict(containerId=cid, volumeName=volume, imageId=image['Id'],
                                    associatedNativeHost=owner['containerId'], automaticReplayForbidden=True))
    for _ in range(80):
        result = subprocess.run(['docker', 'exec', cid, 'pg_isready', '-h', '127.0.0.1',
                                 '-U', 'fixture_admin'], capture_output=True)
        if result.returncode == 0:
            break
        time.sleep(0.25)
    else:
        raise RuntimeError('Original new database did not become ready; retain it')
    pg = json.loads(command(['docker', 'inspect', cid]))[0]
    access['host'] = pg['NetworkSettings']['Networks']['bridge']['IPAddress']
    save('application-access.json', access)
    # The public application role is a database owner, never a server superuser.
    password = access['password'].replace("'", "''")
    sql = "CREATE ROLE console_native LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE NOINHERIT PASSWORD '" + password + "';\n"
    sql += 'CREATE DATABASE console_empty_native OWNER console_native;\n'
    save('role-database-intent.json', dict(role='console_native', database='console_empty_native',
                                          superuser=False, originalNewClusterOnly=True,
                                          automaticReplayForbidden=True))
    result = subprocess.run(['docker', 'exec', '-i', cid, 'psql', '-U', 'fixture_admin', '-d', 'postgres',
                             '-v', 'ON_ERROR_STOP=1'], input=sql, text=True, capture_output=True, timeout=30)
    save('role-database-return.json', dict(returncode=result.returncode, stdout=result.stdout,
                                          stderr=result.stderr))
    if result.returncode:
        raise RuntimeError('Keep the original database preparation and inspect it; never recreate')
    query = "SELECT current_setting('server_version'),(SELECT count(*) FROM pg_tables WHERE schemaname NOT IN ('pg_catalog','information_schema')),(SELECT rolsuper FROM pg_roles WHERE rolname='console_native');"
    verified = command(['docker', 'exec', cid, 'psql', '-U', 'fixture_admin', '-d',
                        'console_empty_native', '-At', '-c', query]).strip().split('|')
    assert verified[1:] == ['0', 'f'], verified
    save('empty-database-observation.json', dict(serverVersion=verified[0], applicationTables=0,
                                                applicationRoleSuperuser=False,
                                                newVolumeOnly=True, consoleInitialized=False,
                                                host=access['host'], publicConsoleOrigin=access['publicConsoleOrigin']))
    print('OWNED_EXTERNAL_PG ' + cid, flush=True)
    print('Dedicated empty database ready with a non-superuser application role.', flush=True)


if __name__ == '__main__':
    main()
