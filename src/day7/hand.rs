use std::collections::HashMap;
use std::str::FromStr;

pub const CARDS: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Hand {
    pub cards: String,
    pub bid: u32,
    pub highest_hand: HandType,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Hand {
    pub fn get_highest_hand(&mut self) -> &HandType {
        let mut cards_map: HashMap<char, u32> = HashMap::new();
        for card in self.cards.chars() {
            if CARDS.contains(&card) {
                let count = cards_map.entry(card).or_insert(0);
                *count += 1;
            }
        }

        let mut cur_highest_hand_type: HandType = HandType::HighCard;
        for entry in cards_map.values() {
            cur_highest_hand_type = self.get_next_hand_type(&cur_highest_hand_type, *entry);
            if cur_highest_hand_type == HandType::FiveOfAKind
                || cur_highest_hand_type == HandType::FourOfAKind
                || cur_highest_hand_type == HandType::FullHouse
                || cur_highest_hand_type == HandType::TwoPair
            {
                break;
            }
        }

        self.highest_hand = cur_highest_hand_type;
        &self.highest_hand
    }

    fn get_next_hand_type(&self, cur_highest_hand_type: &HandType, count: u32) -> HandType {
        match cur_highest_hand_type {
            HandType::FiveOfAKind => HandType::FiveOfAKind,
            HandType::FourOfAKind => HandType::FourOfAKind,
            HandType::FullHouse => HandType::FullHouse,
            HandType::ThreeOfAKind => {
                if count == 2 {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            }
            HandType::TwoPair => HandType::TwoPair,
            HandType::OnePair => {
                if count == 3 {
                    HandType::FullHouse
                } else if count == 2 {
                    HandType::TwoPair
                } else {
                    HandType::OnePair
                }
            }
            HandType::HighCard => {
                if count == 4 {
                    HandType::FourOfAKind
                } else if count == 3 {
                    HandType::ThreeOfAKind
                } else if count == 2 {
                    HandType::OnePair
                } else {
                    HandType::HighCard
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct ParseHandError;

impl FromStr for Hand {
    type Err = ParseHandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vec_str: Vec<&str> = s.trim().split_whitespace().collect::<Vec<_>>();
        Ok(Hand {
            cards: vec_str.get(0).ok_or(ParseHandError)?.to_string(),
            bid: vec_str
                .get(1)
                .ok_or(ParseHandError)?
                .parse::<u32>()
                .map_err(|_| ParseHandError)?,
            highest_hand: HandType::HighCard,
        })
    }
}
