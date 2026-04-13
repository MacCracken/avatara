# avatara

> **Avatara** (Sanskrit: अवतार — descent of the divine, manifestation of archetypes)

**Divine archetype engine** — theological and mythological personality mapping across traditions. Part of the [AGNOS](https://github.com/MacCracken) ecosystem.

[![License: GPL-3.0-only](https://img.shields.io/badge/license-GPL--3.0--only-blue.svg)](LICENSE)

## What It Does

Maps divine and mythological beings across world traditions to composable personality configurations. Not religion simulation — psychometric archetype mapping backed by trait math. Each entity produces an `ArchetypeProfile` (312 bytes) with trait weights, module emphasis, breath phase affinity, and growth direction.

**362 archetypes across 24 traditions.** Written in Cyrius, compiled by cc3 3.10.0+.

## Traditions

| Tradition | Module | Entities |
|-----------|--------|----------|
| **Kabbalah** | `kabbalah` | 10 Sephiroth (Kether through Malkuth) |
| **Angelic** | `angelic` | 7 Archangels, 9 Angelic Orders |
| **Hindu** | `hindu` | Trimurti (3), Devas (11), Avatars of Vishnu (10) |
| **Greek** | `olympian` | 15 deities (12 Olympians + Hades, Hestia, Persephone) |
| **Norse** | `norse` | 13 Aesir/Vanir gods |
| **Egyptian** | `egyptian` | 16 principal deities |
| **Buddhist** | `buddhist` | 7 Bodhisattvas, 5 Dhyani Buddhas |
| **Mesopotamian** | `mesopotamian` | 14 Sumerian/Babylonian deities |
| **Celtic** | `celtic` | 15 Tuatha De Danann & Insular Celtic deities |
| **Shinto** | `shinto` | 15 Japanese Kami |
| **Aztec** | `aztec` | 14 Aztec (Mexica) deities |
| **Maya** | `maya` | 12 Maya deities |
| **Yoruba** | `yoruba` | 14 Orishas |
| **Zoroastrian** | `zoroastrian` | 7 Amesha Spentas, 7 Zoroastrian beings |
| **Taoist** | `taoist` | 8 Immortals, 8 celestial deities |
| **Polynesian** | `polynesian` | 12 Polynesian/Hawaiian deities |
| **Slavic** | `slavic` | 12 pre-Christian Slavic deities |
| **Jain** | `jain` | 24 Tirthankaras |
| **Sikh** | `sikh` | 10 Sikh Gurus |
| **Finnish** | `finnish` | 14 Kalevala figures & Sami spirits |
| **Vodou** | `vodou` | 14 Lwa (Rada, Petwo, Ghede) |
| **Incarnate** | `incarnate` | 56 incarnate masters across 6 sub-traditions |

## Quick Start

```cyrius
include "avatara/src/lib.cyr"

alloc_init();

# Single archetype
var tip = kabbalah_tiphareth();
# prof_warmth(tip) = 0.8, prof_confidence(tip) = 0.8

# Lookup by name
var krishna = lookup("Krishna");
# prof_tradition(krishna) = "Hindu"

# Query by trait
var brave = query_min_trait(PROF_COURAGE, 0.9);
# vec of all entities with courage >= 0.9

# Compose across traditions
var w = vec_new();
vec_push(w, weighted_new(tip, 1.0));
vec_push(w, weighted_new(krishna, 0.8));
var blended = compose(w);

# Historical context
var ctx = context_for_tradition("Hindu");
# ctx_primary(ctx) = "Indus Valley", ctx_start(ctx) = -1500
```

## Design

All traditions map to the same `ArchetypeProfile` output:

- **TraitWeights** — 15 personality dimensions (0.0-1.0), maps 1:1 to bhava's `PersonalityProfile`
- **ModuleEmphasis** — which bhava modules this archetype amplifies (mood, energy, spirit, reasoning, etc.)
- **BreathAffinity** — position on the cosmic breath cycle (Unity through LateExhale through Unity)
- **GrowthDirection** — Differentiate, Integrate, Preserve, Transform, or Still
- **Element** — Fire, Water, Earth, Air, Aether, Light, Darkness, Storm, Mixed
- **Polarity** — Masculine, Feminine, Androgynous, Transcendent
- **CosmicTier** — Supreme, Primordial, Cosmic, Greater, Lesser, Demigod, Master

Archetypes are composable across traditions. A character can carry Kabbalistic Tiphareth + Hindu Vishnu + Greek Athena — reinforcing archetypes amplify, conflicting archetypes create productive internal tension.

## Affinity & Composition Intelligence (v2.3.0)

```cyrius
# How similar are two archetypes? (0.0 to 1.0)
var score = affinity(norse_thor(), olympian_ares());

# Find Thor's closest match across all other traditions
var match = cross_tradition_match(norse_thor());
# Returns the most similar archetype from a different tradition

# Top 5 most similar archetypes to any entity
var top5 = similar_to(norse_thor(), 5);

# Detect conflicting traits (delta > 0.4)
var conflicts = detect_conflicts(kabbalah_gevurah(), kabbalah_chesed());
# Returns: warmth (0.3 vs 0.9), patience (0.3 vs 0.8), humor (0.2 vs 0.6)...
```

## Relationship to AGNOS

```
avatara (this) — divine archetype profiles (362 entities, 24 traditions)
  |
  +-> bhava — emotion/personality engine (archetype overlay)
  +-> joshua — NPC divine archetypes for games
  +-> kiran — game entities
  +-> agnosai — agent personalities with theological depth
  +-> hadara — archetype-to-culture context
```

avatara produces plain f64/enum outputs. It does not depend on bhava — bhava consumes avatara's output through a bridge module.

## Build

Requires cc3 3.10.0+ (Cyrius compiler).

```sh
# Resolve dependencies
cyrius deps

# Build and run tests
cyrius build src/main.cyr build/avatara && ./build/avatara

# Run integration tests
cyrius test tests/avatara.tcyr

# Run benchmarks
cyrius test tests/avatara.bcyr
```

## License

GPL-3.0-only
