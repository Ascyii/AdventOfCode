advent_of_code::solution!(2);

const BAG_SIZES: [u32; 3] = [12, 13, 14];

pub fn part_one(input: &str) -> Option<u32> {
    let mut result: u32 = 0;
    'outer: for line in input.split('\n') {
        let mut raw_args = line.split(": ");
        let game_number = raw_args
            .next()
            .and_then(|game| game.split_whitespace().last())
            .map(|num_str| num_str.parse::<u32>());
        if let Some(args) = raw_args.next() {
            for arg_bag_bundle in args.split("; ") {
                let bag_arg = arg_bag_bundle.split(", ");
                for bagg in bag_arg {
                    let mut bag = bagg.split(' ');
                    if let Some(amount) = bag.next() {
                        let color = bag.next();
                        let max_amount = match color.unwrap() {
                            "red" => BAG_SIZES[0],
                            "green" => BAG_SIZES[1],
                            "blue" => BAG_SIZES[2],
                            _ => 100,
                        };

                        if let Ok(amount_number) = amount.parse::<u32>() {
                            if amount_number > max_amount {
                                continue 'outer;
                            }
                        }
                    }
                }
            }
            // If game check passed add game number to result
            if let Some(Ok(game_n)) = game_number {
                result += game_n;
            }
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result: u32 = 0;
    for line in input.split('\n') {
        let mut raw_args = line.split(": ");
        // Iterate over arguments for the game
        if let Some(args) = raw_args.nth(1) {
            // Reset max cubes
            let mut max_red = 0;
            let mut max_blue = 0;
            let mut max_green = 0;
            // Iterate over different bags
            for arg_bag_bundle in args.split("; ") {
                // Get arguments for the bags
                let bag_arg = arg_bag_bundle.split(", ");
                // Iterate over bags
                for bag in bag_arg {
                    let mut bag_content = bag.split(' ');
                    // Unpack cube amount
                    if let Some(amount) = bag_content.next() {
                        // Get color from bag content
                        let color = bag_content.next();
                        // Parse amount to number for comparison
                        if let Ok(cubes) = amount.parse::<u32>() {
                            match color.unwrap() {
                                // Check whether max bags need to be reset
                                "red" => {
                                    if cubes > max_red {
                                        max_red = cubes
                                    }
                                }
                                "green" => {
                                    if cubes > max_green {
                                        max_green = cubes
                                    }
                                }
                                "blue" => {
                                    if cubes > max_blue {
                                        max_blue = cubes
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
            // Calculate power and add to the result
            result += max_red * max_green * max_blue;
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
        assert_eq!(result, Some(526));
    }
}
