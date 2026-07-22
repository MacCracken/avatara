# Documentation Health

A living ledger of avatara's docs and their currency. Refresh cadence is
**opportunistic** — a doc is touched when related work touches it, not on a
timer. Modeled on cyrius/vidya `doc-health.md`.

Buckets: ✅ Fresh · 🟡 Stale (needs a pass) · 🔵 Dated artifact (frozen on purpose) · ❓ Open question

_Last swept: 2026-07-22 (v2.14.0 — full claim-by-claim verification against source, not a read-through)._

Ground truth at this sweep: **504 archetypes, 37 traditions, 32 tradition modules**;
**37 history mappings**; version **2.14.0**; cyrius pin **6.4.71**; 295 integration
tests; 60 benchmarks; `ArchetypeProfile` = 320-byte `#derive` struct.

## How this sweep was run, and why the method changed

Previous sweeps marked docs fresh by reading them. This one checked every factual
claim against the source and **found 25 stale statements across five documents that
had all been marked ✅ Fresh**, including in this ledger. Reading a doc tells you
whether it is coherent, not whether it is true.

Two findings are worth carrying forward as method:

- **A doc that certifies other docs goes stale twice** — once on its own content and
  once on its certifications. This file claimed `overview.md` was fresh while that
  file said the library had 27 history mappings (it had 35 at the time, 37 now), and
  claimed the ADRs carried "no present-tense claims gone stale" while ADR-004 point 6
  stated the library does not codify Aboriginal Australian or Native American
  traditions — three releases after it began doing exactly that.
- **Executable claims should be executed.** The README's affinity example did not
  compile (`match` is a reserved Cyrius keyword and the example used it as a variable
  name), and its `detect_conflicts` output comment listed three conflicts where the
  code returns two — the third, humor, has a delta of exactly 0.4 and the predicate is
  strictly greater. Neither is findable by reading; both fell out of compiling and
  running the snippets. Every README example is now verified by compilation.

## Structural docs

| Doc | Last touched | Status | Notes |
|-----|--------------|--------|-------|
| `README.md` | 2026-07-22 | ✅ Fresh | 504/37, compiler 6.4.71. Tradition table now states that tradition and module are not one-to-one (`incarnate` spans six traditions; `aboriginal` carries three peoples) — the Incarnate row previously named a tradition string that does not exist. New "Derived Layers" section covers `shadow()` and overlays incl. the generic `overlay_*` registry API, both of which were entirely undocumented here. **All code examples verified by compiling and running them**; two were broken (see above). Version-stamped heading removed — it drifts by construction. |
| `CHANGELOG.md` | 2026-07-22 | ✅ Fresh | Entries through 2.14.0 (per-people Aboriginal split; `history.cyr` bidirectional integrity fix; six recorded refusals). `[Unreleased]` points at the roadmap. |
| `CLAUDE.md` | 2026-07-22 | ✅ Fresh | Version 2.14.0, compiler 6.4.71; main.cyr = 6 smoke checks (was claimed as "~10") / tcyr=295 / bcyr=60. `src/aspect.cyr` added to the module list — it had been absent since 2.8.0 shipped it. Aboriginal line rewritten for the per-people split. |
| `CONTRIBUTING.md` / `SECURITY.md` / `CODE_OF_CONDUCT.md` | — | ✅ Fresh | Verified to carry no version, count or API content, so nothing here can go stale. |
| `cyrius.cyml` | 2026-07-22 | ✅ Fresh | Pin 6.4.71. `[lib]` list verified in both directions against `src/*.cyr` — no module missing, no ghost entry. |

## Development

| Doc | Last touched | Status | Notes |
|-----|--------------|--------|-------|
| `docs/development/roadmap.md` | 2026-07-22 | ✅ Fresh | **Forward-looking only, as its own header always promised** — the Shipped section and the closed/resolved blocks were removed (158 → 87 lines) once verified duplicated in CHANGELOG.md, `src/aboriginal.cyr` and ADR-010. Nothing scheduled before v3.0.0. The Aboriginal arc is retained only as a *blocked* item, reduced to the one thing that can move it: five named bodies nobody has written to. `incarnate_indigenous_*` renaming moved into the v3.0.0 list where it belongs. |
| `docs/development/state.md` | — | ❓ Open | Not split out; volatile state lives in CLAUDE.md + roadmap. Adopt vidya's `state.md` split only if CLAUDE.md churn warrants. |

## Architecture

| Doc | Last touched | Status | Notes |
|-----|--------------|--------|-------|
| `docs/architecture/overview.md` | 2026-07-22 | ✅ Fresh | Was the worst-drifted doc despite being marked fresh: claimed 34 tradition modules (32), 27 history maps (37), 6 incarnate sub-traditions (9), and omitted `domain` from the profile box and `aspect`/`shadow`/`overlay` from the diagram entirely. Diagram now carries a derived-layer box; data flow and `query_*` list corrected. |
| `docs/architecture/adr/001–009` | 2026-07-22 | ✅ Fresh | Point-in-time records, **not** to be rewritten when superseded — but four were contradicted by shipped code with no note saying so, which leaves a reader no way to know. 001/002/003 now carry supersession notes pointing at the Cyrius port (006) and struct migration (008); 003's note states explicitly that field completeness is no longer compiler-verified, which it had claimed as a benefit. 004 point 6 is superseded by the new 010. 005 had two examples backwards against the shipped data (Tiamat is Unity, not EarlyExhale; Ahura Mazda is EarlyExhale, not Unity) and described the exhale/inhale asymmetry the wrong way round — both fixed as factual errors rather than superseded decisions. |
| `docs/architecture/adr/010` | 2026-07-22 | ✅ Fresh | **New.** Named-nation representation of Indigenous traditions, superseding ADR-004 point 6. Records the nine rules that replaced the blanket exclusion, including the v2.14.0 finding that channel ownership and text provenance are separate checks. |

## Benchmarks

| Doc | Last touched | Status | Notes |
|-----|--------------|--------|-------|
| `bench-history.csv` | 2026-07-22 | ✅ Fresh | Rows for 2.4.0–2.14.0 (49/release through 2.8.1; 50 from 2.9.0 with `tarot/all_22`; 52 from 2.10.0; 57 from 2.11.0; 60 from 2.13.0 adding the three `overlay/*` benches, unchanged at 2.14.0; 2.8.0 skipped bench-recording, restored at 2.8.1). One row set per version — the pre-final 2.13.1 rows were replaced rather than appended. Bench is a hard release gate (build + run-to-completion + exit 0). Resolved at 2.10.1: `affinity/similar_to_5` 809 → 73 µs after `similar_to()` moved to bounded top-k. The `aboriginal/all_5` label counted five figures against a module of four from 2.11.0 until 2.14.0. |
| `benchmarks-rust-v-cyrius.md` | 2026-04-12 | 🔵 Dated artifact | Frozen Rust v1.1.0 vs Cyrius v2.0.1 comparison; references cc3 3.7.0 — intentionally historical. |

## Forward commitments

- ✅ ADRs for struct migration (008), checked-allocation (009) and named-nation representation (010) filed.
- Benchmarks are mandatory per release (build+run+exit-0+record) — added to CLAUDE.md § Versioning & Benchmarking after the v2.6.0 bench-include miss.
- **Verify, do not read.** The next sweep should re-run the same checks rather than trusting these ✅s: compile every code example, count every count against source, and check each ADR for decisions the code has since contradicted. Every finding above was invisible to a careful read of the prose.
- `cyrius audit` is **not** a usable gate for this project — its test stage compiles `tests/avatara.tcyr` standalone and fails on `SYS_WRITE`, and it looks for benches in `benches/`. The real gates are `cyrius tests`, `cyrius deps`, the CI lint loop over `src/*.cyr` and `programs/*.cyr`, and the dist-bundle staleness check.
- Consider splitting volatile state into `docs/development/state.md` if CLAUDE.md version/pin churn grows.
