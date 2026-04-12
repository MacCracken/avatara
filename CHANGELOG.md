# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### v2.1.0 roadmap
- Finnish/Sami tradition (Kalevala figures)
- Vodou Lwa (distinct from Yoruba)
- Expand Incarnate: Desert Fathers, Gregory Palamas, Thomas Merton, Attar, Al-Ghazali
- Cross-tradition affinity mapping (Shango ~ Thor ~ Indra ~ Perun)
- hadara integration (archetype-to-culture context)
- itihas integration (historical context queries)

## [2.0.0] — 2026-04-12

### Changed
- **Complete rewrite from Rust to Cyrius** — 18,804 lines Rust to ~15,000 lines Cyrius across 27 modules
- All types use manual memory layout (312-byte ArchetypeProfile with inline TraitWeights + ModuleEmphasis)
- f64 trait/emphasis weights stored as IEEE 754 bit patterns, using f64_* builtins
- QueryBuilder fluent API replaced with procedural query_* filter functions
- Archetype trait replaced with per-entity constructor functions (e.g. `kabbalah_kether()`)
- Lazy-init `all_*()` collection functions with global cache pattern
- Build system: Cargo.toml replaced with cyrius.toml (cc3 compiler)

### Removed
- serde (Cyrius does not have serde equivalent yet)
- thiserror (replaced with integer error codes)
- tracing (replaced with sakshi logging)
- Criterion benchmarks (to be replaced with Cyrius bench framework)
- itihas feature gate (to be re-added as Cyrius include)

### Preserved
- All ~206 archetypes across 19 traditions with identical trait values
- Composition system (weighted blending)
- Registry lookup and query API
- All soul text and spirit text verbatim
- Rust source preserved in rust-old/ for reference

## [1.1.0] — 2026-04-01 (Rust)

Historical context integration via itihas. See rust-old/ for details.

## [1.0.0] — 2026-03-31 (Rust)

Initial release. 19 traditions, ~206 archetypes, composition API, registry, query builder. See rust-old/ for details.
