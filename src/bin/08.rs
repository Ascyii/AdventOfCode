use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    // Create input split
    let mut input = input.split("\n\n");
    // Create the instruction loop
    let mut instructions = input.next().unwrap().chars().cycle();
    // Create the mappings in a fast hashmap
    let mappings: HashMap<&str, (&str, &str)> = input
        .next()
        .unwrap()
        .split('\n')
        .map(|line| {
            let mut line = line.split(" = ");
            let key = line.next().unwrap();
            let args: (&str, &str) = line.next().unwrap()[1..9]
                .split(", ")
                .collect_tuple()
                .unwrap();
            (key, args)
        })
        .collect();
    let mut current = "AAA";
    let mut count: u32 = 0;
    // Start the loop until we find the ZZZ node
    while current != "ZZZ" {
        let instruction = instructions.next().unwrap();
        let node = mappings[current];
        if instruction == 'R' {
            current = node.1;
        } else {
            current = node.0;
        }
        count += 1;
    }
    Some(count)
}

// Helperfunctions part 2
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}
fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}
fn lcm_of_vector(numbers: &Vec<u64>) -> u64 {
    if numbers.is_empty() {
        0
    } else {
        let mut result = numbers[0];

        for &num in &numbers[1..] {
            result = lcm(result, num);
        }

        result
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    // Create input split
    let mut input = input.split("\n\n");
    // Create the instruction loop
    let mut instructions = input.next().unwrap().chars().cycle();
    // Create the mappings in a fast hashmap
    let mappings: HashMap<&str, (&str, &str)> = input
        .next()
        .unwrap()
        .split('\n')
        .map(|line| {
            let mut line = line.split(" = ");
            let key = line.next().unwrap();
            let args: (&str, &str) = line.next().unwrap()[1..9]
                .split(", ")
                .collect_tuple()
                .unwrap();
            (key, args)
        })
        .collect();
    let starters: Vec<&str> = mappings
        .clone()
        .keys()
        .filter(|&&key| key.ends_with("A"))
        .copied()
        .collect();
    let mut results: Vec<u64> = Vec::new();
    // Start the loop until we find the ZZZ node

    // Start the loop until we find the ZZZ node
    for starter in starters {
        let mut current = starter.clone();
        let mut count: u64 = 0;
        while current != starter || count == 0 {
            if current.ends_with('Z') {
                break;
            }
            let instruction = instructions.next().unwrap();
            let node = mappings[current];
            if instruction == 'R' {
                current = node.1;
            } else {
                current = node.0;
            }
            count += 1;
        }
        results.push(count);
    }
    Some(lcm_of_vector(&results))
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
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, None);
    }
}
