# Architecture Overview

> avatara — Divine archetype engine for AGNOS

## System Diagram

```
                    ┌──────────────────────────────────────────┐
                    │              avatara                      │
                    │     divine archetype engine               │
                    ├──────────────────────────────────────────┤
                    │                                          │
                    │  ┌────────────────────────────────────┐  │
                    │  │          20 Tradition Modules       │  │
                    │  │  kabbalah  angelic  hindu  olympian │  │
                    │  │  norse  egyptian  buddhist  celtic  │  │
                    │  │  mesopotamian  shinto  aztec  maya  │  │
                    │  │  yoruba  zoroastrian  taoist        │  │
                    │  │  polynesian  slavic  jain  sikh     │  │
                    │  │  incarnate (6 sub-traditions)       │  │
                    │  └──────────────┬─────────────────────┘  │
                    │                 │                         │
                    │  ┌──────────────┴─────────────────────┐  │
                    │  │       ArchetypeProfile              │  │
                    │  │  15 traits + 14 emphasis + breath   │  │
                    │  │  + growth + element + polarity      │  │
                    │  │  + tier + soul/spirit text          │  │
                    │  └──────────────┬─────────────────────┘  │
                    │                 │                         │
                    │  ┌──────────────┴──────┐  ┌──────────┐  │
                    │  │     compose          │  │ registry │  │
                    │  │  weighted blending   │  │ lookup   │  │
                    │  │  cross-tradition     │  │ query    │  │
                    │  └─────────────────────┘  └──────────┘  │
                    └──────────────────┬───────────────────────┘
                                       │
                    ┌──────────────────┴───────────────────────┐
                    │              Consumers                    │
                    │  bhava — emotion/personality bridge       │
                    │  joshua — NPC divine archetypes           │
                    │  kiran — game entities                    │
                    │  agnosai — agent theological depth        │
                    │  sankhya — ancient science archetypes     │
                    └──────────────────────────────────────────┘
```

## Core Principle

All traditions map to the same `ArchetypeProfile` output. A character can carry Kabbalistic + Hindu + Greek archetypes simultaneously — reinforcing archetypes amplify, conflicting archetypes create productive internal tension.

Avatara produces plain `f64`/enum outputs. It does not depend on bhava — bhava consumes avatara's output through a bridge module.

## Data Flow

1. Tradition module defines entities as Rust enums
2. `Archetype::profile()` returns an `ArchetypeProfile` with fully populated fields
3. `compose::compose()` blends multiple profiles with weighted averaging
4. `registry::query()` finds entities matching criteria
5. Consumer (bhava, joshua, etc.) receives plain `f64`/enum output
