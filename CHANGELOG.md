# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [2.14.3] — 2026-08-31

First of several sweep-and-repair releases. This one is **correctness and integrity only** — no content
corrections, no `tradition` string changes, nothing a consumer can observe except that calls which used
to segfault now return. The audit behind it graded 218 findings across the whole project; the content
and cultural-protocol repairs are deliberately held for 2.14.4 and 2.14.5 so that a crash fix and a
theological correction are never reviewed in the same diff.

### Fixed

- **Seven public functions segfaulted on a profile `validate_profile()` had just certified as `Ok`.**
  `profile_new()` leaves all five string fields NULL, and `validate_profile()` checks the profile
  pointer and the 29 trait/emphasis ranges — never string presence. So the documented
  build-then-populate flow produced a validated profile that killed the process on first use.
  `shadow()`, `compose()`, `similar_to()`, `cross_tradition_match()`, `cross_tradition_matches()`,
  `context_for_profile()` and `profile_aspect_weight()` all died this way (CWE-476), each reproduced at
  exit 139 before the fix and asserted against afterwards.
  The repair is a class fix, not seven patches: new `safe_strlen()` / `safe_streq()` primitives in
  `src/types.cyr`, applied at **every** site in `src/` that reads a profile string — 18 of them across
  `shadow`, `compose`, `affinity`, `registry`, `history`, `overlay`, `aspect`, `tarot` and `iching`.
  Only true literals still call the raw primitives. `safe_streq(0, 0)` is deliberately **false**: two
  profiles with unset names are not thereby the same archetype, which is what `streq(0, 0)` had been
  claiming inside `similar_to()`'s self-skip.
  Finding the class mattered — patching only what the first three reproductions touched would have left
  `similar_to`, both `cross_tradition_*` functions and `context_for_profile` live, and a grep for
  `prof_name`/`prof_tradition` still missed `affinity.cyr:174`, where the NULL arrives through a local.
- **`validate_profile()` certified NaN as in-range.** Every ordered comparison against NaN is false, so
  `f64_lt(v, 0.0)` and `f64_gt(v, 1.0)` both returned 0 and a profile whose traits were **all NaN**
  validated `Ok`. `require_unit_range` now rejects NaN explicitly with `f64_eq(v, v) == 0` — the same
  test `compose()` has always applied to its weights, which is why compose caught what the validator did
  not. `validate_profile`'s doc comment now records the contract it actually enforces: field ranges, not
  string presence.
- **The layout test pinned the constant, not the struct.** It asserted `PROF_SIZE == 320` — a ProfLayout
  enum value compared to a literal — and never `sizeof(Profile)`. Appending a 41st field to
  `struct Profile` left the suite green at 295/295 while every generated `Profile_set_<41st>` wrote 8
  bytes past `profile_new()`'s `xalloc(PROF_SIZE)`, the library's one and only profile allocation site.
  Three documents claimed this test pinned `sizeof`; it did not exist. Now it does, and the mutation
  fails loudly.
- **The traditions-vs-typologies test did not enforce its own invariant.** CLAUDE.md's strongest
  structural rule is guarded by a registry walk that compared byte-exact and case-sensitive, and both
  realistic evasions passed it: `overlay_label()` returns the lowercase key form, so an archetype named
  `"Hero"` never matched `"hero"`, and a tradition string of `"Jungian Archetypes"` never equalled
  `"Jungian"`. Verified by injecting five Jungian archetypes with a tradition string — suite stayed
  green. The comparison is now case-insensitive, and a tradition may not even *contain* a registered
  system's name. Substring matching is applied to **traditions only, never to archetype names**:
  `enneagram_title(ENN_PEACEMAKER)` is `"The Peacemaker"`, which is also Deganawidah in
  `src/incarnate.cyr`, a real Haudenosaunee figure carried under ADR-010. Normalising both sides — the
  obvious fix — would have flagged him as a typology collision.
- **Six `history.cyr` mappings filed traditions under the wrong civilization, or as extinct.**
  Kabbalah was filed under **Phoenicia**, putting Jewish mysticism under the Canaanite civilization;
  it now carries Judea / Medieval Iberia / Ottoman Empire, tracking its own note's arc. Celtic religion
  was filed under the **Kingdom of France** — a polity founded centuries after the tradition it was
  meant to place — now Gaelic Ireland / Medieval Wales / Gaul. **Yoruba** and **Haitian Vodou** were
  both filed under the **Ghana and Songhai Empires**, states of a different region and people; Yoruba
  now carries Ife / Oyo Empire, and Vodou carries Saint-Domingue / Haiti / Dahomey in the Modern era
  rather than the Middle Ages, which its own 1600 start date already contradicted.
  **Maya was recorded as ending in 1500** — coded extinct, though the last independent Maya polity fell
  in 1697 and the 260-day *cholq'ij* count has been kept by highland *aj q'ij* daykeepers without
  interruption since; it is now `LIVING`. The **Aztec** note's "destroyed by Spanish conquest" is
  corrected the same way: the Mexica state fell in 1521, Nahua peoples did not.
- **The README Quick Start did not compile for its intended reader.** It told consumers to
  `include "avatara/src/lib.cyr"`, but `src/lib.cyr` resolves its own includes relative to the working
  directory, so it only builds from inside the avatara repo. From a consumer's own project root it
  fails on the first line. It now includes `dist/avatara.cyr`, the committed consumer bundle that
  `cyrius.cyml` has always described as exactly that. Compiling the snippet from the repo root cannot
  detect this — it has to be built from somewhere else.
- **`src/lib.cyr`'s header is shipped API documentation** — `cyrius.cyml` bundles it as the last module
  of `dist/avatara.cyr`, so it is what consumers read — and it stated **497 archetypes across 34
  traditions** (actual: 504/37), `celtic - 15` (17), `incarnate - 44+` (56), and omitted **ten shipped
  modules** from its list, five of them tradition modules carrying 40 archetypes. All corrected, and the
  list is now checked against the file's own `include` lines.
- **README said `incarnate` spans six traditions; it spans nine** — Mystic, Hindu, Buddhist, Vedic,
  Taoist, plus Lakota, Haudenosaunee, Comanche and Northern Paiute. The README's own table three lines
  below already enumerated all nine. `docs/doc-health.md` repeated the wrong count while certifying the
  README fresh; both fixed.
- **The 2.14.2 entry said a preventive measure was absent that the same release shipped.**
  `release.yml` publishes that text verbatim as the GitHub release body, so it was wrong in public.

### Testing

- **295 -> 311 assertions.** The new ones cover each crash above, `safe_streq(0,0)`/`safe_strlen(0)`,
  NaN rejection and its error code, and the two repaired pins.
- **Every new assertion was mutation-tested**, on the principle that a test which cannot fail is itself
  a defect: reverting the NaN guard fails 2; adding a 41st field fails 2; reverting `shadow_name`'s null
  tolerance crashes the suite outright; injecting an archetype named `"Hero"` under tradition
  `"Jungian Archetypes"` now trips the exclusivity assertion that previously let it through.

### Benchmarks

- **The null-safety hardening costs ~3.5-4% on tradition and civilization scans, and that is a real
  cost, not noise.** `registry/by_tradition` 21882 -> 22644 ns and `history/query_civilization`
  30326 -> 31475 ns (medians of 5 runs; the 2.14.2 sample sits below the whole observed range in both
  cases, which is the test the 2.14.2 entry established for telling a regression from a single-sample
  artifact). Both scan every profile calling `safe_streq` where they previously called `streq`, so one
  extra null check per comparison in the hot loop is exactly the expected shape.
  `registry/query_courage_0.9` moved +4.2% by the raw numbers but its 2.14.2 sample falls *inside* the
  observed spread, so it is noise — and it is a pure trait filter with no string comparison at all,
  which corroborates the attribution: only the string-comparing benchmarks moved.
  Judged worth it: seven reachable segfaults removed for ~4% on two query paths. Suite total across all
  60 benchmarks is flat at 247040 -> 247899 ns (+0.3%).

### Known and deferred

Held for **2.14.4** (content accuracy) and **2.14.5** (protocol and naming, potentially breaking):
Zadkiel attributed to Saturn rather than Jupiter; Guru Har Krishan's age at death; Epona's Roman temple;
Fūjin's Kojiki attribution; Babalú-Ayé carrying San Lázaro iconography; Ezili Dantò's invented
speechlessness; Marzanna and Morana carried as two archetypes of one goddess; Lada; the Slavic module's
Ivanov–Toporov framing; `"Polynesian"` as a pan-ethnic tradition string over Hawaiian and Māori atua;
Máttaráhkká, a Sámi goddess, under tradition `"Finnish"`; Joukahainen's persona calling itself "the
young Lapp"; the Sikh module's lack of a protocol note against express Akal Takht guidance on portraying
the Gurus; ADR-004 point 3's Vaishnava-marking claim that `src/hindu.cyr` does not carry; and three
`src/aboriginal.cyr` sourcing notes that misstate what the Taungurung and Gunaikurnai land councils
actually published — verified against Wayback snapshots predating v2.14.0, so not a case of the source
moving.


## [2.14.2] — 2026-08-31

A toolchain bump, and a benchmark practice that turned out to be measuring itself. Nothing in `src/`
changed; the suite is 295/295, identical to the pre-bump baseline.

### Changed

- **Cyrius pin `6.5.27` -> `6.5.36`.** `cyrius lib sync --full` re-vendored the whole version-matched
  stdlib snapshot (108 files), clearing a `./lib/ shadows version-pinned` warning that was live even
  against the old pin — the local tree was carrying patra 1.13.0 against a pinned 1.13.8. `cyrius deps`
  resolves clean, and `lib/` now matches the 6.5.36 snapshot byte-for-byte. The two failure modes the
  pin-bump checklist exists for — a stale leftover that `lib sync` never deletes, and a `[deps]` name the
  new snapshot dropped — are both absent: all 16 declared stdlib names resolve, and the file set matches
  exactly. Note that `lib/` is gitignored, so the re-sync contributes **zero committed bytes**; the delta
  lands at build time for whoever compiles against the new pin, and consumers do not inherit it by taking
  avatara 2.14.2 — they inherit it when their own pin moves.
- **Of the 22 stdlib files that changed between the two pins, 8 are in avatara's closure and exactly one
  changed symbol is one avatara calls.** The closure is 29 of the snapshot's 102 files, computed by
  following includes from the 16 declared roots rather than assumed; `bayan`, `mabda`, `sigil`, `sandhi`,
  `sankoch`, `vani`, `yukti`, `patra`, `niyama`, `ganita`, `yantra`, `freelist` and the `tls_native` pair
  all changed and are all outside it. Inside it: `fmt`, `io`, `sakshi` and five `syscalls_*` files. The
  `io` `getenv` rewrite is unreachable (no call site anywhere in the closure), the `sakshi` 2.4.10 ->
  2.4.11 change touches only the span path while `src/logging.cyr` calls `sakshi_set_level` and nothing
  else, and every `syscalls_*` change is additive or off-platform — `syscalls_x86_64_linux.cyr`, the file
  avatara actually builds through, is unchanged.
- **The one that matters: `fmt_float` gained a carry fix at 6.5.30.** `fmt_float_buf` now computes the
  rounded fraction *before* emitting the integer part, so a fraction that rounds up to a full unit carries
  into the integer instead of being emitted verbatim — `0.99` at one decimal printed `0.10`, and `3 - 1e-7`
  at seven printed `2.1000000`. avatara has 13 `fmt_float` call sites, all in `programs/`. Their current
  output is unchanged, because no value either example prints falls in the affected band — but the
  defect is latent rather than absent, and near-integers are exactly where a library of 0.0–1.0 trait
  weights lands them under composition.
- **Compiler references corrected across the docs — this is a real floor raise, not bookkeeping.**
  `README.md` (twice) and `CLAUDE.md` still said `6.4.71`; the 2.14.1 pin bump moved the manifest and left
  the prose behind, so they were two bumps stale. All three now say `6.5.36`. The floor genuinely moved:
  the current source still *compiles* and passes 295/295 under 6.4.71, so the old number was not wrong
  about buildability — but 6.4.71 predates the `fmt_float` carry fix, so a contributor honouring the
  documented floor gets a compiler that mis-prints near-integer floats from both example programs.
  Root cause of the drift: `scripts/version-bump.sh` rewrote CLAUDE.md's `- **Version**:` line and never
  the `- **Compiler**:` line, and CI gated VERSION against `cyrius.cyml` but never the pin against prose.
  **Both are now closed**: `version-bump.sh` reads `[package].cyrius` and propagates it into CLAUDE.md's
  Compiler line and both README references, and the `docs` job gained a **Verify toolchain pin
  consistency** step that fails when any of the three disagrees with the pin.

### Benchmarks

- **2.14.1 recorded no rows at all** — the release bumped the pin `6.4.71` -> `6.5.27` and skipped the
  mandatory `scripts/bench-history.sh` step, so the only available baseline for this release is 2.14.0,
  measured on 2026-07-22 under the older toolchain. Not back-fillable. Same class as the 2.8.0 skip
  already noted in `docs/doc-health.md`.
- **The apparent 2.14.0 -> 2.14.2 deltas are single-sample noise, and the investigation that established
  that is worth more than the numbers.** Read naively the table shows `history/traditions_for_civ` +30%
  and `registry/lookup_by_name` +16% against 40–60% improvements almost everywhere else. Neither
  regression is real:
  - `traditions_for_civ` and `traditions_for_era` are **structurally identical** functions
    (`src/history.cyr:328` and `src/history.cyr:362` — same nested loop, same `streq` scan, differing only
    in which field they read). Their *sum* is flat across the bump: 8024 ns -> 8049 ns. One went up by
    almost exactly what the other went down. Which of the pair is the slow one has flipped repeatedly
    across releases; it is a code-layout artifact between two near-twin functions, not a regression.
  - Holding the stdlib fixed at the 6.5.36 snapshot and varying only the compiler, cycc **6.5.28, 6.5.34,
    6.5.35 and 6.5.36 all emit a byte-identical benchmark binary** (same md5, 1198584 bytes). There is no
    codegen change anywhere in that span to attribute anything to. Against 6.4.71 — the one build that
    does differ — nine interleaved runs put every benchmark within ~1%: `lookup_by_name` 2593 vs 2604 ns,
    `traditions_for_civ` 3914 vs 3889, `similar_to_5` 71108 vs 70822.
  - Whole-suite total across all 60 benchmarks: **247528 ns -> 247040 ns**. Flat.
- **The practice needs a caveat recorded: `scripts/bench-history.sh` stores one sample per benchmark per
  release.** For the sub-microsecond rows that is well inside run-to-run spread, so release-over-release
  comparison of those rows detects noise and will keep producing phantom regressions. The µs-scale rows
  (`affinity/*`, `history/context_all_traditions`, `registry/by_tradition`) are the ones worth reading.
  The gate itself — build, run to completion, exit 0 — is unaffected and still valuable.


## [2.14.1] — 2026-08-17

### Changed

- **Cyrius pin `6.4.71` -> `6.5.27`** (2026-08-17, ecosystem-wide ML/AI-arc realign ahead of
  the arc reopening). `cyrius lib sync --full` re-vendored the whole version-matched stdlib
  snapshot, clearing the toolchain-drift and `./lib/ shadows version-pinned` warnings.
  Suite **295/295**, identical to the pre-bump baseline.

## [2.14.0] — 2026-07-22

The Aboriginal Australian expansion this version was planned around **did not happen**, and that is the
release. Six figures were researched against a prior desk pass that had called five of them shippable;
every one was refused. What shipped instead is the structural change that was gating them, four factual
corrections the research turned up in already-shipped content, and a data-integrity fix in `history.cyr`
found while making the first change.

### Changed
- **Aboriginal figures now carry their own peoples' tradition strings** — `Kunwinjku`, `Kulin` and
  `Gunaikurnai` replace the single continent-wide `"Aboriginal Australian"`. This fixes a real artifact
  rather than relabelling one: `cross_tradition_match()` returns one best match per tradition string, so
  the library previously answered "what is the Aboriginal Australian equivalent of Thor?" with a single
  figure — a sentence no Aboriginal person would write. It now returns a Kunwinjku match and a Gunaikurnai
  match separately, which is true. **Nothing is lost**: each people's `history.cyr` mapping lists
  `"Aboriginal Australia"` among its civilizations, so `traditions_for_civilization("Aboriginal Australia")`
  still gathers all three. The continent is a place these peoples share, not a tradition any belongs to,
  and the two fields now draw exactly that distinction. Each people is also dated to its own country —
  Kunwinjku to the ~50,000-year regional floor, Kulin to Murrup Tamboore (~31,000 BP), Gunaikurnai to
  Cloggs Cave (~25,000 cal BP) — where one continent-wide date had stood for all.
  **Consumers reading `prof_tradition()` or `all_traditions()` for these figures must update.**
- **`history.cyr` and the registry now agree in both directions, enforced by test.** Five mappings named
  traditions no profile carried — `"Incarnate Hindu"`, `"Incarnate Buddhist"`, `"Incarnate Mystic"`,
  `"Incarnate Taoist"`, `"Incarnate Sage"` — the same orphan class as the `"Incarnate Indigenous"` mapping
  fixed in 2.12.0, which was never swept for. `"Incarnate Sage"` described Pythagoras, Socrates, Confucius
  and Adi Shankara, none of whom this library carries. In the other direction five tradition strings carried
  profiles that resolved to no mapping at all, so `context_for_tradition()` returned `0` for 19 archetypes.
  `"Incarnate Mystic"` was rekeyed to `"Mystic"` (17 profiles), the three redundant duplicates and the
  aspirational `"Incarnate Sage"` were dropped, and `Vedic`, `Canaanite`, `Etruscan` and `Solar` gained
  mappings. The new test walks both directions rather than pinning a count, so a new tradition or mapping is
  covered the moment it is added.

### Fixed
- **`Goorialla` was attributed to the Lardil and is not theirs.** The Lardil rainbow serpent is **Thuwathu**,
  published by the Lardil's own art centre and their shire; Goorialla is the serpent of Goobalathaldin Dick
  Roughsey's *The Rainbow Serpent* (1975), which the Australian Dictionary of Biography records as fusing
  Gulf material into Cape York repertoires. He belongs to a book, not to a people.
- **The Waugal citation rested on a false positive the file itself defines.** The module credited Noongar
  interpretation "at Kings Park" — Botanic Gardens and Parks Authority material, a WA government authority,
  i.e. this file's own false positive #1 cited as support. Replaced with the South West Aboriginal Land and
  Sea Council's own channel, whose lead form is **Waugal**.
- **The GLaWAC Elder-approval statement was over-read**, and it was the strongest consent claim in the file.
  It was described as approving the stories "for use". It reads that *the spelling* has been approved, to
  ensure consistent public material — approval of **orthography**, silent on content and on third-party
  reuse. Corrected wherever it was relied on.
- **The stated reason for excluding Ngatyi was false.** The file said the material was park interpretation.
  It is not — Badger Bates is a Barkandji Elder and a Native Title Group director publishing in the first
  person. Ngatyi stays excluded on two real grounds: the custodian's express restriction on conduct and
  speech at Ngatyi places, and a name its own custodians spell two ways, which a name-keyed library cannot
  hold. A wrong exclusion reason invites the next survey to re-litigate from a wrong premise.
- The benchmark label `aboriginal/all_5` counted five figures where the module has four, since 2.11.0.
- `history.cyr`'s header claimed 34 mappings; it had 35 before this release and has 37 after.

### Not added, and why
Recorded in the `aboriginal.cyr` header rather than dropped, because "we looked and did not add" is a result:
- **Goorialla** — belongs to a book rather than a people; this library attributes to peoples, not authors.
- **Borun** and **Tuk** — the best channel cited anywhere in the module (the Gunaikurnai RAP in its own
  voice) with ~150 and ~95 published words behind it, one paragraph recirculated across eight sites. Tuk
  performs no action, speaks no word and makes no decision in any source retrieved. This is the module's own
  false-positive #2 arriving exactly as predicted.
- **Waugal** — refused for the *opposite* of the usual reason, and must not be recorded as under-sourced:
  SWALSC publishes ~1,000 words in named Noongar Elders' own voices, and publishes terms requiring written
  permission for "modification, distribution or publication". A GPL-3.0 library that ships a profile and
  inverts it through `shadow()` is both, named in terms. Excluded pending written consent nobody has sought.
- **Namarrkon** — the case rested on two independent Aboriginal-owned channels. Measured against the 2009
  settler-curatorial Samstag text: Injalak Arts shares **zero** five-word sequences with it, but Marrawuddi
  shares **232** — 89% of its content, longest common run 39 words, inheriting the curator's typo and his
  "in this painting" deixis applied to a different painting. Marrawuddi is the settler text copy-edited; an
  Aboriginal-owned publisher does not make the words it reprints a community voice. One channel and 102 words
  survived — less than Borun, already refused.
- **Ngatyi** — see Fixed, above.

### Notes
- No new archetypes: **504 across 37 traditions** (35 → 37 from the split).
- 22 new tests (273 → 295), including both-direction negative controls for the two new invariants.
- Community review remains **blocked** and is now the binding constraint rather than a formality: three
  bodies publish contact details, and nobody has written to any of them. No code change substitutes for it.

## [2.13.1] — 2026-07-22

Two design rulings recorded and enforced, plus the API change one of them implied. No archetypes and no
behaviour change — this release exists so that two decisions stop living only in conversation, and so the
overlay layer is shaped like what it claims to be.

### Changed
- **Traditions and typologies are mutually exclusive, and a test now enforces it.** A tradition is a
  people's own account of who its figures are: it earns archetypes, a `tradition` string, a history mapping
  and a place in `profile_count()`. A typology — the Enneagram, Jung's set, anything of that kind — is a
  modern analytic grid laid over other people's figures from outside, and gets none of those. It may only
  ever exist as an overlay in `src/overlay.cyr`. The Enneagram will not become a tradition with nine
  archetypes; that would put a 20th-century framework on the same shelf as the Sephiroth and the Orishas.
  The new assertion checks that no tradition string and no archetype name equals any of the 14 overlay
  labels, and that `by_tradition("Enneagram")` and `by_tradition("Jungian")` are both empty. Recorded in
  CLAUDE.md § Key Principles and in the `overlay.cyr` header.
- **`shadow()` and `compose()` stay uniform across every tradition — no per-figure opt-out.** The v2.14.0
  research surfaced that `shadow()` emits a "Shadow of <name>" with inverted traits and flipped polarity,
  and that this would apply to figures like Borun and Tuk, the apical ancestors of every living Gunaikurnai
  person. No cited channel contemplated that. The decision is to apply the machinery evenly and say so
  rather than exempt some archetypes silently: a library with quiet exceptions is harder to reason about
  than one that is uniform and documents the consequence. `src/aboriginal.cyr`'s header now states that a
  consumer generating shadow or composed forms is doing something no cited source anticipated and should
  decide for itself whether that suits its use.
- **New: a generic overlay API, because overlays are readings and readings are plural.** Overlay families
  are now registered in an `OverlaySystem` enum, and `overlay_system_count()`, `overlay_system_name(s)`,
  `overlay_system_by_name(name)`, `overlay_label_count(s)`, `overlay_label(s, i)`, `overlay_score(s, p, i)`
  and `overlay_best(s, p)` walk that registry without naming a family. A consumer can enumerate every
  reading available for a profile without knowing which systems exist. Out-of-range systems degrade to
  `"unknown"`/`0`/`0.0` rather than trapping — an unknown system is a caller bug, not a reason to abort a
  scoring loop. Purely additive; every per-family function is unchanged.
- **The exclusivity test is now registry-driven rather than a hardcoded label list.** It walks
  `OverlaySystem`, checking each system's own name *and* each of its labels against every tradition string,
  every archetype name, and `by_tradition()`. A system registered later is covered the moment it is added,
  with no test edit — which matters precisely because new overlay systems are expected. Verified by
  mutation: renaming the Jungian system to "Norse", and the `trickster` label to "Iktomi", each fail the
  suite; reverting restores it.
- **Recorded: overlays do not hardlock meaning.** Interpretation moves — scholarship revises, schools
  disagree, and systems that do not exist yet will want to be laid over figures already carried here.
  Because overlays derive from the profile and store nothing, a new system costs no archetype edits, and
  several may sit over one figure disagreeing with each other. Thor reads as some Enneagram type *and* some
  Jungian role *and* whatever comes next; none of those displaces the others and none of them is what Thor
  is. What an archetype *is* lives in the tradition modules, authored from that tradition's own sources;
  everything in `overlay.cyr` is commentary, and commentary is allowed to be plural and to be wrong.
- 32 new tests (241 → 273).

### Notes
- This **clears the gate on v2.14.0**. The one remaining prerequisite before the Aboriginal expansion is the
  per-people tradition split, which is a code change with its own CHANGELOG entry — not a drive-by edit.
- Community review for v2.14.0 remains **blocked**; that is unchanged and unrelated.

## [2.13.0] — 2026-07-22

**Archetype overlays** — the first layer in this library that sits *on top of* the archetypes rather than
beside them. A tradition module answers "who is this?"; an overlay answers "what shape is this, in a system
that was not built for it?" — what Enneagram type Thor reads as, which Jungian role Iktomi is playing.
Additive API only: **no archetypes added, `profile_count()` stays 504**, the 320-byte layout is untouched,
and the registry does not know overlays exist.

### Added
- **`src/overlay.cyr`** — two overlay families, both index-stable so the API can grow without breaking.
  Everything is derived from the profile's own trait and emphasis weights, following the `aspect.cyr`
  precedent: no per-archetype authoring, no state.
- **Enneagram** — nine types, three centres, wings:
  - `enneagram_count()` (9), `enneagram_number(i)` (1-9), `enneagram_name/title(i)`,
    `enneagram_core_desire(i)` / `enneagram_core_fear(i)`, `enneagram_index_by_name(name)`.
  - `enneagram_centre(i)` with `centre_name(c)` / `centre_emotion(c)` — gut/anger (8,9,1), heart/shame
    (2,3,4), head/fear (5,6,7); the three centres partition the nine types exactly 3/3/3, test-pinned.
  - `enneagram_wing_low(i)` / `enneagram_wing_high(i)` — the types are a **ring**, so 1's low wing is 9 and
    9's high wing is 1. The ring's symmetry (my high wing's low wing is me) is test-pinned.
  - `profile_enneagram_score(p, i)`, `profile_enneagram_type(p)`, `profile_enneagram_wing(p)`,
    `profile_enneagram_centre(p)`.
- **Jungian** — `jungian_count()` (5), `jungian_name/title/desc(i)`, `jungian_index_by_name(name)`,
  `profile_jungian_score(p, i)`, `profile_jungian_role(p)`, for Hero, Shadow, Anima/Animus, Self, Trickster.
- **The composition point with `shadow.cyr`.** The SHADOW overlay and `shadow()` are two different things and
  the module says so: `profile_jungian_score(p, JUNG_SHADOW)` scores how much p *already expresses* shadow
  qualities — the cold, withheld, adversarial register — while `jungian_shadow_form(p)` returns
  `shadow(p)`, a *different archetype* named "Shadow of <p>". Scoring high on the overlay is a property of p;
  the shadow form is a construction. Pinned by `is_shadow_of(jungian_shadow_form(p), p)`.
- 31 new tests (210 → 241) and 3 benchmarks (57 → 60). The load-bearing assertion runs every overlay score
  for **every one of the 504 archetypes** and checks it stays in 0.0-1.0, plus that each archetype's reported
  wing is genuinely adjacent to its dominant type and its reported centre is that type's centre.

### Notes
- **What an overlay score is not.** The module header states it: this is a projection of one system onto
  another, not a diagnosis, and not a claim that a tradition recognises the category. The Enneagram is a
  20th-century typology and Jung's archetypes are 20th-century psychology; neither was in the room when the
  Dagda or Sedna were described. The trait mapping each type reads is documented in the source rather than
  hidden — it is a defensible reading, not the only possible one.
- `tests/avatara.bcyr` and both example programs gained `shadow.cyr`, which they had not previously included,
  since `overlay.cyr` depends on it.
- Overlays are deliberately **not** archetypes: no registry block, no history mapping, no tradition. A test
  pins `profile_count() == 504` in the overlay section specifically to keep that true.

### Benchmarks
- 60/60 recorded for 2.13.0. The overlay path is pure derivation with no allocation:
  `overlay/enneagram_score` ~16 ns for a single type, `overlay/jungian_role` ~73 ns across 5,
  `overlay/enneagram_type` ~120 ns across all 9. No regressions elsewhere.

## [2.12.1] — 2026-07-22

Sourcing corrections to `src/aboriginal.cyr`, found by the desk research done for the v2.14.0 arc. No
archetypes added or removed; three entries carried defects and one exclusion was resting on a false premise.

### Fixed
- **Waa no longer carries the Karatgurk fire sequence.** The seven Karatgurk women, the skin bag, the three
  birds, the blackened feathers and the Pleiades were load-bearing in the entry comment, the `desc` and the
  `spirit_text`. That sequence traces to Aldo Massola, *Bunjil's Cave* (1968) — a non-Indigenous Museum of
  Victoria curator. The **Taungurung Land and Waters Council**, the Registered Aboriginal Party, publishes on
  its own Creation Stories page that Waang obtained fire "through cunning means" and distributes it to
  humanity, and does not name the Karatgurk at all. Where the RAP's account differs from a settler curator's,
  the RAP's account is what this library carries. The entry now follows Taungurung and says so.
- **Tidilick** — renamed from "Tiddalik" and re-attributed to **Gunaikurnai alone**. The **Gunaikurnai Land
  and Waters Aboriginal Corporation** publishes it under an express statement that its stories are approved
  for use by the Elders and Knowledge Holders — the only explicit Elder approval behind any figure in the
  module — and GLaWAC spells it Tidilick and attributes it to Gunaikurnai. The old "Gunai/Kurnai and other
  southeastern peoples" hedge rested on settler collectors (Brough Smyth 1878, Curr, Howitt, Bulmer) and was
  precisely the pan-Aboriginal smear the file's own header disclaims. `desc` notes the common spelling so the
  figure stays findable; the enum member is now `ABOR_TIDILICK`.
- **Bunjil** — "Wathaurong" corrected to **Wadawurrung**, the corporation's own spelling.

### Changed
- **Baiame moved from deferred to a stated exclusion.** The old header held him pending better sourcing.
  That premise was false: community-involved material does exist (Baiame's Ngunnhu is National Heritage
  listed with the creation story in its listed values; the Brewarrina Aboriginal Cultural Museum runs guided
  tours; the Baiame's Ngunnhu Festival runs with Moogahlin Performing Arts; the Wonnarua Nation Aboriginal
  Corporation hosts the Baiame Cave management plan). He is still excluded, on grounds sourcing cannot
  dissolve — the Bora initiation tie, a reported restriction on women seeing the image, an unresolved
  reported prohibition on speaking the name, and a pan-NSW "All-Father" entry breaking the file's own
  name-the-specific-people rule. A Wonnarua elder has publicly asked for Baiame Cave to be closed to the
  public. *The sourcing improved and the answer is still no* — which is a more useful thing for the header to
  record than a deferral.
- **The header now states its exclusion reasoning**, including two worked examples that defeat tempting
  shortcuts (**Uluru**: maximum public circulation plus an express refusal — Parks Australia states it cannot
  license ICIP, so volume of publication proves nothing about consent; **Wandjina**: excellent community
  sourcing plus enforced depiction rights — good sourcing is not a licence either), and two false-positive
  patterns it will not accept as a basis (joint-management/land-agreement material read as cultural
  publication; thin community material padded out to fill the 15-trait struct).
- **Almudj (Kundjeyhmi), Bolung (Jawoyn) and Ngatyi (Barkandji) are named but not instantiated** — so the
  flattening English performs with the single label "Rainbow Serpent" is visible, without giving profiles to
  figures whose public carriage is park interpretation rather than publication by those peoples. Citing is
  not instantiating.

### Notes
- **An open question is now recorded in the header rather than left implicit**: `shadow()` emits a
  "Shadow of <name>" with inverted traits and flipped polarity, and `compose()` blends across traditions. No
  channel cited for any figure contemplated that, and it applies to the four figures already carried. It is a
  code question, not a research one, and it is the gating decision for the v2.14.0 arc — see the roadmap.
- The v2.14.0 arc on the roadmap now carries the full research result: six profiles shippable on the research
  alone (Goorialla, Borun and Tuk, Wagyl, Namarrkon, and Ngatyi behind a spelling confirmation), seven at
  borderline, everything else excluded — and the per-people tradition split identified as the gate that must
  land *before* any expansion.
- Community review remains **blocked**: desk research can establish that material is already public, which is
  checkable; it cannot establish that a people consents to this particular use.

## [2.12.0] — 2026-07-22

**World traditions, part 2** — the three items the 2.11.0 cultural-accuracy review deferred rather than
fixed in passing: **497 → 504 archetypes, 34 → 35 traditions**.

### Changed
- **The pan-ethnic `"Indigenous"` tradition label is retired.** `incarnate.cyr` carried four figures from
  four different nations under one bucket, which contradicted the named-nation approach 2.11.0 adopted and
  forced both `lakota.cyr` and `haudenosaunee.cyr` into awkward deferrals. They now carry their own nations:
  - White Buffalo Calf Woman → **Lakota** (that tradition 10 → 11)
  - Deganawidah → **Haudenosaunee** (6 → 7), and renamed to **"The Peacemaker"**: in many communities the
    personal name is reserved for ceremonial use, and the title is what the review recommended.
  - Quanah Parker → **Comanche** (new tradition)
  - Wovoka → **Northern Paiute** (new tradition)
- **This also fixed a live bug.** The `"Incarnate Indigenous"` history mapping never matched any profile —
  the profiles said `"Indigenous"` — so `context_for_tradition("Indigenous")` silently returned 0. Worse,
  that orphaned mapping filed four North American figures under the **Tonga Empire and Hawaiian Kingdom**.
  It is replaced by proper Comanche (Great Plains, 1700→living) and Northern Paiute (Great Basin) mappings;
  `mapping_count()` 34 → 35.
- **Separators normalised to the em dash** across every consumer-facing `desc`/`soul`/`spirit_text` string.
  The codebase was split 1129 em dash to 133 `" -- "`; the outliers were `celtic.cyr`, `maya.cyr` and the
  four world-tradition modules added in 2.11.0. Now 1274 to 0.

### Added
- **The seven Grandfather Teachings (Nizhwaaswi Gagiikwewin)** join `src/anishinaabe.cyr` (5 → 12),
  restoring the selection balance lost when the wiindigoo was removed in 2.11.0 — the module carried the
  trickster and no counterweight from the material Anishinaabe communities most actively publish.
  Nibwaakaawin (wisdom/beaver), Zaagiidiwin (love/eagle), Minaadendamowin (respect/buffalo),
  Aakodeewin (bravery/bear), Gwayakwaadiziwin (honesty/sabe), Dabaadendiziwin (humility/wolf) and
  Debwewin (truth/turtle). They are carried by their Ojibwe names, which is how they are taught, and are
  modelled as **principles rather than beings**: `TIER_COSMIC` and `POL_ANDROGYNOUS` throughout, pinned by
  test. These are among the most actively community-published Anishinaabe material — taught in schools,
  tribal colleges, health services and child-welfare practice — which is why they clear the bar the
  wiindigoo did not.
- 11 new tests (199 → 210), including negative assertions that the retired label is gone and that the
  personal name "Deganawidah" is no longer a lookup key.

### Notes
- Non-breaking for archetype data and the 320-byte layout, but **tradition strings changed** for four
  profiles; a consumer querying `by_tradition("Indigenous")` now gets an empty result by design.
- The `incarnate_indigenous_*` constructor names and `all_incarnate_indigenous()` are now stale labels for
  figures that no longer carry that tradition. Renaming them is a breaking API change and is queued for
  v3.0.0; the module grouping itself remains a reasonable organisational unit.
- Archetype overlays moved from v2.12.0 to **v2.13.0** to make room for this.

### Benchmarks
- 57/57 recorded for 2.12.0. No regressions; the Anishinaabe collection bench simply reflects its larger
  set (`anishinaabe/all_12`).

## [2.11.0] — 2026-07-22

**World traditions** — organised as **named nations** rather than pan-continental buckets:
**462 → 497 archetypes, 29 → 34 traditions**. Adding Indigenous North American and Aboriginal Australian
material differs in kind from adding Norse or Greek: both carry *active* community protocols around
restricted knowledge (ICIP, the AIATSIS Code of Ethics, nation-specific telling and naming rules), and a
single "Native American" or "Aboriginal Australian" label flattens 500+ and 250+ distinct nations
respectively. The scope below reflects that.

### Added
- **`src/inuit.cyr`** (10) — Sedna, Nanuq, Sila, Torngarsoak, Pinga, Anningaaq, Malina, Amarok,
  Tekkeitsertok, Qailertetang. Animal-masters and conditions of the world rather than sovereigns.
- **`src/lakota.cyr`** (10) — Wakan Tanka, Inyan, Maka, Skan, Wi, Hanwi, Tate, Wakinyan, Tatanka, Iktomi.
- **`src/haudenosaunee.cyr`** (6) — Sky Woman, Teharonhiawako, Tawiskaron, the Three Sisters, the Great
  Turtle, the Thunderers.
- **`src/anishinaabe.cyr`** (5) — Gichi-Manidoo, Nanabozho, Nokomis, Mishipeshu, Animikii.
- **`src/aboriginal.cyr`** (4) — the Rainbow Serpent, Bunjil, Waa, Tiddalik; each attributed to the people
  it belongs to, limited to figures already established in public, community-involved material.
- **`src/celtic.cyr`** gains **Miach** and **Airmed** (15 → 17), completing the Dian Cecht arc from the
  *Cath Maige Tuired* — the last genuinely outstanding item from the roadmap's "deferred additions" (Slavic
  Mokosh/Rod, Polynesian Pele/Kanaloa and Celtic Ogma were already present).
- **History**: five new mappings (29 → 34), including an Aboriginal Australian entry reaching back 65,000
  years and still living.
- 33 new tests (166 → 199 assertions); 5 new benchmarks (52 → 57).

### Changed
- **Toolchain pin bumped 6.4.70 → 6.4.71**; `lib/` wiped and re-resolved via `cyrius deps` (exit 0), all 16
  declared stdlib entries present, snapshot delta empty for the declared subset.

### Deliberately excluded
Two figures were authored, reviewed, and then removed rather than shipped:
- **The wiindigoo** is not carried. Every profile in this library is a *speakable persona* that consumers
  instantiate and voice; in many Anishinaabe communities the wiindigoo is named with care, rarely outside
  winter and not casually in the first person. That is a restriction this format cannot keep, so the figure
  is left out rather than handled badly.
- **Baiame** is held for the 2.11.x arc. Its standard sources are 19th-century settler ethnography rather
  than community-involved publication, "gave the Law and the ceremonies" points at initiation-restricted
  Bora material, and the All-Father framing is substantially contact-influenced.

### Verification
A cultural-accuracy and protocol review (one reviewer per module, plus dedicated sensitivity and technical
passes) returned **46 confirmed findings — 3 critical, 20 major, 23 minor**. All were addressed. The three
criticals are worth recording, because each was a case of a module's own disclaimer not matching its
contents:
- **Wakinyan** carried heyoka mechanics in `desc` and `spirit_text`. Heyoka is a ceremonial obligation
  conferred by a community following a thunder dream — not a temperament and never self-identified into.
  Because consumers read those fields as a personality specification, this built precisely the
  self-identification vector the 1993 Lakota Summit V Declaration objects to. The mechanics were removed;
  the header now states plainly what heyoka is and that this library does not represent it.
- **The Rainbow Serpent** listed Yurlunggur and Julunggul as Yolngu synonyms. They are the Wawilak Sisters'
  ancestral beings, tied to the Djungguwan and Kunapipi complexes — restricted, initiate- and
  gender-limited. That one line falsified the module's own "nothing restricted is included here" claim.
  Removed; the remaining names (Ngalyod, Wagyl, Goorialla) now state *why* each is public, and the header's
  absolute claim was narrowed to what is actually defensible.
- **The Wiindigoo** entry warned about careless naming inside a voice built to be spoken as. Removed (above).
Also corrected: **"Hah-nu-nah"** (a Little Water Society ceremonial name — Converse's own footnote says so;
the everyday word is Ha'no'wa); **Waa's** soul stating living Kulin marriage Law as divine speech;
**Tekkeitsertok's** attribution; **Malina's** and **Hanwi's** narratives, both of which had been inverted;
and **Qailertetang**, which is a masked *pair* at the fall Sedna feast rather than one androgynous figure.

### Notes
- Additive, non-breaking: existing archetypes, traditions, the 320-byte layout and all prior APIs unchanged.
- Three review items were deliberately carried to the roadmap rather than fixed in passing: re-attributing
  the incarnate `"Indigenous"` bucket to named nations, the Anishinaabe selection balance after the
  wiindigoo removal, and the ` -- ` / em-dash separator split.

### Benchmarks
- 57/57 recorded for 2.11.0. The five new `all_*` collection benches land at 5-11 ns, in line with the rest.
  No regressions: `similar_to_5` holds its 2.10.1 improvement at the larger N, which is the top-k fix doing
  what it was meant to do.

## [2.10.1] — 2026-07-22

Performance fix — no API, data, or behavior change. Resolves the O(N²) scaling issue flagged in the 2.10.0
benchmarks.

### Changed
- **`similar_to()` now uses bounded top-k selection instead of sort-then-trim** (`src/affinity.cyr`).
  It previously scored every candidate, ran a full insertion sort over **all** N results, and only then
  trimmed to `max_results` — so a query for the top 5 paid an O(N²) sort over the whole registry. The
  comment said "N is small"; N had reached 460. The result list is now held at `max_results` entries, and a
  candidate that cannot displace the weakest kept entry is discarded without allocating a sim entry or
  shifting anything, making the bounded path O(N·k).
  - **`affinity/similar_to_5`: 809 µs → 73 µs (−91%, an 11× speedup)** — and now faster than at any prior
    release (2.8.1 was 527 µs at N=396). What remains is the N affinity computations themselves.
  - `affinity/cross_tradition_match`, which shares the same path, improved in step (~60 µs).
  - The cost no longer grows with N², so it will not re-degrade as traditions are added.
- Behavior is deliberately identical: same returned set, same descending order, and the same tie handling
  (the `f64_gt` comparison is preserved, so a later equal score still sorts ahead). The unbounded contract
  (`max_results <= 0` returns every candidate, sorted) is unchanged and still pays a full insertion sort,
  since every candidate has to be ordered anyway.

### Verification
- An out-of-tree harness checked the defining top-k property over **48 source/k combinations** (8 sources
  spanning Norse, Kabbalah, Greek, Tarot, I Ching, Hindu and Etruscan × k = 1, 2, 3, 5, 10, 50): every
  returned entry outranks every excluded candidate, results are sorted descending, and the bounded result
  is a score-identical prefix of the fully sorted list.
- 5 regression guards added to the suite (161 → 166 assertions) pinning that equivalence: the unbounded
  call returns all `profile_count() - 1` candidates fully sorted, the top-5 is a score-identical prefix of
  it, the weakest kept entry outranks the strongest excluded one, and a NULL source returns empty.

### Benchmarks
- 52/52 recorded for 2.10.1 (see `bench-history.csv`). The only material movement is the intended
  `similar_to`/`cross_tradition_match` improvement; everything else is within run-to-run noise.

## [2.10.0] — 2026-07-22

**I Ching** — the 64 hexagrams as archetypes, over the eight trigrams: **396 → 460 archetypes,
28 → 29 traditions**. The Zhou Yi reads the world as sixty-four *situations* — not deities but archetypal
configurations of change — each a six-line figure built from two of the eight trigrams (bagua). Structurally
this parallels 2.9.0's Tarot: where the trumps bridge the Sephiroth, the hexagrams bridge the trigrams.

### Added
- **`src/iching.cyr` — the 64 hexagrams** (new "I Ching" tradition), King Wen sequence, The Creative (1)
  through Before Completion (64), each a full `ArchetypeProfile` with traits grounded in the Wilhelm/Baynes
  reading. The profile `name` is the English name — toneless pinyin is ambiguous (Qian, Kun, Bi, Lu, Yi and
  Jian each name *two* hexagrams), so it cannot be the unique key; number, pinyin and trigram image live in
  `desc`, and the constructors are numbered (`iching_hex01` .. `iching_hex64`).
- **Trigram bridge API** — a trigram's enum value *is* its three-line code (bottom line as MSB):
  `TRIGRAM_KUN`=0 (000) … `TRIGRAM_QIAN`=7 (111), so decoding is arithmetic rather than a table.
  - `iching_code(i)` packs the composition as `lower*8 + upper` (one auditable line per hexagram);
    `iching_lower/upper(i)`, `iching_line(i, pos)` (pos 1-6 from the bottom), `iching_number(i)` (1-64),
    `iching_yang_count(i)`, and `iching_for_trigrams(lower, upper)` (total inverse).
  - `trigram_name/image/attribute/family/element(t)` and `trigram_line(t, pos)`, `trigram_count()` (8).
  - `all_iching()` / `iching_count()` (64), `iching_by_index(i)`, `iching_by_number(n)`,
    `iching_index_by_name(name)`.
- **Uniform derivation rules**, documented in the module header and pinned by tests across all 64: element
  is the *upper* trigram's element; polarity follows the yang-line balance (>=4 masculine, <=2 feminine,
  3 androgynous — yielding 22/22/20, the system's own symmetry); tier is `TIER_COSMIC` throughout, as for
  the Sephiroth and the Tarot paths.
- **History**: a 29th mapping (`I Ching`) — Western Zhou Zhou Yi through the Ten Wings and the Song
  Neo-Confucian commentaries, a living tradition; `mapping_count()` 28 → 29.
- Wired into all build roots, both example programs, the registry, and the `[lib]` dist bundle. 45 new
  tests (116 → 161 assertions); 2 new benchmarks (`iching/all_64`, `iching/for_trigrams`, 50 → 52).

### Changed
- **Toolchain pin bumped 6.4.69 → 6.4.70** (`cyrius.cyml` `[package].cyrius`), folded into this release.
  Vendored `lib/` was wiped and re-resolved from scratch via `cyrius deps` (the CI gate, exit 0); all 16
  declared `[deps] stdlib` entries are present and the 6.4.69 → 6.4.70 snapshot delta is empty for the
  declared subset — nothing added, nothing dropped.

### Verification
- The King Wen table (number, name, both trigrams, six lines) was produced independently three times and
  cross-checked: **zero disagreements**, and every row's line pattern decoded to its named trigrams. The
  transcription into `iching_code()` was then diffed deterministically against that table — all 64 match.
  This closes the blind spot the bijection test cannot see: a *transposition* between two hexagrams would
  still be a perfect bijection.
- Adversarial symbolic review of all 64 profiles (four reviewers plus a technical pass) raised five
  findings, of which one survived: hexagram 49 Ge (Revolution) was assigned `DOMAIN_CHAOS`, corrected to
  `DOMAIN_FATE`. *Geming* is the changing of the mandate (*ming*) — one order replacing another, not order
  dissolving; the Judgment carries the four cardinal virtues and the Image is order-establishing, and the
  label contradicted the profile's own spirit text. `DOMAIN_CHAOS` is now reserved for the genuinely
  dysfunctional situations (3, 12, 23, 28, 38).

### Notes
- Additive, non-breaking: existing archetypes, traditions, the 320-byte `ArchetypeProfile` layout, and all
  prior APIs are unchanged.

### Benchmarks
- 52/52 recorded for 2.10.0 (see `bench-history.csv`). New: `iching/all_64` (~31 ns, in line with the other
  cached `all_*` collections) and `iching/for_trigrams` (~1.2 µs — an O(64) scan over the composition table).
- **Two registry-wide scans are reproducibly slower**, and the cause is *not* the compiler bump:
  `affinity/similar_to_5` 585 → 811 µs (+38%) and `history/query_civilization` 40.8 → 51 µs (+26%).
  `similar_to()` sorts **all** N candidates with an insertion sort before trimming to the top 5, so its cost
  grows with N²; N went 396 → 460, and 460²/396² = 1.35 — which accounts for the +38% almost exactly. The
  `query_*` filters are linear scans over the larger registry.
- The compiler bump itself is clean: `kabbalah/single_profile` is N-independent and sits at 317–376 ns
  across re-runs, statistically unchanged from 2.9.0's 361 ns — no codegen regression under 6.4.70.
- **Known scaling issue (backlogged)**: the O(N²) sort in `similar_to()` will degrade with every tradition
  added. The fix is a top-k partial selection (O(N·k)); see the roadmap Backlog.

## [2.9.0] — 2026-07-22

**Tarot Major Arcana** — the 22 trumps as archetypes, bridged to the Tree of Life: **374 → 396 archetypes,
27 → 28 traditions**. The Major Arcana are not deities but archetypal stages of the soul (the Fool's
Journey, 0 → XXI). In the Hermetic Qabalah of the Golden Dawn each trump is a *path* on the Tree of Life —
one of the 22 lines joining the 10 Sephiroth (`kabbalah.cyr`), keyed to a Hebrew letter. This is avatara's
first module to bridge two existing systems.

### Added
- **`src/tarot.cyr` — the 22 Major Arcana** (new "Tarot" tradition): The Fool (0) through The World (XXI),
  each a full `ArchetypeProfile` with traits grounded in Rider–Waite–Smith / Golden Dawn symbolism. All are
  `TIER_COSMIC`, mirroring the Sephiroth they connect. Domains span the set (e.g. Death→`DEATH`,
  The Sun→`SUN`, Wheel of Fortune→`FATE`, Justice→`ORDER`, The Empress→`NATURE`); 0 unspecified.
- **Kabbalah bridge API** — every trump carries its Golden Dawn "32 Paths of Wisdom" attribution:
  - `tarot_hebrew_letter(i)` (Aleph … Tav), `tarot_path(i)` (path number 11–32), and
    `tarot_path_upper(i)` / `tarot_path_lower(i)` (the two `Sephira` enum values, from `kabbalah.cyr`, the
    path joins).
  - `tarot_for_path(path)` (inverse of `tarot_path`; a Sephira-range or out-of-range number → −1) and
    `tarot_connects(i, seph)` ("which trumps meet at this Sephira?").
  - `all_tarot()` / `tarot_count()` (22), `tarot_by_index(i)`, `tarot_index_by_name(name)`.
  - Numbering follows the RWS/Golden Dawn convention (VIII = Strength/Teth/Leo, XI = Justice/Lamed/Libra),
    with orthodox letter attributions (not Crowley's Heh/Tzaddi swap).
- **History**: a 28th mapping (`Tarot`) — 15th-c. Italian tarocchi through the Golden Dawn (1888)
  Tree-of-Life synthesis, a living tradition; `mapping_count()` 27 → 28.
- Wired into all build roots, the registry, and the `[lib]` dist bundle. 34 new tests (82 → 116
  assertions); 1 new benchmark (`tarot/all_22`, 49 → 50).

### Verification
- The 22 (letter → path → Sephiroth-pair) attributions were cross-checked against the Golden Dawn canon by
  independent adversarial review (three canon checkers, zero discrepancies). One initial mislabeling was
  corrected pre-release: Strength's domain `WAR` → `NATURE` — the card is gentle fortitude, not martial,
  and `WAR` is properly The Chariot's.

### Notes
- Additive, non-breaking: existing archetypes, traditions, the 320-byte `ArchetypeProfile` layout, and all
  prior APIs are unchanged.

### Benchmarks
- 50/50 recorded for 2.9.0 (see `bench-history.csv`); `tarot/all_22` added (~13 ns, in line with the other
  cached `all_*` collections). No regressions relative to the 2.8.1 baseline across the shared 49 benches.

## [2.8.1] — 2026-07-22

Toolchain maintenance release — no source changes.

### Changed
- **Toolchain pin bumped 6.2.11 → 6.4.69** (`cyrius.cyml` `[package].cyrius`). Vendored `lib/` was
  re-resolved cleanly from scratch against the 6.4.69 snapshot: the local dir was wiped and repopulated
  via `cyrius deps` (the CI gate), which resolves clean (exit 0) with all 16 `[deps] stdlib` entries
  present in the new snapshot — no declared dependency was dropped, so `[deps]` needed no edit. The
  resolve now vendors the declared-subset-plus-transitive set (29 `.cyr` files) rather than the whole
  snapshot; 68 unused modules left over from a prior `--full` vendor (`async*`, `tls_native_*`, `regex`,
  `http`, `net`, `regression*`, `thread*`, etc. — none referenced by avatara) were pruned, eliminating
  the stale-leftover class that can mask a clean-resolve failure locally.
- No source changes: `dist/avatara.cyr` was regenerated (`cyrius distlib`) and differs from 2.8.0 only in
  its `# Version:` header; the regen is idempotent (byte-stable). Full build, smoke test, and the
  integration suite (82 assertions) pass under 6.4.69.

### Benchmarks
- 49/49 recorded for 2.8.1 (see `bench-history.csv`). No regressions relative to the 2.7.2 baseline — the
  first re-benchmarked release since then. The persistent `kabbalah/single_profile` codegen creep flagged
  under 6.1.34/6.2.11 has **reversed** sharply: ~490 ns → ~286 ns (−42%). Other measurable paths also
  improved (`history/context_for_tradition` 273→192 ns, `history/traditions_active_at` 932→686 ns,
  `affinity/similar_to_5` 571→527 µs). The handful of upward blips (`compose/three_traditions`,
  `registry/lookup_by_name`, `history/traditions_for_civ`) sit in the µs-quantized band the harness rounds
  to whole microseconds (e.g. 2000→2409 ns is "2 µs → 2.4 µs" rounding), and the sub-25 ns `all_*`
  collection benches swing within integer-ns run-to-run noise — neither is a real regression.

## [2.8.0] — 2026-07-09

**Role aspects** — derive an archetype's roles from its personality vector. An archetype is a personality
(15 traits + 14 emphases + soul/spirit + a domain) with no single "role"; a role is a facet of that
personality. New `src/aspect.cyr` derives a small, universal set of aspects from the trait weights, each
mapping one salient trait to a role label, so every one of the 374 archetypes has selectable roles with no
per-archetype authoring. Consumers (thoth's `/role`) enumerate/select an aspect, or override the role with
their own label. Purely additive — no existing API or profile behavior changes.

### Added
- **`src/aspect.cyr`** — trait-derived role aspects:
  - `aspect_count()` (8), `aspect_name(i)` / `aspect_role(i)` (e.g. `measurer` → "the Measurer",
    `mediator` → "the Mediator"), `aspect_trait_offset(i)` (the `PROF_*` trait each aspect derives from),
    `aspect_index_by_name(name)` (exact lookup, −1 if none).
  - `profile_aspect_weight(p, i)` (the archetype's f64 weight for aspect `i`'s trait) and
    `profile_dominant_aspect(p)` (the highest-weighted aspect — an archetype's default role; ties → lowest
    index; null-safe).
  - Included after `types.cyr` in `src/main.cyr` / `src/lib.cyr` and the `[lib]` distlib manifest, so the
    consumer bundle (`dist/avatara.cyr`) carries it.
- **Tests**: aspect enumeration, name/role/offset mapping, `index_by_name` (hit / miss / null), and
  `profile_dominant_aspect` following the strongest mapped trait (82 assertions total).

## [2.7.2] — 2026-06-16

Toolchain maintenance release — no source changes.

### Changed
- **Toolchain pin bumped 6.1.34 → 6.2.11** (`cyrius.cyml` `[package].cyrius`); vendored `lib/` stdlib re-synced to the pinned snapshot via `cyrius lib sync` (88 → 97 `.cyr` files — the 6.2.x snapshot adds `async_agnos`, `thread_agnos`, `regression_agnos`, and the split `tls_native_*` modules; nothing avatara depends on was dropped). `cyrius deps` resolves clean and all 16 `[deps] stdlib` entries remain present in the new snapshot. Full build, smoke test, and integration suite (71 assertions) pass under 6.2.11.

### Benchmarks
- 49/49 recorded for 2.7.2 (see `bench-history.csv`). No meaningful regressions: the collection (`all_*`) benches sit in the sub-25 ns range where the integer-ns averages swing ±2–3 ns run-to-run (re-runs settle back to the 2.7.1 baseline). The one persistent signal is `kabbalah/single_profile` (~446 ns, vs 428 ns in 2.7.1), continuing the toolchain-codegen creep first noted under 6.1.34. No source changes — attributable to the compiler bump.

## [2.7.1] — 2026-06-11

Toolchain maintenance release — no source changes.

### Changed
- **Toolchain pin bumped 6.0.49 → 6.1.34** (`cyrius.cyml` `[package].cyrius`); vendored `lib/` stdlib re-synced to the pinned snapshot via `cyrius lib sync`. Full build, smoke test, and integration suite (71 assertions) pass under 6.1.34.
- **Dropped `json` from `[deps] stdlib`** — the json lib was removed from the cyrius stdlib in 6.1.x and nothing in avatara ever used it; the stale entry broke `cyrius deps` on a clean resolve (CI). Build slims accordingly (663 → 577 unreachable fns in the smoke build).

### Benchmarks
- 49/49 recorded for 2.7.1 (see `bench-history.csv`). One reproducible toolchain-codegen slowdown: `kabbalah/single_profile` 293ns → ~427ns (+46%) under 6.1.34; stable across re-runs, all other deltas within run-to-run noise (`compose/three_traditions` and collection benches return to baseline on re-run). No source changes — attributable to the compiler bump.

### Roadmap
See [docs/development/roadmap.md](docs/development/roadmap.md). All originally-roadmapped items are shipped (through v2.6.0). The former demand-gated backlog is sequenced as additive minors toward a 3.0.0 consolidation: ~~v2.7.0 Canaanite & Etruscan~~ (shipped), **v2.8.0** Tarot Major Arcana, **v2.9.0** I Ching, **v2.10.0** world-traditions completion, **v2.11.0** archetype overlays (Enneagram + Jungian); **v3.0.0** breaking consolidation (Option migration, drop `prof_*` shims, retire public `ProfLayout`, formalize overlays). Affinity-graph caching stays declined.

## [2.7.0] — 2026-06-03

Two new micro-traditions (first of the minors toward 3.0.0): **366 → 374 archetypes, 25 → 27 traditions**.

### Added
- **`src/canaanite.cyr` — Canaanite/Ugaritic** (4): **El** (kindly father/creator, `CREATION`), **Baal** (storm-king/rain, `SKY`), **Asherah** (Lady of the Sea/mother of the gods, `NATURE`), **Anat** (maiden-warrior, `WAR`). Grounded in the Ugaritic Baal Cycle.
- **`src/etruscan.cyr` — Etruscan** (4): **Tinia** (sky-king/graded thunderbolt, `SKY`), **Uni** (sovereign queen, `ORDER`), **Menrva** (wisdom/craft/war, `WISDOM`), **Voltumna** (god of the league, chthonic/earth, `NATURE`). Grounded in the Etrusca Disciplina + the Piacenza Liver.
- Registry aggregation (`all_canaanite` / `all_etruscan` + counts); both wired into all build roots and the dist bundle. 8 new tests (71 total); 3 new benchmarks (solar/canaanite/etruscan, 49 total).

### Notes
- Additive, non-breaking: existing archetypes, traditions, and the 320-byte layout unchanged; all 8 new archetypes domain-assigned (0 unspecified). No history (civilization/era) mappings added — `mapping_count()` stays 27.

## [2.6.0] — 2026-06-03

The Solar Year — avatara lands at the tropical year: **362 → 366 archetypes** (365 + the leap quarter), **24 → 25 traditions**.

### Added
- **`src/solar.cyr` — the "Solar" tradition**: four intercalary archetypes of the days *outside* the year (the calendar's reconciliation with the sun), historically grounded, no inventions:
  - **Wayeb** — the five nameless days ending the Maya Haab (liminal, unlucky; `DOMAIN_FATE`).
  - **Nemontemi** — the five "hollow" days of the Aztec xiuhpohualli (`DOMAIN_FATE`).
  - **Epagomenai** — the days "upon the year," the intercalary days on which the great gods were born outside the 360-day count (`DOMAIN_CREATION`).
  - **Bissextus** — *the quarter*: the Julian leap-day correction (Sosigenes, 46 BCE), the day that does not count yet keeps the calendar true to the sun (`DOMAIN_ORDER`).
- Registry, history-independent: `all_solar()` / `solar_count()`; folded into `all_profiles()`. "Solar" is a cross-cultural thematic grouping, so it has no single history (civilization/era) mapping — `mapping_count()` stays 27.
- 5 solar tests (65 total), including `profile_count() == 366`.

### Changed
- **Cyrius pin 6.0.47 → 6.0.49** (latest).

### Notes
- Additive and non-breaking: existing 362 archetypes, traditions, and the 320-byte layout are unchanged; "Solar" is a new 25th tradition. Consumers including the bundle gain the four archetypes automatically.

### Docs
- Roadmap cleaned up: shipped work collapsed to a summary; the former demand-gated backlog sequenced as additive minors (v2.7.0–v2.11.0) toward a v3.0.0 breaking consolidation.

## [2.5.4] — 2026-06-03

2.5.x closeout — a security/hardening audit (with CVE/0day web research) plus documentation sync.

### Security
- **Audit + web research**: avatara has no network, file, or untrusted-deserialization attack surface, and no public CVEs exist for its stack (Cyrius / AGNOS / sakshi are an internal ecosystem — confirmed via web search). The 2.4.1/2.5.1/2.5.2 hardening (NULL-profile guards, caller-offset bounds-checks via `is_f64_field_offset`, exact `compose` buffer sizing) was verified intact after the struct migration. No high-severity, memory-disclosure, overflow, or out-of-bounds issue is reachable through the public API.
- **Fixed CWE-690** (Unchecked Return Value to NULL Pointer Dereference): the stdlib `alloc()` returns `0` on OOM, and 9 sites wrote into the result unchecked (near-NULL write / UB under memory exhaustion). All heap allocation now routes through a checked **`xalloc(n)`** (`src/types.cyr`) that aborts with a diagnostic on failure — the abort-on-OOM policy Rust/Go allocators use (ADR-009).

### Added
- **ADR-008** — native struct + `#derive(accessors)` migration (records the 2.5.3 decision; was a tracked forward commitment).
- **ADR-009** — checked-allocation / abort-on-OOM policy.

### Docs
- `doc-health.md` re-swept for the 2.5.x line; `roadmap.md` reflects shipped (Result 2.5.0, shadow 2.5.1, domain 2.5.2, struct 2.5.3), declined (affinity graph), and next (v2.6.0 Solar Year).
- **2.5.x benchmark summary**: net 2.5.0 → 2.5.3 (Result allocs + domain field + ~10.4k struct setters) is noise-level on the ≥500ns paths — no regressions across the four-feature line.

## [2.5.3] — 2026-06-03

Architecture: native-struct migration of `ArchetypeProfile` (the deferred 2.5.0 item, unblocked by the cyrius 6.0.47 struct-field-cap raise). Mechanical/behavior-preserving — all 60 tests pass unchanged.

### Changed
- **`ArchetypeProfile` is now a native `#derive(accessors)` `struct Profile`** (40 i64 fields = 320 bytes, declaration order matching the existing offsets). The compiler-generated `Profile_<field>(p)` / `Profile_set_<field>(p, v)` are canonical.
- **~10.4k field writes across 24 modules converted** from `store64(p + PROF_*, v)` to `Profile_set_*(p, v)` (constructors, `profile_new`, `compose`, `shadow`). The loop-based code that iterates computed offset ranges (`profile_new` default-fill, `compose` blend, `affinity`/`error` range scans, `shadow` inversion) keeps raw `load64`/`store64` — 6 such loops retained.
- **The 39 hand-written `prof_*` getters now delegate** to the derived `Profile_*` (e.g. `prof_warmth(p)` → `Profile_warmth(p)`); `prof_*` are kept as consumer-facing compat shims, so no downstream call sites break.
- `ProfLayout` offset enum retained — used by the loop-based code and the layout-assertion test.

### Added
- **Layout-assertion test** (5 asserts, 60 total): `sizeof == 320` and each `Profile_set_*` writes at its `PROF_*` offset — guards the struct↔offset correspondence against future field changes. A named-field struct also makes the 2.4.5 `PROF_SPIRIT` offset-collision class of bug a compile error (duplicate field names are rejected).

### Notes
- Behavior- and perf-neutral: identical codegen semantics; benchmarks within run-to-run noise (single-profile construction ~365ns, affinity score ~102ns).

## [2.5.2] — 2026-06-03

Structural enrichment — the `domain` field (slipped from the 2.4.0 roadmap), plus a toolchain bump.

### Added
- **`domain` field** on `ArchetypeProfile` — a categorical primary-sphere axis orthogonal to the trait/emphasis numbers. New `enum Domain` (20 spheres: Creation, War, Love, Death, Wisdom, Order, Chaos, Nature, Sky, Sea, Fire, Sun, Moon, Fate, Trickery, Healing, Prosperity, Hearth, Transcendence, + Unspecified default). Field appended at offset 312 (profile **312 → 320 bytes**; all prior offsets unchanged). Accessors: `prof_domain(p)`; registry `query_domain(domain)` + `query_count_domain(domain)`.
- A scholarly primary domain assigned to **all 362 archetypes** (0 unspecified), using each archetype's established correspondence. Distribution: transcendence 74, wisdom 38, war 32, order 29, healing 23, love 22, nature 22, death 21, creation 14, sky 16, sea 13, sun 11, fire 9, trickery 9, prosperity 8, moon 6, fate 6, hearth 5, chaos 4.
- 6 domain tests (55 total).

### Changed
- **Cyrius pin 6.0.40 → 6.0.47** — among other fixes, 6.0.47 raised the struct field cap 32 → 256 (our filed proposal), which **unblocks the deferred struct migration** (a 40-field `#derive` struct now compiles). The struct migration remains a separate future release; the domain field here was done on the existing manual offset layout.

### Notes
- Layout grew by 8 bytes (one i64). Consumers reading the bundle inherit the `domain` field and accessors; existing offsets are unchanged, so existing field reads are unaffected.

## [2.5.1] — 2026-06-02

Structural enrichment — the shadow aspect (slipped from the 2.4.0 roadmap).

### Added
- **Shadow aspect** (`src/shadow.cyr`) — `shadow(profile)` returns the dark/inverted form of an archetype: traits & emphases → `1.0 − v`; breath mirrored across unity (UNITY fixed, else `7 − b`); growth (`DIFFERENTIATE↔INTEGRATE`, `PRESERVE↔TRANSFORM`) and polarity (`MASCULINE↔FEMININE`) inverted; element/tier preserved; name → `"Shadow of <name>"`. The inversion is **involutive** — `shadow(shadow(x))` restores every inverted dimension. Plus `is_shadow_of(a, b)`.
- 10 shadow tests (49 total).

### Notes
- **Affinity graph (planned) evaluated and declined.** A pre-computed, pointer-keyed cross-tradition cache regressed the benchmark (`cross_tradition_match` 49µs → 945µs): the common construct-then-query pattern passes fresh profile pointers that miss a pointer-keyed cache, and the per-tradition build path is slower than the existing single O(n) pass. Kept the original `cross_tradition_match`/`cross_tradition_matches`. A non-pointer-keyed (index/name-based) approach could revisit this later.
- **Domain field (planned) deferred** — a layout change + 362-archetype scholarly assignment the roadmap wanted folded into the struct migration, which remains blocked on the cyrius 32-field struct cap.

## [2.5.0] — 2026-06-02

Architecture Modernization — adopts Cyrius tagged `Result<T, E>` for the error
model. **API-breaking** for consumers (return types changed). The companion
struct/`#derive(accessors)` migration is deferred (see Notes).

### Changed (breaking)
- **`lookup(name)` / `lookup_in(tradition, name)`** now return `Result` — `Ok(profile)` or `Err(ERR_UNKNOWN_ARCHETYPE)` — instead of a profile pointer or `0`.
- **`compose(weighted)`** now returns `Result` — `Ok(profile)` or `Err(ERR_INVALID_PARAMETER)` (empty vec / NULL profile / negative or NaN weight / total ≤ 0) — instead of a pointer or `0`.
- **`validate_profile(p)`** now returns `Result` — `Ok(p)` or `Err(ERR_INVALID_PARAMETER | ERR_OUT_OF_RANGE)` — instead of a bare `AvataraError` int.
- `AvataraError` codes are now carried as the `Err` payload (read with `err_code_of`).

### Added
- `lib/result.cyr` to the stdlib deps; `Result`/`Ok`/`Err` + helpers (`is_ok`, `is_err_result`, `result_unwrap`, `result_unwrap_or`, `err_code_of`) are now the error vocabulary.
- **`find_and_validate(name)`** — chains `lookup` + `validate_profile` via the `?` propagation operator (the lookup `Err` short-circuits).

### Consumer migration
- `var p = lookup(n); if (p == 0) {…}` → `var r = lookup(n); if (is_err_result(r) == 1) {…} var p = result_unwrap(r);` (same shape for `compose`). Build on cyrius 6.0.39+ with `result` (or `tagged`) in `[deps] stdlib`.

### Notes
- Internal range predicates (`require_unit_range` / `require_all_unit_range`) stay bare-int — they run in per-field loops where a `Result` heap alloc per element would be wasteful.
- `cross_tradition_match` / `find_mapping` still return `0` for "not found" — those are absence (Option-shaped), not errors; a future pass may move them to `Option`.
- **Deferred:** the `struct` + `#derive(accessors)` migration of `ArchetypeProfile` is blocked — cyrius caps structs at 32 fields and `Profile` has 39. Proposal filed (`cyrius/.../2026-06-02-struct-field-cap-raise.md`); resumes when the cap is raised. This was originally the other half of 2.5.0.

## [2.4.5] — 2026-06-02

Correctness fix, surfaced while scoping the 2.5.0 struct migration. **Behavior-changing** (affinity/compose outputs shift) — isolated in its own release so it's bisectable before the migration.

### Fixed
- **`PROF_SPIRIT` was defined twice** in `ArchetypeProfile`'s layout — offset 176 (spirit *emphasis*, f64) and offset 304 (spirit *text*, ptr). "Last definition wins" collapsed both to 304, so:
  - every archetype's **spirit emphasis was silently stuck at the 0.5 default** — the 316 constructor writes of real emphasis values (e.g. Chokmah's 0.9) landed on offset 304 and were then overwritten by the spirit-text pointer;
  - **`prof_spirit_emph()` returned garbage** (it read the text pointer reinterpreted as an f64).
- Renamed the text constant to `PROF_SPIRIT_TEXT` (offset 304); `PROF_SPIRIT` is now solely offset 176. Repointed the 362 spirit-text constructor writes + the text accessor + `compose()`'s spirit copy + `profile_new()`'s text init at `PROF_SPIRIT_TEXT`; the 316 numeric emphasis writes now correctly land on 176.
- Net effect: the spirit-emphasis dimension is now live (was inert at 0.5 for all 362 archetypes), so `affinity`, `similar_to`, `cross_tradition_match`, `compose`, and `detect_conflicts` now factor in real spirit-emphasis values. `prof_spirit_emph()` returns the real f64.

### Notes
- Layout unchanged (still 312 bytes, same offsets) — only the constant *name* for 304 changed and the emphasis slot at 176 now actually populates. Consumers including `dist/avatara.cyr` pick up both the data fix and the working `prof_spirit_emph()`.

## [2.4.4] — 2026-06-02

2.4.x closeout — documentation + roadmap housekeeping. No source changes.

### Changed
- **Roadmap re-bucketed** (`docs/development/roadmap.md`): added **v2.5.0 — Architecture Modernization** (the deferred `struct` + `#derive(accessors)` migration and `Result<T, E>` error model); slipped **Structural Enrichment** (domain field, affinity graph, shadow aspect) from 2.4.0 → **v2.5.x**; slipped **The Solar Year** to **v2.6.0**. `CHANGELOG [Unreleased]` updated to match.

### Added
- **`docs/doc-health.md`** — living doc-currency ledger (cyrius/vidya convention).

### Notes
- **2.4.x benchmark summary** (net 2.4.0 → 2.4.3, recorded in `bench-history.csv`): every meaningful (≥500ns) path got faster, no regressions — history queries −16% to −25%, `registry/by_tradition` −23%, `registry/query_courage` −29% (from the `query_by_traditions` refactor + the 6.0.40 toolchain). Sub-500ns rows are timer-noise dominated.
- Closes the 2.4.x line: cyrius 3.10.0 → 6.0.40, NULL/overflow hardening, `+=`/`match` modernization, stdlib `f64_le`/`f64_ge` alignment.

## [2.4.3] — 2026-06-02

Toolchain bump + stdlib alignment.

### Changed
- **Cyrius pin 6.0.38 → 6.0.40.** `f64_le`/`f64_ge` landed in stdlib `lib/math.cyr` at 6.0.39 (from the proposal filed during the 2.4.x sweep); 6.0.40 is the current public release and is pinned here.
- **Dropped avatara's local `f64_le`/`f64_ge`** (`src/types.cyr`) — now supplied by stdlib. Keeping them would have collided with the new stdlib definitions (duplicate-fn, last-wins) — the exact shadowing risk the proposal called out.

### Consumer note
- `dist/avatara.cyr` no longer carries `f64_le`/`f64_ge`. Consumers that include the bundle must build on cyrius **6.0.39+** and have `"math"` in their `[deps] stdlib` so the stdlib definitions resolve. (avatara already lists `math`.)

### Notes
- Evaluated the optional `#regalloc` perf hint on the hot `affinity`/`compose` loops and **declined it**: the directive is unused across the cyrius stdlib and reference projects (vidya), the hot path (`affinity` score = 92ns) is already tight, and run-to-run bench variance exceeds any plausible gain. Not worth an ecosystem-unique tuning knob in the source.

## [2.4.2] — 2026-06-02

Language modernization — adopting Cyrius 6.x idioms. No behavior or API
changes; codegen is identical (verified: build, 39 tests, lint, benchmarks).

### Changed
- **Compound assignment** — converted 91 integer accumulator/loop sites (`i = i + 1`, `off = off + 8`, `j = j - 1`, etc.) to `+=` / `-=` across the logic modules (types, error, compose, affinity, registry, history, incarnate). The f64 accumulators stay on the `f64_add` builtins (bit-pattern math — `+=` would be wrong there).
- **`breath_intensity()`** now uses a `match` over `BreathAffinity` instead of an if-chain, gaining a compile-time exhaustiveness check on the enum.

## [2.4.1] — 2026-06-02

Hardening + refactor sweep. No API removals; behavior changes are limited to
rejecting invalid input that previously caused undefined behavior.

### Fixed
- **NULL-profile guards** (`affinity`, `trait_affinity`, `compose`, `conflicts`/`detect_conflicts`, `similar_to`, `cross_tradition_match`, `cross_tradition_matches`) — a `0` returned by `lookup`/`lookup_in`/`cross_tradition_match` on a miss no longer flows into `load64(p + off)` and dereferences near-NULL; these now return a `0`/empty sentinel.
- **`compose()` buffer overflow** — the composite name/tradition buffers were fixed at 512/256 bytes and written unchecked; composing many archetypes (e.g. all 24 traditions) overran them and corrupted the bump heap. Buffers are now sized exactly from the input.
- **`compose()` rejects NULL profiles and NaN weights** (in addition to the existing empty-vec / negative-weight / zero-total guards).
- **Out-of-range query offsets** — `query_min_trait`/`query_max_trait`/`query_count_min_trait`/`filter_min_trait` now bounds-check the caller-supplied `offset` (via `is_f64_field_offset`) and return empty instead of reading outside the 312-byte profile.

### Changed
- **`all_incarnate()`** rebuilt from the six cached subgroup collections instead of re-listing and re-constructing all 56 incarnate profiles — removes a 56-profile double-allocation (~50 LOC).
- **`profile_new()`** default-fills the 29 contiguous trait/emphasis fields with a loop over `TW_FIRST..ME_LAST` instead of 29 literal stores.
- **`affinity`/`trait_affinity`** share a `sum_abs_diff(a, b, first, last)` helper.
- The three history queries (`query_civilization`/`query_era`/`query_active_at`) delegate to a shared `query_by_traditions(trads)`.
- Affinity tuning values are named: `conflict_threshold()` (0.4) and `INCOMPAT_MIN_CONFLICTS` (5).
- `compose()` dominant-entry tie-break now picks the first max-weight entry (matches composite-name ordering).

### Added
- **`validate_profile(p)`** — public entry point wiring up the previously-unused validation layer (`require_unit_range`/`require_all_unit_range`); returns `ERR_INVALID_PARAMETER`/`ERR_OUT_OF_RANGE`/`ERR_NONE`.

### Docs
- Corrected the history-mapping count to 27 (was variously 25/26 in comments and CLAUDE.md; the test already asserted 27).
- Smoke-test banner no longer hardcodes the version string (avoids per-release drift).

## [2.4.0] — 2026-06-02

### Changed
- **Cyrius toolchain 3.10.0 → 6.0.38** — migrated to the current compiler. Build, 39 integration tests, and 45 benchmarks all pass clean.
- **Manifest `cyrius.toml` → `cyrius.cyml`** — new format with `version = "${file:VERSION}"` template (VERSION is now the single source of truth) and `cyrius = "6.0.38"` toolchain pin.
- Compiler version pin moved from the standalone `.cyrius-toolchain` file into `cyrius.cyml` `[package].cyrius`; `.cyrius-toolchain` removed.
- CI/release workflows modernized to match sibling projects (patra, vidya): toolchain version read dynamically from `cyrius.cyml` and installed via the upstream `install.sh`; added a lint gate (warnings-as-errors), a benchmark run, and version-consistency checks.
- Stale committed `lib/` stdlib snapshot removed from version control; dependencies now resolved at build time via `cyrius deps` (lib/ gitignored).

### Added
- **`dist/avatara.cyr`** — single-file consumer bundle generated by `cyrius distlib` (via a new `[lib]` section in `cyrius.cyml`), for consumers (bhava, joshua, kiran, agnosai, hadara).
- Benchmark regression tracking established as a standard versioning practice (`scripts/bench-history.sh` → `bench-history.csv`) to surface deltas and potential regressions across releases.

### Fixed
- Renamed `map_new` → `history_map_new` in `src/history.cyr` to resolve a name collision with stdlib `hashmap.cyr`'s `map_new` (which surfaced as a build warning under cyrius 6.x).

## [2.3.0] — 2026-04-13

### Added
- **Affinity module** (`affinity.cyr`) — composition intelligence system:
  - `affinity(a, b)` — similarity score (0.0 to 1.0) across all 15 traits + 14 emphases
  - `trait_affinity(a, b)` — trait-only similarity (ignores emphasis)
  - `similar_to(profile, max)` — find N most similar archetypes, sorted by score
  - `cross_tradition_match(profile)` — best match from a different tradition
  - `cross_tradition_matches(profile, max)` — best match per foreign tradition
  - `detect_conflicts(a, b)` — traits with >0.4 absolute difference, sorted by delta
  - `is_incompatible(a, b)` — true if 5+ conflicting traits
- 9 new test assertions for affinity, similarity, cross-tradition, and conflict detection (39 total integration tests)

### Changed
- `src/main.cyr` split into slim smoke test (builds fast) + `tests/avatara.tcyr` (full test suite)
- Smoke test prints "all systems nominal" on success

## [2.2.0] — 2026-04-12

### Added
- **Finnish/Sami tradition** (`finnish`) — 14 entities: Kalevala figures (Vainamoinen, Ilmarinen, Lemminkainen, Louhi, Joukahainen, Kullervo, Marjatta) and Finnish/Sami gods (Ukko, Tapio, Mielikki, Ahti, Tuoni, Loviatar, Madderakka)
- **Vodou Lwa tradition** (`vodou`) — 14 entities across three nachon: Rada (Papa Legba, Damballa Wedo, Ayida Wedo, Agwe, Erzulie Freda, Loko), Petwo (Erzulie Dantor, Simbi, Marinette), Ghede (Baron Samedi, Maman Brigitte, Baron Kriminel), plus Ogou Feray and Marasa
- **5 new Incarnate Mystics** — Desert Fathers, Gregory Palamas, Thomas Merton, Attar, Al-Ghazali (incarnate mystic count: 12 to 17)
- History mappings for Finnish and Vodou traditions
- ADR-006: Cyrius port decision record
- Tests for all new entities (190 total assertions)

### Changed
- Total archetypes: 329 to 362 (+33)
- Total traditions: 22 to 24 (+2)
- README rewritten for Cyrius (examples, build instructions, updated tradition table)
- Architecture overview updated for Cyrius data flow
- Roadmap updated (completed items removed, v2.3.0 planned)
- Scripts updated for Cyrius (bench-history.sh, version-bump.sh)

## [2.1.0] — 2026-04-12

### Added
- Rust source removed from repo (preserved in git history at v2.0.0)
- `benchmarks-rust-v-cyrius.md` — full Rust v1.1.0 vs Cyrius v2.0.1 comparison with code metrics, binary size, and all benchmark numbers
- 39 benchmarks (up from 19), matching and exceeding Rust Criterion suite (29)
- 195 test assertions (up from 122), covering all 19 traditions with per-entity spot checks
- Per-tradition tests: angelic, hindu, olympian, egyptian, buddhist, mesopotamian, celtic, shinto, aztec, maya, yoruba, zoroastrian, taoist, polynesian, slavic, jain, sikh, incarnate

## [2.0.1] — 2026-04-12

### Fixed
- `f64_le` and `f64_ge` defined as helpers (not cc3 builtins)
- `sakshi.cyr` include missing from main.cyr
- `history.cyr` formatting (cyrfmt compliance)
- Integration test (`avatara.tcyr`) — corrected assert function names (`assert_neq`, `assert_gte`), added proper exit syscall
- Mapping count corrected to 25 (was incorrectly 26)
- Stray binary artifact removed from repo

### Added
- `require_all_unit_range()` — validates all f64 values in a profile range
- `query_count_min_trait()` — count matching profiles without allocating
- `benchmarks-rust-v-cyrius.md` — Rust v1.0.0 baseline for comparison
- CI/release workflows rewritten for Cyrius (cc3 3.7.0)

## [2.0.0] — 2026-04-12

Complete rewrite from Rust to Cyrius. All ~206 archetypes across 19 traditions preserved with identical trait values, soul text, and spirit text.

### Added
- `src/types.cyr` — 312-byte ArchetypeProfile with inline TraitWeights (15 f64) and ModuleEmphasis (14 f64); 5 enums (BreathAffinity, GrowthDirection, Element, Polarity, CosmicTier); `profile_new()` constructor with 0.5 defaults
- `src/history.cyr` — 25 tradition-to-history mappings with civilization names, era names, temporal ranges, and scholarly notes; `context_for_tradition()`, `traditions_for_civilization()`, `traditions_active_at()`, `traditions_for_era()`
- `src/registry.cyr` — `query_civilization()`, `query_era()`, `query_active_at()` history-based filters
- `tests/avatara.tcyr` — integration test suite (entity counts, range validation, duplicate detection, breath monotonicity, compose invariants, history queries)
- `tests/avatara.bcyr` — benchmarks for all traditions, registry, compose, and history
- `programs/traditions.cyr` — example: explore archetypes, courage query, tradition counts
- `programs/compose.cyr` — example: blend three traditions
- `tests/test.sh` — test runner script
- `lib/bench.cyr` — benchmark framework

### Changed
- Language: Rust → Cyrius (18,804 LOC → ~15,600 LOC across 28 modules)
- Build: `Cargo.toml` → `cyrius.toml` (cc3 compiler)
- Types: manual memory layout with `alloc()`/`store64()`/`load64()`
- f64 weights: IEEE 754 bit patterns with `f64_*` builtins
- QueryBuilder fluent API → procedural `query_*()` filter functions
- `Archetype` trait → per-entity constructor functions (e.g. `kabbalah_kether()`)
- Lazy-init `all_*()` collection functions with global cache pattern
- Logging: tracing → sakshi

### Removed
- serde Serialize/Deserialize
- thiserror (replaced with integer error codes)
- Criterion benchmarks (replaced with Cyrius bench framework)

### Preserved
- All ~206 archetypes across 19 traditions with identical trait values
- Composition system (weighted blending with breath intensity averaging, growth/tier voting)
- Registry lookup and query API
- All soul text and spirit text verbatim
- 25 tradition-to-history mappings with scholarly notes
- Rust source removed (port complete)

## [1.1.0] — 2026-04-01 (Rust)

Historical context integration via itihas. Rust era (source removed in v2.0.1).

## [1.0.0] — 2026-03-31 (Rust)

Initial release. 19 traditions, ~206 archetypes, composition API, registry, query builder. Rust era (source removed in v2.0.1).
