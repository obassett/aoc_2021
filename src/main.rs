// This will be the core cli for launching each day. Removing the need for binaries for each part.
// Need to support args for data file, and day and part to run.
// part should be optional and if not provided run both parts

use structopt::StructOpt;

use lib_aoc2021;

fn execute_exercise(input: &str, day: u8, part: Option<u8>) {
    match day {
        1 => match part {
            Some(1) => {
                println!("Part 1: {}", lib_aoc2021::day1::part1(input));
            }
            Some(2) => {
                println!("Part 2: {}", lib_aoc2021::day1::part2(input));
            }
            None => {
                println!("Part 1: {}", lib_aoc2021::day1::part1(input));
                println!("Part 2: {}", lib_aoc2021::day1::part2(input));
            }
            Some(_) => {
                println!("Invalid part");
            }
        },
        2 => match part {
            Some(1) => {
                println!("Part 1: {}", lib_aoc2021::day2::part1(input));
            }
            Some(2) => {
                println!("Part 2: {}", lib_aoc2021::day2::part2(input));
            }
            None => {
                println!("Part 1: {}", lib_aoc2021::day2::part1(input));
                println!("Part 2: {}", lib_aoc2021::day2::part2(input));
            }
            Some(_) => {
                println!("Invalid part");
            }
        },
        3 => match part {
            Some(1) => {
                println!("Part 1: {}", lib_aoc2021::day3::part1(input));
            }
            Some(2) => {
                println!("Part 2: {}", lib_aoc2021::day3::part2(input));
            }
            None => {
                println!("Part 1: {}", lib_aoc2021::day3::part1(input));
                println!("Part 2: {}", lib_aoc2021::day3::part2(input));
            }
            Some(_) => {
                println!("Invalid part");
            }
        },
        4 => match part {
            Some(1) => {
                println!("Part 1: {}", lib_aoc2021::day4::part1(input).unwrap());
            }
            Some(2) => {
                println!("Part 2: {}", lib_aoc2021::day4::part2(input).unwrap());
            }
            None => {
                println!("Part 1: {}", lib_aoc2021::day4::part1(input).unwrap());
                println!("Part 2: {}", lib_aoc2021::day4::part2(input).unwrap());
            }
            Some(_) => {
                println!("Invalid part");
            }
        },
        5 => match part {
            Some(1) => {
                println!("Part 1: {}", lib_aoc2021::day5::part1(input).unwrap());
            }
            Some(2) => {
                println!("Part 2: {}", lib_aoc2021::day5::part2(input).unwrap());
            }
            None => {
                println!("Part 1: {}", lib_aoc2021::day5::part1(input).unwrap());
                println!("Part 2: {}", lib_aoc2021::day5::part2(input).unwrap());
            }
            Some(_) => {
                println!("Invalid part");
            }
        },
        6 => match part {
            Some(1) => {
                println!("Part 1: {}", lib_aoc2021::day6::part1(input));
            }
            Some(2) => {
                println!("Part 2: {}", lib_aoc2021::day6::part2(input));
            }
            None => {
                println!("Part 1: {}", lib_aoc2021::day6::part1(input));
                println!("Part 2: {}", lib_aoc2021::day6::part2(input));
            }
            Some(_) => {
                println!("Invalid part");
            }
        },
        _ => println!("Day not yet implemented or invalid"),
    }
}

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short = "d", long = "day", default_value = "1")]
    day: u8,

    #[structopt(short = "p", long = "part")]
    part: Option<u8>,

    #[structopt(short = "f", long = "file", default_value = "data/day1/input.txt")]
    file: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();
    let day = args.day;
    let part = args.part;
    let file = args.file;

    let input = lib_aoc2021::utils::read_file(file);
    execute_exercise(&input, day, part);
}
