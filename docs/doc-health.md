# Documentation Health

A living ledger of avatara's docs and their currency. Refresh cadence is
**opportunistic** — a doc is touched when related work touches it, not on a
timer. Modeled on cyrius/vidya `doc-health.md`.

Buckets: ✅ Fresh · 🟡 Stale (needs a pass) · 🔵 Dated artifact (frozen on purpose) · ❓ Open question

_Last swept: 2026-07-22 (v2.8.1 — toolchain pin bump 6.2.11 → 6.4.69)._

Ground truth at this sweep: **374 archetypes, 27 traditions**; version **2.8.1**;
cyrius pin **6.4.69**; 82 integration tests; 49 benchmarks; `ArchetypeProfile`
= 320-byte `#derive` struct.

## Structural docs

| Doc | Last touched | Status | Notes |
|-----|--------------|--------|-------|
| `README.md` | 2026-07-22 | ✅ Fresh | 374/27, compiler 6.4.69; tradition table lists all 26 modules; Quick Start examples updated to the `Result` API (lookup/compose unwrap). |
| `CHANGELOG.md` | 2026-07-22 | ✅ Fresh | Entries through 2.8.1; `[Unreleased]` points at the roadmap (minors → 3.0.0). |
| `CLAUDE.md` | 2026-07-22 | ✅ Fresh | Version 2.8.1, compiler 6.4.69; main.cyr=smoke / tcyr=82 / bcyr=49 corrected; all 26 modules + struct/Result/domain/shadow/xalloc API. |
| `CONTRIBUTING.md` / `SECURITY.md` / `CODE_OF_CONDUCT.md` | — | ✅ Fresh | No version/count/API content. |
| `cyrius.cyml` | 2026-07-22 | ✅ Fresh | Pin 6.4.69; `result` in stdlib deps, `json` dropped (removed from stdlib in 6.1.x, unused); `[lib]` distlib section incl. solar/canaanite/etruscan/shadow. Vendored `lib/` re-resolved clean from scratch (declared subset + transitive = 29 files). |

## Development

| Doc | Last touched | Status | Notes |
|-----|--------------|--------|-------|
| `docs/development/roadmap.md` | 2026-06-03 | ✅ Fresh | Shipped 2.4.x–2.7.0 collapsed to a summary; remaining minors v2.8.0 (Tarot), v2.9.0 (I Ching), v2.10.0 (world-traditions), v2.11.0 (overlays) → v3.0.0 consolidation; affinity-graph declined. |
| `docs/development/state.md` | — | ❓ Open | Not split out; volatile state lives in CLAUDE.md + roadmap Dependencies table. Adopt vidya's `state.md` split only if CLAUDE.md churn warrants. |

## Architecture

| Doc | Last touched | Status | Notes |
|-----|--------------|--------|-------|
| `docs/architecture/overview.md` | 2026-06-03 | ✅ Fresh | 27 tradition modules / 374; 320-byte struct layout; data flow + accessors updated to the `Profile_set_*` / struct world. |
| `docs/architecture/adr/001–009` | 2026-06-03 | ✅ Fresh | Point-in-time decision records (008 struct migration, 009 checked-allocation). No present-tense claims gone stale. |

## Benchmarks

| Doc | Last touched | Status | Notes |
|-----|--------------|--------|-------|
| `bench-history.csv` | 2026-07-22 | ✅ Fresh | Rows for 2.4.0–2.8.1 (49 benches/release; 2.8.0 skipped bench-recording, restored at 2.8.1). Recorded every release via `scripts/bench-history.sh`; bench is a hard release gate (build + run-to-completion + exit 0). The 6.1.34/6.2.11 `kabbalah/single_profile` codegen creep **reversed** under 6.4.69: ~490 ns → ~286 ns (see 2.8.1 CHANGELOG). |
| `benchmarks-rust-v-cyrius.md` | 2026-04-12 | 🔵 Dated artifact | Frozen Rust v1.1.0 vs Cyrius v2.0.1 comparison; references cc3 3.7.0 — intentionally historical. |

## Forward commitments

- ✅ ADRs for struct migration (008) and checked-allocation (009) filed (2.5.x).
- Benchmarks are mandatory per release (build+run+exit-0+record) — added to CLAUDE.md § Versioning & Benchmarking after the v2.6.0 bench-include miss.
- Consider splitting volatile state into `docs/development/state.md` if CLAUDE.md version/pin churn grows.
