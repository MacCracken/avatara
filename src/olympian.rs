//! Greek pantheon — 12 Olympians, Titans, demigods.

use crate::{
    Archetype, ArchetypeProfile, BreathAffinity, GrowthDirection, ModuleEmphasis, TraitWeights,
};
use serde::{Deserialize, Serialize};

/// The 12 Olympians.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Olympian {
    Zeus,
    Hera,
    Poseidon,
    Demeter,
    Athena,
    Apollo,
    Artemis,
    Ares,
    Aphrodite,
    Hephaestus,
    Hermes,
    Dionysus,
}

impl Olympian {
    pub const ALL: &'static [Self] = &[
        Self::Zeus,
        Self::Hera,
        Self::Poseidon,
        Self::Demeter,
        Self::Athena,
        Self::Apollo,
        Self::Artemis,
        Self::Ares,
        Self::Aphrodite,
        Self::Hephaestus,
        Self::Hermes,
        Self::Dionysus,
    ];
}

impl Archetype for Olympian {
    fn name(&self) -> &'static str {
        match self {
            Self::Zeus => "Zeus",
            Self::Hera => "Hera",
            Self::Poseidon => "Poseidon",
            Self::Demeter => "Demeter",
            Self::Athena => "Athena",
            Self::Apollo => "Apollo",
            Self::Artemis => "Artemis",
            Self::Ares => "Ares",
            Self::Aphrodite => "Aphrodite",
            Self::Hephaestus => "Hephaestus",
            Self::Hermes => "Hermes",
            Self::Dionysus => "Dionysus",
        }
    }

    fn tradition(&self) -> &'static str {
        "Greek"
    }

    fn profile(&self) -> ArchetypeProfile {
        // TODO: Full profiles with planetary correspondences
        ArchetypeProfile {
            name: self.name().to_string(),
            tradition: self.tradition().to_string(),
            description: format!("Olympian: {}", self.name()),
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
    fn all_olympians_produce_profiles() {
        for o in Olympian::ALL {
            let p = o.profile();
            assert!(!p.name.is_empty());
            assert_eq!(p.tradition, "Greek");
        }
    }
}
