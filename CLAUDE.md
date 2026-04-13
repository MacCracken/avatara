# Avatara — Claude Code Instructions

## Project Identity

**Avatara** (Sanskrit: अवतार — descent of the divine) — Divine archetype engine: theological and mythological personality mapping across traditions

- **Language**: Cyrius (ported from Rust in v2.0)
- **License**: GPL-3.0-only
- **Version**: SemVer 2.3.0
- **Compiler**: cc3 >= 3.10.0

## Consumers

bhava (emotion/personality — post-v2.0 archetype overlay), joshua (NPC divine archetypes), kiran (game entities), agnosai (agent personalities with theological depth), hadara (archetype-to-culture context — ready for integration)

## Architecture

- `src/lib.cyr` — public API: includes all modules
- `src/main.cyr` — test harness (~80 assertions)
- `src/types.cyr` — ArchetypeProfile layout (312 bytes), TraitWeights (15 f64), ModuleEmphasis (14 f64), enums (BreathAffinity, GrowthDirection, Element, Polarity, CosmicTier)
- `src/error.cyr` — AvataraError enum codes, validation
- `src/compose.cyr` — archetype composition: weighted blending of multiple profiles
- `src/history.cyr` — 25 tradition-to-history mappings (civilization, era, temporal range, notes)
- `src/registry.cyr` — lookup by name, enumeration, query/filter API (includes history-based queries)
- `src/affinity.cyr` — affinity scoring, similarity search, cross-tradition matching, conflict detection
- `src/kabbalah.cyr` — Tree of Life: 10 Sephiroth
- `src/angelic.cyr` — 9 angelic orders, 7 archangels
- `src/hindu.cyr` — Trimurti, 11 Devas, 10 Avatars of Vishnu
- `src/olympian.cyr` — 15 Greek deities (12 Olympians + Hades, Hestia, Persephone)
- `src/norse.cyr` — 13 Aesir/Vanir gods
- `src/egyptian.cyr` — 16 principal deities
- `src/buddhist.cyr` — 7 Bodhisattvas, 5 Dhyani Buddhas
- `src/mesopotamian.cyr` — 14 Sumerian/Babylonian deities
- `src/celtic.cyr` — 15 Tuatha De Danann & Insular Celtic deities
- `src/shinto.cyr` — 15 Japanese Kami
- `src/aztec.cyr` — 14 Aztec (Mexica) deities
- `src/maya.cyr` — 12 Maya deities
- `src/yoruba.cyr` — 14 Yoruba/Ifa Orishas
- `src/zoroastrian.cyr` — 7 Amesha Spentas, 7 Zoroastrian beings
- `src/taoist.cyr` — 8 Immortals, 8 celestial deities
- `src/polynesian.cyr` — 12 Polynesian/Hawaiian deities
- `src/slavic.cyr` — 12 pre-Christian Slavic deities
- `src/jain.cyr` — 24 Tirthankaras
- `src/sikh.cyr` — 10 Sikh Gurus
- `src/finnish.cyr` — 14 Kalevala figures & Sami spirits
- `src/vodou.cyr` — 14 Vodou Lwa (Rada, Petwo, Ghede)
- `src/incarnate.cyr` — 56 incarnate divine figures (Hindu, Buddhist, Mystic, Taoist, Indigenous, Sage)
- `src/logging.cyr` — sakshi logging init
- `tests/avatara.tcyr` — integration test suite
- `tests/avatara.bcyr` — benchmarks
- `programs/traditions.cyr` — example: explore archetypes
- `programs/compose.cyr` — example: blend traditions

## Type System

All values are i64. f64 trait/emphasis weights stored as IEEE 754 bit patterns. Use f64_* builtins for arithmetic and comparison.

- `ArchetypeProfile` — 312 bytes, inline TraitWeights + ModuleEmphasis + enum fields + string pointers
- `profile_new()` — allocates with defaults (traits=0.5, emphasis=0.5, breath=LATE_EXHALE, growth=DIFFERENTIATE)
- Each tradition module: entity functions (e.g. `kabbalah_kether()`) + lazy-init `all_*()` collection + `*_count()`
- Registry: `all_profiles()`, `lookup(name)`, `lookup_in(tradition, name)`, `by_tradition()`, `query_*()` filters
- Compose: `compose(weighted_vec)` — weighted blending with f64 arithmetic
- History: `context_for_tradition()`, `traditions_for_civilization()`, `traditions_active_at()`, `traditions_for_era()`
- Affinity: `affinity()`, `similar_to()`, `cross_tradition_match()`, `cross_tradition_matches()`, `detect_conflicts()`, `is_incompatible()`

## Key Principles

- All traditions map to the same ArchetypeProfile output — composable across cultures
- Plain f64/enum outputs only — no bhava types leak into avatara
- Zero external dependencies except sakshi (logging)
- f64 values in 0.0-1.0 range for all traits and emphases
- Historically and theologically accurate — real traditions, real correspondences
- Respectful representation — these are living traditions for billions of people

## DO NOT

- **Do not commit or push** — the user handles all git operations
- **NEVER use `gh` CLI** — use `curl` to GitHub API only
- Do not invent theological associations — use established correspondences from scholarly sources
- Do not trivialize or mock any tradition
- Do not mix traditions without clear compositional semantics
