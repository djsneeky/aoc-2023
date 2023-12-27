use std::{error::Error, fs, vec};

fn main() {
    let day6_res = solve_day6();
    println!(
        "Day 6 Result:\n\tPart 1: {}\n\tPart 2: {}",
        day6_res.0.unwrap(),
        day6_res.1.unwrap()
    );
}

struct Race {
    pub time: u64,
    pub record: u64,
}

pub fn solve_day6() -> (Result<u64, Box<dyn Error>>, Result<u64, Box<dyn Error>>) {
    (
        solve_day6_part1("src/day6/input.txt"),
        solve_day6_part2("src/day6/input.txt"),
    )
}

fn solve_day6_part1(input_path: &str) -> Result<u64, Box<dyn Error>> {
    let contents: String = fs::read_to_string(input_path)?;

    let mut lines = contents.lines();
    let time_iter = lines
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .trim()
        .split_whitespace()
        .collect::<Vec<_>>()
        .into_iter();
    let distance_iter = lines
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .trim()
        .split_whitespace()
        .collect::<Vec<_>>()
        .into_iter();
    let races: Vec<Race> = time_iter
        .zip(distance_iter)
        .map(|run| Race {
            time: run.0.parse().unwrap(),
            record: run.1.parse().unwrap(),
        })
        .collect();

    let mut all_records: Vec<u64> = vec![];
    for race in races {
        let mut new_records: u64 = 0;
        for time in 0..race.time {
            let distance = (race.time - time) * time;
            if distance > race.record {
                new_records += 1;
            }
        }
        all_records.push(new_records);
    }

    Ok(all_records.iter().fold(1, |acc, x| acc * x))
}

// TODO: could perform an optimization to only look up to total time / 2
fn solve_day6_part2(input_path: &str) -> Result<u64, Box<dyn Error>> {
    let contents: String = fs::read_to_string(input_path)?;

    let mut lines = contents.lines();
    let time: String = lines
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .trim()
        .split_whitespace()
        .collect();
    let distance: String = lines
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .trim()
        .split_whitespace()
        .collect();
    let race = Race {
        time: time.parse().unwrap(),
        record: distance.parse().unwrap(),
    };

    let mut new_records: u64 = 0;
    for time in 0..race.time {
        let distance = (race.time - time) * time;
        if distance > race.record {
            new_records += 1;
        }
    }

    Ok(new_records)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_day6_part1_sample() {
        assert_eq!(solve_day6_part1("src/day6/sample-input.txt").unwrap(), 288);
    }

    #[test]
    fn test_solve_day6_part1() {
        assert_eq!(solve_day6_part1("src/day6/input.txt").unwrap(), 1159152)
    }

    #[test]
    fn test_solve_day6_part2_sample() {
        assert_eq!(
            solve_day6_part2("src/day6/sample-input.txt").unwrap(),
            71503
        );
    }

    #[test]
    fn test_solve_day6_part2() {
        assert_eq!(solve_day6_part2("src/day6/input.txt").unwrap(), 41513103);
    }
}
