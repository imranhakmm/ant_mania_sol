use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant;
use rand::prelude::*;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use rayon::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

pub struct ColonyData {
    pub outgoing: Vec<(Direction, usize)>,
}

pub struct World {
    pub colonies: Vec<ColonyData>,
    pub alive: Vec<bool>,
    pub id_to_name: Vec<String>,
    pub colony_name_to_id: HashMap<String, usize>,
    pub reverse_links: HashMap<usize, HashSet<(usize, Direction)>>,
}

impl World {
    #[inline(always)]
    pub fn new() -> Self {
        World {
            colonies: Vec::new(),
            alive: Vec::new(),
            id_to_name: Vec::new(),
            colony_name_to_id: HashMap::new(),
            reverse_links: HashMap::new(),
        }
    }

    #[inline(always)]
    pub fn get_or_add_colony(&mut self, name: String) -> usize {
        if let Some(&id) = self.colony_name_to_id.get(&name) {
            id
        } else {
            let id = self.colonies.len();
            self.colony_name_to_id.insert(name.clone(), id);
            self.id_to_name.push(name);
            self.colonies.push(ColonyData { outgoing: Vec::new() });
            self.alive.push(true);
            id
        }
    }

    #[inline(always)]
    pub fn add_direction(&mut self, from: usize, dir: Direction, to: usize) {
        self.colonies[from].outgoing.push((dir, to));
        self.reverse_links.entry(to)
            .or_insert_with(HashSet::new)
            .insert((from, dir));
    }

    #[inline(always)]
    pub fn destroy_colony(&mut self, y: usize) {
        if !self.alive[y] {
            return;
        }
        self.alive[y] = false;
        if let Some(incoming) = self.reverse_links.remove(&y) {
            for (x, dir) in incoming {
                self.colonies[x].outgoing.retain(|&(d, t)| !(d == dir && t == y));
            }
        }
        let outgoing = self.colonies[y].outgoing.clone();
        for (dir, z) in outgoing {
            if let Some(links) = self.reverse_links.get_mut(&z) {
                links.remove(&(y, dir));
            }
        }
        self.colonies[y].outgoing.clear();
    }
}

/// Prints the final world state in the same format as the input.
pub fn print_world(world: &World) {
    for id in 0..world.colonies.len() {
        if world.alive[id] {
            let name = &world.id_to_name[id];
            let directions = &world.colonies[id].outgoing;
            if directions.is_empty() {
                println!("{}", name);
            } else {
                let parts: Vec<String> = directions.iter().map(|(dir, target)| {
                    let d = match dir {
                        Direction::North => "north",
                        Direction::South => "south",
                        Direction::East  => "east",
                        Direction::West  => "west",
                    };
                    format!("{}={}", d, world.id_to_name[*target])
                }).collect();
                println!("{} {}", name, parts.join(" "));
            }
        }
    }
}

pub struct Ant {
    pub current_colony: usize,
    pub moves: u32,
    pub alive: bool,
}

#[inline(always)]
pub fn update_ant(ant: &mut Ant, world: &World, rng: &mut SmallRng) {
    if !world.alive[ant.current_colony] {
        ant.alive = false;
        return;
    }
    ant.moves += 1;
    let outgoing = &world.colonies[ant.current_colony].outgoing;
    if !outgoing.is_empty() {
        let idx = rng.gen_range(0..outgoing.len());
        let (_, target_id) = outgoing[idx];
        ant.current_colony = target_id;
    }
}

/// Phase 1: Build the world from the map.
pub fn build_world() -> World {
    let mut world = World::new();
    let map = fs::read_to_string("ant_mania_map.txt")
        .expect("Failed to read ant_mania_map.txt");
    for line in map.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        let colony_name = parts[0].to_string();
        let colony_id = world.get_or_add_colony(colony_name);
        for part in &parts[1..] {
            let (dir_str, target_name) = part.split_once('=')
                .expect("Invalid direction format");
            let dir = match dir_str {
                "north" => Direction::North,
                "south" => Direction::South,
                "east"  => Direction::East,
                "west"  => Direction::West,
                _       => panic!("Invalid direction: {}", dir_str),
            };
            let target_id = world.get_or_add_colony(target_name.to_string());
            world.add_direction(colony_id, dir, target_id);
        }
    }
    world
}

/// Phase 2: Initialize ants.
pub fn init_ants(world: &World, n_ants: usize) -> Vec<Ant> {
    let colony_ids: Vec<usize> = (0..world.colonies.len())
        .filter(|&id| world.alive[id])
        .collect();
    if colony_ids.is_empty() {
        panic!("No alive colonies to place ants.");
    }
    let mut ants = Vec::with_capacity(n_ants);
    let mut global_rng = SmallRng::seed_from_u64(12345);
    for _ in 0..n_ants {
        let colony_id = *colony_ids.choose(&mut global_rng).unwrap();
        ants.push(Ant {
            current_colony: colony_id,
            moves: 0,
            alive: true,
        });
    }
    ants
}

/// Phase A: Movement Phase.
pub fn phase_movement(ants: &mut Vec<Ant>, world: &World) {
    ants.par_iter_mut().enumerate().for_each(|(i, ant)| {
        let mut local_rng = SmallRng::seed_from_u64(i as u64 + 12345);
        update_ant(ant, world, &mut local_rng);
    });
}

/// Phase B: Collision Detection Phase.
/// Returns a vector of vectors containing indices of ants in each colony.
pub fn phase_collision(ants: &Vec<Ant>, world: &World) -> Vec<Vec<usize>> {
    let mut colony_ants: Vec<Vec<usize>> = vec![Vec::new(); world.colonies.len()];
    for (index, ant) in ants.iter().enumerate() {
        if ant.alive {
            colony_ants[ant.current_colony].push(index);
        }
    }
    colony_ants
}

/// Phase C: Cleanup Phase.
/// Processes collisions by marking ants as dead and destroying colonies.
pub fn phase_cleanup(ants: &mut Vec<Ant>, world: &mut World, colony_ants: &Vec<Vec<usize>>) {
    let mut destroyed_colonies = Vec::new();
    for (colony_id, ants_here) in colony_ants.iter().enumerate() {
        if ants_here.len() >= 2 {
            destroyed_colonies.push(colony_id);
            for &index in ants_here {
                ants[index].alive = false;
            }
            println!("{} has been destroyed by ant {} and ant {}!",
                     world.id_to_name[colony_id], ants_here[0], ants_here[1]);
        }
    }
    for colony_id in destroyed_colonies {
        world.destroy_colony(colony_id);
    }
}

/// Runs the simulation loop using the separated phases.
pub fn simulate(ants: &mut Vec<Ant>, world: &mut World) {
    loop {
        phase_movement(ants, world);
        let colony_ants = phase_collision(ants, world);
        phase_cleanup(ants, world, &colony_ants);
        let all_dead = ants.iter().all(|ant| !ant.alive);
        let all_moved_max = ants.iter().filter(|ant| ant.alive).all(|ant| ant.moves >= 10_000);
        if all_dead || all_moved_max {
            break;
        }
    }
}

/// Runs only the simulation phase (after world and ants are built)
/// and returns its execution time.
pub fn run_simulation_phase(n_ants: usize) -> std::time::Duration {
    let mut world = build_world();
    let mut ants = init_ants(&world, n_ants);
    let start = Instant::now();
    simulate(&mut ants, &mut world);
    let elapsed = start.elapsed();
    println!("Final world state:");
    print_world(&world);
    elapsed
}
