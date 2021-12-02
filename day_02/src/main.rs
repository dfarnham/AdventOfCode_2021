use general::read_string_lines;
use std::collections::HashMap;
use std::path::PathBuf;
use structopt::StructOpt;

// https://adventofcode.com/2021/day/2

// Given an input file containing key/value pairs separated by whitespace
// Return a map of summed values for each key and a "depth" calculation
fn get_move_totals(filename: Option<PathBuf>) -> Result<HashMap<String, i32>, Box<dyn std::error::Error>> {
    let mut moves: HashMap<String, i32> = HashMap::new();
    let mut aim = 0;
    let mut depth = 0;
    for line in read_string_lines(filename)? {
        let fields = line.split_whitespace().collect::<Vec<&str>>();
        assert!(
            fields.len() == 2,
            "Expected 2 fields have {}: {:?}",
            fields.len(),
            fields
        );

        let units = moves.entry(fields[0].into()).or_insert(0);
        *units += fields[1].parse::<i32>()?;

        match fields[0] {
            "forward" => {
                depth += aim * fields[1].parse::<i32>()?;
            }
            "down" => aim += fields[1].parse::<i32>()?,
            "up" => aim -= fields[1].parse::<i32>()?,
            _ => panic!("{}", format!("Unknown command {}", fields[0])),
        }
    }
    moves.insert("depth".into(), depth);
    Ok(moves)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[derive(StructOpt)]
    #[structopt(
        name = "Advent of Code: Day 1, part 1\nVersion:",
        about = "Count the number of times a depth measurement increases from the previous measurement. (There is no measurement before the first measurement.)"
    )]
    struct Cli {
        #[structopt(
            short,
            long,
            parse(from_os_str),
            help = "file|stdin -- Input measurements, one per line"
        )]
        input: Option<std::path::PathBuf>,
    }
    let args = Cli::from_args();

    // ==============================================================

    let moves = get_move_totals(args.input.clone())?;
    let forward = moves.get("forward").ok_or("missing key \"forward\"")?;
    let down = moves.get("down").ok_or("missing key \"down\"")?;
    let up = moves.get("up").ok_or("missing key \"up\"")?;
    let depth = moves.get("depth").ok_or("missing key \"depth\"")?;
    println!("Answer Part 1 = {}", forward * (down - up));
    println!("Answer Part 2 = {}", forward * depth);
    Ok(())
}

#[test]
fn part1_example() {
    let file = Some(std::path::PathBuf::from("input-example"));
    let moves = get_move_totals(file).unwrap();
    assert_eq!(
        moves.get("forward").unwrap() * (moves.get("down").unwrap() - moves.get("up").unwrap()),
        150
    );
}

#[test]
fn part1_actual() {
    let file = Some(std::path::PathBuf::from("input-actual"));
    let moves = get_move_totals(file).unwrap();
    assert_eq!(
        moves.get("forward").unwrap() * (moves.get("down").unwrap() - moves.get("up").unwrap()),
        1938402
    );
}

#[test]
fn part2_example() {
    let file = Some(std::path::PathBuf::from("input-example"));
    let moves = get_move_totals(file).unwrap();
    assert_eq!(moves.get("forward").unwrap() * moves.get("depth").unwrap(), 900);
}

#[test]
fn part2_actual() {
    let file = Some(std::path::PathBuf::from("input-actual"));
    let moves = get_move_totals(file).unwrap();
    assert_eq!(moves.get("forward").unwrap() * moves.get("depth").unwrap(), 1947878632);
}
