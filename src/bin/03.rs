use std::collections::HashSet;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut result = 0;
    let engine: Vec<&str> = input.split('\n').collect();
    let mut numbers: Vec<(usize, char)> = Vec::new();
    let total_rows = engine.len() - 1;
    for (row_index, &row_content) in engine.iter().enumerate() {
        let row_lenght = row_content.len() - 1;
        // Clear numbers for safety
        numbers.clear();
        'column_loop: for (index, symbol) in row_content.chars().enumerate() {
            if symbol.is_numeric() {
                numbers.push((index, symbol));
            }
            if !numbers.is_empty() && !symbol.is_numeric() || index == row_lenght {
                for (column, _) in numbers.clone() {
                    for yoffset in 0..=2 {
                        if row_index == 0 && yoffset == 0 {
                            continue;
                        }
                        if row_index == total_rows && yoffset == 2 {
                            continue;
                        }
                        let check_row = row_index + yoffset - 1;
                        for xoffset in 0..=2 {
                            if column == 0 && xoffset == 0 {
                                continue;
                            }
                            if column == row_lenght && xoffset == 2 {
                                continue;
                            }
                            let check_column = column + xoffset - 1;

                            let checked_symbol =
                                engine[check_row].chars().nth(check_column).unwrap();
                            if !checked_symbol.is_alphanumeric() && checked_symbol != '.' {
                                let number_string: String = numbers
                                    .clone()
                                    .into_iter()
                                    .map(|(_, digit)| digit.to_string())
                                    .collect();
                                let number = number_string.parse::<u32>().unwrap();
                                result += number;
                                numbers.clear();
                                continue 'column_loop;
                            }
                        }
                    }
                }
                // Did not find any symbol
                numbers.clear();
            }
        }
    }
    Some(result)
}

// Helperfunction for part two
fn get_full_number(start_number: char, start_index: usize, row_content: &str) -> u32 {
    let mut numbers: Vec<(usize, char)> = Vec::new();
    // Add first number
    numbers.push((start_index, start_number));
    let offsets: [i32; 2] = [-1, 1];
    // Populate numbers based on row content and the start_index
    for offset in offsets {
        let mut cursor = start_index as i32 + offset;
        loop {
            if cursor < 0 || cursor >= row_content.len() as i32 {
                break;
            }
            let to_check = row_content.chars().nth(cursor as usize).unwrap();
            if !to_check.is_alphanumeric() {
                break;
            }
            numbers.push((cursor as usize, to_check));
            cursor += offset;
        }
    }
    numbers.sort_by_key(|&(index, _)| index);
    let number_string: String = numbers
        .clone()
        .into_iter()
        .map(|(_, digit)| digit.to_string())
        .collect();
    number_string.parse::<u32>().unwrap()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result = 0;
    let engine: Vec<&str> = input.split('\n').collect();
    let total_rows = engine.len() - 1;
    for (row_index, &row_content) in engine.iter().enumerate() {
        let row_lenght = row_content.len() - 1;
        for (column, symbol) in row_content.chars().enumerate() {
            // Check for gear symbol
            if symbol == '*' {
                let mut numbers: Vec<u32> = Vec::new();
                for yoffset in 0..=2 {
                    if row_index == 0 && yoffset == 0 || row_index == total_rows && yoffset == 2 {
                        continue;
                    }
                    let check_row = row_index + yoffset - 1;
                    for xoffset in 0..=2 {
                        if column == 0 && xoffset == 0 || column == row_lenght && xoffset == 2 {
                            continue;
                        }
                        let check_column = column + xoffset - 1;
                        let checked_symbol = engine[check_row].chars().nth(check_column).unwrap();
                        if checked_symbol.is_numeric() {
                            // Hit number around gear
                            let number = get_full_number(
                                checked_symbol,
                                check_column,
                                engine.get(check_row).unwrap(),
                            );
                            numbers.push(number);
                        }
                    }
                }
                // Delete duplicate numbers
                let mut unique_set = HashSet::new();
                numbers.retain(|&num| unique_set.insert(num));
                // When gear has 2 adjacent numbers add their product
                if numbers.len() == 2 {
                    result += numbers.iter().product::<u32>();
                }
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
        assert_eq!(result, Some(4389));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
