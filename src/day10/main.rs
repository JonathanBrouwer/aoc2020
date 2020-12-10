extern crate test;

fn part1(inp: &str) -> usize {
    let mut input = parse_input(inp);
    //We add 0 and max+3 to the input, and sort it
    input.push(0);
    input.sort_unstable();
    input.push(input.last().unwrap() + 3);

    //Calculate the differences
    let diffs: Vec<_> = input.windows(2).map(|win| win[1]-win[0]).collect();
    //Count times which 1 and 3 occurs
    let count_1 = diffs.iter().filter(|&&x| x == 1).count();
    let count_3 = diffs.iter().filter(|&&x| x == 3).count();
    //Multiply
    count_1 * count_3
}

fn part2(inp: &str) -> usize {
    //Get input and sort it
    let mut input = parse_input(inp);
    input.sort_unstable();

    //We fold the input
    //Acc contains the previous values which are still relevant (still less than 3 from cur)
    input.iter().fold(vec![(0, 1)], |mut prev, &cur| {
        //Remove all previous values which are no longer relevant
        while prev.first().unwrap().0 + 3 < cur {
            prev.remove(0);
        }
        //New value can be reached by all still relevant previous values summed
        prev.push((cur, prev.iter().map(|&(_num, count)| count).sum()));
        prev
    }).last().unwrap().1
}

#[inline]
fn parse_input(inp: &str) -> Vec<usize> {
    inp.lines().map(|x| x.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use test::Bencher;

    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"));
        assert_eq!(5 * 7, result);
    }

    #[test]
    fn test_part1_ex2() {
        let result = part1(include_str!("example2"));
        assert_eq!(22 * 10, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(2070, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example1"));
        assert_eq!(8, result);
    }

    #[test]
    fn test_part2_ex2() {
        let result = part2(include_str!("example2"));
        assert_eq!(19208, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(24179327893504, result);
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input = test::black_box(include_str!("input"));
        b.iter(|| {
            let result = part1(input);
            assert_eq!(2070, result);
        });
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input = test::black_box(include_str!("input"));
        b.iter(|| {
            let result = part2(input);
            assert_eq!(24179327893504, result);
        });
    }
}



