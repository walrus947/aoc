use std::{io, fs};

const WORDS_TO_INT: [(&str, &str); 9] = [
    ("one", "o1e"),
    ("two", "t2o"),
    ("three", "thre3"),
    ("four", "fou4"),
    ("five", "fiv5"),
    ("six", "si6"),
    ("seven", "seve7"),
    ("eight", "eigh8"),
    ("nine", "nin9")
];

fn part1(input: &str) -> usize {
    let digits = input
        .chars()
        .filter(|chr| chr.is_digit(10))
        .collect::<Vec<_>>();

    (digits.first().unwrap().to_digit(10).unwrap() * 10
        + digits.last().unwrap().to_digit(10).unwrap()) as usize
}
fn part2(input: &str) -> usize {
    let mut line = input.to_owned();

    for (word, number) in WORDS_TO_INT {
        line = line.replace(word, number);
    }

    return part1(&line)
}

fn main() -> io::Result<()>  {
    let file = fs::read_to_string("./day1.input")?;
    let lines = file.lines();

    let part1: usize = lines.clone().map(part1).sum();
    let part2: usize = lines.clone().map(part2).sum();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}
