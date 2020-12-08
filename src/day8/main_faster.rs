extern crate test;

use bit_array::BitArray;
use petgraph::{Graph, Directed};

use crate::day8::main_faster::Instruction::{ACC, JMP, NOP};
use petgraph::graphmap::GraphMap;

fn part1(inp: &str) -> isize {
    let mut computer = Computer::from_input(inp);
    let mut bitmap = BitArray::<u64, typenum::U1024>::from_elem(false);

    //Continue running instructions
    loop {
        //Store previous acc
        let prev_acc = computer.acc;

        //Step, and if it looped, return the prev acc
        computer.step();
        if bitmap[computer.instr_counter] {
            return prev_acc;
        }

        //Mark this instruction as already been there
        bitmap.set(computer.instr_counter, true);
    }
}

fn part2(inp: &str) -> isize {
    let mut computer = Computer::from_input(inp);

    //Try flipping each possible instruction
    for i in 0..computer.instructions.len() {
        //Swap instruction at index i
        let old_instr = computer.instructions[i];
        computer.instructions[i] = match computer.instructions[i] {
            ACC(_) => continue,
            JMP(num) => NOP(num),
            NOP(num) => JMP(num)
        };

        //Run and see if it terminates
        let mut bitmap = BitArray::<u64, typenum::U1024>::from_elem(false);
        //Loop returns if it terminated or looped
        let did_term = loop {
            //Step computer
            computer.step();
            //Did it loop?
            if bitmap[computer.instr_counter] {
                break false;
            }
            //Did it terminate?
            if computer.has_terminated() {
                break true;
            }
            //Mark this instruction as already been there
            bitmap.set(computer.instr_counter, true);
        };
        //If it terminated, return the accumelator
        if did_term {
            return computer.acc;
        }
        //Reset computer for another round
        computer.reset();
        bitmap.clear();
        computer.instructions[i] = old_instr;
    }
    unreachable!();
}

#[derive(Clone)]
struct Computer {
    acc: isize,
    instructions: Vec<Instruction>,
    instr_counter: usize,
}

#[derive(Clone, Copy)]
enum Instruction {
    ACC(isize),
    JMP(isize),
    NOP(isize),
}

impl Computer {
    #[inline]
    fn step(&mut self) {
        //Match on instruction, and let each instruction do its thing
        match self.instructions[self.instr_counter] {
            ACC(num) => {
                self.acc += num;
                self.instr_counter += 1;
            }
            JMP(num) => {
                self.instr_counter = ((self.instr_counter as isize) + num) as usize;
            }
            NOP(_num) => {
                self.instr_counter += 1;
            }
        }
    }

    fn run_until_terminate(&mut self) {
        while !self.has_terminated() {
            self.step();
        }
    }

    #[inline]
    fn has_terminated(&self) -> bool {
        //We terminated if the instruction counter is >= the amount of instructions
        self.instr_counter >= self.instructions.len()
    }

    #[inline]
    fn reset(&mut self) {
        //Reset the computer so it can be ran again
        self.acc = 0;
        self.instr_counter = 0;
    }

    #[inline]
    fn from_input(inp: &str) -> Computer {
        //Parse the computer from the input
        let instr = inp.lines().map(|line| {
            let parts: Vec<_> = line.split(" ").collect();
            match parts[0] {
                "acc" => ACC(parts[1].parse().unwrap()),
                "jmp" => JMP(parts[1].parse().unwrap()),
                "nop" => NOP(parts[1].parse().unwrap()),
                _ => unreachable!()
            }
        }).collect();
        return Computer {
            acc: 0,
            instr_counter: 0,
            instructions: instr,
        };
    }
}

#[cfg(test)]
mod tests {
    use test::Bencher;

    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example"));
        assert_eq!(5, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(1331, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example"));
        assert_eq!(8, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(1121, result);
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input = test::black_box(include_str!("input"));
        b.iter(|| {
            let result = part1(input);
            assert_eq!(1331, result);
        });
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input = test::black_box(include_str!("input"));
        b.iter(|| {
            let result = part2(input);
            assert_eq!(1121, result);
        });
    }
}



