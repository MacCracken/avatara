//! Celestial hierarchy — 9 angelic orders and 7 archangels.
//!
//! The Pseudo-Dionysian hierarchy maps orders of celestial beings from
//! closest to the divine (Seraphim) to closest to humanity (Angels).
//! Each order corresponds to a manifestation intensity level.

use serde::{Deserialize, Serialize};

use crate::{Archetype, ArchetypeProfile, BreathAffinity, GrowthDirection, ModuleEmphasis, TraitWeights};

/// The 7 principal archangels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Archangel {
    /// Michael — Sun, courage, protection, truth.
    Michael,
    /// Gabriel — Moon, communication, revelation, dreams.
    Gabriel,
    /// Raphael — Mercury, healing, travel, knowledge.
    Raphael,
    /// Uriel — Earth, wisdom, repentance, illumination.
    Uriel,
    /// Chamuel — Venus, love, compassion, peaceful relationships.
    Chamuel,
    /// Jophiel — Jupiter, wisdom, understanding, beauty of thought.
    Jophiel,
    /// Zadkiel — Saturn, mercy, forgiveness, transmutation.
    Zadkiel,
}

impl Archangel {
    /// All archangels.
    pub const ALL: &'static [Self] = &[
        Self::Michael, Self::Gabriel, Self::Raphael, Self::Uriel,
        Self::Chamuel, Self::Jophiel, Self::Zadkiel,
    ];
}

/// The 9 angelic orders (Pseudo-Dionysian hierarchy).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum AngelicOrder {
    /// First triad — contemplation of God.
    Seraphim,
    Cherubim,
    Thrones,
    /// Second triad — governance of creation.
    Dominions,
    Virtues,
    Powers,
    /// Third triad — interaction with humanity.
    Principalities,
    Archangels,
    Angels,
}

impl AngelicOrder {
    /// All orders from highest to lowest.
    pub const ALL: &'static [Self] = &[
        Self::Seraphim, Self::Cherubim, Self::Thrones,
        Self::Dominions, Self::Virtues, Self::Powers,
        Self::Principalities, Self::Archangels, Self::Angels,
    ];
}

impl Archetype for Archangel {
    fn name(&self) -> &'static str {
        match self {
            Self::Michael => "Michael",
            Self::Gabriel => "Gabriel",
            Self::Raphael => "Raphael",
            Self::Uriel => "Uriel",
            Self::Chamuel => "Chamuel",
            Self::Jophiel => "Jophiel",
            Self::Zadkiel => "Zadkiel",
        }
    }

    fn tradition(&self) -> &'static str { "Angelic" }

    fn profile(&self) -> ArchetypeProfile {
        // TODO: Full archetype profiles for each archangel
        ArchetypeProfile {
            name: self.name().to_string(),
            tradition: self.tradition().to_string(),
            description: format!("Archangel {}", self.name()),
            traits: TraitWeights::default(),
            emphasis: ModuleEmphasis::default(),
            breath: BreathAffinity::MidExhale,
            growth: GrowthDirection::Preserve,
            soul_text: String::new(),
            spirit_text: String::new(),
        }
    }
}

impl Archetype for AngelicOrder {
    fn name(&self) -> &'static str {
        match self {
            Self::Seraphim => "Seraphim",
            Self::Cherubim => "Cherubim",
            Self::Thrones => "Thrones",
            Self::Dominions => "Dominions",
            Self::Virtues => "Virtues",
            Self::Powers => "Powers",
            Self::Principalities => "Principalities",
            Self::Archangels => "Archangels",
            Self::Angels => "Angels",
        }
    }

    fn tradition(&self) -> &'static str { "Angelic" }

    fn profile(&self) -> ArchetypeProfile {
        // TODO: Full archetype profiles for each order
        ArchetypeProfile {
            name: self.name().to_string(),
            tradition: self.tradition().to_string(),
            description: format!("Angelic order: {}", self.name()),
            traits: TraitWeights::default(),
            emphasis: ModuleEmphasis::default(),
            breath: BreathAffinity::MidExhale,
            growth: GrowthDirection::Preserve,
            soul_text: String::new(),
            spirit_text: String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_archangels_produce_profiles() {
        for a in Archangel::ALL {
            let p = a.profile();
            assert!(!p.name.is_empty());
            assert_eq!(p.tradition, "Angelic");
        }
    }

    #[test]
    fn all_orders_produce_profiles() {
        for o in AngelicOrder::ALL {
            let p = o.profile();
            assert!(!p.name.is_empty());
        }
    }
}
