// Day 1 Challenges from Advent of Code 2021
pub mod day1 {
    // Part 1 - Count number of times value increases from previous value - returns the count
    pub fn part1(input: &str) -> u32 {
        count_increases(parse_input(input))
    }

    // Part 2 - Implement sum of sliding window of 3 values across the input
    pub fn part2(input: &str) -> u32 {
        let measurements = parse_input(input)
            .windows(3)
            .map(|win| win.iter().sum())
            .collect();
        count_increases(measurements)
    }

    // Take in a vector of u32's - increases and accumulator when value > previous value, returns accumulator
    fn count_increases(values: Vec<u32>) -> u32 {
        values
            .windows(2)
            .filter(|w| w[1] > w[0])
            .count()
            .try_into()
            .unwrap()
    }

    fn parse_input(input: &str) -> Vec<u32> {
        input
            .lines()
            .map(|line| line.parse::<u32>().unwrap())
            .collect()
    }
}

// Day 2
pub mod day2 {
    use std::str::FromStr;

    pub fn part1(input: &str) -> i32 {
        let mut my_sub = SubLocation::new();

        for command in parse_input(input) {
            my_sub.move_sub(&command);
        }

        my_sub.get_current_location()
    }

    pub fn part2(input: &str) -> i32 {
        let mut my_sub = SubLocation2::new();

        for command in parse_input(input) {
            my_sub.move_sub(&command);
        }

        my_sub.get_current_location()
    }

    struct SubLocation {
        depth: i32,
        horizontal: i32,
    }

    impl SubLocation {
        fn new() -> Self {
            Self {
                depth: 0,
                horizontal: 0,
            }
        }

        fn move_sub(&mut self, command: &SubCommand) {
            match command.direction {
                SubDirection::Down => {
                    println!("Moving sub down {} spaces.", command.distance);
                    self.depth += command.distance;
                }
                SubDirection::Up => {
                    println!("Moving sub up {} spaces.", command.distance);
                    self.depth -= command.distance;
                }
                SubDirection::Forward => {
                    println!("Moving sub forward {} spaces.", command.distance);
                    self.horizontal += command.distance;
                }
            }
        }

        fn get_current_location(&self) -> i32 {
            self.depth * self.horizontal
        }
    }

    struct SubLocation2 {
        depth: i32,
        horizontal: i32,
        aim: i32,
    }

    impl SubLocation2 {
        fn new() -> Self {
            Self {
                depth: 0,
                horizontal: 0,
                aim: 0,
            }
        }

        // It increases your horizontal position by X units.
        // It increases your depth by your aim multiplied by X.
        fn move_sub(&mut self, command: &SubCommand) {
            match command.direction {
                SubDirection::Down => {
                    println!("Moving sub down {} spaces.", command.distance);
                    self.aim += command.distance;
                }
                SubDirection::Up => {
                    println!("Moving sub up {} spaces.", command.distance);
                    self.aim -= command.distance;
                }
                SubDirection::Forward => {
                    println!("Moving sub forward {} spaces.", command.distance);
                    let depth_change = self.aim * command.distance;
                    self.horizontal += command.distance;
                    self.depth += depth_change;
                }
            }
        }

        fn get_current_location(&self) -> i32 {
            self.depth * self.horizontal
        }
    }

    struct SubCommand {
        direction: SubDirection,
        distance: i32,
    }

    enum SubDirection {
        Forward,
        Up,
        Down,
    }

    // Convert string to SubDirection Enum
    impl FromStr for SubDirection {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.trim().to_lowercase().as_str() {
                "forward" => Ok(SubDirection::Forward),
                "up" => Ok(SubDirection::Up),
                "down" => Ok(SubDirection::Down),
                _ => Err(format!("Invalid SubDirection: {}", s)),
            }
        }
    }

    // Convert string to SubCommand Struct
    impl FromStr for SubCommand {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut split = s.split_whitespace();
            let direction: SubDirection = split.next().unwrap().parse()?;
            let distance: i32 = split.next().unwrap().parse().unwrap();
            Ok(SubCommand {
                direction,
                distance,
            })
        }
    }

    // Parses Input Into Commands
    fn parse_input(input: &str) -> Vec<SubCommand> {
        input
            .lines()
            .map(|line| line.parse::<SubCommand>().unwrap())
            .collect()
    }
}

// Utilities for all the solutions
pub mod utils {
    use std::fs;

    pub fn read_file(filename: std::path::PathBuf) -> String {
        fs::read_to_string(filename).expect("Unable to open file")
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_day1_part1() {
        use super::day1;
        let input = String::from(
            "\
            199\n\
            200\n\
            208\n\
            210\n\
            200\n\
            207\n\
            240\n\
            269\n\
            260\n\
            263\n",
        );

        let expected_result: u32 = 7;
        let result = day1::part1(&input);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_day1_part2() {
        use super::day1;
        let input = String::from(
            "\
            199\n\
            200\n\
            208\n\
            210\n\
            200\n\
            207\n\
            240\n\
            269\n\
            260\n\
            263\n",
        );

        let expected_result: u32 = 5;
        let result = day1::part2(&input);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_day2_part1() {
        use super::day2;
        // Define Input Data
        let input = String::from(
            "forward 5\n\
             down 5\n\
             forward 8\n\
             up 3\n\
             down 8\n\
             forward 2\n",
        );

        let result = day2::part1(&input);
        let expected_result: i32 = 150;
        assert_eq!(result, expected_result)
    }

    #[test]
    fn test_day2_part2() {
        use super::day2;
        // Define Input Data
        let input = String::from(
            "forward 5\n\
             down 5\n\
             forward 8\n\
             up 3\n\
             down 8\n\
             forward 2\n",
        );

        let result = day2::part2(&input);
        let expected_result: i32 = 900;
        assert_eq!(result, expected_result)
    }
}
