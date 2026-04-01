# Roadmap

> avatara — Forward-looking milestones only. Completed work is in [CHANGELOG.md](../../CHANGELOG.md).

## Planned

### v1.1.0 — Composition Intelligence

- [ ] Conflict detection (`AvataraError::Incompatible` — currently reserved)
- [ ] Affinity scoring between archetypes
- [ ] Archetype similarity search ("find entities most like this profile")
- [ ] Weighted composition with shadow dynamics

### v1.2.0 — Tradition Expansion

- [ ] Finnish/Sami tradition (Kalevala: Ukko, Tapio, Ilmarinen, Louhi)
- [ ] Vodou Lwa (Papa Legba, Baron Samedi, Erzulie — distinct from Yoruba)
- [ ] Expand Incarnate: Desert Fathers, Gregory Palamas, Thomas Merton, Attar, Al-Ghazali
- [ ] Cross-tradition affinity mapping (Shango ~ Thor ~ Indra ~ Perun)

### v1.3.0 — Structural Enrichment

- [ ] `domain` field (War, Love, Death, Creation, Knowledge, etc.)
- [ ] Cross-tradition affinity graph (related archetypes across traditions)
- [ ] Shadow aspect support (dark/inverted form of each archetype)

## Future (demand-gated)

- Tarot major arcana mapping (22 archetypes ~ 22 Tree of Life paths)
- I Ching hexagram personality mapping (64 archetypes)
- Enneagram integration (9 types as archetype overlays)
- Jungian archetype layer (Hero, Shadow, Anima/Animus, Self, Trickster)

## Dependencies for Consumer Integration

| Consumer | Status | Bridge |
|----------|--------|--------|
| bhava (emotion/personality) | Post-v2.0 | `ArchetypeProfile` → `PersonalityProfile` |
| joshua (NPC archetypes) | Planned | Direct `ArchetypeProfile` consumption |
| kiran (game entities) | Planned | Via joshua |
| agnosai (agent personalities) | Planned | Direct consumption |
| sankhya (ancient sciences) | Planned | Shared `IncarnateSage` / Vedic bridge |
