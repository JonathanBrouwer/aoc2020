use mcmf::{GraphBuilder, Vertex, Cost, Capacity};
use crate::day16::main::VertexValue::{Rule, Col};
use mcmf::Vertex::Node;

fn part1(inp: &'static str) -> usize {
    //Parse input
    let Input { rules, own_ticket: _, other_tickets } = parse_input(inp);

    //For each number in other tickets
    other_tickets.iter().flatten().filter(|&&num| {
        //If no (not any) rule applies
        !rules.iter().any(|&(_, (r1min, r1max), (r2min, r2max))| {
            (r1min <= num && num <= r1max) || (r2min <= num && num <= r2max)
        })
        //Sum the value
    }).sum()
}

fn part2(inp: &'static str) -> usize {
    //Parse input
    let Input { rules, own_ticket, mut other_tickets } = parse_input(inp);

    //Discard completely invalid tickets from other tickets (Opposite of part1)
    other_tickets = other_tickets.into_iter().filter(|ticket| {
        ticket.iter().all(|&num| {
            rules.iter().any(|&(_, (r1min, r1max), (r2min, r2max))| {
                (r1min <= num && num <= r1max) || (r2min <= num && num <= r2max)
            })
        })
    }).collect();

    //Build a graph
    //Each path in the graph looks like: Source -> Rule -> Column ID -> Sink
    let mut gb = GraphBuilder::new();

    //Create edge from source -> rule
    rules.iter().for_each(|&(name, _, _)| {
        gb.add_edge(Vertex::Source, Rule(name), Capacity(1), Cost(0));
    });

    //Create edge from column id -> sink
    (0..own_ticket.len()).for_each(|i| {
        gb.add_edge(Col(i), Vertex::Sink, Capacity(1), Cost(0));
    });

    //Create edge from rule -> column id if possible
    rules.iter().for_each(|&(name, (r1min, r1max), (r2min, r2max))| {
        //For each column id
        (0..own_ticket.len()).filter(|&i| {
            //If all of the other tickets in this column id can fir this rule, keep it
            other_tickets.iter().all(|row| {
                (r1min <= row[i] && row[i] <= r1max) || (r2min <= row[i] && row[i] <= r2max)
            })
            //For each rule that fits, create the edge
        }).for_each(|i| {
            gb.add_edge(Rule(name), Col(i), Capacity(1), Cost(1));
        })
    });

    //Apply maxflow
    let (cost, paths) = gb.mcmf();
    assert_eq!(cost as usize, own_ticket.len());

    //Map the path to a (rule, col) pair
    paths.iter().map(|path| {
        let rule: &Vertex<VertexValue> = path.vertices()[1];
        let col: &Vertex<VertexValue> = path.vertices()[2];
        match (&rule, &col) {
            (Node(Rule(r)), Node(Col(i))) => (r, i),
            (_, _) => unreachable!()
        }
    //Only keep rules with start with departure
    }).filter(|&(&name, _)| {
        name.starts_with("departure")
    //Map column to value on own ticket
    }).map(|(_, &col)| {
        own_ticket[col]
    //Product
    }).product()
}

#[derive(Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
enum VertexValue {
    Rule(&'static str),
    Col(usize),
}

fn parse_input(inp: &'static str) -> Input {
    let mut split = inp.split("\n\n");
    let (sec1, sec2, sec3) = (split.next().unwrap(), split.next().unwrap(), split.next().unwrap());

    //Parse section 1
    let sec1: Vec<(&str, (usize, usize), (usize, usize))> = sec1.lines().map(|line| {
        let name = line.split(": ").next().unwrap();
        let mut split = line.split(": ").skip(1).next().unwrap().split(" ");
        let (r1, _, r2) = (split.next().unwrap(), split.next().unwrap(), split.next().unwrap());
        let (r1min, r1max) = (r1.split("-").next().unwrap().parse().unwrap(), r1.split("-").skip(1).next().unwrap().parse().unwrap());
        let (r2min, r2max) = (r2.split("-").next().unwrap().parse().unwrap(), r2.split("-").skip(1).next().unwrap().parse().unwrap());
        (name, (r1min, r1max), (r2min, r2max))
    }).collect();

    //Parse section 2
    let sec2: Vec<usize> = sec2.lines().skip(1).next().unwrap().split(",").map(|s| s.parse().unwrap()).collect();

    //Parse section 3
    let sec3: Vec<Vec<usize>> = sec3.lines().skip(1).map(|line| {
        line.split(",").map(|s| s.parse().unwrap()).collect()
    }).collect();

    return Input {
        rules: sec1,
        own_ticket: sec2,
        other_tickets: sec3,
    };
}

struct Input {
    rules: Vec<(&'static str, (usize, usize), (usize, usize))>,
    own_ticket: Vec<usize>,
    other_tickets: Vec<Vec<usize>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"));
        assert_eq!(71, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(20058, result);
    }

    #[test]
    fn test_part2_ex2() {
        let result = part2(include_str!("example2"));
        assert_eq!(11*13, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(366871907221, result);
    }
}



