use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

fn parse_config(args: &[String]) -> &str {
    let filename = &args[1];
    &filename
}

fn read_file<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where
    P: AsRef<Path>,
{
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

enum Depth {
    Increased,
    Decreased,
    Unchanged,
}

impl Depth {
    fn compare_current_depth(current_depth: &u16, previous_depth: &u16) -> Depth {
        if current_depth > previous_depth {
            Depth::Increased
        } else if current_depth < previous_depth {
            Depth::Decreased
        } else {
            Depth::Unchanged
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = parse_config(&args);

    let mut increased_values: u16 = 0;

    if let Ok(lines) = read_file(filename) {
        let mut window: Vec<u16> = Vec::new();
        let mut previous_total: Option<u16> = None;
        let mut current_total: Option<u16> = None;

        for line in lines {
            if let Ok(line) = line {
                window.insert(0, line.parse().unwrap());
                match window.len() {
                    4 => {
                        window.pop();
                        current_total = Some(window.iter().sum());
                    }
                    3 => current_total = Some(window.iter().sum()),
                    5.. => unreachable!(),
                    current_len => {
                        println!("Only have {} values in the window", current_len);
                    }
                }
                match previous_total {
                    Some(previous_total) => {
                        match Depth::compare_current_depth(&current_total.unwrap(), &previous_total)
                        {
                            Depth::Increased => {
                                increased_values += 1;
                                println!("{} (increased)", current_total.unwrap())
                            }
                            Depth::Decreased => println!("{} (decreased)", current_total.unwrap()),
                            Depth::Unchanged => println!("{} (unchanged)", current_total.unwrap()),
                        }
                    }
                    None => {
                        if current_total.is_some() {
                            println!("{} (initial)", current_total.unwrap())
                        }
                    }
                }
                previous_total = current_total;
            }
        }
    }
    println!(
        "The total number of increased values is: {}",
        increased_values
    );
}

// --- Part Two ---
// Considering every single measurement isn't as useful as you expected: there's just too much noise in the data.

// Instead, consider sums of a three-measurement sliding window. Again considering the above example:

// 199  A
// 200  A B
// 208  A B C
// 210    B C D
// 200  E   C D
// 207  E F   D
// 240  E F G
// 269    F G H
// 260      G H
// 263        H
// Start by comparing the first and second three-measurement windows. The measurements in the first window are marked A (199, 200, 208); their sum is 199 + 200 + 208 = 607. The second window is marked B (200, 208, 210); its sum is 618. The sum of measurements in the second window is larger than the sum of the first, so this first comparison increased.

// Your goal now is to count the number of times the sum of measurements in this sliding window increases from the previous sum. So, compare A with B, then compare B with C, then C with D, and so on. Stop when there aren't enough measurements left to create a new three-measurement sum.

// In the above example, the sum of each three-measurement window is as follows:

// A: 607 (N/A - no previous sum)
// B: 618 (increased)
// C: 618 (no change)
// D: 617 (decreased)
// E: 647 (increased)
// F: 716 (increased)
// G: 769 (increased)
// H: 792 (increased)
// In this example, there are 5 sums that are larger than the previous sum.

// Consider sums of a three-measurement sliding window. How many sums are larger than the previous sum?
