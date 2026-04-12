//! Historical context integration via [`itihas`].
//!
//! Maps avatara traditions to their civilizational and temporal context,
//! grounding divine archetypes in the historical reality where they emerged.
//! Each tradition maps to one or more civilizations, eras, and a temporal
//! range — the period during which the tradition's theology was actively
//! forming.
//!
//! Requires the `itihas` feature.
//!
//! # Example
//!
//! ```
//! # #[cfg(feature = "itihas")]
//! # {
//! use avatara::history;
//!
//! let ctx = history::context_for_tradition("Hindu");
//! assert!(!ctx.civilizations.is_empty());
//! assert!(ctx.primary_civilization == "Indus Valley");
//! # }
//! ```

use itihas::civilization::{self, Civilization};
use itihas::era::{self, Era};

use serde::{Deserialize, Serialize};

/// Historical context for a tradition — civilizations, eras, and temporal range.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct HistoricalContext {
    /// Primary civilization associated with this tradition.
    pub primary_civilization: String,
    /// All civilizations where this tradition was practiced.
    pub civilizations: Vec<Civilization>,
    /// Historical eras spanning the tradition's formative period.
    pub eras: Vec<Era>,
    /// Approximate start year of the tradition (negative = BCE).
    pub start_year: i32,
    /// Approximate end year (i32::MAX = living tradition).
    pub end_year: i32,
    /// Brief note on the tradition's historical situation.
    pub historical_note: String,
}

/// Mapping entry: tradition name → historical metadata.
struct TraditionMapping {
    tradition: &'static str,
    primary_civilization: &'static str,
    civilization_names: &'static [&'static str],
    era_names: &'static [&'static str],
    start_year: i32,
    end_year: i32,
    historical_note: &'static str,
}

/// All tradition-to-history mappings.
///
/// Grounded in established scholarship — each mapping reflects the
/// civilizational context where the tradition's theology formed.
const MAPPINGS: &[TraditionMapping] = &[
    TraditionMapping {
        tradition: "Kabbalah",
        primary_civilization: "Phoenicia",
        civilization_names: &["Phoenicia"],
        era_names: &["Middle Ages", "Renaissance"],
        start_year: 100,
        end_year: i32::MAX,
        historical_note: "Rooted in Jewish mysticism from the 1st century CE, \
            flourishing in medieval Provence and Spain (Zohar, 13th c.), \
            with continued development through Lurianic Kabbalah in 16th c. Safed",
    },
    TraditionMapping {
        tradition: "Angelic",
        primary_civilization: "Phoenicia",
        civilization_names: &["Phoenicia", "Byzantine Empire"],
        era_names: &["Classical Antiquity", "Middle Ages"],
        start_year: -600,
        end_year: i32::MAX,
        historical_note: "Angelology developed across Jewish, Christian, and Islamic traditions; \
            systematized by Pseudo-Dionysius (5th c.) and elaborated through medieval scholasticism",
    },
    TraditionMapping {
        tradition: "Hindu",
        primary_civilization: "Indus Valley",
        civilization_names: &["Indus Valley", "Vijayanagara Empire", "Maratha Empire"],
        era_names: &["Vedic Period", "Maurya Period", "Gupta Period", "Mughal Period"],
        start_year: -1500,
        end_year: i32::MAX,
        historical_note: "From Vedic hymns (c. 1500 BCE) through the epics and Puranas, \
            with theological elaboration across Maurya, Gupta, and medieval kingdoms; \
            living tradition with over a billion practitioners",
    },
    TraditionMapping {
        tradition: "Greek",
        primary_civilization: "Ancient Greece",
        civilization_names: &["Ancient Greece", "Roman Empire"],
        era_names: &["Classical Antiquity"],
        start_year: -800,
        end_year: 400,
        historical_note: "Olympian religion shaped by Homeric epics (8th c. BCE), \
            formalized in city-state cult practice, absorbed into Roman syncretism, \
            and declined with Christianization of the empire",
    },
    TraditionMapping {
        tradition: "Norse",
        primary_civilization: "Viking/Norse",
        civilization_names: &["Viking/Norse"],
        era_names: &["Middle Ages"],
        start_year: 200,
        end_year: 1100,
        historical_note: "Germanic-Norse mythology recorded in the Eddas (13th c.) \
            but reflecting traditions from the Migration Period through the Viking Age; \
            displaced by Christianization of Scandinavia",
    },
    TraditionMapping {
        tradition: "Egyptian",
        primary_civilization: "Ancient Egypt",
        civilization_names: &["Ancient Egypt"],
        era_names: &["Bronze Age", "Iron Age", "Classical Antiquity"],
        start_year: -3100,
        end_year: 400,
        historical_note: "Among the longest-lived religious traditions — from predynastic \
            nomes through the Ptolemaic period, with theology evolving across \
            Old, Middle, and New Kingdoms over three millennia",
    },
    TraditionMapping {
        tradition: "Buddhist",
        primary_civilization: "Indus Valley",
        civilization_names: &["Indus Valley", "Ancient China", "Khmer Empire"],
        era_names: &["Maurya Period", "Gupta Period", "Han Dynasty", "Tang Dynasty"],
        start_year: -500,
        end_year: i32::MAX,
        historical_note: "Founded by Siddhartha Gautama (5th c. BCE) in the Gangetic plain, \
            spread under Ashoka's Maurya Empire, diversified into Mahayana and Vajrayana \
            across East and Southeast Asia",
    },
    TraditionMapping {
        tradition: "Mesopotamian",
        primary_civilization: "Mesopotamia",
        civilization_names: &["Mesopotamia", "Assyrian Empire"],
        era_names: &["Bronze Age", "Iron Age"],
        start_year: -3500,
        end_year: -500,
        historical_note: "Sumerian, Akkadian, and Babylonian religion — among humanity's \
            earliest recorded theologies, from Uruk-period temples through the \
            Neo-Babylonian Empire; displaced by Persian and Hellenistic religions",
    },
    TraditionMapping {
        tradition: "Celtic",
        primary_civilization: "Kingdom of France",
        civilization_names: &["Kingdom of France"],
        era_names: &["Iron Age", "Classical Antiquity", "Middle Ages"],
        start_year: -800,
        end_year: 1200,
        historical_note: "Continental and Insular Celtic religion, from Hallstatt culture \
            through La Tène and Irish/Welsh medieval literary traditions; \
            Tuatha Dé Danann preserved in the Lebor Gabála Érenn",
    },
    TraditionMapping {
        tradition: "Shinto",
        primary_civilization: "Tokugawa Shogunate",
        civilization_names: &["Tokugawa Shogunate"],
        era_names: &["Classical Antiquity", "Middle Ages"],
        start_year: -300,
        end_year: i32::MAX,
        historical_note: "Indigenous Japanese spirituality formalized in the Kojiki (712 CE) \
            and Nihon Shoki (720 CE); coexists with Buddhism through shinbutsu-shūgō; \
            living tradition integral to Japanese culture",
    },
    TraditionMapping {
        tradition: "Aztec",
        primary_civilization: "Aztec Empire",
        civilization_names: &["Aztec Empire", "Olmec"],
        era_names: &["Mesoamerican Postclassic"],
        start_year: 1300,
        end_year: 1521,
        historical_note: "Mexica theology synthesizing Toltec heritage with original cosmology, \
            centered on Tenochtitlan; destroyed by Spanish conquest \
            but with roots in earlier Mesoamerican traditions",
    },
    TraditionMapping {
        tradition: "Maya",
        primary_civilization: "Maya",
        civilization_names: &["Maya"],
        era_names: &["Mesoamerican Preclassic", "Mesoamerican Classic", "Mesoamerican Postclassic"],
        start_year: -2000,
        end_year: 1500,
        historical_note: "One of the longest-lived Mesoamerican traditions, from Preclassic \
            origins through Classic period city-states (Tikal, Palenque, Copán) \
            to Postclassic Yucatán; Popol Vuh recorded in the colonial period",
    },
    TraditionMapping {
        tradition: "Yoruba",
        primary_civilization: "Ghana Empire",
        civilization_names: &["Ghana Empire", "Songhai Empire"],
        era_names: &["Middle Ages"],
        start_year: -500,
        end_year: i32::MAX,
        historical_note: "Ifá theology originating in Ile-Ife (present-day Nigeria), \
            with oral tradition stretching back millennia; \
            diaspora traditions (Candomblé, Santería, Vodun) carry it worldwide",
    },
    TraditionMapping {
        tradition: "Zoroastrian",
        primary_civilization: "Persian Empire",
        civilization_names: &["Persian Empire", "Sassanid Empire", "Parthian Empire"],
        era_names: &["Iron Age", "Classical Antiquity", "Middle Ages"],
        start_year: -1500,
        end_year: i32::MAX,
        historical_note: "Founded by Zarathustra, state religion of three Persian empires \
            (Achaemenid, Parthian, Sassanid); profoundly influenced Judaism, \
            Christianity, and Islam; living tradition in Iran and India (Parsis)",
    },
    TraditionMapping {
        tradition: "Taoist",
        primary_civilization: "Ancient China",
        civilization_names: &["Ancient China"],
        era_names: &[
            "Zhou Dynasty",
            "Han Dynasty",
            "Tang Dynasty",
            "Song Dynasty",
            "Ming Dynasty",
        ],
        start_year: -600,
        end_year: i32::MAX,
        historical_note: "From Laozi and Zhuangzi (6th–4th c. BCE) through Celestial Masters \
            and alchemical traditions, deeply woven into Chinese civilization across \
            every dynasty; living tradition",
    },
    TraditionMapping {
        tradition: "Polynesian",
        primary_civilization: "Tonga Empire",
        civilization_names: &["Tonga Empire", "Hawaiian Kingdom"],
        era_names: &["Middle Ages"],
        start_year: -1000,
        end_year: i32::MAX,
        historical_note: "Austronesian spiritual tradition spanning the Pacific triangle — \
            Hawai'i, Aotearoa, Rapa Nui — with shared cosmology diverging \
            through centuries of island isolation; oral tradition preserved by tohunga",
    },
    TraditionMapping {
        tradition: "Slavic",
        primary_civilization: "Byzantine Empire",
        civilization_names: &["Byzantine Empire"],
        era_names: &["Middle Ages"],
        start_year: 300,
        end_year: 1200,
        historical_note: "Pre-Christian Slavic religion reconstructed from chronicles, \
            place names, and folklore; Perun, Veles, and Mokosh displaced by \
            Christianization beginning with Kievan Rus' (988 CE)",
    },
    TraditionMapping {
        tradition: "Jain",
        primary_civilization: "Indus Valley",
        civilization_names: &["Indus Valley", "Vijayanagara Empire"],
        era_names: &["Vedic Period", "Maurya Period", "Gupta Period"],
        start_year: -800,
        end_year: i32::MAX,
        historical_note: "Ancient Indian tradition tracing to Rishabhadeva; historically \
            attested from Parshvanatha (8th c. BCE) and Mahavira (6th c. BCE); \
            major influence on Indian philosophy, art, and non-violence ethics",
    },
    TraditionMapping {
        tradition: "Sikh",
        primary_civilization: "Sikh Empire",
        civilization_names: &["Sikh Empire"],
        era_names: &["Mughal Period"],
        start_year: 1469,
        end_year: i32::MAX,
        historical_note: "Founded by Guru Nanak (1469) in the Punjab during Mughal rule; \
            ten Gurus shaped theology, scripture (Guru Granth Sahib), and martial \
            tradition (Khalsa, 1699); living tradition with 30 million adherents",
    },
    TraditionMapping {
        tradition: "Incarnate Hindu",
        primary_civilization: "Indus Valley",
        civilization_names: &["Indus Valley", "Maratha Empire"],
        era_names: &["Vedic Period", "Gupta Period", "Mughal Period"],
        start_year: -500,
        end_year: i32::MAX,
        historical_note: "Hindu saints, yogis, and reformers — incarnate masters who shaped \
            living Hinduism from Adi Shankara through Ramakrishna and Vivekananda",
    },
    TraditionMapping {
        tradition: "Incarnate Buddhist",
        primary_civilization: "Indus Valley",
        civilization_names: &["Indus Valley", "Ancient China"],
        era_names: &["Maurya Period", "Tang Dynasty"],
        start_year: -300,
        end_year: i32::MAX,
        historical_note: "Buddhist masters who transmitted dharma across Asia — \
            Nagarjuna, Bodhidharma, Padmasambhava, Milarepa — \
            bridging Indian origins with Chinese, Tibetan, and Japanese forms",
    },
    TraditionMapping {
        tradition: "Incarnate Mystic",
        primary_civilization: "Persian Empire",
        civilization_names: &["Persian Empire", "Ottoman Empire"],
        era_names: &["Middle Ages"],
        start_year: 800,
        end_year: i32::MAX,
        historical_note: "Sufi and cross-tradition mystics — Rumi, Ibn Arabi, Kabir, \
            Meister Eckhart — who transcended sectarian boundaries to articulate \
            direct experiential union with the divine",
    },
    TraditionMapping {
        tradition: "Incarnate Taoist",
        primary_civilization: "Ancient China",
        civilization_names: &["Ancient China"],
        era_names: &["Han Dynasty", "Tang Dynasty", "Song Dynasty"],
        start_year: -400,
        end_year: i32::MAX,
        historical_note: "Taoist sages and alchemists — Zhang Daoling, Ge Hong, Sun Simiao — \
            who developed inner alchemy, medicine, and monastic traditions",
    },
    TraditionMapping {
        tradition: "Incarnate Indigenous",
        primary_civilization: "Tonga Empire",
        civilization_names: &["Tonga Empire", "Hawaiian Kingdom"],
        era_names: &["Middle Ages"],
        start_year: -500,
        end_year: i32::MAX,
        historical_note: "Indigenous spiritual leaders and wisdom keepers across traditions — \
            carriers of oral tradition, ceremony, and land-based spirituality",
    },
    TraditionMapping {
        tradition: "Incarnate Sage",
        primary_civilization: "Ancient Greece",
        civilization_names: &["Ancient Greece", "Ancient China"],
        era_names: &["Classical Antiquity", "Zhou Dynasty"],
        start_year: -600,
        end_year: i32::MAX,
        historical_note: "Philosopher-sages of the Axial Age and beyond — Pythagoras, Socrates, \
            Confucius — who laid the foundations of rational and ethical inquiry",
    },
];

/// Get the historical context for a tradition.
///
/// Returns context with civilizations and eras resolved from itihas data.
/// If a civilization or era name doesn't match itihas data (e.g., data
/// evolution), it is silently skipped — the mapping degrades gracefully.
#[must_use]
pub fn context_for_tradition(tradition: &str) -> HistoricalContext {
    let mapping = MAPPINGS.iter().find(|m| m.tradition == tradition);

    match mapping {
        Some(m) => {
            let civilizations: Vec<Civilization> = m
                .civilization_names
                .iter()
                .filter_map(|name| civilization::by_name(name).ok())
                .collect();

            let eras: Vec<Era> = m
                .era_names
                .iter()
                .filter_map(|name| era::by_name(name).ok())
                .collect();

            HistoricalContext {
                primary_civilization: m.primary_civilization.to_string(),
                civilizations,
                eras,
                start_year: m.start_year,
                end_year: m.end_year,
                historical_note: m.historical_note.to_string(),
            }
        }
        None => HistoricalContext {
            primary_civilization: String::new(),
            civilizations: Vec::new(),
            eras: Vec::new(),
            start_year: 0,
            end_year: 0,
            historical_note: format!("No historical mapping for tradition: {tradition}"),
        },
    }
}

/// Get historical context for an archetype profile.
///
/// Convenience wrapper that extracts the tradition from the profile
/// and resolves its historical context.
#[must_use]
pub fn context_for_profile(profile: &crate::ArchetypeProfile) -> HistoricalContext {
    context_for_tradition(&profile.tradition)
}

/// List all traditions that have historical mappings.
#[must_use]
pub fn mapped_traditions() -> Vec<&'static str> {
    MAPPINGS.iter().map(|m| m.tradition).collect()
}

/// Find all traditions associated with a given civilization name.
///
/// Case-sensitive match against itihas civilization names.
#[must_use]
pub fn traditions_for_civilization(civilization_name: &str) -> Vec<&'static str> {
    MAPPINGS
        .iter()
        .filter(|m| m.civilization_names.contains(&civilization_name))
        .map(|m| m.tradition)
        .collect()
}

/// Find all traditions active during a given year.
///
/// Uses the tradition's formative period, not the civilization's lifespan.
#[must_use]
pub fn traditions_active_at(year: i32) -> Vec<&'static str> {
    MAPPINGS
        .iter()
        .filter(|m| m.start_year <= year && year <= m.end_year)
        .map(|m| m.tradition)
        .collect()
}

/// Find all traditions associated with a given era name.
///
/// Case-sensitive match against itihas era names.
#[must_use]
pub fn traditions_for_era(era_name: &str) -> Vec<&'static str> {
    MAPPINGS
        .iter()
        .filter(|m| m.era_names.contains(&era_name))
        .map(|m| m.tradition)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hindu_has_indus_valley() {
        let ctx = context_for_tradition("Hindu");
        assert_eq!(ctx.primary_civilization, "Indus Valley");
        assert!(!ctx.civilizations.is_empty());
        assert!(
            ctx.civilizations
                .iter()
                .any(|c| c.name.as_ref() == "Indus Valley"),
        );
    }

    #[test]
    fn hindu_spans_vedic_to_mughal() {
        let ctx = context_for_tradition("Hindu");
        assert!(ctx.eras.iter().any(|e| e.name.as_ref() == "Vedic Period"));
        assert!(ctx.eras.iter().any(|e| e.name.as_ref() == "Mughal Period"));
    }

    #[test]
    fn norse_maps_to_viking() {
        let ctx = context_for_tradition("Norse");
        assert_eq!(ctx.primary_civilization, "Viking/Norse");
        assert!(
            ctx.civilizations
                .iter()
                .any(|c| c.name.as_ref() == "Viking/Norse"),
        );
    }

    #[test]
    fn egyptian_spans_millennia() {
        let ctx = context_for_tradition("Egyptian");
        assert!(ctx.start_year <= -3000);
        assert!(ctx.end_year >= 300);
    }

    #[test]
    fn unknown_tradition_returns_empty() {
        let ctx = context_for_tradition("Nonexistent");
        assert!(ctx.civilizations.is_empty());
        assert!(ctx.eras.is_empty());
    }

    #[test]
    fn all_mapped_traditions_resolve() {
        for tradition in mapped_traditions() {
            let ctx = context_for_tradition(tradition);
            assert!(
                !ctx.civilizations.is_empty(),
                "{tradition} has no resolved civilizations",
            );
        }
    }

    #[test]
    fn traditions_for_civilization_finds_hindu() {
        let ts = traditions_for_civilization("Indus Valley");
        assert!(ts.contains(&"Hindu"));
        assert!(ts.contains(&"Jain"));
        assert!(ts.contains(&"Buddhist"));
    }

    #[test]
    fn traditions_active_at_500bce() {
        let ts = traditions_active_at(-500);
        assert!(ts.contains(&"Hindu"));
        assert!(ts.contains(&"Egyptian"));
        assert!(ts.contains(&"Mesopotamian"));
        assert!(!ts.contains(&"Sikh"));
    }

    #[test]
    fn traditions_for_era_finds_tang() {
        let ts = traditions_for_era("Tang Dynasty");
        assert!(ts.contains(&"Taoist"));
        assert!(ts.contains(&"Buddhist"));
    }

    #[test]
    fn context_for_profile_works() {
        let profile = crate::registry::lookup("Krishna").unwrap();
        let ctx = context_for_profile(&profile);
        assert_eq!(ctx.primary_civilization, "Indus Valley");
    }

    #[test]
    fn living_traditions_have_max_end_year() {
        for tradition in &["Hindu", "Buddhist", "Sikh", "Jain", "Shinto", "Zoroastrian"] {
            let ctx = context_for_tradition(tradition);
            assert_eq!(
                ctx.end_year,
                i32::MAX,
                "{tradition} should be marked as living tradition",
            );
        }
    }
}
