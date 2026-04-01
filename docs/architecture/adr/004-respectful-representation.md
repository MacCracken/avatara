# ADR-004: Respectful Representation of Living Traditions

**Status**: Accepted
**Date**: 2026-03-31

## Context

Avatara maps beings from traditions that are actively practiced by billions of people. Hindu deities, Sikh Gurus, Buddhist Bodhisattvas, Yoruba Orishas, and many others are not merely "mythology" — they are central to living faiths. The library must treat them with the same scholarly care as any academic reference work.

## Decision

1. **Historically and theologically accurate** — use established correspondences from scholarly sources, not invented associations
2. **Respectful naming** — proper diacritics where possible (Shangó, Ōkuninushi, Dōgen), traditional titles (Guru Nanak, not just "Nanak")
3. **Contested claims documented** — e.g., Buddha as Vishnu's 9th avatar is marked as a Vaishnava claim, not presented as universal fact
4. **Debunked scholarship corrected** — e.g., Ixtab revised per modern ethnohistory rather than perpetuating colonial misreadings
5. **Archangel list sourced** — module doc acknowledges the composite/metaphysical tradition rather than claiming canonical status
6. **Secret/sacred traditions excluded** — Aboriginal Australian Dreamtime and broad "Native American" traditions are not codified, as many are explicitly not meant for outsider systematization
7. **Soul/spirit text is evocative, not doctrinal** — written to capture the archetype's essence without making theological claims

## Consequences

- Some traditions are absent not because they're unimportant but because codifying them would be disrespectful
- Doc comments serve as lightweight scholarly apparatus
- Pull requests adding traditions must include source citations
- The library is a psychometric mapping tool, not a religious text
