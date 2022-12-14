#![feature(test)]

use std::env;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

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
        "7.1" => println!("{}", day7::part1("data/day7.txt")),
        "7.2" => println!("{}", day7::part2("data/day7.txt")),
        "8.1" => println!("{}", day8::part1("data/day8.txt")),
        "8.2" => println!("{}", day8::part2("data/day8.txt")),
        "9.1" => println!("{}", day9::part1("data/day9.txt")),
        "9.2" => println!("{}", day9::part2("data/day9.txt")),
        "10.1" => println!("{}", day10::part1("data/day10-sample.txt")),
        "10.2" => println!("{}", day10::part2("data/day10.txt")),
        "11.1" => println!("{}", day11::part1("data/day11.txt")),
        "11.2" => println!("{}", day11::part2("data/day11.txt")),
        "12.1" => println!("{}", day12::part1("data/day12.txt")),
        "12.2" => println!("{}", day12::part2("data/day12.txt")),
        "13.1" => println!("{}", day13::part1("data/day13.txt")),
        "13.2" => println!("{}", day13::part2("data/day13.txt")),
        "14.1" => println!("{}", day14::part1("data/day14.txt")),
        "14.2" => println!("{}", day14::part2("data/day14.txt")),
        _ => println!("{} not handled", which_puzzle),
    }
    Ok(())
}
