# Contributing to avatara

## Getting Started

```bash
git clone https://github.com/MacCracken/avatara.git
cd avatara
cargo test
```

## Guidelines

- All traditions map to the same `ArchetypeProfile` output — composable across cultures.
- `#[non_exhaustive]` on ALL public enums.
- `#[must_use]` on all pure functions.
- Every type must be `Serialize + Deserialize` (serde).
- Zero unwrap/panic in library code.
- Historically and theologically accurate — real traditions, real correspondences.
- Respectful representation — these are living traditions for billions of people.
- Do not invent theological associations — use established correspondences from scholarly sources.
- Do not trivialize or mock any tradition.
- Zero clippy warnings.

## Adding a New Tradition

1. Create `src/<tradition>.rs` following the existing pattern (see `norse.rs` for a clean example).
2. Define an enum with `#[non_exhaustive]`, serde derives, and an `ALL` const slice.
3. Implement `Archetype` with differentiated `TraitWeights`, `ModuleEmphasis`, `BreathAffinity`, `GrowthDirection`, and soul/spirit text.
4. Add `pub mod <tradition>;` to `lib.rs`.
5. Add 4-5 meaningful tests.
6. Update `CLAUDE.md`, `README.md`, and `CHANGELOG.md`.

## Testing

```bash
cargo test                # all tests
cargo clippy              # lint
cargo bench               # benchmarks
make check                # fmt + clippy + test + audit
```

## License

By contributing you agree that your contributions will be licensed under GPL-3.0-only.
