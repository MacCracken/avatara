//! Integration tests for avatara — cross-tradition invariants.

use avatara::ArchetypeProfile;

/// Collect every profile in the library.
fn all_profiles() -> Vec<ArchetypeProfile> {
    avatara::registry::all_profiles()
}

#[test]
fn total_entity_count() {
    let profiles = all_profiles();
    assert!(
        profiles.len() >= 250,
        "expected at least 250 entities, got {}",
        profiles.len()
    );
}

#[test]
fn all_profiles_have_nonempty_name() {
    for p in all_profiles() {
        assert!(
            !p.name.is_empty(),
            "profile has empty name (tradition: {})",
            p.tradition
        );
    }
}

#[test]
fn all_profiles_have_nonempty_tradition() {
    for p in all_profiles() {
        assert!(
            !p.tradition.is_empty(),
            "profile {} has empty tradition",
            p.name
        );
    }
}

#[test]
fn all_profiles_have_nonempty_description() {
    for p in all_profiles() {
        assert!(
            !p.description.is_empty(),
            "{} ({}) has empty description",
            p.name,
            p.tradition
        );
    }
}

#[test]
fn all_profiles_have_nonempty_soul_text() {
    for p in all_profiles() {
        assert!(
            !p.soul_text.is_empty(),
            "{} ({}) has empty soul_text",
            p.name,
            p.tradition
        );
    }
}

#[test]
fn all_profiles_have_nonempty_spirit_text() {
    for p in all_profiles() {
        assert!(
            !p.spirit_text.is_empty(),
            "{} ({}) has empty spirit_text",
            p.name,
            p.tradition
        );
    }
}

#[test]
fn all_trait_weights_in_range() {
    for p in all_profiles() {
        let t = &p.traits;
        let fields = [
            ("warmth", t.warmth),
            ("humor", t.humor),
            ("empathy", t.empathy),
            ("patience", t.patience),
            ("confidence", t.confidence),
            ("curiosity", t.curiosity),
            ("creativity", t.creativity),
            ("directness", t.directness),
            ("formality", t.formality),
            ("verbosity", t.verbosity),
            ("courage", t.courage),
            ("precision", t.precision),
            ("skepticism", t.skepticism),
            ("autonomy", t.autonomy),
            ("pedagogy", t.pedagogy),
        ];
        for (field, val) in fields {
            assert!(
                (0.0..=1.0).contains(&val),
                "{} ({}) has {} = {} (out of 0.0-1.0)",
                p.name,
                p.tradition,
                field,
                val
            );
        }
    }
}

#[test]
fn all_module_emphasis_in_range() {
    for p in all_profiles() {
        let e = &p.emphasis;
        let fields = [
            ("mood", e.mood),
            ("energy", e.energy),
            ("stress", e.stress),
            ("growth", e.growth),
            ("spirit", e.spirit),
            ("reasoning", e.reasoning),
            ("regulation", e.regulation),
            ("relationship", e.relationship),
            ("flow", e.flow),
            ("belief", e.belief),
            ("intuition", e.intuition),
            ("salience", e.salience),
            ("appraisal", e.appraisal),
            ("eq", e.eq),
        ];
        for (field, val) in fields {
            assert!(
                (0.0..=1.0).contains(&val),
                "{} ({}) has emphasis.{} = {} (out of 0.0-1.0)",
                p.name,
                p.tradition,
                field,
                val
            );
        }
    }
}

#[test]
fn no_duplicate_names_within_tradition() {
    use std::collections::HashMap;
    let mut by_tradition: HashMap<String, Vec<String>> = HashMap::new();
    for p in all_profiles() {
        by_tradition
            .entry(p.tradition.clone())
            .or_default()
            .push(p.name.clone());
    }
    for (tradition, names) in &by_tradition {
        let mut sorted = names.clone();
        sorted.sort();
        for window in sorted.windows(2) {
            assert_ne!(
                window[0], window[1],
                "duplicate name '{}' in tradition '{}'",
                window[0], tradition
            );
        }
    }
}

#[test]
fn serde_roundtrip_all_profiles() {
    for p in all_profiles() {
        let json = serde_json::to_string(&p)
            .unwrap_or_else(|e| panic!("{} ({}) failed to serialize: {}", p.name, p.tradition, e));
        let deser: ArchetypeProfile = serde_json::from_str(&json).unwrap_or_else(|e| {
            panic!("{} ({}) failed to deserialize: {}", p.name, p.tradition, e)
        });
        assert_eq!(p.name, deser.name);
        assert_eq!(p.tradition, deser.tradition);
        assert_eq!(p.breath, deser.breath);
        assert_eq!(p.growth, deser.growth);
    }
}

#[test]
fn breath_affinity_intensity_monotonic() {
    use avatara::BreathAffinity;
    let phases = [
        BreathAffinity::Unity,
        BreathAffinity::EarlyExhale,
        BreathAffinity::MidExhale,
        BreathAffinity::LateExhale,
    ];
    for w in phases.windows(2) {
        assert!(
            w[0].intensity() < w[1].intensity(),
            "{:?}.intensity() should be < {:?}.intensity()",
            w[0],
            w[1]
        );
    }
    let inhale = [
        BreathAffinity::LateExhale,
        BreathAffinity::EarlyInhale,
        BreathAffinity::MidInhale,
        BreathAffinity::LateInhale,
    ];
    for w in inhale.windows(2) {
        assert!(
            w[0].intensity() > w[1].intensity(),
            "{:?}.intensity() should be > {:?}.intensity()",
            w[0],
            w[1]
        );
    }
}

#[test]
#[cfg(feature = "itihas")]
fn all_traditions_have_historical_context() {
    use avatara::history;

    let traditions = avatara::registry::traditions();
    let mapped = history::mapped_traditions();

    for tradition in &traditions {
        let ctx = history::context_for_tradition(tradition);
        assert!(
            !ctx.civilizations.is_empty() || !mapped.iter().any(|m| m == tradition),
            "{tradition} has mapping but resolved no civilizations",
        );
    }
}

#[cfg(feature = "itihas")]
#[test]
fn historical_context_resolves_for_all_profiles() {
    use avatara::history;

    for p in all_profiles() {
        let ctx = history::context_for_profile(&p);
        // Every mapped tradition should have at least one civilization
        if !ctx.primary_civilization.is_empty() {
            assert!(
                !ctx.civilizations.is_empty(),
                "{} ({}) has primary_civilization but no resolved civilizations",
                p.name,
                p.tradition,
            );
        }
    }
}

#[cfg(feature = "itihas")]
#[test]
fn historical_context_eras_within_tradition_range() {
    use avatara::history;

    for tradition in history::mapped_traditions() {
        let ctx = history::context_for_tradition(tradition);
        for era in &ctx.eras {
            // Era should overlap with the tradition's temporal range
            assert!(
                era.start_year <= ctx.end_year && era.end_year >= ctx.start_year,
                "{tradition}: era '{}' ({}-{}) does not overlap tradition range ({}-{})",
                era.name,
                era.start_year,
                era.end_year,
                ctx.start_year,
                ctx.end_year,
            );
        }
    }
}

#[cfg(feature = "itihas")]
#[test]
fn query_by_civilization_returns_correct_traditions() {
    use avatara::registry;

    let results = registry::query().civilization("Ancient Greece").collect();
    assert!(!results.is_empty());
    for p in &results {
        assert!(
            ["Greek", "Incarnate Sage"].contains(&p.tradition.as_str()),
            "unexpected tradition '{}' for Ancient Greece",
            p.tradition,
        );
    }
}

#[cfg(feature = "itihas")]
#[test]
fn query_by_era_returns_correct_traditions() {
    use avatara::registry;

    let results = registry::query().era("Vedic Period").collect();
    assert!(!results.is_empty());
    for p in &results {
        assert!(
            ["Hindu", "Jain", "Incarnate Hindu"].contains(&p.tradition.as_str()),
            "unexpected tradition '{}' for Vedic Period",
            p.tradition,
        );
    }
}

#[cfg(feature = "itihas")]
#[test]
fn historical_context_serde_roundtrip() {
    use avatara::history::{self, HistoricalContext};

    let ctx = history::context_for_tradition("Hindu");
    let json = serde_json::to_string(&ctx).expect("HistoricalContext should serialize");
    let deser: HistoricalContext =
        serde_json::from_str(&json).expect("HistoricalContext should deserialize");
    assert_eq!(ctx.primary_civilization, deser.primary_civilization);
    assert_eq!(ctx.civilizations.len(), deser.civilizations.len());
    assert_eq!(ctx.eras.len(), deser.eras.len());
    assert_eq!(ctx.start_year, deser.start_year);
    assert_eq!(ctx.end_year, deser.end_year);
}

#[test]
fn traditions_cover_expected_set() {
    use std::collections::HashSet;
    let traditions: HashSet<String> = all_profiles().into_iter().map(|p| p.tradition).collect();

    let expected = [
        "Kabbalah",
        "Angelic",
        "Hindu",
        "Greek",
        "Norse",
        "Egyptian",
        "Buddhist",
        "Mesopotamian",
        "Celtic",
        "Shinto",
        "Aztec",
        "Maya",
        "Yoruba",
        "Zoroastrian",
        "Taoist",
        "Mystic",
        "Indigenous",
        "Polynesian",
        "Slavic",
        "Jain",
        "Sikh",
        "Vedic",
    ];
    for t in expected {
        assert!(traditions.contains(t), "missing tradition: {}", t);
    }
}
