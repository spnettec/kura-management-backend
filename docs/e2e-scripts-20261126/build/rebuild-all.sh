#!/bin/bash
# 清空上一轮构建的全部中间产物，再从合并后的 heyoulin-kura 重新编译三个交付物：
#   1. Core deb            kura/build-all.sh（正式入口，BUILD_DOCKER=0，arm64 deb 由第 5 阶段产出）
#   2. agent deb           agent/build-agent.py（stage → cargo test → cargo build → package）
#   3. 管理中心镜像         kura-management-console/docker：docker compose build --no-cache
# 三个都成功后写 release-manifest.json；任一失败则以非 0 退出。
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
WS=/Users/heyoulin/iot-kura-develop/git
LOGS="$HERE/rebuild/$(date +%Y%m%d-%H%M%S)"
mkdir -p "$LOGS"
fail=0
step() { printf '\n== %s ==\n' "$*"; }

step "0 清空上一轮构建的中间产物"
for repo in kura kura-management-agent kura-management-console kura-networking; do
    printf '   %-26s HEAD %s  已跟踪文件改动=%s\n' "$repo" "$(git -C "$WS/$repo" rev-parse --short HEAD)" \
        "$(git -C "$WS/$repo" status --porcelain | grep -vc '^??')"
done
git -C "$WS/kura" worktree remove --force "$HERE/core/source" 2>/dev/null
git -C "$WS/kura" worktree prune
rm -rf "$HERE/core/source" "$HERE/core/release-m2" "$HERE/core/container-tree" "$HERE/core/build-core.py"
rm -rf "$HERE/agent/git" "$HERE/agent/source" "$HERE/agent/target" "$HERE/agent/release"
rm -f "$HERE"/agent/kura-management-agent_*.deb
rm -rf "$HERE"/runs/*/share "$HERE"/runs/*/pki
rm -f "$HERE"/runs/*/.orb-list "$HERE"/runs/*/.orb-after
echo "   已移除：自建 Core 驱动及其 worktree/仓库、agent 的源码导出与编译缓存、各轮的 httpd 共享目录与证书"

step "1 Core deb：build-all.sh"
qualifier="$(date -u +%Y%m%d%H%M)"
echo "$qualifier" > "$HERE/core/build-all-qualifier.txt"
( cd "$WS/kura" && KURA_BUILD_QUALIFIER="$qualifier" BUILD_DOCKER=0 ./build-all.sh ) > "$HERE/core/build-all.log" 2>&1
rc=$?; echo "BUILD_ALL_RC=$rc" >> "$HERE/core/build-all.log"; echo "   build-all rc=$rc  qualifier=$qualifier"
[ "$rc" = 0 ] || fail=1
CORE_DEB=$(ls -t "$WS/kura/kura/distrib/kura-core/target/"kura_*_arm64.deb 2>/dev/null | head -1)
[ -n "$CORE_DEB" ] || fail=1
echo "   $CORE_DEB"

step "2 agent deb：build-agent.py stage test build package"
( cd "$HERE/agent" && python3 -B build-agent.py stage test build package ) > "$HERE/agent/build-agent.log" 2>&1
rc=$?; echo "   build-agent rc=$rc"
[ "$rc" = 0 ] || fail=1
grep -E "^(stage|cargo-test|cargo-build|package) exit" "$HERE/agent/build-agent.log" | sed 's/^/   /'
grep -E "^test result:" "$HERE/agent/cargo-test.log" | awk '{p+=$4; f+=$6; i+=$8} END {printf "   cargo test: passed=%d failed=%d ignored=%d\n", p, f, i}'

step "3 管理中心镜像：docker compose build --no-cache"
( cd "$WS/kura-management-console/docker" && docker compose build --no-cache ) > "$HERE/console/build.log" 2>&1
rc=$?; echo "   console build rc=$rc"
[ "$rc" = 0 ] || fail=1
IMAGE_ID=$(docker image inspect yofc-management-console:latest -f '{{.Id}}' 2>/dev/null)
echo "   image $IMAGE_ID"

step "4 汇总"
CORE_SHA=$(shasum -a 256 "$CORE_DEB" | cut -d' ' -f1)
AGENT_JSON="$HERE/agent/release.json"
python3 - "$HERE/release-manifest.json" "$WS" "$CORE_DEB" "$CORE_SHA" "$AGENT_JSON" "$IMAGE_ID" "$qualifier" "$fail" <<'PYEOF'
import json, subprocess, sys
out, ws, core, core_sha, agent_json, image, qualifier, fail = sys.argv[1:]
def head(repo):
    return subprocess.check_output(['git', '-C', f'{ws}/{repo}', 'rev-parse', 'HEAD'], text=True).strip()
agent = json.load(open(agent_json))
manifest = {
    'failed': int(fail),
    'buildQualifier': qualifier,
    'core': {'deb': core, 'sha256': core_sha, 'revision': head('kura')},
    'agent': {'deb': agent['package'], 'sha256': agent['sha256'], 'revision': agent['revision'],
              'bridgeVersion': agent['bridgeVersion'], 'bridgeSha256': agent['bridgeSha256'],
              'coreLoaderSha256': agent['coreLoaderSha256']},
    'console': {'image': image, 'revision': head('kura-management-console')},
    'networking': {'revision': head('kura-networking')},
}
open(out, 'w').write(json.dumps(manifest, indent=2) + '\n')
print(json.dumps(manifest, indent=2))
PYEOF
[ "$fail" = 0 ] && echo "REBUILD OK" || { echo "REBUILD FAILED"; exit 1; }
