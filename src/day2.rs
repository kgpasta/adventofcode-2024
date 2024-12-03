use crate::common::process_file_by_line;

pub fn day2_part1(file_name: &str) -> i32 {
    let lines = process_file_by_line(file_name).unwrap();

    let mut valid_lines = 0;
    for line in lines {
        let line = line.unwrap();
        let valid_line =
            validate_report(line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect());
        valid_lines += valid_line;
    }

    return valid_lines;
}

pub fn day2_part2(file_name: &str) -> i32 {
    let lines = process_file_by_line(file_name).unwrap();

    let mut valid_lines = 0;
    for line in lines {
        let line = line.unwrap();
        let parsed_vector: Vec<i32> = line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect();
        let mut valid_line = validate_report(parsed_vector.clone());
        let mut counter = 0;
        while valid_line == 0 && counter < parsed_vector.len() {
            let mut new_vector = parsed_vector.clone();
            new_vector.remove(counter);
            valid_line = validate_report(new_vector);
            counter += 1;
        }
        valid_lines += valid_line;
    }

    return valid_lines;
}

fn validate_report(report: Vec<i32>) -> i32 {
    let mut prev_value = report[1];
    let direction = (prev_value - report[0]).signum();
    if direction == 0 {
        return 0;
    }
    let difference = (prev_value - report[0]).abs();
    if difference > 3 || difference == 0 {
        return 0;
    }

    for value in report.iter().skip(2) {
        if direction == 1 {
            if value < &prev_value {
                return 0;
            }
        } else if direction == -1 {
            if value > &prev_value {
                return 0;
            }
        }

        let difference = (value - prev_value).abs();
        if difference > 3 || difference == 0 {
            return 0;
        }

        prev_value = *value;
    }

    return 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day2_part1_sample() {
        let result = day2_part1("data/day2/sample.txt");
        assert_eq!(result, 2);
    }

    #[test]
    fn test_day2_part1_real() {
        let result = day2_part1("data/day2/day2.txt");
        assert_eq!(result, 100);
    }

    #[test]
    fn test_day2_part2_sample() {
        let result = day2_part2("data/day2/sample.txt");
        assert_eq!(result, 4);
    }

    #[test]
    fn test_day2_part2_real() {
        let result = day2_part2("data/day2/day2.txt");
        assert_eq!(result, 612);
    }
}
