use std::{error::Error, fs, str::FromStr};

fn main() {
    let day5_res = solve_day5();
    println!(
        "Day 5 Result:\n\tPart 1: {}\n\tPart 2: {}",
        day5_res.0.unwrap(),
        day5_res.1.unwrap()
    );
}

#[derive(Debug)]
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
            .get(1)
            .ok_or(ParseRangeMapError)?
            .parse::<u64>()
            .map_err(|_| ParseRangeMapError)?;
        let r = range_map_vec_str
            .get(2)
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

pub fn solve_day5() -> (Result<u64, Box<dyn Error>>, Result<u64, Box<dyn Error>>) {
    (
        solve_day5_part1("src/day5/input.txt"),
        solve_day5_part2("src/day5/input.txt"),
    )
}

fn solve_day5_part1(input_path: &str) -> Result<u64, Box<dyn Error>> {
    let contents: String = fs::read_to_string(input_path)?;

    let seeds: Vec<u64> = contents
        .strip_prefix("seeds: ")
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let mut maps: Vec<Vec<RangeMap>> = vec![];
    let maps_str: &str = contents.split_once("\n\n").unwrap().1.trim();
    let maps_str_vec: Vec<&str> = maps_str.split("\n\n").collect();

    for map_str in maps_str_vec {
        let range_maps: Vec<RangeMap> = map_str
            .split('\n')
            .skip(1)
            .map(|s| s.parse::<RangeMap>().unwrap())
            .collect();
        maps.push(range_maps);
    }

    // map seeds through all the maps
    let mut locations: Vec<u64> = vec![];
    for seed in seeds {
        let mut mapped_val: u64 = seed;
        for map in &maps {
            for range_map in map {
                let new_mapped_val: u64 = range_map.get_output_value(mapped_val);
                if new_mapped_val != mapped_val {
                    mapped_val = new_mapped_val;
                    break;
                }
            }
        }
        locations.push(mapped_val);
    }

    let min = locations.iter().min().ok_or("no min found")?.clone();

    Ok(min)
}

fn solve_day5_part2(input_path: &str) -> Result<u64, Box<dyn Error>> {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_day5_part1_sample() {
        assert_eq!(solve_day5_part1("src/day5/sample-input.txt").unwrap(), 35);
    }

    #[test]
    fn test_solve_day5_part1() {
        assert_eq!(solve_day5_part1("src/day5/input.txt").unwrap(), 389056265)
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
