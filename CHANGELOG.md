# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.0] — 2026-03-31

### Added

#### Core Types (`lib.rs`)
- `TraitWeights` — 15-dimension personality profile (warmth, humor, empathy, patience, confidence, curiosity, creativity, directness, formality, verbosity, courage, precision, skepticism, autonomy, pedagogy)
- `ModuleEmphasis` — 14-dimension module amplification weights
- `BreathAffinity` — 7-phase cosmic breath cycle (Unity through LateInhale) with intensity mapping
- `GrowthDirection` — 5 growth modes (Differentiate, Integrate, Preserve, Transform, Still)
- `ArchetypeProfile` — complete output type with traits, emphasis, breath, growth, soul/spirit text
- `Archetype` trait — `profile()`, `name()`, `tradition()` with `#[must_use]`

#### Traditions (16 modules, ~206 entities)
- **Kabbalah** (`kabbalah`) — 10 Sephiroth (Kether through Malkuth) with full profiles
- **Angelic** (`angelic`) — 7 Archangels, 9 Angelic Orders (Pseudo-Dionysian hierarchy)
- **Hindu** (`hindu`) — Trimurti (3), Devas (7), Dashavatara (10 Avatars of Vishnu)
- **Greek** (`olympian`) — 12 Olympians
- **Norse** (`norse`) — 10 Aesir/Vanir gods
- **Egyptian** (`egyptian`) — 12 principal deities
- **Buddhist** (`buddhist`) — 7 Bodhisattvas, 5 Dhyani Buddhas
- **Mesopotamian** (`mesopotamian`) — 14 Sumerian/Babylonian deities
- **Celtic** (`celtic`) — 14 Tuatha De Danann & Insular Celtic deities
- **Shinto** (`shinto`) — 14 Japanese Kami
- **Aztec** (`aztec`) — 14 Aztec (Mexica) deities
- **Maya** (`maya`) — 12 Maya deities
- **Yoruba** (`yoruba`) — 14 Orishas
- **Zoroastrian** (`zoroastrian`) — 7 Amesha Spentas, 7 Zoroastrian beings
- **Taoist** (`taoist`) — 8 Immortals, 8 celestial deities
- **Incarnate** (`incarnate`) — 36 incarnate divine figures across Hindu, Buddhist, Mystic, Taoist, Indigenous traditions

#### Composition API (`compose`)
- `compose()` — weighted blending of multiple `ArchetypeProfile`s
- Trait/emphasis weighted averaging, breath intensity blending, growth direction voting
- Input validation with `AvataraError::InvalidParameter`

#### Registry & Query (`registry`)
- `all_profiles()` — enumerate all ~206 entities
- `lookup(name)` / `lookup_in(tradition, name)` — name-based retrieval
- `traditions()` / `by_tradition()` — tradition enumeration
- `QueryBuilder` — fluent filtering by tradition, breath, growth, min/max trait thresholds

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
