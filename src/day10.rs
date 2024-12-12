use std::collections::HashSet;

use crate::common::{is_on_map, process_file_by_line};

pub fn day10_part1(file_name: &str) -> i32 {
    let map = read_map(file_name);

    let mut count = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 0 {
                count += find_trails((i as i32, j as i32), &mut HashSet::new(), &map);
            }
        }
    }

    count
}

fn read_map(file_name: &str) -> Vec<Vec<i32>> {
    return process_file_by_line(file_name)
        .unwrap()
        .map(|line| {
            let mut row = Vec::new();
            for c in line.unwrap().chars() {
                row.push(c.to_digit(10).unwrap() as i32);
            }
            row
        })
        .collect();
}

fn find_trails(coord: (i32, i32), visited: &mut HashSet<(i32, i32)>, map: &Vec<Vec<i32>>) -> i32 {
    let value = map[coord.0 as usize][coord.1 as usize];
    visited.insert(coord);

    if value == 9 {
        return 1;
    }

    let possible_trails = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut count = 0;
    for trail in possible_trails {
        let new_coord = (coord.0 + trail.0, coord.1 + trail.1);
        if is_on_map(new_coord, (map.len() - 1, map[0].len() - 1))
            && visited.get(&new_coord).is_none()
        {
            let new_value = map[new_coord.0 as usize][new_coord.1 as usize];
            if new_value == value + 1 {
                count += find_trails(new_coord, visited, map);
            }
        }
    }

    return count;
}

fn find_trails_rating(coord: (i32, i32), map: &Vec<Vec<i32>>) -> i32 {
    let value = map[coord.0 as usize][coord.1 as usize];

    if value == 9 {
        return 1;
    }

    let possible_trails = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut count = 0;
    for trail in possible_trails {
        let new_coord = (coord.0 + trail.0, coord.1 + trail.1);
        if is_on_map(new_coord, (map.len() - 1, map[0].len() - 1)) {
            let new_value = map[new_coord.0 as usize][new_coord.1 as usize];
            if new_value == value + 1 {
                count += find_trails_rating(new_coord, map);
            }
        }
    }

    return count;
}

pub fn day10_part2(file_name: &str) -> i32 {
    let map = read_map(file_name);

    let mut count = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 0 {
                count += find_trails_rating((i as i32, j as i32), &map);
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day10_part1_example() {
        let result = day10_part1("data/day10/sample.txt");
        assert_eq!(result, 36);
    }

    #[test]
    fn test_day10_part1_input() {
        let result = day10_part1("data/day10/day10.txt");
        assert_eq!(result, 517);
    }

    #[test]
    fn test_day10_part2_example() {
        let result = day10_part2("data/day10/sample.txt");
        assert_eq!(result, 81);
    }

    #[test]
    fn test_day10_part2_input() {
        let result = day10_part2("data/day10/day10.txt");
        assert_eq!(result, 1116);
    }
}
