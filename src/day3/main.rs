fn part1(inp: &str) -> Result<usize, ()> {
    let input = parse_input(inp);

    //Our current x position
    let mut x = 0;
    //Total amount of trees
    let mut total = 0;

    //Just keep walking over positions until we reach the bottom
    for line in input {
        if line[x % line.len()] { total += 1; }
        x += 3;
    }
    return Ok(total)
}

fn part2(inp: &str) -> Result<usize, ()> {
    let input = parse_input(inp);
    let slopes: Vec<(usize, usize)> = vec![(1,1), (3,1), (5,1), (7,1), (1,2)];

    //Iterate through all slopes
    Ok(slopes.iter().map(|slope| {
        let mut pos = (0,0);
        let mut subtotal = 0;

        //Just keep walking over positions until we reach the bottom
        while pos.1 < input.len() {
            if input[pos.1][pos.0 % input[pos.1].len()] { subtotal += 1; }
            pos.0 += slope.0;
            pos.1 += slope.1;
        }
        subtotal
    }).product())
}

fn parse_input(inp: &str) -> Vec<Vec<bool>> {
    inp.lines().map(|line| {
        line.chars().map(|c| c == '#').collect()
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example")).unwrap();
        assert_eq!(7, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input")).unwrap();
        println!("Part 1: {}", result);
        assert_eq!(289, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example")).unwrap();
        assert_eq!(336, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input")).unwrap();
        println!("Part 2: {}", result);
        assert_eq!(5522401584, result);
    }
}



