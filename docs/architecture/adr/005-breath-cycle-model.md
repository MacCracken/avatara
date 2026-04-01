# ADR-005: Seven-Phase Cosmic Breath Model

**Status**: Accepted
**Date**: 2026-03-31

## Context

Avatara needs a framework for positioning archetypes on a spectrum from undifferentiated unity to full material manifestation and back. This maps to bhava's consciousness model (Scale 7: cosmic breath phase).

Multiple models were considered:
1. Binary (manifest/non-manifest)
2. Three-phase (creation/preservation/destruction — maps to Trimurti but too coarse)
3. Five-phase (Chinese Wu Xing)
4. Seven-phase breath cycle

## Decision

Seven phases modeled as exhalation (unity → form) and inhalation (form → unity):

| Phase | Intensity | Description |
|-------|-----------|-------------|
| Unity | 0.0 | Source/return — undifferentiated |
| EarlyExhale | 0.15 | Consciousness beginning to differentiate |
| MidExhale | 0.50 | Active individuation |
| LateExhale | 1.00 | Maximum manifestation (most entities) |
| EarlyInhale | 0.80 | Form beginning to soften |
| MidInhale | 0.40 | Active dissolution |
| LateInhale | 0.10 | Approaching equanimity |

`BreathAffinity::intensity()` returns the numeric value, enabling composition via weighted averaging (see `compose::blend_breath`).

## Consequences

- Most deities cluster at LateExhale (fully manifest) — this is correct but reduces discriminatory power among manifest gods
- Supreme beings (Olodumare, Ahura Mazda) map to Unity
- Primordial beings (Tiamat, Papatuanuku) map to EarlyExhale
- Dying/returning gods (Baldur, Dumuzid, Osiris) map to EarlyInhale
- Buddhist enlightened beings map to LateInhale (approaching dissolution)
- The model is asymmetric: exhale intensity increases linearly, inhale decreases non-linearly — this is intentional, reflecting the idea that creation is gradual but manifestation peaks sharply
