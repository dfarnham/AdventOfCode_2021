use counter::Counter;
use general::read_data_lines;
use itertools::Itertools;
use structopt::StructOpt;

const PUZZLE_NAME: &str = "Advent of Code: Day 8 -- Version:";
const PUZZLE_ABOUT: &str = "Seven Segment Search: https://adventofcode.com/2021/day/8";

/*
  0:        1:        2:        3:        4:        5:        6:        7:        8:        9:
 aaaa      ....      aaaa      aaaa      ....      aaaa      aaaa      aaaa      aaaa      aaaa
b    c    .    c    .    c    .    c    b    c    b    .    b    .    .    c    b    c    b    c
b    c    .    c    .    c    .    c    b    c    b    .    b    .    .    c    b    c    b    c
 ....      ....      dddd      dddd      dddd      dddd      dddd      ....      dddd      dddd
e    f    .    f    e    .    .    f    .    f    .    f    e    f    .    f    e    f    .    f
e    f    .    f    e    .    .    f    .    f    .    f    e    f    .    f    e    f    .    f
 gggg      ....      gggg      gggg      ....      gggg      gggg      ....      gggg      gggg
*/

fn get_solution(dirty: &[String]) -> Vec<u8> {
    let mut digits = vec![u8::MAX; 14];

    // find 1, 4, 7, 8
    for (i, digit) in dirty.iter().enumerate() {
        match digit.len() {
            2 => digits[i] = 1,
            3 => digits[i] = 7,
            4 => digits[i] = 4,
            7 => digits[i] = 8,
            _ => (),
        }
    }

    // known: 1, 4, 7, 8
    //
    // "3" is a digit.len() == 5 that contains the "1" chars
    let s = &dirty[digits.iter().position(|&n| n == 1).unwrap()];
    for (i, digit) in dirty.iter().enumerate() {
        if digit.len() == 5 {
            let mut found = true;
            for c in s.chars() {
                if !digit.contains(c) {
                    found = false;
                }
            }
            if found {
                digits[i] = 3;
            }
        }
    }

    // known: 1, 3, 4, 7, 8
    //
    // "9" is a digit.len() == 6 that contains the "3" chars
    let s = &dirty[digits.iter().position(|&n| n == 3).unwrap()];
    for (i, digit) in dirty.iter().enumerate() {
        if digit.len() == 6 {
            let mut found = true;
            for c in s.chars() {
                if !digit.contains(c) {
                    found = false;
                }
            }
            if found {
                digits[i] = 9;
            }
        }
    }

    // known: 1, 3, 4, 7, 8, 9
    //
    // "0" is a digit.len() == 6 that contains the "1" chars and is not the "9"
    let s = &dirty[digits.iter().position(|&n| n == 1).unwrap()];
    for (i, digit) in dirty.iter().enumerate() {
        if digit.len() == 6 {
            let mut found = true;
            for c in s.chars() {
                if !digit.contains(c) {
                    found = false;
                }
            }
            if found && digits[i] != 9 {
                digits[i] = 0;
            }
        }
    }

    // known: 0, 1, 3, 4, 7, 8, 9
    //
    // "6" is a digit.len() == 6 that is not 0, 9
    for (i, digit) in dirty.iter().enumerate() {
        if digit.len() == 6 && digits[i] != 0 && digits[i] != 9 {
            digits[i] = 6;
        }
    }

    // known: 0, 1, 3, 4, 6, 7, 8, 9
    //
    // "5" is a digit.len() == 5 that is contained within a "6"
    let s = &dirty[digits.iter().position(|&n| n == 6).unwrap()];
    for (i, digit) in dirty.iter().enumerate() {
        if digit.len() == 5 {
            let mut found = true;
            for c in digit.chars() {
                if !s.contains(c) {
                    found = false;
                }
            }
            if found {
                digits[i] = 5;
            }
        }
    }

    // known: 0, 1, 3, 4, 5, 6, 7, 8, 9
    //
    // "2" is a digit.len() == 5 that is not 3, 5
    for (i, digit) in dirty.iter().enumerate() {
        if digit.len() == 5 && digits[i] != 3 && digits[i] != 5 {
            digits[i] = 2;
        }
    }

    digits[10..].to_vec()
}

fn get_segment_msg(data: &str) -> Vec<String> {
    data.split_whitespace()
        .filter(|s| *s != "|")
        .map(|s| s.trim().parse::<String>().unwrap())
        .map(|s| s.chars().sorted().collect::<String>())
        .collect::<Vec<String>>()
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
    let digits = data
        .iter()
        .map(|line| get_segment_msg(line))
        .collect::<Vec<Vec<String>>>();

    let mut digit_counts = Counter::<u8, usize>::new();
    let mut n = 0;
    for d in digits {
        let msg = get_solution(&d);
        n += 1000 * msg[0] as u32 + 100 * msg[1] as u32 + 10 * msg[2] as u32 + msg[3] as u32;
        digit_counts += msg;
    }

    println!(
        "Answer Part 1 = {}",
        digit_counts[&1] + digit_counts[&4] + digit_counts[&7] + digit_counts[&8]
    );

    println!("Answer Part 2 = {}", n);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn datapoints(filename: &str) -> Vec<Vec<String>> {
        let file = Some(std::path::PathBuf::from(filename));
        let data = read_data_lines::<String>(file).unwrap();
        data.iter()
            .map(|line| get_segment_msg(line))
            .collect::<Vec<Vec<String>>>()
    }

    #[test]
    fn part1_example() {
        let data = datapoints("input-example");
        let mut digit_counts = Counter::<u8, usize>::new();
        for d in data {
            digit_counts += get_solution(&d);
        }
        assert_eq!(
            digit_counts[&1] + digit_counts[&4] + digit_counts[&7] + digit_counts[&8],
            26
        );
    }

    #[test]
    fn part1_actual() {
        let data = datapoints("input-actual");
        let mut digit_counts = Counter::<u8, usize>::new();
        for d in data {
            digit_counts += get_solution(&d);
        }
        assert_eq!(
            digit_counts[&1] + digit_counts[&4] + digit_counts[&7] + digit_counts[&8],
            445
        );
    }

    #[test]
    fn part2_example() {
        let data = datapoints("input-example");
        let mut n = 0;
        for d in data {
            let msg = get_solution(&d);
            n += 1000 * msg[0] as u32 + 100 * msg[1] as u32 + 10 * msg[2] as u32 + msg[3] as u32;
        }
        assert_eq!(n, 61229);
    }

    #[test]
    fn part2_actual() {
        let data = datapoints("input-actual");
        let mut n = 0;
        for d in data {
            let msg = get_solution(&d);
            n += 1000 * msg[0] as u32 + 100 * msg[1] as u32 + 10 * msg[2] as u32 + msg[3] as u32;
        }
        assert_eq!(n, 1043101);
    }
}
