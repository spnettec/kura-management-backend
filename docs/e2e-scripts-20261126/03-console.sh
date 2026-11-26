#!/bin/bash
# 03 部署管理中心：compose 启动控制台与 PostgreSQL，完成首次初始化（pair → operations），
#    核对管理员可以登录。
set -uo pipefail
. "$(dirname "$0")/e2e-common.sh"

step "03 部署管理中心"
(cd "$CONSOLE_REPO/docker" && docker compose up -d) || die "compose up 失败"
docker run -d --name yofc-e2e-pg --network docker_default \
    -e POSTGRES_USER=console -e POSTGRES_PASSWORD="$PG_PASSWORD" -e POSTGRES_DB=console \
    postgres:16-alpine >/dev/null || die "PostgreSQL 启动失败"

for i in $(seq 1 90); do
    health=$(docker inspect -f '{{.State.Health.Status}}' docker-console-1 2>/dev/null)
    docker exec yofc-e2e-pg pg_isready -U console -d console >/dev/null 2>&1 && ready=yes || ready=no
    [ "$health" = healthy ] && [ "$ready" = yes ] && break
    sleep 2
done
[ "$health" = healthy ] && [ "$ready" = yes ] || die "管理中心未就绪：console=$health postgres=$ready"
note "console healthy, postgres ready"

# 首次初始化向导页面（配对之前才看得到）
shot 01-console-setup-wizard "$CONSOLE_ORIGIN/setup"

install -d -m 0700 "$E2E_ROOT/console"
docker run --rm -v docker_console-data:/state:ro alpine cat /state/setup/installation-credential.json \
    > "$E2E_ROOT/console/credential.json" || die "读取安装凭据失败"
note "installation $(python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["installationId"])' "$E2E_ROOT/console/credential.json")"
python3 -B "$SCRIPT_DIR/e2e-console-setup.py" --root "$E2E_ROOT" \
    --db-url "postgres://console:${PG_PASSWORD}@yofc-e2e-pg:5432/console" || die "控制台初始化失败"
sleep 3
shot 02-console-login "$CONSOLE_ORIGIN/"
api login || die "管理员登录失败"
record OK console "初始化完成，管理员可登录"
