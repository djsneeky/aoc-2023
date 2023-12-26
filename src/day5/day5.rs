use std::{error::Error, fs, str::FromStr};

fn main() {
    let day5_res = solve_day5();
    println!(
        "Day 5 Result:\n\tPart 1: {}\n\tPart 2: {}",
        day5_res.0.unwrap(),
        day5_res.1.unwrap()
    );
}

struct RangeMap {
    dest_start: u64,
    src_start: u64,
    range: u64,
}
#[derive(Debug, PartialEq, Eq)]
struct ParseRangeMapError;

impl RangeMap {
    // return output value given a range
    fn get_output_value(&self, input_value: u64) -> u64 {
        if input_value >= self.src_start && input_value < self.src_start + self.range {
            input_value - self.src_start + self.dest_start
        } else {
            input_value
        }
    }
}

impl FromStr for RangeMap {
    type Err = ParseRangeMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let range_map_vec_str: Vec<&str> = s.trim().split_whitespace().collect();

        let ds = range_map_vec_str
            .get(0)
            .ok_or(ParseRangeMapError)?
            .parse::<u64>()
            .map_err(|_| ParseRangeMapError)?;
        let ss = range_map_vec_str
            .get(0)
            .ok_or(ParseRangeMapError)?
            .parse::<u64>()
            .map_err(|_| ParseRangeMapError)?;
        let r = range_map_vec_str
            .get(0)
            .ok_or(ParseRangeMapError)?
            .parse::<u64>()
            .map_err(|_| ParseRangeMapError)?;

        Ok(RangeMap {
            dest_start: ds,
            src_start: ss,
            range: r,
        })
    }
}

pub fn solve_day5() -> (Result<u32, Box<dyn Error>>, Result<u32, Box<dyn Error>>) {
    (
        solve_day5_part1("src/day5/input.txt"),
        solve_day5_part2("src/day5/input.txt"),
    )
}

fn solve_day5_part1(input_path: &str) -> Result<u32, Box<dyn Error>> {
    let contents: String = fs::read_to_string(input_path)?;

    let mut lines = contents.lines();
    let seeds_vec: Vec<u64> = lines
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    lines.next();

    let maps: Vec<Vec<RangeMap>> = 
    

    for line in contents.lines() {
        if line.starts_with("seeds") {
            let seeds_vec: Vec<u64> = contents
                .strip_prefix("seeds: ")
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect();
        }
        let range_map: RangeMap = line.parse::<RangeMap>().unwrap();
    }

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
