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

    //Prealloc memory for to store intermediates
    let mut mem = Vec::with_capacity(input.len() + 1);
    mem.push((0, 1usize));

    //For each input
    for num in input {
        //Get the last 3 items
        let sum = mem.iter().rev()
            .take(3)
            //Take only the ones which have a difference of at most 3
            .take_while(|&&(c, _)| c + 3 >= num)
            //Sum their values
            .map(|&(_, s)| s).sum();
        //Add to mem
        mem.push((num, sum));
    }

    //Take last sum
    mem.last().unwrap().1
}

#[inline]
fn parse_input(inp: &str) -> Vec<usize> {
    inp.lines().map(|x| x.parse().unwrap()).collect()
}

#[cfg(test)]
pub(crate) mod tests {
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
    pub(crate) fn test_part1_real() {
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
    pub(crate) fn test_part2_real() {
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



