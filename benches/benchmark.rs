use criterion::{criterion_group, criterion_main, Criterion};
use ant_mania_sol::{phase_move_and_detect, phase_cleanup, build_world, init_ants, run_simulation_phase};

fn bench_move_detect_phase(c: &mut Criterion) {
    c.bench_function("move and collision detection phase", |b| {
        b.iter(|| {
            let world = build_world();
            let mut ants = init_ants(&world, 100);
            let _ = phase_move_and_detect(&mut ants, &world);
        })
    });
}

fn bench_cleanup_phase(c: &mut Criterion) {
    c.bench_function("cleanup phase", |b| {
        b.iter(|| {
            let world = build_world();
            let mut ants = init_ants(&world, 100);
            let colony_ants = phase_move_and_detect(&mut ants, &world);
            let mut world_clone = world.clone();
            phase_cleanup(&mut ants, &mut world_clone, &colony_ants);
        })
    });
}

fn bench_full_simulation(c: &mut Criterion) {
    c.bench_function("full simulation", |b| {
        b.iter(|| run_simulation_phase(100))
    });
}

criterion_group!(
    benches,
    bench_move_detect_phase,
    bench_cleanup_phase,
    bench_full_simulation
);
criterion_main!(benches);
