# Documentation Health

A living ledger of avatara's docs and their currency. Refresh cadence is
**opportunistic** — a doc is touched when related work touches it, not on a
timer. Modeled on cyrius/vidya `doc-health.md`.

Buckets: ✅ Fresh · 🟡 Stale (needs a pass) · 🔵 Dated artifact (frozen on purpose) · ❓ Open question

_Last swept: 2026-08-31 (v2.14.8 — full claim-by-claim verification by execution, five independent
passes; supersedes the 2026-07-22 / v2.14.0 sweep)._

Ground truth at this sweep, every value executed rather than read: **503 archetypes, 41 traditions,
32 tradition modules**; **41 history mappings**; version **2.14.8**; cyrius pin **6.5.36**; **368**
integration assertions; **60** benchmarks; **3** overlay systems; `ArchetypeProfile` = 320-byte
`#derive` struct.

## Method

**Verify, do not read.** Reading a doc tells you whether it is coherent, not whether it is true. Every
count is executed against source, every code example compiled and run, every gate re-run.

Three findings worth carrying forward as method:

- **A doc that certifies other docs goes stale twice** — once on its own content and once on its
  certifications. Established at the v2.14.0 sweep and confirmed again here: this ledger was the single
  most stale document in the repository, and it was the one asserting everything else was fresh.
- **Executable claims must be executed.** The README's Quick Start tells consumers to
  `include "avatara/dist/avatara.cyr"`. That claim is *only* testable from outside the repository —
  compiling from the repo root passes either way. This sweep built a consumer harness in a separate
  directory and confirmed it, along with every commented expected output in every example.
- **A dated stamp is not a licence to be wrong.** The previous sweep's ground-truth block was defended
  once as a correctly-scoped point-in-time record. That defence failed here: the file's own body had
  since gained rows dated 2026-08-30 and 2026-08-31, so the "last swept 2026-07-22" stamp was
  contradicted by its own contents. A ledger that is partially updated is not dated — it is wrong.

## Structural docs

| Doc | Last touched | Status | Notes |
|-----|--------------|--------|-------|
| `README.md` | 2026-08-31 | ✅ Fresh | 503/41, compiler 6.5.36. **Every code example compiled and run from a directory that is not the repo root**, against `dist/avatara.cyr`, and every commented output matched — including `detect_conflicts` returning exactly 2 (humor's delta is exactly 0.4 and the predicate is strictly greater). Tradition table diffed row by row against `by_tradition()` output; the Entities column sums to exactly 503, and the module-vs-tradition differences are all accounted for by the `incarnate` row. Fixed this sweep: Derived Layers named two overlay systems where there are three, `similar_to` was shown as if it returned profiles rather than sim entries, the Māori row named "Tūmatauenga" which the registry does not carry, Domain was missing from the Design list, and the "maps 1:1 to bhava" claim is 14 of 15 by name. |
| `CLAUDE.md` | 2026-08-31 | ✅ Fresh | Version 2.14.8, compiler 6.5.36, tcyr=368, bcyr=60, all per-module counts verified against source. Fixed this sweep: it never mentioned `safe_strlen`/`safe_streq`, which are a mandatory convention exactly like `xalloc` and the reason seven functions stopped segfaulting at 2.14.3; `validate_profile`'s contract (ranges, not string presence) was stated only in `src/error.cyr`; the overlay bullet omitted `profile_mystic_*`. |
| `CHANGELOG.md` | 2026-08-31 | ✅ Fresh | Entries 2.14.0–2.14.8, each matching a git tag. Supersession markers now applied consistently wherever a later release overturned an earlier claim — the 2.14.0 GLaWAC bullet (superseded 2.14.5) and the 2.14.5 TLaWC bullet (superseded 2.14.7). Corrected "four living religions" to three. |
| `CONTRIBUTING.md` | 2026-08-31 | ✅ Fresh | Ported from Rust and now verified **by running every command it gives a contributor**. Fixed this sweep: the gate table omitted ci.yml's toolchain-pin step while claiming to be exactly what CI enforces; the `cyrius vet` output did not reproduce (the one "missing" dep is the root file itself); the lint count was 1570, now ~1580; and "every enum carries a trailing `_COUNT` sentinel" was false — 15 of 67 have none, including all six public classification enums. |
| `SECURITY.md` | 2026-08-31 | ✅ Fresh | Supported-versions table correct at 2.14.x. Fixed this sweep: the `xalloc` guarantee overclaimed — it covers avatara's own allocations, not the vendored stdlib's — and the NULL-string class fixed at 2.14.3 was undocumented. |
| `CODE_OF_CONDUCT.md` | — | ✅ Fresh | Carries no version, count or API content. Stated as a finding of this sweep, not as an exemption: the previous ledger's "nothing here can go stale" phrasing was applied to `CONTRIBUTING.md` and was false. |
| `cyrius.cyml` | 2026-08-31 | ✅ Fresh | Pin 6.5.36. `[lib]` verified in both directions and in order against `src/*.cyr` — 43 entries, no ghost, no omission, `main.cyr` correctly excluded as the build entry. |
| `src/lib.cyr` header | 2026-08-31 | ✅ Fresh | **Tracked here because it ships.** `cyrius.cyml` bundles it as the last module of `dist/avatara.cyr`, so it is API documentation consumers read. It stated 497/34 until 2.14.3 and omitted ten modules. Now 503/41 with a complete module list, checked against the file's own `include` lines. |
| `src/main.cyr` header | 2026-08-31 | ✅ Fresh | Same class, same fix: it read "34 traditions, 497 archetypes" while the binary it builds printed 503/41 three lines later. Stale since at least 2.11.0 and missed by the 2.14.3 pass, which corrected `lib.cyr` only. |

## Development

| Doc | Last touched | Status | Notes |
|-----|--------------|--------|-------|
| `docs/development/roadmap.md` | 2026-08-31 | ✅ Fresh | Rewritten forward-facing only. Removed: "v2.14.0 was the last planned minor" (seven releases stale), the Aboriginal historical narrative, the six-body table duplicating the sourcing register, and **both version-pinned consumer notes** — one of which had already rotted, stating `all_traditions()` = 39 when 2.14.6 made it 41. Consumer notes now live only in the CHANGELOG, where they cannot drift. |
| `docs/development/sourcing-register.md` | 2026-08-31 | ✅ Fresh | **New at 2.14.7.** Per-figure provenance, verbatim reuse terms and consent status for the four shipped Aboriginal figures and the six refused, plus the request-for-information plan. Makes visible what prose hid: no shipped figure rests on anything resembling a licence, and the module refuses to *add* under terms it continues to *ship* under. 38 figures in the other four high-protocol modules remain unentered. |
| `docs/development/state.md` | — | ❓ Open | Not split out; volatile state lives in CLAUDE.md + roadmap. Adopt vidya's `state.md` split only if CLAUDE.md churn warrants. |

## Architecture

| Doc | Last touched | Status | Notes |
|-----|--------------|--------|-------|
| `docs/architecture/overview.md` | 2026-08-31 | ✅ Fresh | Worst-drifted doc at the previous sweep, and again at this one — the pattern is that diagrams are edited less often than prose. Fixed: the history box read 37 maps (actual 41), the `incarnate` breakdown silently omitted Hindu (13) and Vedic (7), i.e. 20 of 56 figures including the largest group, and the derived-layers line named two overlay systems where there are three. |
| `docs/architecture/adr/001–009` | 2026-08-31 | ✅ Fresh | Point-in-time records, **not** rewritten when superseded — but checked for decisions the code has since contradicted with no note. ADR-004 point 3 (the Buddha marked as a Vaishnava claim) became simply **true** at 2.14.4 and needs no note; point 7 gained an extension note for the 2.14.6 living-persons rule; ADR-002's "every enum carries a `_COUNT` sentinel" was never true of the classification enums the ADR is about and now carries a correction. |
| `docs/architecture/adr/010` | 2026-08-31 | ✅ Fresh | Named-nation representation, heavily exercised across 2.14.5–2.14.7. Two scope notes added rather than changes: rule 8 is a research standard, not a ship gate — all 42 high-protocol figures ship with consent `never-asked` — and rule 9's "modules say so in their headers" is currently true of two modules, not six. |

## Benchmarks

| Doc | Last touched | Status | Notes |
|-----|--------------|--------|-------|
| `bench-history.csv` | 2026-08-31 | ✅ Fresh | Rows for 2.4.0–2.14.8, one set per version, 60 from 2.13.0 onward. **Label renames break per-benchmark continuity and are recorded rather than hidden:** `aboriginal/all_5`→`all_4` (2.14.0), `celtic/all_15`→`all_17` and `incarnate/all_51`→`all_56` (2.14.7), `slavic/all_12`→`all_11` (2.14.8, after the Morana merge). Old series end, new ones begin; nothing is rewritten. 2.14.1 recorded no rows at all. **Method, established 2.14.2 and refined since:** the script records ONE sample per benchmark per release, so ns-scale rows are noise — compare the µs-scale rows and the suite total. It has now separated three cases: real regression (`by_tradition` +4% from the safe_streq hardening), noise (`query_courage_0.9` moving as far but inside its spread), and **workload growth** (`history/context_all_traditions` +11.3%, where the benchmark is O(traditions × mappings) and both went 39→41, predicting +10.5%). |
| `benchmarks-rust-v-cyrius.md` | 2026-04-12 | 🔵 Dated artifact | Frozen Rust v1.1.0 vs Cyrius v2.0.1 comparison; references cc3 3.7.0 — intentionally historical. |

## Forward commitments

- **Verify, do not read.** The next sweep should re-run these checks rather than trusting these ✅s.
  Every finding above was invisible to a careful read of the prose.
- **Compile the README from outside the repo.** The Quick Start's consumer include is not testable
  any other way.
- Benchmarks are mandatory per release (build + run + exit 0 + record).
- `cyrius audit` is **not** a usable gate for this project — its `fmt` stage fails on two files, it
  counts the ~1580 cosmetic line-length warnings the real gate filters out, and it reports undocumented
  public functions against a standard this project does not adopt. Its test and bench stages do pass.
  The real gates are `cyrius deps`, the CI lint loop, `cyrius test`, the bench build+run, the
  dist-staleness check, and the version and toolchain-pin consistency steps.
- **Enter the remaining 38 figures in the sourcing register** — `inuit`, `lakota`, `haudenosaunee`,
  `anishinaabe`. Their sources were checked at v2.14.5 and found sound; it was never written down, so
  each pass re-derives it. This needs no community engagement and is the one blocked item desk work
  can close.
