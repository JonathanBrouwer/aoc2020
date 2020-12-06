extern crate test;

use bit_array::BitArray;

fn part1(inp: &str) -> Result<u32, ()> {
    solve(inp, |a, b| (a | b))
}

fn part2(inp: &str) -> Result<u32, ()> {
    solve(inp, |a, b| (a & b))
}

#[inline]
fn solve<F>(inp: &str, mut foldfun: F) -> Result<u32, ()>
    where F: FnMut(u32, u32) -> u32 {

    //For each group
    return Ok(inp.split("\n\n").map(|group| {
        //Map each person to a bitmap of which letters it consists of
        group.lines().map(|person| {
            //Create bitmap
            let mut bitmap = 0 as u32;
            //Set flags in bitmap
            person.chars().for_each(|c| {
                let i = c as usize - 'a' as usize;
                bitmap |= 1 << i;
            });
            //Return bitmap
            bitmap
        })
            //Fold the persons of the group using the foldfun
            .fold_first(|a, b| foldfun(a, b))
            //Count amount of 1 bits in the u32
            .unwrap().count_ones() as u32
    }).sum::<u32>());
}

#[cfg(test)]
mod tests {
    use test::Bencher;

    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example")).unwrap();
        assert_eq!(11, result);
    }

    #[test]
    fn test_part1_real() {
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
    fn test_part2_real() {
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
