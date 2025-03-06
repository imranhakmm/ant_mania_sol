use criterion::{criterion_group, criterion_main, Criterion};
use ant_mania_sol::run_simulation_phase;

fn bench_full_simulation(c: &mut Criterion) {
    c.bench_function("full simulation soa", |b| {
        b.iter(|| run_simulation_phase(100))
    });
}

criterion_group!(benches, bench_full_simulation);
criterion_main!(benches);
