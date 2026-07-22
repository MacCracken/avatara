# Roadmap

> avatara — forward-looking milestones only. Shipped work lives in [CHANGELOG.md](../../CHANGELOG.md).

## Shipped

- **2.4.x** — toolchain + hardening + language modernization: cyrius 3.10.0 → 6.0.40, `cyrius.cyml` manifest, modern CI/release, dist bundle, bench-on-every-release; NULL/overflow hardening; `+=`/`match` adoption; stdlib `f64_le`/`f64_ge`; spirit-emphasis (`PROF_SPIRIT`) collision fix.
- **2.5.x** — architecture: `Result<T, E>` error model (2.5.0); shadow aspect (2.5.1); `domain` field, all 362 archetypes assigned (2.5.2); native `#derive` `struct Profile` migration (2.5.3); security audit + CWE-690 `xalloc` hardening (2.5.4).
- **2.6.0** — The Solar Year: 25th "Solar" tradition (intercalary archetypes), landing at **366 archetypes (365 + the leap quarter)**.
- **2.7.0** — Canaanite & Etruscan micro-traditions (El/Baal/Asherah/Anat; Tinia/Uni/Menrva/Voltumna) → **374 archetypes, 27 traditions**.
- **2.8.0** — Role aspects: trait-derived role facets (`src/aspect.cyr`) — every one of the 374 archetypes gains selectable roles derived from its personality vector, with no per-archetype authoring. Purely additive; no profile or API behavior changed.
- **2.9.0** — Tarot Major Arcana (`src/tarot.cyr`): the 22 trumps as archetypes, each a *path* on the Tree of Life bridging the Kabbalah module (Hebrew letter + path number 11–32 + the two Sephiroth it joins; Golden Dawn attribution) → **396 archetypes, 28 traditions**. First module to bridge two existing systems.
- **2.10.0** — I Ching (`src/iching.cyr`): the 64 hexagrams of the King Wen sequence, each carrying its six lines and its two constituent trigrams (bagua), with element derived from the upper trigram and polarity from the yang-line balance → **460 archetypes, 29 traditions**. All 64 trigram pairings occur exactly once — a bijection the suite pins.
- **2.10.1** — performance: `similar_to()` switched from sort-then-trim to bounded top-k selection, removing an O(N²) insertion sort over the whole registry (`affinity/similar_to_5` 809 → 73 µs, −91%). Identical results and ordering; the cost no longer grows with N².
- **2.11.0** — World-traditions completion, organised as **named nations** rather than pan-continental buckets: Inuit (10), Lakota (10), Haudenosaunee (6), Anishinaabe (6), and a deliberately scoped Aboriginal Australian set (5, each attributed to its people); plus the outstanding Celtic healers Miach and Airmed → **499 archetypes, 34 traditions**. White Buffalo Calf Woman and Deganawidah remain in the existing "Indigenous" incarnate tradition and are not duplicated. Restricted Dreaming material is excluded by design — see the `aboriginal.cyr` header and the 2.11.x arc below.
- **Toolchain maintenance** — 2.7.2 (pin 6.1.34 → 6.2.11) and 2.8.1 (6.2.11 → 6.4.69) as standalone maintenance releases; 2.10.0 also carried the pin 6.4.69 → 6.4.70. Vendored stdlib re-resolved to the pinned snapshot each time, benches recorded.

The minors below sequence the former demand-gated backlog toward a 3.0.0 consolidation. (Role aspects took the original 2.8.0 slot, so the Tarot → overlays sequence shifted up one minor; Tarot shipped as 2.9.0 and I Ching as 2.10.0.)

## Planned — minors to 3.0.0

Each is additive and non-breaking (new archetypes / traditions / an additive
overlay layer). Historical-accuracy rule stands throughout: established
scholarly correspondences only, no inventions.

- **v2.12.0 — Archetype overlays** — the first cross-cutting layer *on top of* the archetype profiles: Enneagram (9 types) and the Jungian set (Hero, Shadow, Anima/Animus, Self, Trickster — composes with the existing `shadow()`). Additive new API; profiles unchanged.

## Backlog — additive, unscheduled

### The 2.11.x arc — Aboriginal Australian depth

2.11.0 shipped a deliberately narrow Aboriginal Australian set: five figures already
established in public, community-involved material, each attributed to its people, with
restricted Dreaming material excluded. Two follow-ups are queued behind that, in order:

- **v2.11.x — dedicated, community-reviewed sourcing.** Before any expansion, establish
  proper references: AIATSIS-published resources, the relevant land councils' and cultural
  centres' own material, and ideally review by or with the communities whose figures are
  represented. ICIP protocols are the governing consideration, not a formality. This is the
  gate on the item below rather than a nice-to-have alongside it.
- **v2.11.x — fuller coverage, gated on the above.** With sourcing established, widen from
  five figures toward 10-14 and, where the material supports it, split per-people rather
  than carrying one "Aboriginal Australian" tradition label. Not to be attempted from the
  older ethnographic record alone — a good deal of it was recorded without consent, and
  communities have asked that parts of it not be recirculated.

### Carried over from the 2.11.0 review

The 2.11.0 cultural-accuracy review raised three items that were deliberately not
fixed in passing, because each is a separate change rather than a correction:

- **Re-attribute the incarnate "Indigenous" bucket to named nations.** `incarnate.cyr`
  carries White Buffalo Calf Woman, Deganawidah, Quanah Parker and Wovoka — four figures
  from four different nations — under one pan-ethnic `"Indigenous"` tradition label. That
  predates 2.11.0 and is inconsistent with the named-nation approach the new modules use;
  `lakota.cyr` and `haudenosaunee.cyr` both have to defer to it awkwardly as a result.
  Re-attributing them (Lakota, Haudenosaunee, Comanche, Northern Paiute) also needs
  history mappings for the two new nations and retires the orphaned `Incarnate Indigenous`
  mapping. Note that Deganawidah is carried under the personal name; in many communities
  the Peacemaker's name is reserved for ceremonial use, so the title is preferable.
- **Anishinaabe selection balance.** With the wiindigoo removed the module carries the
  trickster and no counterweight from the material Anishinaabe communities most actively
  publish. The Nizhwaaswi Gagiikwewin (Seven Grandfather Teachings — wisdom, love, respect,
  bravery, honesty, humility, truth) are the obvious addition, as seven entries or one
  composite.
- **Separator convention.** `inuit.cyr` now uses an em dash in consumer-facing strings
  while the other four world-tradition modules use ` -- `; the wider codebase is split
  between the two. Worth settling one way and normalising.

### Other additive options not yet assigned to a version:

- **Tarot de Marseille attribution** — expose the older Tarot de Marseille numbering/attribution as an *alternative* view of the 22 trumps, **alongside — not replacing — the shipped Rider–Waite–Smith / Golden Dawn one** (`src/tarot.cyr`). The TdM predates the Golden Dawn esoteric overlay and differs notably: **VIII = Justice, XI = Strength (Force)** (the reverse of the shipped VIII Strength / XI Justice), plus its own pre-Golden-Dawn iconography and the earlier Éliphas Lévi / Oswald Wirth letter attributions. The shipped `tarot_*` data and the Kabbalah path bridge stay canonical; this would add a parallel layer (e.g. `tarot_marseille_number(i)` and/or a variant attribution accessor) so a consumer can select the deck tradition appropriate to its use. No change to existing profiles, API, or the Tree-of-Life bridge.

## v3.0.0 — Consolidation (breaking)

The major bump banks the API cleanups deferred through 2.x:

- Migrate `cross_tradition_match` / `find_mapping` from `0`-on-not-found to `Option` (the absence-vs-error distinction noted in 2.5.0).
- Drop the `prof_*` compat shims — consumers move to the derived `Profile_*` accessors (shims have eased the transition since 2.5.3).
- Retire the public `ProfLayout` offset enum from the consumer surface (internal-only).
- Formalize the overlay subsystem (2.12.0) as first-class API. The "archetype + overlay engine" identity for v3.

## Declined

- **Affinity-graph caching** — a pointer-keyed cross-tradition cache regressed the bench (`cross_tradition_match` 49µs → 945µs); the construct-then-query access pattern misses a pointer-keyed cache. Revisit only with a non-pointer-keyed (index/name-based), bench-proven design.

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
