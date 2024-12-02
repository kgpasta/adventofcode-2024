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
