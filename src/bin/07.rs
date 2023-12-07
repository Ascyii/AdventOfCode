use itertools::Itertools;
use std::collections::HashSet;

advent_of_code::solution!(7);

// Card to hold value and make it comparable
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Card(u8);
impl From<char> for Card {
    fn from(value: char) -> Self {
        let mut digit: u8 = 15;
        if value.is_numeric() {
            digit = value.to_digit(10).unwrap() as u8;
        } else if value == 'T' {
            digit = 10;
        } else if value == 'J' {
            digit = 11;
        } else if value == 'Q' {
            digit = 12;
        } else if value == 'K' {
            digit = 13;
        } else if value == 'A' {
            digit = 14;
        }
        if digit < 15 {
            Card(digit)
        } else {
            panic!()
        }
    }
}

// Hand as container for cards
struct Hand {
    cards: [Card; 5],
}
impl Hand {
    fn create(raw: &str, jokers: bool) -> Self {
        let cards_raw = raw.chars();
        let mut cards = [Card(0); 5];
        for (card, value) in cards.iter_mut().zip(cards_raw) {
            let mut value_new = value;
            if jokers && value == 'J' {
                value_new = '1';
            }
            *card = Card::from(value_new);
        }
        Hand { cards }
    }
    // Evaluate hand for poker scoring 1-7
    fn evaluate(&self, jokers: bool) -> u8 {
        let mut dups: Vec<u8> = Vec::new();
        let mut joker: u8 = 0;
        if jokers {
            joker = self
                .cards
                .iter()
                .filter(|&&x| x == Card(1))
                .count()
                .try_into()
                .unwrap();
        }
        for card in self.get_unique(jokers) {
            let dup: u8 = self
                .cards
                .iter()
                .filter(|&&x| x == card)
                .count()
                .try_into()
                .unwrap();
            dups.push(dup);
        }
        // Apply joker rule
        if joker != 0 {
            dups.sort();
            let mut dups_amount = dups.len();
            if dups_amount == 0 {
                dups_amount += 1;
                dups.push(0);
            }
            dups[dups_amount - 1] += joker;
        }
        // Check type of hand
        if dups.contains(&5) {
            return 7;
        }
        if dups.contains(&4) {
            return 6;
        }
        if dups.contains(&3) && dups.contains(&2) {
            return 5;
        }
        if dups.contains(&3) {
            return 4;
        }
        if dups.iter().filter(|&&x| x == 2).count() == 2 {
            return 3;
        }
        if dups.iter().filter(|&&x| x == 2).count() == 1 {
            return 2;
        }
        1
    }
    fn get_unique(&self, jokers: bool) -> Vec<Card> {
        // Convert the vector to a HashSet to remove duplicates
        let mut unique: HashSet<_> = self.cards.into_iter().collect();
        if jokers {
            unique.remove(&Card(1));
        }
        unique.into_iter().collect()
    }
    // Get identity to compare Hands from right to left
    fn get_identity(&self) -> u128 {
        let mut identity = String::new();
        for card in self.cards {
            let mut card_str = card.0.to_string();
            if card_str.len() == 1 {
                card_str.insert(0, '0');
            }
            identity += &card_str;
        }
        identity.parse::<u128>().unwrap()
    }
}

// Players struct contains hand and bidding value
struct Player {
    hand: Hand,
    value: u8,
    bid: u32,
}
impl Player {
    fn create(value: &str, jokers: bool) -> Self {
        let mut input = value.split(' ');
        // Parse fields from input
        let hand: Hand = Hand::create(input.next().unwrap(), jokers);
        let bid: u32 = input.next().unwrap().parse::<u32>().unwrap();
        let value: u8 = hand.evaluate(jokers);
        // Init player with rank of null
        Player { hand, value, bid }
    }
}

// Runs the game logic
struct Game {
    players: Vec<Player>,
}
impl Game {
    fn create(input: &str, jokers: bool) -> Self {
        let input = input.split('\n');
        let mut players: Vec<Player> = Vec::new();
        for player_arg in input {
            let player = Player::create(player_arg, jokers);
            players.push(player);
        }
        Game { players }
    }
    fn evaluate(&self) -> u32 {
        // Group players into hand values
        let mut player_groups: Vec<Vec<&Player>> = Vec::new();
        for score in (1..=7).rev() {
            player_groups.push(
                self.players
                    .iter()
                    .filter(|&player| player.value == score)
                    .collect_vec(),
            );
        }
        // Set starting rank
        let mut rank: u32 = self.players.len().try_into().unwrap();
        let mut result: u32 = 0;
        for group in player_groups.iter_mut() {
            // Sort players in group by their highcard
            group.sort_by(|a, b| b.hand.get_identity().cmp(&a.hand.get_identity()));
            for player in group {
                // Add multiply of rank and value to result
                result += player.bid * rank;
                // Next player is one rank down
                rank -= 1;
            }
        }
        result
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    // Create game from str
    let game = Game::create(input, false);
    // Evaluate the games value
    Some(game.evaluate())
}

pub fn part_two(input: &str) -> Option<u32> {
    // Run with joker flag
    let game = Game::create(input, true);
    Some(game.evaluate())
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
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        // Normalize the input String
        let input = &input.replace("\r\n", "\n");
        let result = part_two(input);
        assert_eq!(result, Some(5905));
    }
}
