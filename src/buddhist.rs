//! Buddhist archetypes — Bodhisattvas, Dhyani Buddhas, Dharma protectors.

use crate::{
    Archetype, ArchetypeProfile, BreathAffinity, GrowthDirection, ModuleEmphasis, TraitWeights,
};
use serde::{Deserialize, Serialize};

/// Principal Bodhisattvas.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Bodhisattva {
    /// Avalokiteshvara — compassion, empathy, hearing the cries of the world.
    Avalokiteshvara,
    /// Manjushri — wisdom, discernment, cutting through delusion.
    Manjushri,
    /// Vajrapani — power, protection, fierce compassion.
    Vajrapani,
    /// Kshitigarbha — patience, vow to empty all hells.
    Kshitigarbha,
    /// Samantabhadra — practice, universal virtue, action.
    Samantabhadra,
    /// Tara — swift protection, courage, compassionate action.
    Tara,
    /// Maitreya — future Buddha, hope, patience, loving-kindness.
    Maitreya,
}

/// The 5 Dhyani (Meditation) Buddhas — wisdom aspects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum DhyaniBuddha {
    /// Vairochana — all-encompassing wisdom, center.
    Vairochana,
    /// Akshobhya — mirror-like wisdom, east.
    Akshobhya,
    /// Ratnasambhava — equalizing wisdom, south.
    Ratnasambhava,
    /// Amitabha — discriminating wisdom, west.
    Amitabha,
    /// Amoghasiddhi — all-accomplishing wisdom, north.
    Amoghasiddhi,
}

impl Bodhisattva {
    pub const ALL: &'static [Self] = &[
        Self::Avalokiteshvara,
        Self::Manjushri,
        Self::Vajrapani,
        Self::Kshitigarbha,
        Self::Samantabhadra,
        Self::Tara,
        Self::Maitreya,
    ];
}

impl DhyaniBuddha {
    pub const ALL: &'static [Self] = &[
        Self::Vairochana,
        Self::Akshobhya,
        Self::Ratnasambhava,
        Self::Amitabha,
        Self::Amoghasiddhi,
    ];
}

impl Archetype for Bodhisattva {
    fn name(&self) -> &'static str {
        match self {
            Self::Avalokiteshvara => "Avalokiteshvara",
            Self::Manjushri => "Manjushri",
            Self::Vajrapani => "Vajrapani",
            Self::Kshitigarbha => "Kshitigarbha",
            Self::Samantabhadra => "Samantabhadra",
            Self::Tara => "Tara",
            Self::Maitreya => "Maitreya",
        }
    }

    fn tradition(&self) -> &'static str {
        "Buddhist"
    }

    fn profile(&self) -> ArchetypeProfile {
        // TODO: Full profiles — all Bodhisattvas trend toward integration
        ArchetypeProfile {
            name: self.name().to_string(),
            tradition: self.tradition().to_string(),
            description: format!("Bodhisattva: {}", self.name()),
            traits: TraitWeights::default(),
            emphasis: ModuleEmphasis::default(),
            breath: BreathAffinity::EarlyInhale,
            growth: GrowthDirection::Integrate,
            soul_text: String::new(),
            spirit_text: String::new(),
        }
    }
}

impl Archetype for DhyaniBuddha {
    fn name(&self) -> &'static str {
        match self {
            Self::Vairochana => "Vairochana",
            Self::Akshobhya => "Akshobhya",
            Self::Ratnasambhava => "Ratnasambhava",
            Self::Amitabha => "Amitabha",
            Self::Amoghasiddhi => "Amoghasiddhi",
        }
    }

    fn tradition(&self) -> &'static str {
        "Buddhist"
    }

    fn profile(&self) -> ArchetypeProfile {
        // TODO: Full profiles — Dhyani Buddhas are wisdom aspects
        ArchetypeProfile {
            name: self.name().to_string(),
            tradition: self.tradition().to_string(),
            description: format!("Dhyani Buddha: {}", self.name()),
            traits: TraitWeights::default(),
            emphasis: ModuleEmphasis::default(),
            breath: BreathAffinity::LateInhale,
            growth: GrowthDirection::Integrate,
            soul_text: String::new(),
            spirit_text: String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bodhisattvas_trend_toward_integration() {
        for b in Bodhisattva::ALL {
            let p = b.profile();
            assert_eq!(p.growth, GrowthDirection::Integrate);
        }
    }

    #[test]
    fn dhyani_buddhas_are_late_inhale() {
        for d in DhyaniBuddha::ALL {
            let p = d.profile();
            assert_eq!(p.breath, BreathAffinity::LateInhale);
        }
    }
}
