use std::str::FromStr;

struct Game {
    id: u32,
    entries: Vec<Entry>,
}

struct Entry {
    red: u32,
    green: u32,
    blue: u32,
}

impl FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(|c| c == ':' || c == ';').collect();
        
        let id: u32 = parts.get(0).ok_or("Missing game id")?.strip_prefix("Game ")?.parse::<u32>()?;
// for line in lines {
//     let parsed_line: Vec<&str> = line.split(|c| c == ':' || c == ';').collect();
//     if let Some(game_id) = get_game_id(&parsed_line[0]) {
//         for game_data in &parsed_line[1..] {
//             let numbered_colors: Vec<&str> = game_data.split(", ").collect();
//             numbered_colors.iter().find("green")
//         }
//     }
// }
        let id = 
    }
}

fn get_game_id(entry: &str) -> Option<u32> {
    if let Ok(game_id) = entry
        .strip_prefix("Game ")
        .expect("Failed to strip \"Game \"")
        .parse::<u32>()
    {
        Some(game_id)
    } else {
        None
    }
}
