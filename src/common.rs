use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn process_file_by_line(
    filename: &str,
) -> io::Result<impl Iterator<Item = io::Result<String>>> {
    let path = Path::new(filename);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    return Ok(reader.lines());
}

pub fn read_all_lines(filename: &str) -> io::Result<Vec<String>> {
    let path = Path::new(filename);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    return Ok(reader.lines().map(|x| x.unwrap()).collect());
}

pub fn is_on_map((x, y): (i32, i32), (x_max, y_max): (usize, usize)) -> bool {
    return x >= 0 && x <= x_max as i32 && y >= 0 && y <= y_max as i32;
}
