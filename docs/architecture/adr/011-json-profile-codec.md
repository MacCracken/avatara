# ADR-011: ArchetypeProfile JSON in the Rust 1.0.1 serde shape, in its own bundle

**Status**: Accepted
**Date**: 2026-11-19 (released in 2.17.0, 2026-11-20)

> Partly supersedes [ADR-006](006-cyrius-port.md)'s "No serde serialization (Cyrius has no equivalent
> yet)". avatara now has a serializer for one record, the profile, and nothing else.

## Context

The Rust crate derived `Serialize` / `Deserialize` on `ArchetypeProfile`, `TraitWeights`,
`ModuleEmphasis` and the five classification enums, with no `#[serde(...)]` attribute anywhere
(avatara 1.0.1, commit `5b5d93c`, `src/lib.rs`). The Cyrius port carried no serializer of any kind.

The first consumer that needs one is sankhya. Its `correlate()` returns a `MultiCalendarDate` carrying
the seven Saptarishi as `(name, profile)` pairs from `incarnate_sage_*`. The Rust sankhya wrote each as
a `[name, ArchetypeProfile]` tuple through serde. With no avatara serializer, sankhya writes only
`{name, tradition, description}` per sage. Its 3.2.0 decoders return serde-style Results, but they
cannot rebuild a profile, so they only count them.

Two constraints shape the answer:

- **"Zero external dependencies except sakshi."** JSON needs bayan, both to parse and to format f64.
  Making every consumer of `dist/avatara.cyr` take bayan would break the rule for consumers that never
  touch JSON (bhava, joshua, kiran, agnosai, hadara).
- **varna 2.5.2 solved the same problem** (varna ADR 0003): the JSON code lives in `src/json.cyr`,
  bundled only into a separate `dist/varna-json.cyr` through `cyrius.cyml` `[lib.json]`.

## Decision

### 1. The wire format is the Rust 1.0.1 serde shape, plus `domain` last

```
{"name":"…","tradition":"…","description":"…",
 "traits":{"warmth":…,"humor":…, … ,"pedagogy":…},      15 f64, declaration order
 "emphasis":{"mood":…,"energy":…, … ,"eq":…},           14 f64, declaration order
 "breath":"EarlyExhale","growth":"Differentiate","element":"Aether",
 "polarity":"Masculine","tier":"Master",
 "soul_text":"…","spirit_text":"…",
 "domain":"Wisdom"}
```

- **Enums are written by their Rust variant names**: BreathAffinity `Unity` … `LateInhale`,
  GrowthDirection `Differentiate` … `Still`, Element `Fire` … `Mixed`, Polarity `Masculine` …
  `Transcendent`, CosmicTier `Supreme` … `Master`. The Cyrius values equal the Rust declaration
  order, so value *i* is the *i*-th name.
- **`domain` is the one Cyrius-only field.** Domain arrived in 2.5.2 and the Rust never had it. It
  is written last, and it is optional on read: when it is absent, the profile keeps `profile_new()`'s
  `DOMAIN_UNSPECIFIED`, so a Rust 1.0.1 document still decodes. Its 20 names are avatara's own. Each is
  the constant's suffix in the UpperCamelCase a Rust variant would take, in value order: `Unspecified`,
  `Creation`, `War`, `Love`, `Death`, `Wisdom`, `Order`, `Chaos`, `Nature`, `Sky`, `Sea`, `Fire`, `Sun`,
  `Moon`, `Fate`, `Trickery`, `Healing`, `Prosperity`, `Hearth`, `Transcendence`.
- **f64 is written with bayan's Grisu2 formatter** (`bayan_f64_to_json`), whose output reads back to
  the same bits. Non-finite values are written as `null`, as serde_json writes them.
- **Strings** go through the stdlib's JSON escaper (`str_builder_add_json_cstr`). Its rules are
  serde_json's: `\"`, `\\`, `\b \f \n \r \t`, other control bytes as lowercase `\u00xx`, and every
  other byte verbatim.

### 2. API (`src/json.cyr`)

| Function | Returns |
|----------|---------|
| `profile_to_json_into(sb, p)` | Appends one object to str_builder `sb` and returns `0`. Returns `-1` with **nothing written** if `sb` or `p` is NULL, or if an enum field holds a value its enum does not name. |
| `profile_to_json(p)` | The same object as a fresh `Str`, or `0` wherever `_into` would return `-1`. |
| `profile_from_json(node)` | Decodes a bayan value-tree node (`bayan_json_v_*`), so a consumer can decode inside its own parsed document. Returns `Ok(profile)` (a fresh `profile_new()` populated through `Profile_set_*`) or `Err(msg)`. |
| `profile_from_json_text(buf, len)` | Parses `len` bytes of JSON text, then decodes as above. Same Result shape. |

`profile_eq(a, b)` lives in `src/types.cyr`, so both bundles carry it. It compares every one of the
40 fields: strings by content, enums by value, and f64 **by bit pattern**, so `-0.0 ≠ 0.0` and a NaN
equals the identical NaN. That is the equality a lossless round trip needs. A NULL profile equals
nothing, itself included, which is the `safe_streq` rule for "no value". Two NULL *string fields* are
equal and NULL is not equal to `""`: profile_eq asks whether two profiles hold the same bytes, not
whether two names denote one archetype.

### 3. The reader is as strict as serde's derive, in serde's words

The Result is cyrius 6.6's value form. **The Err payload is a `Str` (lib/str.cyr), never 0**, holding
the message. Every decode error is a deserialization error, so there is no error code beside it. A
consumer with an error record of its own (sankhya's `SkErr{code, msg}`) wraps the Str as it stands.

| Input | Message |
|-------|---------|
| a non-object where a struct belongs | ``invalid type: integer `5`, expected struct ArchetypeProfile`` (also `struct TraitWeights`, `struct ModuleEmphasis`) |
| an absent required field | ``missing field `traits` `` |
| a field given twice (in a hand-built tree) | ``duplicate field `name` `` |
| a wrong JSON type | `invalid type: string "0.5", expected f64`; `… expected a string` |
| an unknown enum name | ``unknown variant `Foo`, expected one of `Unity`, `EarlyExhale`, …`` |
| an enum value that is not a string | `invalid type: integer `3`, expected variant identifier` |
| a number that overflows f64 | `number out of range` |
| a string holding U+0000 | `invalid value: string "a\u0000b", expected a string without U+0000` |
| a string that is not UTF-8 | `invalid unicode code point` |

The unexpected value is described the way serde describes it: `null`, ``boolean `true` ``,
``integer `5` ``, ``floating point `1.5` ``, `string "x"`, `sequence`, `map`. Fields decode in document
order and the first error wins, as serde's visitor does. Missing fields are reported after the walk,
in declaration order. Unknown keys are ignored. Values are not range-checked, because serde's derive
does not check them; `validate_profile()` does that.

serde_json appends ` at line L column C` to these messages. bayan's value tree keeps no positions, so
`profile_from_json` stops before it, as sankhya's decoders do. `profile_from_json_text` sees the text,
so it adds a position to its own text errors. Line counts from 1 and the column is the 1-based byte
column of the offending byte, which matches serde_json on the cases checked. Those text errors are
bytes that are not UTF-8 (`invalid unicode code point`) and a raw control byte inside a string
(`control character (\u0000-\u001F) found while parsing a string`), both in serde_json's words. Any
other syntax error is reported in bayan's words.

### 4. Cyrius-only decisions

- **NULL strings round-trip as `null`.** `profile_new()` leaves all five strings NULL, and
  `validate_profile()` accepts that by design. The writer emits `null` for a NULL field, the way the
  stdlib's `str_builder_add_json_cstr` and `#derive(Serialize)` write a NULL `: cstring`, and `null`
  reads back as NULL. So every profile round-trips, including an unpopulated one. The Rust never wrote
  null there, and serde would refuse it (`invalid type: null, expected a string`). Writing `""` was
  rejected because it is lossy: NULL and `""` are different profiles under `profile_eq`.
- **U+0000 in a string is refused.** A C string would stop there and hand back a different value.
  serde accepts it. A *key* holding U+0000 is simply not a known key, so it is ignored.
- **Strings must be UTF-8.** bayan's parser does not check it, and a hand-built tree can hold any
  bytes. Refusing them means a decoded profile always re-encodes to valid JSON.
- **`-0` reads as `+0.0`.** bayan's tree stores the JSON integer `-0` as 0, while serde_json reads
  `-0.0`. avatara itself writes negative zero as `-0.0`, which round-trips exactly.
- **Two forms nobody writes are refused**, as sankhya's decoders refuse them. A struct given as an
  array of its fields, which serde's derive accepts, is `invalid type: sequence, expected struct …`. A
  unit variant in map form (`{"Unity":null}`) is `invalid type: map, expected variant identifier`.
  serde_json's `from_str` says only `expected value` for a non-string enum. avatara keeps sankhya's
  `expected variant identifier`, which names the type.

### 5. Packaging: `dist/avatara-json.cyr`, and `dist/avatara.cyr` stays bayan-free

- `src/json.cyr` is listed only in `cyrius.cyml` `[lib.json]` (the `[lib]` modules plus json.cyr, with
  lib.cyr kept last). `cyrius distlib --all` writes `dist/avatara-json.cyr`, a superset of
  `dist/avatara.cyr`, so a consumer folds one or the other. `src/lib.cyr` does not include json.cyr.
  CI fails if `dist/avatara.cyr` ever names a `bayan_` symbol.
- **The consumer declares bayan**, in one of two forms and never both. Both define the same JSON
  symbols, and the last definition wins.
  - The `bayan` stdlib monolith in `[deps].stdlib`, as sankhya has.
  - bayan's thin `dist/bayan-json.cyr` profile through `[deps.bayan]`, at tag 1.11.1 for cyrius 6.7.18.

  With neither, the build refuses and names the `bayan_json_*` calls. `dist/avatara-json.deps` lists
  neither form, since the choice is the consumer's.
- avatara's own builds take the thin profile through an **optional `[deps.bayan]` with
  `[features] default = ["bayan"]`**, varna's arrangement. Features resolve from the root manifest
  only, so consumers never inherit it.
- **distlib must run with `--no-default-features`.** With bayan in scope, distlib's compile-verify
  finds bayan's leaves already present and under-reports both sidecars: `dist/avatara.deps` shrinks
  from `syscalls sakshi alloc string result vec` to `sakshi`. CI, the release job and
  `scripts/version-bump.sh` all run `cyrius distlib --all --no-default-features`. The local check is
  `cyrius distlib --check --no-default-features`.
- Private names carry `_avj_` / `AVJ_`, not varna's `_json_`, so a consumer can fold both
  `dist/varna-json.cyr` and `dist/avatara-json.cyr`.

## Verification against serde

The Rust 1.0.1 types were compiled in a scratch crate (`git show 5b5d93c:src/lib.rs`, serde 1.0 and
serde_json 1.0.154, outside the repository), and the same documents went through both libraries:

- **serde_json decodes all 503 registered profiles' JSON and re-encodes them byte-identical** to
  avatara's output, less `"domain"`. `tests/json.tcyr` pins the exact text of a profile that
  exercises escapes, `-0.0`, `1.0`, `1/3` and `0.1 + 0.2`, and serde writes that text byte for byte.
- Of 74 malformed and edge documents, 54 match serde_json exactly apart from the position suffix:
  20 byte-identical, and 34 differing only in that suffix. Every missing-field, wrong-type,
  unknown-variant and overflow message in §3 is among them. Of the other 20, 9 are §4's deliberate
  choices and 3 are the number text noted below. The last 8 come from bayan's parser, which runs
  before the decoder:
  - **Duplicate keys.** bayan refuses one at parse time (`duplicate key at line 1 column 18`), where
    serde says ``duplicate field `name` ``. The decoder's own duplicate check covers trees built by
    hand with `bayan_json_v_obj_set`.
  - **Large integers.** An integer beyond i64 is a bayan parse error. serde reads it as f64.
  - **Syntax errors** are worded the way bayan words them.
- **Number text can differ for values no profile holds.** ryu writes `1e-6` and `1e+20` where Grisu2
  writes `0.000001` and `100000000000000000000.0`. The bits are the same either way, and avatara's own
  re-encoding is always byte-identical.

## Consequences

### Positive

- sankhya can carry full profiles again and decode them inside its own document. A Rust-era document
  decodes too.
- Every registered profile round-trips exactly: `profile_eq` on the copy, and byte-identical
  re-encoding.
- `dist/avatara.cyr` and its consumers are unchanged except for `profile_eq`, and the sakshi-only rule
  holds for every consumer that does not ask for JSON.

### Negative

- **A new Profile field must be added to `src/json.cyr`**: its key table, `profile_to_json_into` and
  `_avj_read_top`. A new enum *variant* is a compile error until it has a name, because each name
  `match` is exhaustive. A new *field* is not caught that way, so the round-trip test's re-encoding
  and `profile_eq` are what catch it.
- **The variant names are a wire contract.** Renaming a Rust name, or one of the Domain names, breaks
  stored JSON. The unknown-variant messages in `tests/json.tcyr` pin every list.
- `[lib.json]` repeats the `[lib]` module list, which is one more place to update when a module is
  added. distlib does not refuse a module left out of it. It only adds the missing module's calls
  (`all_<tradition>`) to the undefined-name warning that already lists bayan's 19, so CI fails on any
  undefined name in that warning that is not a `bayan_` one.
- Cost (`tests/avatara.bcyr` tier 8, one Saptarishi profile): encoding takes about 43 µs. bayan's
  Grisu2 accounts for about 750 ns per f64, or roughly 22 µs for the 29 weights, and the stdlib's
  byte-at-a-time escaper for about 12 µs. Decoding a parsed node takes about 6 µs, and parse plus
  decode about 41 µs. Both hot spots are the canonical stdlib and bayan routines, kept on purpose
  rather than duplicated.

### Neutral

- `#derive(Serialize)` is unused. On an enum it would write the constant names
  (`"BREATH_EARLY_EXHALE"`), not the Rust names, and varna established that a struct derive cannot
  write an enum field by name.

## Alternatives considered

- **JSON in `dist/avatara.cyr`.** Rejected: it breaks the sakshi-only rule for five consumers that do
  not use JSON.
- **Enums as numbers.** Rejected: not the Rust shape, unreadable without avatara's source, and an
  out-of-range number would decode silently.
- **A text-only reader.** Rejected as the primary form: sankhya decodes profiles from inside its own
  parsed document. The text form is kept as a convenience.
- **An `AvataraError` code as the Err payload.** Rejected: it loses the message that serde's wording
  exists to carry. An `{code, msg}` record would add API for an error that has only one kind.
