# ADR-007: Affinity System for Cross-Tradition Mapping

**Status**: Accepted
**Date**: 2026-04-13

## Context

Avatara has 362 archetypes across 24 traditions. Consumers need to answer questions like "who is the Norse equivalent of Hindu Indra?" or "which archetypes conflict when composed?" without manually mapping every pair.

## Decision

Implement an affinity system based on mean absolute difference across all trait and emphasis dimensions. Similarity is `1.0 - mean_abs_diff`, giving a score from 0.0 (maximally different) to 1.0 (identical).

### Scoring
- `affinity(a, b)` — full similarity across 15 traits + 14 emphases (29 dimensions)
- `trait_affinity(a, b)` — trait-only (15 dimensions), ignores emphasis

### Search
- `similar_to(profile, N)` — N nearest neighbors across all traditions, sorted by score
- `cross_tradition_match(profile)` — single best match from a different tradition
- `cross_tradition_matches(profile, N)` — best match per foreign tradition, top N

### Conflict Detection
- `detect_conflicts(a, b)` — traits with absolute difference > 0.4, sorted by delta
- `is_incompatible(a, b)` — true if 5+ traits conflict

## Alternatives Considered

1. **Euclidean distance** — more sensitive to outliers. Mean absolute difference is more interpretable and robust to single-trait spikes.
2. **Cosine similarity** — treats profile as vector angle. Doesn't capture magnitude differences (e.g., both archetypes have same trait ratios but different intensity).
3. **Pre-computed affinity matrix** — O(N^2) storage for 362 entities. Deferred to v2.4.0 if runtime cost becomes an issue.
4. **Weighted dimensions** — some traits matter more for "archetype similarity" than others. Current design weights all equally. Can be extended later with a weight vector.

## Consequences

### Positive
- Cross-tradition mapping is automatic and data-driven, not hand-curated
- Conflict detection enables compose() to warn about problematic combinations
- Similar-to search enables discovery ("show me archetypes like Athena")
- O(N) per query with N=362 — fast enough for all consumer patterns

### Negative
- Equal-weight dimensions may not match human intuition about "similar archetypes" in all cases
- No persistence — computed on every call (but profiles are cached, so the cost is just the arithmetic)
