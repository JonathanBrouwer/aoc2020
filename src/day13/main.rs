fn part1(inp: &str) -> usize {
    let time: usize = inp.lines().next().unwrap().parse().unwrap();
    let busses: Vec<_> = inp.lines().skip(1).next().unwrap()
        .split(",")
        .filter(|&s| s != "x")
        .map(|s| s.parse().unwrap())
        .collect();

    let result = busses.iter()
        .map(|id| {
            if time % id == 0 { (id, 0) } else {
                (id, id - (time % id))
            }
        }).min_by_key(|&p| p.1).unwrap();
    result.0 * result.1
}

fn part2(inp: &str) -> usize {
    let busses: Vec<usize> = inp.lines().skip(1).next().unwrap()
        .split(",")
        .map(|s| if s == "x" { "1" } else { s })
        .map(|s| s.parse().unwrap())
        .collect();

    let chinese_input = busses.iter().enumerate()
        .filter(|(_tdif, &busid)| busid != 1)
        .map(|(tdif, &busid)| {
            //t + tdif = 0 % busid
            //t = -tdif % busid
            (modulo(0 - tdif as i64, busid as i64), busid as i64)
        }).collect::<Vec<_>>();
    chinese_remainder(&chinese_input[..]).unwrap() as usize
}

fn modulo(a: i64, b: i64) -> i64 {
    ((a % b) + b) % b
}

//TY rosettacode
fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(inputs: &[(i64, i64)] /* (res, mod) */) -> Option<i64> {
    let prod = inputs.iter().map(|&(_r, m)| m).product::<i64>();

    let mut sum = 0;

    for &(residue, modulus) in inputs {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example"));
        assert_eq!(295, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(1835, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example"));
        assert_eq!(1068781, result);
    }

    #[test]
    fn test_part2_otherexs() {
        assert_eq!(3417, part2("\n17,x,13,19"));
        assert_eq!(754018, part2("\n67,7,59,61"));
        assert_eq!(779210, part2("\n67,x,7,59,61"));
        assert_eq!(1261476, part2("\n67,7,x,59,61"));
        assert_eq!(1202161486, part2("\n1789,37,47,1889"));
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(247086664214628, result);
    }
}



