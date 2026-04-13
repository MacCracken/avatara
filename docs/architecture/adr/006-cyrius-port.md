# ADR-006: Port from Rust to Cyrius

**Status**: Accepted
**Date**: 2026-04-12

## Context

Avatara was originally written in Rust (v1.0.0-v1.1.0, 18,804 LOC). The AGNOS ecosystem is standardizing on Cyrius as its systems language. Sibling projects (itihas, vidya, hadara) have already been ported or written natively in Cyrius.

## Decision

Port the entire avatara codebase from Rust to Cyrius, preserving all archetype data, composition logic, registry queries, and historical context mappings exactly.

## Consequences

### Positive
- Zero external dependencies (was 5: serde, thiserror, tracing, tracing-subscriber, itihas)
- Compile time under 100ms (was 5-60 seconds)
- Lazy-init caching yields 50-2,700x speedup on repeated access patterns
- Direct integration with other Cyrius AGNOS projects (hadara, itihas) via include
- Binary size comparable (~874KB vs ~800KB-1.2MB)

### Negative
- No serde serialization (Cyrius has no equivalent yet)
- No LLVM optimizer — single profile creation 4x slower (249ns vs 63ns)
- Manual memory layout (store64/load64) instead of Rust's type-safe structs
- No borrow checker — memory safety is convention-enforced

### Neutral
- LOC reduced 17% (18,804 to 15,644) — Cyrius is more concise but less type-annotated
- Rust source preserved in git history (v2.0.0 tag) for reference
- QueryBuilder fluent API replaced with procedural query_*() functions — equivalent functionality, different ergonomics

## Trade-off

The ecosystem consistency and compile-time gains outweigh the loss of Rust's type safety for this specific codebase. Avatara is data-heavy (362 archetype definitions) with simple control flow — the risk surface for memory bugs is small, and the caching architecture makes the performance profile superior for real-world consumer patterns.
