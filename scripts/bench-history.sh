#!/usr/bin/env bash
set -euo pipefail
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

SAMPLES="${BENCH_SAMPLES:-50}"
VERSION="$(cat VERSION | tr -d '[:space:]')"
DATE="$(date -u +%Y-%m-%d)"
CSV="bench-history.csv"

[ ! -f "$CSV" ] && echo "date,version,benchmark,time_ns" > "$CSV"

cargo bench -- --sample-size "$SAMPLES" 2>/dev/null | \
  grep -E "^test |time:" | \
  paste - - | \
  sed -E 's/test ([^ ]+) +.*time: +\[([0-9.]+) .*/\1,\2/' | \
  while IFS=, read -r name time_ns; do
    echo "${DATE},${VERSION},${name},${time_ns}" >> "$CSV"
  done

echo "Benchmarks recorded for v${VERSION} (${DATE})"
