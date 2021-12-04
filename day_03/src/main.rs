use general::read_data_lines;
use structopt::StructOpt;

// https://adventofcode.com/2021/day/3

fn get_gamma_epsilon(data: &[u32], num_bits: u8) -> (u32, u32) {
    let mut gamma = 0;
    let mut epsilon = 0;
    let threshold = data.len() as f32 / 2.0;

    for bits in 0..num_bits as u8 {
        let mask = 2_u32.pow(bits.into()) as u32;
        let count = data.iter().filter(|n| (*n & mask) == mask).count();
        match count as f32 >= threshold {
            true => gamma |= mask,
            false => epsilon |= mask,
        }
    }
    (gamma, epsilon)
}

fn get_oxy_co2(data: &[u32], num_bits: u8) -> (u32, u32) {
    let mut oxy = data.to_vec();
    let mut co2 = data.to_vec();

    for bits in (0..num_bits).rev() {
        let mask = 2_u32.pow(bits.into()) as u32;

        if oxy.len() > 1 {
            let set1 = oxy
                .iter()
                .filter(|n| (*n & mask) == mask)
                .copied()
                .collect::<Vec<u32>>();
            let set2 = oxy
                .iter()
                .filter(|n| (*n & mask) != mask)
                .copied()
                .collect::<Vec<u32>>();
            oxy = match set1.len() >= set2.len() {
                true => set1,
                false => set2,
            };
        }

        if co2.len() > 1 {
            let set1 = co2
                .iter()
                .filter(|n| (*n & mask) != mask)
                .copied()
                .collect::<Vec<u32>>();
            let set2 = co2
                .iter()
                .filter(|n| (*n & mask) == mask)
                .copied()
                .collect::<Vec<u32>>();
            co2 = match set1.len() <= set2.len() {
                true => set1,
                false => set2,
            };
        }
    }

    assert!(oxy.len() == 1, "oxy.len() > 1 = {:?}", oxy);
    assert!(co2.len() == 1, "co2.len() > 1 = {:?}", co2);
    (oxy[0], co2[0])
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[derive(StructOpt)]
    #[structopt(name = "Advent of Code: Day 3\nVersion:", about = "Binary Diagnostic")]
    struct Cli {
        #[structopt(
            short,
            long,
            parse(from_os_str),
            help = "file|stdin -- diagnostic binary, one per line"
        )]
        input: Option<std::path::PathBuf>,
    }
    let args = Cli::from_args();

    // ==============================================================

    let binary_data = read_data_lines::<String>(args.input)?
        .iter()
        .map(|s| u32::from_str_radix(s, 2).unwrap())
        .collect::<Vec<u32>>();
    let num_bits = ((*binary_data.iter().max().ok_or("max() failure")? as f32).log2()).round() as u8;

    let (gamma, epsilon) = get_gamma_epsilon(&binary_data, num_bits);
    println!("Answer Part 1 = {}", gamma * epsilon);

    let (oxy, co2) = get_oxy_co2(&binary_data, num_bits);
    println!("Answer Part 2 = {}", oxy * co2);
    Ok(())
}

#[test]
fn part1_example() {
    let file = Some(std::path::PathBuf::from("input-example"));
    let binary_data = read_data_lines::<String>(file)
        .unwrap()
        .iter()
        .map(|s| u32::from_str_radix(s, 2).unwrap())
        .collect::<Vec<u32>>();
    let num_bits = ((*binary_data.iter().max().unwrap() as f32).log2()).round() as u8;
    let (gamma, epsilon) = get_gamma_epsilon(&binary_data, num_bits);
    assert_eq!(gamma, 22);
    assert_eq!(epsilon, 9);
    assert_eq!(gamma * epsilon, 198);
}

#[test]
fn part1_actual() {
    let file = Some(std::path::PathBuf::from("input-actual"));
    let binary_data = read_data_lines::<String>(file)
        .unwrap()
        .iter()
        .map(|s| u32::from_str_radix(s, 2).unwrap())
        .collect::<Vec<u32>>();
    let num_bits = ((*binary_data.iter().max().unwrap() as f32).log2()).round() as u8;
    let (gamma, epsilon) = get_gamma_epsilon(&binary_data, num_bits);
    assert_eq!(gamma * epsilon, 1307354);
}

#[test]
fn part2_example() {
    let file = Some(std::path::PathBuf::from("input-example"));
    let binary_data = read_data_lines::<String>(file)
        .unwrap()
        .iter()
        .map(|s| u32::from_str_radix(s, 2).unwrap())
        .collect::<Vec<u32>>();
    let num_bits = ((*binary_data.iter().max().unwrap() as f32).log2()).round() as u8;
    let (oxy, co2) = get_oxy_co2(&binary_data, num_bits);
    assert_eq!(oxy, 23);
    assert_eq!(co2, 10);
    assert_eq!(oxy * co2, 230);
}

#[test]
fn part2_actual() {
    let file = Some(std::path::PathBuf::from("input-actual"));
    let binary_data = read_data_lines::<String>(file)
        .unwrap()
        .iter()
        .map(|s| u32::from_str_radix(s, 2).unwrap())
        .collect::<Vec<u32>>();
    let num_bits = ((*binary_data.iter().max().unwrap() as f32).log2()).round() as u8;
    let (oxy, co2) = get_oxy_co2(&binary_data, num_bits);
    assert_eq!(oxy * co2, 482500);
}
