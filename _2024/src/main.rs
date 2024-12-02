use std::{fs::{File}, io::{self, Read}, collections::{HashMap}};

fn part2(vec1: &Vec<i32>, vec2: &Vec<i32>) -> i32 {
    let mut hash_counter: HashMap<i32, i32> = vec1.iter().map(|x| (*x, i32::from(0))).collect();
    vec2.iter().for_each(|val| {
        if hash_counter.contains_key(val) {
            let mut count: i32 = *hash_counter.get(val).unwrap();
            count += 1;
            hash_counter.remove(val);
            hash_counter.insert(*val, count);
            log::debug!("({}, {}) -> ({}, {})", val, count, val, count + 1);
        }
    });

    let result2 = hash_counter.iter().map(|(key, value)| key * value).sum();
    return result2;
}

fn part1(input: &str) -> (i32, i32) {
    let mut vec1: Vec<i32> = vec![];
    let mut vec2: Vec<i32> = vec![];

    for line in input.lines() {
        let mut contents = line.split_whitespace();
        vec1.push(contents.next().unwrap().parse::<i32>().unwrap());
        vec2.push(contents.next().unwrap().parse::<i32>().unwrap());
    }

    vec1.sort();
    vec2.sort();

    let result: i32 = std::iter::zip(&vec1, &vec2)
        .map(|(left, right)| (left - right).abs())
        .sum();

    let result2: i32 = part2(&vec1, &vec2);

    return (result, result2);
}

fn main() -> io::Result<()>  {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let mut file = File::open("day1.input")?;
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents)?;

    let (part1, part2): (i32, i32) = part1(&contents);
    log::info!("Part 1: {}", part1);
    log::info!("Part 2: {}", part2);

    Ok(())
}

/*
 * abandoned solution for part1
 * refactoring just made the code worse, so rewriting with
 * new knowledge is just a better idea
fn pair_comparator(input1: &String, input2: &String) -> usize {
    log::debug!("({}, {}) -> ({}, {})", input1, input2, 
                input1.parse::<usize>().unwrap(), input2.parse::<usize>().unwrap());

    return usize::abs_diff(input1.parse::<usize>().unwrap(), input2.parse::<usize>().unwrap());
}

fn part1(vec: &mut Vec<Vec<String>>) -> usize {
    let vec1 = vec.first().unwrap();
    let vec2 = vec.last().unwrap();

    log::info!("Finding distances");

    let mut sum: usize = 0;
    //this is bad since we're popping all elements, so we'd need Vecs for
    //part 1 and part 2
    while (vec1.len() > 0 && vec1.len() == vec2.len()) {
        sum += pair_comparator(&vec1.pop().unwrap(), &vec2.pop().unwrap());
    }

    return sum;

}

fn get_vecs(input: &String) -> Vec<Vec<String>> {
    let mut vec1: Vec<_> = Vec::new();
    let mut vec2: Vec<_> = Vec::new();

    log::info!("Creating vectors");
    input.lines().for_each(|line| {
        let (entry1, entry2) = line.trim().split_once(' ').unwrap();
        let _ = vec1.push(entry1.trim().parse().unwrap());
        let _ = vec2.push(entry2.trim().parse().unwrap());
        log::debug!("({}, {})", entry1, entry2);
    });

    vec1.sort();
    vec2.sort();

    let mut vec: Vec<Vec<_>> = Vec::new();
    vec.push(vec1);
    vec.push(vec2);

    return vec;
}
*/

