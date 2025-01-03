use std::ops::BitXor;

use itertools::Itertools;

use crate::Day;

pub struct Day22;

impl Day for Day22 {
    fn example_input(&self) -> (&'static str, &'static str) {
        (
            r#"1
10
100
2024"#,
            "",
        )
    }

    fn example_solution(&self) -> (&'static str, &'static str) {
        ("37327623", "")
    }

    fn part_1(&mut self, input: String) -> String {
        input
            .lines()
            .map(|num| num.parse::<u64>().expect("should be numbers"))
            .map(|secret| find_secret_after(secret, 2000))
            .sum::<u64>()
            .to_string()
    }

    fn part_2(&mut self, input: String) -> String {
        let all_deltas = input
            .lines()
            .map(|num| num.parse::<u64>().expect("should be numbers"))
            .map(|secret| find_deltas_after(secret, 2000))
            .collect_vec();

        todo!()
    }
}

fn find_secret_after(secret: u64, cycles: usize) -> u64 {
    (0..cycles).fold(secret, |current, _| find_next_secret(current))
}

fn find_deltas_after(secret: u64, cycles: usize) -> Vec<(u64, i32)> {
    let mut deltas = vec![(secret % 10, 0)];

    _ = (0..cycles).fold(secret, |current, _| {
        let this_secret = find_next_secret(current);

        let digit = this_secret % 10;
        deltas.push((digit, deltas.last().unwrap().0 as i32 - digit as i32));

        this_secret
    });

    deltas
}

fn find_next_secret(secret: u64) -> u64 {
    let secret = prune(mix(secret, secret * 64));
    let secret = prune(mix(secret, secret / 32));
    prune(mix(secret, secret * 2048))
}

fn mix(secret: u64, mixing_num: u64) -> u64 {
    secret.bitxor(mixing_num)
}

fn prune(secret: u64) -> u64 {
    secret % 16777216
}
