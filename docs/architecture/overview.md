# Architecture Overview

> avatara — Divine archetype engine for AGNOS

## System Diagram

```
                    +----------------------------------------------+
                    |              avatara                          |
                    |     divine archetype engine (Cyrius)          |
                    +----------------------------------------------+
                    |                                              |
                    |  +----------------------------------------+  |
                    |  |   32 tradition modules -> 503           |  |
                    |  |         archetypes, 41 traditions       |  |
                    |  |  kabbalah  angelic  hindu  olympian     |  |
                    |  |  norse  egyptian  buddhist  celtic      |  |
                    |  |  mesopotamian  shinto  aztec  maya      |  |
                    |  |  yoruba  zoroastrian  taoist            |  |
                    |  |  polynesian  slavic  jain  sikh         |  |
                    |  |  finnish  vodou                         |  |
                    |  |  incarnate (56; Hindu/Buddhist/         |  |
                    |  |    Christian/Vedic/Sufi/Taoist/Jewish   |  |
                    |  |    + 4 named nations)                   |  |
                    |  |  solar (intercalary: 365 + the quarter) |  |
                    |  |  canaanite  etruscan                    |  |
                    |  |  tarot (Tree-of-Life paths, Kabbalah)   |  |
                    |  |  iching (64 hexagrams, 8 trigrams)      |  |
                    |  |  inuit  lakota  haudenosaunee           |  |
                    |  |  anishinaabe                            |  |
                    |  |  aboriginal (Kunwinjku/Kulin/           |  |
                    |  |    Gunaikurnai -- 1 module, 3 peoples)  |  |
                    |  +-------------------+--------------------+  |
                    |                      |                       |
                    |  +-------------------v--------------------+  |
                    |  |       ArchetypeProfile (320 bytes)      |  |
                    |  |  15 traits + 14 emphasis + breath       |  |
                    |  |  + growth + element + polarity          |  |
                    |  |  + tier + domain + soul/spirit text     |  |
                    |  +-------------------+--------------------+  |
                    |             |                    |           |
                    |  +----------v----------+  +------v-------+   |
                    |  | derived FROM the    |  |  registry    |   |
                    |  | profile, stored     |  |  lookup      |   |
                    |  | nowhere:            |  |  query       |   |
                    |  |   aspect  (roles)   |  |  by_tradition|   |
                    |  |   shadow  (inverse) |  +--------------+   |
                    |  |   overlay (typology)|                     |
                    |  +---------------------+                     |
                    |                      |                       |
                    |  +--------+------+------+                    |
                    |  | compose | hist | affi |                   |
                    |  | blend   | 41   | nity |                   |
                    |  | weight  | maps | sim  |                   |
                    |  +---------+------+------+                   |
                    +---------------------+------------------------+
                                          |
                    +---------------------v------------------------+
                    |              Consumers                        |
                    |  bhava — emotion/personality bridge            |
                    |  joshua — NPC divine archetypes                |
                    |  kiran — game entities                         |
                    |  agnosai — agent theological depth             |
                    |  hadara — culture-to-archetype context         |
                    |  sankhya — ancient science archetypes          |
                    +----------------------------------------------+
```

## Core Principle

All traditions map to the same `ArchetypeProfile` output. A character can carry Kabbalistic + Hindu + Greek archetypes simultaneously — reinforcing archetypes amplify, conflicting archetypes create productive internal tension.

Avatara produces plain f64/enum outputs. It does not depend on bhava — bhava consumes avatara's output through a bridge module.

## Data Flow

1. Tradition module defines entities as constructor functions (e.g. `kabbalah_tiphareth()`)
2. Constructor calls `profile_new()`, sets fields via the derived setters `Profile_set_*(p, value)`, returns pointer
3. `all_*()` collection functions cache profiles on first call (lazy-init pattern)
4. `compose()` blends multiple profiles with weighted averaging
5. `registry::all_profiles()` aggregates all traditions into a single cached vec
6. `query_*()` functions filter profiles by trait, breath, growth, element, polarity, tier, domain, civilization, era, year
7. `affinity()` scores similarity between profiles; `similar_to()` finds nearest neighbors (bounded top-k, O(N*k)); `cross_tradition_match()` maps across traditions; `detect_conflicts()` identifies trait tensions
8. Derived layers read *off* a finished profile and store nothing: `aspect.cyr` (trait-derived roles), `shadow.cyr` (`shadow()`, the involutive inversion), `overlay.cyr` (Enneagram, Jungian and Mystic typologies, walked through the `OverlaySystem` registry). None of them adds archetypes, so `profile_count()` is unaffected by any of them
9. Consumer (bhava, joshua, etc.) receives plain f64/enum output via profile accessors

## Type System

All values are i64. f64 trait/emphasis weights stored as IEEE 754 bit patterns. Use f64_* builtins for arithmetic and comparison.

- `ArchetypeProfile` is a `#derive(accessors) struct Profile` (40 i64 fields = 320 bytes, `domain` last at offset 312); `profile_new()` allocates via `xalloc` — checked allocation that aborts on OOM (ADR-009) — with defaults (traits=0.5, emphasis=0.5, breath=LATE_EXHALE, growth=DIFFERENTIATE, domain=UNSPECIFIED)
- Profile fields accessed via the generated `Profile_*` accessors (`prof_*` shims delegate to them); the `ProfLayout` offset constants are retained for loop-based range code
- Enums are integer constants (BREATH_UNITY=0, GROWTH_DIFFERENTIATE=0, etc.)
- Strings are null-terminated C string pointers
- Collections use `vec_new()`/`vec_push()`/`vec_get()` from Cyrius stdlib
