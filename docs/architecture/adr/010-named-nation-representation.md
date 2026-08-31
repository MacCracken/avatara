# ADR-010: Named-Nation Representation of Indigenous Traditions

**Status**: Accepted
**Date**: 2026-07-22
**Supersedes**: ADR-004 point 6

## Context

ADR-004 (2026-03-31) decided, at point 6, that "Aboriginal Australian Dreamtime and broad 'Native
American' traditions are not codified, as many are explicitly not meant for outsider
systematization." That decision was made in good faith and it was wrong — not in its caution, which
was right, but in its shape.

A blanket exclusion is itself a flattening. It treats hundreds of distinct nations with distinct
languages, distinct Law and distinct cosmologies as one undifferentiated category, too sacred to
name — which is the same error, in the opposite direction, as treating them as one pantheon. The
category "Native American traditions" that ADR-004 declined to codify is not a thing that exists.
Lakota, Haudenosaunee, Anishinaabe and Inuit exist. So does the fact that some of what those
peoples hold is public and some is not, and that the line runs *through* each of them rather than
around all of them.

The exclusion also had a practical cost the ADR did not anticipate: the library shipped four
incarnate figures — White Buffalo Calf Woman, the Peacemaker, Quanah Parker and Wovoka — under a
pan-ethnic `"Indigenous"` tradition string, because there was no sanctioned way to name their actual
nations. The blanket policy produced exactly the flattening it was written to prevent.

v2.11.0 (Inuit, Lakota, Haudenosaunee, Anishinaabe, Aboriginal Australian), v2.12.0 (retiring
`"Indigenous"`), v2.12.1 (cultural-protocol corrections) and v2.14.0 (per-people tradition strings)
replaced it with the rules below. This ADR records them so a reader of ADR-004 is not misled.

## Decision

1. **Name the specific people, never a continent and never a pan-ethnic label.** The `tradition`
   field carries Lakota, Haudenosaunee, Anishinaabe, Kunwinjku, Kulin, Gunaikurnai — not
   "Indigenous", not "Native American", not "Aboriginal Australian". Where a continent-wide grouping
   is genuinely useful, it belongs in `history.cyr`'s civilization field, which already supports
   many-to-one: `traditions_for_civilization("Aboriginal Australia")` gathers the peoples without
   asserting they share a pantheon. A place several peoples share is not a tradition any of them
   belongs to, and the two fields must not be conflated.

2. **Community-published channels only.** A figure requires material published *by* the people, or
   with express approval, or authored by a member of that people. Scholarly quality is not the test;
   who is speaking is the test.

3. **Two false-positive patterns are named and refused.**
   (a) *Joint-management and land-agreement material is not cultural publication.* Park interpretive
   signage, plans of management and ILUAs are land administration with Indigenous participation. The
   proof is internal: the same agency silent at one park states plainly at Uluru that it cannot
   license ICIP.
   (b) *Thin community-published material must not be padded to fill the struct.* An
   `ArchetypeProfile` wants 15 traits, 14 emphases, five enums and two paragraphs. Where a community
   has published a hundred words, the rest would be authored here — which is how misattribution
   enters. An impeccable channel with insufficient content is a reason to ask that community for
   more, never a reason to write it yourself.

4. **Check channel ownership *and* text provenance.** Established at v2.14.0 and the least obvious
   rule here. An Indigenous-owned publisher does not make the words it reprints a community voice. A
   Mirarr-owned art centre was found republishing a settler curator's text at 89% verbatim — the
   curator describing himself in the source document as "an outsider", under his own warning that
   some of its content was "potentially inaccurate or generic or stereotyped". Ownership was
   checked; provenance was not. Both must be.

5. **Volume of publication proves nothing about consent, and good sourcing is not a licence.** Uluru
   has maximum public circulation and an express written refusal. The Wandjina have excellent
   community sourcing *and* enforced depiction rights. Neither the abundance nor the quality of
   material substitutes for permission.

6. **Where a representative body's account differs from a settler ethnographer's, the body's account
   wins** — including where the ethnographic version is older, fuller, or more widely reprinted.

7. **Exclusions are recorded with their reasons, in the module.** "We looked and did not add" is a
   result. An unrecorded exclusion gets re-litigated; a wrongly-recorded one gets re-litigated from a
   false premise, which is worse. Both have happened here and both are now fixed in place.

8. **Community review is a real gate, and research cannot satisfy it.** Desk research establishes
   that material is *public*, which is checkable. It cannot establish that a people *consents to
   this particular use*, which is what review is for. Where no community is engaged, the honest
   status is blocked — not "sourced well enough to proceed".

   *Recorded at 2.14.8:* rule 8 is a **research standard, not a ship gate**, and the library does not
   read it as one. All four Aboriginal figures ship with consent status `never-asked`, as do the 38
   figures in the other high-protocol modules. What the rule governs is what may be *claimed* and what
   may be *added*: it is why six candidates were refused at v2.14.0, and why nothing carried here is
   described as consented-to. Every status is tabulated in `docs/development/sourcing-register.md`.

9. **Library machinery applies uniformly, and the consequence is documented.** `shadow()` inverts
   every profile and `compose()` blends across traditions; no figure is silently exempted. A library
   with quiet exceptions is harder to reason about than one that is uniform and says plainly what it
   does. Modules carrying figures whose sources did not contemplate that say so in their headers.

   *Scope note, 2.14.8:* "modules … say so in their headers" is currently true of `src/aboriginal.cyr`
   and `src/incarnate.cyr` only. The other four high-protocol modules — `inuit`, `lakota`,
   `haudenosaunee`, `anishinaabe` — carry no such disclosure. That is a gap in the practice, not a
   change of decision; the rule stands as written.

## Consequences

- Traditions are absent for stated, specific reasons rather than by category. Six Aboriginal
  Australian figures were researched for v2.14.0 and all six refused, each on its own ground — a
  larger research investment than adding them would have taken, producing zero additions and a
  written record of why.
- The bar is high enough that it is routinely *not met* by material that looks authoritative. That
  is the intended behaviour, not a failure of the process.
- Rule 8 means some work terminates in "someone must write to this organisation", which no code
  change resolves. `src/aboriginal.cyr` currently names three such bodies.
- ADR-004's other six points stand unchanged.
