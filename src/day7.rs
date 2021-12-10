fn median(list: &Vec<u64>) -> u64 {
    let mut list = list.clone();
    list.sort();
    let len = list.len();
    if len % 2 == 0 {
        (list[len / 2 - 1] + list[len / 2]) / 2
    } else {
        list[len / 2]
    }
}

fn mean(list: &Vec<u64>) -> u64 {
    let mut sum = 0;
    for i in list {
        sum += i;
    }
    sum / list.len() as u64
}

fn calc_individual_fuel_consumption(distance: u64) -> u64 {
    distance * (distance + 1) / 2
}

fn calc_total_fuel_consumption_of_crabs(crabs: &Vec<u64>, destination: u64) -> u64 {
    let mut sum = 0;
    for crab in crabs {
        let difference = (destination - crab) as i64;
        let abs_diff = difference.abs() as u64;
        sum += calc_individual_fuel_consumption(abs_diff);
    }
    sum
}

pub fn part2(input: &str) -> u64 {
    // parse input
    let crabs: Vec<u64> = input
        .split(",")
        .map(|x| x.trim().parse::<u64>().unwrap())
        .collect();

    // fuel cost is the a progression. each step is costs 1 mmore that the previous
    // e.g. moving 3 steps costs 6 fuel = 1 + 2 + 3
    //          x * (x+1) / 2
    //

    // Start with mean of all distances and check above and below then take the lowest
    let destination = mean(&crabs);

    let mean_consumption = calc_total_fuel_consumption_of_crabs(&crabs, destination);
    let mean_plus_consumption = calc_total_fuel_consumption_of_crabs(&crabs, destination + 1);
    let mean_minus_consumption = calc_total_fuel_consumption_of_crabs(&crabs, destination - 1);

    if mean_consumption < mean_plus_consumption && mean_consumption < mean_minus_consumption {
        println!("mean: {}", mean_consumption);
        return mean_consumption;
    } else if mean_plus_consumption < mean_consumption
        && mean_plus_consumption < mean_minus_consumption
    {
        println!("plus: {}", mean_plus_consumption);
        return mean_plus_consumption;
    } else {
        println!("minus: {}", mean_minus_consumption);
        return mean_minus_consumption;
    }
}

pub fn part1(input: &str) -> u64 {
    // parse input
    let crabs: Vec<u64> = input
        .split(",")
        .map(|x| x.trim().parse::<u64>().unwrap())
        .collect();

    // Find median of all numbers - which is the destination
    let destination = median(&crabs);

    // iterate through the numbers and find the distance to the median
    let mut fuel_consumption = 0;

    for crab in crabs {
        let difference = (destination - crab) as i64;
        fuel_consumption += difference.abs() as u64;
    }

    fuel_consumption
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = String::from("16,1,2,0,4,2,7,1,2,14");
        let result = part1(&input);
        let expected_result = 37;
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_part2() {
        let input = String::from("16,1,2,0,4,2,7,1,2,14");
        let result = part2(&input);
        let expected_result = 168;
        assert_eq!(result, expected_result);
    }
}
