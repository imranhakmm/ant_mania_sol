use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <num_ants>", args[0]);
        return;
    }
    let n: usize = args[1].parse().expect("Invalid number");
    let elapsed = ant_mania_sol::run_simulation_phase(n);
    println!("Simulation phase latency: {:?}", elapsed);
}

