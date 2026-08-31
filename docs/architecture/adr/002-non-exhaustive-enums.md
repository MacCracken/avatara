# ADR-002: Non-Exhaustive Enums on All Public Types

**Status**: Accepted
**Date**: 2026-03-31

> **Superseded by [ADR-006](006-cyrius-port.md).** Cyrius has no `#[non_exhaustive]`. Enums are plain
> integer constants, so adding a variant cannot break a downstream `match` the way it could in Rust, and
> the forward-compatibility problem this ADR solves does not arise in the same form. The intent — that
> new variants are additive and consumers must tolerate unknown values — is still honoured, by
> convention and by the `_COUNT` sentinel each enum carries.
>
> **Correction, 2.14.8:** "each enum carries" a `_COUNT` sentinel was never true of the enums this ADR
> is actually about. The six public classification enums — BreathAffinity, GrowthDirection, Element,
> Polarity, CosmicTier, Domain — carry none, and 15 of 67 enums in `src/` have none. The sentinel is a
> **collection** convention, for enums a caller iterates; a classification read off a profile is a
> closed vocabulary, not a range to walk. The additive-only intent is unaffected and still holds.

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
