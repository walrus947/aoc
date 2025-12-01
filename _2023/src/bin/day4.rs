use std::{io::{self, Read}, fs, usize, collections::HashMap};

use itertools::Itertools;

const BASE: isize = 2;

fn main() -> io::Result<()>  {
    let mut input = fs::File::open("./day4.input")?;
    let mut contents = String::new();

    let _ = input.read_to_string(&mut contents);
    let lines = contents.lines();
    let mut winning_total = 0;
    let mut instance_map = HashMap::new();

    lines.for_each(|line| {
        let mut winning_matches = 0;

        let (winning_text, card_text) = line.split_once('|').unwrap();
        let (game_info, winning_text) = winning_text.split_once(':').unwrap();

        let game_number: usize = game_info
            .chars()
            .skip(5)
            .collect::<String>()
            .trim()
            .parse::<usize>()
            .unwrap();

        let winning_numbers: Vec<usize> = winning_text
            .split_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .collect_vec();

        card_text.trim().split_whitespace().for_each(|num| {
            if winning_numbers.contains(&num.parse::<usize>().unwrap()) {
                winning_matches = winning_matches + 1;
            } 
        });

        let count = instance_map.entry(game_number).or_insert(1).clone();
        for game_number in (game_number + 1usize)..(game_number + winning_matches + 1 as usize) {
            let entry = instance_map.entry(game_number).or_insert(1);
            *entry += count;
        }

        if winning_matches > 0 {
            winning_total = winning_total + BASE.pow((winning_matches - 1).try_into().unwrap());
        }

    });

    println!("Part 1 Total: {}", winning_total);
    println!("Part 2 Total: {}", instance_map.values().sum::<usize>());
    

    Ok(())
}


