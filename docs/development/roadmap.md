# Roadmap

> avatara — **forward-looking only.** Shipped work lives in [CHANGELOG.md](../../CHANGELOG.md), which
> is also where every breaking change and its consumer note is recorded, per release. Nothing on this
> page describes what was done; everything describes what is not done yet and why.

## Planned

**Nothing is scheduled before 3.0.0.** Everything remaining is one of: breaking and banked for 3.0.0
(below), additive but unscheduled (backlog), or blocked on something no code change resolves.

## Blocked — Aboriginal Australian depth

**The constraint is engagement, not research, and the two are not substitutes.** Desk research
establishes that material is *already public*, which is checkable. It cannot establish that a people
*consents to this particular use*, which is what review is for. No community is engaged with this
project, so no figure can move from blocked to cleared by any amount of further reading. ADR-010
rule 8 states this; the point here is only that it is still true.

Two shapes of block, both live:

- **Express restriction.** A body publishes terms that a GPL-3.0 library shipping a profile — and
  inverting it through `shadow()` — does not satisfy. SWALSC over the Waugal is the worked case.
- **Declared gap.** A body publishes no reuse terms at all, so there is nothing to satisfy and nothing
  to breach. Silence is not permission. Wurundjeri Woi-wurrung is here, and its own Country Plan lists
  an ICIP policy as a 2026 action — so this one may resolve itself on their timetable, not ours.

**Six bodies publish contact details and none has been written to.** The per-figure provenance,
verbatim published terms, consent status and the request-for-information plan — one letter per body
rather than per figure — are tabulated in **[sourcing-register.md](sourcing-register.md)**. That file
is the working document; this section exists only to say the gate is shut and why.

If a community engages, the gate reopens and everything carried is revisable at their direction,
including removal.

### Also blocked, and cheaper to move

- **38 figures have no written sourcing provenance** — `inuit` (10), `lakota` (10), `haudenosaunee` (6),
  `anishinaabe` (12). Their sources *were* checked directly against Boas, Rasmussen and Converse and
  found sound; it was never written down, so each pass re-derives it. Entering them in the sourcing
  register needs no community engagement and is the one item in this section that desk work can close.
- **A channel can be excellent and undiscoverable.** The best Wurundjeri account of Bunjil and Waa sits
  inside a PDF; all 51 pages of that site's sitemap mention Waa zero times. Future sourcing passes must
  check publications pages and PDFs, not only crawlable HTML.

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
- **Further overlay systems.** The `OverlaySystem` registry takes a new family without touching any
  archetype, and the generic API enumerates it for free. Overlays are readings — plural, revisable,
  and allowed to disagree over the same figure — so this is the sanctioned home for any analytic grid
  that is not a people's own account of itself.

## v3.0.0 — Consolidation (breaking)

The major bump banks the API cleanups deferred through 2.x:

- Migrate `cross_tradition_match` / `find_mapping` from `0`-on-not-found to `Option` (the
  absence-vs-error distinction noted in 2.5.0).
- Drop the `prof_*` compat shims — consumers move to the derived `Profile_*` accessors (shims have
  eased the transition since 2.5.3).
- Retire the public `ProfLayout` offset enum from the consumer surface (internal-only).
- Formalize the overlay subsystem as first-class API. The "archetype + overlay engine" identity for v3.
- **Restore macrons on the Hawaiian and Māori names.** `Kane`, `Ku`, `Maui`, `Tane`, `Tu` and
  `Papatuanuku` should be `Kāne`, `Kū`, `Māui`, `Tāne`, `Tū` and `Papatūānuku` per ADR-004 point 2,
  which the tradition strings already honour (`Māori`). Renaming an archetype breaks `lookup()`.
- **Rename `incarnate_indigenous_*` and `incarnate_mystic_*`.** Neither `"Indigenous"` nor `"Mystic"`
  is a tradition string any more, but both survive as constructor-name prefixes and in
  `all_incarnate_indigenous()` / `all_incarnate_mystic()`. Renaming is breaking, which is why it waits.

## Declined

- **Affinity-graph caching** — a pointer-keyed cross-tradition cache regressed the bench
  (`cross_tradition_match` 49µs → 945µs); the construct-then-query access pattern misses a
  pointer-keyed cache. Revisit only with a non-pointer-keyed (index/name-based), bench-proven design.
- **Per-figure opt-outs inside `shadow()` / `compose()`.** Proposed for living people and for apical
  ancestors, and refused both times: a quiet exception is exactly what ADR-010 rule 9 exists to forbid.
  The machinery applies uniformly and the consequence is documented instead.

## Watch items

- **`history/context_all_traditions` grows quadratically with tradition count** — it walks every
  tradition calling `context_for_tradition()`, which linearly scans every mapping, so it is
  O(traditions × mappings). It is the one benchmark that moves on a tradition split rather than on a
  code change. A name- or index-keyed mapping lookup would flatten it if it ever matters.

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

> Consumers upgrading across a breaking release should read that release's entry in
> [CHANGELOG.md](../../CHANGELOG.md), which carries the consumer note. They are not duplicated here —
> a version-pinned note on a forward-looking page is stale the moment the next release lands.
