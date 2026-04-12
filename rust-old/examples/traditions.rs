//! Explore archetypes across traditions.

use avatara::Archetype;
use avatara::hindu::Trimurti;
use avatara::kabbalah::Sephira;
use avatara::registry;

fn main() {
    // Single archetype profile
    let tiphareth = Sephira::Tiphareth.profile();
    println!("{} ({})", tiphareth.name, tiphareth.tradition);
    println!("  {}", tiphareth.description);
    println!(
        "  warmth={:.1} confidence={:.1} courage={:.1}",
        tiphareth.traits.warmth, tiphareth.traits.confidence, tiphareth.traits.courage
    );
    println!(
        "  breath={:?} growth={:?}",
        tiphareth.breath, tiphareth.growth
    );
    println!(
        "  element={:?} polarity={:?} tier={:?}",
        tiphareth.element, tiphareth.polarity, tiphareth.tier
    );
    println!();

    // Compare across traditions
    println!("Cosmic breath positions:");
    for t in Trimurti::ALL {
        let p = t.profile();
        println!(
            "  {} — {:?} (intensity {:.2})",
            p.name,
            p.breath,
            p.breath.intensity()
        );
    }
    println!();

    // Query the registry
    let brave = registry::query().min_trait(|t| t.courage, 0.9).collect();
    println!("{} entities with courage >= 0.9:", brave.len());
    for p in &brave {
        println!(
            "  {} ({}) — courage={:.2}",
            p.name, p.tradition, p.traits.courage
        );
    }
    println!();

    // All traditions
    let traditions = registry::traditions();
    let total = registry::all_profiles().len();
    println!(
        "{total} entities across {n} traditions:",
        n = traditions.len()
    );
    for t in &traditions {
        let count = registry::by_tradition(t).len();
        println!("  {t}: {count}");
    }
}
