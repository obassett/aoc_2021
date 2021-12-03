// Day 1 Challenges from Advent of Code 2021

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

// TODO: Handle errors if the parsing fails
fn parse_input(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_part1() {
        use super::*;
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
        let result = part1(&input);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_part2() {
        use super::*;
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
        let result = part2(&input);
        assert_eq!(expected_result, result);
    }
}
