use general::read_data_lines;
use ndarray::{Array, Array2, ArrayView};
use std::collections::HashSet;
use structopt::StructOpt;

const PUZZLE_NAME: &str = "Advent of Code: Day 4 -- Version:";
const PUZZLE_ABOUT: &str = "Giant Squid: https://adventofcode.com/2021/day/4";

const BOARD_DIM: usize = 5;
const MATCH: u32 = u32::MAX;

fn winning_board(board: &Array2<u32>) -> bool {
    for row in 0..BOARD_DIM {
        if BOARD_DIM
            == (0..BOARD_DIM)
                .into_iter()
                .filter(|col| board[[row, *col]] == MATCH)
                .count()
        {
            return true;
        }
    }

    for col in 0..BOARD_DIM {
        if BOARD_DIM
            == (0..BOARD_DIM)
                .into_iter()
                .filter(|row| board[[*row, col]] == MATCH)
                .count()
        {
            return true;
        }
    }

    false
}

fn score_board(board: &Array2<u32>) -> u32 {
    (0..BOARD_DIM)
        .into_iter()
        .map(|row| {
            (0..BOARD_DIM)
                .into_iter()
                .map(|col| board[[row, col]])
                .filter(|n| *n != MATCH)
                .sum::<u32>()
        })
        .sum::<u32>()
}

fn update_board(draw: u32, board: &mut Array2<u32>) {
    for row in 0..BOARD_DIM {
        for col in 0..BOARD_DIM {
            if board[[row, col]] == draw {
                board[[row, col]] = MATCH;
            }
        }
    }
}

fn get_boards(data: &[String]) -> (Vec<u32>, Vec<Array2<u32>>) {
    // random draw is the first line
    let random_draw = data[0]
        .split(',')
        .map(|s| s.trim().parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    // read all the 5x5 boards into an array
    let mut boards = vec![];
    let mut board = Array::zeros((0, BOARD_DIM));
    for (i, line) in data[1..].iter().filter(|s| !s.is_empty()).enumerate() {
        if i % BOARD_DIM == 0 && !board.is_empty() {
            boards.push(board);
            board = Array::zeros((0, BOARD_DIM));
        }
        let row = line
            .split_whitespace()
            .map(|s| s.trim().parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        board.push_row(ArrayView::from(&row)).unwrap();
    }
    if !board.is_empty() {
        boards.push(board);
    }

    // validate all the boards are 5x5 (BOARD_DIM x BOARD_DIM)
    for b in &boards {
        assert_eq!(b.nrows(), BOARD_DIM, "invalid board rows = {}", b.nrows());
        assert_eq!(b.ncols(), BOARD_DIM, "invalid board columns = {}", b.ncols());
    }

    (random_draw, boards)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[derive(StructOpt)]
    #[structopt(name = PUZZLE_NAME, about = PUZZLE_ABOUT)]
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

    let data = read_data_lines::<String>(args.input)?;
    let (random_draw, mut boards) = get_boards(&data);
    //println!("random_draw = {:?}", random_draw);

    for b in &boards {
        assert_eq!(b.nrows(), BOARD_DIM, "invalid board rows = {}", b.nrows());
        assert_eq!(b.ncols(), BOARD_DIM, "invalid board columns = {}", b.ncols());
        //println!("board = {:?}", b);
    }

    let mut score1 = None;
    for draw in random_draw.iter() {
        if score1.is_none() {
            //println!("draw = {}", draw);
            for b in &mut boards {
                update_board(*draw, b);
            }
            for b in &boards {
                if winning_board(b) {
                    //println!("board = {:?}", b);
                    score1 = Some(score_board(b) * draw);
                }
            }
        }
    }

    let mut completed = HashSet::new();
    let mut score2 = None;
    for draw in random_draw.iter() {
        //println!("draw = {}", draw);
        let mut i = 0;
        for b in &mut boards {
            if !completed.contains(&i) {
                update_board(*draw, b);
            }
            i += 1;
        }

        i = 0;
        for b in &boards {
            if !completed.contains(&i) && winning_board(b) {
                //println!("board = {:?}", b);
                completed.insert(i);
                if completed.len() == boards.len() {
                    score2 = Some(score_board(b) * draw);
                }
            }
            i += 1;
        }
    }

    println!("Answer Part 1 = {}", score1.ok_or("no winner")?);
    println!("Answer Part 2 = {}", score2.ok_or("no winner")?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn part1(filename: &str) -> u32 {
        let file = Some(std::path::PathBuf::from(filename));
        let data = read_data_lines::<String>(file).unwrap();
        let (random_draw, mut boards) = get_boards(&data);

        let mut score = None;
        for draw in random_draw.iter() {
            if score.is_none() {
                for b in &mut boards {
                    update_board(*draw, b);
                }
                for b in &boards {
                    if winning_board(b) {
                        score = Some(score_board(b) * draw);
                    }
                }
            }
        }
        score.unwrap()
    }

    fn part2(filename: &str) -> u32 {
        let file = Some(std::path::PathBuf::from(filename));
        let data = read_data_lines::<String>(file).unwrap();
        let (random_draw, mut boards) = get_boards(&data);

        let mut completed = HashSet::new();
        let mut score = None;
        for draw in random_draw.iter() {
            let mut i = 0;
            for b in &mut boards {
                if !completed.contains(&i) {
                    update_board(*draw, b);
                }
                i += 1;
            }

            i = 0;
            for b in &boards {
                if !completed.contains(&i) {
                    if winning_board(b) {
                        completed.insert(i);
                        if completed.len() == boards.len() {
                            score = Some(score_board(b) * draw);
                        }
                    }
                }
                i += 1;
            }
        }
        score.unwrap()
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1("input-example"), 4512);
    }

    #[test]
    fn part1_actual() {
        assert_eq!(part1("input-actual"), 55770);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2("input-example"), 1924);
    }

    #[test]
    fn part2_actual() {
        assert_eq!(part2("input-actual"), 2980);
    }
}
