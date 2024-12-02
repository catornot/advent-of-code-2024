use std::fs::read_to_string;

pub trait Day {
    fn example_input(&self) -> (&'static str, &'static str);
    fn example_solution(&self) -> (&'static str, &'static str);
    fn part_1(&mut self, input: String) -> String;
    fn part_2(&mut self, input: String) -> String;
}

pub fn get_input(day: usize) -> std::io::Result<String> {
    read_to_string(format!("files/day_{day}"))
}
