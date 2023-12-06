advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let cards = input.split('\n');
    let mut score = 0;
    for card in cards {
        let arguments: String = card.split(':').skip(1).collect();
        let mut arguments = arguments.split('|');
        let mut card_score = 0;
        let winnings: Vec<&str> = arguments.next().unwrap().split_whitespace().collect();
        let hand: Vec<&str> = arguments.next().unwrap().split_whitespace().collect();
        for number in hand {
            if winnings.contains(&number) {
                if card_score == 0 {
                    card_score += 1;
                } else {
                    card_score *= 2;
                }
            }
        }
        score += card_score;
    }
    Some(score)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut cards: Vec<(u64, &str)> = input.split('\n').map(|card| (1, card)).collect();
    for card_index in 0..cards.len() {
        let card = cards[card_index];
        let amount = card.0;
        let arguments: String = card.1.split(':').skip(1).collect();
        let mut arguments = arguments.split('|');
        let mut card_score = 0;
        let winnings: Vec<&str> = arguments.next().unwrap().split_whitespace().collect();
        let hand: Vec<&str> = arguments.next().unwrap().split_whitespace().collect();
        for number in hand {
            if winnings.contains(&number) {
                card_score += 1;
            }
        }
        // Update new card amounts
        if card_score != 0 {
            for i in 1..=card_score {
                if let Some((old_amount, _)) = cards.get_mut(i + card_index) {
                    *old_amount += amount;
                }
            }
        }
    }
    // Calculate size of card pile
    Some(cards.iter().map(|(amount, _)| amount).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
