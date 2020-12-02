extern crate test;

fn part1(inp: &str) -> Result<usize, ()> {
    let input = parse_input(inp);
    return Ok(input.iter().filter(|p| p.is_valid_part1()).count());
}

fn part2(inp: &str) -> Result<usize, ()> {
    let input = parse_input(inp);
    return Ok(input.iter().filter(|p| p.is_valid_part2()).count());
}

#[inline]
fn parse_input(inp: &str) -> Vec<Password> {
    let mut rtrn = Vec::new();
    for line in inp.lines() {
        rtrn.push(fast_parse_line(line))
    }
    rtrn
}

#[inline]
fn fast_parse_line(line: &str) -> Password {
    let splits = line.split(&['-', ':', ' '][..]);
    if let &[num1, num2, letter, _, pw] = splits.collect::<Vec<_>>().as_slice() {
        return Password {
            password: pw,
            rule_letter: letter.chars().next().unwrap(),
            rule_firstnum: num1.parse().unwrap(),
            rule_secondnum: num2.parse().unwrap()
        }
    }
    panic!()
}

struct Password<'a> {
    password: &'a str,
    rule_letter: char,
    rule_firstnum: usize,
    rule_secondnum: usize
}

impl Password<'_> {
    fn is_valid_part1(&self) -> bool {
        let count = self.password.chars().filter(|c| *c == self.rule_letter).count();
        self.rule_firstnum <= count && count <= self.rule_secondnum
    }

    fn is_valid_part2(&self) -> bool {
        let let1 = self.password.chars().nth(self.rule_firstnum-1).unwrap() == self.rule_letter;
        let let2 = self.password.chars().nth(self.rule_secondnum-1).unwrap() == self.rule_letter;
        let1 ^ let2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

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

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(|| {
            let input = include_str!("input");
            part2(test::black_box(input)).unwrap();
        });
    }
}



