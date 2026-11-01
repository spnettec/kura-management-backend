#!/usr/bin/env bash
set -euo pipefail

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

cd "$PROJECT_DIR/frontend"
npm ci
npm run typecheck
npm run build

cd "$PROJECT_DIR"
exec mvn -Pwith-frontend clean verify "$@"
