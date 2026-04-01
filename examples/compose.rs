//! Compose archetypes from multiple traditions.

use avatara::Archetype;
use avatara::compose::compose;
use avatara::hindu::Trimurti;
use avatara::kabbalah::Sephira;
use avatara::olympian::Olympian;

fn main() {
    // Blend three traditions: Kabbalistic harmony + Hindu preservation + Greek wisdom
    let profile = compose(&[
        (Sephira::Tiphareth.profile(), 1.0),
        (Trimurti::Vishnu.profile(), 0.8),
        (Olympian::Athena.profile(), 0.6),
    ])
    .expect("valid composition");

    println!("Composite: {}", profile.name);
    println!("Traditions: {}", profile.tradition);
    println!();
    println!("Blended traits:");
    println!(
        "  warmth={:.2}    confidence={:.2}",
        profile.traits.warmth, profile.traits.confidence
    );
    println!(
        "  empathy={:.2}   patience={:.2}",
        profile.traits.empathy, profile.traits.patience
    );
    println!(
        "  precision={:.2} courage={:.2}",
        profile.traits.precision, profile.traits.courage
    );
    println!();
    println!(
        "Breath: {:?} (intensity {:.2})",
        profile.breath,
        profile.breath.intensity()
    );
    println!("Growth: {:?}", profile.growth);
    println!("Element: {:?}", profile.element);
    println!("Tier: {:?}", profile.tier);
    println!();
    println!("Soul text (from dominant):");
    println!("  {}", profile.soul_text);
}
