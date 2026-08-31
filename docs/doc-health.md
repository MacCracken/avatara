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
| `README.md` | 2026-07-22 | ✅ Fresh | 504/37, compiler 6.4.71. Tradition table now states that tradition and module are not one-to-one (`incarnate` spans nine traditions; `aboriginal` carries three peoples) — the Incarnate row previously named a tradition string that does not exist. New "Derived Layers" section covers `shadow()` and overlays incl. the generic `overlay_*` registry API, both of which were entirely undocumented here. **All code examples verified by compiling and running them**; two were broken (see above). Version-stamped heading removed — it drifts by construction. |
| `CHANGELOG.md` | 2026-07-22 | ✅ Fresh | Entries through 2.14.0 (per-people Aboriginal split; `history.cyr` bidirectional integrity fix; six recorded refusals). `[Unreleased]` points at the roadmap. |
| `CLAUDE.md` | 2026-07-22 | ✅ Fresh | Version 2.14.0, compiler 6.4.71; main.cyr = 6 smoke checks (was claimed as "~10") / tcyr=295 / bcyr=60. `src/aspect.cyr` added to the module list — it had been absent since 2.8.0 shipped it. Aboriginal line rewritten for the per-people split. |
| `CONTRIBUTING.md` | 2026-08-30 | ✅ Fresh | **Rewritten — it had never been ported at v2.0.0 and every instruction in it was wrong.** It told contributors to run `cargo test`, `cargo clippy`, `cargo bench` and `make check` (none exist here); required `#[non_exhaustive]`, `#[must_use]`, serde derives and zero clippy warnings, three of which ADR-002 already records as superseded by the Cyrius port; and gave an "adding a tradition" procedure naming `src/<tradition>.rs`, an `Archetype` trait and `lib.rs`. Now documents the real toolchain, the eight CI gates, what is deliberately *not* a gate, the six include roots, and the cultural-protocol rules from ADR-010. **Every command in it was executed against the working tree**, not transcribed from `ci.yml` — that is how the `cyrius audit` note below was found to be stale. |
| `SECURITY.md` | 2026-08-30 | ✅ Fresh | The Supported Versions table named only the retired 1.0.x Rust line and had no 2.x row at all, so a reader was told that nothing currently shipping is supported — while the project was on 2.14.2. Now tracks the current 2.x minor, records 1.0.x as retired at the Cyrius port, and adds a Scope section (no I/O, `xalloc` abort-on-OOM per ADR-009, convention-enforced memory safety per ADR-006, `dist/` regenerated not patched). **Carries version content by construction — revisit on every minor bump.** |
| `CODE_OF_CONDUCT.md` | — | ✅ Fresh | Contributor Covenant v2.1 link and a contact address. No project version, count or API content. |
| `cyrius.cyml` | 2026-07-22 | ✅ Fresh | Pin 6.4.71. `[lib]` list verified in both directions against `src/*.cyr` — no module missing, no ghost entry. |

## Development

| Doc | Last touched | Status | Notes |
|-----|--------------|--------|-------|
| `docs/development/roadmap.md` | 2026-07-22 | ✅ Fresh | **Forward-looking only, as its own header always promised** — the Shipped section and the closed/resolved blocks were removed (158 → 87 lines) once verified duplicated in CHANGELOG.md, `src/aboriginal.cyr` and ADR-010. Nothing scheduled before v3.0.0. The Aboriginal arc is retained only as a *blocked* item, reduced to the one thing that can move it: five named bodies nobody has written to. `incarnate_indigenous_*` renaming moved into the v3.0.0 list where it belongs. |
| `docs/development/sourcing-register.md` | 2026-08-31 | ✅ Fresh | **New at 2.14.7.** Per-figure sourcing provenance, verbatim reuse terms and consent status for the four shipped Aboriginal figures and the six refused, plus the request-for-information plan. Built from existing findings rather than new research — the knowledge was real but scattered across `src/aboriginal.cyr` and three CHANGELOG entries, so each pass rediscovered it. Makes two things visible that prose did not: not one shipped figure rests on anything resembling a licence, and the module refuses to *add* under terms it continues to *ship* under. 38 figures in the other four high-protocol modules are not yet entered. |
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
| `bench-history.csv` | 2026-08-31 | ✅ Fresh | Rows for 2.4.0–2.14.6 (49/release through 2.8.1; 50 from 2.9.0 with `tarot/all_22`; 52 from 2.10.0; 57 from 2.11.0; 60 from 2.13.0 adding the three `overlay/*` benches, unchanged at 2.14.0; 2.8.0 skipped bench-recording, restored at 2.8.1). One row set per version — the pre-final 2.13.1 rows were replaced rather than appended. Bench is a hard release gate (build + run-to-completion + exit 0). Resolved at 2.10.1: `affinity/similar_to_5` 809 → 73 µs after `similar_to()` moved to bounded top-k. The `aboriginal/all_5` label counted five figures against a module of four from 2.11.0 until 2.14.0. **2.14.1 recorded no rows** — that release bumped the pin 6.4.71 -> 6.5.27 and skipped the mandatory bench step, so 2.14.2's only baseline is 2.14.0; same class as the 2.8.0 skip, and not back-fillable. **Recorded at 2.14.2: the script stores one sample per benchmark per release**, which for the sub-microsecond rows is inside run-to-run spread. The 2.14.0 -> 2.14.2 table appeared to show a +30% `history/traditions_for_civ` regression; it is a layout swap with its near-twin `history/traditions_for_era` (their sum moved 8024 -> 8049 ns) and the whole-suite total moved 247528 -> 247040 ns. Compare the µs-scale rows release-over-release; treat the ns-scale rows as a liveness check, not a measurement. At 2.14.3 that method earned its keep in both directions: it confirmed `registry/by_tradition` and `history/query_civilization` as REAL ~4% regressions from the safe_streq hardening (the prior sample fell outside the whole observed spread) while clearing `registry/query_courage_0.9`, which moved as far by the raw numbers but landed inside it. At 2.14.6 it separated a third case: `history/context_all_traditions` +11.3% is neither noise nor regression but **workload growth** — the benchmark is O(traditions x mappings) and both went 39 -> 41, predicting +10.5%. It is the one row that grows quadratically with tradition count. |
| `benchmarks-rust-v-cyrius.md` | 2026-04-12 | 🔵 Dated artifact | Frozen Rust v1.1.0 vs Cyrius v2.0.1 comparison; references cc3 3.7.0 — intentionally historical. |

## Forward commitments

- ✅ ADRs for struct migration (008), checked-allocation (009) and named-nation representation (010) filed.
- Benchmarks are mandatory per release (build+run+exit-0+record) — added to CLAUDE.md § Versioning & Benchmarking after the v2.6.0 bench-include miss.
- **Verify, do not read.** The next sweep should re-run the same checks rather than trusting these ✅s: compile every code example, count every count against source, and check each ADR for decisions the code has since contradicted. Every finding above was invisible to a careful read of the prose.
- `cyrius audit` is **not** a usable gate for this project. _Reason updated 2026-08-30 against pin 6.5.36 — the reason previously recorded here had itself gone stale._ The old grounds (test stage compiling `tests/avatara.tcyr` standalone and failing on `SYS_WRITE`; benches looked for in `benches/`) no longer hold: both stages now pass, 295/295 and 60 benches. It still exits 1, on three things that are not project rules — the `fmt` stage over `src/history.cyr` and `src/affinity.cyr`, a raw count of 1570 lint warnings (verified: **all 1570 are the cosmetic `exceeds 120 characters` warning** the real gate filters out, 0 of any other kind), and 454 undocumented public fns against a doc-coverage standard this project has not adopted. The real gates are the eight in `ci.yml`: `cyrius deps`, the filtered lint loop over `src/*.cyr` and `programs/*.cyr`, the DCE build + ELF check, **both** `programs/*.cyr` builds, `cyrius test tests/avatara.tcyr`, the bench build-and-run, the dist-bundle staleness check, and the docs job (required files + `VERSION`/`cyrius.cyml`/CHANGELOG consistency). All are listed with their exact commands in `CONTRIBUTING.md`.
- **"Nothing here can go stale" is a claim, not an exemption — and it was wrong.** The 2026-07-22 row certifying `CONTRIBUTING.md` and `SECURITY.md` as carrying no version, count or API content was false for both: `CONTRIBUTING.md` was a page of Rust build commands for a project with no Rust in it, and `SECURITY.md`'s support table omitted the entire shipping major. Both had been marked ✅ Fresh without being read against the tree. A doc exempted from checking is a doc that drifts unobserved; no row gets that exemption again.
- Consider splitting volatile state into `docs/development/state.md` if CLAUDE.md version/pin churn grows.
