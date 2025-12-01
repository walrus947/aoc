advent_of_code::solution!(3);

use regex::Regex;
use once_cell::sync::Lazy;

enum Instruction {
    Do,
    Dont,
    Mul(usize, usize)
}

static REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(mul|do|don't)\(((?:\d|,)*?)\)").unwrap());

fn parse_statements(input: &str) -> Vec<Instruction> {
    REGEX
        .captures_iter(input)
        .filter_map(|statement| {
            let (_, [instruction, val]) = statement.extract();

            match instruction {
                "mul" => {
                    let (val1, val2) = val.split_once(",")?;
                    Some(Instruction::Mul(val1.parse().ok()?, val2.parse().ok()?))
                }
                "do" => Some(Instruction::Do),
                "don't" => Some(Instruction::Dont),
                _ => None,
            }

        })
    .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let sum = parse_statements(input)
        .into_iter()
        .filter_map(|instr| match instr {
            Instruction::Mul(val1, val2) => Some(val1 * val2),
            Instruction::Do => None,
            Instruction::Dont => None,
        })
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut sum = 0;
    let statements = parse_statements(input);
    let mut doing = true;

    for statement in statements {
        match statement {
            Instruction::Do => {
                doing = true;
                log::debug!("Entering do statement");
            },
            Instruction::Dont => {
                doing = false;
                log::debug!("Leaving do statement");
            },
            Instruction::Mul(val1, val2) => {
                log::debug!("Entering mul with doing = {}", doing);
                if doing {
                    sum += val1 * val2;
                    log::debug!("Mul done with doing = {}", doing);
                }
            }
        }
    }

    Some(sum)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

/*
 * old solution before refactor
 *
pub fn part_one(input: &str) -> Option<usize> {
    let mult_pattern = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let num_pattern = Regex::new(r"\d+,\d+").unwrap();

    let mut sum: usize = 0;
    for res in mult_pattern.captures_iter(input) {
        let mult = res.get(0).unwrap().as_str();
        let mut vals: Vec<_> = num_pattern
            .captures(mult)
            .unwrap()
            .get(0)
            .unwrap()
            .as_str()
            .split(",")
            .collect::<Vec<_>>();

        assert_eq!(vals.len(), 2);
        let val1: usize = vals.pop().unwrap().parse().unwrap();
        let val2: usize = vals.pop().unwrap().parse().unwrap();
        sum += (val1 * val2);
    }

    Some(sum)

}

pub fn part_two(input: &str) -> Option<usize> {
    let do_regex = Regex::new(r"(^([\s\S]*?)don't\(\)|do\(\)([\s\S]*?)don't\(\))").unwrap();
    let mult_pattern = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let num_pattern = Regex::new(r"\d+,\d+").unwrap();

    let mut sum: usize = 0;
    for res in do_regex.captures_iter(input) {
        let do_str = res.get(0).unwrap().as_str();
        let mult = mult_pattern
            .captures(do_str)
            .unwrap()
            .get(0)
            .unwrap()
            .as_str();

        let mut vals: Vec<_> = num_pattern
            .captures(mult)
            .unwrap()
            .get(0)
            .unwrap()
            .as_str()
            .split(",")
            .collect::<Vec<_>>();

        assert_eq!(vals.len(), 2);
        let val1: usize = vals.pop().unwrap().parse().unwrap();
        let val2: usize = vals.pop().unwrap().parse().unwrap();
        sum += (val1 * val2);
    }

    Some(sum)

}

 */
