use daylibs::*;

fn main() {
    divan::main();
}

#[divan::bench]
fn part_1(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| (Day13, get_input(13).unwrap()))
        .bench_values(|(mut day, input)| {
            day.part_1(divan::black_box(input));
        });
}

#[divan::bench]
fn part_2(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| (Day13, get_input(13).unwrap()))
        .bench_values(|(mut day, input)| {
            day.part_2(divan::black_box(input));
        });
}
