use itertools::Itertools;
use std::collections::HashSet;

type SourcePatterns = [String; 10];
type OutputPatterns = [String; 4];

type InputSource = (SourcePatterns, OutputPatterns);

type CodedNumber = Option<HashSet<char>>;

#[derive(Debug, Clone)]
struct Code {
    one: CodedNumber,
    two: CodedNumber,
    three: CodedNumber,
    four: CodedNumber,
    five: CodedNumber,
    six: CodedNumber,
    seven: CodedNumber,
    eight: CodedNumber,
    nine: CodedNumber,
    zero: CodedNumber,
}

impl Code {
    fn new() -> Code {
        Code {
            one: None,
            two: None,
            three: None,
            four: None,
            five: None,
            six: None,
            seven: None,
            eight: None,
            nine: None,
            zero: None,
        }
    }
}

pub fn part1(input: &str) -> u32 {
    // For this we only need the easy numbers

    let input_source = parse_input(input);

    // Parse the data return a count of all the OutputPatterns that are 1,4,7,8
    // 1's are 2 chars, 4s are 4 chars, 7s are 3 chars, 8s are 7 chars
    let mut results: u32 = 0;

    for (_source, output) in input_source {
        for output_pattern in output {
            match output_pattern.len() {
                2 | 3 | 4 | 7 => {
                    results += 1;
                }
                _ => (),
            }
        }
    }

    results
}

pub fn part2(input: &str) -> u32 {
    // Guessing for this I need to work out all the numbers.
    // See notes in comments for the rules.

    let parsed_input = parse_input(input);
    let mut decoded_output: Vec<u32> = Vec::new();

    for (source_patterns, output_patterns) in parsed_input {
        let code = create_decoder(source_patterns);

        match decode_output(&code, output_patterns) {
            Some(decoded) => decoded_output.push(decoded),
            None => (),
        }
    }

    return decoded_output.iter().sum();
}

fn decode_output(code: &Code, output_patterns: OutputPatterns) -> Option<u32> {
    let mut decoded_patterns: Vec<u32> = Vec::new();

    for output_pattern in output_patterns {
        match output_pattern.chars().collect::<HashSet<char>>() {
            ref coded_ouput if coded_ouput == code.one.as_ref().unwrap() => {
                decoded_patterns.push(1);
            }
            ref coded_ouput if coded_ouput == code.two.as_ref().unwrap() => {
                decoded_patterns.push(2);
            }
            ref coded_ouput if coded_ouput == code.three.as_ref().unwrap() => {
                decoded_patterns.push(3);
            }
            ref coded_ouput if coded_ouput == code.four.as_ref().unwrap() => {
                decoded_patterns.push(4);
            }
            ref coded_ouput if coded_ouput == code.five.as_ref().unwrap() => {
                decoded_patterns.push(5);
            }
            ref coded_ouput if coded_ouput == code.six.as_ref().unwrap() => {
                decoded_patterns.push(6);
            }
            ref coded_ouput if coded_ouput == code.seven.as_ref().unwrap() => {
                decoded_patterns.push(7);
            }
            ref coded_ouput if coded_ouput == code.eight.as_ref().unwrap() => {
                decoded_patterns.push(8);
            }
            ref coded_ouput if coded_ouput == code.nine.as_ref().unwrap() => {
                decoded_patterns.push(9);
            }
            ref coded_ouput if coded_ouput == code.zero.as_ref().unwrap() => {
                decoded_patterns.push(0);
            }
            _ => panic!("Unknown output pattern"),
        }
    }

    match decoded_patterns.iter().join("").parse::<u32>() {
        Ok(decoded_number) => Some(decoded_number),
        Err(_) => None,
    }
}

fn create_decoder(source: SourcePatterns) -> Code {
    let mut code = Code::new();

    // temp placeholder for length 5 codes - (2, 3, 5)
    let mut length_five: Vec<CodedNumber> = Vec::new();
    // temp placeholder for length 6 codes - (0, 6, 9)
    let mut length_six: Vec<CodedNumber> = Vec::new();

    for pattern in source {
        match pattern.len() {
            2 => match &code.one {
                Some(_) => (),
                None => {
                    let mut coded_number = HashSet::new();
                    for c in pattern.chars() {
                        coded_number.insert(c);
                    }
                    code.one = Some(coded_number);
                }
            },
            3 => match &code.seven {
                Some(_) => (),
                None => {
                    let mut coded_number = HashSet::new();
                    for c in pattern.chars() {
                        coded_number.insert(c);
                    }
                    code.seven = Some(coded_number);
                }
            },
            4 => match &code.four {
                Some(_) => (),
                None => {
                    let mut coded_number = HashSet::new();
                    for c in pattern.chars() {
                        coded_number.insert(c);
                    }
                    code.four = Some(coded_number);
                }
            },
            5 => {
                let mut coded_number = HashSet::new();
                for c in pattern.chars() {
                    coded_number.insert(c);
                }
                length_five.push(Some(coded_number));
            }
            6 => {
                let mut coded_number = HashSet::new();
                for c in pattern.chars() {
                    coded_number.insert(c);
                }
                length_six.push(Some(coded_number));
            }
            7 => match &code.eight {
                Some(_) => (),
                None => {
                    let mut coded_number = HashSet::new();
                    for c in pattern.chars() {
                        coded_number.insert(c);
                    }
                    code.eight = Some(coded_number);
                }
            },
            _ => unreachable!("This should never happen"),
        }
    }
    // Initial parsing of all numbers now need to work through the length 5 and 6 patterns
    // To do this we need to break down the segments so we know which code belongs where

    // Whole right side = code.one
    // Top = code.seven - code.one
    // Top Left + Middle = code.four - code. one
    // bottom left + bottom = code.six - code.four and code seven
    // top right = code.one - code.six
    // bottom right = code.one - top right
    // middle = code.six - code.zero

    if code.one.is_some() && code.seven.is_some() && code.four.is_some() {
        let one = code.one.as_ref().unwrap();
        let seven = code.seven.as_ref().unwrap();
        let four = code.four.as_ref().unwrap();

        let top: HashSet<char> = seven.difference(&one).cloned().collect();

        // Length 6 - 0, 6, 9
        for coded_number in length_six.clone() {
            match coded_number {
                // 6 is the only length 6 number that doesn't contain 1
                Some(coded_number) if !coded_number.is_superset(one) => {
                    if code.six.is_some() {
                        // This is a duplicate
                        continue;
                    }
                    code.six = Some(coded_number);
                }
                Some(coded_number)
                    // 9 is made up of 4 and the top row
                    if coded_number.is_superset(&four.union(&top).cloned().collect()) =>
                {
                    if code.nine.is_some() {
                        // This is a duplicate
                        continue;
                    }
                    code.nine = Some(coded_number);
                }
                Some(coded_number)
                    // Zero must not have middle (which we know from 4) and must contain 1
                    if !coded_number.is_superset(four) && coded_number.is_superset(one) =>
                {
                    if code.zero.is_some() {
                        // This is a duplicate
                        continue;
                    }
                    code.zero = Some(coded_number);
                }
                _ => {
                    println!("codes: {:?}", &code);
                    println!("Unknown length 6 pattern: {:?}", coded_number);
                    unreachable!("All length 6 numbers should be in one of the sets");
                }
            }
        }

        // Length 5 - 2, 3, 5
        // bottom left = code.six - code.nine
        let bottom_left: HashSet<char> = code
            .six
            .as_ref()
            .unwrap()
            .difference(&code.nine.as_ref().unwrap())
            .cloned()
            .collect();

        for coded_number in length_five.clone() {
            match coded_number {
                Some(coded_number) if coded_number.is_superset(&bottom_left) => {
                    if code.two.is_some() {
                        // This is a duplicate
                        continue;
                    }
                    code.two = Some(coded_number);
                }
                Some(coded_number) if coded_number.is_superset(one) => {
                    if code.three.is_some() {
                        // This is a duplicate
                        continue;
                    }
                    code.three = Some(coded_number);
                }
                Some(coded_number)
                    if !coded_number.is_superset(one)
                        && !coded_number.is_superset(&bottom_left) =>
                {
                    if code.five.is_some() {
                        // This is a duplicate
                        continue;
                    }
                    code.five = Some(coded_number);
                }
                _ => unreachable!("All length 5 numbers should be in one of the sets"),
            }
        }
    } else {
        panic!("Not all required patterns were found");
    }

    code
}

fn parse_input(input: &str) -> Vec<InputSource> {
    let mut data: Vec<InputSource> = Vec::new();

    for line in input.lines() {
        let mut source_patterns: SourcePatterns = Default::default();
        let mut output_patterns: OutputPatterns = Default::default();

        let mut source_pattern_index = 0;
        let mut output_pattern_index = 0;

        match line.split_once("|") {
            Some((source_patterns_str, output_patterns_str)) => {
                for source_pattern_str in source_patterns_str.trim().split(" ") {
                    source_patterns[source_pattern_index] = source_pattern_str.trim().to_string();
                    source_pattern_index += 1;
                }

                for output_pattern_str in output_patterns_str.trim().split(" ") {
                    output_patterns[output_pattern_index] = output_pattern_str.trim().to_string();
                    output_pattern_index += 1;
                }
            }
            None => {
                panic!("Invalid input");
            }
        }
        data.push((source_patterns, output_patterns));
    }
    data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = String::from(
            "\
            be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe\n\
            edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc\n\
            fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg\n\
            fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb\n\
            aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea\n\
            fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb\n\
            dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe\n\
            bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef\n\
            egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb\n\
            gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce\n\
        ");
        let result = part1(&input);
        let expected_result = 26;
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_part2() {
        let input = String::from(
            "\
            be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe\n\
            edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc\n\
            fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg\n\
            fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb\n\
            aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea\n\
            fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb\n\
            dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe\n\
            bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef\n\
            egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb\n\
            gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce\n\
        ");
        let result = part2(&input);
        let expected_result = 61229;
        assert_eq!(result, expected_result);
    }
}

// Manual notes for how to work out what is what.

// be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |fdgacbe cefdb cefbgd gcbe
// ------------------------------------------------------------
// 8 = length of 7 - cfbegad
// 1 = length of 2 - be
// 7 = length of 3 - bde
// 4 = length of 4 - bceg
// 6 = length of 6 and doens't contain value of 1 - acdefg
// 9 = length of 6 and doesn't contain bottom left - bcdefg
// 0 = length of 6 and has bottom left and value of 1 - abdefg
// 2 = combine top+bottom left and bottom + top and one of right side - abcdf
// 3 = combine right side + top + middle + one of bottom/bottom left = bcdef
// 5 = top + top left + middle + bottom right + bottom   = cdefg

// top known based on 7 - d
// right side known based on 2 - be
// top left and middle known based on 4 - cg
// bottom left and bottom known based on 6 - 7 and 4 - af
// middle = from getting 0 and - doing 6 minus 0  - c
// top right = b
// bottom right = e
// bottom - from working out 3 = f
// bottom left = a

// lengths =

// 0 = 6
// 1 = 2
// 2 = 5
// 3 = 5
// 4 = 4
// 5 = 5
// 6 = 6
// 7 = 3
// 8 = 7
// 9 = 6

// -----------

// Length of:
// 2 - Number 1
// 3 - Number 7
// 4 - Number 4
// 5 - Numbers: 2, 3, 5
// 6 - Number: 0, 6, 9
// 7 - Number: 8
