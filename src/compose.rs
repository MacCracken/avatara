//! Archetype composition — blending multiple archetypes into one profile.
//!
//! The core promise of avatara: archetypes are composable across traditions.
//! A character can carry Kabbalistic + Hindu + Greek archetypes simultaneously,
//! with reinforcing dynamics (similar traits amplify) and conflicting dynamics
//! (opposing traits create productive tension at the midpoint).
//!
//! # Example
//!
//! ```
//! use avatara::Archetype;
//! use avatara::compose::compose;
//! use avatara::kabbalah::Sephira;
//! use avatara::hindu::Trimurti;
//! use avatara::olympian::Olympian;
//!
//! let profile = compose(&[
//!     (Sephira::Tiphareth.profile(), 1.0),
//!     (Trimurti::Vishnu.profile(), 0.8),
//!     (Olympian::Athena.profile(), 0.6),
//! ]).unwrap();
//!
//! // Blended name shows lineage
//! assert!(profile.name.contains("Tiphareth"));
//! assert!(profile.name.contains("Vishnu"));
//! assert!(profile.name.contains("Athena"));
//! ```

use crate::{
    ArchetypeProfile, BreathAffinity, CosmicTier, GrowthDirection, ModuleEmphasis, TraitWeights,
    error::AvataraError,
};

/// Compose multiple archetype profiles into a single blended profile.
///
/// Each entry is `(profile, weight)` where weight controls influence.
/// Weights are normalized internally — `(a, 1.0), (b, 1.0)` is 50/50,
/// `(a, 2.0), (b, 1.0)` is 2:1 in favor of `a`.
///
/// # Errors
///
/// Returns [`AvataraError::InvalidParameter`] if:
/// - The input slice is empty
/// - Any weight is negative
/// - Total weight is zero or negative
#[allow(clippy::missing_panics_doc)] // expect() on non-empty slice verified above
pub fn compose(weighted: &[(ArchetypeProfile, f64)]) -> Result<ArchetypeProfile, AvataraError> {
    if weighted.is_empty() {
        return Err(AvataraError::InvalidParameter(
            "cannot compose empty profile list".to_string(),
        ));
    }

    let total_weight: f64 = weighted.iter().map(|(_, w)| w).sum();
    if total_weight <= 0.0 {
        return Err(AvataraError::InvalidParameter(
            "total weight must be positive".to_string(),
        ));
    }

    for (p, w) in weighted {
        if *w < 0.0 {
            return Err(AvataraError::InvalidParameter(format!(
                "negative weight {} for profile '{}'",
                w, p.name
            )));
        }
    }

    let traits = blend_traits(weighted, total_weight);
    let emphasis = blend_emphasis(weighted, total_weight);
    let breath = blend_breath(weighted, total_weight);
    let growth = blend_growth(weighted);

    let name = weighted
        .iter()
        .filter(|(_, w)| *w > 0.0)
        .map(|(p, _)| p.name.as_str())
        .collect::<Vec<_>>()
        .join(" + ");

    let traditions: Vec<&str> = {
        let mut ts: Vec<&str> = weighted
            .iter()
            .filter(|(_, w)| *w > 0.0)
            .map(|(p, _)| p.tradition.as_str())
            .collect();
        ts.dedup();
        ts
    };
    let tradition = traditions.join(" + ");

    let description = format!(
        "Composite archetype blending {} tradition{}",
        traditions.len(),
        if traditions.len() == 1 { "" } else { "s" }
    );

    // Soul/spirit text from highest-weighted contributor.
    // Safety: we returned early above if `weighted` is empty, so this
    // iterator is guaranteed to yield at least one element.
    let dominant = weighted
        .iter()
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(p, _)| p)
        .expect("non-empty weighted slice (checked above)");

    // Element, polarity, tier from dominant contributor
    let element = dominant.element;
    let polarity = dominant.polarity;
    let tier = blend_tier(weighted);

    Ok(ArchetypeProfile {
        name,
        tradition,
        description,
        traits,
        emphasis,
        breath,
        growth,
        element,
        polarity,
        tier,
        soul_text: dominant.soul_text.clone(),
        spirit_text: dominant.spirit_text.clone(),
    })
}

fn blend_traits(weighted: &[(ArchetypeProfile, f64)], total: f64) -> TraitWeights {
    let mut t = TraitWeights {
        warmth: 0.0,
        humor: 0.0,
        empathy: 0.0,
        patience: 0.0,
        confidence: 0.0,
        curiosity: 0.0,
        creativity: 0.0,
        directness: 0.0,
        formality: 0.0,
        verbosity: 0.0,
        courage: 0.0,
        precision: 0.0,
        skepticism: 0.0,
        autonomy: 0.0,
        pedagogy: 0.0,
    };

    for (p, w) in weighted {
        let f = w / total;
        t.warmth += p.traits.warmth * f;
        t.humor += p.traits.humor * f;
        t.empathy += p.traits.empathy * f;
        t.patience += p.traits.patience * f;
        t.confidence += p.traits.confidence * f;
        t.curiosity += p.traits.curiosity * f;
        t.creativity += p.traits.creativity * f;
        t.directness += p.traits.directness * f;
        t.formality += p.traits.formality * f;
        t.verbosity += p.traits.verbosity * f;
        t.courage += p.traits.courage * f;
        t.precision += p.traits.precision * f;
        t.skepticism += p.traits.skepticism * f;
        t.autonomy += p.traits.autonomy * f;
        t.pedagogy += p.traits.pedagogy * f;
    }

    t
}

fn blend_emphasis(weighted: &[(ArchetypeProfile, f64)], total: f64) -> ModuleEmphasis {
    let mut e = ModuleEmphasis {
        mood: 0.0,
        energy: 0.0,
        stress: 0.0,
        growth: 0.0,
        spirit: 0.0,
        reasoning: 0.0,
        regulation: 0.0,
        relationship: 0.0,
        flow: 0.0,
        belief: 0.0,
        intuition: 0.0,
        salience: 0.0,
        appraisal: 0.0,
        eq: 0.0,
    };

    for (p, w) in weighted {
        let f = w / total;
        e.mood += p.emphasis.mood * f;
        e.energy += p.emphasis.energy * f;
        e.stress += p.emphasis.stress * f;
        e.growth += p.emphasis.growth * f;
        e.spirit += p.emphasis.spirit * f;
        e.reasoning += p.emphasis.reasoning * f;
        e.regulation += p.emphasis.regulation * f;
        e.relationship += p.emphasis.relationship * f;
        e.flow += p.emphasis.flow * f;
        e.belief += p.emphasis.belief * f;
        e.intuition += p.emphasis.intuition * f;
        e.salience += p.emphasis.salience * f;
        e.appraisal += p.emphasis.appraisal * f;
        e.eq += p.emphasis.eq * f;
    }

    e
}

/// Blend breath affinities by weighted-averaging their intensities,
/// then snapping to the nearest `BreathAffinity` variant.
fn blend_breath(weighted: &[(ArchetypeProfile, f64)], total: f64) -> BreathAffinity {
    let avg_intensity: f64 = weighted
        .iter()
        .map(|(p, w)| p.breath.intensity() * (w / total))
        .sum();

    nearest_breath(avg_intensity)
}

fn nearest_breath(intensity: f64) -> BreathAffinity {
    let candidates = [
        BreathAffinity::Unity,
        BreathAffinity::EarlyExhale,
        BreathAffinity::MidExhale,
        BreathAffinity::LateExhale,
        BreathAffinity::EarlyInhale,
        BreathAffinity::MidInhale,
        BreathAffinity::LateInhale,
    ];

    candidates
        .into_iter()
        .min_by(|a, b| {
            let da = (a.intensity() - intensity).abs();
            let db = (b.intensity() - intensity).abs();
            da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
        })
        .unwrap_or(BreathAffinity::LateExhale)
}

/// Resolve growth direction by picking the most common among
/// weighted contributors. Ties go to the highest-weighted instance.
fn blend_growth(weighted: &[(ArchetypeProfile, f64)]) -> GrowthDirection {
    use std::collections::HashMap;
    let mut scores: HashMap<GrowthDirection, f64> = HashMap::new();

    for (p, w) in weighted {
        *scores.entry(p.growth).or_default() += w;
    }

    scores
        .into_iter()
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(g, _)| g)
        .unwrap_or_default()
}

/// Resolve cosmic tier by picking the highest tier among contributors
/// (composing a Supreme + Greater = Supreme).
fn blend_tier(weighted: &[(ArchetypeProfile, f64)]) -> CosmicTier {
    use std::collections::HashMap;
    let mut scores: HashMap<CosmicTier, f64> = HashMap::new();

    for (p, w) in weighted {
        *scores.entry(p.tier).or_default() += w;
    }

    scores
        .into_iter()
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(t, _)| t)
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Archetype;
    use crate::kabbalah::Sephira;
    use crate::hindu::Trimurti;
    use crate::olympian::Olympian;

    #[test]
    fn compose_single_profile_is_identity() {
        let original = Sephira::Tiphareth.profile();
        let composed = compose(&[(original.clone(), 1.0)]).unwrap();
        assert_eq!(composed.name, "Tiphareth");
        assert!((composed.traits.warmth - original.traits.warmth).abs() < 1e-10);
        assert!((composed.traits.courage - original.traits.courage).abs() < 1e-10);
        assert_eq!(composed.breath, original.breath);
        assert_eq!(composed.growth, original.growth);
    }

    #[test]
    fn compose_equal_weights_averages() {
        let a = Sephira::Gevurah.profile(); // directness 0.9
        let b = Sephira::Chesed.profile(); // directness 0.5 (default)
        let composed = compose(&[(a, 1.0), (b, 1.0)]).unwrap();
        assert!((composed.traits.directness - 0.7).abs() < 0.01);
    }

    #[test]
    fn compose_cross_tradition() {
        let result = compose(&[
            (Sephira::Tiphareth.profile(), 1.0),
            (Trimurti::Vishnu.profile(), 0.8),
            (Olympian::Athena.profile(), 0.6),
        ])
        .unwrap();
        assert!(result.name.contains("Tiphareth"));
        assert!(result.name.contains("Vishnu"));
        assert!(result.name.contains("Athena"));
        assert!(result.tradition.contains("Kabbalah"));
        assert!(result.tradition.contains("Hindu"));
        assert!(result.tradition.contains("Greek"));
    }

    #[test]
    fn compose_empty_is_error() {
        let result = compose(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn compose_negative_weight_is_error() {
        let p = Sephira::Kether.profile();
        let result = compose(&[(p, -1.0)]);
        assert!(result.is_err());
    }

    #[test]
    fn compose_zero_weight_excluded_from_name() {
        let a = Sephira::Kether.profile();
        let b = Sephira::Malkuth.profile();
        let composed = compose(&[(a, 1.0), (b, 0.0)]).unwrap();
        assert!(composed.name.contains("Kether"));
        assert!(!composed.name.contains("Malkuth"));
    }

    #[test]
    fn compose_breath_blends_by_intensity() {
        // Unity (0.0) + LateExhale (1.0) at equal weights → ~MidExhale (0.5)
        let unity = Sephira::Kether.profile();
        let manifest = Sephira::Malkuth.profile();
        let composed = compose(&[(unity, 1.0), (manifest, 1.0)]).unwrap();
        assert_eq!(composed.breath, BreathAffinity::MidExhale);
    }

    #[test]
    fn compose_growth_picks_dominant() {
        // Two Preserve + one Transform → Preserve wins
        let vishnu = Trimurti::Vishnu.profile(); // Preserve
        let tiphareth = Sephira::Tiphareth.profile(); // Preserve
        let shiva = Trimurti::Shiva.profile(); // Transform
        let composed = compose(&[
            (vishnu, 1.0),
            (tiphareth, 1.0),
            (shiva, 1.0),
        ])
        .unwrap();
        assert_eq!(composed.growth, GrowthDirection::Preserve);
    }

    #[test]
    fn compose_traits_stay_in_range() {
        let profiles: Vec<_> = Sephira::ALL
            .iter()
            .map(|s| (s.profile(), 1.0))
            .collect();
        let composed = compose(&profiles).unwrap();
        assert!(composed.traits.warmth >= 0.0 && composed.traits.warmth <= 1.0);
        assert!(composed.traits.courage >= 0.0 && composed.traits.courage <= 1.0);
    }
}
