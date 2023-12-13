mod day1;
mod day2;
mod day3;

fn main() {
    let day1_res = day1::solve_day1();
    println!(
        "Day 1 Result:\n\tPart 1: {}\n\tPart 2: {}",
        day1_res.0.unwrap(),
        day1_res.1.unwrap()
    );

    let day2_res = day2::solve_day2();
    println!(
        "Day 2 Result:\n\tPart 1: {}\n\tPart 2: {}",
        day2_res.0.unwrap(),
        day2_res.1.unwrap()
    );

    let day3_res = day3::solve_day3();
    println!(
        "Day 3 Result:\n\tPart 1: {}\n\tPart 2: {}",
        day3_res.0.unwrap(),
        day3_res.1.unwrap()
    );

    // let day2_res = day1::solve_day1_all();
    // println!("Day 1 Result:\n\tPart 1: {}\n\tPart 2: {}", res.0, res.1);
}
