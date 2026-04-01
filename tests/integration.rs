//! Integration tests for avatara — cross-tradition invariants.

use avatara::Archetype;
use avatara::ArchetypeProfile;

/// Collect every profile in the library.
fn all_profiles() -> Vec<ArchetypeProfile> {
    use avatara::{
        angelic::{AngelicOrder, Archangel},
        aztec::AztecGod,
        buddhist::{Bodhisattva, DhyaniBuddha},
        celtic::CelticGod,
        egyptian::EgyptianGod,
        hindu::{Avatar, Deva, Trimurti},
        incarnate::{
            IncarnateBuddhist, IncarnateHindu, IncarnateIndigenous, IncarnateMystic,
            IncarnateTaoist,
        },
        kabbalah::Sephira,
        maya::MayanGod,
        mesopotamian::MesopotamianGod,
        norse::NorseGod,
        olympian::Olympian,
        shinto::Kami,
        taoist::{Immortal, TaoistDeity},
        yoruba::Orisha,
        zoroastrian::{AmeshaSpentas, ZoroastrianBeing},
    };

    let mut profiles = Vec::new();
    for s in Sephira::ALL { profiles.push(s.profile()); }
    for a in Archangel::ALL { profiles.push(a.profile()); }
    for o in AngelicOrder::ALL { profiles.push(o.profile()); }
    for t in Trimurti::ALL { profiles.push(t.profile()); }
    for d in Deva::ALL { profiles.push(d.profile()); }
    for a in Avatar::ALL { profiles.push(a.profile()); }
    for o in Olympian::ALL { profiles.push(o.profile()); }
    for g in NorseGod::ALL { profiles.push(g.profile()); }
    for g in EgyptianGod::ALL { profiles.push(g.profile()); }
    for b in Bodhisattva::ALL { profiles.push(b.profile()); }
    for d in DhyaniBuddha::ALL { profiles.push(d.profile()); }
    for g in MesopotamianGod::ALL { profiles.push(g.profile()); }
    for g in CelticGod::ALL { profiles.push(g.profile()); }
    for k in Kami::ALL { profiles.push(k.profile()); }
    for g in AztecGod::ALL { profiles.push(g.profile()); }
    for g in MayanGod::ALL { profiles.push(g.profile()); }
    for o in Orisha::ALL { profiles.push(o.profile()); }
    for a in AmeshaSpentas::ALL { profiles.push(a.profile()); }
    for z in ZoroastrianBeing::ALL { profiles.push(z.profile()); }
    for i in Immortal::ALL { profiles.push(i.profile()); }
    for d in TaoistDeity::ALL { profiles.push(d.profile()); }
    for i in IncarnateHindu::ALL { profiles.push(i.profile()); }
    for i in IncarnateBuddhist::ALL { profiles.push(i.profile()); }
    for i in IncarnateMystic::ALL { profiles.push(i.profile()); }
    for i in IncarnateTaoist::ALL { profiles.push(i.profile()); }
    for i in IncarnateIndigenous::ALL { profiles.push(i.profile()); }
    profiles
}

#[test]
fn total_entity_count() {
    let profiles = all_profiles();
    // 10 + 7 + 9 + 3 + 7 + 10 + 12 + 10 + 12 + 7 + 5
    // + 14 + 14 + 14 + 14 + 12 + 14 + 7 + 7 + 8 + 8
    // + 10 + 8 + 10 + 4 + 4 = 206 (approx, assert >= 200)
    assert!(
        profiles.len() >= 200,
        "expected at least 200 entities, got {}",
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
        assert!(!p.tradition.is_empty(), "profile {} has empty tradition", p.name);
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
                "{} ({}) has {} = {} (out of 0.0–1.0)",
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
                "{} ({}) has emphasis.{} = {} (out of 0.0–1.0)",
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
        let json = serde_json::to_string(&p).unwrap_or_else(|e| {
            panic!("{} ({}) failed to serialize: {}", p.name, p.tradition, e)
        });
        let deser: ArchetypeProfile = serde_json::from_str(&json).unwrap_or_else(|e| {
            panic!(
                "{} ({}) failed to deserialize: {}",
                p.name, p.tradition, e
            )
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
fn traditions_cover_expected_set() {
    use std::collections::HashSet;
    let traditions: HashSet<String> = all_profiles().into_iter().map(|p| p.tradition).collect();

    let expected = [
        "Kabbalah", "Angelic", "Hindu", "Greek", "Norse", "Egyptian", "Buddhist",
        "Mesopotamian", "Celtic", "Shinto", "Aztec", "Maya", "Yoruba", "Zoroastrian",
        "Taoist", "Mystic", "Indigenous",
    ];
    for t in expected {
        assert!(traditions.contains(t), "missing tradition: {}", t);
    }
}
