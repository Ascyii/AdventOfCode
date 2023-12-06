use std::str::Split;

advent_of_code::solution!(5);

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

pub fn part_two(_input: &str) -> Option<u64> {
    None
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
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
