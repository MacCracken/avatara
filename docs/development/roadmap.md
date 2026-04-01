# Avatara Roadmap

> **Status**: Pre-1.0 | **Current**: 0.1.0

## Completed

### 0.1.0 — Scaffold (2026-03-31)

- [x] Common types: TraitWeights, ModuleEmphasis, BreathAffinity, GrowthDirection, ArchetypeProfile
- [x] Archetype trait for polymorphic profile generation
- [x] Kabbalah: 10 Sephiroth with full trait/emphasis/breath/growth profiles + soul/spirit text
- [x] Angelic: 7 archangels, 9 orders (enum + stub profiles)
- [x] Hindu: Trimurti with breath mapping, 7 Devas, 10 Avatars (enum + stub profiles)
- [x] Greek: 12 Olympians (enum + stub profiles)
- [x] Norse: 10 Aesir/Vanir (enum + stub profiles)
- [x] Egyptian: 12 principal deities (enum + stub profiles)
- [x] Buddhist: 7 Bodhisattvas (integration-trending), 5 Dhyani Buddhas (late inhale)
- [x] Error types with thiserror
- [x] Optional structured logging
- [x] Initial criterion benchmarks

## Backlog

### 0.2.0 — Full Tradition Profiles

- [ ] Angelic: full trait/emphasis/breath profiles for all 7 archangels + 9 orders
- [ ] Hindu: full profiles for all Devas, Avatars of Vishnu with yuga correspondences
- [ ] Greek: full profiles with planetary correspondences (Athena=Mercury, Aphrodite=Venus, etc.)
- [ ] Norse: full profiles with runic correspondences
- [ ] Egyptian: full profiles with decan/star correspondences
- [ ] Buddhist: full profiles for all Bodhisattvas and Dhyani Buddhas with wisdom-poison mappings

### 0.3.0 — Composition Engine

- [ ] `CompositeArchetype` — merge multiple archetypes with weighted influence
- [ ] Reinforcement detection (aligned archetypes amplify)
- [ ] Tension detection (conflicting archetypes create productive internal dynamics)
- [ ] Cross-tradition correspondence table (Michael ↔ Mars ↔ Ares ↔ Horus ↔ Tyr)

### 0.4.0 — Bhava Bridge Items

- [ ] `archetype_to_personality()` — ArchetypeProfile → bhava PersonalityProfile trait levels
- [ ] `archetype_to_identity()` — soul_text/spirit_text → bhava IdentityContent
- [ ] `archetype_mood_baseline()` — emphasis → mood vector baseline nudges
- [ ] `archetype_growth_direction()` — growth + breath → bhava GrowthLedger pressure
- [ ] `composite_to_personality()` — CompositeArchetype → blended profile with tension dynamics

### 0.5.0 — Extended Traditions

- [ ] Celtic: Tuatha Dé Danann, Dagda, Brigid, Morrigan, Lugh, tree correspondences
- [ ] Mesopotamian: Sumerian/Babylonian pantheon, Inanna, Enki, Marduk
- [ ] Mesoamerican: Aztec (Quetzalcoatl, Tezcatlipoca), Maya (Itzamna, Kukulkan)
- [ ] Shinto: Amaterasu, Susanoo, Tsukuyomi, kami classification
- [ ] African diaspora: Yoruba Orishas (Ogun, Yemoja, Shango, Oshun)

## Future (demand-gated)

- Tarot major arcana mapping (22 archetypes → 22 Tree of Life paths)
- I Ching hexagram personality mapping (64 archetypes)
- Enneagram integration (9 types as archetype overlays)
- Jungian archetype layer (Hero, Shadow, Anima/Animus, Self, Trickster)

## v1.0 Criteria

- [ ] All 7 initial traditions have complete profiles (no stub TODO)
- [ ] Composition engine handles cross-tradition blending
- [ ] Bhava bridge items implemented and tested
- [ ] All types serde roundtrip tested
- [ ] 80%+ test coverage
- [ ] Published on crates.io
