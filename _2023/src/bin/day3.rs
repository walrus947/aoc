use std::{io::{self, Read}, fs, usize};
use itertools::Itertools;

/*
 * Possible solutions:
 *  1.) Walk the file until you reach a symbol. dfs on a symbol to find adjacent numbers and add to
 *    list. add all the parsed numbers together to get a sum
 *  2.) Walk the file taking note of a number. Once you get a number, dfs around the entire number
 *    for a symbol. If a symbol is found, add it 
 *  3.) dfs on every number and stack add the number until you reach a new period. Once you reach a
 *    new period, see if the number should be added.
 *
 *  File does not give the correct answer. Bug not found
 */
fn main() -> io::Result<()>  {
    let mut input = fs::File::open("./day3.input")?;
    let mut contents = String::new();

    let _ = input.read_to_string(&mut contents);

    let input_vec: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<_>>();

    let mut possible_num: String = String::new();
    let mut symbol_adj: bool = false;
    let mut sum: i32 = 0;

    for row in 0..input_vec.len() {
        let line = &input_vec[row];
        for col in 0..line.len() {
            let char = line[col];

            if char.is_digit(10) {
                possible_num.push(char);
                if is_symbol_adj(row, col, &input_vec) && !symbol_adj {
                    symbol_adj = true;
                }
            } else if possible_num.len() != 0 {
                if symbol_adj {
                    sum += possible_num.parse::<i32>().unwrap();
                    //println!("{} has been added to sum", possible_num);
                }
                symbol_adj = false;
                possible_num = "".to_string();
            }
        }
    }

    println!("{}", sum);
    Ok(())
}

fn is_symbol_adj(row: usize, col: usize, input_vec: &Vec<Vec<char>>) -> bool {
    let mut symbol_found = false;
    let row = row as i32;
    let col = col as i32;

    for (i, j) in [row - 1, row, row + 1]
        .iter()
        .cartesian_product([col - 1, col, col + 1]) {
            if let Some(c) = input_vec
                .get(*i as usize)
                    .and_then(|line| line.get(j as usize)) {

                if !c.is_digit(10) && c != &'.' {
                    symbol_found = true;
                    break;
                }
            }
    }

    return symbol_found;
}
