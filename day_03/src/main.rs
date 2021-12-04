use general::read_data_lines;
use structopt::StructOpt;

// https://adventofcode.com/2021/day/3

fn get_gamma_epsilon(data: &[u32]) -> (u32, u32) {
    let mut gamma = 0;
    let mut epsilon = 0;

    // how many bits does the largest value in the dataset occupy
    let nbits = ((*data.iter().max().expect("max() failure") as f32).log2()).round() as usize;
    let masks = (0..nbits).into_iter().map(|i| 1 << i).collect::<Vec<u32>>();
    for bit in 0..nbits {
        let mask = masks[bit];
        let count = data.iter().filter(|n| (*n & mask) == mask).count();
        // are there more bits "on" than "off" in this position?
        match count >= data.len() - count {
            true => gamma |= mask,
            false => epsilon |= mask,
        }
    }
    (gamma, epsilon)
}

fn get_oxy_co2(data: &[u32]) -> (u32, u32) {
    let mut oxy = data.to_vec();
    let mut co2 = data.to_vec();

    // how many bits does the largest value in the dataset occupy
    let nbits = ((*data.iter().max().expect("max() failure") as f32).log2()).round() as usize;
    let masks = (0..nbits).into_iter().map(|i| 1 << i).collect::<Vec<u32>>();
    for bit in (0..nbits).rev() {
        let mask = masks[bit];

        if oxy.len() > 1 {
            let set1 = oxy
                .iter()
                .filter(|n| (*n & mask) == mask)
                .copied()
                .collect::<Vec<u32>>();
            oxy = match set1.len() >= oxy.len() - set1.len() {
                true => set1,
                false => oxy
                    .iter()
                    .filter(|n| (*n & mask) != mask)
                    .copied()
                    .collect::<Vec<u32>>(),
            };
        }

        if co2.len() > 1 {
            let set1 = co2
                .iter()
                .filter(|n| (*n & mask) != mask)
                .copied()
                .collect::<Vec<u32>>();
            co2 = match set1.len() <= co2.len() - set1.len() {
                true => set1,
                false => co2
                    .iter()
                    .filter(|n| (*n & mask) == mask)
                    .copied()
                    .collect::<Vec<u32>>(),
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

#[test]
fn part1_example() {
    let file = Some(std::path::PathBuf::from("input-example"));
    let data = read_data_lines::<String>(file)
        .unwrap()
        .iter()
        .map(|s| u32::from_str_radix(s, 2).unwrap())
        .collect::<Vec<u32>>();
    let (gamma, epsilon) = get_gamma_epsilon(&data);
    assert_eq!(gamma, 22);
    assert_eq!(epsilon, 9);
    assert_eq!(gamma * epsilon, 198);
}

#[test]
fn part1_actual() {
    let file = Some(std::path::PathBuf::from("input-actual"));
    let data = read_data_lines::<String>(file)
        .unwrap()
        .iter()
        .map(|s| u32::from_str_radix(s, 2).unwrap())
        .collect::<Vec<u32>>();
    let (gamma, epsilon) = get_gamma_epsilon(&data);
    assert_eq!(gamma * epsilon, 1307354);
}

#[test]
fn part2_example() {
    let file = Some(std::path::PathBuf::from("input-example"));
    let data = read_data_lines::<String>(file)
        .unwrap()
        .iter()
        .map(|s| u32::from_str_radix(s, 2).unwrap())
        .collect::<Vec<u32>>();
    let (oxy, co2) = get_oxy_co2(&data);
    assert_eq!(oxy, 23);
    assert_eq!(co2, 10);
    assert_eq!(oxy * co2, 230);
}

#[test]
fn part2_actual() {
    let file = Some(std::path::PathBuf::from("input-actual"));
    let data = read_data_lines::<String>(file)
        .unwrap()
        .iter()
        .map(|s| u32::from_str_radix(s, 2).unwrap())
        .collect::<Vec<u32>>();
    let (oxy, co2) = get_oxy_co2(&data);
    assert_eq!(oxy * co2, 482500);
}
