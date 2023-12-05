mod day1;
mod day2;

fn main() {
    let day1_res = day1::solve_day1_all();
    println!(
        "Day 1 Result:\n\tPart 1: {}\n\tPart 2: {}",
        day1_res.0, day1_res.1
    );

    let day2_res = day2::solve_day2_all();
    println!(
        "Day 2 Result:\n\tPart 1: {}\n\tPart 2: {}",
        day2_res.0, day2_res.1
    );

    // let day2_res = day1::solve_day1_all();
    // println!("Day 1 Result:\n\tPart 1: {}\n\tPart 2: {}", res.0, res.1);
}
