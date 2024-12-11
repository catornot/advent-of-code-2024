use std::collections::HashMap;

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
        let mut cache = HashMap::new();
        input
            .split_whitespace()
            .map(|rock_num| rock_num.parse::<u64>().expect("has to be a number"))
            .map(|rock| rock_rule::<25>(rock, 0, &mut cache))
            .sum::<u64>()
            .to_string()
    }

    fn part_2(&mut self, input: String) -> String {
        if input.is_empty() {
            return "".to_string();
        }

        let mut cache = HashMap::new();
        input
            .split_whitespace()
            .map(|rock_num| rock_num.parse::<u64>().expect("has to be a number"))
            .map(|rock| rock_rule::<75>(rock, 0, &mut cache))
            .sum::<u64>()
            .to_string()
    }
}

fn rock_rule<const DEPTH: usize>(
    rock: u64,
    depth: usize,
    cache: &mut HashMap<(u64, u64), u64>,
) -> u64 {
    if depth >= DEPTH {
        return 1;
    }
    let next_depth = depth + 1;

    if let Some(count) = cache.get(&(rock, depth as u64)).copied() {
        return count;
    }

    let count = if rock == 0 {
        rock_rule::<DEPTH>(1, next_depth, cache)
    } else if (rock.ilog10() + 1) % 2 == 0 {
        let half_len = (rock.ilog10() + 1) / 2;
        let first_half = rock / 10u64.pow(half_len);
        rock_rule::<DEPTH>(first_half, next_depth, cache)
            + rock_rule::<DEPTH>(rock - first_half * 10u64.pow(half_len), next_depth, cache)
    } else {
        rock_rule::<DEPTH>(rock * 2024, next_depth, cache)
    };

    _ = cache.insert((rock, depth as u64), count);
    count
}
