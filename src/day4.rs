use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::Day;

pub struct Day4;

impl Day for Day4 {
    fn example_input(&self) -> (&'static str, &'static str) {
        (
            r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#,
            r#".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
.........."#,
        )
    }

    fn example_solution(&self) -> (&'static str, &'static str) {
        ("18", "9")
    }

    fn part_1(&mut self, input: String) -> String {
        let grid: Vec<Vec<char>> = input
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();

        let y_len = grid[0].len();
        (0..grid.len())
            .into_par_iter()
            .map(|x| (0..y_len).map(|y| count_xmas(&grid, (x, y))).sum::<usize>())
            .sum::<usize>()
            .to_string()
    }

    fn part_2(&mut self, input: String) -> String {
        let grid: Vec<Vec<char>> = input
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();

        let y_len = grid[0].len();
        (0..grid.len())
            .into_par_iter()
            .map(|x| {
                (0..y_len)
                    .map(|y| count_x_mas(&grid, (x, y)))
                    .sum::<usize>()
            })
            .sum::<usize>()
            .to_string()
    }
}

pub fn count_xmas(grid: &[Vec<char>], pos: (usize, usize)) -> usize {
    let cell = grid[pos.0][pos.1];

    if cell != 'X' {
        return 0;
    }

    let offsets = [
        [[1, 0], [2, 0], [3, 0isize]],    // left
        [[-1, 0], [-2, 0], [-3, 0isize]], // rigth
        [[0, -1], [0, -2], [0, -3]],      // down
        [[0, 1], [0, 2], [0, 3]],         // up
        [[1, -1], [2, -2], [3, -3]],      // down left
        [[1, 1], [2, 2], [3, 3]],         // up left
        [[-1, -1], [-2, -2], [-3, -3]],   // down rigth
        [[-1, 1], [-2, 2], [-3, 3]],      // up rigth
    ];

    let count = offsets
        .into_iter()
        .map(|offsts| {
            offsts
                .iter()
                .zip("MAS".chars())
                .take_while(|(offset, c)| is_correct_char(pos, offset, grid, *c).is_some())
                .count()
                == 3
        })
        .filter(|b| *b)
        .count();

    count
}

fn is_correct_char(
    pos: (usize, usize),
    offset: &[isize; 2],
    grid: &[Vec<char>],
    c: char,
) -> Option<()> {
    let pos = pos
        .0
        .checked_add_signed(offset[0])
        .and_then(|x| Some((x, pos.1.checked_add_signed(offset[1])?)))?;
    (*grid.get(pos.0)?.get(pos.1)? == c).then_some(())
}

pub fn count_x_mas(grid: &[Vec<char>], pos: (usize, usize)) -> usize {
    let cell = grid[pos.0][pos.1];

    if cell != 'A' {
        return 0;
    }

    let offsets = [
        [[[-1, 1], [-1, -1]], [[1, 1isize], [1, -1]]], // left
        [[[1, 1], [1, -1]], [[-1, 1], [-1, -1]]],      // right
        [[[-1, 1], [1, 1]], [[-1, -1], [1, -1]]],      // up
        [[[-1, -1], [1, -1]], [[-1, 1], [1, 1]]],      // down
    ];

    offsets
        .into_iter()
        .find(|offsts| {
            offsts
                .iter()
                .zip("MS".chars())
                .flat_map(|(inner, c)| inner.iter().map(move |offset| (offset, c)))
                .take_while(|(offset, c)| is_correct_char(pos, offset, grid, *c).is_some())
                .count()
                == 4
        })
        .map(|_| 1)
        .unwrap_or(0)
}
