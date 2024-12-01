use crate::Day;

pub struct Day1;

impl Day for Day1 {
    fn example_input(&self) -> (&'static str, &'static str) {
        (
            r#"3   4
4 3
2 5
1 3
3 9
3 3"#,
            r#"3   4
4   3
2   5
1   3
3   9
3   3"#,
        )
    }

    fn example_solution(&self) -> (&'static str, &'static str) {
        ("11", "31")
    }

    fn part_1(&mut self, input: String) -> String {
        let (left, rigth) = get_left_rigth_distances(input);

        left.into_iter()
            .zip(rigth)
            .map(|(left, rigth)| left.max(rigth) - left.min(rigth))
            .sum::<u64>()
            .to_string()
    }

    fn part_2(&mut self, input: String) -> String {
        let (left, rigth) = get_left_rigth_distances(input);

        left.into_iter()
            .map(|left_dis| {
                rigth
                    .iter()
                    .filter(move |rigth_dis| left_dis == **rigth_dis)
                    .count() as u64
                    * left_dis
            })
            .sum::<u64>()
            .to_string()
    }
}

fn get_left_rigth_distances(input: String) -> (Vec<u64>, Vec<u64>) {
    let size = input.lines().count();

    let (mut left, mut rigth) = input
        .lines()
        .filter_map(|distances| distances.split_once(" "))
        .map(|(dis_left, dis_rigth)| (dis_left.trim(), dis_rigth.trim()))
        .filter_map(|(dis_left, dis_rigth)| {
            Some((
                dis_left.parse::<u64>().ok()?,
                dis_rigth.parse::<u64>().ok()?,
            ))
        })
        .fold(
            (vec![], vec![]),
            |(mut left, mut rigth), (dis_left, dis_rigth)| {
                left.push(dis_left);
                rigth.push(dis_rigth);
                (left, rigth)
            },
        );

    if left.len() != size || rigth.len() != size {
        panic!("invalid len lost stuff!");
    }

    left.sort();
    rigth.sort();
    (left, rigth)
}
