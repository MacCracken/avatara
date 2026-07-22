# Roadmap

> avatara — forward-looking milestones only. Shipped work lives in [CHANGELOG.md](../../CHANGELOG.md).

## Planned

**Nothing is scheduled before 3.0.0.** v2.14.0 was the last planned minor. Everything remaining is
either breaking (3.0.0, below), additive but unscheduled (backlog), or blocked on something no code
change resolves.

## Blocked — Aboriginal Australian depth

The arc closed at v2.14.0 with **six candidates researched and six refused**; the grounds are recorded
in the `src/aboriginal.cyr` header and the 2.14.0 CHANGELOG entry, and the standing rules are ADR-010.
None of that is revisited here. What matters going forward is the single reason the arc cannot move:

**The constraint is engagement, not research, and the two are not substitutes.** Desk research
establishes that material is *already public*, which is checkable. It cannot establish that a people
*consents to this particular use*, which is what review is for. No community is engaged with this
project. The refusal that makes this concrete is the Waugal: its sourcing is excellent — roughly a
thousand words in named Noongar Elders' own voices — and SWALSC's published terms require written
permission for "modification, distribution or publication", which is precisely what a GPL-3.0 library
that ships a profile and inverts it through `shadow()` does. More research cannot answer that. Asking
can.

Five bodies publish contact details, and nobody has written to any of them:

| body | question |
|---|---|
| SWALSC | Written consent for the Waugal, whose terms expressly cover modification and distribution |
| GLaWAC | Whether more of the Borun and Tuk account may be published, and on what terms |
| Injalak Arts | Reuse terms for Namarrkon — their site states none, so shipping would rest on silence |
| Barkandji Native Title Group | The Barkandji-preferred spelling, Ngatyi vs Ngatji, which their own publications split on |
| Jali LALC / the Dirawong Trust | Dirawong, the one borderline candidate never assessed to conclusion |

If a community engages, the gate reopens and everything carried is revisable at their direction,
including removal.

## Backlog — additive, unscheduled

- **Tarot de Marseille attribution** — expose the older Tarot de Marseille numbering/attribution as an
  *alternative* view of the 22 trumps, **alongside — not replacing — the shipped Rider–Waite–Smith /
  Golden Dawn one** (`src/tarot.cyr`). The TdM predates the Golden Dawn esoteric overlay and differs
  notably: **VIII = Justice, XI = Strength (Force)** (the reverse of the shipped VIII Strength / XI
  Justice), plus its own pre-Golden-Dawn iconography and the earlier Éliphas Lévi / Oswald Wirth
  letter attributions. The shipped `tarot_*` data and the Kabbalah path bridge stay canonical; this
  would add a parallel layer (e.g. `tarot_marseille_number(i)` and/or a variant attribution accessor)
  so a consumer can select the deck tradition appropriate to its use. No change to existing profiles,
  API, or the Tree-of-Life bridge.

## v3.0.0 — Consolidation (breaking)

The major bump banks the API cleanups deferred through 2.x:

- Migrate `cross_tradition_match` / `find_mapping` from `0`-on-not-found to `Option` (the
  absence-vs-error distinction noted in 2.5.0).
- Drop the `prof_*` compat shims — consumers move to the derived `Profile_*` accessors (shims have
  eased the transition since 2.5.3).
- Retire the public `ProfLayout` offset enum from the consumer surface (internal-only).
- Formalize the overlay subsystem (shipped 2.13.0) as first-class API. The "archetype + overlay
  engine" identity for v3.
- **Rename `incarnate_indigenous_*`.** The four figures stopped carrying an `"Indigenous"` tradition
  in 2.12.0, but their constructor names and the `all_incarnate_indigenous()` collection still say so.
  Renaming is breaking, which is why it waited for this bump.

## Declined

- **Affinity-graph caching** — a pointer-keyed cross-tradition cache regressed the bench
  (`cross_tradition_match` 49µs → 945µs); the construct-then-query access pattern misses a
  pointer-keyed cache. Revisit only with a non-pointer-keyed (index/name-based), bench-proven design.

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

> **Note for consumers at v2.14.0:** the Aboriginal figures' `tradition` strings changed from the
> single `"Aboriginal Australian"` to `Kunwinjku`, `Kulin` and `Gunaikurnai`. Anything reading
> `prof_tradition()` or `all_traditions()` for those figures needs updating;
> `traditions_for_civilization("Aboriginal Australia")` gathers all three.
