use general::read_data_lines;
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use structopt::StructOpt;

const PUZZLE_NAME: &str = "Advent of Code: Day 12 -- Version:";
const PUZZLE_ABOUT: &str = "Passage Pathing: https://adventofcode.com/2021/day/12";

fn get_graph(data: &[String]) -> HashMap<String, HashSet<String>> {
    // parsing rules for data[String]
    // Example:
    //   "start-A",
    //   "start-b",
    //   "A-c",
    //   "A-b",
    //   "b-d",
    //   "A-end",
    //   "b-end"
    // Returns:
    //   graph = {
    //      "start": {"b", "A"},
    //      "b": {"d", "A", "end"},
    //      "A": {"c", "b", "end"}}
    //      "d": {"b"},
    //      "c": {"A"},
    //      "end": {"A", "b"},
    //   }

    let mut graph = HashMap::new();
    for s in data.iter() {
        let nodes = s.trim().split('-').map(|s| s.to_string()).collect::<Vec<String>>();
        assert_eq!(nodes.len(), 2, "expected 2 nodes: {:?}", nodes);
        let (a, b) = (nodes[0].clone(), nodes[1].clone());
        let h = graph.entry(a.clone()).or_insert_with(HashSet::<String>::new);
        h.insert(b.clone());
        if a != "start" && b != "end" {
            let h = graph.entry(b).or_insert_with(HashSet::<String>::new);
            h.insert(a);
        }
    }
    graph
}

fn visit2(
    graph: &HashMap<String, HashSet<String>>,
    node: &str,
    special: &str,
    visited: &mut HashMap<String, usize>,
    paths: &mut Vec<String>,
    solutions: &mut HashSet<Vec<String>>,
) {
    if node == "end" {
        //println!("paths = {:?}", paths);
        solutions.insert(paths.to_vec());
        return;
    }

    if node.to_lowercase() == node {
        let count = visited.entry(node.to_string()).or_insert(0);
        *count += 1;
    }

    if let Some(items) = graph.get(node) {
        for item in items {
            if !visited.contains_key(item) || (item == special && visited.get(special) < Some(&2)) {
                paths.push(item.to_string());
                visit2(graph, item, special, visited, paths, solutions);
                paths.pop();
                if visited.contains_key(item) {
                    let count = visited.get_mut(item).unwrap();
                    *count -= 1;
                    if *count == 0 {
                        visited.remove(item);
                    }
                }
            }
        }
    }
}

fn solution2(graph: &HashMap<String, HashSet<String>>) -> usize {
    let mut visited = HashMap::<String, usize>::new();
    let mut paths = vec![];
    let mut solutions = HashSet::<Vec<String>>::new();
    for k in graph.keys().sorted() {
        if &k.to_lowercase() == k && k != "end" {
            visit2(graph, "start", k, &mut visited, &mut paths, &mut solutions);
        }
    }
    solutions.len()
}

fn visit1(
    graph: &HashMap<String, HashSet<String>>,
    node: &str,
    visited: &mut HashSet<String>,
    paths: &mut Vec<String>,
    solutions: &mut usize,
) {
    if node == "end" {
        //println!("paths = {:?}", paths);
        *solutions += 1;
        return;
    }

    if node.to_lowercase() == node {
        visited.insert(node.to_string());
    }

    if let Some(items) = graph.get(node) {
        for item in items {
            if !visited.contains(item) {
                paths.push(item.to_string());
                visit1(graph, item, visited, paths, solutions);
                paths.pop();
                visited.remove(item);
            }
        }
    }
}

fn solution1(graph: &HashMap<String, HashSet<String>>) -> usize {
    let mut visited = HashSet::<String>::new();
    let mut paths = vec![];
    let mut solutions = 0;
    visit1(graph, "start", &mut visited, &mut paths, &mut solutions);
    solutions
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
    let graph = get_graph(&data);
    //println!("graph = {:?}", graph);
    println!("Answer Part 1 = {}", solution1(&graph));
    println!("Answer Part 2 = {}", solution2(&graph));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_data(filename: &str) -> Vec<String> {
        let file = Some(std::path::PathBuf::from(filename));
        read_data_lines::<String>(file).unwrap()
    }

    #[test]
    fn part1_example() {
        let data = get_data("input-example");
        let graph = get_graph(&data);
        assert_eq!(solution1(&graph), 10);
    }

    #[test]
    fn part1_example2() {
        let data = get_data("input-example2");
        let graph = get_graph(&data);
        assert_eq!(solution1(&graph), 19);
    }

    #[test]
    fn part1_example3() {
        let data = get_data("input-example3");
        let graph = get_graph(&data);
        assert_eq!(solution1(&graph), 226);
    }

    #[test]
    fn part1_actual() {
        let data = get_data("input-actual");
        let graph = get_graph(&data);
        assert_eq!(solution1(&graph), 4186);
    }

    #[test]
    fn part2_example() {
        let data = get_data("input-example");
        let graph = get_graph(&data);
        assert_eq!(solution2(&graph), 36);
    }

    #[test]
    fn part2_example2() {
        let data = get_data("input-example2");
        let graph = get_graph(&data);
        assert_eq!(solution2(&graph), 103);
    }

    #[test]
    fn part2_example3() {
        let data = get_data("input-example3");
        let graph = get_graph(&data);
        assert_eq!(solution2(&graph), 3509);
    }

    #[test]
    fn part2_actual() {
        let data = get_data("input-actual");
        let graph = get_graph(&data);
        assert_eq!(solution2(&graph), 92111);
    }
}
