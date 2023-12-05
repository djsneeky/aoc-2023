use std::fs;

mod game;

fn read_file(path: &str) -> String {
    let contents: String = fs::read_to_string(path).expect("Failed to read file");
    contents
}

// fn parse_

pub fn solve_day2_all() -> (u32, u32) {
    (0, 0)
}

pub fn solve_day2_part1() -> u32 {
    let contents: String = read_file("src/day2/input.txt");
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_day2_part1() {
        assert_eq!(solve_day2_part1(), 0);
    }
}
