use std::collections::HashMap;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<i32> {
    let mut start = ((0, 0), [(0, 0), (0, 0), (0, 0), (0, 0)]);
    let mappings: HashMap<(usize, usize), Option<[(i32, i32); 2]>> = input
        .lines()
        .enumerate()
        .flat_map(|(row_index, row)| {
            row.chars().enumerate().map(move |(col_index, value)| {
                let mut directions: Option<[(i32, i32); 2]> = Some([(0, 0), (0, 0)]);
                match value {
                    '.' => directions = None,
                    'J' => {
                        directions = Some([
                            (col_index as i32 - 1, row_index as i32),
                            (col_index as i32, row_index as i32 + 1),
                        ])
                    }
                    'L' => {
                        directions = Some([
                            (col_index as i32, row_index as i32 - 1),
                            (col_index as i32 + 1, row_index as i32),
                        ])
                    }
                    '|' => {
                        directions = Some([
                            (col_index as i32, row_index as i32 - 1),
                            (col_index as i32, row_index as i32 + 1),
                        ])
                    }
                    '-' => {
                        directions = Some([
                            (col_index as i32 - 1, row_index as i32),
                            (col_index as i32 + 1, row_index as i32),
                        ])
                    }
                    'F' => {
                        directions = Some([
                            (col_index as i32, row_index as i32 + 1),
                            (col_index as i32 + 1, row_index as i32),
                        ])
                    }
                    '7' => {
                        directions = Some([
                            (col_index as i32 - 1, row_index as i32),
                            (col_index as i32, row_index as i32 + 1),
                        ])
                    }
                    'S' => {
                        directions = {
                            start = ((col_index, row_index), [(0, 0), (0, 0), (0, 0), (0, 0)]);
                            None
                        }
                    }
                    _ => panic!(),
                }
                ((row_index, col_index), directions)
            })
        })
        .collect();
     dbg!(&mappings);
    None
}

pub fn part_two(_input: &str) -> Option<i32> {
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
