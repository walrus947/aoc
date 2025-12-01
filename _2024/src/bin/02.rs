use std::{fs::{read_to_string}, io};
use num::{signum};

const CEILING: usize = 3;
const FLOOR: usize = 0; //must be exclusively increasing/decreasing

fn part1(report: &str) -> usize {
    let levels: Vec<_> = report.split(' ').collect::<Vec<_>>();
    let mut debug_diffs: Vec<isize> = Vec::new();
    let mut past_level: isize = isize::MIN;
    let mut order: isize = 0;

    for level in &levels {
        let level: isize = level.trim().parse().unwrap();
        if past_level == isize::MIN { //first pass to get two vals
            past_level = level;
            continue;
        }

        let abs_diff: usize = isize::abs_diff(level, past_level);
        let is_diff_inbounds: bool = abs_diff <= CEILING && abs_diff > FLOOR;
        debug_diffs.push(level - past_level);
        let is_order_enforced: bool = signum(level - past_level) == signum(order); //second pass
                                                                                   //will always
                                                                                   //be false
        if (!is_diff_inbounds) || (!is_order_enforced && order != 0) {
            log::error!("Level: ({}), Order: {} -> FAIL", report, order);
            log::debug!("Diffs: ({:?})", debug_diffs);
            return 0;
        }
        order = level - past_level;
        past_level = level;
    }

    log::info!("Level: ({}), Order: {} -> PASS", report, order);
    log::debug!("Diffs: ({:?})", debug_diffs);
    return 1;
}

//part2 does not work. it is failing 6 cases

//now, a report may have a single error before failing -> but we have to retry with removing
//element, order is now established by the third level since element 2 can be wrong
fn part2(report: Vec<&str>, mut errors: usize) -> usize {
    let mut debug_diffs: Vec<isize> = Vec::new();
    let mut past_level: isize = isize::MIN;
    let mut order: isize = 0;

    for level in &report {
        let level = level.parse().unwrap();
        if past_level == isize::MIN { //first pass to get two vals
            past_level = level;
            continue;
        }

        let abs_diff: usize = isize::abs_diff(level, past_level);
        let is_diff_inbounds: bool = abs_diff <= CEILING && abs_diff > FLOOR;
        debug_diffs.push(level - past_level);
        let is_order_enforced: bool = signum(level - past_level) == signum(order);
        let invalid_level: bool = (!is_diff_inbounds) || (!is_order_enforced && order != 0);
        let invalid_first_pair: bool = !is_diff_inbounds && order == 0;

        //if the first pair is invalid, either the first or second level could be wrong
        if invalid_first_pair {
            if errors == 1 {
                return 0;
            }

            log::warn!("FIRST ERROR at FIRST PAIR -> ({}, {})", past_level, level);
            let mut without_first: Vec<&str> = report.clone();
            let mut without_second: Vec<&str> = report.clone();

            without_first.remove(0);
            without_second.remove(1);

            log::info!("WITHOUT FIRST -> {:?}", without_first);
            log::info!("WITHOUT SECOND -> {:?}", without_second);

            let result = part2(without_first, 1) + part2(without_second, 1);
            if result >= 1 {
                return 1;
            } else {
                return 0;
            }

        }

        if invalid_level {
            if errors == 0 { //if this is the first error
                log::warn!("FIRST ERROR -> ({}, {})", past_level, level);
                errors = 1;
                continue;
            }

            log::error!("Level: ({:?}), Order: {} -> FAIL", report, order);
            log::debug!("Diffs: ({:?})", debug_diffs);
            return 0;
        }
        order = level - past_level;
        past_level = level;
    }

    log::info!("Level: ({:?}), Order: {} -> PASS", report, order);
    log::debug!("Diffs: ({:?})", debug_diffs);
    return 1;
}

fn main() -> io::Result<()>  {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let file = read_to_string("./day2.input")?;
    let lines = file.lines();

    //let part1: usize = lines.clone().map(part1).sum();
    let mut part2_sum: usize = 0;
    lines.for_each(|report| {
       part2_sum += part2(report.split(' ').collect::<Vec<_>>(), 0);
    });

    //log::info!("Part 1: {}", part1);
    log::info!("Part 2: {}", part2_sum);
    //log::info!("Part 2: {}", part2);


    Ok(())
}
