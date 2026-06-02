#!/usr/bin/env bash
# Record the benchmark suite into bench-history.csv, keyed by date + version,
# so deltas and potential regressions are visible across releases. Run as a
# standard step of every version bump, then diff against the prior release's
# rows before tagging. See CLAUDE.md § Versioning & Benchmarking.
set -euo pipefail
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

VERSION="$(tr -d '[:space:]' < VERSION)"
DATE="$(date -u +%Y-%m-%d)"
CSV="bench-history.csv"

[ ! -f "$CSV" ] && echo "date,version,benchmark,time_ns" > "$CSV"

echo "Building benchmarks..."
cyrius deps 2>/dev/null
CYRIUS_DCE=1 cyrius build tests/avatara.bcyr /tmp/avatara_bench

echo "Running benchmarks..."
# Parse lines like "  name/case: 226ns avg (min=... max=...) [N iters]" and
# normalize the avg time to nanoseconds. awk handles the unit math (no bc).
/tmp/avatara_bench 2>&1 | awk -v date="$DATE" -v ver="$VERSION" '
    /:[[:space:]]+[0-9.]+[a-z]+[[:space:]]+avg/ {
        name = $1; sub(/:$/, "", name)
        for (i = 1; i <= NF; i++) if ($i == "avg") { v = $(i-1); break }
        unit = v; gsub(/[0-9.]/, "", unit)
        num  = v; gsub(/[a-z]/,  "", num)
        mult = (unit == "ns") ? 1 : \
               (unit == "us") ? 1000 : \
               (unit == "ms") ? 1000000 : \
               (unit == "s")  ? 1000000000 : 1
        printf "%s,%s,%s,%d\n", date, ver, name, num * mult
    }
' >> "$CSV"

RECORDED=$(grep -c ",${VERSION}," "$CSV")
echo "Benchmarks recorded for ${VERSION} (${DATE}): ${RECORDED} entries -> ${CSV}"
rm -f /tmp/avatara_bench
