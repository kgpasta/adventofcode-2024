use std::{
    collections::{HashMap, HashSet},
    vec,
};

use crate::common::process_file_by_line;

pub fn day5_part1(file_name: &str) -> i32 {
    let (rule_map, pages) = get_rules_pages(file_name);

    let valid_pages = pages
        .iter()
        .filter(|page| check_valid_page(page, &rule_map));

    valid_pages.map(|page| get_middle_element(page)).sum()
}

fn get_rules_pages(file_name: &str) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let lines: Vec<String> = process_file_by_line(file_name)
        .unwrap()
        .map(|line| line.unwrap())
        .filter(|line| !line.is_empty())
        .collect();

    let (rules, pages) = lines.split_at(lines.partition_point(|line| line.contains("|")));

    let rule_map = rules.iter().fold(HashMap::new(), |mut acc, line| {
        let parts: Vec<&str> = line.split("|").collect();
        let first = parts[0].parse::<i32>().unwrap();
        let second = parts[1].parse::<i32>().unwrap();
        if !acc.contains_key(&first) {
            acc.insert(first, vec![]);
        }
        acc.get_mut(&first).unwrap().push(second);
        acc
    });

    let pages: Vec<Vec<i32>> = pages
        .iter()
        .map(|page| page.split(",").map(|x| x.parse::<i32>().unwrap()).collect())
        .collect();

    return (rule_map, pages);
}

fn get_middle_element(collect: &Vec<i32>) -> i32 {
    collect[collect.len() / 2]
}

fn check_valid_page(collect: &Vec<i32>, rule_map: &HashMap<i32, Vec<i32>>) -> bool {
    let mut value_set = HashSet::new();
    for val in collect {
        if rule_map.contains_key(val) {
            let values = rule_map.get(val).unwrap();
            for value in values {
                if value_set.contains(value) {
                    return false;
                }
            }
        }

        value_set.insert(val);
    }

    return true;
}

pub fn day5_part2(file_name: &str) -> i32 {
    let (rule_map, pages) = get_rules_pages(file_name);

    let invalid_pages: Vec<Vec<i32>> = pages
        .iter()
        .filter(|page| !check_valid_page(page, &rule_map))
        .cloned()
        .collect();

    let mut sum = 0;
    for mut page in invalid_pages {
        let mut index = 1;

        while index < page.len() {
            let value = page[index];

            let prev_values = page[0..index].to_vec();
            let empty_vec = vec![];
            let rule_values = rule_map.get(&value).unwrap_or(&empty_vec);

            let mut updated = false;
            for i in 0..prev_values.len() {
                let prev_value = prev_values[i];
                if rule_values.contains(&prev_value) {
                    page.remove(index);
                    page.insert(i, value);
                    updated = true;
                    break;
                }
            }

            if !updated {
                index += 1;
            }
        }

        sum += get_middle_element(&page);
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day5_part1_example() {
        let result = day5_part1("data/day5/sample.txt");
        assert_eq!(result, 143);
    }

    #[test]
    fn test_day5_part1_input() {
        let result = day5_part1("data/day5/day5.txt");
        assert_eq!(result, 4185);
    }

    #[test]
    fn test_day5_part2_example() {
        let result = day5_part2("data/day5/sample.txt");
        assert_eq!(result, 123);
    }

    #[test]
    fn test_day5_part2_input() {
        let result = day5_part2("data/day5/day5.txt");
        assert_eq!(result, 4480);
    }
}
