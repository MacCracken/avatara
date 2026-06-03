# ADR-008: Native struct + #derive(accessors) for ArchetypeProfile

**Status**: Accepted
**Date**: 2026-06-03 (v2.5.3)

## Context

`ArchetypeProfile` was a manually laid-out heap blob: `alloc(SIZE)` + a `ProfLayout` offset enum (`PROF_WARMTH = 24`, …) + ~10k `store64(p + PROF_X, v)` writes across 24 modules + ~40 hand-written `prof_*` accessors. This idiom produced the v2.4.5 bug — `PROF_SPIRIT` was accidentally defined at two offsets (176 and 304), "last definition wins" silently routed every spirit-emphasis write to the wrong slot, and nothing caught it because no field is named.

Cyrius gained `#derive(accessors)` structs, but capped structs at 32 fields; `Profile` has 40 (incl. the v2.5.2 `domain`). The migration was deferred (ADR tracked in doc-health) until cyrius **6.0.47** raised the cap to 256 (our filed proposal `2026-06-02-struct-field-cap-raise.md`).

## Decision

Define `ArchetypeProfile` as a `#derive(accessors) struct Profile` of 40 i64 fields in declaration order matching the existing offsets (sizeof == 320). The compiler-generated `Profile_<field>(p)` / `Profile_set_<field>(p, v)` are canonical. Mechanically convert the ~10.4k `store64(p + PROF_*)` writes to `Profile_set_*`. Keep `prof_*` getters as thin delegating shims (consumer compat). Retain the `ProfLayout` offset enum for the loop-based code that iterates computed offset ranges (`profile_new` default-fill, `compose` blend, `affinity`/`error` range scans, `shadow` inversion) and for the layout-assertion test.

## Consequences

### Positive
- Offset-collision bugs (the 2.4.5 class) are now **compile errors** — duplicate field names are rejected.
- Field access is named and self-documenting; no manual offset arithmetic at write sites.
- A layout-assertion test pins `sizeof == 320` and each `Profile_set_*` to its `PROF_*` offset, guarding future field changes.

### Negative / Neutral
- A function-call layer at field access (setters/getters wrap `store64`/`load64`); benchmarks confirm it is perf-neutral (within run-to-run noise).
- The offset enum is retained (not eliminated) because range-loop code needs computed offsets — the struct and the enum coexist as two views of the same bytes.
- Large mechanical diff (~10.4k sites); landed as its own release (2.5.3), behavior-preserving (all 60 tests pass unchanged).

## Trade-off

The compile-time safety against offset collisions — the exact failure that shipped undetected in 2.4.5 — justifies a large but mechanical, fully test-covered migration. Keeping `prof_*` shims means zero downstream churn for the five consumers.
