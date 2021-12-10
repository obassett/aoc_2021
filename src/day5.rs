use std::collections::HashMap;
use thiserror::Error;

// Makes it easier to reference the x,y coordinates of a point
type MapPoint = (i32, i32);
// A line is defined by a start point and an end point
type MapLine = (MapPoint, MapPoint);

#[derive(Debug, Error)]
pub enum InputParseError {
    #[error("No Input")]
    NoInput,
    #[error("Invalid input")]
    InvalidFormat,
    #[error("Unable to Parse input as i32")]
    ParseIntError(#[from] std::num::ParseIntError),
}

fn process_line(input_line: &str) -> Result<MapLine, InputParseError> {
    let split_input = input_line.split_once(" -> ");

    match split_input {
        Some((start_points, end_points)) => {
            let (start_x, start_y) = match start_points.split_once(",") {
                Some((x, y)) => (x.parse::<i32>()?, y.parse::<i32>()?),
                None => return Err(InputParseError::InvalidFormat),
            };
            let (end_x, end_y) = match end_points.split_once(",") {
                Some((x, y)) => (x.parse::<i32>()?, y.parse::<i32>()?),
                None => return Err(InputParseError::InvalidFormat),
            };
            Ok(((start_x, start_y), (end_x, end_y)))
        }
        None => Err(InputParseError::InvalidFormat),
    }
}

struct VentMap {
    data: HashMap<MapPoint, i32>,
}

impl VentMap {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    fn add_point(&mut self, point: MapPoint) {
        if self.data.contains_key(&point) {
            *self.data.get_mut(&point).unwrap() += 1;
        } else {
            self.data.insert(point, 1);
        }
    }

    fn get_danger_level(&self, level: i32) -> i32 {
        // iterate through the values and return the number of entries that are >= level.
        self.data.iter().filter(|(_, &v)| v >= level).count() as i32
    }

    fn line_between_points(&mut self, start: MapPoint, end: MapPoint, diagonal: bool) {
        let (x_start, y_start) = start;
        let (x_end, y_end) = end;

        // Work out the direction of the line or None if x stays the same
        let x_range = if x_start < x_end {
            Some((x_start..=x_end).into_iter().collect::<Vec<i32>>())
        } else if x_start > x_end {
            Some((x_end..=x_start).rev().into_iter().collect::<Vec<i32>>())
        } else {
            None
        };

        // Work out the direction of the line or None if y stays the same
        let y_range = if y_start < y_end {
            Some((y_start..=y_end).into_iter().collect::<Vec<i32>>())
        } else if y_start > y_end {
            Some((y_end..=y_start).rev().into_iter().collect::<Vec<i32>>())
        } else {
            None
        };

        match x_range {
            Some(x_range) => match y_range {
                Some(y_range) => {
                    if diagonal {
                        x_range.iter().zip(y_range.iter()).for_each(|(&x, &y)| {
                            self.add_point((x, y));
                            // println!("Adding Point for diagonal Range{:?}", (x, y));
                        });
                    }
                }
                None => {
                    x_range.iter().for_each(|&x| {
                        self.add_point((x, y_start));
                        // println!("Adding Point for Horizontal range{:?}", (x, y_start));
                    });
                }
            },
            None => {
                match y_range {
                    Some(y_range) => {
                        y_range.iter().for_each(|&y| {
                            self.add_point((x_start, y));
                            // println!("Adding Point for Vertical range{:?}", (x_start, y));
                        });
                    }
                    None => {
                        // Add single point
                        self.add_point(start);
                    }
                }
            }
        }
    }
}

fn parse_input(input: &str) -> Result<Vec<MapLine>, InputParseError> {
    let mut output_vector = Vec::new();
    for line in input.lines() {
        match process_line(line) {
            Ok(line) => output_vector.push(line),
            Err(e) => return Err(e),
        }
    }
    Ok(output_vector)
}

pub fn part1(input: &str) -> Result<i32, InputParseError> {
    //
    let danger_threshold = 2;
    // For part 1 we ignore the diagonal lines
    let diagonal = false;

    // Parse input
    let map_lines = parse_input(input)?;

    //Define Data Structure
    let mut map = VentMap::new();
    //iterates through all points between start points and destination point -
    for (start_point, end_point) in map_lines {
        map.line_between_points(start_point, end_point, diagonal);
    }

    Ok(map.get_danger_level(danger_threshold))
}

pub fn part2(input: &str) -> Result<i32, InputParseError> {
    //
    let danger_threshold = 2;
    // For part 2 we care about the diagonal lines
    let diagonal = true;

    // Parse input
    let map_lines = parse_input(input)?;

    //Define Data Structure
    let mut map = VentMap::new();
    //iterates through all points between start points and destination point -
    for (start_point, end_point) in map_lines {
        map.line_between_points(start_point, end_point, diagonal);
    }

    Ok(map.get_danger_level(danger_threshold))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_between_points() {
        let mut map = VentMap::new();
        map.line_between_points((0, 6), (6, 0), true);

        let result = map.data.len();
        let expected: usize = 7;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part1() {
        let input: String = String::from(
            "\
            0,9 -> 5,9\n\
            8,0 -> 0,8\n\
            9,4 -> 3,4\n\
            2,2 -> 2,1\n\
            7,0 -> 7,4\n\
            6,4 -> 2,0\n\
            0,9 -> 2,9\n\
            3,4 -> 1,4\n\
            0,0 -> 8,8\n\
            5,5 -> 8,2\n\
            ",
        );

        let result = match part1(&input) {
            Ok(result) => result,
            Err(e) => panic!("Error: {}", e),
        };
        let expected_result = 5;
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_part2() {
        let input: String = String::from(
            "\
            0,9 -> 5,9\n\
            8,0 -> 0,8\n\
            9,4 -> 3,4\n\
            2,2 -> 2,1\n\
            7,0 -> 7,4\n\
            6,4 -> 2,0\n\
            0,9 -> 2,9\n\
            3,4 -> 1,4\n\
            0,0 -> 8,8\n\
            5,5 -> 8,2\n\
            ",
        );

        let result = match part2(&input) {
            Ok(result) => result,
            Err(e) => panic!("Error: {}", e),
        };
        let expected_result = 12;
        assert_eq!(result, expected_result);
    }
}
