use std::{error::Error, fs, vec};

use crate::hand::CARDS;

mod hand;

fn main() {
    let day7_res = solve_day7();
    println!(
        "Day 5 Result:\n\tPart 1: {}\n\tPart 2: {}",
        day7_res.0.unwrap(),
        day7_res.1.unwrap()
    );
}

pub fn solve_day7() -> (Result<u32, Box<dyn Error>>, Result<u32, Box<dyn Error>>) {
    (
        solve_day7_part1("src/day7/input.txt"),
        solve_day7_part2("src/day7/input.txt"),
    )
}

fn solve_day7_part1(input_path: &str) -> Result<u32, Box<dyn Error>> {
    let contents: String = fs::read_to_string(input_path)?;

    let mut hands: Vec<hand::Hand> = vec![];
    for line in contents.lines() {
        let mut hand: hand::Hand = line.parse::<hand::Hand>().unwrap();
        hand.get_highest_hand();
        hands.push(hand);
    }
    // sort hands according to enum
    hands.sort_by(|a, b| {
        let mut order = a.highest_hand.cmp(&b.highest_hand);

        if order == std::cmp::Ordering::Equal {
            for (card_a, card_b) in a.cards.chars().into_iter().zip(b.cards.chars().into_iter()) {
                if card_a == card_b {
                    continue;
                }
                let card_order = CARDS
                    .iter()
                    .position(|&c| c == card_b)
                    .cmp(&CARDS.iter().position(|&c| c == card_a));

                if card_order != std::cmp::Ordering::Equal {
                    order = card_order;
                }
                // std::cmp::Ordering::Equal
            }
        }
        order
    });

    let mut winnings: u32 = 0;
    for (index, hand) in hands.iter().enumerate() {
        winnings += hand.bid * (index as u32 + 1);
    }

    Ok(winnings)
}

fn solve_day7_part2(input_path: &str) -> Result<u32, Box<dyn Error>> {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_day7_part1_sample() {
        assert_eq!(solve_day7_part1("src/day7/sample-input.txt").unwrap(), 0);
    }

    #[test]
    fn test_solve_day7_part1() {
        assert_eq!(solve_day7_part1("src/day7/input.txt").unwrap(), 0)
    }

    #[test]
    fn test_solve_day7_part2_sample() {
        assert_eq!(solve_day7_part2("src/day7/sample-input.txt").unwrap(), 0);
    }

    #[test]
    fn test_solve_day7_part2() {
        assert_eq!(solve_day7_part2("src/day7/input.txt").unwrap(), 0);
    }
}
