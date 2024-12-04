use std::collections::HashSet;

use crate::common::read_all_lines;

pub fn day4_part1(file_name: &str) -> i32 {
    let grid = create_grid(read_all_lines(file_name).unwrap());
    let mut total_count = 0;
    let mut total_set: HashSet<Coord> = HashSet::new();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'X' {
                let (count, set) = try_all_directions(&grid, i, j);
                total_set.extend(set);
                total_count += count;
            }
        }
    }

    print_debug(&grid, &total_set);

    total_count
}

fn print_debug(grid: &Vec<Vec<char>>, set: &HashSet<Coord>) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if set.contains(&Coord {
                i: i as i32,
                j: j as i32,
            }) {
                print!("{}", grid[i][j]);
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Coord {
    i: i32,
    j: i32,
}

fn try_all_directions(grid: &Vec<Vec<char>>, i: usize, j: usize) -> (i32, HashSet<Coord>) {
    let directions: Vec<(i32, i32)> = vec![
        (0, 1),
        (0, -1),
        (-1, 0),
        (1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];
    let mut count = 0;
    let word = "XMAS";
    let mut total_set: HashSet<Coord> = HashSet::new();

    for (di, dj) in directions {
        let mut current_i = i as i32;
        let mut current_j = j as i32;
        let mut next_letter_index = 0;
        let mut set: HashSet<Coord> = HashSet::new();
        set.insert(Coord {
            i: current_i,
            j: current_j,
        });

        while !is_out_of_bounds(grid, current_i, current_j) {
            if grid[current_i as usize][current_j as usize]
                != word.chars().nth(next_letter_index).unwrap_or('.')
            {
                break;
            }

            next_letter_index += 1;
            current_i += di;
            current_j += dj;

            if next_letter_index == word.len() {
                count += 1;
                total_set.extend(set.clone());
            } else {
                set.insert(Coord {
                    i: current_i,
                    j: current_j,
                });
            }
        }
    }

    (count, total_set)
}

fn is_out_of_bounds(grid: &Vec<Vec<char>>, i: i32, j: i32) -> bool {
    i < 0 || i >= grid.len() as i32 || j < 0 || j >= grid[i as usize].len() as i32
}

fn create_grid(lines: Vec<String>) -> Vec<Vec<char>> {
    let mut grid = vec![];
    for line in lines {
        let mut row = vec![];
        for c in line.chars() {
            row.push(c);
        }
        grid.push(row);
    }
    grid
}

pub fn day4_part2(file_name: &str) -> i32 {
    let grid = create_grid(read_all_lines(file_name).unwrap());
    let mut total_count = 0;
    let mut total_set: HashSet<Coord> = HashSet::new();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'A' {
                let (count, set) = try_diagonals(&grid, i, j);
                total_set.extend(set);
                total_count += count;
            }
        }
    }

    print_debug(&grid, &total_set);

    total_count
}

fn try_diagonals(grid: &Vec<Vec<char>>, i: usize, j: usize) -> (i32, HashSet<Coord>) {
    let mut count = 0;
    let mut total_set: HashSet<Coord> = HashSet::new();

    let current_i = i as i32;
    let current_j = j as i32;

    if try_diagonal(&grid, vec![(1, 1), (-1, -1)], current_i, current_j) {
        if try_diagonal(grid, vec![(-1, 1), (1, -1)], current_i, current_j) {
            count += 1;
            total_set.insert(Coord {
                i: current_i,
                j: current_j,
            });
            total_set.insert(Coord {
                i: current_i + 1,
                j: current_j + 1,
            });
            total_set.insert(Coord {
                i: current_i - 1,
                j: current_j - 1,
            });
            total_set.insert(Coord {
                i: current_i + 1,
                j: current_j - 1,
            });
            total_set.insert(Coord {
                i: current_i - 1,
                j: current_j + 1,
            });
        }
    }

    (count, total_set)
}

fn try_diagonal(
    grid: &Vec<Vec<char>>,
    diagonal: Vec<(i32, i32)>,
    current_i: i32,
    current_j: i32,
) -> bool {
    let mut word: HashSet<char> = "MS".chars().collect();
    for (di, dj) in diagonal {
        let x = current_i + di;
        let y = current_j + dj;

        if is_out_of_bounds(grid, x, y) {
            return false;
        }

        word.remove(&grid[x as usize][y as usize]);

        if word.is_empty() {
            return true;
        }
    }

    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day4_part1_example() {
        let result = day4_part1("data/day4/sample.txt");
        assert_eq!(result, 18);
    }

    #[test]
    fn test_day4_part1_input() {
        let result = day4_part1("data/day4/day4.txt");
        assert_eq!(result, 2514);
    }

    #[test]
    fn test_day4_part2_example() {
        let result = day4_part2("data/day4/sample.txt");
        assert_eq!(result, 9);
    }

    #[test]
    fn test_day4_part2_input() {
        let result = day4_part2("data/day4/day4.txt");
        assert_eq!(result, 1888);
    }
}
