use itertools::Itertools;

use crate::Day;

pub struct Day9;

impl Day for Day9 {
    fn example_input(&self) -> (&'static str, &'static str) {
        ("2333133121414131402", "2333133121414131402")
    }

    fn example_solution(&self) -> (&'static str, &'static str) {
        ("1928", "2858")
    }

    fn part_1(&mut self, input: String) -> String {
        let mut file_system = build_file_rep(input.as_str());

        let mut last_view = file_system.len() - 1;
        let mut move_view = 0;
        while last_view >= move_view {
            if file_system.get(last_view).unwrap().is_none() {
                last_view -= 1;
            } else if file_system.get(move_view).unwrap().is_some() {
                move_view += 1;
            } else if file_system.get(last_view).unwrap().is_some()
                && file_system.get(move_view).unwrap().is_none()
            {
                let id = file_system.get_mut(last_view).unwrap().take().unwrap();
                file_system.get_mut(move_view).unwrap().replace(id);

                last_view -= 1;
                move_view += 1;
            } else {
                unreachable!()
            }
        }

        file_system
            .into_iter()
            .enumerate()
            .take_while(|(_, id)| id.is_some())
            .map(|(i, id)| id.unwrap() as usize * i)
            .sum::<usize>()
            .to_string()
    }

    fn part_2(&mut self, input: String) -> String {
        let mut file_system = build_file_rep(input.as_str());

        for i in (0..=file_system.iter().cloned().flatten().max().unwrap()).rev() {
            part_2_compact_fs(&mut file_system, i);
        }

        file_system
            .into_iter()
            .enumerate()
            .map(|(i, id)| id.unwrap_or_default() as usize * i)
            .sum::<usize>()
            .to_string()
    }
}

#[allow(unused)]
fn print_fs(file_system: &[Option<i32>]) {
    for id in file_system.iter().map(|id| {
        id.and_then(|id| char::from_digit(id as u32, 10))
            .unwrap_or('.')
    }) {
        print!("{}", id);
    }
    println!();
}

fn part_2_compact_fs(file_system: &mut [Option<i32>], move_id: i32) {
    let last_view = file_system
        .iter()
        .enumerate()
        .rev()
        .find_map(|(i, id)| matches!(id, Some(id) if *id == move_id).then_some(i))
        .expect("all ids should exist");
    let current_width = file_system
        .get(last_view)
        .unwrap()
        .map(|id| {
            file_system
                .get(..=last_view)
                .unwrap()
                .iter()
                .rev()
                .take_while(move |other| **other == Some(id))
                .count()
        })
        .expect("a width should exist for this id");

    let mut move_view = 0;
    while last_view >= move_view + current_width {
        if !file_system
            .get(move_view..(move_view + current_width))
            .unwrap()
            .iter()
            .all(|slot| slot.is_none())
        {
            move_view += 1;
        } else if file_system.get(last_view).cloned().unwrap() == Some(move_id)
            && file_system
                .get(move_view..(move_view + current_width))
                .unwrap()
                .iter()
                .all(|slot| slot.is_none())
        {
            let mut id = {
                let id = file_system
                    .get_mut((last_view - current_width + 1)..=last_view)
                    .unwrap();

                id.iter_mut().map(|id| id.take()).collect_vec()
            };

            file_system
                .get_mut(move_view..(move_view + current_width))
                .unwrap()
                .swap_with_slice(&mut id);

            return;
        } else {
            unreachable!()
        }
    }
}

fn build_file_rep(input: &str) -> Vec<Option<i32>> {
    let mut id = -1;
    let mut file_system = Vec::with_capacity(input.chars().count());
    for (i, c) in input.chars().enumerate() {
        let size = c.to_digit(10).expect("has to be a number");

        let this_id = if i % 2 == 0 {
            id += 1;
            Some(id)
        } else {
            None
        };

        file_system.extend((0..size).map(|_| this_id));
    }
    file_system
}
