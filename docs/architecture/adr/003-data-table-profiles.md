# ADR-003: Data-Table Match Arms for Profile Generation

**Status**: Accepted
**Date**: 2026-03-31

## Context

Each entity needs a full `ArchetypeProfile` with ~40 fields (15 traits + 14 emphasis + breath + growth + element + polarity + tier + 3 strings). With ~280 entities, this is ~11,000 data points. How should this data be stored?

Options considered:
1. **External data files** (JSON/TOML) loaded at runtime
2. **Const tables** with compile-time arrays
3. **Match-based data tables** — `profile()` is one giant match with struct literals per variant

## Decision

Option 3: match-based data tables. Each `profile()` method is a large match expression:

```rust
fn profile(&self) -> ArchetypeProfile {
    let (traits, emphasis, breath, growth, desc, soul, spirit) = match self {
        Self::Kether => (TraitWeights { ... }, ModuleEmphasis { ... }, ...),
        Self::Chokmah => (...),
        // ...
    };
    ArchetypeProfile { name: self.name().to_string(), ... }
}
```

## Consequences

- **No runtime I/O** — profiles are compiled into the binary
- **Type-safe** — the compiler verifies all fields are present for all variants
- **Pedantic clippy warns** `too_many_lines` on profile() methods — accepted as inherent to the data-table pattern
- **Modification requires recompilation** — but this is appropriate for curated theological data
- **Zero dependencies** for data loading — no serde_json at runtime
- Trade-off: each tradition module file is 400–2000 lines, mostly data
