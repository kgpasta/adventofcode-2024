use std::collections::HashMap;
use std::collections::HashSet;

use crate::common::process_file_by_line;

pub fn day8_part1(file_name: &str) -> i32 {
    let (antennas, size) = create_antenna_sets(file_name);

    let mut valid_total_antinodes: HashSet<(i32, i32)> = HashSet::new();
    for (_, locations) in antennas.iter() {
        let location_list: Vec<_> = locations.iter().collect();
        for idx1 in 0..locations.len() {
            for idx2 in idx1 + 1..locations.len() {
                let (x1, y1) = location_list.get(idx1).unwrap();
                let (x2, y2) = location_list.get(idx2).unwrap();
                let (intx1, inty1) = (*x1 as i32, *y1 as i32);
                let (intx2, inty2) = (*x2 as i32, *y2 as i32);

                let distance_vector = (intx2 - intx1, inty2 - inty1);

                let antinode1 = (
                    intx1 + distance_vector.0 * -1,
                    inty1 + distance_vector.1 * -1,
                );
                let antinode2 = (intx2 + distance_vector.0, inty2 + distance_vector.1);

                if is_on_map(antinode1, size) {
                    valid_total_antinodes.insert(antinode1);
                }
                if is_on_map(antinode2, size) {
                    valid_total_antinodes.insert(antinode2);
                }
            }
        }
    }

    return valid_total_antinodes.len() as i32;
}

fn is_on_map((x, y): (i32, i32), (x_max, y_max): (usize, usize)) -> bool {
    return x >= 0 && x <= x_max as i32 && y >= 0 && y <= y_max as i32;
}

fn create_antenna_sets(
    file_name: &str,
) -> (HashMap<char, HashSet<(usize, usize)>>, (usize, usize)) {
    let mut antennas: HashMap<char, HashSet<(usize, usize)>> = HashMap::new();
    let mut max_x = 0;
    let mut max_y = 0;
    for (y, result) in process_file_by_line(file_name).unwrap().enumerate() {
        max_y = y;

        for (x, freq) in result.unwrap().chars().enumerate() {
            max_x = x;

            if freq == '.' {
                continue;
            }

            if antennas.contains_key(&freq) {
                antennas.get_mut(&freq).unwrap().insert((x, y));
            } else {
                let mut set = HashSet::new();
                set.insert((x, y));
                antennas.insert(freq, set);
            }
        }
    }

    (antennas, (max_x, max_y))
}

pub fn day8_part2(file_name: &str) -> i32 {
    0 // TODO: Implement solution
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day8_part1_example() {
        let result = day8_part1("data/day8/sample.txt");
        assert_eq!(result, 14);
    }

    #[test]
    fn test_day8_part1_input() {
        let result = day8_part1("data/day8/day8.txt");
        assert_eq!(result, 0);
    }

    #[test]
    fn test_day8_part2_example() {
        let result = day8_part2("data/day8/sample.txt");
        assert_eq!(result, 0);
    }

    #[test]
    fn test_day8_part2_input() {
        let result = day8_part2("data/day8/day8.txt");
        assert_eq!(result, 0);
    }
}
