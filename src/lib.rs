//! Avatara — Divine archetype engine for AGNOS
//!
//! Sanskrit: अवतार (avatara) — descent of the divine, manifestation of archetypes
//!
//! Maps theological and mythological beings across traditions to personality
//! configurations. Not religion simulation — psychometric archetype mapping.
//! Each divine entity maps to trait profiles, module emphasis, breath phase,
//! and growth direction using the same infrastructure as bhava's zodiac engine.
//!
//! # Traditions
//!
//! - [`kabbalah`] — Tree of Life: 10 Sephiroth
//! - [`angelic`] — 9 angelic orders, 7 archangels, celestial hierarchy
//! - [`hindu`] — Trimurti, 11 Devas, 10 Avatars of Vishnu
//! - [`olympian`] — Greek pantheon: 15 deities (12 Olympians + Hades, Hestia, Persephone)
//! - [`norse`] — Aesir and Vanir: 13 gods
//! - [`egyptian`] — 16 principal deities
//! - [`buddhist`] — 7 Bodhisattvas, 5 Dhyani Buddhas
//! - [`mesopotamian`] — Sumerian/Babylonian: 14 deities
//! - [`celtic`] — Tuatha Dé Danann & Insular Celtic: 15 deities
//! - [`shinto`] — Japanese Kami: 15 deities
//! - [`aztec`] — Aztec (Mexica): 14 deities
//! - [`maya`] — Maya: 12 deities
//! - [`yoruba`] — Yoruba/Ifá: 14 Orishas
//! - [`zoroastrian`] — Amesha Spentas and Zoroastrian beings: 14 figures
//! - [`taoist`] — Eight Immortals and celestial deities: 16 figures
//! - [`polynesian`] — Polynesian/Hawaiian: 12 deities
//! - [`slavic`] — Pre-Christian Slavic: 12 deities
//! - [`jain`] — 24 Tirthankaras
//! - [`sikh`] — 10 Sikh Gurus
//! - [`incarnate`] — Incarnate divine figures across traditions: 44 masters
//!
//! # Design
//!
//! All traditions map to a common output: `ArchetypeProfile` containing
//! trait weights, module emphasis, breath phase affinity, and growth
//! direction. Composable across traditions — an entity can carry
//! Kabbalistic + Hindu + Greek archetypes simultaneously, with
//! reinforcing and conflicting dynamics.
//!
//! Avatara produces plain f64/enum outputs. It does not depend on bhava —
//! bhava consumes avatara's output through a bridge module, same pattern
//! as jyotish/tara/tanmatra.
//!
//! # Example
//!
//! ```
//! use avatara::Archetype;
//! use avatara::kabbalah::Sephira;
//!
//! let tiphareth = Sephira::Tiphareth.profile();
//! assert!(tiphareth.traits.warmth > 0.5);
//! assert!(tiphareth.traits.confidence > 0.5);
//! ```

pub mod compose;
pub mod error;
pub mod registry;

pub mod angelic;
pub mod aztec;
pub mod buddhist;
pub mod celtic;
pub mod egyptian;
pub mod hindu;
pub mod incarnate;
pub mod jain;
pub mod kabbalah;
pub mod maya;
pub mod mesopotamian;
pub mod norse;
pub mod olympian;
pub mod polynesian;
pub mod sikh;
pub mod shinto;
pub mod slavic;
pub mod taoist;
pub mod yoruba;
pub mod zoroastrian;

pub mod logging;

// ── Common types ──────────────────────────────────────────────────────

use serde::{Deserialize, Serialize};

/// Trait weights for an archetype — normalized 0.0–1.0.
///
/// These map 1:1 to bhava's 15-trait `PersonalityProfile` via the bridge.
/// Named for clarity in the theological domain rather than reusing bhava names.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TraitWeights {
    pub warmth: f64,
    pub humor: f64,
    pub empathy: f64,
    pub patience: f64,
    pub confidence: f64,
    pub curiosity: f64,
    pub creativity: f64,
    pub directness: f64,
    pub formality: f64,
    pub verbosity: f64,
    pub courage: f64,
    pub precision: f64,
    pub skepticism: f64,
    pub autonomy: f64,
    pub pedagogy: f64,
}

impl Default for TraitWeights {
    /// All traits at neutral midpoint (0.5).
    fn default() -> Self {
        Self {
            warmth: 0.5,
            humor: 0.5,
            empathy: 0.5,
            patience: 0.5,
            confidence: 0.5,
            curiosity: 0.5,
            creativity: 0.5,
            directness: 0.5,
            formality: 0.5,
            verbosity: 0.5,
            courage: 0.5,
            precision: 0.5,
            skepticism: 0.5,
            autonomy: 0.5,
            pedagogy: 0.5,
        }
    }
}

/// Module emphasis — which bhava modules this archetype amplifies.
///
/// Values 0.0 (no emphasis) to 1.0 (maximum emphasis). The bridge
/// uses these as multipliers on module-specific parameters.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ModuleEmphasis {
    pub mood: f64,
    pub energy: f64,
    pub stress: f64,
    pub growth: f64,
    pub spirit: f64,
    pub reasoning: f64,
    pub regulation: f64,
    pub relationship: f64,
    pub flow: f64,
    pub belief: f64,
    pub intuition: f64,
    pub salience: f64,
    pub appraisal: f64,
    pub eq: f64,
}

impl Default for ModuleEmphasis {
    /// All modules at neutral midpoint (0.5).
    fn default() -> Self {
        Self {
            mood: 0.5,
            energy: 0.5,
            stress: 0.5,
            growth: 0.5,
            spirit: 0.5,
            reasoning: 0.5,
            regulation: 0.5,
            relationship: 0.5,
            flow: 0.5,
            belief: 0.5,
            intuition: 0.5,
            salience: 0.5,
            appraisal: 0.5,
            eq: 0.5,
        }
    }
}

/// Manifestation phase affinity — where on the cosmic breath this archetype sits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub enum BreathAffinity {
    /// Source/return — undifferentiated unity.
    Unity,
    /// Consciousness beginning to differentiate.
    EarlyExhale,
    /// Active individuation.
    MidExhale,
    /// Maximum manifestation — most entities live here.
    #[default]
    LateExhale,
    /// Form beginning to soften.
    EarlyInhale,
    /// Active dissolution of patterns.
    MidInhale,
    /// Approaching equanimity.
    LateInhale,
}

impl BreathAffinity {
    /// Manifestation intensity: 0.0 (unity) to 1.0 (full form).
    #[must_use]
    #[inline]
    pub fn intensity(&self) -> f64 {
        match self {
            Self::Unity => 0.0,
            Self::EarlyExhale => 0.15,
            Self::MidExhale => 0.5,
            Self::LateExhale => 1.0,
            Self::EarlyInhale => 0.8,
            Self::MidInhale => 0.4,
            Self::LateInhale => 0.1,
        }
    }
}

/// Growth direction — whether the archetype pushes toward differentiation or integration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub enum GrowthDirection {
    /// Become more distinct, individuated.
    #[default]
    Differentiate,
    /// Become more integrated, unified.
    Integrate,
    /// Neither — preservation, stability.
    Preserve,
    /// Neither — transformation, destruction of form.
    Transform,
    /// Still — no growth pressure.
    Still,
}

/// Complete archetype profile — the output of any tradition's mapping.
///
/// This is what bhava's bridge module consumes. All fields are plain
/// values — no bhava types leak into avatara.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArchetypeProfile {
    /// Human-readable name of the archetype.
    pub name: String,
    /// Tradition this archetype belongs to.
    pub tradition: String,
    /// Brief description of the archetype's essence.
    pub description: String,
    /// Personality trait weights (0.0–1.0).
    pub traits: TraitWeights,
    /// Which bhava modules this archetype emphasizes.
    pub emphasis: ModuleEmphasis,
    /// Where on the cosmic breath this archetype sits.
    pub breath: BreathAffinity,
    /// Growth direction this archetype pushes toward.
    pub growth: GrowthDirection,
    /// Soul-level identity text (maps to bhava `IdentityLayer::Soul`).
    pub soul_text: String,
    /// Spirit-level identity text (maps to bhava `IdentityLayer::Spirit`).
    pub spirit_text: String,
}

/// Trait for any entity that can produce an archetype profile.
pub trait Archetype {
    /// Generate the full archetype profile for this entity.
    #[must_use]
    fn profile(&self) -> ArchetypeProfile;

    /// Short name for display.
    #[must_use]
    fn name(&self) -> &'static str;

    /// Tradition this entity belongs to.
    #[must_use]
    fn tradition(&self) -> &'static str;
}

pub use error::AvataraError;
