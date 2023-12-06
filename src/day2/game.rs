use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Game {
    id: u32,
    entries: Vec<Entry>,
}

#[derive(Debug, PartialEq)]
struct Entry {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseGameError;

// impl FromStr for Entry {
//     type Err = ParseGameError;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {}
// }

impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(|c| c == ':' || c == ';').collect();

        let parsed_id: u32 = parts
            .get(0)
            .ok_or(ParseGameError)?
            .strip_prefix("Game ")
            .ok_or(ParseGameError)?
            .parse::<u32>()
            .map_err(|_| ParseGameError)?;

        let mut parsed_entries: Vec<Entry> = vec![];

        for entries_str in &parts[1..] {
            for entry_str in entries_str.split(";").into_iter() {
                let mut entry = Entry {
                    red: 0,
                    green: 0,
                    blue: 0,
                };
                let pairs: Vec<&str> = entry_str.trim().split(", ").collect();
                for pair in pairs {
                    let parts: Vec<&str> = pair.splitn(2, ' ').collect();
                    let count: u32 = parts[0].parse::<u32>().unwrap();
                    let color = parts[1].trim();
                    match color {
                        "red" => entry.red += count,
                        "green" => entry.green += count,
                        "blue" => entry.blue += count,
                        _ => panic!(),
                    }
                }
                parsed_entries.push(entry);
            }
        }

        Ok(Game {
            id: parsed_id,
            entries: parsed_entries,
        })
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.id)
    }
}
