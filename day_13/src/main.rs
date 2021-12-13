use general::read_data_lines;
use ndarray::{s, Array2};
use structopt::StructOpt;

const PUZZLE_NAME: &str = "Advent of Code: Day 13 -- Version:";
const PUZZLE_ABOUT: &str = "Transparent Origami: https://adventofcode.com/2021/day/13";

fn get_data(data: &[String]) -> (Array2<u32>, Vec<i32>) {
    let mut folds = vec![];
    let mut x = vec![];
    let mut y = vec![];

    for line in data {
        match line.contains(',') {
            true => {
                let points = line
                    .trim()
                    .split(',')
                    .map(|s| s.to_string().parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                assert_eq!(points.len(), 2, "expected 2 points: {:?}", points);
                x.push(points[0]);
                y.push(points[1]);
            }
            false => {
                if !line.is_empty() {
                    let parts = line.trim().split('=').map(|s| s.into()).collect::<Vec<String>>();
                    assert_eq!(parts.len(), 2, "expected 2 parts: {:?}", parts);
                    match parts[0].as_ref() {
                        "fold along x" => folds.push(-parts[1].parse::<i32>().unwrap()),
                        "fold along y" => folds.push(parts[1].parse::<i32>().unwrap()),
                        _ => panic!("{}", format!("unknown instruction: {}", parts[0])),
                    };
                }
            }
        }
    }

    let xmax = *x.iter().max().expect("xmax failure");
    let ymax = *y.iter().max().expect("ymax max failure");

    // create a new Array2
    let mut paper = Array2::zeros((xmax + 1, ymax + 1));

    for (i, j) in x.into_iter().zip(y.into_iter()) {
        paper[[i, j]] = 1;
    }
    (paper, folds)
}

fn fold_up(paper: &Array2<u32>, pos: usize) -> Array2<u32> {
    // copy over elements from paper not being folded
    let mut folded = paper
        .slice(s![0..paper.nrows(), 0..pos.max(paper.ncols() - pos - 1)])
        .to_owned();

    // update with folded items from paper
    // if the sum > 0 it is set to 1
    for i in 0..folded.nrows() {
        for (c, j) in ((pos + 1)..paper.ncols()).enumerate() {
            let a = j - 2 - 2 * c;
            folded[[i, a]] = 1.min(folded[[i, a]] + paper[[i, j]]);
        }
    }
    folded
}

fn fold_left(paper: &Array2<u32>, pos: usize) -> Array2<u32> {
    // copy over elements from paper not being folded
    let mut folded = paper
        .slice(s![0..pos.max(paper.nrows() - pos - 1), 0..paper.ncols()])
        .to_owned();

    // update with folded items from paper
    // if the sum > 0 it is set to 1
    for (c, i) in ((pos + 1)..paper.nrows()).enumerate() {
        let a = i - 2 - 2 * c;
        for j in 0..folded.ncols() {
            folded[[a, j]] = 1.min(folded[[a, j]] + paper[[i, j]]);
        }
    }
    folded
}

fn get_message(paper: &Array2<u32>) -> String {
    let mut message = "".to_string();
    for row in paper.t().rows() {
        for elem in row {
            match elem {
                0 => message += " ",
                _ => message += "#",
            };
        }
        message += "\n";
    }
    message
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
    let (mut paper, instructions) = get_data(&data);

    // instructions are < 0 for "left", > 0 for "up"
    for instruction in &instructions[0..1] {
        paper = match instruction < &0 {
            true => fold_left(&paper, instruction.abs() as usize),
            false => fold_up(&paper, *instruction as usize),
        };
    }

    println!("Answer Part 1 = {}", paper.sum());

    for instruction in &instructions[1..] {
        paper = match instruction < &0 {
            true => fold_left(&paper, instruction.abs() as usize),
            false => fold_up(&paper, *instruction as usize),
        };
    }

    print!("Answer Part 2 =\n{}", get_message(&paper));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_testdata(filename: &str) -> Vec<String> {
        let file = Some(std::path::PathBuf::from(filename));
        read_data_lines::<String>(file).unwrap()
    }

    #[test]
    fn part1_example() {
        let data = get_testdata("input-example");
        let (mut paper, instructions) = get_data(&data);
        for instruction in &instructions[0..1] {
            paper = match instruction < &0 {
                true => fold_left(&paper, instruction.abs() as usize),
                false => fold_up(&paper, *instruction as usize),
            };
        }
        assert_eq!(paper.sum(), 17);
    }

    #[test]
    fn part1_actual() {
        let data = get_testdata("input-actual");
        let (mut paper, instructions) = get_data(&data);
        for instruction in &instructions[0..1] {
            paper = match instruction < &0 {
                true => fold_left(&paper, instruction.abs() as usize),
                false => fold_up(&paper, *instruction as usize),
            };
        }
        assert_eq!(paper.sum(), 790);
    }

    #[test]
    fn part2_example() {
        let data = get_testdata("input-example");
        let (mut paper, instructions) = get_data(&data);
        for instruction in &instructions {
            paper = match instruction < &0 {
                true => fold_left(&paper, instruction.abs() as usize),
                false => fold_up(&paper, *instruction as usize),
            };
        }
        let message = get_message(&paper);
        let expected = "#####\n#   #\n#   #\n#   #\n#####\n     \n     \n";
        assert_eq!(message, expected);
    }

    #[test]
    fn part2_actual() {
        let data = get_testdata("input-actual");
        let (mut paper, instructions) = get_data(&data);
        for instruction in &instructions {
            paper = match instruction < &0 {
                true => fold_left(&paper, instruction.abs() as usize),
                false => fold_up(&paper, *instruction as usize),
            };
        }
        let message = get_message(&paper);
        let expected = "###   ##  #  # #### ###  ####   ##  ##  \n#  # #  # #  #    # #  # #       # #  # \n#  # #    ####   #  ###  ###     # #    \n###  # ## #  #  #   #  # #       # #    \n#    #  # #  # #    #  # #    #  # #  # \n#     ### #  # #### ###  #     ##   ##  \n";
        assert_eq!(message, expected);
    }
}
