use std::collections::{HashMap, HashSet};
use mcmf::{GraphBuilder, Vertex, Capacity, Cost};
use itertools::Itertools;
use mcmf::Vertex::Node;

fn part1(inp: &str) -> usize {
    let input = parse_input(inp);

    let mut possibilities: HashMap<&str, Vec<&str>> = HashMap::new();
    for (ingrds, allergs) in &input {
        for allerg in allergs {
            if possibilities.contains_key(allerg) {
                let pos = possibilities.get_mut(allerg).unwrap();
                pos.retain(|el| ingrds.contains(el));
            } else {
                possibilities.insert(allerg, ingrds.clone());
            }
        }
    }

    let possible: HashSet<&str> = possibilities.values().flatten().map(|x| *x).collect();
    input.iter().map(|(i, a)| i).flatten().filter(|x| !possible.contains(*x)).count()


}

fn part2(inp: &str) -> String {
    let input = parse_input(inp);

    let mut possibilities: HashMap<&str, Vec<&str>> = HashMap::new();
    for (ingrds, allergs) in &input {
        for allerg in allergs {
            if possibilities.contains_key(allerg) {
                let pos = possibilities.get_mut(allerg).unwrap();
                pos.retain(|el| ingrds.contains(el));
            } else {
                possibilities.insert(allerg, ingrds.clone());
            }
        }
    }

    let possible_ingredients: HashSet<&str> = possibilities.values().flatten().map(|x| *x).collect();


    //Build a graph
    //Each path in the graph looks like: Source -> Allergen -> Ingredient -> Sink
    let mut gb = GraphBuilder::new();

    //Create edge from source -> Allergen
    possibilities.keys().for_each(|&allerg| {
        gb.add_edge(Vertex::Source, allerg, Capacity(1), Cost(0));
    });

    //Create edge from Ingredient -> sink
    possible_ingredients.iter().for_each(|&ingredient| {
        gb.add_edge(ingredient, Vertex::Sink, Capacity(1), Cost(0));
    });

    //Create edge from Allergen -> Ingredient if possible
    possibilities.iter().for_each(|(&allerg, ingredients)| {
        ingredients.iter().for_each(|&ingredient| {
            gb.add_edge(allerg, ingredient, Capacity(1), Cost(1));
        })
    });

    //Apply maxflow
    let (cost, paths) = gb.mcmf();
    assert_eq!(cost as usize, possible_ingredients.len());

    //Map the path to a (allergen, ingredient) pair
    paths.iter().map(|path| {
        let allerg: &Vertex<&str> = path.vertices()[1];
        let ingred: &Vertex<&str> = path.vertices()[2];
        match (&allerg, &ingred) {
            (Node(allerg), Node(ingred)) => (*allerg, *ingred),
            _ => unreachable!()
        }
    }).sorted_by_key(|(a,i)| *a)
        .map(|(a, i)| i).join(",")
}

fn parse_input(inp: &str) -> Vec<(Vec<&str>, Vec<&str>)> {
    inp.lines().map(|line| {
        let part1 = line.split(" (contains ").next().unwrap();
        let part2 = line.split(" (contains ").nth(1).unwrap().split(")").next().unwrap();
        (part1.split(" ").collect(), part2.split(", ").collect())
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example"));
        assert_eq!(5, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(1977, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example"));
        assert_eq!("mxmxvkd,sqjhc,fvjkl", result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!("dpkvsdk,xmmpt,cxjqxbt,drbq,zmzq,mnrjrf,kjgl,rkcpxs", result);
    }
}



