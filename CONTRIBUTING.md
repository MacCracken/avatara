# Contributing to avatara

avatara is written in **Cyrius**, not Rust. It was ported in v2.0.0
([ADR-006](docs/architecture/adr/006-cyrius-port.md)); there is no `cargo`, no `Makefile` and no
`crates.io` dependency. Everything below is the current toolchain.

## Getting Started

### Prerequisites

The only requirement is the Cyrius toolchain, at the version pinned in `cyrius.cyml`
`[package].cyrius`. **That pin is the single source of truth** — nothing else in the repo hardcodes a
toolchain version, and CI reads it dynamically. Bumping it is a deliberate change, separate from a
package-version bump.

```bash
git clone https://github.com/MacCracken/avatara.git
cd avatara
curl -sSf https://raw.githubusercontent.com/MacCracken/cyrius/main/scripts/install.sh | \
  CYRIUS_VERSION="$(grep '^cyrius = ' cyrius.cyml | head -1 | sed 's/cyrius = "\(.*\)"/\1/')" sh
export PATH="$HOME/.cyrius/bin:$PATH"
```

Confirm you are on the pinned toolchain — `cyrius --version` prints both the installed version and
the manifest pin, and they must agree:

```bash
cyrius --version
```

### Build and test

```bash
cyrius deps
mkdir -p build
CYRIUS_DCE=1 cyrius build src/main.cyr build/avatara
./build/avatara
cyrius test tests/avatara.tcyr
```

`./build/avatara` is a smoke test: it prints the archetype and tradition totals and `all systems
nominal`. `cyrius test` runs the integration suite and must end `0 failed`.

## The Gates

These are exactly what `.github/workflows/ci.yml` enforces. Run them before opening a PR; a green
local run is a green CI run.

| Gate | Command | Enforces |
|------|---------|----------|
| Dependencies | `cyrius deps` | `[deps].stdlib` resolves into `lib/` against the pin |
| Lint | loop below | zero non-cosmetic `warn` lines in `src/*.cyr` and `programs/*.cyr` |
| Build | `CYRIUS_DCE=1 cyrius build src/main.cyr build/avatara` | the library compiles; output is an ELF |
| Examples | `CYRIUS_DCE=1 cyrius build programs/traditions.cyr build/traditions` and the same for `programs/compose.cyr` | **both** examples compile (see [Adding a Tradition](#adding-a-new-tradition)) |
| Tests | `cyrius test tests/avatara.tcyr` | the integration suite ends `0 failed` |
| Benchmarks | `CYRIUS_DCE=1 cyrius build tests/avatara.bcyr build/bench && ./build/bench` | the bench suite builds, runs to completion and exits 0 |
| Dist bundle | `cyrius distlib --check` | `dist/avatara.cyr` matches `src/` |
| Docs | required files present; `VERSION` == `cyrius.cyml`; `VERSION` appears in `CHANGELOG.md` | version consistency |
| Toolchain pin | the `[package].cyrius` pin must match CLAUDE.md's `- **Compiler**:` line and both `Cyrius compiler N+` references in README | prose cannot drift from the pin |

The lint gate is a filtered loop, not a bare `cyrius lint`. Line-length warnings are cosmetic and
excluded; everything else fails:

```bash
fail=0
for f in src/*.cyr programs/*.cyr; do
  out=$(cyrius lint "$f" 2>&1 || true)
  bad=$(echo "$out" | grep -E '^[[:space:]]*warn ' | grep -v 'exceeds 120 characters' || true)
  if [ -n "$bad" ]; then echo "lint: warnings in $f"; echo "$bad"; fail=1; fi
done
[ $fail -eq 0 ] || exit 1
```

For the dist gate, CI regenerates and diffs (`cyrius distlib` then `git diff --exit-code
dist/avatara.cyr`); `cyrius distlib --check` is the equivalent local check that does not write. Either
way, **regenerate and commit `dist/avatara.cyr` whenever `src/` changes** — it is the committed bundle
consumers include, and a stale bundle fails CI.

### What is *not* a gate

- **`cyrius audit`** — do not use it as a pre-PR check. It currently exits non-zero on this repo and
  none of its complaints correspond to a project rule: it fails the `fmt` stage on two files, counts
  the ~1580 cosmetic line-length lint warnings the real gate filters out, and reports undocumented
  public functions against a doc-coverage standard this project does not adopt. Its test and bench
  stages do pass. See [`docs/doc-health.md`](docs/doc-health.md).
- **`cyrius fmt --check`** — not enforced. `src/affinity.cyr`, `src/history.cyr` and
  `tests/avatara.tcyr` do not match canonical continuation indentation today. Do not reformat them as
  a drive-by; it produces a large diff with no reviewable content.
- **`cyrius doc --check`** — not enforced. Module and function comments are expected, but the coverage
  tool is not a gate.
- **`cyrius check`** — useful while iterating, but note it defaults to standalone and will report
  `undefined function 'sakshi_set_level'` on any root that includes `src/logging.cyr`. Use
  `cyrius check --with-deps <root>` for a meaningful answer.
- **`cyrius vet <root>`** — not a gate, but a useful audit of the include graph. Run from the repo
  root it reports `43 deps, 0 untrusted, 1 missing`; the one "missing" is the root file itself,
  which vet resolves relative to the parent directory. Not a defect.

## Code Guidelines

### Cyrius, not Rust

The old Rust-era rules are void. There is no `#[non_exhaustive]`, no `#[must_use]`, no serde and no
clippy in Cyrius — see [ADR-002](docs/architecture/adr/002-non-exhaustive-enums.md), which records its
own supersession. The intent behind those rules survives in Cyrius form:

- **Enums are plain integer constants.** Adding a variant cannot break a downstream `match`, so
  forward-compatibility is achieved by convention: **collection** enums carry a trailing `_COUNT`
  sentinel (`CAN_COUNT = 4`) so callers can iterate them, and consumers must tolerate unknown values.
  The six public *classification* enums — BreathAffinity, GrowthDirection, Element, Polarity,
  CosmicTier, Domain — deliberately carry no sentinel: they are closed vocabularies read off a
  profile, not ranges to walk.
- **No serialization.** Profiles are plain memory; there is no derive to add.
- **Errors are values, not panics.** Fallible public functions return `Result` from `lib/result.cyr` —
  `Ok(x)` / `Err(code)`, inspected with `is_ok` / `is_err_result` / `result_unwrap` / `err_code_of`,
  and propagated with `?`. `AvataraError` codes are the `Err` payload. Loop-hot internal predicates
  (`require_unit_range`, `require_all_unit_range`) stay bare-int deliberately.

### Types and memory

- **All values are i64.** `f64` trait and emphasis weights are stored as IEEE 754 bit patterns; use the
  `f64_*` builtins for arithmetic and comparison, never `==` or `<`.
- **All trait and emphasis values are in 0.0–1.0.** `validate_profile()` enforces it.
- **`ArchetypeProfile` is a native `#derive(accessors)` `struct Profile`** in `src/types.cyr` — 40 i64
  fields, 320 bytes. The compiler-generated `Profile_<field>()` / `Profile_set_<field>()` are canonical
  and are what constructors call. The `prof_*` functions are thin consumer-facing shims. The
  `ProfLayout` offset enum is retained for the loop-based code (compose, affinity, error) and for the
  layout-assertion test — keep the two in step.
- **Allocate through `xalloc(n)`, never raw `alloc`.** `xalloc` is the checked allocator that aborts on
  OOM ([ADR-009](docs/architecture/adr/009-checked-allocation-policy.md), CWE-690).

### Content

- Historically and theologically accurate — real traditions, real correspondences.
- Do not invent theological associations; use established correspondences from scholarly sources, and
  cite them in the module header.
- Do not trivialize or mock any tradition. These are living traditions for billions of people.
- Do not mix traditions without clear compositional semantics.
- Plain f64/enum outputs only — no consumer types (bhava's especially) leak into avatara.
- Zero external dependencies except sakshi (logging).

## Adding a New Tradition

Every `include` root re-lists the source modules by hand; there is no single include graph. A new
module must be added to **all six roots** or the build breaks. Full checklist:

1. **`src/<tradition>.cyr`** — the module. Follow `src/canaanite.cyr` for a short, clean example:
   a `enum` of entity indices ending in `_COUNT`, one `fn <tradition>_<entity>()` per figure building a
   profile via `profile_new()` + `Profile_set_*`, then a lazy-init `all_<tradition>()` collection and
   `<tradition>_count()`. Open with a header comment naming the sources.

2. **The six include roots** — add `include "src/<tradition>.cyr"` to each, in dependency order (after
   anything it references, before `compose`/`registry`):

   - `src/lib.cyr`
   - `src/main.cyr`
   - `tests/avatara.tcyr`
   - `tests/avatara.bcyr`
   - `programs/traditions.cyr`
   - `programs/compose.cyr`

   **The two `programs/` roots are the ones people miss**, and they fail asymmetrically.
   `src/registry.cyr::all_profiles()` calls `all_<tradition>()`, and both programs include the
   registry — so both need the module. `programs/traditions.cyr` calls `all_profiles()` and fails
   loudly with `undefined function 'all_<tradition>'`; `programs/compose.cyr` can still build when DCE
   elides the unreachable call, silently masking the miss. Build both.

3. **`cyrius.cyml` `[lib].modules`** — add `"src/<tradition>.cyr"` in the same order as `src/lib.cyr`.
   `cyrius distlib` reads this section; miss it and the bundle ships without your module.

4. **`src/registry.cyr`** — add an `all_<tradition>()` block to `all_profiles()`.

5. **`src/history.cyr`** — add a `history_map_new(...)` mapping and bump the mapping count in the file
   header. This is not optional: a test walks the mapping table in both directions, so a tradition
   carried with no history context fails, as does a mapping naming a tradition with no profiles.

6. **Count assertions** — `tests/avatara.tcyr` asserts `profile_count()`, `mapping_count()` and a
   per-tradition `<tradition>_count()`. Update all three. If you add a benchmark, `tests/avatara.bcyr`
   states the bench total in a comment and a `println` — keep them in step.

7. **Docs** — `CLAUDE.md` (module list and counts), `README.md` (count line and tradition table),
   `docs/architecture/overview.md` (module list and the diagram's counts), `CHANGELOG.md`, and
   `docs/doc-health.md` (ground-truth line).

Then run the full gate set, and `scripts/bench-history.sh` if you are cutting a release.

### Tradition is the people, not the region

The `tradition` string is a people's own name for itself — `Kunwinjku`, `Lakota`, `Gunaikurnai` — never
a continent and never a pan-ethnic label. Where a wider grouping is genuinely useful it belongs in
`src/history.cyr`'s civilization field, which is many-to-one:
`traditions_for_civilization("Aboriginal Australia")` gathers the peoples without asserting they share
a pantheon. See [ADR-010](docs/architecture/adr/010-named-nation-representation.md).

Note that tradition and module are not one-to-one: `src/incarnate.cyr` spans several traditions and
`src/aboriginal.cyr` carries three peoples.

### Traditions and typologies are mutually exclusive

A **tradition** is a people's own account of who its figures are, and lives in `registry`/`history` as
real archetypes. A **typology** — Enneagram, Jungian, astrology, anything of that kind — is a modern
analytic grid and may only ever exist as an **overlay**: derived over a finished profile in
`src/overlay.cyr`, never instantiated as archetypes, never given a `tradition` string, never counted in
`profile_count()`.

This is enforced, not merely encouraged. A test walks the `OverlaySystem` registry and asserts that no
system name and no label collides with a tradition or an archetype name, and that `profile_count()` is
unchanged by overlays — so a system registered later is covered automatically, without editing the
test. Overlays are plural readings rather than definitions: several may sit over the same figure and
disagree, which is exactly why they derive and store nothing. `src/aspect.cyr` and `src/shadow.cyr`
follow the same rule — they read a finished profile and store nothing.

### Traditions with live community protocols

Adding a tradition that carries **active community protocols** — restricted, secret-sacred,
initiate-only or gender-restricted material — is categorically different from adding Norse or Greek,
and needs a cultural-protocol review of its own, separate from the accuracy pass. Read the header of
`src/aboriginal.cyr` and [ADR-010](docs/architecture/adr/010-named-nation-representation.md) before
starting.

Every profile in this library is a *speakable persona* handed to consumers, and inverted by `shadow()`.
That format cannot carry a figure under a live naming or ceremonial restriction, however carefully the
prose is written. In practice:

- Require material published **by** the people, with express approval, or authored by a member of that
  people. Scholarly quality is not the test; who is speaking is the test.
- Attribute each figure to its people in `desc`, not only in a comment.
- State inclusion **criteria** and demonstrate them per figure. A blanket "nothing restricted is
  included here" is a claim the contents have repeatedly broken, and a header that asserts a limit its
  own entries violate is evidence against the code.
- Do not pad thin material to fill the struct. If a figure's entire published record is a few dozen
  words, a 15-trait profile would be mostly authored here, and that is a reason to refuse.
- Check the publisher's terms. Excellent sourcing that forbids reproduction or modification is still a
  refusal, and must be recorded as a licensing refusal rather than a sourcing one.
- **Record refusals and their reasons in the module header.** "We looked and did not add" is a result
  worth keeping; see the v2.14.0 survey block in `src/aboriginal.cyr`.

## Versioning and Releases

`VERSION` is the single source of truth. `cyrius.cyml` derives from it via the `${file:VERSION}`
template, so the manifest needs no edit.

```bash
scripts/version-bump.sh 2.15.0
```

That writes `VERSION`, updates the version line in `CLAUDE.md`, propagates the `[package].cyrius`
pin into CLAUDE.md's `- **Compiler**:` line and both README `Cyrius compiler N+` references, stubs a
`CHANGELOG.md` section and regenerates `dist/avatara.cyr`. Filling in the CHANGELOG entry is manual, and CI requires the version
to appear there.

**Benchmark every release.** `scripts/bench-history.sh` builds and runs `tests/avatara.bcyr` and
appends the timings to `bench-history.csv`, keyed by date and version. Compare against the previous
release's rows and call out any meaningful slowdown in the CHANGELOG.

```bash
scripts/bench-history.sh
```

Two things to know about that file. It **appends**, so re-running it for a version that already has
rows duplicates them — delete the superseded rows rather than leaving both. And it stores **one sample
per benchmark per release**, which for the sub-microsecond rows is inside run-to-run spread: compare
the µs-scale rows release-over-release and treat the ns-scale rows as a liveness check, not a
measurement.

## Documentation

`docs/doc-health.md` is a living ledger of every doc and its currency, refreshed opportunistically —
when related work touches a doc, not on a timer. If your change makes a doc stale, fix the doc and its
row in the same PR.

The standing rule there is **verify, do not read**: compile every code example, count every count
against source, and check each ADR for decisions the code has since contradicted. A doc that reads
coherently can still be false throughout, and a doc that certifies other docs goes stale twice.

ADRs in `docs/architecture/adr/` are point-in-time records and are **not** rewritten when superseded.
Add a supersession note at the top pointing at the ADR or release that replaced the decision. Correct
outright factual errors in place; that is different from revising a decision.

## Pull Requests

- Keep the diff to the change. Reformatting, doc rewrites and unrelated cleanups belong in their own
  PRs — see the note on `cyrius fmt` above.
- Run the full gate set locally first.
- Update `CHANGELOG.md` under `## [Unreleased]`.
- Regenerate and commit `dist/avatara.cyr` if `src/` changed.

## License

By contributing you agree that your contributions will be licensed under GPL-3.0-only.
