use std::time::SystemTime;

fn part1(inp: &str) -> Result<usize, ()> {
    let input = parse_input(inp);
    return Ok(input.iter().filter(
        |l| {
            let count = l.password.chars().filter(|c| *c == l.rule_letter).count();
            l.rule_firstnum <= count && count <= l.rule_secondnum
        }
    ).count());
}

fn part2(inp: &str) -> Result<usize, ()> {
    let input = parse_input(inp);
    return Ok(input.iter().filter(
        |l| {
            let let1 = l.password.chars().nth(l.rule_firstnum-1).unwrap() == l.rule_letter;
            let let2 = l.password.chars().nth(l.rule_secondnum-1).unwrap() == l.rule_letter;
            let1 ^ let2
        }
    ).count());
}

fn parse_input(inp: &str) -> Vec<Line> {
    let now = SystemTime::now();

    let mut rtrn = Vec::new();
    for line in inp.lines() {
        rtrn.push(fast_parse_line(line))
    }
    println!("Input parse time: {} ms", now.elapsed().unwrap().as_nanos());
    rtrn
}

fn fast_parse_line(line: &str) -> Line {
    let splits = line.split(&['-', ':', ' '][..]);
    if let &[num1, num2, letter, _, pw] = splits.collect::<Vec<_>>().as_slice() {
        return Line {
            password: pw,
            rule_letter: letter.chars().next().unwrap(),
            rule_firstnum: num1.parse().unwrap(),
            rule_secondnum: num2.parse().unwrap()
        }
    }
    panic!()
}

struct Line<'a> {
    password: &'a str,
    rule_letter: char,
    rule_firstnum: usize,
    rule_secondnum: usize
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::SystemTime;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example")).unwrap();
        assert_eq!(2, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input")).unwrap();
        println!("Part 1: {}", result);
        assert_eq!(460, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example")).unwrap();
        assert_eq!(1, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input")).unwrap();
        println!("Part 2: {}", result);
        assert_eq!(251, result);
    }

    #[test]
    fn test_part2_bench() {
        let now = SystemTime::now();

        let count = 100;
        let input = include_str!("input");
        for _ in 0..count {
            part2(input).unwrap();
        }

        println!("Part 2 time: {} ns", now.elapsed().unwrap().as_nanos()/count);
    }
}



