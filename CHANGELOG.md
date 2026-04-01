# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### v1.2.0 roadmap
- Finnish/Sami tradition (Kalevala figures)
- Vodou Lwa (distinct from Yoruba)
- Expand Incarnate: Desert Fathers, Gregory Palamas, Thomas Merton, Attar, Al-Ghazali
- Cross-tradition affinity mapping (Shango ~ Thor ~ Indra ~ Perun)
- Conflict detection (`AvataraError::Incompatible`)
- Affinity scoring between archetypes
- Archetype similarity search

## [1.1.0] — 2026-04-01

### Added

#### Historical Context Integration (`history`) — feature-gated behind `itihas`
- `HistoricalContext` struct — civilizations, eras, temporal range, and historical notes for each tradition
- `context_for_tradition()` / `context_for_profile()` — resolve any tradition or archetype profile to its civilizational and temporal context via itihas data
- `traditions_for_civilization()` — reverse lookup: which traditions belong to a given civilization
- `traditions_for_era()` — which traditions were active in a given era
- `traditions_active_at(year)` — which traditions existed at a point in time
- `mapped_traditions()` — list all traditions with historical mappings
- 26 tradition-to-history mappings with scholarly historical notes

#### Registry Query Bridge (`registry`) — feature-gated behind `itihas`
- `QueryBuilder::civilization()` — filter archetypes by associated civilization
- `QueryBuilder::era()` — filter archetypes by associated era
- `QueryBuilder::active_at()` — filter archetypes by year of tradition activity
- All composable with existing filters (`.min_trait()`, `.element()`, `.tradition()`, etc.)

#### Dependencies
- `itihas` v1 as optional dependency (feature-gated)

#### Tests & Benchmarks
- 15 new unit tests in `history` module (context resolution, reverse lookups, temporal validation)
- 4 new registry query tests for civilization/era/year filters
- 6 new integration tests (context resolution, era overlap, query correctness, serde roundtrip)
- 9 new criterion benchmarks across `history` and `registry_itihas` groups

### Changed
- `deny.toml` — removed 7 unused license allowances (BSD-2-Clause, BSD-3-Clause, ISC, OpenSSL, Unicode-DFS-2016, Zlib, Apache-2.0 WITH LLVM-exception)

## [1.0.1] — 2026-04-01

### Fixed
- Benchmark script sed regex updated for Criterion 0.8 output format (was failing to parse `[low unit median unit high unit]`)
- Release workflow tag trigger changed from `v*` to `[0-9]*` to match unprefixed SemVer tags
- Release workflow version verification no longer expects `v` prefix
- Registry benchmarks correctly classified as Tier 3 (cross-tradition) in BENCHMARKS.md

## [1.0.0] — 2026-03-31

### Added

#### Core Types (`lib.rs`)
- `TraitWeights` — 15-dimension personality profile (warmth, humor, empathy, patience, confidence, curiosity, creativity, directness, formality, verbosity, courage, precision, skepticism, autonomy, pedagogy)
- `ModuleEmphasis` — 14-dimension module amplification weights
- `BreathAffinity` — 7-phase cosmic breath cycle (Unity through LateInhale) with intensity mapping
- `GrowthDirection` — 5 growth modes (Differentiate, Integrate, Preserve, Transform, Still)
- `ArchetypeProfile` — complete output type with traits, emphasis, breath, growth, element, polarity, tier, soul/spirit text
- `Archetype` trait — `profile()`, `name()`, `tradition()` with `#[must_use]`
- `Element` — classical element association (Fire, Water, Earth, Air, Aether, Light, Darkness, Storm, Mixed)
- `Polarity` — masculine/feminine/androgynous/transcendent
- `CosmicTier` — cosmic hierarchy (Supreme, Primordial, Cosmic, Greater, Lesser, Demigod, Master)

#### Traditions (20 modules, ~280 entities)
- **Kabbalah** (`kabbalah`) — 10 Sephiroth (Kether through Malkuth)
- **Angelic** (`angelic`) — 7 Archangels, 9 Angelic Orders (Pseudo-Dionysian hierarchy)
- **Hindu** (`hindu`) — Trimurti (3), Devas (11), Dashavatara (10 Avatars of Vishnu)
- **Greek** (`olympian`) — 15 deities (12 Olympians + Hades, Hestia, Persephone)
- **Norse** (`norse`) — 13 Aesir/Vanir gods
- **Egyptian** (`egyptian`) — 16 principal deities
- **Buddhist** (`buddhist`) — 7 Bodhisattvas, 5 Dhyani Buddhas
- **Mesopotamian** (`mesopotamian`) — 14 Sumerian/Babylonian deities
- **Celtic** (`celtic`) — 15 Tuatha De Danann & Insular Celtic deities
- **Shinto** (`shinto`) — 15 Japanese Kami
- **Aztec** (`aztec`) — 14 Aztec (Mexica) deities
- **Maya** (`maya`) — 12 Maya deities
- **Yoruba** (`yoruba`) — 14 Orishas
- **Zoroastrian** (`zoroastrian`) — 7 Amesha Spentas, 7 Zoroastrian beings
- **Taoist** (`taoist`) — 8 Immortals, 8 celestial deities
- **Polynesian** (`polynesian`) — 12 Polynesian/Hawaiian deities
- **Slavic** (`slavic`) — 12 pre-Christian Slavic deities
- **Jain** (`jain`) — 24 Tirthankaras
- **Sikh** (`sikh`) — 10 Sikh Gurus
- **Incarnate** (`incarnate`) — 51 incarnate divine figures across Hindu, Buddhist, Mystic, Taoist, Indigenous, Vedic traditions

#### Composition API (`compose`)
- `compose()` — weighted blending of multiple `ArchetypeProfile`s
- Trait/emphasis weighted averaging, breath intensity blending, growth direction voting
- Input validation with `AvataraError::InvalidParameter`

#### Registry & Query (`registry`)
- `all_profiles()` — enumerate all ~280 entities
- `lookup(name)` / `lookup_in(tradition, name)` — name-based retrieval
- `traditions()` / `by_tradition()` — tradition enumeration
- `QueryBuilder` — fluent filtering by tradition, breath, growth, element, polarity, tier, min/max trait thresholds

#### Error Handling (`error`)
- `AvataraError::InvalidParameter` — composition validation
- `AvataraError::UnknownArchetype` — lookup failures
- `AvataraError::Incompatible` — reserved for future archetype conflict detection

#### Infrastructure
- 133 tests (117 unit + 12 integration + 4 doc-tests)
- 30 criterion benchmarks covering all traditions + composition + registry
- `logging` feature gate with tracing-subscriber
- Full serde support (Serialize + Deserialize) on all types
- `#[non_exhaustive]` on all 29 public enums
