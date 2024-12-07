use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::Day;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Op {
    Mul,
    Add,
    Concat,
}

pub struct Day7;

impl Day for Day7 {
    fn example_input(&self) -> (&'static str, &'static str) {
        let input = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;
        (input, input)
    }

    fn example_solution(&self) -> (&'static str, &'static str) {
        ("3749", "11387")
    }

    fn part_1(&mut self, input: String) -> String {
        parse_and_solve(&input, 2).to_string()
    }

    fn part_2(&mut self, input: String) -> String {
        parse_and_solve(&input, 3).to_string()
    }
}

fn parse_and_solve(input: &str, max_op: usize) -> u64 {
    input
        .lines()
        .map(|line| line.split_once(':').expect("how to be valid"))
        .map(|(result, numbers)| {
            (
                result
                    .trim()
                    .parse::<u64>()
                    .expect("has to be a valid number result"),
                numbers
                    .split_whitespace()
                    .map(|number| number.parse::<u64>().expect("has to be a valid number"))
                    .collect_vec(),
            )
        })
        .collect_vec()
        .into_par_iter()
        .filter(|(result, numbers)| solve_system(*result, numbers, max_op))
        .map(|(result, _)| result)
        .sum::<u64>()
}

fn solve_system(result: u64, numbers: &[u64], max_op: usize) -> bool {
    let op_size = numbers.len() - 1;
    let permutations = max_op.pow(op_size as u32);

    (0..permutations)
        .into_par_iter()
        .filter(move |version| {
            get_op_iter(op_size, *version, max_op)
                .enumerate()
                .fold(numbers[0], |last, (i, op)| {
                    perform(op, last, numbers[i + 1])
                })
                == result
        })
        .count()
        != 0
}

fn get_op_iter(op_size: usize, version: usize, max_op: usize) -> impl Iterator<Item = Op> {
    const MAP: [Op; 3] = [Op::Mul, Op::Add, Op::Concat];
    (0..op_size)
        .map(move |i| version / max_op.pow(i as u32) % max_op)
        .map(|op_num| MAP[op_num])
}

fn perform(op: Op, left: u64, rigth: u64) -> u64 {
    match op {
        Op::Mul => left * rigth,
        Op::Add => left + rigth,
        Op::Concat => left * 10u64.pow(rigth.ilog10() + 1) + rigth,
    }
}
