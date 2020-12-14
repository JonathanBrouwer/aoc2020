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
                mask.iter()
                    .zip(to_bitmap(index).iter())
                    .fold(vec![0usize], |vec, (mbit, vbit)| {
                        match (mbit, vbit) {
                            (BIT(false), &vbit) => vec.iter().map(|&a| (a<<1)+(vbit as usize)).collect(),
                            (BIT(true), _) => vec.iter().map(|&a| (a<<1)+1).collect(),
                            (DONT_CARE, _) => {
                                vec.iter().flat_map(|&a| {
                                    vec![(a<<1)+0, (a<<1)+1]
                                }).map(|a| a).collect()
                            }
                        }
                    }).iter().for_each(|&a| {
                    hmap.insert(a, val);
                });
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

#[derive(Copy, Clone, Debug)]
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
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"));
        assert_eq!(165, result);
    }

    #[test]
    fn test_part1_real() {
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
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(3705162613854, result);
    }
}



