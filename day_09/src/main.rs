use general::read_data_lines;
use ndarray::{Array, Array2, ArrayView};
use std::collections::BTreeSet;
use structopt::StructOpt;

const PUZZLE_NAME: &str = "Advent of Code: Day 9 -- Version:";
const PUZZLE_ABOUT: &str = "Smoke Basin: https://adventofcode.com/2021/day/9";

fn get_heatmap(data: &[String]) -> Array2<u32> {
    // row parsing rules for data[String]
    let get_row = |s: &str| {
        s.chars()
            .map(|s| s.to_string().parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
    };

    // use data[0] to size the new Array2
    let row = get_row(&data[0]);
    let mut heatmap = Array::zeros((0, row.len()));
    heatmap.push_row(ArrayView::from(&row)).unwrap();

    // process remaining data[1..]
    for line in &data[1..] {
        heatmap.push_row(ArrayView::from(&get_row(line))).unwrap();
    }
    heatmap
}

fn get_lowpoints(heatmap: &Array2<u32>) -> Vec<(usize, usize)> {
    let nrow = heatmap.nrows();
    let ncol = heatmap.ncols();

    let is_lowpoint = |r, c, n| {
        (r == 0 || heatmap[[r - 1, c]] > n)
            && (r + 1 == nrow || heatmap[[r + 1, c]] > n)
            && (c == 0 || heatmap[[r, c - 1]] > n)
            && (c + 1 == ncol || heatmap[[r, c + 1]] > n)
    };

    let mut lowpoints = vec![];
    for row in 0..nrow {
        for col in 0..ncol {
            if is_lowpoint(row, col, heatmap[[row, col]]) {
                lowpoints.push((row, col));
            }
        }
    }
    lowpoints
}

fn find_basin(heatmap: &Array2<u32>, point: &(usize, usize), basin: &mut BTreeSet<(usize, usize)>) {
    if basin.contains(point) {
        return;
    }
    basin.insert(*point);

    let (r, c) = *point;
    let n = heatmap[[r, c]];

    let mut adjacents = vec![];
    if r != 0 && heatmap[[r - 1, c]] > n {
        adjacents.push((r - 1, c));
    }
    if r + 1 < heatmap.nrows() && heatmap[[r + 1, c]] > n {
        adjacents.push((r + 1, c));
    }
    if c != 0 && heatmap[[r, c - 1]] > n {
        adjacents.push((r, c - 1));
    }
    if c + 1 < heatmap.ncols() && heatmap[[r, c + 1]] > n {
        adjacents.push((r, c + 1));
    }

    adjacents
        .iter()
        .filter(|(r, c)| heatmap[[*r, *c]] != 9)
        .for_each(|p| find_basin(heatmap, p, basin));
}

fn get_solution1(heatmap: &Array2<u32>) -> u32 {
    get_lowpoints(heatmap)
        .iter()
        .map(|(r, c)| heatmap[[*r, *c]] + 1)
        .sum::<u32>()
}

fn get_solution2(heatmap: &Array2<u32>) -> u32 {
    let mut basin_sizes = vec![];

    for point in get_lowpoints(heatmap).iter() {
        let mut basin = BTreeSet::<(usize, usize)>::new();
        find_basin(heatmap, point, &mut basin);
        basin_sizes.push(basin.len());
    }

    assert!(basin_sizes.len() > 2);
    basin_sizes.sort_by(|a, b| b.cmp(a));
    //(basin_sizes[0] * basin_sizes[1] * basin_sizes[2]) as u32
    basin_sizes.iter().take(3).fold(1, |acc, x| acc * x) as u32
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
    let heatmap = get_heatmap(&data);
    //println!("heatmap = {:?}", heatmap);

    println!("Answer Part 1 = {}", get_solution1(&heatmap));
    println!("Answer Part 2 = {}", get_solution2(&heatmap));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn datapoints(filename: &str) -> Array2<u32> {
        let file = Some(std::path::PathBuf::from(filename));
        let data = read_data_lines::<String>(file).unwrap();
        get_heatmap(&data)
    }

    #[test]
    fn part1_example() {
        let data = datapoints("input-example");
        assert_eq!(get_solution1(&data), 15);
    }

    #[test]
    fn part1_actual() {
        let data = datapoints("input-actual");
        assert_eq!(get_solution1(&data), 572);
    }

    #[test]
    fn part2_example() {
        let data = datapoints("input-example");
        assert_eq!(get_solution2(&data), 1134);
    }

    #[test]
    fn part2_actual() {
        let data = datapoints("input-actual");
        assert_eq!(get_solution2(&data), 847044);
    }
}
