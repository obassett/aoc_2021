pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;

// Utilities for all the solutions
pub mod utils {
    use std::fs;

    pub fn read_file(filename: std::path::PathBuf) -> String {
        fs::read_to_string(filename).expect("Unable to open file")
    }
}
