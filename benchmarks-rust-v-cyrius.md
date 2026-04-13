# Benchmarks: Rust v1.0.0 vs Cyrius v2.0.0

Rust benchmarks from Criterion v0.8, 10 samples, 2026-04-01.
Cyrius benchmarks pending first run on cc3 3.7.0.

## Rust v1.0.0 Baseline (Criterion)

### Tier 1: Single Profile

| Benchmark | Rust Median | Cyrius Median |
|-----------|-------------|---------------|
| kabbalah/single_profile | 62.943 ns | TBD |
| registry/lookup_by_name | 52.858 us | TBD |

### Tier 2: Per-Tradition Groups

| Benchmark | Rust Median | Cyrius Median |
|-----------|-------------|---------------|
| kabbalah/all_10 | 738.50 ns | TBD |
| angelic/all_archangels | 475.15 ns | TBD |
| angelic/all_orders | 616.28 ns | TBD |
| hindu/all_trimurti | 179.62 ns | TBD |
| hindu/all_devas | 765.06 ns | TBD |
| hindu/all_avatars | 751.58 ns | TBD |
| olympian/all_15 | 1.0389 us | TBD |
| norse/all_13 | 910.57 ns | TBD |
| egyptian/all_16 | 1.1647 us | TBD |
| celtic/all_15 | 1.0801 us | TBD |
| shinto/all_15 | 1.0747 us | TBD |
| aztec/all_14 | 969.44 ns | TBD |
| maya/all_12 | 895.02 ns | TBD |
| yoruba/all_14 | 1.0109 us | TBD |
| zoroastrian/all_beings | 452.24 ns | TBD |
| taoist/all_immortals | 504.65 ns | TBD |
| taoist/all_deities | 520.96 ns | TBD |
| incarnate/all_hindu | 953.10 ns | TBD |
| incarnate/all_buddhist | 790.46 ns | TBD |
| incarnate/all_mystic | 890.76 ns | TBD |
| incarnate/all_taoist | 331.40 ns | TBD |
| polynesian/all_12 | 863.25 ns | TBD |
| slavic/all_12 | 869.14 ns | TBD |
| jain/all_24 | 1.7018 us | TBD |
| sikh/all_10 | 705.91 ns | TBD |

### Tier 3: Cross-Tradition

| Benchmark | Rust Median | Cyrius Median |
|-----------|-------------|---------------|
| registry/all_profiles | 52.470 us | TBD |

## Notes

- Rust: compiled with `--release`, Criterion v0.8, LLVM optimizations
- Cyrius: cc3 3.7.0, static ELF, no optimizer, direct x86_64 codegen
- Cyrius uses lazy-init caching (`all_*()` builds once, returns cached vec on subsequent calls) which means first-call is allocation-heavy but subsequent calls are pointer returns
- Rust allocated fresh `ArchetypeProfile` structs per call (String fields = heap alloc per profile)
- Expect Cyrius to be faster on cached lookups (pointer return vs heap alloc), potentially slower on first-build (sequential `alloc()` + `store64()` calls)
