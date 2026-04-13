# Roadmap

> avatara — Forward-looking milestones only. Completed work is in [CHANGELOG.md](../../CHANGELOG.md).

## Planned

### v2.3.0 — Composition Intelligence

- [ ] Cross-tradition affinity mapping (Shango ~ Thor ~ Indra ~ Perun)
- [ ] Affinity scoring between archetypes
- [ ] Archetype similarity search ("find entities most like this profile")
- [ ] Conflict detection (`AvataraError::Incompatible`)

### v2.4.0 — Structural Enrichment

- [ ] `domain` field (War, Love, Death, Creation, Knowledge, etc.)
- [ ] Cross-tradition affinity graph (related archetypes across traditions)
- [ ] Shadow aspect support (dark/inverted form of each archetype)

## Future (demand-gated)

- Tarot major arcana mapping (22 archetypes ~ 22 Tree of Life paths)
- I Ching hexagram personality mapping (64 archetypes)
- Enneagram integration (9 types as archetype overlays)
- Jungian archetype layer (Hero, Shadow, Anima/Animus, Self, Trickster)
- Additional traditions: Aboriginal Australian, Native American (specific nations), Inuit

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
