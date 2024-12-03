use crate::common::{process_file_by_line, read_all_lines};
use regex::Regex;

pub fn day3_part1(file_name: &str) -> i32 {
    process_file_by_line(file_name)
        .unwrap()
        .map(|line| find_muls(line.unwrap()))
        .sum()
}

fn find_muls(unwrap: String) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.find_iter(&unwrap)
        .map(|m| {
            let cap = re.captures(m.as_str()).unwrap();
            cap[1].parse::<i32>().unwrap() * cap[2].parse::<i32>().unwrap()
        })
        .sum()
}

pub fn day3_part2(file_name: &str) -> i32 {
    let mut text = read_all_lines(file_name).unwrap().join("");

    let mut index = 0;
    let mut total_count = 0;
    while index < text.len() {
        let end_index = text.find("don't").unwrap_or(text.len());
        let subtext = &text[index..end_index];
        let count = find_muls(subtext.to_string());

        total_count += count;

        if end_index == text.len() {
            break;
        }
        text = text[end_index + 5..].to_string();
        index = text.find("do").unwrap_or(text.len());
    }

    total_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day3_part1_example() {
        let result = day3_part1("data/day3/sample.txt");
        assert_eq!(result, 161);
    }

    #[test]
    fn test_day3_part1_input() {
        let result = day3_part1("data/day3/day3.txt");
        assert_eq!(result, 181345830);
    }

    #[test]
    fn test_day3_part2_example() {
        let result = day3_part2("data/day3/sample2.txt");
        assert_eq!(result, 48);
    }

    #[test]
    fn test_day3_part2_input() {
        let result = day3_part2("data/day3/day3.txt");
        assert_eq!(result, 98729041);
    }
}
