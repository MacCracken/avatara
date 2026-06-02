# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Roadmap
See [docs/development/roadmap.md](docs/development/roadmap.md). Next up:
- **v2.5.0 — Architecture Modernization**: `struct` + `#derive(accessors)` for `ArchetypeProfile`, then `Result<T, E>` + `?` for `error.cyr` (both deferred from the 2.4.x sweep — layout/API-breaking; do the struct migration first).
- **v2.5.1 — `domain` field** (slipped from 2.4.0): categorical axis; layout 312 → 320, folded into the struct migration.
- **v2.5.2 — Cross-tradition affinity graph**: pre-computed/cached, additive, must beat the 2.4.x bench baseline.
- **v2.5.3 — Shadow aspect support**: pure `shadow(profile)` derivation with defined inversion semantics.
- **v2.6.0 — The Solar Year** (362 → 365.25 archetypes).

## [2.4.5] — 2026-06-02

Correctness fix, surfaced while scoping the 2.5.0 struct migration. **Behavior-changing** (affinity/compose outputs shift) — isolated in its own release so it's bisectable before the migration.

### Fixed
- **`PROF_SPIRIT` was defined twice** in `ArchetypeProfile`'s layout — offset 176 (spirit *emphasis*, f64) and offset 304 (spirit *text*, ptr). "Last definition wins" collapsed both to 304, so:
  - every archetype's **spirit emphasis was silently stuck at the 0.5 default** — the 316 constructor writes of real emphasis values (e.g. Chokmah's 0.9) landed on offset 304 and were then overwritten by the spirit-text pointer;
  - **`prof_spirit_emph()` returned garbage** (it read the text pointer reinterpreted as an f64).
- Renamed the text constant to `PROF_SPIRIT_TEXT` (offset 304); `PROF_SPIRIT` is now solely offset 176. Repointed the 362 spirit-text constructor writes + the text accessor + `compose()`'s spirit copy + `profile_new()`'s text init at `PROF_SPIRIT_TEXT`; the 316 numeric emphasis writes now correctly land on 176.
- Net effect: the spirit-emphasis dimension is now live (was inert at 0.5 for all 362 archetypes), so `affinity`, `similar_to`, `cross_tradition_match`, `compose`, and `detect_conflicts` now factor in real spirit-emphasis values. `prof_spirit_emph()` returns the real f64.

### Notes
- Layout unchanged (still 312 bytes, same offsets) — only the constant *name* for 304 changed and the emphasis slot at 176 now actually populates. Consumers including `dist/avatara.cyr` pick up both the data fix and the working `prof_spirit_emph()`.

## [2.4.4] — 2026-06-02

2.4.x closeout — documentation + roadmap housekeeping. No source changes.

### Changed
- **Roadmap re-bucketed** (`docs/development/roadmap.md`): added **v2.5.0 — Architecture Modernization** (the deferred `struct` + `#derive(accessors)` migration and `Result<T, E>` error model); slipped **Structural Enrichment** (domain field, affinity graph, shadow aspect) from 2.4.0 → **v2.5.x**; slipped **The Solar Year** to **v2.6.0**. `CHANGELOG [Unreleased]` updated to match.

### Added
- **`docs/doc-health.md`** — living doc-currency ledger (cyrius/vidya convention).

### Notes
- **2.4.x benchmark summary** (net 2.4.0 → 2.4.3, recorded in `bench-history.csv`): every meaningful (≥500ns) path got faster, no regressions — history queries −16% to −25%, `registry/by_tradition` −23%, `registry/query_courage` −29% (from the `query_by_traditions` refactor + the 6.0.40 toolchain). Sub-500ns rows are timer-noise dominated.
- Closes the 2.4.x line: cyrius 3.10.0 → 6.0.40, NULL/overflow hardening, `+=`/`match` modernization, stdlib `f64_le`/`f64_ge` alignment.

## [2.4.3] — 2026-06-02

Toolchain bump + stdlib alignment.

### Changed
- **Cyrius pin 6.0.38 → 6.0.40.** `f64_le`/`f64_ge` landed in stdlib `lib/math.cyr` at 6.0.39 (from the proposal filed during the 2.4.x sweep); 6.0.40 is the current public release and is pinned here.
- **Dropped avatara's local `f64_le`/`f64_ge`** (`src/types.cyr`) — now supplied by stdlib. Keeping them would have collided with the new stdlib definitions (duplicate-fn, last-wins) — the exact shadowing risk the proposal called out.

### Consumer note
- `dist/avatara.cyr` no longer carries `f64_le`/`f64_ge`. Consumers that include the bundle must build on cyrius **6.0.39+** and have `"math"` in their `[deps] stdlib` so the stdlib definitions resolve. (avatara already lists `math`.)

### Notes
- Evaluated the optional `#regalloc` perf hint on the hot `affinity`/`compose` loops and **declined it**: the directive is unused across the cyrius stdlib and reference projects (vidya), the hot path (`affinity` score = 92ns) is already tight, and run-to-run bench variance exceeds any plausible gain. Not worth an ecosystem-unique tuning knob in the source.

## [2.4.2] — 2026-06-02

Language modernization — adopting Cyrius 6.x idioms. No behavior or API
changes; codegen is identical (verified: build, 39 tests, lint, benchmarks).

### Changed
- **Compound assignment** — converted 91 integer accumulator/loop sites (`i = i + 1`, `off = off + 8`, `j = j - 1`, etc.) to `+=` / `-=` across the logic modules (types, error, compose, affinity, registry, history, incarnate). The f64 accumulators stay on the `f64_add` builtins (bit-pattern math — `+=` would be wrong there).
- **`breath_intensity()`** now uses a `match` over `BreathAffinity` instead of an if-chain, gaining a compile-time exhaustiveness check on the enum.

## [2.4.1] — 2026-06-02

Hardening + refactor sweep. No API removals; behavior changes are limited to
rejecting invalid input that previously caused undefined behavior.

### Fixed
- **NULL-profile guards** (`affinity`, `trait_affinity`, `compose`, `conflicts`/`detect_conflicts`, `similar_to`, `cross_tradition_match`, `cross_tradition_matches`) — a `0` returned by `lookup`/`lookup_in`/`cross_tradition_match` on a miss no longer flows into `load64(p + off)` and dereferences near-NULL; these now return a `0`/empty sentinel.
- **`compose()` buffer overflow** — the composite name/tradition buffers were fixed at 512/256 bytes and written unchecked; composing many archetypes (e.g. all 24 traditions) overran them and corrupted the bump heap. Buffers are now sized exactly from the input.
- **`compose()` rejects NULL profiles and NaN weights** (in addition to the existing empty-vec / negative-weight / zero-total guards).
- **Out-of-range query offsets** — `query_min_trait`/`query_max_trait`/`query_count_min_trait`/`filter_min_trait` now bounds-check the caller-supplied `offset` (via `is_f64_field_offset`) and return empty instead of reading outside the 312-byte profile.

### Changed
- **`all_incarnate()`** rebuilt from the six cached subgroup collections instead of re-listing and re-constructing all 56 incarnate profiles — removes a 56-profile double-allocation (~50 LOC).
- **`profile_new()`** default-fills the 29 contiguous trait/emphasis fields with a loop over `TW_FIRST..ME_LAST` instead of 29 literal stores.
- **`affinity`/`trait_affinity`** share a `sum_abs_diff(a, b, first, last)` helper.
- The three history queries (`query_civilization`/`query_era`/`query_active_at`) delegate to a shared `query_by_traditions(trads)`.
- Affinity tuning values are named: `conflict_threshold()` (0.4) and `INCOMPAT_MIN_CONFLICTS` (5).
- `compose()` dominant-entry tie-break now picks the first max-weight entry (matches composite-name ordering).

### Added
- **`validate_profile(p)`** — public entry point wiring up the previously-unused validation layer (`require_unit_range`/`require_all_unit_range`); returns `ERR_INVALID_PARAMETER`/`ERR_OUT_OF_RANGE`/`ERR_NONE`.

### Docs
- Corrected the history-mapping count to 27 (was variously 25/26 in comments and CLAUDE.md; the test already asserted 27).
- Smoke-test banner no longer hardcodes the version string (avoids per-release drift).

## [2.4.0] — 2026-06-02

### Changed
- **Cyrius toolchain 3.10.0 → 6.0.38** — migrated to the current compiler. Build, 39 integration tests, and 45 benchmarks all pass clean.
- **Manifest `cyrius.toml` → `cyrius.cyml`** — new format with `version = "${file:VERSION}"` template (VERSION is now the single source of truth) and `cyrius = "6.0.38"` toolchain pin.
- Compiler version pin moved from the standalone `.cyrius-toolchain` file into `cyrius.cyml` `[package].cyrius`; `.cyrius-toolchain` removed.
- CI/release workflows modernized to match sibling projects (patra, vidya): toolchain version read dynamically from `cyrius.cyml` and installed via the upstream `install.sh`; added a lint gate (warnings-as-errors), a benchmark run, and version-consistency checks.
- Stale committed `lib/` stdlib snapshot removed from version control; dependencies now resolved at build time via `cyrius deps` (lib/ gitignored).

### Added
- **`dist/avatara.cyr`** — single-file consumer bundle generated by `cyrius distlib` (via a new `[lib]` section in `cyrius.cyml`), for consumers (bhava, joshua, kiran, agnosai, hadara).
- Benchmark regression tracking established as a standard versioning practice (`scripts/bench-history.sh` → `bench-history.csv`) to surface deltas and potential regressions across releases.

### Fixed
- Renamed `map_new` → `history_map_new` in `src/history.cyr` to resolve a name collision with stdlib `hashmap.cyr`'s `map_new` (which surfaced as a build warning under cyrius 6.x).

## [2.3.0] — 2026-04-13

### Added
- **Affinity module** (`affinity.cyr`) — composition intelligence system:
  - `affinity(a, b)` — similarity score (0.0 to 1.0) across all 15 traits + 14 emphases
  - `trait_affinity(a, b)` — trait-only similarity (ignores emphasis)
  - `similar_to(profile, max)` — find N most similar archetypes, sorted by score
  - `cross_tradition_match(profile)` — best match from a different tradition
  - `cross_tradition_matches(profile, max)` — best match per foreign tradition
  - `detect_conflicts(a, b)` — traits with >0.4 absolute difference, sorted by delta
  - `is_incompatible(a, b)` — true if 5+ conflicting traits
- 9 new test assertions for affinity, similarity, cross-tradition, and conflict detection (39 total integration tests)

### Changed
- `src/main.cyr` split into slim smoke test (builds fast) + `tests/avatara.tcyr` (full test suite)
- Smoke test prints "all systems nominal" on success

## [2.2.0] — 2026-04-12

### Added
- **Finnish/Sami tradition** (`finnish`) — 14 entities: Kalevala figures (Vainamoinen, Ilmarinen, Lemminkainen, Louhi, Joukahainen, Kullervo, Marjatta) and Finnish/Sami gods (Ukko, Tapio, Mielikki, Ahti, Tuoni, Loviatar, Madderakka)
- **Vodou Lwa tradition** (`vodou`) — 14 entities across three nachon: Rada (Papa Legba, Damballa Wedo, Ayida Wedo, Agwe, Erzulie Freda, Loko), Petwo (Erzulie Dantor, Simbi, Marinette), Ghede (Baron Samedi, Maman Brigitte, Baron Kriminel), plus Ogou Feray and Marasa
- **5 new Incarnate Mystics** — Desert Fathers, Gregory Palamas, Thomas Merton, Attar, Al-Ghazali (incarnate mystic count: 12 to 17)
- History mappings for Finnish and Vodou traditions
- ADR-006: Cyrius port decision record
- Tests for all new entities (190 total assertions)

### Changed
- Total archetypes: 329 to 362 (+33)
- Total traditions: 22 to 24 (+2)
- README rewritten for Cyrius (examples, build instructions, updated tradition table)
- Architecture overview updated for Cyrius data flow
- Roadmap updated (completed items removed, v2.3.0 planned)
- Scripts updated for Cyrius (bench-history.sh, version-bump.sh)

## [2.1.0] — 2026-04-12

### Added
- Rust source removed from repo (preserved in git history at v2.0.0)
- `benchmarks-rust-v-cyrius.md` — full Rust v1.1.0 vs Cyrius v2.0.1 comparison with code metrics, binary size, and all benchmark numbers
- 39 benchmarks (up from 19), matching and exceeding Rust Criterion suite (29)
- 195 test assertions (up from 122), covering all 19 traditions with per-entity spot checks
- Per-tradition tests: angelic, hindu, olympian, egyptian, buddhist, mesopotamian, celtic, shinto, aztec, maya, yoruba, zoroastrian, taoist, polynesian, slavic, jain, sikh, incarnate

## [2.0.1] — 2026-04-12

### Fixed
- `f64_le` and `f64_ge` defined as helpers (not cc3 builtins)
- `sakshi.cyr` include missing from main.cyr
- `history.cyr` formatting (cyrfmt compliance)
- Integration test (`avatara.tcyr`) — corrected assert function names (`assert_neq`, `assert_gte`), added proper exit syscall
- Mapping count corrected to 25 (was incorrectly 26)
- Stray binary artifact removed from repo

### Added
- `require_all_unit_range()` — validates all f64 values in a profile range
- `query_count_min_trait()` — count matching profiles without allocating
- `benchmarks-rust-v-cyrius.md` — Rust v1.0.0 baseline for comparison
- CI/release workflows rewritten for Cyrius (cc3 3.7.0)

## [2.0.0] — 2026-04-12

Complete rewrite from Rust to Cyrius. All ~206 archetypes across 19 traditions preserved with identical trait values, soul text, and spirit text.

### Added
- `src/types.cyr` — 312-byte ArchetypeProfile with inline TraitWeights (15 f64) and ModuleEmphasis (14 f64); 5 enums (BreathAffinity, GrowthDirection, Element, Polarity, CosmicTier); `profile_new()` constructor with 0.5 defaults
- `src/history.cyr` — 25 tradition-to-history mappings with civilization names, era names, temporal ranges, and scholarly notes; `context_for_tradition()`, `traditions_for_civilization()`, `traditions_active_at()`, `traditions_for_era()`
- `src/registry.cyr` — `query_civilization()`, `query_era()`, `query_active_at()` history-based filters
- `tests/avatara.tcyr` — integration test suite (entity counts, range validation, duplicate detection, breath monotonicity, compose invariants, history queries)
- `tests/avatara.bcyr` — benchmarks for all traditions, registry, compose, and history
- `programs/traditions.cyr` — example: explore archetypes, courage query, tradition counts
- `programs/compose.cyr` — example: blend three traditions
- `tests/test.sh` — test runner script
- `lib/bench.cyr` — benchmark framework

### Changed
- Language: Rust → Cyrius (18,804 LOC → ~15,600 LOC across 28 modules)
- Build: `Cargo.toml` → `cyrius.toml` (cc3 compiler)
- Types: manual memory layout with `alloc()`/`store64()`/`load64()`
- f64 weights: IEEE 754 bit patterns with `f64_*` builtins
- QueryBuilder fluent API → procedural `query_*()` filter functions
- `Archetype` trait → per-entity constructor functions (e.g. `kabbalah_kether()`)
- Lazy-init `all_*()` collection functions with global cache pattern
- Logging: tracing → sakshi

### Removed
- serde Serialize/Deserialize
- thiserror (replaced with integer error codes)
- Criterion benchmarks (replaced with Cyrius bench framework)

### Preserved
- All ~206 archetypes across 19 traditions with identical trait values
- Composition system (weighted blending with breath intensity averaging, growth/tier voting)
- Registry lookup and query API
- All soul text and spirit text verbatim
- 25 tradition-to-history mappings with scholarly notes
- Rust source removed (port complete)

## [1.1.0] — 2026-04-01 (Rust)

Historical context integration via itihas. Rust era (source removed in v2.0.1).

## [1.0.0] — 2026-03-31 (Rust)

Initial release. 19 traditions, ~206 archetypes, composition API, registry, query builder. Rust era (source removed in v2.0.1).
