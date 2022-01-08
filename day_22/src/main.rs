use general::read_data_lines;
use regex::Regex;
use structopt::StructOpt;

const PUZZLE_NAME: &str = "Advent of Code: Day 22 -- Version:";
const PUZZLE_ABOUT: &str = "Reactor Reboot: https://adventofcode.com/2021/day/22";

#[derive(Debug, Copy, Clone, PartialEq)]
struct Cuboid {
    state: bool,
    xrange: (isize, isize),
    yrange: (isize, isize),
    zrange: (isize, isize),
}

fn get_data(data: &[String]) -> Vec<Cuboid> {
    let re = Regex::new(r"(on|off)\s+x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)").unwrap();
    let mut cuboids = vec![];

    for line in data {
        let captures = re.captures(line).unwrap();
        let state = captures.get(1).map(|s| s.as_str() == "on").unwrap();
        let xrange = (
            captures.get(2).map(|s| s.as_str().parse::<isize>().unwrap()).unwrap(),
            captures.get(3).map(|s| s.as_str().parse::<isize>().unwrap()).unwrap(),
        );
        let yrange = (
            captures.get(4).map(|s| s.as_str().parse::<isize>().unwrap()).unwrap(),
            captures.get(5).map(|s| s.as_str().parse::<isize>().unwrap()).unwrap(),
        );
        let zrange = (
            captures.get(6).map(|s| s.as_str().parse::<isize>().unwrap()).unwrap(),
            captures.get(7).map(|s| s.as_str().parse::<isize>().unwrap()).unwrap(),
        );
        assert!(xrange.0 <= xrange.1);
        assert!(yrange.0 <= yrange.1);
        assert!(zrange.0 <= zrange.1);
        cuboids.push(Cuboid {
            state,
            xrange,
            yrange,
            zrange,
        });
    }
    cuboids
}

fn universe(cuboids: &[Cuboid]) -> Cuboid {
    let xmin = cuboids.iter().map(|c| c.xrange.0).min().unwrap();
    let xmax = cuboids.iter().map(|c| c.xrange.1).max().unwrap();

    let ymin = cuboids.iter().map(|c| c.yrange.0).min().unwrap();
    let ymax = cuboids.iter().map(|c| c.yrange.1).max().unwrap();

    let zmin = cuboids.iter().map(|c| c.zrange.0).min().unwrap();
    let zmax = cuboids.iter().map(|c| c.zrange.1).max().unwrap();

    Cuboid {
        state: false,
        xrange: (xmin, xmax),
        yrange: (ymin, ymax),
        zrange: (zmin, zmax),
    }
}

fn universe_on_count(universe: &[Cuboid]) -> usize {
    universe
        .iter()
        .filter(|c| c.state)
        .map(|c| (1 + c.xrange.1 - c.xrange.0) * (1 + c.yrange.1 - c.yrange.0) * (1 + c.zrange.1 - c.zrange.0))
        .sum::<isize>() as usize
}

fn overlap(a: &Cuboid, b: &Cuboid) -> bool {
    let (axmin, axmax) = a.xrange;
    let (aymin, aymax) = a.yrange;
    let (azmin, azmax) = a.zrange;
    let (bxmin, bxmax) = b.xrange;
    let (bymin, bymax) = b.yrange;
    let (bzmin, bzmax) = b.zrange;

    (((axmin >= bxmin && axmin <= bxmax) || (axmax >= bxmin && axmax <= bxmax))
        || ((bxmin >= axmin && bxmin <= axmax) || (bxmax >= axmin && bxmax <= axmax)))
        && (((aymin >= bymin && aymin <= bymax) || (aymax >= bymin && aymax <= bymax))
            || ((bymin >= aymin && bymin <= aymax) || (bymax >= aymin && bymax <= aymax)))
        && (((azmin >= bzmin && azmin <= bzmax) || (azmax >= bzmin && azmax <= bzmax))
            || ((bzmin >= azmin && bzmin <= azmax) || (bzmax >= azmin && bzmax <= azmax)))
}

// returns a list of non-ovlapping cuboids
// if "a" overlaps "b" then "b" is split into all the non-ovlapping cuboids
//
fn split(a: &Cuboid, b: &Cuboid) -> Vec<Cuboid> {
    let mut cub = vec![];

    if overlap(a, b) {
        let (axmin, axmax) = a.xrange;
        let (aymin, aymax) = a.yrange;
        let (azmin, azmax) = a.zrange;
        let (mut bxmin, mut bxmax) = b.xrange;
        let (mut bymin, mut bymax) = b.yrange;
        let (bzmin, bzmax) = b.zrange;

        if axmin > bxmin && axmin <= bxmax {
            cub.push(Cuboid {
                state: b.state,
                xrange: (bxmin, axmin - 1),
                yrange: b.yrange,
                zrange: b.zrange,
            });
            bxmin = axmin;
        }
        if axmax < bxmax && axmax >= bxmin {
            cub.push(Cuboid {
                state: b.state,
                xrange: (axmax + 1, bxmax),
                yrange: b.yrange,
                zrange: b.zrange,
            });
            bxmax = axmax;
        }

        if aymin > bymin && aymin <= bymax {
            cub.push(Cuboid {
                state: b.state,
                xrange: (bxmin, bxmax),
                yrange: (bymin, aymin - 1),
                zrange: b.zrange,
            });
            bymin = aymin;
        }
        if aymax < bymax && aymax >= bymin {
            cub.push(Cuboid {
                state: b.state,
                xrange: (bxmin, bxmax),
                yrange: (aymax + 1, bymax),
                zrange: b.zrange,
            });
            bymax = aymax;
        }

        if azmin > bzmin && aymin <= bymax {
            cub.push(Cuboid {
                state: b.state,
                xrange: (bxmin, bxmax),
                yrange: (bymin, bymax),
                zrange: (bzmin, azmin - 1),
            });
        }
        if azmax < bzmax && azmax >= bzmin {
            cub.push(Cuboid {
                state: b.state,
                xrange: (bxmin, bxmax),
                yrange: (bymin, bymax),
                zrange: (azmax + 1, bzmax),
            });
        }
    } else {
        cub.push(*b);
    }
    cub
}

fn repartition(cuboid: &Cuboid, universe: &[Cuboid]) -> Vec<Cuboid> {
    let mut new_universe = vec![];

    for c in universe {
        // if cuboid overlaps c then c is split into non-overlapping cuboids
        new_universe.extend(split(cuboid, c));
    }
    new_universe.push(*cuboid);
    new_universe
}

fn solution1(cuboids: &[Cuboid]) -> usize {
    let mut universe = vec![universe(cuboids)];
    for cuboid in cuboids.iter().filter(|c| {
        c.xrange.0 >= -50
            && c.xrange.1 <= 50
            && c.yrange.0 >= -50
            && c.yrange.1 <= 50
            && c.zrange.0 >= -50
            && c.zrange.1 <= 50
    }) {
        universe = repartition(cuboid, &universe);
    }
    universe_on_count(&universe)
}

fn solution2(cuboids: &[Cuboid]) -> usize {
    let mut universe = vec![universe(cuboids)];
    for cuboid in cuboids {
        universe = repartition(cuboid, &universe);
    }
    universe_on_count(&universe)
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
    let cuboids = get_data(&data);
    println!("Answer Part 1 = {}", solution1(&cuboids));
    println!("Answer Part 2 = {}", solution2(&cuboids));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data(filename: &str) -> Vec<Cuboid> {
        let file = Some(std::path::PathBuf::from(filename));
        get_data(&read_data_lines::<String>(file).unwrap())
    }

    #[test]
    fn part1_example() {
        let cuboids = get_test_data("input-example");
        assert_eq!(590784, solution1(&cuboids));
    }

    #[test]
    fn part1_actual() {
        let cuboids = get_test_data("input-actual");
        assert_eq!(551693, solution1(&cuboids));
    }

    #[test]
    fn part2_example() {
        let cuboids = get_test_data("input-example2");
        assert_eq!(474140, solution1(&cuboids));
        assert_eq!(2758514936282235, solution2(&cuboids));
    }

    #[test]
    fn part2_actual() {
        let cuboids = get_test_data("input-actual");
        assert_eq!(1165737675582132, solution2(&cuboids));
    }
}
