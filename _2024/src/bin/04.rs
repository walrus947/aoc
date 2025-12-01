advent_of_code::solution!(4);

use glam::IVec2;
use std::collections::HashMap;

const DIRECTIONS: [[IVec2; 3]; 8] = [
    //east
    [
        IVec2::new(0, 1),
        IVec2::new(0, 2),
        IVec2::new(0, 3),
    ],
    //northeast
    [
        IVec2::new(1, 1),
        IVec2::new(2, 2),
        IVec2::new(3, 3),
    ],
    //north
    [
        IVec2::new(1, 0),
        IVec2::new(2, 0),
        IVec2::new(3, 0),
    ],
    //northwest
    [
        IVec2::new(-1, 1),
        IVec2::new(-2, 2),
        IVec2::new(-3, 3),
    ],
    //west
    [
        IVec2::new(-1, 0),
        IVec2::new(-2, 0),
        IVec2::new(-3, 0),
    ],
    //southwest
    [
        IVec2::new(-1, -1),
        IVec2::new(-2, -2),
        IVec2::new(-3, -3),
    ],
    //south
    [
        IVec2::new(0, -1),
        IVec2::new(0, -2),
        IVec2::new(0, -3),
    ],
    //southeast
    [
        IVec2::new(1, -1),
        IVec2::new(2, -2),
        IVec2::new(3, -3),
    ],
];


const PART2_DIRECTIONS: [[IVec2; 2]; 4] = [
    [
        IVec2::new(1, 1),
        IVec2::new(-1, -1),
    ],
    [
        IVec2::new(-1, 1),
        IVec2::new(1, -1),
    ],
    [
        IVec2::new(-1, -1),
        IVec2::new(1, 1),
    ],
    [
        IVec2::new(1, -1),
        IVec2::new(-1, 1),
    ],
];

/* find each occurance of x and use the direction offsets to get every
 * cardinal vector with x; filter that for xmas and count the number
 * of correct vectors
 */
pub fn part_one(input: &str) -> Option<u32> {
    let coordinates = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, val)| {
                (IVec2::new(x as i32, y as i32), val)
            })
        })
        .collect::<HashMap<IVec2, char>>();

    let x_mas = ['M', 'A', 'S'];
    let result: usize = coordinates
        .iter()
        .filter(|(_pos, val)| **val == 'X')
        .map(|(pos, val)| {
            let total = DIRECTIONS
                .iter()
                .map(|x_mas_pos| {
                    x_mas_pos
                        .iter()
                        .map(|offset| {
                            coordinates.get(&(pos + offset))
                        })
                        .enumerate()
                            .all(|(idx, val)| {
                                x_mas.get(idx) == val
                            })
                })
                .filter(|bool| *bool)
                .count();
            total
        }).sum();
    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let coordinates = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, val)| {
                (IVec2::new(x as i32, y as i32), val)
            })
        })
        .collect::<HashMap<IVec2, char>>();

    let x_mas = ['M', 'S'];
    let result: usize = coordinates
        .iter()
        .filter(|(_pos, val)| **val == 'A')
        .filter(|(pos, _val)| {
            PART2_DIRECTIONS
                .iter()
                .map(|x_mas_pos| {
                    x_mas_pos
                        .iter()
                        .map(|offset| {
                            coordinates.get(&(*pos + offset))
                        })
                        .enumerate()
                        .all(|(idx, val)| {
                            x_mas.get(idx) == val
                        })
                })
                .filter(|bool| *bool)
                .count() == 2
        }).count();

    Some(result as u32)
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
