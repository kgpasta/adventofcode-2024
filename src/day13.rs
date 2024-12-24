use crate::common::process_file_by_line;
use nalgebra::{Matrix2, Vector2};
use regex::Regex;

#[derive(Copy, Clone, Debug)]
struct Coord {
    x: i64,
    y: i64,
}

#[derive(Copy, Clone, Debug)]
struct Machine {
    a: Coord,
    b: Coord,
    prize: Coord,
}

const A_COST: i64 = 3;
const MAX_PUSHES: i64 = 100;

pub fn day13_part1(file_name: &str) -> i64 {
    let machines = parse_input(file_name);

    let mut total_cost = 0;
    for machine in machines {
        let result = solve_equations(machine);
        if result.is_some() {
            let (a, b) = result.unwrap();
            println!("A = {}, B = {}", a, b);
            println!("Cost: {}", a * A_COST + b);
            println!("Total cost: {}", total_cost);

            if a <= MAX_PUSHES && b <= MAX_PUSHES {
                total_cost += a * A_COST + b;
            }
        }
    }
    total_cost
}

fn solve_equations(machine: Machine) -> Option<(i64, i64)> {
    let a = Matrix2::new(
        machine.a.x as f64,
        machine.b.x as f64,
        machine.a.y as f64,
        machine.b.y as f64,
    );

    let b = Vector2::new(machine.prize.x as f64, machine.prize.y as f64);

    return match a.lu().solve(&b) {
        Some(solution) => {
            let a_int = solution[0].round() as i64;
            let b_int = solution[1].round() as i64;
            if (machine.a.x * a_int + machine.b.x * b_int == machine.prize.x)
                && (machine.a.y * a_int + machine.b.y * b_int == machine.prize.y)
            {
                Some((a_int, b_int))
            } else {
                None
            }
        }
        None => None,
    };
}

fn parse_input(file_name: &str) -> Vec<Machine> {
    let mut machines = vec![];

    let mut a = Coord { x: 0, y: 0 };
    let mut b = Coord { x: 0, y: 0 };
    let mut prize = Coord { x: 0, y: 0 };
    for res in process_file_by_line(file_name).unwrap() {
        let line = res.unwrap();
        if line.contains("Button A") {
            let parts = parse_numbers(line.as_str());
            a.x = parts[0];
            a.y = parts[1];
        } else if line.contains("Button B") {
            let parts = parse_numbers(line.as_str());
            b.x = parts[0];
            b.y = parts[1];
        } else if line.contains("Prize") {
            let parts = parse_prize(line.as_str());
            prize.x = parts[0];
            prize.y = parts[1];
        } else {
            machines.push(Machine { a, b, prize });
        }
    }

    machines.push(Machine { a, b, prize });

    machines
}

fn parse_numbers(input: &str) -> Vec<i64> {
    let re = Regex::new(r"[XY]\+(\d+)").unwrap();
    re.captures_iter(input)
        .map(|cap| cap[1].parse::<i64>().unwrap())
        .collect()
}

fn parse_prize(input: &str) -> Vec<i64> {
    let re = Regex::new(r"[XY]\=(\d+)").unwrap();
    re.captures_iter(input)
        .map(|cap| cap[1].parse::<i64>().unwrap())
        .collect()
}

pub fn day13_part2(file_name: &str) -> i64 {
    let machines = parse_input(file_name);

    let mut total_cost = 0;
    for mut machine in machines {
        machine.prize.x += 10000000000000;
        machine.prize.y += 10000000000000;
        let result = solve_equations(machine);
        if result.is_some() {
            let (a, b) = result.unwrap();
            println!("A = {}, B = {}", a, b);
            println!("Cost: {}", a * A_COST + b);
            println!("Total cost: {}", total_cost);

            total_cost += a * A_COST + b;
        }
    }
    total_cost
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day13_part1_example() {
        let result = day13_part1("data/day13/sample.txt");
        assert_eq!(result, 480);
    }

    #[test]
    fn test_day13_part1_input() {
        let result = day13_part1("data/day13/day13.txt");
        assert_eq!(result, 35997);
    }

    #[test]
    fn test_day13_part2_example() {
        let result = day13_part2("data/day13/sample.txt");
        assert_eq!(result, 875318608908);
    }

    #[test]
    fn test_day13_part2_input() {
        let result = day13_part2("data/day13/day13.txt");
        assert_eq!(result, 82510994362072);
    }
}
