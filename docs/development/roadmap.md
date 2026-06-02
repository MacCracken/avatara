# Roadmap

> avatara — Forward-looking milestones only. Completed work is in [CHANGELOG.md](../../CHANGELOG.md).

## Planned

> Note: 2.4.0–2.4.3 were a toolchain + hardening + language-modernization line
> (cyrius 3.10.0 → 6.0.40, NULL/overflow hardening, `+=`/`match` adoption,
> stdlib `f64_le`/`f64_ge`). The feature items below were not part of that line
> and have been re-bucketed accordingly.

### v2.5.0 — Architecture Modernization

Deferred from the 2.4.x sweep because the blast radius is too large for a patch
line (see CHANGELOG 2.4.x). Breaking at the layout/API level — minor bump.
**Do the struct migration first** — it rewrites the field-access surface that
every 2.5.x item below builds on, so landing it first avoids re-doing the
domain-field and shadow work against the old `store64`/offset idiom.

- [ ] **`struct` + `#derive(accessors)` for `ArchetypeProfile`** — replace the manual 312-byte blob (offset enums + `store64`/`load64` + ~39 hand-written `prof_*` accessors) with a native struct. ~10k `store64(p + PROF_*)` write sites across 23 files. Requires: a `sizeof(Profile) == 312` + offset-assertion test, a mechanical migration, and `prof_*` compat shims so consumers (bhava/joshua/kiran/agnosai/hadara) don't break.
- [ ] **`Result<T, E>` + `?` for `error.cyr`** — replace integer error codes / `0`-on-error returns with tagged sum-type results. API-breaking for consumers; pair with the accessor migration.

### v2.5.1 — `domain` field (slipped from 2.4.0)

A categorical axis orthogonal to the trait/emphasis numbers: what an archetype
is *about*. **Layout-changing** (adds one i64 field → profile grows 312 → 320
bytes), so it lands right after / folded into the 2.5.0 struct migration to
avoid a second pass over all constructors + a second layout-assertion bump.

- [ ] `enum Domain` in `types.cyr` — War, Love, Death, Creation, Knowledge, Order, Chaos, Nature, Fate, Trickery, Healing, … (settle the closed set first; default `DOMAIN_UNSPECIFIED`).
- [ ] Add the field to `ArchetypeProfile` (struct field post-migration), update `profile_new()` default, add `prof_domain()` (+ derived setter).
- [ ] Assign a domain to each of the 362 archetypes across the 23 tradition modules (scholarly correspondence, no inventions).
- [ ] `query_domain(domain)` registry filter + count helper.
- [ ] Tests: per-tradition domain spot-checks; layout assertion updated to 320; `query_domain` coverage.
- **Acceptance:** every archetype has a non-default domain; `sizeof == 320` asserted; consumers still build against the bundle.

### v2.5.2 — Cross-tradition affinity graph (pre-computed, stored)

Today `cross_tradition_match`/`cross_tradition_matches` rescan `all_profiles()`
on every call (O(n) per query). Pre-compute the cross-tradition best-match graph
once and serve lookups from it. **Additive — no layout change.**

- [ ] Lazy-init `affinity_graph()` building, per profile, its best match in each *other* tradition (reuse `affinity()` + the existing sort).
- [ ] `cross_tradition_match`/`cross_tradition_matches` read the cached graph instead of rescanning; keep signatures identical.
- [ ] Optional public accessor to enumerate the stored edges for a profile.
- [ ] Bench: `affinity/cross_tradition_match` must improve vs the 2.4.x baseline in `bench-history.csv` (record the delta); guard the build/cache cost.
- **Acceptance:** identical results to the current scan (regression test cross-checks graph vs brute force on a sample), measurable speedup, no new warnings.

### v2.5.3 — Shadow aspect support

The dark/inverted form of each archetype. Spec as a **pure derivation
function** (no stored fields, no layout change) so it composes with everything.

- [ ] `shadow(profile)` → new `ArchetypeProfile` with defined inversion semantics: traits/emphases → `1.0 - value`; polarity flipped (masculine↔feminine, transcendent fixed); breath mirrored across the cosmic-breath cycle; growth direction inverted (differentiate↔integrate, transform↔preserve); element/tier preserved; name/soul/spirit prefixed/marked as shadow.
- [ ] Settle and document each inversion rule before coding (this is the design-sensitive part — get the breath/growth mirrors right).
- [ ] `is_shadow_of(a, b)` convenience check.
- [ ] Tests: `shadow(shadow(x))` ≈ `x` (involution within rounding); shadow of a high-warmth archetype is low-warmth; respectful-representation review of shadow naming/text.
- **Acceptance:** involution property holds; no tradition is trivialized by its shadow text.

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
