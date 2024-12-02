use std::ops::Not;

use crate::Day;

pub struct Day2;

impl Day for Day2 {
    fn example_input(&self) -> (&'static str, &'static str) {
        (
            r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#,
            r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#,
        )
    }

    fn example_solution(&self) -> (&'static str, &'static str) {
        ("2", "4")
    }

    fn part_1(&mut self, input: String) -> String {
        input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|num| num.parse::<i32>().expect("it's supposed to a number"))
                    .collect::<Vec<i32>>()
            })
            .filter_map(|report| {
                let dir = (report[0] - report[1]).signum();

                report
                    .windows(2)
                    .map(|window| window[0] - window[1])
                    .any(|diff| diff.abs() > 3 || diff == 0 || diff.signum() != dir)
                    .not()
                    .then_some(())
            })
            .count()
            .to_string()
    }

    fn part_2(&mut self, input: String) -> String {
        input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|num| num.parse::<i32>().expect("it's supposed to a number"))
                    .collect::<Vec<i32>>()
            })
            .filter_map(|report| {
                // brute force but whaterever
                for i in 0..report.len() {
                    let mut report = report.clone();
                    report.remove(i);

                    let dir = (report[0] - report[1]).signum();
                    if report
                        .windows(2)
                        .map(|window| window[0] - window[1])
                        .any(|diff| diff.abs() > 3 || diff == 0 || diff.signum() != dir)
                        .not()
                    {
                        return Some(());
                    }
                }

                None
            })
            .count()
            .to_string()
    }
}
