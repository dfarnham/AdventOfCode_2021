use general::read_data_lines;
use ndarray::{Array, Array2, ArrayView};
use structopt::StructOpt;

const PUZZLE_NAME: &str = "Advent of Code: Day 25 -- Version:";
const PUZZLE_ABOUT: &str = "Sea Cucumber: https://adventofcode.com/2021/day/25";

#[derive(Clone, PartialEq)]
enum Cell {
    East,
    South,
    Empty,
}

fn get_image(data: &[String]) -> Array2<Cell> {
    // row parsing rules for data[String]
    let get_row = |s: &str| {
        s.chars()
            .map(|c| match c {
                '>' => Cell::East,
                'v' => Cell::South,
                _ => Cell::Empty,
            })
            .collect::<Vec<_>>()
    };

    // use data[0] to size the new Array2
    let row = get_row(&data[0]);
    let mut grid = Array::from_elem((0, row.len()), Cell::Empty);

    // push the 1st row
    grid.push_row(ArrayView::from(&row)).unwrap();

    // process remaining data[1..]
    for line in &data[1..] {
        grid.push_row(ArrayView::from(&get_row(line))).unwrap();
    }
    grid
}

#[allow(dead_code)]
fn display(image: &Array2<Cell>) {
    for row in image.rows() {
        for elem in row {
            match elem {
                Cell::East => print!(">"),
                Cell::South => print!("v"),
                Cell::Empty => print!("."),
            };
        }
        println!();
    }
    println!();
}

fn solution1(image: &Array2<Cell>) -> usize {
    //display(&image);
    let nrows = image.nrows();
    let ncols = image.ncols();
    let mut new_image = image.clone();
    let mut steps = 0;

    loop {
        let mut stuck = true;
        steps += 1;

        for mut row in new_image.rows_mut() {
            // find all the ">." cells in the row and add to "swap" list 
            let swaps = (0..row.len())
                .filter(|&i| row[i] == Cell::East && row[(i + 1) % ncols] == Cell::Empty)
                .collect::<Vec<_>>();

            // if the swaps list is ever not empty, we're unstuck (stuck = false)
            stuck &= swaps.is_empty();

            // turn ">." into ".>"
            for i in swaps {
                row[i] = Cell::Empty;
                row[(i + 1) % ncols] = Cell::East;
            }
        }

        for mut col in new_image.columns_mut() {
            // find all the "v." cells in the column and add to "swap" list 
            let swaps = (0..col.len())
                .filter(|&j| col[j] == Cell::South && col[(j + 1) % nrows] == Cell::Empty)
                .collect::<Vec<_>>();

            // if the swaps list is ever not empty, we're unstuck (stuck = false)
            stuck &= swaps.is_empty();

            // turn "v." into ".v"
            for j in swaps {
                col[j] = Cell::Empty;
                col[(j + 1) % nrows] = Cell::South;
            }
        }

        // all swap lists were empty
        if stuck {
            break;
        }
    }
    steps
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
    let image = get_image(&data);
    println!("Answer Part 1 = {}", solution1(&image));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data(filename: &str) -> Array2<Cell> {
        let file = Some(std::path::PathBuf::from(filename));
        get_image(&read_data_lines::<String>(file).unwrap())
    }

    #[test]
    fn part1_example() {
        let image = get_test_data("input-example");
        assert_eq!(58, solution1(&image));
    }

    #[test]
    fn part1_actual() {
        let image = get_test_data("input-actual");
        assert_eq!(360, solution1(&image));
    }
}
