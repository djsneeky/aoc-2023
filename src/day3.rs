use std::{error::Error, fs};

// use ndarray::{Array, Array2};

const SYMBOLS: [char; 10] = ['@', '#', '$', '%', '&', '*', '-', '+', '=', '/'];

struct Cell {
    data: char,
    visited: bool,
}

type Matrix = Vec<Vec<Cell>>;

pub fn solve_day3() -> (Result<u32, Box<dyn Error>>, Result<u32, Box<dyn Error>>) {
    (solve_day3_part1(), solve_day3_part2())
}

fn solve_day3_part1() -> Result<u32, Box<dyn Error>> {
    todo!();
}

fn solve_day3_part2() -> Result<u32, Box<dyn Error>> {
    todo!();
}

fn solve_day3_sample() -> Result<u32, Box<dyn Error>> {
    let contents: String = fs::read_to_string("src/day3/sample-input.txt")?;
    let matrix: Vec<Vec<char>> = parse_input(&contents)?;

    let sum = sum_numbers_around_symbols(matrix, &SYMBOLS)?;
    Ok(sum)
}

fn sum_numbers_around_symbols(
    matrix: Vec<Vec<char>>,
    symbols: &[char],
) -> Result<u32, Box<dyn Error>> {
    let mut sum: u32 = 0;
    for (i, row) in matrix.iter().enumerate() {
        for (j, element) in row.iter().enumerate() {
            if symbols.contains(element) {
                if let Some((row_found_idx, col_found_idx)) = find_number(&matrix, i, j) {
                    let number = get_complete_number(&matrix, row_found_idx, col_found_idx);
                    sum += number;
                }
            }
        }
    }
    Ok(sum)
}

fn find_number(
    matrix: &Vec<Vec<char>>,
    row_index: usize,
    col_index: usize,
) -> Option<(usize, usize)> {
    if matrix[row_index - 1][col_index - 1].is_ascii_digit() {
        Some((row_index - 1, col_index - 1))
    } else if matrix[row_index - 1][col_index].is_ascii_digit() {
        Some((row_index - 1, col_index))
    } else if matrix[row_index - 1][col_index + 1].is_ascii_digit() {
        Some((row_index - 1, col_index + 1))
    } else if matrix[row_index][col_index - 1].is_ascii_digit() {
        Some((row_index, col_index - 1))
    } else if matrix[row_index][col_index + 1].is_ascii_digit() {
        Some((row_index, col_index + 1))
    } else if matrix[row_index + 1][col_index - 1].is_ascii_digit() {
        Some((row_index + 1, col_index - 1))
    } else if matrix[row_index + 1][col_index].is_ascii_digit() {
        Some((row_index + 1, col_index))
    } else if matrix[row_index + 1][col_index + 1].is_ascii_digit() {
        Some((row_index + 1, col_index + 1))
    } else {
        None
    }
}

fn get_complete_number(matrix: &Vec<Vec<char>>, row_index: usize, col_index: usize) -> u32 {
    let mut number: String = String::new();
    let mut i: usize = 0;

    loop {
        if let Some(index) = col_index.checked_sub(i) {
            if matrix[row_index][index].is_ascii_digit() {
                number.insert(0, matrix[row_index][index]);
                i += 1;
            } else {
                break;
            }
        } else {
            break;
        }
    }
    let mut j: usize = 1;
    loop {
        if let Some(index) = col_index.checked_add(j) {
            if matrix[row_index][index].is_ascii_digit() {
                number.push(matrix[row_index][index]);
                j += 1;
            } else {
                break;
            }
        } else {
            break;
        }
    }
    println!("{number}");
    number
        .parse::<u32>()
        .expect("completed number should be parseable")
}

fn parse_input(input: &str) -> Result<Vec<Vec<char>>, Box<dyn Error>> {
    let matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    Ok(matrix)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_day3_sample() {
        assert_eq!(solve_day3_sample().unwrap(), 4361);
    }

    #[test]
    fn test_solve_day3_part1() {
        assert_eq!(solve_day3_part1().unwrap(), 0)
    }

    #[test]
    fn test_solve_day3_part2() {
        assert_eq!(solve_day3_part2().unwrap(), 0);
    }
}
