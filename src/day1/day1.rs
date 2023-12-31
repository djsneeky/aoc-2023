use std::{error::Error, fs, usize};

fn main() {
    let day1_res = solve_day1();
    println!(
        "Day 1 Result:\n\tPart 1: {}\n\tPart 2: {}",
        day1_res.0.unwrap(),
        day1_res.1.unwrap()
    );
}

enum SearchDirection {
    Forward,
    Reverse,
}

const WORDS_AND_NUMBERS: [&str; 20] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0", "1", "2",
    "3", "4", "5", "6", "7", "8", "9",
];

fn combine_first_last_int(input: &str) -> Option<u32> {
    let first = match input.find(|c: char| c.is_digit(10)) {
        Some(idx) => input
            .chars()
            .skip(idx)
            .take(1)
            .collect::<String>()
            .parse::<u32>()
            .unwrap(),
        None => return None,
    };
    let last = match input.rfind(|c: char| c.is_digit(10)) {
        Some(idx) => input
            .chars()
            .skip(idx)
            .take(1)
            .collect::<String>()
            .parse::<u32>()
            .unwrap(),
        None => return None,
    };

    Some((first * 10) + last)
}

fn combine_first_last_int_str(input: &str) -> Option<u32> {
    if let Some(first_word) = find_first_word(input, &WORDS_AND_NUMBERS, SearchDirection::Forward) {
        if let Some(last_word) =
            find_first_word(input, &WORDS_AND_NUMBERS, SearchDirection::Reverse)
        {
            let first_number: u32 = match first_word.parse::<u32>() {
                Ok(parsed_value) => parsed_value,
                Err(_) => word_to_number(first_word).expect(&format!(
                    "Failed to convert first_word \"{}\" to a number",
                    first_word
                )),
            };
            let last_number: u32 = match last_word.parse::<u32>() {
                Ok(parsed_value) => parsed_value,
                Err(_) => word_to_number(last_word).expect(&format!(
                    "Failed to convert last_word \"{}\" to a number",
                    last_word
                )),
            };
            return Some(first_number * 10 + last_number);
        }
    }
    None
}

fn find_first_word<'a>(
    input: &'a str,
    words: &'a [&'a str],
    loc: SearchDirection,
) -> Option<&'a str> {
    let mut index: Option<usize> = None;
    let mut return_word: Option<&str> = None;
    match loc {
        SearchDirection::Forward => {
            for &word in words {
                if let Some(input_index) = input.find(word) {
                    match index {
                        Some(last_index) => {
                            if input_index < last_index {
                                index = Some(input_index);
                                return_word = Some(word);
                            }
                        }
                        None => {
                            index = Some(input_index);
                            return_word = Some(word);
                        }
                    }
                }
            }
        }
        SearchDirection::Reverse => {
            for &word in words {
                if let Some(input_index) = input.rfind(word) {
                    match index {
                        Some(last_index) => {
                            if input_index > last_index {
                                index = Some(input_index);
                                return_word = Some(word);
                            }
                        }
                        None => {
                            index = Some(input_index);
                            return_word = Some(word);
                        }
                    }
                }
            }
        }
    }
    return_word
}

fn word_to_number(word: &str) -> Option<u32> {
    match word.trim().to_lowercase().as_str() {
        "zero" => Some(0),
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}

pub fn solve_day1() -> (Result<u32, Box<dyn Error>>, Result<u32, Box<dyn Error>>) {
    (solve_day1_part1(), solve_day1_part2())
}

pub fn solve_day1_part1() -> Result<u32, Box<dyn Error>> {
    let contents: String = fs::read_to_string("src/day1/input.txt")?;
    let vec_contents: Vec<&str> = contents.split('\n').collect();
    let mut sum: u32 = 0;
    for content in vec_contents {
        let res: Option<u32> = combine_first_last_int(content);
        match res {
            Some(x) => sum += x,
            None => continue,
        }
    }
    Ok(sum)
}

pub fn solve_day1_part2() -> Result<u32, Box<dyn Error>> {
    let contents: String = fs::read_to_string("src/day1/input.txt")?;
    let vec_contents: Vec<&str> = contents.split('\n').collect();
    let mut sum: u32 = 0;
    for content in vec_contents {
        let res: Option<u32> = combine_first_last_int_str(content);
        match res {
            Some(x) => sum += x,
            None => continue,
        }
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combine_first_last_int() {
        let input: String = "jk1lmno2pqr3yup4lol".to_string();
        assert_eq!(combine_first_last_int(&input), Some(14));
    }

    #[test]
    fn test_combine_first_last_int_str() {
        let input: String = "onejk1lmno2pqr3yup4loltwothree".to_string();
        assert_eq!(combine_first_last_int_str(&input), Some(13));
    }

    #[test]
    fn test_find_first_word_forward_word() {
        let input: String = "lolonejk5lmno2pqr3yup4lol2threexyz".to_string();
        assert_eq!(
            find_first_word(&input, &WORDS_AND_NUMBERS, SearchDirection::Forward),
            Some("one")
        );
    }

    #[test]
    fn test_find_first_word_reverse_word() {
        let input: String = "onejk1lmno2pqr3yup4lol2threexyz".to_string();
        assert_eq!(
            find_first_word(&input, &WORDS_AND_NUMBERS, SearchDirection::Reverse),
            Some("three")
        );
    }

    #[test]
    fn test_find_first_word_forward_number() {
        let input: String = "loljk5lmno2pqr3yup4lol2threexyz".to_string();
        assert_eq!(
            find_first_word(&input, &WORDS_AND_NUMBERS, SearchDirection::Forward),
            Some("5")
        );
    }

    #[test]
    fn test_find_first_word_reverse_number() {
        let input: String = "onejk1lmno2pqr3yup4lol2three7xyz".to_string();
        assert_eq!(
            find_first_word(&input, &WORDS_AND_NUMBERS, SearchDirection::Reverse),
            Some("7")
        );
    }

    #[test]
    fn test_solve_day1_part1() {
        assert_eq!(solve_day1_part1().unwrap(), 55538);
    }

    #[test]
    fn test_solve_day1_part2() {
        assert_eq!(solve_day1_part2().unwrap(), 54875);
    }
}
