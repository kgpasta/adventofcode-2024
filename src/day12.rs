use std::{
    collections::{HashMap, HashSet},
    vec,
};

use crate::common::{create_grid, is_on_map, read_all_lines};

pub fn day12_part1(file_name: &str) -> i32 {
    let text = read_all_lines(file_name).unwrap();
    let grid = create_grid(text);

    let mut coord_to_region: HashMap<(usize, usize), usize> = HashMap::new();
    let mut regions: Vec<(HashSet<(i32, i32)>, HashSet<((i32, i32), (i32, i32))>)> = vec![];

    let mut region_counter = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if coord_to_region.contains_key(&(i, j)) {
                continue;
            }

            let mut visited: HashSet<(i32, i32)> = HashSet::new();
            let mut sides_visited: HashSet<((i32, i32), (i32, i32))> = HashSet::new();
            let coord = (i as i32, j as i32);
            visited.insert(coord);
            let region_data =
                calculate_region(&grid, coord, grid[i][j], &mut visited, &mut sides_visited);

            let region_clone = region_data.clone();
            regions.push((region_data, sides_visited));
            for coord in region_clone {
                coord_to_region.insert((coord.0 as usize, coord.1 as usize), region_counter);
            }
            region_counter += 1;
        }
    }

    return regions.iter().map(|region| get_price(region)).sum();
}

fn get_price(region: &(HashSet<(i32, i32)>, HashSet<((i32, i32), (i32, i32))>)) -> i32 {
    let area = region.0.len() as i32;
    let perimeter = region.1.len() as i32;
    return area * perimeter;
}

fn get_bulk_price(region: &(HashSet<(i32, i32)>, HashSet<((i32, i32), (i32, i32))>)) -> i32 {
    let area = region.0.len() as i32;
    let mut top_sides: Vec<HashSet<(i32, i32)>> = vec![];
    let mut bottom_sides: Vec<HashSet<(i32, i32)>> = vec![];
    let mut left_sides: Vec<HashSet<(i32, i32)>> = vec![];
    let mut right_sides: Vec<HashSet<(i32, i32)>> = vec![];

    println!("{:?}", region.1);

    for (coord, direction) in region.1.clone() {
        if direction.0 == -1 {
            let top_neighbor = (coord.0, coord.1 + 1);
            let bottom_neighbor: (i32, i32) = (coord.0, coord.1 - 1);

            update_sides(&mut top_sides, coord, top_neighbor, bottom_neighbor);
        } else if direction.0 == 1 {
            let top_neighbor = (coord.0, coord.1 + 1);
            let bottom_neighbor: (i32, i32) = (coord.0, coord.1 - 1);

            update_sides(&mut bottom_sides, coord, top_neighbor, bottom_neighbor);
        } else if direction.1 == -1 {
            let left_neighbor = (coord.0 + 1, coord.1);
            let right_neighbor = (coord.0 - 1, coord.1);

            update_sides(&mut left_sides, coord, left_neighbor, right_neighbor);
        } else {
            let left_neighbor = (coord.0 + 1, coord.1);
            let right_neighbor = (coord.0 - 1, coord.1);
            update_sides(&mut right_sides, coord, left_neighbor, right_neighbor);
        }
    }

    print_map(region, &top_sides, &bottom_sides, &left_sides, &right_sides);
    println!("area: {:?}", area);
    println!(
        "sides:{:?}, {:?}, {:?}, {:?}",
        top_sides.len(),
        bottom_sides.len(),
        left_sides.len(),
        right_sides.len()
    );
    let price = area
        * (top_sides.len() as i32
            + bottom_sides.len() as i32
            + left_sides.len() as i32
            + right_sides.len() as i32);
    println!("price: {:?}", price);
    return price;
}

fn update_sides(
    sides: &mut Vec<HashSet<(i32, i32)>>,
    coord: (i32, i32),
    neighbor_one: (i32, i32),
    neighbor_two: (i32, i32),
) {
    let existing_indices: Vec<usize> = sides
        .iter()
        .enumerate()
        .filter(|(_, vector)| vector.contains(&neighbor_one) || vector.contains(&neighbor_two))
        .map(|(idx, _)| idx)
        .collect();

    let mut merged_set = HashSet::new();
    for idx in existing_indices.iter().rev() {
        merged_set.extend(sides.remove(*idx));
    }
    merged_set.insert(coord);

    if merged_set.len() == 0 {
        let mut new_set = HashSet::new();
        new_set.insert(coord);
        sides.push(new_set);
    } else {
        sides.push(merged_set);
    }
}

fn print_map(
    region: &(HashSet<(i32, i32)>, HashSet<((i32, i32), (i32, i32))>),
    top: &Vec<HashSet<(i32, i32)>>,
    bottom: &Vec<HashSet<(i32, i32)>>,
    left: &Vec<HashSet<(i32, i32)>>,
    right: &Vec<HashSet<(i32, i32)>>,
) {
    let mut map: Vec<Vec<String>> = vec![vec!["...".to_string(); 100]; 100];
    for (x, y) in region.0.iter() {
        map[(*x + 1) as usize][(*y + 1) as usize] = "###".to_string();
    }

    let mut count = 0;
    for side in top.iter() {
        for (x, y) in side.iter() {
            let value = format!("T{:?}", count);
            if map[(*x + 1) as usize][(*y + 1) as usize] == "..." {
                map[(*x + 1) as usize][(*y + 1) as usize] = value;
            } else {
                map[(*x + 1) as usize][(*y + 1) as usize] += value.as_str();
            }
        }
        count += 1;
    }

    count = 0;
    for side in bottom.iter() {
        for (x, y) in side.iter() {
            let value = format!("B{:?}", count);
            if map[(*x + 1) as usize][(*y + 1) as usize] == "..." {
                map[(*x + 1) as usize][(*y + 1) as usize] = value;
            } else {
                map[(*x + 1) as usize][(*y + 1) as usize] += value.as_str();
            }
        }
        count += 1;
    }

    count = 0;
    for side in left.iter() {
        for (x, y) in side.iter() {
            let value = format!("L{:?}", count);
            if map[(*x + 1) as usize][(*y + 1) as usize] == "..." {
                map[(*x + 1) as usize][(*y + 1) as usize] = value;
            } else {
                map[(*x + 1) as usize][(*y + 1) as usize] += value.as_str();
            }
        }
        count += 1;
    }

    count = 0;
    for side in right.iter() {
        for (x, y) in side.iter() {
            let value = format!("R{:?}", count);
            if map[(*x + 1) as usize][(*y + 1) as usize] == "..." {
                map[(*x + 1) as usize][(*y + 1) as usize] = value;
            } else {
                map[(*x + 1) as usize][(*y + 1) as usize] += value.as_str();
            }
        }
        count += 1;
    }

    for row in map {
        println!("{:?}", row);
    }
}

fn calculate_region(
    grid: &Vec<Vec<char>>,
    coord: (i32, i32),
    plant: char,
    visited: &mut HashSet<(i32, i32)>,
    sides_visited: &mut HashSet<((i32, i32), (i32, i32))>,
) -> HashSet<(i32, i32)> {
    let neighbors: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (-1, 0), (0, -1)];
    let size = (grid.len() - 1, grid[0].len() - 1);

    let mut result = HashSet::new();
    result.insert(coord);

    for neighbor in neighbors {
        let new_coord = (coord.0 as i32 + neighbor.0, coord.1 as i32 + neighbor.1);
        if !is_on_map(new_coord, size) {
            sides_visited.insert((new_coord, neighbor));
            continue;
        }

        let new_plant = grid[new_coord.0 as usize][new_coord.1 as usize];
        if plant != new_plant {
            sides_visited.insert((new_coord, neighbor));
        } else if !visited.contains(&new_coord) {
            visited.insert(new_coord);

            let sub_result = calculate_region(grid, new_coord, plant, visited, sides_visited);
            for res in sub_result {
                result.insert(res);
            }
        }
    }

    return result;
}

pub fn day12_part2(file_name: &str) -> i32 {
    let text = read_all_lines(file_name).unwrap();
    let grid = create_grid(text);

    let mut coord_to_region: HashMap<(usize, usize), usize> = HashMap::new();
    let mut regions: Vec<(HashSet<(i32, i32)>, HashSet<((i32, i32), (i32, i32))>)> = vec![];

    let mut region_counter = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if coord_to_region.contains_key(&(i, j)) {
                continue;
            }

            let mut visited: HashSet<(i32, i32)> = HashSet::new();
            let mut sides_visited: HashSet<((i32, i32), (i32, i32))> = HashSet::new();
            let coord = (i as i32, j as i32);
            visited.insert(coord);
            let region_data =
                calculate_region(&grid, coord, grid[i][j], &mut visited, &mut sides_visited);

            let region_clone = region_data.clone();
            regions.push((region_data, sides_visited));
            for coord in region_clone {
                coord_to_region.insert((coord.0 as usize, coord.1 as usize), region_counter);
            }
            region_counter += 1;
        }
    }

    return regions.iter().map(|region| get_bulk_price(region)).sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day12_part1_example() {
        let result = day12_part1("data/day12/sample_2.txt");
        assert_eq!(result, 1930);
    }

    #[test]
    fn test_day12_part1_input() {
        let result = day12_part1("data/day12/day12.txt");
        assert_eq!(result, 1522850);
    }

    #[test]
    fn test_day12_part2_example() {
        let result = day12_part2("data/day12/sample_2.txt");
        assert_eq!(result, 1206);
    }

    #[test]
    fn test_day12_part2_input() {
        let result = day12_part2("data/day12/day12.txt");
        assert_eq!(result, 0);
    }
}
