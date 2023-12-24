use std::{error::Error, fs};

fn main() {
    let day4_res = solve_day4();
    println!(
        "Day 4 Result:\n\tPart 1: {}\n\tPart 2: {}",
        day4_res.0.unwrap(),
        day4_res.1.unwrap()
    );
}

#[derive(Debug)]
struct Card {
    input_numbers: Vec<u32>,
    winning_numbers: Vec<u32>,
}

type Cards = Vec<Card>;

pub fn solve_day4() -> (Result<u32, Box<dyn Error>>, Result<u32, Box<dyn Error>>) {
    (solve_day4_part1(), solve_day4_part2())
}

fn solve_day4_part1() -> Result<u32, Box<dyn Error>> {
    let contents: String = fs::read_to_string("src/day4/input.txt")?;

    let cards: Vec<Card> = parse_input(contents);

    let mut sum: u32 = 0;
    for card in cards {
        sum += get_point_value(&card);
    }
    Ok(sum)
}

fn solve_day4_part2() -> Result<u32, Box<dyn Error>> {
    todo!();
}

#[allow(dead_code)]
fn solve_day4_sample() -> Result<u32, Box<dyn Error>> {
    let contents: String = fs::read_to_string("src/day4/sample-input.txt")?;

    let cards = parse_input(contents);

    let mut sum: u32 = 0;
    for card in cards {
        sum += get_point_value(&card);
    }
    Ok(sum)
}

fn parse_input(contents: String) -> Cards {
    let mut cards: Cards = vec![];
    for line in contents.lines() {
        let list: Vec<&str> = line.split(&[':', '|']).skip(1).map(|s| s.trim()).collect();
        let card = Card {
            input_numbers: list
                .get(0)
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect(),
            winning_numbers: list
                .get(1)
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect(),
        };
        cards.push(card);
    }
    cards
}

fn get_point_value(card: &Card) -> u32 {
    let mut score: u32 = 0;
    for input_value in &card.input_numbers {
        for winning_value in &card.winning_numbers {
            if input_value == winning_value {
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
            }
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_day4_sample() {
        assert_eq!(solve_day4_sample().unwrap(), 13);
    }

    #[test]
    fn test_solve_day4_part1() {
        assert_eq!(solve_day4_part1().unwrap(), 25174)
    }

    #[test]
    fn test_solve_day4_part2() {
        assert_eq!(solve_day4_part2().unwrap(), 0);
    }
}
