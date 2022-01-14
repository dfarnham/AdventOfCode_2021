use counter::Counter;
use general::{read_data_lines, split_on};
use structopt::StructOpt;

const PUZZLE_NAME: &str = "Advent of Code: Day 7 -- Version:";
const PUZZLE_ABOUT: &str = "The Treachery of Whales: https://adventofcode.com/2021/day/7";

fn get_solution2(data: &[u32]) -> (usize, u32) {
    let counts = data.iter().collect::<Counter<_>>();

    let min = **counts.keys().min().expect("min() failure");
    let max = **counts.keys().max().expect("max() failure");

    let mut best: Option<(_, _)> = None;
    let sum_to_n = |n| (n * (n + 1) / 2) as usize;
    for pos in min..=max {
        let left_cost = (min..pos)
            .into_iter()
            .map(|i| counts[&i] * sum_to_n(pos - i))
            .sum::<usize>();
        let right_cost = (pos..=max)
            .into_iter()
            .map(|i| counts[&i] * sum_to_n(i - pos))
            .sum::<usize>();
        match left_cost + right_cost {
            n if best.is_none() || n < best.unwrap().0 => best = Some((n, pos)),
            _ => break,
        }
    }
    best.expect("no solution chosen")
}

fn get_solution1(data: &[u32]) -> (usize, u32) {
    let counts = data.iter().collect::<Counter<_>>();

    let mut left_ptr = **counts.keys().min().expect("min() failure");
    let mut right_ptr = **counts.keys().max().expect("max() failure");

    let mut left_mass = counts[&left_ptr];
    let mut rigt_mass = counts[&right_ptr];
    let mut cost = 0;
    while left_ptr != right_ptr {
        match left_mass < rigt_mass {
            true => {
                cost += left_mass;
                left_ptr += 1;
                left_mass += counts[&left_ptr];
            }
            false => {
                cost += rigt_mass;
                right_ptr -= 1;
                rigt_mass += counts[&right_ptr];
            }
        }
    }
    //println!("left_mass = {}, rigt_mass = {}, cost = {}", left_mass, rigt_mass, cost);
    (cost, left_ptr)
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
            help = "file|stdin -- puzzle input"
        )]
        input: Option<std::path::PathBuf>,
    }
    let args = Cli::from_args();

    // ==============================================================

    let data = read_data_lines::<String>(args.input)?;
    let data = split_on::<u32>(&data[0], ',')?;

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

    fn datapoints(filename: &str) -> Vec<u32> {
        let file = Some(std::path::PathBuf::from(filename));
        let data = read_data_lines::<String>(file).unwrap();
        let line = &data[0];
        split_on::<u32>(line, ',').unwrap()
    }

    #[test]
    fn part1_example() {
        let data = datapoints("input-example");
        let (fuel_consumption, position) = get_solution1(&data);
        assert_eq!(fuel_consumption, 37);
        assert_eq!(position, 2);
    }

    #[test]
    fn part1_actual() {
        let data = datapoints("input-actual");
        let (fuel_consumption, position) = get_solution1(&data);
        assert_eq!(fuel_consumption, 349769);
        assert_eq!(position, 331);
    }

    #[test]
    fn part2_example() {
        let data = datapoints("input-example");
        let (fuel_consumption, position) = get_solution2(&data);
        assert_eq!(fuel_consumption, 168);
        assert_eq!(position, 5);
    }

    #[test]
    fn part2_actual() {
        let data = datapoints("input-actual");
        let (fuel_consumption, position) = get_solution2(&data);
        assert_eq!(fuel_consumption, 99540554);
        assert_eq!(position, 479);
    }
}
