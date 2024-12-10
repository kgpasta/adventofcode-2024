use crate::common::read_all_lines;

pub fn day9_part1(file_name: &str) -> i64 {
    let text = read_all_lines(file_name).unwrap().join("");
    let (mut expanded_vector, mut stack) = expand_vector(text.as_str());
    let desired_size = stack.len();

    for i in 0..expanded_vector.len() {
        if i == desired_size {
            break;
        }

        if expanded_vector[i] == -1 {
            expanded_vector[i] = stack.pop().unwrap();
        }
    }

    let clamped_vector = expanded_vector
        .iter()
        .cloned()
        .take(desired_size)
        .collect::<Vec<i32>>();

    calculate_checksum(clamped_vector)
}

fn calculate_checksum(clamped_vector: Vec<i32>) -> i64 {
    let mut total_sum = 0;
    for i in 0..clamped_vector.len() {
        if clamped_vector[i] == -1 {
            continue;
        }
        total_sum += clamped_vector[i] as i64 * i as i64;
    }

    total_sum
}

fn expand_vector(text: &str) -> (Vec<i32>, Vec<i32>) {
    let mut id_counter = 0;
    let mut expanded_vector = vec![];
    let mut stack = vec![];
    for (idx, character) in text.chars().enumerate() {
        let size = character.to_digit(10).unwrap() as usize;

        for _ in 0..size {
            if idx % 2 == 0 {
                expanded_vector.push(id_counter);
                stack.push(id_counter);
            } else {
                expanded_vector.push(-1)
            }
        }

        if idx % 2 == 0 {
            id_counter += 1;
        }
    }

    (expanded_vector, stack)
}

pub fn day9_part2(file_name: &str) -> i64 {
    let text = read_all_lines(file_name).unwrap().join("");
    let (mut expanded_vector, _) = expand_vector(text.as_str());

    let mut index = expanded_vector.len() - 1;
    while index > 0 {
        let (starting_index, length) = find_next_empty_space(&expanded_vector, index);
        if starting_index > 0 {
            for i in 0..length {
                expanded_vector[starting_index + i] = expanded_vector[index - i];
                expanded_vector[index - i] = -1;
            }
        }

        index -= length;
    }

    calculate_checksum(expanded_vector)
}

fn find_next_empty_space(expanded_vector: &[i32], index: usize) -> (usize, usize) {
    let first_value = expanded_vector[index];
    let mut current_index = index - 1;

    while expanded_vector[current_index] == first_value {
        if current_index == 0 {
            break;
        }
        current_index -= 1
    }

    let length = index - current_index;

    let mut length_left = length;
    for (idx, character) in expanded_vector.iter().enumerate() {
        if idx > index {
            break;
        }

        if character == &-1 {
            length_left -= 1;
        } else {
            length_left = length;
        }

        if length_left == 0 {
            return (idx - length + 1, length);
        }
    }

    return (0, length);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day9_part1_example() {
        let result = day9_part1("data/day9/sample.txt");
        assert_eq!(result, 1928);
    }

    #[test]
    fn test_day9_part1_input() {
        let result = day9_part1("data/day9/day9.txt");
        assert_eq!(result, 6370402949053);
    }

    #[test]
    fn test_day9_part2_example() {
        let result = day9_part2("data/day9/sample.txt");
        assert_eq!(result, 2858);
    }

    #[test]
    fn test_day9_part2_input() {
        let result = day9_part2("data/day9/day9.txt");
        assert_eq!(result, 6398096697992);
    }
}
