use itertools::{Itertools, Product};
use std::mem;
use std::ops::{Index, IndexMut};
use crate::day17::gridmd::GridMD;
use crate::day17::gridmd_iterator::GridMDIterator;

pub const MAX_POS: usize = 14;
pub const MAX_NEG: usize = 7;

fn part1(inp: &str) -> usize {
    solve::<3>(inp)
}

fn part2(inp: &str) -> usize {
    solve::<4>(inp)
}

///Run the cellular automata in DIM dimensions, returning the number of active cells.
fn solve<const DIM: usize>(inp: &str) -> usize {
    //Parse input
    let mut state = GridMD::<DIM>::new();
    parse_input(inp, &mut state);

    //Run for 6 iterations, swapping the state grids each time
    let mut new_state = GridMD::<DIM>::new();
    for _ in 0..6 {
        next(&state, &mut new_state);
        mem::swap(&mut state, &mut new_state)
    }

    //Count amount of active cells
    state.vec.iter().filter(|&&b| b).count()
}

fn next<const DIM: usize>(state: &GridMD<DIM>, new_state: &mut GridMD<DIM>) {
    //Loop through all cells in the MIN..=MAX hypercube (this doesn't loop through the edges, avoiding the need for bound checking)
    const MIN: isize = -(MAX_NEG as isize) + 1;
    const MAX: isize = MAX_POS as isize - 1;
    for coord in GridMDIterator::<DIM, MIN, MAX>::new() {
        //Loop through the -1..=1 hypercube and count all the active cells
        let mut count = 0;
        for dcoord in GridMDIterator::<DIM, -1, 1>::new() {
            //Don't count the middle cell
            if dcoord == [0; DIM] { continue; }

            //Calculate the final coord = coord + dcoord (rust doens't have array addition :c )
            let mut fcoord: [isize; DIM] = [0; DIM];
            for (i, (c, dc)) in coord.iter().zip(&dcoord).enumerate() {
                fcoord[i] = c + dc;
            }

            //Add to count if active
            if state[fcoord] {
                count += 1;
            }
        }

        //Calculate new state based on old state and count
        new_state[coord] = match (state[coord], count) {
            (true, 2 | 3) => true,
            (false, 3) => true,
            _ => false
        };
    }
}

///Parse the input to a DIM GridMD
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



