# Documentation Health

A living ledger of avatara's docs and their currency. Refresh cadence is
**opportunistic** — a doc is touched when related work touches it, not on a
timer. Modeled on cyrius/vidya `doc-health.md`.

Buckets: ✅ Fresh · 🟡 Stale (needs a pass) · 🔵 Dated artifact (frozen on purpose) · ❓ Open question

_Last swept: 2026-06-03 (v2.5.4 — 2.5.x closeout + security audit)._

## Structural docs

| Doc | Last touched | Status | Notes |
|-----|--------------|--------|-------|
| `README.md` | 2026-06-03 | ✅ Fresh | Compiler pin 6.0.47; 320-byte profile with domain. |
| `CHANGELOG.md` | 2026-06-03 | ✅ Fresh | Entries through 2.5.4; `[Unreleased]` points at the roadmap. |
| `CLAUDE.md` | 2026-06-03 | ✅ Fresh | Version 2.5.4, compiler 6.0.47; struct/Result/domain/shadow API + xalloc documented. |
| `CONTRIBUTING.md` | — | ✅ Fresh | No version-specific content. |
| `SECURITY.md` | — | ✅ Fresh | — |
| `cyrius.cyml` | 2026-06-03 | ✅ Fresh | Pin 6.0.47; `result` in stdlib deps; `[lib]` distlib section incl. shadow. |

## Development

| Doc | Last touched | Status | Notes |
|-----|--------------|--------|-------|
| `docs/development/roadmap.md` | 2026-06-03 | ✅ Fresh | 2.5.x shipped (Result 2.5.0, shadow 2.5.1, domain 2.5.2, struct migration 2.5.3); affinity-graph declined; v2.6.0 Solar Year next. |
| `docs/development/state.md` | — | ❓ Open | Not yet split out. Volatile state (cyrius pin, consumer status) lives in CLAUDE.md + the roadmap Dependencies table. Adopt vidya's `state.md` split if CLAUDE.md churn warrants. |

## Architecture

| Doc | Last touched | Status | Notes |
|-----|--------------|--------|-------|
| `docs/architecture/overview.md` | 2026-06-03 | ✅ Fresh | 320-byte layout (domain), "27 maps", data flow, accessor pattern match current source. |
| `docs/architecture/adr/001–009` | 2026-06-03 | ✅ Fresh | ADR-008 (native struct migration) + ADR-009 (checked-allocation/OOM policy) added at 2.5.x. Decision records are point-in-time. |

## Benchmarks

| Doc | Last touched | Status | Notes |
|-----|--------------|--------|-------|
| `bench-history.csv` | 2026-06-03 | ✅ Fresh | Rows for 2.4.0–2.5.4. Net 2.5.0→2.5.3 (Result allocs + domain field + struct setters): noise-level on ≥500ns paths, no regressions. Recorded every release via `scripts/bench-history.sh`. |
| `benchmarks-rust-v-cyrius.md` | 2026-04-12 | 🔵 Dated artifact | Frozen Rust v1.1.0 vs Cyrius v2.0.1 comparison; references cc3 3.7.0 — intentionally not updated (historical record). |

## Forward commitments

- ✅ (done 2.5.3/2.5.4) ADR for the struct migration → ADR-008; checked-allocation policy → ADR-009.
- Consider splitting volatile state into `docs/development/state.md` (vidya pattern) if CLAUDE.md version/pin churn becomes noisy.
- Security posture (2.5.4 audit): no network/file/untrusted-input surface, no public CVEs in the stack; CWE-690 (unchecked alloc) closed via `xalloc`. Re-audit if an I/O or deserialization surface is ever added.
