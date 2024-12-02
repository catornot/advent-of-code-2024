use daylibs::*;

fn main() {
    divan::main();
}

#[divan::bench]
fn part_1(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| (Day2, get_input(2).unwrap()))
        .bench_values(|(mut day, input)| {
            divan::black_box(day.part_1(input));
        });
}

#[divan::bench]
fn part_2(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| (Day2, get_input(2).unwrap()))
        .bench_values(|(mut day, input)| {
            divan::black_box(day.part_2(input));
        });
}
