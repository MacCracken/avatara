#!/usr/bin/env bash
set -euo pipefail
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

VERSION="$(tr -d '[:space:]' < VERSION)"
DATE="$(date -u +%Y-%m-%d)"
CSV="bench-history.csv"

[ ! -f "$CSV" ] && echo "date,version,benchmark,time_ns" > "$CSV"

echo "Building benchmarks..."
cyrius deps 2>/dev/null
cyrius build tests/avatara.bcyr /tmp/avatara_bench

echo "Running benchmarks..."
/tmp/avatara_bench 2>&1 | grep -E '^\s+\S+:' | while read -r line; do
    name=$(echo "$line" | sed 's/:.*//' | tr -d ' ')
    # Extract avg time and unit
    avg=$(echo "$line" | grep -oP '\d+[a-z]*\s+avg' | grep -oP '\d+[a-z]*')
    time_ns="$avg"
    case "$avg" in
        *us) time_ns="$(echo "${avg%us} * 1000" | bc)" ;;
        *ns) time_ns="${avg%ns}" ;;
        *ms) time_ns="$(echo "${avg%ms} * 1000000" | bc)" ;;
    esac
    echo "${DATE},${VERSION},${name},${time_ns}" >> "$CSV"
done

echo "Benchmarks recorded for ${VERSION} (${DATE})"
rm -f /tmp/avatara_bench
