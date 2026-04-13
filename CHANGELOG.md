# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### v2.4.0 roadmap
- `domain` field (War, Love, Death, Creation, Knowledge, etc.)
- Cross-tradition affinity graph (pre-computed, stored)
- Shadow aspect support (dark/inverted form of each archetype)

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
