extern crate test;

use std::ops::{BitAnd, BitOr};

fn part1(inp: &str) -> Result<u32, ()> {
    solve(inp, u32::bitor)
}

fn part2(inp: &str) -> Result<u32, ()> {
    solve(inp, u32::bitand)
}

#[inline(always)]
fn solve<F>(inp: &str, mut foldfun: F) -> Result<u32, ()>
    where F: FnMut(u32, u32) -> u32 {

    //For each group
    return Ok(inp.split("\n\n").map(|group| {
        //Map each person to a bitmap of which letters it consists of
        group.lines().map(|person| {
            person.bytes().fold(0u32, |acc, c| {
                acc | 1 << (c - b'a')
            })
        })
            //Fold the persons of the group using the foldfun
            .fold_first(|a, b| foldfun(a, b))
            //Count amount of 1 bits in the u32
            .unwrap().count_ones() as u32
    }).sum::<u32>());
}

#[cfg(test)]
pub(crate) mod tests {
    use test::Bencher;

    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example")).unwrap();
        assert_eq!(11, result);
    }

    #[test]
    pub(crate) fn test_part1_real() {
        let result = part1(include_str!("input")).unwrap();
        println!("Part 1: {}", result);
        assert_eq!(7110, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example")).unwrap();
        assert_eq!(6, result);
    }

    #[test]
    pub(crate) fn test_part2_real() {
        let result = part2(include_str!("input")).unwrap();
        println!("Part 2: {}", result);
        assert_eq!(3628, result);
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input = test::black_box(include_str!("input"));
        b.iter(|| {
            let result = part1(input).unwrap();
            assert_eq!(7110, result);
        });
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input = test::black_box(include_str!("input"));
        b.iter(|| {
            let result = part2(input).unwrap();
            assert_eq!(3628, result);
        });
    }
}
