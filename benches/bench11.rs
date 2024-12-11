use daylibs::*;

fn main() {
    divan::main();
}

#[divan::bench]
fn part_1(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| (Day11, get_input(11).unwrap()))
        .bench_values(|(mut day, input)| {
            day.part_1(divan::black_box(input));
        });
}

#[cfg(feature = "all-benches")]
#[divan::bench]
fn part_2(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| (Day11, get_input(11).unwrap()))
        .bench_values(|(mut day, input)| {
            day.part_2(divan::black_box(input));
        });
}
