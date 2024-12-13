use peroxide::{
    fuga::{Matrix, Shape, R},
    prelude::SimplerLinearAlgebra,
    traits::float::FloatWithPrecision,
};

use crate::Day;

type Pos = (u64, u64);

#[derive(Debug, Clone)]
struct Game {
    a: Pos,
    b: Pos,
    prize: Pos,
}

pub struct Day13;

impl Day for Day13 {
    fn example_input(&self) -> (&'static str, &'static str) {
        let input = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#;

        (input, input)
    }

    fn example_solution(&self) -> (&'static str, &'static str) {
        ("480", "875318608908")
    }

    fn part_1(&mut self, input: String) -> String {
        solve(&input, 0)
    }

    fn part_2(&mut self, input: String) -> String {
        solve(&input, 10000000000000)
    }
}

fn solve(input: &str, error: u64) -> String {
    let mut problems = Vec::new();
    let mut lines = input.lines().peekable();
    while lines.peek().is_some() {
        problems.push(Game {
            a: lines
                .next()
                .unwrap()
                .split_once(':')
                .and_then(|(_, rigth)| {
                    rigth.split_once(',').and_then(|(x, y)| {
                        Some((
                            x.split_once('+').unwrap().1.parse().ok()?,
                            y.split_once('+').unwrap().1.parse().ok()?,
                        ))
                    })
                })
                .expect("should have a pos for a"),
            b: lines
                .next()
                .unwrap()
                .split_once(':')
                .and_then(|(_, rigth)| {
                    rigth.split_once(',').and_then(|(x, y)| {
                        Some((
                            x.split_once('+').unwrap().1.parse().ok()?,
                            y.split_once('+').unwrap().1.parse().ok()?,
                        ))
                    })
                })
                .expect("should have a pos for b"),
            prize: lines
                .next()
                .unwrap()
                .split_once(':')
                .and_then(|(_, rigth)| {
                    rigth.split_once(',').and_then(|(x, y)| {
                        Some((
                            x.split_once('=').unwrap().1.parse::<u64>().ok()? + error,
                            y.split_once('=').unwrap().1.parse::<u64>().ok()? + error,
                        ))
                    })
                })
                .expect("should have a pos for prize"),
        });

        _ = lines.next();
    }

    problems
        .into_iter()
        .filter_map(solve_game)
        .sum::<u64>()
        .to_string()
}

const A_COST: u64 = 3;
const B_COST: u64 = 1;
fn solve_game(game: Game) -> Option<u64> {
    let result = Matrix::new(
        vec![
            game.a.0 as f64,
            game.b.0 as f64,
            game.prize.0 as f64,
            game.a.1 as f64,
            game.b.1 as f64,
            game.prize.1 as f64,
        ],
        2,
        3,
        Shape::Row,
    )
    .rref();

    const PRESICSION: usize = 2;
    (result.data[0] == 1.
        && result.data[4] == 1.
        && (result.data[2].trunc() == result.data[2].round_with_precision(PRESICSION)
            || result.data[2].trunc() + 1. == result.data[2].round_with_precision(PRESICSION))
        && (result.data[5].trunc() == result.data[5].round_with_precision(PRESICSION)
            || result.data[5].trunc() + 1. == result.data[5].round_with_precision(PRESICSION)))
    .then(|| {
        result.data[2].round_with_precision(PRESICSION) as u64 * A_COST
            + result.data[5].round_with_precision(PRESICSION) as u64 * B_COST
    })
}
