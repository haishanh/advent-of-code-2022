use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let args: Vec<String> = env::args().collect();
    let which_puzzle = &args[1];
    match &which_puzzle[..] {
        "1.1" => day1::part1(),
        "1.2" => day1::part2(),
        "2.1" => day2::part1(),
        "2.2" => day2::part2(),
        "3.1" => day3::part1(),
        "3.2" => day3::part2(),
        "4.1" => day4::part1(),
        "4.2" => day4::part2(),
        "5.1" => {
            day5::part1();
            ()
        }
        "5.2" => {
            day5::part2();
            ()
        }
        _ => println!("{} not handled", which_puzzle),
    }
    Ok(())
}
