# ADR-001: Common Output Type Across All Traditions

**Status**: Accepted
**Date**: 2026-03-31

> **Mechanism superseded by [ADR-006](006-cyrius-port.md) and [ADR-008](008-native-struct-migration.md); the decision stands.** The single shared output type is still the design, but it is no longer a Rust
> trait. `ArchetypeProfile` is a `#derive(accessors) struct Profile` (40 i64 fields, 320 bytes) returned
> by per-entity constructor functions; there is no `Archetype` trait and no `fn profile(&self)`. Read the
> Rust below as the reasoning, not as the current API.

## Context

Avatara maps divine and mythological beings from 20+ traditions to personality profiles. Each tradition has radically different theological structures — the Kabbalistic Tree of Life is nothing like the Yoruba Orisha system, which is nothing like the Jain Tirthankara lineage.

Consumers (bhava, joshua, agnosai) need a single, predictable interface regardless of which tradition produced the archetype.

## Decision

All traditions map to one struct: `ArchetypeProfile`, containing:

- `TraitWeights` — 15 f64 personality dimensions (0.0–1.0)
- `ModuleEmphasis` — 14 f64 module amplification weights
- `BreathAffinity` — 7-phase cosmic breath cycle position
- `GrowthDirection` — 5 growth modes
- `Element` — 9 classical element associations
- `Polarity` — 4 gender/polarity classifications
- `CosmicTier` — 7 hierarchy levels
- `soul_text` / `spirit_text` — identity layer strings

The `Archetype` trait enforces this: `fn profile(&self) -> ArchetypeProfile`.

## Consequences

- Composability: any two entities from any traditions can be blended via `compose()`
- Consumers never import tradition-specific types — only `ArchetypeProfile`
- Adding a new tradition requires zero changes to consumer code
- Trade-off: some tradition-specific nuance is lost in translation (e.g., Kabbalistic path relationships, Yoruba Ifá divination structure)
