use counter::Counter;
use general::read_data_lines;
use structopt::StructOpt;

const PUZZLE_NAME: &str = "Advent of Code: Day 10 -- Version:";
const PUZZLE_ABOUT: &str = "Syntax Scoring: https://adventofcode.com/2021/day/10";

fn get_solutions(data: &[String]) -> (u64, u64) {
    let mut illegal = Counter::<char, u64>::new();
    let mut scores = vec![];
    for line in data {
        let mut stack = vec![];
        let mut corrupt_line = false;
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' | ']' | '}' | '>' => {
                    let p = stack.pop();
                    if p.is_none()
                        || p == Some('(') && c != ')'
                        || p == Some('[') && c != ']'
                        || p == Some('{') && c != '}'
                        || p == Some('<') && c != '>'
                    {
                        illegal[&c] += 1;
                        corrupt_line = true;
                        break;
                    }
                }
                _ => panic!("{}", format!("unknown char: {}", c)),
            }
        }

        if !corrupt_line {
            let mut score = 0;
            while let Some(c) = stack.pop() {
                score *= 5;
                match c {
                    '(' => score += 1,
                    '[' => score += 2,
                    '{' => score += 3,
                    '<' => score += 4,
                    _ => panic!("{}", format!("unexpected char: {}", c)),
                }
            }
            scores.push(score);
        }
    }
    assert_eq!(scores.len() % 2, 1, "scores must be and odd number: {}", scores.len());
    scores.sort_unstable();
    (
        3 * illegal[&')'] + 57 * illegal[&']'] + 1197 * illegal[&'}'] + 25137 * illegal[&'>'],
        scores[scores.len() / 2],
    )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[derive(StructOpt)]
    #[structopt(name = PUZZLE_NAME, about = PUZZLE_ABOUT)]
    struct Cli {
        #[structopt(
            short,
            long,
            parse(from_os_str),
            help = "file|stdin -- lines of open/close delimiter characters"
        )]
        input: Option<std::path::PathBuf>,
    }
    let args = Cli::from_args();

    // ==============================================================

    let data = read_data_lines::<String>(args.input)?;
    let (p1, p2) = get_solutions(&data);
    println!("Answer Part 1 = {}", p1);
    println!("Answer Part 2 = {}", p2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_data(filename: &str) -> Vec<String> {
        let file = Some(std::path::PathBuf::from(filename));
        read_data_lines::<String>(file).unwrap()
    }

    #[test]
    fn part1_example() {
        let data = get_data("input-example");
        assert_eq!(get_solutions(&data).0, 26397);
    }

    #[test]
    fn part1_actual() {
        let data = get_data("input-actual");
        assert_eq!(get_solutions(&data).0, 464991);
    }

    #[test]
    fn part2_example() {
        let data = get_data("input-example");
        assert_eq!(get_solutions(&data).1, 288957);
    }

    #[test]
    fn part2_actual() {
        let data = get_data("input-actual");
        assert_eq!(get_solutions(&data).1, 3662008566);
    }
}
