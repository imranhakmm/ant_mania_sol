use criterion::{criterion_group, criterion_main, Criterion};
use ant_mania_sol::{phase_movement, phase_collision, phase_cleanup, build_world, init_ants, run_simulation_phase};

fn bench_movement_phase(c: &mut Criterion) {
    c.bench_function("movement phase", |b| {
        b.iter(|| {
            let world = build_world();
            let mut ants = init_ants(&world, 100);
            phase_movement(&mut ants, &world);
        })
    });
}

fn bench_collision_phase(c: &mut Criterion) {
    c.bench_function("collision detection phase", |b| {
        b.iter(|| {
            let world = build_world();
            let mut ants = init_ants(&world, 100);
            phase_movement(&mut ants, &world);
            let _colony_ants = phase_collision(&ants, &world);
        })
    });
}

fn bench_cleanup_phase(c: &mut Criterion) {
    c.bench_function("cleanup phase", |b| {
        b.iter(|| {
            let world = build_world();
            let mut ants = init_ants(&world, 100);
            phase_movement(&mut ants, &world);
            let colony_ants = phase_collision(&ants, &world);
            // Clone world so that each iteration starts fresh.
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
    bench_movement_phase, 
    bench_collision_phase, 
    bench_cleanup_phase,
    bench_full_simulation
);
criterion_main!(benches);
