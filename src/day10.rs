use std::collections::HashSet;

use itertools::Itertools;

type Pos = (usize, usize);
type Dir = (isize, isize);

use crate::Day;

pub struct Day10;

impl Day for Day10 {
    fn example_input(&self) -> (&'static str, &'static str) {
        let input = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;
        (input, input)
    }

    fn example_solution(&self) -> (&'static str, &'static str) {
        ("36", "81")
    }

    fn part_1(&mut self, input: String) -> String {
        solve(&input, false)
    }

    fn part_2(&mut self, input: String) -> String {
        solve(&input, true)
    }
}

fn solve(input: &str, allow_path_back: bool) -> String {
    let grid = get_grid(input);

    let trailheads = grid
        .iter()
        .enumerate()
        .flat_map(|(x, line)| {
            line.iter()
                .enumerate()
                .filter_map(move |(y, num)| 0.eq(num).then_some((x, y)))
        })
        .collect_vec();

    let mut trails = Vec::new();

    for head in trailheads {
        let mut traveled = HashSet::new();
        follow_path(
            &grid,
            &mut trails,
            Vec::new(),
            &mut traveled,
            head,
            None,
            allow_path_back,
        );
    }

    trails.retain(|trail| {
        trail
            .first()
            .cloned()
            .and_then(|first| {
                (get_tile(&grid, first) == Some(0)
                    && get_tile(&grid, trail.last().cloned()?) == Some(9))
                .then_some(())
            })
            .is_some()
    });

    trails.len().to_string()
}

fn follow_path(
    grid: &[Vec<u32>],
    trails: &mut Vec<Vec<Pos>>,
    mut trail: Vec<Pos>,
    total_traveled: &mut HashSet<Pos>,
    pos: Pos,
    from: Option<Dir>,
    allow_path_back: bool,
) {
    let current = get_tile(grid, pos).unwrap();

    if !allow_path_back && trail.iter().any(|other| pos == *other) {
        return;
    }
    trail.push(pos);
    total_traveled.insert(pos);

    if current == 9 {
        trails.push(trail);
        return;
    }

    for (dir, new_pos) in [(-1, 0), (1, 0), (0, 1), (0, -1)]
        .into_iter()
        .filter(move |pos| Some(*pos) != from.map(invert_dir))
        .filter_map(|dir| Some((dir, get_tile_offset(grid, pos, dir)?)))
        .filter_map(|(dir, tile)| (tile.checked_sub(1) == Some(current)).then_some(dir))
        .filter_map(|dir| {
            Some((
                dir,
                (
                    pos.0.checked_add_signed(dir.0)?,
                    pos.1.checked_add_signed(dir.1)?,
                ),
            ))
        })
    {
        if !allow_path_back && total_traveled.contains(&new_pos) {
            continue;
        }

        follow_path(
            grid,
            trails,
            trail.clone(),
            total_traveled,
            new_pos,
            Some(dir),
            allow_path_back,
        );
    }
}

fn invert_dir(dir: (isize, isize)) -> (isize, isize) {
    match dir {
        (-1, 0) => (1, 0),
        (1, 0) => (-1, 0),
        (0, 1) => (0, -1),
        (0, -1) => (0, 1),
        _ => unreachable!(),
    }
}

pub fn get_tile_offset(grid: &[Vec<u32>], pos: Pos, offset: Dir) -> Option<u32> {
    get_tile(
        grid,
        (
            pos.0.checked_add_signed(offset.0)?,
            pos.1.checked_add_signed(offset.1)?,
        ),
    )
}

pub fn get_tile(grid: &[Vec<u32>], pos: Pos) -> Option<u32> {
    grid.get(pos.0).and_then(|line| line.get(pos.1)).cloned()
}

fn get_grid(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap_or(99)))
        .map(|iter| iter.collect_vec())
        .collect_vec()
}
