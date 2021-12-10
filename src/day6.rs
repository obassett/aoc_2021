use std::collections::HashMap;

pub fn part2(input: &str) -> u64 {
    let mut world: HashMap<i32, u64> = HashMap::new();
    let day_counter = 256;

    input
        .split(",")
        .map(|fish| fish.trim().parse::<i32>().unwrap())
        .for_each(|fish| *world.entry(fish).or_insert(0) += 1);

    for _ in 1..=day_counter {
        let val = *world.get(&0).unwrap_or(&0);
        for i in 0..=8 {
            *world.entry(i).or_insert(0) = *world.entry(i + 1).or_insert(0);
        }
        *world.entry(6).or_insert(0) += val;
        *world.entry(8).or_insert(0) += val;
    }

    world.values().sum()
}

pub fn part1(input: &str) -> u64 {
    let mut world: HashMap<i32, u64> = HashMap::new();
    let day_counter = 80;

    input
        .split(",")
        .map(|fish| fish.trim().parse::<i32>().unwrap())
        .for_each(|fish| *world.entry(fish).or_insert(0) += 1);

    for _ in 1..=day_counter {
        let val = *world.get(&0).unwrap_or(&0);
        for i in 0..=8 {
            *world.entry(i).or_insert(0) = *world.entry(i + 1).or_insert(0);
        }
        *world.entry(6).or_insert(0) += val;
        *world.entry(8).or_insert(0) += val;
    }

    world.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = String::from("3,4,3,1,2");
        let result = part1(&input);
        let expected_result = 5934;
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_part2() {
        let input = String::from("3,4,3,1,2");
        let result = part2(&input);
        let expected_result: u64 = 26984457539;
        assert_eq!(result, expected_result);
    }
}
