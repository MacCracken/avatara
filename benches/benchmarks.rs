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

/// Benchmark generating every single profile in the library.
fn bench_all_traditions(c: &mut Criterion) {
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

    c.bench_function("all_traditions/every_profile", |b| {
        b.iter(|| {
            for s in Sephira::ALL { black_box(s.profile()); }
            for a in Archangel::ALL { black_box(a.profile()); }
            for o in AngelicOrder::ALL { black_box(o.profile()); }
            for t in Trimurti::ALL { black_box(t.profile()); }
            for d in Deva::ALL { black_box(d.profile()); }
            for a in Avatar::ALL { black_box(a.profile()); }
            for o in Olympian::ALL { black_box(o.profile()); }
            for g in NorseGod::ALL { black_box(g.profile()); }
            for g in EgyptianGod::ALL { black_box(g.profile()); }
            for b_ in Bodhisattva::ALL { black_box(b_.profile()); }
            for d in DhyaniBuddha::ALL { black_box(d.profile()); }
            for g in MesopotamianGod::ALL { black_box(g.profile()); }
            for g in CelticGod::ALL { black_box(g.profile()); }
            for k in Kami::ALL { black_box(k.profile()); }
            for g in AztecGod::ALL { black_box(g.profile()); }
            for g in MayanGod::ALL { black_box(g.profile()); }
            for o in Orisha::ALL { black_box(o.profile()); }
            for a in AmeshaSpentas::ALL { black_box(a.profile()); }
            for z in ZoroastrianBeing::ALL { black_box(z.profile()); }
            for i in Immortal::ALL { black_box(i.profile()); }
            for d in TaoistDeity::ALL { black_box(d.profile()); }
            for i in IncarnateHindu::ALL { black_box(i.profile()); }
            for i in IncarnateBuddhist::ALL { black_box(i.profile()); }
            for i in IncarnateMystic::ALL { black_box(i.profile()); }
            for i in IncarnateTaoist::ALL { black_box(i.profile()); }
            for i in IncarnateIndigenous::ALL { black_box(i.profile()); }
        })
    });
}

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
    bench_all_traditions,
);
criterion_main!(benches);
