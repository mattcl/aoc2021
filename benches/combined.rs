use aoc::{
    bingo::{Board, Runner},
    camera::Manual,
    cave::CaveSystem,
    chiton::{ChitonGrid, Pathfinding},
    crab::{ArithmeticSub, LinearSub, Swarm},
    decoder::Transmission,
    diagnostic::Diagnostic,
    fish::{Homework, Sim},
    generic::prelude::*,
    heightmap::HeightMap,
    navigation::Program,
    octopus::OctopusGrid,
    polymer::Polymerizer,
    probe::{Launcher, Target},
    scanner::Mapper,
    sonar::Report,
    ssd::Solver,
    submarine::{AimableSubmarine, Moveable, Submarine},
    util::{load_input, parse_input},
    vents::Vents,
};
use criterion::{black_box, criterion_group, Criterion};
use rustc_hash::FxHashSet;
use std::{convert::TryFrom, str::FromStr, time::Duration};

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("AoC Twenty Twenty-One");
    group.measurement_time(Duration::new(10, 0));
    group.bench_function("All Problems", |b| {
        b.iter(|| {
            // 001
            let lines = load_input("001").expect("could not load input");
            let report = Report::try_from(lines).expect("invalid input");
            report.count_increases();
            report.count_increases();

            // 002
            let lines = load_input("002").expect("could not load input");
            let commands = parse_input(&lines).expect("invalid input");
            let mut sub = Submarine::new();
            for command in &commands {
                sub.execute(command);
            }
            sub.location_hash();

            let mut sub = AimableSubmarine::new();
            for command in &commands {
                sub.execute(command);
            }
            sub.location_hash();

            // 003
            let lines = load_input("003").expect("could not load input");
            let d = Diagnostic::try_from(&lines).expect("Could not create diagnostic");
            d.power_consumption();
            d.life_support_rating()
                .expect("Could not get life support rating");

            // 004
            let lines = load_input("004").expect("could not load input");
            let mut runner: Runner<Board> = Runner::try_from(lines).expect("Input was invalid");
            runner.play().expect("Could not find a winner");
            runner
                .par_find_last_scoring()
                .expect("Could not find a winner");

            // 005
            let lines = load_input("005").expect("could not load input");
            let mut grid = Vents::try_from(&lines).expect("Could not construct grid");
            grid.count_multi_overlap();
            grid.prune_diagonal();
            grid.count_multi_overlap();

            // 006
            let lines = load_input("006").expect("could not load input");
            let line = lines.first().expect("Input was empty");
            let sim = Sim::from_str(&line).expect("Could not make sim");
            sim.fast_population_after(black_box(80));
            sim.fast_population_after(black_box(256));

            // 007
            let lines = load_input("007").expect("could not load input");
            let line = lines.first().expect("input was empty");
            let swarm: Swarm<LinearSub> = Swarm::from_str(&line).expect("Could not make swarm");
            swarm.cheapest_expenditure();

            let swarm: Swarm<ArithmeticSub> = Swarm::from_str(&line).expect("Could not make swarm");
            swarm.cheapest_expenditure();

            // 008
            let lines = load_input("008").expect("could not load input");
            let solver = Solver::try_from(lines).expect("Could not parse input");
            solver.rhs_count_known();
            solver
                .par_rhs_values_sum()
                .expect("could not find solution");

            // 009
            let lines = load_input("009").expect("could not load input");
            let hm = HeightMap::try_from(lines).expect("could not parse heightmap");
            hm.total_risk();
            hm.largest_basins().expect("coudl not find largest basins");

            // 010
            let input = load_input("010").expect("could not load input");
            let program = Program::from(parse_input(&input).expect("could not parse input"));
            let check = program.check();
            check.score_corruptions();
            check.score_completions();

            // 011
            let lines = load_input("011").expect("could not load input");
            let mut grid = OctopusGrid::try_from(lines).expect("could not parse input");
            grid.simulate(black_box(100));
            grid.simulate_until_sync();

            // 012
            let lines = load_input("012").expect("could not load input");
            let cave_system = CaveSystem::try_from(lines).expect("could not parse input");
            cave_system
                .paths_semi_par(false)
                .expect("could not find paths");
            cave_system
                .paths_semi_par(true)
                .expect("could not find paths");

            // 013
            let lines = load_input("013").expect("could not load input");
            let manual = Manual::try_from(lines).expect("could not parse input");
            manual.first_instruction().count_visible();
            manual.folded().to_string();

            // 014
            let lines = load_input("014").expect("could not load input");
            let poly = Polymerizer::try_from(lines).expect("could not parse input");
            poly.iterations_fast(10);
            poly.iterations_fast(40);

            // 015
            let lines = load_input("015").expect("could not load input");
            let grid = ChitonGrid::try_from(lines).expect("could not parse input");
            let scale = 1;
            grid.shortest(scale, &grid.top_left(), &grid.bottom_right())
                .expect("could not find path");
            let scale = 5;
            grid.shortest(scale, &grid.top_left(), &grid.scaled_bottom_right(scale))
                .expect("could not find path");

            // 016
            let line = load_input("016")
                .expect("could not load input")
                .first()
                .cloned()
                .expect("input was empty");

            let transmission = Transmission::from_str(&line).expect("could not parse input");
            transmission.version_sum();
            transmission.value();

            // 017
            let line = load_input("017")
                .expect("could not load input")
                .first()
                .cloned()
                .expect("input was empty");

            let target = Target::from_str(&line).expect("could not parse input");
            let launcher = Launcher {};
            launcher.launch(&target);

            // 018
            let lines = load_input("018").expect("could not load input");
            let homework = Homework::try_from(lines).expect("could not parse input");
            homework.sum().expect("could not find sum");
            homework
                .largest_magnitude_of_pairs()
                .expect("could not find magnitude");

            // 019
            let lines = load_input("019").expect("could not load input");
            let mut mapper = Mapper::try_from(lines).expect("could not parse input");
            let mut beacons = FxHashSet::default();
            mapper.correlate(&mut beacons);
            mapper
                .largest_distance()
                .expect("could not find largest distance");
        })
    });
    group.finish();
}

criterion_group!(benches, bench);
