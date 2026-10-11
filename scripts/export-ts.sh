#!/usr/bin/env bash
# Write TypeScript definitions for every wire type.
#
#   scripts/export-ts.sh [out-dir]
#
# Default out-dir is ./bindings (gitignored). The TypeScript client package
# (modules/typescript/cnctd-server) points this at its own src/generated.
set -euo pipefail
cd "$(dirname "$0")/.."
OUT="${1:-bindings}"
mkdir -p "$OUT"
TS_RS_EXPORT_DIR="$(cd "$OUT" && pwd)" cargo test --features ts --lib --quiet export_bindings
echo "wrote:"
ls -1 "$OUT"
