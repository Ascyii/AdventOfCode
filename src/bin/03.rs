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
                        if row_index == 0 && yoffset == 0 { continue; }
                        if row_index == total_rows && yoffset == 2 { continue; }
                        let check_row = row_index  + yoffset - 1;
                        for xoffset in 0..=2 {
                            if column == 0 && xoffset == 0 { continue; }
                            if column == row_lenght && xoffset == 2 { continue; }
                            let check_column = column  + xoffset - 1;

                            let checked_symbol = engine[check_row].chars().nth(check_column).unwrap();
                            if !checked_symbol.is_alphanumeric() && checked_symbol != '.' {
                                let number_string: String = numbers.clone().into_iter().map(|(_, digit)| digit.to_string()).collect();
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

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
