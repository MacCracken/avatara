# Roadmap

> avatara — Forward-looking milestones only. Completed work is in [CHANGELOG.md](../../CHANGELOG.md).

## Planned

### v2.4.0 — Structural Enrichment

- [ ] `domain` field (War, Love, Death, Creation, Knowledge, etc.)
- [ ] Cross-tradition affinity graph (pre-computed, stored)
- [ ] Shadow aspect support (dark/inverted form of each archetype)

### v2.5.0 — The Solar Year (362 → 365.25)

The joke: avatara at 362 archetypes is *just shy* of the tropical year (365.25 days). Land at the solar revolution. Three whole archetypes + one "quarter" (the leap-day / intercalary correction) — historically grounded across three themes, pick any mix:

- **A. Calendrical / solar correction (most literal fit)**
  - Mayan **Wayeb** — 5 unlucky nameless days ending the Haab year (extend `maya.cyr`)
  - Egyptian **Epagomenal Days** — 5 intercalary days (birthdays of Osiris, Horus, Set, Isis, Nephthys) added outside the 360-day civil year (extend `egyptian.cyr`)
  - Zoroastrian **Gatha days** — 5 intercalary days between Spenta Mainyu and Ahunavaiti (extend `zoroastrian.cyr`)
  - The "quarter" lands here naturally: any of the above represents the calendar's structural correction to the solar year

- **B. Complete under-represented traditions**
  - Polynesian: Pele-aspects, Kanaloa-aspects
  - Slavic: Mokosh-aspects, Rod
  - Celtic: Ogma, Miach, Airmed (Tuatha healers)

- **C. New micro-tradition**
  - Canaanite / Ugaritic: El, Baal, Asherah, Anat
  - Etruscan: Tinia, Uni, Menrva, Voltumna

Historical-accuracy rule stands: use established correspondences from scholarly sources, no inventions. Land on 365.25 cleanly or overshoot — either is fine.

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
