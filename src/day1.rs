use crate::common::process_file_by_line;
use std::collections::{BinaryHeap, HashMap};

pub fn day1_part1(file_name: &str) -> i32 {
    let mut heap1 = BinaryHeap::new();
    let mut heap2 = BinaryHeap::new();
    process_file_by_line(format!("data/day1/{file_name}").as_str())
        .unwrap()
        .for_each(|line| map_line_to_heaps(parse_lines(&line.unwrap()), &mut heap1, &mut heap2));

    let mut total_distance = 0;
    while !heap1.is_empty() && !heap2.is_empty() {
        let value1 = heap1.pop().unwrap();
        let value2 = heap2.pop().unwrap();
        let distance = (value1 - value2).abs();
        total_distance += distance;
    }

    return total_distance;
}

pub fn day1_part2(file_name: &str) -> i32 {
    let mut heap1 = BinaryHeap::new();
    let mut hashmap2 = HashMap::new();
    process_file_by_line(format!("data/day1/{file_name}").as_str())
        .unwrap()
        .for_each(|line| {
            map_line_to_heap_and_hashmap(parse_lines(&line.unwrap()), &mut heap1, &mut hashmap2)
        });

    let mut total_similarity = 0;
    while !heap1.is_empty() {
        let value1 = heap1.pop().unwrap();
        let value2 = hashmap2.get(&value1).unwrap_or(&0);
        let similarity = value1 * value2;
        total_similarity += similarity;
    }

    return total_similarity;
}

fn parse_lines(line: &str) -> Vec<i32> {
    return line
        .split("   ")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
}

fn map_line_to_heaps(values: Vec<i32>, heap1: &mut BinaryHeap<i32>, heap2: &mut BinaryHeap<i32>) {
    heap1.push(values[0]);
    heap2.push(values[1]);
}

fn map_line_to_heap_and_hashmap(
    values: Vec<i32>,
    heap: &mut BinaryHeap<i32>,
    hashmap: &mut HashMap<i32, i32>,
) {
    heap.push(values[0]);
    let current_count = hashmap.get(&values[1]).unwrap_or(&0);
    hashmap.insert(values[1], current_count + 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1_part1_sample() {
        let result = day1_part1("sample.txt");
        assert_eq!(result, 11);
    }

    #[test]
    fn test_day1_part1_real() {
        let result = day1_part1("day1.txt");
        assert_eq!(result, 2066446);
    }

    #[test]
    fn test_day1_part2_sample() {
        let result = day1_part2("sample.txt");
        assert_eq!(result, 31);
    }

    #[test]
    fn test_day1_part2_real() {
        let result = day1_part2("day1.txt");
        assert_eq!(result, 24931009);
    }
}
