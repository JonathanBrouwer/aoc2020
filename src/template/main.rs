fn part1(inp: &str) -> Result<i64, ()> {
    let input = parse_input(inp);

    return Err(());
}

fn part2(inp: &str) -> Result<i64, ()> {
    let input = parse_input(inp);

    return Err(());
}

fn parse_input(inp: &str) -> () {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example")).unwrap();
        assert_eq!(0, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input")).unwrap();
        println!("Part 1: {}", result);
        assert_eq!(0, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example")).unwrap();
        assert_eq!(0, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input")).unwrap();
        println!("Part 2: {}", result);
        assert_eq!(0, result);
    }
}



