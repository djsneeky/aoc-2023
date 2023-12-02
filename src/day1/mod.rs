use std::fs;

fn find_first_and_last_int_combined(input: &str) -> Option<u32> {
    let chars: Vec<u32> = input
        .chars()
        .filter(|x: &char| x.is_digit(10))
        .map(|c: char| c.to_digit(10).unwrap())
        .collect();
    let first: &u32 = match chars.first() {
        Some(x) => x,
        None => return None,
    };
    let last: &u32 = match chars.last() {
        Some(x) => x,
        None => return None,
    };
    Some((first * 10) + last)
}

fn read_file(path: &str) -> String {
    let contents: String = fs::read_to_string(path).expect("Failed to read file");
    contents
}

pub fn solve_day1() -> u32 {
    let contents: String = read_file("src/day1/input.txt");
    let vec_contents: Vec<&str> = contents.split('\n').collect();
    let mut sum: u32 = 0;
    for content in vec_contents {
        let res: Option<u32> = find_first_and_last_int_combined(content);
        match res {
            Some(x) => sum += x,
            None => continue,
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_first_and_last_int_combined() {
        let input: String = "jk1lmno2pqr3yup4lol".to_string();
        assert_eq!(find_first_and_last_int_combined(&input), Some(14))
    }
}
