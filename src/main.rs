use scaffold::create_puzzle_files;

pub mod common;
pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod scaffold;

fn main() {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        create_puzzle_files(14)
            .await
            .expect("Failed to create puzzle files");
    });
}
