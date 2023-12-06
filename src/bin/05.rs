use std::str::Split;

advent_of_code::solution!(5);

// Takes split and returns a ready to use vector with all the instructions
fn parse_mappings(input: Split<&str>) -> Vec<Vec<(u64, u64, u64)>> {
    let mut maps: Vec<Vec<(u64, u64, u64)>> = Vec::new();
    for map_input in input {
        let mut map: Vec<(u64, u64, u64)> = Vec::new();
        let lines = map_input.split('\n').skip(1);
        for line in lines {
            let mut line = line.split(' ');
            if let Ok(destination) = line.next().unwrap().parse::<u64>() {
                if let Ok(source) = line.next().unwrap().parse::<u64>() {
                    if let Ok(length) = line.next().unwrap().parse::<u64>() {
                        map.push((destination, source, length));
                    }
                }
            }
        }
        maps.push(map);
    }
    maps
}

// Bruteforce approach for part one
fn walk_mappings(seed: u64, mappings: &Vec<Vec<(u64, u64, u64)>>) -> u64 {
    let mut current = seed;
    for map in mappings {
        'map: for line in map {
            let (destination, source, lenght) = line;
            if current >= *source && current < (source + lenght) {
                let offset = current - source;
                current = destination + offset;
                break 'map;
            }
        }
    }
    current
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut input = input.split("\n\n");
    let mut results: Vec<u64> = Vec::new();
    let seeds = input.next().unwrap().split(' ').skip(1);
    let mappings = parse_mappings(input);
    for seed in seeds {
        let seed: u64 = seed.parse::<u64>().unwrap();
        results.push(walk_mappings(seed, &mappings));
    }
    Some(*results.iter().min().unwrap())
}

pub fn part_two(input: &str) -> Option<u64> {
    // Parse input
    let mut parts = input.split("\n\n");
    let inputs_str = parts.next().expect("Invalid input").trim();
    let inputs: Vec<u64> = inputs_str
        .split(':')
        .nth(1)
        .expect("Invalid input")
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();
    // Init seeds vec
    let mut seeds = Vec::new();
    // Populate seed ranges
    for i in (0..inputs.len()).step_by(2) {
        seeds.push((inputs[i], inputs[i] + inputs[i + 1]));
    }
    let blocks = parse_mappings(parts);
    for ranges in blocks {
        // Reset new seed vec
        let mut new_seeds = Vec::new();
        // Iterate over all seeds
        'seeds: while let Some((seed_start, seed_end)) = seeds.pop() {
            for range in &ranges {
                let (destination, source, lenght) = *range;
                let overlapping_start = seed_start.max(source);
                let overlapping_end = seed_end.min(source + lenght);
                if overlapping_start < overlapping_end {
                    new_seeds.push((
                        overlapping_start - source + destination,
                        overlapping_end - source + destination,
                    ));
                    if overlapping_start > seed_start {
                        seeds.push((seed_start, overlapping_start));
                    }
                    if seed_end > overlapping_end {
                        seeds.push((overlapping_end, seed_end));
                    }
                    continue 'seeds;
                }
            }
            // If seed range is not overlapping with instructions just push initial values
            new_seeds.push((seed_start, seed_end));
        }
        // Replace seed for new instruction block
        seeds = new_seeds;
    }
    // Return the minimun destination (final seed vector)
    let min_seed = seeds
        .iter()
        .min_by_key(|&(x, _)| x)
        .expect("No seeds found");
    Some(min_seed.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        // Normalize the input String
        let input = &input.replace("\r\n", "\n");
        let result = part_one(input);
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        // Normalize the input String
        let input = &input.replace("\r\n", "\n");
        let result = part_two(input);
        assert_eq!(result, Some(46));
    }
}
