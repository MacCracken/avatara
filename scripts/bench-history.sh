#!/usr/bin/env bash
set -euo pipefail
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

SAMPLES="${BENCH_SAMPLES:-50}"
VERSION="$(tr -d '[:space:]' < VERSION)"
DATE="$(date -u +%Y-%m-%d)"
CSV="bench-history.csv"
MD="BENCHMARKS.md"

[ ! -f "$CSV" ] && echo "date,version,benchmark,time_ns" > "$CSV"

# Collect results into a temp file for dual use (CSV + markdown)
RESULTS="$(mktemp)"
trap 'rm -f "$RESULTS"' EXIT

# Criterion 0.8 output format (all regular ASCII spaces):
#   name time:   [61.185 ns 61.523 ns 61.698 ns]
#                 ^low  unit ^median unit ^high  unit
# Extract benchmark name, median value, and unit.
cargo bench --bench benchmarks -- --sample-size "$SAMPLES" --quiet 2>/dev/null | \
  grep -E '^\S.*\btime:\s+\[' | \
  sed -E 's/^([^ ]+) +time: +\[[0-9.]+ [a-zµ]+ ([0-9.]+) ([a-zµ]+) .*/\1,\2,\3/' | \
  while IFS=, read -r name val unit; do
    time_ns="$val"
    case "$unit" in
      ns) ;;
      µs) time_ns="$(echo "$val * 1000" | bc)" ;;
      ms) time_ns="$(echo "$val * 1000000" | bc)" ;;
      s)  time_ns="$(echo "$val * 1000000000" | bc)" ;;
    esac
    echo "${DATE},${VERSION},${name},${time_ns}" >> "$CSV"
    echo "${name},${val} ${unit}" >> "$RESULTS"
  done

# --- Generate BENCHMARKS.md ---

single_rows=""
group_rows=""
global_rows=""

while IFS=, read -r name timing; do
  row="| ${name} | ${timing} |"
  case "$name" in
    */single_profile|*/lookup_by_name)
      single_rows="${single_rows}${row}"$'\n' ;;
    all_traditions/*|compose/*|registry/*)
      global_rows="${global_rows}${row}"$'\n' ;;
    *)
      group_rows="${group_rows}${row}"$'\n' ;;
  esac
done < "$RESULTS"

cat > "$MD" <<EOF
# Benchmarks

> ${VERSION} — ${DATE} — ${SAMPLES} samples

## Tier 1: Single Profile

| Benchmark | Median |
|-----------|--------|
${single_rows}
## Tier 2: Per-Tradition Groups

| Benchmark | Median |
|-----------|--------|
${group_rows}
## Tier 3: Cross-Tradition

| Benchmark | Median |
|-----------|--------|
${global_rows}
EOF

echo "Benchmarks recorded for ${VERSION} (${DATE})"
