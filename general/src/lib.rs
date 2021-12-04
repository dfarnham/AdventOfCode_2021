use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use std::str::FromStr;

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

// Reads the lines of a file and returns them as a Vec of the supplied type
pub fn read_data_lines<T>(filename: Option<PathBuf>) -> Result<Vec<T>, Box<dyn std::error::Error>>
where
    T: FromStr,
    <T as FromStr>::Err: 'static,
    <T as FromStr>::Err: std::error::Error,
{
    let mut values = vec![];
    match filename {
        Some(file) if file.as_os_str() != "-" => {
            for line in read_lines(file)? {
                values.push(line?.trim().parse::<T>()?);
            }
            Ok(values)
        }
        _ => {
            // STDIN
            for line in io::BufReader::new(io::stdin()).lines() {
                values.push(line?.trim().parse::<T>()?);
            }
            Ok(values)
        }
    }
}
