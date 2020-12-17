use itertools::{Itertools, Product};
use std::mem;
use std::ops::{Index, IndexMut};
use crate::day17::gridmd::GridMD;
use crate::day17::gridmd_iterator::GridMDIterator;
use packed_simd_2::*;

pub const MAX_POS: usize = 15; //Should be: 8+6+1 = 15
pub const MAX_NEG: usize = 7;  //Should be: 6+1 = 7

fn debug_2d(inp: &str) -> usize { solve::<2>(inp) }

fn part1(inp: &str) -> usize {
    solve::<3>(inp)
}

fn part2(inp: &str) -> usize {
    solve::<4>(inp)
}

///Run the cellular automata in DIM dimensions, returning the number of active cells.
fn solve<const DIM: usize>(inp: &str) -> usize {
    //Parse input
    let mut state: GridMD<u8, DIM> = parse_input(inp);

    //Run for 6 iterations, swapping the state grids each time
    let mut new_state = GridMD::<u8, DIM>::new(0);

    state.print();
    for _ in 0..6 {
        next(&state, &mut new_state);
        mem::swap(&mut state, &mut new_state);
        println!("------");
        state.print();
    }

    //Count amount of active cells
    state.vec.iter().filter(|&&b| b == 1).count()
}

fn next<const DIM: usize>(state: &GridMD<u8, DIM>, state_new: &mut GridMD<u8, DIM>) {
    //Loop through all cells in the MIN..=MAX hypercube (this doesn't loop through the edges, avoiding the need for bound checking)
    const MIN: isize = -(MAX_NEG as isize);
    const MAX: isize = MAX_POS as isize;

    let to_skip: usize = (0..DIM).map(|p| (MAX_NEG + 1 + MAX_POS).pow(p as u32)).sum();
    for coord in GridMDIterator::<DIM, MIN, MAX>::new().skip(to_skip).step_by(64) {
        // println!("-- Coord: {:?}", coord);
        //Loop through the -1..=1 hypercube and count all the active cells
        let mut count = u8x64::splat(0);
        // let mut count = 0;
        for dcoord in GridMDIterator::<DIM, -1, 1>::new() {
            //Don't count the middle cell
            if dcoord == [0; DIM] { continue; }

            //Calculate the final coord = coord + dcoord (rust doens't have array addition :c )
            let mut fcoord: [isize; DIM] = [0; DIM];
            for (i, (c, dc)) in coord.iter().zip(&dcoord).enumerate() {
                fcoord[i] = c + dc;
            }

            //Add to count if active
            // println!("Fcoord: {:?}", fcoord);
            count += u8x64::from_slice_unaligned(&state[fcoord..]);
        }
        // print!("{:?} ", count);

        //Calculate new state based on old state and count
        let mut new = u8x64::from_slice_unaligned(&state[coord..]);
        new = count.eq(u8x64::splat(2)).select(new, u8x64::splat(0));
        new = count.eq(u8x64::splat(3)).select(u8x64::splat(1), new);
        new.write_to_slice_unaligned(&mut state_new[coord..]);
    }
}

///Parse the input to a DIM GridMD
fn parse_input<const DIM: usize>(inp: &str) -> GridMD<u8, DIM> {
    let mut state = GridMD::<u8, DIM>::new(0);

    inp.lines().enumerate().for_each(|(y, l)| {
        l.chars().enumerate().for_each(|(x, c)| {
            let mut arg = [0; DIM];
            arg[0] = x as isize;
            arg[1] = y as isize;
            state[arg] = (c == '#') as u8
        })
    });

    state
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_square() {
        let result = debug_2d(include_str!("example_square"));
        assert_eq!(12, result)
    }

    #[test]
    fn test_example_2d() {
        let result = debug_2d(include_str!("example"));
        assert_eq!(5, result)
    }

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

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input = test::black_box(include_str!("input"));
        b.iter(|| {
            let result = part1(input);
            assert_eq!(289, result);
        });
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input = test::black_box(include_str!("input"));
        b.iter(|| {
            let result = part2(input);
            assert_eq!(2084, result);
        });
    }
}



