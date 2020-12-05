extern crate test;

use std::cmp::{max, min};

fn part1(inp: &str) -> Result<usize, ()> {
    let input = parse_input(inp);
    return Ok(*input.iter().max().unwrap());
}

fn part2(inp: &str) -> Result<usize, ()> {
    let input = parse_input(inp);
    part2_solve1(&input)
}

fn part2_solve1(input: &Vec<usize>) -> Result<usize, ()> {
    //Find min, max, sum
    let (min, max, sum_is) = input.iter().fold((usize::max_value(), usize::min_value(), 0), |acc, &cur| {
        (min(acc.0, cur), max(acc.1, cur), acc.2 + cur)
    });

    // We sum the range (min ..= max), and subtract all numbers we have in our vec,
    // we will only have the missing number left.
    let sum_shouldbe: usize = (min..=max).sum();
    return Ok(sum_shouldbe - sum_is);
}

fn part2_solve2(input: &Vec<usize>) -> Result<usize, ()> {
    //Set flags of which numbers are present
    let mut flags = [false; 1024];
    for &num in input {
        flags[num] = true;
    }

    Ok(flags.iter().enumerate()
        //Skip while seats are empty
        .skip_while(|(_, &f)| !f)
        //Skip while seats are full
        .skip_while(|(_, &f)| f)
        //Next seat is ours
        .next().unwrap().0)
}

fn parse_input(inp: &str) -> Vec<usize> {
    //For each input line
    inp.lines().map(|line| {
        //Map F/L to 0 (lower half) and B/R to 1 (upper half)
        line.chars().map(|c| match c {
            'F' | 'L' => 0 as usize,
            'B' | 'R' => 1 as usize,
            _ => panic!("Invalid input")
            // Map vec of 0/1 to the binary number this vec represents
        }).fold(0, |acc, b| acc * 2 + b as usize)
    }).collect()
}

#[cfg(test)]
mod tests {
    use std::cmp::{max, min};

    use super::*;
    use super::test::Bencher;

    #[test]
    fn test_part1_ex1() {
        assert_eq!(357, part1("FBFBBFFRLR").unwrap());
        assert_eq!(567, part1("BFFFBBFRRR").unwrap());
        assert_eq!(119, part1("FFFBBBFRRR").unwrap());
        assert_eq!(820, part1("BBFFBBFRLL").unwrap());
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input")).unwrap();
        println!("Part 1: {}", result);
        assert_eq!(828, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input")).unwrap();
        println!("Part 2: {}", result);
        assert_eq!(565, result);
    }

    #[bench]
    fn bench_part2_opt1(b: &mut Bencher) {
        let input = test::black_box(include_str!("input"));
        let input = parse_input(input);
        b.iter(|| {
            let result = part2_solve2(&input).unwrap();
            assert_eq!(565, result);
        });
    }

    #[bench]
    fn bench_part2_opt2(b: &mut Bencher) {
        let input = test::black_box(include_str!("input"));
        let input = parse_input(input);
        b.iter(|| {
            let result = part2_solve2(&input).unwrap();
            assert_eq!(565, result);
        });
    }

    #[bench]
    fn bench_sum_range_a(b: &mut Bencher) {
        let vec: Vec<usize> = test::black_box((1..=10000).collect());
        b.iter(|| {
            let r = (*vec.iter().min().unwrap(), *vec.iter().max().unwrap(), vec.iter().sum::<usize>());
            assert_eq!(r, (1, 10000, 50005000));
        });
    }

    #[bench]
    fn bench_sum_range_b(b: &mut Bencher) {
        let vec: Vec<usize> = test::black_box((1..=10000).collect());
        b.iter(|| {
            let r = vec.iter().fold((usize::max_value(), usize::min_value(), 0), |acc, &cur| {
                (min(acc.0, cur), max(acc.1, cur), acc.2 + cur)
            });
            assert_eq!(r, (1, 10000, 50005000));
        });
    }

    #[bench]
    fn bench_input_parse(b: &mut Bencher) {
        let input: &str = test::black_box(include_str!("input"));
        let correct_result = test::black_box(parse_input(input));
        b.iter(|| {
            let result = parse_input(input);
            assert_eq!(correct_result, result);
        });
    }
}



