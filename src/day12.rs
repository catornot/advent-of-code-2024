use std::{
    collections::{HashMap, HashSet},
    ops::Add,
};

use itertools::Itertools;

use crate::Day;

pub struct Day12;

impl Day for Day12 {
    fn example_input(&self) -> (&'static str, &'static str) {
        let input = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;
        (input, input)
    }

    fn example_solution(&self) -> (&'static str, &'static str) {
        ("1930", "")
    }

    fn part_1(&mut self, input: String) -> String {
        let grid = get_grid(&input);
        let adjacency = grid
            .iter()
            .enumerate()
            .flat_map(|(x, line)| {
                line.iter()
                    .copied()
                    .enumerate()
                    .map(move |(y, c)| ((x as isize, y as isize), c))
            })
            .fold(
                HashMap::<_, Vec<(HashSet<_>, HashSet<_>)>>::new(),
                |mut map, (pos, c)| {
                    let adjacency = map.entry(c).or_default();

                    let adjacent = adjacency
                        .iter()
                        .position(|adjacent| adjacent.1.contains(&pos))
                        .unwrap_or_else(|| {
                            adjacency.push((HashSet::new(), HashSet::new()));
                            adjacency.len() - 1
                        });
                    let adjacent = &mut adjacency[adjacent];

                    for offset_pos in [(-1, 0), (0isize, -1), (1, 0isize), (0, 1)]
                        .into_iter()
                        .map(|offset| (pos.0 + offset.0, pos.1 + offset.1))
                    {
                        _ = adjacent.1.insert(offset_pos);
                    }
                    adjacent.0.insert(pos);

                    map
                },
            );
        let adjacency = merge_adjacent_adjancy_maps(adjacency); // repeat this a few times to account for errors in the merge lol
        let adjacency = merge_adjacent_adjancy_maps(adjacency);
        let adjacency = merge_adjacent_adjancy_maps(adjacency);
        let adjacency = merge_adjacent_adjancy_maps(adjacency);
        let areas = adjacency.iter().fold(HashMap::new(), |mut map, (c, maps)| {
            for (i, adjacent) in maps.iter().enumerate() {
                map.entry((*c, i)).or_insert(adjacent.0.len() as u32);
            }
            map
        });
        let mut perimeter = areas.keys().copied().fold(HashMap::new(), |mut map, c| {
            _ = map.insert(c, 0);
            map
        });

        for pos in (-1..grid.len().add(1) as isize)
            .flat_map(|x| (-1..grid[0].len().add(1) as isize).map(move |y| (x, y)))
        {
            for c in areas.keys().copied().map(|(c, _)| c).unique() {
                let Some(i) = adjacency
                    .get(&c)
                    .unwrap()
                    .iter()
                    .position(|adjacent| adjacent.1.contains(&pos))
                else {
                    continue;
                };

                *perimeter.entry((c, i)).or_default() += get_sides(&grid, pos, c);
            }
        }

        areas
            .into_iter()
            .inspect(|(letter, area)| {
                println!("{}: {area} * {}", letter.0, *perimeter.get(letter).unwrap());
            })
            .map(|(letter, area)| *perimeter.get(&letter).unwrap() * area)
            .sum::<u32>()
            .to_string()
    }

    fn part_2(&mut self, input: String) -> String {
        _ = input;
        "".to_string()
    }
}

type AdjencyMap = HashMap<char, Vec<(HashSet<(isize, isize)>, HashSet<(isize, isize)>)>>;
fn merge_adjacent_adjancy_maps(mut adjacency_map: AdjencyMap) -> AdjencyMap {
    let mut new_adjacency: AdjencyMap = HashMap::new();

    for c in adjacency_map.keys().copied().collect_vec().into_iter() {
        let adjacent_here = new_adjacency.entry(c).or_insert(vec![adjacency_map
            .get_mut(&c)
            .unwrap()
            .pop()
            .unwrap()]);

        for existing in adjacency_map.remove(&c).unwrap().into_iter() {
            if let Some(overlapping) = adjacent_here
                .iter_mut()
                .find(|other| other.1.intersection(&existing.0).next().is_some())
            {
                overlapping.0.extend(existing.0);
                overlapping.1.extend(existing.1);
            } else {
                adjacent_here.push(existing);
            }
        }
    }

    new_adjacency
}

fn get_sides(grid: &[Vec<char>], pos: (isize, isize), letter: char) -> u32 {
    if pos
        .0
        .try_into()
        .ok()
        .and_then(|x: usize| grid.get(x))
        .and_then(|line| Some((line, pos.1.try_into().ok()?)))
        .and_then(|(line, y): (_, usize)| line.get(y))
        .copied()
        .unwrap_or('\0')
        == letter
    {
        return 0;
    }

    [(-1, 0), (0, -1), (1, 0), (0, 1)]
        .into_iter()
        .filter_map(|offset| {
            Some(
                grid.as_ref()
                    .get(TryInto::<usize>::try_into(pos.0 as i32 + offset.0).ok()?)
                    .as_ref()?
                    .get(TryInto::<usize>::try_into(pos.1 as i32 + offset.1).ok()?)
                    .as_ref()?,
            )
            .cloned()
            .cloned()
        })
        .filter(|cell| *cell == letter)
        .count() as u32
}

fn get_grid(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec()
}
