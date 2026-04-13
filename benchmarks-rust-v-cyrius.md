# Avatara: Rust v1.1.0 vs Cyrius v2.0.1

## Code Comparison

| Metric | Rust v1.1.0 | Cyrius v2.0.1 | Delta |
|--------|-------------|---------------|-------|
| Source files | 26 (.rs) | 28 (.cyr) | +2 |
| Source LOC | 18,804 | 15,644 | -17% |
| Test assertions | 188 | 195 | +4% |
| Benchmarks | 29 | 39 | +34% |
| External dependencies | 5 | 0 | -5 |
| Archetypes | ~280 | 329 | +49 |
| Traditions | 19 | 22 | +3 |
| Binary size | ~800KB-1.2MB | 874KB | comparable |
| Compile time | 5-60s | <100ms | 50-600x faster |

## Benchmark Comparison

Rust: Criterion v0.8, 10 samples, --release, LLVM -O2, 2026-04-01.
Cyrius: cc3 3.7.0, bench.cyr, 1000 iterations, no optimizer, 2026-04-12.

### Tier 1: Single Profile

| Benchmark | Rust | Cyrius | Notes |
|-----------|------|--------|-------|
| kabbalah/single_profile | 63 ns | 249 ns | Cyrius: heap alloc per call; Rust: stack struct |

### Tier 2: Per-Tradition (cached access)

| Benchmark | Rust | Cyrius | Speedup |
|-----------|------|--------|---------|
| kabbalah/all_10 | 739 ns | 8 ns | **92x** |
| angelic/all_archangels | 475 ns | 11 ns | **43x** |
| angelic/all_orders | 616 ns | 9 ns | **68x** |
| hindu/all_trimurti | 180 ns | 5 ns | **36x** |
| hindu/all_devas | 765 ns | 10 ns | **77x** |
| hindu/all_avatars | 752 ns | 9 ns | **84x** |
| olympian/all_15 | 1,039 ns | 15 ns | **69x** |
| norse/all_13 | 911 ns | 10 ns | **91x** |
| egyptian/all_16 | 1,165 ns | 15 ns | **78x** |
| celtic/all_15 | 1,080 ns | 18 ns | **60x** |
| shinto/all_15 | 1,075 ns | 10 ns | **108x** |
| aztec/all_14 | 969 ns | 16 ns | **61x** |
| maya/all_12 | 895 ns | 10 ns | **90x** |
| yoruba/all_14 | 1,011 ns | 11 ns | **92x** |
| zoroastrian/all_14 | 452 ns | 16 ns | **28x** |
| taoist/all_immortals | 505 ns | 9 ns | **56x** |
| taoist/all_deities | 521 ns | 9 ns | **58x** |
| incarnate/all_hindu | 953 ns | 11 ns | **87x** |
| incarnate/all_buddhist | 790 ns | 14 ns | **56x** |
| incarnate/all_mystic | 891 ns | 9 ns | **99x** |
| incarnate/all_taoist | 331 ns | 5 ns | **66x** |
| polynesian/all_12 | 863 ns | 12 ns | **72x** |
| slavic/all_12 | 869 ns | 10 ns | **87x** |
| jain/all_24 | 1,702 ns | 20 ns | **85x** |
| sikh/all_10 | 706 ns | 10 ns | **71x** |

**Average cached speedup: ~72x**

### Tier 3: Registry

| Benchmark | Rust | Cyrius | Notes |
|-----------|------|--------|-------|
| registry/all_profiles | 52,470 ns | 19 ns | **2,761x** (cached pointer return) |
| registry/lookup_by_name | 52,858 ns | 1 us | **53x** |
| registry/query_courage_0.9 | — | 4 us | — |
| registry/by_tradition | — | 13 us | — |

### Tier 4: Compose

| Benchmark | Rust | Cyrius |
|-----------|------|--------|
| compose/three_traditions | — | 2 us |

### Tier 5: History

| Benchmark | Rust | Cyrius |
|-----------|------|--------|
| history/context_for_tradition | — | 170 ns |
| history/context_all_traditions | — | 16 us |
| history/traditions_for_civ | — | 2 us |
| history/traditions_for_era | — | 3 us |
| history/traditions_active_at | — | 434 ns |
| history/query_civilization | — | 31 us |

## Why Cyrius Is Faster on Cached Access

Rust allocated fresh `ArchetypeProfile` structs with heap-allocated `String` fields on every call. Each `all_profiles()` call created ~280 profiles x 5 String fields = ~1,400 heap allocations.

Cyrius builds profiles once into a bump allocator, caches them in global vecs, and returns the cached pointer on subsequent calls. A cached `all_profiles()` is a single `load64` — 19ns.

## Why Rust Is Faster on Single Profile Creation

Rust's `Sephira::Tiphareth.profile()` builds a stack-allocated struct with LLVM-optimized field writes — 63ns. Cyrius `kabbalah_tiphareth()` calls `alloc()` (bump pointer advance) then 40+ `store64()` instructions — 249ns. No optimizer to batch or elide stores.

## Key Takeaway

For library consumers (bhava, joshua, kiran, hadara) who call `all_profiles()`, `lookup()`, and `by_tradition()` repeatedly, Cyrius is **50-2,700x faster** due to caching. The 4x slower single-profile creation is irrelevant — profiles are created once at init.
