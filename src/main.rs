use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let args: Vec<String> = env::args().collect();
    let which_puzzle = &args[1];
    match &which_puzzle[..] {
        "1.1" => println!("{}", day1::part1("data/day1.txt")),
        "1.2" => println!("{}", day1::part2("data/day1.txt")),
        "2.1" => println!("{}", day2::part1()),
        "2.2" => println!("{}", day2::part2()),
        "3.1" => println!("{}", day3::part1()),
        "3.2" => println!("{}", day3::part2()),
        "4.1" => println!("{}", day4::part1()),
        "4.2" => println!("{}", day4::part2()),
        "5.1" => println!("{}", day5::part1()),
        "5.2" => println!("{}", day5::part2()),
        "6.1" => println!("{}", day6::part1()),
        "6.2" => println!("{}", day6::part2()),
        _ => println!("{} not handled", which_puzzle),
    }
    Ok(())
}
