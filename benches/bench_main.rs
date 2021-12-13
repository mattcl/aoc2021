use criterion::criterion_main;

mod day_001;
mod day_002;
mod day_003;
mod day_004;
mod day_005;
mod day_006;
mod day_007;
mod day_008;
mod day_009;
mod day_010;
mod day_011;
mod day_012;
mod day_013;

criterion_main! {
    day_001::benches,
    day_002::benches,
    day_003::benches,
    day_004::benches,
    day_005::benches,
    day_006::benches,
    day_007::benches,
    day_008::benches,
    day_009::benches,
    day_010::benches,
    day_011::benches,
    day_012::benches,
    day_013::benches,
}
