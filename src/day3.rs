pub fn day3_part1(input: &str) -> i32 {
    0 // TODO: Implement solution
}

pub fn day3_part2(input: &str) -> i32 {
    0 // TODO: Implement solution
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day3_part1_example() {
        let input = std::fs::read_to_string("data/day3/example.txt").unwrap();
        assert_eq!(day3_part1(&input), 0);
    }

    #[test]
    fn test_day3_part1_input() {
        let input = std::fs::read_to_string("data/day3/input.txt").unwrap();
        assert_eq!(day3_part1(&input), 0);
    }

    #[test]
    fn test_day3_part2_example() {
        let input = std::fs::read_to_string("data/day3/example.txt").unwrap();
        assert_eq!(day3_part2(&input), 0);
    }

    #[test]
    fn test_day3_part2_input() {
        let input = std::fs::read_to_string("data/day3/input.txt").unwrap();
        assert_eq!(day3_part2(&input), 0);
    }
}