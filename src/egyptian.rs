//! Egyptian pantheon — Ennead, Ogdoad, cosmic deities.

use crate::{
    Archetype, ArchetypeProfile, BreathAffinity, GrowthDirection, ModuleEmphasis, TraitWeights,
};
use serde::{Deserialize, Serialize};

/// Principal Egyptian deities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum EgyptianGod {
    Ra,
    Thoth,
    Maat,
    Osiris,
    Isis,
    Anubis,
    Horus,
    Set,
    Hathor,
    Ptah,
    Sekhmet,
    Bastet,
}

impl EgyptianGod {
    pub const ALL: &'static [Self] = &[
        Self::Ra,
        Self::Thoth,
        Self::Maat,
        Self::Osiris,
        Self::Isis,
        Self::Anubis,
        Self::Horus,
        Self::Set,
        Self::Hathor,
        Self::Ptah,
        Self::Sekhmet,
        Self::Bastet,
    ];
}

impl Archetype for EgyptianGod {
    fn name(&self) -> &'static str {
        match self {
            Self::Ra => "Ra",
            Self::Thoth => "Thoth",
            Self::Maat => "Ma'at",
            Self::Osiris => "Osiris",
            Self::Isis => "Isis",
            Self::Anubis => "Anubis",
            Self::Horus => "Horus",
            Self::Set => "Set",
            Self::Hathor => "Hathor",
            Self::Ptah => "Ptah",
            Self::Sekhmet => "Sekhmet",
            Self::Bastet => "Bastet",
        }
    }

    fn tradition(&self) -> &'static str {
        "Egyptian"
    }

    fn profile(&self) -> ArchetypeProfile {
        // TODO: Full profiles
        ArchetypeProfile {
            name: self.name().to_string(),
            tradition: self.tradition().to_string(),
            description: format!("Egyptian: {}", self.name()),
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
    fn all_egyptian_gods_produce_profiles() {
        for g in EgyptianGod::ALL {
            let p = g.profile();
            assert!(!p.name.is_empty());
            assert_eq!(p.tradition, "Egyptian");
        }
    }
}
