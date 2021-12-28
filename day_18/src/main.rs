use general::read_data_lines;
use structopt::StructOpt;

#[macro_use]
extern crate json;

type SnailNum = json::JsonValue;

const PUZZLE_NAME: &str = "Advent of Code: Day 18 -- Version:";
const PUZZLE_ABOUT: &str = "Snailfish: https://adventofcode.com/2021/day/18";

fn get_data(data: &[String]) -> Vec<SnailNum> {
    let mut nums = vec![];
    for line in data {
        nums.push(json::parse(line).unwrap())
    }
    nums
}

fn jnum(n: &json::JsonValue) -> u64 {
    match *n {
        json::JsonValue::Number(x) => {
            let f: f64 = x.into();
            f.round() as u64
        }
        _ => panic!("not a Number"),
    }
}

fn magnitude(n: &SnailNum) -> u64 {
    match n.is_number() {
        true => jnum(n),
        false => 3 * magnitude(&n[0]) + 2 * magnitude(&n[1]),
    }
}

fn split(n: &mut SnailNum) -> bool {
    if n[0].is_array() {
        if split(&mut n[0]) {
            return true;
        }
    } else {
        let x = jnum(&n[0]);
        if x > 9 {
            let x = (x as f64) / 2.0;
            n[0] = array![x.floor(), x.ceil()];
            return true;
        }
    }

    if n[1].is_array() {
        if split(&mut n[1]) {
            return true;
        }
    } else {
        let x = jnum(&n[1]);
        if x > 9 {
            let x = (x as f64) / 2.0;
            n[1] = array![x.floor(), x.ceil()];
            return true;
        }
    }

    false
}

fn add_l(n: &mut SnailNum, val: u64) {
    if n[0].is_number() {
        n[0] = (jnum(&n[0]) + val).into();
    } else {
        add_l(&mut n[0], val);
    }
}

fn add_r(n: &mut SnailNum, val: u64) {
    if n[1].is_number() {
        n[1] = (jnum(&n[1]) + val).into();
    } else {
        add_r(&mut n[1], val);
    }
}

fn explode_it(n: &mut SnailNum, depth: usize) -> Option<(u64, u64)> {
    if depth == 4 && n[0].is_number() && n[1].is_number() {
        return Some((jnum(&n[0]), jnum(&n[1])));
    }

    if n[0].is_array() {
        if let Some(pair) = explode_it(&mut n[0], depth + 1) {
            if depth == 3 {
                n[0] = 0.into();
            }

            // add pair.1 to the 1st pair.0 found in n[1]
            if n[1].is_array() {
                add_l(&mut n[1], pair.1);
            } else {
                n[1] = (jnum(&n[1]) + pair.1).into();
            }

            // pair.1 has just been added, zero it for subsequent additions
            return Some((pair.0, 0));
        }
    }

    if n[1].is_array() {
        if let Some(pair) = explode_it(&mut n[1], depth + 1) {
            if depth == 3 {
                n[1] = 0.into();
            }

            // add pair.0 to the 1st pair.1 found in n[0]
            if n[0].is_array() {
                add_r(&mut n[0], pair.0);
            } else {
                n[0] = (jnum(&n[0]) + pair.0).into();
            }

            // pair.0 has just been added, zero it for subsequent additions
            return Some((0, pair.1));
        }
    }

    None
}

fn explode(n: &mut SnailNum) {
    while explode_it(n, 0).is_some() {}
}

fn reduce(n: &mut SnailNum) {
    loop {
        explode(n);
        if !split(n) {
            break;
        }
    }
}

fn solution1(nums: &[SnailNum]) -> u64 {
    let mut sum = nums[0].clone();
    for n in &nums[1..] {
        sum = array![sum.clone(), n.clone()];
        reduce(&mut sum);
    }
    magnitude(&sum)
}

fn solution2(nums: &[SnailNum]) -> u64 {
    let mut best = 0;
    for a in &nums[..] {
        for b in &nums[..] {
            if a != b {
                let mut sum = array![a.clone(), b.clone()];
                reduce(&mut sum);
                best = best.max(magnitude(&sum));
            }
        }
    }
    best
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
    let snail_nums = get_data(&data);
    println!("Answer Part 1 = {}", solution1(&snail_nums));
    println!("Answer Part 2 = {}", solution2(&snail_nums));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data(filename: &str) -> Vec<SnailNum> {
        let file = Some(std::path::PathBuf::from(filename));
        let data = read_data_lines::<String>(file).unwrap();
        get_data(&data)
    }

    #[test]
    fn test1() {
        let mut snail_num = json::parse("[[[[[9,8],1],2],3],4]").unwrap();
        explode(&mut snail_num);
        let expect = json::parse("[[[[0,9],2],3],4]").unwrap();
        assert_eq!(snail_num, expect);
    }

    #[test]
    fn test2() {
        let mut snail_num = json::parse("[7,[6,[5,[4,[3,2]]]]]").unwrap();
        explode(&mut snail_num);
        let expect = json::parse("[7,[6,[5,[7,0]]]]").unwrap();
        assert_eq!(snail_num, expect);
    }

    #[test]
    fn test3() {
        let mut snail_num = json::parse("[[6,[5,[4,[3,2]]]],1]").unwrap();
        explode(&mut snail_num);
        let expect = json::parse("[[6,[5,[7,0]]],3]").unwrap();
        assert_eq!(snail_num, expect);
    }

    #[test]
    fn test4() {
        let mut snail_num = json::parse("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]").unwrap();
        explode(&mut snail_num);
        let expect = json::parse("[[3,[2,[8,0]]],[9,[5,[7,0]]]]").unwrap();
        assert_eq!(snail_num, expect);
    }

    #[test]
    fn test5() {
        let mut snail_num = json::parse("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]").unwrap();
        explode(&mut snail_num);
        let expect = json::parse("[[[[0,7],4],[15,[0,13]]],[1,1]]").unwrap();
        assert_eq!(snail_num, expect);
    }

    #[test]
    fn test6() {
        let mut snail_num = json::parse("[[[[0,7],4],[15,[0,13]]],[1,1]]").unwrap();
        split(&mut snail_num);
        let expect = json::parse("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]").unwrap();
        assert_eq!(snail_num, expect);
    }

    #[test]
    fn test7() {
        let mut snail_num = json::parse("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]").unwrap();
        split(&mut snail_num);
        let expect = json::parse("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]").unwrap();
        assert_eq!(snail_num, expect);
    }

    #[test]
    fn test8() {
        let mut snail_num = json::parse("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]").unwrap();
        reduce(&mut snail_num);
        let expect = json::parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").unwrap();
        assert_eq!(snail_num, expect);
    }

    #[test]
    fn test9() {
        let snail_num = json::parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").unwrap();
        assert_eq!(magnitude(&snail_num), 1384);
    }

    #[test]
    fn part1_example() {
        let data = get_test_data("input-example");
        assert_eq!(solution1(&data), 4140);
    }

    #[test]
    fn part1_actual() {
        let data = get_test_data("input-actual");
        assert_eq!(solution1(&data), 4235);
    }

    #[test]
    fn part2_example() {
        let data = get_test_data("input-example");
        assert_eq!(solution2(&data), 3993);
    }

    #[test]
    fn part2_actual() {
        let data = get_test_data("input-actual");
        assert_eq!(solution2(&data), 4659);
    }
}
