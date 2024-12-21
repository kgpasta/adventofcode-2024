use std::collections::HashMap;

use crate::common::read_all_lines;

const BLINKS_ONE: i32 = 25;
const BLINKS_TWO: i32 = 75;

pub fn day11_part1(file_name: &str) -> i64 {
    let mut stones = read_stones(file_name);
    let mut cached_values: HashMap<i64, (i64, i64)> = HashMap::new();

    for _i in 0..BLINKS_ONE {
        let new_stones = split_stones(&stones, &mut cached_values);

        stones = new_stones;
    }

    stones.len() as i64
}

fn split_stones(stones: &[i64], cached_values: &mut HashMap<i64, (i64, i64)>) -> Vec<i64> {
    let mut new_stones: Vec<i64> = Vec::new();

    for stone in stones {
        if cached_values.contains_key(stone) {
            let (one, two) = cached_values.get(stone).unwrap();
            new_stones.push(*one);
            if *two != 0 {
                new_stones.push(*two);
            }
        } else {
            let (one, two) = calculate_next_values(*stone);
            cached_values.insert(*stone, (one, two));
            new_stones.push(one);
            if two != 0 {
                new_stones.push(two);
            }
        }
    }

    new_stones
}

fn calculate_next_values(stone: i64) -> (i64, i64) {
    let stone_str: String = stone.to_string();

    if stone == 0 {
        return (1, 0);
    } else if stone_str.len() % 2 == 0 {
        let half = stone_str.len() / 2;
        let (left, right) = stone_str.split_at(half);
        let left_num = i64::from_str_radix(left, 10).unwrap();
        let right_num = i64::from_str_radix(right, 10).unwrap();

        return (left_num, right_num);
    } else {
        return (stone * 2024, 0);
    }
}

fn read_stones(file_name: &str) -> Vec<i64> {
    read_all_lines(file_name)
        .unwrap()
        .first()
        .unwrap()
        .split(' ')
        .map(|char| i64::from_str_radix(char, 10).unwrap())
        .collect()
}

pub fn day11_part2(file_name: &str) -> i64 {
    let stones = read_stones(file_name);
    let mut cached_values: HashMap<(i64, i32), i64> = HashMap::new();

    let mut count = 0;
    for stone in stones {
        count += split_stones_fast(stone, BLINKS_TWO, &mut cached_values);
    }

    count
}

fn split_stones_fast(
    stone: i64,
    blinks_left: i32,
    cached_values: &mut HashMap<(i64, i32), i64>,
) -> i64 {
    if cached_values.contains_key(&(stone, blinks_left)) {
        return *cached_values.get(&(stone, blinks_left)).unwrap();
    }

    if blinks_left == 0 {
        return 1;
    }

    let stone_str: String = stone.to_string();

    if stone == 0 {
        let result = split_stones_fast(1, blinks_left - 1, cached_values);
        cached_values.insert((stone, blinks_left), result);
    } else if stone_str.len() % 2 == 0 {
        let half = stone_str.len() / 2;
        let (left, right) = stone_str.split_at(half);
        let left_num = i64::from_str_radix(left, 10).unwrap();
        let right_num = i64::from_str_radix(right, 10).unwrap();

        let result = split_stones_fast(left_num, blinks_left - 1, cached_values)
            + split_stones_fast(right_num, blinks_left - 1, cached_values);
        cached_values.insert((stone, blinks_left), result);
    } else {
        let result = split_stones_fast(stone * 2024, blinks_left - 1, cached_values);
        cached_values.insert((stone, blinks_left), result);
    }

    return *cached_values.get(&(stone, blinks_left)).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day11_part1_example() {
        let result = day11_part1("data/day11/sample.txt");
        assert_eq!(result, 55312);
    }

    #[test]
    fn test_day11_part1_input() {
        let result = day11_part1("data/day11/day11.txt");
        assert_eq!(result, 0);
    }

    #[test]
    fn test_day11_part2_example() {
        let result = day11_part2("data/day11/sample.txt");
        assert_eq!(result, 65601038650482);
    }

    #[test]
    fn test_day11_part2_input() {
        let result = day11_part2("data/day11/day11.txt");
        assert_eq!(result, 266820198587914);
    }
}
