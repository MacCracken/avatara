# ADR-002: Non-Exhaustive Enums on All Public Types

**Status**: Accepted
**Date**: 2026-03-31

## Context

Theological and mythological traditions are living systems. New entities may be added (e.g., discovering a new inscription that reveals a previously unknown deity), and existing categorizations may be refined. If downstream consumers match exhaustively on our enums, adding a variant becomes a breaking change.

## Decision

All 29+ public enums carry `#[non_exhaustive]`:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Sephira { ... }
```

This applies to:
- Tradition entity enums (Sephira, Olympian, NorseGod, etc.)
- Classification enums (BreathAffinity, GrowthDirection, Element, Polarity, CosmicTier)
- Error enum (AvataraError)

## Consequences

- Adding entities or classification variants is a non-breaking change
- Downstream consumers must include wildcard arms in match statements
- This enables semver-compatible tradition expansion across minor versions
