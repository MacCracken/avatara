# Roadmap

> avatara — Forward-looking milestones only. Completed work is in [CHANGELOG.md](../../CHANGELOG.md).

## Planned

> Note: 2.4.0–2.4.3 were a toolchain + hardening + language-modernization line
> (cyrius 3.10.0 → 6.0.40, NULL/overflow hardening, `+=`/`match` adoption,
> stdlib `f64_le`/`f64_ge`). The feature items below were not part of that line
> and have been re-bucketed accordingly.

### v2.5.0 — Result error model ✅ (shipped)

- [x] **`Result<T, E>` + `?`** — `lookup`/`lookup_in`/`compose`/`validate_profile` now return `Result` (`Ok(profile)` / `Err(AvataraError)`) instead of `0`-on-error / bare int codes; added `find_and_validate` demonstrating `?`. API-breaking for consumers (see CHANGELOG 2.5.0).

### Blocked — struct migration (waiting on cyrius)

- [ ] **`struct` + `#derive(accessors)` for `ArchetypeProfile`** — replace the manual 312-byte blob (offset enums + `store64`/`load64` + ~39 hand-written `prof_*` accessors) with a native struct. **Blocked**: cyrius caps structs at 32 fields and `Profile` has 39 — proposal filed at `cyrius/docs/development/proposals/2026-06-02-struct-field-cap-raise.md`. Resume when the cap is raised upstream (the named-field struct would also make the 2.4.5 `PROF_SPIRIT` offset-collision class of bug a compile error). When unblocked: `sizeof == 312` + offset-assertion test, mechanical `store64 → Profile_set_*` migration (~10k sites), `prof_*` compat shims for consumers.
- This migration was to precede the layout-changing items below (esp. the `domain` field); with it blocked, sequence those independently or wait.

### v2.5.1 — Shadow aspect ✅ (shipped)

- [x] **`shadow(profile)`** + **`is_shadow_of(a, b)`** (`src/shadow.cyr`) — involutive inversion: traits/emphases → `1.0 − v`; breath mirrored across unity; growth (`DIFFERENTIATE↔INTEGRATE`, `PRESERVE↔TRANSFORM`) and polarity (`MASCULINE↔FEMININE`) inverted; element/tier preserved; name → `"Shadow of …"`. 10 tests; involution verified.

### Affinity graph — evaluated, declined

- A pre-computed, **pointer-keyed** cross-tradition cache regressed the bench (`cross_tradition_match` 49µs → 945µs): the construct-then-query pattern passes fresh profile pointers that miss the cache, and the per-tradition build path is slower than the existing single-pass scan. Reverted; kept the original O(n) functions. **Revisit only with a non-pointer-keyed design** (index/name-based) that fits the access pattern and is bench-proven.

### Deferred — `domain` field (blocked-adjacent)

A categorical axis (War, Love, Death, Creation, …) orthogonal to the
trait/emphasis numbers. **Layout-changing** (312 → 320) + a 362-archetype
scholarly assignment the roadmap wanted folded into the struct migration (still
blocked on the cyrius 32-field cap). Resume alongside that migration, or do it
standalone on the manual layout if it's wanted sooner.

- [ ] `enum Domain` (settle the closed set; default `DOMAIN_UNSPECIFIED`) + `PROF_DOMAIN` field + `prof_domain()` + `query_domain()`.
- [ ] Assign a scholarly domain to all 362 archetypes (no inventions); layout assertion → 320.

### v2.6.0 — The Solar Year (362 → 365.25)

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
