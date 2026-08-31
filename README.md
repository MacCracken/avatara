# avatara

> **Avatara** (Sanskrit: अवतार — descent of the divine, manifestation of archetypes)

**Divine archetype engine** — theological and mythological personality mapping across traditions. Part of the [AGNOS](https://github.com/MacCracken) ecosystem.

[![License: GPL-3.0-only](https://img.shields.io/badge/license-GPL--3.0--only-blue.svg)](LICENSE)

## What It Does

Maps divine and mythological beings across world traditions to composable personality configurations. Not religion simulation — psychometric archetype mapping backed by trait math. Each entity produces an `ArchetypeProfile` (320 bytes) with trait weights, module emphasis, breath phase affinity, growth direction, and primary domain.

**504 archetypes across 37 traditions** (incl. the solar-year intercalary set, the 22 Tarot Major Arcana, the 64 I Ching hexagrams, and named-nation Inuit, Lakota, Haudenosaunee, Anishinaabe and Aboriginal Australian traditions). Written in Cyrius, compiled by the Cyrius compiler 6.5.36+.

## Traditions

Tradition and module are not one-to-one: `incarnate` spans nine traditions, and `aboriginal` carries
three peoples. The `tradition` field is what `by_tradition()` and `cross_tradition_match()` key on.

| Tradition | Module | Entities |
|-----------|--------|----------|
| **Kabbalah** | `kabbalah` | 10 Sephiroth (Kether through Malkuth) |
| **Angelic** | `angelic` | 7 Archangels, 9 Angelic Orders |
| **Hindu** | `hindu` | Trimurti (3), Devas (11), Avatars of Vishnu (10) |
| **Greek** | `olympian` | 15 deities (12 Olympians + Hades, Hestia, Persephone) |
| **Norse** | `norse` | 13 Aesir/Vanir gods |
| **Egyptian** | `egyptian` | 16 principal deities |
| **Buddhist** | `buddhist` | 7 Bodhisattvas, 5 Dhyani Buddhas |
| **Mesopotamian** | `mesopotamian` | 14 Sumerian/Babylonian deities |
| **Celtic** | `celtic` | 17 Tuatha De Danann & Insular Celtic deities |
| **Shinto** | `shinto` | 15 Japanese Kami |
| **Aztec** | `aztec` | 14 Aztec (Mexica) deities |
| **Maya** | `maya` | 12 Maya deities |
| **Yoruba** | `yoruba` | 14 Orishas |
| **Zoroastrian** | `zoroastrian` | 7 Amesha Spentas, 7 Zoroastrian beings |
| **Taoist** | `taoist` | 8 Immortals, 8 celestial deities |
| **Polynesian** | `polynesian` | 12 Polynesian/Hawaiian deities |
| **Slavic** | `slavic` | 12 pre-Christian Slavic deities |
| **Jain** | `jain` | 24 Tirthankaras |
| **Sikh** | `sikh` | 10 Sikh Gurus |
| **Finnish** | `finnish` | 14 Kalevala figures & Sami spirits |
| **Vodou** | `vodou` | 14 Lwa (Rada, Petwo, Ghede) |
| **Mystic / Vedic** *(+ Hindu, Buddhist, Taoist above)* | `incarnate` | 56 incarnate masters. `incarnate` is a module, not a tradition — its figures carry Mystic (17), Hindu (13), Buddhist (10), Vedic (7), Taoist (5), and four who carry their own nations (Lakota, Haudenosaunee, Comanche, Northern Paiute) |
| **Solar** | `solar` | 4 intercalary archetypes (the days upon the year + the leap quarter) |
| **Canaanite** | `canaanite` | 4 Ugaritic deities (El, Baal, Asherah, Anat) |
| **Etruscan** | `etruscan` | 4 Etruscan deities (Tinia, Uni, Menrva, Voltumna) |
| **Tarot** | `tarot` | 22 Major Arcana as the Tree-of-Life paths (bridges Kabbalah) |
| **I Ching** | `iching` | 64 hexagrams (King Wen sequence) over the eight trigrams |
| **Inuit** | `inuit` | 10 Arctic spirits and animal-masters |
| **Lakota** | `lakota` | 10 wakan powers |
| **Haudenosaunee** | `haudenosaunee` | 6 figures of the Six Nations creation account |
| **Anishinaabe** | `anishinaabe` | 5 manidoog + the 7 Grandfather Teachings |
| **Kunwinjku** | `aboriginal` | Ngalyod, of western Arnhem Land |
| **Kulin** | `aboriginal` | Bunjil and Waa, of central Victoria |
| **Gunaikurnai** | `aboriginal` | Tidilick, of Gippsland |

## Quick Start

```cyrius
# dist/avatara.cyr is the committed consumer bundle — include that, not src/.
# (src/lib.cyr resolves its own includes relative to the repo root, so it only
# builds from inside avatara itself.)
include "avatara/dist/avatara.cyr"

alloc_init();

# Single archetype
var tip = kabbalah_tiphareth();
# prof_warmth(tip) = 0.8, prof_confidence(tip) = 0.8

# Lookup by name — returns Result (Ok(profile) / Err(ERR_UNKNOWN_ARCHETYPE))
var kr = lookup("Krishna");
var krishna = result_unwrap(kr);   # or: if (is_err_result(kr) == 1) { ... }
# prof_tradition(krishna) = "Hindu"

# Query by trait
var brave = query_min_trait(PROF_COURAGE, 0.9);
# vec of all entities with courage >= 0.9

# Compose across traditions — also returns Result
var w = vec_new();
vec_push(w, weighted_new(tip, 1.0));
vec_push(w, weighted_new(krishna, 0.8));
var blended = result_unwrap(compose(w));

# Historical context
var ctx = context_for_tradition("Hindu");
# ctx_primary(ctx) = "Indus Valley", ctx_start(ctx) = -1500
```

## Design

All traditions map to the same `ArchetypeProfile` output:

- **TraitWeights** — 15 personality dimensions (0.0-1.0), maps 1:1 to bhava's `PersonalityProfile`
- **ModuleEmphasis** — which bhava modules this archetype amplifies (mood, energy, spirit, reasoning, etc.)
- **BreathAffinity** — position on the cosmic breath cycle (Unity through LateExhale through Unity)
- **GrowthDirection** — Differentiate, Integrate, Preserve, Transform, or Still
- **Element** — Fire, Water, Earth, Air, Aether, Light, Darkness, Storm, Mixed
- **Polarity** — Masculine, Feminine, Androgynous, Transcendent
- **CosmicTier** — Supreme, Primordial, Cosmic, Greater, Lesser, Demigod, Master

Archetypes are composable across traditions. A character can carry Kabbalistic Tiphareth + Hindu Vishnu + Greek Athena — reinforcing archetypes amplify, conflicting archetypes create productive internal tension.

## Affinity & Composition

```cyrius
# How similar are two archetypes? (0.0 to 1.0)
var score = affinity(norse_thor(), olympian_ares());

# Find Thor's closest match across all other traditions
var closest = cross_tradition_match(norse_thor());
# Returns the most similar archetype from a different tradition
# (note: `match` is a reserved keyword in Cyrius and cannot name a variable)

# Top 5 most similar archetypes to any entity
var top5 = similar_to(norse_thor(), 5);

# Detect conflicting traits (delta strictly > 0.4)
var conflicts = detect_conflicts(kabbalah_gevurah(), kabbalah_chesed());
# Returns 2, largest delta first: warmth (0.3 vs 0.9), patience (0.3 vs 0.8).
# Humor (0.2 vs 0.6) is exactly 0.4 and so does NOT qualify — the comparison
# is strict.
```

## Derived Layers

Two layers read *off* a profile rather than sitting beside it. Neither stores anything, neither adds
archetypes, and `profile_count()` is unchanged by either.

```cyrius
# Shadow — the inverted form. Traits become 1-v, breath/growth/polarity mirror,
# element and tier are kept. Involutive: shadow(shadow(p)) == p.
var dark_thor = shadow(norse_thor());
# prof_name(dark_thor) = "Shadow of Thor"
var yes = is_shadow_of(dark_thor, norse_thor());

# Overlays — cross-cutting typologies derived from the profile's own weights.
var etype = profile_enneagram_type(norse_thor());   # index; also _wing/_centre/_score
var role  = profile_jungian_role(norse_thor());     # Hero/Shadow/Anima/Self/Trickster

# Or walk every registered system without naming one — so a system added later
# is enumerable by code written today.
for (var s = 0; s < overlay_system_count(); s += 1) {
    var best = overlay_best(s, norse_thor());
    # overlay_system_name(s), overlay_label(s, best), overlay_score(s, p, i)
}
```

An overlay is **one reading, not the reading**. The Enneagram and Jung's set are 20th-century
frameworks; neither was in the room when the Dagda or Sedna were described. Several overlays may sit
over the same figure and disagree, and none of them displaces what the tradition modules say the
archetype *is*.

Traditions and typologies are therefore mutually exclusive, and a test enforces it: a typology may
only ever be an overlay — never a `tradition` string, never an archetype, never counted. The
Enneagram will not become a tradition with nine archetypes.

## Relationship to AGNOS

```
avatara (this) — divine archetype profiles (504 entities, 37 traditions)
  |
  +-> bhava — emotion/personality engine (archetype overlay)
  +-> joshua — NPC divine archetypes for games
  +-> kiran — game entities
  +-> agnosai — agent personalities with theological depth
  +-> hadara — archetype-to-culture context
```

avatara produces plain f64/enum outputs. It does not depend on bhava — bhava consumes avatara's output through a bridge module.

## Build

Requires the Cyrius compiler 6.5.36+ (pinned in `cyrius.cyml`).

```sh
# Resolve dependencies
cyrius deps

# Build and run the smoke test
cyrius build src/main.cyr build/avatara && ./build/avatara

# Run integration tests
cyrius test tests/avatara.tcyr

# Run benchmarks
cyrius build tests/avatara.bcyr build/bench && ./build/bench
```

## License

GPL-3.0-only
