use reqwest;
use std::env;
use std::fs::{self, File};
use std::path::Path;

pub async fn create_puzzle_files(day: u32) -> Result<(), Box<dyn std::error::Error>> {
    let data_dir = format!("data/day{}", day);
    fs::create_dir_all(&data_dir)?;

    let session = env::var("AOC_SESSION").expect("AOC_SESSION environment variable not set");
    let client = reqwest::Client::new();

    let input_url = format!("https://adventofcode.com/2024/day/{}/input", day);
    let input = client
        .get(&input_url)
        .header("Cookie", format!("session={}", session))
        .send()
        .await?
        .text()
        .await?;

    let input_path = format!("{}/day{}.txt", data_dir, day);
    fs::write(&input_path, input)?;

    let example_path = format!("{}/sample.txt", data_dir);
    if !Path::new(&example_path).exists() {
        File::create(&example_path)?;
    }

    let solution_content = format!(
        r#"pub fn day{}_part1(input: &str) -> i32 {{
    0 // TODO: Implement solution
}}

pub fn day{}_part2(input: &str) -> i32 {{
    0 // TODO: Implement solution
}}

#[cfg(test)]
mod tests {{
    use super::*;

    #[test]
    fn test_day{}_part1_example() {{
        let input = std::fs::read_to_string("data/day{}/example.txt").unwrap();
        assert_eq!(day{}_part1(&input), 0);
    }}

    #[test]
    fn test_day{}_part1_input() {{
        let input = std::fs::read_to_string("data/day{}/input.txt").unwrap();
        assert_eq!(day{}_part1(&input), 0);
    }}

    #[test]
    fn test_day{}_part2_example() {{
        let input = std::fs::read_to_string("data/day{}/example.txt").unwrap();
        assert_eq!(day{}_part2(&input), 0);
    }}

    #[test]
    fn test_day{}_part2_input() {{
        let input = std::fs::read_to_string("data/day{}/input.txt").unwrap();
        assert_eq!(day{}_part2(&input), 0);
    }}
}}"#,
        day, day, day, day, day, day, day, day, day, day, day, day, day, day
    );

    let solution_path = format!("src/day{}.rs", day);
    fs::write(&solution_path, solution_content)?;

    println!("Created files for day {}:", day);
    println!("- {}", input_path);
    println!("- {}", example_path);
    println!("- {}", solution_path);

    Ok(())
}
