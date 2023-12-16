use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(16);

struct Position(usize, usize);

pub fn part_one(input: &str) -> Option<u32> {
    let input = input.lines().collect_vec();
    let energized: HashSet<Position> = HashSet::new();
    dbg!(&input);
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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