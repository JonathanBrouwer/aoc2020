fn part1(inp: &str) -> usize {
    let input = parse_input(inp);

    return 0;
    // return Err(())
}

fn part2(inp: &str) -> usize {
    let input = parse_input(inp);

    return 0;
    // return Err(())
}

fn parse_input(inp: &str) -> () {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example"));
        assert_eq!(0, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(0, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example"));
        assert_eq!(0, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(0, result);
    }
}



