# Security Policy

## Reporting a Vulnerability

If you discover a security vulnerability, please report it responsibly:

1. **Do not** open a public issue
2. Email: security@agnos.dev
3. Include: description, reproduction steps, potential impact

We will respond within 48 hours and aim to release a fix within 7 days.

## Supported Versions

Support tracks the current minor of the 2.x line. `VERSION` is the single source of truth for the
current release.

| Version | Supported | Notes |
|---------|-----------|-------|
| 2.14.x  | Yes       | Current release line (Cyrius) |
| 2.0–2.13 | No       | Fixes land in the current 2.x minor — upgrade |
| 1.0.x   | No        | Rust implementation, retired at the v2.0.0 Cyrius port ([ADR-006](docs/architecture/adr/006-cyrius-port.md)) |

## Scope

avatara is a data library: it computes archetype profiles in memory and returns plain f64 and enum
values. It opens no sockets, reads no files at runtime, and takes no untrusted input beyond the
archetype and tradition names passed to its lookup and query functions.

- **Dependencies.** Zero external dependencies except sakshi (logging). The Cyrius stdlib is vendored
  from the toolchain snapshot pinned in `cyrius.cyml` `[package].cyrius`.
- **Allocation.** All heap allocation routes through the checked `xalloc(n)`, which aborts on OOM
  rather than returning a null that would be dereferenced
  ([ADR-009](docs/architecture/adr/009-checked-allocation-policy.md), CWE-690).
- **Memory safety is convention-enforced.** Cyrius has no borrow checker
  ([ADR-006](docs/architecture/adr/006-cyrius-port.md)), so bounds and null handling are the
  contributor's responsibility. Reports of out-of-bounds reads, null dereferences or unchecked
  allocations in `src/` are in scope.
- **`dist/avatara.cyr`** is the committed consumer bundle, generated from `src/` by `cyrius distlib`.
  Report issues against `src/`; the bundle is regenerated, not patched.
