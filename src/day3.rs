use ndarray::prelude::*;
use ndarray::OwnedRepr;

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

fn parse_input(input: &str) -> ArrayBase<OwnedRepr<u32>, Dim<[usize; 2]>> {
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
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
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
}

// --- Day 3: Binary Diagnostic ---
// The submarine has been making some odd creaking noises, so you ask it to produce a diagnostic report just in case.

// The diagnostic report (your puzzle input) consists of a list of binary numbers which, when decoded properly, can tell you many useful things about the conditions of the submarine. The first parameter to check is the power consumption.

// You need to use the binary numbers in the diagnostic report to generate two new binary numbers (called the gamma rate and the epsilon rate). The power consumption can then be found by multiplying the gamma rate by the epsilon rate.

// Each bit in the gamma rate can be determined by finding the most common bit in the corresponding position of all numbers in the diagnostic report. For example, given the following diagnostic report:

// 00100
// 11110
// 10110
// 10111
// 10101
// 01111
// 00111
// 11100
// 10000
// 11001
// 00010
// 01010
// Considering only the first bit of each number, there are five 0 bits and seven 1 bits. Since the most common bit is 1, the first bit of the gamma rate is 1.

// The most common second bit of the numbers in the diagnostic report is 0, so the second bit of the gamma rate is 0.

// The most common value of the third, fourth, and fifth bits are 1, 1, and 0, respectively, and so the final three bits of the gamma rate are 110.

// So, the gamma rate is the binary number 10110, or 22 in decimal.

// The epsilon rate is calculated in a similar way; rather than use the most common bit, the least common bit from each position is used. So, the epsilon rate is 01001, or 9 in decimal. Multiplying the gamma rate (22) by the epsilon rate (9) produces the power consumption, 198.

// Use the binary numbers in your diagnostic report to calculate the gamma rate and epsilon rate, then multiply them together. What is the power consumption of the submarine? (Be sure to represent your answer in decimal, not binary.)
