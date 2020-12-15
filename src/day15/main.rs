use std::collections::HashMap;

fn part1(inp: &str) -> usize {
    brute_force(inp, 2020)
}

fn part2(inp: &str) -> usize {
    brute_force(inp, 30000000)
}

fn brute_force(inp: &str, to: usize) -> usize {
    let input = parse_input(inp);

    // let mut map = [0; 40000000];
    let mut map = HashMap::new();
    let mut i = 1;
    for &num in &input {
        map.insert(num, i);
        i+=1;
    }
    //Last number that has been spoken
    let mut last = *input.last().unwrap();
    //How many turns it was last spoken before, if any
    let mut last_before = None;
    loop {
        last = last_before.unwrap_or(0);
        last_before = map.get(&last).map(|&l| i-l);
        map.insert(last, i);
        if i == to {
            println!("{}", map.values().max().unwrap());
            return last;
        }
        i += 1;
    }
}

fn parse_input(inp: &str) -> Vec<usize> {
    inp.split(",").map(|s| s.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        assert_eq!(436, part1("0,3,6"));
        assert_eq!(1, part1("1,3,2"));
        assert_eq!(10, part1("2,1,3"));
        assert_eq!(27, part1("1,2,3"));
        assert_eq!(78, part1("2,3,1"));
        assert_eq!(438, part1("3,2,1"));
        assert_eq!(1836, part1("3,1,2"));
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(929, result);
    }

    #[test]
    fn test_part2_ex1() {
        assert_eq!(175594, part2("0,3,6"));
        assert_eq!(2578, part2("1,3,2"));
        assert_eq!(3544142, part2("2,1,3"));
        assert_eq!(261214, part2("1,2,3"));
        assert_eq!(6895259, part2("2,3,1"));
        assert_eq!(18, part2("3,2,1"));
        assert_eq!(362, part2("3,1,2"));
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(16671510, result);
    }
}
