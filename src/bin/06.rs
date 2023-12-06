advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut input = input.split('\n');
    let times = input.next().unwrap().split_whitespace().skip(1);
    let distances = input.next().unwrap().split_whitespace().skip(1);
    let mut combinations = 1;
    for (time, record) in times.zip(distances) {
        let time: u32 = time.parse::<u32>().unwrap();
        let record: u32 = record.parse::<u32>().unwrap();
        let mut winnings = 0;
        for velocity in 2..time {
            let time = time - velocity;
            let distance = time * velocity;
            if distance > record {
                winnings += 1;
            }
        }
        if winnings != 0 {
            combinations *= winnings;
        }
    }
    if combinations == 1 {
        combinations = 0;
    }
    Some(combinations)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut input = input.split('\n');
    let time: String = input.next().unwrap().split_whitespace().skip(1).collect();
    let record: String = input.next().unwrap().split_whitespace().skip(1).collect();
    let time: u64 = time.parse::<u64>().unwrap();
    let record: u64 = record.parse::<u64>().unwrap();
    let mut winnings = 0;
    for velocity in 2..time {
        let time = time - velocity;
        let distance = time * velocity;
        if distance > record {
            winnings += 1;
        }
    }
    Some(winnings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
