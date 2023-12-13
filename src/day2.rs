use std::cmp::max;
use std::error::Error;
use std::fs;

mod game;

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

pub fn solve_day2() -> (Result<u32, Box<dyn Error>>, Result<u32, Box<dyn Error>>) {
    (solve_day2_part1(), solve_day2_part2())
}

pub fn solve_day2_part1() -> Result<u32, Box<dyn Error>> {
    let contents: String = fs::read_to_string("src/day2/input.txt")?;
    let mut count: u32 = 0;
    for line in contents.lines() {
        let game = line.parse::<game::Game>().unwrap();
        let mut invalid_entry: bool = false;
        for entry in game.entries {
            if entry.red > MAX_RED || entry.green > MAX_GREEN || entry.blue > MAX_BLUE {
                invalid_entry = true;
                break;
            }
        }
        if !invalid_entry {
            count += game.id;
        }
    }
    Ok(count)
}

pub fn solve_day2_part2() -> Result<u32, Box<dyn Error>> {
    let contents: String = fs::read_to_string("src/day2/input.txt")?;
    let mut sum_power = 0;
    for line in contents.lines() {
        let game = line.parse::<game::Game>().unwrap();
        let mut max_entry: game::Entry = game::Entry {
            red: 0,
            green: 0,
            blue: 0,
        };
        for entry in game.entries {
            max_entry.red = max(max_entry.red, entry.red);
            max_entry.green = max(max_entry.green, entry.green);
            max_entry.blue = max(max_entry.blue, entry.blue);
        }
        sum_power += max_entry.red * max_entry.green * max_entry.blue;
    }
    Ok(sum_power)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_day2_part1() {
        assert_eq!(solve_day2_part1().unwrap(), 2505)
    }

    #[test]
    fn test_solve_day2_part2() {
        assert_eq!(solve_day2_part2().unwrap(), 70265);
    }
}
