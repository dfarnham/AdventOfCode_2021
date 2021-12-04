use general::read_data_lines;
use ndarray::{Array, ArrayBase, ArrayView, Dim, OwnedRepr};
use std::collections::HashSet;
use structopt::StructOpt;

// https://adventofcode.com/2021/day/4
const PUZZLE_NAME: &str = "Advent of Code: Day 4 -- Version:";
const PUZZLE_ABOUT: &str = "Giant Squid";

const BOARD_SIZE: usize = 5;

fn winning_board(board: &ArrayBase<OwnedRepr<u32>, Dim<[usize; 2]>>) -> bool {
    for row in 0..BOARD_SIZE {
        let mut count = 0;
        for col in 0..BOARD_SIZE {
            if board[[row, col]] == u32::MAX {
                count += 1;
            }
        }

        if count == BOARD_SIZE {
            return true;
        }
    }

    for col in 0..BOARD_SIZE {
        let mut count = 0;
        for row in 0..BOARD_SIZE {
            if board[[row, col]] == u32::MAX {
                count += 1;
            }
        }

        if count == BOARD_SIZE {
            return true;
        }
    }

    false
}

fn score_board(board: &ArrayBase<OwnedRepr<u32>, Dim<[usize; 2]>>) -> u32 {
    let mut score = 0;
    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            if board[[row, col]] != u32::MAX {
                score += board[[row, col]];
            }
        }
    }
    score
}

fn update_board(draw: u32, board: &mut ArrayBase<OwnedRepr<u32>, Dim<[usize; 2]>>) {
    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            if board[[row, col]] == draw {
                board[[row, col]] = u32::MAX;
            }
        }
    }
}

#[allow(clippy::type_complexity)]
fn get_boards(data: &[String]) -> (Vec<u32>, Vec<ArrayBase<OwnedRepr<u32>, Dim<[usize; 2]>>>) {
    // random draw is the first line
    let random_draw = data[0]
        .split(',')
        .map(|s| s.trim().parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    // read all the 5x5 boards into an array
    let mut boards = vec![];
    let mut board = Array::zeros((0, BOARD_SIZE));
    for (i, line) in data[1..].iter().filter(|s| !s.is_empty()).enumerate() {
        if i % BOARD_SIZE == 0 && !board.is_empty() {
            boards.push(board);
            board = Array::zeros((0, BOARD_SIZE));
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

    // validate all the boards are 5x5 (BOARD_SIZE x BOARD_SIZE)
    for b in &boards {
        assert_eq!(b.nrows(), BOARD_SIZE, "invalid board rows = {}", b.nrows());
        assert_eq!(b.ncols(), BOARD_SIZE, "invalid board columns = {}", b.ncols());
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
        assert_eq!(b.nrows(), BOARD_SIZE, "invalid board rows = {}", b.nrows());
        assert_eq!(b.ncols(), BOARD_SIZE, "invalid board columns = {}", b.ncols());
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
