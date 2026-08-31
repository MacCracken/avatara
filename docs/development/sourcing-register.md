# Sourcing Register — Aboriginal Australian figures

Per-figure sourcing provenance and consent status, and the input to planning requests for information.

**Why this file exists.** The knowledge was real but scattered — across `src/aboriginal.cyr` comments
and three CHANGELOG entries — so every pass rediscovered it. The v2.14.5 audit re-derived the TLaWC
provenance question from scratch, and TLaWC's ICIP notice sat unnoticed for two releases despite being
live since February 2024. This is not new policy; it is existing findings made cumulative.

**The ceiling, stated once.** ADR-010 rule 8: desk research establishes that material is *already
public*, which is checkable. It cannot establish that a people *consents to this particular use*. No
amount of third-party or online research moves a figure from `never-asked` to `cleared`. Only a body
answering does. Everything in the "terms" column below is therefore evidence about **availability**,
never about **permission**.

Scope: the four Aboriginal Australian figures currently shipped, plus the refusals, so a later pass
does not re-research a closed question. The other four high-protocol modules (`inuit`, `lakota`,
`haudenosaunee`, `anishinaabe` — 38 figures) are **not yet entered**; see Gaps.

**Terms classes.** `restrictive-icip` = express ICIP notice requiring prior written consent ·
`restrictive-copyright` = rights reserved, no reuse terms granted · `approved-own-material` = a body
approved the material as *its own* public material, which is not a third-party licence ·
`permissive` = express reuse grant · `silent` = no terms stated at all.

**Consent statuses.** `never-asked` · `drafted` · `sent` · `pending` · `cleared` · `declined`.

## Shipped

| figure | people | channel | own voice? | terms class | consent |
|---|---|---|---|---|---|
| Ngalyod | Kunwinjku | Injalak Arts (Kunwinjku-governed, Gunbalanya); Maningrida Arts & Culture | **Yes** — shares *zero* five-word sequences with the settler curatorial text and tells a different account entirely (measured v2.14.0) | `restrictive-copyright` | `never-asked` |
| Bunjil | Kulin | **WWCHAC** (Wurundjeri Woi-wurrung RAP) — *re-cited from TLaWC at 2.14.7* | **Yes** — ~960 own-voice words, named Elders, William Barak quoted; **0** five-gram overlap with Wikipedia | `silent` | `never-asked` |
| Waa | Kulin | **WWCHAC** — *re-cited from TLaWC at 2.14.7* | **Yes** — ~400 own-voice words, 0.4% Wikipedia overlap | `silent` | `never-asked` |
| Tidilick | Gunaikurnai | GLaWAC, `gunaikurnai.org/our-culture/stories/` | **Yes** — GLaWAC publishes the telling in its own voice (Snake, not the settler-derived eel) | `approved-own-material` | `never-asked` |

### Verbatim terms

- **Injalak Arts** — `Copyright © Injalak Arts` and nothing further. Reserves rights rather than
  granting them. *Silence buys less than express terms would:* this same silence contributed to
  refusing **Namarrkon** at v2.14.0, while Ngalyod, resting on the same channel, ships.
- **WWCHAC (Wurundjeri Woi-wurrung)** — **no reuse terms at all.** The 80-page Country Plan contains
  zero occurrences of "copyright", "©", "all rights" or "reproduc", and the site sitemap carries no
  terms-of-use page. Its own plan lists **"ICIP policy developed 2026"** as a pending action, so this is
  a *declared gap*, not a settled position. The nearest statement is a protocol, not a licence: "It is
  cultural protocol to engage with Traditional Owners to seek permission to use their language and
  words." **Silence is not permission.**
- **TLaWC** — no longer the cited channel for either figure, but recorded because the notice is real and
  because TLaWC remains a Kulin RAP. Verified verbatim 2026-08-31 — "You may only deal with the content of this website with
  the prior written consent of TLaWC, the copyright owner and/or the Traditional Custodians of that
  ICIP, and with attribution." And: "You are not permitted to commercialise any plant, bushfood, or
  medicinal knowledge, cultural practices or otherwise appropriate ICIP shared with you through our
  website."
- **GLaWAC** — two sentences, in two sections. Above the stories: "The stories which appear on this
  website have been approved for use by the Elders and Knowledge Holders to ensure balanced and
  consistent public material is provided, and we thank them for their generosity." Under Cultural
  Protocols: "The spelling of those which appear on this website has been approved for use…" **Neither
  addresses third-party reuse.**

### Terms across the other Kulin RAPs, checked 2026-08-31

| body | on Bunjil/Waa | terms |
|---|---|---|
| Wadawurrung (WTOAC) | Yes, own voice, 0% Wikipedia | **Express restrictive** — "not to be reproduced without permission" |
| Dja Dja Wurrung (DJAARA) | Yes, own voice, 0% Wikipedia | **Express restrictive** — "cannot be reproduced in part or whole … without prior written approval" |
| Bunurong (BLCAC) | Nothing on Bunjil | — |
| Federation of Victorian Traditional Owner Corps | **Rule 4 false positive** — its Bunjil material is an explicit summary of Brough-Smyth (1878), footnoted to Brough Smyth, Curr, Dawson, Beveridge and Massola | not used |

**The name is unsettled across the four RAPs** — Wurundjeri "Bunjil", Wadawurrung "Bundjil", DJAARA
"Bundjiyl" (2025 Country Plan), TLaWC "Bundjil". This is the Ngatyi/Ngatji problem one level up, and
there is no confederacy-wide authority to appeal to: Kulin is an alliance, not a single authority.

### The pattern this table makes visible

**Not one of the four rests on anything resembling a licence.** Each channel either restricts reuse
outright, reserves rights without granting them, or approves material as its own. That is the honest
picture, and it was not visible while the same facts lived in prose.

It also exposes a standing asymmetry, recorded rather than resolved: the module **refused to add** the
Waugal over SWALSC's express terms and **refused Namarrkon** partly over Injalak's silence, while
**continuing to ship** Bunjil and Waa under TLaWC's express terms and Ngalyod under that same Injalak
silence. The defensible version is that declining to add costs nothing whereas removal erases a
people's creator figure from a library carrying everyone else's — but ADR-010 rule 5 says in terms that
"good sourcing is not a licence", so it is not a clean fit. Treated as an open tension.

## Refused, with grounds — do not re-research

| figure | people | ground | class |
|---|---|---|---|
| Waugal | Noongar | SWALSC terms expressly cover "modification, distribution or publication" — which is what shipping a profile and inverting it via `shadow()` is. Sourcing is *excellent*; this is not a thin-sourcing case | terms |
| Namarrkon | Kunwinjku | One channel, 102 words — below Borun (~150) and Tuk (~95), both already refused. Injalak states no reuse terms. Its own text closes on "guardian of the laws", a jurisdiction over living operative Law that `shadow()` would invert into a distributable Law-guardian with lawfulness set to 1−v | content + terms |
| Borun / Tuk | Gunaikurnai | Best channel cited anywhere here, but ~150 and ~95 published words. Borun's published life is an itinerary; Tuk performs no action, speaks no word, makes no decision in any source retrieved | content |
| Goorialla | — | Belongs to a book (Roughsey 1975), not to a people. The Lardil serpent is **Thuwathu** | channel |
| Ngatyi | Barkandji | Name unsettled among its own custodians (Ngatyi vs Ngatji) | channel |
| Baiame | — | Settler ethnography, and custodians asked for Baiame Cave to be closed to the public — a custodian asking for *less* access is the clearest signal there is | channel + consent |

**Two worked examples kept because each defeats a tempting shortcut.** *Uluru*: maximum public
circulation **and** an express written refusal — volume of publication proves nothing about consent.
*Wandjina*: excellent community sourcing **and** restriction — good sourcing is not a licence.

## Requests for information — the plan

One letter per body, not per figure. Drafting these is the only thing that moves any row's consent
status; no code change can.

| body | figures | question | status |
|---|---|---|---|
| **WWCHAC** (Wurundjeri Woi-wurrung) | Bunjil, Waa | Consent for two shipped profiles. Their ICIP policy is listed in their own plan as due in 2026 — so the ask is partly *when*, and whether the pending policy will cover third-party reuse of the kind here (a speakable persona, invertible via `shadow()`) | `never-asked` |
| TLaWC | — (no longer cited) | Courtesy notice that the citation moved to WWCHAC, and that its ICIP notice was the prompt for checking | `never-asked` |
| Injalak Arts | Ngalyod (+ Namarrkon, refused) | Reuse terms — the site states none. Would reuse as a speakable persona, invertible via `shadow()`, be acceptable? | `never-asked` |
| GLaWAC | Tidilick (+ Borun, Tuk, refused) | Does the Elders' approval contemplate third-party reuse? And may more of the Borun and Tuk account be published, on what terms? | `never-asked` |
| SWALSC | Waugal (refused) | Written consent, which their terms expressly require | `never-asked` |
| Barkandji Native Title Group | Ngatyi (refused) | The Barkandji-preferred spelling, which their own publications split on | `never-asked` |
| Jali LALC / Dirawong Trust | Dirawong (never assessed) | The one borderline candidate never assessed to conclusion | `never-asked` |

Every row reads `never-asked`. That is the true state, and making it a field rather than a paragraph
is the point: the ceiling is visible instead of implied.

## Gaps

- **38 figures unentered** — `inuit` (10), `lakota` (10), `haudenosaunee` (6), `anishinaabe` (12).
  Their provenance *was* checked at v2.14.5 against Boas, Rasmussen and Converse directly, and found
  sound, but it was never written down. Sourcing-statement density tells the story: `aboriginal` has
  ~2.5 statements per figure after three dedicated reviews; the other four average **0.15**.
- No row here has been checked against a body's *current* terms since the date noted. Terms change;
  the TLaWC notice appeared without anyone here noticing for two releases.
