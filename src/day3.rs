use std::num::ParseIntError;

use ndarray::prelude::*;
use ndarray::OwnedRepr;

#[derive(Debug)]
enum BitCriteria {
    LeastCommon = 0,
    MostCommon = 1,
}

#[derive(Debug)]
enum SignificantBit {
    Zero = 0,
    One = 1,
    Equal,
}

pub fn part2(input: &str) -> u32 {
    let array_input = parse_input2(input);

    // Getting Oxygen Generator
    // Start with the first column, and find the most significant bit

    // Get Oxygen Generator
    // 1. Loop through the array while len < 1;
    // 2. Find the most significant bit
    // 3. Filter out the most significant bit
    let oxygen_generator_rating_str = convert_bit_vector_to_string(
        filter_by_criteria(&array_input, BitCriteria::MostCommon)[0].clone(),
    );

    // println!("O2{:?}", oxygen_generator_rating_str);

    let oxygen_generator_rating =
        convert_bit_string_to_u32(oxygen_generator_rating_str.as_str()).unwrap();

    let co2_generator_rating_str = convert_bit_vector_to_string(
        filter_by_criteria(&array_input, BitCriteria::LeastCommon)[0].clone(),
    );

    // println!("CO2:{:?}", co2_generator_rating_str);

    let co2_generator_rating =
        convert_bit_string_to_u32(co2_generator_rating_str.as_str()).unwrap();

    let life_support_rating = oxygen_generator_rating * co2_generator_rating;

    return life_support_rating;
}

fn convert_bit_string_to_u32(array: &str) -> Result<u32, ParseIntError> {
    match u32::from_str_radix(array, 2) {
        Ok(num) => Ok(num),
        Err(e) => Err(e),
    }
}

fn convert_bit_vector_to_string(array: Vec<u8>) -> String {
    array
        .iter()
        .map(|f| match f {
            0 => "0",
            1 => "1",
            _ => unreachable!("This should never happen"),
        })
        .collect::<Vec<&str>>()
        .join("")
}

fn filter_by_criteria(array: &Vec<Vec<u8>>, criteria: BitCriteria) -> Vec<Vec<u8>> {
    let mut filtered_array = array.clone();

    // Loop through the array columns, remove those that don't match criteria until no columns remain or only 1 row is left.
    let mut bit_position = 0;

    while filtered_array.len() > 1 {
        // Make sure we also haven't already filtered through all the bits
        if bit_position > filtered_array[0].len() {
            break;
        }

        // Get the most significant bit
        let significant_bit = get_significant_bit(
            filtered_array
                .iter()
                .map(|row| row[bit_position])
                .collect::<Vec<u8>>(),
        );

        // Iterate through rows in array and if the bit position doesn't match the critera then drop the row.
        filtered_array = filtered_array
            .iter()
            .filter(|column| match criteria {
                BitCriteria::MostCommon => {
                    // We want to filter based on the signifcant bit, or 1 if equal
                    // println!(
                    //     "{:?} {:?} {:?} {:?}",
                    //     column, bit_position, criteria, significant_bit
                    // );
                    match significant_bit {
                        SignificantBit::Zero => column[bit_position] == 0,
                        SignificantBit::One | SignificantBit::Equal => column[bit_position] == 1,
                    }
                }
                BitCriteria::LeastCommon => {
                    // We want to filter based on the opposite of the signficant bit, or 0 if equal
                    match significant_bit {
                        SignificantBit::Zero => column[bit_position] == 1,
                        SignificantBit::One | SignificantBit::Equal => column[bit_position] == 0,
                    }
                }
            })
            .map(|row| row.to_owned())
            .collect();
        bit_position += 1
    }

    return filtered_array;
}

fn get_significant_bit(array: Vec<u8>) -> SignificantBit {
    let mut num_ones = 0;
    let mut num_zeros = 0;

    for bit in array {
        match bit {
            0 => num_zeros += 1,
            1 => num_ones += 1,
            _ => unreachable!("There should only be ones and zeros"),
        }
    }

    match num_ones {
        num_ones if num_ones < num_zeros => SignificantBit::Zero,
        num_ones if num_ones == num_zeros => SignificantBit::Equal,
        num_ones if num_ones > num_zeros => SignificantBit::One,
        _ => unreachable!("We should never get here!"),
    }
}

pub fn part1(input: &str) -> u32 {
    // Input is a x position binary string
    // Parse the input into  2 dimensional array (using ndarray)
    let array_input = parse_input(input);

    let mut episilon_rate_str = String::new();
    // Count the 1's in each vector tuples in each position .filter.count maybe?

    let array_rows = array_input.shape()[0];

    array_input.axis_iter(Axis(1)).for_each(|column| {
        if column.iter().filter(|&&x| x == 1).count() > (array_rows / 2) as usize {
            episilon_rate_str.push_str("1")
        } else {
            episilon_rate_str.push_str("0")
        }
    });

    let episilon_rate = u32::from_str_radix(episilon_rate_str.as_str(), 2).unwrap();
    let gamma_rate =
        u32::from_str_radix(binary_string_flip(&episilon_rate_str).as_str(), 2).unwrap();
    episilon_rate * gamma_rate
}

fn binary_string_flip(input: &str) -> String {
    let mut output = String::new();
    for c in input.chars() {
        if c == '0' {
            output.push_str("1")
        } else {
            output.push_str("0")
        }
    }
    return output;
}

fn parse_input(input: &str) -> ArrayBase<OwnedRepr<u8>, Dim<[usize; 2]>> {
    //

    let lines = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '0' => 0b0,
                    '1' => 0b1,
                    _ => panic!("Invalid input"),
                })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();
    let length_rows = lines.len();
    let length_columns = lines[0].len();
    println!("Rows: {}", length_rows);
    println!("Columns: {}", length_columns);
    // println!("{:?}", lines);
    let mut input_array = Array2::zeros((length_rows, length_columns));
    for (i, row) in lines.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            input_array[[i, j]] = *col;
        }
    }
    input_array
}

fn parse_input2(input: &str) -> Vec<Vec<u8>> {
    //

    let lines = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '0' => 0b0,
                    '1' => 0b1,
                    _ => panic!("Invalid input"),
                })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();
    // let length_rows = lines.len();
    // let length_columns = lines[0].len();
    // println!("Rows: {}", length_rows);
    // println!("Columns: {}", length_columns);
    // // println!("{:?}", lines);
    // let mut input_array = Array2::zeros((length_rows, length_columns));
    // for (i, row) in lines.iter().enumerate() {
    //     for (j, col) in row.iter().enumerate() {
    //         input_array[[i, j]] = *col;
    //     }
    // }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input: String = String::from(
            "\
        00100\n\
        11110\n\
        10110\n\
        10111\n\
        10101\n\
        01111\n\
        00111\n\
        11100\n\
        10000\n\
        11001\n\
        00010\n\
        01010\n",
        );

        let result = part1(&input);
        let expected_result = 198;
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_part2() {
        let input: String = String::from(
            "\
        00100\n\
        11110\n\
        10110\n\
        10111\n\
        10101\n\
        01111\n\
        00111\n\
        11100\n\
        10000\n\
        11001\n\
        00010\n\
        01010\n",
        );

        let result = part2(&input);
        let expected_result = 230;
        assert_eq!(result, expected_result);
    }
}

// --- Part Two ---
// Next, you should verify the life support rating, which can be determined by multiplying the oxygen generator rating by the CO2 scrubber rating.

// Both the oxygen generator rating and the CO2 scrubber rating are values that can be found in your diagnostic report - finding them is the tricky part. Both values are located using a similar process that involves filtering out values until only one remains. Before searching for either rating value, start with the full list of binary numbers from your diagnostic report and consider just the first bit of those numbers. Then:

// Keep only numbers selected by the bit criteria for the type of rating value for which you are searching. Discard numbers which do not match the bit criteria.
// If you only have one number left, stop; this is the rating value for which you are searching.
// Otherwise, repeat the process, considering the next bit to the right.
// The bit criteria depends on which type of rating value you want to find:

// To find oxygen generator rating, determine the most common value (0 or 1) in the current bit position, and keep only numbers with that bit in that position. If 0 and 1 are equally common, keep values with a 1 in the position being considered.
// To find CO2 scrubber rating, determine the least common value (0 or 1) in the current bit position, and keep only numbers with that bit in that position. If 0 and 1 are equally common, keep values with a 0 in the position being considered.
// For example, to determine the oxygen generator rating value using the same example diagnostic report from above:

// Start with all 12 numbers and consider only the first bit of each number. There are more 1 bits (7) than 0 bits (5), so keep only the 7 numbers with a 1 in the first position: 11110, 10110, 10111, 10101, 11100, 10000, and 11001.
// Then, consider the second bit of the 7 remaining numbers: there are more 0 bits (4) than 1 bits (3), so keep only the 4 numbers with a 0 in the second position: 10110, 10111, 10101, and 10000.
// In the third position, three of the four numbers have a 1, so keep those three: 10110, 10111, and 10101.
// In the fourth position, two of the three numbers have a 1, so keep those two: 10110 and 10111.
// In the fifth position, there are an equal number of 0 bits and 1 bits (one each). So, to find the oxygen generator rating, keep the number with a 1 in that position: 10111.
// As there is only one number left, stop; the oxygen generator rating is 10111, or 23 in decimal.
// Then, to determine the CO2 scrubber rating value from the same example above:

// Start again with all 12 numbers and consider only the first bit of each number. There are fewer 0 bits (5) than 1 bits (7), so keep only the 5 numbers with a 0 in the first position: 00100, 01111, 00111, 00010, and 01010.
// Then, consider the second bit of the 5 remaining numbers: there are fewer 1 bits (2) than 0 bits (3), so keep only the 2 numbers with a 1 in the second position: 01111 and 01010.
// In the third position, there are an equal number of 0 bits and 1 bits (one each). So, to find the CO2 scrubber rating, keep the number with a 0 in that position: 01010.
// As there is only one number left, stop; the CO2 scrubber rating is 01010, or 10 in decimal.
// Finally, to find the life support rating, multiply the oxygen generator rating (23) by the CO2 scrubber rating (10) to get 230.

// Use the binary numbers in your diagnostic report to calculate the oxygen generator rating and CO2 scrubber rating, then multiply them together. What is the life support rating of the submarine? (Be sure to represent your answer in decimal, not binary.)
