# Roadmap

> avatara — forward-looking milestones only. Shipped work lives in [CHANGELOG.md](../../CHANGELOG.md).

## Shipped

- **2.4.x** — toolchain + hardening + language modernization: cyrius 3.10.0 → 6.0.40, `cyrius.cyml` manifest, modern CI/release, dist bundle, bench-on-every-release; NULL/overflow hardening; `+=`/`match` adoption; stdlib `f64_le`/`f64_ge`; spirit-emphasis (`PROF_SPIRIT`) collision fix.
- **2.5.x** — architecture: `Result<T, E>` error model (2.5.0); shadow aspect (2.5.1); `domain` field, all 366 archetypes assigned (2.5.2); native `#derive` `struct Profile` migration (2.5.3); security audit + CWE-690 `xalloc` hardening (2.5.4).
- **2.6.0** — The Solar Year: 25th "Solar" tradition (intercalary archetypes), landing at **366 archetypes (365 + the leap quarter)**.

All originally-roadmapped items are complete. The path below sequences the
former demand-gated backlog into minors toward a 3.0.0 consolidation.

## Planned — minors to 3.0.0

Each is additive and non-breaking (new archetypes / traditions / an additive
overlay layer). Historical-accuracy rule stands throughout: established
scholarly correspondences only, no inventions.

- **v2.7.0 — Canaanite & Etruscan** — two new micro-traditions (Canaanite/Ugaritic: El, Baal, Asherah, Anat; Etruscan: Tinia, Uni, Menrva, Voltumna). ~8 entities.
- **v2.8.0 — Tarot Major Arcana** — 22 archetypes, mapped to the 22 Tree-of-Life paths (bridges the existing Kabbalah module).
- **v2.9.0 — I Ching** — 64 hexagram archetypes.
- **v2.10.0 — World-traditions completion** — Aboriginal Australian, Native American (specific nations), Inuit; plus the deferred Polynesian / Slavic / Celtic additions (Pele/Kanaloa aspects, Mokosh aspects/Rod, Ogma/Miach/Airmed).
- **v2.11.0 — Archetype overlays** — the first cross-cutting layer *on top of* the archetype profiles: Enneagram (9 types) and the Jungian set (Hero, Shadow, Anima/Animus, Self, Trickster — composes with the existing `shadow()`). Additive new API; profiles unchanged.

## v3.0.0 — Consolidation (breaking)

The major bump banks the API cleanups deferred through 2.x:

- Migrate `cross_tradition_match` / `find_mapping` from `0`-on-not-found to `Option` (the absence-vs-error distinction noted in 2.5.0).
- Drop the `prof_*` compat shims — consumers move to the derived `Profile_*` accessors (shims have eased the transition since 2.5.3).
- Retire the public `ProfLayout` offset enum from the consumer surface (internal-only).
- Formalize the overlay subsystem (2.11.0) as first-class API. The "archetype + overlay engine" identity for v3.

## Declined

- **Affinity-graph caching** — a pointer-keyed cross-tradition cache regressed the bench (`cross_tradition_match` 49µs → 945µs); the construct-then-query access pattern misses a pointer-keyed cache. Revisit only with a non-pointer-keyed (index/name-based), bench-proven design.

## Dependencies for Consumer Integration

| Consumer | Status | Bridge |
|----------|--------|--------|
| itihas (world history) | **v2.0** (Cyrius) | `history` module — tradition to civilization/era mapping |
| hadara (culture) | **v0.1** (Cyrius) | Ready for archetype-to-culture integration |
| bhava (emotion/personality) | Planned | `ArchetypeProfile` to `PersonalityProfile` |
| joshua (NPC archetypes) | Planned | Direct `ArchetypeProfile` consumption |
| kiran (game entities) | Planned | Via joshua |
| agnosai (agent personalities) | Planned | Direct consumption |
| sankhya (ancient sciences) | Planned | Shared `IncarnateSage` / Vedic bridge |
