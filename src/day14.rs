use regex::Regex;

use crate::common::process_file_by_line;

#[derive(Copy, Clone, Debug)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

pub fn day14_part1(file_name: &str) -> i32 {
    let robots = parse_input(file_name);
    println!("{:?}", robots);
    let seconds = 100;
    let x_max: i32 = 101;
    let y_max: i32 = 103;

    let mut bottom_left_count = 0;
    let mut top_left_count = 0;
    let mut bottom_right_count = 0;
    let mut top_right_count = 0;

    let x_quad = x_max / 2;
    let y_quad = y_max / 2;
    let mut ending_locations = vec![];
    for robot in robots {
        let (endx, endy) = get_ending_location(robot, seconds, x_max, y_max);
        ending_locations.push((endx, endy));
        if endx < x_quad && endy < y_quad {
            println!("top_left: {:?}, {:?}", endx, endy);
            top_left_count += 1
        } else if endx > x_quad && endy < y_quad {
            println!("top_right: {:?}, {:?}", endx, endy);
            top_right_count += 1
        } else if endx < x_quad && endy > y_quad {
            println!("bottom_left: {:?}, {:?}", endx, endy);
            bottom_left_count += 1
        } else if endx > x_quad && endy > y_quad {
            println!("bottom_right: {:?}, {:?}", endx, endy);
            bottom_right_count += 1
        } else {
            println!("middle: {:?}, {:?}", endx, endy);
        }
    }

    print_ending_locations(ending_locations, x_max, y_max);

    println!(
        "{:?}, {:?}, {:?}, {:?}",
        top_left_count, top_right_count, bottom_left_count, bottom_right_count
    );

    bottom_left_count * bottom_right_count * top_left_count * top_right_count
}

fn print_ending_locations(ending_locations: Vec<(i32, i32)>, x_max: i32, y_max: i32) {
    for y in 0..y_max {
        for x in 0..x_max {
            let mut found = false;
            for (endx, endy) in &ending_locations {
                if x == *endx && y == *endy {
                    print!("#");
                    found = true;
                    break;
                }
            }
            if !found {
                print!(".");
            }
        }
        println!();
    }
}

fn get_ending_location(robot: Robot, seconds: i32, max_x: i32, max_y: i32) -> (i32, i32) {
    let mut x = robot.position.0 + robot.velocity.0 * seconds;
    let mut y = robot.position.1 + robot.velocity.1 * seconds;

    x = x % max_x;
    y = y % max_y;

    if x < 0 {
        x = max_x - x.abs();
    }
    if y < 0 {
        y = max_y - y.abs();
    }

    return (x, y);
}

fn parse_input(file_name: &str) -> Vec<Robot> {
    let mut robots = vec![];
    for res in process_file_by_line(file_name).unwrap() {
        let line = res.unwrap();
        let parts = parse_position_velocity(line.as_str()).unwrap();
        robots.push(Robot {
            position: parts.0,
            velocity: parts.1,
        });
    }

    robots
}

fn parse_position_velocity(input: &str) -> Option<((i32, i32), (i32, i32))> {
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    if let Some(caps) = re.captures(input) {
        let px = caps[1].parse::<i32>().unwrap();
        let py = caps[2].parse::<i32>().unwrap();
        let vx = caps[3].parse::<i32>().unwrap();
        let vy = caps[4].parse::<i32>().unwrap();
        return Some(((px, py), (vx, vy)));
    }
    None
}

pub fn day14_part2(file_name: &str) -> i32 {
    let robots = parse_input(file_name);
    println!("{:?}", robots);
    let x_max: i32 = 101;
    let y_max: i32 = 103;

    let x_quad = x_max / 2;
    let y_quad = y_max / 2;
    let mut min_danger_score: i64 = i64::MAX;
    let mut danger_seconds = 0;

    for seconds in 0..(x_max * y_max) {
        let mut bottom_left_count: i64 = 0;
        let mut top_left_count: i64 = 0;
        let mut bottom_right_count: i64 = 0;
        let mut top_right_count: i64 = 0;

        let mut ending_locations = vec![];
        for robot in &robots {
            let (endx, endy) = get_ending_location(*robot, seconds, x_max, y_max);
            ending_locations.push((endx, endy));

            if endx < x_quad && endy < y_quad {
                top_left_count += 1
            } else if endx > x_quad && endy < y_quad {
                top_right_count += 1
            } else if endx < x_quad && endy > y_quad {
                bottom_left_count += 1
            } else if endx > x_quad && endy > y_quad {
                bottom_right_count += 1
            } else {
            }
        }

        let danger_score = bottom_left_count
            .checked_mul(bottom_right_count)
            .unwrap_or(i64::MAX)
            .checked_mul(top_left_count)
            .unwrap_or(i64::MAX)
            .checked_mul(top_right_count)
            .unwrap_or(i64::MAX);
        if danger_score < min_danger_score {
            min_danger_score = danger_score;
            println!("danger score: {:?}", danger_score);
            println!("danger seconds: {:?}", seconds);
            danger_seconds = seconds;
            print_ending_locations(ending_locations, x_max, y_max);
            println!("");
        }
    }

    danger_seconds
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day14_part1_example() {
        let result = day14_part1("data/day14/sample.txt");
        assert_eq!(result, 12);
    }

    #[test]
    fn test_day14_part1_input() {
        let result = day14_part1("data/day14/day14.txt");
        assert_eq!(result, 231019008);
    }

    #[test]
    fn test_day14_part2_input() {
        let result = day14_part2("data/day14/day14.txt");
        assert_eq!(result, 8280);
    }
}
