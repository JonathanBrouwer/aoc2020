extern crate itertools;
extern crate test;

use self::itertools::Itertools;

fn part1(inp: &str, lookback: usize) -> usize {
    let input = parse_input(inp);

    // For each window of size 26
    *input.windows(lookback + 1).find(|&slice| {
        // Find a window where there is no combination which adds up to the target
        slice[0..lookback].iter().tuple_combinations().all(|(a,b)| a+b != slice[lookback])
    }).unwrap().last().unwrap()
}

fn part2(inp: &str, target: usize) -> usize {
    let input = parse_input(inp);

    //We are doing a walking sum
    let mut min_index = 0;
    let mut max_index = 0;
    let mut cur_sum = input[0];

    //While we did not reach our target
    while cur_sum != target {
        //If our current sum is too big, remove the first element of our range
        //Otherwise, add the element directly after the last element
        if cur_sum > target {
            cur_sum -= input[min_index];
            min_index += 1;
        } else {
            max_index += 1;
            cur_sum += input[max_index];
        }
    }

    //Compute minmax and add the min and max
    let minmax = input[min_index..=max_index].iter().minmax().into_option().unwrap();
    minmax.0 + minmax.1
}

#[inline]
fn parse_input(inp: &str) -> Vec<usize> {
    inp.lines().map(|x|x.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example"), 5);
        assert_eq!(127, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"), 25);
        println!("Part 1: {}", result);
        assert_eq!(675280050, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example"), 127);
        assert_eq!(62, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"), 675280050);
        println!("Part 2: {}", result);
        assert_eq!(96081673, result);
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input = test::black_box(include_str!("input"));
        b.iter(|| {
            let result = part1(input, 25);
            assert_eq!(675280050, result);
        });
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input = test::black_box(include_str!("input"));
        b.iter(|| {
            let result = part2(input, 675280050);
            assert_eq!(96081673, result);
        });
    }
}



