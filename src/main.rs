use scaffold::create_puzzle_files;

pub mod common;
pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod scaffold;

fn main() {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        create_puzzle_files(4)
            .await
            .expect("Failed to create puzzle files");
    });
}
