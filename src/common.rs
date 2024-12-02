use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn process_file_by_line<F>(filename: &str, mut line_processor: F) -> io::Result<()>
where
    F: FnMut(&str) -> (),
{
    let path = Path::new(filename);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        line_processor(&line);
    }

    Ok(())
}
