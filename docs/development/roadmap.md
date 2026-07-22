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
- **2.12.0** — World traditions, part 2: the three items the 2.11.0 review deferred. The pan-ethnic `"Indigenous"` label is retired — White Buffalo Calf Woman, the Peacemaker, Quanah Parker and Wovoka now carry Lakota, Haudenosaunee, Comanche and Northern Paiute (which also fixed an orphaned history mapping that had filed all four under the Tonga Empire). The seven Grandfather Teachings (Nizhwaaswi Gagiikwewin) join `anishinaabe.cyr`, restoring the balance lost when the wiindigoo was dropped. Separators normalised to the em dash codebase-wide → **504 archetypes, 35 traditions**.
- **Toolchain maintenance** — 2.7.2 (pin 6.1.34 → 6.2.11) and 2.8.1 (6.2.11 → 6.4.69) as standalone maintenance releases; 2.10.0 carried 6.4.69 → 6.4.70 and 2.11.0 carried 6.4.70 → 6.4.71. Vendored stdlib re-resolved to the pinned snapshot each time, benches recorded.

The minors below sequence the former demand-gated backlog toward a 3.0.0 consolidation. (Role aspects took the original 2.8.0 slot, so the Tarot → overlays sequence shifted up one minor; Tarot shipped as 2.9.0 and I Ching as 2.10.0.)

## Planned — minors to 3.0.0

Each is additive and non-breaking (new archetypes / traditions / an additive
overlay layer). Historical-accuracy rule stands throughout: established
scholarly correspondences only, no inventions.

- **v2.13.0 — Archetype overlays** — the first cross-cutting layer *on top of* the archetype profiles: Enneagram (9 types) and the Jungian set (Hero, Shadow, Anima/Animus, Self, Trickster — composes with the existing `shadow()`). Additive new API; profiles unchanged.
- **v2.14.0 — Aboriginal Australian depth** — widen `src/aboriginal.cyr` beyond the four figures 2.11.0
  shipped, on the basis of documented public, community-involved sourcing. See the arc below for what is
  established and what is blocked.

## Backlog — additive, unscheduled

### The v2.14.0 arc — Aboriginal Australian depth

**Desk research is complete** (2026-07-22, five research angles plus an adversarial provenance
pass, 159 figure assessments). Results below. The defects it found in already-shipped entries
were fixed in 2.12.1; Baiame moved from held to a stated exclusion in the same release.

**Shippable on the research alone — six profiles, in this order and not before step 4:**
- **Goorialla** (Lardil, Mornington Island) — the strongest candidate in the survey, and the only
  one clearing the bar on its strongest limb: authored and illustrated by Goobalathaldin Dick
  Roughsey, a Lardil man, in *The Rainbow Serpent* (1975), CBCA Picture Book of the Year, still in
  print, in primary curricula via Reading Australia. Attribute as "as published by Goobalathaldin
  Dick Roughsey", not as "the Lardil Law".
- **Borun** (pelican) and **Tuk** (musk duck), Gunaikurnai — two profiles, carried as a pair
  because the account *is* their meeting. GLaWAC's own Stories & Songlines page under its express
  Elder-approval statement. Blocked behind the shadow() decision below: they are the apical
  ancestors of every living Gunaikurnai person.
- **Wagyl** (Noongar) — Whadjuk Noongar public interpretation on the Derbarl Yerrigan and at
  Kings Park.
- **Namarrkon**, the Lightning Man (Kunwinjku) — same Aboriginal-owned art-centre channel as Ngalyod.
- **Ngatyi** (Barkandji), one paired entry — gated on confirming the Barkandji-preferred spelling
  with the Barkandji Native Title Group, because "Ngatji" is also a Ngarrindjeri word for a totemic
  relation and this library is name-keyed through `lookup()`.

Seven more sit at borderline behind a specific nameable step and must not be assumed to convert:
Budj Bim and Dirawong are the two likeliest; Ngurunderi, Tjilbruke, Akurra, the Mimih and the
Wurundjeri Birrarung trio are weaker. Everything else assessed is excluded.

**Step 4 is the gate, and it is a code change: the per-people tradition split.** Set `tradition`
to the people — Kunwinjku, Kulin, Gunaikurnai, Lardil, Noongar — rather than one continent-wide
"Aboriginal Australian" string. Do this *before* expanding, because the arithmetic runs the wrong
way: four entries under one continent-wide label is a rounding error, ten is a pantheon. It also
fixes a real artifact rather than relabelling one — `cross_tradition_match()` returns exactly one
best match per tradition string, so the library currently asserts "the Aboriginal Australian
equivalent of Thor is X". Split per people and it returns a Kunwinjku match and a Gunaikurnai match
separately, which is true. Cost: `by_tradition()` is a plain `streq` and is free; `history.cyr`
needs one mapping per people; `tests/avatara.tcyr` pins the count, the `all_traditions()` presence
and the history context; `all_traditions()` output shape changes for every consumer; `dist/` must be
regenerated. A deliberate minor with a CHANGELOG entry, not a drive-by edit. Do **not** add a
`people` field — `ArchetypeProfile` is a fixed 40-field / 320-byte layout with a pinned assertion
test and a committed dist bundle.

**OPEN DECISION — `shadow()` and `compose()`.** No source pass raised this; it was found in the
code. `shadow()` emits a profile named "Shadow of <name>" with every trait inverted and polarity
flipped; `compose()` blends across traditions by weight. So the library will generate "Shadow of
Borun", and will flip Tuk — the mother of the five Gunaikurnai clans — from feminine to masculine.
No channel cited anywhere in the survey contemplated that: not GLaWAC's Elder approval, not an art
centre's product text. **This already applies to the four figures carried today**, so it is not an
argument about expansion; it is an argument that the library must state what it does to a profile
once one exists, and possibly opt some profiles out of `shadow()` and `compose()` at code level.
Options: (a) do nothing and document it; (b) an opt-out flag honoured by `shadow()`/`compose()`;
(c) restrict the inversion to trait values and leave name and polarity alone. This is the single
most substantive gap in the module's ethical apparatus and it is the maintainer's call.

**STILL BLOCKED — community review.** Desk research established that material is *already public*,
which is checkable. It cannot establish that a people *consents to this particular use*, which is
what review is for, and no community is engaged with this project. Two courtesy emails are the only
open research actions: the Barkandji Native Title Group (Ngatyi spelling) and Jali LALC or the
Dirawong Trust (Dirawong). If a community ever engages, the gate reopens and everything here is
revisable at their direction, including removal.

**Two false-positive patterns, now encoded in the module header.** (1) Joint-management and
land-agreement material — park interpretive signage, plans of management, ILUAs — is land
administration with Aboriginal participation, not a people publishing in its own voice; this alone
sank Warramurrungundji, Almudj, Bolung, Gurangatch and Mirragan, Akurra and Gulaga. (2) Thin
community-published material must not be padded to fill the 15-trait struct; an impeccable channel
with three sentences behind it is a reason to ask that community for more, never to write it
yourself.

### Carried over from the 2.11.0 review

All three items shipped in 2.12.0: the `"Indigenous"` re-attribution, the Seven Grandfather
Teachings, and the separator normalisation. One follow-on remains:

- **`incarnate_indigenous_*` function names are now stale.** The four figures no longer carry
  an "Indigenous" tradition, but their constructor names and the `all_incarnate_indigenous()`
  collection still say so. Renaming them is a breaking API change and belongs in v3.0.0.

### Other additive options not yet assigned to a version:

- **Tarot de Marseille attribution** — expose the older Tarot de Marseille numbering/attribution as an *alternative* view of the 22 trumps, **alongside — not replacing — the shipped Rider–Waite–Smith / Golden Dawn one** (`src/tarot.cyr`). The TdM predates the Golden Dawn esoteric overlay and differs notably: **VIII = Justice, XI = Strength (Force)** (the reverse of the shipped VIII Strength / XI Justice), plus its own pre-Golden-Dawn iconography and the earlier Éliphas Lévi / Oswald Wirth letter attributions. The shipped `tarot_*` data and the Kabbalah path bridge stay canonical; this would add a parallel layer (e.g. `tarot_marseille_number(i)` and/or a variant attribution accessor) so a consumer can select the deck tradition appropriate to its use. No change to existing profiles, API, or the Tree-of-Life bridge.

## v3.0.0 — Consolidation (breaking)

The major bump banks the API cleanups deferred through 2.x:

- Migrate `cross_tradition_match` / `find_mapping` from `0`-on-not-found to `Option` (the absence-vs-error distinction noted in 2.5.0).
- Drop the `prof_*` compat shims — consumers move to the derived `Profile_*` accessors (shims have eased the transition since 2.5.3).
- Retire the public `ProfLayout` offset enum from the consumer surface (internal-only).
- Formalize the overlay subsystem (2.13.0) as first-class API. The "archetype + overlay engine" identity for v3.

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
