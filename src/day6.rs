use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::collections::{HashMap, HashSet};

const DEFAULT_DIR: (isize, isize) = (-1, 0isize);

use crate::{get_grid, Day};

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
        let mut covered_position = HashMap::new();
        let guard = get_guard_pos(&input);

        simulate_guard_on_grid(guard, &grid, &mut covered_position);

        // add one since it cuts the last position
        covered_position.into_iter().len().to_string()
    }

    fn part_2(&mut self, input: String) -> String {
        let grid = get_grid(&input);
        let mut covered_position = HashMap::new();
        let guard = get_guard_pos(&input);

        simulate_guard_on_grid(guard, &grid, &mut covered_position);

        covered_position
            .into_par_iter()
            .map(|(pos, _)| (pos.0 as usize, pos.1 as usize))
            .filter(|pos| (pos.0 as isize, pos.1 as isize) != guard && grid[pos.0][pos.1] != '#')
            .map(|pos| {
                let mut grid = grid.clone();
                grid[pos.0][pos.1] = 'o';
                grid
            })
            .filter_map(|grid| {
                let mut covered_position = HashMap::new();
                simulate_guard_on_grid(guard, &grid, &mut covered_position).then_some(())
            })
            .count()
            .to_string()
    }
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

// returns true if has passed a cell in the same direction meaning it has completed a loop
fn simulate_guard_on_grid(
    mut guard: (isize, isize),
    grid: &[Vec<char>],
    covered_position: &mut HashMap<(isize, isize), HashSet<(isize, isize)>>,
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

        if !covered_position.entry(guard).or_default().insert(dir) {
            return true;
        }

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

    // push last position of the guard
    _ = covered_position.entry(guard).or_default().insert(dir);

    false
}
