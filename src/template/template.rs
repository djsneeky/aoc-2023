use std::{error::Error, fs};

fn main() {
    let day5_res = solve_day5();
    println!(
        "Day 5 Result:\n\tPart 1: {}\n\tPart 2: {}",
        day5_res.0.unwrap(),
        day5_res.1.unwrap()
    );
}

pub fn solve_day5() -> (Result<u32, Box<dyn Error>>, Result<u32, Box<dyn Error>>) {
    (
        solve_day5_part1("src/day5/input.txt"),
        solve_day5_part2("src/day5/input.txt"),
    )
}

fn solve_day5_part1(input_path: &str) -> Result<u32, Box<dyn Error>> {
    todo!();
}

fn solve_day5_part2(input_path: &str) -> Result<u32, Box<dyn Error>> {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_day5_part1_sample() {
        assert_eq!(solve_day5_part1("src/day5/sample-input.txt").unwrap(), 0);
    }

    #[test]
    fn test_solve_day5_part1() {
        assert_eq!(solve_day5_part1("src/day5/input.txt").unwrap(), 0)
    }

    #[test]
    fn test_solve_day5_part2_sample() {
        assert_eq!(solve_day5_part2("src/day5/sample-input.txt").unwrap(), 0);
    }

    #[test]
    fn test_solve_day5_part2() {
        assert_eq!(solve_day5_part2("src/day5/input.txt").unwrap(), 0);
    }
}
