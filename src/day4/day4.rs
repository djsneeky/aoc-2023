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
    copies: u32,
}

type Cards = Vec<Card>;

pub fn solve_day4() -> (Result<u32, Box<dyn Error>>, Result<u32, Box<dyn Error>>) {
    (
        solve_day4_part1("src/day4/input.txt"),
        solve_day4_part2("src/day4/input.txt"),
    )
}

fn solve_day4_part1(input_path: &str) -> Result<u32, Box<dyn Error>> {
    let contents: String = fs::read_to_string(input_path)?;

    let cards: Vec<Card> = parse_input(contents);

    let mut sum: u32 = 0;
    for card in cards {
        sum += get_point_value(&card);
    }
    Ok(sum)
}

fn solve_day4_part2(input_path: &str) -> Result<u32, Box<dyn Error>> {
    let contents: String = fs::read_to_string(input_path)?;

    let mut cards: Vec<Card> = parse_input(contents);
    let mut mut_refs: Vec<_> = cards.iter_mut().collect();
    let mut total_copies: u32 = 0;

    let cards_len = mut_refs.len() as u32;
    for i in 0..cards_len {
        let n: u32 = get_num_matches(&mut_refs[i as usize]);
        // iterate over the next n cards and increment copies
        if n > 0 {
            let end_index: u32 = if i + n > cards_len { i + n } else { cards_len };
            for j in i + 1..end_index {
                mut_refs[j as usize].copies += 1;
                total_copies += 1;
            }
        }
    }
    Ok(total_copies)
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
            copies: 0,
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

fn get_num_matches(card: &Card) -> u32 {
    let mut matches: u32 = 0;
    for input_value in &card.input_numbers {
        for winning_value in &card.winning_numbers {
            if input_value == winning_value {
                matches += 1;
            }
        }
    }
    matches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_day4_part1_sample() {
        assert_eq!(solve_day4_part1("src/day4/sample-input.txt").unwrap(), 13);
    }

    #[test]
    fn test_solve_day4_part1() {
        assert_eq!(solve_day4_part1("src/day4/input.txt").unwrap(), 25174)
    }

    #[test]
    fn test_solve_day4_part2_sample() {
        assert_eq!(solve_day4_part2("src/day4/sample-input.txt").unwrap(), 30);
    }

    #[test]
    fn test_solve_day4_part2() {
        assert_eq!(solve_day4_part2("src/day4/input.txt").unwrap(), 0);
    }
}
