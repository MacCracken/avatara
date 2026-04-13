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
                    |  |        24 Tradition Modules (362)       |  |
                    |  |  kabbalah  angelic  hindu  olympian     |  |
                    |  |  norse  egyptian  buddhist  celtic      |  |
                    |  |  mesopotamian  shinto  aztec  maya      |  |
                    |  |  yoruba  zoroastrian  taoist            |  |
                    |  |  polynesian  slavic  jain  sikh         |  |
                    |  |  finnish  vodou                         |  |
                    |  |  incarnate (6 sub-traditions, 56)       |  |
                    |  +-------------------+--------------------+  |
                    |                      |                       |
                    |  +-------------------v--------------------+  |
                    |  |       ArchetypeProfile (312 bytes)      |  |
                    |  |  15 traits + 14 emphasis + breath       |  |
                    |  |  + growth + element + polarity           |  |
                    |  |  + tier + soul/spirit text               |  |
                    |  +-------------------+--------------------+  |
                    |                      |                       |
                    |  +--------+------+------+  +-----------+    |
                    |  | compose | hist | affi |  | registry  |    |
                    |  | blend   | 27   | nity |  | lookup    |    |
                    |  | weight  | maps | sim  |  | query     |    |
                    |  +---------+------+------+  +-----------+    |
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
2. Constructor calls `profile_new()`, sets fields via `store64(p + PROF_*, value)`, returns pointer
3. `all_*()` collection functions cache profiles on first call (lazy-init pattern)
4. `compose()` blends multiple profiles with weighted averaging
5. `registry::all_profiles()` aggregates all traditions into a single cached vec
6. `query_*()` functions filter profiles by trait, breath, growth, element, polarity, tier, civilization, era, year
7. `affinity()` scores similarity between profiles; `similar_to()` finds nearest neighbors; `cross_tradition_match()` maps across traditions; `detect_conflicts()` identifies trait tensions
8. Consumer (bhava, joshua, etc.) receives plain f64/enum output via profile accessors

## Type System

All values are i64. f64 trait/emphasis weights stored as IEEE 754 bit patterns. Use f64_* builtins for arithmetic and comparison.

- `profile_new()` allocates 312 bytes with defaults (traits=0.5, emphasis=0.5)
- Profile fields accessed via offset constants (PROF_WARMTH, PROF_COURAGE, etc.)
- Enums are integer constants (BREATH_UNITY=0, GROWTH_DIFFERENTIATE=0, etc.)
- Strings are null-terminated C string pointers
- Collections use `vec_new()`/`vec_push()`/`vec_get()` from Cyrius stdlib
