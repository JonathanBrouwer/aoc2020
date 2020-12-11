extern crate test;

use std::collections::HashMap;

use petgraph::prelude::*;
use petgraph::visit::{IntoNeighbors, Reversed};

fn part1(inp: &str) -> usize {
    //Parse input into graph
    let graph = parse_input(inp);

    //Do dfs on the graph
    let mut dfs = Dfs::new(&graph, "shiny gold bag");

    //Count all nodes found
    let mut count = 0;
    while let Some(_) = dfs.next(&graph) {
        count += 1;
    }

    //Substract 1, since we don't count the shiny gold bag itself
    return count - 1;
}

fn part2(inp: &str) -> usize {
    //Parse input into graph
    let mut graph = parse_input(inp);

    //Toposort the graph
    let mut tps: Vec<&str> = petgraph::algo::toposort(&graph, None).unwrap();

    //Walk over toposort
    let mut contains_in = HashMap::<&str, usize>::new();
    for node in tps {
        //For each node, keep track of the total amount of bags it contains
        let weight: usize = Reversed(&graph).neighbors(node).map(|nb| {
            (contains_in.get(nb).unwrap() + 1) * (*graph.edge_weight(nb, node).unwrap())
        }).sum();
        contains_in.insert(node, weight);

        //Stop if we found the shiny gold bag
        if node == "shiny gold bag" { return weight; }
    }
    unreachable!();
}

#[inline(always)]
fn parse_input(inp: &str) -> GraphMap<&str, usize, Directed> {
    GraphMap::<&str, usize, Directed>::from_edges(inp.lines().flat_map(|line| {
        // Remove dot at end of line
        let line = &line[..line.len() - 1];

        //Parse line into inputs and outputs
        let (inp, outp) = line.split_once(" contain ").unwrap();
        //Make sure name is not plural
        let inp = inp.strip_suffix("s").unwrap();

        //Return the edges
        outp.split(", ")
            //Skip things that have no outgoing edges
            .filter(|&c| c != "no other bags")
            //Parse name of outputs
            .map(parse_name)
            //Map to (source, dest, weight)
            .map(|(k, v)| (k, inp, v)).collect::<Vec<_>>()
    }))
}

#[inline(always)]
fn parse_name<'a>(mut inp: &'a str) -> (&'a str, usize) {
    if inp.ends_with('s') {
        inp = &inp[..inp.len() - 1];
    }
    let split = inp.split_once(" ").unwrap();
    (split.1, split.0.parse().unwrap())
}

#[cfg(test)]
pub(crate) mod tests {
    use test::Bencher;

    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"));
        assert_eq!(4, result);
    }

    #[test]
    pub(crate) fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(378, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example1"));
        assert_eq!(32, result);
    }

    #[test]
    fn test_part2_ex2() {
        let result = part2(include_str!("example2"));
        assert_eq!(126, result);
    }

    #[test]
    pub(crate) fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(27526, result);
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input = test::black_box(include_str!("input"));
        b.iter(|| {
            let result = part1(input);
            assert_eq!(378, result);
        });
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input = test::black_box(include_str!("input"));
        b.iter(|| {
            let result = part2(input);
            assert_eq!(27526, result);
        });
    }

    #[bench]
    fn bench_input_parse(b: &mut Bencher) {
        let input: &str = test::black_box(include_str!("input"));
        b.iter(|| {
            parse_input(input);
        });
    }
}



