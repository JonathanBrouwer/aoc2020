use std::mem;

fn part1(inp: &str) -> usize {
    brute_force(inp, 2020)
}

fn part2(inp: &str) -> usize {
    brute_force(inp, 30000000)
}

fn brute_force(inp: &str, to: usize) -> usize {
    let input = parse_input(inp);

    let mut map = vec![0u32; to];
    for (i, &num) in input.iter().enumerate() {
        map[num] = (i+1) as u32;
    }
    //Last number that has been spoken
    let mut last = 0u32;
    for i in (input.len()+1) as u32..to as u32 {
        //UNSAFE: The size of the map is `to`, and the function can't grow faster, so unchecked is safe
        unsafe {
            let map_last = mem::replace(map.get_unchecked_mut(last as usize), i);
            last = if map_last != 0 { i - map_last } else { 0 };
        }
    }
    return last as usize;
}

fn parse_input(inp: &str) -> Vec<usize> {
    inp.split(",").map(|s| s.parse().unwrap()).collect()
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use test::Bencher;

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
    pub(crate) fn test_part1_real() {
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
    pub(crate) fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(16671510, result);
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input = test::black_box(include_str!("input"));
        b.iter(|| {
            part1(input)
        });
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input = test::black_box(include_str!("input"));
        b.iter(|| {
            part2(input)
        });
    }
}
