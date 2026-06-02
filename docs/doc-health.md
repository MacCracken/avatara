# Documentation Health

A living ledger of avatara's docs and their currency. Refresh cadence is
**opportunistic** — a doc is touched when related work touches it, not on a
timer. Modeled on cyrius/vidya `doc-health.md`.

Buckets: ✅ Fresh · 🟡 Stale (needs a pass) · 🔵 Dated artifact (frozen on purpose) · ❓ Open question

_Last swept: 2026-06-02 (v2.4.4 — 2.4.x closeout)._

## Structural docs

| Doc | Last touched | Status | Notes |
|-----|--------------|--------|-------|
| `README.md` | 2026-06-02 | ✅ Fresh | Build commands + compiler pin refreshed to 6.0.40 at 2.4.3. |
| `CHANGELOG.md` | 2026-06-02 | ✅ Fresh | Entries through 2.4.4; `[Unreleased]` points at the re-bucketed roadmap. |
| `CLAUDE.md` | 2026-06-02 | ✅ Fresh | Version 2.4.4, compiler 6.0.40, Versioning & Benchmarking section current. |
| `CONTRIBUTING.md` | — | ✅ Fresh | No version-specific content. |
| `SECURITY.md` | — | ✅ Fresh | — |
| `cyrius.cyml` | 2026-06-02 | ✅ Fresh | Pin 6.0.40; `${file:VERSION}` template; `[lib]` distlib section. |

## Development

| Doc | Last touched | Status | Notes |
|-----|--------------|--------|-------|
| `docs/development/roadmap.md` | 2026-06-02 | ✅ Fresh | Re-bucketed at 2.4.4: 2.5.0 architecture items added; Structural Enrichment slipped from 2.4.0 → 2.5.x; Solar Year → 2.6.0. |
| `docs/development/state.md` | — | ❓ Open | Not yet split out. Volatile state (cyrius pin, consumer status) currently lives in CLAUDE.md + the Dependencies table in roadmap.md. Adopt vidya's `state.md` split if CLAUDE.md churn warrants. |

## Architecture

| Doc | Last touched | Status | Notes |
|-----|--------------|--------|-------|
| `docs/architecture/overview.md` | 2026-06-02 | ✅ Fresh | Verified at 2.4.4: 312-byte layout, "27 maps", data flow, and accessor pattern all match current source. |
| `docs/architecture/adr/001–007` | — | ✅ Fresh | Decision records are point-in-time; no refresh needed. A future ADR should cover the planned 2.5.0 struct/`#derive` migration when it lands. |

## Benchmarks

| Doc | Last touched | Status | Notes |
|-----|--------------|--------|-------|
| `bench-history.csv` | 2026-06-02 | ✅ Fresh | Rows for 2.4.0–2.4.4. Net 2.4.0→2.4.3: meaningful (≥500ns) paths −16% to −29%, no regressions. Recorded every release via `scripts/bench-history.sh` (see CLAUDE.md § Versioning & Benchmarking). |
| `benchmarks-rust-v-cyrius.md` | 2026-04-12 | 🔵 Dated artifact | Frozen Rust v1.1.0 vs Cyrius v2.0.1 comparison. Methodology line references cc3 3.7.0 — intentionally not updated (historical record). |

## Forward commitments

- When the v2.5.0 struct/`#derive` migration lands, add an ADR and re-sweep `architecture/overview.md`.
- Consider splitting volatile state into `docs/development/state.md` (vidya pattern) if CLAUDE.md version/pin churn becomes noisy.
