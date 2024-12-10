use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use crate::Day;

type Pos = (usize, usize);

pub struct Day8;

impl Day for Day8 {
    fn example_input(&self) -> (&'static str, &'static str) {
        let input = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;
        (input, input)
    }

    fn example_solution(&self) -> (&'static str, &'static str) {
        ("14", "34")
    }

    fn part_1(&mut self, input: String) -> String {
        let antenas = get_antenas(&input);

        let x_limit = input.lines().count();
        let y_limit = input.lines().next().unwrap().len();

        get_antinodes_from_antenas(&antenas, x_limit, y_limit, get_antinodes)
            .len()
            .to_string()
    }

    fn part_2(&mut self, input: String) -> String {
        let antenas = get_antenas(&input);

        let x_limit = input.lines().count();
        let y_limit = input.lines().next().unwrap().len();

        let mut antinodes =
            get_antinodes_from_antenas(&antenas, x_limit, y_limit, get_antinodes_many);

        for positions in antenas.into_values() {
            antinodes.extend(positions);
        }

        antinodes.len().to_string()
    }
}

fn get_antinodes_from_antenas(
    antenas: &HashMap<char, Vec<(usize, usize)>>,
    x_limit: usize,
    y_limit: usize,
    antinodes_func: fn(Pos, Pos) -> Vec<Pos>,
) -> HashSet<(usize, usize)> {
    antenas
        .clone()
        .into_values()
        .flat_map(move |antenas| find_antinodes_positions(antenas, antinodes_func))
        .filter(|pos| pos.0 < x_limit)
        .filter(|pos| pos.1 < y_limit)
        .fold(HashSet::new(), |mut set, position| {
            _ = set.insert(position);
            set
        })
}

fn get_antenas(input: &str) -> HashMap<char, Vec<(usize, usize)>> {
    input
        .lines()
        .enumerate()
        .flat_map(|(x, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(y, c)| c.is_alphanumeric().then_some((c, (x, y))))
        })
        .fold(HashMap::<char, Vec<Pos>>::new(), |mut map, (c, pos)| {
            map.entry(c).or_default().push(pos);
            map
        })
}

fn find_antinodes_positions(
    positions: Vec<(usize, usize)>,
    antinodes_func: fn(Pos, Pos) -> Vec<Pos>,
) -> impl Iterator<Item = Pos> {
    positions
        .into_iter()
        .combinations(2)
        .map(|both| (both[0], both[1]))
        .flat_map(move |(left, rigth)| antinodes_func(left, rigth))
}

fn get_antinodes_many(first: Pos, second: Pos) -> Vec<Pos> {
    let (first, second) = (
        (first.0 as isize, first.1 as isize),
        (second.0 as isize, second.1 as isize),
    );

    (1..100)
        .map(|i| i as isize)
        .flat_map(move |i| {
            [
                (
                    first.0 + (first.0 - second.0) * i,
                    first.1 + (first.1 - second.1) * i,
                ),
                (
                    second.0 + (second.0 - first.0) * i,
                    second.1 + (second.1 - first.1) * i,
                ),
            ]
            .into_iter()
        })
        .filter_map(|pos| Some((pos.0.try_into().ok()?, pos.1.try_into().ok()?)))
        .collect_vec()
}

fn get_antinodes(first: Pos, second: Pos) -> Vec<Pos> {
    let (first, second) = (
        (first.0 as isize, first.1 as isize),
        (second.0 as isize, second.1 as isize),
    );

    [
        (
            first.0 + (first.0 - second.0),
            first.1 + (first.1 - second.1),
        ),
        (
            second.0 + (second.0 - first.0),
            second.1 + (second.1 - first.1),
        ),
    ]
    .into_iter()
    .filter_map(|pos| Some((pos.0.try_into().ok()?, pos.1.try_into().ok()?)))
    .collect_vec()
}

#[test]
fn test_antinodes() {
    let first = (0, 4);
    let second = (0, 2);

    assert_eq!(get_antinodes(first, second), vec![(0, 6), (0, 0)]);

    let first = (7, 4);
    let second = (6, 2);

    assert_eq!(get_antinodes(first, second), vec![(8, 6), (5, 0)]);
}
