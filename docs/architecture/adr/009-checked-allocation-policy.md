# ADR-009: Checked allocation (abort-on-OOM) via xalloc

**Status**: Accepted
**Date**: 2026-06-03 (v2.5.4)

## Context

A v2.5.4 security/hardening audit (with CVE/0day web research) found avatara has no network, file, or untrusted-deserialization attack surface, and no public CVEs exist for its stack (Cyrius/AGNOS/sakshi are an internal ecosystem). The one real weakness class was **CWE-690 — Unchecked Return Value to NULL Pointer Dereference**: the stdlib `alloc()` returns `0` on OOM / invalid size (it does not abort), and avatara had 9 sites that `store64`/`store8`/`memcpy` into an `alloc()` result without a NULL check. Under memory exhaustion this is a near-NULL write — undefined behavior (a crash, and worse in some environments), not a defined failure.

Options considered: (a) thread a `0`/`Result` sentinel through all ~362 entity constructors — too invasive; (b) accept the risk — but the audit was explicitly requested; (c) a single checked-allocation helper with a defined OOM policy.

## Decision

Route every avatara heap allocation through `xalloc(n)` (`src/types.cyr`): it calls `alloc(n)`, and on `0` writes a diagnostic to stderr and `exit(134)`. OOM becomes a **defined, diagnosed abort** instead of silent UB. This is the same OOM policy as Rust's and Go's default allocators (abort, don't return null).

## Consequences

### Positive
- Eliminates the CWE-690 class at all 9 sites with one helper (one place to change the OOM policy).
- Converts undefined near-NULL writes into a clean, logged process exit.
- Allocation sizes in avatara are tiny (≤320 bytes) and never attacker-controlled, so the abort only fires under genuine host-wide memory exhaustion.

### Negative / Neutral
- A library that `exit()`s on OOM terminates the host process. This is the accepted industry norm (Rust/Go) and appropriate here: profile construction is unrecoverable under OOM, and the host is already failing. Documented so consumers know the contract.
- No measurable perf impact (one branch per allocation; allocations are not hot after lazy-init caching).

## Trade-off

For a pure in-process data library with tiny, bounded, non-attacker-controlled allocations, a defined abort-on-OOM is strictly safer than unchecked NULL propagation and far less invasive than threading error returns through every constructor.
