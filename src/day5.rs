use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{cmp::Ordering, collections::HashMap};

use crate::Day;

pub struct Day5;

impl Day for Day5 {
    fn example_input(&self) -> (&'static str, &'static str) {
        let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

        (input, input)
    }

    fn example_solution(&self) -> (&'static str, &'static str) {
        ("143", "123")
    }

    fn part_1(&mut self, input: String) -> String {
        let split_point = input.lines().position(|line| line.is_empty()).unwrap();

        let (_, pages_after) = find_pages_order(&input, split_point);

        let correct_pages = input
            .lines()
            .skip(split_point + 1)
            .collect_vec()
            .into_par_iter()
            .map(|line| {
                line.split(',')
                    .map(|num| num.parse::<u32>().expect("has to be a num"))
                    .collect_vec()
            })
            .filter_map(|pages| {
                pages
                    .iter()
                    .enumerate()
                    .filter_map(|(i, page)| pages_after.get(page).map(move |after| (after, i + 1)))
                    .find_map(|(after, i)| {
                        pages
                            .get(i..)?
                            .iter()
                            .any(|page| after.iter().any(|after| page == after))
                            .then_some(None)
                    })
                    .unwrap_or(Some(pages))
            })
            .collect::<Vec<Vec<u32>>>();

        correct_pages
            .into_iter()
            .map(|pages| *pages.get(pages.len() / 2).expect("cannot be empty"))
            .sum::<u32>()
            .to_string()
    }

    fn part_2(&mut self, input: String) -> String {
        let split_point = input.lines().position(|line| line.is_empty()).unwrap();

        let (pages_before, pages_after) = find_pages_order(&input, split_point);

        let incorrect_pages = input
            .lines()
            .skip(split_point + 1)
            .collect_vec()
            .into_par_iter()
            .map(|line| {
                line.split(',')
                    .map(|num| num.parse::<u32>().expect("has to be a num"))
                    .collect_vec()
            })
            .filter_map(|pages| {
                pages
                    .iter()
                    .enumerate()
                    .filter_map(|(i, page)| pages_after.get(page).map(move |after| (after, i + 1)))
                    .find_map(|(after, i)| {
                        pages
                            .get(i..)?
                            .iter()
                            .any(|page| after.iter().any(|after| page == after))
                            .then_some(())
                    })?;

                Some(pages)
            })
            .collect::<Vec<Vec<u32>>>();

        let sorted_pages = incorrect_pages
            .into_par_iter()
            .map(|mut pages| {
                pages.sort_by(|this, other| {
                    if *this == *other {
                        Ordering::Equal
                    } else if pages_before
                        .get(this)
                        .and_then(|pages| pages.iter().find(move |page| **page == *other))
                        .is_some()
                    {
                        Ordering::Less
                    } else if pages_after
                        .get(other)
                        .and_then(|pages| pages.iter().find(move |page| **page == *this))
                        .is_some()
                    {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                });
                pages
            })
            .collect::<Vec<Vec<u32>>>();

        sorted_pages
            .into_iter()
            .map(|pages| *pages.get(pages.len() / 2).expect("cannot be empty"))
            .sum::<u32>()
            .to_string()
    }
}

fn find_pages_order(
    input: &str,
    split_point: usize,
) -> (HashMap<u32, Vec<u32>>, HashMap<u32, Vec<u32>>) {
    input
        .lines()
        .enumerate()
        .take_while(|(i, _)| *i < split_point)
        .map(|(_, line)| line.split_once('|').expect("has to be seperated by |"))
        .map(|(before, after)| {
            (
                before.trim().parse::<u32>().expect("must be a number"),
                after.trim().parse::<u32>().expect("must be a number"),
            )
        })
        .fold(
            (HashMap::new(), HashMap::new()),
            |(mut pages_before, mut pages_after), (before, after)| {
                pages_before.entry(before).or_insert(Vec::new()).push(after);
                pages_after.entry(after).or_insert(Vec::new()).push(before);
                (pages_before, pages_after)
            },
        )
}
