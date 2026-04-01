# Avatara — Claude Code Instructions

## Project Identity

**Avatara** (Sanskrit: अवतार — descent of the divine) — Divine archetype engine: theological and mythological personality mapping across traditions

- **Type**: Flat library crate
- **License**: GPL-3.0-only
- **MSRV**: 1.89
- **Version**: SemVer 1.0.0

## Consumers

bhava (emotion/personality — post-v2.0 archetype overlay), joshua (NPC divine archetypes), kiran (game entities), agnosai (agent personalities with theological depth)

## Architecture

- `src/lib.rs` — common types: `TraitWeights`, `ModuleEmphasis`, `BreathAffinity`, `GrowthDirection`, `ArchetypeProfile`, `Archetype` trait
- `src/compose.rs` — archetype composition: weighted blending of multiple profiles
- `src/registry.rs` — lookup by name, enumeration, query/filter API
- `src/kabbalah.rs` — Tree of Life: 10 Sephiroth
- `src/angelic.rs` — 9 angelic orders, 7 archangels
- `src/hindu.rs` — Trimurti, 11 Devas, 10 Avatars of Vishnu
- `src/olympian.rs` — 15 Greek deities (12 Olympians + Hades, Hestia, Persephone)
- `src/norse.rs` — 13 Aesir/Vanir gods
- `src/egyptian.rs` — 16 principal deities
- `src/buddhist.rs` — 7 Bodhisattvas, 5 Dhyani Buddhas
- `src/mesopotamian.rs` — 14 Sumerian/Babylonian deities
- `src/celtic.rs` — 15 Tuatha Dé Danann & Insular Celtic deities
- `src/shinto.rs` — 15 Japanese Kami
- `src/aztec.rs` — 14 Aztec (Mexica) deities
- `src/maya.rs` — 12 Maya deities
- `src/yoruba.rs` — 14 Yoruba/Ifá Orishas
- `src/zoroastrian.rs` — 7 Amesha Spentas, 7 Zoroastrian beings
- `src/taoist.rs` — 8 Immortals, 8 celestial deities
- `src/polynesian.rs` — 12 Polynesian/Hawaiian deities
- `src/slavic.rs` — 12 pre-Christian Slavic deities
- `src/jain.rs` — 24 Tirthankaras
- `src/sikh.rs` — 10 Sikh Gurus
- `src/incarnate.rs` — 44 incarnate divine figures (Hindu, Buddhist, Mystic, Taoist, Indigenous)
- `src/error.rs` — `AvataraError` enum
- `src/logging.rs` — tracing-subscriber init (feature-gated)

## Key Principles

- All traditions map to the same `ArchetypeProfile` output — composable across cultures
- Plain f64/enum outputs only — no bhava types leak into avatara
- `#[non_exhaustive]` on ALL public enums
- `#[must_use]` on all pure functions
- Every type must be Serialize + Deserialize (serde)
- Zero unwrap/panic in library code
- Historically and theologically accurate — real traditions, real correspondences
- Respectful representation — these are living traditions for billions of people

## DO NOT

- **Do not commit or push** — the user handles all git operations
- **NEVER use `gh` CLI** — use `curl` to GitHub API only
- Do not invent theological associations — use established correspondences from scholarly sources
- Do not trivialize or mock any tradition
- Do not mix traditions without clear compositional semantics
