use crate::day14::main::Instruction::{SET_MASK, WRITE};
use crate::day14::main::BitMaskBit::{BIT, DONT_CARE};
use std::collections::HashMap;

fn part1(inp: &str) -> usize {
    let input = parse_input(inp);

    let mut hmap = HashMap::new();
    let mut mask = [DONT_CARE; 36];
    for instr in input {
        match instr {
            SET_MASK(new_mask) => mask = new_mask,
            WRITE(index, val) => {
                let mut bitmap: [bool; 36] = [false; 36];
                mask.iter().zip(to_bitmap(val).iter()).map(|(mbit, vbit)| match (mbit, vbit) {
                    (DONT_CARE, &v) => v,
                    (BIT(v), _) => *v
                }).enumerate().for_each(|(i, v)| bitmap[i] = v);
                hmap.insert(index, bitmap);
            }
        }
    }

    hmap.values().map(|v| {
        v.iter().fold(0, |acc, b| acc * 2 + *b as usize)
    }).sum()
}

fn part2(inp: &str) -> usize {
    let input = parse_input(inp);

    let mut hmap = HashMap::new();
    let mut mask = [DONT_CARE; 36];
    for instr in input {
        match instr {
            SET_MASK(new_mask) => mask = new_mask,
            WRITE(index, val) => {
                let mut address = mask.iter()
                    .zip(to_bitmap(index).iter())
                    .fold(0usize, |num, (mbit, vbit)| {
                        match (mbit, vbit) {
                            (BIT(false), &vbit) => (num << 1) + (vbit as usize),
                            (BIT(true), _) => (num << 1) + (1 as usize),
                            (DONT_CARE, _) => (num << 1) + (0 as usize),
                        }
                    });
                let x_indices: Vec<_> = mask.iter().enumerate().filter(|&(i, &v)| v == DONT_CARE).map(|(i, _)| (36-i-1)).collect();
                for x_values in 0..1<<x_indices.len() {
                    for (i, &index) in x_indices.iter().enumerate() {
                        if x_values & (1 << i) != 0 {
                            address |= 1 << index;
                        }else{
                            address &= !(1 << index);
                        }
                    }
                    hmap.insert(address, val);
                }
            }
        }
    }

    hmap.values().sum()
}

fn parse_input(inp: &str) -> Vec<Instruction> {
    inp.lines().map(|line| {
        if line.starts_with("mask = ") {
            let mut bitmap = [DONT_CARE; 36];
            line.chars().skip(7).enumerate()
                .map(|(i, c)| match c {
                    '0' => (i, BIT(false)),
                    '1' => (i, BIT(true)),
                    'X' => (i, DONT_CARE),
                    _ => unreachable!()
                }).for_each(|(i, v)| bitmap[i] = v);
            SET_MASK(bitmap)
        } else {
            let index: usize = line.split("[").skip(1).next().unwrap().split("]").next().unwrap().parse().unwrap();
            let val: usize = line.split(" = ").skip(1).next().unwrap().parse().unwrap();

            WRITE(index, val)
        }
    }).collect()
}

fn to_bitmap(val: usize) -> [bool; 36] {
    let mut bitmap = [false; 36];
    for i in 0..36 {
        if val & (1 << i) != 0 {
            bitmap[36 - i - 1] = true;
        }
    }
    bitmap
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum BitMaskBit {
    BIT(bool),
    DONT_CARE,
}

#[derive(Copy, Clone, Debug)]
enum Instruction {
    SET_MASK([BitMaskBit; 36]),
    WRITE(usize, usize),
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"));
        assert_eq!(165, result);
    }

    #[test]
    pub(crate) fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(7611244640053, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example2"));
        assert_eq!(208, result);
    }

    #[test]
    pub(crate) fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(3705162613854, result);
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



