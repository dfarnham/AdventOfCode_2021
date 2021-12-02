use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

// https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
//
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// Read the lines of a filename into a Vec<i32>
// A filename of "-" or None is treated as stdin
pub fn read_int_lines(filename: Option<PathBuf>) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut values = vec![];
    match filename {
        Some(file) if file.as_os_str() != "-" => {
            for line in read_lines(file)? {
                values.push(line?.trim().parse::<i32>()?);
            }
            Ok(values)
        }
        _ => {
            // STDIN
            for line in io::BufReader::new(io::stdin()).lines() {
                values.push(line?.trim().parse::<i32>()?);
            }
            Ok(values)
        }
    }
}

// Read the lines of a filename into a Vec<String>
// A filename of "-" or None is treated as stdin
pub fn read_string_lines(filename: Option<PathBuf>) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut lines = vec![];
    match filename {
        Some(file) if file.as_os_str() != "-" => {
            for line in read_lines(file)? {
                lines.push(line?.trim().to_string());
            }
            Ok(lines)
        }
        _ => {
            // STDIN
            for line in io::BufReader::new(io::stdin()).lines() {
                lines.push(line?.trim().to_string());
            }
            Ok(lines)
        }
    }
}
