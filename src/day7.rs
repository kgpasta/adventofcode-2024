use crate::common::process_file_by_line;

struct Equation {
    calibration_result: i64,
    arguments: Vec<i64>,
}

pub fn day7_part1(file_name: &str) -> i64 {
    return process_file_by_line(file_name)
        .unwrap()
        .map(|x| map_line_to_equation(x.unwrap()))
        .filter(|x| check_if_valid_line(0, 0, x))
        .map(|x| x.calibration_result)
        .sum();
}

fn check_if_valid_line(current_result: i64, index: usize, x: &Equation) -> bool {
    if index >= x.arguments.len() {
        return current_result == x.calibration_result;
    }

    let current_argument = x.arguments[index as usize];
    return check_if_valid_line(current_result + current_argument, index + 1, x)
        || check_if_valid_line(current_result * current_argument, index + 1, x);
}

fn check_if_valid_line_concat(current_result: i64, index: usize, x: &Equation) -> bool {
    if index >= x.arguments.len() || current_result > x.calibration_result {
        return current_result == x.calibration_result;
    }

    let current_argument = x.arguments[index as usize];
    let mut concat = current_result.to_string();
    concat.push_str(&current_argument.to_string());
    return check_if_valid_line_concat(current_result + current_argument, index + 1, x)
        || check_if_valid_line_concat(concat.parse::<i64>().unwrap(), index + 1, x)
        || check_if_valid_line_concat(current_result * current_argument, index + 1, x);
}

fn map_line_to_equation(x: String) -> Equation {
    let parts: Vec<&str> = x.split(":").collect();

    return Equation {
        calibration_result: parts[0].parse::<i64>().unwrap(),
        arguments: parts[1]
            .trim()
            .split(" ")
            .map(|x| x.parse::<i64>().unwrap())
            .collect(),
    };
}

pub fn day7_part2(file_name: &str) -> i64 {
    return process_file_by_line(file_name)
        .unwrap()
        .map(|x| map_line_to_equation(x.unwrap()))
        .filter(|x| check_if_valid_line_concat(x.arguments[0], 1, x))
        .map(|x| x.calibration_result)
        .sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day7_part1_example() {
        let result = day7_part1("data/day7/sample.txt");
        assert_eq!(result, 3749);
    }

    #[test]
    fn test_day7_part1_input() {
        let result = day7_part1("data/day7/day7.txt");
        assert_eq!(result, 6083020304036);
    }

    #[test]
    fn test_day7_part2_example() {
        let result = day7_part2("data/day7/sample.txt");
        assert_eq!(result, 11387);
    }

    #[test]
    fn test_day7_part2_input() {
        let result = day7_part2("data/day7/day7.txt");
        assert_eq!(result, 59002246504791);
    }
}
