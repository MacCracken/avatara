//! Archetype registry — lookup, enumeration, and query.
//!
//! Provides access to all ~206 archetypes in the library by name,
//! tradition, or trait criteria.
//!
//! # Example
//!
//! ```
//! use avatara::registry;
//!
//! // Lookup by name
//! let krishna = registry::lookup("Krishna").unwrap();
//! assert_eq!(krishna.tradition, "Hindu");
//!
//! // Lookup within a tradition
//! let odin = registry::lookup_in("Norse", "Odin").unwrap();
//! assert!(odin.traits.curiosity > 0.8);
//!
//! // Query by trait threshold
//! let courageous = registry::query()
//!     .min_trait(|t| t.courage, 0.9)
//!     .collect();
//! assert!(courageous.len() > 5);
//! ```

use crate::{
    Archetype, ArchetypeProfile, BreathAffinity, CosmicTier, Element, GrowthDirection, Polarity,
    TraitWeights, error::AvataraError,
};

/// Return all archetype profiles in the library.
#[must_use]
pub fn all_profiles() -> Vec<ArchetypeProfile> {
    use crate::{
        angelic::{AngelicOrder, Archangel},
        aztec::AztecGod,
        buddhist::{Bodhisattva, DhyaniBuddha},
        celtic::CelticGod,
        egyptian::EgyptianGod,
        hindu::{Avatar, Deva, Trimurti},
        incarnate::{
            IncarnateBuddhist, IncarnateHindu, IncarnateIndigenous, IncarnateMystic, IncarnateSage,
            IncarnateTaoist,
        },
        jain::Tirthankara,
        kabbalah::Sephira,
        maya::MayanGod,
        mesopotamian::MesopotamianGod,
        norse::NorseGod,
        olympian::Olympian,
        polynesian::PolynesianGod,
        shinto::Kami,
        sikh::Guru,
        slavic::SlavicGod,
        taoist::{Immortal, TaoistDeity},
        yoruba::Orisha,
        zoroastrian::{AmeshaSpentas, ZoroastrianBeing},
    };

    let mut profiles = Vec::with_capacity(280);

    for s in Sephira::ALL {
        profiles.push(s.profile());
    }
    for a in Archangel::ALL {
        profiles.push(a.profile());
    }
    for o in AngelicOrder::ALL {
        profiles.push(o.profile());
    }
    for t in Trimurti::ALL {
        profiles.push(t.profile());
    }
    for d in Deva::ALL {
        profiles.push(d.profile());
    }
    for a in Avatar::ALL {
        profiles.push(a.profile());
    }
    for o in Olympian::ALL {
        profiles.push(o.profile());
    }
    for g in NorseGod::ALL {
        profiles.push(g.profile());
    }
    for g in EgyptianGod::ALL {
        profiles.push(g.profile());
    }
    for b in Bodhisattva::ALL {
        profiles.push(b.profile());
    }
    for d in DhyaniBuddha::ALL {
        profiles.push(d.profile());
    }
    for g in MesopotamianGod::ALL {
        profiles.push(g.profile());
    }
    for g in CelticGod::ALL {
        profiles.push(g.profile());
    }
    for k in Kami::ALL {
        profiles.push(k.profile());
    }
    for g in AztecGod::ALL {
        profiles.push(g.profile());
    }
    for g in MayanGod::ALL {
        profiles.push(g.profile());
    }
    for o in Orisha::ALL {
        profiles.push(o.profile());
    }
    for a in AmeshaSpentas::ALL {
        profiles.push(a.profile());
    }
    for z in ZoroastrianBeing::ALL {
        profiles.push(z.profile());
    }
    for i in Immortal::ALL {
        profiles.push(i.profile());
    }
    for d in TaoistDeity::ALL {
        profiles.push(d.profile());
    }
    for p in PolynesianGod::ALL {
        profiles.push(p.profile());
    }
    for s in SlavicGod::ALL {
        profiles.push(s.profile());
    }
    for t in Tirthankara::ALL {
        profiles.push(t.profile());
    }
    for g in Guru::ALL {
        profiles.push(g.profile());
    }
    for i in IncarnateHindu::ALL {
        profiles.push(i.profile());
    }
    for i in IncarnateBuddhist::ALL {
        profiles.push(i.profile());
    }
    for i in IncarnateMystic::ALL {
        profiles.push(i.profile());
    }
    for i in IncarnateTaoist::ALL {
        profiles.push(i.profile());
    }
    for i in IncarnateIndigenous::ALL {
        profiles.push(i.profile());
    }
    for i in IncarnateSage::ALL {
        profiles.push(i.profile());
    }

    profiles
}

/// Look up an archetype profile by name (case-sensitive).
///
/// Returns the first match across all traditions. Use [`lookup_in`]
/// to disambiguate if the same name appears in multiple traditions.
///
/// # Errors
///
/// Returns [`AvataraError::UnknownArchetype`] if no entity matches the name.
pub fn lookup(name: &str) -> Result<ArchetypeProfile, AvataraError> {
    all_profiles()
        .into_iter()
        .find(|p| p.name == name)
        .ok_or_else(|| AvataraError::UnknownArchetype(name.to_string()))
}

/// Look up an archetype profile by tradition and name (both case-sensitive).
///
/// # Errors
///
/// Returns [`AvataraError::UnknownArchetype`] if no entity matches.
pub fn lookup_in(tradition: &str, name: &str) -> Result<ArchetypeProfile, AvataraError> {
    all_profiles()
        .into_iter()
        .find(|p| p.tradition == tradition && p.name == name)
        .ok_or_else(|| AvataraError::UnknownArchetype(format!("{tradition}/{name}")))
}

/// List all unique tradition names in the library.
#[must_use]
pub fn traditions() -> Vec<String> {
    let mut ts: Vec<String> = all_profiles().into_iter().map(|p| p.tradition).collect();
    ts.sort();
    ts.dedup();
    ts
}

/// List all profiles within a given tradition.
#[must_use]
pub fn by_tradition(tradition: &str) -> Vec<ArchetypeProfile> {
    all_profiles()
        .into_iter()
        .filter(|p| p.tradition == tradition)
        .collect()
}

/// Create a query builder for filtering archetypes.
#[must_use]
pub fn query() -> QueryBuilder {
    QueryBuilder::new()
}

type ProfileFilter = Box<dyn Fn(&ArchetypeProfile) -> bool>;

/// Builder for filtering archetype profiles by various criteria.
pub struct QueryBuilder {
    filters: Vec<ProfileFilter>,
}

impl QueryBuilder {
    fn new() -> Self {
        Self {
            filters: Vec::new(),
        }
    }

    /// Filter to a specific tradition.
    #[must_use]
    pub fn tradition(mut self, tradition: &str) -> Self {
        let t = tradition.to_string();
        self.filters.push(Box::new(move |p| p.tradition == t));
        self
    }

    /// Filter to a specific breath affinity.
    #[must_use]
    pub fn breath(mut self, breath: BreathAffinity) -> Self {
        self.filters.push(Box::new(move |p| p.breath == breath));
        self
    }

    /// Filter to a specific growth direction.
    #[must_use]
    pub fn growth(mut self, growth: GrowthDirection) -> Self {
        self.filters.push(Box::new(move |p| p.growth == growth));
        self
    }

    /// Filter to a specific element.
    #[must_use]
    pub fn element(mut self, element: Element) -> Self {
        self.filters.push(Box::new(move |p| p.element == element));
        self
    }

    /// Filter to a specific polarity.
    #[must_use]
    pub fn polarity(mut self, polarity: Polarity) -> Self {
        self.filters.push(Box::new(move |p| p.polarity == polarity));
        self
    }

    /// Filter to a specific cosmic tier.
    #[must_use]
    pub fn tier(mut self, tier: CosmicTier) -> Self {
        self.filters.push(Box::new(move |p| p.tier == tier));
        self
    }

    /// Filter by a minimum trait value.
    ///
    /// ```
    /// use avatara::registry;
    ///
    /// let brave = registry::query()
    ///     .min_trait(|t| t.courage, 0.9)
    ///     .collect();
    /// assert!(!brave.is_empty());
    /// ```
    #[must_use]
    pub fn min_trait(mut self, accessor: fn(&TraitWeights) -> f64, min: f64) -> Self {
        self.filters
            .push(Box::new(move |p| accessor(&p.traits) >= min));
        self
    }

    /// Filter by a maximum trait value.
    #[must_use]
    pub fn max_trait(mut self, accessor: fn(&TraitWeights) -> f64, max: f64) -> Self {
        self.filters
            .push(Box::new(move |p| accessor(&p.traits) <= max));
        self
    }

    /// Collect all matching profiles.
    #[must_use]
    pub fn collect(self) -> Vec<ArchetypeProfile> {
        all_profiles()
            .into_iter()
            .filter(|p| self.filters.iter().all(|f| f(p)))
            .collect()
    }

    /// Count matching profiles without allocating.
    #[must_use]
    pub fn count(self) -> usize {
        all_profiles()
            .into_iter()
            .filter(|p| self.filters.iter().all(|f| f(p)))
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookup_krishna() {
        let p = lookup("Krishna").unwrap();
        assert_eq!(p.tradition, "Hindu");
        assert!(p.traits.warmth > 0.8);
    }

    #[test]
    fn lookup_unknown_returns_error() {
        let r = lookup("NonexistentGod");
        assert!(r.is_err());
        match r.unwrap_err() {
            AvataraError::UnknownArchetype(name) => assert_eq!(name, "NonexistentGod"),
            _ => panic!("expected UnknownArchetype"),
        }
    }

    #[test]
    fn lookup_in_tradition() {
        let p = lookup_in("Norse", "Odin").unwrap();
        assert!(p.traits.curiosity > 0.8);
    }

    #[test]
    fn lookup_in_wrong_tradition() {
        let r = lookup_in("Greek", "Odin");
        assert!(r.is_err());
    }

    #[test]
    fn all_profiles_count() {
        let profiles = all_profiles();
        assert!(profiles.len() >= 200);
    }

    #[test]
    fn traditions_list() {
        let ts = traditions();
        assert!(ts.contains(&"Kabbalah".to_string()));
        assert!(ts.contains(&"Hindu".to_string()));
        assert!(ts.contains(&"Norse".to_string()));
        assert!(ts.contains(&"Aztec".to_string()));
    }

    #[test]
    fn by_tradition_filters() {
        let norse = by_tradition("Norse");
        assert_eq!(norse.len(), 13);
        for p in &norse {
            assert_eq!(p.tradition, "Norse");
        }
    }

    #[test]
    fn query_courage_above_09() {
        let brave = query().min_trait(|t| t.courage, 0.9).collect();
        assert!(!brave.is_empty());
        for p in &brave {
            assert!(
                p.traits.courage >= 0.9,
                "{} has courage {}",
                p.name,
                p.traits.courage
            );
        }
    }

    #[test]
    fn query_transform_growth() {
        let transformers = query().growth(GrowthDirection::Transform).collect();
        assert!(!transformers.is_empty());
        for p in &transformers {
            assert_eq!(p.growth, GrowthDirection::Transform);
        }
    }

    #[test]
    fn query_combined_filters() {
        let results = query()
            .tradition("Hindu")
            .min_trait(|t| t.courage, 0.8)
            .collect();
        assert!(!results.is_empty());
        for p in &results {
            assert_eq!(p.tradition, "Hindu");
            assert!(p.traits.courage >= 0.8);
        }
    }

    #[test]
    fn query_unity_breath() {
        let unity = query().breath(BreathAffinity::Unity).collect();
        assert!(!unity.is_empty());
        for p in &unity {
            assert_eq!(p.breath, BreathAffinity::Unity);
        }
    }

    #[test]
    fn query_count_matches_collect_len() {
        let count = query().min_trait(|t| t.warmth, 0.9).count();
        let collected = query().min_trait(|t| t.warmth, 0.9).collect();
        assert_eq!(count, collected.len());
    }
}
