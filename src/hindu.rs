//! Hindu divine archetypes — Trimurti, Devas, Avatars, Shakti.
//!
//! The Trimurti maps directly to the cosmic breath: Brahma (exhale/creation),
//! Vishnu (preservation/form), Shiva (inhale/dissolution). Devas govern
//! specific domains. Avatars are manifestations of the divine in specific forms.

use serde::{Deserialize, Serialize};

use crate::{Archetype, ArchetypeProfile, BreathAffinity, GrowthDirection, ModuleEmphasis, TraitWeights};

/// The Trimurti — three aspects of the supreme.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Trimurti {
    /// Brahma — creator, exhale of consciousness.
    Brahma,
    /// Vishnu — preserver, sustainer of form.
    Vishnu,
    /// Shiva — transformer/destroyer, inhale back to source.
    Shiva,
}

/// Principal Devas — divine beings governing cosmic functions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Deva {
    /// Indra — king of devas, thunder, energy, courage.
    Indra,
    /// Saraswati — knowledge, arts, music, learning.
    Saraswati,
    /// Lakshmi — abundance, beauty, grace, prosperity.
    Lakshmi,
    /// Ganesha — remover of obstacles, wisdom, new beginnings.
    Ganesha,
    /// Hanuman — devotion, strength, selfless service.
    Hanuman,
    /// Kali — time, transformation, fierce compassion.
    Kali,
    /// Durga — protection, strength, the invincible.
    Durga,
}

/// The 10 Avatars of Vishnu (Dashavatara).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Avatar {
    Matsya,
    Kurma,
    Varaha,
    Narasimha,
    Vamana,
    Parashurama,
    Rama,
    Krishna,
    Buddha,
    Kalki,
}

impl Trimurti {
    pub const ALL: &'static [Self] = &[Self::Brahma, Self::Vishnu, Self::Shiva];
}

impl Deva {
    pub const ALL: &'static [Self] = &[
        Self::Indra, Self::Saraswati, Self::Lakshmi, Self::Ganesha,
        Self::Hanuman, Self::Kali, Self::Durga,
    ];
}

impl Avatar {
    pub const ALL: &'static [Self] = &[
        Self::Matsya, Self::Kurma, Self::Varaha, Self::Narasimha,
        Self::Vamana, Self::Parashurama, Self::Rama, Self::Krishna,
        Self::Buddha, Self::Kalki,
    ];
}

impl Archetype for Trimurti {
    fn name(&self) -> &'static str {
        match self {
            Self::Brahma => "Brahma",
            Self::Vishnu => "Vishnu",
            Self::Shiva => "Shiva",
        }
    }

    fn tradition(&self) -> &'static str { "Hindu" }

    fn profile(&self) -> ArchetypeProfile {
        // TODO: Full profiles
        let (breath, growth) = match self {
            Self::Brahma => (BreathAffinity::EarlyExhale, GrowthDirection::Differentiate),
            Self::Vishnu => (BreathAffinity::LateExhale, GrowthDirection::Preserve),
            Self::Shiva => (BreathAffinity::MidInhale, GrowthDirection::Transform),
        };
        ArchetypeProfile {
            name: self.name().to_string(),
            tradition: self.tradition().to_string(),
            description: format!("{} of the Trimurti", self.name()),
            traits: TraitWeights::default(),
            emphasis: ModuleEmphasis::default(),
            breath,
            growth,
            soul_text: String::new(),
            spirit_text: String::new(),
        }
    }
}

impl Archetype for Deva {
    fn name(&self) -> &'static str {
        match self {
            Self::Indra => "Indra",
            Self::Saraswati => "Saraswati",
            Self::Lakshmi => "Lakshmi",
            Self::Ganesha => "Ganesha",
            Self::Hanuman => "Hanuman",
            Self::Kali => "Kali",
            Self::Durga => "Durga",
        }
    }

    fn tradition(&self) -> &'static str { "Hindu" }

    fn profile(&self) -> ArchetypeProfile {
        // TODO: Full profiles
        ArchetypeProfile {
            name: self.name().to_string(),
            tradition: self.tradition().to_string(),
            description: format!("Deva: {}", self.name()),
            traits: TraitWeights::default(),
            emphasis: ModuleEmphasis::default(),
            breath: BreathAffinity::LateExhale,
            growth: GrowthDirection::Differentiate,
            soul_text: String::new(),
            spirit_text: String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trimurti_breath_mapping() {
        assert_eq!(Trimurti::Brahma.profile().breath, BreathAffinity::EarlyExhale);
        assert_eq!(Trimurti::Vishnu.profile().breath, BreathAffinity::LateExhale);
        assert_eq!(Trimurti::Shiva.profile().breath, BreathAffinity::MidInhale);
    }

    #[test]
    fn all_devas_produce_profiles() {
        for d in Deva::ALL {
            let p = d.profile();
            assert!(!p.name.is_empty());
            assert_eq!(p.tradition, "Hindu");
        }
    }
}
