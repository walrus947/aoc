advent_of_code::solution!(5);

use std::collections::{HashMap};
use std::cmp;


fn create_rules(input: &str) -> HashMap<u32, Vec<u32>> {
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    input.lines().for_each(|line| {
        let mut split_iter = line.split("|");
        let num: u32 = split_iter.next().unwrap().parse().unwrap();
        let rule: u32 = split_iter.next().unwrap().parse().unwrap();

        let mut num_rules: Vec<u32> = Vec::new();
        if rules.contains_key(&num) {
            num_rules = rules.remove(&num).unwrap();
        }

        num_rules.push(rule);
        rules.insert(num, num_rules);

    });

    return rules;
}

fn process_update(update: &str, rules: &HashMap<u32, Vec<u32>>) -> u32 {
    let mut cur_pages: Vec<u32> = Vec::new();

    let pages = update.split(",");
    for str_page in pages {
        let page: u32 = str_page.parse().unwrap();
        if !rules.contains_key(&page) { cur_pages.push(page); continue; }
        let rule: &Vec<u32> = rules.get(&page).unwrap();
        let allowed_page: bool = rule.iter().all(|element| {
           !cur_pages.contains(element)
        });

        if !allowed_page {
            println!("FAIL at {} with Rule {:?} -> Page: {}", str_page, rule, update);
            return 0;
        }
        cur_pages.push(page);
    }

    let middle_page: u32 = *cur_pages.get(cur_pages.len() / 2).unwrap();
    println!("SUCCESS -> Page: {:?} with middle {}",cur_pages, middle_page);
    return middle_page;
}

fn correct_update(update: &Vec<u32>, rules: &HashMap<u32, Vec<u32>>) -> u32 {
    let mut cur_pages: Vec<u32> = Vec::new();
    cur_pages.push(*update.get(0).unwrap());
    println!("cur_pages: {:?}", cur_pages);
    let mut idx: usize = 1;

    while idx < update.len() {
        let page: u32 = *update.get(idx).unwrap();
        if !rules.contains_key(&page) {
            cur_pages.push(page);
            idx += 1;
            continue;
        }
        let rule: &Vec<u32> = rules.get(&page).unwrap();

        let mut ydx: usize = 0;

        while ydx < cur_pages.len() {
            let past_page = cur_pages.get(ydx).unwrap();
            let past_rule = rules.get(&page).unwrap();
            if rule.contains(past_page) || past_rule.contains(&page) {
                println!("{} MUST GO BEFORE {}", page, past_page);
                break;
            }
            ydx += 1;
        }

        cur_pages.insert(ydx, page);
        println!("cur_pages: {:?}", cur_pages);
        idx += 1;
    }

    let middle_page: u32 = *cur_pages.get(cur_pages.len() / 2).unwrap();
    println!("SUCCESS AFTER FAILURE -> Page: {:?} with middle {} and len {}",
             cur_pages, middle_page, cur_pages.len());
    return middle_page;
}

fn process_update_part2(update: &str, rules: &HashMap<u32, Vec<u32>>) -> u32 {
    let mut cur_pages: Vec<u32> = Vec::new();

    let pages = update.split(",");
    for str_page in pages {
        let page: u32 = str_page.parse().unwrap();
        if !rules.contains_key(&page) { cur_pages.push(page); continue; }
        let rule: &Vec<u32> = rules.get(&page).unwrap();
        let allowed_page: bool = rule.iter().all(|element| {
           !cur_pages.contains(element)
        });

        if !allowed_page {
            let update_vec: Vec<u32> = update
                .split(",")
                .map(|element| element.trim().parse().unwrap())
                .collect::<Vec<_>>();

            return correct_update(&update_vec, rules);
        }
        cur_pages.push(page);
    }

    return 0;
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut parts_iter = input.split("\n\n");
    let rules: HashMap<u32, Vec<u32>> = create_rules(parts_iter.next().unwrap());
    let mut sum: u32 = 0;
    parts_iter
        .next()
        .unwrap()
        .lines()
        .for_each(|update| {
            sum += process_update(update, &rules);
        });
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut parts_iter = input.split("\n\n");
    let rules: HashMap<u32, Vec<u32>> = create_rules(parts_iter.next().unwrap());
    let mut sum: u32 = 0;
    parts_iter
        .next()
        .unwrap()
        .lines()
        .for_each(|update| {
            sum += process_update_part2(update, &rules);
        });
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
