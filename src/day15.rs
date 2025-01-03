#![allow(dead_code)]
use std::{hint::unreachable_unchecked, ops::Add};

use itertools::Itertools;

use crate::{get_grid, Day};

const SEARCH: &str = r#"#######

"#;
const BOX: char = 'O';
const BOX_L: char = '[';
const BOX_R: char = ']';
const EMPTY: char = '.';
const FISH: char = '@';
const BARRIER: char = '#';

pub struct Day15;

impl Day for Day15 {
    fn example_input(&self) -> (&'static str, &'static str) {
        let input = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#;
        (input, input)
    }

    fn example_solution(&self) -> (&'static str, &'static str) {
        ("10092", "")
    }

    fn part_1(&mut self, input: String) -> String {
        let (mut grid, mut lanternfish, actions) = get_simulation_data(&input);

        for action in actions.chars().filter(|c| *c != '\n') {
            let dir = action_to_dir(action);
            let mut this_pos = offset_pos(lanternfish, dir);
            let mut last_pos = lanternfish;
            while let Some(this_char) = get_on_grid(&grid, this_pos) {
                if this_char == EMPTY && last_pos != lanternfish {
                    set_grid(&mut grid, lanternfish, EMPTY);
                    set_grid(&mut grid, this_pos, BOX);
                    lanternfish = offset_pos(lanternfish, dir);
                    break;
                } else if this_char == BARRIER {
                    break;
                } else if this_char == EMPTY && last_pos == lanternfish {
                    set_grid(&mut grid, lanternfish, EMPTY);
                    lanternfish = offset_pos(lanternfish, dir);
                    break;
                }

                last_pos = this_pos;
                this_pos = offset_pos(this_pos, dir);
            }

            set_grid(&mut grid, lanternfish, FISH);
        }

        get_gps_sum(grid)
    }

    fn part_2(&mut self, input: String) -> String {
        let (grid, mut lanternfish, actions) = get_simulation_data(&input);

        let mut grid = grid
            .into_iter()
            .map(|line| {
                line.into_iter().flat_map(|c| match c {
                    '#' => "##".chars(),
                    'O' => "[]".chars(),
                    '.' => "..".chars(),
                    '@' => "@.".chars(),
                    _ => unreachable!(),
                })
            })
            .map(|line| line.collect_vec())
            .collect_vec();

        for action in actions.chars().filter(|c| *c != '\n') {
            let dir = action_to_dir(action);
            let mut this_pos = offset_pos(lanternfish, dir);
            let mut last_pos = lanternfish;
            while let Some(this_char) = get_on_grid(&grid, this_pos) {
                if this_char == EMPTY && last_pos != lanternfish {
                    set_grid(&mut grid, lanternfish, EMPTY);
                    set_grid(&mut grid, this_pos, BOX);
                    lanternfish = offset_pos(lanternfish, dir);
                    break;
                } else if this_char == BARRIER {
                    break;
                } else if this_char == EMPTY && last_pos == lanternfish {
                    set_grid(&mut grid, lanternfish, EMPTY);
                    lanternfish = offset_pos(lanternfish, dir);
                    break;
                }

                last_pos = this_pos;
                this_pos = offset_pos(this_pos, dir);
            }

            set_grid(&mut grid, lanternfish, FISH);
        }

        get_gps_sum(grid)
    }
}

fn get_simulation_data(input: &str) -> (Vec<Vec<char>>, (isize, isize), &str) {
    let split_point = input
        .find(SEARCH)
        .expect("split point should exist")
        .add(SEARCH.len() - 1);
    let grid = get_grid(input.split_at(split_point).0.trim());

    let lanternfish = grid
        .iter()
        .enumerate()
        .flat_map(|(x, line)| line.iter().enumerate().map(move |(y, c)| ((x, y), *c)))
        .find_map(|(pos, c)| (c == '@').then_some(pos))
        .map(|pos| (pos.0 as isize, pos.1 as isize))
        .expect("the lanternfish should exist");
    let actions = input.split_at(split_point).1.trim();

    (grid, lanternfish, actions)
}

fn get_gps_sum(grid: Vec<Vec<char>>) -> String {
    grid.into_iter()
        .enumerate()
        .flat_map(|(x, line)| {
            line.into_iter()
                .enumerate()
                .filter_map(move |(y, c)| (c == BOX || c == BOX_L).then_some((x, y)))
        })
        .map(|(x, y)| x * 100 + y)
        .sum::<usize>()
        .to_string()
}

fn offset_pos(abs: (isize, isize), offset: (isize, isize)) -> (isize, isize) {
    (abs.0 + offset.0, abs.1 + offset.1)
}

fn get_on_grid(grid: &[Vec<char>], pos: (isize, isize)) -> Option<char> {
    pos.0
        .try_into()
        .ok()
        .and_then(|x: usize| Some((x, pos.1.try_into().ok()?)))
        .and_then(|(x, y): (usize, usize)| grid.get(x).and_then(|line| line.get(y)))
        .copied()
}

fn set_grid(grid: &mut [Vec<char>], pos: (isize, isize), c: char) {
    let Some(pos): Option<(usize, usize)> = pos
        .0
        .try_into()
        .ok()
        .and_then(|x: usize| Some((x, pos.1.try_into().ok()?)))
    else {
        return;
    };

    if let Some(point) = grid.get_mut(pos.0).and_then(|line| line.get_mut(pos.1)) {
        *point = c
    }
}

fn action_to_dir(action: char) -> (isize, isize) {
    match action {
        '^' => (-1isize, 0isize),
        '>' => (0, 1),
        '<' => (0, -1),
        'v' => (1, 0),
        _ => unsafe { unreachable_unchecked() },
    }
}

#[allow(unused)]
fn print_grid(grid: &[Vec<char>]) {
    for line in grid.iter() {
        for c in line.iter() {
            print!("{c}");
        }
        println!();
    }
}
