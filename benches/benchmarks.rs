use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn bench_kabbalah(c: &mut Criterion) {
    use avatara::Archetype;
    use avatara::kabbalah::Sephira;

    let mut group = c.benchmark_group("kabbalah");
    group.bench_function("sephira_profile", |b| {
        b.iter(|| black_box(Sephira::Tiphareth).profile())
    });
    group.finish();
}

criterion_group!(benches, bench_kabbalah);
criterion_main!(benches);
