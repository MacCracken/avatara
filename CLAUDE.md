# Avatara — Claude Code Instructions

## Project Identity

**Avatara** (Sanskrit: अवतार — descent of the divine) — Divine archetype engine: theological and mythological personality mapping across traditions

- **Language**: Cyrius (ported from Rust in v2.0)
- **License**: GPL-3.0-only
- **Version**: SemVer 2.14.0
- **Compiler**: cyrius >= 6.4.71 (pinned in `cyrius.cyml` `[package].cyrius`)

## Consumers

bhava (emotion/personality — post-v2.0 archetype overlay), joshua (NPC divine archetypes), kiran (game entities), agnosai (agent personalities with theological depth), hadara (archetype-to-culture context — ready for integration), sankhya (ancient sciences — planned; shared IncarnateSage/Vedic bridge)

## Architecture

- `src/lib.cyr` — public API: includes all modules
- `src/main.cyr` — smoke test (6 checks: profile count, tradition count, lookup, compose, affinity, history)
- `src/types.cyr` — ArchetypeProfile layout (320 bytes), TraitWeights (15 f64), ModuleEmphasis (14 f64), enums (BreathAffinity, GrowthDirection, Element, Polarity, CosmicTier, Domain)
- `src/error.cyr` — AvataraError enum codes, validation
- `src/aspect.cyr` — role aspects: universal roles DERIVED from the trait weights, no per-archetype authoring (added 2.8.0). Like `shadow` and `overlay`, it reads off a finished profile and stores nothing
- `src/compose.cyr` — archetype composition: weighted blending of multiple profiles
- `src/history.cyr` — 37 tradition-to-history mappings (civilization, era, temporal range, notes). A test walks both directions: no mapping may name a tradition with no profiles, and every tradition carried must resolve a context
- `src/registry.cyr` — lookup by name, enumeration, query/filter API (includes history-based queries)
- `src/affinity.cyr` — affinity scoring, similarity search, cross-tradition matching, conflict detection
- `src/shadow.cyr` — shadow aspect: `shadow(profile)` (involutive inversion), `is_shadow_of(a, b)`
- `src/overlay.cyr` — archetype overlays: cross-cutting typologies DERIVED over the profile, not archetypes (no registry/history entry, `profile_count()` unaffected). Enneagram (9 types, 3 centres, wings — the types are a ring) and Jungian (Hero/Shadow/Anima/Self/Trickster), both registered in the `OverlaySystem` enum. Overlays are **plural readings, not definitions**: several may sit over the same figure and disagree, and a new system is added without touching any archetype. `jungian_shadow_form(p)` is the composition point with `shadow.cyr`: the SHADOW overlay scores what p expresses, `shadow(p)` constructs a different archetype
- `src/kabbalah.cyr` — Tree of Life: 10 Sephiroth
- `src/tarot.cyr` — 22 Tarot Major Arcana as the 22 Tree-of-Life paths (bridges `kabbalah`): each trump carries its Hebrew letter, path number (11–32), and the two Sephiroth its path connects (`tarot_path`, `tarot_path_upper/lower`, `tarot_for_path`, `tarot_connects`); Golden Dawn attribution, VIII=Strength/XI=Justice
- `src/iching.cyr` — 64 I Ching hexagrams (King Wen sequence) over the eight trigrams (bagua): each hexagram carries its number, six lines, and two constituent trigrams (`iching_code` packs lower*8+upper; `iching_lower/upper/line/number/yang_count`, `iching_for_trigrams`, `trigram_name/image/attribute/family/element`). Element derives from the upper trigram, polarity from the yang-line balance; all 64 trigram pairings occur exactly once (bijection, test-pinned)
- `src/angelic.cyr` — 9 angelic orders, 7 archangels
- `src/hindu.cyr` — Trimurti, 11 Devas, 10 Avatars of Vishnu
- `src/olympian.cyr` — 15 Greek deities (12 Olympians + Hades, Hestia, Persephone)
- `src/norse.cyr` — 13 Aesir/Vanir gods
- `src/egyptian.cyr` — 16 principal deities
- `src/buddhist.cyr` — 7 Bodhisattvas, 5 Dhyani Buddhas
- `src/mesopotamian.cyr` — 14 Sumerian/Babylonian deities
- `src/celtic.cyr` — 17 Tuatha De Danann & Insular Celtic deities (incl. the healers Miach & Airmed)
- `src/shinto.cyr` — 15 Japanese Kami
- `src/aztec.cyr` — 14 Aztec (Mexica) deities
- `src/maya.cyr` — 12 Maya deities
- `src/yoruba.cyr` — 14 Yoruba/Ifa Orishas
- `src/zoroastrian.cyr` — 7 Amesha Spentas, 7 Zoroastrian beings
- `src/taoist.cyr` — 8 Immortals, 8 celestial deities
- `src/polynesian.cyr` — 12 Polynesian/Hawaiian deities
- `src/slavic.cyr` — 12 pre-Christian Slavic deities
- `src/jain.cyr` — 24 Tirthankaras
- `src/sikh.cyr` — 10 Sikh Gurus
- `src/finnish.cyr` — 14 Kalevala figures & Sami spirits
- `src/vodou.cyr` — 14 Vodou Lwa (Rada, Petwo, Ghede)
- `src/solar.cyr` — 4 intercalary archetypes: the days upon the year (Wayeb, Nemontemi, Epagomenai) + the leap quarter (Bissextus) — avatara's landing at the solar year (365 + ¼)
- `src/canaanite.cyr` — 4 Canaanite/Ugaritic deities (El, Baal, Asherah, Anat)
- `src/etruscan.cyr` — 4 Etruscan deities (Tinia, Uni, Menrva, Voltumna)
- `src/incarnate.cyr` — 56 incarnate divine figures (Hindu, Buddhist, Mystic, Taoist, Sage, and four North American figures re-attributed in 2.12.0 to Lakota / Haudenosaunee / Comanche / Northern Paiute — the pan-ethnic "Indigenous" label is retired)
- `src/inuit.cyr` — 10 Inuit spirits and powers (Sedna, Nanuq, Sila, the animal-masters)
- `src/lakota.cyr` — 10 Lakota wakan powers (Wakan Tanka, Inyan, Skan, Wakinyan, Iktomi)
- `src/haudenosaunee.cyr` — 6 Six Nations figures (Sky Woman, the twins, Three Sisters, Great Turtle)
- `src/anishinaabe.cyr` — 5 Anishinaabe manidoog + the 7 Grandfather Teachings (Nizhwaaswi Gagiikwewin), carried by their Ojibwe names as cosmic-tier principles; the wiindigoo is deliberately not carried — see module header
- `src/aboriginal.cyr` — 4 figures of three Aboriginal Australian peoples, carried under their own peoples' tradition strings (Kunwinjku, Kulin, Gunaikurnai), not a continent-wide label; `traditions_for_civilization("Aboriginal Australia")` gathers them. Restricted Dreaming material deliberately excluded, and the v2.14.0 survey's six refusals are recorded with their reasons (see module header)
- `src/logging.cyr` — sakshi logging init
- `tests/avatara.tcyr` — integration test suite (295 assertions)
- `tests/avatara.bcyr` — benchmarks (60)
- `programs/traditions.cyr` — example: explore archetypes
- `programs/compose.cyr` — example: blend traditions

## Type System

All values are i64. f64 trait/emphasis weights stored as IEEE 754 bit patterns. Use f64_* builtins for arithmetic and comparison.

- `ArchetypeProfile` — a native `#derive(accessors)` `struct Profile` (`src/types.cyr`), 40 i64 fields = 320 bytes (incl. `domain` at offset 312). The compiler-generated `Profile_<field>(p)` / `Profile_set_<field>(p, v)` are canonical; constructors set fields via `Profile_set_*`. The `prof_*` accessors are thin consumer-facing shims that delegate to `Profile_*`. The `ProfLayout` offset enum is retained for the loop-based code (compose/affinity/error iterate offset ranges) and the layout-assertion test.
- `profile_new()` — allocates with defaults (traits=0.5, emphasis=0.5, breath=LATE_EXHALE, growth=DIFFERENTIATE)
- All heap allocation routes through `xalloc(n)` (`src/types.cyr`) — checked alloc that aborts on OOM (CWE-690 guard; abort-on-OOM policy, see ADR-009). Use `xalloc`, not raw `alloc`, for new profile/struct allocations.
- Each tradition module: entity functions (e.g. `kabbalah_kether()`) + lazy-init `all_*()` collection + `*_count()`
- Registry: `all_profiles()`, `by_tradition()`, `query_*()` filters; `lookup(name)` / `lookup_in(tradition, name)` return `Result` (`Ok(profile)` / `Err(ERR_UNKNOWN_ARCHETYPE)`); `find_and_validate(name)` chains lookup + validate via `?`
- Compose: `compose(weighted_vec)` — weighted blending; returns `Result` (`Ok(profile)` / `Err(ERR_INVALID_PARAMETER)`)
- Errors: `Result<T, E>` from `lib/result.cyr` (`is_ok` / `is_err_result` / `result_unwrap` / `err_code_of`); `validate_profile(p)` returns `Result`. `AvataraError` codes are the `Err` payload. Internal range predicates (`require_unit_range`/`require_all_unit_range`) stay bare-int (loop-hot)
- History: `context_for_tradition()`, `traditions_for_civilization()`, `traditions_active_at()`, `traditions_for_era()`
- Affinity: `affinity()`, `similar_to()`, `cross_tradition_match()`, `cross_tradition_matches()`, `detect_conflicts()`, `is_incompatible()`. `similar_to(src, k)` uses bounded top-k selection (O(N·k)) — do not reintroduce a full sort before trimming; `k <= 0` still means "all, sorted"
- Shadow: `shadow(profile)` — inverted form (traits→1−v, breath/growth/polarity mirrored, element/tier kept); involutive; `is_shadow_of(a, b)`
- Domain: every archetype has a `Domain` (primary sphere, offset 312); `prof_domain(p)`, `query_domain(domain)`, `query_count_domain(domain)`
- Overlays: `profile_enneagram_type/wing/centre/score(p)`, `profile_jungian_role/score(p)`, `jungian_shadow_form(p)`. Derived, never stored; adding an overlay must not change `profile_count()`. Generic registry API — `overlay_system_count/name/by_name`, `overlay_label_count/label`, `overlay_score(s,p,i)`, `overlay_best(s,p)` — enumerates every registered system without naming one; out-of-range systems degrade (`"unknown"`/0/0.0), they do not trap

## Key Principles

- All traditions map to the same ArchetypeProfile output — composable across cultures
- Plain f64/enum outputs only — no bhava types leak into avatara
- Zero external dependencies except sakshi (logging)
- f64 values in 0.0-1.0 range for all traits and emphases
- Historically and theologically accurate — real traditions, real correspondences
- Respectful representation — these are living traditions for billions of people
- **Traditions and typologies are mutually exclusive.** A tradition is a people's own account of who its
  figures are and lives in `registry`/`history` with real archetypes. A typology (Enneagram, Jungian, and
  anything like them) is a modern analytic grid and may only ever exist as an **overlay** — derived over
  the profile, never instantiated as archetypes, never given a `tradition` string, never counted in
  `profile_count()`. `src/overlay.cyr` is the sanctioned form. Do not add typology types as archetypes;
  a test enforces this, walking the `OverlaySystem` registry so a system registered later is covered
  automatically. Overlays do not hardlock meaning: they are readings, plural and revisable, and new
  systems may be laid over traditions already carried — which is exactly why they derive and store nothing

## Versioning & Benchmarking

- **VERSION is the single source of truth.** `cyrius.cyml` derives its version via the `${file:VERSION}` template; CHANGELOG.md must carry a matching `## [X.Y.Z]` entry. Bump with `scripts/version-bump.sh X.Y.Z` (updates VERSION + CLAUDE.md, stubs the CHANGELOG, regenerates `dist/avatara.cyr`). CI gates version consistency.
- **Toolchain pin** lives only in `cyrius.cyml` `[package].cyrius` — there is no `.cyrius-toolchain` file. CI reads it dynamically and installs via the upstream `install.sh`. Bumping the cyrius version is a deliberate, separate change from the package version.
- **Benchmark every release.** Run `scripts/bench-history.sh` as a standard step of each version bump — it builds `tests/avatara.bcyr`, runs the suite, and appends timings to `bench-history.csv` (keyed by date + version). Compare against the prior release's rows to discover deltas and surface potential regressions before tagging; call out any meaningful slowdown in the CHANGELOG. CI also runs the benchmark suite to catch build/perf breakage on every push.
- **`dist/avatara.cyr`** is the committed consumer bundle (`cyrius distlib`, driven by the `[lib]` section). Regenerate and commit it whenever `src/` changes; CI fails if it is stale.

## DO NOT

- **Do not commit or push** — the user handles all git operations
- **NEVER use `gh` CLI** — use `curl` to GitHub API only
- Do not invent theological associations — use established correspondences from scholarly sources
- Do not trivialize or mock any tradition
- Do not mix traditions without clear compositional semantics
