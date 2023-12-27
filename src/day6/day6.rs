use std::{error::Error, fs};

fn main() {
    let day6_res = solve_day6();
    println!(
        "Day 6 Result:\n\tPart 1: {}\n\tPart 2: {}",
        day6_res.0.unwrap(),
        day6_res.1.unwrap()
    );
}

pub fn solve_day6() -> (Result<u32, Box<dyn Error>>, Result<u32, Box<dyn Error>>) {
    (
        solve_day6_part1("src/day6/input.txt"),
        solve_day6_part2("src/day6/input.txt"),
    )
}

fn solve_day6_part1(input_path: &str) -> Result<u32, Box<dyn Error>> {
    todo!();
}

fn solve_day6_part2(input_path: &str) -> Result<u32, Box<dyn Error>> {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_day6_part1_sample() {
        assert_eq!(solve_day6_part1("src/day6/sample-input.txt").unwrap(), 0);
    }

    #[test]
    fn test_solve_day6_part1() {
        assert_eq!(solve_day6_part1("src/day6/input.txt").unwrap(), 0)
    }

    #[test]
    fn test_solve_day6_part2_sample() {
        assert_eq!(solve_day6_part2("src/day6/sample-input.txt").unwrap(), 0);
    }

    #[test]
    fn test_solve_day6_part2() {
        assert_eq!(solve_day6_part2("src/day6/input.txt").unwrap(), 0);
    }
}
