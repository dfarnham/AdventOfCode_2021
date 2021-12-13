use general::read_data_lines;
use ndarray::Array2;
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
                    .map(|s| s.to_string().parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();
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

    let xmax = *x.iter().max().expect("xmax failure") as usize;
    let ymax = *y.iter().max().expect("ymax max failure") as usize;

    // The new Array2
    let mut paper = Array2::zeros((1 + xmax, 1 + ymax));

    for (i, j) in x.iter().zip(y.iter()) {
        paper[[*i as usize, *j as usize]] = 1;
    }
    (paper, folds)
}

fn foldit(paper: &Array2<u32>, direction: i32) -> Array2<u32> {
    // negative direction represent X
    // positive direction represent Y

    let xpos = direction.abs() as usize;
    let ypos = direction.abs() as usize;

    let mut folded = match direction < 0 {
        true => {
            let sz = xpos.max(paper.nrows() - xpos - 1) as usize;
            Array2::zeros((sz as usize, paper.ncols()))
        }
        false => {
            let sz = ypos.max(paper.ncols() - ypos - 1) as usize;
            Array2::zeros((paper.nrows(), sz as usize))
        }
    };

    match direction < 0 {
        true => {
            for i in 0..xpos {
                for j in 0..folded.ncols() {
                    folded[[i, j]] = paper[[i, j]];
                }
            }
            for (c, i) in ((xpos + 1)..paper.nrows()).enumerate() {
                let a = i - 2 * c - 2;
                for j in 0..folded.ncols() {
                    folded[[a, j]] = 1.min(folded[[a, j]] + paper[[i, j]]);
                }
            }
        }
        false => {
            for i in 0..folded.nrows() {
                for j in 0..ypos {
                    folded[[i, j]] = paper[[i, j]];
                }
            }
            for i in 0..folded.nrows() {
                for (c, j) in ((ypos + 1)..paper.ncols()).enumerate() {
                    let b = j - 2 * c - 2;
                    folded[[i, b]] = 1.min(folded[[i, b]] + paper[[i, j]]);
                }
            }
        }
    };

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

    paper = foldit(&paper, instructions[0]);
    println!("Answer Part 1 = {}", paper.sum());

    for instruction in &instructions[1..] {
        paper = foldit(&paper, *instruction);
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
        paper = foldit(&paper, instructions[0]);
        assert_eq!(paper.sum(), 17);
    }

    #[test]
    fn part1_actual() {
        let data = get_testdata("input-actual");
        let (mut paper, instructions) = get_data(&data);
        paper = foldit(&paper, instructions[0]);
        assert_eq!(paper.sum(), 790);
    }

    #[test]
    fn part2_example() {
        let data = get_testdata("input-example");
        let (mut paper, instructions) = get_data(&data);
        for instruction in &instructions {
            paper = foldit(&paper, *instruction);
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
            paper = foldit(&paper, *instruction);
        }
        let message = get_message(&paper);
        let expected = "###   ##  #  # #### ###  ####   ##  ##  \n#  # #  # #  #    # #  # #       # #  # \n#  # #    ####   #  ###  ###     # #    \n###  # ## #  #  #   #  # #       # #    \n#    #  # #  # #    #  # #    #  # #  # \n#     ### #  # #### ###  #     ##   ##  \n";
        assert_eq!(message, expected);
    }
}
