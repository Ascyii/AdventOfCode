// Helpers
fn extract_numbers(input_str: &str) -> Vec<(usize, u32)> {
    input_str
        .chars()
        .enumerate()
        .filter_map(|(index, c)| {
            if c.is_numeric() {
                Some((index, c.to_digit(10).unwrap()))
            } else { None }
        })
        .collect()
}
fn concatenating_numbers(num1: &u32, num2: &u32) -> u32 {
    let merged_str = format!("{}{}", num1, num2);
    let merged_num: u32 = merged_str.parse().unwrap();
    merged_num
}
const NUMBER_STRINGS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut result: u32 = 0;
    for line in input.split("\n") {
        let numbers = extract_numbers(line);
        // Get the first number
        if let Some((_, first)) = numbers.first() {
            if let Some((_, last)) = numbers.last() {
                result += concatenating_numbers(first, last);
            }
        }

    }
    
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result: u32 = 0;
    for line in input.split("\n") {
        // Init numbers vec with all numbers
        let mut numbers = extract_numbers(line);
        // Get numbers written out
        for (index, &number_string) in NUMBER_STRINGS.iter().enumerate() {
            let number = index + 1;
            for (index, _) in line.match_indices(number_string) {
                numbers.push((index, number as u32));
            }
        }
        // Sort numbers dependen where there are in the line
        numbers.sort_by_key(|&(index, _)| index);
        // Get first and last and merge them together
        if let Some((_, first)) = numbers.first() {
            if let Some((_, last)) = numbers.last() {
                // Add number from line to restult
                result += concatenating_numbers(first, last);
            }
        }
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(42));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }
}
