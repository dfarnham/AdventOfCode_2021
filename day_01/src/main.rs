use general::read_int_lines;
use structopt::StructOpt;

// Given an input array:
// Count the number of times the sum of measurements in a provided sliding window increases
fn increase_window_count(array: &[i32], window: usize) -> usize {
    assert!(window > 0, "Window must be > 0");
    assert!(
        array.len() > window,
        "Array length: {} must be greater than the window size: {}",
        array.len(),
        window
    );
    (0..(array.len() - window))
        .filter(|i| {
            array[*i..(*i + window)].iter().sum::<i32>()
                < array[(*i + 1)..(*i + 1 + window)].iter().sum::<i32>()
        })
        .count()
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

    let measurements = read_int_lines(args.input)?;
    println!("Answer Part 1 = {}", increase_window_count(&measurements, 1));
    println!("Answer Part 2 = {}", increase_window_count(&measurements, 3));
    Ok(())
}


#[test]
#[should_panic]
fn empty_array() {
    let measurements = vec![];
    increase_window_count(&measurements, 1);
}

#[test]
#[should_panic]
fn array_too_small() {
    let measurements = vec![199];
    increase_window_count(&measurements, 1);
}

#[test]
#[should_panic]
fn invalid_window() {
    let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    increase_window_count(&measurements, 0);
}

#[test]
fn part1_example() {
    let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    assert_eq!(increase_window_count(&measurements, 1), 7);
}

#[test]
fn part2_example() {
    let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    assert_eq!(increase_window_count(&measurements, 3), 5);
}
