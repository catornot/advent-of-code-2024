use itertools::Itertools;
use std::{
    collections::HashSet,
    ops::{Add, Not},
};

const DEFAULT_DIR: (isize, isize) = (-1, 0isize);

use crate::Day;

pub struct Day6;

impl Day for Day6 {
    fn example_input(&self) -> (&'static str, &'static str) {
        let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;
        (input, input)
    }

    fn example_solution(&self) -> (&'static str, &'static str) {
        ("41", "6")
    }

    fn part_1(&mut self, input: String) -> String {
        let grid = get_grid(&input);
        let mut covered_position: HashSet<(isize, isize)> = HashSet::new();
        let guard = get_guard_pos(&input);

        simulate_guard_on_grid(guard, &grid, &mut covered_position);

        covered_position.into_iter().len().add(1).to_string()
    }

    fn part_2(&mut self, input: String) -> String {
        let grid = get_grid(&input);
        let guard = get_guard_pos(&input);

        let x_len = grid.len();
        let y_len = grid[0].len();

        // (0..x_len)
        //     // .into_par_iter()
        //     // .flat_map_iter(|x| (0..y_len).map(move |y| (x, y)))
        //     .flat_map(|x| (0..y_len).map(move |y| (x, y)))
        //     .filter(|pos| (pos.0 as isize, pos.1 as isize) != guard)
        //     .filter_map(|pos| {
        //         let c = grid[pos.0][pos.1];
        //         grid[pos.0][pos.1] = 'o';

        //         let mut covered_position: HashSet<(isize, isize)> = HashSet::new();
        //         let mut passed = 0;

        //         let r = simulate_guard_on_grid(guard, &grid, &mut covered_position, |pos, dir| {
        //             if pos == guard && dir == DEFAULT_DIR {
        //                 passed += 1;
        //             }

        //             passed > 1
        //         })
        //         .not()
        //         .then_some(());

        //         grid[pos.0][pos.1] = c;
        //         r
        //     })
        //     .count()
        //     .to_string()

        (0..x_len)
            // .into_par_iter()
            // .flat_map_iter(|x| (0..y_len).map(move |y| (x, y)))
            .flat_map(|x| (0..y_len).map(move |y| (x, y)))
            .filter(|pos| (pos.0 as isize, pos.1 as isize) != guard)
            .map(|pos| {
                let mut grid = grid.clone();
                grid[pos.0][pos.1] = 'o';
                grid
            })
            .filter_map(|grid| {
                let mut covered_position: HashSet<(isize, isize)> = HashSet::new();
                simulate_guard_on_grid(guard, &grid, &mut covered_position)
                    .not()
                    .then_some(())
            })
            .count()
            .to_string()
    }
}

fn get_grid(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| if c == '^' { '.' } else { c }))
        .map(|iter| iter.collect_vec())
        .collect_vec()
}

fn get_guard_pos(input: &str) -> (isize, isize) {
    let Some(guard) = input.lines().enumerate().find_map(|(x, line)| {
        line.chars()
            .enumerate()
            .find_map(move |(y, c)| (c == '^').then_some((x, y)))
            .map(|(x, y)| (x as isize, y as isize))
    }) else {
        panic!("guard cannot be missnig");
    };

    guard
}

fn simulate_guard_on_grid(
    mut guard: (isize, isize),
    grid: &[Vec<char>],
    covered_position: &mut HashSet<(isize, isize)>,
) -> bool {
    let mut dir = DEFAULT_DIR;

    loop {
        let next_pos = (guard.0 + dir.0, guard.1 + dir.1);

        let Some(object) = next_pos
            .0
            .try_into()
            .ok()
            .and_then(|x| Some((x, next_pos.1.try_into().ok()?)))
            .and_then(|(x, y): (usize, usize)| grid.get(x).and_then(|line| line.get(y)))
        else {
            break;
        };

        _ = covered_position.insert(guard);

        if *object != '.' {
            dir = match dir {
                (0, 1) => (1, 0),
                (1, 0) => (0, -1),
                (0, -1) => (-1, 0),
                (-1, 0) => (0, 1),
                _ => unreachable!(),
            };
        } else {
            guard = next_pos;
        }
    }

    false
}
