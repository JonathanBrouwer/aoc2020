use itertools::{Itertools, Product};
use std::mem;
use std::ops::{Index, IndexMut};
use crate::day17::gridmd::GridMD;

pub const MAX_POS: usize = 14;
pub const MAX_NEG: usize = 7;

fn part1(inp: &str) -> usize {
    solve::<3>(inp)
}

fn part2(inp: &str) -> usize {
    solve::<4>(inp)
}

fn solve<const DIM: usize>(inp: &str) -> usize {
    let mut state = GridMD::<DIM>::new();
    parse_input(inp, &mut state);
    let mut new_state = GridMD::<DIM>::new();
    for _ in 0..6 {
        next(&state, &mut new_state);
        mem::swap(&mut state, &mut new_state)
    }
    state.vec.iter().filter(|&&b| b).count()
}

fn next<const DIM: usize>(state: &GridMD<DIM>, new_state: &mut GridMD<DIM>) {
    const MIN: isize = -(MAX_NEG as isize) + 1;
    const MAX: isize = MAX_POS as isize - 1;
    for coord in GridMDIterator::<DIM, MIN, MAX>::new() {
        let mut count = 0;
        for dcoord in GridMDIterator::<DIM, -1, 1>::new() {
            if dcoord == [0; DIM] { continue; }
            let mut fcoord: [isize; DIM] = [0; DIM];
            for (i, (c, dc)) in coord.iter().zip(&dcoord).enumerate() {
                fcoord[i] = c + dc;
            }
            if state[fcoord] {
                count += 1;
            }
        }
        new_state[coord] = match (state[coord], count) {
            (true, 2 | 3) => true,
            (false, 3) => true,
            _ => false
        };
    }
}

struct GridMDIterator<const DIM: usize, const MIN: isize, const MAX: isize> {
    last: [isize; DIM]
}

impl<const DIM: usize, const MIN: isize, const MAX: isize> GridMDIterator<DIM, MIN, MAX> {
    fn new() -> Self {
        let mut res = GridMDIterator { last: [MIN; DIM] };
        res.last[DIM-1] = MIN-1;
        res
    }
}

impl<const DIM: usize, const MIN: isize, const MAX: isize> Iterator for GridMDIterator<DIM, MIN, MAX> {
    type Item = [isize; DIM];

    fn next(&mut self) -> Option<Self::Item> {
        for i in (0..DIM).rev() {
            if self.last[i] == MAX {
                self.last[i] = MIN;
            } else {
                self.last[i] += 1;
                return Some(self.last);
            }
        }
        return None;
    }
}

fn parse_input<const DIM: usize>(inp: &str, state: &mut GridMD<DIM>) -> () {
    inp.lines().enumerate().for_each(|(y, l)| {
        l.chars().enumerate().for_each(|(x, c)| {
            let mut arg = [0; DIM];
            arg[0] = x as isize;
            arg[1] = y as isize;
            state[arg] = c == '#'
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example"));
        assert_eq!(112, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(289, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example"));
        assert_eq!(848, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(2084, result);
    }
}



