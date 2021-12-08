use counter::Counter;
use general::read_data_lines;
use std::str::FromStr;
use structopt::StructOpt;

// https://adventofcode.com/2021/day/7
const PUZZLE_NAME: &str = "Advent of Code: Day 7 -- Version:";
const PUZZLE_ABOUT: &str = "The Treachery of Whales";

fn get_solution2(data: &[u32]) -> (usize, u32) {
    let counts = data.iter().collect::<Counter<_>>();

    let min = match counts.keys().min() {
        Some(&n) => *n,
        None => panic!("min() failure"),
    };
    let max = match counts.keys().max() {
        Some(&n) => *n,
        None => panic!("max() failure"),
    };

    let mut best_cost = usize::MAX;
    let mut best_pos = min;
    for pos in min..=max {
        let left_cost = (min..pos)
            .into_iter()
            .map(|i| counts[&i] * ((pos - i) * (pos - i + 1) / 2) as usize)
            .sum::<usize>();
        let right_cost = (pos..=max)
            .into_iter()
            .map(|i| counts[&i] * ((i - pos) * (i - pos + 1) / 2) as usize)
            .sum::<usize>();

        match left_cost + right_cost {
            n if n < best_cost => {
                best_cost = n;
                best_pos = pos
            }
            _ => break,
        }
    }

    (best_cost, best_pos)
}

fn get_solution1(data: &[u32]) -> (usize, u32) {
    let counts = data.iter().collect::<Counter<_>>();

    let mut left = match counts.keys().min() {
        Some(&n) => *n,
        None => panic!("min() failure"),
    };
    let mut right = match counts.keys().max() {
        Some(&n) => *n,
        None => panic!("max() failure"),
    };

    let mut left_count = counts[&left];
    let mut right_count = counts[&right];
    let mut cost = 0;
    while left != right {
        match left_count < right_count {
            true => {
                cost += left_count;
                left += 1;
                left_count += counts[&left];
            }
            false => {
                cost += right_count;
                right -= 1;
                right_count += counts[&right];
            }
        }
    }
    (cost, left)
}

fn get_data<T>(data: &str) -> Result<Vec<T>, Box<dyn std::error::Error>>
where
    T: FromStr,
    <T as FromStr>::Err: std::error::Error,
{
    Ok(data
        .split(',')
        .map(|s| s.trim().parse::<T>().unwrap())
        .collect::<Vec<T>>())
}

#[allow(unused_variables)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[derive(StructOpt)]
    #[structopt(name = PUZZLE_NAME, about = PUZZLE_ABOUT)]
    struct Cli {
        #[structopt(
            short,
            long,
            parse(from_os_str),
            help = "file|stdin -- line containing comma separated positions"
        )]
        input: Option<std::path::PathBuf>,
    }
    let args = Cli::from_args();

    // ==============================================================

    let data = read_data_lines::<String>(args.input)?;
    let data = get_data::<u32>(&data[0])?;

    let (fuel_consumption, position) = get_solution1(&data);
    //println!("Position = {}", position);
    println!("Answer Part 1 = {}", fuel_consumption);

    let (fuel_consumption, position) = get_solution2(&data);
    //println!("Position = {}", position);
    println!("Answer Part 2 = {}", fuel_consumption);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_filedata(filename: &str) -> Vec<u32> {
        let file = Some(std::path::PathBuf::from(filename));
        let data = read_data_lines::<String>(file).unwrap();
        get_data::<u32>(&data[0]).unwrap()
    }

    #[test]
    fn part1_example() {
        let data = get_filedata("input-example");
        let (fuel_consumption, position) = get_solution1(&data);
        assert_eq!(fuel_consumption, 37);
        assert_eq!(position, 2);
    }

    #[test]
    fn part1_actual() {
        let data = get_filedata("input-actual");
        let (fuel_consumption, position) = get_solution1(&data);
        assert_eq!(fuel_consumption, 349769);
        assert_eq!(position, 331);
    }

    #[test]
    fn part2_example() {
        let data = get_filedata("input-example");
        let (fuel_consumption, position) = get_solution2(&data);
        assert_eq!(fuel_consumption, 168);
        assert_eq!(position, 5);
    }

    #[test]
    fn part2_actual() {
        let data = get_filedata("input-actual");
        let (fuel_consumption, position) = get_solution2(&data);
        assert_eq!(fuel_consumption, 99540554);
        assert_eq!(position, 479);
    }
}
