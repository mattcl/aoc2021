use criterion::criterion_main;

mod day_001;
mod day_002;

criterion_main! {
    day_001::benches,
    day_002::benches,
}
