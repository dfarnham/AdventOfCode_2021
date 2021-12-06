use counter::Counter;
use general::read_data_lines;
use structopt::StructOpt;

// https://adventofcode.com/2021/day/6
const PUZZLE_NAME: &str = "Advent of Code: Day 6 -- Version:";
const PUZZLE_ABOUT: &str = "Lanternfish";

fn cycle(data: &[u8], days: u32) -> usize {
    let counts = data.iter().collect::<Counter<_>>();
    let mut state = [
        counts[&0], counts[&1], counts[&2], counts[&3], counts[&4], counts[&5], counts[&6], counts[&7], counts[&8],
    ];
    for _ in 0..days {
        state = [
            state[1],
            state[2],
            state[3],
            state[4],
            state[5],
            state[6],
            state[7] + state[0],
            state[8],
            state[0],
        ];
    }
    state.iter().sum::<usize>()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[derive(StructOpt)]
    #[structopt(name = PUZZLE_NAME, about = PUZZLE_ABOUT)]
    struct Cli {
        #[structopt(
            short,
            long,
            parse(from_os_str),
            help = "file|stdin -- line containing comma separated ages in range [0,8]"
        )]
        input: Option<std::path::PathBuf>,
    }
    let args = Cli::from_args();

    // ==============================================================

    let data = read_data_lines::<String>(args.input)?;
    let data = data[0]
        .split(',')
        .map(|s| s.trim().parse::<u8>().unwrap())
        .collect::<Vec<u8>>();

    println!("Answer Part 1 = {}", cycle(&data, 80));
    println!("Answer Part 2 = {}", cycle(&data, 256));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_data(filename: &str) -> Vec<u8> {
        let file = Some(std::path::PathBuf::from(filename));
        let data = read_data_lines::<String>(file).unwrap();
        let data = data[0]
            .split(',')
            .map(|s| s.trim().parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        data
    }

    #[test]
    fn part1_example() {
        let data = get_data("input-example");
        assert_eq!(cycle(&data, 18), 26);
        assert_eq!(cycle(&data, 80), 5934);
    }

    #[test]
    fn part1_actual() {
        let data = get_data("input-actual");
        assert_eq!(cycle(&data, 80), 358214);
    }

    #[test]
    fn part2_example() {
        let data = get_data("input-example");
        assert_eq!(cycle(&data, 256), 26984457539);
    }

    #[test]
    fn part2_actual() {
        let data = get_data("input-actual");
        assert_eq!(cycle(&data, 256), 1622533344325);
    }
}
