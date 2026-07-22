# Documentation Health

A living ledger of avatara's docs and their currency. Refresh cadence is
**opportunistic** — a doc is touched when related work touches it, not on a
timer. Modeled on cyrius/vidya `doc-health.md`.

Buckets: ✅ Fresh · 🟡 Stale (needs a pass) · 🔵 Dated artifact (frozen on purpose) · ❓ Open question

_Last swept: 2026-07-22 (v2.10.1 — I Ching, plus the similar_to top-k performance fix)._

Ground truth at this sweep: **460 archetypes, 29 traditions**; version **2.10.1**;
cyrius pin **6.4.70**; 166 integration tests; 52 benchmarks; `ArchetypeProfile`
= 320-byte `#derive` struct.

## Structural docs

| Doc | Last touched | Status | Notes |
|-----|--------------|--------|-------|
| `README.md` | 2026-07-22 | ✅ Fresh | 460/29, compiler 6.4.70; tradition table lists all 27 modules incl. Tarot and I Ching; Quick Start examples on the `Result` API (lookup/compose unwrap). |
| `CHANGELOG.md` | 2026-07-22 | ✅ Fresh | Entries through 2.10.1 (I Ching + the similar_to top-k fix); `[Unreleased]` points at the roadmap (minors → 3.0.0). |
| `CLAUDE.md` | 2026-07-22 | ✅ Fresh | Version 2.10.1, compiler 6.4.70; main.cyr=smoke / tcyr=166 / bcyr=52; all tradition modules incl. `src/tarot.cyr` and `src/iching.cyr` + struct/Result/domain/shadow/xalloc API. |
| `CONTRIBUTING.md` / `SECURITY.md` / `CODE_OF_CONDUCT.md` | — | ✅ Fresh | No version/count/API content. |
| `cyrius.cyml` | 2026-07-22 | ✅ Fresh | Pin 6.4.70; `result` in stdlib deps, `json` dropped (removed from stdlib in 6.1.x, unused); `[lib]` distlib section incl. iching/tarot/solar/canaanite/etruscan/shadow. Vendored `lib/` re-resolved clean from scratch (declared subset + transitive = 29 files). |

## Development

| Doc | Last touched | Status | Notes |
|-----|--------------|--------|-------|
| `docs/development/roadmap.md` | 2026-07-22 | ✅ Fresh | Shipped through 2.10.1 (I Ching + similar_to top-k fix); remaining minors v2.11.0 (world-traditions), v2.12.0 (overlays) → v3.0.0 consolidation; Backlog: additive Tarot de Marseille attribution (alongside, not replacing, RWS/Golden Dawn); affinity-graph declined. |
| `docs/development/state.md` | — | ❓ Open | Not split out; volatile state lives in CLAUDE.md + roadmap Dependencies table. Adopt vidya's `state.md` split only if CLAUDE.md churn warrants. |

## Architecture

| Doc | Last touched | Status | Notes |
|-----|--------------|--------|-------|
| `docs/architecture/overview.md` | 2026-07-22 | ✅ Fresh | 29 tradition modules / 460 (Tarot + I Ching added); 320-byte struct layout; data flow + accessors updated to the `Profile_set_*` / struct world. |
| `docs/architecture/adr/001–009` | 2026-06-03 | ✅ Fresh | Point-in-time decision records (008 struct migration, 009 checked-allocation). No present-tense claims gone stale. |

## Benchmarks

| Doc | Last touched | Status | Notes |
|-----|--------------|--------|-------|
| `bench-history.csv` | 2026-07-22 | ✅ Fresh | Rows for 2.4.0–2.10.1 (49/release through 2.8.1; 50 from 2.9.0 with `tarot/all_22`; 52 from 2.10.0 adding the two `iching/*` benches; 2.8.0 skipped bench-recording, restored at 2.8.1). Recorded every release via `scripts/bench-history.sh`; bench is a hard release gate (build + run-to-completion + exit 0). Resolved at 2.10.1: `affinity/similar_to_5` 809 → 73 µs after `similar_to()` moved to bounded top-k selection; the N² sort is gone. |
| `benchmarks-rust-v-cyrius.md` | 2026-04-12 | 🔵 Dated artifact | Frozen Rust v1.1.0 vs Cyrius v2.0.1 comparison; references cc3 3.7.0 — intentionally historical. |

## Forward commitments

- ✅ ADRs for struct migration (008) and checked-allocation (009) filed (2.5.x).
- Benchmarks are mandatory per release (build+run+exit-0+record) — added to CLAUDE.md § Versioning & Benchmarking after the v2.6.0 bench-include miss.
- Consider splitting volatile state into `docs/development/state.md` if CLAUDE.md version/pin churn grows.
