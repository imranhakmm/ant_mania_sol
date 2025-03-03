use criterion::{criterion_group, criterion_main, Criterion};
use ant_mania_sol::run_simulation_phase;

fn bench_simulation_phase(c: &mut Criterion) {
    c.bench_function("simulation phase big0", |b| {
        b.iter(|| run_simulation_phase(100))
    });
}

criterion_group!(benches, bench_simulation_phase);
criterion_main!(benches);
