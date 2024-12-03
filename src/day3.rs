use crate::Day;

pub struct Day3;

#[derive(Debug)]
struct Mul {
    num1: i32,
    num2: i32,
}

impl Day for Day3 {
    fn example_input(&self) -> (&'static str, &'static str) {
        (
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        )
    }

    fn example_solution(&self) -> (&'static str, &'static str) {
        ("161", "48")
    }

    fn part_1(&mut self, input: String) -> String {
        let mut muls = Vec::new();
        let mut input = input.as_str();

        while !input.is_empty() {
            if let Some(mul) = build_mul(&mut input) {
                muls.push(mul);
            }
        }

        muls.into_iter()
            .map(|mul| mul.num1 * mul.num2)
            .sum::<i32>()
            .to_string()
    }

    fn part_2(&mut self, input: String) -> String {
        let mut mul_enabled = true;
        let mut muls = Vec::new();
        let mut input = input.as_str();

        const MUL: &str = "mul";
        const DO: &str = "do()";
        const DONOT: &str = "don't()";

        while !input.is_empty() {
            match (&mut input, mul_enabled) {
                (input, true) if input.starts_with(MUL) => {
                    if let Some(mul) = build_mul(input) {
                        muls.push(mul);
                    }
                }
                (input, false) if input.starts_with(MUL) => _ = shrink_str(input, MUL.len()),
                (input, _) if input.starts_with(DO) => {
                    _ = shrink_str(input, DO.len());
                    mul_enabled = true;
                }
                (input, _) if input.starts_with(DONOT) => {
                    _ = shrink_str(input, DONOT.len());
                    mul_enabled = false;
                }
                (input, _) => _ = shrink_str(input, 1),
            }
        }

        muls.into_iter()
            .map(|mul| mul.num1 * mul.num2)
            .sum::<i32>()
            .to_string()
    }
}

fn build_mul(input: &mut &str) -> Option<Mul> {
    if shrink_str(input, 1) != Some("m")
        || shrink_str(input, 1) != Some("u")
        || shrink_str(input, 1) != Some("l")
    {
        return None;
    }

    let open_bracket = shrink_str(input, 1).and_then(|s| s.chars().next())?;

    if open_bracket != '(' {
        return None;
    }

    let num1 = shrink_str(
        input,
        input.chars().take_while(|c| c.is_ascii_digit()).count(),
    )
    .and_then(|s| s.parse::<i32>().ok())?;

    let comma = shrink_str(input, 1).and_then(|s| s.chars().next())?;

    if comma != ',' {
        return None;
    }

    let num2 = shrink_str(
        input,
        input.chars().take_while(|c| c.is_ascii_digit()).count(),
    )
    .and_then(|s| s.parse::<i32>().ok())?;

    let closed_bracket = shrink_str(input, 1).and_then(|s| s.chars().next())?;

    if closed_bracket != ')' {
        return None;
    }

    Some(Mul { num1, num2 })
}

fn shrink_str<'a>(s: &mut &'a str, amount: usize) -> Option<&'a str> {
    let slice = s.get(..amount)?;
    *s = s.get(amount..).unwrap_or("");

    Some(slice)
}
