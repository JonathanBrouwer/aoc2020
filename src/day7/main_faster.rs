extern crate test;

use std::collections::{HashMap, HashSet};

fn part1(inp: &str) -> Result<usize, ()> {
    let (_input, input_rev) = parse_input(inp);

    let mut found = HashSet::new();
    let mut found_to_continue = Vec::new();
    found_to_continue.push("shiny gold bag");
    found.insert("shiny gold bag");

    while !found_to_continue.is_empty() {
        let next = found_to_continue.remove(found_to_continue.len() - 1);
        for &(name, _) in input_rev.get(next).unwrap_or(&Vec::new()) {
            if !found.contains(name) {
                found.insert(name);
                found_to_continue.push(name);
            }
        }
    }

    return Ok(found.len() - 1);
}

fn part2(inp: &str) -> Result<usize, ()> {
    let (input, input_rev) = parse_input(inp);

    let mut left: HashMap<&str, usize> = input.iter().map(|(&k, v)| (k, v.can_contain.len())).collect();
    let mut done: Vec<&str> = left.iter().filter_map(|(&k, &v)| if v == 0 { Some(k) } else { None }).collect();
    let mut done_total: HashMap<&str, usize> = done.iter().map(|&k| (k, 0)).collect();

    while !done.is_empty() {
        let next = done.remove(done.len() - 1);
        for &(k, _v) in input_rev.get(next).unwrap_or(&Vec::new()) {
            left.insert(k, *left.get(k).unwrap() - 1);
            //If this thing is now done
            if *left.get(k).unwrap() == 0 {
                done.push(k);
                done_total.insert(k, input.get(k).unwrap().can_contain.iter().map(|&(k, v)| {
                    (done_total.get(k).unwrap() + 1) * v
                }).sum());
            }
        }
    }

    return Ok(*done_total.get("shiny gold bag").unwrap());
    // return Err(())
}

#[inline]
fn parse_input(inp: &str) -> (HashMap<&str, Baggage>, HashMap<&str, Vec<(&str, usize)>>) {
    let mut map = HashMap::new();

    //Parse input
    for line in inp.lines() {
        //Remove dot at end
        let line = &line[..line.len() - 1];
        //Parse line into inputs and outputs
        let (inp, outp) = line.split_once(" contain ").unwrap();
        //Make sure name is not plural
        let inp = inp.strip_suffix("s").unwrap();
        //Parse the outputs into a vec
        let outps: Vec<(&str, usize)> = outp.split(", ").filter(|&c| c != "no other bags").map(parse_name).collect();
        //Insert into map
        map.insert(inp, Baggage { name: &inp, can_contain: outps });
    }

    //Set can_be_contained_in
    let mut rev_map: HashMap<&str, Vec<(&str, usize)>> = HashMap::new();
    for (&name, bag) in &map {
        for (in_name, in_count) in bag.can_contain.clone() {
            if !rev_map.contains_key(in_name) {
                rev_map.insert(in_name, Vec::new());
            }
            let vec = rev_map.get_mut(in_name).unwrap();
            vec.push((name, in_count));
        }
    }

    (map, rev_map)
}

#[derive(Hash, Eq, PartialEq)]
struct Baggage<'a> {
    name: &'a str,
    can_contain: Vec<(&'a str, usize)>,
}

fn parse_name<'a>(mut inp: &'a str) -> (&'a str, usize) {
    if inp.ends_with('s') {
        inp = &inp[..inp.len() - 1];
    }
    let split = inp.split_once(" ").unwrap();
    (split.1, split.0.parse().unwrap())
}

#[cfg(test)]
mod tests {
    use test::Bencher;

    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1")).unwrap();
        assert_eq!(4, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input")).unwrap();
        println!("Part 1: {}", result);
        assert_eq!(378, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example1")).unwrap();
        assert_eq!(32, result);
    }

    #[test]
    fn test_part2_ex2() {
        let result = part2(include_str!("example2")).unwrap();
        assert_eq!(126, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input")).unwrap();
        println!("Part 2: {}", result);
        assert_eq!(27526, result);
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input = test::black_box(include_str!("input"));
        b.iter(|| {
            let result = part1(input).unwrap();
            assert_eq!(378, result);
        });
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input = test::black_box(include_str!("input"));
        b.iter(|| {
            let result = part2(input).unwrap();
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



