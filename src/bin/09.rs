advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i64> {
    let mut result: i64 = 0;
    for line in input.lines() {
        let mut results: Vec<i64> = Vec::new();
        let mut values: Vec<i64> = line
            .split_whitespace()
            .map(|v| v.parse::<i64>().unwrap())
            .collect();
        while !values.iter().all(|&v| v == 0) {
            results.push(*values.last().unwrap());
            values = values.windows(2).map(|w| w[1] - w[0]).collect();
        }
        result += results.iter().sum::<i64>();
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut result: i64 = 0;
    for line in input.lines() {
        let mut results: Vec<i64> = Vec::new();
        let mut values: Vec<i64> = line
            .split_whitespace()
            .map(|v| v.parse::<i64>().unwrap())
            .collect();
        while !values.iter().all(|&v| v == 0) {
            results.push(*values.first().unwrap());
            values = values.windows(2).map(|w| w[1] - w[0]).collect();
        }
        // Calculate first
        let mut first: i64 = 0;
        for n in results.iter().rev() {
            first = n - first;
        }
        result += first;
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
