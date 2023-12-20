use core::str;
use std::{io, fs, usize};

fn main() -> io::Result<()>  {
    let file = fs::read_to_string("./day2.input")?;
    let lines = file.lines();
    let mut count = 0;


    let (mut part1_total, mut part2_total) = (0, 0);

    lines.for_each(|line| {
        let (game_number, game_info) = line.trim_start_matches("Game ").split_once(':').unwrap();
        let mut game_highest = (0, 0, 0);
        game_info.trim().split([';', ',']).for_each(|round| {
            let (amount, color) = round.trim().split_once(' ').unwrap();
            let amount = amount.parse().unwrap();

            match color {
                "red" => {
                    if (amount > game_highest.0) {
                        game_highest.0 = amount;
                    }
                }

                "green" => {
                    if (amount > game_highest.1) {
                        game_highest.1 = amount;
                    }
                }

                "blue" => {
                    if (amount > game_highest.2) {
                        game_highest.2 = amount;
                    }
                }

                _ => { unreachable!(); }
            }
        });

        
        if (game_highest.0 <= 12 && game_highest.1 <= 13 && game_highest.2 <= 14) {
            part1_total += game_number.parse::<usize>().unwrap();
        }

        part2_total += (game_highest.0 * game_highest.1 * game_highest.2);

    });

    println!("{}", part1_total);
    println!("{}", part2_total);

    Ok(())
}

