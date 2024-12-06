use std::collections::HashSet;

use crate::common::{process_file_by_line, read_all_lines};

pub fn day6_part1(file_name: &str) -> i32 {
    let mut map = construct_map(file_name);

    let mut guard_position = find_guard_position(&map);
    let mut position_set: HashSet<(i32, i32)> = HashSet::new();

    while guard_is_on_map(&map, guard_position) {
        let (i, j) = guard_position;
        let (newi, newj, new_orientation) = get_next_position(&map, guard_position);
        map[i as usize][j as usize] = 'X';
        position_set.insert((i, j));
        guard_position = (newi as i32, newj as i32);

        if guard_is_on_map(&map, guard_position) {
            map[newi as usize][newj as usize] = new_orientation;
        }
    }

    position_set.len() as i32
}

fn guard_is_on_map(map: &Vec<Vec<char>>, guard_position: (i32, i32)) -> bool {
    let (i, j) = guard_position;
    return i >= 0 && i < map.len() as i32 && j >= 0 && j < map[i as usize].len() as i32;
}

fn get_next_position(map: &Vec<Vec<char>>, guard_position: (i32, i32)) -> (i32, i32, char) {
    let (i, j) = guard_position;
    let mut current_orientation = map[i as usize][j as usize];
    let (mut nexti, mut nextj) = guard_position.clone();

    while nexti == i && nextj == j {
        (nexti, nextj) = match current_orientation {
            '^' => (i - 1, j),
            'v' => (i + 1, j),
            '<' => (i, j - 1),
            '>' => (i, j + 1),
            _ => panic!("Invalid guard position"),
        };

        if (nexti < 0 || nexti >= map.len() as i32)
            || (nextj < 0 || nextj >= map[nexti as usize].len() as i32)
        {
            return (nexti, nextj, current_orientation);
        }

        if map[nexti as usize][nextj as usize] == '#' {
            nexti = i;
            nextj = j;
            current_orientation = match current_orientation {
                '^' => '>',
                'v' => '<',
                '<' => '^',
                '>' => 'v',
                _ => panic!("Invalid guard position"),
            };
        }
    }

    return (nexti, nextj, current_orientation);
}

fn find_guard_position(map: &Vec<Vec<char>>) -> (i32, i32) {
    let mut guard_position: (i32, i32) = (-1, -1);
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == '^' || map[i][j] == 'v' || map[i][j] == '<' || map[i][j] == '>' {
                guard_position = (i as i32, j as i32);
                return guard_position;
            }
        }
    }

    return guard_position;
}

fn construct_map(file_name: &str) -> Vec<Vec<char>> {
    return process_file_by_line(file_name)
        .map(|line| line.map(|x| x.unwrap().chars().collect::<Vec<char>>()))
        .unwrap()
        .collect::<Vec<Vec<char>>>();
}

pub fn day6_part2(file_name: &str) -> i32 {
    0 // TODO: Implement solution
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day6_part1_example() {
        let result = day6_part1("data/day6/sample.txt");
        assert_eq!(result, 41);
    }

    #[test]
    fn test_day6_part1_input() {
        let result = day6_part1("data/day6/day6.txt");
        assert_eq!(result, 4903);
    }

    #[test]
    fn test_day6_part2_example() {
        let result = day6_part2("data/day6/sample.txt");
        assert_eq!(result, 0);
    }

    #[test]
    fn test_day6_part2_input() {
        let result = day6_part2("data/day6/day6.txt");
        assert_eq!(result, 0);
    }
}