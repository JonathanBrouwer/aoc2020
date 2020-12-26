fn part1(inp: &str) -> usize {
    let (pk1, pk2) = parse_input(inp);

    //Reverse engineer number 1
    let mut rest = pk1;
    let mut loopsize1 = 0;
    while rest != 1 {
        if rest % 7 == 0 {
            rest /= 7;
            loopsize1 += 1;
        } else {
            rest += 20201227;
        }
    }

    //Produce encryption key using loop size 1 and pk2
    let mut encrkey = 1;
    for _ in 0..loopsize1 {
        encrkey *= pk2;
        encrkey %= 20201227;
    }

    return encrkey;
}

fn parse_input(inp: &str) -> (usize, usize) {
    let pk1: usize = inp.lines().nth(0).unwrap().parse().unwrap();
    let pk2: usize = inp.lines().nth(1).unwrap().parse().unwrap();
    (pk1, pk2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example"));
        assert_eq!(14897079, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(2947148, result);
    }
}



