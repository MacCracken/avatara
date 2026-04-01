//! Norse pantheon — Aesir, Vanir, Norns.

use crate::{
    Archetype, ArchetypeProfile, BreathAffinity, GrowthDirection, ModuleEmphasis, TraitWeights,
};
use serde::{Deserialize, Serialize};

/// Aesir and Vanir gods.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum NorseGod {
    Odin,
    Thor,
    Freya,
    Loki,
    Tyr,
    Baldur,
    Heimdall,
    Frigg,
    Njord,
    Freyr,
}

impl NorseGod {
    pub const ALL: &'static [Self] = &[
        Self::Odin,
        Self::Thor,
        Self::Freya,
        Self::Loki,
        Self::Tyr,
        Self::Baldur,
        Self::Heimdall,
        Self::Frigg,
        Self::Njord,
        Self::Freyr,
    ];
}

impl Archetype for NorseGod {
    fn name(&self) -> &'static str {
        match self {
            Self::Odin => "Odin",
            Self::Thor => "Thor",
            Self::Freya => "Freya",
            Self::Loki => "Loki",
            Self::Tyr => "Tyr",
            Self::Baldur => "Baldur",
            Self::Heimdall => "Heimdall",
            Self::Frigg => "Frigg",
            Self::Njord => "Njord",
            Self::Freyr => "Freyr",
        }
    }

    fn tradition(&self) -> &'static str {
        "Norse"
    }

    fn profile(&self) -> ArchetypeProfile {
        // TODO: Full profiles
        ArchetypeProfile {
            name: self.name().to_string(),
            tradition: self.tradition().to_string(),
            description: format!("Norse: {}", self.name()),
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
    fn all_norse_gods_produce_profiles() {
        for g in NorseGod::ALL {
            let p = g.profile();
            assert!(!p.name.is_empty());
            assert_eq!(p.tradition, "Norse");
        }
    }
}
