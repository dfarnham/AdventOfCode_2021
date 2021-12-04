use general::read_data_lines;
use std::collections::HashMap;
use std::path::PathBuf;
use structopt::StructOpt;

// https://adventofcode.com/2021/day/2

// Given an input file containing key/value pairs separated by whitespace
// Return a map of summed values for each key and a "depth" calculation
fn get_move_data(filename: Option<PathBuf>) -> Result<HashMap<String, i32>, Box<dyn std::error::Error>> {
    let mut move_data: HashMap<String, i32> = HashMap::new();
    let mut aim = 0;
    let mut depth = 0;

    for line in read_data_lines::<String>(filename)? {
        let fields = line.split_whitespace().collect::<Vec<&str>>();
        assert!(
            fields.len() == 2,
            "Expected 2 fields have {}: {:?}",
            fields.len(),
            fields
        );

        let units = move_data.entry(fields[0].into()).or_insert(0);
        let value = fields[1].parse::<i32>()?;
        *units += value;

        match fields[0] {
            "forward" => depth += aim * value,
            "down" => aim += value,
            "up" => aim -= value,
            _ => panic!("{}", format!("Unknown command {}", fields[0])),
        }
    }
    move_data.insert("depth".into(), depth);
    Ok(move_data)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[derive(StructOpt)]
    #[structopt(name = "Advent of Code: Day 2\nVersion:", about = "Dive!")]
    struct Cli {
        #[structopt(
            short,
            long,
            parse(from_os_str),
            help = "file|stdin -- command (forward/up/down unit), one per line"
        )]
        input: Option<std::path::PathBuf>,
    }
    let args = Cli::from_args();

    // ==============================================================

    let move_data = get_move_data(args.input)?;
    let forward = move_data.get("forward").ok_or("missing key \"forward\"")?;
    let up = move_data.get("up").ok_or("missing key \"up\"")?;
    let down = move_data.get("down").ok_or("missing key \"down\"")?;
    let depth = move_data.get("depth").ok_or("missing key \"depth\"")?;
    println!("Answer Part 1 = {}", forward * (down - up));
    println!("Answer Part 2 = {}", forward * depth);
    Ok(())
}

#[test]
fn part1_example() {
    let file = Some(std::path::PathBuf::from("input-example"));
    let move_data = get_move_data(file).unwrap();
    assert_eq!(
        move_data.get("forward").unwrap() * (move_data.get("down").unwrap() - move_data.get("up").unwrap()),
        150
    );
}

#[test]
fn part1_actual() {
    let file = Some(std::path::PathBuf::from("input-actual"));
    let move_data = get_move_data(file).unwrap();
    assert_eq!(
        move_data.get("forward").unwrap() * (move_data.get("down").unwrap() - move_data.get("up").unwrap()),
        1938402
    );
}

#[test]
fn part2_example() {
    let file = Some(std::path::PathBuf::from("input-example"));
    let move_data = get_move_data(file).unwrap();
    assert_eq!(move_data.get("forward").unwrap() * move_data.get("depth").unwrap(), 900);
}

#[test]
fn part2_actual() {
    let file = Some(std::path::PathBuf::from("input-actual"));
    let move_data = get_move_data(file).unwrap();
    assert_eq!(
        move_data.get("forward").unwrap() * move_data.get("depth").unwrap(),
        1947878632
    );
}
