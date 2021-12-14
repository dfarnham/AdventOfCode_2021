use general::read_data_lines;
use rayon::prelude::*;
use structopt::StructOpt;

const PUZZLE_NAME: &str = "Advent of Code: Day 3 -- Version:";
const PUZZLE_ABOUT: &str = "Binary Diagnostic: https://adventofcode.com/2021/day/3";

// how many bits does the largest value in the dataset occupy
fn nbits(data: &[u32]) -> usize {
    ((*data.iter().max().expect("max() failure") as f32).log2()).round() as usize
}

fn get_gamma_epsilon(data: &[u32]) -> (u32, u32) {
    let mut gamma = 0;
    let mut epsilon = 0;

    let nbits = nbits(data);
    let masks = (0..nbits).into_iter().map(|i| 1 << i).collect::<Vec<u32>>();
    for mask in masks.iter() {
        let count = data.par_iter().filter(|n| (*n & mask) == *mask).count();
        // are there more bits "on" than "off" in this position?
        match 2 * count >= data.len() {
            true => gamma |= mask,
            false => epsilon |= mask,
        }
    }
    (gamma, epsilon)
}

fn mask_data(data: &[u32], mask: u32) -> (Vec<u32>, Vec<u32>) {
    assert!(mask != 0, "mask == 0, data = {:?}", data);
    let mut masked = vec![];
    let mut unmasked = vec![];
    for n in data.iter() {
        match (n & mask) == mask {
            true => masked.push(*n),
            false => unmasked.push(*n),
        }
    }
    (masked, unmasked)
}

fn get_co2(data: &[u32], mask: u32) -> u32 {
    match data.len() == 1 {
        true => data[0],
        false => {
            let (masked, unmasked) = mask_data(data, mask);
            match unmasked.len() <= masked.len() {
                true => get_co2(&unmasked, mask >> 1),
                false => get_co2(&masked, mask >> 1),
            }
        }
    }
}

fn get_oxy(data: &[u32], mask: u32) -> u32 {
    match data.len() == 1 {
        true => data[0],
        false => {
            let (masked, unmasked) = mask_data(data, mask);
            match masked.len() >= unmasked.len() {
                true => get_oxy(&masked, mask >> 1),
                false => get_oxy(&unmasked, mask >> 1),
            }
        }
    }
}

fn get_oxy_co2(data: &[u32]) -> (u32, u32) {
    // how many bits does the largest value in the dataset occupy
    let nbits = nbits(data);
    let mask = 1 << (nbits - 1);
    (get_oxy(data, mask), get_co2(data, mask))
}

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
    let data = read_data_lines::<String>(args.input)?
        .iter()
        .map(|s| u32::from_str_radix(s, 2).unwrap())
        .collect::<Vec<u32>>();

    let (gamma, epsilon) = get_gamma_epsilon(&data);
    //println!("gamma = {}, epsilon = {}", gamma, epsilon);
    println!("Answer Part 1 = {}", gamma * epsilon);

    let (oxy, co2) = get_oxy_co2(&data);
    //println!("oxy = {}, co2 = {}", oxy, co2);
    println!("Answer Part 2 = {}", oxy * co2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_data(filename: &str) -> Vec<u32> {
        let file = Some(std::path::PathBuf::from(filename));
        let data = read_data_lines::<String>(file)
            .unwrap()
            .iter()
            .map(|s| u32::from_str_radix(s, 2).unwrap())
            .collect::<Vec<u32>>();
        data
    }

    #[test]
    fn part1_example() {
        let (gamma, epsilon) = get_gamma_epsilon(&get_data("input-example"));
        assert_eq!(gamma, 22);
        assert_eq!(epsilon, 9);
        assert_eq!(gamma * epsilon, 198);
    }

    #[test]
    fn part1_actual() {
        let (gamma, epsilon) = get_gamma_epsilon(&get_data("input-actual"));
        assert_eq!(gamma * epsilon, 1307354);
    }

    #[test]
    fn part2_example() {
        let (oxy, co2) = get_oxy_co2(&get_data("input-example"));
        assert_eq!(oxy, 23);
        assert_eq!(co2, 10);
        assert_eq!(oxy * co2, 230);
    }

    #[test]
    fn part2_actual() {
        let (oxy, co2) = get_oxy_co2(&get_data("input-actual"));
        assert_eq!(oxy * co2, 482500);
    }
}
