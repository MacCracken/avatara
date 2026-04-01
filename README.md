# avatara

> **Avatara** (Sanskrit: अवतार — descent of the divine, manifestation of archetypes)

**Divine archetype engine** — theological and mythological personality mapping across traditions. Part of the [AGNOS](https://github.com/MacCracken) ecosystem.

[![License: GPL-3.0-only](https://img.shields.io/badge/license-GPL--3.0--only-blue.svg)](LICENSE)

## What It Does

Maps divine and mythological beings across world traditions to composable personality configurations. Not religion simulation — psychometric archetype mapping backed by trait math. Each entity produces an `ArchetypeProfile` with trait weights, module emphasis, breath phase affinity, and growth direction.

## Traditions

| Tradition | Module | Entities |
|-----------|--------|----------|
| **Kabbalah** | `kabbalah` | 10 Sephiroth (Kether → Malkuth) |
| **Angelic** | `angelic` | 7 Archangels, 9 Angelic Orders |
| **Hindu** | `hindu` | Trimurti, 11 Devas, 10 Avatars of Vishnu |
| **Greek** | `olympian` | 15 deities (12 Olympians + Hades, Hestia, Persephone) |
| **Norse** | `norse` | 13 Aesir/Vanir gods |
| **Egyptian** | `egyptian` | 16 principal deities |
| **Buddhist** | `buddhist` | 7 Bodhisattvas, 5 Dhyani Buddhas |
| **Mesopotamian** | `mesopotamian` | 14 Sumerian/Babylonian deities |
| **Celtic** | `celtic` | 15 Tuatha Dé Danann & Insular Celtic deities |
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
| **Incarnate** | `incarnate` | 44 incarnate divine figures across 5 traditions |

## Quick Start

```rust
use avatara::Archetype;
use avatara::kabbalah::Sephira;

let tiphareth = Sephira::Tiphareth.profile();
println!("{}: {}", tiphareth.name, tiphareth.description);
// Tiphareth: Beauty — harmony and balance, the heart of the tree, solar center

// Breath phase maps to cosmic cycle position
assert_eq!(tiphareth.breath, avatara::BreathAffinity::LateExhale);

// Trait weights ready for bhava personality mapping
assert!(tiphareth.traits.warmth > 0.5);
assert!(tiphareth.traits.confidence > 0.5);
```

## Design

All traditions map to the same `ArchetypeProfile` output:

- **TraitWeights** — 15 personality dimensions (0.0–1.0), maps 1:1 to bhava's `PersonalityProfile`
- **ModuleEmphasis** — which bhava modules this archetype amplifies (mood, energy, spirit, reasoning, etc.)
- **BreathAffinity** — position on the cosmic breath cycle (Unity → LateExhale → Unity)
- **GrowthDirection** — Differentiate, Integrate, Preserve, Transform, or Still

Archetypes are composable across traditions. A character can carry Kabbalistic Tiphareth + Hindu Vishnu + Greek Athena — reinforcing archetypes amplify, conflicting archetypes create productive internal tension.

## Relationship to AGNOS

```
avatara (this) — divine archetype profiles
  → bhava — emotion/personality engine (post-v2.0 bridge)
  → joshua — NPC divine archetypes for games
  → agnosai — agent personalities with theological depth
```

avatara produces plain f64/enum outputs. It does not depend on bhava — bhava consumes avatara's output through a bridge module.

## License

GPL-3.0-only
