// Day 1 Challenges from Advent of Code 2021

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
// TODO: Handle errors if the parsing fails
fn parse_input(input: &str) -> Vec<SubCommand> {
    input
        .lines()
        .map(|line| line.parse::<SubCommand>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        // Define Input Data
        let input = String::from(
            "forward 5\n\
             down 5\n\
             forward 8\n\
             up 3\n\
             down 8\n\
             forward 2\n",
        );
        let result = part1(&input);
        let expected_result: i32 = 150;
        assert_eq!(result, expected_result)
    }

    #[test]
    fn test_part2() {
        // Define Input Data
        let input = String::from(
            "forward 5\n\
             down 5\n\
             forward 8\n\
             up 3\n\
             down 8\n\
             forward 2\n",
        );

        let result = part2(&input);
        let expected_result: i32 = 900;
        assert_eq!(result, expected_result)
    }
}
