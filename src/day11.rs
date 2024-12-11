use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::Day;

pub struct Day11;

impl Day for Day11 {
    fn example_input(&self) -> (&'static str, &'static str) {
        ("125 17", "")
    }

    fn example_solution(&self) -> (&'static str, &'static str) {
        ("55312", "")
    }

    fn part_1(&mut self, input: String) -> String {
        input
            .split_whitespace()
            .map(|rock_num| rock_num.parse::<u64>().expect("has to be a number"))
            .map(|rock| rock_rule::<25>(rock, 0))
            .sum::<u64>()
            .to_string()
    }

    fn part_2(&mut self, input: String) -> String {
        if input.is_empty() {
            return "".to_string();
        }

        input
            .split_whitespace()
            .map(|rock_num| rock_num.parse::<u64>().expect("has to be a number"))
            .collect_vec()
            .into_par_iter()
            .map(|rock| rock_rule::<75>(rock, 0))
            .sum::<u64>()
            .to_string()
    }
}

fn rock_rule<const DEPTH: usize>(rock: u64, depth: usize) -> u64 {
    if depth >= DEPTH {
        return 1;
    }
    let next_depth = depth + 1;

    if rock == 0 {
        rock_rule::<DEPTH>(1, next_depth)
    } else if (rock.ilog10() + 1) % 2 == 0 {
        let half_len = (rock.ilog10() + 1) / 2;
        let first_half = rock / 10u64.pow(half_len);
        rock_rule::<DEPTH>(first_half, next_depth)
            + rock_rule::<DEPTH>(rock - first_half * 10u64.pow(half_len), next_depth)
    } else {
        rock_rule::<DEPTH>(rock * 2024, next_depth)
    }
}
