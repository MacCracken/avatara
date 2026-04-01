# Avatara — Claude Code Instructions

## Project Identity

**Avatara** (Sanskrit: अवतार — descent of the divine) — Divine archetype engine: theological and mythological personality mapping across traditions

- **Type**: Flat library crate
- **License**: GPL-3.0-only
- **MSRV**: 1.89
- **Version**: SemVer 0.1.0

## Consumers

bhava (emotion/personality — post-v2.0 archetype overlay), joshua (NPC divine archetypes), kiran (game entities), agnosai (agent personalities with theological depth)

## Architecture

- `src/lib.rs` — common types: `TraitWeights`, `ModuleEmphasis`, `BreathAffinity`, `GrowthDirection`, `ArchetypeProfile`, `Archetype` trait
- `src/kabbalah.rs` — Tree of Life: 10 Sephiroth with full trait/emphasis/breath profiles
- `src/angelic.rs` — 9 angelic orders, 7 archangels
- `src/hindu.rs` — Trimurti, 7 Devas, 10 Avatars of Vishnu
- `src/olympian.rs` — 12 Olympians
- `src/norse.rs` — 10 Aesir/Vanir gods
- `src/egyptian.rs` — 12 principal deities
- `src/buddhist.rs` — 7 Bodhisattvas, 5 Dhyani Buddhas
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
