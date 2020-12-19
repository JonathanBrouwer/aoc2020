use std::collections::HashMap;
use crate::day19::main_part1::Rule::{OPTION, LITERAL, CONCAT};

fn part1(input: &'static str) -> usize {
    let (rules, words) = parse_input(input);

    words.iter().filter(|word| {
        let res= parse(word, &rules, 0);
        if res.is_err() { return false }
        let res = res.unwrap();
        res.is_empty()
    }).count()
}

fn parse(input: &'static str, rules: &HashMap<usize, Rule>, rule: usize) -> Result<&'static str, ()> {
    match &rules[&rule] {
        LITERAL(c) => {
            if input.starts_with(*c) {
                Ok(&input[1..])
            } else {
                Err(())
            }
        }
        CONCAT(subrules) => {
            let mut output = input;
            for &subrule in subrules {
                output = parse(output, rules, subrule)?;
            }
            Ok(output)
        }
        OPTION(subrules1, subrules2) => {
            //Try matching subrules1
            let mut output = input;
            let mut success = true;
            for &subrule in subrules1 {
                let res = parse(output, rules, subrule);
                if res.is_ok() {
                    output = res.unwrap();
                } else {
                    success = false;
                    break;
                }
            }

            if !success {
                let mut output = input;
                for &subrule in subrules2 {
                    output = parse(output, rules, subrule)?;
                }
                Ok(output)
            } else {
                Ok(output)
            }
        }
    }
}

fn parse_input(inp: &'static str) -> (HashMap<usize, Rule>, Vec<&'static str>) {
    let rules: HashMap<usize, Rule> = inp.split("\n\n").next().unwrap().lines().map(|line| {
        let id: usize = line.split(": ").next().unwrap().parse().unwrap();
        let secondpart = line.split(": ").nth(1).unwrap();
        (id, if secondpart.starts_with("\"") {
            LITERAL(secondpart.chars().nth(1).unwrap())
        } else if secondpart.contains("|") {
            let mut parts: Vec<Vec<usize>> = secondpart.split("|")
                .map(|s| s.trim())
                .map(|s| s.split(" ").map(|n| n.parse().unwrap()).collect())
                .collect();
            let p1 = parts.remove(0);
            let p2 = parts.remove(0);
            OPTION(p1, p2)
        } else {
            CONCAT(secondpart.trim().split(" ").map(|n| n.parse().unwrap()).collect())
        })
    }).collect();

    let words: Vec<&str> = inp.split("\n\n").nth(1).unwrap().lines().collect();
    (rules, words)
}

enum Rule {
    LITERAL(char),
    CONCAT(Vec<usize>),
    OPTION(Vec<usize>, Vec<usize>)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"));
        assert_eq!(2, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(250, result);
    }
}



