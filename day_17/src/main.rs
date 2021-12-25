use general::read_data_lines;
use regex::Regex;
use std::collections::HashSet;
use structopt::StructOpt;

const PUZZLE_NAME: &str = "Advent of Code: Day 17 -- Version:";
const PUZZLE_ABOUT: &str = "Trick Shot: https://adventofcode.com/2021/day/17";

fn get_target_area(data: &str) -> (i64, i64, i64, i64) {
    let re = Regex::new(r"target\s+area:\s+x=(\d+)\.\.(\d+),\s+y=(-\d+)\.\.(-\d+)").unwrap();
    let captures = re.captures(data).unwrap();
    (
        captures.get(1).map(|s| s.as_str().parse::<i64>().unwrap()).unwrap(),
        captures.get(2).map(|s| s.as_str().parse::<i64>().unwrap()).unwrap(),
        captures.get(3).map(|s| s.as_str().parse::<i64>().unwrap()).unwrap(),
        captures.get(4).map(|s| s.as_str().parse::<i64>().unwrap()).unwrap(),
    )
}

fn solutions(data: &str) -> (i64, usize) {
    let (xmin, xmax, ymin, ymax) = get_target_area(data);

    let mut best_y = i64::MIN;
    let mut velocity = HashSet::new();

    for n in ((2.0 * xmin as f64).sqrt().round() as i64)..=xmax {
        for m in ymin..ymin.abs() {
            let (mut x, mut y) = (0, 0);
            let (mut xv, mut yv) = (n as i64, m as i64);
            let mut max_y = 0;
            let mut success = false;
            for _step in 0..(2 * ymin.abs()) {
                if x + xv == x && (x < xmin || x > xmax) {
                    break;
                }

                x += xv;
                y += yv;
                xv = match xv < 0 {
                    true => xv + 1,
                    false => 0.max(xv - 1),
                };
                yv -= 1;

                if x >= xmin && x <= xmax && y >= ymin && y <= ymax {
                    success = true;
                    velocity.insert((n, m));
                }
                max_y = max_y.max(y);
            }
            if success && max_y > best_y {
                best_y = max_y;
            }
        }
    }
    (best_y, velocity.len())
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

    let data = read_data_lines::<String>(args.input)?;
    let (best, n) = solutions(&data[0]);
    println!("Answer Part 1 = {:?}", best);
    println!("Answer Part 2 = {:?}", n);

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
        assert_eq!(solutions(&data[0]).0, 45);
    }

    #[test]
    fn part1_actual() {
        let data = get_data("input-actual");
        assert_eq!(solutions(&data[0]).0, 6786);
    }

    #[test]
    fn part2_example() {
        let data = get_data("input-example");
        assert_eq!(solutions(&data[0]).1, 112);
    }

    #[test]
    fn part2_actual() {
        let data = get_data("input-actual");
        assert_eq!(solutions(&data[0]).1, 2313);
    }
}
