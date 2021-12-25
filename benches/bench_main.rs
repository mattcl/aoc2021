use aoc::{
    alu::PrecompiledSolver,
    amphipod::Amphipod,
    bingo::{FastBoard, Runner},
    camera::Manual,
    cave::CaveSystem,
    chiton::ChitonGrid,
    crab::Crabs,
    cucumber::Cucumber,
    decoder::TransmissionWrapper,
    diagnostic::DiagnosticWrapper,
    dirac::Games,
    fish::{Homework, Sim},
    heightmap::HeightMap,
    navigation::Program,
    octopus::OctopusGrid,
    polymer::Polymerizer,
    probe::Launcher,
    reactor::Procedure,
    scanner::Mapper,
    sonar::Report,
    ssd::Matcher,
    submarine::Subs,
    trench::Enhancer,
    vents::Vents,
};

use aoc_helpers::{aoc_benches, Solver};
use criterion::criterion_main;

aoc_benches! {
    20,
    (
        day_001,
        Report,
        "part 1 counting increases",
        "part 2 counting windowed increases"
    ),
    (
        day_002,
        Subs,
        "part 1 submarine movement",
        "part 2 aimable submarine movement"
    ),
    (
        day_003,
        DiagnosticWrapper,
        "part 1 power consumption",
        "part 2 life support rating"
    ),
    (
        day_004,
        Runner<FastBoard>,
        "part 1 first to score",
        "part 2 last to score"
    ),
    (
        day_005,
        Vents,
        "part 1 only horizontal and vertical",
        "part 2 including diagonals"
    ),
    (
        day_006,
        Sim,
        "part 1 population after 80 days",
        "part 2 population after 256 days"
    ),
    (
        day_007,
        Crabs,
        "part 1 linear cost",
        "part 2 arithmetic cost"
    ),
    (
        day_008,
        Matcher,
        "part 1 unambiguous count",
        "part 2 determining digits"
    ),
    (
        day_009,
        HeightMap,
        "part 1 finding lowpoints",
        "part 2 finding largest basins"
    ),
    (
        day_010,
        Program,
        "part 1 score corrupted",
        "part 2 score completions"
    ),
    (
        day_011,
        OctopusGrid,
        "part 1 simulating 100 steps",
        "part 2 finding the first sync step"
    ),
    (
        day_012,
        CaveSystem,
        "part 1 visiting small caves at most once",
        "part 2 allowing a single double visit"
    ),
    (
        day_013,
        Manual,
        "part 1 first fold",
        "part 2 folding to completion"
    ),
    (
        day_014,
        Polymerizer,
        "part 1 10 iterations",
        "part 2 40 iterations"
    ),
    (
        day_015,
        ChitonGrid,
        "part 1 path through normal grid",
        "part 2 path through enlarged grid"
    ),
    (
        day_016,
        TransmissionWrapper,
        "part 1 summing versions",
        "part 2 calculating value"
    ),
    (
        day_017,
        Launcher,
        "part 1 and 2 combined highest and distinct"
    ),
    (
        day_018,
        Homework,
        "part 1 finding magnitude of sum",
        "part 2 finding largest magnitude of any two elements"
    ),
    (
        day_019,
        Mapper,
        "part 1 and 2 combined solution"
    ),
    (
        day_020,
        Enhancer,
        "part 1 enhanced twice",
        "part 2 enhanced fifty times"
    ),
    (
        day_021,
        Games,
        "part 1 deterministic die",
        "part 2 dirac die"
    ),
    (
        day_022,
        Procedure,
        "part 1 region limited volume",
        "part 2 total volume"
    ),
    (
        day_023,
        Amphipod,
        "part 1 small burrow",
        "part 2 large burrow"
    ),
    // So I don't know how I feel about the solution for day 24 here, since it's
    // specifically solving inputs with the exact format of the MONAD program
    (
        day_024,
        PrecompiledSolver,
        "part 1 largest valid value",
        "part 2 smallest valid value"
    ),
    (
        day_025,
        Cucumber,
        "steps until stabilization"
    )
}

criterion_main! {
    benches
}
