use crate::day19::main_part2::Rule::{LITERAL, OPTION, CONCAT};
use std::collections::HashMap;

fn part2(input: &'static str) -> usize {
    let (mut rules, words) = parse_input(input);

    //Update rules
    rules.insert(8, OPTION(vec![42, 8], vec![42]));
    rules.insert(11, OPTION(vec![42, 11, 31], vec![42, 31]));

    //For each word, test if it matches
    words.iter().filter(|word| {
        //It matches if there was a way to parse it such that the entire string is consumed
        let res= parse(word, &rules, 0);
        res.contains(&"")
    }).count()
}

fn parse(input: &'static str, rules: &HashMap<usize, Rule>, rule: usize) -> Vec<&'static str> {
    match &rules[&rule] {
        LITERAL(c) => {
            //We can parse this if the string starts with c
            if input.starts_with(*c) {
                vec![&input[1..]]
            } else {
                vec![]
            }
        }
        CONCAT(subrules) => {
            //Parse each subrule, and feed it the output of the previous one
            let mut output = vec![input];
            for &subrule in subrules {
                output = output.iter().flat_map(|s| parse(s, rules, subrule)).collect();
            }
            output
        }
        OPTION(subrules1, subrules2) => {
            //Try matching subrules1
            let mut output1 = {
                let mut output = vec![input];
                for &subrule in subrules1 {
                    output = output.iter().flat_map(|s| parse(s, rules, subrule)).collect();
                }
                output
            };

            //Try matching subrules2
            let mut output2 = {
                let mut output = vec![input];
                for &subrule in subrules2 {
                    output = output.iter().flat_map(|s| parse(s, rules, subrule)).collect();
                }
                output
            };

            //Our options are subrules1 and subrules2 concatenated.
            output1.append(&mut output2);
            output1
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
pub(crate) mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_part2_test() {
        let result = part2(include_str!("test"));
        assert_eq!(3, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example2"));
        assert_eq!(12, result);
    }

    #[test]
    pub(crate) fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(359, result);
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input = test::black_box(include_str!("input"));
        b.iter(|| {
            let result = part2(input);
            assert_eq!(359, result);
        });
    }
}



