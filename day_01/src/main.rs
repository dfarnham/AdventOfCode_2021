use general::read_data_lines;
use num::Num;
use structopt::StructOpt;

const PUZZLE_NAME: &str = "Advent of Code: Day 1 -- Version:";
const PUZZLE_ABOUT: &str = "Sonar Sweep: https://adventofcode.com/2021/day/1";

// Given an input array:
// Count the number of times the sum of measurements in a provided sliding window increases
fn count_window_increase<'a, T>(array: &'a [T], window: usize) -> usize
where
    T: Num + std::cmp::PartialOrd + std::iter::Sum<&'a T>,
{
    assert!(window > 0, "Window must be > 0");
    assert!(
        array.len() > window,
        "Array length: {} must be greater than the window size: {}",
        array.len(),
        window
    );

    (0..(array.len() - window))
        .into_iter()
        .filter(|&i| array[i..(i + window)].iter().sum::<T>() < array[(i + 1)..=(i + window)].iter().sum::<T>())
        .count()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[derive(StructOpt)]
    #[structopt(name = PUZZLE_NAME, about = PUZZLE_ABOUT)]
    struct Cli {
        #[structopt(short, long, parse(from_os_str), help = "file|stdin -- puzzle input")]
        input: Option<std::path::PathBuf>,
    }
    let args = Cli::from_args();

    // ==============================================================

    let measurements = read_data_lines::<u32>(args.input)?;
    println!("Answer Part 1 = {}", count_window_increase(&measurements, 1));
    println!("Answer Part 2 = {}", count_window_increase(&measurements, 3));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_data(filename: &str) -> Vec<u32> {
        let file = Some(std::path::PathBuf::from(filename));
        read_data_lines::<u32>(file).unwrap()
    }

    #[test]
    #[should_panic]
    fn empty_array() {
        let measurements = Vec::<i32>::new();
        let window = 1;
        count_window_increase(&measurements, window);
    }

    #[test]
    #[should_panic]
    fn array_too_small() {
        let measurements = vec![199];
        let window = 1;
        count_window_increase(&measurements, window);
    }

    #[test]
    #[should_panic]
    fn invalid_window() {
        let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let window = 0;
        count_window_increase(&measurements, window);
    }

    #[test]
    fn part1_example() {
        let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let window = 1;
        assert_eq!(count_window_increase(&measurements, window), 7);

        let measurements = get_data("input-example");
        assert_eq!(count_window_increase(&measurements, window), 7);
    }

    #[test]
    fn part2_example() {
        let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let window = 3;
        assert_eq!(count_window_increase(&measurements, window), 5);

        let measurements = get_data("input-example");
        assert_eq!(count_window_increase(&measurements, window), 5);
    }

    #[test]
    fn part1_actual() {
        let measurements = get_data("input-actual");
        let window = 1;
        assert_eq!(count_window_increase(&measurements, window), 1233);
    }

    #[test]
    fn part2_actual() {
        let measurements = get_data("input-actual");
        let window = 3;
        assert_eq!(count_window_increase(&measurements, window), 1275);
    }
}
