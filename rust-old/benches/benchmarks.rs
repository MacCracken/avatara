use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

use avatara::Archetype;

fn bench_kabbalah(c: &mut Criterion) {
    use avatara::kabbalah::Sephira;

    let mut group = c.benchmark_group("kabbalah");
    group.bench_function("single_profile", |b| {
        b.iter(|| black_box(Sephira::Tiphareth).profile())
    });
    group.bench_function("all_profiles", |b| {
        b.iter(|| {
            for s in Sephira::ALL {
                black_box(s.profile());
            }
        })
    });
    group.finish();
}

fn bench_angelic(c: &mut Criterion) {
    use avatara::angelic::{AngelicOrder, Archangel};

    let mut group = c.benchmark_group("angelic");
    group.bench_function("all_archangels", |b| {
        b.iter(|| {
            for a in Archangel::ALL {
                black_box(a.profile());
            }
        })
    });
    group.bench_function("all_orders", |b| {
        b.iter(|| {
            for o in AngelicOrder::ALL {
                black_box(o.profile());
            }
        })
    });
    group.finish();
}

fn bench_hindu(c: &mut Criterion) {
    use avatara::hindu::{Avatar, Deva, Trimurti};

    let mut group = c.benchmark_group("hindu");
    group.bench_function("all_trimurti", |b| {
        b.iter(|| {
            for t in Trimurti::ALL {
                black_box(t.profile());
            }
        })
    });
    group.bench_function("all_devas", |b| {
        b.iter(|| {
            for d in Deva::ALL {
                black_box(d.profile());
            }
        })
    });
    group.bench_function("all_avatars", |b| {
        b.iter(|| {
            for a in Avatar::ALL {
                black_box(a.profile());
            }
        })
    });
    group.finish();
}

fn bench_olympian(c: &mut Criterion) {
    use avatara::olympian::Olympian;

    c.bench_function("olympian/all_profiles", |b| {
        b.iter(|| {
            for o in Olympian::ALL {
                black_box(o.profile());
            }
        })
    });
}

fn bench_norse(c: &mut Criterion) {
    use avatara::norse::NorseGod;

    c.bench_function("norse/all_profiles", |b| {
        b.iter(|| {
            for g in NorseGod::ALL {
                black_box(g.profile());
            }
        })
    });
}

fn bench_egyptian(c: &mut Criterion) {
    use avatara::egyptian::EgyptianGod;

    c.bench_function("egyptian/all_profiles", |b| {
        b.iter(|| {
            for g in EgyptianGod::ALL {
                black_box(g.profile());
            }
        })
    });
}

fn bench_buddhist(c: &mut Criterion) {
    use avatara::buddhist::{Bodhisattva, DhyaniBuddha};

    let mut group = c.benchmark_group("buddhist");
    group.bench_function("all_bodhisattvas", |b| {
        b.iter(|| {
            for b_ in Bodhisattva::ALL {
                black_box(b_.profile());
            }
        })
    });
    group.bench_function("all_dhyani_buddhas", |b| {
        b.iter(|| {
            for d in DhyaniBuddha::ALL {
                black_box(d.profile());
            }
        })
    });
    group.finish();
}

fn bench_mesopotamian(c: &mut Criterion) {
    use avatara::mesopotamian::MesopotamianGod;

    c.bench_function("mesopotamian/all_profiles", |b| {
        b.iter(|| {
            for g in MesopotamianGod::ALL {
                black_box(g.profile());
            }
        })
    });
}

fn bench_celtic(c: &mut Criterion) {
    use avatara::celtic::CelticGod;

    c.bench_function("celtic/all_profiles", |b| {
        b.iter(|| {
            for g in CelticGod::ALL {
                black_box(g.profile());
            }
        })
    });
}

fn bench_shinto(c: &mut Criterion) {
    use avatara::shinto::Kami;

    c.bench_function("shinto/all_profiles", |b| {
        b.iter(|| {
            for k in Kami::ALL {
                black_box(k.profile());
            }
        })
    });
}

fn bench_aztec(c: &mut Criterion) {
    use avatara::aztec::AztecGod;

    c.bench_function("aztec/all_profiles", |b| {
        b.iter(|| {
            for g in AztecGod::ALL {
                black_box(g.profile());
            }
        })
    });
}

fn bench_maya(c: &mut Criterion) {
    use avatara::maya::MayanGod;

    c.bench_function("maya/all_profiles", |b| {
        b.iter(|| {
            for g in MayanGod::ALL {
                black_box(g.profile());
            }
        })
    });
}

fn bench_yoruba(c: &mut Criterion) {
    use avatara::yoruba::Orisha;

    c.bench_function("yoruba/all_profiles", |b| {
        b.iter(|| {
            for o in Orisha::ALL {
                black_box(o.profile());
            }
        })
    });
}

fn bench_zoroastrian(c: &mut Criterion) {
    use avatara::zoroastrian::{AmeshaSpentas, ZoroastrianBeing};

    let mut group = c.benchmark_group("zoroastrian");
    group.bench_function("all_amesha_spentas", |b| {
        b.iter(|| {
            for a in AmeshaSpentas::ALL {
                black_box(a.profile());
            }
        })
    });
    group.bench_function("all_beings", |b| {
        b.iter(|| {
            for z in ZoroastrianBeing::ALL {
                black_box(z.profile());
            }
        })
    });
    group.finish();
}

fn bench_taoist(c: &mut Criterion) {
    use avatara::taoist::{Immortal, TaoistDeity};

    let mut group = c.benchmark_group("taoist");
    group.bench_function("all_immortals", |b| {
        b.iter(|| {
            for i in Immortal::ALL {
                black_box(i.profile());
            }
        })
    });
    group.bench_function("all_deities", |b| {
        b.iter(|| {
            for d in TaoistDeity::ALL {
                black_box(d.profile());
            }
        })
    });
    group.finish();
}

fn bench_incarnate(c: &mut Criterion) {
    use avatara::incarnate::{
        IncarnateBuddhist, IncarnateHindu, IncarnateIndigenous, IncarnateMystic, IncarnateTaoist,
    };

    let mut group = c.benchmark_group("incarnate");
    group.bench_function("all_hindu", |b| {
        b.iter(|| {
            for i in IncarnateHindu::ALL {
                black_box(i.profile());
            }
        })
    });
    group.bench_function("all_buddhist", |b| {
        b.iter(|| {
            for i in IncarnateBuddhist::ALL {
                black_box(i.profile());
            }
        })
    });
    group.bench_function("all_mystic", |b| {
        b.iter(|| {
            for i in IncarnateMystic::ALL {
                black_box(i.profile());
            }
        })
    });
    group.bench_function("all_taoist", |b| {
        b.iter(|| {
            for i in IncarnateTaoist::ALL {
                black_box(i.profile());
            }
        })
    });
    group.bench_function("all_indigenous", |b| {
        b.iter(|| {
            for i in IncarnateIndigenous::ALL {
                black_box(i.profile());
            }
        })
    });
    group.finish();
}

fn bench_polynesian(c: &mut Criterion) {
    use avatara::polynesian::PolynesianGod;

    c.bench_function("polynesian/all_profiles", |b| {
        b.iter(|| {
            for g in PolynesianGod::ALL {
                black_box(g.profile());
            }
        })
    });
}

fn bench_slavic(c: &mut Criterion) {
    use avatara::slavic::SlavicGod;

    c.bench_function("slavic/all_profiles", |b| {
        b.iter(|| {
            for g in SlavicGod::ALL {
                black_box(g.profile());
            }
        })
    });
}

fn bench_jain(c: &mut Criterion) {
    use avatara::jain::Tirthankara;

    c.bench_function("jain/all_profiles", |b| {
        b.iter(|| {
            for t in Tirthankara::ALL {
                black_box(t.profile());
            }
        })
    });
}

fn bench_sikh(c: &mut Criterion) {
    use avatara::sikh::Guru;

    c.bench_function("sikh/all_profiles", |b| {
        b.iter(|| {
            for g in Guru::ALL {
                black_box(g.profile());
            }
        })
    });
}

/// Benchmark generating every single profile in the library via registry.
fn bench_all_traditions(c: &mut Criterion) {
    c.bench_function("all_traditions/every_profile", |b| {
        b.iter(|| {
            for p in avatara::registry::all_profiles() {
                black_box(p);
            }
        })
    });
}

fn bench_registry(c: &mut Criterion) {
    use avatara::registry;

    let mut group = c.benchmark_group("registry");
    group.bench_function("all_profiles", |b| {
        b.iter(|| black_box(registry::all_profiles()))
    });
    group.bench_function("lookup_by_name", |b| {
        b.iter(|| black_box(registry::lookup("Krishna")))
    });
    group.bench_function("query_courage_09", |b| {
        b.iter(|| black_box(registry::query().min_trait(|t| t.courage, 0.9).collect()))
    });
    group.finish();
}

fn bench_compose(c: &mut Criterion) {
    use avatara::compose::compose;
    use avatara::hindu::Trimurti;
    use avatara::kabbalah::Sephira;
    use avatara::olympian::Olympian;

    let profiles = vec![
        (Sephira::Tiphareth.profile(), 1.0),
        (Trimurti::Vishnu.profile(), 0.8),
        (Olympian::Athena.profile(), 0.6),
    ];

    c.bench_function("compose/three_traditions", |b| {
        b.iter(|| black_box(compose(&profiles)))
    });
}

#[cfg(feature = "itihas")]
fn bench_history(c: &mut Criterion) {
    use avatara::history;

    let mut group = c.benchmark_group("history");
    group.bench_function("context_for_tradition", |b| {
        b.iter(|| black_box(history::context_for_tradition("Hindu")))
    });
    group.bench_function("context_all_traditions", |b| {
        b.iter(|| {
            for t in history::mapped_traditions() {
                black_box(history::context_for_tradition(t));
            }
        })
    });
    group.bench_function("traditions_for_civilization", |b| {
        b.iter(|| black_box(history::traditions_for_civilization("Indus Valley")))
    });
    group.bench_function("traditions_for_era", |b| {
        b.iter(|| black_box(history::traditions_for_era("Tang Dynasty")))
    });
    group.bench_function("traditions_active_at", |b| {
        b.iter(|| black_box(history::traditions_active_at(-500)))
    });
    group.finish();
}

#[cfg(feature = "itihas")]
fn bench_registry_itihas(c: &mut Criterion) {
    use avatara::registry;

    let mut group = c.benchmark_group("registry_itihas");
    group.bench_function("query_by_civilization", |b| {
        b.iter(|| black_box(registry::query().civilization("Ancient Greece").collect()))
    });
    group.bench_function("query_by_era", |b| {
        b.iter(|| black_box(registry::query().era("Vedic Period").collect()))
    });
    group.bench_function("query_active_at", |b| {
        b.iter(|| black_box(registry::query().active_at(-500).collect()))
    });
    group.bench_function("query_civilization_plus_trait", |b| {
        b.iter(|| {
            black_box(
                registry::query()
                    .civilization("Indus Valley")
                    .min_trait(|t| t.empathy, 0.8)
                    .collect(),
            )
        })
    });
    group.finish();
}

#[cfg(not(feature = "itihas"))]
criterion_group!(
    benches,
    bench_kabbalah,
    bench_angelic,
    bench_hindu,
    bench_olympian,
    bench_norse,
    bench_egyptian,
    bench_buddhist,
    bench_mesopotamian,
    bench_celtic,
    bench_shinto,
    bench_aztec,
    bench_maya,
    bench_yoruba,
    bench_zoroastrian,
    bench_taoist,
    bench_incarnate,
    bench_polynesian,
    bench_slavic,
    bench_jain,
    bench_sikh,
    bench_all_traditions,
    bench_registry,
    bench_compose,
);

#[cfg(feature = "itihas")]
criterion_group!(
    benches,
    bench_kabbalah,
    bench_angelic,
    bench_hindu,
    bench_olympian,
    bench_norse,
    bench_egyptian,
    bench_buddhist,
    bench_mesopotamian,
    bench_celtic,
    bench_shinto,
    bench_aztec,
    bench_maya,
    bench_yoruba,
    bench_zoroastrian,
    bench_taoist,
    bench_incarnate,
    bench_polynesian,
    bench_slavic,
    bench_jain,
    bench_sikh,
    bench_all_traditions,
    bench_registry,
    bench_compose,
    bench_history,
    bench_registry_itihas,
);

criterion_main!(benches);
